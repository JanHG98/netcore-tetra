#!/usr/bin/env bash
set -euo pipefail
systemctl disable --now netcore-sip-switch.service 2>/dev/null || true
rm -f /etc/systemd/system/netcore-sip-switch.service
rm -f /usr/local/bin/netcore-sip-switch
rm -f /var/lib/asterisk/agi-bin/netcore-sip-route.py
rm -f /etc/asterisk/netcore-pjsip.conf /etc/asterisk/netcore-extensions.conf /etc/asterisk/netcore-rtp.conf
systemctl daemon-reload
systemctl restart asterisk.service 2>/dev/null || true
printf 'Konfiguration /etc/netcore/sip-switch.toml und Daten /var/lib/netcore-sip-switch bleiben erhalten.\n'
