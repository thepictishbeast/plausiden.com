//! Services page. Copy ported from the current React SPA (2026-04-24).

use maud::{Markup, html};

use super::layout::page;

/// Render the services body.
///
/// BUG ASSUMPTION: Copy is ported from the production React SPA. Tests assert
/// on service-name presence and section structure rather than exact prose.
#[must_use]
pub fn render() -> Markup {
    let body = html! {
        section class="services" {
            div class="inner" {
                h1 { "Services" }
                p class="lede" {
                    "Six practice areas. Each engagement is scoped to a concrete outcome, "
                    "with a fixed deliverable and no vendor lock."
                }

                ul class="service-list" {
                    li {
                        h3 { "IT Operations" }
                        p {
                            "Complete infrastructure management designed to keep your business running smoothly. "
                            "Monitoring, maintenance, and support so you can focus on growth."
                        }
                        ul class="sub" {
                            li { "Cloud Management" }
                            li { "Help Desk Support" }
                            li { "Hardware Solutions" }
                        }
                    }
                    li {
                        h3 { "Cyber Security" }
                        p {
                            "Defense-in-depth strategies to protect your critical assets. "
                            "From compliance audits to real-time threat detection, we secure your digital perimeter."
                        }
                        ul class="sub" {
                            li { "Endpoint Protection" }
                            li { "Security Training" }
                            li { "Security Protocol Review" }
                        }
                    }
                    li {
                        h3 { "Software Development" }
                        p {
                            "Custom software solutions tailored to your unique workflows. "
                            "Scalable, secure, maintainable — and auditable by you."
                        }
                    }
                    li {
                        h3 { "Network Solutions" }
                        p {
                            "Robust network architecture and operational excellence."
                        }
                        ul class="sub" {
                            li { "Network Architecture" }
                            li { "Network Design" }
                            li { "Connectivity Audit" }
                        }
                    }
                    li {
                        h3 { "AI Strategy Consulting" }
                        p {
                            "Pragmatic guidance for where AI actually earns its keep in your stack — "
                            "and where it doesn't. Includes NLP solutions and integration planning."
                        }
                    }
                    li {
                        h3 { "Compliance & Audits" }
                        p {
                            "Evidence-based attestation for the regulatory regimes your clients expect. "
                            "Engagements are delivered with reproducible artifacts you can hand to an auditor."
                        }
                    }
                }

                p class="note" {
                    "Not sure what you need? "
                    a href="/contact" { "Send an encrypted inquiry" }
                    " and describe the problem in plain language."
                }
            }
        }
    };
    page("Services — PlausiDen", "/services", body)
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
    fn services_lists_six_practice_areas() {
        let s = render().into_string();
        for name in [
            "IT Operations",
            "Cyber Security",
            "Software Development",
            "Network Solutions",
            "AI Strategy",
            "Compliance",
        ] {
            assert!(s.contains(name), "services page missing: {name}");
        }
    }

    #[test]
    fn services_mentions_key_sub_capabilities() {
        let s = render().into_string();
        for cap in [
            "Cloud Management",
            "Help Desk",
            "Endpoint Protection",
            "Security Training",
            "Network Architecture",
            "Connectivity Audit",
        ] {
            assert!(
                s.contains(cap),
                "services page missing sub-capability: {cap}"
            );
        }
    }
}
