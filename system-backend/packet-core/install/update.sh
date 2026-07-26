#!/usr/bin/env bash
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
if [[ $EUID -ne 0 ]]; then echo "Bitte als root/sudo ausführen." >&2; exit 1; fi
cd "$REPO_ROOT"
cargo build --release -p netcore-packet-core
systemctl stop netcore-packet-core.service
install -m 0755 target/release/netcore-packet-core /usr/local/bin/netcore-packet-core
install -m 0644 system-backend/packet-core/systemd/netcore-packet-core.service /etc/systemd/system/netcore-packet-core.service
source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../../shared/install" && pwd)/lxc-network.sh"
netcore_configure_lxc_endpoint "/etc/netcore/packet-core.toml" "packet-core" "8160"
systemctl daemon-reload
systemctl start netcore-packet-core.service
systemctl --no-pager --full status netcore-packet-core.service
