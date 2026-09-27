#!/usr/bin/env bash
set -Eeuo pipefail
if [[ ${1:-controller} == vm ]]; then
  shift
  exec bash "$(dirname -- "${BASH_SOURCE[0]}")/install-vm.sh" "$@"
fi
if [[ ${1:-controller} == controller && -f /etc/systemd/system/netcore-image-builder.service ]]; then
  [[ $# -eq 0 ]] || shift
  exec bash "$(dirname -- "${BASH_SOURCE[0]}")/install-vm.sh" "$@"
fi
exec bash "$(dirname -- "${BASH_SOURCE[0]}")/install.sh" "$@"
