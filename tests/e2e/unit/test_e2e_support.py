# NETCORE-KOMMENTAR – Was: Enthält automatische Prüfungen für test e2e support.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from tests.e2e.netcore_e2e.context import E2EContext
from tests.e2e.netcore_e2e.http import query_url
from tests.e2e.netcore_e2e.inventory import load_inventory, validate_inventory
from tests.e2e.netcore_e2e.model import CheckResult, RunReport
from tests.e2e.netcore_e2e.report import write_json, write_junit
from tests.e2e.netcore_e2e.scenarios import SCENARIOS

ROOT = Path(__file__).resolve().parents[3]


# Was: Bündelt Daten und Verhalten für e2 esupport tests.
# Warum: Zusammengehöriger Zustand und seine Operationen bleiben dadurch an einer klaren Stelle.
class E2ESupportTests(unittest.TestCase):
    # Was: Prüft automatisch den Fall inventory.
    # Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    def test_inventory(self) -> None:
        inventory = load_inventory(ROOT / "deploy/open-lab/inventory.example.toml")
        self.assertEqual(validate_inventory(inventory), [])
        self.assertEqual(inventory.contract_version, "netcore.v1")
        self.assertGreaterEqual(len(inventory.services), 17)

    # Was: Prüft automatisch den Fall reports.
    # Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    def test_reports(self) -> None:
        report = RunReport(run_id="unit", started_at="2026-01-01T00:00:00Z")
        report.add(CheckResult(name="ok", status="passed", duration_ms=1.0, evidence={"a": 1}))
        report.add(CheckResult(name="skip", status="skipped", duration_ms=0.0, detail="not requested"))
        with tempfile.TemporaryDirectory() as temp:
            json_path = Path(temp) / "report.json"
            xml_path = Path(temp) / "junit.xml"
            write_json(report, json_path)
            write_junit(report, xml_path)
            value = json.loads(json_path.read_text())
            self.assertEqual(value["run_id"], "unit")
            self.assertIn("testsuite", xml_path.read_text())

    # Was: Prüft automatisch den Fall scenario registry and fixture numbers.
    # Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    def test_scenario_registry_and_fixture_numbers(self) -> None:
        expected = {
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
        self.assertEqual(set(SCENARIOS), expected)
        inventory = load_inventory(ROOT / "deploy/open-lab/inventory.example.toml")
        report = RunReport(run_id="deterministic-run", started_at="2026-01-01T00:00:00Z")
        context = E2EContext(inventory=inventory, report=report, artifacts_dir=ROOT / "tests/e2e/artifacts/unit")
        first = context.fixture_numbers()
        second = context.fixture_numbers()
        self.assertEqual(first, second)
        issi_a, issi_b, gssi = first
        self.assertEqual(issi_b, issi_a + 1)
        self.assertTrue(7_000_000 <= issi_a < 7_500_000)
        self.assertTrue(2_000_000 <= gssi < 2_500_000)

    # Was: Prüft automatisch den Fall query url.
    # Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    def test_query_url(self) -> None:
        url = query_url("http://127.0.0.1:8210", "/api/v1/logs", contains="A B", limit=100)
        self.assertEqual(url, "http://127.0.0.1:8210/api/v1/logs?contains=A+B&limit=100")


# Was: Startet den Programmablauf nur dann, wenn diese Datei direkt ausgeführt wird.
# Warum: Beim Import als Modul sollen nur Funktionen bereitstehen und keine Nebenwirkungen automatisch starten.
if __name__ == "__main__":
    unittest.main()
