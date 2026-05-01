//! Homepage view. DOM matches the production React site (2026-04-24 snapshot)
//! so visual parity is preserved. Classes reference the copied production
//! Tailwind/shadcn stylesheet at `/static/index-CWVVhmVm.css`.

use loom_components::card::{FeatureCard, FeatureCardStyle};
use loom_components::{Badge, BadgeSize, BadgeTone};
use loom_components::{
    Button, ButtonSize, ButtonType, ButtonVariant, Decoration, Heading, HeadingLevel, HeadingTone,
    HeadingVariant, Lede, Section, SectionPadding, SectionTheme, SectionWidth,
};
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
        div class="flex items-center gap-3" { // loom-allow: check-line row chrome — icon + text on dark band
            (PreEscaped(icons::CIRCLE_CHECK.render()))
            span class="text-lg text-slate-200" { (text) } // loom-allow: muted-on-dark prose; Lede{OnDark} forces text-slate-400 (one shade darker)
        }
    }
}

/// Final CTA band — tinted Loom Section + Heading + Lede + Button.
fn final_cta_band() -> Markup {
    let cta_button = Button {
        label: "Start Your Journey",
        variant: ButtonVariant::Primary,
        size: ButtonSize::Lg,
        aria_label: None,
        icon: None,
        decoration: Decoration::SoftShadow,
        button_type: ButtonType::Button,
    }
    .render();
    let body = html! {
        div class="text-center reveal" {
            div class="mb-6" {
                (Heading {
                    text: "Ready to elevate your IT strategy?",
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Section,
                    tone: HeadingTone::Ink,
                }.render())
            }
            div class="mb-8 max-w-2xl mx-auto" { // loom-allow: centered narrow paragraph wrapper
                (Lede {
                    text: "Contact us today for a free consultation and discover how we can optimize your operations.",
                    tone: HeadingTone::Ink,
                }.render())
            }
            a href="/contact" { (cta_button) }
        }
    };
    Section {
        body: &body,
        theme: SectionTheme::Tinted,
        width: SectionWidth::Default,
        padding: SectionPadding::Loose,
    }
    .render()
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
        section class="relative pt-32 pb-20 md:pt-48 md:pb-32 overflow-hidden bg-slate-50" { // loom-allow: home hero shell — pt-32/48 pb-20/32 cadence is bigger than Loom Hero's pt-32/44 pb-16/24, intentional for the front door
            div class="absolute inset-0 bg-[linear-gradient(to_right,#80808012_1px,transparent_1px),linear-gradient(to_bottom,#80808012_1px,transparent_1px)] bg-[size:24px_24px]" {} // loom-allow: SVG grid fleck
            div class="absolute top-0 right-0 w-1/3 h-full bg-gradient-to-l from-primary/5 to-transparent skew-x-12 transform origin-top-right translate-x-32" {} // loom-allow: skewed primary-tint accent — same shape Loom Hero emits, kept here because home hero shell diverges from Loom Hero's outer
            div class="container relative mx-auto px-4 md:px-6 z-10" { // loom-allow: hero container with fleck stacking
                div class="max-w-3xl" { // loom-allow: hero content max-w-3xl
                    div {
                        div class="mb-6 animate-fade-in-up" { (Badge { label: "Professional IT Solutions", tone: BadgeTone::Primary, size: BadgeSize::Md }.render()) } // loom-allow: animation hook on Badge wrapper
                        h1 class="font-display text-5xl md:text-6xl lg:text-7xl font-bold text-slate-900 leading-[1.1] mb-6 animate-fade-in-up delay-1" { // loom-allow: home hero h1 — text-5xl/6xl/7xl one step bigger than Loom Heading{Display} (4xl/5xl/6xl); the front door warrants the upsize
                            "Comprehensive IT for the " span class="text-primary" { "Modern Enterprise" } // loom-allow: two-tone headline accent — Loom Heading takes a single &str
                        }
                        p class="text-lg md:text-xl text-slate-600 mb-8 max-w-2xl leading-relaxed animate-fade-in-up delay-2" { // loom-allow: hero subheadline — Lede emits no animation hook + uses mb-4 not mb-8
                            "PlausiDen LLC delivers general yet specific technology solutions. From cyber security to AI automation, we power your digital transformation."
                        }
                        div class="flex flex-col sm:flex-row gap-4 animate-fade-in-up delay-3" { // loom-allow: CTA cluster with delay-3 animation hook
                            a href="/contact" {
                                button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring hover-elevate active-elevate-2 bg-primary text-primary-foreground border border-primary-border min-h-10 text-lg px-8 py-6 rounded-xl shadow-lg shadow-primary/25 hover:shadow-xl hover:-translate-y-0.5 transition-all" { // loom-allow: hero primary CTA — has hover-elevate + active-elevate-2 hooks Loom Button doesn't emit
                                    "Get a Free Consultation"
                                    (PreEscaped(icons::ARROW_RIGHT.render_with_class("w-5 h-5 ml-2"))) // loom-allow: SVG class attribute on inline icon
                                }
                            }
                            a href="/services" {
                                button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 hover-elevate active-elevate-2 border [border-color:var(--button-outline)] shadow-xs active:shadow-none min-h-10 text-lg px-8 py-6 rounded-xl bg-white/50 backdrop-blur-sm hover:bg-white border-slate-200" { // loom-allow: hero secondary CTA — translucent backdrop-blur over fleck doesn't fit Loom Button{Outline}
                                    "Explore Services"
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---------- Everything Your Business Needs ----------
        section class="py-24 bg-white" { // loom-allow: services band — py-24 cadence above Loom Section{Loose}
            div class="container mx-auto px-4 md:px-6" { // loom-allow: full-width container
                div class="text-center max-w-3xl mx-auto mb-16 reveal" { // loom-allow: centred caption above grid + scroll-reveal hook
                    div class="mb-4" {
                        (Heading {
                            text: "Everything Your Business Needs",
                            level: HeadingLevel::H2,
                            variant: HeadingVariant::Section,
                            tone: HeadingTone::Ink,
                        }.render())
                    }
                    (Lede {
                        text: "We provide end-to-end solutions that cover every aspect of your technology stack.",
                        tone: HeadingTone::Ink,
                    }.render())
                }
                @let svg_server = icons::SERVER.render();
                @let svg_shield = icons::SHIELD.render_with_class("w-7 h-7 text-primary group-hover:text-white transition-colors"); // loom-allow: SVG class attribute with group-hover hook
                @let svg_brain = icons::BRAIN_CIRCUIT.render();
                @let svg_settings = icons::SETTINGS.render();
                @let svg_code = icons::CODE.render();
                @let svg_cpu = icons::CPU.render();
                div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 reveal reveal-delay-1" { // loom-allow: 3-up service-card grid + scroll-reveal hook
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
        section class="py-24 bg-slate-900 text-white relative overflow-hidden" { // loom-allow: dark Why-Industry-Leaders band — has 2 decorative blobs + image, bigger shape than Loom Section{Dark}
            div class="absolute top-0 right-0 w-96 h-96 bg-primary/20 rounded-full blur-3xl -translate-y-1/2 translate-x-1/2" {} // loom-allow: top-right primary-blob blur
            div class="absolute bottom-0 left-0 w-64 h-64 bg-blue-500/10 rounded-full blur-3xl translate-y-1/2 -translate-x-1/2" {} // loom-allow: bottom-left blue-blob blur
            div class="container relative mx-auto px-4 md:px-6" { // loom-allow: container with blob stacking
                div class="grid grid-cols-1 lg:grid-cols-2 gap-16 items-center" { // loom-allow: 2-up split — text left, image right
                    div class="reveal" { // loom-allow: text-column scroll-reveal hook
                        div class="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-white/10 text-white text-sm font-medium mb-6 backdrop-blur-sm border border-white/10" { // loom-allow: glass-morphism eyebrow with inline icon — pending Badge::Eyebrow with icon slot
                            (PreEscaped(icons::TERMINAL.render()))
                            span { "Excellence in Execution" }
                        }
                        h2 class="font-display text-4xl md:text-5xl font-bold mb-6 leading-tight" { "Why Industry Leaders Choose PlausiDen" } // loom-allow: dark-band h2 — Heading{Section,OnDark} would emit text-3xl md:text-4xl, one size step smaller
                        p class="text-slate-400 text-lg mb-8 leading-relaxed" { // loom-allow: dark-band lede — text-slate-400 matches Loom Lede{OnDark} but mb-8 is bigger than Lede's no-margin
                            "In a complex digital landscape, you need a partner who understands the big picture while obsessing over the details. We bridge the gap between technical complexity and business value."
                        }
                        div class="space-y-4" { // loom-allow: vertical rhythm between check-lines
                            (check_line("Enterprise-grade security standards"))
                            (check_line("24/7 Operational support availability"))
                            (check_line("Customized strategies, not cookie-cutter solutions"))
                            (check_line("Deep expertise across hardware and software"))
                        }
                        div class="mt-10" { // loom-allow: spacer above the dark-band CTA
                            a href="/about" {
                                button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 hover-elevate active-elevate-2 border [border-color:var(--button-outline)] shadow-xs active:shadow-none min-h-9 text-white border-white/20 hover:bg-white/10 hover:text-white rounded-xl px-8 py-6 text-lg" { // loom-allow: outline-on-dark CTA — Loom Button{Outline} doesn't have a dark-band variant
                                    "Learn About Our Mission"
                                }
                            }
                        }
                    }
                    div class="relative reveal reveal-delay-2" { // loom-allow: image-column scroll-reveal hook
                        div class="absolute inset-0 bg-gradient-to-tr from-primary/30 to-transparent rounded-2xl transform rotate-3 scale-105" {} // loom-allow: brand-tinted glow behind image
                        div class="relative rounded-2xl overflow-hidden border border-white/10 shadow-2xl" { // loom-allow: image card chrome
                            img src="/static/images/hero-team.jpg" alt="Team collaboration" class="w-full h-auto object-cover"; // loom-allow: full-width responsive image
                            div class="absolute inset-0 bg-gradient-to-t from-slate-900/30 via-slate-900/10 to-transparent" {} // loom-allow: bottom-up dark gradient over image
                        }
                    }
                }
            }
        }

        (final_cta_band())
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
