//! Microbenchmarks for the request render hot path.
//!
//! Walks the in-process router (the same one main.rs serves) and
//! measures end-to-end latency for each route we expect to be hot.
//! Results live under `target/criterion/`.
//!
//! Why bench in-process: the network layer is the user's nginx +
//! tower-http stack; what we own is the Maud render time and the
//! handler closures. Benching that surface keeps us honest about
//! whether a refactor regressed the render path, without conflating
//! with kernel/network noise.

#![allow(missing_docs)]

use axum::body::{Body, to_bytes};
use axum::http::Request;
use criterion::{Criterion, criterion_group, criterion_main};
use plausiden_site::{build_router, inquiry::InquiryState};
use tower::ServiceExt;

fn fetch_blocking(rt: &tokio::runtime::Runtime, path: &str) -> Vec<u8> {
    rt.block_on(async {
        let app = build_router(InquiryState::new());
        let resp = app
            .oneshot(
                Request::builder()
                    .uri(path)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = to_bytes(resp.into_body(), 256 * 1024).await.unwrap();
        body.to_vec()
    })
}

fn bench_routes(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build current-thread runtime");

    let mut group = c.benchmark_group("render");
    for path in &[
        "/",
        "/services",
        "/about",
        "/contact",
        "/blog",
        "/blog/why-thundercrab",
        "/blog/plausible-deniability",
        "/sitemap.xml",
        "/blog/rss.xml",
    ] {
        group.bench_function(*path, |b| {
            b.iter(|| {
                let body = fetch_blocking(&rt, path);
                criterion::black_box(body);
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_routes);
criterion_main!(benches);
