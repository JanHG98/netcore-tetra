#!/usr/bin/env bash
# NETCORE-KOMMENTAR – Was: Enthält die Logik oder Einstellungen für check protocol inventory.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"
python3 -S tools/protocol_inventory.py --check
