#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "${SCRIPT_DIR}/../../.." && pwd)"
SERVICE_DIR="${ROOT}/system-backend/alarm-workflow"

apt-get update
apt-get install -y --no-install-recommends python3 mosquitto-clients ca-certificates

python3 -m py_compile "${SERVICE_DIR}/src/netcore_alarm_workflow.py" "${SERVICE_DIR}/src/netcore_alarm_workflow_launcher.py"
install -d -m 0755 \
  /etc/netcore \
  /var/lib/netcore-alarm-workflow \
  /usr/local/lib/netcore-alarm-workflow \
  /usr/local/share/netcore-alarm-workflow
install -m 0644 "${SERVICE_DIR}/src/netcore_alarm_workflow.py" /usr/local/lib/netcore-alarm-workflow/netcore_alarm_workflow.py
install -m 0755 "${SERVICE_DIR}/src/netcore_alarm_workflow_launcher.py" /usr/local/bin/netcore-alarm-workflow
install -m 0644 "${SERVICE_DIR}/web-ui/index.html" /usr/local/share/netcore-alarm-workflow/index.html
install -m 0644 "${SERVICE_DIR}/systemd/netcore-alarm-workflow.service" /etc/systemd/system/netcore-alarm-workflow.service

if [[ ! -f /etc/netcore/alarm-workflow.toml ]]; then
  install -m 0644 "${SERVICE_DIR}/config/alarm-workflow.example.toml" /etc/netcore/alarm-workflow.toml
  IP="$(hostname -I | awk '{print $1}')"
  if [[ -n "${IP}" ]]; then
    sed -i "s/0.0.0.0:8270/${IP}:8270/" /etc/netcore/alarm-workflow.toml
  fi
fi

systemctl daemon-reload
systemctl enable --now netcore-alarm-workflow.service
systemctl --no-pager --full status netcore-alarm-workflow.service || true
