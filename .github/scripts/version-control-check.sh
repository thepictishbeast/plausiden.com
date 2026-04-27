#!/usr/bin/env bash
#
# version-control audit: automated portion.
#
# VENDORED COPY of the canonical script at:
#   https://github.com/thepictishbeast/PlausiDen-Audits/blob/main/audits/version-control/check.sh
#
# Vendored because PlausiDen-Audits is currently private; un-vendor
# when the audits repo goes public or a PAT is wired into Actions
# secrets. Same caveat as .github/scripts/coupling-check.sh.
#
# Walks a git repo and reports findings against the locally-checkable
# subset of audits/version-control/checklist.md. Items that need the
# GitHub API (branch protection rules, required status checks, key
# registration) are flagged as "manual" — they require gh CLI plus a
# repo-scoped token, which CI doesn't have for cross-repo audits.
#
# Usage:
#   ./check.sh <project-root>
#
# Exit codes:
#   0   no findings
#   1   findings detected (printed to stdout)
#   2   misuse / missing project root
#
# Findings are printed as `LEVEL  category  detail` lines, machine-
# parseable by a downstream collector.

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
if [[ ! -d "$ROOT/.git" ]]; then
    echo "not a git repo: $ROOT" >&2
    exit 2
fi

cd "$ROOT"

findings=0
report() {
    local level="$1"; shift
    local category="$1"; shift
    local detail="$*"
    printf '%-9s %-22s %s\n' "$level" "$category" "$detail"
    if [[ "$level" == "FAIL" ]]; then
        findings=$((findings + 1))
    fi
}

echo "== version-control audit: $ROOT =="
echo

# --- Author hygiene -------------------------------------------------
# A merged commit whose author is "root" or "unknown" is a finding —
# the audit trail must point at a real human or a named agent.
if git log --pretty='%ae' -100 2>/dev/null | grep -qiE '^(root|unknown|admin)@'; then
    bad=$(git log --pretty='%h %ae' -100 | grep -iE '^[a-f0-9]+ (root|unknown|admin)@' | head -3)
    report "FAIL" "author-hygiene" "commits authored by root/unknown/admin (last 100):"
    while IFS= read -r line; do
        echo "    $line"
    done <<< "$bad"
else
    report "OK" "author-hygiene" "no root/unknown commits in last 100"
fi

# --- Generated files not tracked -----------------------------------
# target/, node_modules/, dist/, build/, .venv/, __pycache__/ should
# never be checked in. If git tracks any path under these prefixes
# the .gitignore rule is missing or was bypassed.
GEN_PATHS=(target/ node_modules/ dist/ build/ .venv/ __pycache__/)
for p in "${GEN_PATHS[@]}"; do
    matches=$(git ls-files "$p" 2>/dev/null | head -3 || true)
    if [[ -n "$matches" ]]; then
        report "FAIL" "generated-tracked" "$p is tracked (sample):"
        while IFS= read -r f; do
            echo "    $f"
        done <<< "$matches"
    fi
done

# --- Large blobs ----------------------------------------------------
# Files >1MB in current HEAD bloat every clone forever. LFS or remove.
large=$(git ls-files | while IFS= read -r f; do
    if [[ -f "$f" ]]; then
        sz=$(wc -c < "$f")
        if (( sz > 1048576 )); then
            printf '%d %s\n' "$sz" "$f"
        fi
    fi
done | sort -rn | head -5)
if [[ -n "$large" ]]; then
    report "FAIL" "large-blobs" "files >1MB in HEAD (top 5):"
    while IFS= read -r line; do
        echo "    $line bytes"
    done <<< "$large"
else
    report "OK" "large-blobs" "no tracked files >1MB"
fi

# --- --no-verify in workflows --------------------------------------
# Bypassing pre-commit / pre-push hooks in CI defeats the gate the
# hook is meant to enforce. If a pipeline must skip a hook, the
# bypass should carry an inline justification (we look for the
# "AVP-" or "SHIP-DECISION:" prefix as an accepted exception marker).
if [[ -d .github/workflows ]] || [[ -f .gitlab-ci.yml ]] || [[ -d scripts ]]; then
    bypasses=$(grep -rn -- '--no-verify\|--no-gpg-sign\|skip-hooks' \
        .github/workflows/ .gitlab-ci.yml scripts/ 2>/dev/null \
        | grep -vE 'AVP-|SHIP-DECISION:' || true)
    if [[ -n "$bypasses" ]]; then
        report "FAIL" "hook-bypass" "unjustified --no-verify / skip-hooks in CI:"
        while IFS= read -r line; do
            echo "    $line"
        done <<< "$bypasses"
    else
        report "OK" "hook-bypass" "no unjustified hook bypasses in CI"
    fi
fi

# --- Lockfiles present and current ---------------------------------
# A manifest without a lockfile is a non-reproducible build.
declare -A MANIFEST_LOCK=(
    [Cargo.toml]=Cargo.lock
    [package.json]=package-lock.json
    [pyproject.toml]=uv.lock
    [Pipfile]=Pipfile.lock
    [go.mod]=go.sum
)
for manifest in "${!MANIFEST_LOCK[@]}"; do
    lock="${MANIFEST_LOCK[$manifest]}"
    if [[ -f "$manifest" && ! -f "$lock" ]]; then
        # Pipfile/uv have multiple acceptable lock filenames; we don't
        # try to enumerate every variant — manifest existence without
        # ANY companion lockfile is the actual failure mode.
        case "$manifest" in
            pyproject.toml)
                # uv.lock / poetry.lock / pdm.lock / requirements.txt
                if [[ ! -f uv.lock && ! -f poetry.lock && ! -f pdm.lock && ! -f requirements.txt ]]; then
                    report "WARN" "lockfile-missing" "$manifest present but no Python lockfile"
                fi
                ;;
            *)
                report "FAIL" "lockfile-missing" "$manifest present but $lock missing"
                ;;
        esac
    fi
done

# --- Signed commits on main ----------------------------------------
# Spot-check the last 20 commits on the current branch. CI runs in a
# detached HEAD typically, so we use HEAD; an unsigned recent commit
# is a finding (subject to the "agent commits without GPG" carve-out
# documented in the checklist — we WARN not FAIL since CI may run on
# a contributor branch).
unsigned=$(git log --pretty='%H %G?' -20 2>/dev/null | awk '$2 == "N"' | head -3)
if [[ -n "$unsigned" ]]; then
    report "WARN" "unsigned-commits" "unsigned commits in last 20 (sample):"
    while IFS= read -r line; do
        echo "    $line"
    done <<< "$unsigned"
fi

# --- Manual items --------------------------------------------------
# These cannot be checked from a single git repo + filesystem; they
# require GitHub API access with a token scoped to the target repo.
report "MANUAL" "branch-protection" "verify via gh api: required PR reviews, force-push disabled, deletion disabled"
report "MANUAL" "status-checks" "verify via gh api: required status checks present + up to date"
report "MANUAL" "secrets-history" "run gitleaks/trufflehog separately (heavy + needs full history)"
report "MANUAL" "release-tags" "verify release tags are ancestors of main + signed"

echo
if (( findings > 0 )); then
    echo "FAIL: $findings finding(s) above. Triage and either fix, ship-decision, or document as accepted."
    exit 1
fi
echo "OK: automated portion clean. Manual items above still require human verification."
exit 0
