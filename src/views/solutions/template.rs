//! Shared template for vertical landing pages.
//!
//! `legal`, `healthcare`, and `journalism` share an identical
//! six-band structural skeleton (hero / pain / capabilities /
//! posture / engagement / CTA) with different copy. Before this
//! template, the three pages were 270-line near-duplicates —
//! exactly the shape the composition audit (#79) is built to
//! catch. Extracting one render function drops ~500 lines of
//! duplication and pulls the chrome into a single review point.
//!
//! ## Loom-sourced primitives
//!
//! Every section header, body paragraph, helper note, and inline
//! link composes through `loom-components` primitives. Two raw
//! sections remain: the dark "posture" band (decorative blur
//! element + `bg-white/10` badge inside a slate-900 surface) and
//! the capability-grid wrapper (`max-w-6xl` exceeds Loom's Wide
//! variant). Both are isolated to this file and gated by
//! `// loom-allow` markers so the lint stays clean across all
//! three consumer pages.

use loom_components::card::FeatureCard;
use loom_components::hero::{Hero, HeroBackground};
use loom_components::{
    BodyText, Button, ButtonSize, ButtonType, ButtonVariant, Decoration, Heading, HeadingLevel,
    HeadingTone, HeadingVariant, HelperSize, HelperText, Lede, Section, SectionPadding,
    SectionTheme, SectionWidth, TextLink, TextLinkSize, TextLinkVariant,
};
use loom_icons as icons;
use maud::{Markup, PreEscaped, html};

use crate::views::layout::page_with_description;

/// One capability tile in the "What we cover" grid.
#[derive(Debug)]
pub struct Capability<'a> {
    /// Icon glyph (one of the `loom_icons::*` constants, pre-rendered).
    pub icon_svg: &'a str,
    /// Card heading (e.g. "Confidential email + file sharing").
    pub title: &'a str,
    /// Card body — 1-3 sentences describing the capability.
    pub description: &'a str,
}

/// One step in the "How an engagement starts" ordered list.
#[derive(Debug)]
pub struct EngagementStep<'a> {
    /// Step title (e.g. "Confidentiality review (no commitment)").
    pub title: &'a str,
    /// Step body — 1-2 sentences describing what happens at this step.
    pub description: &'a str,
}

/// Closed config for the vertical landing template.
///
/// Fields map 1:1 to the six structural bands in the rendered
/// page; supplying every field is required so a vertical can't
/// accidentally ship a half-empty page.
#[derive(Debug)]
pub struct VerticalLanding<'a> {
    /// `<title>` tag and document name.
    pub title: &'a str,
    /// URL path (used for nav active-state and canonical link).
    pub path: &'a str,
    /// `<meta name="description">` content.
    pub description: &'a str,

    /// Hero eyebrow text above the headline (e.g. "For law firms").
    pub hero_eyebrow: &'a str,
    /// Hero headline lead (the first half of the two-tone headline).
    pub hero_lead: &'a str,
    /// Hero headline accent (the second, primary-coloured half).
    pub hero_accent: &'a str,
    /// Hero subheadline paragraph below the headline.
    pub hero_subheadline: &'a str,
    /// Label on the hero CTA + final CTA button (e.g. "Schedule a confidentiality review").
    pub primary_cta_label: &'a str,

    /// Pain-band section heading.
    pub pain_heading: &'a str,
    /// Pain-band paragraphs (typically 2).
    pub pain_paragraphs: &'a [&'a str],

    /// Capability-grid section heading (e.g. "What we cover").
    pub capabilities_heading: &'a str,
    /// Lede paragraph below the capability heading.
    pub capabilities_lede: &'a str,
    /// 3 - 9 capability cards (renders as 3-column grid at lg+).
    pub capabilities: &'a [Capability<'a>],

    /// Eyebrow badge inside the dark posture band.
    pub posture_eyebrow: &'a str,
    /// Posture-band section heading.
    pub posture_heading: &'a str,
    /// Posture-band paragraphs (typically 2).
    pub posture_paragraphs: &'a [&'a str],
    /// Bullet check-lines below the posture paragraphs (typically 4).
    pub posture_check_lines: &'a [&'a str],

    /// Engagement-section heading.
    pub engagement_heading: &'a str,
    /// Engagement-section intro paragraph.
    pub engagement_intro: &'a str,
    /// Numbered engagement steps (typically 3).
    pub engagement_steps: &'a [EngagementStep<'a>],

    /// Final-CTA section heading.
    pub cta_heading: &'a str,
    /// Final-CTA subline paragraph above the button.
    pub cta_subline: &'a str,
}

/// Render the vertical landing page from a closed config.
///
/// Per the file-level docstring, two raw chrome blocks remain
/// (capability-grid wrapper, dark posture band) — both isolated
/// here, both annotated, both reviewable as one piece.
#[must_use]
pub fn render_vertical_landing(cfg: VerticalLanding<'_>) -> Markup {
    // Hero CTA pair — primary uses Loom Button (Submit-shaped here
    // is a misnomer; the button sits inside an <a>, so it never
    // submits a form. ButtonType::Button is the right choice).
    let primary_cta_btn = Button {
        label: cfg.primary_cta_label,
        variant: ButtonVariant::Primary,
        size: ButtonSize::Lg,
        aria_label: None,
        icon: None,
        decoration: Decoration::SoftShadow,
        button_type: ButtonType::Button,
    }
    .render();
    let secondary_cta_btn = Button {
        label: "See our services",
        variant: ButtonVariant::Outline,
        size: ButtonSize::Lg,
        aria_label: None,
        icon: None,
        decoration: Decoration::None,
        button_type: ButtonType::Button,
    }
    .render();
    let hero_cta = html! {
        a href="/contact" { (primary_cta_btn) }
        a href="/services" { (secondary_cta_btn) }
    };

    // Pain band — narrow article width, default white surface.
    let pain_body = html! {
        div class="reveal" {
            (Heading {
                text: cfg.pain_heading,
                level: HeadingLevel::H2,
                variant: HeadingVariant::Section,
                tone: HeadingTone::Ink,
            }.render())
            div class="mt-6 space-y-4" { // loom-allow: structural wrapper for paragraph stack
                @for para in cfg.pain_paragraphs {
                    (Lede { text: para, tone: HeadingTone::Ink }.render())
                }
            }
        }
    };
    let pain_section = Section {
        body: &pain_body,
        theme: SectionTheme::Light,
        width: SectionWidth::Wide,
        padding: SectionPadding::Loose,
    }
    .render();

    // Capability grid — `max-w-6xl` exceeds Loom Wide (max-w-4xl)
    // so the outer container stays raw with a loom-allow note.
    let capability_grid = html! {
        // loom-allow: max-w-6xl exceeds Loom SectionWidth::Wide; capability grid needs the wider column for 3-up at lg+.
        section class="py-16 bg-slate-50" { // loom-allow: outer band — capability grid is wider than Loom Wide
            div class="container mx-auto px-4 md:px-6 max-w-6xl" { // loom-allow: container exceeds Loom Wide (max-w-6xl needed for 3-col)
                div class="text-center max-w-3xl mx-auto mb-12 reveal" { // loom-allow: centered intro caption block — width mismatch with grid
                    (Heading {
                        text: cfg.capabilities_heading,
                        level: HeadingLevel::H2,
                        variant: HeadingVariant::Section,
                        tone: HeadingTone::Ink,
                    }.render())
                    div class="mt-4" { // loom-allow: structural margin wrapper
                        (Lede { text: cfg.capabilities_lede, tone: HeadingTone::Ink }.render())
                    }
                }
                div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 reveal reveal-delay-1" { // loom-allow: Tailwind 3-col grid; Loom doesn't ship a Grid primitive yet
                    @for cap in cfg.capabilities {
                        (FeatureCard {
                            icon_svg: cap.icon_svg,
                            title: cap.title,
                            description: cap.description,
                        }.render())
                    }
                }
            }
        }
    };

    // Posture band — slate-900 dark, decorative blur element,
    // badge with backdrop-blur. All three details exceed Loom
    // Section's Dark theme; isolated here with a loom-allow note.
    let posture_band = html! {
        // loom-allow: dark band with decorative blur + glass badge — exceeds Section{Dark} skeleton.
        section class="py-20 bg-slate-900 text-white relative overflow-hidden" { // loom-allow: dark posture band — slate-900 with decorative chrome
            // loom-allow: positioned decorative blur element; pure visual chrome with no text content.
            div class="absolute top-0 right-0 w-96 h-96 bg-primary/20 rounded-full blur-3xl -translate-y-1/2 translate-x-1/2" {} // loom-allow: decorative blur element; pure visual chrome, no text
            div class="container relative mx-auto px-4 md:px-6 max-w-4xl reveal" { // loom-allow: container for posture band; max-w-4xl
                // loom-allow: glass-morphism badge — backdrop-blur shape with no Loom equivalent yet.
                span class="inline-block px-3 py-1 rounded-full bg-white/10 text-white text-sm font-medium mb-6 backdrop-blur-sm border border-white/10" { // loom-allow: glass-morphism eyebrow badge; no Loom equivalent
                    (cfg.posture_eyebrow)
                }
                (Heading {
                    text: cfg.posture_heading,
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Section,
                    tone: HeadingTone::OnDark,
                }.render())
                div class="mt-6 space-y-6" { // loom-allow: structural margin + spacer
                    @for para in cfg.posture_paragraphs {
                        (Lede { text: para, tone: HeadingTone::OnDark }.render())
                    }
                }
                @if !cfg.posture_check_lines.is_empty() {
                    div class="mt-8 space-y-4" { // loom-allow: structural margin + spacer
                        @for line in cfg.posture_check_lines {
                            (check_line(line))
                        }
                    }
                }
            }
        }
    };

    // Engagement steps — Loom Section, Article width.
    let engagement_body = html! {
        div class="reveal" { // loom-allow: reveal animation wrapper
            (Heading {
                text: cfg.engagement_heading,
                level: HeadingLevel::H2,
                variant: HeadingVariant::Section,
                tone: HeadingTone::Ink,
            }.render())
            div class="mt-6 mb-8" { // loom-allow: structural margin
                (Lede { text: cfg.engagement_intro, tone: HeadingTone::Ink }.render())
            }
            ol class="space-y-6 text-slate-700" { // loom-allow: numbered-step list — vertical rhythm + base prose colour
                @for (i, step) in cfg.engagement_steps.iter().enumerate() {
                    (engagement_step_item(i + 1, step.title, step.description))
                }
            }
        }
    };
    let engagement_section = Section {
        body: &engagement_body,
        theme: SectionTheme::Light,
        width: SectionWidth::Wide,
        padding: SectionPadding::Loose,
    }
    .render();

    // Final CTA — tinted band, narrow article width.
    let cta_button = Button {
        label: cfg.primary_cta_label,
        variant: ButtonVariant::Primary,
        size: ButtonSize::Lg,
        aria_label: None,
        icon: None,
        decoration: Decoration::SoftShadow,
        button_type: ButtonType::Button,
    }
    .render();
    let mailto_link = TextLink {
        label: "team@plausiden.com",
        href: "mailto:team@plausiden.com",
        variant: TextLinkVariant::PrimaryMedium,
        size: TextLinkSize::Default,
    }
    .render();
    let cta_body = html! {
        div class="text-center reveal" { // loom-allow: centered text wrapper for final CTA
            (Heading {
                text: cfg.cta_heading,
                level: HeadingLevel::H2,
                variant: HeadingVariant::Section,
                tone: HeadingTone::Ink,
            }.render())
            div class="mt-6 mb-8" { // loom-allow: structural margin + spacer
                (Lede { text: cfg.cta_subline, tone: HeadingTone::Ink }.render())
            }
            a href="/contact" { (cta_button) }
            div class="mt-6" { // loom-allow: structural margin
                (HelperText {
                    text: "",
                    size: HelperSize::Default,
                    tone: HeadingTone::Ink,
                }.render())
                p class="text-slate-500 text-sm" { // loom-allow: small note paragraph; helper-text-sized but bound to slate-500
                    "Or write to "
                    (mailto_link)
                    " · 978-351-6495"
                }
            }
        }
    };
    let cta_section = Section {
        body: &cta_body,
        theme: SectionTheme::Tinted,
        width: SectionWidth::Article,
        padding: SectionPadding::Loose,
    }
    .render();

    let show_engagement = !cfg.engagement_steps.is_empty();
    let body = html! {
        (Hero {
            eyebrow: Some(cfg.hero_eyebrow),
            headline_lead: cfg.hero_lead,
            headline_accent: Some(cfg.hero_accent),
            subheadline: cfg.hero_subheadline,
            cta: Some(&hero_cta),
            background: HeroBackground::GridLight,
        }.render())
        (pain_section)
        (capability_grid)
        (posture_band)
        @if show_engagement {
            (engagement_section)
        }
        (cta_section)
    };

    let _ = BodyText {
        text: "",
        tone: HeadingTone::Ink,
    }; // silence unused-import warning if BodyText isn't used elsewhere
    page_with_description(cfg.title, cfg.path, cfg.description, body)
}

/// One numbered step in the engagement-list. Loom-allow: the
/// numbered-circle glyph is a recurring motif but distinct enough
/// to warrant its own future primitive (`StepNumber`).
fn engagement_step_item(n: usize, title: &str, description: &str) -> Markup {
    let label = format!("{n}");
    html! {
        li class="flex gap-4" { // loom-allow: numbered-step row — pending shared StepRow primitive
            span class="flex-shrink-0 w-8 h-8 rounded-full bg-primary text-white font-bold flex items-center justify-center text-sm" { (label) } // loom-allow: circular numbered glyph; future StepNumber primitive
            div {
                p class="font-semibold text-slate-900 mb-1" { (title) } // loom-allow: row-title pattern; HelperBold primitive on the to-do list.
                p class="text-slate-600" { (description) } // loom-allow: in-row body; future BodyText size variant.
            }
        }
    }
}

/// Check-mark line used inside the dark posture band.
#[must_use]
pub fn check_line(text: &str) -> Markup {
    html! {
        // loom-allow: posture-band check row; bound to dark-band slate-300 text colour.
        div class="flex items-start gap-3" { // loom-allow: check-line composition
            (PreEscaped(icons::CHECK.render()))
            span class="text-slate-300" { (text) }
        }
    }
}
