//! Encrypted Inquiry page. Renamed from the old site's "Secure Drop" to avoid
//! the `SecureDrop` (whistleblower platform) name collision.
//!
//! v1 renders a plain HTML form as a no-JS fallback. v1.1 will layer
//! client-side `age` encryption via WASM so the server only ever sees
//! ciphertext.

use maud::{Markup, PreEscaped, html};

use super::layout::page;

/// Render the Encrypted Inquiry page body.
///
/// BUG ASSUMPTION: The form POSTs to `/contact`; the POST handler is not yet
/// implemented. Submitting today will return 405 Method Not Allowed from the
/// `GET`-only route, by design — the form is rendered for shape review only.
///
/// SECURITY: `autocomplete="off"` discourages browsers from saving draft
/// inputs to autofill. The message field has a 5000-char cap (minor
/// resource-exhaustion mitigation). v1.1 will add WASM-side age encryption
/// so the server never sees plaintext.
#[must_use]
pub fn render() -> Markup {
    let body = html! {
        section class="inquiry" {
            div class="inner" {
                h1 { "Encrypted Inquiry" }
                p class="lede" {
                    "Send a message. No account required. No cookies set. "
                    "No identifying metadata recorded."
                }

                // v1: plain HTML form, works with JavaScript disabled (Tor Browser
                // Safest mode is first-class). v1.1 progressively enhances with
                // client-side `age` encryption before POST.
                form method="post" action="/contact" autocomplete="off" {
                    label for="name" { "Name (optional)" }
                    input id="name" name="name" type="text" maxlength="100";

                    label for="reply_to" { "How should we reach you?" }
                    input id="reply_to" name="reply_to" type="text" maxlength="200";

                    label for="message" { "Message" }
                    textarea id="message" name="message" rows="8" maxlength="5000" required {}

                    button type="submit" { "Send" }
                }

                p class="note" {
                    "Prefer Tor? This site is also reachable as a v3 onion service "
                    "(configured in v1.1)."
                }
            }
        }

        // Placeholder for the v1.1 WASM encryption hook. Kept as a comment so
        // a grep for "age" / "encrypt" in the code surfaces this site.
        (PreEscaped("<!-- v1.1: <script src=\"/static/inquiry.js\"></script> -->"))
    };
    page("Encrypted Inquiry — PlausiDen", "/contact", body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contact_shows_encrypted_inquiry_heading() {
        let s = render().into_string();
        assert!(s.contains("<h1>Encrypted Inquiry</h1>"));
    }

    #[test]
    fn contact_has_form_posting_to_self() {
        let s = render().into_string();
        assert!(s.contains("method=\"post\""));
        assert!(s.contains("action=\"/contact\""));
    }

    #[test]
    fn contact_form_disables_autocomplete() {
        let s = render().into_string();
        assert!(s.contains("autocomplete=\"off\""));
    }

    #[test]
    fn contact_does_not_set_cookies_at_render() {
        let s = render().into_string();
        assert!(!s.to_ascii_lowercase().contains("set-cookie"));
    }
}
