#!/usr/bin/env python3
from __future__ import annotations

import json
import os
import stat
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
BASE = ROOT / "system-backend/iot-gateway"
CONTRACTS = ROOT / "system-backend/shared/contracts"
REQUIRED = [
    "Cargo.toml",
    "README.md",
    "src/main.rs",
    "src/config.rs",
    "src/command.rs",
    "src/model.rs",
    "src/state.rs",
    "src/mqtt.rs",
    "src/poller.rs",
    "src/http.rs",
    "config/iot-gateway.example.toml",
    "install/install.sh",
    "install/update.sh",
    "install/migrate-phase5-config.sh",
    "src/home_assistant.rs",
    "src/homematic.rs",
    "docs/home-assistant.md",
    "docs/homematic-ip.md",
    "install/uninstall.sh",
    "systemd/netcore-iot-gateway.service",
    "docs/architecture.md",
    "docs/mqtt-contract.md",
    "docs/open-lab-mode.md",
]
CONTRACT_REQUIRED = [
    "src/command.rs",
    "COMMAND_MODEL_V1.md",
    "schemas/netcore-command-v1.schema.json",
    "schemas/netcore-command-ack-v1.schema.json",
    "examples/netcore-command-virtual-relay-set.json",
    "examples/netcore-command-ack-succeeded.json",
]


def fail(message: str) -> None:
    print(f"ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)


def main() -> int:
    for relative in REQUIRED:
        if not (BASE / relative).is_file():
            fail(f"missing {BASE.relative_to(ROOT) / relative}")
    for relative in CONTRACT_REQUIRED:
        if not (CONTRACTS / relative).is_file():
            fail(f"missing {CONTRACTS.relative_to(ROOT) / relative}")

    cargo = tomllib.loads((BASE / "Cargo.toml").read_text(encoding="utf-8"))
    if cargo.get("package", {}).get("name") != "netcore-iot-gateway":
        fail("wrong Cargo package name")

    config = tomllib.loads((BASE / "config/iot-gateway.example.toml").read_text(encoding="utf-8"))
    if config["security"]["mode"] != "open_lab":
        fail("IoT Gateway must remain explicit open_lab in Phase 5")
    if config["mqtt"]["execute_commands"] is not False:
        fail("deprecated mqtt.execute_commands must stay false")
    commands = config.get("commands", {})
    if commands.get("enabled") is not True:
        fail("Phase 5 command processing must be enabled")
    if commands.get("mode") != "open_lab_sandbox":
        fail("Phase 5 must use the open_lab_sandbox executor")
    if commands.get("default_deny") is not True:
        fail("Phase 5 must be default deny")
    if commands.get("allow_retained") is not False:
        fail("retained commands must be rejected by default")
    if config["server"]["bind"].split(":")[-1] != "8240":
        fail("IoT Gateway management port must be 8240")

    source_ids = {item["id"] for item in config.get("sources", [])}
    expected_sources = {"node-gateway", "mobility-core", "call-control", "sds-router"}
    if source_ids != expected_sources:
        fail(f"event sources differ: {sorted(source_ids)}")

    policies = config.get("command_policies", [])
    expected_commands = {
        "virtual.relay.set",
        "virtual.light.set",
        "virtual.button.press",
    }
    policy_commands = {
        command_type
        for policy in policies
        if policy.get("effect") == "allow" and policy.get("enabled") is True
        for command_type in policy.get("command_types", [])
    }
    if policy_commands != expected_commands:
        fail(f"enabled sandbox allow policies differ: {sorted(policy_commands)}")
    enabled_policies = [policy for policy in policies if policy.get("enabled") is True]
    if any(prefix != "lab-" for p in enabled_policies for prefix in p.get("target_prefixes", [])):
        fail("enabled sandbox policies must stay below the lab- target prefix")
    if config.get("home_assistant", {}).get("discovery_enabled") is not True:
        fail("Home Assistant discovery must be enabled")
    if config.get("home_assistant", {}).get("allow_command_egress") is not False:
        fail("Home Assistant command egress must default to false")
    if config.get("homematic", {}).get("enabled") is not False:
        fail("direct Homematic XML-RPC must default to disabled")
    if config.get("homematic", {}).get("allow_writes") is not False:
        fail("Homematic writes must default to false")

    workspace = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
    if "system-backend/iot-gateway" not in workspace["workspace"]["members"]:
        fail("IoT Gateway is missing from workspace members")

    services = tomllib.loads((ROOT / "system-backend/services.toml").read_text(encoding="utf-8"))
    service = next((item for item in services["services"] if item.get("name") == "iot-gateway"), None)
    if not service or service.get("management_port") != 8240:
        fail("services.toml does not contain iot-gateway on port 8240")

    source = "\n".join(
        path.read_text(encoding="utf-8", errors="replace")
        for path in sorted((BASE / "src").glob("*.rs"))
    )
    markers = [
        "netcore-event-v1",
        "netcore-command-v1",
        "netcore-command-ack-v1",
        "/commands/#",
        "/acks/",
        "open_lab_sandbox",
        "default_deny",
        "allow_retained",
        "duplicate_command_id",
        "execute_command",
        "discovery_messages",
        "ingest_home_assistant_state",
        "ccu_xml_rpc",
        "persist_command_ledger",
        "persist_virtual_devices",
        "/api/v1/policies",
        "/api/v1/virtual-devices",
        "/health/ready",
        "INDEX_HTML",
    ]
    docs = (BASE / "README.md").read_text(encoding="utf-8")
    for marker in markers:
        if marker not in source and marker not in docs:
            fail(f"implementation marker missing: {marker}")

    contract_source = (CONTRACTS / "src/command.rs").read_text(encoding="utf-8")
    for marker in [
        "NETCORE_COMMAND_SCHEMA_V1",
        "NETCORE_COMMAND_ACK_SCHEMA_V1",
        "CommandAckStatus",
        "CommandPolicyDecision",
        "NetCoreCommand",
        "CommandAck",
    ]:
        if marker not in contract_source:
            fail(f"command contract marker missing: {marker}")

    for script in (BASE / "install").glob("*.sh"):
        if not script.stat().st_mode & stat.S_IXUSR:
            fail(f"installer is not executable: {script.relative_to(ROOT)}")

    for path in ROOT.rglob("*.json"):
        if "target" in path.parts or ".git" in path.parts:
            continue
        try:
            json.loads(path.read_text(encoding="utf-8"))
        except UnicodeDecodeError:
            continue
        except json.JSONDecodeError as error:
            fail(f"invalid JSON {path.relative_to(ROOT)}: {error}")

    print("IoT Gateway Phase 5 static package check: OK")
    print("- OPEN LAB without login/tokens/TLS")
    print("- netcore-command-v1 and netcore-command-ack-v1")
    print("- persistent command ledger, audit and duplicate suppression")
    print("- default-deny policy engine; retained commands rejected")
    print("- virtual relay/light/button OPEN-LAB sandbox")
    print("- Home Assistant MQTT Discovery and state ingress")
    print("- optional Homematic CCU XML-RPC; writes disabled by default")
    return 0


if __name__ == "__main__":
    os.environ.setdefault("PYTHONDONTWRITEBYTECODE", "1")
    raise SystemExit(main())
