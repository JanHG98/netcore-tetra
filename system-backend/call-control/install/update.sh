#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"
[[ ${EUID} -eq 0 ]] || { echo "Bitte als root ausführen." >&2; exit 1; }
cd "$REPO_ROOT"
# Compile while the installed service keeps running. Preserve configuration and
# the last working binary if compilation fails.
cargo build --release -p netcore-call-control
install -o root -g root -m 0755 target/release/netcore-call-control /usr/local/bin/netcore-call-control.new
if [[ -f /usr/local/bin/netcore-call-control ]]; then
  cp -p /usr/local/bin/netcore-call-control /usr/local/bin/netcore-call-control.previous
fi
mv -f /usr/local/bin/netcore-call-control.new /usr/local/bin/netcore-call-control
install -m 0644 system-backend/call-control/systemd/netcore-call-control.service /etc/systemd/system/netcore-call-control.service
systemctl daemon-reload
systemctl restart netcore-call-control.service
systemctl --no-pager --full status netcore-call-control.service
