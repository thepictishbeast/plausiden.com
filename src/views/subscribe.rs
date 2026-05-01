//! `/subscribe` — non-technical instructions for following Field Notes
//! via RSS / Atom.
//!
//! The bare `/blog/rss.xml` link rendered as XML in most browsers,
//! which reads as "broken" to a non-technical reader. This page
//! explains what RSS is, recommends a few mainstream readers, and
//! gives the feed URL prominently so anyone who already knows the
//! drill can copy it.

use loom_components::{
    Badge, BadgeSize, BadgeTone, Button, ButtonSize, ButtonType, ButtonVariant, Decoration,
    Heading, HeadingLevel, HeadingTone, HeadingVariant, Lede, Section, SectionPadding,
    SectionTheme, SectionWidth, TextLink, TextLinkSize, TextLinkVariant,
};
use maud::{Markup, html};

use crate::views::layout::page;

/// One reader recommendation.
struct Reader {
    /// Display name.
    name: &'static str,
    /// Short pitch — what kind of user this fits.
    pitch: &'static str,
    /// Platforms it runs on.
    platforms: &'static str,
    /// Cost framing — kept abstract because plans change.
    cost: &'static str,
}

const READERS: &[Reader] = &[
    Reader {
        name: "NetNewsWire",
        pitch: "Open-source. Clean, simple, no account required.",
        platforms: "Mac, iPhone, iPad",
        cost: "Free",
    },
    Reader {
        name: "Feedly",
        pitch: "Web-based; works in any browser; no install.",
        platforms: "Web, iOS, Android",
        cost: "Free tier covers a few feeds; paid tier for more",
    },
    Reader {
        name: "Inoreader",
        pitch: "Web-based; richer organization than Feedly's free tier.",
        platforms: "Web, iOS, Android",
        cost: "Free tier; paid for advanced features",
    },
    Reader {
        name: "Reeder",
        pitch: "Polished native app on Apple platforms.",
        platforms: "Mac, iPhone, iPad",
        cost: "One-time purchase",
    },
    Reader {
        name: "Thunderbird",
        pitch: "If you're already using it for email, it reads RSS too.",
        platforms: "Mac, Windows, Linux",
        cost: "Free, open-source",
    },
];

/// Render `/subscribe`.
#[must_use]
pub fn render() -> Markup {
    // Custom hero band — eyebrow pill + h1 + lede sit inside a
    // grid-flecked slate-50 surface that diverges from Loom Hero
    // (which expects a two-tone two-line headline + CTA pair).
    // Migrate the inner typography to Loom; leave the hero shell
    // raw with annotations.
    let hero_band = html! {
        section class="relative pt-32 pb-16 md:pt-44 md:pb-24 bg-slate-50 overflow-hidden" { // loom-allow: custom hero shell — pt-32/44 cadence below Loom Section padding scale
            div class="container relative mx-auto px-4 md:px-6 z-10 max-w-3xl" { // loom-allow: hero container max-w-3xl with z-10 stacking
                // loom-allow: eyebrow pill — recurring badge pattern; future Badge::Eyebrow primitive.
                div class="mb-6" { (Badge { label: "Following Field Notes", tone: BadgeTone::Primary, size: BadgeSize::Md }.render()) }
                div class="mb-4" {
                    (Heading {
                        text: "Subscribe.",
                        level: HeadingLevel::H1,
                        variant: HeadingVariant::Display,
                        tone: HeadingTone::Ink,
                    }.render())
                }
                (Lede {
                    text: "Field Notes ships as an RSS feed — a small file your computer or phone checks for new posts so you don't have to. There's no account, no email signup, no analytics tracking who's reading what. Pick a reader app, paste in the feed link, done.",
                    tone: HeadingTone::Ink,
                }.render())
            }
        }
    };

    // The four content sections share the same container shape
    // (`max-w-3xl`, py-16). Compose each with Loom Section::Article.
    let feed_link_body = html! {
        div class="mb-4" {
            (Heading {
                text: "The feed link",
                level: HeadingLevel::H2,
                variant: HeadingVariant::Sub,
                tone: HeadingTone::Ink,
            }.render())
        }
        div class="mb-4" {
            (Lede {
                text: "Copy this URL and paste it into your reader of choice:",
                tone: HeadingTone::Ink,
            }.render())
        }
        div class="rounded-lg border border-slate-200 bg-slate-50 p-4 font-mono text-sm break-all select-all" { // loom-allow: copyable mono URL block — pending CodeBlock primitive
            "https://plausiden.com/blog/rss.xml"
        }
        p class="text-sm text-slate-500 mt-3" { // loom-allow: helper note pattern; HelperText{Default} omits mt-3
            "If you click it directly, your browser will probably show raw XML — that's normal. Browsers don't render feeds; reader apps do."
        }
    };
    let feed_link_section = Section {
        body: &feed_link_body,
        theme: SectionTheme::Light,
        width: SectionWidth::Article,
        padding: SectionPadding::Default,
    }
    .render();

    let readers_body = html! {
        div class="mb-4" {
            (Heading {
                text: "If you don't already use a reader",
                level: HeadingLevel::H2,
                variant: HeadingVariant::Sub,
                tone: HeadingTone::Ink,
            }.render())
        }
        div class="mb-8" {
            (Lede {
                text: "Any of these will work. Pick one that fits the platforms you actually use; they all do the same fundamental thing — pull the feed periodically and show you new posts.",
                tone: HeadingTone::Ink,
            }.render())
        }
        div class="space-y-4" { // loom-allow: vertical rhythm between reader cards
            @for r in READERS {
                div class="rounded-lg border border-slate-200 bg-white p-5" { // loom-allow: reader-card shell — pending ReaderCard primitive when shape recurs
                    div class="flex items-baseline justify-between gap-4 mb-2" { // loom-allow: card meta-row — title left, cost right
                        h3 class="font-display text-lg font-bold text-slate-900" { (r.name) } // loom-allow: card-title pattern
                        span class="text-xs text-slate-500" { (r.cost) } // loom-allow: micro-meta label
                    }
                    p class="text-sm text-slate-600 leading-relaxed mb-1" { (r.pitch) } // loom-allow: card-body small
                    p class="text-xs text-slate-500" { // loom-allow: micro-meta line
                        strong { "Runs on: " } (r.platforms)
                    }
                }
            }
        }
        p class="text-sm text-slate-500 mt-6" { // loom-allow: footer-note pattern
            "Not endorsements — these are mainstream readers we've seen work. Any RSS or Atom-compatible reader will read the feed; the format is a published standard, not a vendor-specific protocol."
        }
    };
    let readers_section = Section {
        body: &readers_body,
        theme: SectionTheme::Muted,
        width: SectionWidth::Article,
        padding: SectionPadding::Default,
    }
    .render();

    let how_to_body = html! {
        div class="mb-4" {
            (Heading {
                text: "How to add the feed (most readers)",
                level: HeadingLevel::H2,
                variant: HeadingVariant::Sub,
                tone: HeadingTone::Ink,
            }.render())
        }
        ol class="space-y-3 text-slate-600 leading-relaxed list-decimal pl-6" { // loom-allow: numbered prose list; future BodyList primitive
            li { "Install one of the readers above." }
            li { "Open it. Find the option labeled \"Add feed,\" \"Add subscription,\" \"+\" or similar." }
            li {
                "Paste in: "
                code class="text-primary font-mono text-sm" { "https://plausiden.com/blog/rss.xml" } // loom-allow: inline code; future InlineCode primitive
            }
            li { "The reader will fetch the feed and show the existing posts. New posts will appear automatically as they ship." }
        }
        p class="text-sm text-slate-500 mt-6" { // loom-allow: footer-note pattern
            "We don't see who subscribed, when they fetched the feed, or which posts they read. The feed is a static file; readers fetch it directly. No tracking pixels, no per-reader URLs, nothing logged on our end beyond a generic web hit."
        }
    };
    let how_to_section = Section {
        body: &how_to_body,
        theme: SectionTheme::Light,
        width: SectionWidth::Article,
        padding: SectionPadding::Default,
    }
    .render();

    let cta_button = Button {
        label: "See the latest posts",
        variant: ButtonVariant::Primary,
        size: ButtonSize::Lg,
        aria_label: None,
        icon: None,
        decoration: Decoration::SoftShadow,
        button_type: ButtonType::Button,
    }
    .render();
    let cta_body = html! {
        div class="text-center" {
            div class="mb-4" {
                (Heading {
                    text: "Or just bookmark the page",
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Sub,
                    tone: HeadingTone::Ink,
                }.render())
            }
            div class="mb-6" {
                (Lede {
                    text: "",
                    tone: HeadingTone::Ink,
                }.render())
                p class="text-slate-600 leading-relaxed" { // loom-allow: prose with inline link; standard BodyText doesn't support inline children
                    "If a reader app sounds like more setup than it's worth, bookmark "
                    (TextLink { label: "the Field Notes index", href: "/blog", variant: TextLinkVariant::Underlined, size: TextLinkSize::Default }.render())
                    " and check back when the mood strikes. New posts go up at the top."
                }
            }
            a href="/blog" { (cta_button) }
        }
    };
    let cta_section = Section {
        body: &cta_body,
        theme: SectionTheme::Tinted,
        width: SectionWidth::Narrow,
        padding: SectionPadding::Default,
    }
    .render();

    let body = html! {
        (hero_band)
        (feed_link_section)
        (readers_section)
        (how_to_section)
        (cta_section)
    };
    page("Subscribe — PlausiDen", "/subscribe", body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_feed_url_prominently() {
        let s = render().into_string();
        // The literal feed URL must appear at least twice (once in the
        // copyable code block, once in the step-by-step list).
        assert!(s.matches("https://plausiden.com/blog/rss.xml").count() >= 2);
    }

    #[test]
    fn lists_every_reader() {
        let s = render().into_string();
        for r in READERS {
            assert!(s.contains(r.name), "missing reader: {}", r.name);
        }
    }

    #[test]
    fn explains_no_tracking() {
        let s = render().into_string();
        assert!(s.contains("no analytics"));
        assert!(s.contains("No tracking pixels"));
    }
}
