//! About page. DOM is captured verbatim from the production React site
//! (rendered via Playwright 2026-04-25) and baked in with `include_str!`.
//!
//! SECURITY: Post-capture scrub replaced `images.unsplash.com` → self-hosted,
//! `/secure-drop` → `/contact`, and removed the placeholder testimonial.

use maud::{Markup, PreEscaped, html};

use super::layout::page;

const ABOUT_HTML: &str = include_str!("../pages/about.html");

/// Render the about body verbatim from the captured production DOM.
///
/// BUG ASSUMPTION: Production /about currently ships a minimal body. Re-capture
/// when production iterates.
#[must_use]
pub fn render() -> Markup {
    let body = html! {
        (PreEscaped(ABOUT_HTML))
    };
    page("About — PlausiDen", "/about", body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn about_renders_nonempty() {
        let s = render().into_string();
        assert!(
            s.len() > 2000,
            "about page unexpectedly short: {} bytes",
            s.len()
        );
    }

    #[test]
    fn about_has_no_unsplash_origin() {
        assert!(!render().into_string().contains("images.unsplash.com"));
    }
}
