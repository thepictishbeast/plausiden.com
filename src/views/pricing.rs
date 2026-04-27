//! `/pricing-transparency` — plain-English statement of how `PlausiDen`
//! prices engagements. Distinguishes us from MSPs that hide pricing
//! until a sales call.

use loom_components::hero::{Hero, HeroBackground};
use maud::{Markup, PreEscaped, html};

use super::layout::page_with_description;

const PRICING_DESCRIPTION: &str = "How PlausiDen prices engagements. Hourly + retainer + fixed-scope ranges, plain English, no bait-and-switch. We'd rather you know up front whether we're affordable than waste your time on a sales call.";

const ICON_CHECK: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-5 h-5 text-emerald-600 mt-0.5 shrink-0"><polyline points="20 6 9 17 4 12"/></svg>"#;

/// Render `/pricing-transparency`.
#[must_use]
#[allow(clippy::too_many_lines)] // Single composed page; logically one view.
pub fn render() -> Markup {
    let body = html! {

        (Hero {
            eyebrow: Some("Pricing"),
            headline_lead: "What it costs,",
            headline_accent: Some("before we get on a call."),
            subheadline: "We'd rather you know up front whether we're affordable than waste your time on a sales call. Here are the ranges. Specific quotes follow the intake conversation; nothing on this page is a binding offer.",
            cta: None,
            background: HeroBackground::GridLight,
        }.render())

        section class="py-16 bg-white" {
            div class="container mx-auto px-4 md:px-6 max-w-4xl space-y-12" {

                div class="reveal" {
                    h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mb-4" { "Hourly engagements" }
                    p class="text-slate-600 text-lg leading-relaxed mb-4" {
                        "For ongoing work without a fixed scope: configuration changes, incident response, ad-hoc audits."
                    }
                    p class="text-slate-900 font-semibold text-2xl mb-2" { "$185 – $275 / hour" }
                    p class="text-slate-500 text-sm" {
                        "Senior engineer rate. Higher end for after-hours / weekend / on-call. Tracked in 15-minute increments. Invoiced monthly with itemized work log."
                    }
                }

                div class="reveal" {
                    h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mb-4" { "Retainer engagements" }
                    p class="text-slate-600 text-lg leading-relaxed mb-4" {
                        "Predictable monthly cost for ongoing operational support — patching, monitoring, periodic audit prep."
                    }
                    p class="text-slate-900 font-semibold text-2xl mb-2" { "$2,500 – $9,500 / month" }
                    p class="text-slate-500 text-sm" {
                        "Sized to staff count + service surface. Includes a fixed hour bucket; overflow at the standard hourly rate. 30-day cancellation; no long-term lock-in."
                    }
                }

                div class="reveal" {
                    h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mb-4" { "Fixed-scope projects" }
                    p class="text-slate-600 text-lg leading-relaxed mb-4" {
                        "For one-time deliverables with a clear shape: cloud migration, mail server self-hosting, security audit + remediation, vertical-specific compliance posture."
                    }
                    p class="text-slate-900 font-semibold text-2xl mb-2" { "$8,000 – $60,000 per project" }
                    p class="text-slate-500 text-sm" {
                        "Quoted after a paid discovery (typically $1,500 – $3,000, credited toward the project if you hire us). Discovery deliverable is yours regardless — you can take it elsewhere."
                    }
                }

                div class="reveal" {
                    h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mb-4" { "Discovery / scoping" }
                    p class="text-slate-600 text-lg leading-relaxed mb-4" {
                        "When the shape is unclear or you're shopping vendors. We produce a written assessment of your current state, top three risks, and a recommended next-step plan."
                    }
                    p class="text-slate-900 font-semibold text-2xl mb-2" { "$1,500 – $3,000, fixed" }
                    p class="text-slate-500 text-sm" {
                        "Two-week turnaround. Yours to keep regardless of next steps."
                    }
                }
            }
        }

        section class="py-16 bg-slate-50" {
            div class="container mx-auto px-4 md:px-6 max-w-4xl reveal" {
                h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-6" { "What we don't do" }
                ul class="space-y-3 text-slate-700 text-lg" {
                    li class="flex items-start gap-3" {
                        (PreEscaped(ICON_CHECK))
                        span { strong { "No \"call for pricing.\" " } "If we're a bad fit on price, you should know in 30 seconds, not three phone calls." }
                    }
                    li class="flex items-start gap-3" {
                        (PreEscaped(ICON_CHECK))
                        span { strong { "No bait-and-switch. " } "The proposal we send is what you pay; scope changes require a written change order with a new price." }
                    }
                    li class="flex items-start gap-3" {
                        (PreEscaped(ICON_CHECK))
                        span { strong { "No long-term lock-in. " } "Retainers are 30-day cancellable. We'd rather earn renewal than collect a termination fee." }
                    }
                    li class="flex items-start gap-3" {
                        (PreEscaped(ICON_CHECK))
                        span { strong { "No referral kickbacks. " } "When we recommend a third-party tool or vendor, we are not paid to do so. Recommendations are based on fit." }
                    }
                    li class="flex items-start gap-3" {
                        (PreEscaped(ICON_CHECK))
                        span { strong { "No license-arbitrage markup. " } "If we resell software (Microsoft 365, etc.) we pass through at cost." }
                    }
                }
            }
        }

        section class="py-20 bg-slate-900 text-white" {
            div class="container mx-auto px-4 md:px-6 max-w-4xl reveal" {
                h2 class="font-display text-3xl md:text-4xl font-bold mb-6 leading-tight" {
                    "If our rates don't fit, we'll tell you who does."
                }
                p class="text-slate-400 text-lg leading-relaxed" {
                    "We're not a fit for every budget. If you're a 1-2 person practice that needs $50/month tier IT support, you should hire someone other than us — and we'll happily refer. The intake conversation is a free filter that protects your time as much as ours."
                }
            }
        }

        section class="py-20 bg-primary/5" {
            div class="container mx-auto px-4 md:px-6 text-center max-w-3xl reveal" {
                h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-6" { "Ready to talk numbers?" }
                a href="/contact" {
                    button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium bg-primary text-primary-foreground border border-primary-border min-h-10 px-8 py-6 rounded-xl text-lg shadow-xl shadow-primary/20 hover:-translate-y-0.5 transition-all" {
                        "Schedule an intake call"
                    }
                }
            }
        }
    };
    page_with_description("Pricing — PlausiDen", "/pricing-transparency", PRICING_DESCRIPTION, body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_nonempty() {
        assert!(render().into_string().len() > 4000);
    }

    #[test]
    fn shows_concrete_dollar_ranges() {
        let s = render().into_string();
        // Specific number presence — if a future edit removes them, the
        // page becomes "call for pricing" theater. Fail loudly.
        assert!(s.contains("$185"));
        assert!(s.contains("$2,500"));
        assert!(s.contains("$8,000"));
        assert!(s.contains("$1,500"));
    }

    #[test]
    fn lists_what_we_dont_do_promises() {
        let s = render().into_string();
        for promise in &[
            "call for pricing",
            "bait-and-switch",
            "long-term lock-in",
            "referral kickbacks",
        ] {
            assert!(s.to_lowercase().contains(&promise.to_lowercase()), "missing: {promise}");
        }
    }
}
