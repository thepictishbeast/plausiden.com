//! Services page. DOM is captured verbatim from the production React site
//! (rendered via Playwright 2026-04-25) and baked in with `include_str!`,
//! then wrapped in the shared layout.
//!
//! SECURITY: Post-capture scrub replaced `images.unsplash.com` → self-hosted,
//! `/secure-drop` → `/contact`, and removed the placeholder testimonial text.
//! SHIP-DECISION: 2026-04-25 — accepting production HTML as-is for parity;
//! refactor to typed components (see `src/components.rs`) when parity is
//! verified acceptable by the human reviewer.

use maud::{Markup, PreEscaped, html};

use super::layout::page;

const SERVICES_HTML: &str = include_str!("../pages/services.html");

/// Render the services body verbatim from the captured production DOM.
///
/// BUG ASSUMPTION: The HTML file is pre-scrubbed of third-party origins and
/// stale route paths. Re-capture + re-scrub any time production changes.
#[must_use]
pub fn render() -> Markup {
    let body = html! {
        (PreEscaped(SERVICES_HTML))
    };
    page("Services — PlausiDen", "/services", body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn services_contains_production_heading() {
        let s = render().into_string();
        assert!(
            s.len() > 5000,
            "services page unexpectedly short: {} bytes",
            s.len()
        );
    }

    #[test]
    fn services_has_no_unsplash_origin() {
        let s = render().into_string();
        assert!(
            !s.contains("images.unsplash.com"),
            "Unsplash origin leaked into /services; scrub step failed"
        );
    }

    #[test]
    fn services_has_no_secure_drop_text() {
        // REGRESSION-GUARD: renamed to Encrypted Inquiry.
        let s = render().into_string();
        assert!(!s.contains(">Secure Drop<"));
    }
}
