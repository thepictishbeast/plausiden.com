//! PlausiDen site entrypoint.
//!
//! Design principles: one binary, zero state, zero third-party, zero logs by default.
//! Everything user-visible is either a static file or a compile-time-rendered Maud view.

use std::net::SocketAddr;
use std::time::Duration;

use axum::Router;
use tokio::signal;
use tracing_subscriber::{EnvFilter, fmt};

mod handlers;
mod security;
mod views;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "warn".into()))
        .with_target(false)
        .compact()
        .init();

    let app = build_router();

    let bind: SocketAddr = std::env::var("PLAUSIDEN_BIND")
        .unwrap_or_else(|_| "127.0.0.1:8080".into())
        .parse()?;

    let listener = tokio::net::TcpListener::bind(bind).await?;
    tracing::info!(%bind, "plausiden-site listening");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

fn build_router() -> Router {
    use axum::http::StatusCode;
    use axum::routing::get;
    use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, trace::TraceLayer};

    Router::new()
        .route("/", get(handlers::home))
        .route("/services", get(handlers::services))
        .route("/contact", get(handlers::contact))
        .route("/healthz", get(handlers::healthz))
        .nest_service("/static", tower_http::services::ServeDir::new("static"))
        .layer(security::headers_layer())
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
        .layer(TraceLayer::new_for_http())
        .fallback(handlers::not_found)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("ctrl_c handler");
    };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("sigterm handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }
    tracing::info!("shutdown signal received");
}
