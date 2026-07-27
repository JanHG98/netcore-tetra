#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
if ! id -u netcore >/dev/null 2>&1; then
  useradd --system --home /var/lib/netcore-control-room --shell /usr/sbin/nologin netcore
fi
install -d -m 0755 /opt/netcore/control-room /etc/netcore-control-room
install -d -o netcore -g netcore -m 0750 /var/lib/netcore-control-room
cargo build --release --package netcore-control-room --manifest-path "$ROOT/Cargo.toml"
install -m 0755 "$ROOT/target/release/netcore-control-room" /usr/local/bin/netcore-control-room
install -m 0644 "$ROOT/system-backend/control-room/systemd/netcore-control-room.service" /etc/systemd/system/netcore-control-room.service
if [[ ! -f /etc/netcore-control-room/control-room.toml ]]; then
  install -o root -g netcore -m 0640 "$ROOT/system-backend/control-room/config/control-room.example.toml" /etc/netcore-control-room/control-room.toml
fi
chown root:netcore /etc/netcore-control-room/control-room.toml
chmod 0640 /etc/netcore-control-room/control-room.toml
source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../../shared/install" && pwd)/lxc-network.sh"
netcore_configure_lxc_endpoint "/etc/netcore-control-room/control-room.toml" "control-room" "9010"
chown root:netcore /etc/netcore-control-room/control-room.toml
chmod 0640 /etc/netcore-control-room/control-room.toml
systemctl daemon-reload
systemctl enable --now netcore-control-room.service
for _ in {1..20}; do
  systemctl is-active --quiet netcore-control-room.service && break
  sleep 0.25
done
if ! systemctl is-active --quiet netcore-control-room.service; then
  echo "Control Room konnte nicht gestartet werden. Letzte Logs:" >&2
  journalctl -u netcore-control-room.service -n 80 --no-pager >&2 || true
  exit 1
fi
if command -v curl >/dev/null 2>&1; then
  for _ in {1..20}; do
    curl -fsS "http://${NETCORE_DETECTED_LXC_IP}:9010/health/live" >/dev/null 2>&1 && break
    sleep 0.25
  done
  if ! curl -fsS "http://${NETCORE_DETECTED_LXC_IP}:9010/health/live" >/dev/null; then
    echo "Control Room läuft, aber der HTTP-Healthcheck ist nicht erreichbar." >&2
    ss -lntp | grep ':9010' >&2 || true
    journalctl -u netcore-control-room.service -n 80 --no-pager >&2 || true
    exit 1
  fi
fi
echo "Control Room WebUI erreichbar: http://${NETCORE_DETECTED_LXC_IP}:9010/"
