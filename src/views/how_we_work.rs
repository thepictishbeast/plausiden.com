//! `/how-we-work` — mid-funnel page describing `PlausiDen`'s engagement
//! model and operating posture. Linked from the footer and from each
//! vertical landing page.

use loom_components::card::FeatureCard;
use loom_components::hero::{Hero, HeroBackground};
use loom_components::{
    Button, ButtonSize, ButtonType, ButtonVariant, Decoration, Heading, HeadingLevel, HeadingTone,
    HeadingVariant, Lede, Section, SectionPadding, SectionTheme, SectionWidth,
};
use maud::{Markup, html};

use super::layout::page_with_description;

const HWW_DESCRIPTION: &str = "How PlausiDen engages, ships, and hands off. Written-down doctrine, in-writing proposals, scope-limited access, audit-ready documentation. The operating posture behind every deliverable.";

// loom-allow: inline-SVG icon string — w-6 h-6 text-primary lives inside the SVG `class=` attribute, not as a Maud-emitted utility chain
const ICON_DOC: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-6 h-6 text-primary"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg>"#; // loom-allow: SVG class attribute, see comment above
const ICON_SHIELD: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-6 h-6 text-primary"><path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"></path></svg>"#; // loom-allow: SVG class attribute
const ICON_AUDIT: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-6 h-6 text-primary"><path d="M9 11l3 3 8-8"/><path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"/></svg>"#; // loom-allow: SVG class attribute
const ICON_USERS: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-6 h-6 text-primary"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>"#; // loom-allow: SVG class attribute

struct Commitment<'a> {
    icon: &'a str,
    title: &'a str,
    description: &'a str,
}

const COMMITMENTS: &[Commitment<'_>] = &[
    Commitment {
        icon: ICON_DOC,
        title: "Written proposals",
        description: "Specific scope, specific deliverables, specific price — never \"depends on what we find.\" If a project's shape is too uncertain to price, we propose a paid discovery instead, with a fixed-cost cap and a deliverable you can take elsewhere.",
    },
    Commitment {
        icon: ICON_SHIELD,
        title: "Scope-limited access",
        description: "We touch only what we've agreed to touch. No roving credentials, no \"we'll just SSH in to fix it,\" no production access we can't justify in writing. When a problem is outside our scope, we say so and refer.",
    },
    Commitment {
        icon: ICON_AUDIT,
        title: "Audit-ready documentation",
        description: "Every choice we make is annotated in code. When your malpractice carrier, regulator, or new vendor asks why something is configured the way it is, the answer is in the code, not in our heads.",
    },
    Commitment {
        icon: ICON_USERS,
        title: "Real handoff",
        description: "We aim for engagements you can run without us. The deliverable is documentation a competent successor can use to take over. If you eventually hire in-house IT, we hand them a clean baton — not a black box.",
    },
];

struct Step<'a> {
    title: &'a str,
    description: &'a str,
}

const STEPS: &[Step<'_>] = &[
    Step {
        title: "Intake conversation (no commitment)",
        description: "A 45-minute call about your current setup, current pain, and the obligations that shape your decisions. Mutual NDA before; we leave with enough to write a real proposal.",
    },
    Step {
        title: "Written proposal",
        description: "Specific scope, specific deliverables, specific price. Includes which of your existing tools stay vs. change, what the 90-day picture looks like, and what success looks like.",
    },
    Step {
        title: "Implementation + handoff",
        description: "We do the work; you get documentation that lets your next vendor (or in-house IT) run it without us. Optional ongoing retainer for monitoring + incident response.",
    },
];

fn step_item(n: usize, step: &Step<'_>) -> Markup {
    let label = format!("{n}");
    html! {
        li class="flex gap-4" { // loom-allow: numbered-step row; pending shared StepRow primitive
            span class="flex-shrink-0 w-8 h-8 rounded-full bg-primary text-white font-bold flex items-center justify-center text-sm" { (label) } // loom-allow: circular step badge; pending StepNumber primitive
            div {
                p class="font-semibold text-slate-900 mb-1" { (step.title) } // loom-allow: row-title pattern
                p class="text-slate-600" { (step.description) } // loom-allow: in-row body
            }
        }
    }
}

/// Render `/how-we-work`.
#[must_use]
pub fn render() -> Markup {
    let commitments_body = html! {
        div class="text-center max-w-3xl mx-auto mb-12 reveal" { // loom-allow: centered intro caption (same shape as solutions template)
            div class="mb-4" {
                (Heading {
                    text: "The four commitments",
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Section,
                    tone: HeadingTone::Ink,
                }.render())
            }
            (Lede {
                text: "What clients can hold us to, regardless of engagement size.",
                tone: HeadingTone::Ink,
            }.render())
        }
        div class="grid grid-cols-1 md:grid-cols-2 gap-6 reveal reveal-delay-1" { // loom-allow: 2-col commitments grid; Loom doesn't ship a Grid primitive
            @for c in COMMITMENTS {
                (FeatureCard {
                    icon_svg: c.icon,
                    title: c.title,
                    description: c.description,
                }.render())
            }
        }
    };
    let commitments_section = html! {
        section class="py-16 bg-white" { // loom-allow: capability band — py-16 cadence, not Loom Section
            div class="container mx-auto px-4 md:px-6 max-w-6xl" { // loom-allow: capability container — max-w-6xl wider than Loom Wide
                (commitments_body)
            }
        }
    };

    let steps_body = html! {
        div class="reveal" {
            div class="mb-6" {
                (Heading {
                    text: "The three-step engagement",
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Section,
                    tone: HeadingTone::Ink,
                }.render())
            }
            div class="mb-8" {
                (Lede {
                    text: "Every engagement, regardless of vertical, runs through the same three steps. The conversation is short; the proposal that follows is specific.",
                    tone: HeadingTone::Ink,
                }.render())
            }
            ol class="space-y-6 text-slate-700" { // loom-allow: numbered-step list — vertical rhythm + base prose colour
                @for (i, step) in STEPS.iter().enumerate() {
                    (step_item(i + 1, step))
                }
            }
        }
    };
    let steps_section = Section {
        body: &steps_body,
        theme: SectionTheme::Muted,
        width: SectionWidth::Wide,
        padding: SectionPadding::Loose,
    }
    .render();

    let dark_body = html! {
        div class="reveal" {
            (Heading {
                text: "We will tell you if we're not the right fit.",
                level: HeadingLevel::H2,
                variant: HeadingVariant::Section,
                tone: HeadingTone::OnDark,
            }.render())
            div class="mt-6 space-y-4" { // loom-allow: spacer + vertical rhythm between dark-band Lede paragraphs
                (Lede {
                    text: "We don't pretend to be a fit for every problem. If your situation calls for a 50-person managed-service provider with a 24/7 NOC, we'll say so. If you'd be better served by a single senior contractor for the next quarter, we'll say so. The intake conversation is meant to surface that — and we'd rather lose a sale than take an engagement we can't finish well.",
                    tone: HeadingTone::OnDark,
                }.render())
                (Lede {
                    text: "When we are the right fit, you'll know within the first conversation.",
                    tone: HeadingTone::OnDark,
                }.render())
            }
        }
    };
    let dark_section = Section {
        body: &dark_body,
        theme: SectionTheme::Dark,
        width: SectionWidth::Wide,
        padding: SectionPadding::Loose,
    }
    .render();

    let cta_button = Button {
        label: "Schedule an intake call",
        variant: ButtonVariant::Primary,
        size: ButtonSize::Lg,
        aria_label: None,
        icon: None,
        decoration: Decoration::SoftShadow,
        button_type: ButtonType::Button,
    }
    .render();
    let cta_body = html! {
        div class="text-center reveal" {
            div class="mb-6" {
                (Heading {
                    text: "Want to start a conversation?",
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Section,
                    tone: HeadingTone::Ink,
                }.render())
            }
            a href="/contact" { (cta_button) }
        }
    };
    let cta_section = Section {
        body: &cta_body,
        theme: SectionTheme::Tinted,
        width: SectionWidth::Article,
        padding: SectionPadding::Loose,
    }
    .render();

    let body = html! {
        (Hero {
            eyebrow: Some("Operating model"),
            headline_lead: "How we work,",
            headline_accent: Some("in writing."),
            subheadline: "Most consultancies have a tribal sense of how to ship. We wrote ours down. Every engagement runs on the same operating posture: doctrine-driven code, scope-limited access, in-writing proposals, audit-ready documentation.",
            cta: None,
            background: HeroBackground::GridLight,
        }.render())
        (commitments_section)
        (steps_section)
        (dark_section)
        (cta_section)
    };

    page_with_description(
        "How we work — PlausiDen",
        "/how-we-work",
        HWW_DESCRIPTION,
        body,
    )
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
