#!/usr/bin/env bash
set -euo pipefail
if [[ ! -f /etc/netcore/tbs-sip-fallback.toml ]]; then
  echo "Fallback nicht konfiguriert: /etc/netcore/tbs-sip-fallback.toml fehlt (Erstinstallation oder Wiederherstellung erforderlich)." >&2
  exit 2
fi
/usr/local/bin/netcore-tbs-sip-fallback --config /etc/netcore/tbs-sip-fallback.toml --status
