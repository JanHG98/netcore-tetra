#!/usr/bin/env python3
from pathlib import Path
import json
import re
import sys
import tomllib

ROOT = Path(__file__).resolve().parents[1]
errors = []

def need(path: str, tokens=()):
    p = ROOT / path
    if not p.is_file():
        errors.append(f"missing file: {path}")
        return ""
    text = p.read_text(encoding="utf-8")
    for token in tokens:
        if token not in text:
            errors.append(f"{path}: missing token {token!r}")
    return text

config_path = ROOT / "system-backend/iot-gateway/config/iot-gateway.example.toml"
try:
    with config_path.open("rb") as handle:
        config = tomllib.load(handle)
except Exception as exc:
    errors.append(f"example TOML invalid: {exc}")
    config = {}

if config:
    if config.get("security", {}).get("mode") != "open_lab":
        errors.append("security.mode is not open_lab")
    if not config.get("home_assistant", {}).get("discovery_enabled"):
        errors.append("Home Assistant discovery is not enabled")
    if config.get("home_assistant", {}).get("allow_command_egress"):
        errors.append("Home Assistant command egress must default to false")
    if config.get("homematic", {}).get("enabled"):
        errors.append("direct Homematic adapter must default to disabled")
    if config.get("homematic", {}).get("allow_writes"):
        errors.append("Homematic writes must default to false")
    policies = {item.get("id"): item for item in config.get("command_policies", [])}
    for policy_id in (
        "allow-openlab-virtual-relays",
        "allow-openlab-virtual-lights",
        "allow-openlab-virtual-buttons",
        "allow-openlab-homeassistant-lab-bridge",
        "allow-openlab-homematic-lab-writes",
    ):
        if policy_id not in policies:
            errors.append(f"missing policy: {policy_id}")
    for policy_id in (
        "allow-openlab-homeassistant-lab-bridge",
        "allow-openlab-homematic-lab-writes",
    ):
        if policies.get(policy_id, {}).get("enabled"):
            errors.append(f"real integration policy must default disabled: {policy_id}")

need("system-backend/iot-gateway/src/home_assistant.rs", (
    "home_assistant_discovery",
    "virtual_relay",
    "virtual_light",
    "virtual_button",
))
need("system-backend/iot-gateway/src/homematic.rs", (
    "getValue",
    "setValue",
    "allow_writes",
    "ccu_xml_rpc",
))
need("system-backend/iot-gateway/src/state.rs", (
    "ingest_home_assistant_state",
    "process_home_assistant_command",
    "record_homematic_success",
))
need("system-backend/iot-gateway/src/mqtt.rs", (
    "home_assistant.status_topic",
    "home_assistant_state_ingress_topic",
    "home_assistant_command_prefix",
))
need("system-backend/iot-gateway/install/migrate-phase5-config.sh")
need("INSTALLATION-MQTT-PHASE5-HOME-ASSISTANT-HOMEMATIC.md")
need("Docs/MQTT_PHASE5_HOME_ASSISTANT_HOMEMATIC_OPENLAB.md")

for pdf in ROOT.rglob("*.pdf"):
    errors.append(f"PDF must not be included: {pdf.relative_to(ROOT)}")
workflow = ROOT / ".github" / "workflows"
if workflow.exists():
    errors.append(".github/workflows must not be included")

for script in (ROOT / "system-backend/iot-gateway/install").glob("*.sh"):
    if script.stat().st_mode & 0o111 == 0:
        errors.append(f"script is not executable: {script.relative_to(ROOT)}")

if errors:
    print("Phase-5 check FAILED")
    for error in errors:
        print(f"- {error}")
    sys.exit(1)
print("OK: Phase 5 Home Assistant/Homematic OPEN-LAB package is wired")
