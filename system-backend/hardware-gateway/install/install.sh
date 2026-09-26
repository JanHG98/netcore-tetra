#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
apt-get update
apt-get install -y python3 mosquitto-clients
install -d /etc/netcore /var/lib/netcore-hardware-gateway
install -m 0755 "$ROOT/system-backend/hardware-gateway/src/netcore_hardware_gateway.py" /usr/local/bin/netcore-hardware-gateway
install -m 0644 "$ROOT/system-backend/hardware-gateway/systemd/netcore-hardware-gateway.service" /etc/systemd/system/
if [[ ! -f /etc/netcore/hardware-gateway.toml ]]; then
  cp "$ROOT/system-backend/hardware-gateway/config/hardware-gateway.example.toml" /etc/netcore/hardware-gateway.toml
  IP="$(hostname -I | awk '{print $1}')"; sed -i "s/0.0.0.0:8250/${IP}:8250/" /etc/netcore/hardware-gateway.toml
fi
systemctl daemon-reload
systemctl enable --now netcore-hardware-gateway
systemctl --no-pager --full status netcore-hardware-gateway || true
