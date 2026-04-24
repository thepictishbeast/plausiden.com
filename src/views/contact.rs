//! Encrypted Inquiry page. Renamed from the old site's "Secure Drop" to avoid the
//! SecureDrop (whistleblower platform) name collision.
//!
//! v1 renders a plain HTML form as a no-JS fallback. v1.1 will layer client-side age
//! encryption via WASM so the server only ever sees ciphertext.

use maud::{Markup, PreEscaped, html};

use super::layout::page;

pub(crate) fn render() -> Markup {
    let body = html! {
        section class="inquiry" {
            h1 { "Encrypted Inquiry" }
            p {
                "Send a message. No account required. No cookies set. \
                 No identifying metadata recorded."
            }

            // Plain HTML form. Works with JavaScript disabled (Tor Browser safest mode).
            // v1.1 will progressively enhance this to encrypt client-side before POST.
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

        // Tiny, no-trackers-required stub for future WASM encryption hook.
        // Loaded locally from /static, satisfies strict CSP (script-src 'self').
        (PreEscaped("<!-- v1.1: <script src=\"/static/inquiry.js\"></script> -->"))
    };
    page("Encrypted Inquiry", body)
}
