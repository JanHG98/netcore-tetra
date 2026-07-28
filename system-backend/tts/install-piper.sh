#!/usr/bin/env bash
# NETCORE-KOMMENTAR – Was: Enthält die Logik oder Einstellungen für install piper.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

set -euo pipefail

SERVICE_USER="${SERVICE_USER:-bluestation}"
SERVICE_GROUP="${SERVICE_GROUP:-$SERVICE_USER}"
DEFAULT_VOICE="${DEFAULT_VOICE:-${VOICE:-de_DE-thorsten-medium}}"
VOICE_LIST="${VOICE_LIST:-de_DE-thorsten-medium de_DE-thorsten-high de_DE-karlsson-low de_DE-pavoque-low de_DE-thorsten_emotional-medium}"
VENV="${VENV:-/opt/netcore-piper}"
VOICE_DIR="${VOICE_DIR:-/var/lib/netcore/piper}"
TTS_CACHE="${TTS_CACHE:-/var/cache/netcore/tts}"
TTS_TEMPLATES="${TTS_TEMPLATES:-/var/lib/netcore/tts/templates}"
PIPER_PORT="${PIPER_PORT:-5005}"
UNIT_PATH="/etc/systemd/system/netcore-piper.service"

# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if [[ ${EUID} -ne 0 ]]; then
  echo "Run this installer as root (sudo)." >&2
  exit 1
fi
# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if ! id "$SERVICE_USER" >/dev/null 2>&1; then
  echo "Service user '$SERVICE_USER' does not exist. Set SERVICE_USER and SERVICE_GROUP." >&2
  exit 1
fi

# Was: Installiert oder aktualisiert benötigte Systempakete.
# Warum: Die Dienste benötigen diese Werkzeuge und Bibliotheken für Build und Betrieb.
apt-get update
# Was: Installiert oder aktualisiert benötigte Systempakete.
# Warum: Die Dienste benötigen diese Werkzeuge und Bibliotheken für Build und Betrieb.
apt-get install -y python3 python3-venv curl
python3 -m venv "$VENV"
"$VENV/bin/python" -m pip install --upgrade pip
"$VENV/bin/python" -m pip install --upgrade 'piper-tts[http]'

# Was: Kopiert eine Datei mit festgelegten Rechten und Eigentümern.
# Warum: Korrekte Dateirechte sind für einen sicheren und reproduzierbaren Dienststart notwendig.
install -d -o "$SERVICE_USER" -g "$SERVICE_GROUP" -m 0750 \
  "$VOICE_DIR" "$TTS_CACHE" "$TTS_TEMPLATES"

read -r -a voices <<< "$VOICE_LIST"
# Was: Prüft die folgende Voraussetzung und führt den passenden Zweig aus.
# Warum: Fehlende Rechte, Dateien oder Einstellungen sollen früh und verständlich behandelt werden.
if [[ ! " ${voices[*]} " =~ " ${DEFAULT_VOICE} " ]]; then
  voices+=("$DEFAULT_VOICE")
fi
# Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung gilt.
# Warum: Gleichartige Installations- oder Prüfaufgaben werden dadurch vollständig abgearbeitet.
for voice in "${voices[@]}"; do
  echo "Downloading/checking Piper voice: $voice"
  runuser -u "$SERVICE_USER" -- \
    "$VENV/bin/python" -m piper.download_voices --data-dir "$VOICE_DIR" "$voice"
done

sed \
  -e "s|^User=.*|User=$SERVICE_USER|" \
  -e "s|^Group=.*|Group=$SERVICE_GROUP|" \
  -e "s|^WorkingDirectory=.*|WorkingDirectory=$VOICE_DIR|" \
  -e "s|^Environment=HOME=.*|Environment=HOME=$VOICE_DIR|" \
  -e "s|^ExecStart=.*|ExecStart=$VENV/bin/python -m piper.http_server -m $DEFAULT_VOICE --data-dir $VOICE_DIR --host 127.0.0.1 --port $PIPER_PORT|" \
  -e "s|^ReadWritePaths=.*|ReadWritePaths=$VOICE_DIR $TTS_CACHE|" \
  "$(dirname "$0")/netcore-piper.service" > "$UNIT_PATH"
chmod 0644 "$UNIT_PATH"

# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl daemon-reload
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl enable --now netcore-piper.service
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl restart netcore-piper.service
# Was: Steuert den zugehörigen systemd-Dienst.
# Warum: Systemd soll Start, Stopp, Neustart und automatischen Boot des Dienstes zuverlässig verwalten.
systemctl --no-pager --full status netcore-piper.service || true

echo
echo "Piper voices available on port $PIPER_PORT:"
# Was: Ruft eine HTTP-Schnittstelle auf oder lädt Daten darüber.
# Warum: Damit lässt sich die Erreichbarkeit prüfen oder eine benötigte Ressource automatisiert abrufen.
curl --fail --silent --show-error "http://127.0.0.1:$PIPER_PORT/voices" \
  | "$VENV/bin/python" -c 'import json,sys; print("\n".join(sorted(json.load(sys.stdin).keys())))'
echo
