#!/usr/bin/env bash
set -euo pipefail
BROKER_HOST="${1:?Aufruf: $0 IP-DES-MQTT-BROKERS}"
CONFIG="/etc/netcore/rf-monitor.toml"
[[ -f "${CONFIG}" ]] || { echo "Fehlt: ${CONFIG}" >&2; exit 1; }
python3 - "${CONFIG}" "${BROKER_HOST}" <<'PY'
from pathlib import Path
import re, sys
path=Path(sys.argv[1]); host=sys.argv[2]
text=path.read_text()
start=text.index('[mqtt]')
end=text.find('\n[',start+1)
if end<0: end=len(text)
block=text[start:end]
block=re.sub(r'(?m)^host\s*=\s*"[^"]*"',f'host = "{host}"',block)
path.write_text(text[:start]+block+text[end:])
PY
systemctl restart netcore-rf-monitor.service
systemctl --no-pager --full status netcore-rf-monitor.service || true
