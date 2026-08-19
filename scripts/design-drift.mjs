#!/usr/bin/env node
// Count the distinct design values ACTUALLY IN USE across every route.
//
// Grepping class names answers a different question than the one that matters.
// `p-6` and `px-6 py-6` are the same rendered padding and different strings;
// `rounded-xl` on an element whose parent sets a radius is not what the eye
// sees. The only honest count comes from getComputedStyle on the live DOM, so
// that is what this does.
//
// Routes come from sitemap.xml rather than a hand-kept list: every previous
// hand-kept list in this repo went stale (blog posts, /404, /docs/*, the OG
// card table, cms-store in the deploy script).
//
// Usage: node scripts/design-drift.mjs [baseUrl]

import { createRequire } from 'node:module';

// ESM ignores NODE_PATH, so resolve Playwright by path.
const require = createRequire('/home/paul/e2e-probe/');
const { chromium } = require('playwright');

const BASE = process.argv[2] || 'http://127.0.0.1:8099';

// Categories are (selector, properties) pairs. Keep the selectors structural —
// a selector written in terms of the classes we are auditing would only ever
// find the values we already expected.
const CATEGORIES = [
  {
    name: 'card padding',
    // A "card" is anything with its own border or shadow that holds content.
    pick: (el, cs) =>
      (cs.borderTopWidth !== '0px' || cs.boxShadow !== 'none') &&
      el.children.length > 0 &&
      el.getBoundingClientRect().width > 120,
    read: (cs) => `${cs.paddingTop} ${cs.paddingRight} ${cs.paddingBottom} ${cs.paddingLeft}`,
  },
  {
    name: 'card radius',
    pick: (el, cs) =>
      (cs.borderTopWidth !== '0px' || cs.boxShadow !== 'none') &&
      el.children.length > 0 &&
      el.getBoundingClientRect().width > 120,
    read: (cs) => cs.borderRadius,
  },
  {
    name: 'card border',
    pick: (el, cs) => cs.borderTopWidth !== '0px' && el.children.length > 0,
    read: (cs) => `${cs.borderTopWidth} ${cs.borderTopStyle} ${cs.borderTopColor}`,
  },
  {
    name: 'shadow',
    pick: (_el, cs) => cs.boxShadow !== 'none',
    read: (cs) => cs.boxShadow,
  },
  {
    name: 'body text colour',
    pick: (el, cs) => hasOwnText(el) && parseFloat(cs.fontSize) <= 20,
    read: (cs) => cs.color,
  },
  {
    name: 'font size',
    pick: (el) => hasOwnText(el),
    read: (cs) => cs.fontSize,
  },
  {
    name: 'button',
    pick: (el) => el.matches('a[class*="bg-"], button, a[class*="border"]') && hasOwnText(el),
    read: (cs) =>
      `h=${cs.height} pad=${cs.paddingTop}/${cs.paddingLeft} r=${cs.borderRadius} fs=${cs.fontSize} fw=${cs.fontWeight}`,
  },
  {
    name: 'section vertical padding',
    pick: (el) => el.tagName === 'SECTION',
    read: (cs) => `${cs.paddingTop} / ${cs.paddingBottom}`,
  },
  {
    name: 'container max-width',
    pick: (el, cs) => cs.maxWidth !== 'none' && el.children.length > 0,
    read: (cs) => cs.maxWidth,
  },
];

// Attached to globalThis, not declared as a local: the `pick` bodies below are
// rebuilt with `new Function`, which evaluates in global scope and cannot see
// anything an `eval()` declared in this closure. Getting that wrong made three
// categories report "0 distinct values" — which reads as perfect consistency
// and was really the predicate throwing on every element.
function hasOwnTextSrc() {
  return `globalThis.hasOwnText = function (el) {
    for (const n of el.childNodes) {
      if (n.nodeType === 3 && n.textContent.trim().length > 1) return true;
    }
    return false;
  }`;
}

async function routes() {
  const res = await fetch(`${BASE}/sitemap.xml`);
  const xml = await res.text();
  return [...xml.matchAll(/<loc>([^<]+)<\/loc>/g)].map((m) =>
    m[1].replace(/^https?:\/\/[^/]+/, ''),
  );
}

async function main() {
  const list = await routes();
  if (list.length === 0) throw new Error('sitemap.xml yielded no routes');

  const browser = await chromium.launch();
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });

  // value -> Map(route -> count)
  const tally = new Map(CATEGORIES.map((c) => [c.name, new Map()]));

  for (const route of list) {
    const resp = await page.goto(BASE + route, { waitUntil: 'domcontentloaded' });
    if (!resp || resp.status() >= 400) throw new Error(`${route} -> ${resp && resp.status()}`);

    const found = await page.evaluate(
      ({ cats, helper }) => {
        eval(helper); // eslint-disable-line no-eval
        const out = {};
        for (const c of cats) {
          const pick = new Function('return ' + c.pick)();
          const read = new Function('return ' + c.read)();
          const seen = {};
          for (const el of document.querySelectorAll('main *, footer *, header *')) {
            const cs = getComputedStyle(el);
            if (cs.display === 'none' || cs.visibility === 'hidden') continue;
            // Deliberately not wrapped in try/catch: a predicate that throws
            // must crash the run, not quietly classify every element as "no".
            if (!pick(el, cs)) continue;
            const v = read(cs);
            seen[v] = (seen[v] || 0) + 1;
          }
          out[c.name] = seen;
        }
        return out;
      },
      {
        helper: hasOwnTextSrc(),
        cats: CATEGORIES.map((c) => ({
          name: c.name,
          pick: c.pick.toString(),
          read: c.read.toString(),
        })),
      },
    );

    for (const [cat, seen] of Object.entries(found)) {
      const bucket = tally.get(cat);
      for (const [value, count] of Object.entries(seen)) {
        if (!bucket.has(value)) bucket.set(value, new Map());
        bucket.get(value).set(route, count);
      }
    }
  }

  await browser.close();

  console.log(`Design drift across ${list.length} routes at 1440x900\n`);
  for (const c of CATEGORIES) {
    const bucket = tally.get(c.name);
    const rows = [...bucket.entries()]
      .map(([value, byRoute]) => ({
        value,
        total: [...byRoute.values()].reduce((a, b) => a + b, 0),
        routes: byRoute.size,
        where: [...byRoute.keys()],
      }))
      .sort((a, b) => b.total - a.total);
    // An empty category means the predicate matched nothing, which looks
    // identical to "flawlessly consistent" in the output. Never let it pass.
    if (rows.length === 0) {
      console.error(`FAIL: category "${c.name}" matched no elements on any route.`);
      console.error('      That is a broken predicate, not a clean result.');
      process.exitCode = 1;
      continue;
    }
    console.log(`${c.name.toUpperCase()} — ${rows.length} distinct value(s)`);
    for (const r of rows) {
      const rare = r.routes <= 3 ? `   <-- only ${r.where.join(', ')}` : '';
      console.log(`   ${String(r.total).padStart(4)}x  ${r.routes} route(s)  ${r.value}${rare}`);
    }
    console.log('');
  }
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
