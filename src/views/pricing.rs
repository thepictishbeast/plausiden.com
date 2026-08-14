//! `/pricing-transparency` — plain-English statement of how `PlausiDen`
//! prices engagements. Distinguishes us from MSPs that hide pricing
//! until a sales call.

use loom_components::hero::{Hero, HeroBackground};
use loom_components::{
    Button, ButtonShape, ButtonSize, ButtonType, ButtonVariant, Decoration, DefinitionRow, Eyebrow,
    EyebrowSize, Heading, HeadingLevel, HeadingTone, HeadingVariant, HelperSize, HelperText, Lede,
    Section, SectionPadding, SectionTheme, SectionWidth, TextLink, TextLinkSize, TextLinkVariant,
};
use maud::{Markup, PreEscaped, html};

use super::layout::page_with_description;

const PRICING_DESCRIPTION: &str = "How PlausiDen prices engagements. Hourly + retainer + fixed-scope ranges, plain English, no bait-and-switch. We'd rather you know up front whether we're affordable than waste your time on a sales call.";

const ICON_CHECK: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-5 h-5 text-emerald-600 mt-0.5 shrink-0"><polyline points="20 6 9 17 4 12"/></svg>"#; // loom-allow: SVG class attribute, not Maud-emitted utility chain

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

/// The published rate card for security assessment work.
///
/// These mirror `PlausiDen-Salesman`'s `salesman-quote` crate, which is what
/// actually produces a client's quote. The page renders its prose *from* these
/// constants rather than restating them in text, so the words and the numbers
/// cannot drift apart, and `rate_card_matches_the_quoting_tool` checks them
/// against the crate's source when it is checked out alongside.
///
/// Publishing a day rate at all is the point. Most firms in this market will
/// not, which is why a buyer cannot compare two proposals without a call.
const DAY_RATE_USD: u32 = 1_600;
const SETUP_REPORTING_FEE_USD: u32 = 1_200;
const EXPEDITED_SURCHARGE_PCT: u32 = 25;
const EXTENDED_RETEST_FEE_USD: u32 = 1_000;
const RETEST_WINDOW_DAYS: u32 = 30;

/// The worked example, in half-days.
///
/// Taken from the quoting tool's own regression case: a 30-endpoint
/// application with two roles at standard complexity scopes to 6.5 days
/// (1.5 day scaffold, 0.15 day per endpoint, 0.5 day for the second role).
/// Using its example rather than inventing one means the figure on this page
/// is the figure the tool would quote.
const EXAMPLE_HALF_DAYS: u32 = 13;
const EXAMPLE_ENDPOINTS: u32 = 30;

/// Format whole dollars with thousands separators.
fn usd(n: u32) -> String {
    let digits = n.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3 + 1);
    out.push('$');
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}

/// Testing days rendered as a decimal, from half-day units.
fn days_label(half_days: u32) -> String {
    if half_days % 2 == 0 {
        format!("{}", half_days / 2)
    } else {
        format!("{}.5", half_days / 2)
    }
}

/// Line items for the assessment rate card, built from the constants above.
fn rate_card_rows() -> Vec<(String, String)> {
    vec![
        (
            format!("{} per engineer-day", usd(DAY_RATE_USD)),
            "An eight-hour day of senior testing time, billed in half-days. The person testing is the person who scoped it and the person who writes the report.".to_owned(),
        ),
        (
            format!("{} flat, once per engagement", usd(SETUP_REPORTING_FEE_USD)),
            "Scoping, threat modeling, report synthesis and the readout call. Charged once whether the assessment runs two days or twenty.".to_owned(),
        ),
        (
            "Included".to_owned(),
            format!("A retest of high and critical findings within {RETEST_WINDOW_DAYS} days. Not an upsell, not a second engagement — verifying the fix is part of the job."),
        ),
        (
            format!("+{EXPEDITED_SURCHARGE_PCT}%"),
            "Rush turnaround or testing outside business hours. Applied to the testing days, and only when you ask for it.".to_owned(),
        ),
        (
            format!("{}", usd(EXTENDED_RETEST_FEE_USD)),
            format!("Optional. Extends the retest window from {RETEST_WINDOW_DAYS} to 60 days when a fix has to wait on a release train."),
        ),
    ]
}

/// The assessment pricing band.
///
/// /services and /sample-report describe the security work in detail, and
/// until now the pricing page had no number for it at all — a buyer who read
/// the sample report and wanted one had to ask. That is the exact "call for
/// pricing" behaviour this page's own promises reject.
fn assessment_band() -> Markup {
    let testing = EXAMPLE_HALF_DAYS * DAY_RATE_USD / 2;
    let total = testing + SETUP_REPORTING_FEE_USD;
    let body = html! {
        div class="pd-reveal" {
            (Eyebrow { text: "Security assessments", size: EyebrowSize::Section }.render())
            div class="mt-3 mb-6" { // loom-allow: eyebrow-to-heading spacer, matches /services and /about
                (Heading {
                    text: "What a penetration test costs.",
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Section,
                    tone: HeadingTone::Ink,
                }.render())
            }
            (Lede {
                text: "Assessment work is priced from a published day rate rather than a range, because a range is not much use when you are comparing two proposals. Here is the whole rate card and the arithmetic.",
                tone: HeadingTone::Ink,
            }.render())
            dl class="border-t border-slate-200 mt-12" { // loom-allow: hairline definition list, same pattern as the /services standards table
                @for (amount, detail) in rate_card_rows() {
                    (DefinitionRow { term: &amount, description: &detail }.render())
                }
            }
            div class="mt-12" { // loom-allow: worked-example block rhythm
                (Eyebrow { text: "Worked example", size: EyebrowSize::Subhead }.render())
                p class="text-slate-600 text-[15px] md:text-base leading-relaxed font-light" {
                    "A web application with roughly " (EXAMPLE_ENDPOINTS) " endpoints and two user roles scopes to "
                    (days_label(EXAMPLE_HALF_DAYS)) " days of testing. That is " (usd(testing)) " of testing time plus the "
                    (usd(SETUP_REPORTING_FEE_USD)) " flat fee, so " strong { (usd(total)) } " — agreed in writing before anything starts, "
                    "with the retest included. If the scope changes mid-engagement, the price conversation happens before the work does."
                }
                p class="text-slate-500 text-sm mt-6" { // loom-allow: prose with inline link
                    "The method behind that number is on the "
                    (TextLink { label: "services page", href: "/services", variant: TextLinkVariant::Underlined, size: TextLinkSize::Default }.render())
                    ", and the report it produces is "
                    (TextLink { label: "published in full", href: "/sample-report", variant: TextLinkVariant::Underlined, size: TextLinkSize::Default }.render())
                    "."
                }
            }
        }
    };
    Section {
        body: &body,
        theme: SectionTheme::Light,
        width: SectionWidth::Article,
        padding: SectionPadding::Loose,
    }
    .render()
}

fn tier_card(tier: &Tier<'_>) -> Markup {
    html! {
        div class="pd-reveal" {
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
            p class="text-slate-900 font-semibold text-2xl mb-2" { (tier.price) } // loom-allow: large display-priced figure — text-2xl semibold doesn't fit any Loom typography step
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

    let assessment_section = assessment_band();

    let promises_body = html! {
        div class="pd-reveal" {
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
        div class="pd-reveal" {
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
        label: "Book a scoping call",
        variant: ButtonVariant::Primary,
        size: ButtonSize::Lg,
        aria_label: None,
        icon: None,
        decoration: Decoration::SoftShadow,
        button_type: ButtonType::Button,
        shape: ButtonShape::default(),
    }
    .render();
    let cta_body = html! {
        div class="text-center pd-reveal" {
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
        (assessment_section)
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
    /// The worked example must be arithmetic a reader can redo, and it
    /// must come from the constants rather than from a number typed into
    /// prose. A pricing page whose example does not add up is worse than
    /// one with no example.
    #[test]
    fn the_worked_example_adds_up() {
        let testing = EXAMPLE_HALF_DAYS * DAY_RATE_USD / 2;
        let total = testing + SETUP_REPORTING_FEE_USD;
        assert_eq!(testing, 10_400, "testing subtotal changed");
        assert_eq!(total, 11_600, "worked-example total changed");

        let page = render().into_string();
        for figure in [
            usd(testing),
            usd(total),
            usd(DAY_RATE_USD),
            usd(SETUP_REPORTING_FEE_USD),
        ] {
            assert!(
                page.contains(&figure),
                "the worked example no longer shows {figure}"
            );
        }
        assert!(
            page.contains("6.5 days"),
            "the example no longer states the day count it prices"
        );
    }

    /// Thousands separators, because $10400 on a pricing page looks like a
    /// typo and undermines the one thing this page is selling.
    #[test]
    fn currency_is_formatted_for_humans() {
        assert_eq!(usd(1_600), "$1,600");
        assert_eq!(usd(11_600), "$11,600");
        assert_eq!(usd(900), "$900");
        assert_eq!(usd(1_000_000), "$1,000,000");
        assert_eq!(days_label(13), "6.5");
        assert_eq!(days_label(4), "2");
    }

    /// The published rates must equal the ones the quoting tool charges.
    ///
    /// A prospect who reads a number here and receives a different one in
    /// their quote has caught the firm being careless with money, on the
    /// page that exists to prove it is not. `salesman-quote` lives in a
    /// sibling repository and is not a build dependency, so this reads its
    /// source directly and reports plainly when it is not checked out
    /// rather than passing quietly and pretending it verified something.
    #[test]
    fn rate_card_matches_the_quoting_tool() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../PlausiDen-Salesman/crates/salesman-quote/src/lib.rs"
        );
        let Ok(src) = std::fs::read_to_string(path) else {
            eprintln!(
                "NOTE: {path} is not present, so the published rates were NOT checked against \
                 the quoting tool. Clone PlausiDen-Salesman alongside this repo to enable it."
            );
            return;
        };
        for (name, ours) in [
            ("DAILY_RATE", DAY_RATE_USD),
            ("SETUP_REPORTING_FEE", SETUP_REPORTING_FEE_USD),
            ("EXPEDITED_SURCHARGE_PCT", EXPEDITED_SURCHARGE_PCT),
            ("EXTENDED_RETEST_FEE", EXTENDED_RETEST_FEE_USD),
        ] {
            let needle = format!("pub const {name}: u32 = ");
            let theirs: u32 = src
                .split(&needle)
                .nth(1)
                .unwrap_or_else(|| panic!("{name} not found in salesman-quote"))
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '_')
                .filter(|c| *c != '_')
                .collect::<String>()
                .parse()
                .unwrap_or_else(|e| panic!("could not parse {name}: {e}"));
            assert_eq!(
                ours, theirs,
                "the website publishes {name} as {ours} but salesman-quote charges {theirs}; \
                 a prospect would read one number here and receive another in their quote"
            );
        }
    }

    fn final_cta_points_to_contact() {
        let s = render().into_string();
        assert!(s.contains(r#"href="/contact""#));
    }
}
