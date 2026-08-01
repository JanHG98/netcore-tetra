#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FALLBACK_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

if [[ $# -lt 7 ]]; then
  cat >&2 <<'USAGE'
Usage:
  install-tbs-local-fallback.sh \
    <NODE-ID> <TBS-IP> <SIP-SWITCH-IP> <CENTRAL-USER> <CENTRAL-PASSWORD> \
    <PBX-IP> <PBX-FALLBACK-ID> [PBX-AUTH-USER] [PBX-PASSWORD]

Example OPEN LAB without PBX auth:
  ./install-tbs-local-fallback.sh \
    SRV-M-TBS-01 10.0.1.101 10.0.20.33 \
    tbs-srv-m-tbs-01 openlab-central \
    10.0.1.160 netcore-tbs-01
USAGE
  exit 2
fi

NODE_ID="$1"
TBS_IP="$2"
SWITCH_IP="$3"
CENTRAL_USER="$4"
CENTRAL_PASSWORD="$5"
PBX_IP="$6"
PBX_FALLBACK_ID="$7"
PBX_AUTH_USER="${8:-}"
PBX_PASSWORD="${9:-}"
NATIVE_USER="netcore-native-$(printf '%s' "$NODE_ID" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9_.:-]/-/g')"
NATIVE_PASSWORD="openlab-${NATIVE_USER}"

apt-get update
DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
  asterisk python3 ca-certificates curl

install -d -m 0755 /etc/netcore /var/lib/netcore-tbs-sip-fallback
install -m 0755 "${FALLBACK_DIR}/src/netcore_tbs_sip_fallback.py" /usr/local/bin/netcore-tbs-sip-fallback
install -m 0755 "${FALLBACK_DIR}/install/migrate-phase11c-config.py" /usr/local/lib/netcore-tbs-sip-migrate-phase11c.py
install -m 0644 "${FALLBACK_DIR}/systemd/netcore-tbs-sip-failover.service" /etc/systemd/system/netcore-tbs-sip-failover.service

if [[ -f /etc/netcore/tbs-sip-fallback.toml ]]; then
  cp -a /etc/netcore/tbs-sip-fallback.toml "/etc/netcore/tbs-sip-fallback.toml.bak.$(date +%Y%m%d-%H%M%S)"
fi

python3 - "$FALLBACK_DIR/config/tbs-sip-fallback.example.toml" /etc/netcore/tbs-sip-fallback.toml \
  "$NODE_ID" "$TBS_IP" "$SWITCH_IP" "$CENTRAL_USER" "$CENTRAL_PASSWORD" "$PBX_IP" "$PBX_FALLBACK_ID" "$NATIVE_USER" "$NATIVE_PASSWORD" "$PBX_AUTH_USER" "$PBX_PASSWORD" <<'PY'
from pathlib import Path
import sys

src, dst, node, tbs_ip, switch_ip, central_user, central_password, pbx_ip, pbx_id, native_user, native_password, pbx_auth_user, pbx_password = sys.argv[1:]
text = Path(src).read_text(encoding="utf-8")
replacements = {
    'node_id = "SRV-M-TBS-01"': f'node_id = "{node}"',
    'username = "netcore-tbs-native"': f'username = "{native_user}"',
    'password = "openlab-native"': f'password = "{native_password}"',
    'contact_host = "127.0.0.1"': f'contact_host = "{tbs_ip}"',
    'host = "10.0.20.33"': f'host = "{switch_ip}"',
    'username = "tbs-srv-m-tbs-01"': f'username = "{central_user}"',
    'password = "openlab-central"': f'password = "{central_password}"',
    'host = "10.0.1.160"': f'host = "{pbx_ip}"',
    'username = "netcore-tbs-01"': f'username = "{pbx_id}"',
    'auth_username = ""': f'auth_username = "{pbx_auth_user}"',
    'password = ""': f'password = "{pbx_password}"',
    'from_user = "netcore-tbs-01"': f'from_user = "{pbx_id}"',
    'contact_user = "netcore-tbs-01"': f'contact_user = "{pbx_id}"',
    'match = ["10.0.1.160"]': f'match = ["{pbx_ip}"]',
}
for old, new in replacements.items():
    if old not in text:
        raise SystemExit(f"template marker missing: {old}")
    text = text.replace(old, new, 1)
Path(dst).write_text(text, encoding="utf-8")
PY
chmod 0640 /etc/netcore/tbs-sip-fallback.toml

ensure_include() {
  local file="$1"
  local include="$2"
  touch "$file"
  if ! grep -Fqx "#include ${include}" "$file"; then
    printf '\n; NetCore-TETRA Phase 11c managed include\n#include %s\n' "$include" >>"$file"
  fi
}
ensure_include /etc/asterisk/pjsip.conf netcore-tbs-fallback-pjsip.conf
ensure_include /etc/asterisk/pjsip.conf netcore-active-registration.conf
ensure_include /etc/asterisk/extensions.conf netcore-tbs-fallback-extensions.conf
ensure_include /etc/asterisk/rtp.conf netcore-tbs-fallback-rtp.conf

/usr/local/bin/netcore-tbs-sip-fallback --config /etc/netcore/tbs-sip-fallback.toml --render

systemctl daemon-reload
systemctl enable --now asterisk.service
systemctl restart asterisk.service
systemctl enable --now netcore-tbs-sip-failover.service

cat <<EOF

Phase 11c ist installiert.

Normalbetrieb:
  lokale TBS -> lokaler Asterisk -> zentraler SIP-Switch
  direkte PBX-Registrierung: AUS

Bestätigter Ausfall:
  zentrale Registrierung wird entfernt
  direkte PBX-Registrierung wird aktiviert

Status:
  systemctl status netcore-tbs-sip-failover --no-pager --full
  /usr/local/bin/netcore-tbs-sip-fallback --status

Native TBS-Konfiguration:
  /etc/netcore/tbs-asterisk-local-snippet.toml
EOF
