//! Security response headers. Applied to every response as a single middleware layer.
//!
//! Policy rationale lives in README.md under "Hardening".

use axum::http::{HeaderName, HeaderValue};
use tower_http::set_header::SetResponseHeaderLayer;

/// Strict, third-party-free CSP. No inline, no eval, no remote origins.
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

const PERMISSIONS_POLICY: &str = "accelerometer=(), ambient-light-sensor=(), autoplay=(), \
                                  battery=(), camera=(), display-capture=(), \
                                  document-domain=(), encrypted-media=(), geolocation=(), \
                                  gyroscope=(), magnetometer=(), microphone=(), \
                                  midi=(), payment=(), picture-in-picture=(), \
                                  publickey-credentials-get=(), screen-wake-lock=(), \
                                  sync-xhr=(), usb=(), web-share=(), xr-spatial-tracking=(), \
                                  interest-cohort=()";

/// Construct the tower layer that stamps security headers on every response.
#[must_use]
pub(crate) fn headers_layer() -> tower::layer::util::Stack<
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
        .layer(static_header(
            "strict-transport-security",
            "max-age=63072000; includeSubDomains; preload",
        ))
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
