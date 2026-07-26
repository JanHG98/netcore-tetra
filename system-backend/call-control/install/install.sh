#!/usr/bin/env bash
set -euo pipefail
[[ ${EUID} -eq 0 ]] || { echo "Bitte als root ausführen." >&2; exit 1; }
cd "$(dirname "$0")/../../.."
systemctl stop netcore-call-control.service 2>/dev/null || true
rm -f /usr/local/bin/netcore-call-control
rm -rf target/release/netcore-call-control target/release/deps/netcore_call_control-* target/release/.fingerprint/netcore-call-control-*
cargo clean -p netcore-call-control 2>/dev/null || true
cargo build --release -p netcore-call-control
install -o root -g root -m 0755 target/release/netcore-call-control /usr/local/bin/netcore-call-control
install -d -o root -g root -m 0755 /etc/netcore
if [[ ! -f /etc/netcore/call-control.toml ]]; then
  install -o root -g root -m 0644 system-backend/call-control/config/call-control.example.toml /etc/netcore/call-control.toml
fi
if ! id netcore >/dev/null 2>&1; then
  useradd --system --home /var/lib/netcore-call-control --shell /usr/sbin/nologin netcore
fi
install -d -o netcore -g netcore -m 0750 /var/lib/netcore-call-control
install -o root -g root -m 0644 system-backend/call-control/systemd/netcore-call-control.service /etc/systemd/system/netcore-call-control.service
source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../../shared/install" && pwd)/lxc-network.sh"
netcore_configure_lxc_endpoint "/etc/netcore/call-control.toml" "call-control" "8120"
systemctl daemon-reload
systemctl enable --now netcore-call-control.service
