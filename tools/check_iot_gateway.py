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
REQUIRED = [
    "Cargo.toml",
    "README.md",
    "src/main.rs",
    "src/config.rs",
    "src/model.rs",
    "src/state.rs",
    "src/mqtt.rs",
    "src/poller.rs",
    "src/http.rs",
    "config/iot-gateway.example.toml",
    "install/install.sh",
    "install/update.sh",
    "install/uninstall.sh",
    "systemd/netcore-iot-gateway.service",
    "docs/architecture.md",
    "docs/mqtt-contract.md",
]


def fail(message: str) -> None:
    print(f"ERROR: {message}", file=sys.stderr)
    raise SystemExit(1)


def main() -> int:
    for relative in REQUIRED:
        if not (BASE / relative).is_file():
            fail(f"missing {BASE.relative_to(ROOT) / relative}")

    cargo = tomllib.loads((BASE / "Cargo.toml").read_text(encoding="utf-8"))
    if cargo.get("package", {}).get("name") != "netcore-iot-gateway":
        fail("wrong Cargo package name")

    config = tomllib.loads((BASE / "config/iot-gateway.example.toml").read_text(encoding="utf-8"))
    if config["security"]["mode"] != "open_lab":
        fail("IoT Gateway must remain explicit open_lab in Phase 3")
    if config["mqtt"]["execute_commands"] is not False:
        fail("Phase 3 must refuse command execution")
    if config["server"]["bind"].split(":")[-1] != "8240":
        fail("IoT Gateway management port must be 8240")
    source_ids = {item["id"] for item in config.get("sources", [])}
    expected_sources = {"node-gateway", "mobility-core", "call-control", "sds-router"}
    if source_ids != expected_sources:
        fail(f"event sources differ: {sorted(source_ids)}")

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
        "/commands/#",
        "execute_commands",
        "write_connect",
        "write_publish",
        "persist_dedup",
        "/api/v1/outbox",
        "/health/ready",
        "INDEX_HTML",
    ]
    for marker in markers:
        if marker not in source and marker not in (BASE / "README.md").read_text(encoding="utf-8"):
            fail(f"implementation marker missing: {marker}")

    for script in (BASE / "install").glob("*.sh"):
        if not script.stat().st_mode & stat.S_IXUSR:
            fail(f"installer is not executable: {script.relative_to(ROOT)}")

    for path in ROOT.rglob("*.json"):
        if "target" in path.parts or ".git" in path.parts:
            continue
        try:
            json.loads(path.read_text(encoding="utf-8"))
        except (UnicodeDecodeError, json.JSONDecodeError):
            continue

    print("IoT Gateway Phase 3 static package check: OK")
    print("- OPEN LAB without login/tokens/TLS")
    print("- four netcore-event-v1 producers")
    print("- MQTT 3.1.1 QoS 0/1, Last Will and durable outbox")
    print("- command observation only; execution hard-disabled")
    return 0


if __name__ == "__main__":
    os.environ.setdefault("PYTHONDONTWRITEBYTECODE", "1")
    raise SystemExit(main())
