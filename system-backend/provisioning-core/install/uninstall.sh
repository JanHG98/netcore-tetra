#!/usr/bin/env bash
set -euo pipefail
[[ ${EUID} -eq 0 ]] || { echo "Bitte als root ausführen." >&2; exit 1; }
systemctl disable --now netcore-provisioning-core.service 2>/dev/null || true
rm -f /etc/systemd/system/netcore-provisioning-core.service /usr/local/bin/netcore-provisioning-core
systemctl daemon-reload
echo "Binary und Dienst entfernt. /etc/netcore/provisioning-core.toml bleibt erhalten."
