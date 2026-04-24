//! Homepage view. Copy ported from the current React SPA (2026-04-24) with
//! visual system matched to the current site's slate/blue/corporate aesthetic.

use maud::{Markup, html};

use super::layout::page;

/// Render the homepage body inside the shared [`page`] chrome.
///
/// BUG ASSUMPTION: Copy is ported from the production React SPA at
/// [www.plausiden.com](https://www.plausiden.com). Tests below assert on
/// structural identity (section headings, service count) rather than specific
/// copy lines, so routine editorial changes do not break the suite.
#[must_use]
#[allow(clippy::too_many_lines)] // Maud DSL inflates line count; the function is still one logical unit.
pub fn render() -> Markup {
    let body = html! {
        section class="hero" {
            div class="inner" {
                p class="eyebrow" { "Professional IT Solutions" }
                h1 { "Comprehensive IT for the " span class="accent" { "sovereign enterprise" } "." }
                p class="lede" {
                    "Founded on the principle that technology should be an enabler, not a barrier — "
                    "PlausiDen provides IT solutions that are both broad in scope and precise in execution."
                }
                p class="cta-row" {
                    a href="/services" class="btn btn-primary" { "Explore Services" }
                    a href="/contact" class="btn btn-ghost" { "Encrypted Inquiry" }
                }
            }
        }

        section class="services-preview" {
            div class="inner" {
                h2 { "Everything Your Business Needs" }
                p class="lede" { "Customized strategies, not cookie-cutter solutions." }
                ul class="service-grid" {
                    li {
                        h3 { "IT Operations" }
                        p {
                            "Complete infrastructure management designed to keep your business running smoothly. "
                            "We handle monitoring, maintenance, and support so you can focus on growth."
                        }
                    }
                    li {
                        h3 { "Cyber Security" }
                        p {
                            "Defense-in-depth strategies to protect your critical assets. "
                            "From compliance audits to real-time threat detection, we secure your digital perimeter."
                        }
                    }
                    li {
                        h3 { "Software Development" }
                        p {
                            "Custom software solutions tailored to your unique workflows. "
                            "We build scalable, secure, and maintainable applications."
                        }
                    }
                    li {
                        h3 { "Network Solutions" }
                        p {
                            "Robust network architecture and operational excellence — "
                            "designed, deployed, and kept running without drama."
                        }
                    }
                    li {
                        h3 { "AI Strategy" }
                        p {
                            "Pragmatic guidance for where AI actually earns its keep in your stack, "
                            "and where it doesn't."
                        }
                    }
                    li {
                        h3 { "Compliance & Audits" }
                        p {
                            "Evidence-based attestation for the regulatory regimes your clients expect."
                        }
                    }
                }
                p { a href="/services" class="link-forward" { "See every service →" } }
            }
        }

        section class="principles" {
            div class="inner" {
                h2 { "Integrity in every interaction" }
                p class="lede" {
                    "PlausiDen builds systems that leak nothing they don't have to. "
                    "Private by architecture — not by policy."
                }
                ul {
                    li {
                        strong { "Enterprise-grade security standards." }
                        " Every engagement treats your data like it's ours too — because the "
                        "posture that protects your clients is the same one that protects us."
                    }
                    li {
                        strong { "Sovereign by default." }
                        " Our tools and deployments are self-hosted, auditable, and yours to inspect. "
                        "No vendor hostage, no surprise telemetry."
                    }
                    li {
                        strong { "Defense by absence." }
                        " The most secure component is the one that doesn't exist. We remove surface "
                        "area before we add layers."
                    }
                }
            }
        }

        section class="cta-final" {
            div class="inner" {
                h2 { "Ready to elevate your IT strategy?" }
                p {
                    "Contact us for a free consultation and discover how we can optimize your operations."
                }
                p class="cta-row" style="justify-content: center;" {
                    a href="/contact" class="btn btn-primary" { "Start an Encrypted Inquiry" }
                }
            }
        }
    };
    page("Home", body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_has_hero_tagline() {
        let s = render().into_string();
        assert!(s.contains("Professional IT Solutions"));
        assert!(s.contains("Comprehensive IT"));
    }

    #[test]
    fn home_enumerates_six_services_in_preview() {
        let s = render().into_string();
        assert!(s.matches("<h3>").count() >= 6);
        for name in [
            "IT Operations",
            "Cyber Security",
            "Software Development",
            "Network Solutions",
            "AI Strategy",
            "Compliance",
        ] {
            assert!(s.contains(name), "services grid missing: {name}");
        }
    }

    #[test]
    fn home_links_to_services_and_contact() {
        let s = render().into_string();
        assert!(s.contains("href=\"/services\""));
        assert!(s.contains("href=\"/contact\""));
    }

    #[test]
    fn home_contains_principles_section() {
        let s = render().into_string();
        assert!(s.contains("Integrity in every interaction"));
        assert!(s.contains("Sovereign by default"));
        assert!(s.contains("Defense by absence"));
    }
}
