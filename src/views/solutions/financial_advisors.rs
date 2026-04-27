//! `/solutions/financial-advisors` — vertical landing page for small
//! RIAs and wealth-management practices.

use loom_components::card::FeatureCard;
use loom_components::hero::{Hero, HeroBackground};
use loom_icons as icons;
use maud::{Markup, html};

use crate::views::layout::page_with_description;

const FA_DESCRIPTION: &str = "IT infrastructure for small RIAs and wealth-management practices. SEC custody-rule-aware, SOC 2-friendly, BCP-prepared. We design the technical posture custodians and clients are starting to ask about — before they ask.";

/// Render `/solutions/financial-advisors`.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn render() -> Markup {
    let cta = html! {
        a href="/contact" {
            button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring bg-primary text-primary-foreground border border-primary-border min-h-10 text-lg px-8 py-6 rounded-xl shadow-lg shadow-primary/25 hover:shadow-xl hover:-translate-y-0.5 transition-all" {
                "Schedule a custodian-readiness review"
            }
        }
        a href="/services" {
            button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium border border-slate-200 min-h-10 text-lg px-8 py-6 rounded-xl bg-white/50 backdrop-blur-sm hover:bg-white" {
                "See our services"
            }
        }
    };
    let svg_lock = icons::LOCK.render();
    let svg_users = icons::USERS.render();
    let svg_file = icons::FILE_TEXT.render();
    let svg_shield = icons::SHIELD.render();
    let svg_audit = icons::CLIPBOARD_CHECK.render();
    let body = html! {

        (Hero {
            eyebrow: Some("For RIAs + wealth management"),
            headline_lead: "Custodians are asking. ",
            headline_accent: Some("Be ready before they do."),
            subheadline: "Custodians and clients increasingly want technical evidence: SOC 2 attestation, written information security plans, vendor management documentation, BCP rehearsal logs. We design the posture that produces those answers — sized to a small advisory practice, not a wirehouse.",
            cta: Some(&cta),
            background: HeroBackground::GridLight,
        }.render())

        section class="py-20 bg-white" {
            div class="container mx-auto px-4 md:px-6 max-w-4xl reveal" {
                h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-6" {
                    "What \"good IT for an advisory practice\" means"
                }
                p class="text-slate-600 text-lg leading-relaxed mb-4" {
                    "Most small advisory practices outgrow their IT before they outgrow their compliance. The Microsoft 365 tenant works until a custodian's vendor questionnaire arrives, until a client demands evidence of how their data is protected, until the WISP that was \"on the list\" needs to be a real document with real controls."
                }
                p class="text-slate-600 text-lg leading-relaxed" {
                    "We design IT for the moment those questions stop being theoretical. The posture is sized to a one-to-twenty-advisor practice — no enterprise theater, no shelfware. You get documentation that survives a custodian review, an SEC examination preparation, and a malpractice carrier renewal."
                }
            }
        }

        section class="py-16 bg-slate-50" {
            div class="container mx-auto px-4 md:px-6 max-w-6xl" {
                div class="text-center max-w-3xl mx-auto mb-12 reveal" {
                    h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-4" { "What we cover" }
                    p class="text-slate-600 text-lg" {
                        "Capability areas where small advisory practices most often need help."
                    }
                }
                div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 reveal reveal-delay-1" {
                    (FeatureCard {
                        icon_svg: &svg_lock,
                        title: "Client-data isolation by advisor",
                        description: "Per-advisor access scopes that mirror how books actually run. Departing advisor takes their book, not the firm's. New advisor onboards with a clean access surface, not the previous person's leftover Drive shares.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_audit,
                        title: "SOC 2-aligned controls + WISP",
                        description: "Written information security plan that reflects what the systems actually do, not boilerplate. Control documentation organized for SOC 2 auditor review, custodian vendor questionnaires, and SEC examination preparation.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_file,
                        title: "Books + records retention discipline",
                        description: "Email + document retention that satisfies Rule 204-2 of the Advisers Act — non-erasable, non-rewritable, indexed. Audit trails that survive a regulatory document request.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_users,
                        title: "Vendor management discipline",
                        description: "Inventory of every cloud service holding client data, a real BAA-equivalent agreement with each, periodic re-reviews. Custodians ask for this; we have the template.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_shield,
                        title: "Wire-fraud + impersonation defenses",
                        description: "Email authentication tuned for the specific lures targeting advisory practices: client-impersonation wires, fee-quarter spoofing, custodian-portal lookalikes. Endpoint defenses sized to a small practice.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_audit,
                        title: "BCP that's rehearsed, not aspirational",
                        description: "Business continuity plan documented + tested annually. Restoration runbooks for the realistic scenarios (laptop failure, ransomware, key-employee departure). The carrier renewal questionnaire becomes a five-minute fill-in instead of a fire drill.",
                    }.render())
                }
            }
        }

        section class="py-20 bg-slate-900 text-white relative overflow-hidden" {
            div class="absolute top-0 right-0 w-96 h-96 bg-primary/20 rounded-full blur-3xl -translate-y-1/2 translate-x-1/2" {}
            div class="container relative mx-auto px-4 md:px-6 max-w-4xl reveal" {
                span class="inline-block px-3 py-1 rounded-full bg-white/10 text-white text-sm font-medium mb-6 backdrop-blur-sm border border-white/10" {
                    "Why advisory practices come to us"
                }
                h2 class="font-display text-3xl md:text-4xl font-bold mb-6 leading-tight" {
                    "We design infrastructure that produces the evidence regulators ask for."
                }
                p class="text-slate-400 text-lg leading-relaxed mb-6" {
                    "Most vendors say they understand SEC compliance and ask you to take their word for it. We design pipelines where the controls are documented inline, the audit trail is reproducible from logs, and the answer to \"can you show this?\" is \"yes, here\" instead of \"let me check.\""
                }
                p class="text-slate-400 text-lg leading-relaxed mb-8" {
                    "When the custodian-side vendor review arrives, you forward a packet. When the SEC examiner's letter arrives, you respond on time, in writing, with evidence. Both events go from existential threats to routine paperwork."
                }
            }
        }

        section class="py-20 bg-primary/5" {
            div class="container mx-auto px-4 md:px-6 text-center max-w-3xl reveal" {
                h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-6" { "Ready to talk?" }
                p class="text-slate-600 text-lg mb-8" {
                    "Tell us what's on your plate — even if you're not sure whether it's an IT problem yet. The first conversation is free, the NDA is mutual, and we'll tell you if we're not the right fit."
                }
                a href="/contact" {
                    button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium bg-primary text-primary-foreground border border-primary-border min-h-10 px-8 py-6 rounded-xl text-lg shadow-xl shadow-primary/20 hover:-translate-y-0.5 transition-all" {
                        "Schedule a custodian-readiness review"
                    }
                }
                p class="text-slate-500 text-sm mt-6" {
                    "Or write to "
                    a href="mailto:team@plausiden.com" class="text-primary font-medium" { "team@plausiden.com" }
                    " · 978-351-6495"
                }
            }
        }
    };
    page_with_description(
        "Financial Advisor IT — PlausiDen",
        "/solutions/financial-advisors",
        FA_DESCRIPTION,
        body,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_nonempty() {
        assert!(render().into_string().len() > 5000);
    }

    #[test]
    fn hero_references_custodians_and_audience() {
        let s = render().into_string();
        assert!(s.contains("Custodians"));
        assert!(s.to_lowercase().contains("rias") || s.to_lowercase().contains("advisor"));
    }

    /// REGRESSION-GUARD: must not give legal/financial/regulatory advice.
    #[test]
    fn no_advice_claim() {
        let s = render().into_string().to_lowercase();
        for forbidden in &[
            "legal advice",
            "financial advice",
            "investment advice",
            "we advise",
        ] {
            assert!(!s.contains(forbidden), "forbidden phrase: {forbidden}");
        }
    }

    #[test]
    fn final_cta_points_to_contact() {
        let s = render().into_string();
        assert!(s.contains(r#"href="/contact""#));
        assert!(s.contains("team@plausiden.com"));
    }
}
