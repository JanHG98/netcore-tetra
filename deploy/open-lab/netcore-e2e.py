#!/usr/bin/env python3
# NETCORE-KOMMENTAR – Was: Beschreibt Installation oder Betrieb von netcore e2e.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

from __future__ import annotations

import runpy
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
runpy.run_path(str(ROOT / "tests/e2e/netcore_open_lab_e2e.py"), run_name="__main__")
