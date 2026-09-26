# NETCORE-KOMMENTAR – Was: Enthält automatische Prüfungen für test edge fallback reference.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

from __future__ import annotations

import json
import tempfile
import tomllib
import unittest
from collections import deque
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]


# Was: Führt den Arbeitsschritt `classify_mode` für classify mode aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def classify_mode(
    *,
    gateway_connected: bool,
    matrix_fresh: bool,
    required_levels: list[str],
    unhealthy_for: float,
    healthy_for: float,
    enter_after: float = 15.0,
    recover_after: float = 20.0,
) -> str:
    full_isolation = not gateway_connected or not matrix_fresh
    partial_failure = any(level != "available" for level in required_levels)
    if full_isolation:
        return "isolated" if unhealthy_for >= enter_after else "degraded"
    if partial_failure:
        return "degraded"
    return "online" if healthy_for >= recover_after else "recovering"


# Was: Führt den Arbeitsschritt `service_available` für Dienst available aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def service_available(*, gateway_connected: bool, matrix_fresh: bool, level: str | None) -> bool:
    return gateway_connected and matrix_fresh and level == "available"


# Was: Diese Funktion liest jsonl with torn tail.
# Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
def read_jsonl_with_torn_tail(path: Path) -> deque[dict]:
    contents = path.read_text(encoding="utf-8")
    complete_final_line = contents.endswith("\n")
    lines = contents.splitlines()
    records: deque[dict] = deque()
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for index, line in enumerate(lines):
        # Was: Führt einen fehleranfälligen Abschnitt mit geregelter Fehlerbehandlung aus.
        # Warum: Ein einzelner Fehler soll kontrolliert gemeldet oder aufgefangen werden, statt den gesamten Dienst unverständlich abzubrechen.
        try:
            records.append(json.loads(line))
        except json.JSONDecodeError:
            if index + 1 == len(lines) and not complete_final_line:
                break
            raise
    return records


# Was: Bündelt Daten und Verhalten für edge fallback reference tests.
# Warum: Zusammengehöriger Zustand und seine Operationen bleiben dadurch an einer klaren Stelle.
class EdgeFallbackReferenceTests(unittest.TestCase):
    # Was: Prüft automatisch den Fall partial failure is degraded not full isolation.
    # Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    def test_partial_failure_is_degraded_not_full_isolation(self) -> None:
        levels = ["available"] * 5 + ["unavailable"]
        self.assertEqual(
            classify_mode(
                gateway_connected=True,
                matrix_fresh=True,
                required_levels=levels,
                unhealthy_for=3600,
                healthy_for=0,
            ),
            "degraded",
        )

    # Was: Prüft automatisch den Fall Gateway or lease loss enters full isolation und weitere Angaben.
    # Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    def test_gateway_or_lease_loss_enters_full_isolation_after_hysteresis(self) -> None:
        healthy = ["available"] * 6
        self.assertEqual(
            classify_mode(
                gateway_connected=False,
                matrix_fresh=False,
                required_levels=healthy,
                unhealthy_for=14,
                healthy_for=0,
            ),
            "degraded",
        )
        self.assertEqual(
            classify_mode(
                gateway_connected=False,
                matrix_fresh=False,
                required_levels=healthy,
                unhealthy_for=15,
                healthy_for=0,
            ),
            "isolated",
        )
        self.assertEqual(
            classify_mode(
                gateway_connected=True,
                matrix_fresh=False,
                required_levels=healthy,
                unhealthy_for=15,
                healthy_for=0,
            ),
            "isolated",
        )

    # Was: Prüft automatisch den Fall stale matrix fails every central Dienst closed.
    # Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    def test_stale_matrix_fails_every_central_service_closed(self) -> None:
        self.assertTrue(service_available(gateway_connected=True, matrix_fresh=True, level="available"))
        self.assertFalse(service_available(gateway_connected=True, matrix_fresh=False, level="available"))
        self.assertFalse(service_available(gateway_connected=False, matrix_fresh=True, level="available"))
        self.assertFalse(service_available(gateway_connected=True, matrix_fresh=True, level="unknown"))

    # Was: Prüft automatisch den Fall recovery hysteresis.
    # Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    def test_recovery_hysteresis(self) -> None:
        levels = ["available"] * 6
        self.assertEqual(
            classify_mode(
                gateway_connected=True,
                matrix_fresh=True,
                required_levels=levels,
                unhealthy_for=0,
                healthy_for=19,
            ),
            "recovering",
        )
        self.assertEqual(
            classify_mode(
                gateway_connected=True,
                matrix_fresh=True,
                required_levels=levels,
                unhealthy_for=0,
                healthy_for=20,
            ),
            "online",
        )

    # Was: Prüft automatisch den Fall torn final spool Datensatz does not destroy und weitere Angaben.
    # Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    def test_torn_final_spool_record_does_not_destroy_previous_records(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "edge.jsonl"
            path.write_text('{"sequence":1}\n{"sequence":2}\n{"sequence":', encoding="utf-8")
            self.assertEqual([item["sequence"] for item in read_jsonl_with_torn_tail(path)], [1, 2])

    # Was: Prüft automatisch den Fall inventory Gateway and TETRA-Basisstation fallback maps are und weitere Angaben.
    # Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    def test_inventory_gateway_and_tbs_fallback_maps_are_identical(self) -> None:
        inventory = tomllib.loads((ROOT / "deploy/open-lab/inventory.example.toml").read_text(encoding="utf-8"))
        runtime = {item["name"] for item in inventory["services"]}
        tbs = tomllib.loads((ROOT / "config.toml").read_text(encoding="utf-8"))["edge_fallback"]
        gateway = tomllib.loads(
            (ROOT / "system-backend/node-gateway/config/node-gateway.example.toml").read_text(encoding="utf-8")
        )["service_monitor"]
        targets = {item["name"]: item for item in gateway["targets"]}
        self.assertEqual(set(tbs["service_fallbacks"]), runtime)
        self.assertEqual(set(targets), runtime - {"node-gateway"})
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for name, target in targets.items():
            self.assertEqual(target["fallback_mode"], tbs["service_fallbacks"][name])
        self.assertFalse(tbs["unknown_service_is_available"])
        self.assertGreaterEqual(tbs["service_matrix_lease_secs"], gateway["interval_secs"] * 2)


# Was: Startet den Programmablauf nur dann, wenn diese Datei direkt ausgeführt wird.
# Warum: Beim Import als Modul sollen nur Funktionen bereitstehen und keine Nebenwirkungen automatisch starten.
if __name__ == "__main__":
    unittest.main()
