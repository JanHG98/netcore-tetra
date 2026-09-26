#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT=${REPO_ROOT:-$(cd "${SCRIPT_DIR}/../../.." && pwd)}
exec env REPO_ROOT="${REPO_ROOT}" INSTALL_LOCAL_MQTT_BROKER="${INSTALL_LOCAL_MQTT_BROKER:-0}" \
  "${REPO_ROOT}/system-backend/iot-gateway/install/install.sh"
