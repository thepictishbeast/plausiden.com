//! Axum request handlers. Thin shim over the Maud views in `crate::views`.

use axum::http::StatusCode;
use axum::response::IntoResponse;
use maud::Markup;

pub(crate) async fn home() -> Markup {
    crate::views::home::render()
}

pub(crate) async fn services() -> Markup {
    crate::views::services::render()
}

pub(crate) async fn contact() -> Markup {
    crate::views::contact::render()
}

pub(crate) async fn not_found() -> (StatusCode, Markup) {
    (StatusCode::NOT_FOUND, crate::views::not_found::render())
}

pub(crate) async fn healthz() -> impl IntoResponse {
    // Cheap and cookie-free. Used only by local healthcheck, not advertised.
    (StatusCode::OK, "ok")
}
