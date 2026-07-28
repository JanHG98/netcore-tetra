#!/usr/bin/env python3
# NETCORE-KOMMENTAR – Was: Enthält automatische Prüfungen für open lab contract test.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

from __future__ import annotations

import json
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
INVENTORY = ROOT / "deploy/open-lab/inventory.example.toml"
GENERATED = ROOT / "deploy/open-lab/generated"
URL_RE = re.compile(r"(?P<scheme>https?|wss?)://(?P<host>[^/:\s\"']+):(?P<port>[0-9]{2,5})")


# Was: Startet das Programm, lädt die benötigten Einstellungen und übergibt an den eigentlichen Dienstablauf.
# Warum: Ein klarer Einstiegspunkt hält Startreihenfolge, Fehlerausgabe und geordnetes Beenden zusammen.
def main() -> int:
    errors: list[str] = []
    with INVENTORY.open("rb") as handle:
        inventory = tomllib.load(handle)
    services = {item["name"]: item for item in inventory["services"]}
    by_port = {int(item["port"]): item for item in services.values()}
    if len(by_port) != len(services):
        errors.append("management ports are not unique")

    catalog = json.loads((GENERATED / "service-catalog.json").read_text())
    if {item["name"] for item in catalog["services"]} != set(services):
        errors.append("generated service catalog differs from inventory")
    if catalog["contract_version"] != "netcore.v1":
        errors.append("catalog contract version is not netcore.v1")

    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for name, service in services.items():
        config = GENERATED / "configs" / name / Path(service["config_target"]).name
        if not config.is_file():
            errors.append(f"missing rendered config for {name}")
            continue
        text = config.read_text()
        # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
        # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
        for match in URL_RE.finditer(text):
            port = int(match.group("port"))
            target = by_port.get(port)
            if target and match.group("host") != target["host"]:
                errors.append(f"{name}: port {port} still points to {match.group('host')} instead of {target['host']}")
        # Was: Führt einen fehleranfälligen Abschnitt mit geregelter Fehlerbehandlung aus.
        # Warum: Ein einzelner Fehler soll kontrolliert gemeldet oder aufgefangen werden, statt den gesamten Dienst unverständlich abzubrechen.
        try:
            tomllib.loads(text)
        except Exception as error:
            errors.append(f"{name}: rendered TOML invalid: {error}")

    schema_dir = ROOT / "system-backend/shared/contracts/schemas"
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for schema in schema_dir.glob("*.json"):
        # Was: Führt einen fehleranfälligen Abschnitt mit geregelter Fehlerbehandlung aus.
        # Warum: Ein einzelner Fehler soll kontrolliert gemeldet oder aufgefangen werden, statt den gesamten Dienst unverständlich abzubrechen.
        try:
            value = json.loads(schema.read_text())
            if value.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
                errors.append(f"{schema.name}: unexpected JSON Schema draft")
        except Exception as error:
            errors.append(f"{schema.name}: invalid JSON: {error}")

    if errors:
        print("\n".join(errors), file=sys.stderr)
        return 1
    print(f"Open-Lab contract integration reference: OK ({len(services)} services)")
    return 0


# Was: Startet den Programmablauf nur dann, wenn diese Datei direkt ausgeführt wird.
# Warum: Beim Import als Modul sollen nur Funktionen bereitstehen und keine Nebenwirkungen automatisch starten.
if __name__ == "__main__":
    raise SystemExit(main())
