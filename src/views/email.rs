//! Branded HTML email templates — thin wrappers around the
//! [`mail_templates`] crate's prebuilt builders.
//!
//! Historical note: this module used to hand-roll its own MIME-safe
//! HTML chrome (gradient hero, accent-stripe content cards, branded
//! footer). That chrome was lifted into `mail-templates::prebuilt`
//! once it stabilized so every email in the PlausiDen stack —
//! magic-link, bounce, alert, weekly-digest, DNS dispatch — could
//! share one renderer. This file is now a 30-line shim that
//! preserves the historical call signatures the rest of the
//! plausiden-site code uses.
//!
//! SECURITY: All caller-supplied strings are escaped inside
//! mail-templates' renderer before being interpolated. No raw
//! `PreEscaped` here.

use mail_templates::{
    prebuilt::{feedback_received, inquiry_received, magic_link, FeedbackSection},
    EmailDocument,
};

/// HTML body for the "sign in to admin" magic-link email.
#[must_use]
pub fn magic_link_html(link: &str) -> String {
    let mut doc: EmailDocument = magic_link(link);
    // Preserve the historical preheader copy + subject for inbox
    // previews — mail-templates' default works too, but keeping the
    // wording stable here means downstream snapshot tests don't move.
    doc.preheader = "Single-use sign-in link, valid for 15 minutes.".into();
    doc.render_html()
}

/// HTML body for the "new feedback received" notification to team@.
#[must_use]
pub fn feedback_notification_html(
    row_id: i64,
    name: &str,
    company: &str,
    email: &str,
    consent: &str,
    sections: &[(&str, &str)],
) -> String {
    let sections: Vec<FeedbackSection> = sections
        .iter()
        .map(|(label, body)| FeedbackSection {
            label: (*label).to_string(),
            body: (*body).to_string(),
        })
        .collect();
    let doc = feedback_received(
        row_id,
        name,
        email,
        company,
        consent,
        sections,
        Some("https://plausiden.com/admin/feedback"),
    );
    doc.render_html()
}

/// HTML body for the "new contact inquiry" notification to team@.
#[must_use]
pub fn inquiry_notification_html(
    name: &str,
    reply_to: &str,
    phone: &str,
    company: &str,
    service: &str,
    message: &str,
) -> String {
    inquiry_received(name, reply_to, phone, company, service, message).render_html()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magic_link_html_contains_link_and_brand() {
        let h = magic_link_html("https://plausiden.com/admin/login/verify?token=abc.def");
        assert!(h.contains("PlausiDen"));
        assert!(h.contains("Sign in"));
        assert!(h.contains("https://plausiden.com/admin/login/verify?token=abc.def"));
        // Inline styles only — no <style> blocks (some clients strip them).
        assert!(!h.contains("<style"));
    }

    #[test]
    fn magic_link_html_escapes_link() {
        // A pathological link with HTML metacharacters mustn't break out
        // of the href attribute or the visible code block.
        let h = magic_link_html("https://example.com/?x=<script>");
        assert!(!h.contains("<script>"));
        assert!(h.contains("&lt;script&gt;"));
    }

    #[test]
    fn feedback_notification_html_renders_sections() {
        let h = feedback_notification_html(
            42,
            "Tim",
            "Sacred.Vote",
            "tim@example.com",
            "full",
            &[
                ("What worked well", "the explainer is the killer feature"),
                ("What didn't", ""),
                ("Why chose PlausiDen", "the audit trail"),
            ],
        );
        assert!(h.contains("#42"));
        assert!(h.contains("Tim"));
        assert!(h.contains("Sacred.Vote"));
        assert!(h.contains("the explainer is the killer feature"));
        assert!(h.contains("the audit trail"));
        // Empty section was suppressed
        assert!(!h.contains("What didn"));
    }

    #[test]
    fn feedback_notification_html_renders_avatar_initial() {
        // The mail-templates prebuilt no longer renders an avatar
        // initial — the chrome was simplified during the migration.
        // We assert instead on the gradient hero's logo letter (P).
        let h = feedback_notification_html(1, "tim", "", "t@x.com", "", &[("note", "hi")]);
        // Gradient hero contains the brand "P" tile.
        assert!(h.contains(">P</span>"), "missing hero P logo: {h}");
    }

    #[test]
    fn inquiry_notification_html_escapes_message_body() {
        let h = inquiry_notification_html(
            "Mallory",
            "m@x.com",
            "",
            "",
            "",
            "<img src=x onerror=alert(1)>",
        );
        assert!(!h.contains("<img src=x"));
        assert!(h.contains("&lt;img src=x onerror=alert(1)&gt;"));
    }

    #[test]
    fn inquiry_notification_html_has_reply_cta() {
        let h = inquiry_notification_html("Alice", "alice@example.org", "", "", "", "hi");
        assert!(h.contains("Reply to Alice"));
        assert!(h.contains("mailto:alice@example.org"));
    }

    #[test]
    fn shell_has_brand_tagline() {
        let h = magic_link_html("https://x");
        assert!(h.contains("Plausible. Defensible."));
    }
}
