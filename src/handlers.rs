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

/// Render the Encrypted Inquiry form (`GET /contact`).
///
/// BUG ASSUMPTION: v1 returns a plain HTML form. v1.1 will progressively enhance
/// with WASM-side age encryption; until then form POSTs hit a handler (not yet
/// wired) that must validate a double-submit CSRF nonce and rate-limit per IP.
pub async fn contact() -> Markup {
    crate::views::contact::render()
}

/// Fallback handler for unmatched paths. Returns 404 with a styled page.
///
/// BUG ASSUMPTION: The `404 + Markup` tuple is picked up by Axum's
/// `IntoResponse` impl and becomes a correctly-statused HTML response. This is
/// exercised in the router test in `main.rs`.
pub async fn not_found() -> (StatusCode, Markup) {
    (StatusCode::NOT_FOUND, crate::views::not_found::render())
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
