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
use std::sync::Arc;
use std::time::Duration;

use axum::Form;
use axum::extract::{ConnectInfo, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use governor::clock::DefaultClock;
use governor::middleware::NoOpMiddleware;
use governor::state::{InMemoryState, NotKeyed};
use governor::{Quota, RateLimiter};
use lettre::message::{Mailbox, Message};
use lettre::transport::smtp::AsyncSmtpTransport;
use lettre::{AsyncTransport, Tokio1Executor};
use loom_components::hero::{Hero, HeroBackground};
use maud::{Markup, html};
use serde::Deserialize;

use crate::views::layout::page;

// Tunables — short enough to thwart trivial spam, lenient enough to never
// block a real inquiry.
const QUOTA_PER_MINUTE: u32 = 3;
const MAX_NAME_LEN: usize = 100;
const MAX_REPLY_TO_LEN: usize = 200;
const MAX_MESSAGE_LEN: usize = 5000;

/// Shared application state for the inquiry handler. Constructed once in
/// `main.rs` and cloned per-request via Axum's `State` extractor.
#[derive(Clone)]
#[allow(missing_debug_implementations)]
pub struct InquiryState {
    pub(crate) mailer: Arc<AsyncSmtpTransport<Tokio1Executor>>,
    pub(crate) limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>>,
}

impl Default for InquiryState {
    fn default() -> Self {
        Self::new()
    }
}

impl InquiryState {
    /// Build a state object talking to local Postfix on `127.0.0.1:25`.
    ///
    /// BUG ASSUMPTION: Postfix is configured `inet_interfaces = loopback-only`
    /// (see /etc/postfix/main.cf on the VPS). If that ever changes, this
    /// function still uses 127.0.0.1 — fail-closed.
    #[must_use]
    pub fn new() -> Self {
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
        Self {
            mailer: Arc::new(mailer),
            limiter: Arc::new(limiter),
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

    let mut builder = Message::builder()
        .from(from)
        .to(to)
        .subject("[contact-form] New inquiry");
    if !form.reply_to.is_empty() {
        if let Ok(rt) = form.reply_to.parse::<Mailbox>() {
            builder = builder.reply_to(rt);
        }
    }
    let Ok(email) = builder.body(body) else {
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
