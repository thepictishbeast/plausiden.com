//! `/solutions/healthcare` — vertical landing page for small healthcare
//! practices and their administrators.
//!
//! Audience: practice manager, IT lead, or owner-operator at an
//! independent practice (1-25 providers). Already pre-qualified by an
//! outbound email; the page confirms fit and produces a contact-form
//! submit.
//!
//! BUG ASSUMPTION: We never claim "HIPAA compliant" — that's a
//! determination only a covered entity can make. We claim our systems
//! are "designed around HIPAA's Security Rule" or similar.

use loom_components::card::FeatureCard;
use loom_components::{TextLink, TextLinkSize, TextLinkVariant};
use loom_components::hero::{Hero, HeroBackground};
use loom_icons as icons;
use maud::{Markup, PreEscaped, html};

use crate::views::layout::page_with_description;

const HC_DESCRIPTION: &str = "IT infrastructure designed around HIPAA's Security Rule for small healthcare practices. Self-hosted email, audit-ready ePHI handling, BAA-ready posture. Built for practices that take patient confidentiality as the floor, not the ceiling.";

/// Render the healthcare-vertical landing page.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn render() -> Markup {
    let cta = html! {
        a href="/contact" {
            button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring bg-primary text-primary-foreground border border-primary-border min-h-10 text-lg px-8 py-6 rounded-xl shadow-lg shadow-primary/25 hover:shadow-xl hover:-translate-y-0.5 transition-all" {
                "Schedule a security review"
            }
        }
        a href="/services" {
            button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium border border-slate-200 min-h-10 text-lg px-8 py-6 rounded-xl bg-white/50 backdrop-blur-sm hover:bg-white" {
                "See our services"
            }
        }
    };
    let svg_lock = icons::LOCK.render();
    let svg_heart = icons::HEART.render();
    let svg_file = icons::FILE_TEXT.render();
    let svg_audit = icons::CLIPBOARD_CHECK.render();
    let svg_users = icons::USERS.render();
    let svg_shield = icons::SHIELD.render();
    let body = html! {

        (Hero {
            eyebrow: Some("For healthcare practices"),
            headline_lead: "Patient confidentiality,",
            headline_accent: Some("designed in."),
            subheadline: "HIPAA's Security Rule is the floor for every IT decision in your practice. We design infrastructure that satisfies the rule by construction — and gives you the documentation that BAAs, breach-notification timelines, and OCR audits will eventually ask for.",
            cta: Some(&cta),
            background: HeroBackground::GridLight,
        }.render())

        section class="py-20 bg-white" {
            div class="container mx-auto px-4 md:px-6 max-w-4xl" {
                div class="reveal" {
                    h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-6" {
                        "What \"good IT for a small practice\" actually means"
                    }
                    p class="text-slate-600 text-lg leading-relaxed mb-4" {
                        "Most managed-service providers treat a five-provider clinic the same as a real-estate office: a Microsoft 365 tenant, a backup script, a help desk number. That works until OCR sends a letter, until your malpractice carrier asks specific questions about ePHI handling, until a former employee's laptop walks out the door."
                    }
                    p class="text-slate-600 text-lg leading-relaxed" {
                        "We start somewhere different: with the Security Rule's actual administrative, physical, and technical safeguards, then design the technical posture that satisfies them. The result tends to look unfamiliar to general-purpose IT shops — and like a relief to administrators who've been translating between counsel, carrier, and vendor for years."
                    }
                }
            }
        }

        section class="py-16 bg-slate-50" {
            div class="container mx-auto px-4 md:px-6 max-w-6xl" {
                div class="text-center max-w-3xl mx-auto mb-12 reveal" {
                    h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-4" {
                        "What we cover"
                    }
                    p class="text-slate-600 text-lg" {
                        "Concrete capability areas where small practices most often need help."
                    }
                }
                div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 reveal reveal-delay-1" {
                    (FeatureCard {
                        icon_svg: &svg_lock,
                        title: "ePHI-aware email + messaging",
                        description: "Self-hosted mail with TLS-required transport, DKIM/SPF/DMARC enforced, encrypted-at-rest storage. Patient communication routed without third-party content scanners. Bcc-to-self workflows that survive an OCR records request.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_heart,
                        title: "EHR integration without the surface",
                        description: "Sane integration with whichever EHR you actually run (Athena, eClinicalWorks, Cerner, OpenEMR). We harden the access surface around the EHR rather than replace it; we never recommend a switch unless the existing one is the actual bottleneck.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_file,
                        title: "Document handling with retention discipline",
                        description: "Document storage with HIPAA-aligned access control, retention policies that honor state record-keeping rules, and audit trails that survive the inevitable \"who saw this chart?\" question.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_audit,
                        title: "BAA + audit-ready posture",
                        description: "Logs, access reviews, control documentation, and risk assessments organized for OCR audits, malpractice questionnaires, and downstream BAA partner reviews. We've answered these questions before; we know which evidence each reviewer wants.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_users,
                        title: "Workforce access discipline",
                        description: "User and group structure mirroring how the practice actually runs — providers, nurses, billing, admin, contractors. Onboarding/offboarding scripts that don't leave a former employee with stale ePHI access. Termination is a one-command operation, not a 12-step checklist.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_shield,
                        title: "Threat-modeled defense",
                        description: "Phishing resistance tuned to the lures targeting clinics (fake referral attachments, billing-software impersonations, ransomware aimed at practices unable to operate offline). Endpoint and network defenses sized to the practice — no enterprise theater you can't operate.",
                    }.render())
                }
            }
        }

        section class="py-20 bg-slate-900 text-white relative overflow-hidden" {
            div class="absolute top-0 right-0 w-96 h-96 bg-primary/20 rounded-full blur-3xl -translate-y-1/2 translate-x-1/2" {}
            div class="container relative mx-auto px-4 md:px-6 max-w-4xl reveal" {
                span class="inline-block px-3 py-1 rounded-full bg-white/10 text-white text-sm font-medium mb-6 backdrop-blur-sm border border-white/10" {
                    "Why practices come to us"
                }
                h2 class="font-display text-3xl md:text-4xl font-bold mb-6 leading-tight" {
                    "We design infrastructure where the privacy guarantee is provable, not promised."
                }
                p class="text-slate-400 text-lg leading-relaxed mb-6" {
                    "Most vendors say they take privacy seriously and ask you to take their word for it. We design pipelines where the promise is verifiable from the architecture — where the answer to \"could this system have leaked this chart?\" is sometimes \"no, by construction\" instead of \"let me check the logs.\""
                }
                p class="text-slate-400 text-lg leading-relaxed mb-8" {
                    "That posture matters in two places. First, when OCR or your malpractice carrier asks pointed questions, you have answers your IT vendor can defend in writing. Second, when something goes wrong — and infrastructure eventually does — your incident response is shorter because the blast radius was bounded by design."
                }
                div class="space-y-4" {
                    (check_line("Engagements scope-limited to what we agreed to touch — no roving access to your EHR or chart store."))
                    (check_line("Configuration changes proposed in writing before they ship; nothing changes silently."))
                    (check_line("Documentation that survives a personnel change at our shop or yours."))
                    (check_line("Plain-English explanations of every choice, so your malpractice carrier and outside auditor can each understand what's in place."))
                }
            }
        }

        section class="py-20 bg-white" {
            div class="container mx-auto px-4 md:px-6 max-w-4xl reveal" {
                h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-6" {
                    "How an engagement starts"
                }
                p class="text-slate-600 text-lg leading-relaxed mb-8" {
                    "There's no template. Every practice we work with starts with a different bottleneck — a flagged carrier questionnaire, a botched cloud migration, a ransomware near-miss, an employee who left with the wrong things on a laptop. The intake conversation is short; the proposal that follows is specific."
                }
                ol class="space-y-6 text-slate-700" {
                    li class="flex gap-4" {
                        span class="flex-shrink-0 w-8 h-8 rounded-full bg-primary text-white font-bold flex items-center justify-center text-sm" { "1" }
                        div {
                            p class="font-semibold text-slate-900 mb-1" { "Security review (no commitment)" }
                            p class="text-slate-600" { "A 45-minute call about your current setup, current pain, and the specific obligations that shape your decisions. We sign a mutual NDA before; we leave with enough to write a real proposal." }
                        }
                    }
                    li class="flex gap-4" {
                        span class="flex-shrink-0 w-8 h-8 rounded-full bg-primary text-white font-bold flex items-center justify-center text-sm" { "2" }
                        div {
                            p class="font-semibold text-slate-900 mb-1" { "Written proposal + BAA scope" }
                            p class="text-slate-600" { "Specific scope, specific deliverables, specific price. Includes a draft BAA scope and clarity on which of your existing tools stay vs. change." }
                        }
                    }
                    li class="flex gap-4" {
                        span class="flex-shrink-0 w-8 h-8 rounded-full bg-primary text-white font-bold flex items-center justify-center text-sm" { "3" }
                        div {
                            p class="font-semibold text-slate-900 mb-1" { "Implementation + handoff" }
                            p class="text-slate-600" { "We do the work; you get documentation that lets your next vendor (or in-house IT person) run it without us. Optional ongoing retainer for monitoring + incident response." }
                        }
                    }
                }
            }
        }

        section class="py-20 bg-primary/5" {
            div class="container mx-auto px-4 md:px-6 text-center max-w-3xl reveal" {
                h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-6" {
                    "Ready to talk?"
                }
                p class="text-slate-600 text-lg mb-8" {
                    "Tell us what's on your plate — even if you're not sure whether it's an IT problem yet. The first conversation is free, the NDA is mutual, and we'll tell you if we're not the right fit."
                }
                a href="/contact" {
                    button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium bg-primary text-primary-foreground border border-primary-border min-h-10 px-8 py-6 rounded-xl text-lg shadow-xl shadow-primary/20 hover:-translate-y-0.5 transition-all" {
                        "Schedule a security review"
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
        "Healthcare IT — PlausiDen",
        "/solutions/healthcare",
        HC_DESCRIPTION,
        body,
    )
}

fn check_line(text: &str) -> Markup {
    html! {
        div class="flex items-start gap-3" {
            (PreEscaped(icons::CHECK.render()))
            span class="text-slate-300" { (text) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_nonempty() {
        let s = render().into_string();
        assert!(s.len() > 5000);
    }

    #[test]
    fn hero_references_security_rule_and_audience() {
        let s = render().into_string();
        assert!(s.contains("Security Rule"));
        assert!(s.contains("healthcare practices"));
    }

    #[test]
    fn capability_grid_present() {
        let s = render().into_string();
        for cap in &[
            "ePHI-aware email",
            "EHR integration",
            "Document handling",
            "BAA + audit-ready",
            "Workforce access",
            "Threat-modeled defense",
        ] {
            assert!(s.contains(cap), "missing capability card: {cap}");
        }
    }

    /// REGRESSION-GUARD: the page must not claim "HIPAA compliant"
    /// outright — only a covered entity can determine that.
    #[test]
    fn no_unconditional_hipaa_compliant_claim() {
        let s = render().into_string().to_lowercase();
        assert!(
            !s.contains("hipaa compliant"),
            "page implies HIPAA compliance"
        );
        assert!(!s.contains("we are compliant with hipaa"));
    }

    #[test]
    fn no_legal_or_medical_advice_claim() {
        let s = render().into_string().to_lowercase();
        for forbidden in &["legal advice", "medical advice", "we advise"] {
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
