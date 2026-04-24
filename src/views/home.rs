//! Homepage view. DOM matches the production React site (2026-04-24 snapshot)
//! so visual parity is preserved. Classes reference the copied production
//! Tailwind/shadcn stylesheet at `/static/index-CWVVhmVm.css`.

use maud::{Markup, PreEscaped, html};

use super::layout::page;

// --- Lucide SVG icons, inlined to match the React output ---

const ICON_SERVER: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-server w-7 h-7 text-primary group-hover:text-white transition-colors"><rect width="20" height="8" x="2" y="2" rx="2" ry="2"></rect><rect width="20" height="8" x="2" y="14" rx="2" ry="2"></rect><line x1="6" x2="6.01" y1="6" y2="6"></line><line x1="6" x2="6.01" y1="18" y2="18"></line></svg>"#;

const ICON_SHIELD: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-shield w-7 h-7 text-primary group-hover:text-white transition-colors"><path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"></path></svg>"#;

const ICON_BRAIN: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-brain-circuit w-7 h-7 text-primary group-hover:text-white transition-colors"><path d="M12 5a3 3 0 1 0-5.997.125 4 4 0 0 0-2.526 5.77 4 4 0 0 0 .556 6.588A4 4 0 1 0 12 18Z"></path><path d="M9 13a4.5 4.5 0 0 0 3-4"></path><path d="M6.003 5.125A3 3 0 0 0 6.401 6.5"></path><path d="M3.477 10.896a4 4 0 0 1 .585-.396"></path><path d="M6 18a4 4 0 0 1-1.967-.516"></path><path d="M12 13h4"></path><path d="M12 18h6a2 2 0 0 1 2 2v1"></path><path d="M12 8h8"></path><path d="M16 8V5a2 2 0 0 1 2-2"></path><circle cx="16" cy="13" r=".5"></circle><circle cx="18" cy="3" r=".5"></circle><circle cx="20" cy="21" r=".5"></circle><circle cx="20" cy="8" r=".5"></circle></svg>"#;

const ICON_SETTINGS: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-settings w-7 h-7 text-primary group-hover:text-white transition-colors"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"></path><circle cx="12" cy="12" r="3"></circle></svg>"#;

const ICON_CODE: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-code w-7 h-7 text-primary group-hover:text-white transition-colors"><polyline points="16 18 22 12 16 6"></polyline><polyline points="8 6 2 12 8 18"></polyline></svg>"#;

const ICON_CPU: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-cpu w-7 h-7 text-primary group-hover:text-white transition-colors"><rect width="16" height="16" x="4" y="4" rx="2"></rect><rect width="6" height="6" x="9" y="9" rx="1"></rect><path d="M15 2v2"></path><path d="M15 20v2"></path><path d="M2 15h2"></path><path d="M2 9h2"></path><path d="M20 15h2"></path><path d="M20 9h2"></path><path d="M9 2v2"></path><path d="M9 20v2"></path></svg>"#;

const ICON_ARROW_RIGHT: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-arrow-right w-5 h-5 ml-2"><path d="M5 12h14"></path><path d="m12 5 7 7-7 7"></path></svg>"#;

const ICON_TERMINAL: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-terminal w-4 h-4"><polyline points="4 17 10 11 4 5"></polyline><line x1="12" x2="20" y1="19" y2="19"></line></svg>"#;

const ICON_CHECK: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-circle-check w-6 h-6 text-primary shrink-0"><circle cx="12" cy="12" r="10"></circle><path d="m9 12 2 2 4-4"></path></svg>"#;

fn service_card(icon_svg: &str, title: &str, desc: &str) -> Markup {
    html! {
        div {
            div class="shadcn-card rounded-xl bg-card text-card-foreground h-full border border-slate-100 shadow-lg hover:shadow-xl transition-all duration-300 hover:border-primary/20 group" {
                div class="p-8" {
                    div class="w-14 h-14 rounded-2xl bg-primary/5 flex items-center justify-center mb-6 group-hover:bg-primary group-hover:text-white transition-colors duration-300" {
                        (PreEscaped(icon_svg))
                    }
                    h3 class="font-display text-xl font-bold text-slate-900 mb-3" { (title) }
                    p class="text-slate-600 leading-relaxed" { (desc) }
                }
            }
        }
    }
}

fn check_line(text: &str) -> Markup {
    html! {
        div class="flex items-center gap-3" {
            (PreEscaped(ICON_CHECK))
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
                        span class="inline-block px-4 py-1.5 rounded-full bg-primary/10 text-primary font-semibold text-sm mb-6 border border-primary/20" { "Professional IT Solutions" }
                        h1 class="font-display text-5xl md:text-6xl lg:text-7xl font-bold text-slate-900 leading-[1.1] mb-6" {
                            "Comprehensive IT for the " span class="text-primary" { "Modern Enterprise" }
                        }
                        p class="text-lg md:text-xl text-slate-600 mb-8 max-w-2xl leading-relaxed" {
                            "PlausiDen LLC delivers general yet specific technology solutions. From cyber security to AI automation, we power your digital transformation."
                        }
                        div class="flex flex-col sm:flex-row gap-4" {
                            a href="/contact" {
                                button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring hover-elevate active-elevate-2 bg-primary text-primary-foreground border border-primary-border min-h-10 text-lg px-8 py-6 rounded-xl shadow-lg shadow-primary/25 hover:shadow-xl hover:-translate-y-0.5 transition-all" {
                                    "Get a Free Consultation"
                                    (PreEscaped(ICON_ARROW_RIGHT))
                                }
                            }
                            a href="/services" {
                                button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring hover-elevate active-elevate-2 border shadow-xs active:shadow-none min-h-10 text-lg px-8 py-6 rounded-xl bg-white/50 backdrop-blur-sm hover:bg-white border-slate-200" {
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
                div class="text-center max-w-3xl mx-auto mb-16" {
                    h2 class="font-display text-3xl md:text-4xl font-bold text-slate-900 mb-4" { "Everything Your Business Needs" }
                    p class="text-slate-600 text-lg" { "We provide end-to-end solutions that cover every aspect of your technology stack." }
                }
                div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8" {
                    (service_card(ICON_SERVER, "IT Operations", "Robust infrastructure management and operational excellence."))
                    (service_card(ICON_SHIELD, "Cyber Security", "Advanced threat protection and compliance assurance."))
                    (service_card(ICON_BRAIN, "Artificial Intelligence", "Intelligent systems and predictive analytics for your enterprise."))
                    (service_card(ICON_SETTINGS, "Automation & IoT", "Operational efficiency through industrial-grade automation."))
                    (service_card(ICON_CODE, "Software", "Custom development tailored to your specific needs."))
                    (service_card(ICON_CPU, "Hardware", "Enterprise-grade procurement and deployment."))
                }
            }
        }

        // ---------- Why Industry Leaders Choose PlausiDen ----------
        section class="py-24 bg-slate-900 text-white relative overflow-hidden" {
            div class="absolute top-0 right-0 w-96 h-96 bg-primary/20 rounded-full blur-3xl -translate-y-1/2 translate-x-1/2" {}
            div class="absolute bottom-0 left-0 w-64 h-64 bg-blue-500/10 rounded-full blur-3xl translate-y-1/2 -translate-x-1/2" {}
            div class="container relative mx-auto px-4 md:px-6" {
                div class="grid grid-cols-1 lg:grid-cols-2 gap-16 items-center" {
                    div {
                        div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-white/10 text-white text-sm font-medium mb-6 backdrop-blur-sm border border-white/10" {
                            (PreEscaped(ICON_TERMINAL))
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
                                button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring hover-elevate active-elevate-2 border shadow-xs active:shadow-none min-h-9 text-white border-white/20 hover:bg-white/10 hover:text-white rounded-xl px-8 py-6 text-lg" {
                                    "Learn About Our Mission"
                                }
                            }
                        }
                    }
                    div class="relative" {
                        div class="absolute inset-0 bg-gradient-to-tr from-primary/20 to-transparent rounded-2xl transform rotate-3 scale-105" {}
                        div class="relative rounded-2xl overflow-hidden border border-white/10 shadow-2xl" {
                            img src="/static/images/hero-team.jpg" alt="Team collaboration" class="w-full h-auto object-cover";
                            div class="absolute inset-0 bg-gradient-to-t from-slate-900/80 to-transparent" {}
                            // Testimonial overlay intentionally omitted — empty per the
                            // REGRESSION-GUARD below and per the production removal
                            // (PlausiDen.com commit 95a57fb + gray-box-class scrub).
                        }
                    }
                }
            }
        }

        // ---------- Final CTA band ----------
        section class="py-20 bg-primary/5" {
            div class="container mx-auto px-4 md:px-6 text-center" {
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
