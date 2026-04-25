//! About page. Matches the production React site's aesthetic (light hero with
//! grid pattern) with the `PlausiDen` mission copy.

use maud::{Markup, html};

use super::layout::page;

/// Render the About page body.
///
/// BUG ASSUMPTION: Copy is placeholder aligned with the production "Learn
/// About Our Mission" framing from the home page. Real About copy can be
/// pasted from the React site once available.
#[must_use]
pub fn render() -> Markup {
    let body = html! {
        section class="relative pt-32 pb-20 md:pt-48 md:pb-32 overflow-hidden bg-slate-50" {
            div class="absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:24px_24px]" {}
            div class="absolute top-0 right-0 w-1/3 h-full bg-gradient-to-l from-primary/5 to-transparent skew-x-12 transform origin-top-right translate-x-32" {}
            div class="container relative mx-auto px-4 md:px-6 z-10" {
                div class="max-w-3xl" {
                    span class="inline-block px-4 py-1.5 rounded-full bg-primary/10 text-primary font-semibold text-sm mb-6 border border-primary/20" { "Our Mission" }
                    h1 class="font-display text-5xl md:text-6xl lg:text-7xl font-bold text-slate-900 leading-[1.1] mb-6" {
                        "Technology " span class="text-primary" { "as an enabler" } ", not a barrier."
                    }
                    p class="text-lg md:text-xl text-slate-600 mb-8 max-w-2xl leading-relaxed" {
                        "PlausiDen LLC was founded on the principle that enterprise IT should adapt to your business — "
                        "not the other way around. We bridge the gap between technical complexity and business value, "
                        "delivering solutions that are general in scope and specific in execution."
                    }
                }
            }
        }

        section class="py-24 bg-white" {
            div class="container mx-auto px-4 md:px-6" {
                div class="max-w-3xl mx-auto" {
                    h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-6" { "What we stand for" }
                    p class="text-slate-600 text-lg leading-relaxed mb-6" {
                        "We treat your infrastructure like it's our own. The security posture that protects your "
                        "customers is the same posture we apply to every engagement. Enterprise-grade is the floor, "
                        "not the ceiling."
                    }
                    p class="text-slate-600 text-lg leading-relaxed mb-6" {
                        "Sovereignty by default. The systems we build and deploy are yours to inspect, yours to "
                        "audit, yours to operate. No vendor hostage, no surprise telemetry, no opaque dependencies."
                    }
                    p class="text-slate-600 text-lg leading-relaxed" {
                        "Customized strategies, not cookie-cutter solutions. Every business has its own shape. "
                        "Our job is to understand yours, and build technology that fits."
                    }
                }
            }
        }

        section class="py-20 bg-primary/5" {
            div class="container mx-auto px-4 md:px-6 text-center" {
                h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-6" { "Ready to work together?" }
                p class="text-slate-600 text-lg mb-8 max-w-2xl mx-auto" {
                    "Tell us what you're building. We'll tell you where we can help."
                }
                a href="/contact" {
                    button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 hover-elevate active-elevate-2 bg-primary text-primary-foreground border border-primary-border min-h-10 px-8 py-6 rounded-xl text-lg shadow-xl shadow-primary/20" {
                        "Start a Conversation"
                    }
                }
            }
        }
    };
    page("About — PlausiDen", "/about", body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn about_has_hero_heading() {
        let s = render().into_string();
        assert!(s.contains("Our Mission"));
        assert!(s.contains("as an enabler"));
    }

    #[test]
    fn about_links_to_contact() {
        let s = render().into_string();
        assert!(s.contains("href=\"/contact\""));
    }
}
