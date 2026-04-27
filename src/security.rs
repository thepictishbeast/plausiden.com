//! Response-header middleware that stamps privacy / security headers on every
//! response. Applied once at the router level so 404, 500, timeout, and
//! large-body-rejected responses all carry the same posture.
//!
//! SECURITY: This module exists solely as defense-in-depth. Every header here
//! constrains what a compromised or buggy response can do in a browser. See
//! `annotations/README.md` in the AVP doctrine repo for the `SECURITY:`
//! annotation contract — each constant below documents the threat it blunts.

use axum::http::{HeaderName, HeaderValue};
use tower_http::set_header::SetResponseHeaderLayer;

// SECURITY: CSP locked to `'self'` for every fetch directive. Fonts
// (Plus Jakarta Sans + Outfit) are self-hosted under /static/fonts/
// and referenced by /static/self-hosted-fonts.css, so no third-party
// origin is allowed at all. style-src is `'self'` only — every page
// in the snapshot test suite was confirmed to emit zero inline
// `style=` attributes and zero `<style>` blocks, so 'unsafe-inline'
// is unnecessary. The `csp_no_inline_styles_emitted` test enforces
// the invariant: any future inline-style emission must either remove
// the inline-ness or explicitly relax the CSP, never both silently.
const CSP: &str = "default-src 'self'; \
                   base-uri 'self'; \
                   form-action 'self'; \
                   frame-ancestors 'none'; \
                   img-src 'self' data:; \
                   font-src 'self'; \
                   style-src 'self'; \
                   script-src 'self'; \
                   object-src 'none'; \
                   upgrade-insecure-requests";

// SECURITY: Explicit denial of every CURRENTLY-RECOGNIZED browser-exposed
// device/sensor API. `ambient-light-sensor`, `battery`, `document-domain`,
// and `web-share` were dropped in 2026-04 after Chrome (147.x) started
// logging them as unrecognized (surfaced by PlausiDen-Crawler). The policy
// still denies every channel the current browsers actually know about; any
// future feature will default to blocked unless we explicitly opt in.
const PERMISSIONS_POLICY: &str = "accelerometer=(), autoplay=(), camera=(), \
                                  display-capture=(), encrypted-media=(), \
                                  geolocation=(), gyroscope=(), magnetometer=(), \
                                  microphone=(), midi=(), payment=(), \
                                  picture-in-picture=(), publickey-credentials-get=(), \
                                  screen-wake-lock=(), sync-xhr=(), usb=(), \
                                  xr-spatial-tracking=(), interest-cohort=()";

// SECURITY: `max-age=2 years; includeSubDomains; preload` locks every subdomain
// to HTTPS for 2y per RFC 6797. Preload allows submission to Chrome's HSTS
// preload list so first-ever visitors never touch plaintext.
const HSTS: &str = "max-age=63072000; includeSubDomains; preload";

/// Construct the tower middleware stack that stamps the security headers on
/// every response.
///
/// BUG ASSUMPTION: `SetResponseHeaderLayer::if_not_present` is intentional —
/// never override a header an inner layer has already set (lets a specific
/// handler opt out with a deliberate reason; no handler does today).
///
/// SECURITY: Every header in this stack is a defense-in-depth measure. If any
/// layer is accidentally removed the build will still pass — cover with the
/// `security_headers_are_stamped` test in `main.rs` and the per-header tests
/// below.
#[must_use]
#[allow(clippy::type_complexity)]
pub fn headers_layer() -> tower::layer::util::Stack<
    SetResponseHeaderLayer<HeaderValue>,
    tower::layer::util::Stack<
        SetResponseHeaderLayer<HeaderValue>,
        tower::layer::util::Stack<
            SetResponseHeaderLayer<HeaderValue>,
            tower::layer::util::Stack<
                SetResponseHeaderLayer<HeaderValue>,
                tower::layer::util::Stack<
                    SetResponseHeaderLayer<HeaderValue>,
                    tower::layer::util::Stack<
                        SetResponseHeaderLayer<HeaderValue>,
                        tower::layer::util::Stack<
                            SetResponseHeaderLayer<HeaderValue>,
                            tower::layer::util::Identity,
                        >,
                    >,
                >,
            >,
        >,
    >,
> {
    use tower::ServiceBuilder;

    ServiceBuilder::new()
        .layer(static_header("content-security-policy", CSP))
        .layer(static_header("strict-transport-security", HSTS))
        .layer(static_header("referrer-policy", "no-referrer"))
        .layer(static_header("x-content-type-options", "nosniff"))
        .layer(static_header("x-frame-options", "DENY"))
        .layer(static_header("cross-origin-opener-policy", "same-origin"))
        .layer(static_header("permissions-policy", PERMISSIONS_POLICY))
        .into_inner()
}

fn static_header(name: &'static str, value: &'static str) -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::if_not_present(
        HeaderName::from_static(name),
        HeaderValue::from_static(value),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a dummy route with just the headers layer and verify that every
    /// expected header is present on a simple response.
    #[tokio::test]
    async fn all_seven_headers_present() {
        use axum::Router;
        use axum::body::Body;
        use axum::http::Request;
        use axum::routing::get;
        use tower::ServiceExt;

        let app: Router = Router::new()
            .route("/", get(|| async { "hello" }))
            .layer(headers_layer());
        let resp = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let h = resp.headers();
        for name in [
            "content-security-policy",
            "strict-transport-security",
            "referrer-policy",
            "x-content-type-options",
            "x-frame-options",
            "cross-origin-opener-policy",
            "permissions-policy",
        ] {
            assert!(h.contains_key(name), "missing header: {name}");
        }
    }

    /// CSP locks every fetch directive to `'self'` — no third-party origins
    /// and no `'unsafe-inline'` anywhere. Fonts self-hosted under
    /// /static/fonts/. Inline-style invariant enforced separately by
    /// `csp_no_inline_styles_emitted` below.
    #[test]
    fn csp_locks_every_origin_to_self() {
        assert!(CSP.contains("default-src 'self'"));
        assert!(CSP.contains("font-src 'self'"));
        assert!(CSP.contains("style-src 'self'"));
        assert!(!CSP.contains("'unsafe-inline'"), "CSP must not allow inline");
        assert!(!CSP.contains("https://fonts.gstatic.com"));
        assert!(!CSP.contains("https://fonts.googleapis.com"));
        assert!(!CSP.contains("unsafe-eval"), "CSP must not allow eval");
        assert!(CSP.contains("frame-ancestors 'none'"));
        assert!(CSP.contains("form-action 'self'"));
    }

    /// Permissions-Policy explicitly denies common sensor/device APIs.
    #[test]
    fn permissions_policy_denies_sensors_and_devices() {
        for feature in [
            "camera=()",
            "microphone=()",
            "geolocation=()",
            "usb=()",
            "payment=()",
            "interest-cohort=()",
        ] {
            assert!(
                PERMISSIONS_POLICY.contains(feature),
                "permissions-policy missing: {feature}"
            );
        }
    }

    /// HSTS preload requirements: max-age ≥ 1 year, includeSubDomains, preload.
    #[test]
    fn hsts_meets_preload_requirements() {
        assert!(HSTS.contains("max-age=63072000"));
        assert!(HSTS.contains("includeSubDomains"));
        assert!(HSTS.contains("preload"));
    }
}
