#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"

apt-get update
apt-get install -y --no-install-recommends python3 mosquitto-clients ca-certificates

python3 -m py_compile "${ROOT}/system-backend/alarm-workflow/src/netcore_alarm_workflow.py"
install -d -m 0755 /etc/netcore /var/lib/netcore-alarm-workflow
install -m 0755 "${ROOT}/system-backend/alarm-workflow/src/netcore_alarm_workflow.py" /usr/local/bin/netcore-alarm-workflow
install -m 0644 "${ROOT}/system-backend/alarm-workflow/systemd/netcore-alarm-workflow.service" /etc/systemd/system/netcore-alarm-workflow.service

if [[ ! -f /etc/netcore/alarm-workflow.toml ]]; then
  install -m 0644 "${ROOT}/system-backend/alarm-workflow/config/alarm-workflow.example.toml" /etc/netcore/alarm-workflow.toml
  IP="$(hostname -I | awk '{print $1}')"
  if [[ -n "${IP}" ]]; then
    sed -i "s/0.0.0.0:8270/${IP}:8270/" /etc/netcore/alarm-workflow.toml
  fi
fi

systemctl daemon-reload
systemctl enable --now netcore-alarm-workflow.service
systemctl --no-pager --full status netcore-alarm-workflow.service || true
