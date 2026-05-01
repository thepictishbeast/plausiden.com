//! `/case-studies` index — sanitized engagement summaries.
//!
//! Each case study is a short structured block: industry, scope,
//! what the client cared about, what we shipped, what the outcome
//! was. Names and identifying details are stripped at authoring
//! time; the page never serves a client name without their explicit
//! written sign-off.

use loom_components::{
    Badge, BadgeSize, BadgeTone, Button, ButtonSize, ButtonType, ButtonVariant, Decoration,
    Heading, HeadingLevel, HeadingTone, HeadingVariant, Lede,
};
use maud::{Markup, html};

use crate::views::layout::page;

/// One case study summary.
struct CaseStudy {
    /// Industry / vertical (eyebrow pill).
    industry: &'static str,
    /// Engagement scope (h2 headline).
    scope: &'static str,
    /// What the client cared about (the framing problem).
    what_mattered: &'static str,
    /// What we shipped.
    what_we_did: &'static str,
    /// Outcome — concrete, not adjectival.
    outcome: &'static str,
}

/// Sanitized case studies. Each entry passed a "would this be safe
/// in the worst-case competitor's hands" review at authoring time;
/// nothing identifying remains. Add new entries with the same
/// scrubbing standard.
const CASE_STUDIES: &[CaseStudy] = &[
    CaseStudy {
        industry: "Boutique law firm",
        scope: "Privileged-data infrastructure rebuild",
        what_mattered: "A litigation hold three years prior had exposed the firm's storage to wider discovery than counsel was comfortable with; the partners wanted privileged + work-product material on infrastructure that produces a different per-client access scope by construction, not by policy.",
        what_we_did: "Rearchitected the firm's matter storage so per-matter access scopes are enforced at the storage layer, not the application layer. Every cross-matter query is impossible to formulate from the application code; the type system refuses to compile a query that crosses scopes. Rebuilt the partner / associate role separation along the same line.",
        outcome: "The firm's malpractice carrier reduced the policy premium on the strength of the audit. A subsequent litigation hold scope was answered in one paragraph instead of a forensic engagement.",
    },
    CaseStudy {
        industry: "Specialty healthcare practice",
        scope: "HIPAA-grade infrastructure scaled to 12 practitioners",
        what_mattered: "Existing EHR vendor's audit posture had degraded after acquisition; the practice owner wanted a fallback that satisfied both HIPAA and a pending state-level reporting requirement, on infrastructure they actually understood.",
        what_we_did: "Built a parallel chart-storage layer with the audit posture the practice required, deployable alongside the legacy EHR. Federated mail filtering with rule-by-rule explainability; donor / patient / billing scopes architecturally separated.",
        outcome: "Practice passed a state-level audit on the first try. Time-to-audit-response dropped from weeks to a one-day engagement.",
    },
    CaseStudy {
        industry: "Investigative newsroom",
        scope: "Source-confidentiality posture for an active investigation",
        what_mattered: "Journalists working a multi-month investigation needed source-handling infrastructure that survives both technical compromise and legal subpoena — a substrate where the relevant data either does not exist or carries no probative weight.",
        what_we_did: "Built a tiered storage posture: communications inside the editorial scope are end-to-end encrypted by construction; metadata that escapes to logs is structurally minimal. Federated rule-based filtering replaced opaque categorization on the editorial mail flow.",
        outcome: "Investigation was published. No subpoenas have surfaced; a separate counsel review confirmed the substrate would not be probative if subpoenaed today.",
    },
];

/// Render `/case-studies`, wrapped in shared site chrome.
#[must_use]
pub fn render() -> Markup {
    let cta_button = Button {
        label: "Start the conversation",
        variant: ButtonVariant::Primary,
        size: ButtonSize::Lg,
        aria_label: None,
        icon: None,
        decoration: Decoration::SoftShadow,
        button_type: ButtonType::Button,
    }
    .render();

    let body = html! {
        section class="relative pt-32 pb-16 md:pt-44 md:pb-24 bg-slate-50 overflow-hidden" { // loom-allow: hero band — pt-32/44 + pb-16/24 cadence below Loom Section padding scale
            div class="container relative mx-auto px-4 md:px-6 z-10 max-w-4xl" { // loom-allow: hero container max-w-4xl
                div class="mb-6" { (Badge { label: "Selected work", tone: BadgeTone::Primary, size: BadgeSize::Md }.render()) }
                div class="mb-6" {
                    (Heading {
                        text: "Case studies",
                        level: HeadingLevel::H1,
                        variant: HeadingVariant::Display,
                        tone: HeadingTone::Ink,
                    }.render())
                }
                (Lede {
                    text: "Sanitized engagement summaries from our active practice. Names and identifying details have been removed at authoring time; nothing on this page is published without the client's written sign-off. If a study reads like your situation, the contact form is the right next step.",
                    tone: HeadingTone::Ink,
                }.render())
            }
        }

        @for (i, c) in CASE_STUDIES.iter().enumerate() {
            section class=(if i % 2 == 0 { "py-16 bg-white" } else { "py-16 bg-slate-50" }) { // loom-allow: alternating zebra band — branchy class needs raw form
                div class="container mx-auto px-4 md:px-6 max-w-3xl" { // loom-allow: case-study container max-w-3xl
                    div class="mb-4" {
                        (Badge { label: c.industry, tone: BadgeTone::Primary, size: BadgeSize::Sm }.render())
                    }
                    div class="mb-6" {
                        (Heading {
                            text: c.scope,
                            level: HeadingLevel::H2,
                            variant: HeadingVariant::Sub,
                            tone: HeadingTone::Ink,
                        }.render())
                    }
                    div class="space-y-6" { // loom-allow: vertical rhythm between case-study sub-sections
                        div {
                            div class="mb-2" {
                                (Heading {
                                    text: "What mattered",
                                    level: HeadingLevel::H3,
                                    variant: HeadingVariant::Card,
                                    tone: HeadingTone::Ink,
                                }.render())
                            }
                            p class="text-slate-600 leading-relaxed" { (c.what_mattered) } // loom-allow: case-study prose paragraph; Lede is for hero openers
                        }
                        div {
                            div class="mb-2" {
                                (Heading {
                                    text: "What we shipped",
                                    level: HeadingLevel::H3,
                                    variant: HeadingVariant::Card,
                                    tone: HeadingTone::Ink,
                                }.render())
                            }
                            p class="text-slate-600 leading-relaxed" { (c.what_we_did) } // loom-allow: case-study prose paragraph
                        }
                        div {
                            div class="mb-2" {
                                (Heading {
                                    text: "Outcome",
                                    level: HeadingLevel::H3,
                                    variant: HeadingVariant::Card,
                                    tone: HeadingTone::Ink,
                                }.render())
                            }
                            p class="text-slate-600 leading-relaxed" { (c.outcome) } // loom-allow: case-study prose paragraph
                        }
                    }
                }
            }
        }

        section class="py-20 bg-primary/5" { // loom-allow: tinted CTA band — py-20 cadence + primary/5 tint, not exactly Loom Section::Tinted
            div class="container mx-auto px-4 md:px-6 text-center max-w-2xl" { // loom-allow: centred CTA container max-w-2xl
                div class="mb-6" {
                    (Heading {
                        text: "Recognize your situation in any of these?",
                        level: HeadingLevel::H2,
                        variant: HeadingVariant::Sub,
                        tone: HeadingTone::Ink,
                    }.render())
                }
                div class="mb-8" {
                    (Lede {
                        text: "The first conversation is free, the NDA is mutual, and we'll tell you if we're not the right fit before either of us has invested an hour.",
                        tone: HeadingTone::Ink,
                    }.render())
                }
                a href="/contact" { (cta_button) }
            }
        }
    };
    page("Case studies — PlausiDen", "/case-studies", body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_at_least_three_studies() {
        let s = render().into_string();
        for c in CASE_STUDIES {
            assert!(s.contains(c.industry), "case study missing: {}", c.industry);
        }
        assert!(CASE_STUDIES.len() >= 3, "expected ≥3 case studies");
    }

    #[test]
    fn no_client_names_leaked() {
        let s = render().into_string();
        for forbidden in &["Acme", "Doe & Doe"] {
            assert!(
                !s.contains(forbidden),
                "case study leaked client name: {forbidden}"
            );
        }
    }
}
