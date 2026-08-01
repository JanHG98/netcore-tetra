#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FALLBACK_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
install -m 0755 "${FALLBACK_DIR}/src/netcore_tbs_sip_fallback.py" /usr/local/bin/netcore-tbs-sip-fallback
install -m 0755 "${FALLBACK_DIR}/install/migrate-phase11c-config.py" /usr/local/lib/netcore-tbs-sip-migrate-phase11c.py
install -m 0644 "${FALLBACK_DIR}/systemd/netcore-tbs-sip-failover.service" /etc/systemd/system/netcore-tbs-sip-failover.service
python3 /usr/local/lib/netcore-tbs-sip-migrate-phase11c.py --config /etc/netcore/tbs-sip-fallback.toml
/usr/local/bin/netcore-tbs-sip-fallback --config /etc/netcore/tbs-sip-fallback.toml --render
systemctl daemon-reload
systemctl restart asterisk.service
systemctl enable --now netcore-tbs-sip-failover.service
/usr/local/bin/netcore-tbs-sip-fallback --config /etc/netcore/tbs-sip-fallback.toml --status || true
