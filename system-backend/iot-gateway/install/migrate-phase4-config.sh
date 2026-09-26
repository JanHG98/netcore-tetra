#!/usr/bin/env bash
set -euo pipefail

CONFIG=${CONFIG:-/etc/netcore/iot-gateway.toml}
EXAMPLE=${EXAMPLE:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/config/iot-gateway.example.toml}

if [[ ! -f ${CONFIG} ]]; then
  exit 0
fi

python3 - "${CONFIG}" "${EXAMPLE}" <<'PY'
from pathlib import Path
import re
import sys

path = Path(sys.argv[1])
example_path = Path(sys.argv[2])
text = path.read_text(encoding="utf-8")
example = example_path.read_text(encoding="utf-8")

# Phase-3-Konfigurationen behalten ihre individuellen Hosts und Quellen. Es
# werden nur die neuen Phase-4-Blöcke beziehungsweise Storage-Schlüssel ergänzt.
if "\n[commands]\n" not in text:
    commands = re.search(r"\n\[commands\]\n.*?(?=\n\[|\n\[\[)", example, re.S)
    if not commands:
        raise SystemExit("[commands] fehlt in der Beispielkonfiguration")
    insert_at = text.find("\n[storage]\n")
    if insert_at < 0:
        text += commands.group(0) + "\n"
    else:
        text = text[:insert_at] + commands.group(0) + "\n" + text[insert_at:]

storage_defaults = {
    "command_ledger_file": '"command-ledger.json"',
    "command_audit_file": '"command-audit.ndjson"',
    "virtual_state_file": '"virtual-device-state.json"',
    "command_ledger_limit": "50000",
}
for key, value in storage_defaults.items():
    if re.search(rf"(?m)^\s*{re.escape(key)}\s*=", text):
        continue
    match = re.search(r"(?ms)^\[storage\]\n(.*?)(?=^\[|^\[\[|\Z)", text)
    if not match:
        raise SystemExit("[storage] fehlt in der bestehenden Konfiguration")
    position = match.end(1)
    text = text[:position] + f"{key} = {value}\n" + text[position:]

if "[[command_policies]]" not in text:
    policies = re.search(r"\n\[\[command_policies\]\].*?(?=\n\[\[sources\]\])", example, re.S)
    if not policies:
        raise SystemExit("command_policies fehlen in der Beispielkonfiguration")
    source_pos = text.find("\n[[sources]]")
    if source_pos < 0:
        text += policies.group(0) + "\n"
    else:
        text = text[:source_pos] + policies.group(0) + "\n" + text[source_pos:]

path.write_text(text, encoding="utf-8")
PY
