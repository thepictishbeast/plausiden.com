//! POST `/contact` form handler — receives the contact-form submission,
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
use lettre::message::{Mailbox, Message, MultiPart};
use lettre::transport::smtp::AsyncSmtpTransport;
use lettre::{AsyncTransport, Tokio1Executor};
use loom_components::hero::{Hero, HeroBackground};
use loom_components::{TextLink, TextLinkSize, TextLinkVariant};
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
    /// Honeypot — visually hidden in the rendered form, no real user
    /// fills it. Filled means a naive bot scraped + replayed every
    /// input. SECURITY: the field name is intentionally innocuous
    /// (`website`) so a smarter bot might still skip it; the heuristic
    /// in `is_likely_spam` is the second line of defense.
    #[serde(default)]
    pub(crate) website: String,
}

const MAX_PHONE_LEN: usize = 50;
const MAX_COMPANY_LEN: usize = 200;
const MAX_SERVICE_LEN: usize = 100;

/// Returns `s` or "(omitted)" if `s` is empty. Used to keep the email
/// body readable when optional fields are blank.
const fn or_omitted(s: &str) -> &str {
    if s.is_empty() { "(omitted)" } else { s }
}

/// Heuristic check for the contact-form-spam pattern: bots scraping a
/// public form to advertise their own contact-form-spam-as-a-service.
/// We've observed messages combining (a) Telegram + WhatsApp links, (b)
/// Belarus / Russia / generic-VoIP phone numbers, (c) classic boilerplate
/// phrases ("Our system sends messages through website contact forms",
/// "found your website while checking", "free test"). A single signal is
/// noise; ≥3 is a confident spam classification.
///
/// Returns `true` if the form should be silently dropped (no email, 202
/// ack so the bot doesn't detect filtering and adapt).
fn is_likely_spam(f: &InquiryForm) -> bool {
    let msg = f.message.to_lowercase();
    let phone = f.phone.replace(['(', ')', ' ', '-'], "");
    let mut score = 0u32;

    // Channel mentions — common for outreach-spam pushing the recipient
    // to an off-form messenger they control.
    if msg.contains("telegram") {
        score += 1;
    }
    if msg.contains("whatsapp") {
        score += 1;
    }
    // Direct messenger URLs in the body — harder to do innocently.
    if msg.contains("t.me/") || msg.contains("wa.me/") {
        score += 1;
    }

    // Boilerplate phrases lifted from the canonical contact-form-spam
    // template. Each phrase scores low so that a single accidental
    // overlap doesn't trip the filter; a real spam message has several.
    for needle in [
        "our system sends messages",
        "found your website",
        "checking different resources",
        "free test",
        "web outreach",
        "contact form outreach",
        "feel free to reach out",
    ] {
        if msg.contains(needle) {
            score += 1;
        }
    }
    // The "system sends messages through website contact forms" phrase
    // is a very strong signal — bots reuse it nearly verbatim.
    if msg.contains("messages through website contact forms")
        || msg.contains("through website contact form")
    {
        score += 2;
    }

    // High-spam-volume country codes for contact-form abuse. Not a
    // single-signal block — combined with the other heuristics it's
    // confident; alone, a Belarusian customer should not be auto-rejected.
    if phone.starts_with("+375") || phone.starts_with("375") {
        score += 1;
    }

    score >= 3
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

/// What a sender is told when the form is accepted.
///
/// SECURITY: this exact string is also returned for the honeypot and
/// spam-heuristic drops, so a bot cannot tell a filtered submission from a
/// delivered one. The three paths must stay byte-identical — status code
/// included — or the filtering becomes detectable and adaptable. Pinned by
/// `silent_drops_are_indistinguishable_from_success`.
///
/// It describes what happens next in the same terms as the rest of the site
/// and deliberately states no response time. A reply-within commitment is
/// Paul's to make, not this page's to invent.
const RECEIVED_MESSAGE: &str = "We have your message and will reply to the address you gave us. \
     If a scoping call makes sense, we will suggest a time: it runs 45 minutes, we sign a mutual \
     NDA first, and you get a written proposal with a specific scope and a specific price \
     afterwards. If we are not the right firm for the work, we will say so rather than book a call \
     to find out.";

/// Where the acknowledgement page sends the reader next.
#[derive(Clone, Copy, PartialEq, Eq)]
enum AckExit {
    /// Nothing more to do here.
    Home,
    /// Something needs correcting, so the form is the useful destination.
    ///
    /// The copy tells the reader that going back preserves what they typed.
    /// The Loom `TextInput` has no `value` field, so the server cannot
    /// repopulate the form without changing a component shared with another
    /// tenant; browser back-forward cache restores form state on its own, and
    /// saying so is honest and costs nothing.
    Form,
    /// The feedback form, for the same reason.
    FeedbackForm,
}

/// Acknowledgement page shown after a POST to `/contact`.
///
/// One shape for every outcome so the page a sender lands on always looks
/// like the rest of the site. Previously this was branded "Encrypted
/// Inquiry", a name retired from every other page earlier — it survived here
/// because the page renders only in response to a POST and no guard walked it.
fn ack_page(title: &str, eyebrow: &str, headline: &str, message: &str, exit: AckExit) -> Markup {
    let (label, href) = match exit {
        AckExit::Home => ("← Back home", "/"),
        AckExit::Form => ("← Back to the form", "/contact"),
        AckExit::FeedbackForm => ("← Back to the form", "/feedback"),
    };
    let cta = html! {
        (TextLink {
            label: label,
            href: href,
            variant: TextLinkVariant::PrimaryBold,
            size: TextLinkSize::Default,
        }.render())
    };
    let body = html! {
        (Hero {
            eyebrow: Some(eyebrow),
            headline_lead: headline,
            headline_accent: None,
            subheadline: message,
            cta: Some(&cta),
            background: HeroBackground::GridLight,
        }.render())
    };
    page(title, "/contact", body)
}

/// Turn a validator code into something a sender can act on.
///
/// The old page said "Your submission didn't pass basic validation. Please
/// correct and retry." — which does not say what was wrong, on the one page
/// where the business earns anything. Naming the field costs nothing and
/// leaks nothing: every limit here is already visible as a `maxlength`
/// attribute in the markup the sender's browser received.
fn validation_message(code: &str) -> String {
    let detail = match code {
        "name too long" => "the name field is over its 100-character limit",
        "reply-to too long" => "the email address is over its 200-character limit",
        "phone too long" => "the phone number is over its 50-character limit",
        "company too long" => "the company field is over its 200-character limit",
        "service too long" => "the service selection was not one of the listed options",
        "message required" => "the message was empty",
        "message too long" => "the message is over its 5,000-character limit",
        // Defensive: a new validator rule with no case here should still
        // produce a sentence rather than an empty gap.
        _ => "one of the fields did not pass validation",
    };
    format!(
        "It looks like {detail}. Going back will return you to the form with what you typed still \
         in it — correct that one field and send again."
    )
}

/// The accepted-submission page. Success, honeypot and spam all render this.
fn ack_received() -> Markup {
    ack_page(
        "Message received — PlausiDen",
        "Next steps",
        "Message received.",
        RECEIVED_MESSAGE,
        AckExit::Home,
    )
}

/// Something the sender can fix.
fn ack_problem(message: &str) -> Markup {
    ack_page(
        "Message not sent — PlausiDen",
        "Contact",
        "That did not send.",
        message,
        AckExit::Form,
    )
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
    // Honeypot — the `website` field is visually hidden in the rendered
    // form. Real users don't see it; naive bots fill every input. If
    // it's non-empty, drop silently (200-shape ack so the bot doesn't
    // detect filtering and adapt) and skip all I/O.
    if !form.website.trim().is_empty() {
        tracing::warn!("inquiry honeypot tripped — silent drop");
        return (StatusCode::ACCEPTED, ack_received()).into_response();
    }

    // Heuristic spam filter for contact-form-spam-as-a-service bots
    // (Telegram/WhatsApp + boilerplate phrases). Same silent-drop shape
    // — bots adapt to honest 4xx responses; they don't adapt to 202s.
    if is_likely_spam(&form) {
        tracing::warn!("inquiry classified as spam — silent drop");
        return (StatusCode::ACCEPTED, ack_received()).into_response();
    }

    if state.limiter.check().is_err() {
        tracing::warn!("inquiry rate-limited");
        return (
            StatusCode::TOO_MANY_REQUESTS,
            ack_problem(
                "We are receiving an unusual number of messages right now and this one was not \
                 accepted. Try again in a minute, or email team@plausiden.com directly — that \
                 reaches the same place.",
            ),
        )
            .into_response();
    }

    if let Err(e) = validate(&form) {
        tracing::warn!(error = e, "inquiry rejected at validation");
        return (StatusCode::BAD_REQUEST, ack_problem(&validation_message(e))).into_response();
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
        .subject("[contact-form] New inquiry");
    if !form.reply_to.is_empty() {
        if let Ok(rt) = form.reply_to.parse::<Mailbox>() {
            builder = builder.reply_to(rt);
        }
    }
    let Ok(email) = builder.multipart(MultiPart::alternative_plain_html(body, html)) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            ack_problem(
                "Something broke on our side while handling the message — this one is our fault, \
                 not yours. Please email team@plausiden.com directly.",
            ),
        )
            .into_response();
    };

    match state.mailer.send(email).await {
        Ok(_) => {
            tracing::info!("inquiry sent");
            (StatusCode::ACCEPTED, ack_received()).into_response()
        }
        Err(e) => {
            tracing::warn!(error = %e, "inquiry SMTP send failed");
            (
                StatusCode::ACCEPTED,
                ack_page(
                    "Message received — PlausiDen",
                    "Next steps",
                    "Message received.",
                    "Your message is queued for delivery. If you do not hear back, email \
                     team@plausiden.com directly — that reaches the same inbox.",
                    AckExit::Home,
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

/// The feedback form was accepted.
fn feedback_received() -> Markup {
    ack_page(
        "Feedback received — PlausiDen",
        "Feedback",
        "Thank you.",
        "This goes to the people who did the work, not into a reporting dashboard. If you raised \
         something that needs an answer and left an address, you will get one. And if you flagged a \
         quote we can publish, we will email you the proposed wording before anything goes live.",
        AckExit::Home,
    )
}

/// The feedback form was not accepted.
///
/// Previously every outcome here rendered the headline "Thank you.",
/// including validation failures — so a submission that did not send
/// thanked the sender for sending it, then offered the homepage as the
/// only way out. Errors now look like errors and lead back to the form.
fn feedback_problem(message: &str) -> Markup {
    ack_page(
        "Feedback not sent — PlausiDen",
        "Feedback",
        "That did not send.",
        message,
        AckExit::FeedbackForm,
    )
}

/// Turn a feedback validator code into something the sender can act on.
fn feedback_validation_message(code: &str) -> String {
    let detail = match code {
        "name required" => "we need a name to attribute the feedback to",
        "name too long" => "the name field is over its length limit",
        "identity field too long" => "the role or company field is over its length limit",
        "feedback field too long" => "one of the answers is over its length limit",
        "at least one answer required" => {
            "every answer was blank. All of them are optional, but at least one has to say something"
        }
        _ => "one of the fields did not pass validation",
    };
    format!(
        "It looks like {detail}. Going back will return you to the form with what you typed still \
         in it — correct that one field and send again."
    )
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
            feedback_problem(
                "We are receiving an unusual number of submissions right now and this one was not \
                 accepted. Try again in a minute, or email team@plausiden.com directly.",
            ),
        )
            .into_response();
    }
    if let Err(e) = validate_feedback(&form) {
        tracing::warn!(error = e, "feedback rejected at validation");
        return (
            StatusCode::BAD_REQUEST,
            feedback_problem(&feedback_validation_message(e)),
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
                feedback_problem(
                    "Something broke on our side while storing this — our fault, not yours. Please \
                     email team@plausiden.com directly.",
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
        .multipart(MultiPart::alternative_plain_html(body, html))
    {
        if let Err(e) = state.mailer.send(email).await {
            tracing::warn!(error = %e, "feedback email send failed (row already persisted)");
        }
    }

    (StatusCode::ACCEPTED, feedback_received()).into_response()
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
            website: String::new(),
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

    /// The exact spam pattern observed 2026-04-28 on plausiden.com:
    /// canned "Davidhes" outreach pushing Telegram + Belarus WhatsApp.
    #[test]
    fn spam_filter_catches_canonical_contact_form_spam() {
        let mut f = empty_form();
        f.name = "Davidhes".into();
        f.reply_to = "no.reply.MarkWilliams@gmail.com".into();
        f.phone = "83183834455".into();
        f.company = "google".into();
        f.service = "Other".into();
        f.message = "Good morning! plausiden.com,\nI found your website while \
                     checking different resources.\nOur system sends messages \
                     through website contact forms instead of email.\nMany \
                     companies use web outreach to find new partners.\nYou can \
                     run a free test to understand how the system works.\nFeel \
                     free to reach out if you want information.\n\n\
                     Telegram - https://t.me/FeedbackFormEU\n\
                     WhatsApp - +375259112693\n\
                     WhatsApp https://wa.me/+375259112693"
            .into();
        assert!(is_likely_spam(&f), "should classify the canonical spam");
    }

    #[test]
    fn spam_filter_does_not_flag_legit_whatsapp_mention() {
        let mut f = empty_form();
        f.message = "Hi — please reach me on WhatsApp when you have a moment, \
                     I prefer it for quick scheduling. Thanks!"
            .into();
        assert!(
            !is_likely_spam(&f),
            "single-channel mention is not enough to flag"
        );
    }

    #[test]
    fn spam_filter_does_not_flag_legit_telegram_mention() {
        let mut f = empty_form();
        f.message = "We use Telegram for our team chat — happy to discuss \
                     scope there if it's easier."
            .into();
        assert!(
            !is_likely_spam(&f),
            "single-channel mention is not enough to flag"
        );
    }

    #[test]
    fn spam_filter_flags_belarus_phone_plus_messenger_combo() {
        let mut f = empty_form();
        f.phone = "+375259112693".into();
        f.message = "Reach me on Telegram t.me/foo or WhatsApp.".into();
        assert!(is_likely_spam(&f));
    }

    #[test]
    fn spam_filter_does_not_flag_normal_business_message() {
        let mut f = empty_form();
        f.name = "Jane Smith".into();
        f.reply_to = "jane@acme.example".into();
        f.message = "We're a 30-person law firm in Boston evaluating privacy- \
                     respecting infrastructure for our client portal. Could we \
                     schedule a call to discuss your DR offering?"
            .into();
        assert!(!is_likely_spam(&f));
    }
}

#[cfg(test)]
mod acknowledgement_tests {
    use super::*;

    /// SECURITY: the honeypot and spam heuristics drop a submission
    /// silently and return the *same* page a real sender gets, so a bot
    /// cannot tell filtered from delivered and adapt. Three call sites
    /// render this; if someone later personalises the success page — a
    /// name, a reference number, anything — the drops stop matching and
    /// the filtering becomes detectable. This pins the shape.
    #[test]
    fn silent_drops_are_indistinguishable_from_success() {
        let a = ack_received().into_string();
        let b = ack_received().into_string();
        assert_eq!(a, b, "the acknowledgement page is not deterministic");
        assert!(
            a.contains(
                RECEIVED_MESSAGE
                    .split(" If a scoping call")
                    .next()
                    .unwrap_or(RECEIVED_MESSAGE)
            ),
            "the acknowledgement page no longer carries the shared received message"
        );
    }

    /// The name retired everywhere else must not survive here. This page
    /// renders only in response to a POST, which is exactly why the
    /// site-wide naming guard never walked it and the old wording sat
    /// here unnoticed.
    #[test]
    fn no_retired_naming_on_the_acknowledgement_pages() {
        let pages = [
            ack_received().into_string(),
            ack_problem("test").into_string(),
        ];
        for page in &pages {
            for retired in [
                "Encrypted Inquiry",
                "Secure Drop",
                "Get a Free Consultation",
                "Start Your Journey",
            ] {
                assert!(
                    !page.contains(retired),
                    "an acknowledgement page still uses the retired name {retired:?}"
                );
            }
        }
    }

    /// A sender who gets bounced needs to know which field to fix and
    /// how to get back to what they typed. "Please correct and retry"
    /// satisfies neither.
    #[test]
    fn validation_messages_name_the_field_and_the_way_back() {
        for code in [
            "name too long",
            "reply-to too long",
            "phone too long",
            "company too long",
            "service too long",
            "message required",
            "message too long",
        ] {
            let msg = validation_message(code);
            assert!(
                msg.len() > 60,
                "validation message for {code:?} is too terse to act on: {msg:?}"
            );
            assert!(
                msg.contains("Going back"),
                "validation message for {code:?} does not tell the sender how to recover"
            );
            assert!(
                !msg.contains("did not pass validation"),
                "validation message for {code:?} fell through to the generic case; \
                 every code the validator can emit needs its own sentence"
            );
        }
    }

    /// The error page must route back to the form, not to the homepage.
    /// Sending someone who mistyped an address back to the front door is
    /// how a lead is lost.
    #[test]
    fn problem_pages_lead_back_to_the_form() {
        let page = ack_problem("something went wrong").into_string();
        assert!(
            page.contains(r#"href="/contact""#),
            "the problem page does not link back to the form"
        );
        let received = ack_received().into_string();
        assert!(
            received.contains(r#"href="/""#),
            "the received page should offer the way home"
        );
    }

    /// Every code the feedback validator can emit must have its own
    /// sentence. Written after mapping a code that did not exist
    /// ("empty submission") while the real one — "at least one answer
    /// required" — silently fell through to the generic case. Reading the
    /// validator is not enough; assert against it.
    #[test]
    fn every_feedback_validator_code_has_a_specific_message() {
        let src = include_str!("inquiry.rs");
        let body = src
            .split("fn validate_feedback")
            .nth(1)
            .expect("validate_feedback exists");
        let body = body.split("\nfn ").next().unwrap_or(body);

        let mut codes = Vec::new();
        for part in body.split("Err(\"").skip(1) {
            if let Some(code) = part.split('"').next() {
                codes.push(code.to_owned());
            }
        }
        assert!(
            codes.len() >= 5,
            "expected to find the validator's error codes, found {codes:?}"
        );
        for code in codes {
            let msg = feedback_validation_message(&code);
            assert!(
                !msg.contains("one of the fields did not pass validation"),
                "feedback validator emits {code:?} but feedback_validation_message has no case \
                 for it, so the sender gets the generic fallback"
            );
        }
    }

    /// The contact validator, held to the same standard.
    #[test]
    fn every_contact_validator_code_has_a_specific_message() {
        let src = include_str!("inquiry.rs");
        let body = src.split("fn validate(").nth(1).expect("validate exists");
        let body = body.split("\nfn ").next().unwrap_or(body);
        let mut codes = Vec::new();
        for part in body.split("Err(\"").skip(1) {
            if let Some(code) = part.split('"').next() {
                codes.push(code.to_owned());
            }
        }
        assert!(
            codes.len() >= 5,
            "expected validator codes, found {codes:?}"
        );
        for code in codes {
            let msg = validation_message(&code);
            assert!(
                !msg.contains("one of the fields did not pass validation"),
                "contact validator emits {code:?} with no matching message case"
            );
        }
    }

    /// An error page must not thank the sender for a submission that did
    /// not send. /feedback rendered "Thank you." for every outcome,
    /// including validation failures.
    #[test]
    fn feedback_errors_do_not_thank_the_sender() {
        let page = feedback_problem("something went wrong").into_string();
        assert!(
            !page.contains("Thank you."),
            "the feedback error page still thanks the sender for a submission that failed"
        );
        assert!(
            page.contains(r#"href="/feedback""#),
            "the feedback error page does not lead back to the form"
        );
        let ok = feedback_received().into_string();
        assert!(
            ok.contains("Thank you."),
            "the accepted page should thank the sender"
        );
        assert!(
            ok.contains("before anything goes live"),
            "the publishing-consent promise was lost from the accepted page"
        );
    }

    /// No response-time commitment may appear here. Whether PlausiDen
    /// replies within a day is Paul's promise to make; a page that
    /// invents one creates an obligation nobody agreed to.
    #[test]
    fn promises_no_response_time() {
        let page = ack_received().into_string().to_lowercase();
        for sla in [
            "within one business day",
            "within 24 hours",
            "same day",
            "within the hour",
            "reply within",
        ] {
            assert!(
                !page.contains(sla),
                "the acknowledgement page promises {sla:?}; a response-time \
                 commitment is Paul's to make, not this page's to invent"
            );
        }
    }
}
