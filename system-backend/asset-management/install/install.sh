#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SERVICE_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
REPO_ROOT="$(cd "${SERVICE_DIR}/../.." && pwd)"
apt-get update
apt-get install -y --no-install-recommends python3 mosquitto-clients ca-certificates
install -d -m 0755 /etc/netcore /var/lib/netcore-asset-management
install -m 0755 "${SERVICE_DIR}/src/netcore_asset_management.py" /usr/local/bin/netcore-asset-management
if [[ ! -f /etc/netcore/asset-management.toml ]]; then
  install -m 0644 "${SERVICE_DIR}/config/asset-management.example.toml" /etc/netcore/asset-management.toml
fi
install -m 0644 "${SERVICE_DIR}/systemd/netcore-asset-management.service" /etc/systemd/system/netcore-asset-management.service
source "${REPO_ROOT}/system-backend/shared/install/lxc-network.sh"
netcore_configure_lxc_endpoint "/etc/netcore/asset-management.toml" "asset-management" "8290"
systemctl daemon-reload
systemctl enable --now netcore-asset-management.service
systemctl status netcore-asset-management.service --no-pager --full || true
