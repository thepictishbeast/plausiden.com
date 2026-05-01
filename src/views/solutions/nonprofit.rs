//! `/solutions/nonprofit` — vertical landing page for small nonprofits
//! handling donor and beneficiary data.

use loom_components::card::FeatureCard;
use loom_components::{TextLink, TextLinkSize, TextLinkVariant};
use loom_components::hero::{Hero, HeroBackground};
use loom_icons as icons;
use maud::{Markup, html};

use crate::views::layout::page_with_description;

const NP_DESCRIPTION: &str = "IT infrastructure for small nonprofits. Donor data hardened against breach, beneficiary confidentiality preserved, audit trails ready for grant reviews + state charity examiners. Sized to a 5–50 person mission, not an enterprise.";

/// Render `/solutions/nonprofit`.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn render() -> Markup {
    let cta = html! {
        a href="/contact" {
            button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring bg-primary text-primary-foreground border border-primary-border min-h-10 text-lg px-8 py-6 rounded-xl shadow-lg shadow-primary/25 hover:shadow-xl hover:-translate-y-0.5 transition-all" {
                "Schedule a privacy review"
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
    let svg_heart = icons::HEART.render();
    let body = html! {

        (Hero {
            eyebrow: Some("For nonprofits + advocacy orgs"),
            headline_lead: "Donor trust + beneficiary confidentiality,",
            headline_accent: Some("designed in."),
            subheadline: "Donors trust you with money; beneficiaries trust you with their stories. Both deserve infrastructure that takes \"don't share this\" as a structural property, not a courtesy. We design it for the funding scale you actually operate at.",
            cta: Some(&cta),
            background: HeroBackground::GridLight,
        }.render())

        section class="py-20 bg-white" {
            div class="container mx-auto px-4 md:px-6 max-w-4xl reveal" {
                h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-6" {
                    "Nonprofit IT, sized to a real mission"
                }
                p class="text-slate-600 text-lg leading-relaxed mb-4" {
                    "Most IT vendors price nonprofits like they price for-profits, then offer a 10% discount. We don't. We size engagements to actual nonprofit budgets, default to recommending free + open-source where it serves you, and produce documentation grant funders can read without an interpreter."
                }
                p class="text-slate-600 text-lg leading-relaxed" {
                    "Mission-aligned organizations also tend to handle some of the most sensitive data in the country: domestic-violence survivor records, asylum-seeker case files, abortion-fund applications, immigrant-services intake. We design the technical posture that protects both your donors and the people you serve."
                }
            }
        }

        section class="py-16 bg-slate-50" {
            div class="container mx-auto px-4 md:px-6 max-w-6xl" {
                div class="text-center max-w-3xl mx-auto mb-12 reveal" {
                    h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-4" { "What we cover" }
                    p class="text-slate-600 text-lg" { "Capability areas where small nonprofits most often need help." }
                }
                div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 reveal reveal-delay-1" {
                    (FeatureCard {
                        icon_svg: &svg_lock,
                        title: "Donor + beneficiary data isolation",
                        description: "Per-program / per-fund / per-cohort access scopes. The development team sees donors; case-management sees beneficiaries; auditors see what they need without seeing each other. Compromise of one role doesn't expose the other.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_heart,
                        title: "Threat-modeled by program type",
                        description: "Different programs have different adversaries. Domestic-violence services design for stalkerware threat models; immigrant-services design for state-level surveillance; advocacy orgs design against subpoena-driven discovery. We tailor by program.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_file,
                        title: "Grant + funder reporting",
                        description: "Reporting infrastructure that produces the right answer per funder, on schedule, without re-keying spreadsheets. Audit trails that satisfy state-charity-bureau examination + IRS Form 990 supporting docs.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_users,
                        title: "Volunteer + contractor onboarding",
                        description: "Onboarding flow for volunteers, interns, contractors, and case workers that scopes access to the program they actually work on. Termination is a one-command operation, not a 12-step checklist.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_shield,
                        title: "Phishing + impersonation defenses",
                        description: "Tuned to the lures that target nonprofits: fake grant-portal login pages, donor-impersonation fund redirects, board-member-spoofing wire requests. Endpoint defenses sized to a small staff.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_audit,
                        title: "Audit-ready documentation",
                        description: "Logs, access reviews, control documentation organized for grant-funder vendor reviews, state charity registrations, and 501(c)(3) governance audits. Often the single biggest gap a small nonprofit has.",
                    }.render())
                }
            }
        }

        section class="py-20 bg-slate-900 text-white relative overflow-hidden" {
            div class="absolute top-0 right-0 w-96 h-96 bg-primary/20 rounded-full blur-3xl -translate-y-1/2 translate-x-1/2" {}
            div class="container relative mx-auto px-4 md:px-6 max-w-4xl reveal" {
                span class="inline-block px-3 py-1 rounded-full bg-white/10 text-white text-sm font-medium mb-6 backdrop-blur-sm border border-white/10" {
                    "Why nonprofits come to us"
                }
                h2 class="font-display text-3xl md:text-4xl font-bold mb-6 leading-tight" {
                    "Mission alignment in the technical posture, not just the marketing."
                }
                p class="text-slate-400 text-lg leading-relaxed mb-6" {
                    "Most IT vendors say they support nonprofits by offering a discount. We support them by designing infrastructure that reflects mission alignment in the architecture: donors are not products, beneficiaries are not data points, and access scopes mirror the trust your mission requires."
                }
                p class="text-slate-400 text-lg leading-relaxed" {
                    "We also know nonprofit budgets are real budgets. Engagements are scoped tightly, recommendations default to free + open-source where it serves you, and we have a sliding-scale option for organizations under $500k annual budget."
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
                        "Schedule a privacy review"
                    }
                }
                p class="text-slate-500 text-sm mt-6" {
                    "Or write to "
                    (TextLink { label: "team@plausiden.com", href: "mailto:team@plausiden.com", variant: TextLinkVariant::PrimaryMedium, size: TextLinkSize::Default }.render())
                    " · 978-351-6495"
                }
            }
        }
    };
    page_with_description(
        "Nonprofit IT — PlausiDen",
        "/solutions/nonprofit",
        NP_DESCRIPTION,
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
    fn hero_references_donors_and_beneficiaries() {
        let s = render().into_string();
        assert!(s.contains("Donor"));
        assert!(s.contains("eneficiar"));
    }

    #[test]
    fn no_advice_claim() {
        let s = render().into_string().to_lowercase();
        for forbidden in &["legal advice", "we advise", "tax advice"] {
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
