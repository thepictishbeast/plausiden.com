//! `/case-studies` index — sanitized engagement summaries.
//!
//! Each case study is a short structured block: industry, scope,
//! what the client cared about, what we shipped, what the outcome
//! was. Names and identifying details are stripped at authoring
//! time; the page never serves a client name without their explicit
//! written sign-off.

use loom_components::{Badge, BadgeSize, BadgeTone};
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
    let body = html! {
        section class="relative pt-32 pb-16 md:pt-44 md:pb-24 bg-slate-50 overflow-hidden" {
            div class="container relative mx-auto px-4 md:px-6 z-10 max-w-4xl" {
                div class="mb-6" { (Badge { label: "Selected work", tone: BadgeTone::Primary, size: BadgeSize::Md }.render()) }
                h1 class="font-display text-4xl md:text-5xl lg:text-6xl font-bold text-slate-900 leading-[1.1] mb-6" {
                    "Case studies"
                }
                p class="text-lg md:text-xl text-slate-600 max-w-2xl leading-relaxed" {
                    "Sanitized engagement summaries from our active practice. Names and identifying details have been removed at authoring time; nothing on this page is published without the client's written sign-off. If a study reads like your situation, the contact form is the right next step."
                }
            }
        }

        @for (i, c) in CASE_STUDIES.iter().enumerate() {
            section class=(if i % 2 == 0 { "py-16 bg-white" } else { "py-16 bg-slate-50" }) {
                div class="container mx-auto px-4 md:px-6 max-w-3xl" {
                    span class="inline-block px-3 py-1 rounded-full bg-primary/10 text-primary font-semibold text-xs mb-4 border border-primary/20" {
                        (c.industry)
                    }
                    h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mb-6" {
                        (c.scope)
                    }
                    div class="space-y-6" {
                        div {
                            h3 class="font-display text-lg font-semibold text-slate-900 mb-2" {
                                "What mattered"
                            }
                            p class="text-slate-600 leading-relaxed" { (c.what_mattered) }
                        }
                        div {
                            h3 class="font-display text-lg font-semibold text-slate-900 mb-2" {
                                "What we shipped"
                            }
                            p class="text-slate-600 leading-relaxed" { (c.what_we_did) }
                        }
                        div {
                            h3 class="font-display text-lg font-semibold text-slate-900 mb-2" {
                                "Outcome"
                            }
                            p class="text-slate-600 leading-relaxed" { (c.outcome) }
                        }
                    }
                }
            }
        }

        section class="py-20 bg-primary/5" {
            div class="container mx-auto px-4 md:px-6 text-center max-w-2xl" {
                h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-6" {
                    "Recognize your situation in any of these?"
                }
                p class="text-slate-600 text-lg mb-8" {
                    "The first conversation is free, the NDA is mutual, and we'll tell you if we're not the right fit before either of us has invested an hour."
                }
                a href="/contact" {
                    button type="button" class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium bg-primary text-primary-foreground border border-primary-border min-h-10 px-8 py-6 rounded-xl text-lg shadow-xl shadow-primary/20 hover:-translate-y-0.5 transition-all" {
                        "Start the conversation"
                    }
                }
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
