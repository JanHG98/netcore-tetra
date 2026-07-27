#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
cargo build --release --package netcore-control-room --manifest-path "$ROOT/Cargo.toml"
systemctl stop netcore-control-room.service
install -m 0755 "$ROOT/target/release/netcore-control-room" /usr/local/bin/netcore-control-room
install -m 0644 "$ROOT/system-backend/control-room/systemd/netcore-control-room.service" /etc/systemd/system/netcore-control-room.service
source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../../shared/install" && pwd)/lxc-network.sh"
netcore_configure_lxc_endpoint "/etc/netcore-control-room/control-room.toml" "control-room" "9010"
chown root:netcore /etc/netcore-control-room/control-room.toml
chmod 0640 /etc/netcore-control-room/control-room.toml
systemctl daemon-reload
systemctl start netcore-control-room.service
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
