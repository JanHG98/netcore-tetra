#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"
install -m 0755 "$ROOT/system-backend/hardware-gateway/src/netcore_hardware_gateway.py" /usr/local/bin/netcore-hardware-gateway
install -m 0644 "$ROOT/system-backend/hardware-gateway/systemd/netcore-hardware-gateway.service" /etc/systemd/system/
systemctl daemon-reload
systemctl restart netcore-hardware-gateway
systemctl --no-pager --full status netcore-hardware-gateway || true
