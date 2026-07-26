#!/usr/bin/env bash
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
if [[ $EUID -ne 0 ]]; then echo "Bitte als root/sudo ausführen." >&2; exit 1; fi
systemctl stop netcore-media-switch.service 2>/dev/null || true
rm -f /usr/local/bin/netcore-media-switch
rm -rf "$REPO_ROOT/target"
cd "$REPO_ROOT"
cargo clean
cargo build --release -p netcore-media-switch
install -m 0755 target/release/netcore-media-switch /usr/local/bin/netcore-media-switch
install -m 0644 system-backend/media-switch/systemd/netcore-media-switch.service /etc/systemd/system/netcore-media-switch.service
source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../../shared/install" && pwd)/lxc-network.sh"
netcore_configure_lxc_endpoint "/etc/netcore/media-switch.toml" "media-switch" "8130"
systemctl daemon-reload
systemctl restart netcore-media-switch.service
systemctl --no-pager --full status netcore-media-switch.service
