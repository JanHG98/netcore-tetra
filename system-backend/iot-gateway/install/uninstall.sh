#!/usr/bin/env bash
set -euo pipefail
if [[ ${EUID} -ne 0 ]]; then
  echo "Bitte als root ausführen." >&2
  exit 1
fi
systemctl disable --now netcore-iot-gateway.service 2>/dev/null || true
rm -f /etc/systemd/system/netcore-iot-gateway.service
rm -f /usr/local/bin/netcore-iot-gateway
systemctl daemon-reload
if [[ "${PURGE_CONFIG:-0}" == "1" ]]; then
  rm -f /etc/netcore/iot-gateway.toml
fi
if [[ "${PURGE_DATA:-0}" == "1" ]]; then
  rm -rf /var/lib/netcore-iot-gateway
fi
echo "IoT Gateway entfernt. Mosquitto wurde absichtlich nicht deinstalliert."
