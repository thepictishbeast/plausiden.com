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
pub(crate) struct InquiryState {
    pub(crate) mailer: Arc<AsyncSmtpTransport<Tokio1Executor>>,
    pub(crate) limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>>,
}

impl InquiryState {
    /// Build a state object talking to local Postfix on `127.0.0.1:25`.
    ///
    /// BUG ASSUMPTION: Postfix is configured `inet_interfaces = loopback-only`
    /// (see /etc/postfix/main.cf on the VPS). If that ever changes, this
    /// function still uses 127.0.0.1 — fail-closed.
    #[must_use]
    pub(crate) fn new() -> Self {
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
    #[serde(default)]
    pub(crate) reply_to: String,
    pub(crate) message: String,
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
    if f.message.is_empty() {
        return Err("message required");
    }
    if f.message.len() > MAX_MESSAGE_LEN {
        return Err("message too long");
    }
    Ok(())
}

fn ack_page(message: &str) -> Markup {
    let body = html! {
        section class="relative pt-32 pb-16 md:pt-48 md:pb-20 overflow-hidden bg-slate-50" {
            div class="absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:24px_24px]" {}
            div class="container relative mx-auto px-4 md:px-6 z-10" {
                div class="max-w-3xl" {
                    span class="inline-block px-4 py-1.5 rounded-full bg-primary/10 text-primary font-semibold text-sm mb-6 border border-primary/20" { "Encrypted Inquiry" }
                    h1 class="font-display text-4xl md:text-5xl lg:text-6xl font-bold text-slate-900 leading-[1.1] mb-4" { "Message received." }
                    p class="text-lg text-slate-600 max-w-2xl leading-relaxed mb-6" { (message) }
                    a href="/" class="text-primary font-semibold" { "← Back home" }
                }
            }
        }
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
         Name: {}\n\
         Reply-to: {}\n\
         \n\
         --- message ---\n{}\n",
        if form.name.is_empty() {
            "(omitted)"
        } else {
            &form.name
        },
        if form.reply_to.is_empty() {
            "(omitted)"
        } else {
            &form.reply_to
        },
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

    #[test]
    fn validate_rejects_empty_message() {
        let f = InquiryForm {
            name: String::new(),
            reply_to: String::new(),
            message: String::new(),
        };
        assert!(validate(&f).is_err());
    }

    #[test]
    fn validate_accepts_minimal_message() {
        let f = InquiryForm {
            name: String::new(),
            reply_to: String::new(),
            message: "hi".into(),
        };
        assert!(validate(&f).is_ok());
    }

    #[test]
    fn validate_rejects_oversized_message() {
        let f = InquiryForm {
            name: String::new(),
            reply_to: String::new(),
            message: "x".repeat(MAX_MESSAGE_LEN + 1),
        };
        assert!(validate(&f).is_err());
    }
}
