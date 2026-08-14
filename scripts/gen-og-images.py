#!/usr/bin/env python3
"""Generate the per-route Open Graph cards in static/og/.

WHY THIS EXISTS

Every page shipped `og:image` pointing at `static/og-default.svg`. No major
platform renders SVG in a link preview — Slack, LinkedIn, X and Meta all
require a raster format — so every link to this site posted in an email or a
Slack channel unfurled with no image at all. On a B2B site whose links travel
almost entirely through those channels, that is the preview a buyer forms an
impression from.

These cards are raster PNGs at the 1200x630 every platform expects, one per
route, carrying that page's own headline rather than a shared generic panel.

RUNNING IT

    python3 scripts/gen-og-images.py

Output is committed. The script is deterministic: same inputs, same bytes, so
re-running it on an unchanged tree produces no diff. It needs `rsvg-convert`
(librsvg) and `fonttools`, both of which are build-time only — the server
serves plain files and has no image dependency.

FONTS

The brand faces ship as woff2 for the browser, which fontconfig cannot read.
The script decompresses them to TTF in a scratch directory and points
fontconfig at it for the duration of the render, so the cards are set in the
same Outfit the site uses rather than a fallback that would look like a
different company's artwork.
"""

from __future__ import annotations

import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path
from xml.sax.saxutils import escape

REPO = Path(__file__).resolve().parent.parent
STATIC = REPO / "static"
OUT = STATIC / "og"

# Brand palette, lifted from the existing card so the family stays recognisable.
BG_FROM, BG_TO = "#0d488a", "#0a2c52"
ACCENT_FROM, ACCENT_TO = "#3b82f6", "#1e40af"

# (slug, eyebrow, headline, footer note)
#
# The headline is the page's own promise, not a description of it. A preview
# that restates the page title twice wastes the only three seconds it gets.
CARDS: list[tuple[str, str, str, str]] = [
    (
        "default",
        "IT, security and disaster recovery",
        "The IT team for practices that hold confidential client data",
        "plausiden.com · Massachusetts",
    ),
    (
        "home",
        "IT, security and disaster recovery",
        "The IT team for practices that hold confidential client data",
        "Law firms · medical · financial advisers · newsrooms · nonprofits",
    ),
    (
        "services",
        "Services",
        "Comprehensive IT, specifically scoped",
        "PTES · NIST SP 800-115 · OWASP ASVS · MITRE ATT&CK",
    ),
    (
        "sample-report",
        "Sample deliverable",
        "Read the report before you buy",
        "A worked finding, end to end — reproduction, evidence, retest",
    ),
    (
        "pricing",
        "Pricing",
        "Published rates, and the arithmetic",
        "$1,600 per engineer-day · $1,200 flat · 30-day retest included",
    ),
    (
        "about",
        "About",
        "The unglamorous parts, done properly",
        "A woman-owned business in Massachusetts, small on purpose",
    ),
    (
        "case-studies",
        "Selected work",
        "Sanitized engagement summaries",
        "What mattered, what we shipped, what changed",
    ),
    (
        "how-we-work",
        "Engagement model",
        "Fixed scope, fixed price, named engineer",
        "Written proposals · scope-limited access · a real handoff",
    ),
    (
        "contact",
        "Contact",
        "Book a 45-minute scoping call",
        "Mutual NDA first, then a written proposal",
    ),
    (
        "capabilities",
        "Capabilities",
        "What we can actually do",
        "plausiden.com · Massachusetts",
    ),
]

# Baked faces: (unique family name, source family, weight on the wght axis).
FACE_DISPLAY = "PDDisplay600"
FACE_DISPLAY_BOLD = "PDDisplay700"
FACE_BODY = "PDBody400"
FACE_BODY_SEMI = "PDBody600"
FACES: list[tuple[str, str, int]] = [
    (FACE_DISPLAY, "Outfit", 600),
    (FACE_DISPLAY_BOLD, "Outfit", 700),
    (FACE_BODY, "Plus Jakarta Sans", 400),
    (FACE_BODY_SEMI, "Plus Jakarta Sans", 600),
]

# Rough advance width per character at 1px, measured for Outfit 600 and used
# only to decide line breaks. Being a little conservative costs a few pixels of
# right margin; being wrong the other way clips a headline.
WORDMARK = "PlausiDen LLC"

CHAR_W = 0.52


def wrap(text: str, font_px: int, max_px: int, max_lines: int) -> list[str]:
    """Greedy wrap on whole words."""
    limit = max(1, int(max_px / (font_px * CHAR_W)))
    words, lines, cur = text.split(), [], ""
    for w in words:
        cand = f"{cur} {w}".strip()
        if len(cand) <= limit or not cur:
            cur = cand
        else:
            lines.append(cur)
            cur = w
    if cur:
        lines.append(cur)
    if len(lines) > max_lines:
        lines = lines[:max_lines]
        lines[-1] = lines[-1].rstrip(" ,.") + "…"
    return lines


def card_svg(eyebrow: str, headline: str, note: str) -> str:
    """One 1200x630 card."""
    # Shrink the headline a step when it needs three lines, so a long promise
    # keeps the same optical weight as a short one.
    size = 68
    lines = wrap(headline, size, 1000, 3)
    if len(lines) >= 3:
        size = 58
        lines = wrap(headline, size, 1000, 3)

    line_h = int(size * 1.18)
    block_h = line_h * len(lines)
    start_y = 300 - block_h // 2 + size

    # First line's baseline is start_y; its cap top is roughly one font size
    # above that. Sit the eyebrow 34px clear of it.
    eyebrow_y = start_y - size - 34

    tspans = "".join(
        f'<tspan x="80" y="{start_y + i * line_h}">{escape(l)}</tspan>'
        for i, l in enumerate(lines)
    )

    return f"""<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1200 630" width="1200" height="630">
  <defs>
    <linearGradient id="bg" x1="0" y1="0" x2="1" y2="1">
      <stop offset="0%" stop-color="{BG_FROM}"/>
      <stop offset="100%" stop-color="{BG_TO}"/>
    </linearGradient>
    <linearGradient id="accent" x1="0" y1="0" x2="1" y2="1">
      <stop offset="0%" stop-color="{ACCENT_FROM}"/>
      <stop offset="100%" stop-color="{ACCENT_TO}"/>
    </linearGradient>
  </defs>

  <rect width="1200" height="630" fill="url(#bg)"/>

  <g opacity="0.06" stroke="#ffffff" stroke-width="1">
    <path d="M0 105H1200M0 210H1200M0 315H1200M0 420H1200M0 525H1200"/>
    <path d="M150 0V630M300 0V630M450 0V630M600 0V630M750 0V630M900 0V630M1050 0V630"/>
  </g>

  <!-- brand row -->
  <g transform="translate(80, 72)">
    <rect x="0" y="0" width="46" height="46" rx="10" fill="url(#accent)"/>
    <svg x="11" y="11" width="24" height="24" viewBox="0 0 24 24" fill="none"
         stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
      <path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/>
    </svg>
    <text x="62" y="31" font-family="{FACE_DISPLAY_BOLD}" font-size="26" fill="#ffffff">PlausiDen<tspan fill="#93c5fd" xml:space="preserve"> LLC</tspan></text>
  </g>

  <!-- eyebrow: sits a fixed gap above the cap-height of the first headline
       line. Deriving it from the block height put it on top of the wordmark
       whenever the headline wrapped to two lines. -->
  <text x="80" y="{eyebrow_y}" font-family="{FACE_BODY_SEMI}" font-size="19" letter-spacing="3.4" fill="#93c5fd">{escape(eyebrow.upper())}</text>

  <!-- headline -->
  <text font-family="{FACE_DISPLAY}" font-size="{size}" fill="#ffffff">{tspans}</text>

  <!-- hairline + note -->
  <path d="M80 545H1120" stroke="#ffffff" stroke-opacity="0.16" stroke-width="1"/>
  <text x="80" y="583" font-family="{FACE_BODY}" font-size="21"
        fill="#cbd5e1">{escape(note)}</text>
</svg>
"""


def build_font_dir(scratch: Path) -> Path:
    """Prepare TTF faces fontconfig can actually use.

    Three things make this less trivial than "decompress the woff2":

    1. The faces are Google Fonts *subsets*, sliced by unicode-range. One of
       the Plus Jakarta files carries seven glyphs. Picking the wrong slice
       gives a card of empty boxes, which is what the first attempt rendered.
       So: choose the slice with the widest cmap per family.
    2. They are variable fonts whose default instance is the lightest weight —
       the family name reads "Outfit Thin". Asking for `font-family="Outfit"`
       matches nothing, and asking for `font-weight="600"` on a variable face
       that fontconfig treats as static does not pick up the weight axis.
       So: pin each axis with the instancer and bake a static face.
    3. Two faces then share a family name and differ only by weight, which is
       exactly the ambiguity that produced thin headlines. So: give each baked
       face its own unmistakable family name and ask for that by name.

    System font directories are included as a fallback so a character missing
    from a subset degrades to a real glyph instead of a box.
    """
    from fontTools.ttLib import TTFont
    from fontTools.varLib import instancer

    fonts = scratch / "fonts"
    fonts.mkdir(parents=True, exist_ok=True)

    # Choose each family's slice by coverage of the characters these cards
    # actually set, NOT by glyph count.
    #
    # Counting glyphs picks the latin-ext slice, which has more of them and
    # almost none of the ones we need: its charset is "20 41 a0 c1 100-130..."
    # — space, a capital A, then Latin Extended-A. Every other character fell
    # back to the default monospace, so the footer line rendered as terminal
    # text on an otherwise typeset card. Score against the real corpus and the
    # right slice wins on merit.
    needed = set(WORDMARK)
    for _slug, eyebrow, headline, note in CARDS:
        needed |= set(eyebrow.upper()) | set(eyebrow) | set(headline) | set(note)
    needed.discard("\n")

    # family -> (best source path, covered, missing)
    best: dict[str, tuple[Path, int, set[str]]] = {}
    for src in sorted((STATIC / "fonts").glob("*.woff2")):
        try:
            f = TTFont(str(src), fontNumber=0)
            raw = f["name"].getDebugName(1) or ""
            # "Outfit Thin" -> "Outfit"; "Plus Jakarta Sans" stays.
            family = raw.replace(" Thin", "").strip()
            cmap = f.getBestCmap()
            missing = {c for c in needed if ord(c) not in cmap}
            covered = len(needed) - len(missing)
            if family not in best or covered > best[family][1]:
                best[family] = (src, covered, missing)
        except Exception as e:  # noqa: BLE001
            print(f"  skip {src.name}: {e}", file=sys.stderr)

    for family, (src, covered, missing) in sorted(best.items()):
        print(f"  {family}: {covered}/{len(needed)} needed characters ({src.name})")
        if missing:
            # Loud rather than fatal: a card missing one dash is still worth
            # shipping, but nobody should discover it by looking at a preview.
            shown = "".join(sorted(missing))[:40]
            print(f"    WARNING: no glyph for {shown!r}", file=sys.stderr)

    for face, family, weight in FACES:
        src = best.get(family)
        if not src:
            print(f"  MISSING source for {family}", file=sys.stderr)
            continue
        f = TTFont(str(src[0]), fontNumber=0)
        if "fvar" in f:
            axes = {a.axisTag: weight for a in f["fvar"].axes if a.axisTag == "wght"}
            if axes:
                f = instancer.instantiateVariableFont(f, axes, inplace=False)
        # Rename so each weight is its own family and nothing has to guess.
        for rec in f["name"].names:
            if rec.nameID in (1, 4, 16):
                rec.string = face
            elif rec.nameID in (2, 17):
                rec.string = "Regular"
        f.flavor = None
        f.save(str(fonts / f"{face}.ttf"))
        print(f"  baked {face} (from {family} @ wght {weight})")

    conf = scratch / "fonts.conf"
    conf.write_text(
        f"""<?xml version="1.0"?>
<!DOCTYPE fontconfig SYSTEM "fonts.dtd">
<fontconfig>
  <dir>{fonts}</dir>
  <dir>/usr/share/fonts</dir>
  <dir>/usr/local/share/fonts</dir>
  <cachedir>{scratch / "fc-cache"}</cachedir>
</fontconfig>
"""
    )
    return conf


def main() -> int:
    if not shutil.which("rsvg-convert"):
        print("rsvg-convert not found (install librsvg2-bin)", file=sys.stderr)
        return 1

    OUT.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory() as td:
        scratch = Path(td)
        conf = build_font_dir(scratch)
        env = dict(os.environ, FONTCONFIG_FILE=str(conf))

        for slug, eyebrow, headline, note in CARDS:
            svg_path = scratch / f"{slug}.svg"
            svg_path.write_text(card_svg(eyebrow, headline, note), encoding="utf-8")
            png_path = OUT / f"{slug}.png"
            subprocess.run(
                [
                    "rsvg-convert",
                    "--width=1200",
                    "--height=630",
                    "--format=png",
                    "--output",
                    str(png_path),
                    str(svg_path),
                ],
                check=True,
                env=env,
            )
            print(f"  {png_path.relative_to(REPO)}  {png_path.stat().st_size // 1024} KB")

    print(f"generated {len(CARDS)} cards")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
