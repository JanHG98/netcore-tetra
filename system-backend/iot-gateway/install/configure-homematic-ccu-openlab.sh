#!/usr/bin/env bash
set -euo pipefail

CONFIG=${CONFIG:-/etc/netcore/iot-gateway.toml}
CCU_HOST=${1:-}
CCU_PORT=${2:-2010}

if [[ -z ${CCU_HOST} ]]; then
  echo "Verwendung: $0 <CCU-IP-ODER-HOSTNAME> [PORT]" >&2
  exit 1
fi
if [[ ! -f ${CONFIG} ]]; then
  echo "Konfiguration fehlt: ${CONFIG}" >&2
  exit 1
fi

python3 - "${CONFIG}" "${CCU_HOST}" "${CCU_PORT}" <<'PY'
from pathlib import Path
import json
import re
import sys
path=Path(sys.argv[1]); host=sys.argv[2]; port=int(sys.argv[3])
if not 1 <= port <= 65535: raise SystemExit("ungültiger Port")
text=path.read_text(encoding="utf-8")
match=re.search(r"(?ms)^\[homematic\]\n(.*?)(?=^\[|\Z)",text)
if not match: raise SystemExit("[homematic] fehlt; zuerst update.sh ausführen")
block=match.group(0)
def setv(block,key,value):
    pattern=rf"(?m)^{re.escape(key)}\s*=.*$"
    if not re.search(pattern,block): return block+f"{key} = {value}\n"
    return re.sub(pattern,f"{key} = {value}",block)
block=setv(block,"enabled","true")
block=setv(block,"mode",'"ccu_xml_rpc"')
block=setv(block,"ccu_host",json.dumps(host))
block=setv(block,"ccu_port",str(port))
block=setv(block,"allow_writes","false")
text=text[:match.start()]+block+text[match.end():]
path.write_text(text,encoding="utf-8")
PY

echo "Direktes CCU-Polling vorbereitet: ${CCU_HOST}:${CCU_PORT}"
echo "Schreibzugriffe bleiben gesperrt. Jetzt [[homematic_datapoints]] ergänzen und Dienst neu starten."
