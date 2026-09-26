#!/usr/bin/env bash
set -euo pipefail

CONFIG=${CONFIG:-/etc/netcore/iot-gateway.toml}
EXAMPLE=${EXAMPLE:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/config/iot-gateway.example.toml}

[[ -f ${CONFIG} ]] || exit 0

python3 - "${CONFIG}" "${EXAMPLE}" <<'PY'
from pathlib import Path
import re
import sys

path = Path(sys.argv[1])
example_path = Path(sys.argv[2])
text = path.read_text(encoding="utf-8")
example = example_path.read_text(encoding="utf-8")

def section(source: str, name: str) -> str:
    match = re.search(rf"(?ms)^\[{re.escape(name)}\]\n.*?(?=^\[|\Z)", source)
    if not match:
        raise SystemExit(f"[{name}] fehlt in der Beispielkonfiguration")
    return match.group(0).rstrip() + "\n\n"

def insert_before(text: str, marker: str, block: str) -> str:
    position = text.find(marker)
    if position < 0:
        return text.rstrip() + "\n\n" + block
    return text[:position] + block + text[position:]

# Bestehende Hosts, Ports, Policies und Quellen bleiben unangetastet.
if not re.search(r"(?m)^\[home_assistant\]\s*$", text):
    text = insert_before(text, "[commands]\n", section(example, "home_assistant"))
if not re.search(r"(?m)^\[homematic\]\s*$", text):
    text = insert_before(text, "[commands]\n", section(example, "homematic"))
if not re.search(r"(?m)^\[commands\]\s*$", text):
    text = insert_before(text, "[storage]\n", section(example, "commands"))

storage_defaults = {
    "command_ledger_file": '"command-ledger.json"',
    "command_audit_file": '"command-audit.ndjson"',
    "virtual_state_file": '"virtual-device-state.json"',
    "external_state_file": '"external-entity-state.json"',
    "homematic_state_file": '"homematic-datapoint-state.json"',
    "command_ledger_limit": "50000",
}
for key, value in storage_defaults.items():
    if re.search(rf"(?m)^\s*{re.escape(key)}\s*=", text):
        continue
    match = re.search(r"(?ms)^\[storage\]\n(.*?)(?=^\[|^\[\[|\Z)", text)
    if not match:
        raise SystemExit("[storage] fehlt in der bestehenden Konfiguration")
    position = match.end(1)
    insertion = f"{key} = {value}\n"
    text = text[:position] + insertion + text[position:]

# Nur fehlende Standard-Policies ergänzen; individuelle Regeln bleiben erhalten.
def policy_blocks(source: str):
    return re.findall(r"(?ms)^\[\[command_policies\]\]\n.*?(?=^\[\[|^\[|\Z)", source)

existing_ids = set(re.findall(r'(?ms)^\[\[command_policies\]\].*?^id\s*=\s*"([^"]+)"', text))
blocks = []
for block in policy_blocks(example):
    match = re.search(r'(?m)^id\s*=\s*"([^"]+)"', block)
    if match and match.group(1) not in existing_ids:
        blocks.append(block.rstrip() + "\n\n")
if blocks:
    source_pos = text.find("[[sources]]")
    payload = "".join(blocks)
    if source_pos < 0:
        text = text.rstrip() + "\n\n" + payload
    else:
        text = text[:source_pos] + payload + text[source_pos:]

path.write_text(text, encoding="utf-8")
PY

python3 - "${CONFIG}" <<'PY'
from pathlib import Path
import sys
import tomllib
with Path(sys.argv[1]).open("rb") as handle:
    tomllib.load(handle)
print(f"Phase-5-Konfiguration geprüft: {sys.argv[1]}")
PY
