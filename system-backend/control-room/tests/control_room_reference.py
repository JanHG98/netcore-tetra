#!/usr/bin/env python3
# NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Leitstellenfunktionen und Bedienoberflächen.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

"""Dependency, incident and federated-summary reference model for Control Room."""
from dataclasses import dataclass


# Was: Bündelt Daten und Verhalten für Dienst.
# Warum: Zusammengehöriger Zustand und seine Operationen bleiben dadurch an einer klaren Stelle.
@dataclass
class Service:
    name: str
    critical: bool
    failures: int = 0
    status: str = "unknown"


# Was: Diese Funktion wendet den vorgesehenen Arbeitsschritt.
# Warum: Die Änderung wird dadurch nur über einen definierten und prüfbaren Weg wirksam.
def apply(service: Service, live: bool, ready: bool, threshold: int = 3) -> str | None:
    service.status = "healthy" if live and ready else "degraded" if live else "offline"
    service.failures = 0 if service.status == "healthy" else service.failures + 1
    if service.status == "offline" and service.failures >= threshold:
        return f"service:{service.name}"
    return None


# Was: Führt den Arbeitsschritt `first_metric` für first metric aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def first_metric(summaries: dict[str, dict], candidates: list[tuple[str, str]]) -> int | None:
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for service, field in candidates:
        value = summaries.get(service, {}).get(field)
        if isinstance(value, int) and value >= 0:
            return value
    return None


# Was: Führt den Arbeitsschritt `sum_metrics` für sum Messwerte aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def sum_metrics(summaries: dict[str, dict], candidates: list[tuple[str, str]]) -> int | None:
    values = [summaries.get(service, {}).get(field) for service, field in candidates]
    values = [value for value in values if isinstance(value, int) and value >= 0]
    return sum(values) if values else None


# Was: Startet das Programm, lädt die benötigten Einstellungen und übergibt an den eigentlichen Dienstablauf.
# Warum: Ein klarer Einstiegspunkt hält Startreihenfolge, Fehlerausgabe und geordnetes Beenden zusammen.
def main() -> None:
    service = Service("call-control", True)
    assert apply(service, False, False) is None
    assert apply(service, False, False) is None
    assert apply(service, False, False) == "service:call-control"
    assert service.status == "offline"
    assert apply(service, True, True) is None
    assert service.failures == 0

    summaries = {
        "node-gateway": {"connected_nodes": 3},
        "subscriber-core": {"observed_registered": 41},
        "call-control": {"calls_active": 2},
        "sds-router": {"queued": 4, "offline": 3, "in_flight": 1},
    }
    assert first_metric(summaries, [("node-gateway", "connected_nodes")]) == 3
    assert first_metric(summaries, [("subscriber-core", "observed_registered")]) == 41
    assert first_metric(summaries, [("call-control", "calls_active")]) == 2
    assert sum_metrics(summaries, [("sds-router", "queued"), ("sds-router", "offline"), ("sds-router", "in_flight")]) == 8
    assert first_metric(summaries, [("packet-core", "contexts_ready")]) is None
    print("Control Room reference model: OK")


# Was: Startet den Programmablauf nur dann, wenn diese Datei direkt ausgeführt wird.
# Warum: Beim Import als Modul sollen nur Funktionen bereitstehen und keine Nebenwirkungen automatisch starten.
if __name__ == "__main__":
    main()
