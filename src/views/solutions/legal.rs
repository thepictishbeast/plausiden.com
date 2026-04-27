//! `/solutions/legal` — vertical landing page for law-firm IT.
//!
//! Audience: a managing partner, IT director, or office administrator
//! at a small-to-mid-sized firm who clicked through from an outbound
//! email. Already pre-qualified by the email; the page's job is to
//! confirm we're the right fit and produce a contact-form submit.
//!
//! BUG ASSUMPTION: We never make legal-compliance claims that imply we
//! provide legal advice. Wording is "designed around" / "built for" —
//! never "compliant with" without a specific, verified scope.

use loom_components::card::FeatureCard;
use loom_components::hero::{Hero, HeroBackground};
use loom_icons as icons;
use maud::{Markup, PreEscaped, html};

use crate::views::layout::page_with_description;

const LEGAL_DESCRIPTION: &str = "IT infrastructure designed around a law firm's duty of confidentiality. Self-hosted email, matter-aware document handling, audit-ready compliance posture. We design pipelines where the privacy guarantee is provable, not promised.";

// Icons sourced from loom-icons registry. See PlausiDen-Loom/loom-icons.

/// Render the law-firm vertical landing page.
#[must_use]
#[allow(clippy::too_many_lines)] // Single composed page; logically one view.
pub fn render() -> Markup {
    let cta = html! {
        a href="/contact" {
            button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring bg-primary text-primary-foreground border border-primary-border min-h-10 text-lg px-8 py-6 rounded-xl shadow-lg shadow-primary/25 hover:shadow-xl hover:-translate-y-0.5 transition-all" {
                "Schedule a confidentiality review"
            }
        }
        a href="/services" {
            button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium border border-slate-200 min-h-10 text-lg px-8 py-6 rounded-xl bg-white/50 backdrop-blur-sm hover:bg-white" {
                "See our services"
            }
        }
    };
    // Pre-render icon SVGs once so FeatureCard's &str fields can borrow.
    let svg_lock = icons::LOCK.render();
    let svg_file = icons::FILE_TEXT.render();
    let svg_audit = icons::CLIPBOARD_CHECK.render();
    let svg_users = icons::USERS.render();
    let svg_shield = icons::SHIELD.render();
    let body = html! {

        (Hero {
            eyebrow: Some("For law firms"),
            headline_lead: "IT infrastructure your duty of confidentiality",
            headline_accent: Some("can rest on."),
            subheadline: "Your obligations under ABA Model Rule 1.6 don't pause at the firewall. We design and operate the systems behind a modern practice — email, document handling, client communication, audit trails — for firms whose work demands a posture stronger than \"trust us.\"",
            cta: Some(&cta),
            background: HeroBackground::GridLight,
        }.render())

        // -------- The pain --------
        section class="py-20 bg-white" {
            div class="container mx-auto px-4 md:px-6 max-w-4xl" {
                div class="reveal" {
                    h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-6" {
                        "What \"good IT for a law firm\" actually means"
                    }
                    p class="text-slate-600 text-lg leading-relaxed mb-4" {
                        "Most managed-service providers treat law firms like any other small business: a few seats, a Microsoft 365 tenant, a backup script, a help desk. That works until it doesn't — until the day a client asks who has access to their matter file, until an opposing counsel issues a discovery preservation letter, until your malpractice carrier asks specific questions on the renewal questionnaire."
                    }
                    p class="text-slate-600 text-lg leading-relaxed" {
                        "We start somewhere different: with the legal-ethics constraints that actually shape your practice, then design the technical posture that satisfies them. The result tends to look unfamiliar to general-purpose IT shops — and like a relief to firm administrators who've had to translate between counsel and a vendor for years."
                    }
                }
            }
        }

        // -------- Capability cards --------
        section class="py-16 bg-slate-50" {
            div class="container mx-auto px-4 md:px-6 max-w-6xl" {
                div class="text-center max-w-3xl mx-auto mb-12 reveal" {
                    h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-4" {
                        "What we cover"
                    }
                    p class="text-slate-600 text-lg" {
                        "Concrete capability areas where firms most often need help. Engagements typically start with one and expand."
                    }
                }
                div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 reveal reveal-delay-1" {
                    (FeatureCard {
                        icon_svg: &svg_lock,
                        title: "Confidential email + file sharing",
                        description: "Self-hosted mail with TLS-required transport, DKIM/SPF/DMARC enforced, encrypted-at-rest storage, and routing that never sends client data through third-party content scanners. No \"smart features\" that require reading messages.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_file,
                        title: "Matter-aware document handling",
                        description: "Document management with per-matter access control, retention policies that honor preservation obligations, and audit trails that survive the inevitable \"who saw what, when?\" question. Designed to make e-discovery production faster, not slower.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_audit,
                        title: "Compliance-ready audit posture",
                        description: "Logs, access reviews, and control documentation organized for state-bar inquiries, cyber-liability questionnaires, and client-side vendor reviews. We've answered these questions before; we know which evidence each reviewer actually wants.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_users,
                        title: "Conflicts and access discipline",
                        description: "User and group structure that mirrors how matters actually run — partner / associate / paralegal / outside counsel — so an ethical wall is enforced by the file system, not by an attorney remembering not to look. Onboarding and offboarding scripts that don't leave a former associate with stale access.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_shield,
                        title: "Threat-modeled defense",
                        description: "Phishing resistance tuned for the lures that actually target lawyers (wire-fraud impersonations, false subpoenas, malicious filing-portal lookalikes). Endpoint and network defenses sized to the firm — no enterprise theater you can't operate.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_file,
                        title: "Continuity and recovery",
                        description: "Backups that survive ransomware (immutable, tested, restorable to a known-good point). Documented runbooks for the scenarios most likely to take a small firm offline. Retainer-grade response if the worst day happens.",
                    }.render())
                }
            }
        }

        // -------- The posture (differentiator) --------
        section class="py-20 bg-slate-900 text-white relative overflow-hidden" {
            div class="absolute top-0 right-0 w-96 h-96 bg-primary/20 rounded-full blur-3xl -translate-y-1/2 translate-x-1/2" {}
            div class="container relative mx-auto px-4 md:px-6 max-w-4xl reveal" {
                span class="inline-block px-3 py-1 rounded-full bg-white/10 text-white text-sm font-medium mb-6 backdrop-blur-sm border border-white/10" {
                    "Why firms come to us"
                }
                h2 class="font-display text-3xl md:text-4xl font-bold mb-6 leading-tight" {
                    "We design infrastructure where the privacy guarantee is provable, not promised."
                }
                p class="text-slate-400 text-lg leading-relaxed mb-6" {
                    "Most vendors say they take privacy seriously and ask you to take their word for it. We design pipelines where the promise is verifiable from the architecture — where the answer to \"could this system have leaked X?\" is sometimes \"no, by construction\" instead of \"let me check the logs.\""
                }
                p class="text-slate-400 text-lg leading-relaxed mb-8" {
                    "That posture matters in two places. First, when a client or opposing counsel asks pointed questions about your data handling, you have answers your IT vendor can defend in writing. Second, when something goes wrong — and infrastructure eventually does — your incident response is shorter because the blast radius was bounded by design."
                }
                div class="space-y-4" {
                    (check_line("Engagements scope-limited to what we agreed to touch — no roving access to your file server."))
                    (check_line("Configuration changes proposed in writing before they ship; nothing changes silently."))
                    (check_line("Documentation that survives a personnel change at our shop or yours."))
                    (check_line("Plain-English explanations of every choice, so your malpractice carrier and outside auditor can each understand what's in place."))
                }
            }
        }

        // -------- Engagement model --------
        section class="py-20 bg-white" {
            div class="container mx-auto px-4 md:px-6 max-w-4xl reveal" {
                h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-6" {
                    "How an engagement starts"
                }
                p class="text-slate-600 text-lg leading-relaxed mb-8" {
                    "There's no template. Every firm we work with starts with a different bottleneck — a flagged renewal questionnaire, a botched cloud migration, a client demand we read on their behalf, an associate who left with the wrong things on a laptop. The intake conversation is short; the proposal that follows is specific."
                }
                ol class="space-y-6 text-slate-700" {
                    li class="flex gap-4" {
                        span class="flex-shrink-0 w-8 h-8 rounded-full bg-primary text-white font-bold flex items-center justify-center text-sm" { "1" }
                        div {
                            p class="font-semibold text-slate-900 mb-1" { "Confidentiality review (no commitment)" }
                            p class="text-slate-600" { "A 45-minute call about your current setup, current pain, and the specific obligations that shape your decisions. We sign an NDA before; we leave with enough to write a real proposal." }
                        }
                    }
                    li class="flex gap-4" {
                        span class="flex-shrink-0 w-8 h-8 rounded-full bg-primary text-white font-bold flex items-center justify-center text-sm" { "2" }
                        div {
                            p class="font-semibold text-slate-900 mb-1" { "Written proposal" }
                            p class="text-slate-600" { "Specific scope, specific deliverables, specific price. Includes which of your existing tools stay, which we replace, and what the 90-day picture looks like." }
                        }
                    }
                    li class="flex gap-4" {
                        span class="flex-shrink-0 w-8 h-8 rounded-full bg-primary text-white font-bold flex items-center justify-center text-sm" { "3" }
                        div {
                            p class="font-semibold text-slate-900 mb-1" { "Implementation + handoff" }
                            p class="text-slate-600" { "We do the work; you get documentation that lets your next vendor (or in-house IT person) run it without us. Optional ongoing retainer for monitoring and incident response." }
                        }
                    }
                }
            }
        }

        // -------- Final CTA --------
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
                        "Schedule a confidentiality review"
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
        "Legal IT — PlausiDen",
        "/solutions/legal",
        LEGAL_DESCRIPTION,
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

    /// Hero must speak to law firms and surface ABA Rule 1.6, the
    /// foundational confidentiality obligation. If a marketing pass
    /// strips this, the page loses its hook.
    #[test]
    fn hero_references_aba_rule() {
        let s = render().into_string();
        assert!(s.contains("Rule 1.6"));
        assert!(s.contains("law firms"));
    }

    #[test]
    fn capability_grid_present() {
        let s = render().into_string();
        for cap in &[
            "Confidential email",
            "Matter-aware document handling",
            "Compliance-ready audit posture",
            "Threat-modeled defense",
        ] {
            assert!(s.contains(cap), "missing capability card: {cap}");
        }
    }

    #[test]
    fn engagement_steps_numbered() {
        let s = render().into_string();
        assert!(s.contains("Confidentiality review"));
        assert!(s.contains("Written proposal"));
        assert!(s.contains("Implementation + handoff"));
    }

    #[test]
    fn final_cta_points_to_contact() {
        let s = render().into_string();
        assert!(s.contains(r#"href="/contact""#));
        assert!(s.contains("team@plausiden.com"));
    }

    /// REGRESSION-GUARD: we must not accidentally claim the page
    /// itself constitutes legal advice.
    #[test]
    fn no_legal_advice_claim() {
        let s = render().into_string();
        // Forbidden phrases that imply we're giving legal advice
        for forbidden in &["legal advice", "this advice", "we advise"] {
            assert!(
                !s.to_lowercase().contains(forbidden),
                "page implies legal advice: '{forbidden}'"
            );
        }
    }
}
