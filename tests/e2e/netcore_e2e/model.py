# NETCORE-KOMMENTAR – Was: Enthält automatische Prüfungen für model.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path
from typing import Any


# Was: Bündelt Daten und Verhalten für Dienst.
# Warum: Zusammengehöriger Zustand und seine Operationen bleiben dadurch an einer klaren Stelle.
@dataclass(frozen=True)
class Service:
    name: str
    host: str
    port: int
    unit: str
    user: str
    depends_on: tuple[str, ...] = ()

    # Was: Führt den Arbeitsschritt `base_url` für base url aus.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    @property
    def base_url(self) -> str:
        return f"http://{self.host}:{self.port}"


# Was: Bündelt Daten und Verhalten für inventory.
# Warum: Zusammengehöriger Zustand und seine Operationen bleiben dadurch an einer klaren Stelle.
@dataclass(frozen=True)
class Inventory:
    path: Path
    version: int
    contract_version: str
    mode: str
    ssh_user: str
    ssh_options: tuple[str, ...]
    health_timeout_secs: int
    services: tuple[Service, ...]

    # Was: Führt den Arbeitsschritt `by_name` für by name aus.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    @property
    def by_name(self) -> dict[str, Service]:
        return {service.name: service for service in self.services}


# Was: Bündelt Daten und Verhalten für check result.
# Warum: Zusammengehöriger Zustand und seine Operationen bleiben dadurch an einer klaren Stelle.
@dataclass
class CheckResult:
    name: str
    status: str
    duration_ms: float
    detail: str = ""
    service: str | None = None
    scenario: str | None = None
    evidence: dict[str, Any] = field(default_factory=dict)

    # Was: Führt den Arbeitsschritt `failed` für failed aus.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    @property
    def failed(self) -> bool:
        return self.status == "failed"

    # Was: Führt den Arbeitsschritt `skipped` für skipped aus.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    @property
    def skipped(self) -> bool:
        return self.status == "skipped"


# Was: Bündelt Daten und Verhalten für run report.
# Warum: Zusammengehöriger Zustand und seine Operationen bleiben dadurch an einer klaren Stelle.
@dataclass
class RunReport:
    run_id: str
    started_at: str
    finished_at: str | None = None
    mode: str = "open_lab"
    inventory: str = ""
    results: list[CheckResult] = field(default_factory=list)
    metadata: dict[str, Any] = field(default_factory=dict)

    # Was: Diese Funktion fügt den vorgesehenen Arbeitsschritt.
    # Warum: Das Einfügen wird so einheitlich geprüft und verwaltet.
    def add(self, result: CheckResult) -> None:
        self.results.append(result)

    # Was: Führt den Arbeitsschritt `failures` für failures aus.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    @property
    def failures(self) -> int:
        return sum(result.failed for result in self.results)

    # Was: Führt den Arbeitsschritt `skipped` für skipped aus.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    @property
    def skipped(self) -> int:
        return sum(result.skipped for result in self.results)

    # Was: Führt den Arbeitsschritt `passed` für passed aus.
    # Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    @property
    def passed(self) -> int:
        return sum(result.status == "passed" for result in self.results)
