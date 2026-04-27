//! `plausiden-site` entrypoint.
//!
//! Design principles: one binary, zero state, zero third-party, zero logs by default.
//! Everything user-visible is either a static file or a compile-time-rendered `Maud` view.
//!
//! Governed by the `PlausiDen` AVP Doctrine. Every public function carries a
//! `BUG ASSUMPTION:` annotation; every defense-in-depth carries a `SECURITY:`
//! annotation (see `annotations/README.md` in the doctrine repo).

#![doc(html_no_source)]
// The lib carving exposed several internal items as pub. These lints
// fire on existing code that was previously pub(crate); they're
// noisy without changing correctness. Allow at the lib level until
// the conciseness audit (PlausiDen-Audits/audits/conciseness) does
// a sweep.
#![allow(clippy::doc_markdown)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::too_long_first_doc_paragraph)]

use std::time::Duration;

use axum::Router;

pub mod components;
pub mod handlers;
pub mod inquiry;
pub mod sandbox;
pub mod security;
pub mod views;

/// Request processing timeout. Matches the `TimeoutLayer` installed below.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

/// Construct the fully-wired router. Exposed at crate scope so tests can hit
/// the same graph the production binary serves.
///
/// BUG ASSUMPTION: Layer ordering matters — compression must not run before the
/// security headers are installed, or the headers could disappear from errored
/// responses that short-circuit past the header layer.
///
/// SECURITY: The security-headers layer is applied first so every response
/// (including 404, 500, timeout, large-body-rejected) carries the lockdown
/// headers. The static file service is nested under `/static` and cannot
/// traverse outside that directory (see [`tower_http::services::ServeDir`]).
pub fn build_router(inquiry_state: inquiry::InquiryState) -> Router {
    use axum::http::StatusCode;
    use axum::routing::get;
    use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, trace::TraceLayer};

    Router::new()
        .route("/", get(handlers::home))
        .route("/services", get(handlers::services))
        .route("/about", get(handlers::about))
        .route("/case-studies", get(handlers::case_studies))
        .route("/contact", get(handlers::contact).post(inquiry::submit))
        .route("/blog", get(handlers::blog_index))
        .route("/blog/{slug}", get(handlers::blog_post))
        .route("/solutions/legal", get(handlers::solutions_legal))
        .route("/solutions/healthcare", get(handlers::solutions_healthcare))
        .route("/solutions/journalism", get(handlers::solutions_journalism))
        .route(
            "/solutions/financial-advisors",
            get(handlers::solutions_financial_advisors),
        )
        .route("/solutions/nonprofit", get(handlers::solutions_nonprofit))
        .route("/how-we-work", get(handlers::how_we_work))
        .route("/pricing-transparency", get(handlers::pricing))
        .route("/sitemap.xml", get(handlers::sitemap_xml)) // COUPLING-EXEMPT: served to crawlers, not clicked from UI
        .route("/robots.txt", get(handlers::robots_txt)) // COUPLING-EXEMPT: served to crawlers, not clicked from UI
        .route("/blog/rss.xml", get(handlers::blog_rss))
        .route("/privacy-directive", get(handlers::privacy))
        .route("/terms-of-service", get(handlers::terms))
        .route("/healthz", get(handlers::healthz)) // COUPLING-EXEMPT: internal liveness probe, never advertised
        .route("/status", get(handlers::status)) // COUPLING-EXEMPT: discovered via status.plausiden.com out-of-band, not via in-site nav
        .nest_service(
            "/static",
            // Long-cache the static dir. CSS bundle name + favicon are
            // content-addressed; if a file changes we'll bump its name.
            // `immutable` lets browsers skip revalidation entirely.
            tower::ServiceBuilder::new()
                .layer(
                    tower_http::set_header::SetResponseHeaderLayer::if_not_present(
                        axum::http::header::CACHE_CONTROL,
                        axum::http::HeaderValue::from_static("public, max-age=604800, immutable"),
                    ),
                )
                .service(tower_http::services::ServeDir::new("static")),
        )
        .with_state(inquiry_state)
        .layer(security::headers_layer())
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            REQUEST_TIMEOUT,
        ))
        .layer(TraceLayer::new_for_http())
        .fallback(handlers::not_found)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    /// Router wiring sanity: the root route returns 200 and renders the
    /// homepage heading.
    #[tokio::test]
    async fn root_returns_home() {
        let app = build_router(crate::inquiry::InquiryState::new());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = to_bytes(resp.into_body(), 64 * 1024).await.unwrap();
        let s = std::str::from_utf8(&body).unwrap();
        assert!(
            s.contains("Professional IT Solutions"),
            "home body eyebrow missing"
        );
        assert!(
            s.contains("Modern Enterprise"),
            "home body hero headline missing"
        );
    }

    /// An unknown path returns 404 with the not-found view, not a 500 or a
    /// raw string.
    #[tokio::test]
    async fn unknown_path_returns_styled_404() {
        let app = build_router(crate::inquiry::InquiryState::new());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/does-not-exist")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let body = to_bytes(resp.into_body(), 32 * 1024).await.unwrap();
        assert!(std::str::from_utf8(&body).unwrap().contains("Nothing here"));
    }

    /// Every route stamps the core security headers. Spot-check three of them
    /// on a fresh request.
    #[tokio::test]
    async fn security_headers_are_stamped() {
        let app = build_router(crate::inquiry::InquiryState::new());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let h = resp.headers();
        assert!(h.contains_key("content-security-policy"));
        assert!(h.contains_key("strict-transport-security"));
        assert!(h.contains_key("referrer-policy"));
    }

    /// `/blog` lists the published posts; the most recent is linked.
    #[tokio::test]
    async fn blog_index_links_known_post() {
        let app = build_router(crate::inquiry::InquiryState::new());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/blog")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = to_bytes(resp.into_body(), 64 * 1024).await.unwrap();
        let s = std::str::from_utf8(&body).unwrap();
        assert!(s.contains("Field Notes"));
        assert!(s.contains("/blog/federated-rule-learning"));
    }

    /// `/blog/<known-slug>` returns 200 + the post body.
    #[tokio::test]
    async fn blog_post_returns_known_post() {
        let app = build_router(crate::inquiry::InquiryState::new());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/blog/federated-rule-learning")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = to_bytes(resp.into_body(), 128 * 1024).await.unwrap();
        let s = std::str::from_utf8(&body).unwrap();
        assert!(s.contains("Federated rule learning"));
        // Excerpt's signature line should be in the body
        assert!(s.contains("compose, don't compromise"));
    }

    /// `/sitemap.xml` lists every public route + every published post.
    #[tokio::test]
    async fn sitemap_lists_routes_and_posts() {
        let app = build_router(crate::inquiry::InquiryState::new());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/sitemap.xml")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let ct = resp
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap();
        assert!(ct.contains("xml"));
        let body = to_bytes(resp.into_body(), 16 * 1024).await.unwrap();
        let s = std::str::from_utf8(&body).unwrap();
        assert!(s.contains("<urlset"));
        assert!(s.contains("https://plausiden.com/"));
        assert!(s.contains("https://plausiden.com/solutions/legal"));
        assert!(s.contains("https://plausiden.com/blog/federated-rule-learning"));
        // Healthz must NOT be listed — internal liveness only.
        assert!(!s.contains("/healthz"));
    }

    /// `/blog/rss.xml` returns an Atom feed of published posts.
    #[tokio::test]
    async fn blog_rss_emits_atom_feed() {
        let app = build_router(crate::inquiry::InquiryState::new());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/blog/rss.xml")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let ct = resp
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap();
        assert!(ct.contains("atom") || ct.contains("xml"));
        let body = to_bytes(resp.into_body(), 32 * 1024).await.unwrap();
        let s = std::str::from_utf8(&body).unwrap();
        assert!(s.contains("<feed"));
        assert!(s.contains("<entry>"));
        assert!(s.contains("https://plausiden.com/blog/federated-rule-learning"));
        assert!(s.contains("https://plausiden.com/blog/avp-doctrine"));
    }

    /// `/robots.txt` allows everything and points at the sitemap.
    #[tokio::test]
    async fn robots_txt_points_at_sitemap() {
        let app = build_router(crate::inquiry::InquiryState::new());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/robots.txt")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = to_bytes(resp.into_body(), 4 * 1024).await.unwrap();
        let s = std::str::from_utf8(&body).unwrap();
        assert!(s.contains("User-agent: *"));
        assert!(s.contains("Sitemap: https://plausiden.com/sitemap.xml"));
    }

    /// Every page emits OpenGraph + Twitter card metadata.
    #[tokio::test]
    async fn pages_emit_og_metadata() {
        let app = build_router(crate::inquiry::InquiryState::new());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/blog/federated-rule-learning")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = to_bytes(resp.into_body(), 128 * 1024).await.unwrap();
        let s = std::str::from_utf8(&body).unwrap();
        assert!(s.contains(r#"property="og:title""#));
        assert!(s.contains(r#"property="og:description""#));
        assert!(s.contains(r#"property="og:url""#));
        assert!(s.contains(r#"name="twitter:card""#));
        // Per-page description must be the post's excerpt, not the
        // site default — confirms page_with_description is wired.
        assert!(s.contains("How sorting rules can get smarter"));
        // JSON-LD Organization
        assert!(s.contains("application/ld+json"));
        assert!(s.contains("\"PlausiDen LLC\""));
    }

    /// `/blog/<unknown-slug>` returns 404 with the styled not-found.
    #[tokio::test]
    async fn blog_post_returns_404_for_unknown_slug() {
        let app = build_router(crate::inquiry::InquiryState::new());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/blog/never-written")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let body = to_bytes(resp.into_body(), 32 * 1024).await.unwrap();
        assert!(std::str::from_utf8(&body).unwrap().contains("Nothing here"));
    }

    /// Pages must emit no inline `style="…"` attributes and no
    /// `<style>` blocks. The CSP forbids them; this test catches a
    /// regression at PR-time, before the browser refuses to apply
    /// the style and the visual breaks silently.
    ///
    /// REGRESSION-GUARD: dropped `'unsafe-inline'` from style-src on
    /// 2026-04-27 after confirming zero inline styles in every
    /// rendered snapshot. Any future inline emission must either
    /// remove it or explicitly relax CSP, never both silently.
    #[tokio::test]
    async fn csp_no_inline_styles_emitted() {
        for path in [
            "/",
            "/services",
            "/about",
            "/contact",
            "/blog",
            "/blog/why-thundercrab",
            "/solutions/legal",
            "/pricing-transparency",
        ] {
            let body = fetch_body(path).await;
            assert!(
                !body.contains("style=\""),
                "{path}: inline style= emitted; CSP forbids it"
            );
            assert!(
                !body.contains("<style"),
                "{path}: inline <style> block emitted; CSP forbids it"
            );
        }
    }

    /// Health check is cheap, body-only, and does not set cookies.
    #[tokio::test]
    async fn healthz_is_cookie_free() {
        let app = build_router(crate::inquiry::InquiryState::new());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/healthz")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(resp.headers().get("set-cookie").is_none());
    }

    /// Fetch one route through the router and return its decoded body.
    ///
    /// BUG ASSUMPTION: Bodies fit in 256 KiB. The largest snapshot today
    /// (a long blog post) is ~50 KiB — leaves 5× headroom for ordinary
    /// growth before the cap needs revisiting.
    pub(super) async fn fetch_body(path: &str) -> String {
        let app = build_router(crate::inquiry::InquiryState::new());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri(path)
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            resp.status(),
            StatusCode::OK,
            "unexpected status for {path}"
        );
        let body = to_bytes(resp.into_body(), 256 * 1024).await.unwrap();
        String::from_utf8(body.to_vec()).expect("rendered body must be utf-8")
    }
}

/// Snapshot tests for every public route. Any byte-level change to a
/// rendered page must be approved with `cargo insta review` before it
/// can land — making accidental layout regressions impossible to merge
/// silently.
///
/// SECURITY: These tests do not touch the network (the in-process
/// router serves them) and use a fresh `InquiryState` per test, so
/// they cannot leak SMTP credentials or cross-test rate-limit state.
#[cfg(test)]
mod snapshots {
    use super::tests::fetch_body;

    /// Drive the assertion for one route. Insta dedupes on the snapshot
    /// name, so passing the route's unique name keeps every snapshot
    /// in its own `.snap` file.
    macro_rules! snap_route {
        ($name:ident, $path:expr) => {
            #[tokio::test]
            async fn $name() {
                let body = fetch_body($path).await;
                insta::assert_snapshot!(stringify!($name), body);
            }
        };
    }

    snap_route!(home, "/");
    snap_route!(services, "/services");
    snap_route!(about, "/about");
    snap_route!(case_studies, "/case-studies");
    snap_route!(contact, "/contact");
    snap_route!(blog_index, "/blog");
    snap_route!(blog_post_federated, "/blog/federated-rule-learning");
    snap_route!(blog_post_avp, "/blog/avp-doctrine");
    snap_route!(blog_post_provable_privacy, "/blog/provable-privacy");
    snap_route!(blog_post_why_thundercrab, "/blog/why-thundercrab");
    snap_route!(
        blog_post_plausible_deniability,
        "/blog/plausible-deniability"
    );
    snap_route!(solutions_legal, "/solutions/legal");
    snap_route!(solutions_healthcare, "/solutions/healthcare");
    snap_route!(solutions_journalism, "/solutions/journalism");
    snap_route!(
        solutions_financial_advisors,
        "/solutions/financial-advisors"
    );
    snap_route!(solutions_nonprofit, "/solutions/nonprofit");
    snap_route!(how_we_work, "/how-we-work");
    snap_route!(pricing, "/pricing-transparency");
    snap_route!(privacy, "/privacy-directive");
    snap_route!(terms, "/terms-of-service");
    snap_route!(sitemap, "/sitemap.xml");
    snap_route!(robots, "/robots.txt");
    snap_route!(blog_rss, "/blog/rss.xml");
}
