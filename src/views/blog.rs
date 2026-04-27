//! Blog surface — index (`/blog`) and individual post (`/blog/<slug>`).
//!
//! The blog uses the same site chrome as the marketing pages but with a
//! tighter prose container. No CMS, no markdown engine; posts are Maud
//! functions registered in [`crate::views::posts`].

use maud::{Markup, html};

use super::layout::{PageMeta, page, page_with_meta};
use super::posts::{POSTS, Post, by_slug};
use loom_components::card::LinkCard;

/// JSON escape a string for safe embedding inside a JSON literal in the
/// Article schema. Covers the characters Maud doesn't escape inside a
/// `<script type="application/ld+json">` block (which is HTML-CDATA-like
/// at the parser level — but we still need real JSON validity).
fn json_escape(s: &str) -> String {
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
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

/// Render the blog index — list of published posts, newest first.
#[must_use]
pub fn index() -> Markup {
    let body = html! {
        section class="relative pt-32 pb-16 md:pt-44 md:pb-20 overflow-hidden bg-slate-50" {
            div class="absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:24px_24px]" {}
            div class="container relative mx-auto px-4 md:px-6 z-10 max-w-4xl" {
                span class="inline-block px-4 py-1.5 rounded-full bg-primary/10 text-primary font-semibold text-sm mb-6 border border-primary/20" { "Field Notes" }
                h1 class="font-display text-4xl md:text-5xl lg:text-6xl font-bold text-slate-900 leading-[1.1] mb-4" {
                    "Notes from the build floor."
                }
                p class="text-lg text-slate-600 max-w-2xl leading-relaxed mb-4" {
                    "How we think about privacy, infrastructure, and shipping. The thesis behind the work — sanitized so we can talk about it without exposing client systems."
                }
                p class="text-sm text-slate-500" {
                    "Get new posts in your reader of choice — "
                    a href="/subscribe" class="text-primary font-semibold underline" { "subscribe instructions" }
                    "."
                }
            }
        }

        section class="py-16 md:py-20 bg-white" {
            div class="container mx-auto px-4 md:px-6 max-w-4xl" {
                @if POSTS.is_empty() {
                    p class="text-slate-600" {
                        "No published posts yet — we're drafting. "
                        a href="/contact" class="text-primary font-semibold underline" { "Tell us what you'd want to read." }
                    }
                } @else {
                    div class="grid gap-8" {
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
        section class="relative pt-32 pb-12 md:pt-40 md:pb-16 overflow-hidden bg-slate-50" {
            div class="absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:24px_24px]" {}
            div class="container relative mx-auto px-4 md:px-6 z-10 max-w-3xl" {
                a href="/blog" class="text-sm text-slate-500 hover:text-primary transition-colors" {
                    "← Field Notes"
                }
                div class="mt-6" {
                    span class="inline-block px-3 py-1 rounded-full bg-primary/10 text-primary font-semibold text-xs border border-primary/20" {
                        (post.category)
                    }
                    h1 class="font-display text-4xl md:text-5xl font-bold text-slate-900 leading-[1.1] mt-4 mb-4" {
                        (post.title)
                    }
                    p class="text-sm text-slate-500" {
                        (post.published) " · " (post.read_time)
                    }
                }
            }
        }

        article class="py-12 md:py-16 bg-white" {
            div class="container mx-auto px-4 md:px-6" {
                div class="prose prose-slate max-w-2xl mx-auto leading-relaxed text-slate-700" {
                    ((post.render)())
                }
                div class="max-w-2xl mx-auto mt-16 pt-8 border-t border-slate-200" {
                    p class="text-slate-600" {
                        "Working on something where this kind of thinking matters? "
                        a href="/contact" class="text-primary font-semibold underline" { "Get in touch." }
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
        div class="flex items-center gap-3 text-xs text-slate-500" {
            span class="inline-block px-2.5 py-0.5 rounded-full bg-primary/10 text-primary font-semibold border border-primary/20" {
                (post.category)
            }
            span { (post.published) }
            span class="text-slate-300" { "·" }
            span { (post.read_time) }
        }
        h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mt-3 group-hover:text-primary transition-colors" {
            (post.title)
        }
        p class="text-slate-600 mt-3 leading-relaxed" {
            (post.excerpt)
        }
        p class="mt-4 text-primary font-semibold text-sm" {
            "Read more →"
        }
    };
    html! {
        div class="reveal" {
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
