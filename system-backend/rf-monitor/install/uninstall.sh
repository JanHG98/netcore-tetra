#!/usr/bin/env bash
set -euo pipefail
systemctl disable --now netcore-rf-monitor.service 2>/dev/null || true
rm -f /etc/systemd/system/netcore-rf-monitor.service /usr/local/bin/netcore-rf-monitor
systemctl daemon-reload
printf '%s\n' 'Konfiguration und Daten bleiben erhalten:' '/etc/netcore/rf-monitor.toml' '/var/lib/netcore-rf-monitor/'
