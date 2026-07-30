#!/usr/bin/env bash
# NETCORE-KOMMENTAR – Was: Enthält die Logik oder Einstellungen für install extra voices.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

set -euo pipefail

SERVICE_USER="${SERVICE_USER:-netcore-media-library}"
VENV="${VENV:-/opt/netcore-piper}"
VOICE_DIR="${VOICE_DIR:-/var/lib/netcore-media-library/piper}"
PIPER_PORT="${PIPER_PORT:-5005}"
VOICE_LIST="${VOICE_LIST:-de_DE-thorsten-high de_DE-karlsson-low de_DE-pavoque-low de_DE-thorsten_emotional-medium}"

# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if [[ ${EUID} -ne 0 ]]; then
  echo "Run this helper as root (sudo)." >&2
  exit 1
fi
# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if ! id "$SERVICE_USER" >/dev/null 2>&1; then
  echo "Service user '$SERVICE_USER' does not exist." >&2
  exit 1
fi
# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if [[ ! -x "$VENV/bin/python" ]]; then
  echo "Piper virtual environment not found at $VENV." >&2
  exit 1
fi

# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d -o "$SERVICE_USER" -g "$(id -gn "$SERVICE_USER")" -m 0750 "$VOICE_DIR"
read -r -a voices <<< "$VOICE_LIST"
# Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung gilt.
# Warum: Gleichartige Installations- oder Prüfaufgaben werden dadurch vollständig abgearbeitet.
for voice in "${voices[@]}"; do
  echo "Downloading/checking Piper voice: $voice"
  runuser -u "$SERVICE_USER" -- \
    "$VENV/bin/python" -m piper.download_voices --data-dir "$VOICE_DIR" "$voice"
done

# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl restart netcore-piper.service
sleep 2

echo
echo "Available Piper voices:"
# Was: Ruft eine HTTP-Schnittstelle auf oder lädt Daten darüber.
# Warum: Damit lässt sich die Erreichbarkeit prüfen oder eine benötigte Ressource automatisiert abrufen.
curl --fail --silent --show-error "http://127.0.0.1:$PIPER_PORT/voices" \
  | "$VENV/bin/python" -c 'import json,sys; print("\n".join(sorted(json.load(sys.stdin).keys())))'
