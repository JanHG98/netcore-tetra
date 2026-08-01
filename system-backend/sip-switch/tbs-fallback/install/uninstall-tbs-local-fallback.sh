#!/usr/bin/env bash
set -euo pipefail
systemctl disable --now netcore-tbs-sip-failover.service 2>/dev/null || true
rm -f /etc/systemd/system/netcore-tbs-sip-failover.service
rm -f /usr/local/bin/netcore-tbs-sip-fallback /usr/local/lib/netcore-tbs-sip-migrate-phase11c.py
rm -f /etc/asterisk/netcore-tbs-fallback-pjsip.conf
rm -f /etc/asterisk/netcore-tbs-fallback-extensions.conf
rm -f /etc/asterisk/netcore-tbs-fallback-rtp.conf
rm -f /etc/asterisk/netcore-registration-central.conf
rm -f /etc/asterisk/netcore-registration-pbx-direct.conf
rm -f /etc/asterisk/netcore-active-registration.conf
rm -f /etc/netcore/tbs-asterisk-local-snippet.toml
sed -i '/NetCore-TETRA Phase 11[bc] managed include/,+1d' /etc/asterisk/pjsip.conf /etc/asterisk/extensions.conf /etc/asterisk/rtp.conf 2>/dev/null || true
systemctl daemon-reload
systemctl restart asterisk.service || true
printf 'Phase 11c Dateien entfernt. /etc/netcore/tbs-sip-fallback.toml bleibt als Sicherung erhalten.\n'
