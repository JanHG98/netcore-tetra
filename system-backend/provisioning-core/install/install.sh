#!/usr/bin/env bash
set -euo pipefail
[[ ${EUID} -eq 0 ]] || { echo "Bitte als root ausführen." >&2; exit 1; }
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT"

find_cargo() {
  if command -v cargo >/dev/null 2>&1; then command -v cargo; return; fi
  local candidate
  for candidate in /root/.cargo/bin/cargo /home/*/.cargo/bin/cargo; do
    [[ -x "$candidate" ]] && { printf '%s\n' "$candidate"; return; }
  done
  echo "cargo wurde nicht gefunden. Rust zuerst per rustup installieren." >&2
  exit 1
}
CARGO_BIN="$(find_cargo)"
export PATH="$(dirname "$CARGO_BIN"):$PATH"

systemctl stop netcore-provisioning-core.service 2>/dev/null || true
rm -f /usr/local/bin/netcore-provisioning-core
rm -rf target/release/netcore-provisioning-core target/release/deps/netcore_provisioning_core-* target/release/.fingerprint/netcore-provisioning-core-*
"$CARGO_BIN" build --release -p netcore-provisioning-core

getent group netcore >/dev/null || groupadd --system netcore
id netcore >/dev/null 2>&1 || useradd --system --gid netcore --home-dir /var/lib/netcore-provisioning-core --shell /usr/sbin/nologin netcore
install -d -o netcore -g netcore -m 0750 /var/lib/netcore-provisioning-core
install -d -o root -g root -m 0755 /etc/netcore
install -o root -g root -m 0755 target/release/netcore-provisioning-core /usr/local/bin/netcore-provisioning-core
if [[ ! -f /etc/netcore/provisioning-core.toml ]]; then
  install -o root -g root -m 0644 system-backend/provisioning-core/config/provisioning-core.example.toml /etc/netcore/provisioning-core.toml
fi
install -o root -g root -m 0644 system-backend/provisioning-core/systemd/netcore-provisioning-core.service /etc/systemd/system/netcore-provisioning-core.service

source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../../shared/install" && pwd)/lxc-network.sh"
netcore_configure_lxc_endpoint "/etc/netcore/provisioning-core.toml" "provisioning-core" "8125"

systemctl daemon-reload
systemctl enable --now netcore-provisioning-core.service
systemctl --no-pager --full status netcore-provisioning-core.service
echo "Provisioning Core installiert. Konfiguration: /etc/netcore/provisioning-core.toml"
echo "WebUI: http://$(hostname -I | awk '{print $1}'):8125/"
echo "WARNUNG: OPEN LAB ohne Anmeldung, Tokens oder TLS."
