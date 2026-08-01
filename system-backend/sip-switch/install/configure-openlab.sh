#!/usr/bin/env bash
set -euo pipefail
if [[ $# -lt 3 ]]; then
  echo "Usage: $0 <PBX-IP/HOST> <MOBILITY-CORE-IP/HOST> <MQTT-BROKER-IP/HOST> [pbx-mode]" >&2
  exit 2
fi
PBX_HOST="$1"
MOBILITY_HOST="$2"
MQTT_HOST="$3"
PBX_MODE="${4:-ip_trunk}"
CONFIG=/etc/netcore/sip-switch.toml
python3 - "$CONFIG" "$PBX_HOST" "$MOBILITY_HOST" "$MQTT_HOST" "$PBX_MODE" <<'PY'
from pathlib import Path
import re, sys
path = Path(sys.argv[1])
text = path.read_text(encoding="utf-8")

def set_in_section(text, section, key, value):
    pattern = rf"(?ms)(^\[{re.escape(section)}\]\s*.*?)(?=^\[|\Z)"
    match = re.search(pattern, text)
    if not match:
        raise SystemExit(f"section [{section}] not found")
    block = match.group(1)
    line = f'{key} = "{value}"'
    if re.search(rf"(?m)^{re.escape(key)}\s*=.*$", block):
        block = re.sub(rf"(?m)^{re.escape(key)}\s*=.*$", line, block)
    else:
        block = block.rstrip() + "\n" + line + "\n"
    return text[:match.start(1)] + block + text[match.end(1):]

text = set_in_section(text, "pbx", "host", sys.argv[2])
text = set_in_section(text, "pbx", "mode", sys.argv[5])
text = set_in_section(text, "mobility_core", "base_url", f"http://{sys.argv[3]}:8090")
text = set_in_section(text, "mqtt", "host", sys.argv[4])
path.write_text(text, encoding="utf-8")
PY
/usr/local/bin/netcore-sip-switch --config "$CONFIG" --render-asterisk
systemctl restart asterisk.service
systemctl restart netcore-sip-switch.service
