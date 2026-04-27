# Architecture decision log

Short, dated decisions about path-not-taken choices. Each entry is
load-bearing: future contributors should read this before re-litigating.

## 2026-04-27 — No analytics, no telemetry

**Decision:** plausiden-site ships zero web analytics. No Plausible
self-hosted, no Matomo, no GA, no first-party log analysis on a
cadence beyond ops triage.

**Why:**
- The site exists to convert prospects who specifically want a
  privacy-first vendor. Loading any analytics — even self-hosted
  Plausible — would require changing what we tell them in the first
  90 seconds of the homepage.
- Conversion attribution is a measurement nice-to-have, not a
  measurement need. We have a phone number and an email address; if
  those start ringing, we have signal. We don't need cohort funnels
  to know whether the marketing is working.
- Self-hosted Plausible would still require running another service,
  another DB, another logrotate, another vulnerability surface. The
  marginal value is below the marginal cost.

**What we do instead:** read the nginx access log on demand when a
specific question comes up ("did the post we shared get any
engagement?"). The nginx log already has IP-stripped + minimum
fields per the privacy posture. No dashboard, no aggregation, no
retention beyond the rotated window.

**Reversal trigger:** if conversion drops measurably and we have no
signal about why, we can revisit. Today the funnel volume is small
enough that operational sense covers it.

Resolves #48.

---

## 2026-04-27 — Loom integration: vendored copies + path deps for now

**Decision:** plausiden-site (and Thundercrab, when its UI lands)
consume PlausiDen-Loom via Cargo path deps in local dev and via
vendored snapshots in CI, until either (a) PlausiDen-Loom goes
public on crates.io or (b) a workspace-monorepo migration lands.

**Considered:**
1. **Git deps with PAT.** Add a personal access token to GitHub
   Actions secrets, point Cargo at `git = "https://github.com/..."`.
   Works, but couples deploys to a token rotation cadence and adds
   one more credential surface to manage.
2. **Public crates.io publish.** Ship loom-tokens / loom-components
   to crates.io. Cleanest long-term but requires we commit to a
   public versioning + breaking-change cadence; we're not ready.
3. **Monorepo with all PlausiDen-* crates as workspace members.**
   Cleanest local dev but loses the "each repo audits independently"
   property we depend on.
4. **Path deps + vendored copies (chosen).** Path dep in local dev
   so changes are immediate; vendored copy of any cross-repo
   audit/check.sh into `.github/scripts/` so CI doesn't need
   cross-repo auth. Re-vendor on every Loom release.

**Why path-deps-plus-vendored:**
- Local dev: zero friction; touching loom-components and seeing
  the diff in plausiden-site is a fast feedback loop.
- CI: zero cross-repo auth; the consuming repo carries everything
  it needs to audit itself.
- Cost: a vendoring drift risk. Mitigated by the version-control
  audit (#92) and a header comment on every vendored file pointing
  at the canonical source.

**Reversal trigger:** when (a) crates.io publish becomes the right
move, or (b) we hit the third place where vendoring would be
needed. The Wirth's-law line is "twice is coincidence, thrice is
a refactor."

Resolves #18, #52.
