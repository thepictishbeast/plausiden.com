//! Legal pages — Sovereign Privacy Directive and Sovereign Terms of Service.
//!
//! BUG ASSUMPTION: Both pages currently ship **placeholder bodies** flagged as
//! under legal review. Legal copy on a live site has real liability weight;
//! auto-generated filler (`PlausiDen` is committed to protecting your
//! privacy…) can be worse than no page. These are honest placeholders until
//! the real policies are drafted. Tests below guard that the routes render
//! 200 and surface the "under review" disclaimer.

use loom_components::{
    Badge, BadgeShape, BadgeSize, BadgeTone, Heading, HeadingLevel, HeadingTone, HeadingVariant, Lede,
};
use maud::{Markup, html};

use super::layout::page;

fn legal_shell(title: &str, current: &str, heading: &str, subheading: &str) -> Markup {
    let body = html! {
        section class="relative pt-32 pb-16 md:pt-48 md:pb-20 overflow-hidden bg-slate-50" { // loom-allow: hero band — pt-32/48 cadence below Loom Section padding scale
            div class="absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:24px_24px]" {} // loom-allow: SVG grid fleck — same pattern as blog hero
            div class="container relative mx-auto px-4 md:px-6 z-10" { // loom-allow: hero container with z-10 fleck stacking
                div class="max-w-3xl" { // loom-allow: hero content max-w-3xl
                    div class="mb-6" { (Badge { label: "Legal", tone: BadgeTone::Primary, size: BadgeSize::Md, shape: BadgeShape::default() }.render()) }
                    div class="mb-4" {
                        (Heading {
                            text: heading,
                            level: HeadingLevel::H1,
                            variant: HeadingVariant::Display,
                            tone: HeadingTone::Ink,
                        }.render())
                    }
                    (Lede { text: subheading, tone: HeadingTone::Ink }.render())
                }
            }
        }

        section class="py-16 bg-white" { // loom-allow: legal-body band — py-16 cadence
            div class="container mx-auto px-4 md:px-6" { // loom-allow: container scope
                div class="max-w-3xl mx-auto" { // loom-allow: legal-body container max-w-3xl
                    div class="rounded-xl border border-amber-200 bg-amber-50 p-6 mb-10" { // loom-allow: amber placeholder-disclaimer panel — semantic warn colour, no Loom WarnPanel primitive
                        p class="text-sm text-amber-900 font-medium mb-2" { "Placeholder page — under legal review" } // loom-allow: panel title — bolded amber
                        p class="text-sm text-amber-800 leading-relaxed" { // loom-allow: panel body prose with inline link
                            "This document is being drafted with counsel. Until it's published, the operative "
                            "policy is: we collect nothing from site visitors (no cookies, no analytics, no tracking), "
                            "and we engage with clients under written agreements executed per engagement. For the "
                            "specific terms of an engagement or a data-handling question, "
                            a href="/contact" class="underline hover:text-amber-700" { "contact us" } // loom-allow: in-amber-band link — not a generic TextLink target
                            "."
                        }
                    }
                    p class="text-slate-600 text-base leading-relaxed" { // loom-allow: legal-body prose paragraph with inline mailto
                        "Last updated: placeholder. A complete " (title) " will be published here once "
                        "reviewed. If you need the current terms for a specific engagement, write to "
                        a href="mailto:team@plausiden.com" { "team@plausiden.com" }
                        "."
                    }
                }
            }
        }
    };
    page(title, current, body)
}

/// Sovereign Privacy Directive placeholder.
///
/// BUG ASSUMPTION: Route is reachable from the footer; must return 200.
#[must_use]
pub fn privacy() -> Markup {
    legal_shell(
        "Sovereign Privacy Directive — PlausiDen",
        "/privacy-directive",
        "Sovereign Privacy Directive",
        "How we handle information when you interact with this site or engage PlausiDen for services.",
    )
}

/// Sovereign Terms of Service placeholder.
///
/// BUG ASSUMPTION: Route is reachable from the footer; must return 200.
#[must_use]
pub fn terms() -> Markup {
    legal_shell(
        "Sovereign Terms of Service — PlausiDen",
        "/terms-of-service",
        "Sovereign Terms of Service",
        "The terms under which PlausiDen LLC provides services and under which this site may be used.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn privacy_has_placeholder_disclaimer() {
        let s = privacy().into_string();
        assert!(s.contains("Sovereign Privacy Directive"));
        assert!(s.contains("Placeholder page"));
    }

    #[test]
    fn terms_has_placeholder_disclaimer() {
        let s = terms().into_string();
        assert!(s.contains("Sovereign Terms of Service"));
        assert!(s.contains("Placeholder page"));
    }

    #[test]
    fn both_link_to_contact() {
        assert!(privacy().into_string().contains("href=\"/contact\""));
        assert!(terms().into_string().contains("href=\"/contact\""));
    }
}
