//! Passwordless admin login + feedback dashboard.
//!
//! Flow: user enters their email at `/admin/login` → server checks the
//! email against an allowlist → server emails a single-use magic link
//! `/admin/login/verify?token=…` → on click, server verifies the token
//! and sets a 7-day session cookie → user lands on `/admin/feedback`.
//!
//! SECURITY: The admin secret (HMAC key) is read once at startup from
//! `PLAUSIDEN_ADMIN_SECRET`. If unset, every admin route refuses every
//! request — fail-closed by default. Magic-link tokens carry a random
//! `jti` (token id) which is recorded in `admin_consumed_tokens` after
//! redemption; replay attempts within the link's TTL are rejected. The
//! session cookie is HMAC-signed but stateless: revoking it requires
//! rotating `PLAUSIDEN_ADMIN_SECRET`.
//!
//! BUG ASSUMPTION: The admin email allowlist is small (≤5). The
//! check is a `.contains()` over a `Vec<String>`; not algorithmically
//! interesting at this size, and lower latency than a HashSet for
//! typical 1–3 admin emails.

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::Form;
use axum::extract::{Query, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Redirect, Response};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use blake3::Hash;
use chrono::{TimeZone, Utc};
use lettre::message::{Mailbox, Message, MultiPart};
use lettre::transport::smtp::AsyncSmtpTransport;
use lettre::{AsyncTransport, Tokio1Executor};
use rand::RngCore;
use serde::Deserialize;

use crate::feedback_store::FeedbackStore;
use crate::views::admin as admin_views;

/// Magic-link TTL: 15 minutes. Long enough for the user to switch
/// from inbox to browser; short enough that a leaked link decays.
const MAGIC_LINK_TTL_SECS: u64 = 15 * 60;
/// Session-cookie TTL: 7 days. Re-login each week is a reasonable
/// admin posture.
const SESSION_TTL_SECS: u64 = 7 * 24 * 60 * 60;
/// Cookie name for the admin session.
const SESSION_COOKIE: &str = "pd_admin_session";

/// Shared admin state. Cloned per-request via Axum's `State`
/// extractor.
#[derive(Clone)]
pub struct AdminState {
    /// HMAC key for tokens + sessions. Empty string disables every
    /// admin route — fail-closed.
    pub(crate) secret: Arc<String>,
    /// Allowlist of admin email addresses. Empty list disables every
    /// admin route — fail-closed.
    pub(crate) allowed_emails: Arc<Vec<String>>,
    /// Feedback store for both data viewing and the consumed-tokens
    /// table.
    pub(crate) feedback: Arc<FeedbackStore>,
    /// Local SMTP transport used to send magic links.
    pub(crate) mailer: Arc<AsyncSmtpTransport<Tokio1Executor>>,
}

impl std::fmt::Debug for AdminState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AdminState")
            .field("secret_set", &!self.secret.is_empty())
            .field("allowed_count", &self.allowed_emails.len())
            .finish_non_exhaustive()
    }
}

impl AdminState {
    /// Returns `true` when both the secret and allowlist are set.
    /// Routes refuse every request when this is `false`.
    fn is_configured(&self) -> bool {
        !self.secret.is_empty() && !self.allowed_emails.is_empty()
    }

    fn is_allowed(&self, email: &str) -> bool {
        let normalized = email.trim().to_ascii_lowercase();
        self.allowed_emails
            .iter()
            .any(|e| e.to_ascii_lowercase() == normalized)
    }
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

fn keyed_mac(secret: &str, payload: &str) -> Hash {
    let mut key = [0u8; 32];
    let bytes = secret.as_bytes();
    let n = bytes.len().min(32);
    key[..n].copy_from_slice(&bytes[..n]);
    blake3::keyed_hash(&key, payload.as_bytes())
}

/// Constant-time hex string compare. Inputs must be ASCII hex of the
/// same length; mismatched lengths short-circuit to false (and that
/// asymmetry is fine — the lengths are both controlled by us, not the
/// attacker).
fn ct_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff: u8 = 0;
    for (x, y) in a.bytes().zip(b.bytes()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// Build a magic-link token. Format: base64url(`v1|email|exp|jti`)`.`base64url(mac).
/// The `jti` is 16 random bytes hex-encoded; the receiver records it in
/// `admin_consumed_tokens` on first redemption.
fn mint_magic_token(secret: &str, email: &str) -> (String, u64, String) {
    let exp = unix_now() + MAGIC_LINK_TTL_SECS;
    let mut jti_bytes = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut jti_bytes);
    let jti = hex_encode(&jti_bytes);
    let payload = format!("v1|{email}|{exp}|{jti}");
    let mac = keyed_mac(secret, &payload);
    let token = format!(
        "{}.{}",
        URL_SAFE_NO_PAD.encode(payload.as_bytes()),
        URL_SAFE_NO_PAD.encode(mac.as_bytes()),
    );
    (token, exp, jti)
}

/// Verify a magic-link token. Returns `Ok((email, jti, exp_unix))` on
/// success, `Err(reason)` otherwise. The caller is responsible for
/// recording the JTI in `admin_consumed_tokens` and rejecting replays.
fn verify_magic_token(secret: &str, token: &str) -> Result<(String, String, u64), &'static str> {
    let (payload_b64, mac_b64) = token.split_once('.').ok_or("malformed token")?;
    let payload_bytes = URL_SAFE_NO_PAD
        .decode(payload_b64.as_bytes())
        .map_err(|_| "bad payload encoding")?;
    let payload_str = std::str::from_utf8(&payload_bytes).map_err(|_| "non-utf8 payload")?;
    let mac_bytes = URL_SAFE_NO_PAD
        .decode(mac_b64.as_bytes())
        .map_err(|_| "bad mac encoding")?;
    let expected = keyed_mac(secret, payload_str);
    if mac_bytes.len() != 32 {
        return Err("bad mac length");
    }
    // blake3 hashes are 32 bytes; constant-time compare via the helper.
    if !ct_eq(&hex_encode(&mac_bytes), &hex_encode(expected.as_bytes())) {
        return Err("bad mac");
    }
    let mut parts = payload_str.splitn(4, '|');
    let version = parts.next().ok_or("bad payload")?;
    if version != "v1" {
        return Err("bad version");
    }
    let email = parts.next().ok_or("bad payload")?.to_string();
    let exp: u64 = parts
        .next()
        .ok_or("bad payload")?
        .parse()
        .map_err(|_| "bad exp")?;
    let jti = parts.next().ok_or("bad payload")?.to_string();
    if exp < unix_now() {
        return Err("expired");
    }
    Ok((email, jti, exp))
}

/// Build a session cookie value. Format mirrors the magic link but
/// without a JTI (sessions are stateless, not single-use).
fn mint_session(secret: &str, email: &str) -> (String, u64) {
    let exp = unix_now() + SESSION_TTL_SECS;
    let payload = format!("v1|{email}|{exp}");
    let mac = keyed_mac(secret, &payload);
    let value = format!(
        "{}.{}",
        URL_SAFE_NO_PAD.encode(payload.as_bytes()),
        URL_SAFE_NO_PAD.encode(mac.as_bytes()),
    );
    (value, exp)
}

fn verify_session(secret: &str, value: &str) -> Result<String, &'static str> {
    let (payload_b64, mac_b64) = value.split_once('.').ok_or("malformed session")?;
    let payload_bytes = URL_SAFE_NO_PAD
        .decode(payload_b64.as_bytes())
        .map_err(|_| "bad encoding")?;
    let payload_str = std::str::from_utf8(&payload_bytes).map_err(|_| "non-utf8 payload")?;
    let mac_bytes = URL_SAFE_NO_PAD
        .decode(mac_b64.as_bytes())
        .map_err(|_| "bad encoding")?;
    let expected = keyed_mac(secret, payload_str);
    if mac_bytes.len() != 32 {
        return Err("bad mac length");
    }
    if !ct_eq(&hex_encode(&mac_bytes), &hex_encode(expected.as_bytes())) {
        return Err("bad mac");
    }
    let mut parts = payload_str.splitn(3, '|');
    let version = parts.next().ok_or("bad payload")?;
    if version != "v1" {
        return Err("bad version");
    }
    let email = parts.next().ok_or("bad payload")?.to_string();
    let exp: u64 = parts
        .next()
        .ok_or("bad payload")?
        .parse()
        .map_err(|_| "bad exp")?;
    if exp < unix_now() {
        return Err("expired");
    }
    Ok(email)
}

/// Extract the admin session cookie from a request, returning the
/// session email if valid. Used by gated handlers to check authn.
fn session_email(state: &AdminState, headers: &HeaderMap) -> Option<String> {
    if state.secret.is_empty() {
        return None;
    }
    let cookie_header = headers.get(header::COOKIE)?.to_str().ok()?;
    for part in cookie_header.split(';') {
        let kv = part.trim();
        if let Some(value) = kv.strip_prefix(&format!("{SESSION_COOKIE}=")) {
            return verify_session(&state.secret, value).ok();
        }
    }
    None
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push(hex_char(b >> 4));
        s.push(hex_char(b & 0x0f));
    }
    s
}

const fn hex_char(n: u8) -> char {
    match n {
        0..=9 => (b'0' + n) as char,
        _ => (b'a' + n - 10) as char,
    }
}

// ----- Routes ----------------------------------------------------------------

/// `GET /admin` — route to the dashboard if signed in, login otherwise.
/// Saves users from typing the longer URL.
pub(crate) async fn admin_root(
    State(state): State<crate::inquiry::InquiryState>,
    headers: HeaderMap,
) -> Response {
    if !state.admin.is_configured() {
        return admin_disabled_response();
    }
    if session_email(&state.admin, &headers).is_some() {
        Redirect::to("/admin/feedback").into_response()
    } else {
        Redirect::to("/admin/login").into_response()
    }
}

/// `GET /admin/login` — render the email-entry form.
pub(crate) async fn login_form(State(state): State<crate::inquiry::InquiryState>) -> Response {
    if !state.admin.is_configured() {
        return admin_disabled_response();
    }
    admin_views::login(None).into_response()
}

#[derive(Debug, Deserialize)]
pub(crate) struct LoginPost {
    #[serde(default)]
    pub(crate) email: String,
}

/// `POST /admin/login` — accept the email, send the magic link.
///
/// SECURITY: The response is identical whether the email is in the
/// allowlist or not — preventing enumeration of admin addresses.
pub(crate) async fn login_post(
    State(state): State<crate::inquiry::InquiryState>,
    Form(form): Form<LoginPost>,
) -> Response {
    if !state.admin.is_configured() {
        return admin_disabled_response();
    }
    let email = form.email.trim().to_string();
    if email.is_empty() || email.len() > 200 || !email.contains('@') {
        return admin_views::login(Some("That doesn't look like an email address. Try again."))
            .into_response();
    }

    if state.admin.is_allowed(&email) {
        let (token, _exp, _jti) = mint_magic_token(&state.admin.secret, &email);
        let link = format!("https://plausiden.com/admin/login/verify?token={token}");
        if let Err(e) = send_magic_email(&state.admin.mailer, &email, &link).await {
            tracing::warn!(error = %e, "magic link email send failed");
        } else {
            tracing::info!("magic link sent");
        }
    } else {
        // Fixed delay to flatten the timing signal (an unauthorized
        // address would otherwise return faster than an authorized one).
        tokio::time::sleep(std::time::Duration::from_millis(450)).await;
        tracing::warn!("admin login attempted with non-allowlisted email");
    }
    admin_views::magic_link_sent(&email).into_response()
}

#[derive(Debug, Deserialize)]
pub(crate) struct VerifyQuery {
    #[serde(default)]
    pub(crate) token: String,
}

/// `GET /admin/login/verify?token=…` — consume the magic link and
/// install the session cookie.
pub(crate) async fn verify(
    State(state): State<crate::inquiry::InquiryState>,
    Query(q): Query<VerifyQuery>,
) -> Response {
    if !state.admin.is_configured() {
        return admin_disabled_response();
    }
    let (email, jti, exp) = match verify_magic_token(&state.admin.secret, &q.token) {
        Ok(t) => t,
        Err(reason) => {
            tracing::warn!(reason, "magic link verify failed");
            return admin_views::login_error(
                "This link is no longer valid. Request a fresh one below.",
            )
            .into_response();
        }
    };
    if !state.admin.is_allowed(&email) {
        // Defense in depth — the email shouldn't have made it past
        // login_post, but if it did (secret rotation, allowlist edit),
        // the verify must still refuse.
        return admin_views::login_error("That account is no longer authorized.").into_response();
    }
    let exp_iso = i64::try_from(exp)
        .ok()
        .and_then(|s| Utc.timestamp_opt(s, 0).single())
        .map_or_else(|| Utc::now().to_rfc3339(), |dt| dt.to_rfc3339());
    match state.admin.feedback.consume_token(&jti, &exp_iso).await {
        Ok(true) => {}
        Ok(false) => {
            return admin_views::login_error(
                "That magic link has already been used. Request a fresh one below.",
            )
            .into_response();
        }
        Err(e) => {
            tracing::warn!(error = %e, "consume_token failed");
            return admin_views::login_error(
                "Something went wrong on our end. Try requesting a fresh link.",
            )
            .into_response();
        }
    }

    let (cookie, _exp) = mint_session(&state.admin.secret, &email);
    let mut headers = HeaderMap::new();
    let cookie_value = format!(
        "{SESSION_COOKIE}={cookie}; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age={SESSION_TTL_SECS}",
    );
    if let Ok(v) = HeaderValue::from_str(&cookie_value) {
        headers.insert(header::SET_COOKIE, v);
    }
    (
        StatusCode::SEE_OTHER,
        headers,
        Redirect::to("/admin/feedback"),
    )
        .into_response()
}

/// `POST /admin/logout` — clear the session cookie.
pub(crate) async fn logout() -> Response {
    let mut headers = HeaderMap::new();
    let clear = format!("{SESSION_COOKIE}=; Path=/; HttpOnly; Secure; SameSite=Lax; Max-Age=0");
    if let Ok(v) = HeaderValue::from_str(&clear) {
        headers.insert(header::SET_COOKIE, v);
    }
    (StatusCode::SEE_OTHER, headers, Redirect::to("/admin/login")).into_response()
}

/// `GET /admin/feedback` — gated dashboard listing every feedback row.
pub(crate) async fn feedback_dashboard(
    State(state): State<crate::inquiry::InquiryState>,
    headers: HeaderMap,
) -> Response {
    if !state.admin.is_configured() {
        return admin_disabled_response();
    }
    let Some(email) = session_email(&state.admin, &headers) else {
        return Redirect::to("/admin/login").into_response();
    };
    let rows = match state.feedback.list_all().await {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!(error = %e, "feedback list failed");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                admin_views::login_error("Couldn't load the dashboard. Try again."),
            )
                .into_response();
        }
    };
    admin_views::feedback_dashboard(&email, &rows).into_response()
}

fn admin_disabled_response() -> Response {
    (
        StatusCode::NOT_FOUND,
        admin_views::login_error("Admin login is not configured on this server."),
    )
        .into_response()
}

async fn send_magic_email(
    mailer: &AsyncSmtpTransport<Tokio1Executor>,
    to: &str,
    link: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let from: Mailbox = "PlausiDen Admin <team@plausiden.com>".parse()?;
    let to_mb: Mailbox = to.parse()?;
    let plain = format!(
        "Sign in to PlausiDen admin\n\n\
         Use this link within 15 minutes to sign in:\n\n\
         {link}\n\n\
         If you didn't request this, ignore the message — the link\n\
         expires automatically and one-time use is enforced.\n\n\
         — PlausiDen LLC\n",
    );
    let html = crate::views::email::magic_link_html(link);
    let email = Message::builder()
        .from(from)
        .to(to_mb)
        .subject("Sign in to PlausiDen admin")
        .multipart(MultiPart::alternative_plain_html(plain, html))?;
    mailer.send(email).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state() -> AdminState {
        let store = FeedbackStore::open_in_memory().unwrap();
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous("127.0.0.1")
            .port(2525)
            .build();
        AdminState {
            secret: Arc::new("test-secret-32-bytes-or-fewer-ok".to_string()),
            allowed_emails: Arc::new(vec!["admin@plausiden.com".into()]),
            feedback: store,
            mailer: Arc::new(mailer),
        }
    }

    #[test]
    fn magic_token_round_trip() {
        let s = state();
        let (tok, exp, jti) = mint_magic_token(&s.secret, "admin@plausiden.com");
        let (email, jti2, exp2) = verify_magic_token(&s.secret, &tok).expect("valid token");
        assert_eq!(email, "admin@plausiden.com");
        assert_eq!(jti, jti2);
        assert_eq!(exp, exp2);
    }

    #[test]
    fn magic_token_rejects_tampered_payload() {
        let s = state();
        let (tok, _, _) = mint_magic_token(&s.secret, "admin@plausiden.com");
        // Flip a byte in the payload portion.
        let dot = tok.find('.').unwrap();
        let mut bytes: Vec<u8> = tok.into_bytes();
        bytes[dot - 1] ^= 0x01;
        let bad = String::from_utf8(bytes).unwrap();
        assert!(verify_magic_token(&s.secret, &bad).is_err());
    }

    #[test]
    fn magic_token_rejects_wrong_secret() {
        let s = state();
        let (tok, _, _) = mint_magic_token(&s.secret, "admin@plausiden.com");
        assert!(verify_magic_token("different-secret", &tok).is_err());
    }

    #[test]
    fn session_round_trip() {
        let s = state();
        let (cookie, _) = mint_session(&s.secret, "admin@plausiden.com");
        let email = verify_session(&s.secret, &cookie).expect("valid session");
        assert_eq!(email, "admin@plausiden.com");
    }

    #[test]
    fn allowlist_is_case_insensitive() {
        let s = state();
        assert!(s.is_allowed("Admin@PlausiDen.com"));
        assert!(s.is_allowed("admin@plausiden.com"));
        assert!(!s.is_allowed("attacker@elsewhere.com"));
    }

    #[tokio::test]
    async fn consume_token_rejects_replay() {
        let s = state();
        let exp = Utc::now().to_rfc3339();
        assert!(s.feedback.consume_token("jti-1", &exp).await.unwrap());
        // Second time: replay rejected
        assert!(!s.feedback.consume_token("jti-1", &exp).await.unwrap());
    }
}
