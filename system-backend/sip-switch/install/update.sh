#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SERVICE_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
install -m 0755 "${SERVICE_DIR}/src/netcore_sip_switch.py" /usr/local/bin/netcore-sip-switch
install -m 0755 "${SERVICE_DIR}/agi/netcore-sip-route.py" /var/lib/asterisk/agi-bin/netcore-sip-route.py
install -m 0644 "${SERVICE_DIR}/systemd/netcore-sip-switch.service" /etc/systemd/system/netcore-sip-switch.service
if [[ ! -f /etc/netcore/sip-switch.toml ]]; then
  install -m 0640 "${SERVICE_DIR}/config/sip-switch.example.toml" /etc/netcore/sip-switch.toml
fi
SIP_SWITCH_BIND=$(sed -n '0,/^[[:space:]]*bind[[:space:]]*=/s/^[[:space:]]*bind[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' /etc/netcore/sip-switch.toml)
SIP_SWITCH_HOST=${SIP_SWITCH_BIND%:*}
if [[ -z "$SIP_SWITCH_HOST" || "$SIP_SWITCH_HOST" == "0.0.0.0" ]]; then
  SIP_SWITCH_HOST=127.0.0.1
fi
printf 'NETCORE_SIP_SWITCH_URL=http://%s:8300\n' "$SIP_SWITCH_HOST" >/etc/netcore/sip-switch-agi.env
chmod 0644 /etc/netcore/sip-switch-agi.env
/usr/local/bin/netcore-sip-switch --config /etc/netcore/sip-switch.toml --render-asterisk
systemctl daemon-reload
systemctl restart asterisk.service
systemctl restart netcore-sip-switch.service
systemctl status netcore-sip-switch.service --no-pager --full || true
