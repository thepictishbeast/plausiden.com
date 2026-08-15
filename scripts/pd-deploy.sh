#!/usr/bin/env bash
#
# pd-deploy — atomically deploy the plausiden-site release binary
# and static assets to /opt/plausiden-site, restart the systemd unit,
# and verify the new binary is serving.
#
# Usage:    sudo /home/admin/plausiden-site/scripts/pd-deploy.sh
# Optional: pass --dry-run to print actions without executing.
#
# Designed to be invoked via NOPASSWD sudoers entry — see
# scripts/pd-deploy.sudoers for the rule.
#
# Exit codes:
#   0   success
#   1   build artifact missing
#   2   copy/rename failed
#   3   service did not become active within timeout
#   4   sanity check (curl /healthz) failed

set -euo pipefail

DRY_RUN=0
[[ "${1:-}" == "--dry-run" ]] && DRY_RUN=1

run() {
    if [[ $DRY_RUN -eq 1 ]]; then
        echo "DRY:  $*"
    else
        echo "+ $*"
        "$@"
    fi
}

# Resolved from this script's own location, so the deploy always ships the
# checkout it was invoked from. The previous hardcoded /home/admin/plausiden-site
# stopped existing, which meant every deploy through this script failed at step 1
# and the real deploys were being done by hand.
SRC_DIR="${PD_SRC_DIR:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
DST_DIR="/opt/plausiden-site"
BIN_SRC="$SRC_DIR/target/release/plausiden-site"
BIN_DST="$DST_DIR/plausiden-site"
STAGING="$DST_DIR/plausiden-site.new"
STATIC_SRC="$SRC_DIR/static/."
STATIC_DST="$DST_DIR/static/"
CMS_SRC="$SRC_DIR/cms-store/."
CMS_DST="$DST_DIR/cms-store/"
SERVICE="plausiden-site"

# 1. Verify build artifact exists.
if [[ ! -x "$BIN_SRC" ]]; then
    echo "FAIL: $BIN_SRC missing or not executable. Run \`cargo build --release\` first." >&2
    exit 1
fi

# 2. Stage new binary at a non-busy filename.
run cp "$BIN_SRC" "$STAGING"

# 2a. Force mode and ownership rather than inheriting them from the build tree.
#
# cargo writes target/release with the umask of whoever built it. A build tree
# under a 750 home directory yields a 750 binary, cp preserves that, and the
# service — which runs as `plausiden`, not root — then fails at EXEC with
# "Permission denied" (status 203). That took the site down once. The mode is a
# property of the deployment, so set it explicitly instead of hoping.
run chmod 755 "$STAGING"
run chown root:root "$STAGING"

# 3. Atomic rename onto the busy executable. Linux allows this.
run mv -f "$STAGING" "$BIN_DST"

# 4. Mirror static assets (cp -r is fine — these are not held open).
run cp -r "$STATIC_SRC" "$STATIC_DST"

# 4a. Normalise ownership and mode on the copied assets.
#
# cp preserves whatever the source had, and the source is whatever umask the
# person or script that generated the file happened to run under. The Open
# Graph cards arrived as root:root 640 inside a 750 directory; the service runs
# as `plausiden`, so every card 404'd while the page happily advertised it in a
# meta tag. Same failure as the 750 binary in step 2a, one directory over:
# a file's mode is a property of the deployment, not of its author.
run chown -R root:root "$STATIC_DST"
run find "$STATIC_DST" -type d -exec chmod 755 {} +
run find "$STATIC_DST" -type f -exec chmod 644 {} +

# 4b. Mirror the CMS content store.
#
# This step did not exist. The binary and static/ deployed; cms-store/ did not,
# so the /docs pages served whatever TOML happened to be on the box — a copy
# from 10 July, months stale. Editing that content in the repo, committing it,
# and deploying changed nothing at all, and nothing reported a failure: the
# pages still rendered, just from the old text. Caught by fixing a banned word
# in /docs/ecosystem, deploying, and finding the word still live.
#
# Same ownership and mode normalisation as static/, for the same reason: the
# service runs as `plausiden` and cannot read what it is not permitted to.
run cp -r "$CMS_SRC" "$CMS_DST"
run chown -R root:root "$CMS_DST"
run find "$CMS_DST" -type d -exec chmod 755 {} +
run find "$CMS_DST" -type f -exec chmod 644 {} +

# 5. Restart service.
run systemctl restart "$SERVICE"

# 6. Wait for active state, max 15 seconds.
for i in $(seq 1 15); do
    if systemctl is-active --quiet "$SERVICE"; then
        echo "OK:   $SERVICE active after ${i}s"
        break
    fi
    sleep 1
    if [[ $i -eq 15 ]]; then
        echo "FAIL: $SERVICE did not become active within 15s" >&2
        systemctl status "$SERVICE" --no-pager >&2
        exit 3
    fi
done

# 7. Sanity check: hit /healthz on loopback. Skipped on dry-run.
if [[ $DRY_RUN -eq 0 ]]; then
    if ! curl -fsS http://127.0.0.1:8080/healthz | grep -q '^ok$'; then
        echo "FAIL: /healthz did not return 'ok'" >&2
        exit 4
    fi
    echo "OK:   /healthz returns 'ok'"
fi

echo "DEPLOY COMPLETE"
