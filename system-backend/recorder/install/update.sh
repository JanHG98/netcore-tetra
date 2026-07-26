#!/usr/bin/env bash
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
if [[ $EUID -ne 0 ]]; then echo "Bitte als root/sudo ausführen." >&2; exit 1; fi
systemctl stop netcore-recorder.service 2>/dev/null || true
rm -f /usr/local/bin/netcore-recorder
rm -rf "$REPO_ROOT/target"
cd "$REPO_ROOT"
cargo clean
cargo build --release -p netcore-recorder
install -m 0755 target/release/netcore-recorder /usr/local/bin/netcore-recorder
install -m 0644 system-backend/recorder/systemd/netcore-recorder.service /etc/systemd/system/netcore-recorder.service
source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../../shared/install" && pwd)/lxc-network.sh"
netcore_configure_lxc_endpoint "/etc/netcore/recorder.toml" "recorder" "8140"
systemctl daemon-reload
systemctl restart netcore-recorder.service
systemctl --no-pager --full status netcore-recorder.service
