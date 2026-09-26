#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FALLBACK_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
source "${FALLBACK_DIR}/../install/check-host-role.sh"
netcore_check_sip_host_role tbs

# Check prerequisites before replacing any installed file or touching services.
if [[ ! -f /etc/netcore/tbs-sip-fallback.toml ]]; then
  cat >&2 <<EOF
Fallback-Konfiguration fehlt: /etc/netcore/tbs-sip-fallback.toml
Update abgebrochen; keine Dateien oder Dienste wurden geändert.
Bei einer bestehenden Installation zuerst die Konfiguration wiederherstellen.
Für eine Erstinstallation Parameter anzeigen mit:
  bash "${SCRIPT_DIR}/install-tbs-local-fallback.sh" --help
EOF
  exit 2
fi

install -m 0755 "${FALLBACK_DIR}/src/netcore_tbs_sip_fallback.py" /usr/local/bin/netcore-tbs-sip-fallback
install -m 0755 "${FALLBACK_DIR}/install/migrate-phase11c-config.py" /usr/local/lib/netcore-tbs-sip-migrate-phase11c.py
install -m 0644 "${FALLBACK_DIR}/systemd/netcore-tbs-sip-failover.service" /etc/systemd/system/netcore-tbs-sip-failover.service
python3 /usr/local/lib/netcore-tbs-sip-migrate-phase11c.py --config /etc/netcore/tbs-sip-fallback.toml
systemctl stop netcore-tbs-sip-failover.service
trap 'systemctl start netcore-tbs-sip-failover.service' EXIT
/usr/local/bin/netcore-tbs-sip-fallback --config /etc/netcore/tbs-sip-fallback.toml --render
systemctl daemon-reload
asterisk -rx "core reload"
systemctl enable netcore-tbs-sip-failover.service
systemctl restart netcore-tbs-sip-failover.service
trap - EXIT
/usr/local/bin/netcore-tbs-sip-fallback --config /etc/netcore/tbs-sip-fallback.toml --status || true
