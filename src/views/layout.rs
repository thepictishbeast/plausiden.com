//! Shared page chrome. Every page is wrapped in this function so the head,
//! nav, and footer stay consistent.

use maud::{DOCTYPE, Markup, html};

/// Render a page with the site-wide chrome. `title` appears in the tab,
/// `body` is the per-page main content.
///
/// BUG ASSUMPTION: `title` is displayed raw in the `<title>` tag; Maud escapes
/// it, so the caller can pass any string. The suffix ` — PlausiDen` is always
/// appended — tests depend on this to disambiguate page identity.
///
/// SECURITY: The page declares no external origins in `<link>` or `<script>`
/// tags. Any future addition of a third-party URL here must be reviewed
/// against the `data-leak` and `supersociety` audits — third-party hosts
/// reintroduce a surveillance channel.
#[must_use]
#[allow(clippy::needless_pass_by_value)] // Markup is PreEscaped<String>; consuming is idiomatic for a composition helper.
pub fn page(title: &str, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="color-scheme" content="light";
                meta name="robots" content="index, follow";
                title { (title) " — PlausiDen" }
                link rel="icon" type="image/svg+xml" href="/static/favicon.svg";
                link rel="icon" type="image/png" sizes="96x96" href="/static/favicon-96x96.png";
                link rel="apple-touch-icon" sizes="180x180" href="/static/apple-touch-icon.png";
                link rel="manifest" href="/static/site.webmanifest";
                link rel="stylesheet" href="/static/style.css";
            }
            body {
                header class="site-header" {
                    div class="inner" {
                        a href="/" class="brand" { "PlausiDen" }
                        nav {
                            a href="/" { "Home" }
                            a href="/services" { "Services" }
                            a href="/contact" { "Encrypted Inquiry" }
                        }
                    }
                }
                main { (body) }
                footer class="site-footer" {
                    div class="inner" {
                        p {
                            "© PlausiDen. No cookies, no tracking, no logs. "
                            a href="/contact" { "Encrypted Inquiry" }
                            "."
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_emits_doctype_and_lang() {
        let s = page("Test", html! { p { "x" } }).into_string();
        assert!(s.starts_with("<!DOCTYPE html>"));
        assert!(s.contains("<html lang=\"en\">"));
    }

    #[test]
    fn page_sets_title_suffix() {
        let s = page("About", html! {}).into_string();
        assert!(s.contains("<title>About — PlausiDen</title>"));
    }

    #[test]
    fn page_has_no_external_origin() {
        // SECURITY: third-party URLs would defeat the CSP default-src 'self'
        // posture. Any https:// in a src/href here would need an explicit
        // SHIP-DECISION: override.
        let s = page("X", html! {}).into_string();
        assert!(
            !s.contains("https://"),
            "external origin leaked into layout"
        );
        assert!(!s.contains("http://"), "plaintext URL leaked into layout");
    }

    #[test]
    fn page_nav_mentions_encrypted_inquiry_not_secure_drop() {
        // REGRESSION-GUARD: the renamed call-to-action must not silently
        // revert to the old "Secure Drop" wording.
        let s = page("X", html! {}).into_string();
        assert!(s.contains("Encrypted Inquiry"));
        assert!(!s.contains("Secure Drop"));
    }
}
