//! Axum request handlers. Thin shims over the Maud views in [`crate::views`].
//!
//! Every handler is stateless. No handler takes an `Extension`, `State`, or
//! cookie — the site has none. A handler that compiles is already cookie-free
//! and session-free by construction.

use axum::http::StatusCode;
use axum::response::IntoResponse;
use maud::Markup;

/// Render the homepage (`GET /`).
///
/// BUG ASSUMPTION: Rendering is pure and cannot fail; if the Maud macro
/// generates invalid UTF-8 the compiler would have rejected the crate.
pub async fn home() -> Markup {
    crate::views::home::render()
}

/// Render the services overview (`GET /services`).
///
/// BUG ASSUMPTION: Same purity guarantee as [`home`].
pub async fn services() -> Markup {
    crate::views::services::render()
}

/// Render the about page (`GET /about`).
///
/// BUG ASSUMPTION: Same purity guarantee as [`home`].
pub async fn about() -> Markup {
    crate::views::about::render()
}

/// Render the case-studies index (`GET /case-studies`).
///
/// BUG ASSUMPTION: Same purity guarantee as [`home`]. Studies are
/// hard-coded constants vetted at authoring time; nothing client-
/// identifying is interpolated at render time.
pub async fn case_studies() -> Markup {
    crate::views::case_studies::render()
}

/// Render the in-house-stack page (`GET /capabilities`).
///
/// BUG ASSUMPTION: Same purity guarantee as [`home`]. Copy is
/// intentionally general per the "tools mature" doctrine — no
/// version pins, no vendor names, no feature lists.
pub async fn capabilities() -> Markup {
    crate::views::capabilities::render()
}

/// Render the feedback + testimonial form (`GET /feedback`).
pub async fn feedback() -> Markup {
    crate::views::feedback::render()
}

/// Render the operational status page (`GET /status`). Self-reports
/// the running process's uptime + build identity. The route is
/// public but COUPLING-EXEMPT: discovered out-of-band (status.plausiden.com
/// when that subdomain ships), not linked from the marketing UI.
pub async fn status() -> Markup {
    crate::views::status::render()
}

/// Render the RSS / Atom subscribe instructions page (`GET /subscribe`).
pub async fn subscribe() -> Markup {
    crate::views::subscribe::render()
}

/// Render the Sovereign Privacy Directive placeholder (`GET /privacy-directive`).
pub async fn privacy() -> Markup {
    crate::views::legal::privacy()
}

/// Render the Sovereign Terms of Service placeholder (`GET /terms-of-service`).
pub async fn terms() -> Markup {
    crate::views::legal::terms()
}

/// Render the Encrypted Inquiry form (`GET /contact`).
///
/// BUG ASSUMPTION: v1 returns a plain HTML form. v1.1 will progressively enhance
/// with WASM-side age encryption; until then form POSTs hit a handler (not yet
/// wired) that must validate a double-submit CSRF nonce and rate-limit per IP.
pub async fn contact() -> Markup {
    crate::views::contact::render()
}

/// Render the blog index (`GET /blog`).
pub async fn blog_index() -> Markup {
    crate::views::blog::index()
}

/// Render the legal-vertical landing page (`GET /solutions/legal`).
pub async fn solutions_legal() -> Markup {
    crate::views::solutions::legal::render()
}

/// Render the healthcare-vertical landing page (`GET /solutions/healthcare`).
pub async fn solutions_healthcare() -> Markup {
    crate::views::solutions::healthcare::render()
}

/// Render the journalism-vertical landing page (`GET /solutions/journalism`).
pub async fn solutions_journalism() -> Markup {
    crate::views::solutions::journalism::render()
}

/// Render the financial-advisors-vertical landing page.
pub async fn solutions_financial_advisors() -> Markup {
    crate::views::solutions::financial_advisors::render()
}

/// Render the nonprofit-vertical landing page.
pub async fn solutions_nonprofit() -> Markup {
    crate::views::solutions::nonprofit::render()
}

/// Render `/sample-report` — the deliverable, published before purchase.
pub async fn sample_report() -> Markup {
    crate::views::sample_report::render()
}

/// Render `/how-we-work` — engagement model + four commitments.
pub async fn how_we_work() -> Markup {
    crate::views::how_we_work::render()
}

/// Render `/pricing-transparency` — concrete rate ranges + posture.
pub async fn pricing() -> Markup {
    crate::views::pricing::render()
}

/// Render an individual blog post (`GET /blog/:slug`). Returns the
/// styled 404 view for unknown slugs.
///
/// BUG ASSUMPTION: Axum extracts `slug` from the path; we treat unknown
/// slugs as not-found rather than redirecting to the index, so external
/// links to a removed post fail loudly instead of silently shifting.
pub async fn blog_post(
    axum::extract::Path(slug): axum::extract::Path<String>,
) -> (StatusCode, Markup) {
    crate::views::blog::post(&slug).map_or_else(
        || (StatusCode::NOT_FOUND, crate::views::not_found::render()),
        |body| (StatusCode::OK, body),
    )
}
/// Fallback handler for unmatched paths. Returns 404 with a styled page.
///
/// BUG ASSUMPTION: The `404 + Markup` tuple is picked up by Axum's
/// `IntoResponse` impl and becomes a correctly-statused HTML response. This is
/// exercised in the router test in `main.rs`.
pub async fn not_found() -> (StatusCode, Markup) {
    (StatusCode::NOT_FOUND, crate::views::not_found::render())
}

/// Public routes included in `/sitemap.xml`, with hint metadata for
/// crawlers: `changefreq` (how often we expect the page to change) and
/// `priority` (relative importance vs. other URLs on the same site,
/// 0.0–1.0). `/healthz` is intentionally excluded.
const SITEMAP_ROUTES: &[(&str, &str, &str)] = &[
    ("/", "weekly", "1.0"),
    ("/services", "monthly", "0.9"),
    ("/capabilities", "monthly", "0.9"),
    ("/case-studies", "monthly", "0.8"),
    ("/about", "monthly", "0.7"),
    ("/contact", "yearly", "0.8"),
    ("/feedback", "yearly", "0.6"),
    ("/blog", "weekly", "0.9"),
    ("/subscribe", "yearly", "0.5"),
    ("/solutions/legal", "monthly", "0.8"),
    ("/solutions/healthcare", "monthly", "0.8"),
    ("/solutions/journalism", "monthly", "0.8"),
    ("/solutions/financial-advisors", "monthly", "0.8"),
    ("/solutions/nonprofit", "monthly", "0.8"),
    ("/sample-report", "monthly", "0.8"),
    ("/how-we-work", "monthly", "0.7"),
    ("/pricing-transparency", "monthly", "0.7"),
    ("/privacy-directive", "yearly", "0.4"),
    ("/terms-of-service", "yearly", "0.4"),
];

/// `GET /sitemap.xml` — auto-generated from `SITEMAP_ROUTES` + every
/// blog-post slug + every CMS-backed published page under
/// `/docs/{slug}`. Search engines fetch this; humans don't.
pub async fn sitemap_xml(
    axum::extract::State(cms): axum::extract::State<crate::cms::CmsState>,
) -> impl IntoResponse {
    use std::fmt::Write as _;
    let latest_post_date = crate::views::posts::POSTS
        .first()
        .map_or("2026-01-01", |p| p.published);
    let mut out = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#,
    );
    for (path, changefreq, priority) in SITEMAP_ROUTES {
        let _ = writeln!(
            out,
            "  <url><loc>https://plausiden.com{path}</loc><lastmod>{latest_post_date}</lastmod><changefreq>{changefreq}</changefreq><priority>{priority}</priority></url>",
        );
    }
    for post in crate::views::posts::POSTS {
        let _ = writeln!(
            out,
            "  <url><loc>https://plausiden.com/blog/{slug}</loc><lastmod>{date}</lastmod><changefreq>yearly</changefreq><priority>0.7</priority></url>",
            slug = post.slug,
            date = post.published,
        );
    }
    // CMS-backed pages — only the Published ones get sitemap entries.
    // Drafts / Reviewed / Archived are excluded by construction so an
    // accidental publish-by-sitemap-listing is impossible.
    for entry in cms.published_entries() {
        let _ = writeln!(
            out,
            "  <url><loc>https://plausiden.com/docs/{slug}</loc><lastmod>{date}</lastmod><changefreq>monthly</changefreq><priority>0.6</priority></url>",
            slug = entry.slug,
            date = entry.updated_at,
        );
    }
    out.push_str("</urlset>\n");
    ([(axum::http::header::CONTENT_TYPE, "application/xml")], out)
}

/// `GET /robots.txt` — allow everything, point at the sitemap.
pub async fn robots_txt() -> impl IntoResponse {
    (
        [(
            axum::http::header::CONTENT_TYPE,
            "text/plain; charset=utf-8",
        )],
        "User-agent: *\nAllow: /\nSitemap: https://plausiden.com/sitemap.xml\n",
    )
}

/// `GET /blog/rss.xml` — Atom feed of every published post. Auto-
/// generated from the same `POSTS` registry the index uses.
///
/// SECURITY: We emit only metadata (title, excerpt, link, date,
/// category). No author email, no IP, no analytics token. Feed
/// readers + LLM crawlers can ingest the firehose without any
/// per-reader identifier.
pub async fn blog_rss() -> impl IntoResponse {
    use std::fmt::Write as _;
    let mut out = String::with_capacity(4096);
    out.push_str(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>PlausiDen — Insights</title>
  <subtitle>Notes on infrastructure, privacy, and how we work.</subtitle>
  <link href="https://plausiden.com/blog" rel="alternate"/>
  <link href="https://plausiden.com/blog/rss.xml" rel="self"/>
  <id>https://plausiden.com/blog</id>
"#,
    );
    if let Some(latest) = crate::views::posts::POSTS.first() {
        let _ = writeln!(out, "  <updated>{}T00:00:00Z</updated>", latest.published);
    }
    for post in crate::views::posts::POSTS {
        let _ = writeln!(
            out,
            "  <entry>\n\
             \x20\x20\x20\x20<title>{title}</title>\n\
             \x20\x20\x20\x20<link href=\"https://plausiden.com/blog/{slug}\" rel=\"alternate\"/>\n\
             \x20\x20\x20\x20<id>https://plausiden.com/blog/{slug}</id>\n\
             \x20\x20\x20\x20<published>{date}T00:00:00Z</published>\n\
             \x20\x20\x20\x20<updated>{date}T00:00:00Z</updated>\n\
             \x20\x20\x20\x20<category term=\"{category}\"/>\n\
             \x20\x20\x20\x20<summary>{excerpt}</summary>\n\
             \x20\x20</entry>",
            title = xml_escape(post.title),
            slug = post.slug,
            date = post.published,
            category = xml_escape(post.category),
            excerpt = xml_escape(post.excerpt),
        );
    }
    out.push_str("</feed>\n");
    (
        [(
            axum::http::header::CONTENT_TYPE,
            "application/atom+xml; charset=utf-8",
        )],
        out,
    )
}

/// Minimal XML escaper for `<`, `>`, `&`, `"`, `'`. Sufficient for
/// short text inside element bodies and attribute values.
fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        match c {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(c),
        }
    }
    out
}

/// Liveness probe (`GET /healthz`). Used by local health-checks, not advertised
/// in the page navigation.
///
/// BUG ASSUMPTION: Returning a plain `"ok"` body is intentional — machine
/// readers expect a short, stable response, not JSON. Do not expose process
/// internals here (would leak fingerprinting information).
///
/// SECURITY: Intentionally returns no body beyond `ok`. No version string, no
/// hostname, no uptime — anything more is a fingerprinting signal.
pub async fn healthz() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn home_renders_nonempty() {
        let markup = home().await;
        assert!(!markup.into_string().is_empty());
    }

    #[tokio::test]
    async fn services_renders_nonempty() {
        let markup = services().await;
        assert!(!markup.into_string().is_empty());
    }

    #[tokio::test]
    async fn contact_offers_a_way_to_make_contact() {
        // This replaces a test that claimed to check for an "Encrypted Inquiry"
        // heading in the /contact body. There was never such a heading — the
        // assertion was satisfied by the nav CTA that every page embeds, so it
        // passed while testing nothing about this page. Worse, the label
        // advertised an encrypted intake the page does not implement.
        //
        // Assert what the page must genuinely provide instead: a reachable
        // human and a working form.
        let markup = contact().await.into_string();
        assert!(
            markup.contains("team@plausiden.com"),
            "email address present"
        );
        assert!(markup.contains("978-351-6495"), "phone number present");
        assert!(markup.contains("<form"), "message form present");
        assert!(
            !markup.contains("Encrypted Inquiry"),
            "do not advertise an encrypted intake until one exists"
        );
    }

    #[tokio::test]
    async fn contact_does_not_mention_old_secure_drop() {
        // REGRESSION-GUARD: the old site called this form "Secure Drop"; that
        // naming collides with the SecureDrop whistleblower platform. Renamed
        // in the current-site commit 95a57fb; must not regress here.
        let markup = contact().await.into_string();
        assert!(
            !markup.contains("Secure Drop"),
            "Secure Drop leaked back into /contact view"
        );
    }

    #[tokio::test]
    async fn not_found_returns_404_status() {
        let (status, markup) = not_found().await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(!markup.into_string().is_empty());
    }

    #[tokio::test]
    async fn healthz_is_short_and_cookie_free() {
        let resp = healthz().await.into_response();
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(resp.headers().get("set-cookie").is_none());
        let body = axum::body::to_bytes(resp.into_body(), 1024).await.unwrap();
        assert_eq!(&body[..], b"ok");
    }
}
