//! Homepage view. DOM matches the production React site (2026-04-24 snapshot)
//! so visual parity is preserved. Classes reference the copied production
//! Tailwind/shadcn stylesheet at `/static/index-CWVVhmVm.css`.

use loom_components::card::{FeatureCard, FeatureCardStyle};
use loom_components::{Badge, BadgeShape, BadgeSize, BadgeTone};
use loom_components::{
    Button, ButtonShape, ButtonSize, ButtonType, ButtonVariant, Decoration, Heading, HeadingLevel,
    HeadingTone, HeadingVariant, Lede, Section, SectionPadding, SectionTheme, SectionWidth,
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
        label: "Book a scoping call",
        variant: ButtonVariant::Primary,
        size: ButtonSize::Lg,
        aria_label: None,
        icon: None,
        decoration: Decoration::SoftShadow,
        button_type: ButtonType::Button,
        shape: ButtonShape::default(),
    }
    .render();
    let body = html! {
        div class="text-center reveal" {
            div class="mb-6" {
                (Heading {
                    text: "Want to know what this would cost you?",
                    level: HeadingLevel::H2,
                    variant: HeadingVariant::Section,
                    tone: HeadingTone::Ink,
                }.render())
            }
            div class="mb-8 max-w-2xl mx-auto" { // loom-allow: centered narrow paragraph wrapper
                (Lede {
                    text: "A short call is enough to scope the work and put a fixed price on it. If we are not the right fit, we will say so and point you somewhere better.",
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
                        // The badge used to read "Professional IT Solutions",
                        // which asserts nothing a competitor could not also
                        // claim. Woman-owned is checkable, and it is a real
                        // procurement lever: many enterprises run supplier
                        // diversity targets, so it can decide a shortlist.
                        // Every line here is promoted from copy that already
                        // exists deeper in the site (/services, /pricing-
                        // transparency) rather than written fresh. The old hero
                        // ("Comprehensive IT for the Modern Enterprise" /
                        // "general yet specific technology solutions") was true
                        // of every IT firm on earth and named no industry, no
                        // company size, no geography and no price — while
                        // /services was already saying all four out loud, two
                        // clicks away.
                        div class="mb-6 animate-fade-in-up" { (Badge { label: "IT, security and disaster recovery · Massachusetts", tone: BadgeTone::Primary, size: BadgeSize::Md, shape: BadgeShape::default() }.render()) } // loom-allow: animation hook on Badge wrapper
                        h1 class="font-display text-5xl md:text-6xl lg:text-7xl font-bold text-slate-900 leading-[1.1] mb-6 animate-fade-in-up delay-1" { // loom-allow: home hero h1 — text-5xl/6xl/7xl one step bigger than Loom Heading{Display} (4xl/5xl/6xl); the front door warrants the upsize
                            "The IT team for practices that hold " span class="text-primary" { "confidential client data" } // loom-allow: two-tone headline accent — Loom Heading takes a single &str
                        }
                        p class="text-lg md:text-xl text-slate-600 mb-8 max-w-2xl leading-relaxed animate-fade-in-up delay-2" { // loom-allow: hero subheadline — Lede emits no animation hook + uses mb-4 not mb-8
                            "We run IT operations, security and disaster recovery for law firms, medical practices, financial advisers, newsrooms and nonprofits with 5 to 100 staff. Our rates are published. If we are the wrong fit on price, you will know in thirty seconds instead of three phone calls."
                        }
                        // The published rate card is the sharpest thing this
                        // firm has and it was reachable only from the footer.
                        // Naming the numbers on the front door is the whole
                        // point of having them.
                        p class="text-sm text-slate-500 mb-8 max-w-2xl animate-fade-in-up delay-2" { // loom-allow: hero trust strip — matches the subheadline column, one step down the type scale
                            "Woman-owned · Greater Boston · Mutual NDA before the first call · Written fixed-price proposals, never “depends what we find”"
                        }
                        div class="flex flex-col sm:flex-row gap-4 animate-fade-in-up delay-3" { // loom-allow: CTA cluster with delay-3 animation hook
                            a href="/contact" {
                                button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring hover-elevate active-elevate-2 bg-primary text-primary-foreground border border-primary-border min-h-10 text-lg px-8 py-6 rounded-xl shadow-lg shadow-primary/25 hover:shadow-xl hover:-translate-y-0.5 transition-all" { // loom-allow: hero primary CTA — has hover-elevate + active-elevate-2 hooks Loom Button doesn't emit
                                    // Same words as the nav CTA. It was "Get a
                                    // Free Consultation" here and "Book a
                                    // scoping call" there — one action should
                                    // not have two names on the same screen.
                                    "Book a scoping call"
                                    (PreEscaped(icons::ARROW_RIGHT.render_with_class("w-5 h-5 ml-2"))) // loom-allow: SVG class attribute on inline icon
                                }
                            }
                            // Points at the rate card rather than the services
                            // list: a buyer who is price-qualifying is closer to
                            // buying than one still browsing, and this is the
                            // page most competitors refuse to publish at all.
                            a href="/pricing-transparency" {
                                button class="inline-flex items-center justify-center gap-2 whitespace-nowrap font-medium focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 hover-elevate active-elevate-2 border [border-color:var(--button-outline)] shadow-xs active:shadow-none min-h-10 text-lg px-8 py-6 rounded-xl bg-white/50 backdrop-blur-sm hover:bg-white border-slate-200" { // loom-allow: hero secondary CTA — translucent backdrop-blur over fleck doesn't fit Loom Button{Outline}
                                    "See our rates"
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---------- What changed for three clients ----------
        //
        // These outcomes were already published on /case-studies and had never
        // appeared on the page people actually land on. They are the strongest
        // evidence the firm has — a carrier repricing a policy, a state audit
        // passed first time, counsel signing off on a source-protection
        // posture — and a buyer had to find the Case Studies tab to see any of
        // it. Nothing here is new or embellished: each line is the outcome
        // field of an existing study, shortened, and links to the full one.
        section class="py-20 bg-white border-b border-slate-100" { // loom-allow: proof strip — lighter cadence than the py-24 bands either side so it reads as a supporting row, not a third pillar
            div class="container mx-auto px-4 md:px-6" { // loom-allow: full-width container
                div class="text-center max-w-3xl mx-auto mb-12 reveal" { // loom-allow: centred caption; matches the services band above
                    div class="mb-4" {
                        (Heading {
                            text: "What changed for three clients",
                            level: HeadingLevel::H2,
                            variant: HeadingVariant::Section,
                            tone: HeadingTone::Ink,
                        }.render())
                    }
                    (Lede {
                        text: "Sanitized summaries of real engagements. Identifying details are removed, and nothing appears here without the client's written sign-off.",
                        tone: HeadingTone::Ink,
                    }.render())
                }
                div class="grid grid-cols-1 md:grid-cols-3 gap-8 reveal reveal-delay-1" { // loom-allow: 3-up proof row; Loom ships no Grid primitive
                    @for (who, what) in [
                        ("Boutique law firm",
                         "Their malpractice carrier reduced the policy premium on the strength of the audit. The next litigation hold was answered in one paragraph instead of a forensic engagement."),
                        ("Specialty healthcare practice",
                         "Passed a state-level audit on the first try. Time to answer an audit request dropped from weeks to a single day."),
                        ("Investigative newsroom",
                         "The investigation was published. No subpoenas have surfaced, and a separate counsel review confirmed the substrate would not be probative if one arrived."),
                    ] {
                        div class="pd-proof" { // loom-allow: site-owned hairline-rule treatment (motion.css) — quieter than the FeatureCards above so proof reads as evidence, not another product tile. Deliberately NOT bundle utilities: pl-6 and border-primary/30 are absent from the frozen CSS and silently render as 0 padding and default grey.
                            p class="text-sm font-semibold text-primary mb-2" { (who) } // loom-allow: attribution eyebrow
                            p class="text-slate-600 leading-relaxed" { (what) } // loom-allow: body copy matching the services-band description size
                        }
                    }
                }
                div class="text-center mt-12" { // loom-allow: single centred link below the proof row
                    a href="/case-studies" class="text-primary font-semibold pd-underline" { "Read the full case studies" } // loom-allow: inline text link using the site-owned underline animation
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
                    (service_card(&svg_server, "IT Operations", "Monitoring, documented patch windows, tested restores and real runbooks — so the answer to 'who fixes this' is never 'the person who is good with computers'."))
                    (service_card(&svg_shield, "Cyber Security", "Hardening, access reviews and the security questionnaires your clients send before they will sign. Findings reach you when we find them, not at the end."))
                    (service_card(&svg_brain, "Artificial Intelligence", "Practical automation of the work that eats your week — intake, document handling, reporting — kept on infrastructure you control."))
                    (service_card(&svg_settings, "Automation & IoT", "Control and monitoring for equipment that has to keep running, with the network segmented so a sensor cannot reach your case files."))
                    (service_card(&svg_code, "Software", "Small, well-scoped builds where an off-the-shelf tool does not fit — and an honest answer when one does."))
                    (service_card(&svg_cpu, "Hardware", "Specification, procurement and deployment, including the disposal step most firms forget: drives leaving the building are wiped or destroyed."))
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
                            span { "How we run an engagement" }
                        }
                        h2 class="font-display text-4xl md:text-5xl font-bold mb-6 leading-tight" { "What working with us actually looks like" } // loom-allow: dark-band h2 — Heading{Section,OnDark} would emit text-3xl md:text-4xl, one size step smaller
                        p class="text-slate-400 text-lg mb-8 leading-relaxed" { // loom-allow: dark-band lede — text-slate-400 matches Loom Lede{OnDark} but mb-8 is bigger than Lede's no-margin
                            "Most of what goes wrong in IT is not exotic. It is an unpatched box nobody owned, a backup that was never restored from, or a permission granted in 2019 and never removed. We work the boring things first, in writing, and tell you which risks we are accepting and why."
                        }
                        div class="space-y-4" { // loom-allow: vertical rhythm between check-lines
                            (check_line("Fixed scope and a fixed price agreed before any work starts"))
                            (check_line("The engineer who did the work answers your call — not an account manager"))
                            (check_line("Critical findings sent the day we find them, never held for the report"))
                            (check_line("A retest after you fix things, included — not sold back to you"))
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
        "PlausiDen LLC — IT & Cybersecurity for Massachusetts Law, Medical & Financial Practices",
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
        // The eyebrow now states the service lines and the region rather
        // than the content-free "Professional IT Solutions".
        assert!(s.contains("IT, security and disaster recovery"));
        // The hero must name who it is for. "Comprehensive IT for the
        // Modern Enterprise" named no industry, size, place or price.
        assert!(s.contains("confidential client data"));
        assert!(
            s.contains("5 to 100 staff"),
            "hero states the size of client we serve"
        );
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
    fn home_states_commitments_not_unverifiable_claims() {
        // Was home_has_why_industry_leaders_section, pinning the heading "Why
        // Industry Leaders Choose PlausiDen" and the bullet "Enterprise-grade
        // security standards". Neither could be checked by a reader: we cannot
        // name an industry leader who chose us, and "enterprise-grade" means
        // whatever the writer wants. For a firm selling trust, a claim a
        // prospect can quietly disprove costs more than it earns.
        //
        // The section now promises things the engagement itself proves.
        let s = render().into_string();
        assert!(s.contains("What working with us actually looks like"));
        assert!(s.contains("Fixed scope and a fixed price agreed before any work starts"));
        assert!(s.contains("retest after you fix things, included"));
        assert!(
            !s.contains("Industry Leaders"),
            "no social proof we cannot substantiate"
        );
    }

    #[test]
    fn home_has_final_cta() {
        let s = render().into_string();
        // The closing band asks the question a buyer is actually holding,
        // and its button carries the same words as the nav and hero CTAs.
        // The site previously named one action four different ways ("Get a
        // Free Consultation", "Start Your Journey", "Schedule an intake
        // call", "Start the conversation"), which reads as four offers.
        assert!(s.contains("Want to know what this would cost you"));
        assert!(
            s.matches("Book a scoping call").count() >= 3,
            "one action, one name: nav, hero and closing band must agree"
        );
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
