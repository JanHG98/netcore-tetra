#!/usr/bin/env bash
# NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Weiterleitung von SDS- und Statusnachrichten.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

set -euo pipefail
# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if [[ $EUID -ne 0 ]]; then echo "Bitte als root/sudo ausführen." >&2; exit 1; fi
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl disable --now netcore-sds-router.service 2>/dev/null || true
# Was: Entfernt nicht mehr benötigte Dateien oder alte Zustände.
# Warum: Veraltete Reste könnten einen erneuten Start oder eine Neuinstallation verfälschen.
rm -f /etc/systemd/system/netcore-sds-router.service
# Was: Entfernt nicht mehr benötigte Dateien oder alte Zustände.
# Warum: Veraltete Reste könnten einen erneuten Start oder eine Neuinstallation verfälschen.
rm -f /usr/local/bin/netcore-sds-router
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl daemon-reload
printf '%s\n' "Konfiguration und Queue bleiben unter /etc/netcore und /var/lib/netcore-sds-router erhalten."
