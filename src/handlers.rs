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

/// Static list of public routes included in `/sitemap.xml`.
/// `/healthz` is intentionally excluded — internal liveness probe.
const SITEMAP_ROUTES: &[&str] = &[
    "/",
    "/services",
    "/about",
    "/blog",
    "/contact",
    "/solutions/legal",
    "/solutions/healthcare",
    "/solutions/journalism",
    "/privacy-directive",
    "/terms-of-service",
];

/// `GET /sitemap.xml` — auto-generated from `SITEMAP_ROUTES` + every
/// blog-post slug. Search engines fetch this; humans don't.
pub async fn sitemap_xml() -> impl IntoResponse {
    use std::fmt::Write as _;
    let mut out = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#,
    );
    for path in SITEMAP_ROUTES {
        let _ = writeln!(out, "  <url><loc>https://plausiden.com{path}</loc></url>");
    }
    for post in crate::views::posts::POSTS {
        let _ = writeln!(
            out,
            "  <url><loc>https://plausiden.com/blog/{slug}</loc><lastmod>{date}</lastmod></url>",
            slug = post.slug,
            date = post.published,
        );
    }
    out.push_str("</urlset>\n");
    (
        [(axum::http::header::CONTENT_TYPE, "application/xml")],
        out,
    )
}

/// `GET /robots.txt` — allow everything, point at the sitemap.
pub async fn robots_txt() -> impl IntoResponse {
    (
        [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "User-agent: *\nAllow: /\nSitemap: https://plausiden.com/sitemap.xml\n",
    )
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
