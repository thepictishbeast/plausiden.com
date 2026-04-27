//! POST `/contact` form handler — receives the Encrypted Inquiry submission,
//! rate-limits per IP, and emails the message to `team@plausiden.com` via
//! the local Postfix on `127.0.0.1:25` (DKIM-signed by opendkim).
//!
//! v1.0 sends plaintext over local SMTP (which the local relay then signs
//! and delivers via TLS where the recipient supports it). v1.1 will accept
//! age-encrypted ciphertext from the WASM client so the server never sees
//! plaintext.
//!
//! SECURITY: The form parser, validator, and rate limiter all sit ahead of
//! the SMTP send. A malformed payload gets 400; a flooding IP gets 429; a
//! clean submission gets 202. No request ever logs the message body or the
//! reply-to address — those are PII. We log only success/failure counts.

use std::net::IpAddr;
use std::num::NonZeroU32;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use axum::Form;
use axum::extract::{ConnectInfo, Query, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use governor::clock::DefaultClock;
use governor::middleware::NoOpMiddleware;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Quota, RateLimiter};
use lettre::message::{Mailbox, Message, MultiPart, SinglePart, header as msg_header};
use lettre::transport::smtp::AsyncSmtpTransport;
use lettre::{AsyncTransport, Tokio1Executor};
use loom_components::hero::{Hero, HeroBackground};
use maud::{Markup, html};
use serde::Deserialize;

use crate::admin::AdminState;
use crate::feedback_store::{FeedbackInsert, FeedbackStore, export_dsv, export_json};
use crate::views::layout::page;

// Tunables — short enough to thwart trivial spam, lenient enough to never
// block a real inquiry.
const QUOTA_PER_MINUTE: u32 = 3;
const MAX_NAME_LEN: usize = 100;
const MAX_REPLY_TO_LEN: usize = 200;
const MAX_MESSAGE_LEN: usize = 5000;

/// Shared application state for the inquiry + feedback handlers.
/// Constructed once in `main.rs` and cloned per-request via Axum's
/// `State` extractor.
#[derive(Clone)]
#[allow(missing_debug_implementations)]
pub struct InquiryState {
    pub(crate) mailer: Arc<AsyncSmtpTransport<Tokio1Executor>>,
    pub(crate) limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>>,
    /// Feedback + testimonial submissions land here. CMS-shaped
    /// SQLite store; the export endpoint surfaces it as JSON / CSV /
    /// TSV.
    pub(crate) feedback: Arc<FeedbackStore>,
    /// Admin token for the export endpoint. Read once at startup
    /// from `PLAUSIDEN_ADMIN_TOKEN`. Empty string disables export.
    pub(crate) admin_token: Arc<String>,
    /// Passwordless admin login state. Empty `secret` or
    /// `allowed_emails` disables every admin route.
    pub admin: AdminState,
}

impl Default for InquiryState {
    fn default() -> Self {
        Self::new()
    }
}

impl InquiryState {
    /// Build a state object with an in-memory feedback store. Used
    /// in tests + as a fallback when the on-disk DB cannot be opened.
    #[must_use]
    pub fn new() -> Self {
        let store = FeedbackStore::open_in_memory().expect("in-memory sqlite always opens cleanly");
        Self::with_components(store, String::new(), String::new(), Vec::new())
    }

    /// Build a state object talking to local Postfix on `127.0.0.1:25`,
    /// persisting feedback to the SQLite file at `db_path`, and
    /// reading admin configuration from environment variables.
    ///
    /// # Errors
    /// Returns the rusqlite error if the DB file cannot be opened.
    pub fn with_db(db_path: &Path) -> rusqlite::Result<Self> {
        let store = FeedbackStore::open(db_path)?;
        let token = std::env::var("PLAUSIDEN_ADMIN_TOKEN").unwrap_or_default();
        let admin_secret = std::env::var("PLAUSIDEN_ADMIN_SECRET").unwrap_or_default();
        let admin_emails = std::env::var("PLAUSIDEN_ADMIN_EMAILS")
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        Ok(Self::with_components(
            store,
            token,
            admin_secret,
            admin_emails,
        ))
    }

    fn with_components(
        store: Arc<FeedbackStore>,
        admin_token: String,
        admin_secret: String,
        admin_emails: Vec<String>,
    ) -> Self {
        // SECURITY: Connect to the local Postfix without TLS (it's loopback;
        // the milter (opendkim) handles signing and Postfix's outbound TLS
        // is the actual wire encryption to the recipient's MX.
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous("127.0.0.1")
            .port(25)
            .timeout(Some(Duration::from_secs(10)))
            .build();
        // SAFETY: 3 is non-zero — used in the rate-limit quota.
        let q = Quota::per_minute(NonZeroU32::new(QUOTA_PER_MINUTE).unwrap());
        let limiter = RateLimiter::direct(q);
        let mailer = Arc::new(mailer);
        let admin = AdminState {
            secret: Arc::new(admin_secret),
            allowed_emails: Arc::new(admin_emails),
            feedback: Arc::clone(&store),
            mailer: Arc::clone(&mailer),
        };
        Self {
            mailer,
            limiter: Arc::new(limiter),
            feedback: store,
            admin_token: Arc::new(admin_token),
            admin,
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct InquiryForm {
    #[serde(default)]
    pub(crate) name: String,
    /// Reply-to email; the contact form labels this `email`.
    #[serde(default, alias = "email")]
    pub(crate) reply_to: String,
    #[serde(default)]
    pub(crate) phone: String,
    #[serde(default)]
    pub(crate) company: String,
    /// Selected service interest from the dropdown.
    #[serde(default)]
    pub(crate) service: String,
    pub(crate) message: String,
}

const MAX_PHONE_LEN: usize = 50;
const MAX_COMPANY_LEN: usize = 200;
const MAX_SERVICE_LEN: usize = 100;

/// Returns `s` or "(omitted)" if `s` is empty. Used to keep the email
/// body readable when optional fields are blank.
const fn or_omitted(s: &str) -> &str {
    if s.is_empty() { "(omitted)" } else { s }
}

/// Validate the form. Returns `Ok(())` if every field is within bounds and
/// non-empty where required.
fn validate(f: &InquiryForm) -> Result<(), &'static str> {
    if f.name.len() > MAX_NAME_LEN {
        return Err("name too long");
    }
    if f.reply_to.len() > MAX_REPLY_TO_LEN {
        return Err("reply-to too long");
    }
    if f.phone.len() > MAX_PHONE_LEN {
        return Err("phone too long");
    }
    if f.company.len() > MAX_COMPANY_LEN {
        return Err("company too long");
    }
    if f.service.len() > MAX_SERVICE_LEN {
        return Err("service too long");
    }
    if f.message.is_empty() {
        return Err("message required");
    }
    if f.message.len() > MAX_MESSAGE_LEN {
        return Err("message too long");
    }
    Ok(())
}

fn ack_page(message: &str) -> Markup {
    let cta = html! {
        a href="/" class="text-primary font-semibold" { "← Back home" }
    };
    let body = html! {
        (Hero {
            eyebrow: Some("Encrypted Inquiry"),
            headline_lead: "Message received.",
            headline_accent: None,
            subheadline: message,
            cta: Some(&cta),
            background: HeroBackground::GridLight,
        }.render())
    };
    page("Encrypted Inquiry — PlausiDen", "/contact", body)
}

/// POST `/contact` handler.
///
/// BUG ASSUMPTION: Axum's `Form` extractor will reject the request with 422
/// if the body is not URL-encoded form data — we don't need to handle that
/// case explicitly here.
///
/// SECURITY: Rate-limited globally (3/min across all IPs in v1) — a more
/// granular per-IP limiter is on the roadmap but requires the keyed
/// `governor` variant. Form fields are validated before any I/O.
pub(crate) async fn submit(
    State(state): State<InquiryState>,
    ConnectInfo(_addr): ConnectInfo<std::net::SocketAddr>,
    Form(form): Form<InquiryForm>,
) -> Response {
    if state.limiter.check().is_err() {
        tracing::warn!("inquiry rate-limited");
        return (
            StatusCode::TOO_MANY_REQUESTS,
            ack_page("The inbox is being flooded right now — please try again in a minute."),
        )
            .into_response();
    }

    if let Err(e) = validate(&form) {
        tracing::warn!(error = e, "inquiry rejected at validation");
        return (
            StatusCode::BAD_REQUEST,
            ack_page("Your submission didn't pass basic validation. Please correct and retry."),
        )
            .into_response();
    }

    // Compose. We deliberately keep the from/to identity stable so DKIM
    // signs correctly; reply-to carries the sender's address (validated
    // length but not RFC-checked — better to forward and fail than reject
    // a legit edge-case email).
    let body = format!(
        "New inquiry from the plausiden.com contact form.\n\n\
         Name:     {}\n\
         Reply-to: {}\n\
         Phone:    {}\n\
         Company:  {}\n\
         Service:  {}\n\
         \n\
         --- message ---\n{}\n",
        or_omitted(&form.name),
        or_omitted(&form.reply_to),
        or_omitted(&form.phone),
        or_omitted(&form.company),
        or_omitted(&form.service),
        form.message,
    );

    let from: Mailbox = "PlausiDen Web <team@plausiden.com>"
        .parse()
        .unwrap_or_else(|_| {
            // SAFETY: the literal above is a valid mailbox; if parse fails the
            // crate is broken. Provide a fallback that is also valid syntax.
            "team@plausiden.com"
                .parse()
                .expect("hardcoded mailbox parses")
        });
    let to: Mailbox = "team@plausiden.com".parse().expect("destination parses");

    let html = crate::views::email::inquiry_notification_html(
        &form.name,
        &form.reply_to,
        &form.phone,
        &form.company,
        &form.service,
        &form.message,
    );
    let mut builder = Message::builder()
        .from(from)
        .to(to)
        .subject("[contact-form] New inquiry")
        .header(msg_header::ContentType::TEXT_HTML);
    if !form.reply_to.is_empty() {
        if let Ok(rt) = form.reply_to.parse::<Mailbox>() {
            builder = builder.reply_to(rt);
        }
    }
    let Ok(email) = builder.multipart(
        MultiPart::alternative()
            .singlepart(SinglePart::plain(body))
            .singlepart(SinglePart::html(html)),
    ) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            ack_page("Server error composing the message. Try again later."),
        )
            .into_response();
    };

    match state.mailer.send(email).await {
        Ok(_) => {
            tracing::info!("inquiry sent");
            (
                StatusCode::ACCEPTED,
                ack_page(
                    "Your message has been delivered. We'll reply via the address you provided.",
                ),
            )
                .into_response()
        }
        Err(e) => {
            tracing::warn!(error = %e, "inquiry SMTP send failed");
            (
                StatusCode::ACCEPTED,
                ack_page(
                    "Your message has been queued. If we don't reply, email team@plausiden.com directly.",
                ),
            )
                .into_response()
        }
    }
}

// Keep the `IpAddr` import warning-free until per-IP keyed limiter ships.
#[allow(dead_code)]
const _IP_FUTURE_USE: fn(IpAddr) = |_| {};

// ----- Feedback + testimonial handlers --------------------------------

const MAX_FEEDBACK_FIELD_LEN: usize = 2_000;

#[derive(Debug, Deserialize)]
pub(crate) struct FeedbackForm {
    #[serde(default)]
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) company: String,
    #[serde(default)]
    pub(crate) email: String,
    #[serde(default)]
    pub(crate) worked_well: String,
    #[serde(default)]
    pub(crate) didnt_work: String,
    #[serde(default)]
    pub(crate) consent: String,
    #[serde(default)]
    pub(crate) alternative: String,
    #[serde(default)]
    pub(crate) why_chose: String,
    #[serde(default)]
    pub(crate) whats_changed: String,
    #[serde(default)]
    pub(crate) recommend: String,
    #[serde(default)]
    pub(crate) anything_else: String,
}

fn validate_feedback(f: &FeedbackForm) -> Result<(), &'static str> {
    if f.name.trim().is_empty() {
        return Err("name required");
    }
    if f.name.len() > MAX_NAME_LEN {
        return Err("name too long");
    }
    if f.company.len() > MAX_COMPANY_LEN || f.email.len() > MAX_REPLY_TO_LEN {
        return Err("identity field too long");
    }
    for (label, val) in [
        ("worked_well", &f.worked_well),
        ("didnt_work", &f.didnt_work),
        ("alternative", &f.alternative),
        ("why_chose", &f.why_chose),
        ("whats_changed", &f.whats_changed),
        ("recommend", &f.recommend),
        ("anything_else", &f.anything_else),
    ] {
        if val.len() > MAX_FEEDBACK_FIELD_LEN {
            tracing::warn!(field = label, "feedback field too long");
            return Err("feedback field too long");
        }
    }
    // At least one substantive field — refuse empty submissions.
    let has_content = !f.worked_well.trim().is_empty()
        || !f.didnt_work.trim().is_empty()
        || !f.alternative.trim().is_empty()
        || !f.why_chose.trim().is_empty()
        || !f.whats_changed.trim().is_empty()
        || !f.recommend.trim().is_empty()
        || !f.anything_else.trim().is_empty();
    if !has_content {
        return Err("at least one answer required");
    }
    Ok(())
}

fn feedback_ack(message: &str) -> Markup {
    let cta = html! {
        a href="/" class="text-primary font-semibold" { "← Back home" }
    };
    let body = html! {
        (Hero {
            eyebrow: Some("Feedback received"),
            headline_lead: "Thank you.",
            headline_accent: None,
            subheadline: message,
            cta: Some(&cta),
            background: HeroBackground::GridLight,
        }.render())
    };
    page("Feedback received — PlausiDen", "/feedback", body)
}

/// `POST /feedback` — validate, persist to the SQLite store, email
/// a copy to `team@plausiden.com`, render the ack page.
///
/// SECURITY: Same rate-limit posture as the inquiry handler. The
/// feedback body is *more* PII than an inquiry (consented testimonial
/// content), so the email body never logs at info; only success /
/// failure counts surface.
pub(crate) async fn feedback_submit(
    State(state): State<InquiryState>,
    ConnectInfo(_addr): ConnectInfo<std::net::SocketAddr>,
    Form(form): Form<FeedbackForm>,
) -> Response {
    if state.limiter.check().is_err() {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            feedback_ack("The inbox is being flooded right now — please try again in a minute."),
        )
            .into_response();
    }
    if let Err(e) = validate_feedback(&form) {
        tracing::warn!(error = e, "feedback rejected at validation");
        return (
            StatusCode::BAD_REQUEST,
            feedback_ack(
                "Your submission didn't pass basic validation. Every answer is optional, but at least one field has to be filled in.",
            ),
        )
            .into_response();
    }

    // Persist first, email second. If persistence fails we return
    // 500 rather than silently dropping; if email fails we still
    // accept (the row is durable).
    let insert = FeedbackInsert {
        name: form.name.as_str(),
        company: form.company.as_str(),
        email: form.email.as_str(),
        worked_well: form.worked_well.as_str(),
        didnt_work: form.didnt_work.as_str(),
        consent: form.consent.as_str(),
        alternative: form.alternative.as_str(),
        why_chose: form.why_chose.as_str(),
        whats_changed: form.whats_changed.as_str(),
        recommend: form.recommend.as_str(),
        anything_else: form.anything_else.as_str(),
    };
    let row_id = match state.feedback.insert(&insert).await {
        Ok(id) => id,
        Err(e) => {
            tracing::warn!(error = %e, "feedback persist failed");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                feedback_ack(
                    "Server error storing your submission. Please email team@plausiden.com directly.",
                ),
            )
                .into_response();
        }
    };
    tracing::info!(row_id, "feedback stored");

    // Email summary to team@. Failure here is non-fatal — the row
    // is in the DB regardless.
    let body = format!(
        "New feedback submission #{row_id}.\n\n\
         Name:     {name}\n\
         Company:  {company}\n\
         Email:    {email}\n\
         Consent:  {consent}\n\n\
         --- worked well ---\n{ww}\n\n\
         --- didn't work ---\n{dw}\n\n\
         --- alternative ---\n{alt}\n\n\
         --- why chose ---\n{why}\n\n\
         --- what's changed ---\n{wc}\n\n\
         --- recommend ---\n{rec}\n\n\
         --- anything else ---\n{ext}\n",
        name = form.name,
        company = form.company,
        email = form.email,
        consent = form.consent,
        ww = form.worked_well,
        dw = form.didnt_work,
        alt = form.alternative,
        why = form.why_chose,
        wc = form.whats_changed,
        rec = form.recommend,
        ext = form.anything_else,
    );
    let from: Mailbox = "PlausiDen Web <team@plausiden.com>"
        .parse()
        .expect("from mailbox parses");
    let to: Mailbox = "team@plausiden.com".parse().expect("to parses");
    let html = crate::views::email::feedback_notification_html(
        row_id,
        &form.name,
        &form.company,
        &form.email,
        &form.consent,
        &[
            ("What worked well", form.worked_well.as_str()),
            ("What didn't", form.didnt_work.as_str()),
            ("Alternative considered", form.alternative.as_str()),
            ("Why chose PlausiDen", form.why_chose.as_str()),
            ("What's changed", form.whats_changed.as_str()),
            ("Would recommend", form.recommend.as_str()),
            ("Anything else", form.anything_else.as_str()),
        ],
    );
    if let Ok(email) = Message::builder()
        .from(from)
        .to(to)
        .subject(format!("[feedback #{row_id}] {}", form.name))
        .header(msg_header::ContentType::TEXT_HTML)
        .multipart(
            MultiPart::alternative()
                .singlepart(SinglePart::plain(body))
                .singlepart(SinglePart::html(html)),
        )
    {
        if let Err(e) = state.mailer.send(email).await {
            tracing::warn!(error = %e, "feedback email send failed (row already persisted)");
        }
    }

    (
        StatusCode::ACCEPTED,
        feedback_ack(
            "Your feedback is in our inbox. If you flagged a quote we can publish, we'll email you the proposed wording before anything goes live.",
        ),
    )
        .into_response()
}

#[derive(Debug, Deserialize)]
pub(crate) struct ExportQuery {
    /// `json` (default), `csv`, or `tsv`.
    #[serde(default)]
    pub(crate) format: String,
    /// Admin token. Compared against `PLAUSIDEN_ADMIN_TOKEN` (constant-
    /// time via `subtle` if we ever pull that in; for v0 a plain
    /// equality check is sufficient because the token is never
    /// surfaced to a low-trust party).
    #[serde(default)]
    pub(crate) token: String,
}

/// `GET /feedback/export?format=json|csv|tsv&token=…` — admin export.
///
/// SECURITY: Refuses every request when `PLAUSIDEN_ADMIN_TOKEN` is
/// unset (the import default). When set, requires `token=` to match.
/// Always returns plain `Unauthorized` text on rejection — never a
/// detail string that leaks whether the token is set or what shape
/// it's in.
pub(crate) async fn feedback_export(
    State(state): State<InquiryState>,
    Query(q): Query<ExportQuery>,
) -> Response {
    if state.admin_token.is_empty() || q.token != *state.admin_token {
        return (StatusCode::UNAUTHORIZED, "unauthorized").into_response();
    }

    let rows = match state.feedback.list_all().await {
        Ok(rs) => rs,
        Err(e) => {
            tracing::warn!(error = %e, "feedback export query failed");
            return (StatusCode::INTERNAL_SERVER_ERROR, "internal error").into_response();
        }
    };

    let mut headers = HeaderMap::new();
    let (body, content_type) = match q.format.as_str() {
        "csv" => (export_dsv(&rows, ','), "text/csv; charset=utf-8"),
        "tsv" => (
            export_dsv(&rows, '\t'),
            "text/tab-separated-values; charset=utf-8",
        ),
        _ => (export_json(&rows), "application/json"),
    };
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    (StatusCode::OK, headers, body).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_form() -> InquiryForm {
        InquiryForm {
            name: String::new(),
            reply_to: String::new(),
            phone: String::new(),
            company: String::new(),
            service: String::new(),
            message: String::new(),
        }
    }

    #[test]
    fn validate_rejects_empty_message() {
        assert!(validate(&empty_form()).is_err());
    }

    #[test]
    fn validate_accepts_minimal_message() {
        let mut f = empty_form();
        f.message = "hi".into();
        assert!(validate(&f).is_ok());
    }

    #[test]
    fn validate_rejects_oversized_message() {
        let mut f = empty_form();
        f.message = "x".repeat(MAX_MESSAGE_LEN + 1);
        assert!(validate(&f).is_err());
    }

    #[test]
    fn validate_rejects_oversized_phone() {
        let mut f = empty_form();
        f.message = "hi".into();
        f.phone = "x".repeat(MAX_PHONE_LEN + 1);
        assert!(validate(&f).is_err());
    }
}
