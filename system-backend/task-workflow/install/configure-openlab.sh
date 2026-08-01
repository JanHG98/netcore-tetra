#!/usr/bin/env bash
set -euo pipefail
if [[ $# -lt 2 ]]; then
  echo "Aufruf: $0 <MQTT-IP> <SDS-ROUTER-IP> [DEFAULT-GSSI]" >&2
  exit 2
fi
MQTT_IP="$1"
SDS_IP="$2"
DEFAULT_GSSI="${3:-15201}"
CONFIG=/etc/netcore/task-workflow.toml
python3 - "$CONFIG" "$MQTT_IP" "$SDS_IP" "$DEFAULT_GSSI" <<'PY'
from pathlib import Path
import re, sys
path=Path(sys.argv[1]); text=path.read_text(encoding='utf-8')

def replace_in_section(text, section, key, value):
    pattern=rf'(?ms)(^\[{re.escape(section)}\]\s*.*?)(?=^\[|\Z)'
    m=re.search(pattern,text)
    if not m: raise SystemExit(f'Sektion [{section}] fehlt')
    block=m.group(1)
    replacement=f'{key} = {value}'
    if re.search(rf'(?m)^{re.escape(key)}\s*=.*$',block):
        block=re.sub(rf'(?m)^{re.escape(key)}\s*=.*$',replacement,block)
    else:
        block=block.rstrip()+"\n"+replacement+"\n"
    return text[:m.start(1)]+block+text[m.end(1):]

text=replace_in_section(text,'mqtt','host',f'"{sys.argv[2]}"')
text=replace_in_section(text,'sds_router','base_url',f'"http://{sys.argv[3]}:8150"')
text=replace_in_section(text,'sds_router','default_destination',str(int(sys.argv[4])))
path.write_text(text,encoding='utf-8')
PY
systemctl restart netcore-task-workflow.service
echo "Task Workflow: MQTT=${MQTT_IP}:1883 SDS=http://${SDS_IP}:8150 Default-GSSI=${DEFAULT_GSSI}"
