#!/usr/bin/env bash
# NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Registrierung, Aufenthaltsbereiche und Teilnehmermobilität.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

set -euo pipefail
REPO_ROOT=${REPO_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)}
exec env REPO_ROOT="$REPO_ROOT" "$REPO_ROOT/system-backend/mobility-core/install/install.sh"
