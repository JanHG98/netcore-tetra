#!/usr/bin/env bash
set -Eeuo pipefail
[[ ${EUID} -eq 0 ]] || { echo 'Bitte als root ausfuehren.' >&2; exit 1; }
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
apt-get update
apt-get install -y --no-install-recommends python3 ca-certificates curl
exec bash "${SCRIPT_DIR}/deploy.sh" install
