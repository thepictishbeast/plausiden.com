# plausiden-site production-readiness audit

Living checklist for the Rust rewrite (`next.plausiden.com` → eventual
`plausiden.com`). Items grouped by whether they block production cutover.

Last reviewed: 2026-04-26.

## BLOCKING — fix before cutover

- [x] **Clippy clean under `-D warnings`.** `inquiry.rs` had a
  `manual_let_else` suggestion; converted. `cargo clippy -- -D warnings`
  passes.
- [ ] **Contact form a11y.** 5 axe-core violations: every `<input>` and
  `<textarea>` reports as "unnamed textbox" because the baked React DOM
  uses `useId()`-generated `for=":r0:-form-item"` etc. that fail to
  bind via accessible-name computation in axe. Fix: rewrite the form
  in proper Maud with stable IDs and visible `<label>`s. Same Tailwind
  classes, no visual change.
- [ ] **Hero image gradient too dark.** The replaced-testimonial image
  (`/static/images/hero-team.jpg`) carries a `bg-gradient-to-t
  from-slate-900/80 to-transparent` overlay — 80% opacity dark slate.
  Brand positioning is "the future is bright." Lighten to `from-slate-900/30`
  or change to `from-primary/20` (brand-tinted instead of darkening).
- [ ] **No on-scroll / on-load animations.** The React production site
  uses Framer Motion (`initial/animate/whileInView`). Baking the DOM
  via Playwright captures the post-animation state, so the static
  output is a frozen still. Fix: add CSS-only fade-in-up keyframes +
  a tiny IntersectionObserver hook (extend `static/menu.js`) that
  adds an `is-visible` class on scroll. Honor `prefers-reduced-motion`.
- [ ] **Footer copyright telegraphs founding window.** Currently
  `© 2026 PlausiDen LLC` — a sophisticated prospect reads "very young
  company." Change to `© PlausiDen LLC` (no year) or
  `© 2024–2026 PlausiDen LLC` if dating is preferred.

## SHOULD-FIX before serious outbound

- [ ] **Verify contact-form delivery end-to-end.** POST `/contact`
  goes through `inquiry::submit` → Postfix → `team@plausiden.com`.
  Test from staging with a known-good payload; confirm receipt in the
  team mailbox; monitor logs for governor rejections.
- [ ] **Mobile-viewport visual review.** Crawler `plausiden-site-mobile`
  passes 14/14 steps with no console/page errors. Worth a manual eye
  pass on small details (nav menu open state, contact form spacing).
- [ ] **Privacy + Terms** are honest placeholders ("under legal review
  with counsel"). Decision: leave as-is — a young company's "drafted
  with counsel" reads more professional than fake-real boilerplate.
  Replace when real text comes back from counsel.
- [x] **Add `/blog` surface.** `/blog` index + `/blog/{slug}` route, posts
  registered as Maud functions in `src/views/posts/`. "Field Notes" link
  in nav. First post (federated rule learning) shipped.

## Loom migration progress

Tracked via `~/PlausiDen-Loom/target/release/loom lint ~/plausiden-site`.

| Date | Count | Note |
|------|-------|------|
| 2026-04-26 | 192 | Pre-Loom baseline |
| 2026-04-26 | 187 | After Button migration |
| 2026-04-26 | 183 | After contact-form migration |
| 2026-04-26 | 178 | After legal capability cards + blog post cards migrated to FeatureCard / LinkCard |
| 2026-04-27 | 340 | After shipping /how-we-work, /pricing, /solutions/healthcare, /solutions/journalism, AVP doctrine post (new content brought new debt faster than migration retired old debt) |

Path to lint-zero, in order of leverage:

- [ ] `loom-icons` registry — collapses hundreds of inline SVG class strings (`"w-6 h-6 text-primary"` etc.) into typed icon constants. Single highest-leverage migration target.
- [ ] Migrate `components/card.rs` ServiceCard to Loom Card.
- [ ] Migrate `inquiry.rs` ack page to use Hero + Section primitives.
- [ ] Migrate `solutions/<vertical>` Section bodies (capability grid is Loom; surrounding bands are not).

## POLISH — improves outbound conversion but not blocking

- [ ] **Vertical landing page: `/solutions/legal`.** Law firms is the
  natural first vertical (privacy-regulated, premium rates,
  "we don't read your data" is a real feature). Salesman aims emails
  at it.
- [ ] **OG / Twitter card metadata.** Currently no `<meta property="og:*">`
  or `<meta name="twitter:*">` — when a prospect shares the URL in
  Slack/email, the preview is bare. Add image, title, description.
- [ ] **JSON-LD `Organization` schema.** SEO floor; helps Google
  understand the site identity.
- [ ] **`sitemap.xml` + `robots.txt`.** Robots already exists?
  (Verify.) Sitemap improves indexing.
- [ ] **AVP Doctrine landing page** — a sanitized public version of
  the operating doctrine. "How PlausiDen works" is a recruiter +
  buyer differentiator.

## NICE-TO-HAVE

- [ ] Lighthouse baseline (perf, a11y, SEO, best-practices) — track
  delta over time.
- [ ] Page-weight audit. Static HTML + Tailwind via the production
  CSS bundle is already lean; verify by measuring.
- [ ] `prefers-reduced-motion` verification on the new animation pass.

## NOT FIXING (consciously)

- **Generalist positioning.** Per prior conversation, the user has
  consciously chosen generalist front door + outbound-driven vertical
  pages. The Salesman does the differentiating work in email; the
  site exists to land prospects already pre-qualified by the email.
- **Named contact vs `team@plausiden.com`.** Signals "small team"
  intentionally. Keep.
