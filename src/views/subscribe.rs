//! `/subscribe` — non-technical instructions for following Field Notes
//! via RSS / Atom.
//!
//! The bare `/blog/rss.xml` link rendered as XML in most browsers,
//! which reads as "broken" to a non-technical reader. This page
//! explains what RSS is, recommends a few mainstream readers, and
//! gives the feed URL prominently so anyone who already knows the
//! drill can copy it.

use loom_components::{TextLink, TextLinkSize, TextLinkVariant};
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
    let body = html! {
        section class="relative pt-32 pb-16 md:pt-44 md:pb-24 bg-slate-50 overflow-hidden" {
            div class="container relative mx-auto px-4 md:px-6 z-10 max-w-3xl" {
                span class="inline-block px-4 py-1.5 rounded-full bg-primary/10 text-primary font-semibold text-sm mb-6 border border-primary/20" {
                    "Following Field Notes"
                }
                h1 class="font-display text-4xl md:text-5xl font-bold text-slate-900 leading-[1.1] mb-4" {
                    "Subscribe."
                }
                p class="text-lg text-slate-600 max-w-2xl leading-relaxed" {
                    "Field Notes ships as an RSS feed — a small file your computer or phone checks for new posts so you don't have to. There's no account, no email signup, no analytics tracking who's reading what. Pick a reader app, paste in the feed link, done."
                }
            }
        }

        section class="py-16 bg-white" {
            div class="container mx-auto px-4 md:px-6 max-w-3xl" {
                h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mb-4" {
                    "The feed link"
                }
                p class="text-slate-600 leading-relaxed mb-4" {
                    "Copy this URL and paste it into your reader of choice:"
                }
                div class="rounded-lg border border-slate-200 bg-slate-50 p-4 font-mono text-sm break-all select-all" {
                    "https://plausiden.com/blog/rss.xml"
                }
                p class="text-sm text-slate-500 mt-3" {
                    "If you click it directly, your browser will probably show raw XML — that's normal. Browsers don't render feeds; reader apps do."
                }
            }
        }

        section class="py-16 bg-slate-50" {
            div class="container mx-auto px-4 md:px-6 max-w-3xl" {
                h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mb-4" {
                    "If you don't already use a reader"
                }
                p class="text-slate-600 leading-relaxed mb-8" {
                    "Any of these will work. Pick one that fits the platforms you actually use; they all do the same fundamental thing — pull the feed periodically and show you new posts."
                }
                div class="space-y-4" {
                    @for r in READERS {
                        div class="rounded-lg border border-slate-200 bg-white p-5" {
                            div class="flex items-baseline justify-between gap-4 mb-2" {
                                h3 class="font-display text-lg font-bold text-slate-900" { (r.name) }
                                span class="text-xs text-slate-500" { (r.cost) }
                            }
                            p class="text-sm text-slate-600 leading-relaxed mb-1" { (r.pitch) }
                            p class="text-xs text-slate-500" {
                                strong { "Runs on: " } (r.platforms)
                            }
                        }
                    }
                }
                p class="text-sm text-slate-500 mt-6" {
                    "Not endorsements — these are mainstream readers we've seen work. Any RSS or Atom-compatible reader will read the feed; the format is a published standard, not a vendor-specific protocol."
                }
            }
        }

        section class="py-16 bg-white" {
            div class="container mx-auto px-4 md:px-6 max-w-3xl" {
                h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mb-4" {
                    "How to add the feed (most readers)"
                }
                ol class="space-y-3 text-slate-600 leading-relaxed list-decimal pl-6" {
                    li { "Install one of the readers above." }
                    li { "Open it. Find the option labeled \"Add feed,\" \"Add subscription,\" \"+\" or similar." }
                    li {
                        "Paste in: "
                        code class="text-primary font-mono text-sm" { "https://plausiden.com/blog/rss.xml" }
                    }
                    li { "The reader will fetch the feed and show the existing posts. New posts will appear automatically as they ship." }
                }
                p class="text-sm text-slate-500 mt-6" {
                    "We don't see who subscribed, when they fetched the feed, or which posts they read. The feed is a static file; readers fetch it directly. No tracking pixels, no per-reader URLs, nothing logged on our end beyond a generic web hit."
                }
            }
        }

        section class="py-16 bg-primary/5" {
            div class="container mx-auto px-4 md:px-6 max-w-2xl text-center" {
                h2 class="font-display text-2xl md:text-3xl font-bold text-slate-900 mb-4" {
                    "Or just bookmark the page"
                }
                p class="text-slate-600 leading-relaxed mb-6" {
                    "If a reader app sounds like more setup than it's worth, bookmark "
                    (TextLink { label: "the Field Notes index", href: "/blog", variant: TextLinkVariant::Underlined, size: TextLinkSize::Default }.render())
                    " and check back when the mood strikes. New posts go up at the top."
                }
                a href="/blog" {
                    button type="button" class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium bg-primary text-primary-foreground border border-primary-border min-h-10 px-8 py-6 rounded-xl text-lg shadow-xl shadow-primary/20 hover:-translate-y-0.5 transition-all" {
                        "See the latest posts"
                    }
                }
            }
        }
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
