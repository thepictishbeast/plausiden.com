//! `/solutions/journalism` — vertical landing page for independent
//! journalists, small newsrooms, and investigative collaboratives.
//!
//! Audience: editor-in-chief or technical lead at a 1-25 person
//! newsroom; or a freelance investigative journalist working with
//! sensitive sources. Pre-qualified by outbound; the page confirms
//! fit and produces a contact-form submit.

use loom_components::card::FeatureCard;
use loom_components::hero::{Hero, HeroBackground};
use loom_icons as icons;
use maud::{Markup, PreEscaped, html};

use crate::views::layout::page_with_description;

const J_DESCRIPTION: &str = "Source-confidentiality-first IT for newsrooms and investigative journalists. Encrypted communications, hardened endpoints, threat models that account for state-level adversaries. We build the infrastructure most newsrooms cannot afford to build alone.";

/// Render the journalism-vertical landing page.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn render() -> Markup {
    let cta = html! {
        a href="/contact" {
            button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring bg-primary text-primary-foreground border border-primary-border min-h-10 text-lg px-8 py-6 rounded-xl shadow-lg shadow-primary/25 hover:shadow-xl hover:-translate-y-0.5 transition-all" {
                "Schedule a threat-model conversation"
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
    let svg_globe = icons::GLOBE.render();
    let svg_audit = icons::CLIPBOARD_CHECK.render();
    let body = html! {

        (Hero {
            eyebrow: Some("For journalists + newsrooms"),
            headline_lead: "Sources should be able to trust you.",
            headline_accent: Some("Your tools should too."),
            subheadline: "Source confidentiality is your most binding obligation, and the threat model is real. We build infrastructure for newsrooms whose adversaries include state-level actors, well-resourced corporate counsel, and routine credential phishing. Same posture, sized to your team.",
            cta: Some(&cta),
            background: HeroBackground::GridLight,
        }.render())

        section class="py-20 bg-white" {
            div class="container mx-auto px-4 md:px-6 max-w-4xl" {
                div class="reveal" {
                    h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-6" {
                        "What good infrastructure looks like for a newsroom"
                    }
                    p class="text-slate-600 text-lg leading-relaxed mb-4" {
                        "Most IT vendors treat newsrooms like any other small business: a Microsoft 365 tenant, a help desk, occasional patching. That's not enough. Your threat model includes targeted phishing tuned to specific bylines, supply-chain attacks on commodity collaboration tools, lawful and unlawful demands for source identification, and the very real risk of a former staffer's laptop becoming a source-list disclosure."
                    }
                    p class="text-slate-600 text-lg leading-relaxed" {
                        "We start somewhere different. We design assuming compromise is possible and bound the blast radius accordingly. The result tends to look unfamiliar to general-purpose IT shops — and like a relief to editors who've been doing this work in their head."
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
                        "Concrete capabilities where small newsrooms most often need help."
                    }
                }
                div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 reveal reveal-delay-1" {
                    (FeatureCard {
                        icon_svg: &svg_lock,
                        title: "Encrypted source channels",
                        description: "Self-hosted secure dropbox alternative — without the SecureDrop operational overhead. End-to-end encrypted intake routing to specific journalists, key rotation discipline, anonymous-tip workflows that don't depend on Tor literacy from the source.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_users,
                        title: "Newsroom-aware access discipline",
                        description: "Per-story / per-investigation access scopes. An investigative team's working files are visible to that team only — not to the metro desk, not to legal until it's time. Onboarding/offboarding scripts that don't leave a former freelancer with stale source access.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_file,
                        title: "Document handling with metadata discipline",
                        description: "Workflow that strips metadata before publication, preserves it for verification audit trails, and treats document-level access as a first-class concept. Designed for the moment a leaked PDF needs to be cleaned without losing the chain-of-custody record.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_shield,
                        title: "Endpoint hardening sized to your staff",
                        description: "Threat-modeled laptop + phone configurations: full-disk encryption, application allowlists, USB policy, DLP. Tuned to your reporters' actual workflows — not enterprise theater that gets disabled the first time it blocks a deadline.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_globe,
                        title: "Tor + onion publishing",
                        description: "Secondary publication channel via Tor onion service. Submitted as part of standard infrastructure, not a side project. Your readers in countries where the clearnet site is blocked still reach you.",
                    }.render())
                    (FeatureCard {
                        icon_svg: &svg_audit,
                        title: "Subpoena-ready records discipline",
                        description: "Logs, retention policies, and access trails organized for the moment legal calls. Documentation that survives a subpoena response, a 230-c-2 takedown demand, or a Pulitzer-side records request. We know which evidence each reviewer wants because we have prepared this kind of dossier before.",
                    }.render())
                }
            }
        }

        section class="py-20 bg-slate-900 text-white relative overflow-hidden" {
            div class="absolute top-0 right-0 w-96 h-96 bg-primary/20 rounded-full blur-3xl -translate-y-1/2 translate-x-1/2" {}
            div class="container relative mx-auto px-4 md:px-6 max-w-4xl reveal" {
                span class="inline-block px-3 py-1 rounded-full bg-white/10 text-white text-sm font-medium mb-6 backdrop-blur-sm border border-white/10" {
                    "Why newsrooms come to us"
                }
                h2 class="font-display text-3xl md:text-4xl font-bold mb-6 leading-tight" {
                    "Source confidentiality is a property of the architecture, not a promise."
                }
                p class="text-slate-400 text-lg leading-relaxed mb-6" {
                    "Most vendors say they take source confidentiality seriously and ask you to take their word for it. We design pipelines where the promise is verifiable from the architecture — where the answer to \"could this system have leaked the source list?\" is sometimes \"no, by construction\" instead of \"let me check the logs.\""
                }
                p class="text-slate-400 text-lg leading-relaxed mb-8" {
                    "When something goes wrong — and infrastructure eventually does — your incident response is shorter because the blast radius was bounded by design. When legal calls, you have answers your IT vendor can defend in writing."
                }
                div class="space-y-4" {
                    (check_line("Engagements scope-limited to what we agreed to touch — no roving access to your source list or working files."))
                    (check_line("Configuration changes proposed in writing before they ship; nothing changes silently."))
                    (check_line("Documentation that survives a personnel change at our shop or yours."))
                    (check_line("Plain-English explanations of every choice, so editorial leadership understands what's in place and why."))
                }
            }
        }

        section class="py-20 bg-white" {
            div class="container mx-auto px-4 md:px-6 max-w-4xl reveal" {
                h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-6" {
                    "How an engagement starts"
                }
                p class="text-slate-600 text-lg leading-relaxed mb-8" {
                    "There's no template. Every newsroom we work with starts with a different bottleneck — a recent phishing campaign, a pending subpoena, a beat that suddenly needs onion publication, a Slack workspace that became a de-facto source list. The intake conversation is short; the proposal that follows is specific."
                }
                ol class="space-y-6 text-slate-700" {
                    li class="flex gap-4" {
                        span class="flex-shrink-0 w-8 h-8 rounded-full bg-primary text-white font-bold flex items-center justify-center text-sm" { "1" }
                        div {
                            p class="font-semibold text-slate-900 mb-1" { "Threat-model conversation (no commitment)" }
                            p class="text-slate-600" { "A 45-minute call about your beat, your sources' adversaries, your current pain. We sign a mutual NDA before; we leave with enough to write a real proposal." }
                        }
                    }
                    li class="flex gap-4" {
                        span class="flex-shrink-0 w-8 h-8 rounded-full bg-primary text-white font-bold flex items-center justify-center text-sm" { "2" }
                        div {
                            p class="font-semibold text-slate-900 mb-1" { "Written proposal" }
                            p class="text-slate-600" { "Specific scope, specific deliverables, specific price. We adjust to grant cycles where applicable." }
                        }
                    }
                    li class="flex gap-4" {
                        span class="flex-shrink-0 w-8 h-8 rounded-full bg-primary text-white font-bold flex items-center justify-center text-sm" { "3" }
                        div {
                            p class="font-semibold text-slate-900 mb-1" { "Implementation + handoff" }
                            p class="text-slate-600" { "We do the work; you get documentation that lets your next vendor (or in-house technologist) run it without us. Optional ongoing retainer for monitoring + incident response." }
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
                        "Schedule a threat-model conversation"
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
        "Journalism IT — PlausiDen",
        "/solutions/journalism",
        J_DESCRIPTION,
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
    fn hero_references_sources_and_audience() {
        let s = render().into_string();
        assert!(s.contains("Source"));
        assert!(s.contains("journalists") || s.contains("newsrooms"));
    }

    #[test]
    fn capability_grid_present() {
        let s = render().into_string();
        for cap in &[
            "Encrypted source channels",
            "Newsroom-aware access",
            "Document handling",
            "Endpoint hardening",
            "Tor + onion publishing",
            "Subpoena-ready records",
        ] {
            assert!(s.contains(cap), "missing capability card: {cap}");
        }
    }

    #[test]
    fn no_legal_advice_claim() {
        let s = render().into_string().to_lowercase();
        for forbidden in &["legal advice", "we advise"] {
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
