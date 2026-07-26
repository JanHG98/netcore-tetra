#!/usr/bin/env bash
set -euo pipefail
[[ ${EUID} -eq 0 ]] || { echo "Bitte als root ausführen." >&2; exit 1; }
cd "$(dirname "$0")/../../.."
systemctl stop netcore-group-core.service 2>/dev/null || true
rm -f /usr/local/bin/netcore-group-core
rm -rf target/release/netcore-group-core target/release/deps/netcore_group_core-* target/release/.fingerprint/netcore-group-core-*
cargo clean -p netcore-group-core 2>/dev/null || true
cargo build --release -p netcore-group-core
install -o root -g root -m 0755 target/release/netcore-group-core /usr/local/bin/netcore-group-core
install -d -o root -g root -m 0755 /etc/netcore
if [[ ! -f /etc/netcore/group-core.toml ]]; then install -o root -g root -m 0644 system-backend/group-core/config/group-core.example.toml /etc/netcore/group-core.toml; fi
if ! id netcore >/dev/null 2>&1; then useradd --system --home /var/lib/netcore-group-core --shell /usr/sbin/nologin netcore; fi
install -d -o netcore -g netcore -m 0750 /var/lib/netcore-group-core
install -o root -g root -m 0644 system-backend/group-core/systemd/netcore-group-core.service /etc/systemd/system/netcore-group-core.service
source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../../shared/install" && pwd)/lxc-network.sh"
netcore_configure_lxc_endpoint "/etc/netcore/group-core.toml" "group-core" "8110"
systemctl daemon-reload
systemctl enable --now netcore-group-core.service
