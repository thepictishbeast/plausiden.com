//! Shared page chrome. Matches the production React site's `<head>` and
//! nav / footer structure so visual parity is preserved across server-rendered
//! pages.

use loom_components::ButtonVariant;
use loom_components::footer::{Footer, FooterColumn, FooterItem, FooterLegalLink, FooterStyle};
use loom_components::nav::{Nav, NavCta, NavLink, NavStyle};
use loom_icons as icons;
use maud::{DOCTYPE, Markup, PreEscaped, html};

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
pub const DEFAULT_DESCRIPTION: &str = "IT operations, security and disaster recovery for Massachusetts law firms, medical practices, financial advisers, newsrooms and nonprofits of 5 to 100 staff. Published rates, fixed-price proposals.";

/// JSON-LD for the firm itself, emitted in every page head.
///
/// Typed `ProfessionalService` (a subtype of `LocalBusiness`) rather than the
/// bare `Organization` it used to be. That distinction is the point: a local
/// buyer searching "IT support for law firms near me" is served by the local
/// business graph, and an `Organization` with no service area, no opening
/// hours and no catalogue does not appear in it. `areaServed` and
/// `hasOfferCatalog` are what let a search engine answer "do they do this,
/// and do they do it here" without parsing the page.
///
/// Everything asserted here is stated on the site: the service lines from
/// /services, the audience from /services, the price band from
/// /pricing-transparency. No review counts or ratings — those are structured
/// data you can be penalised for inventing, and we have none to report.
const JSON_LD_ORGANIZATION: &str = r#"{"@context":"https://schema.org","@type":"ProfessionalService","name":"PlausiDen LLC","url":"https://plausiden.com","email":"team@plausiden.com","telephone":"+1-978-351-6495","address":{"@type":"PostalAddress","addressRegion":"MA","addressCountry":"US"},"areaServed":[{"@type":"AdministrativeArea","name":"Greater Boston"},{"@type":"State","name":"Massachusetts"},{"@type":"Country","name":"United States"}],"priceRange":"$$","description":"IT operations, security and disaster recovery for law firms, medical practices, financial advisers, newsrooms and nonprofits of 5 to 100 staff in Massachusetts. Published rates and fixed-price written proposals.","knowsAbout":["IT operations","Cyber security","Disaster recovery","Network segmentation","Access review","Backup and restore testing","Security questionnaires","Industrial automation","Software development"],"hasOfferCatalog":{"@type":"OfferCatalog","name":"Services","itemListElement":[{"@type":"Offer","itemOffered":{"@type":"Service","name":"IT Operations","description":"Monitoring, documented patch windows, tested restores and runbooks."}},{"@type":"Offer","itemOffered":{"@type":"Service","name":"Cyber Security","description":"Hardening, access review, and the security questionnaires clients send before signing."}},{"@type":"Offer","itemOffered":{"@type":"Service","name":"Disaster Recovery","description":"Recovery posture engineered to be tested, not just documented."}}]}}"#;

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

/// Which Open Graph card a route shows when it has not set one explicitly.
///
/// Every page used to point at `static/og-default.svg`, and no major platform
/// renders SVG in a link preview — Slack, LinkedIn, X and Meta all require a
/// raster format. So every link to this site pasted into an email or a Slack
/// channel unfurled with no image at all, which is most of how B2B links
/// travel. The cards are PNGs generated by `scripts/gen-og-images.py`.
///
/// Routes without an entry fall back to the default card rather than to
/// nothing, so a new page is always shareable even before anyone draws it one.
/// `og_cards_exist_for_every_route` fails the build if a mapping here points
/// at a file that is not in the repository.
fn og_card_for(route: &str) -> &'static str {
    match route {
        "/" => "/static/og/home.png",
        "/services" => "/static/og/services.png",
        "/sample-report" => "/static/og/sample-report.png",
        "/pricing-transparency" => "/static/og/pricing.png",
        "/about" => "/static/og/about.png",
        "/case-studies" => "/static/og/case-studies.png",
        "/how-we-work" => "/static/og/how-we-work.png",
        "/contact" => "/static/og/contact.png",
        "/capabilities" => "/static/og/capabilities.png",
        _ => "/static/og/default.png",
    }
}

/// Shared site <head>. Emits canonical URL + OpenGraph + Twitter
/// card + Organization JSON-LD; layers the per-page `PageMeta`
/// over the site defaults.
fn head_tag(meta: &PageMeta<'_>) -> Markup {
    let canonical = format!("{SITE_ORIGIN}{}", meta.current);
    let og_image_url = meta.og_image.map_or_else(
        || format!("{SITE_ORIGIN}{}", og_card_for(meta.current)),
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
            meta property="og:image:type" content="image/png";
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
        // The page's own title is "Why Plausible Privacy Software". A footer
        // that shows only the acronym asks a first-time reader to decode a
        // product name before they know the product exists.
        label: "Plausible Privacy Software",
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
        assert!(
            !s.contains("Secure Drop"),
            "must not revert to 'Secure Drop'"
        );
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

#[cfg(test)]
mod motion_css_guards {
    //! Guards on static/motion.css. It loads last, so anything it declares beats
    //! the Tailwind bundle — which is the point, and also the hazard.

    const MOTION_CSS: &str = include_str!("../../static/motion.css");

    /// Drop /* ... */ blocks so assertions inspect declarations rather than the
    /// commentary about them.
    fn strip_comments(css: &str) -> String {
        let mut out = String::with_capacity(css.len());
        let mut rest = css;
        while let Some(start) = rest.find("/*") {
            out.push_str(&rest[..start]);
            match rest[start + 2..].find("*/") {
                Some(end) => rest = &rest[start + 2 + end + 2..],
                None => return out, // unterminated; the balance test reports it
            }
        }
        out.push_str(rest);
        out
    }

    #[test]
    fn nav_toggle_display_stays_inside_a_mobile_media_query() {
        // REGRESSION-GUARD. A tap-target fix once set `display: inline-flex` on
        // #mobile-menu-toggle unconditionally. Because this file loads after the
        // bundle it beat `lg:hidden`, and the menu button shipped to production
        // visible on desktop beside the full nav. Nothing failed; it took
        // looking at a screenshot to notice.
        //
        // The rule may exist, but only where the button is meant to be shown.
        let Some(idx) = MOTION_CSS.find("#mobile-menu-toggle") else {
            return; // rule removed entirely is also fine
        };
        let before = &MOTION_CSS[..idx];
        let opened = before.matches("@media").count();
        let closed_blocks = before.matches("\n}").count();
        assert!(
            opened > closed_blocks.saturating_sub(1) || before.contains("max-width: 1023.98px"),
            "#mobile-menu-toggle must be scoped to a mobile media query — an \
             unscoped display here overrides lg:hidden and shows the menu \
             button on desktop"
        );
        assert!(
            MOTION_CSS.contains("max-width: 1023.98px"),
            "the mobile scope for the nav toggle is missing"
        );
    }

    #[test]
    fn css_comments_are_balanced() {
        // A malformed comment silently kills the rules that follow it. That is
        // exactly what happened while fixing the invisible-heading bug: an edit
        // left explanatory prose outside a comment block, the parser discarded
        // the rule after it, and two rebuild-and-measure cycles were spent
        // wondering why a correct selector "did not work". Unbalanced markers
        // are cheap to detect and expensive to debug.
        assert_eq!(
            MOTION_CSS.matches("/*").count(),
            MOTION_CSS.matches("*/").count(),
            "unbalanced CSS comment markers — rules after the break are dropped"
        );
    }

    #[test]
    fn dark_band_headings_can_inherit_their_colour() {
        // The bundle's base rule paints every h1-h6 slate-900, and an element
        // rule beats an inherited value, so a plain <h2> on a dark band renders
        // invisible. The homepage shipped that way. This override lets a
        // heading with no colour of its own follow its band.
        //
        // The exclusion list must name colour utilities only: an earlier
        // version excluded [class*="text-"], which also caught size utilities
        // like text-4xl and matched nothing.
        assert!(
            MOTION_CSS.contains(".text-white"),
            "dark-band heading override is missing"
        );
        // Inspect the DECLARATIONS, not the prose. The comment above that
        // override quotes the broken selector while explaining why it was
        // wrong, so a substring search over the whole file flags the
        // explanation as the defect — which it did, on the first run.
        assert!(
            !strip_comments(MOTION_CSS).contains(r#":not([class*="text-"])"#),
            "exclusion is too broad — it would also exclude size utilities like \
             text-4xl, so the rule would silently match nothing"
        );
    }

    /// The scroll reveal must never be able to leave content invisible.
    ///
    /// An earlier version of this site hid every section with `opacity: 0`
    /// and relied on an IntersectionObserver to bring it back. On mobile the
    /// observer did not fire and the page rendered a hero above a blank void.
    /// The replacement is only safe because of where the hidden state lives:
    /// the baseline is visible, and the animation that starts from
    /// transparent is nested inside `@supports (animation-timeline: view())`,
    /// so a browser that cannot run the animation never applies its start
    /// state. Nesting is the whole safety property, so assert the nesting.
    /// Every card a route can point at must exist in the repository.
    ///
    /// A missing file does not fail a build or a request — it fails silently
    /// in someone else's Slack, weeks later, as a link with no preview. The
    /// generator writes these; this makes sure nobody renames a card without
    /// updating the mapping, or adds a mapping before running the generator.
    /// Every published post must have a pre-rendered card on disk.
    ///
    /// Posts are the one part of this site that grows without touching the
    /// layout, so this is where a missing card is most likely: someone adds a
    /// post, ships it, and the share preview is blank until somebody notices
    /// months later. The generator derives its list from the same POSTS slice,
    /// so the fix is always "run scripts/gen-og-images.py".
    #[test]
    fn every_post_has_an_og_card() {
        for post in crate::views::posts::POSTS {
            let card = format!(
                "{}/static/og/blog-{}.png",
                env!("CARGO_MANIFEST_DIR"),
                post.slug
            );
            assert!(
                std::path::Path::new(&card).is_file(),
                "post {:?} has no Open Graph card at static/og/blog-{}.png; \
                 run scripts/gen-og-images.py",
                post.title,
                post.slug
            );
        }
    }

    /// The mobile drawer must hand focus back when it closes.
    ///
    /// Closing puts `display: none` on an ancestor of whatever is focused, and
    /// the browser answers by dropping focus to <body>. Measured before the
    /// fix: a keyboard user opened the drawer, pressed Escape, and landed at
    /// the top of the document with no visible focus, having to tab through
    /// the whole header again to get anywhere.
    ///
    /// There is no JavaScript test harness in this crate, so this reads the
    /// source. It is a weak check — it cannot prove the handler runs — but the
    /// behaviour itself is driven with real key presses during each tick's
    /// verification, and this stops the restoration being deleted as dead
    /// code by someone who cannot see why it is there.
    #[test]
    fn drawer_close_restores_focus_to_its_toggle() {
        const MENU_JS: &str = include_str!("../../static/menu.js");
        assert!(
            MENU_JS.contains("restoreFocus"),
            "menu.js no longer restores focus when the drawer closes; Escape \
             will drop the keyboard user at the top of the document"
        );
        assert!(
            MENU_JS.contains("btn.focus()"),
            "menu.js no longer moves focus back to the menu toggle"
        );
        // Restoration must be conditional on focus actually being inside the
        // drawer, or Escape pressed while typing would yank the caret away.
        assert!(
            MENU_JS.contains("menu.contains(document.activeElement)"),
            "focus restoration is unconditional; Escape in a form field would \
             steal the caret to the menu button"
        );
    }

    #[test]
    fn og_cards_exist_for_every_route() {
        let routes = [
            "/",
            "/services",
            "/sample-report",
            "/pricing-transparency",
            "/about",
            "/case-studies",
            "/how-we-work",
            "/contact",
            "/capabilities",
            "/a-route-that-does-not-exist",
        ];
        for route in routes {
            let card = super::og_card_for(route);
            assert!(
                card.ends_with(".png"),
                "{route} points at {card}, which is not a raster format; no major \
                 platform renders SVG in a link preview"
            );
            let path = format!(
                "{}/{}",
                env!("CARGO_MANIFEST_DIR"),
                card.trim_start_matches('/')
            );
            assert!(
                std::path::Path::new(&path).is_file(),
                "{route} points at {card} but that file is not in the repository; \
                 run scripts/gen-og-images.py"
            );
        }
    }

    #[test]
    fn reveal_can_never_leave_content_invisible() {
        let css = strip_comments(MOTION_CSS);

        // The baseline, outside every gate, must be visible.
        let baseline = css
            .split(".pd-reveal {")
            .nth(1)
            .expect("motion.css must define a .pd-reveal baseline")
            .split('}')
            .next()
            .expect("baseline block is closed");
        assert!(
            baseline.contains("opacity: 1"),
            "the ungated .pd-reveal baseline no longer sets opacity: 1, so a browser \
             that cannot animate could render the element invisible"
        );

        // Every use of a scroll timeline must sit inside the support gate.
        let gate = "@supports (animation-timeline: view())";
        assert!(
            css.contains(gate),
            "the scroll-timeline @supports gate is gone; the hidden start state is \
             now unconditional"
        );
        let after_gate = css.split(gate).nth(1).expect("gate has a body");
        assert!(
            after_gate.trim_start().starts_with('{'),
            "the @supports gate no longer opens a block"
        );
        assert!(
            after_gate.contains("prefers-reduced-motion: no-preference"),
            "the reveal animation is no longer also gated on reduced-motion"
        );
        assert_eq!(
            css.matches("animation-timeline").count(),
            2,
            "animation-timeline appears somewhere other than the single gated rule \
             (once in the @supports condition, once in the declaration); an ungated \
             occurrence can hide content"
        );
    }

    #[test]
    fn shadows_are_ink_tinted_not_pure_black() {
        // The bundle's heavy shadows are pure black (shadow-2xl is 25% black at
        // 50px blur), which is what makes a page read as a template. These are
        // overridden with slate-900-tinted, lower-opacity values.
        assert!(
            MOTION_CSS.contains(".shadow-xl"),
            "shadow scale is overridden"
        );
        assert!(
            MOTION_CSS.contains("rgb(15 23 42 /"),
            "shadows must be tinted with the body ink, not pure black"
        );
    }
}
