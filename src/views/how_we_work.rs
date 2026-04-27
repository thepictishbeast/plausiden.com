//! `/how-we-work` — mid-funnel page describing `PlausiDen`'s engagement
//! model and operating posture. Linked from the footer and from each
//! vertical landing page.

use loom_components::card::FeatureCard;
use loom_components::hero::{Hero, HeroBackground};
use maud::{Markup, html};

use super::layout::page_with_description;

const HWW_DESCRIPTION: &str = "How PlausiDen engages, ships, and hands off. Written-down doctrine, in-writing proposals, scope-limited access, audit-ready documentation. The operating posture behind every deliverable.";

const ICON_DOC: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-6 h-6 text-primary"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>"#;

const ICON_SHIELD: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-6 h-6 text-primary"><path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"></path></svg>"#;

const ICON_AUDIT: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-6 h-6 text-primary"><path d="M9 11l3 3 8-8"/><path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"/></svg>"#;

const ICON_USERS: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-6 h-6 text-primary"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>"#;

/// Render `/how-we-work`.
#[must_use]
pub fn render() -> Markup {
    let body = html! {

        (Hero {
            eyebrow: Some("Operating model"),
            headline_lead: "How we work,",
            headline_accent: Some("in writing."),
            subheadline: "Most consultancies have a tribal sense of how to ship. We wrote ours down. Every engagement runs on the same operating posture: doctrine-driven code, scope-limited access, in-writing proposals, audit-ready documentation.",
            cta: None,
            background: HeroBackground::GridLight,
        }.render())

        section class="py-16 bg-white" {
            div class="container mx-auto px-4 md:px-6 max-w-6xl" {
                div class="text-center max-w-3xl mx-auto mb-12 reveal" {
                    h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-4" { "The four commitments" }
                    p class="text-slate-600 text-lg" { "What clients can hold us to, regardless of engagement size." }
                }
                div class="grid grid-cols-1 md:grid-cols-2 gap-6 reveal reveal-delay-1" {
                    (FeatureCard {
                        icon_svg: ICON_DOC,
                        title: "Written proposals",
                        description: "Specific scope, specific deliverables, specific price — never \"depends on what we find.\" If a project's shape is too uncertain to price, we propose a paid discovery instead, with a fixed-cost cap and a deliverable you can take elsewhere.",
                    }.render())
                    (FeatureCard {
                        icon_svg: ICON_SHIELD,
                        title: "Scope-limited access",
                        description: "We touch only what we've agreed to touch. No roving credentials, no \"we'll just SSH in to fix it,\" no production access we can't justify in writing. When a problem is outside our scope, we say so and refer.",
                    }.render())
                    (FeatureCard {
                        icon_svg: ICON_AUDIT,
                        title: "Audit-ready documentation",
                        description: "Every choice we make is annotated in code. When your malpractice carrier, regulator, or new vendor asks why something is configured the way it is, the answer is in the code, not in our heads.",
                    }.render())
                    (FeatureCard {
                        icon_svg: ICON_USERS,
                        title: "Real handoff",
                        description: "We aim for engagements you can run without us. The deliverable is documentation a competent successor can use to take over. If you eventually hire in-house IT, we hand them a clean baton — not a black box.",
                    }.render())
                }
            }
        }

        section class="py-20 bg-slate-50" {
            div class="container mx-auto px-4 md:px-6 max-w-4xl reveal" {
                h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-6" { "The three-step engagement" }
                p class="text-slate-600 text-lg leading-relaxed mb-8" {
                    "Every engagement, regardless of vertical, runs through the same three steps. The conversation is short; the proposal that follows is specific."
                }
                ol class="space-y-6 text-slate-700" {
                    li class="flex gap-4" {
                        span class="flex-shrink-0 w-8 h-8 rounded-full bg-primary text-white font-bold flex items-center justify-center text-sm" { "1" }
                        div {
                            p class="font-semibold text-slate-900 mb-1" { "Intake conversation (no commitment)" }
                            p class="text-slate-600" { "A 45-minute call about your current setup, current pain, and the obligations that shape your decisions. Mutual NDA before; we leave with enough to write a real proposal." }
                        }
                    }
                    li class="flex gap-4" {
                        span class="flex-shrink-0 w-8 h-8 rounded-full bg-primary text-white font-bold flex items-center justify-center text-sm" { "2" }
                        div {
                            p class="font-semibold text-slate-900 mb-1" { "Written proposal" }
                            p class="text-slate-600" { "Specific scope, specific deliverables, specific price. Includes which of your existing tools stay vs. change, what the 90-day picture looks like, and what success looks like." }
                        }
                    }
                    li class="flex gap-4" {
                        span class="flex-shrink-0 w-8 h-8 rounded-full bg-primary text-white font-bold flex items-center justify-center text-sm" { "3" }
                        div {
                            p class="font-semibold text-slate-900 mb-1" { "Implementation + handoff" }
                            p class="text-slate-600" { "We do the work; you get documentation that lets your next vendor (or in-house IT) run it without us. Optional ongoing retainer for monitoring + incident response." }
                        }
                    }
                }
            }
        }

        section class="py-20 bg-slate-900 text-white" {
            div class="container mx-auto px-4 md:px-6 max-w-4xl reveal" {
                h2 class="font-display text-3xl md:text-4xl font-bold mb-6 leading-tight" { "We will tell you if we're not the right fit." }
                p class="text-slate-400 text-lg leading-relaxed mb-4" {
                    "We don't pretend to be a fit for every problem. If your situation calls for a 50-person managed-service provider with a 24/7 NOC, we'll say so. If you'd be better served by a single senior contractor for the next quarter, we'll say so. The intake conversation is meant to surface that — and we'd rather lose a sale than take an engagement we can't finish well."
                }
                p class="text-slate-400 text-lg leading-relaxed" {
                    "When we are the right fit, you'll know within the first conversation."
                }
            }
        }

        section class="py-20 bg-primary/5" {
            div class="container mx-auto px-4 md:px-6 text-center max-w-3xl reveal" {
                h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-6" { "Want to start a conversation?" }
                a href="/contact" {
                    button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium bg-primary text-primary-foreground border border-primary-border min-h-10 px-8 py-6 rounded-xl text-lg shadow-xl shadow-primary/20 hover:-translate-y-0.5 transition-all" {
                        "Schedule an intake call"
                    }
                }
            }
        }
    };
    page_with_description("How we work — PlausiDen", "/how-we-work", HWW_DESCRIPTION, body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_nonempty() {
        assert!(render().into_string().len() > 4000);
    }

    #[test]
    fn lists_four_commitments() {
        let s = render().into_string();
        for c in &[
            "Written proposals",
            "Scope-limited access",
            "Audit-ready documentation",
            "Real handoff",
        ] {
            assert!(s.contains(c), "missing commitment: {c}");
        }
    }

    #[test]
    fn engagement_steps_numbered() {
        let s = render().into_string();
        assert!(s.contains("Intake conversation"));
        assert!(s.contains("Written proposal"));
        assert!(s.contains("Implementation + handoff"));
    }
}
