#!/usr/bin/env node
//
// a11y-contrast — WCAG 2.1 SC 1.4.3 contrast sweep over every public route.
//
//   node scripts/a11y-contrast.mjs [--base http://127.0.0.1:8099]
//
// Exits 0 if every text element meets its threshold, 1 otherwise.
//
// WHY THIS IS A SCRIPT AND NOT A cargo test
//
// Contrast is a property of rendered pixels, not of markup. It needs the
// element's computed colour and the computed background of whatever ancestor
// actually paints behind it — which means the cascade, which means a browser.
// A static approximation was tried in this repo and reported eleven failures
// the browser had already cleared, all of them the footer tagline, because a
// string check cannot see an ancestor's background. `eyebrow_contrast` in
// src/lib.rs is deliberately narrow for that reason: it stops one specific
// regression and does not pretend to be an audit. This is the audit.
//
// WHY IT TESTS ITSELF FIRST
//
// A checker that silently stops checking reports a clean site, which is
// indistinguishable from a clean site right up until someone looks. This repo
// has been bitten by that shape three times: a `contains` that passed on a
// substring, a word-boundary that never matched a plural, and a test with no
// #[test] attribute that had never run once. So before trusting a pass, this
// injects colours of known ratio and asserts the detector catches exactly the
// ones it should. If the self-test fails the run aborts and nothing is
// reported as clean.
//
// It also reports COVERAGE. Anything it cannot resolve a solid background for
// is counted as skipped rather than passed, because "I did not look" and "I
// looked and it was fine" are different answers and only one of them is
// evidence.

// Playwright is not a dependency of this repo — it is a Rust project and
// should not grow a package.json for one audit. Resolve it from wherever the
// machine already has it. ESM ignores NODE_PATH, so this cannot be solved
// with an env var alone; PLAYWRIGHT_PATH is offered for anywhere unlisted.
async function loadChromium() {
  const candidates = [
    process.env.PLAYWRIGHT_PATH,
    'playwright',
    '/home/paul/e2e-probe/node_modules/playwright/index.js',
    '/usr/local/lib/node_modules/playwright/index.js',
  ].filter(Boolean);
  const tried = [];
  for (const c of candidates) {
    try {
      const mod = await import(c);
      return mod.chromium ?? mod.default?.chromium;
    } catch (e) {
      tried.push(`${c} (${e.code || 'failed'})`);
    }
  }
  throw new Error(
    `playwright not found. Tried:\n  ${tried.join('\n  ')}\n` +
    'Install it (npm i -g playwright) or set PLAYWRIGHT_PATH to its index.js.'
  );
}

const BASE = (() => {
  const i = process.argv.indexOf('--base');
  return i > -1 ? process.argv[i + 1] : 'http://127.0.0.1:8099';
})();

// Phone portrait, phone landscape (where the header broke once), tablet,
// desktop. Contrast does not change with width, but which elements render
// and at what font size does — large-text gets a 3.0 threshold, not 4.5.
const VIEWPORTS = [
  { w: 390, h: 844, name: '390x844' },
  { w: 844, h: 390, name: '844x390' },
  { w: 768, h: 1024, name: '768x1024' },
  { w: 1440, h: 900, name: '1440x900' },
];

/** Routes come from the site's own sitemap, never a list kept here. */
async function routes(page) {
  const res = await page.goto(`${BASE}/sitemap.xml`, { waitUntil: 'domcontentloaded' });
  if (!res || res.status() !== 200) {
    throw new Error(`cannot read ${BASE}/sitemap.xml — is the site running?`);
  }
  const xml = await page.content();
  const found = [...xml.matchAll(/<loc>\s*([^<\s]+)\s*<\/loc>/g)]
    .map((m) => new URL(m[1]).pathname);
  if (found.length < 5) {
    throw new Error(`sitemap yielded only ${found.length} routes; refusing to report a pass`);
  }
  return [...new Set(found)];
}

/**
 * Runs inside the page. Returns every text-bearing element that fails, plus
 * counts of what was examined and what could not be resolved.
 */
/* c8 ignore start */
function sweep() {
  const luminance = (rgb) => {
    const c = rgb.map((v) => {
      v /= 255;
      return v <= 0.03928 ? v / 12.92 : Math.pow((v + 0.055) / 1.055, 2.4);
    });
    return 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
  };
  const nums = (s) => (s.match(/[\d.]+/g) || []).map(Number);
  const contrast = (fg, bg) => {
    const [hi, lo] = [luminance(fg), luminance(bg)].sort((a, b) => b - a);
    return (hi + 0.05) / (lo + 0.05);
  };

  // Walk up for the first ancestor that paints an opaque colour. Returns null
  // rather than guessing when it meets a gradient or image it cannot flatten.
  const backgroundOf = (el) => {
    let n = el;
    while (n && n !== document.documentElement) {
      const cs = getComputedStyle(n);
      const c = nums(cs.backgroundColor);
      if (c.length >= 3 && (c[3] === undefined || c[3] > 0.95)) return c.slice(0, 3);
      if (cs.backgroundImage && cs.backgroundImage !== 'none') return null;
      n = n.parentElement;
    }
    return [255, 255, 255];
  };

  const failures = [];
  let examined = 0;
  let skipped = 0;

  for (const el of document.querySelectorAll('main *, header *, footer *')) {
    // Only elements holding their own text, so a paragraph is judged once
    // rather than once per wrapping div.
    const ownsText = [...el.childNodes]
      .some((n) => n.nodeType === Node.TEXT_NODE && n.textContent.trim().length > 1);
    if (!ownsText) continue;

    const cs = getComputedStyle(el);
    if (cs.visibility === 'hidden' || cs.display === 'none' || Number(cs.opacity) === 0) continue;
    const box = el.getBoundingClientRect();
    if (box.width === 0 || box.height === 0) continue;

    const bg = backgroundOf(el);
    if (!bg) { skipped += 1; continue; }
    examined += 1;

    const size = parseFloat(cs.fontSize);
    const bold = Number(cs.fontWeight) >= 700;
    // SC 1.4.3: 18pt (24px), or 14pt (18.66px) bold, counts as large text.
    const required = size >= 24 || (size >= 18.66 && bold) ? 3.0 : 4.5;
    const got = contrast(nums(cs.color).slice(0, 3), bg);

    if (got < required - 0.01) {
      failures.push({
        text: el.textContent.trim().slice(0, 60),
        ratio: Number(got.toFixed(2)),
        required,
        fontSize: Math.round(size),
        color: cs.color,
        background: `rgb(${bg.join(', ')})`,
        classes: String(el.className || '').slice(0, 70),
      });
    }
  }
  return { failures, examined, skipped };
}
/* c8 ignore stop */

/**
 * Prove the detector still detects. Injects three known ratios on white and
 * asserts the two failing ones are caught and the passing one is not.
 */
async function selfTest(page) {
  await page.goto(`${BASE}/`, { waitUntil: 'networkidle' });
  const result = await page.evaluate((sweepSrc) => {
    const run = new Function(`${sweepSrc}; return sweep();`);
    const host = document.querySelector('main');
    const mk = (colour, label) => {
      const p = document.createElement('p');
      p.textContent = `contrast self test ${label}`;
      p.style.color = colour;
      p.style.backgroundColor = 'rgb(255,255,255)';
      p.style.fontSize = '16px';
      host.appendChild(p);
      return p;
    };
    // 1.60:1 and 2.85:1 must fail; 4.60:1 must pass. Values chosen either
    // side of the 4.5 threshold so an off-by-one in the comparison shows up.
    const nodes = [mk('rgb(200,200,200)', 'a'), mk('rgb(148,148,148)', 'b'), mk('rgb(117,117,117)', 'c')];
    const { failures } = run();
    const caught = failures.filter((f) => f.text.startsWith('contrast self test'));
    nodes.forEach((n) => n.remove());
    return {
      caughtLabels: caught.map((f) => f.text.slice(-1)).sort(),
      ratios: caught.map((f) => f.ratio),
    };
  }, sweep.toString());

  const expected = ['a', 'b'];
  const got = result.caughtLabels;
  if (got.join(',') !== expected.join(',')) {
    throw new Error(
      `SELF-TEST FAILED: expected the detector to flag exactly ${expected.join(' and ')} ` +
      `(the sub-4.5 samples) but it flagged [${got.join(', ')}]. ` +
      'Refusing to report the site as clean with a broken detector.'
    );
  }
  return result.ratios;
}

const chromium = await loadChromium();
const browser = await chromium.launch();
const page = await browser.newPage();
let exitCode = 0;

try {
  const selfTestRatios = await selfTest(page);
  console.log(`self-test ok — detector caught planted ${selfTestRatios.join(':1 and ')}:1\n`);

  const paths = await routes(page);
  console.log(`sweeping ${paths.length} routes from sitemap.xml at ${VIEWPORTS.length} viewports\n`);

  let examined = 0;
  let skipped = 0;
  const failures = [];

  for (const vp of VIEWPORTS) {
    await page.setViewportSize({ width: vp.w, height: vp.h });
    for (const path of paths) {
      const res = await page.goto(BASE + path, { waitUntil: 'networkidle' });
      if (!res || res.status() !== 200) {
        console.error(`  ${path} — HTTP ${res ? res.status() : 'no response'}`);
        exitCode = 1;
        continue;
      }
      const r = await page.evaluate(sweep);
      examined += r.examined;
      skipped += r.skipped;
      for (const f of r.failures) failures.push({ ...f, route: path, viewport: vp.name });
    }
  }

  const coverage = examined + skipped === 0 ? 0 : (examined / (examined + skipped)) * 100;
  console.log(`examined ${examined} text elements, could not resolve a background for ${skipped}`);
  console.log(`coverage ${coverage.toFixed(1)}%\n`);

  if (failures.length) {
    exitCode = 1;
    console.error(`${failures.length} contrast failure(s):\n`);
    for (const f of failures) {
      console.error(
        `  ${f.route} @ ${f.viewport}\n` +
        `    "${f.text}"\n` +
        `    ${f.ratio}:1, needs ${f.required}:1 — ${f.color} on ${f.background} at ${f.fontSize}px\n` +
        `    class="${f.classes}"\n`
      );
    }
  } else {
    console.log('no contrast failures.');
  }
} catch (err) {
  console.error(String(err.message || err));
  exitCode = 1;
} finally {
  await browser.close();
}

process.exit(exitCode);
