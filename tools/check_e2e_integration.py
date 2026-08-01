#!/usr/bin/env python3
# NETCORE-KOMMENTAR – Was: Enthält die Logik oder Einstellungen für check e2e integration.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

from __future__ import annotations

import json
import os
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REQUIRED_FILES = [
    "tests/e2e/README.md",
    "tests/e2e/netcore_open_lab_e2e.py",
    "tests/e2e/netcore_e2e/context.py",
    "tests/e2e/netcore_e2e/http.py",
    "tests/e2e/netcore_e2e/inventory.py",
    "tests/e2e/netcore_e2e/mock_tbs.py",
    "tests/e2e/netcore_e2e/model.py",
    "tests/e2e/netcore_e2e/report.py",
    "tests/e2e/netcore_e2e/scenarios.py",
    "tests/e2e/netcore_e2e/wait.py",
    "tests/e2e/netcore_e2e/websocket.py",
    "tests/e2e/fixtures/open_lab_test_plan.toml",
    "tests/e2e/on_air_evidence.schema.json",
    "tests/e2e/on_air_template.json",
    "tests/e2e/validate_on_air_evidence.py",
    "tests/e2e/unit/test_e2e_support.py",
    "tests/e2e/unit/test_edge_fallback_reference.py",
    "deploy/open-lab/netcore-e2e.py",
    "Docs/OPEN_LAB_E2E_RUNBOOK.md",
    "Docs/SWMI_CORE_1_PACKAGE_Q_E2E_INTEGRATION.md",
    "Docs/SWMI_CORE_1_PACKAGE_Q_APPLY.md",
    ".github/workflows/swmi-core-e2e-integration.yml",
]
EXECUTABLE_FILES = [
    "tests/e2e/netcore_open_lab_e2e.py",
    "tests/e2e/validate_on_air_evidence.py",
    "deploy/open-lab/netcore-e2e.py",
    "deploy/open-lab/netcore-deploy.py",
    "tools/check_e2e_integration.py",
]
EXPECTED_SERVICES = {
    "node-gateway",
    "mobility-core",
    "subscriber-core",
    "group-core",
    "call-control",
    "media-switch",
    "recorder",
    "sds-router",
    "packet-core",
    "ip-gateway",
    "security-core",
    "kmf",
    "transit",
    "application-gateway",
    "media-library",
    "control-room",
    "observability",
    "iot-gateway",
}
EXPECTED_SCENARIOS = {
    "contracts",
    "node-gateway",
    "edge-fallback-contract",
    "subscriber-group",
    "call-media-recorder",
    "sds",
    "packet-data",
    "observability",
    "control-room-federation",
    "platform-services",
    "restart-restore",
    "fault-matrix",
    "edge-service-outages",
}


# Was: Diese Funktion führt den vorgesehenen Arbeitsschritt.
# Warum: Der Lebenszyklus des Dienstes bleibt so an einer zentralen Stelle steuerbar.
def run(command: list[str], errors: list[str]) -> None:
    env = os.environ.copy()
    env["PYTHONDONTWRITEBYTECODE"] = "1"
    result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, env=env)
    if result.returncode:
        detail = result.stderr.strip() or result.stdout.strip()
        errors.append(f"command failed ({' '.join(command)}): {detail}")


# Was: Startet das Programm, lädt die benötigten Einstellungen und übergibt an den eigentlichen Dienstablauf.
# Warum: Ein klarer Einstiegspunkt hält Startreihenfolge, Fehlerausgabe und geordnetes Beenden zusammen.
def main() -> int:
    errors: list[str] = []

    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for relative in REQUIRED_FILES:
        if not (ROOT / relative).is_file():
            errors.append(f"missing {relative}")

    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for relative in EXECUTABLE_FILES:
        path = ROOT / relative
        if path.is_file() and not path.stat().st_mode & 0o111:
            errors.append(f"not executable: {relative}")

    inventory_path = ROOT / "deploy/open-lab/inventory.example.toml"
    if inventory_path.is_file():
        with inventory_path.open("rb") as handle:
            inventory = tomllib.load(handle)
        names = {str(service.get("name")) for service in inventory.get("services", [])}
        if names != EXPECTED_SERVICES:
            errors.append(f"inventory services differ: missing={sorted(EXPECTED_SERVICES - names)} extra={sorted(names - EXPECTED_SERVICES)}")
        if inventory.get("contract_version") != "netcore.v1":
            errors.append("inventory contract_version must be netcore.v1")
        if inventory.get("mode") != "open_lab":
            errors.append("E2E package must remain explicit open_lab")

    plan_path = ROOT / "tests/e2e/fixtures/open_lab_test_plan.toml"
    if plan_path.is_file():
        with plan_path.open("rb") as handle:
            plan = tomllib.load(handle)
        declared: set[str] = set()
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for profile in plan.get("profiles", {}).values():
            declared.update(str(value) for value in profile.get("scenarios", []))
        if declared != EXPECTED_SCENARIOS:
            errors.append(f"test plan scenarios differ: missing={sorted(EXPECTED_SCENARIOS - declared)} extra={sorted(declared - EXPECTED_SCENARIOS)}")
        fault = plan.get("profiles", {}).get("fault", {})
        if not fault.get("destructive") or not fault.get("requires_ssh"):
            errors.append("fault profile must be marked destructive and requires_ssh")

    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for relative in ("tests/e2e/on_air_evidence.schema.json", "tests/e2e/on_air_template.json"):
        path = ROOT / relative
        if path.is_file():
            # Was: Führt einen fehleranfälligen Abschnitt mit geregelter Fehlerbehandlung aus.
            # Warum: Ein einzelner Fehler soll kontrolliert gemeldet oder aufgefangen werden, statt den gesamten Dienst unverständlich abzubrechen.
            try:
                json.loads(path.read_text(encoding="utf-8"))
            except Exception as error:
                errors.append(f"invalid JSON {relative}: {error}")

    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for path in (ROOT / "tests/e2e").rglob("*.py"):
        # Was: Führt einen fehleranfälligen Abschnitt mit geregelter Fehlerbehandlung aus.
        # Warum: Ein einzelner Fehler soll kontrolliert gemeldet oder aufgefangen werden, statt den gesamten Dienst unverständlich abzubrechen.
        try:
            compile(path.read_text(encoding="utf-8"), str(path), "exec")
        except SyntaxError as error:
            errors.append(f"Python syntax error {path.relative_to(ROOT)}: {error}")

    run([sys.executable, "-m", "unittest", "discover", "-s", "tests/e2e/unit", "-v"], errors)
    run([sys.executable, "tests/e2e/netcore_open_lab_e2e.py", "--validate-only", "--profile", "smoke"], errors)
    run([sys.executable, "tests/e2e/netcore_open_lab_e2e.py", "--validate-only", "--profile", "full"], errors)
    run([sys.executable, "tests/e2e/netcore_open_lab_e2e.py", "--validate-only", "--profile", "fault"], errors)
    run([sys.executable, "deploy/open-lab/netcore-deploy.py", "test", "--profile", "full", "--validate-only"], errors)
    run([sys.executable, "tests/e2e/validate_on_air_evidence.py"], errors)
    run([sys.executable, "deploy/open-lab/netcore-deploy.py", "validate"], errors)
    run([sys.executable, "deploy/open-lab/netcore-deploy.py", "render"], errors)

    pdfs = list(ROOT.rglob("*.pdf"))
    if pdfs:
        errors.append(f"repository package contains PDF files: {len(pdfs)}")
    caches = [path for path in ROOT.rglob("__pycache__") if path.is_dir()]
    pyc = list(ROOT.rglob("*.pyc"))
    if caches or pyc:
        errors.append(f"runtime Python caches present: directories={len(caches)} pyc={len(pyc)}")

    if errors:
        print("\n".join(errors), file=sys.stderr)
        return 1
    print(f"Cross-LXC E2E integration package check: OK (18 services, {len(EXPECTED_SCENARIOS)} scenarios)")
    return 0


# Was: Startet den Programmablauf nur dann, wenn diese Datei direkt ausgeführt wird.
# Warum: Beim Import als Modul sollen nur Funktionen bereitstehen und keine Nebenwirkungen automatisch starten.
if __name__ == "__main__":
    raise SystemExit(main())
