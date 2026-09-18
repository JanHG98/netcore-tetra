#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"
[[ ${EUID} -eq 0 ]] || { echo "Bitte als root ausführen." >&2; exit 1; }
cd "$REPO_ROOT"
# Compile while the installed service keeps running. Preserve configuration and
# the last working binary if compilation fails.
cargo build --release -p netcore-media-switch
install -o root -g root -m 0755 target/release/netcore-media-switch /usr/local/bin/netcore-media-switch.new
if [[ -f /usr/local/bin/netcore-media-switch ]]; then
  cp -p /usr/local/bin/netcore-media-switch /usr/local/bin/netcore-media-switch.previous
fi
mv -f /usr/local/bin/netcore-media-switch.new /usr/local/bin/netcore-media-switch
install -m 0644 system-backend/media-switch/systemd/netcore-media-switch.service /etc/systemd/system/netcore-media-switch.service
systemctl daemon-reload
systemctl restart netcore-media-switch.service
systemctl --no-pager --full status netcore-media-switch.service
