//! Services page. Content will be ported from the current site during the
//! pre-cutover polish pass.

use maud::{Markup, html};

use super::layout::page;

/// Render the services body.
///
/// BUG ASSUMPTION: Copy here is placeholder and tests assert only on
/// structural invariants (the `<h1>`, the bulleted list), not specific copy
/// strings that may change.
#[must_use]
pub fn render() -> Markup {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn services_has_top_heading() {
        let s = render().into_string();
        assert!(s.contains("<h1>Services</h1>"));
    }

    #[test]
    fn services_has_three_list_items() {
        let s = render().into_string();
        // Sanity bound: at least 3 <li> entries on the page.
        assert!(s.matches("<li>").count() >= 3);
    }
}
