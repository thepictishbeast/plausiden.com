//! `/pricing-transparency` — plain-English statement of how `PlausiDen`
//! prices engagements. Distinguishes us from MSPs that hide pricing
//! until a sales call.

use loom_components::hero::{Hero, HeroBackground};
use loom_components::{
    Button, ButtonSize, ButtonType, ButtonVariant, Decoration, Heading, HeadingLevel, HeadingTone,
    HeadingVariant, HelperSize, HelperText, Lede, Section, SectionPadding, SectionTheme,
    SectionWidth,
};
use maud::{Markup, PreEscaped, html};

use super::layout::page_with_description;

const PRICING_DESCRIPTION: &str = "How PlausiDen prices engagements. Hourly + retainer + fixed-scope ranges, plain English, no bait-and-switch. We'd rather you know up front whether we're affordable than waste your time on a sales call.";

const ICON_CHECK: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-5 h-5 text-emerald-600 mt-0.5 shrink-0"><polyline points="20 6 9 17 4 12"/></svg>"#;

/// One pricing tier. The four tiers (hourly / retainer / fixed-scope /
/// discovery) share the same shape and only differ in copy.
struct Tier<'a> {
    title: &'a str,
    lede: &'a str,
    price: &'a str,
    helper: &'a str,
}

const TIERS: &[Tier<'_>] = &[
    Tier {
        title: "Hourly engagements",
        lede: "For ongoing work without a fixed scope: configuration changes, incident response, ad-hoc audits.",
        price: "$185 – $275 / hour",
        helper: "Senior engineer rate. Higher end for after-hours / weekend / on-call. Tracked in 15-minute increments. Invoiced monthly with itemized work log.",
    },
    Tier {
        title: "Retainer engagements",
        lede: "Predictable monthly cost for ongoing operational support — patching, monitoring, periodic audit prep.",
        price: "$2,500 – $9,500 / month",
        helper: "Sized to staff count + service surface. Includes a fixed hour bucket; overflow at the standard hourly rate. 30-day cancellation; no long-term lock-in.",
    },
    Tier {
        title: "Fixed-scope projects",
        lede: "For one-time deliverables with a clear shape: cloud migration, mail server self-hosting, security audit + remediation, vertical-specific compliance posture.",
        price: "$8,000 – $60,000 per project",
        helper: "Quoted after a paid discovery (typically $1,500 – $3,000, credited toward the project if you hire us). Discovery deliverable is yours regardless — you can take it elsewhere.",
    },
    Tier {
        title: "Discovery / scoping",
        lede: "When the shape is unclear or you're shopping vendors. We produce a written assessment of your current state, top three risks, and a recommended next-step plan.",
        price: "$1,500 – $3,000, fixed",
        helper: "Two-week turnaround. Yours to keep regardless of next steps.",
    },
];

const PROMISES: &[(&str, &str)] = &[
    (
        "No \"call for pricing.\" ",
        "If we're a bad fit on price, you should know in 30 seconds, not three phone calls.",
    ),
    (
        "No bait-and-switch. ",
        "The proposal we send is what you pay; scope changes require a written change order with a new price.",
    ),
    (
        "No long-term lock-in. ",
        "Retainers are 30-day cancellable. We'd rather earn renewal than collect a termination fee.",
    ),
    (
        "No referral kickbacks. ",
        "When we recommend a third-party tool or vendor, we are not paid to do so. Recommendations are based on fit.",
    ),
    (
        "No license-arbitrage markup. ",
        "If we resell software (Microsoft 365, etc.) we pass through at cost.",
    ),
];

fn tier_card(tier: &Tier<'_>) -> Markup {
    html! {
        div class="reveal" {
            div class="mb-4" {
                (Heading {
                    text: tier.title,
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Sub,
                    tone: HeadingTone::Ink,
                }.render())
            }
            div class="mb-4" {
                (Lede { text: tier.lede, tone: HeadingTone::Ink }.render())
            }
            // loom-allow: large display-priced figure; bound to a recurring
            // 4-card pattern that doesn't fit any current Loom typography step.
            p class="text-slate-900 font-semibold text-2xl mb-2" { (tier.price) }
            (HelperText {
                text: tier.helper,
                size: HelperSize::Default,
                tone: HeadingTone::Ink,
            }.render())
        }
    }
}

/// Render `/pricing-transparency`.
#[must_use]
pub fn render() -> Markup {
    let tiers_body = html! {
        div class="space-y-12" { // loom-allow: vertical rhythm between the 4 tier cards
            @for tier in TIERS {
                (tier_card(tier))
            }
        }
    };
    let tiers_section = Section {
        body: &tiers_body,
        theme: SectionTheme::Light,
        width: SectionWidth::Wide,
        padding: SectionPadding::Default,
    }
    .render();

    let promises_body = html! {
        div class="reveal" {
            div class="mb-6" {
                (Heading {
                    text: "What we don't do",
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Section,
                    tone: HeadingTone::Ink,
                }.render())
            }
            ul class="space-y-3 text-slate-700 text-lg" { // loom-allow: list rhythm + body-size; ul-specific not in Loom
                @for (lead, body) in PROMISES {
                    li class="flex items-start gap-3" { // loom-allow: check-row pattern; future CheckRow primitive
                        (PreEscaped(ICON_CHECK))
                        span { strong { (lead) } (body) }
                    }
                }
            }
        }
    };
    let promises_section = Section {
        body: &promises_body,
        theme: SectionTheme::Muted,
        width: SectionWidth::Wide,
        padding: SectionPadding::Default,
    }
    .render();

    let dark_body = html! {
        div class="reveal" {
            (Heading {
                text: "If our rates don't fit, we'll tell you who does.",
                level: HeadingLevel::H2,
                variant: HeadingVariant::Section,
                tone: HeadingTone::OnDark,
            }.render())
            div class="mt-6" {
                (Lede {
                    text: "We're not a fit for every budget. If you're a 1-2 person practice that needs $50/month tier IT support, you should hire someone other than us — and we'll happily refer. The intake conversation is a free filter that protects your time as much as ours.",
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
                    text: "Ready to talk numbers?",
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
            eyebrow: Some("Pricing"),
            headline_lead: "What it costs,",
            headline_accent: Some("before we get on a call."),
            subheadline: "We'd rather you know up front whether we're affordable than waste your time on a sales call. Here are the ranges. Specific quotes follow the intake conversation; nothing on this page is a binding offer.",
            cta: None,
            background: HeroBackground::GridLight,
        }.render())
        (tiers_section)
        (promises_section)
        (dark_section)
        (cta_section)
    };

    page_with_description(
        "Pricing — PlausiDen",
        "/pricing-transparency",
        PRICING_DESCRIPTION,
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
            assert!(
                s.to_lowercase().contains(&promise.to_lowercase()),
                "missing: {promise}"
            );
        }
    }

    /// Final CTA must point to /contact; otherwise the page can't
    /// produce a conversion.
    #[test]
    fn final_cta_points_to_contact() {
        let s = render().into_string();
        assert!(s.contains(r#"href="/contact""#));
    }
}
