//! Services page. Content ported from the old site during migration.

use maud::{Markup, html};

use super::layout::page;

pub(crate) fn render() -> Markup {
    let body = html! {
        section class="services" {
            h1 { "Services" }
            p class="lede" {
                "Engagements are scoped to a single outcome and a fixed deliverable. \
                 No retainers, no vendor lock."
            }
            ul class="service-list" {
                li {
                    h3 { "Infrastructure audit" }
                    p { "Red-team review of an existing deployment, with prioritised remediation." }
                }
                li {
                    h3 { "Sovereign migration" }
                    p { "Move a workload off a hostile host onto infrastructure you control." }
                }
                li {
                    h3 { "Zero-state web rebuild" }
                    p { "Rebuild a site or app with no tracking, no accounts, no third-party dependencies." }
                }
            }
            p class="note" {
                "For anything outside these, use the encrypted inquiry form."
            }
        }
    };
    page("Services", body)
}
