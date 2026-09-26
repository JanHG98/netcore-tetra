#!/usr/bin/env bash
set -euo pipefail
CONFIG="/etc/netcore/alarm-workflow.toml"
MQTT_HOST="${1:?MQTT broker IP required}"
SDS_ROUTER_HOST="${2:?SDS Router IP required}"

python3 - "${CONFIG}" "${MQTT_HOST}" "${SDS_ROUTER_HOST}" <<'PY'
from pathlib import Path
import re, sys
path=Path(sys.argv[1]); mqtt=sys.argv[2]; sds=sys.argv[3]
text=path.read_text(encoding='utf-8')
text=re.sub(r'(?ms)(\[mqtt\].*?^host\s*=\s*)"[^"]+"', rf'\1"{mqtt}"', text, count=1)
text=re.sub(r'(?ms)(\[sds_router\].*?^base_url\s*=\s*)"[^"]+"', rf'\1"http://{sds}:8150"', text, count=1)
path.write_text(text, encoding='utf-8')
PY

systemctl restart netcore-alarm-workflow.service
systemctl --no-pager --full status netcore-alarm-workflow.service || true
