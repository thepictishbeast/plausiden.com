#!/usr/bin/env node
// Keyboard-path audit. Semantics: a stop passes when focus produces ANY
// computed outline/box-shadow delta vs its blurred state (perceivable
// change), measured with REAL Tab presses. Instrument lessons baked in:
// identity-based cycle detection (consecutive identical labels are real,
// e.g. 8x "Open service detail"), transitions frozen (mid-flight reads
// lie both ways), bypassCSP for the freeze style. Delta-semantics note:
// an element with a RESTING shadow that vanishes on focus also counts
// as a visible change - that is correct, not a bug (see 2026-08-22
// control run). Run with the site on 127.0.0.1:8099.
import { createRequire } from 'node:module';
const require = createRequire('/home/paul/e2e-probe/');
const { chromium } = require('playwright');

const BASE = process.argv[2] || 'http://127.0.0.1:8099';
const ROUTES = ['/', '/services', '/contact', '/pricing-transparency', '/blog/avp-doctrine'];
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, bypassCSP: true });

for (const route of ROUTES) {
  await page.goto(BASE + route, { waitUntil: 'networkidle' });
  // transitions mid-flight make blurred-vs-focused reads lie in both
  // directions (proven by the control run: 2/10 escapes) — freeze them.
  await page.addStyleTag({ content: '* { transition: none !important; animation: none !important; }' });
  const stops = [];
  await page.evaluate(() => { window.__kbdFirst = null; });
  for (let i = 0; i < 80; i++) {
    await page.keyboard.press('Tab');
    const info = await page.evaluate(() => {
      const el = document.activeElement;
      if (!el || el === document.body) return null;
      // identity-based cycle detection: consecutive identical LABELS are
      // legitimate (eight "Open service detail" cards), so terminate only
      // when focus returns to the first stop.
      if (window.__kbdFirst === el) return null;
      if (!window.__kbdFirst) window.__kbdFirst = el;
      const cs = getComputedStyle(el);
      const focused = { o: cs.outlineStyle + '/' + cs.outlineWidth, bs: cs.boxShadow };
      // blur to read the resting state, then restore focus
      el.blur();
      const rs = getComputedStyle(el);
      const resting = { o: rs.outlineStyle + '/' + rs.outlineWidth, bs: rs.boxShadow };
      el.focus();
      const r = el.getBoundingClientRect();
      const name = (el.getAttribute('aria-label') || el.textContent || el.getAttribute('href') || '').trim().slice(0, 30);
      return {
        sig: el.tagName + '|' + name,
        tag: el.tagName.toLowerCase(),
        name,
        visibleIndicator: focused.o !== resting.o || focused.bs !== resting.bs,
        offscreen: r.bottom < 0 || r.top > innerHeight * 20 || r.width === 0 || r.height === 0,
      };
    });
    if (!info) break;               // cycled back to start/body
    stops.push(info);
  }
  const noInd = stops.filter(s => !s.visibleIndicator);
  const hidden = stops.filter(s => s.offscreen);
  console.log(`${route}: ${stops.length} stops, first=[${stops[0]?.tag} "${stops[0]?.name}"]`);
  if (noInd.length) console.log(`  NO-INDICATOR (${noInd.length}): ` + noInd.map(s => `${s.tag}"${s.name}"`).slice(0, 8).join(', '));
  if (hidden.length) console.log(`  OFFSCREEN/ZERO-SIZE (${hidden.length}): ` + hidden.map(s => `${s.tag}"${s.name}"`).slice(0, 5).join(', '));
}
await browser.close();
