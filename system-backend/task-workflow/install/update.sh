#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SERVICE_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
REPO_ROOT="$(cd "${SERVICE_DIR}/../.." && pwd)"

python3 -m py_compile "${SERVICE_DIR}/src/netcore_task_workflow.py"
install -d -m 0755 /etc/netcore /var/lib/netcore-task-workflow
install -m 0755 "${SERVICE_DIR}/src/netcore_task_workflow.py" /usr/local/bin/netcore-task-workflow
if [[ ! -f /etc/netcore/task-workflow.toml ]]; then
  install -m 0644 "${SERVICE_DIR}/config/task-workflow.example.toml" /etc/netcore/task-workflow.toml
fi
install -m 0644 "${SERVICE_DIR}/systemd/netcore-task-workflow.service" /etc/systemd/system/netcore-task-workflow.service

source "${REPO_ROOT}/system-backend/shared/install/lxc-network.sh"
netcore_configure_lxc_endpoint "/etc/netcore/task-workflow.toml" "task-workflow" "8280"

systemctl daemon-reload
systemctl enable netcore-task-workflow.service
systemctl restart netcore-task-workflow.service
systemctl status netcore-task-workflow.service --no-pager --full || true
