//! Blog surface — index (`/blog`) and individual post (`/blog/<slug>`).
//!
//! The blog uses the same site chrome as the marketing pages but with a
//! tighter prose container. No CMS, no markdown engine; posts are Maud
//! functions registered in [`crate::views::posts`].

use maud::{Markup, html};

use super::layout::{PageMeta, page, page_with_meta};
use super::posts::{POSTS, Post, by_slug};
use loom_components::card::LinkCard;
use loom_components::{
    Badge, BadgeSize, BadgeTone, Heading, HeadingLevel, HeadingTone, HeadingVariant, Lede,
    TextLink, TextLinkSize, TextLinkVariant,
};

/// JSON escape a string for safe embedding inside a JSON literal in the
/// Article schema. Covers the characters Maud doesn't escape inside a
/// `<script type="application/ld+json">` block (which is HTML-CDATA-like
/// at the parser level — but we still need real JSON validity).
fn json_escape(s: &str) -> String {
    use std::fmt::Write as _;
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '<' => out.push_str("\\u003c"),
            '>' => out.push_str("\\u003e"),
            '&' => out.push_str("\\u0026"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out
}

/// Render the blog index — list of published posts, newest first.
#[must_use]
pub fn index() -> Markup {
    let body = html! {
        section class="relative pt-32 pb-16 md:pt-44 md:pb-20 overflow-hidden bg-slate-50" { // loom-allow: grid-fleck hero shell — pt-32/44 cadence + fleck overlay don't fit Loom Section
            div class="absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:24px_24px]" {} // loom-allow: SVG grid fleck — decorative pattern, no Loom primitive
            div class="container relative mx-auto px-4 md:px-6 z-10 max-w-4xl" { // loom-allow: hero container max-w-4xl with z-10 fleck stacking
                div class="mb-6" { (Badge { label: "Field Notes", tone: BadgeTone::Primary, size: BadgeSize::Md }.render()) }
                div class="mb-4" {
                    (Heading {
                        text: "Notes from the build floor.",
                        level: HeadingLevel::H1,
                        variant: HeadingVariant::Display,
                        tone: HeadingTone::Ink,
                    }.render())
                }
                div class="mb-4" {
                    (Lede {
                        text: "How we think about privacy, infrastructure, and shipping. The thesis behind the work — sanitized so we can talk about it without exposing client systems.",
                        tone: HeadingTone::Ink,
                    }.render())
                }
                p class="text-sm text-slate-500" { // loom-allow: subscribe sub-line; HelperText{Default} doesn't support inline TextLink children
                    "Get new posts in your reader of choice — "
                    (TextLink { label: "subscribe instructions", href: "/subscribe", variant: TextLinkVariant::Underlined, size: TextLinkSize::Default }.render())
                    "."
                }
            }
        }

        section class="py-16 md:py-20 bg-white" { // loom-allow: posts-grid band with custom py-16/20 cadence; not Section::Default
            div class="container mx-auto px-4 md:px-6 max-w-4xl" { // loom-allow: posts-grid container max-w-4xl
                @if POSTS.is_empty() {
                    p class="text-slate-600" { // loom-allow: empty-state prose with inline TextLink
                        "No published posts yet — we're drafting. "
                        (TextLink { label: "Tell us what you'd want to read.", href: "/contact", variant: TextLinkVariant::Underlined, size: TextLinkSize::Default }.render())
                    }
                } @else {
                    div class="grid gap-8" { // loom-allow: card-stack grid spacing — no Loom Stack primitive yet
                        @for post in POSTS {
                            (post_card(post))
                        }
                    }
                }
            }
        }
    };
    page("Field Notes — PlausiDen", "/blog", body)
}

/// Render a single post page given its slug. Returns `None` if the
/// slug isn't registered — caller turns that into a 404.
#[must_use]
pub fn post(slug: &str) -> Option<Markup> {
    let post = by_slug(slug)?;
    let body = html! {
        section class="relative pt-32 pb-12 md:pt-40 md:pb-16 overflow-hidden bg-slate-50" { // loom-allow: post-hero shell — pt-32/40 cadence + grid fleck don't fit Loom Section
            div class="absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:24px_24px]" {} // loom-allow: SVG grid fleck — same pattern as index hero
            div class="container relative mx-auto px-4 md:px-6 z-10 max-w-3xl" { // loom-allow: post-hero container max-w-3xl with fleck stacking
                (TextLink {
                    label: "← Field Notes",
                    href: "/blog",
                    variant: TextLinkVariant::Subtle,
                    size: TextLinkSize::Small,
                }.render())
                div class="mt-6" {
                    (Badge { label: post.category, tone: BadgeTone::Primary, size: BadgeSize::Sm }.render())
                    div class="mt-4 mb-4" { // loom-allow: spacing wrapper between Badge eyebrow and Heading
                        (Heading {
                            text: post.title,
                            level: HeadingLevel::H1,
                            variant: HeadingVariant::Display,
                            tone: HeadingTone::Ink,
                        }.render())
                    }
                    p class="text-sm text-slate-500" { // loom-allow: post meta line — date · read-time, no Loom Meta primitive
                        (post.published) " · " (post.read_time)
                    }
                }
            }
        }

        article class="py-12 md:py-16 bg-white" { // loom-allow: post-body article shell — py-12 cadence + prose container scope, not Loom Section
            div class="container mx-auto px-4 md:px-6" { // loom-allow: container chrome wrapping prose
                div class="prose prose-slate max-w-2xl mx-auto leading-relaxed text-slate-700" { // loom-allow: long-form prose container — Tailwind typography plugin scope, no Loom equivalent
                    ((post.render)())
                }
                div class="max-w-2xl mx-auto mt-16 pt-8 border-t border-slate-200" { // loom-allow: post-footer divider chrome
                    p class="text-slate-600" { // loom-allow: post-footer prose with inline TextLink
                        "Working on something where this kind of thinking matters? "
                        (TextLink { label: "Get in touch.", href: "/contact", variant: TextLinkVariant::Underlined, size: TextLinkSize::Default }.render())
                    }
                }
            }
        }
    };
    let title = format!("{} — PlausiDen", post.title);
    let current = format!("/blog/{}", post.slug);
    let og_image = format!("/og/blog/{}.svg", post.slug);
    let article_jsonld = format!(
        r#"{{"@context":"https://schema.org","@type":"Article","headline":"{title}","description":"{desc}","datePublished":"{date}","mainEntityOfPage":"https://plausiden.com{path}","image":"https://plausiden.com{img}","author":{{"@type":"Organization","name":"PlausiDen LLC","url":"https://plausiden.com"}},"publisher":{{"@type":"Organization","name":"PlausiDen LLC","url":"https://plausiden.com","logo":{{"@type":"ImageObject","url":"https://plausiden.com/static/favicon-96x96.png"}}}}}}"#,
        title = json_escape(post.title),
        desc = json_escape(post.excerpt),
        date = post.published,
        path = current,
        img = og_image,
    );
    Some(page_with_meta(
        &PageMeta {
            title: &title,
            current: &current,
            description: post.excerpt,
            og_image: Some(&og_image),
            og_type: "article",
            extra_json_ld: &article_jsonld,
        },
        body,
    ))
}

fn post_card(post: &Post) -> Markup {
    let href = format!("/blog/{}", post.slug);
    let body = html! {
        div class="flex items-center gap-3 text-xs uppercase tracking-wider font-semibold text-slate-500" { // loom-allow: card meta-row chrome — eyebrow + dot-separated dates with stronger letter-spacing for "label" feel
            (Badge { label: post.category, tone: BadgeTone::Primary, size: BadgeSize::Sm }.render())
            span class="font-medium tracking-normal normal-case text-slate-500" { (post.published) } // loom-allow: nested meta date — overrides parent uppercase chrome
            span class="text-slate-300" { "·" } // loom-allow: meta dot separator
            span class="font-medium tracking-normal normal-case text-slate-500" { (post.read_time) } // loom-allow: nested read-time — overrides parent uppercase chrome
        }
        h2 class="font-display text-2xl md:text-3xl lg:text-[2rem] font-bold text-slate-900 mt-4 leading-[1.2] tracking-tight group-hover:text-primary transition-colors duration-200" { // loom-allow: card-headline with tighter tracking + lg breakpoint scaling; Heading{Sub} omits group-hover hook for hover-state coupling with surrounding LinkCard
            (post.title)
        }
        p class="text-slate-600 mt-4 leading-relaxed text-[15px] md:text-base font-light" { // loom-allow: card excerpt — light-weight prose for typographic contrast against bold headline; Lede is for hero openers
            (post.excerpt)
        }
        div class="mt-5 flex items-center gap-2 text-primary font-semibold text-sm" { // loom-allow: read-more affordance with arrow that animates on hover
            span { "Read more" }
            span class="inline-block transition-transform duration-300 group-hover:translate-x-1" { "→" } // loom-allow: arrow translate-on-hover micro-interaction
        }
    };
    html! {
        div class="reveal group relative transition-all duration-300 hover:-translate-y-1" { // loom-allow: post-card wrapper — vertical-lift + shadow-grow hover + accent-stripe; scroll-fade reveal layered on top
            // Decorative accent stripe — invisible by default, animates in on hover from left edge, ties the hover state to the brand color.
            div class="absolute left-0 top-6 bottom-6 w-1 bg-primary rounded-full origin-top scale-y-0 group-hover:scale-y-100 transition-transform duration-300 ease-out" {} // loom-allow: positioned accent stripe — animates scaleY 0→1 on parent group hover
            (LinkCard { href: &href, body: &body }.render())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_lists_known_post() {
        let s = index().into_string();
        assert!(s.contains("Field Notes"));
        assert!(s.contains("Federated rule learning"));
        // Slug-derived URL on the card
        assert!(s.contains("/blog/federated-rule-learning"));
    }

    #[test]
    fn post_renders_title_and_body() {
        let m = post("federated-rule-learning").expect("known slug");
        let s = m.into_string();
        assert!(s.contains("Federated rule learning"));
        // Returns to index breadcrumb
        assert!(s.contains(r#"href="/blog""#));
        // CTA at the bottom
        assert!(s.contains("/contact"));
    }

    #[test]
    fn post_returns_none_for_unknown_slug() {
        assert!(post("nope").is_none());
    }

    #[test]
    fn post_uses_full_title_in_page_title() {
        let m = post("federated-rule-learning").expect("known slug");
        let s = m.into_string();
        assert!(s.contains("<title>Federated rule learning"));
    }
}
