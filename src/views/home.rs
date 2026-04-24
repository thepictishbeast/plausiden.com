//! Homepage view. Placeholder content — will be replaced with ported marketing copy.

use maud::{Markup, html};

use super::layout::page;

pub(crate) fn render() -> Markup {
    let body = html! {
        section class="hero" {
            h1 { "Sovereign infrastructure. Nothing to surveil." }
            p class="lede" {
                "PlausiDen builds systems that leak nothing they don't have to. \
                 Private by architecture — not by policy."
            }
            p class="cta-row" {
                a href="/services" class="btn btn-primary" { "What we do" }
                a href="/contact" class="btn btn-ghost" { "Encrypted Inquiry" }
            }
        }

        section class="principles" {
            h2 { "Principles" }
            ul {
                li {
                    strong { "Zero state." }
                    " No sessions, no cookies, no database of visitors. \
                     We cannot be compelled to reveal what we never collect."
                }
                li {
                    strong { "Opaque by default." }
                    " Contact submissions are encrypted in your browser before they leave it. \
                     Our server never sees plaintext."
                }
                li {
                    strong { "Auditable end-to-end." }
                    " Reproducible builds, signed releases, transparent provenance."
                }
                li {
                    strong { "Defense by absence." }
                    " The most secure component is the one that doesn't exist."
                }
            }
        }
    };
    page("Home", body)
}
