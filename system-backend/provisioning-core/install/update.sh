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
  echo "cargo wurde nicht gefunden." >&2
  exit 1
}
CARGO_BIN="$(find_cargo)"
export PATH="$(dirname "$CARGO_BIN"):$PATH"
"$CARGO_BIN" build --release -p netcore-provisioning-core
install -o root -g root -m 0755 target/release/netcore-provisioning-core /usr/local/bin/netcore-provisioning-core
install -o root -g root -m 0644 system-backend/provisioning-core/systemd/netcore-provisioning-core.service /etc/systemd/system/netcore-provisioning-core.service
systemctl daemon-reload
systemctl restart netcore-provisioning-core.service
systemctl --no-pager --full status netcore-provisioning-core.service
