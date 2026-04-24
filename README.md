<!-- repo-label: product -->
<!-- repo-class: plausiden-marketing-website -->
<!-- repo-consumes: PlausiDen-AVP-Doctrine (tier definitions), PlausiDen-Obs (inline until crate publishes) -->
<!-- repo-consumed-by: leaf -->

# plausiden-site

Rust rewrite of plausiden.com. Single-binary, zero-state, privacy-first.

> Part of the [PlausiDen ecosystem](https://github.com/thepictishbeast/PlausiDen-Meta/blob/main/ECOSYSTEM_GUIDE.md).
> Not on the [`PRIORITY.md`](https://github.com/thepictishbeast/PlausiDen-Meta/blob/main/PRIORITY.md)
> tier list — this repo is object-level business work (the plausiden.com
> marketing surface), not doctrine or product tooling. It still conforms to
> the ecosystem guide: AVP-graded, Obs-doctrine inline, harvest-participating.

## Design principles

- **One binary.** Axum + Maud + Tokio. No nginx in the long run; rustls-acme, quinn, and
  arti (in-process Tor) on the roadmap. For v1 the binary runs behind the existing nginx.
- **Zero state.** No sessions, no cookies (except a future CSRF nonce on form POST), no database.
- **Zero third-party.** No CDNs, no fonts from Google, no analytics, no pixel tags.
  Strict CSP with `default-src 'self'` only.
- **JS-disabled parity.** Every page works with JavaScript turned off, so Tor Browser
  safest-mode visitors are first-class.
- **Compile-time correctness.** Maud templates are type-checked HTML. `#![forbid(unsafe_code)]`.
  `clippy::pedantic` enforced in CI.
- **Defence by absence.** The fastest, most private, least-exploitable component is the
  one that doesn't exist.

## Layout

```
src/
├── main.rs        server bootstrap, router, graceful shutdown
├── handlers.rs    thin axum handler shims
├── security.rs    response-header middleware (CSP, HSTS, Permissions-Policy, …)
├── views.rs       module root for Maud views
└── views/
    ├── layout.rs  shared page chrome
    ├── home.rs
    ├── services.rs
    ├── contact.rs (Encrypted Inquiry)
    └── not_found.rs
static/            favicons, manifest, self-hosted stylesheet
```

## Local development

Prerequisites: Rust stable ≥ 1.82.

```bash
cargo run
# then open http://127.0.0.1:8080/
```

Override the bind address with `PLAUSIDEN_BIND=0.0.0.0:8080 cargo run`.

## Build

```bash
cargo build --release
```

Production release target on the VPS:

```bash
# One-time
rustup target add x86_64-unknown-linux-musl
sudo apt install -y musl-tools

# Build a fully-static single-file binary
cargo build --release --target x86_64-unknown-linux-musl
ls -la target/x86_64-unknown-linux-musl/release/plausiden-site
```

The musl binary has no glibc dependency; it copies to any Linux host and runs.

## Hardening (ship in v1 unless noted)

- [x] Strict CSP, HSTS preload, Referrer-Policy no-referrer, Permissions-Policy lockdown
- [x] `#![forbid(unsafe_code)]`; `clippy::pedantic` in CI
- [x] Response compression (br/gzip), request timeout, sensible limits
- [ ] Rate limiting on form endpoints (`governor`) — v1.0 before public submission goes live
- [ ] Landlock + seccomp sandboxing at startup
- [ ] Client-side age encryption of inquiry payloads (WASM)
- [ ] Reproducible musl release + Sigstore signing + Rekor attestation
- [ ] Tor v3 hidden service via in-process Arti
- [ ] PQ-hybrid TLS (waiting on Let's Encrypt ecosystem)

## Deploy (v1, behind existing nginx)

`nginx` proxies HTTPS → `127.0.0.1:8080`.

systemd unit lives under `deploy/plausiden-site.service` (to be added). Runs as a
dedicated unprivileged user `plausiden`, with `NoNewPrivileges`, `ProtectSystem=strict`,
`ProtectHome=true`, `PrivateTmp`, `PrivateDevices`, empty `CapabilityBoundingSet`,
restricted syscall filter, restricted address families.

## Non-goals

- No SPA framework. No WebAssembly in the main page payload.
- No user accounts. No visitor tracking of any kind.
- No third-party scripts, fonts, or images. Ever.
