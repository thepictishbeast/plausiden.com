//! Shared page chrome. Matches the production React site's `<head>` and
//! nav / footer structure so visual parity is preserved across server-rendered
//! pages.

use loom_components::footer::{Footer, FooterColumn, FooterItem, FooterLegalLink};
use loom_components::nav::{Nav, NavCta, NavLink};
use loom_components::ButtonVariant;
use loom_icons as icons;
use maud::{DOCTYPE, Markup, PreEscaped, html};

const ICON_SHIELD_SM: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-shield w-4 h-4 text-emerald-600 group-hover:scale-110 transition-transform"><path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"></path></svg>"#;

const ICON_PHONE_SM: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-phone w-4 h-4"><path d="M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"></path></svg>"#;

/// Top-nav links shown on every page. Order is left-to-right.
const NAV_LINKS: &[NavLink<'static>] = &[
    NavLink { href: "/", label: "Home" },
    NavLink { href: "/services", label: "Services" },
    NavLink { href: "/about", label: "About" },
    NavLink { href: "/blog", label: "Field Notes" },
    NavLink { href: "/contact", label: "Contact" },
];

/// Top-nav CTA buttons. Both route to /contact today; the variant /
/// icon distinguish the two affordances visually.
const NAV_CTAS: &[NavCta<'static>] = &[
    NavCta {
        href: "/contact",
        label: "Encrypted Inquiry",
        variant: ButtonVariant::OutlineSuccess,
        icon: Some(ICON_SHIELD_SM),
        aria_label: Some("Encrypted Inquiry"),
    },
    NavCta {
        href: "/contact",
        label: "Get a Quote",
        variant: ButtonVariant::Primary,
        icon: Some(ICON_PHONE_SM),
        aria_label: Some("Get a Quote"),
    },
];

/// Canonical site origin used for absolute URLs in metadata.
const SITE_ORIGIN: &str = "https://plausiden.com";

/// Default page description used when a caller doesn't supply one.
/// Tuned for SEO + social previews — single sentence under 160 chars.
pub const DEFAULT_DESCRIPTION: &str = "Comprehensive IT for the modern enterprise — cybersecurity, AI automation, cloud infrastructure, software development. Built for teams that take confidentiality seriously.";

/// JSON-LD Organization schema. Helps search engines understand the
/// site identity. Emitted once in every page head.
const JSON_LD_ORGANIZATION: &str = r#"{"@context":"https://schema.org","@type":"Organization","name":"PlausiDen LLC","url":"https://plausiden.com","email":"team@plausiden.com","telephone":"+1-978-351-6495","address":{"@type":"PostalAddress","addressRegion":"MA","addressCountry":"US"},"description":"Comprehensive IT solutions for the modern enterprise — cybersecurity, AI automation, cloud infrastructure, software development."}"#;

/// Shared site <head>. Loads the production Tailwind/shadcn CSS and the two
/// Google Fonts the CSS actually references (`Plus Jakarta Sans`, `Outfit`).
/// Emits `OpenGraph` + Twitter card metadata and a canonical URL so links
/// shared in Slack/email/social preview cleanly.
///
/// SECURITY: Fonts are self-hosted under `/static/fonts/`; no
/// third-party origin is referenced. CSP in `crate::security` locks
/// every fetch directive to `'self'`.
fn head_tag(title: &str, current: &str, description: &str) -> Markup {
    let canonical = format!("{SITE_ORIGIN}{current}");
    html! {
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1";
            meta name="color-scheme" content="light";
            meta name="robots" content="index, follow";
            meta name="description" content=(description);
            meta name="apple-mobile-web-app-title" content="PlausiDen";
            title { (title) }

            // Canonical + OpenGraph + Twitter card.
            link rel="canonical" href=(canonical);
            meta property="og:type" content="website";
            meta property="og:site_name" content="PlausiDen LLC";
            meta property="og:title" content=(title);
            meta property="og:description" content=(description);
            meta property="og:url" content=(canonical);
            meta property="og:image" content=(format!("{SITE_ORIGIN}/static/og-default.svg"));
            meta property="og:image:width" content="1200";
            meta property="og:image:height" content="630";
            meta property="og:image:alt" content="PlausiDen LLC — Privacy-first IT for the modern enterprise";
            meta name="twitter:card" content="summary_large_image";
            meta name="twitter:title" content=(title);
            meta name="twitter:description" content=(description);
            meta name="twitter:image" content=(format!("{SITE_ORIGIN}/static/og-default.svg"));

            // JSON-LD: tells crawlers who we are without parsing the page body.
            script type="application/ld+json" { (PreEscaped(JSON_LD_ORGANIZATION)) }

            link rel="icon" type="image/png" href="/static/favicon-96x96.png" sizes="96x96";
            link rel="icon" type="image/svg+xml" href="/static/favicon.svg";
            link rel="shortcut icon" href="/static/favicon.ico";
            link rel="apple-touch-icon" sizes="180x180" href="/static/apple-touch-icon.png";
            link rel="manifest" href="/static/site.webmanifest";
            link rel="stylesheet" href="/static/self-hosted-fonts.css";
            link rel="stylesheet" href="/static/index-CWVVhmVm.css";
            link rel="stylesheet" href="/static/animations.css";
            script src="/static/menu.js" defer {}
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
    }
    .render()
}

// Footer content lives in static slices so the typed Loom Footer
// primitive can borrow them by reference.
const FOOTER_COMPANY: &[FooterItem<'static>] = &[
    FooterItem::Link { href: "/", label: "Home" },
    FooterItem::Link { href: "/about", label: "About Us" },
    FooterItem::Link { href: "/services", label: "Services" },
    FooterItem::Link { href: "/blog", label: "Field Notes" },
    FooterItem::Link { href: "/how-we-work", label: "How We Work" },
    FooterItem::Link { href: "/pricing-transparency", label: "Pricing" },
    FooterItem::Link { href: "/contact", label: "Contact" },
];

const FOOTER_SOLUTIONS: &[FooterItem<'static>] = &[
    FooterItem::Link { href: "/solutions/legal", label: "Legal" },
    FooterItem::Link { href: "/solutions/healthcare", label: "Healthcare" },
    FooterItem::Link { href: "/solutions/journalism", label: "Journalism" },
    FooterItem::Link { href: "/solutions/financial-advisors", label: "Financial Advisors" },
    FooterItem::Link { href: "/solutions/nonprofit", label: "Nonprofits" },
    FooterItem::Text { text: "IT Operations" },
    FooterItem::Text { text: "Cyber Security" },
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
    FooterColumn { heading: "Company", items: FOOTER_COMPANY },
    FooterColumn { heading: "Solutions", items: FOOTER_SOLUTIONS },
    FooterColumn { heading: "Contact", items: FOOTER_CONTACT },
];

const FOOTER_LEGAL: &[FooterLegalLink<'static>] = &[
    FooterLegalLink { href: "/privacy-directive", label: "Sovereign Privacy Directive" },
    FooterLegalLink { href: "/terms-of-service", label: "Sovereign Terms of Service" },
];

/// Shared footer. Composed entirely from the typed `loom_components::Footer`
/// primitive. Visual contract preserved exactly.
fn footer() -> Markup {
    Footer {
        brand_logo: &icons::SHIELD,
        brand_name: "PlausiDen",
        brand_accent: "LLC",
        brand_tagline: "Providing comprehensive, high-quality IT solutions that empower modern enterprises. General yet specific excellence in technology.",
        columns: FOOTER_COLUMNS,
        copyright: "© PlausiDen LLC. All rights reserved.",
        legal_links: FOOTER_LEGAL,
    }
    .render()
}

/// Render a page with the site-wide chrome and the default site
/// description. Use [`page_with_description`] when a page wants a
/// page-specific description for OG/Twitter.
///
/// BUG ASSUMPTION: The nav + footer here mirror the production React site's
/// rendered DOM. Classes are Tailwind + shadcn/ui; styling lives in
/// `/static/index-CWVVhmVm.css` (copy of the production bundle).
///
/// SECURITY: The page declares two external origins: `fonts.googleapis.com`
/// (stylesheet) and `fonts.gstatic.com` (font binary). Matches the production
/// site's font loading. Consider self-hosting both Plus Jakarta Sans and
/// Outfit to revert the CSP relaxation.
#[must_use]
pub fn page(title: &str, current: &str, body: Markup) -> Markup {
    page_with_description(title, current, DEFAULT_DESCRIPTION, body)
}

/// Render a page with a per-page description. Used by views that
/// want their meta-description to differ from the site default
/// (vertical landing pages, individual blog posts, etc.).
#[must_use]
#[allow(clippy::needless_pass_by_value)] // Markup is PreEscaped<String>; consuming is idiomatic for a composition helper.
pub fn page_with_description(
    title: &str,
    current: &str,
    description: &str,
    body: Markup,
) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            (head_tag(title, current, description))
            body {
                div id="root" {
                    div class="flex flex-col min-h-screen font-body text-slate-900" {
                        (nav(current))
                        main class="flex-grow" {
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
    fn page_nav_uses_encrypted_inquiry_not_secure_drop() {
        // REGRESSION-GUARD: the renamed call-to-action must not silently
        // revert to the old 'Secure Drop' wording.
        let s = page("X", "/", html! {}).into_string();
        assert!(s.contains("Encrypted Inquiry"));
        assert!(!s.contains("Secure Drop"));
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
