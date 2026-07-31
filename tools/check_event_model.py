#!/usr/bin/env python3
"""Static acceptance checks for MQTT Phase 2 / netcore-event-v1."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EVENT_TYPE = re.compile(r"^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$")


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def main() -> int:
    schema_path = ROOT / "system-backend/shared/contracts/schemas/netcore-event-v1.schema.json"
    example_path = ROOT / "system-backend/shared/contracts/examples/netcore-event-subscriber-route-changed.json"
    event_rs = ROOT / "system-backend/shared/contracts/src/event.rs"

    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    example = json.loads(example_path.read_text(encoding="utf-8"))
    require(schema["properties"]["schema"]["const"] == "netcore-event-v1", "schema const mismatch")
    require(example["schema"] == "netcore-event-v1", "example schema mismatch")
    require(bool(EVENT_TYPE.fullmatch(example["event_type"])), "example event_type is invalid")
    require(example["source"]["service"], "example source.service is empty")
    require(example["source"]["instance"], "example source.instance is empty")
    require(example["subject"]["type"] == "subscriber", "example subject mismatch")

    source = event_rs.read_text(encoding="utf-8")
    constants = re.findall(r'pub const [A-Z0-9_]+: &str = "([a-z0-9_.]+)";', source)
    catalog = [value for value in constants if "." in value]
    require(len(catalog) >= 30, "event catalog is unexpectedly small")
    require(len(catalog) == len(set(catalog)), "event catalog contains duplicate values")
    invalid = [value for value in catalog if not EVENT_TYPE.fullmatch(value)]
    require(not invalid, f"invalid event catalog entries: {invalid}")

    services = ["node-gateway", "mobility-core", "call-control", "sds-router"]
    for service in services:
        cargo = (ROOT / f"system-backend/{service}/Cargo.toml").read_text(encoding="utf-8")
        state = (ROOT / f"system-backend/{service}/src/state.rs").read_text(encoding="utf-8")
        http = (ROOT / f"system-backend/{service}/src/http.rs").read_text(encoding="utf-8")
        require("netcore-contracts" in cargo, f"{service}: contracts dependency missing")
        require("netcore-service-common" in cargo, f"{service}: service-common dependency missing")
        require("NetCoreEvent" in state, f"{service}: canonical event type not used")
        require("canonical" in state, f"{service}: legacy record lacks canonical field")
        require('/api/v1/events/netcore' in http, f"{service}: canonical endpoint missing")

    print("OK: netcore-event-v1 schema, catalog, examples and four producers are wired")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"ERROR: {error}", file=sys.stderr)
        raise SystemExit(1)
