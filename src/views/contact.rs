//! Contact page. DOM is captured verbatim from the production React site
//! (rendered via Playwright 2026-04-25) and baked in with `include_str!`.
//!
//! Note: The form POST handler is not yet wired — submitting returns 405.
//! Form delivery (rate limit, lettre SMTP, age client-side encryption) is
//! tracked in `plausiden_rust.md` memory and blocked on SMTP-relay decision.

use maud::{Markup, PreEscaped, html};

use super::layout::page;

const CONTACT_HTML: &str = include_str!("../pages/contact.html");

/// Render the contact body verbatim from the captured production DOM.
///
/// BUG ASSUMPTION: Form submission currently 405s — the POST route isn't
/// wired yet. Re-capture when production iterates.
#[must_use]
pub fn render() -> Markup {
    let body = html! {
        (PreEscaped(CONTACT_HTML))
    };
    page("Contact — PlausiDen", "/contact", body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contact_renders_nonempty() {
        let s = render().into_string();
        assert!(
            s.len() > 4000,
            "contact page unexpectedly short: {} bytes",
            s.len()
        );
    }

    #[test]
    fn contact_has_no_unsplash_origin() {
        assert!(!render().into_string().contains("images.unsplash.com"));
    }

    #[test]
    fn contact_has_no_secure_drop_label() {
        assert!(!render().into_string().contains(">Secure Drop<"));
    }
}
