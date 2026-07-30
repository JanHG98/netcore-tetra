#!/usr/bin/env bash
# NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für die Kopplung von TETRA-Paketdaten an IP-Netze.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

set -euo pipefail
[[ $EUID -eq 0 ]] || { echo "Bitte als root/sudo ausführen." >&2; exit 1; }
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl disable --now netcore-ip-gateway.service 2>/dev/null || true
# Was: Entfernt nicht mehr benötigte Dateien oder alte Zustände.
# Warum: Veraltete Reste könnten einen erneuten Start oder eine Neuinstallation verfälschen.
rm -f /etc/systemd/system/netcore-ip-gateway.service /usr/local/bin/netcore-ip-gateway
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl daemon-reload
cat <<'NOTE'
Binärdatei und systemd-Unit wurden entfernt.
Bewusst erhalten:
  /etc/netcore/ip-gateway.toml
  /var/lib/netcore-ip-gateway/
Die nftables-Tabellen netcore_ip_gateway und netcore_ip_gateway_nat können bei Bedarf manuell entfernt werden.
NOTE
