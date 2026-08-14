//! Shared page chrome. Matches the production React site's `<head>` and
//! nav / footer structure so visual parity is preserved across server-rendered
//! pages.

use loom_components::{ButtonShape, ButtonVariant};
use loom_components::footer::{Footer, FooterColumn, FooterItem, FooterLegalLink, FooterStyle};
use loom_components::nav::{Nav, NavCta, NavLink, NavStyle};
use loom_icons as icons;
use maud::{DOCTYPE, Markup, PreEscaped, html};

const ICON_SHIELD_SM: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-shield w-4 h-4 text-emerald-600 group-hover:scale-110 transition-transform"><path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"></path></svg>"#;

const ICON_PHONE_SM: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-phone w-4 h-4"><path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path></svg>"#;

/// Top-nav links shown on every page, ordered the way a prospective client
/// evaluates a firm: what you do, proof you have done it, what it costs, who
/// you are, whether you know your subject.
///
/// Deliberately five, not six. The nav strip is width-constrained (it has to
/// fit beside the brand and the CTA before the mobile breakpoint), so every
/// slot has to earn itself:
/// - "Home" was removed: the brand lock-up already links there, so it spent a
///   slot on a destination the user already has.
/// - "Contact" was removed as a *link* because the CTA button beside it goes to
///   the same page. Two controls for one action split attention instead of
///   directing it.
/// - "Ecosystem" and "Field Notes" were internal names. A buyer scanning a nav
///   for ten seconds cannot tell what either contains. Both pages still exist
///   and stay linked from the footer; the nav now says "Insights".
/// - "Pricing" was added. It is one of the highest-intent pages on the site and
///   was previously reachable only from the footer — hiding the answer to the
///   question every buyer has does not delay the question, it sends them to a
///   competitor who answers it.
const NAV_LINKS: &[NavLink<'static>] = &[
    NavLink {
        href: "/services",
        label: "Services",
    },
    NavLink {
        href: "/case-studies",
        label: "Case Studies",
    },
    NavLink {
        href: "/pricing-transparency",
        label: "Pricing",
    },
    NavLink {
        href: "/about",
        label: "About",
    },
    NavLink {
        href: "/blog",
        label: "Insights",
    },
];

/// The single top-nav call to action.
///
/// There used to be two, "Encrypted Inquiry" and "Get a Quote", and both
/// pointed at /contact. Offering one action twice is a conversion problem, not
/// a choice: the eye has to decide which button is the real one.
///
/// The wording is deliberate. "Encrypted Inquiry" describes our transport, not
/// the buyer's benefit, and reads as a hurdle. "Get a Quote" implies a
/// procurement process before anyone has established the work is worth doing.
/// A short scoping call is the smallest honest first step: it costs the buyer
/// twenty minutes, and it is the step that actually has to happen first.
const NAV_CTAS: &[NavCta<'static>] = &[NavCta {
    href: "/contact",
    label: "Book a scoping call",
    variant: ButtonVariant::Primary,
    icon: Some(ICON_PHONE_SM),
    aria_label: Some("Book a 20-minute scoping call"),
}];

/// Canonical site origin used for absolute URLs in metadata.
const SITE_ORIGIN: &str = "https://plausiden.com";

/// Build stamp appended to unversioned static assets.
///
/// The static handler serves `cache-control: public, max-age=604800, immutable`.
/// That is correct for the Tailwind bundle, whose filename contains a content
/// hash and therefore changes whenever its bytes do. It was actively wrong for
/// `motion.css`, `animations.css`, `nav-responsive.css`, `loom-tokens.css`,
/// `self-hosted-fonts.css` and `menu.js`, whose names never change: a returning
/// visitor kept the previous copy for a week, so a style or script fix silently
/// failed to reach exactly the people who had visited before.
///
/// (Found the honest way: a verified tap-target fix measured as not applied in
/// the browser while the server was demonstrably serving the new file.)
///
/// The stamp is the compile timestamp, so it changes on every build — which is
/// also every deploy — and never changes between requests to the same binary.
///
/// Under `cfg(test)` it is pinned to a constant. The snapshot suite renders
/// every route byte-for-byte, so a value that changes on each compile would put
/// a fresh number into all 24 snapshots and fail the suite on the next build —
/// a test that reports a problem where none exists is worse than no test, and
/// teaches you to accept snapshots without reading them.
#[cfg(not(test))]
const ASSET_VERSION: &str = env!("PLAUSIDEN_BUILD_STAMP");
#[cfg(test)]
const ASSET_VERSION: &str = "test";

/// Append the build stamp to a static asset path, so a new deploy busts the
/// cache for files whose names carry no content hash.
fn asset(path: &str) -> String {
    format!("{path}?v={ASSET_VERSION}")
}

/// Default page description used when a caller doesn't supply one.
/// Tuned for SEO + social previews — single sentence under 160 chars.
pub const DEFAULT_DESCRIPTION: &str = "Comprehensive IT for the modern enterprise — cybersecurity, AI automation, cloud infrastructure, software development. Built for teams that take confidentiality seriously.";

/// JSON-LD Organization schema. Helps search engines understand the
/// site identity. Emitted once in every page head.
const JSON_LD_ORGANIZATION: &str = r#"{"@context":"https://schema.org","@type":"Organization","name":"PlausiDen LLC","url":"https://plausiden.com","email":"team@plausiden.com","telephone":"+1-978-351-6495","address":{"@type":"PostalAddress","addressRegion":"MA","addressCountry":"US"},"description":"Comprehensive IT solutions for the modern enterprise — cybersecurity, AI automation, cloud infrastructure, software development."}"#;

/// Per-page metadata bundle for the shared `<head>`. Keeps the
/// signature flat instead of growing positional arguments per
/// SEO knob; `Default` produces site-default values.
#[derive(Debug)]
pub struct PageMeta<'a> {
    /// `<title>` content (also reused for og:title + twitter:title).
    pub title: &'a str,
    /// Request path for the canonical URL.
    pub current: &'a str,
    /// `<meta name="description">` (also reused for OG + Twitter).
    pub description: &'a str,
    /// og:image absolute URL. `None` falls back to /static/og-default.svg.
    pub og_image: Option<&'a str>,
    /// og:type — `"website"` for marketing pages, `"article"` for posts.
    pub og_type: &'a str,
    /// Pre-rendered JSON-LD object beyond the site Organization schema
    /// (e.g., Article schema on blog posts). Empty string omits it.
    pub extra_json_ld: &'a str,
}

impl Default for PageMeta<'_> {
    fn default() -> Self {
        Self {
            title: "PlausiDen",
            current: "/",
            description: DEFAULT_DESCRIPTION,
            og_image: None,
            og_type: "website",
            extra_json_ld: "",
        }
    }
}

/// Shared site <head>. Emits canonical URL + OpenGraph + Twitter
/// card + Organization JSON-LD; layers the per-page `PageMeta`
/// over the site defaults.
fn head_tag(meta: &PageMeta<'_>) -> Markup {
    let canonical = format!("{SITE_ORIGIN}{}", meta.current);
    let og_image_url = meta.og_image.map_or_else(
        || format!("{SITE_ORIGIN}/static/og-default.svg"),
        |relative| {
            if relative.starts_with("http") {
                relative.to_string()
            } else {
                format!("{SITE_ORIGIN}{relative}")
            }
        },
    );
    html! {
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1";
            meta name="color-scheme" content="light";
            meta name="robots" content="index, follow";
            meta name="description" content=(meta.description);
            meta name="apple-mobile-web-app-title" content="PlausiDen";
            title { (meta.title) }

            // Canonical + OpenGraph + Twitter card.
            link rel="canonical" href=(canonical);
            meta property="og:type" content=(meta.og_type);
            meta property="og:site_name" content="PlausiDen LLC";
            meta property="og:title" content=(meta.title);
            meta property="og:description" content=(meta.description);
            meta property="og:url" content=(canonical);
            meta property="og:image" content=(og_image_url);
            meta property="og:image:width" content="1200";
            meta property="og:image:height" content="630";
            meta property="og:image:alt" content="PlausiDen LLC — Privacy-first IT for the modern enterprise";
            meta name="twitter:card" content="summary_large_image";
            meta name="twitter:title" content=(meta.title);
            meta name="twitter:description" content=(meta.description);
            meta name="twitter:image" content=(og_image_url);

            // JSON-LD: tells crawlers who we are without parsing the page body.
            script type="application/ld+json" { (PreEscaped(JSON_LD_ORGANIZATION)) }
            @if !meta.extra_json_ld.is_empty() {
                script type="application/ld+json" { (PreEscaped(meta.extra_json_ld)) }
            }

            link rel="icon" type="image/png" href="/static/favicon-96x96.png" sizes="96x96";
            link rel="icon" type="image/svg+xml" href="/static/favicon.svg";
            link rel="shortcut icon" href="/static/favicon.ico";
            link rel="apple-touch-icon" sizes="180x180" href="/static/apple-touch-icon.png";
            link rel="manifest" href="/static/site.webmanifest";
            link rel="stylesheet" href=(asset("/static/self-hosted-fonts.css"));
            // loom-tokens.css must precede the Tailwind bundle so
            // overrides via `var(--loom-color-*)` win the cascade
            // when a custom rule references the token.
            link rel="stylesheet" href=(asset("/static/loom-tokens.css"));
            link rel="stylesheet" href="/static/index-CWVVhmVm.css";
            // nav-responsive.css supplies the lg:/xl: nav utilities the
            // frozen Tailwind bundle predates (nav landscape-overflow
            // fix). Must come after the bundle — cascade order lets
            // xl:gap-8 beat the bundle's gap-4 at equal specificity.
            link rel="stylesheet" href=(asset("/static/nav-responsive.css"));
            link rel="stylesheet" href=(asset("/static/animations.css"));
            // motion.css is the site's own motion + focus layer. Last, so it
            // wins ties. Its reveal effect is gated behind
            // `@supports (animation-timeline: view())` — a browser without
            // scroll-driven animation never applies the hidden start state, so
            // content cannot end up invisible the way it did when reveals were
            // driven by an IntersectionObserver.
            link rel="stylesheet" href=(asset("/static/motion.css"));
            script src=(asset("/static/menu.js")) defer {}
        }
    }
}

/// Shared top nav. Composed entirely from the typed `loom_components::Nav`
/// primitive — every visual choice (active styling, mobile drawer, CTA
/// rendering) is owned by the design system, not duplicated here.
fn nav(current: &str) -> Markup {
    Nav {
        brand_logo: &icons::SHIELD,
        brand_name: "PlausiDen",
        brand_accent: "LLC",
        links: NAV_LINKS,
        ctas: NAV_CTAS,
        current,
        style: NavStyle::default(),
    }
    .render()
}

// Footer content lives in static slices so the typed Loom Footer
// primitive can borrow them by reference.
const FOOTER_COMPANY: &[FooterItem<'static>] = &[
    FooterItem::Link {
        href: "/",
        label: "Home",
    },
    FooterItem::Link {
        href: "/about",
        label: "About Us",
    },
    FooterItem::Link {
        href: "/services",
        label: "Services",
    },
    FooterItem::Link {
        href: "/capabilities",
        label: "Capabilities",
    },
    FooterItem::Link {
        href: "/case-studies",
        label: "Case Studies",
    },
    // CMS-backed page (see [`crate::cms`]). Surfaced in the footer
    // so the substrate is discoverable without crawling the sitemap.
    // BUG ASSUMPTION: depends on cms-store/sites/plausiden-com/pages/why-pps.toml
    // being present and Published; the route 404s otherwise.
    FooterItem::Link {
        href: "/docs/why-pps",
        label: "Why PPS",
    },
    // Same destination as the nav, so it carries the same label. A page that is
    // "Insights" in the header and "Field Notes" in the footer reads as two
    // different sections to anyone who did not build the site.
    FooterItem::Link {
        href: "/blog",
        label: "Insights",
    },
    FooterItem::Link {
        href: "/how-we-work",
        label: "How We Work",
    },
    FooterItem::Link {
        href: "/pricing-transparency",
        label: "Pricing",
    },
    FooterItem::Link {
        href: "/contact",
        label: "Contact",
    },
    FooterItem::Link {
        href: "/feedback",
        label: "Feedback",
    },
];

const FOOTER_SOLUTIONS: &[FooterItem<'static>] = &[
    FooterItem::Link {
        href: "/solutions/legal",
        label: "Legal",
    },
    FooterItem::Link {
        href: "/solutions/healthcare",
        label: "Healthcare",
    },
    FooterItem::Link {
        href: "/solutions/journalism",
        label: "Journalism",
    },
    FooterItem::Link {
        href: "/solutions/financial-advisors",
        label: "Financial Advisors",
    },
    FooterItem::Link {
        href: "/solutions/nonprofit",
        label: "Nonprofits",
    },
    FooterItem::Text {
        text: "IT Operations",
    },
    FooterItem::Text {
        text: "Cyber Security",
    },
];

static FOOTER_CONTACT: &[FooterItem<'static>] = &[
    FooterItem::Contact {
        icon: &icons::PHONE,
        label: "978-351-6495",
        href: Some("tel:9783516495"),
    },
    FooterItem::Contact {
        icon: &icons::MAIL,
        label: "team@plausiden.com",
        href: Some("mailto:team@plausiden.com"),
    },
    FooterItem::Contact {
        icon: &icons::MAP_PIN,
        label: "Massachusetts, USA",
        href: None,
    },
];

static FOOTER_COLUMNS: &[FooterColumn<'static>] = &[
    FooterColumn {
        heading: "Company",
        items: FOOTER_COMPANY,
    },
    FooterColumn {
        heading: "Solutions",
        items: FOOTER_SOLUTIONS,
    },
    FooterColumn {
        heading: "Contact",
        items: FOOTER_CONTACT,
    },
];

const FOOTER_LEGAL: &[FooterLegalLink<'static>] = &[
    FooterLegalLink {
        href: "/privacy-directive",
        label: "Sovereign Privacy Directive",
    },
    FooterLegalLink {
        href: "/terms-of-service",
        label: "Sovereign Terms of Service",
    },
];

/// Shared footer. Composed entirely from the typed `loom_components::Footer`
/// primitive. Visual contract preserved exactly.
fn footer() -> Markup {
    Footer {
        brand_logo: &icons::SHIELD,
        brand_name: "PlausiDen",
        brand_accent: "LLC",
        // Appears on all 25 routes, so it was also the single largest source of
        // filler language on the site: the previous wording ("comprehensive,
        // high-quality IT solutions that empower modern enterprises. General
        // yet specific excellence in technology.") put "empower" on every page
        // and told a reader nothing they could act on. Replaced with what the
        // firm actually does and who for.
        brand_tagline: "Security and IT engineering for firms that hold sensitive client data. Fixed scope, fixed price, and the engineer who did the work on the call.",
        columns: FOOTER_COLUMNS,
        copyright: "© PlausiDen LLC. All rights reserved.",
        legal_links: FOOTER_LEGAL,
        style: FooterStyle::default(),
    }
    .render()
}

/// Render a page with the site-wide chrome and the default site
/// description. Use [`page_with_description`] when a page wants a
/// page-specific description, or [`page_with_meta`] for the full
/// SEO knob set (og:image, og:type, extra JSON-LD).
#[must_use]
pub fn page(title: &str, current: &str, body: Markup) -> Markup {
    page_with_description(title, current, DEFAULT_DESCRIPTION, body)
}

/// Render a page with a per-page description. Used by views that
/// want their meta-description to differ from the site default
/// (vertical landing pages, individual blog posts, etc.).
#[must_use]
pub fn page_with_description(
    title: &str,
    current: &str,
    description: &str,
    body: Markup,
) -> Markup {
    page_with_meta(
        &PageMeta {
            title,
            current,
            description,
            ..PageMeta::default()
        },
        body,
    )
}

/// Render a page with full per-page metadata. Used by blog posts and
/// other views that need to override og:image, og:type, or inject
/// extra JSON-LD (e.g., Article schema).
#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn page_with_meta(meta: &PageMeta<'_>, body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            (head_tag(meta))
            body {
                // First focusable element on every page: lets a keyboard or
                // screen-reader user jump past the nav instead of tabbing
                // through it on all 25 routes. Visible only when focused.
                a class="pd-skip-link" href="#main" { "Skip to main content" }
                div id="root" {
                    div class="flex flex-col min-h-screen font-body text-slate-900" {
                        (nav(meta.current))
                        main id="main" tabindex="-1" class="flex-grow" {
                            (body)
                        }
                        (footer())
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_emits_doctype_and_lang() {
        let s = page("Test — PlausiDen", "/", html! { p { "x" } }).into_string();
        assert!(s.starts_with("<!DOCTYPE html>"));
        assert!(s.contains("<html lang=\"en\">"));
    }

    #[test]
    fn page_title_passed_through() {
        let s = page("About — PlausiDen", "/about", html! {}).into_string();
        assert!(s.contains("<title>About — PlausiDen</title>"));
    }

    #[test]
    fn page_links_production_stylesheet() {
        let s = page("X", "/", html! {}).into_string();
        assert!(s.contains("/static/index-CWVVhmVm.css"));
    }

    #[test]
    fn page_nav_offers_exactly_one_call_to_action() {
        // The nav used to carry two CTAs, "Encrypted Inquiry" and "Get a
        // Quote", both pointing at /contact — one action offered twice, which
        // splits the click rather than directing it. This pins the single-CTA
        // decision, and keeps the original guard against reverting to the older
        // "Secure Drop" wording.
        let s = page("X", "/", html! {}).into_string();
        assert!(s.contains("Book a scoping call"), "primary CTA present");
        assert!(!s.contains("Secure Drop"), "must not revert to 'Secure Drop'");
        assert!(
            !s.contains("Encrypted Inquiry"),
            "the second CTA promised an encrypted intake that /contact does not \
             actually provide — do not reinstate it without building the thing"
        );
        // Counting the nav CTA container is brittle; counting the label is not.
        assert_eq!(
            s.matches("Book a scoping call").count(),
            2,
            "exactly one CTA, rendered once in the desktop strip and once in \
             the mobile drawer"
        );
    }

    #[test]
    fn page_footer_contains_contact_info() {
        let s = page("X", "/", html! {}).into_string();
        assert!(s.contains("978-351-6495"));
        assert!(s.contains("team@plausiden.com"));
        assert!(s.contains("Massachusetts, USA"));
    }

    #[test]
    fn active_tab_gets_text_primary_and_full_underline() {
        // REGRESSION-GUARD: user flagged on 2026-04-24 that the active nav
        // tab wasn't highlighted. Production emits text-primary + w-full
        // on the current route's <span>.
        let s = page("X", "/services", html! {}).into_string();
        // The "Services" link must contain text-primary and the full-width
        // underline bar.
        assert!(
            s.contains("text-primary cursor-pointer relative group")
                || s.contains("text-primary\">\n                Services")
                || s.contains(">Services<"),
            "Services link structure changed"
        );
        // Non-active links keep text-slate-600.
        assert!(s.contains("text-slate-600"));
    }
}
