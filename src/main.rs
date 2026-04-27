//! `plausiden-site` entrypoint.
//!
//! Design principles: one binary, zero state, zero third-party, zero logs by default.
//! Everything user-visible is either a static file or a compile-time-rendered `Maud` view.
//!
//! Governed by the `PlausiDen` AVP Doctrine. Every public function carries a
//! `BUG ASSUMPTION:` annotation; every defense-in-depth carries a `SECURITY:`
//! annotation (see `annotations/README.md` in the doctrine repo).

#![doc(html_no_source)]

use std::net::SocketAddr;
use std::time::Duration;

use axum::Router;
use tokio::signal;
use tracing_subscriber::{EnvFilter, fmt};

mod components;
mod handlers;
mod inquiry;
mod sandbox;
mod security;
mod views;

/// Default bind address if `PLAUSIDEN_BIND` is unset. Loopback only — production
/// deployment expects nginx (v1) or a future in-process TLS terminator (v2) in front.
const DEFAULT_BIND: &str = "127.0.0.1:8080";

/// Graceful shutdown grace period; after this the runtime drops in-flight tasks.
const SHUTDOWN_GRACE: Duration = Duration::from_secs(15);

/// Request processing timeout. Matches the `TimeoutLayer` installed below.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

/// Process entrypoint.
///
/// BUG ASSUMPTION: `PLAUSIDEN_BIND`, if set, must parse as a `SocketAddr`. A
/// malformed value returns an error and exits before `listen(2)` — safer than
/// silently falling back to the default (which could mask a deploy misconfig).
///
/// SECURITY: We bind exactly one address and never accept runtime plaintext
/// routing changes. The process is one-shot: reconfiguration means redeploy.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()))
        .with_target(false)
        .compact()
        .init();

    let app = build_router(inquiry::InquiryState::new());

    let bind: SocketAddr = std::env::var("PLAUSIDEN_BIND")
        .unwrap_or_else(|_| DEFAULT_BIND.into())
        .parse()?;

    let listener = tokio::net::TcpListener::bind(bind).await?;
    tracing::info!(%bind, "plausiden-site listening");

    // SECURITY: in-process Landlock sandbox. Applied AFTER the listener is
    // bound (so the process still had permission to access the syscall) and
    // BEFORE accepting traffic (so any handler runs inside the restricted
    // filesystem view). Static dir is the only allowed read path; writes are
    // forbidden entirely.
    let _ = sandbox::apply("static");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    Ok(())
}

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
pub(crate) fn build_router(inquiry_state: inquiry::InquiryState) -> Router {
    use axum::http::StatusCode;
    use axum::routing::get;
    use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, trace::TraceLayer};

    Router::new()
        .route("/", get(handlers::home))
        .route("/services", get(handlers::services))
        .route("/about", get(handlers::about))
        .route("/contact", get(handlers::contact).post(inquiry::submit))
        .route("/blog", get(handlers::blog_index))
        .route("/blog/{slug}", get(handlers::blog_post))
        .route("/solutions/legal", get(handlers::solutions_legal))
        .route("/solutions/healthcare", get(handlers::solutions_healthcare))
        .route("/solutions/journalism", get(handlers::solutions_journalism))
        .route("/sitemap.xml", get(handlers::sitemap_xml))
        .route("/robots.txt", get(handlers::robots_txt))
        .route("/blog/rss.xml", get(handlers::blog_rss))
        .route("/privacy-directive", get(handlers::privacy))
        .route("/terms-of-service", get(handlers::terms))
        .route("/healthz", get(handlers::healthz))
        .nest_service("/static", tower_http::services::ServeDir::new("static"))
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

/// Wait for SIGINT or SIGTERM, then return so `axum::serve`'s graceful shutdown
/// can drain connections up to [`SHUTDOWN_GRACE`].
///
/// BUG ASSUMPTION: On non-Unix targets `terminate` is pending forever, so only
/// ctrl-c terminates. That's fine; production runs on Linux.
async fn shutdown_signal() {
    let ctrl_c = async {
        // SAFETY: A process that cannot install a SIGINT handler is in an
        // unrecoverable state; panicking here is the correct abort path.
        signal::ctrl_c().await.expect("ctrl_c handler install");
    };
    #[cfg(unix)]
    let terminate = async {
        // SAFETY: Same as above — a process without signal-handling cannot
        // participate in graceful shutdown; hard abort is correct.
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("sigterm handler install")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
    tracing::info!(
        grace_secs = SHUTDOWN_GRACE.as_secs(),
        "shutdown signal received"
    );
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
        let ct = resp.headers().get("content-type").unwrap().to_str().unwrap();
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
        let ct = resp.headers().get("content-type").unwrap().to_str().unwrap();
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
}
