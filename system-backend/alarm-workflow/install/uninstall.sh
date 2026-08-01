#!/usr/bin/env bash
set -euo pipefail
systemctl disable --now netcore-alarm-workflow.service 2>/dev/null || true
rm -f /etc/systemd/system/netcore-alarm-workflow.service /usr/local/bin/netcore-alarm-workflow
systemctl daemon-reload
printf '%s\n' \
  'Konfiguration und Daten bleiben erhalten:' \
  '/etc/netcore/alarm-workflow.toml' \
  '/var/lib/netcore-alarm-workflow/'
