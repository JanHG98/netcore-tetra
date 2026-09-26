#!/usr/bin/env bash
set -euo pipefail

CONFIG=${CONFIG:-/etc/netcore/iot-gateway.toml}

usage() {
  cat <<'EOF'
Verwendung:
  configure-openlab.sh NODE_GATEWAY MOBILITY_CORE CALL_CONTROL SDS_ROUTER [MQTT_BROKER]

Die Werte dürfen IPv4-Adressen oder DNS-Namen sein. MQTT_BROKER ist optional
und bleibt ohne Angabe unverändert (bei lokaler Installation normalerweise 127.0.0.1).

Beispiel:
  ./install/configure-openlab.sh 10.0.1.150 10.0.1.151 10.0.1.152 10.0.1.153
EOF
}

if [[ ${EUID} -ne 0 ]]; then
  echo "Bitte als root ausführen." >&2
  exit 1
fi
if [[ $# -lt 4 || $# -gt 5 ]]; then
  usage >&2
  exit 2
fi
if [[ ! -f ${CONFIG} ]]; then
  echo "Konfigurationsdatei fehlt: ${CONFIG}" >&2
  exit 1
fi

NODE_GATEWAY=$1
MOBILITY_CORE=$2
CALL_CONTROL=$3
SDS_ROUTER=$4
MQTT_BROKER=${5:-}

python3 - "${CONFIG}" "${NODE_GATEWAY}" "${MOBILITY_CORE}" "${CALL_CONTROL}" "${SDS_ROUTER}" "${MQTT_BROKER}" <<'PY'
from pathlib import Path
import re
import sys

path = Path(sys.argv[1])
hosts = {
    "node-gateway": (sys.argv[2], 8080),
    "mobility-core": (sys.argv[3], 8090),
    "call-control": (sys.argv[4], 8120),
    "sds-router": (sys.argv[5], 8150),
}
mqtt_host = sys.argv[6]
text = path.read_text(encoding="utf-8")

for source_id, (host, port) in hosts.items():
    pattern = re.compile(
        rf'(\[\[sources\]\]\s*\n(?:[^\n]*\n)*?id\s*=\s*"{re.escape(source_id)}"\s*\n(?:[^\n]*\n)*?url\s*=\s*)"[^"]*"',
        re.MULTILINE,
    )
    replacement = rf'\1"http://{host}:{port}/api/v1/events/netcore"'
    text, count = pattern.subn(replacement, text, count=1)
    if count != 1:
        raise SystemExit(f"Quellabschnitt nicht eindeutig gefunden: {source_id}")

if mqtt_host:
    mqtt_pattern = re.compile(
        r'(\[mqtt\]\s*\n(?:[^\n]*\n)*?host\s*=\s*)"[^"]*"',
        re.MULTILINE,
    )
    text, count = mqtt_pattern.subn(rf'\1"{mqtt_host}"', text, count=1)
    if count != 1:
        raise SystemExit("mqtt.host wurde nicht eindeutig gefunden")

path.write_text(text, encoding="utf-8")
PY

systemctl restart netcore-iot-gateway.service
systemctl --no-pager --full status netcore-iot-gateway.service

echo
echo "Event-Quellen wurden gesetzt. Aktuelle Werte:"
grep -E '^(host|id|url)[[:space:]]*=' "${CONFIG}"
