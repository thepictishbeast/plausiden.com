//! Homepage view. DOM matches the production React site (2026-04-24 snapshot)
//! so visual parity is preserved. Classes reference the copied production
//! Tailwind/shadcn stylesheet at `/static/index-CWVVhmVm.css`.

use loom_components::card::{FeatureCard, FeatureCardStyle};
use loom_icons as icons;
use maud::{Markup, PreEscaped, html};

use super::layout::page;

fn service_card(icon_svg: &str, title: &str, description: &str) -> Markup {
    FeatureCard {
        icon_svg,
        title,
        description,
    }
    .render_with_style(FeatureCardStyle::Bold)
}

fn check_line(text: &str) -> Markup {
    html! {
        div class="flex items-center gap-3" {
            (PreEscaped(icons::CIRCLE_CHECK.render()))
            span class="text-lg text-slate-200" { (text) }
        }
    }
}

/// Render the homepage body. Produces the production React site's DOM
/// verbatim (Tailwind classes, shadcn wrappers, Lucide SVGs) so the site
/// looks identical to `www.plausiden.com` while being server-rendered.
///
/// BUG ASSUMPTION: The `shadcn-card`, `hover-elevate`, `bg-primary` etc.
/// classes are defined in the copied production stylesheet
/// (`/static/index-CWVVhmVm.css`). Removing that file breaks the styling.
///
/// SECURITY: The testimonial overlay was removed on the production site
/// (commit `95a57fb` on PlausiDen.com). This Rust port preserves that state
/// by rendering the overlay card empty; the REGRESSION-GUARD test below
/// keeps the placeholder testimonial string (see commit `95a57fb`) from
/// leaking back.
#[must_use]
#[allow(clippy::too_many_lines)] // Maud DSL inflates line count; logically one composed page.
pub fn render() -> Markup {
    let body = html! {
        // ---------- Hero ----------
        section class="relative pt-32 pb-20 md:pt-48 md:pb-32 overflow-hidden bg-slate-50" {
            div class="absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:24px_24px]" {}
            div class="absolute top-0 right-0 w-1/3 h-full bg-gradient-to-l from-primary/5 to-transparent skew-x-12 transform origin-top-right translate-x-32" {}
            div class="container relative mx-auto px-4 md:px-6 z-10" {
                div class="max-w-3xl" {
                    div {
                        span class="inline-block px-4 py-1.5 rounded-full bg-primary/10 text-primary font-semibold text-sm mb-6 border border-primary/20 animate-fade-in-up" { "Professional IT Solutions" }
                        h1 class="font-display text-5xl md:text-6xl lg:text-7xl font-bold text-slate-900 leading-[1.1] mb-6 animate-fade-in-up delay-1" {
                            "Comprehensive IT for the " span class="text-primary" { "Modern Enterprise" }
                        }
                        p class="text-lg md:text-xl text-slate-600 mb-8 max-w-2xl leading-relaxed animate-fade-in-up delay-2" {
                            "PlausiDen LLC delivers general yet specific technology solutions. From cyber security to AI automation, we power your digital transformation."
                        }
                        div class="flex flex-col sm:flex-row gap-4 animate-fade-in-up delay-3" {
                            a href="/contact" {
                                button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring hover-elevate active-elevate-2 bg-primary text-primary-foreground border border-primary-border min-h-10 text-lg px-8 py-6 rounded-xl shadow-lg shadow-primary/25 hover:shadow-xl hover:-translate-y-0.5 transition-all" {
                                    "Get a Free Consultation"
                                    (PreEscaped(icons::ARROW_RIGHT.render_with_class("w-5 h-5 ml-2")))
                                }
                            }
                            a href="/services" {
                                button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 hover-elevate active-elevate-2 border [border-color:var(--button-outline)] shadow-xs active:shadow-none min-h-10 text-lg px-8 py-6 rounded-xl bg-white/50 backdrop-blur-sm hover:bg-white border-slate-200" {
                                    "Explore Services"
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---------- Everything Your Business Needs ----------
        section class="py-24 bg-white" {
            div class="container mx-auto px-4 md:px-6" {
                div class="text-center max-w-3xl mx-auto mb-16 reveal" {
                    h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-4" { "Everything Your Business Needs" }
                    p class="text-slate-600 text-lg" { "We provide end-to-end solutions that cover every aspect of your technology stack." }
                }
                @let svg_server = icons::SERVER.render();
                @let svg_shield = icons::SHIELD.render_with_class("w-7 h-7 text-primary group-hover:text-white transition-colors");
                @let svg_brain = icons::BRAIN_CIRCUIT.render();
                @let svg_settings = icons::SETTINGS.render();
                @let svg_code = icons::CODE.render();
                @let svg_cpu = icons::CPU.render();
                div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 reveal reveal-delay-1" {
                    (service_card(&svg_server, "IT Operations", "Robust infrastructure management and operational excellence."))
                    (service_card(&svg_shield, "Cyber Security", "Advanced threat protection and compliance assurance."))
                    (service_card(&svg_brain, "Artificial Intelligence", "Intelligent systems and predictive analytics for your enterprise."))
                    (service_card(&svg_settings, "Automation & IoT", "Operational efficiency through industrial-grade automation."))
                    (service_card(&svg_code, "Software", "Custom development tailored to your specific needs."))
                    (service_card(&svg_cpu, "Hardware", "Enterprise-grade procurement and deployment."))
                }
            }
        }

        // ---------- Why Industry Leaders Choose PlausiDen ----------
        section class="py-24 bg-slate-900 text-white relative overflow-hidden" {
            div class="absolute top-0 right-0 w-96 h-96 bg-primary/20 rounded-full blur-3xl -translate-y-1/2 translate-x-1/2" {}
            div class="absolute bottom-0 left-0 w-64 h-64 bg-blue-500/10 rounded-full blur-3xl translate-y-1/2 -translate-x-1/2" {}
            div class="container relative mx-auto px-4 md:px-6" {
                div class="grid grid-cols-1 lg:grid-cols-2 gap-16 items-center" {
                    div class="reveal" {
                        div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-white/10 text-white text-sm font-medium mb-6 backdrop-blur-sm border border-white/10" {
                            (PreEscaped(icons::TERMINAL.render()))
                            span { "Excellence in Execution" }
                        }
                        h2 class="font-display text-4xl md:text-5xl font-bold mb-6 leading-tight" { "Why Industry Leaders Choose PlausiDen" }
                        p class="text-slate-400 text-lg mb-8 leading-relaxed" {
                            "In a complex digital landscape, you need a partner who understands the big picture while obsessing over the details. We bridge the gap between technical complexity and business value."
                        }
                        div class="space-y-4" {
                            (check_line("Enterprise-grade security standards"))
                            (check_line("24/7 Operational support availability"))
                            (check_line("Customized strategies, not cookie-cutter solutions"))
                            (check_line("Deep expertise across hardware and software"))
                        }
                        div class="mt-10" {
                            a href="/about" {
                                button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 hover-elevate active-elevate-2 border [border-color:var(--button-outline)] shadow-xs active:shadow-none min-h-9 text-white border-white/20 hover:bg-white/10 hover:text-white rounded-xl px-8 py-6 text-lg" {
                                    "Learn About Our Mission"
                                }
                            }
                        }
                    }
                    div class="relative reveal reveal-delay-2" {
                        // Brand-tinted glow behind the image — was previously
                        // a 3D-rotated card; kept the shape, lightened the tint.
                        div class="absolute inset-0 bg-gradient-to-tr from-primary/30 to-transparent rounded-2xl transform rotate-3 scale-105" {}
                        div class="relative rounded-2xl overflow-hidden border border-white/10 shadow-2xl" {
                            img src="/static/images/hero-team.jpg" alt="Team collaboration" class="w-full h-auto object-cover";
                            // Lighter overlay so the image reads "the future is bright"
                            // rather than dimming it into a dark slate. The previous
                            // value (`from-slate-900/80`) was an over-correction left
                            // behind when the testimonial card was removed.
                            div class="absolute inset-0 bg-gradient-to-t from-slate-900/30 via-slate-900/10 to-transparent" {}
                        }
                    }
                }
            }
        }

        // ---------- Final CTA band ----------
        section class="py-20 bg-primary/5" {
            div class="container mx-auto px-4 md:px-6 text-center reveal" {
                h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-6" { "Ready to elevate your IT strategy?" }
                p class="text-slate-600 text-lg mb-8 max-w-2xl mx-auto" {
                    "Contact us today for a free consultation and discover how we can optimize your operations."
                }
                a href="/contact" {
                    button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring hover-elevate active-elevate-2 bg-primary text-primary-foreground border border-primary-border min-h-10 px-8 py-6 rounded-xl text-lg shadow-xl shadow-primary/20" {
                        "Start Your Journey"
                    }
                }
            }
        }
    };
    page(
        "PlausiDen — Comprehensive IT for the Modern Enterprise",
        "/",
        body,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_has_hero_tagline() {
        let s = render().into_string();
        assert!(s.contains("Professional IT Solutions"));
        assert!(s.contains("Comprehensive IT for the"));
        assert!(s.contains("Modern Enterprise"));
    }

    #[test]
    fn home_has_six_service_cards() {
        let s = render().into_string();
        for name in [
            "IT Operations",
            "Cyber Security",
            "Artificial Intelligence",
            "Automation &amp; IoT",
            "Software",
            "Hardware",
        ] {
            assert!(s.contains(name), "home services grid missing: {name}");
        }
    }

    #[test]
    fn home_has_why_industry_leaders_section() {
        let s = render().into_string();
        assert!(s.contains("Why Industry Leaders Choose PlausiDen"));
        assert!(s.contains("Enterprise-grade security standards"));
    }

    #[test]
    fn home_has_final_cta() {
        let s = render().into_string();
        assert!(s.contains("Ready to elevate your IT strategy"));
        assert!(s.contains("Start Your Journey"));
    }

    #[test]
    fn home_has_no_testimonial_text() {
        // REGRESSION-GUARD: the testimonial was removed from the live React
        // site (commit 95a57fb); the Rust port must never re-introduce it.
        let s = render().into_string();
        assert!(!s.contains("transformed our infrastructure"));
    }

    #[test]
    fn home_does_not_use_unsplash_origin_at_runtime() {
        // SECURITY: hero-team.jpg is self-hosted. If the img src ever points
        // to images.unsplash.com, it defeats the privacy posture.
        let s = render().into_string();
        assert!(!s.contains("images.unsplash.com"));
    }
}
