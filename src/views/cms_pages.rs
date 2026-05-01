//! Renderer for `cms_core::Page` → Maud `Markup`.
//!
//! The CMS substrate stores pages as TOML on disk (see
//! `cms-core::storage::FsStorage`); this module is the read-side
//! adapter that walks a [`Page`]'s sections and blocks and emits
//! Loom-composed HTML. The rendering surface is intentionally
//! restricted to the `BlockKind` enum — adding a new kind requires
//! both a `BlockKind` variant in `cms-core` and a match arm here,
//! so a typo in a page TOML can't ship as raw HTML.
//!
//! SECURITY: Every interpolation goes through Maud's escape pass.
//! There is no `PreEscaped` here — TOML field values are never
//! treated as HTML, even for the Markdown block (which still
//! routes through a paragraph-by-paragraph escape; rich Markdown
//! support is deferred until a vetted parser is absorbed under the
//! FOSS-absorption protocol).

use cms_core::page::{Block, BlockKind, FieldValue, Page, Section, SectionTheme};
use loom_components::{
    Heading, HeadingLevel, HeadingTone, HeadingVariant, Lede, Section as LoomSection,
    SectionPadding, SectionTheme as LoomSectionTheme, SectionWidth,
};
use maud::{Markup, html};

use super::layout::page;

/// Render a single CMS [`Page`] into the shared site shell.
#[must_use]
pub fn render(p: &Page, current_path: &str) -> Markup {
    let body = html! {
        @for section in &p.sections {
            (render_section(section))
        }
    };
    let title = format!("{} — PlausiDen", p.title);
    page(&title, current_path, body)
}

/// Render one [`Section`] band. Composes through `loom_components::Section`
/// so the band chrome (theme, max-width, padding) stays inside the design
/// system.
fn render_section(s: &Section) -> Markup {
    let inner = html! {
        @if let Some(anchor) = &s.anchor {
            // SECURITY: anchor IDs come from operator-edited TOML;
            // Maud escapes the attribute value, and the slug shape
            // is validated upstream at write time.
            div id=(anchor) {}
        }
        @for block in &s.blocks {
            (render_block(block))
        }
    };
    LoomSection {
        body: &inner,
        theme: map_theme(s.theme),
        width: SectionWidth::Default,
        padding: SectionPadding::Default,
    }
    .render()
}

/// Render one [`Block`]. Each `BlockKind` maps to a Loom primitive
/// composition; missing fields render as empty (defensive — bad
/// TOML shouldn't 500 the page).
fn render_block(b: &Block) -> Markup {
    match b.kind {
        BlockKind::Hero => render_hero(b),
        BlockKind::HeadingBody => render_heading_body(b),
        BlockKind::PullQuote => render_pull_quote(b),
        BlockKind::Cta => render_cta(b),
        BlockKind::Markdown => render_markdown(b),
        // Card grid, image, video are accepted in the schema but
        // their renderers are deferred until the corresponding
        // Loom primitives have a stable surface. They render as
        // a placeholder note so an editor sees that the block
        // exists but isn't yet visible.
        BlockKind::CardGrid | BlockKind::Image | BlockKind::Video => render_placeholder(b),
    }
}

fn render_hero(b: &Block) -> Markup {
    let headline = text_field(b, "headline").unwrap_or("");
    let lede = text_field(b, "lede").unwrap_or("");
    html! {
        div class="mb-4" {
            (Heading {
                text: headline,
                level: HeadingLevel::H1,
                variant: HeadingVariant::Display,
                tone: HeadingTone::Ink,
            }.render())
        }
        @if !lede.is_empty() {
            (Lede { text: lede, tone: HeadingTone::Ink }.render())
        }
    }
}

fn render_heading_body(b: &Block) -> Markup {
    let heading = text_field(b, "heading").unwrap_or("");
    let body = text_field(b, "body").unwrap_or("");
    html! {
        div class="mb-4" {
            (Heading {
                text: heading,
                level: HeadingLevel::H2,
                variant: HeadingVariant::Sub,
                tone: HeadingTone::Ink,
            }.render())
        }
        @for paragraph in body.split("\n\n").filter(|p| !p.trim().is_empty()) {
            p class="text-slate-600 leading-relaxed mb-4" { (paragraph) } // loom-allow: CMS prose; Lede is for hero openers, BodyText lacks a paragraph variant
        }
    }
}

fn render_pull_quote(b: &Block) -> Markup {
    let quote = text_field(b, "quote").unwrap_or("");
    let attribution = text_field(b, "attribution").unwrap_or("");
    html! {
        blockquote class="border-l-4 border-primary pl-6 py-2 my-6" { // loom-allow: pull-quote shell — promote to Loom PullQuote when a second consumer lands
            p class="text-xl font-display text-slate-900 italic leading-relaxed" { (quote) } // loom-allow: pull-quote body — same primitive promotion
            @if !attribution.is_empty() {
                cite class="block text-sm text-slate-500 mt-3 not-italic" { "— " (attribution) } // loom-allow: pull-quote attribution — same primitive promotion
            }
        }
    }
}

fn render_cta(b: &Block) -> Markup {
    let heading = text_field(b, "heading").unwrap_or("");
    let body = text_field(b, "body").unwrap_or("");
    let label = text_field(b, "label").unwrap_or("Continue");
    let href = text_field(b, "href").unwrap_or("/contact");
    html! {
        div class="text-center" {
            div class="mb-4" {
                (Heading {
                    text: heading,
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Sub,
                    tone: HeadingTone::Ink,
                }.render())
            }
            @if !body.is_empty() {
                div class="mb-6" {
                    (Lede { text: body, tone: HeadingTone::Ink }.render())
                }
            }
            a href=(href) class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium bg-primary text-primary-foreground border border-primary-border min-h-10 px-8 py-6 rounded-xl text-lg shadow-xl shadow-primary/20 hover:-translate-y-0.5 transition-all" { // loom-allow: CMS CTA link — button-shaped <a>; future Loom LinkButton lands here
                (label)
            }
        }
    }
}

fn render_markdown(b: &Block) -> Markup {
    let body = text_field(b, "body").unwrap_or("");
    html! {
        @for paragraph in body.split("\n\n").filter(|p| !p.trim().is_empty()) {
            p class="text-slate-600 leading-relaxed mb-4" { (paragraph) } // loom-allow: CMS-rendered Markdown paragraph — same rationale as render_heading_body
        }
    }
}

fn render_placeholder(b: &Block) -> Markup {
    let kind = match b.kind {
        BlockKind::CardGrid => "card grid",
        BlockKind::Image => "image",
        BlockKind::Video => "video",
        _ => "block",
    };
    html! {
        div class="rounded-lg border border-amber-200 bg-amber-50 p-4 my-4 text-sm text-amber-900" { // loom-allow: editor-facing placeholder; rejected at publish time by planned audit
            "[" (kind) " block — renderer pending]"
        }
    }
}

fn map_theme(t: SectionTheme) -> LoomSectionTheme {
    match t {
        SectionTheme::Light => LoomSectionTheme::Light,
        SectionTheme::Muted => LoomSectionTheme::Muted,
        SectionTheme::Dark => LoomSectionTheme::Dark,
        SectionTheme::Tinted => LoomSectionTheme::Tinted,
    }
}

fn text_field<'a>(b: &'a Block, key: &str) -> Option<&'a str> {
    match b.fields.get(key)? {
        FieldValue::Text(s) | FieldValue::Url(s) => Some(s.as_str()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cms_core::page::{Page, Section};
    use std::collections::BTreeMap;

    fn page_with_blocks(sections: Vec<Section>) -> Page {
        let mut p = Page::draft("test", "Test Page");
        p.sections = sections;
        p
    }

    fn block(kind: BlockKind, fields: &[(&str, &str)]) -> Block {
        let mut map = BTreeMap::new();
        for (k, v) in fields {
            map.insert((*k).to_string(), FieldValue::Text((*v).to_string()));
        }
        Block { kind, fields: map }
    }

    #[test]
    fn renders_hero_block() {
        let p = page_with_blocks(vec![Section {
            anchor: None,
            theme: SectionTheme::Light,
            blocks: vec![block(
                BlockKind::Hero,
                &[
                    ("headline", "Welcome to PlausiDen."),
                    ("lede", "We build privacy infrastructure."),
                ],
            )],
        }]);
        let s = render(&p, "/docs/test").into_string();
        assert!(s.contains("Welcome to PlausiDen."));
        assert!(s.contains("We build privacy infrastructure."));
    }

    #[test]
    fn renders_heading_body_block_with_paragraph_split() {
        let p = page_with_blocks(vec![Section {
            anchor: None,
            theme: SectionTheme::Light,
            blocks: vec![block(
                BlockKind::HeadingBody,
                &[
                    ("heading", "Why PPS"),
                    ("body", "First paragraph.\n\nSecond paragraph."),
                ],
            )],
        }]);
        let s = render(&p, "/docs/test").into_string();
        assert!(s.contains("Why PPS"));
        assert!(s.contains("First paragraph."));
        assert!(s.contains("Second paragraph."));
    }

    #[test]
    fn renders_pull_quote_with_attribution() {
        let p = page_with_blocks(vec![Section {
            anchor: None,
            theme: SectionTheme::Muted,
            blocks: vec![block(
                BlockKind::PullQuote,
                &[
                    ("quote", "Trust nothing."),
                    ("attribution", "AVP-2"),
                ],
            )],
        }]);
        let s = render(&p, "/docs/test").into_string();
        assert!(s.contains("Trust nothing."));
        assert!(s.contains("AVP-2"));
    }

    #[test]
    fn renders_cta_block_with_link() {
        let p = page_with_blocks(vec![Section {
            anchor: None,
            theme: SectionTheme::Tinted,
            blocks: vec![block(
                BlockKind::Cta,
                &[
                    ("heading", "Ready?"),
                    ("body", "Reach out."),
                    ("label", "Contact us"),
                    ("href", "/contact"),
                ],
            )],
        }]);
        let s = render(&p, "/docs/test").into_string();
        assert!(s.contains("Ready?"));
        assert!(s.contains("Contact us"));
        assert!(s.contains("href=\"/contact\""));
    }

    #[test]
    fn empty_fields_render_without_panic() {
        let p = page_with_blocks(vec![Section {
            anchor: None,
            theme: SectionTheme::Light,
            blocks: vec![block(BlockKind::HeadingBody, &[])],
        }]);
        let s = render(&p, "/docs/test").into_string();
        // The page chrome should still render even when fields are missing.
        assert!(s.contains("Test Page"));
    }

    #[test]
    fn anchor_emits_id_target() {
        let p = page_with_blocks(vec![Section {
            anchor: Some("intro".into()),
            theme: SectionTheme::Light,
            blocks: vec![block(BlockKind::HeadingBody, &[("heading", "Intro")])],
        }]);
        let s = render(&p, "/docs/test").into_string();
        assert!(s.contains("id=\"intro\""));
    }

    #[test]
    fn unrendered_kinds_show_placeholder() {
        let p = page_with_blocks(vec![Section {
            anchor: None,
            theme: SectionTheme::Light,
            blocks: vec![block(BlockKind::CardGrid, &[])],
        }]);
        let s = render(&p, "/docs/test").into_string();
        assert!(s.contains("card grid"));
        assert!(s.contains("renderer pending"));
    }

    #[test]
    fn html_in_field_is_escaped() {
        // SECURITY: Operator-edited content must not become live HTML.
        let p = page_with_blocks(vec![Section {
            anchor: None,
            theme: SectionTheme::Light,
            blocks: vec![block(
                BlockKind::HeadingBody,
                &[("heading", "<script>alert(1)</script>"), ("body", "<img onerror=x>")],
            )],
        }]);
        let s = render(&p, "/docs/test").into_string();
        assert!(!s.contains("<script>alert(1)</script>"));
        assert!(s.contains("&lt;script&gt;"));
    }
}
