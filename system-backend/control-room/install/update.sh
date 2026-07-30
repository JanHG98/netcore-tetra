#!/usr/bin/env bash
# NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Leitstellenfunktionen und Bedienoberflächen.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
# Was: Baut oder prüft die Rust-Komponenten.
# Warum: So wird vor Installation oder Start sichergestellt, dass der Quellcode technisch verwendbar ist.
cargo build --release --package netcore-control-room --manifest-path "$ROOT/Cargo.toml"
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl stop netcore-control-room.service
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -m 0755 "$ROOT/target/release/netcore-control-room" /usr/local/bin/netcore-control-room
# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -m 0644 "$ROOT/system-backend/control-room/systemd/netcore-control-room.service" /etc/systemd/system/netcore-control-room.service
source "$(cd "$(dirname "${BASH_SOURCE[0]}")/../../shared/install" && pwd)/lxc-network.sh"
netcore_configure_lxc_endpoint "/etc/netcore-control-room/control-room.toml" "control-room" "9010"
chown root:netcore /etc/netcore-control-room/control-room.toml
chmod 0640 /etc/netcore-control-room/control-room.toml
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl daemon-reload
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl start netcore-control-room.service
# Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung gilt.
# Warum: Gleichartige Installations- oder Prüfaufgaben werden dadurch vollständig abgearbeitet.
for _ in {1..20}; do
  # Was: Steuert den zugehörigen systemd-Dienst.
  # Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
  systemctl is-active --quiet netcore-control-room.service && break
  sleep 0.25
done
# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if ! systemctl is-active --quiet netcore-control-room.service; then
  echo "Control Room konnte nicht gestartet werden. Letzte Logs:" >&2
  journalctl -u netcore-control-room.service -n 80 --no-pager >&2 || true
  exit 1
fi
# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if command -v curl >/dev/null 2>&1; then
  # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung gilt.
  # Warum: Gleichartige Installations- oder Prüfaufgaben werden dadurch vollständig abgearbeitet.
  for _ in {1..20}; do
    # Was: Ruft eine HTTP-Schnittstelle auf oder lädt Daten darüber.
    # Warum: Damit lässt sich die Erreichbarkeit prüfen oder eine benötigte Ressource automatisiert abrufen.
    curl -fsS "http://${NETCORE_DETECTED_LXC_IP}:9010/health/live" >/dev/null 2>&1 && break
    sleep 0.25
  done
  # Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
  # Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
  if ! curl -fsS "http://${NETCORE_DETECTED_LXC_IP}:9010/health/live" >/dev/null; then
    echo "Control Room läuft, aber der HTTP-Healthcheck ist nicht erreichbar." >&2
    ss -lntp | grep ':9010' >&2 || true
    journalctl -u netcore-control-room.service -n 80 --no-pager >&2 || true
    exit 1
  fi
fi
echo "Control Room WebUI erreichbar: http://${NETCORE_DETECTED_LXC_IP}:9010/"
