#!/usr/bin/env bash
set -euo pipefail
systemctl disable --now netcore-task-workflow.service 2>/dev/null || true
rm -f /etc/systemd/system/netcore-task-workflow.service /usr/local/bin/netcore-task-workflow
systemctl daemon-reload
printf '%s\n' 'Konfiguration und Daten bleiben unter /etc/netcore und /var/lib/netcore-task-workflow erhalten.'
