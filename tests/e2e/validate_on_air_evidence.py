#!/usr/bin/env python3
from __future__ import annotations

import argparse
import hashlib
import json
import sys
from datetime import datetime
from pathlib import Path
from typing import Any

REQUIRED_TEST_IDS = {
    "AIR-REG-001",
    "AIR-GRP-001",
    "AIR-IND-001",
    "AIR-SDS-001",
    "AIR-PD-001",
    "AIR-RESTORE-001",
    "AIR-REC-001",
}
VALID_RESULTS = {"passed", "failed", "blocked", "not_run"}


def parse_time(value: Any, label: str, errors: list[str]) -> None:
    if not isinstance(value, str):
        errors.append(f"{label} must be a string")
        return
    try:
        datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError:
        errors.append(f"{label} is not an ISO-8601 timestamp: {value!r}")


def validate(path: Path, *, require_complete: bool, verify_artifacts: bool) -> list[str]:
    errors: list[str] = []
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except Exception as error:
        return [f"invalid JSON: {error}"]
    if value.get("schema_version") != 1:
        errors.append("schema_version must be 1")
    for field in ("run_id", "started_at", "finished_at", "site", "network", "devices", "tests"):
        if field not in value:
            errors.append(f"missing top-level field {field}")
    parse_time(value.get("started_at"), "started_at", errors)
    parse_time(value.get("finished_at"), "finished_at", errors)
    devices = value.get("devices")
    if not isinstance(devices, list) or not devices:
        errors.append("devices must be a non-empty list")
        devices = []
    known_issis: set[int] = set()
    vendors: set[str] = set()
    for index, device in enumerate(devices):
        if not isinstance(device, dict):
            errors.append(f"devices[{index}] must be an object")
            continue
        for field in ("vendor", "model", "firmware", "issi"):
            if field not in device or device[field] in {"", None}:
                errors.append(f"devices[{index}] missing {field}")
        issi = device.get("issi")
        if isinstance(issi, int) and 1 <= issi <= 16_777_215:
            if issi in known_issis:
                errors.append(f"duplicate device ISSI {issi}")
            known_issis.add(issi)
        else:
            errors.append(f"devices[{index}].issi outside 24-bit SSI range")
        if isinstance(device.get("vendor"), str):
            vendors.add(device["vendor"].strip().lower())
    if require_complete and len(vendors) < 2:
        errors.append("complete on-air evidence must include at least two device vendors")

    tests = value.get("tests")
    if not isinstance(tests, list) or not tests:
        errors.append("tests must be a non-empty list")
        tests = []
    ids: set[str] = set()
    for index, test in enumerate(tests):
        if not isinstance(test, dict):
            errors.append(f"tests[{index}] must be an object")
            continue
        test_id = test.get("id")
        if not isinstance(test_id, str) or not test_id:
            errors.append(f"tests[{index}] missing id")
            continue
        if test_id in ids:
            errors.append(f"duplicate test id {test_id}")
        ids.add(test_id)
        result = test.get("result")
        if result not in VALID_RESULTS:
            errors.append(f"{test_id}: invalid result {result!r}")
        if require_complete and result != "passed":
            errors.append(f"{test_id}: complete evidence requires result=passed, got {result!r}")
        parse_time(test.get("observed_at"), f"{test_id}.observed_at", errors)
        for issi in test.get("devices", []):
            if issi not in known_issis:
                errors.append(f"{test_id}: references unknown device ISSI {issi}")
        artifacts = test.get("artifacts", [])
        if not isinstance(artifacts, list):
            errors.append(f"{test_id}.artifacts must be a list")
            continue
        for artifact in artifacts:
            if not isinstance(artifact, dict):
                errors.append(f"{test_id}: artifact must be an object")
                continue
            digest = artifact.get("sha256", "")
            if not isinstance(digest, str) or len(digest) != 64 or any(char not in "0123456789abcdef" for char in digest):
                errors.append(f"{test_id}: artifact has invalid sha256")
                continue
            if verify_artifacts:
                artifact_path = path.parent / str(artifact.get("path", ""))
                if not artifact_path.is_file():
                    errors.append(f"{test_id}: artifact does not exist: {artifact_path}")
                elif hashlib.sha256(artifact_path.read_bytes()).hexdigest() != digest:
                    errors.append(f"{test_id}: artifact hash mismatch: {artifact_path}")
    missing = REQUIRED_TEST_IDS - ids
    if missing:
        errors.append(f"missing required tests: {', '.join(sorted(missing))}")
    return errors


def main() -> int:
    parser = argparse.ArgumentParser(description="Validate NetCore-Tetra manual on-air evidence JSON")
    parser.add_argument("path", type=Path, nargs="?", default=Path(__file__).with_name("on_air_template.json"))
    parser.add_argument("--require-complete", action="store_true")
    parser.add_argument("--verify-artifacts", action="store_true")
    args = parser.parse_args()
    errors = validate(args.path, require_complete=args.require_complete, verify_artifacts=args.verify_artifacts)
    if errors:
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        return 1
    print(f"OK: on-air evidence {args.path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
