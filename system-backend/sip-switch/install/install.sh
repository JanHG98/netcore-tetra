#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SERVICE_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
REPO_ROOT="$(cd "${SERVICE_DIR}/../.." && pwd)"

apt-get update
DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
  asterisk python3 mosquitto-clients ca-certificates curl

install -d -m 0755 /etc/netcore /var/lib/netcore-sip-switch /var/lib/asterisk/agi-bin
install -m 0755 "${SERVICE_DIR}/src/netcore_sip_switch.py" /usr/local/bin/netcore-sip-switch
install -m 0755 "${SERVICE_DIR}/agi/netcore-sip-route.py" /var/lib/asterisk/agi-bin/netcore-sip-route.py
if [[ ! -f /etc/netcore/sip-switch.toml ]]; then
  install -m 0640 "${SERVICE_DIR}/config/sip-switch.example.toml" /etc/netcore/sip-switch.toml
fi
install -m 0644 "${SERVICE_DIR}/systemd/netcore-sip-switch.service" /etc/systemd/system/netcore-sip-switch.service

source "${REPO_ROOT}/system-backend/shared/install/lxc-network.sh"
netcore_configure_lxc_endpoint "/etc/netcore/sip-switch.toml" "sip-switch" "8300"

SIP_SWITCH_IP="${NETCORE_LXC_IP:-$(hostname -I | awk '{print $1}')}"
printf 'NETCORE_SIP_SWITCH_URL=http://%s:8300\n' "$SIP_SWITCH_IP" >/etc/netcore/sip-switch-agi.env
chmod 0644 /etc/netcore/sip-switch-agi.env

ensure_include() {
  local file="$1"
  local include="$2"
  touch "$file"
  if ! grep -Fqx "#include ${include}" "$file"; then
    printf '\n; NetCore-TETRA managed include\n#include %s\n' "$include" >>"$file"
  fi
}
ensure_include /etc/asterisk/pjsip.conf netcore-pjsip.conf
ensure_include /etc/asterisk/extensions.conf netcore-extensions.conf
ensure_include /etc/asterisk/rtp.conf netcore-rtp.conf

/usr/local/bin/netcore-sip-switch --config /etc/netcore/sip-switch.toml --render-asterisk
systemctl daemon-reload
systemctl enable --now asterisk.service
systemctl restart asterisk.service
systemctl enable --now netcore-sip-switch.service
systemctl status netcore-sip-switch.service --no-pager --full || true
printf '\nSIP Switch WebUI: http://%s:8300/\nSIP listener: %s:5060/udp\n' "$SIP_SWITCH_IP" "$SIP_SWITCH_IP"
