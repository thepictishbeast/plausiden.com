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
/// public but COUPLING-EXEMPT in the audit since it's discovered
/// out-of-band (status.plausiden.com when that subdomain ships).
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

/// `GET /og/blog/:slug.svg` — dynamically render a 1200×630 SVG social-
/// preview card for one blog post. Same brand chrome as the default
/// card but with the post title and category baked in.
///
/// SECURITY: The slug is matched against the static `POSTS` registry —
/// only published posts get a card. Title/category strip to a known-
/// safe character set before rendering, so a hostile slug (already
/// blocked by the registry) couldn't inject SVG markup.
pub async fn og_blog(
    axum::extract::Path(slug_with_ext): axum::extract::Path<String>,
) -> impl IntoResponse {
    let slug = slug_with_ext.strip_suffix(".svg").unwrap_or(&slug_with_ext);
    let Some(post) = crate::views::posts::by_slug(slug) else {
        return (
            StatusCode::NOT_FOUND,
            [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
            String::from("not found\n"),
        );
    };

    let title_lines = wrap_for_og(post.title, 26);
    let mut title_blocks = String::new();
    use std::fmt::Write as _;
    for (i, line) in title_lines.iter().enumerate() {
        let y = 340 + (i as i32 * 70);
        let _ = write!(
            title_blocks,
            "<text x=\"80\" y=\"{y}\" font-family=\"system-ui, -apple-system, sans-serif\" font-size=\"60\" font-weight=\"700\" fill=\"#ffffff\">{line}</text>",
            line = svg_text_escape(line),
        );
    }

    let svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1200 630" width="1200" height="630" role="img" aria-label="{aria}">
<defs>
<linearGradient id="bg" x1="0" y1="0" x2="1" y2="1">
<stop offset="0%" stop-color="#0d488a"/>
<stop offset="100%" stop-color="#0a2c52"/>
</linearGradient>
<linearGradient id="accent" x1="0" y1="0" x2="1" y2="1">
<stop offset="0%" stop-color="#3b82f6"/>
<stop offset="100%" stop-color="#1e40af"/>
</linearGradient>
</defs>
<rect width="1200" height="630" fill="url(#bg)"/>
<g opacity="0.06" stroke="#ffffff" stroke-width="1">
<path d="M0 105H1200M0 210H1200M0 315H1200M0 420H1200M0 525H1200"/>
<path d="M150 0V630M300 0V630M450 0V630M600 0V630M750 0V630M900 0V630M1050 0V630"/>
</g>
<g transform="translate(80, 80)">
<rect x="0" y="0" width="56" height="56" rx="10" fill="url(#accent)"/>
<svg x="12" y="12" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
<path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/>
</svg>
<text x="74" y="40" font-family="system-ui, -apple-system, sans-serif" font-size="32" font-weight="700" fill="#ffffff">PlausiDen <tspan fill="#3b82f6">LLC</tspan></text>
</g>
<text x="80" y="240" font-family="system-ui, -apple-system, sans-serif" font-size="22" font-weight="600" fill="#3b82f6" letter-spacing="2">{category}</text>
{title_blocks}
<text x="80" y="560" font-family="system-ui, -apple-system, sans-serif" font-size="22" font-weight="400" fill="#cbd5e1">Field Notes · plausiden.com/blog</text>
</svg>
"##,
        aria = svg_text_escape(post.title),
        category = svg_text_escape(&post.category.to_uppercase()),
    );

    (
        StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "image/svg+xml; charset=utf-8")],
        svg,
    )
}

/// Wrap `text` into lines no longer than `max_chars`, splitting on
/// word boundaries. Returns at most 4 lines (truncating with an
/// ellipsis if the title overflows the card).
fn wrap_for_og(text: &str, max_chars: usize) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if current.is_empty() {
            current.push_str(word);
        } else if current.len() + 1 + word.len() <= max_chars {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(std::mem::take(&mut current));
            current.push_str(word);
            if lines.len() == 3 {
                break;
            }
        }
    }
    if !current.is_empty() && lines.len() < 4 {
        lines.push(current);
    }
    if lines.len() == 4 {
        let last = lines.last_mut().unwrap();
        if last.len() > max_chars - 1 {
            last.truncate(max_chars - 1);
        }
        last.push('…');
    }
    lines
}

/// Escape text for embedding inside an SVG `<text>` element body or
/// attribute. Same cover set as the XML escaper but reused locally to
/// keep the OG handler self-contained.
fn svg_text_escape(s: &str) -> String {
    xml_escape(s)
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
    ("/how-we-work", "monthly", "0.7"),
    ("/pricing-transparency", "monthly", "0.7"),
    ("/privacy-directive", "yearly", "0.4"),
    ("/terms-of-service", "yearly", "0.4"),
];

/// `GET /sitemap.xml` — auto-generated from `SITEMAP_ROUTES` + every
/// blog-post slug. Search engines fetch this; humans don't.
pub async fn sitemap_xml() -> impl IntoResponse {
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
            "  <url><loc>https://plausiden.com{path}</loc><lastmod>{date}</lastmod><changefreq>{changefreq}</changefreq><priority>{priority}</priority></url>",
            date = latest_post_date,
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
  <title>PlausiDen — Field Notes</title>
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
    async fn contact_contains_encrypted_inquiry_heading() {
        let markup = contact().await.into_string();
        assert!(
            markup.contains("Encrypted Inquiry"),
            "expected 'Encrypted Inquiry' heading in /contact body"
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
