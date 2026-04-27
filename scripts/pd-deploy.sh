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

SRC_DIR="/home/admin/plausiden-site"
DST_DIR="/opt/plausiden-site"
BIN_SRC="$SRC_DIR/target/release/plausiden-site"
BIN_DST="$DST_DIR/plausiden-site"
STAGING="$DST_DIR/plausiden-site.new"
STATIC_SRC="$SRC_DIR/static/."
STATIC_DST="$DST_DIR/static/"
SERVICE="plausiden-site"

# 1. Verify build artifact exists.
if [[ ! -x "$BIN_SRC" ]]; then
    echo "FAIL: $BIN_SRC missing or not executable. Run \`cargo build --release\` first." >&2
    exit 1
fi

# 2. Stage new binary at a non-busy filename.
run cp "$BIN_SRC" "$STAGING"

# 3. Atomic rename onto the busy executable. Linux allows this.
run mv -f "$STAGING" "$BIN_DST"

# 4. Mirror static assets (cp -r is fine — these are not held open).
run cp -r "$STATIC_SRC" "$STATIC_DST"

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
