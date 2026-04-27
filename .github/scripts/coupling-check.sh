#!/usr/bin/env bash
#
# backend-frontend coupling check.
#
# VENDORED COPY of the canonical script at:
#   https://github.com/thepictishbeast/PlausiDen-Audits/blob/main/audits/backend-frontend/check.sh
#
# Vendored because PlausiDen-Audits is currently a private repo and
# raw.githubusercontent.com 404s without a PAT. When the audits repo
# goes public (or a PAT is wired into Actions secrets), delete this
# file and switch the workflow back to a curl from origin.
#
# Walks an Axum + Maud Rust project and reports orphans:
#
#   1. Backend routes (`.route("/<path>"`) with no frontend consumer
#      (`href="/<path>"` or `action="/<path>"` anywhere in views/).
#
#   2. Frontend hrefs/actions to local paths that don't resolve to a
#      registered backend route or static asset.
#
# Honors `// COUPLING-EXEMPT: <reason>` annotations on either side —
# any line containing that marker is ignored by the check.
#
# Usage:
#   ./check.sh <project-root>
#
# Exit codes:
#   0   no unannotated orphans
#   1   orphans found (printed to stdout)
#   2   misuse / missing project root

set -euo pipefail

ROOT="${1:-}"
if [[ -z "$ROOT" ]]; then
    echo "usage: $0 <project-root>" >&2
    exit 2
fi
if [[ ! -d "$ROOT" ]]; then
    echo "not a directory: $ROOT" >&2
    exit 2
fi

# --- 1. Collect declared backend routes ----------------------------
# Match `.route("/path"` or `.route("/path/{param}"` lines, extract the
# literal path. Skip any line carrying COUPLING-EXEMPT.
ROUTES=$(grep -rEA1 '\.route\(' "$ROOT/src" 2>/dev/null \
    | grep -v 'COUPLING-EXEMPT' \
    | grep -oE '"/[a-zA-Z0-9_{}/.-]*"' \
    | tr -d '"' \
    | sort -u || true)

# --- 2. Collect declared static asset prefixes ---------------------
# `nest_service("/static",...)` mounts a tree; everything under that
# prefix is allowed.
STATIC_PREFIXES=$(grep -rEA1 '\.nest_service\(' "$ROOT/src" 2>/dev/null \
    | grep -v 'COUPLING-EXEMPT' \
    | grep -oE '"/[a-zA-Z0-9_/.-]+"' \
    | tr -d '"' \
    | sort -u || true)

# --- 3. Collect frontend href + action references -------------------
# Three forms collected, all from src/ (not just views/):
#   a) Maud attribute: href="/path" or action="/path"
#   b) Rust struct field: href: "/path"
#   c) HTML-rendered string: href=\"/path\" inside escaped Rust strings
# Skip COUPLING-EXEMPT lines.
HREFS_ATTR=$(grep -rE '(href|action)="[^"]+"' "$ROOT/src" 2>/dev/null \
    | grep -v 'COUPLING-EXEMPT' \
    | grep -oE '(href|action)="[^"]+"' \
    | sed -E 's/^(href|action)="([^"]+)"$/\2/' || true)
HREFS_FIELD=$(grep -rE 'href:[[:space:]]*"/' "$ROOT/src" 2>/dev/null \
    | grep -v 'COUPLING-EXEMPT' \
    | grep -oE 'href:[[:space:]]*"[^"]+"' \
    | sed -E 's/^href:[[:space:]]*"([^"]+)"$/\1/' || true)
HREFS=$(printf '%s\n%s\n' "$HREFS_ATTR" "$HREFS_FIELD" \
    | grep -E '^/' \
    | sort -u || true)

# --- 4. Find orphan backend routes (no frontend consumer) ----------
# A route is a consumer match if any href/action equals the route
# OR matches the prefix-with-param form (e.g. /blog/{slug} consumed
# by /blog/federated-rule-learning).
ORPHAN_ROUTES=()
while IFS= read -r route; do
    [[ -z "$route" ]] && continue
    # Strip trailing /{param} so /blog/{slug} matches /blog/anything
    route_prefix=$(echo "$route" | sed -E 's|\{[^}]+\}.*||; s|/$||')
    found=0
    while IFS= read -r href; do
        [[ -z "$href" ]] && continue
        if [[ "$href" == "$route" ]] || [[ -n "$route_prefix" && "$href" == "$route_prefix"* ]]; then
            found=1
            break
        fi
    done <<< "$HREFS"
    if [[ $found -eq 0 ]]; then
        ORPHAN_ROUTES+=("$route")
    fi
done <<< "$ROUTES"

# --- 5. Find orphan frontend hrefs (no backend route + not static) -
ORPHAN_HREFS=()
while IFS= read -r href; do
    [[ -z "$href" ]] && continue
    found=0
    # Static prefix match.
    while IFS= read -r prefix; do
        [[ -z "$prefix" ]] && continue
        if [[ "$href" == "$prefix"* ]]; then
            found=1
            break
        fi
    done <<< "$STATIC_PREFIXES"
    [[ $found -eq 1 ]] && continue
    # Route match (exact or under a /{param} pattern).
    while IFS= read -r route; do
        [[ -z "$route" ]] && continue
        route_prefix=$(echo "$route" | sed -E 's|\{[^}]+\}.*||; s|/$||')
        if [[ "$href" == "$route" ]] || [[ -n "$route_prefix" && "$href" == "$route_prefix"* ]]; then
            found=1
            break
        fi
    done <<< "$ROUTES"
    if [[ $found -eq 0 ]]; then
        ORPHAN_HREFS+=("$href")
    fi
done <<< "$HREFS"

# --- 6. Report ------------------------------------------------------
exit_code=0

echo "== backend-frontend coupling check: $ROOT =="
echo "Routes declared:   $(echo "$ROUTES" | wc -l)"
echo "Static prefixes:   $(echo "$STATIC_PREFIXES" | wc -l)"
echo "Frontend refs:     $(echo "$HREFS" | wc -l)"
echo

if [[ ${#ORPHAN_ROUTES[@]} -gt 0 ]]; then
    echo "FAIL: backend routes with no frontend consumer (${#ORPHAN_ROUTES[@]}):"
    for r in "${ORPHAN_ROUTES[@]}"; do
        echo "  - $r"
    done
    echo "  (Add a frontend consumer or annotate the route line with"
    echo "   '// COUPLING-EXEMPT: <reason>' to suppress.)"
    echo
    exit_code=1
else
    echo "OK: every backend route has a frontend consumer."
fi

if [[ ${#ORPHAN_HREFS[@]} -gt 0 ]]; then
    echo "FAIL: frontend href/action targets with no backend route or static asset (${#ORPHAN_HREFS[@]}):"
    for h in "${ORPHAN_HREFS[@]}"; do
        echo "  - $h"
    done
    echo "  (Either add the backend route, fix the typo, or annotate"
    echo "   the offending line with '// COUPLING-EXEMPT: <reason>'.)"
    echo
    exit_code=1
else
    echo "OK: every frontend href resolves to a backend route or static asset."
fi

exit $exit_code
