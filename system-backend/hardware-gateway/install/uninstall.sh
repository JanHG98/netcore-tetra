#!/usr/bin/env bash
set -euo pipefail
systemctl disable --now netcore-hardware-gateway 2>/dev/null || true
rm -f /etc/systemd/system/netcore-hardware-gateway.service /usr/local/bin/netcore-hardware-gateway
systemctl daemon-reload
