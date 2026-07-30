# NETCORE-KOMMENTAR – Was: Enthält automatische Prüfungen für report.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

from __future__ import annotations

import json
import xml.etree.ElementTree as ET
from dataclasses import asdict
from pathlib import Path

from .model import RunReport


# Was: Diese Funktion schreibt JSON-Daten.
# Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
def write_json(report: RunReport, path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(asdict(report), indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


# Was: Diese Funktion schreibt junit.
# Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
def write_junit(report: RunReport, path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    suite = ET.Element(
        "testsuite",
        {
            "name": "netcore-open-lab-e2e",
            "tests": str(len(report.results)),
            "failures": str(report.failures),
            "skipped": str(report.skipped),
        },
    )
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for result in report.results:
        case = ET.SubElement(
            suite,
            "testcase",
            {
                "name": result.name,
                "classname": result.scenario or result.service or "netcore.e2e",
                "time": f"{result.duration_ms / 1000.0:.6f}",
            },
        )
        if result.failed:
            failure = ET.SubElement(case, "failure", {"message": result.detail[:500]})
            failure.text = result.detail
        elif result.skipped:
            skipped = ET.SubElement(case, "skipped")
            skipped.text = result.detail
        output = ET.SubElement(case, "system-out")
        output.text = json.dumps(result.evidence, indent=2, ensure_ascii=False)
    ET.ElementTree(suite).write(path, encoding="utf-8", xml_declaration=True)
