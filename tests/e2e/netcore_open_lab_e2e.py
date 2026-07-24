#!/usr/bin/env python3
from __future__ import annotations

import argparse
import sys
import uuid
from datetime import datetime, timezone
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
if str(HERE) not in sys.path:
    sys.path.insert(0, str(HERE))

from netcore_e2e.context import E2EContext, now_iso  # noqa: E402
from netcore_e2e.http import HttpClient  # noqa: E402
from netcore_e2e.inventory import load_inventory, validate_inventory  # noqa: E402
from netcore_e2e.mock_tbs import MockTbs  # noqa: E402
from netcore_e2e.model import CheckResult, RunReport  # noqa: E402
from netcore_e2e.report import write_json, write_junit  # noqa: E402
from netcore_e2e.scenarios import DEFAULT_FULL, DEFAULT_SMOKE, SCENARIOS  # noqa: E402

DEFAULT_INVENTORY = ROOT / "deploy/open-lab/inventory.example.toml"
DEFAULT_ARTIFACTS = ROOT / "tests/e2e/artifacts"


def parse_scenarios(args: argparse.Namespace) -> list[str]:
    if args.scenario:
        values: list[str] = []
        for item in args.scenario:
            values.extend(part.strip() for part in item.split(",") if part.strip())
        return values
    if args.profile == "smoke":
        return list(DEFAULT_SMOKE)
    if args.profile == "fault":
        return ["contracts", "node-gateway", "subscriber-group", "restart-restore", "fault-matrix"]
    return list(DEFAULT_FULL)


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Run NetCore-Tetra cross-LXC Open-Lab integration scenarios using only the Python standard library."
    )
    parser.add_argument("--inventory", type=Path, default=DEFAULT_INVENTORY)
    parser.add_argument("--profile", choices=("smoke", "full", "fault"), default="smoke")
    parser.add_argument("--scenario", action="append", help="scenario name or comma-separated names; overrides --profile")
    parser.add_argument("--list-scenarios", action="store_true")
    parser.add_argument("--validate-only", action="store_true", help="validate inventory and selected scenario names without network access")
    parser.add_argument("--allow-mutations", action="store_true", help="allow fixture creation and application traffic")
    parser.add_argument("--allow-restarts", action="store_true", help="allow SSH/systemd fault and restart tests")
    parser.add_argument("--keep-fixtures", action="store_true")
    parser.add_argument("--no-mock-tbs", action="store_true")
    parser.add_argument("--strict-ready", action="store_true", help="treat HTTP 503 readiness as failure")
    parser.add_argument("--timeout", type=float, default=25.0)
    parser.add_argument("--run-id", default=None)
    parser.add_argument("--node-id", default=None)
    parser.add_argument("--artifacts-dir", type=Path, default=DEFAULT_ARTIFACTS)
    args = parser.parse_args()

    if args.list_scenarios:
        for name in SCENARIOS:
            print(name)
        return 0

    inventory = load_inventory(args.inventory)
    errors = validate_inventory(inventory)
    selected = parse_scenarios(args)
    unknown = [name for name in selected if name not in SCENARIOS]
    errors.extend(f"unknown scenario: {name}" for name in unknown)
    if errors:
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        return 2
    if args.validate_only:
        print(
            f"OK: inventory={args.inventory} services={len(inventory.services)} "
            f"contract={inventory.contract_version} mode={inventory.mode} scenarios={','.join(selected)}"
        )
        return 0

    run_id = args.run_id or f"e2e-{datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%SZ')}-{uuid.uuid4().hex[:8]}"
    artifacts = args.artifacts_dir / run_id
    artifacts.mkdir(parents=True, exist_ok=True)
    report = RunReport(run_id=run_id, started_at=now_iso(), mode=inventory.mode, inventory=str(args.inventory))
    report.metadata.update(
        {
            "profile": args.profile,
            "scenarios": selected,
            "allow_mutations": args.allow_mutations,
            "allow_restarts": args.allow_restarts,
            "strict_ready": args.strict_ready,
            "contract_version": inventory.contract_version,
        }
    )
    context = E2EContext(
        inventory=inventory,
        report=report,
        artifacts_dir=artifacts,
        timeout=args.timeout,
        strict_ready=args.strict_ready,
        allow_mutations=args.allow_mutations,
        allow_restarts=args.allow_restarts,
        keep_fixtures=args.keep_fixtures,
        client=HttpClient(timeout=min(max(args.timeout / 3.0, 3.0), 15.0)),
    )

    needs_mock = any(
        name not in {
            "contracts",
            "control-room-federation",
            "platform-services",
            "observability",
            "restart-restore",
        }
        for name in selected
    )
    if needs_mock and not args.no_mock_tbs:
        node_gateway = inventory.by_name.get("node-gateway")
        if node_gateway is None:
            report.add(CheckResult(name="start mock TBS", status="failed", duration_ms=0.0, detail="node-gateway missing"))
        else:
            node_id = args.node_id or f"tbs-e2e-{uuid.uuid4().hex[:8]}"
            ws_url = f"ws://{node_gateway.host}:{node_gateway.port}/ws/node"
            try:
                context.mock_tbs = MockTbs(ws_url, node_id=node_id)
                context.mock_tbs.start()
                report.metadata["mock_tbs_node_id"] = node_id
                print(f"Mock TBS connected: {node_id} -> {ws_url}")
            except BaseException as error:
                report.add(CheckResult(name="start mock TBS", status="failed", duration_ms=0.0, detail=f"{type(error).__name__}: {error}"))
                print(f"FAIL  [bootstrap] mock TBS: {error}")

    try:
        for name in selected:
            print(f"\n=== Scenario: {name} ===")
            SCENARIOS[name](context)
    finally:
        context.cleanup()
        if context.mock_tbs is not None:
            context.mock_tbs.stop()
            report.metadata["mock_tbs_errors"] = list(context.mock_tbs.errors)
            report.metadata["mock_tbs_commands"] = len(context.mock_tbs.received_commands)
            report.metadata["mock_tbs_downlink_media_frames"] = context.mock_tbs.downlink_media_frames
        report.finished_at = now_iso()
        write_json(report, artifacts / "report.json")
        write_junit(report, artifacts / "junit.xml")
        (artifacts / "summary.txt").write_text(
            f"run_id={run_id}\npassed={report.passed}\nfailed={report.failures}\nskipped={report.skipped}\n",
            encoding="utf-8",
        )

    print(
        f"\nE2E result: passed={report.passed} failed={report.failures} skipped={report.skipped} "
        f"artifacts={artifacts}"
    )
    return 1 if report.failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
