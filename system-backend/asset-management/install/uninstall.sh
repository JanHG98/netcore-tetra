#!/usr/bin/env bash
set -euo pipefail
systemctl disable --now netcore-asset-management.service 2>/dev/null || true
rm -f /etc/systemd/system/netcore-asset-management.service /usr/local/bin/netcore-asset-management
systemctl daemon-reload
printf 'Konfiguration und Daten bleiben unter /etc/netcore und /var/lib/netcore-asset-management erhalten.\n'
