//! `/about` — who the firm is, structurally, and what that means for a buyer.
//!
//! Previously a captured React DOM baked in with `include_str!`. That made it
//! the single largest source of design drift on the site: it carried its own
//! button row, its own hero radius and its own spacing, all of which diverged
//! from the Loom compositions every other page uses. It rendered a `flex gap-4`
//! CTA row that overflowed the viewport at 390px, and a `rounded-3xl` hero
//! image where the homepage hero used `rounded-2xl`. Fixing the blob meant
//! hand-matching patterns; composing it from Loom means the patterns cannot
//! drift in the first place.
//!
//! Typography-led, with no hero image. /services, /how-we-work and
//! /sample-report are all text-led, and a generic stock photograph of an office
//! tower was the only thing on the site claiming a scale this firm does not
//! have. Restraint is the point.
//!
//! Nothing here asserts a fact Paul has not supplied: no client count, no years
//! in business, no certification, no headcount. The persuasion is structural —
//! who we work with, how the firm is set up, what we refuse, and what a client
//! can hold us to.

use loom_components::card::FeatureCard;
use loom_components::hero::{Hero, HeroBackground};
use loom_components::{
    Button, ButtonShape, ButtonSize, ButtonType, ButtonVariant, Decoration, Eyebrow, EyebrowSize,
    Heading, HeadingLevel, HeadingTone, HeadingVariant, Lede, Section, SectionPadding,
    SectionTheme, SectionWidth, TextLink, TextLinkSize, TextLinkVariant,
};
use maud::{Markup, html};

use super::layout::page_with_description;

const ABOUT_DESCRIPTION: &str = "A woman-owned IT, security and disaster-recovery firm in Greater Boston, working with practices of 5 to 100 staff that hold confidential client data.";

// loom-allow: inline-SVG icon strings — the w-6 h-6 text-primary chain lives inside the SVG class attribute, not as Maud-emitted utilities
const ICON_USERS: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-6 h-6 text-primary"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>"#; // loom-allow: SVG class attribute
const ICON_BUILDING: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-6 h-6 text-primary"><rect width="16" height="20" x="4" y="2" rx="2"/><path d="M9 22v-4h6v4"/><path d="M8 6h.01M16 6h.01M12 6h.01M12 10h.01M12 14h.01M16 10h.01M16 14h.01M8 10h.01M8 14h.01"/></svg>"#; // loom-allow: SVG class attribute
const ICON_NO: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-6 h-6 text-primary"><circle cx="12" cy="12" r="10"/><path d="m4.9 4.9 14.2 14.2"/></svg>"#; // loom-allow: SVG class attribute

struct Fact<'a> {
    icon: &'a str,
    title: &'a str,
    description: &'a str,
}

/// The three structural facts a buyer needs, as cards.
///
/// "What we do not do" earns its place: refusing work is the cheapest
/// credible signal a small firm has, and it is the one a scan-and-bill
/// shop cannot copy without losing revenue.
const FACTS: &[Fact<'_>] = &[
    Fact {
        icon: ICON_USERS,
        title: "Who we work with",
        description: "Practices of 5 to 100 staff that hold confidential client information: law firms, medical practices, financial advisers, newsrooms and nonprofits. Usually the IT function is one capable person plus a reseller, and it has outgrown that.",
    },
    Fact {
        icon: ICON_BUILDING,
        title: "How we are set up",
        description: "A woman-owned business in Massachusetts, led by Chief Executive Officer Deborah Armstrong, serving Greater Boston and clients working remotely. Small enough that the person you meet is the person who does the work.",
    },
    Fact {
        icon: ICON_NO,
        title: "What we do not do",
        description: "We do not bill by the hour and hope, resell hardware we do not stand behind, or take work we are not the right firm for. When something sits outside what we are good at, we say so and point you at someone better.",
    },
];

/// Commitments a client can check, phrased so failure is observable.
///
/// Deliberately overlapping with the six on /how-we-work rather than
/// inventing a second vocabulary: the same promise should read the same
/// way wherever a buyer meets it.
const HOLD_US_TO: &[&str] = &[
    "A fixed price, agreed in writing before any work starts. If the scope changes, the price conversation happens before the work does, not on the invoice.",
    "The engineer who does the work answers your call. Not an account manager who relays the question and comes back tomorrow.",
    "Anything critical reaches you the day we find it. We do not hold a live issue back to preserve the shape of a final report.",
    "Documentation your next vendor could take over from. If you replace us, the handover is a folder, not a negotiation.",
];

/// Render `/about`.
#[must_use]
pub fn render() -> Markup {
    let body = html! {
        (Hero {
            eyebrow: Some("About us"),
            headline_lead: "The unglamorous parts,",
            headline_accent: Some("done properly."),
            subheadline: "PlausiDen runs the IT and security function for organizations that cannot afford to be careless with other people's information. We are a woman-owned business in Massachusetts, small on purpose, and we would rather turn work down than take an engagement we are the wrong firm for.",
            cta: None,
            background: HeroBackground::GridLight,
        }.render())

        (opening_band())
        (facts_band())
        (small_on_purpose_band())
        (commitments_band())
        (closing_band())
    };
    page_with_description(
        "About PlausiDen — Woman-Owned IT & Security, Massachusetts",
        "/about",
        ABOUT_DESCRIPTION,
        body,
    )
}

/// What the work actually consists of.
fn opening_band() -> Markup {
    let body = html! {
        div class="pd-reveal" {
            (Lede {
                text: "The work is patching on a schedule someone signed off, backups that have actually been restored from, access that is removed the day somebody leaves, and documentation good enough that your next vendor could take over without us. None of it is interesting. All of it is what fails.",
                tone: HeadingTone::Ink,
            }.render())
            p class="text-slate-600 text-[15px] md:text-base leading-relaxed font-light mt-6" { // loom-allow: body paragraph following a Lede
                "Most of the risk in a small practice is not exotic. It is a forgotten server nobody has logged into since a migration, a backup nobody has tested, a permission granted years ago for a project that ended, or a shared login three people still use. We work those first, in writing, and tell you plainly which risks we are accepting for now and why."
            }
            p class="text-slate-600 text-[15px] md:text-base leading-relaxed font-light mt-6" { // loom-allow: body paragraph
                "That order matters. A firm that opens with the exciting findings and leaves the boring ones for a later phase has optimised its report, not your exposure."
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

/// The three structural facts, as a card grid.
fn facts_band() -> Markup {
    let body = html! {
        div class="pd-reveal" {
            (Eyebrow { text: "The firm", size: EyebrowSize::Section }.render())
            // Carries the H2 for this band. FeatureCard emits an H3 per card,
            // so without a heading here the document jumped H1 -> H3 and a
            // screen-reader user lost the level that groups the three.
            div class="mt-3 mb-10" { // loom-allow: eyebrow-to-heading spacer, wider before a card grid
                (Heading {
                    text: "Who we are, and who we are not for.",
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Section,
                    tone: HeadingTone::Ink,
                }.render())
            }
        }
        div class="grid grid-cols-1 md:grid-cols-3 gap-8 pd-reveal" { // loom-allow: three-up FeatureCard grid, same shape as /how-we-work
            @for f in FACTS {
                (FeatureCard {
                    icon_svg: f.icon,
                    title: f.title,
                    description: f.description,
                }.render())
            }
        }
    };
    Section {
        body: &body,
        theme: SectionTheme::Muted,
        width: SectionWidth::Wide,
        padding: SectionPadding::Loose,
    }
    .render()
}

/// Why the firm is small, framed as a buyer benefit rather than an apology.
///
/// This is the honest answer to the objection a skeptical buyer actually
/// has — "you are tiny, why not hire a real consultancy" — and answering
/// it in public is cheaper than answering it on every call.
fn small_on_purpose_band() -> Markup {
    let body = html! {
        div class="pd-reveal" {
            (Eyebrow { text: "Scale", size: EyebrowSize::Section }.render())
            div class="mt-3 mb-6" { // loom-allow: eyebrow-to-heading spacer, matches /services and /sample-report
                (Heading {
                    text: "Small on purpose.",
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Section,
                    tone: HeadingTone::Ink,
                }.render())
            }
            p class="text-slate-600 text-[15px] md:text-base leading-relaxed font-light" { // loom-allow: body paragraph
                "A larger firm would sell you the same engagement and staff it differently. The partner who scoped the work would hand it to whoever was free that month, the findings would be written by someone you have not met, and your questions would route through an account manager. That model exists because it scales, not because it produces better work."
            }
            p class="text-slate-600 text-[15px] md:text-base leading-relaxed font-light mt-6" { // loom-allow: body paragraph
                "We are structured the other way. The engineer in the scoping call is the engineer doing the work and writing the report, and they take your call afterwards. That caps how much we can run at once, which is a real limit and occasionally means we cannot start when you would like. It is a trade we have made deliberately, and if the date is what matters most, say so early and we will tell you honestly whether we can meet it."
            }
            p class="text-slate-500 text-sm mt-8" { // loom-allow: prose with two inline links
                "The engagement mechanics are on "
                (TextLink { label: "how we work", href: "/how-we-work", variant: TextLinkVariant::Underlined, size: TextLinkSize::Default }.render())
                ", and you can read "
                (TextLink { label: "the report we produce", href: "/sample-report", variant: TextLinkVariant::Underlined, size: TextLinkSize::Default }.render())
                " before buying one."
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

/// Checkable commitments, using the shared proof-rule treatment.
fn commitments_band() -> Markup {
    let body = html! {
        div class="pd-reveal" {
            (Eyebrow { text: "Commitments", size: EyebrowSize::Section }.render())
            div class="mt-3 mb-8" { // loom-allow: eyebrow-to-heading spacer
                (Heading {
                    text: "What you can hold us to.",
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Section,
                    tone: HeadingTone::Ink,
                }.render())
            }
            ul class="space-y-4" { // loom-allow: commitment list, same pd-proof treatment as /services and /sample-report
                @for item in HOLD_US_TO {
                    li class="pd-proof text-slate-600 text-[15px] md:text-base leading-relaxed font-light" { (item) }
                }
            }
        }
    };
    Section {
        body: &body,
        theme: SectionTheme::Muted,
        width: SectionWidth::Article,
        padding: SectionPadding::Loose,
    }
    .render()
}

/// Closing call to action on the dark band, matching the other pages.
fn closing_band() -> Markup {
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
    let body = html! {
        div class="text-center pd-reveal" {
            div class="mb-6" {
                (Heading {
                    text: "Tell us what is on your plate.",
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Section,
                    tone: HeadingTone::Ink,
                }.render())
            }
            div class="mb-8" {
                (Lede {
                    text: "Forty-five minutes, a mutual NDA, and a written proposal afterwards with a specific scope and a specific price. If we are not the right firm for it, we will say so on the call rather than after the invoice.",
                    tone: HeadingTone::Ink,
                }.render())
            }
            a href="/contact" { (cta_button) }
            p class="text-slate-500 text-sm mt-6" { // loom-allow: prose with inline link
                "Or write to "
                (TextLink { label: "team@plausiden.com", href: "mailto:team@plausiden.com", variant: TextLinkVariant::PrimaryMedium, size: TextLinkSize::Default }.render())
                " · 978-351-6495"
            }
        }
    };
    Section {
        body: &body,
        theme: SectionTheme::Tinted,
        width: SectionWidth::Article,
        padding: SectionPadding::Loose,
    }
    .render()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn about_renders_nonempty() {
        let s = render().into_string();
        assert!(
            s.len() > 2000,
            "about page unexpectedly short: {} bytes",
            s.len()
        );
    }

    /// The page must not name Paul or William in any role. The company
    /// officer is Deborah Armstrong; this is a standing instruction, and
    /// an About page is the most likely place for it to be violated by a
    /// well-meaning edit.
    #[test]
    fn no_individual_is_named_as_owner_except_the_ceo() {
        let s = render().into_string();
        for forbidden in ["Paul", "William"] {
            assert!(
                !s.contains(forbidden),
                "/about names {forbidden:?}; the named officer is Deborah Armstrong"
            );
        }
        assert!(
            s.contains("Deborah Armstrong"),
            "/about no longer names the Chief Executive Officer"
        );
        assert!(
            s.contains("woman-owned"),
            "/about no longer states that the business is woman-owned"
        );
    }

    /// Claims that would need Paul to supply evidence. An About page is
    /// where "15 years" and "200+ clients" grow back, so assert their
    /// absence rather than trusting review.
    #[test]
    fn makes_no_claim_we_cannot_back() {
        let s = render().into_string().to_lowercase();
        for claim in [
            "years of experience",
            "years in business",
            "clients trust",
            "trusted by",
            "industry leaders",
            "award",
            "certified",
            "iso 27001",
            "soc 2 certified",
        ] {
            assert!(
                !s.contains(claim),
                "/about contains the unsupported claim {claim:?}; \
                 persuade with method and commitments, not with proof we do not have"
            );
        }
    }

    /// Every commitment should describe something a client could observe
    /// us failing to do. A vague promise is not a commitment.
    #[test]
    fn commitments_are_specific_enough_to_fail() {
        for item in HOLD_US_TO {
            assert!(
                item.split_whitespace().count() >= 15,
                "commitment too vague to be checkable: {item:?}"
            );
        }
    }

    #[test]
    fn links_to_the_pages_that_answer_the_next_question() {
        let s = render().into_string();
        for href in ["/contact", "/how-we-work", "/sample-report"] {
            assert!(s.contains(href), "/about no longer links to {href}");
        }
    }
}
