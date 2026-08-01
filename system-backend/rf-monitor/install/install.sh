#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"

apt-get update
apt-get install -y --no-install-recommends python3 mosquitto-clients ca-certificates
install -d -m 0755 /etc/netcore /var/lib/netcore-rf-monitor
install -m 0755 "${ROOT}/system-backend/rf-monitor/src/netcore_rf_monitor.py" /usr/local/bin/netcore-rf-monitor
install -m 0644 "${ROOT}/system-backend/rf-monitor/systemd/netcore-rf-monitor.service" /etc/systemd/system/netcore-rf-monitor.service

if [[ ! -f /etc/netcore/rf-monitor.toml ]]; then
  install -m 0644 "${ROOT}/system-backend/rf-monitor/config/rf-monitor.example.toml" /etc/netcore/rf-monitor.toml
  IP="$(hostname -I | awk '{print $1}')"
  if [[ -n "${IP}" ]]; then
    sed -i "s/0.0.0.0:8260/${IP}:8260/" /etc/netcore/rf-monitor.toml
  fi
fi

systemctl daemon-reload
systemctl enable --now netcore-rf-monitor.service
systemctl --no-pager --full status netcore-rf-monitor.service || true
