#!/usr/bin/env bash
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
[[ $EUID -eq 0 ]] || { echo "Bitte als root/sudo ausführen." >&2; exit 1; }
cd "$REPO_ROOT"
cargo build --release -p netcore-ip-gateway
systemctl stop netcore-ip-gateway.service
install -m 0755 target/release/netcore-ip-gateway /usr/local/bin/netcore-ip-gateway
install -m 0644 system-backend/ip-gateway/systemd/netcore-ip-gateway.service /etc/systemd/system/netcore-ip-gateway.service
source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../../shared/install" && pwd)/lxc-network.sh"
netcore_configure_lxc_endpoint "/etc/netcore/ip-gateway.toml" "ip-gateway" "8170"
systemctl daemon-reload
systemctl start netcore-ip-gateway.service
systemctl --no-pager --full status netcore-ip-gateway.service
