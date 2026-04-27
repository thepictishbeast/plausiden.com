//! Blog post registry.
//!
//! Each post is a sibling module that exports a `render() -> Markup`
//! returning just the body (no page chrome). The static [`POSTS`]
//! slice declares metadata + the render fn pointer.
//!
//! BUG ASSUMPTION: `slug` values are URL-safe (lowercase, dashes only),
//! match the file's module name 1:1, and are unique across the slice.
//! The `slug_uniqueness` test guards uniqueness; URL-safety is by
//! convention (caught at PR review).
//!
//! How to add a post:
//!   1. Create `src/views/posts/<slug>.rs` with `pub fn render() -> Markup`.
//!   2. Add `pub mod <slug>;` below.
//!   3. Add a [`Post`] entry to [`POSTS`] in chronological order
//!      (newest first — the index renders in slice order).

use maud::Markup;

pub mod avp_doctrine;
pub mod federated_learning;
pub mod plausible_deniability;
pub mod provable_privacy;
pub mod why_thundercrab;

/// Metadata + render pointer for one published post.
#[derive(Debug, Clone, Copy)]
pub struct Post {
    /// URL slug. The post lives at `/blog/<slug>`.
    pub slug: &'static str,
    /// Display title.
    pub title: &'static str,
    /// One-paragraph hook shown on the index card. ~150-220 chars.
    pub excerpt: &'static str,
    /// Eyebrow tag — single category like "Engineering", "Privacy",
    /// or "Architecture". Drives the small pill above the title.
    pub category: &'static str,
    /// ISO date `YYYY-MM-DD`. Sorted display only, not used for routing.
    pub published: &'static str,
    /// Approximate read time, e.g., `"8 min read"`.
    pub read_time: &'static str,
    /// Renders the post body (the prose content; chrome is wrapped
    /// around it by the blog view).
    pub render: fn() -> Markup,
}

/// All published posts. Add the newest at the top.
///
/// SECURITY: This slice is the *only* surface that wires post metadata
/// to its renderer. A typo'd `render` pointer would fail the compile;
/// a typo'd slug would either 404 (caller-side) or collide (caught by
/// [`tests::slug_uniqueness`]).
pub const POSTS: &[Post] = &[
    Post {
        slug: "plausible-deniability",
        title: "What plausible deniability means in our architecture",
        excerpt: "We named the company after a phrase. Most privacy products sell hiding; we sell unreliability. What that buys, where it lands, and the bright lines we won't cross to ship it.",
        category: "Architecture",
        published: "2026-04-27",
        read_time: "11 min read",
        render: plausible_deniability::render,
    },
    Post {
        slug: "why-thundercrab",
        title: "Why we're building Thundercrab",
        excerpt: "Modern consumer mail clients replaced an open, auditable filtering standard with opaque ML categorization that users can't see, edit, or carry between providers. We're building the local-first GUI for transparent mail rules — and writing about why nobody else has.",
        category: "Engineering",
        published: "2026-04-27",
        read_time: "10 min read",
        render: why_thundercrab::render,
    },
    Post {
        slug: "provable-privacy",
        title: "Provable-by-construction privacy",
        excerpt: "Two ways to promise to protect data: a policy you trust, or a property the type system enforces. We try to pick the second whenever the engineering allows. What that changes about audits, incident response, and the trade-offs people accept.",
        category: "Architecture",
        published: "2026-04-27",
        read_time: "9 min read",
        render: provable_privacy::render,
    },
    Post {
        slug: "avp-doctrine",
        title: "How a written-down doctrine changes what \"shipping\" means",
        excerpt: "We wrote our taste down. Every public function carries a BUG ASSUMPTION annotation; every defense-in-depth carries a SECURITY annotation; CI enforces both. Why we think this is the right pace.",
        category: "Operating Model",
        published: "2026-04-27",
        read_time: "7 min read",
        render: avp_doctrine::render,
    },
    Post {
        slug: "federated-rule-learning",
        title: "Federated rule learning, without ever reading your mail",
        excerpt: "How sorting rules can get smarter from the collective without any provider — including us — seeing message content. A note on what we built and why we think it matters.",
        category: "Architecture",
        published: "2026-04-26",
        read_time: "8 min read",
        render: federated_learning::render,
    },
];

/// Look up a post by slug. Returns `None` if not registered.
#[must_use]
pub fn by_slug(slug: &str) -> Option<&'static Post> {
    POSTS.iter().find(|p| p.slug == slug)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn slug_uniqueness() {
        let mut seen = HashSet::new();
        for post in POSTS {
            assert!(seen.insert(post.slug), "duplicate slug: {}", post.slug);
        }
    }

    #[test]
    fn slugs_are_url_safe() {
        for post in POSTS {
            assert!(
                post.slug
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'),
                "slug not URL-safe: {}",
                post.slug
            );
            assert!(!post.slug.starts_with('-'));
            assert!(!post.slug.ends_with('-'));
        }
    }

    #[test]
    fn dates_are_iso() {
        for post in POSTS {
            assert_eq!(
                post.published.len(),
                10,
                "published not YYYY-MM-DD: {}",
                post.published
            );
            assert_eq!(&post.published[4..5], "-");
            assert_eq!(&post.published[7..8], "-");
        }
    }

    #[test]
    fn every_post_renders_nonempty() {
        for post in POSTS {
            let body = (post.render)().into_string();
            assert!(
                body.len() > 500,
                "post {} body unexpectedly short: {} bytes",
                post.slug,
                body.len()
            );
        }
    }

    #[test]
    fn by_slug_finds_known_and_misses_unknown() {
        assert!(by_slug("federated-rule-learning").is_some());
        assert!(by_slug("does-not-exist").is_none());
    }
}
