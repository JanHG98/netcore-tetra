#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"

DASHBOARD_URL="${1:-http://127.0.0.1:8080}"
RF_MONITOR_URL="${2:-http://10.0.20.29:8260}"
STATION_ID="${3:-$(hostname -s)}"

install -d -m 0755 /etc/netcore
install -m 0755 "${ROOT}/system-backend/rf-monitor/examples/tbs-agent/netcore-rf-agent.py" /usr/local/bin/netcore-rf-agent
install -m 0644 "${ROOT}/system-backend/rf-monitor/examples/tbs-agent/netcore-rf-agent.service" /etc/systemd/system/netcore-rf-agent.service
if [[ ! -f /etc/netcore/rf-agent.toml ]]; then
  install -m 0600 "${ROOT}/system-backend/rf-monitor/examples/tbs-agent/rf-agent.example.toml" /etc/netcore/rf-agent.toml
  sed -i "s|station_id = \"TBS-01\"|station_id = \"${STATION_ID}\"|" /etc/netcore/rf-agent.toml
  sed -i "s|node_id = \"TBS-01\"|node_id = \"${STATION_ID}\"|" /etc/netcore/rf-agent.toml
  sed -i "s|name = \"NetCore TBS 01\"|name = \"${STATION_ID}\"|" /etc/netcore/rf-agent.toml
  sed -i "s|base_url = \"http://127.0.0.1:8080\"|base_url = \"${DASHBOARD_URL}\"|" /etc/netcore/rf-agent.toml
  sed -i "s|telemetry_url = \"http://10.0.20.29:8260/api/v1/telemetry\"|telemetry_url = \"${RF_MONITOR_URL%/}/api/v1/telemetry\"|" /etc/netcore/rf-agent.toml
fi
systemctl daemon-reload
systemctl enable --now netcore-rf-agent.service
systemctl --no-pager --full status netcore-rf-agent.service || true
