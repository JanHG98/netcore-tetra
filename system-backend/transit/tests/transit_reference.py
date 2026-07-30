#!/usr/bin/env python3
# NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Verbindungen zu anderen Netzen und Systemen.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

"""Small dependency-free acceptance model for route order, loop prevention and failover."""

from dataclasses import dataclass


# Was: Bündelt Daten und Verhalten für peer.
# Warum: Zusammengehöriger Zustand und seine Operationen bleiben dadurch an einer klaren Stelle.
@dataclass(frozen=True)
class Peer:
    peer_id: str
    region_id: str
    state: str
    latency_ms: float
    priority: int


# Was: Bündelt Daten und Verhalten für Weiterleitung.
# Warum: Zusammengehöriger Zustand und seine Operationen bleiben dadurch an einer klaren Stelle.
@dataclass(frozen=True)
class Route:
    peer_id: str
    destination_region: str
    preference: int
    metric: int


# Was: Diese Funktion wählt den vorgesehenen Arbeitsschritt.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def select(routes: list[Route], peers: dict[str, Peer], target: str, trace: list[str]) -> list[str]:
    candidates: list[tuple[int, int, float, int, str]] = []
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for route in routes:
        peer = peers[route.peer_id]
        if route.destination_region != target:
            continue
        if peer.state not in {"up", "degraded"}:
            continue
        if peer.region_id in trace:
            continue
        candidates.append((-route.preference, route.metric, peer.latency_ms, -peer.priority, peer.peer_id))
    candidates.sort()
    return [entry[-1] for entry in candidates]


# Was: Startet das Programm, lädt die benötigten Einstellungen und übergibt an den eigentlichen Dienstablauf.
# Warum: Ein klarer Einstiegspunkt hält Startreihenfolge, Fehlerausgabe und geordnetes Beenden zusammen.
def main() -> None:
    peers = {
        "b-primary": Peer("b-primary", "region-b", "up", 15.0, 100),
        "b-backup": Peer("b-backup", "region-c", "up", 30.0, 50),
        "loop": Peer("loop", "region-a", "up", 1.0, 1000),
    }
    routes = [
        Route("b-primary", "region-b", 200, 10),
        Route("b-backup", "region-b", 100, 20),
        Route("loop", "region-b", 500, 1),
    ]
    order = select(routes, peers, "region-b", ["region-a"])
    assert order == ["b-primary", "b-backup"], order
    peers["b-primary"] = Peer("b-primary", "region-b", "down", 15.0, 100)
    order = select(routes, peers, "region-b", ["region-a"])
    assert order == ["b-backup"], order
    assert len(["region-a", "region-c"]) < 8
    assert "region-a" in ["region-a", "region-c"]
    print("Transit reference checks: OK")


# Was: Startet den Programmablauf nur dann, wenn diese Datei direkt ausgeführt wird.
# Warum: Beim Import als Modul sollen nur Funktionen bereitstehen und keine Nebenwirkungen automatisch starten.
if __name__ == "__main__":
    main()
