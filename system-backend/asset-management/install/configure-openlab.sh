#!/usr/bin/env bash
set -euo pipefail
if [[ $# -lt 4 ]]; then
  echo "Usage: $0 <mqtt-host> <subscriber-core-host> <mobility-core-host> <task-workflow-host>" >&2
  exit 2
fi
CFG=/etc/netcore/asset-management.toml
python3 - "$CFG" "$1" "$2" "$3" "$4" <<'PY'
from pathlib import Path
import re,sys
path=Path(sys.argv[1]); text=path.read_text()
sections={
'mqtt':('host',sys.argv[2]),
'upstreams.subscriber_core':('base_url',f'http://{sys.argv[3]}:8100'),
'upstreams.mobility_core':('base_url',f'http://{sys.argv[4]}:8090'),
'upstreams.task_workflow':('base_url',f'http://{sys.argv[5]}:8280'),
}
for section,(key,value) in sections.items():
    pattern=rf'(\[{re.escape(section)}\][\s\S]*?\n{re.escape(key)}\s*=\s*)"[^"]*"'
    text,n=re.subn(pattern,rf'\1"{value}"',text,count=1)
    if n!=1: raise SystemExit(f'could not update [{section}] {key}')
path.write_text(text)
PY
systemctl restart netcore-asset-management.service
systemctl status netcore-asset-management.service --no-pager --full || true
