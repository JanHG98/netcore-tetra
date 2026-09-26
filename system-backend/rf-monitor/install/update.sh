#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"

python3 -m py_compile "${ROOT}/system-backend/rf-monitor/src/netcore_rf_monitor.py"
install -m 0755 "${ROOT}/system-backend/rf-monitor/src/netcore_rf_monitor.py" /usr/local/bin/netcore-rf-monitor
install -m 0644 "${ROOT}/system-backend/rf-monitor/systemd/netcore-rf-monitor.service" /etc/systemd/system/netcore-rf-monitor.service
if [[ ! -f /etc/netcore/rf-monitor.toml ]]; then
  install -d -m 0755 /etc/netcore
  install -m 0644 "${ROOT}/system-backend/rf-monitor/config/rf-monitor.example.toml" /etc/netcore/rf-monitor.toml
fi
install -d -m 0755 /var/lib/netcore-rf-monitor
systemctl daemon-reload
systemctl restart netcore-rf-monitor.service
systemctl --no-pager --full status netcore-rf-monitor.service || true
