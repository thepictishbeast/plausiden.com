//! `plausiden-site` binary entrypoint.
//!
//! Thin wrapper around the library crate. The router, modules,
//! handlers, and tests all live in `lib.rs`; this file is just the
//! tokio runtime + signal handling.
//!
//! Design principles: one binary, zero state, zero third-party,
//! zero logs by default. Everything user-visible is either a
//! static file or a compile-time-rendered Maud view.
//!
//! Governed by the PlausiDen AVP Doctrine. Every public function
//! carries a `BUG ASSUMPTION:` annotation; every defense-in-depth
//! carries a `SECURITY:` annotation.

#![doc(html_no_source)]

use std::net::SocketAddr;
use std::time::Duration;

use plausiden_site::{build_router, inquiry::InquiryState, sandbox};
use tokio::signal;
use tracing_subscriber::{EnvFilter, fmt};

/// Default bind address if `PLAUSIDEN_BIND` is unset. Loopback only —
/// production deployment expects nginx in front.
const DEFAULT_BIND: &str = "127.0.0.1:8080";

/// Graceful shutdown grace period; after this the runtime drops in-flight tasks.
const SHUTDOWN_GRACE: Duration = Duration::from_secs(15);

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

    let app = build_router(InquiryState::new());

    let bind: SocketAddr = std::env::var("PLAUSIDEN_BIND")
        .unwrap_or_else(|_| DEFAULT_BIND.into())
        .parse()?;

    let listener = tokio::net::TcpListener::bind(bind).await?;
    tracing::info!(%bind, "plausiden-site listening");

    // SECURITY: in-process Landlock sandbox. Applied AFTER the listener is
    // bound (so the process still had permission to access the syscall) and
    // BEFORE accepting traffic (so any handler runs inside the restricted
    // filesystem view). Static dir is the only allowed read path.
    let _ = sandbox::apply("static");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    Ok(())
}

/// Wait for SIGINT or SIGTERM, then return so `axum::serve`'s graceful
/// shutdown can drain connections up to [`SHUTDOWN_GRACE`].
///
/// BUG ASSUMPTION: On non-Unix targets `terminate` is pending forever, so
/// only ctrl-c terminates. That's fine; production runs on Linux.
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
