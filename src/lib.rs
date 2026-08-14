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
use axum::extract::FromRef;

pub mod admin;
pub mod cms;
pub mod components;
pub mod feedback_store;
pub mod handlers;
pub mod inquiry;
pub mod sandbox;
pub mod security;
pub mod views;

/// Request processing timeout. Matches the `TimeoutLayer` installed below.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

/// Aggregate axum state. Holds every per-process resource a handler
/// might need (currently inquiry-form rate limiter + SMTP transport,
/// CMS storage). Each substate is exposed via `FromRef` so a
/// handler extracts only what it depends on.
///
/// Manual `Debug` (no derive) because `InquiryState` carries an SMTP
/// transport that does not implement `Debug`.
#[derive(Clone)]
pub struct AppState {
    /// Inquiry-form state (rate limiter, SMTP transport, feedback store).
    pub inquiry: inquiry::InquiryState,
    /// CMS storage state. `CmsState::default()` when no store is
    /// configured; `/docs/*` then 404s.
    pub cms: cms::CmsState,
}

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState")
            .field("cms", &self.cms)
            .finish_non_exhaustive()
    }
}

impl FromRef<AppState> for inquiry::InquiryState {
    fn from_ref(s: &AppState) -> Self {
        s.inquiry.clone()
    }
}

impl FromRef<AppState> for cms::CmsState {
    fn from_ref(s: &AppState) -> Self {
        s.cms.clone()
    }
}

/// Construct the fully-wired router. Exposed at crate scope so tests can hit
/// the same graph the production binary serves. CMS state is taken from
/// `PLAUSIDEN_CMS_ROOT` env var; tests that need to point CMS at a fixture
/// directory call [`build_router_with_state`] directly.
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
    build_router_with_state(AppState {
        inquiry: inquiry_state,
        cms: cms::CmsState::from_env(),
    })
}

/// Construct the router with an explicit [`AppState`]. Used by integration
/// tests that need to inject a CMS storage pointed at a fixture directory.
pub fn build_router_with_state(state: AppState) -> Router {
    use axum::http::StatusCode;
    use axum::routing::get;
    use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, trace::TraceLayer};

    Router::new()
        .route("/", get(handlers::home))
        .route("/services", get(handlers::services))
        .route("/about", get(handlers::about))
        .route("/capabilities", get(handlers::capabilities))
        .route("/case-studies", get(handlers::case_studies))
        .route("/contact", get(handlers::contact).post(inquiry::submit))
        .route(
            "/feedback",
            get(handlers::feedback).post(inquiry::feedback_submit),
        )
        .route("/feedback/export", get(inquiry::feedback_export)) // COUPLING-EXEMPT: admin token-gated, never linked from UI
        .route("/admin", get(admin::admin_root))
        .route(
            "/admin/login",
            get(admin::login_form).post(admin::login_post),
        )
        .route("/admin/login/verify", get(admin::verify)) // COUPLING-EXEMPT: hit only via emailed magic link, not via a UI href
        .route("/admin/logout", axum::routing::post(admin::logout))
        .route("/admin/feedback", get(admin::feedback_dashboard)) // COUPLING-EXEMPT: reached via /admin redirect after sign-in, not via a UI href
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
        .route("/sample-report", get(handlers::sample_report))
        .route("/how-we-work", get(handlers::how_we_work))
        .route("/pricing-transparency", get(handlers::pricing))
        .route("/sitemap.xml", get(handlers::sitemap_xml)) // COUPLING-EXEMPT: served to crawlers, not clicked from UI
        .route("/robots.txt", get(handlers::robots_txt)) // COUPLING-EXEMPT: served to crawlers, not clicked from UI
        .route("/blog/rss.xml", get(handlers::blog_rss)) // COUPLING-EXEMPT: surfaced as a copyable absolute URL on /subscribe (not as a UI <a href>), and consumed by RSS readers, not clicked from the site
        .route("/privacy-directive", get(handlers::privacy))
        .route("/terms-of-service", get(handlers::terms))
        .route("/subscribe", get(handlers::subscribe))
        .route("/healthz", get(handlers::healthz)) // COUPLING-EXEMPT: internal liveness probe, never advertised
        .route("/status", get(handlers::status)) // COUPLING-EXEMPT: discovered via status.plausiden.com out-of-band, not via in-site nav
        // CMS-backed content. The store is opened lazily (see
        // [`crate::cms`]); when not configured the route 404s, so
        // adding it costs nothing on a deployment that doesn't yet
        // ship CMS pages.
        .route("/docs/{slug}", get(cms::serve_doc))
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
        .with_state(state)
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
            s.contains("IT, security and disaster recovery"),
            "home body eyebrow missing"
        );
        assert!(
            s.contains("confidential client data"),
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
        assert!(s.contains("Insights"));
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

    /// A post's Open Graph card is served as a real PNG.
    ///
    /// This replaced `/og/blog/<slug>.svg`, a route that rendered a card on
    /// demand as image/svg+xml. No link preview renders SVG, so every post
    /// shared anywhere showed no image. The cards are now pre-rendered files.
    #[tokio::test]
    async fn blog_og_card_is_a_real_png() {
        let app = build_router(crate::inquiry::InquiryState::new());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/static/og/blog-federated-rule-learning.png")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let ct = resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default()
            .to_owned();
        assert!(ct.contains("png"), "expected a PNG content-type, got {ct}");
        let body = to_bytes(resp.into_body(), 512 * 1024).await.unwrap();
        // PNG magic. A card that is secretly still SVG would sail past a
        // filename check, which is exactly how the old one survived.
        assert_eq!(
            &body[..8],
            b"\x89PNG\r\n\x1a\n",
            "the card is not a PNG despite the .png extension"
        );
    }

    /// Per-post pages emit the per-post og:image URL and og:type=article.
    #[tokio::test]
    async fn blog_post_emits_per_post_og_image_and_article_type() {
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
        assert!(
            s.contains("/static/og/blog-federated-rule-learning.png"),
            "per-post og:image URL missing"
        );
        assert!(
            s.contains(r#"property="og:type" content="article""#),
            "og:type=article missing"
        );
        // Article JSON-LD
        assert!(s.contains(r#""@type":"Article""#));
        assert!(s.contains(r#""datePublished":"2026-04-26""#));
    }

    /// `/docs/{slug}` round-trip — the seeded `why-pps` page renders a 200
    /// with hero + heading-body + cta content visible.
    ///
    /// The CMS state is constructed explicitly from the manifest-relative
    /// fixture directory so the test does not race the production env-var
    /// path. No `unsafe_code` — the state is injected through axum, not
    /// pulled from process-global state.
    #[tokio::test]
    async fn docs_slug_serves_published_page() {
        let app = build_router_with_state(cms_test_state());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/docs/why-pps")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = to_bytes(resp.into_body(), 64 * 1024).await.unwrap();
        let s = std::str::from_utf8(&body).unwrap();
        assert!(s.contains("Why Plausible Privacy Software"));
        assert!(s.contains("substrate"));
        assert!(s.contains("Start the conversation"));
    }

    /// Unknown CMS slugs return 404 with the styled not-found view.
    #[tokio::test]
    async fn docs_unknown_slug_returns_styled_404() {
        let app = build_router_with_state(cms_test_state());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/docs/never-published")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        let body = to_bytes(resp.into_body(), 32 * 1024).await.unwrap();
        assert!(std::str::from_utf8(&body).unwrap().contains("Nothing here"));
    }

    /// Build an [`AppState`] whose CMS layer is opened on the
    /// manifest-relative `cms-store/` fixture. Cargo runs tests with
    /// `CARGO_MANIFEST_DIR` set to the crate root so the path
    /// resolves without an absolute base.
    fn cms_test_state() -> AppState {
        AppState {
            inquiry: crate::inquiry::InquiryState::new(),
            cms: crate::cms::CmsState::from_root(std::path::Path::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/cms-store"
            ))),
        }
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
            // A real inline-style attribute is always ` style="` (preceded by
            // whitespace). A bare `style="` substring false-positives on data
            // attributes like `data-loom-nav-style="standard"`, which are
            // legitimate and carry no CSS.
            assert!(
                !body.contains(" style=\""),
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
    snap_route!(capabilities, "/capabilities");
    snap_route!(case_studies, "/case-studies");
    snap_route!(feedback, "/feedback");
    snap_route!(subscribe, "/subscribe");
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
    snap_route!(sample_report, "/sample-report");
    snap_route!(how_we_work, "/how-we-work");
    snap_route!(pricing, "/pricing-transparency");
    snap_route!(privacy, "/privacy-directive");
    snap_route!(terms, "/terms-of-service");
    snap_route!(sitemap, "/sitemap.xml");
    snap_route!(robots, "/robots.txt");
    snap_route!(blog_rss, "/blog/rss.xml");

    /// CMS-backed page snapshot. Distinct from `snap_route!` because
    /// it threads an explicit [`crate::AppState`] with a fixture-rooted
    /// CMS state — the macro form would call `from_env` and depend
    /// on the test process's cwd matching the manifest dir.
    #[tokio::test]
    async fn cms_doc_why_pps() {
        use axum::body::to_bytes;
        use axum::http::{Request, StatusCode};
        use tower::ServiceExt;
        let state = crate::AppState {
            inquiry: crate::inquiry::InquiryState::new(),
            cms: crate::cms::CmsState::from_root(std::path::Path::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/cms-store"
            ))),
        };
        let app = crate::build_router_with_state(state);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/docs/why-pps")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = to_bytes(resp.into_body(), 256 * 1024).await.unwrap();
        let s = String::from_utf8(body.to_vec()).unwrap();
        insta::assert_snapshot!("cms_doc_why_pps", s);
    }
}

#[cfg(test)]
mod cta_naming {
    //! The site sells one first step. It should have one name.

    /// Every wording the offer has carried at some point. Each was a real
    /// button on a real page: the homepage hero said one thing, the nav
    /// another, /services and /pricing a third, /how-we-work a fourth. A buyer
    /// moving between pages could reasonably conclude they were four different
    /// offers, and the one that converts is the one they recognise.
    const RETIRED_CTA_LABELS: &[&str] = &[
        "Get a Free Consultation",
        "Start Your Journey",
        "Schedule an intake call",
        "Want to start a conversation",
        "Encrypted Inquiry",
        // The blog was "Field Notes" in the badge, the back-link, the RSS
        // title and three places on /subscribe, while the nav and footer said
        // "Insights". One section, two names, depending which page you landed
        // on first.
        "Field Notes",
    ];

    const CURRENT_CTA_LABEL: &str = "Book a scoping call";

    #[test]
    fn no_page_uses_a_retired_call_to_action_name() {
        // Use the shared page list rather than a private one. A guard that
        // walks only the routes someone remembered to add will keep passing
        // for the pages nobody remembered — which is how "Field Notes"
        // survived on the blog and subscribe pages after the rest of the site
        // moved to "Insights".
        let pages: Vec<String> = super::utility_class_coverage::rendered_pages()
            .into_iter()
            .map(|(_, html)| html)
            .collect();
        for (i, page) in pages.iter().enumerate() {
            for label in RETIRED_CTA_LABELS {
                assert!(
                    !page.contains(label),
                    "page {i} still uses the retired call-to-action {label:?}; \
                     the site offers one first step and it is {CURRENT_CTA_LABEL:?}"
                );
            }
        }
    }
}

#[cfg(test)]
mod plain_language {
    //! Vocabulary that survives only because nobody objects to it.

    /// Words a reader has seen on a thousand vendor sites and therefore reads
    /// past. Each carries no information a competitor could not also claim, so
    /// each is space spent saying nothing. They are cheap to write and cheap to
    /// disbelieve, which is the worst combination for a firm selling judgement.
    const FILLER: &[&str] = &[
        "empower",
        "elevate",
        "leverage",
        "seamless",
        "holistic",
        "cutting-edge",
        "best-in-class",
        "world-class",
        "tailored to your",
        "digital landscape",
        "comprehensive solutions",
        "turnkey",
        "next-generation",
        "ever-evolving",
        "synergy",
        "in today's",
    ];

    /// Visible prose only: strip tags so class names and attributes are not
    /// mistaken for copy. Without this the check trips on the utility class
    /// `hover-elevate`, reporting the word "elevate" on a page whose text never
    /// uses it — a guard that cries wolf gets switched off.
    fn visible_text(html: &str) -> String {
        let mut out = String::with_capacity(html.len());
        let mut inside_tag = false;
        for ch in html.chars() {
            match ch {
                '<' => inside_tag = true,
                '>' => inside_tag = false,
                c if !inside_tag => out.push(c),
                _ => {}
            }
        }
        out
    }

    #[test]
    fn marketing_pages_avoid_filler_vocabulary() {
        let pages: Vec<(&str, String)> = vec![
            ("/", crate::views::home::render().into_string()),
            ("/services", crate::views::services::render().into_string()),
            (
                "/capabilities",
                crate::views::capabilities::render().into_string(),
            ),
            (
                "/sample-report",
                crate::views::sample_report::render().into_string(),
            ),
            (
                "/how-we-work",
                crate::views::how_we_work::render().into_string(),
            ),
            (
                "/pricing-transparency",
                crate::views::pricing::render().into_string(),
            ),
            (
                "/case-studies",
                crate::views::case_studies::render().into_string(),
            ),
            ("/about", crate::views::about::render().into_string()),
        ];
        for (route, html) in &pages {
            let lower = visible_text(html).to_lowercase();
            for word in FILLER {
                assert!(
                    !lower.contains(word),
                    "{route} contains the filler word {word:?}; say the specific \
                     thing instead, or say nothing"
                );
            }
        }
    }
}

#[cfg(test)]
mod utility_class_coverage {
    //! The frozen bundle fails silently, so we measure instead of trusting it.
    //!
    //! `static/index-*.css` is a Tailwind build produced by a React app that no
    //! longer exists, and there is no build tool wired up to regenerate it.
    //! Tailwind only emits CSS for classes it saw at build time, so any utility
    //! added to a template afterwards resolves to nothing at all: no warning, no
    //! console error, no visual hint beyond "that section looks a bit off".
    //!
    //! This has shipped three separate defects. `pl-6` collapsed to zero indent
    //! on the homepage proof columns. Every `mb-2` on the site applied no margin
    //! (measured: `margin-bottom: 0px` on a live paragraph). The service-card
    //! icon tiles asked for `bg-gradient-to-br ... ring-1` and painted nothing,
    //! because gradients were never compiled. A census found 71 such classes in
    //! live use across 8 routes.
    //!
    //! The fix is a gap-fill layer at the end of `static/motion.css`. This test
    //! is what stops that layer from drifting out of date: it renders every
    //! page, harvests every class actually emitted, and asserts each one has a
    //! definition in some stylesheet we ship. Adding a class the bundle never
    //! compiled now fails the build instead of quietly doing nothing.

    use std::collections::BTreeSet;

    /// Classes that deliberately carry no styling of their own.
    ///
    /// `group` and `peer` exist only so descendants can target them
    /// (`.group:hover .group-hover\:x`). Lucide stamps `lucide lucide-<name>`
    /// on every icon as a hook for consumers. `shadcn-card` is a leftover
    /// component marker whose visual treatment comes from the utilities sitting
    /// next to it. None of these should be "fixed" by inventing CSS for them.
    fn is_marker(class: &str) -> bool {
        // `prose`/`prose-slate` are Tailwind Typography plugin classes the
        // frozen bundle never contained. Left inert deliberately — see the
        // blog gap-fill note in static/motion.css.
        matches!(
            class,
            "group" | "peer" | "shadcn-card" | "lucide" | "prose" | "prose-slate"
        ) || class.starts_with("lucide-")
            || class.starts_with("group/")
            || class.starts_with("peer/")
    }

    /// Every stylesheet we serve, concatenated.
    ///
    /// Read from disk rather than `include_str!` so a newly added stylesheet is
    /// picked up without anyone remembering to update this list.
    pub(super) fn all_stylesheets() -> String {
        let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/static");
        let mut css = String::new();
        for entry in std::fs::read_dir(dir).expect("static/ must be readable") {
            let path = entry.expect("readable dir entry").path();
            if path.extension().is_some_and(|e| e == "css") {
                css.push_str(&std::fs::read_to_string(&path).expect("readable stylesheet"));
                css.push('\n');
            }
        }
        assert!(!css.is_empty(), "no stylesheets found in static/");
        css
    }

    /// Render a class name the way Tailwind writes it into a selector.
    ///
    /// `md:p-6` is emitted as `.md\:p-6`, `bg-primary/10` as `.bg-primary\/10`,
    /// `text-[15px]` as `.text-\[15px\]`.
    fn css_escape(class: &str) -> String {
        const NEEDS_ESCAPE: &str = r#":[]./&%!#(),<>+*~='"$^|?{}\ "#;
        let mut out = String::with_capacity(class.len() * 2);
        for ch in class.chars() {
            if NEEDS_ESCAPE.contains(ch) {
                out.push('\\');
            }
            out.push(ch);
        }
        out
    }

    /// Does any stylesheet define a rule for this class?
    ///
    /// The trailing character matters. Without it `.mb-1` would match inside
    /// `.mb-12` and report a missing class as present — the exact failure mode
    /// that made an earlier hand-rolled census claim `space-y-4` was undefined
    /// when the real selector was `.space-y-4>:not([hidden])`.
    fn is_defined(class: &str, css: &str) -> bool {
        let needle = format!(".{}", css_escape(class));
        css.match_indices(&needle).any(|(at, _)| {
            css[at + needle.len()..].chars().next().is_some_and(|c| {
                matches!(
                    c,
                    '{' | ',' | ':' | '>' | '~' | '[' | '.' | ')' | ' ' | '\n' | '\t' | '\r'
                )
            })
        })
    }

    /// Pull every class off every `class="..."` attribute in rendered markup.
    pub(super) fn classes_in(html: &str) -> BTreeSet<String> {
        let mut found = BTreeSet::new();
        let mut rest = html;
        while let Some(start) = rest.find("class=\"") {
            rest = &rest[start + 7..];
            let Some(end) = rest.find('"') else { break };
            let value = rest[..end]
                .replace("&amp;", "&")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&quot;", "\"")
                .replace("&#39;", "'");
            found.extend(value.split_whitespace().map(str::to_owned));
            rest = &rest[end..];
        }
        found
    }

    /// Every route we render, as (path, markup).
    ///
    /// Shared with `custom_property_coverage` so both guards always look at
    /// exactly the same set of pages — a route added to one and not the other
    /// is a hole neither test would report.
    pub(super) fn rendered_pages() -> Vec<(&'static str, String)> {
        let mut pages: Vec<(&'static str, String)> = vec![
            ("/", crate::views::home::render().into_string()),
            ("/services", crate::views::services::render().into_string()),
            ("/pricing", crate::views::pricing::render().into_string()),
            (
                "/sample-report",
                crate::views::sample_report::render().into_string(),
            ),
            (
                "/how-we-work",
                crate::views::how_we_work::render().into_string(),
            ),
            (
                "/case-studies",
                crate::views::case_studies::render().into_string(),
            ),
            (
                "/capabilities",
                crate::views::capabilities::render().into_string(),
            ),
            ("/about", crate::views::about::render().into_string()),
            ("/contact", crate::views::contact::render().into_string()),
            ("/feedback", crate::views::feedback::render().into_string()),
            ("/404", crate::views::not_found::render().into_string()),
        ];
        // Blog posts too. They were missing, and the gap was not academic: the
        // guard passed for weeks while every post rendered bullet lists with no
        // bullets and inline code with no background, because `list-disc`,
        // `bg-slate-100` and `px-1.5` are not in the frozen bundle. A guard
        // that only walks the pages someone remembered to list will keep
        // finding nothing wherever nobody remembered.
        for post in crate::views::posts::POSTS {
            if let Some(markup) = crate::views::blog::post(post.slug) {
                pages.push((post.slug, markup.into_string()));
            }
        }
        pages.push(("/blog", crate::views::blog::index().into_string()));
        pages
    }

    #[test]
    fn every_class_we_render_has_css_behind_it() {
        let pages = rendered_pages();

        let css = all_stylesheets();
        let mut undefined: Vec<(String, Vec<&str>)> = Vec::new();

        let mut all: BTreeSet<String> = BTreeSet::new();
        for (_, html) in &pages {
            all.extend(classes_in(html));
        }

        for class in &all {
            if is_marker(class) || is_defined(class, &css) {
                continue;
            }
            let routes: Vec<&str> = pages
                .iter()
                .filter(|(_, html)| classes_in(html).contains(class))
                .map(|(route, _)| *route)
                .collect();
            undefined.push((class.clone(), routes));
        }

        assert!(
            undefined.is_empty(),
            "{} utility class(es) are used in rendered markup but defined in no \
             stylesheet we ship, so they apply nothing at all:\n{}\n\n\
             static/index-*.css is a frozen Tailwind build with no build tool: it \
             only contains classes the original React app happened to use. Add the \
             missing rules to the gap-fill section at the end of static/motion.css \
             (site-owned, loaded last), or switch to a class the bundle already \
             compiled. If a class is a styling-free marker, add it to is_marker().",
            undefined.len(),
            undefined
                .iter()
                .map(|(c, routes)| format!("  {c}  (on {})", routes.join(", ")))
                .collect::<Vec<_>>()
                .join("\n"),
        );
    }
}

#[cfg(test)]
mod eyebrow_contrast {
    //! The section eyebrow must stay legible.
    //!
    //! A browser sweep of every text node on fourteen routes found 26 WCAG AA
    //! failures. Sixteen came from one pattern I had repeated across four
    //! pages: the 10px letterspaced eyebrow in `text-slate-400`, measuring
    //! 2.45:1 against a 4.5 requirement. slate-400 measures 2.56 on white and
    //! slate-300 measures 1.48, so neither can pass below 24px however it is
    //! used; slate-500 measures 4.76 and is the lightest grey that clears it.
    //!
    //! Scoped deliberately to the eyebrow — small + uppercase + letterspaced —
    //! rather than to all small text. A first attempt flagged every small grey
    //! element and reported eleven failures the browser had already cleared:
    //! all of them the footer tagline, where slate-400 on slate-900 is correct
    //! and is there because of an earlier contrast fix. A static check cannot
    //! see an ancestor's background, so it should only judge what it can
    //! actually determine. The authoritative sweep runs in a browser; this
    //! stops one specific regression from returning quietly.

    /// Greys that cannot reach 4.5:1 against white at eyebrow sizes.
    const TOO_LIGHT: &[&str] = &["text-slate-300", "text-slate-400"];

    #[test]
    fn section_eyebrows_use_a_legible_grey() {
        let mut offenders: Vec<String> = Vec::new();
        for (route, html) in super::utility_class_coverage::rendered_pages() {
            for attr in html.split("class=\"").skip(1) {
                let Some(value) = attr.split('"').next() else {
                    continue;
                };
                let classes: Vec<&str> = value.split_whitespace().collect();
                // The eyebrow signature: tiny, capitalised, letterspaced.
                let is_eyebrow = classes.iter().any(|c| c.starts_with("text-[1"))
                    && classes.contains(&"uppercase")
                    && classes.iter().any(|c| c.starts_with("tracking-["));
                if !is_eyebrow {
                    continue;
                }
                if let Some(grey) = classes.iter().find(|c| TOO_LIGHT.contains(c)) {
                    offenders.push(format!("{route}: eyebrow uses {grey} in {value:?}"));
                }
            }
        }
        offenders.sort();
        offenders.dedup();
        assert!(
            offenders.is_empty(),
            "{} section eyebrow(s) use a grey that cannot reach 4.5:1 on white. \
             Use text-slate-500 (4.76:1), the lightest grey that passes:\n{}",
            offenders.len(),
            offenders.join("\n")
        );
    }
}

#[cfg(test)]
mod retired_class_names {
    //! Classes that were replaced and must not come back.

    /// The bare `reveal` class belonged to a JavaScript IntersectionObserver
    /// whose CSS was neutered — `.reveal` and `.reveal.is-visible` both
    /// declared `opacity: 1` — after it left content invisible on mobile.
    /// What survived was an observer walking every tagged element on load to
    /// produce no visual change. Scroll reveal is now `pd-reveal` in
    /// motion.css, driven by CSS with no JavaScript in the path.
    ///
    /// A template that reaches for `reveal` again gets no animation and no
    /// error, which is exactly the kind of silence this codebase keeps
    /// getting caught by.
    #[test]
    fn no_page_uses_the_retired_reveal_class() {
        for (route, html) in super::utility_class_coverage::rendered_pages() {
            for class_attr in html.split("class=\"").skip(1) {
                let Some(value) = class_attr.split('"').next() else {
                    continue;
                };
                for token in value.split_whitespace() {
                    assert!(
                        token != "reveal" && !token.starts_with("reveal-delay-"),
                        "{route} still uses the retired {token:?} class; \
                         scroll reveal is `pd-reveal`, defined in static/motion.css"
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod custom_property_coverage {
    //! The other way the frozen bundle fails silently.
    //!
    //! `utility_class_coverage` proves a class has a rule behind it. It cannot
    //! prove the rule *works*. The bundle ships
    //! `.font-mono { font-family: var(--font-mono) }` and never defines
    //! `--font-mono` anywhere, because the token layer lived in the React app's
    //! theme and did not survive the bake. A `var()` with no definition and no
    //! fallback makes the entire declaration invalid at computed-value time:
    //! the property silently falls back to its inherited or initial value, and
    //! nothing anywhere reports a problem.
    //!
    //! Measured consequences before this test existed: the evidence transcript
    //! on /sample-report rendered in the body sans-serif rather than monospace,
    //! every primary call-to-action carried a 1px white ring from an undefined
    //! `--primary-border`, and secondary buttons drew a near-black edge because
    //! `--button-outline` fell through to `currentColor`.
    //!
    //! Scoped to properties a *rendered* page can actually reach. The bundle
    //! also references some thirty Radix and sidebar tokens belonging to
    //! components this site never renders; those are dead weight, not defects,
    //! and failing the build over them would train everyone to ignore this test.

    use std::collections::BTreeSet;

    /// Split a stylesheet into `(selector, declaration-body)` pairs.
    ///
    /// Deliberately naive — it does not model at-rules, so a `@media` block's
    /// prelude appears as one "selector" with an empty-ish body and its inner
    /// rules appear as their own pairs. That is fine here: we only ever ask
    /// whether a declaration mentions a property and which classes gate it.
    fn rules(css: &str) -> Vec<(String, String)> {
        let mut out = Vec::new();
        let mut selector = String::new();
        let mut body = String::new();
        let mut in_body = false;
        for ch in css.chars() {
            match ch {
                '{' if !in_body => {
                    in_body = true;
                    body.clear();
                }
                '}' if in_body => {
                    in_body = false;
                    out.push((selector.trim().to_owned(), body.clone()));
                    selector.clear();
                }
                _ if in_body => body.push(ch),
                _ => selector.push(ch),
            }
        }
        out
    }

    /// Every class named in a selector, unescaped.
    ///
    /// `.md\:p-6` yields `md:p-6`, matching how the class is written in markup.
    fn classes_in_selector(selector: &str) -> BTreeSet<String> {
        let mut found = BTreeSet::new();
        let bytes: Vec<char> = selector.chars().collect();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == '.'
                && i + 1 < bytes.len()
                && (bytes[i + 1].is_alphanumeric()
                    || bytes[i + 1] == '_'
                    || bytes[i + 1] == '-'
                    || bytes[i + 1] == '\\')
            {
                let mut name = String::new();
                i += 1;
                while i < bytes.len() {
                    let c = bytes[i];
                    if c == '\\' {
                        // escaped character: take the next one literally
                        if i + 1 < bytes.len() {
                            name.push(bytes[i + 1]);
                            i += 2;
                            continue;
                        }
                        i += 1;
                    } else if c.is_alphanumeric() || c == '_' || c == '-' {
                        name.push(c);
                        i += 1;
                    } else {
                        break;
                    }
                }
                if !name.is_empty() {
                    found.insert(name);
                }
            } else {
                i += 1;
            }
        }
        found
    }

    #[test]
    fn custom_properties_used_by_live_rules_are_defined() {
        let css = super::utility_class_coverage::all_stylesheets();
        let pages = super::utility_class_coverage::rendered_pages();

        let mut rendered: BTreeSet<String> = BTreeSet::new();
        for (_, html) in &pages {
            rendered.extend(super::utility_class_coverage::classes_in(html));
        }

        // Anything with a `--name:` declaration counts as defined, wherever it
        // is scoped. A property defined only under a selector we never render
        // would be a false negative, which is the safe direction to err.
        let defined: BTreeSet<String> = css
            .match_indices("--")
            .filter_map(|(at, _)| {
                let rest = &css[at..];
                let name: String = rest
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
                    .collect();
                let after = rest[name.len()..].trim_start();
                after.starts_with(':').then_some(name)
            })
            .collect();

        let mut broken: Vec<(String, String, String)> = Vec::new();
        for (selector, body) in rules(&css) {
            // A rule only matters if every class it names is on the page.
            // Matching on *any* class reports the sidebar tokens as live,
            // because their selectors happen to contain `.group`.
            let gates = classes_in_selector(&selector);
            if gates.is_empty() || !gates.iter().all(|c| rendered.contains(c)) {
                continue;
            }
            let mut from = 0;
            while let Some(rel) = body[from..].find("var(") {
                let at = from + rel + 4;
                let name: String = body[at..]
                    .trim_start()
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
                    .collect();
                from = at;
                if name.is_empty() {
                    continue;
                }
                // A fallback makes the reference safe: var(--x, 1rem) is valid
                // even with --x undefined.
                let has_fallback = body[at + name.len()..].trim_start().starts_with(',');
                if !has_fallback && !defined.contains(&name) {
                    broken.push((name, selector.clone(), body.trim().to_owned()));
                }
            }
        }

        broken.sort();
        broken.dedup();
        assert!(
            broken.is_empty(),
            "{} CSS custom propert(y/ies) are referenced by rules that rendered \
             markup actually matches, but are defined nowhere and have no \
             fallback. The whole declaration is invalid at computed-value time, \
             so the property silently keeps its inherited or initial value:\n{}\n\n\
             Define them in the token block at the top of static/motion.css, or \
             give the reference a fallback: var(--x, <sensible default>). Check \
             how the value is consumed first — hsl(var(--x) / a) needs bare \
             channels like `214 32% 91%`, a bare border-color needs a colour.",
            broken.len(),
            broken
                .iter()
                .map(|(prop, sel, body)| format!(
                    "  {prop}\n      selector: {}\n      declares: {}",
                    sel.chars().take(90).collect::<String>(),
                    body.chars().take(90).collect::<String>()
                ))
                .collect::<Vec<_>>()
                .join("\n"),
        );
    }
}
