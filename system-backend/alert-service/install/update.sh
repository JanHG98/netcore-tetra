#!/usr/bin/env bash
set -Eeuo pipefail
[[ ${EUID} -eq 0 ]] || { echo 'Bitte als root ausfuehren.' >&2; exit 1; }
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
[[ -f /etc/netcore/alert-service.toml && -f /etc/netcore/alert-service.env ]] || {
  echo 'Keine vollstaendige Installation gefunden. Zuerst install.sh ausfuehren.' >&2
  exit 1
}
exec bash "${SCRIPT_DIR}/deploy.sh" update
