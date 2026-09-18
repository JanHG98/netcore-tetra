#!/usr/bin/env bash
# Shared installer; keeps config, credentials and the delivery database intact.
set -Eeuo pipefail
[[ ${EUID} -eq 0 ]] || { echo 'Bitte als root ausfuehren.' >&2; exit 1; }
[[ "${1:-}" == install || "${1:-}" == update ]] || { echo 'Aufruf: deploy.sh install|update' >&2; exit 1; }
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
SERVICE_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
APP_DIR=/usr/local/lib/netcore-alert-service
UNIT=netcore-alert-service.service
UNIT_PATH=/etc/systemd/system/netcore-alert-service.service
CONFIG=/etc/netcore/alert-service.toml
ENV_FILE=/etc/netcore/alert-service.env

command -v systemctl >/dev/null
python3 -c 'import sys, tomllib, sqlite3; assert sys.version_info >= (3, 11), "Python >= 3.11 erforderlich"'
[[ -f "${SERVICE_DIR}/main.py" && -d "${SERVICE_DIR}/static" ]]
# Compile in memory: no writes to the checkout and no service interruption on failure.
python3 - "${SERVICE_DIR}" <<'PY'
from pathlib import Path
import sys
for source in Path(sys.argv[1]).glob('*.py'):
    compile(source.read_bytes(), str(source), 'exec')
PY

getent group netcore-alert >/dev/null || groupadd --system netcore-alert
id netcore-alert >/dev/null 2>&1 || useradd --system --gid netcore-alert \
  --home-dir /var/lib/netcore-alert-service --shell /usr/sbin/nologin netcore-alert
install -d -m 0755 /etc/netcore /usr/local/lib
install -d -o netcore-alert -g netcore-alert -m 0750 /var/lib/netcore-alert-service
if [[ ! -f "$CONFIG" ]]; then
  install -o root -g netcore-alert -m 0640 "${SERVICE_DIR}/config/alert-service.example.toml" "$CONFIG"
fi
if [[ ! -f "$ENV_FILE" ]]; then
  (umask 077; python3 - <<'PY' >"$ENV_FILE"
import secrets
print('NETCORE_ALERT_TOKEN=' + secrets.token_urlsafe(32))
print('# NETCORE_CONTROL_ROOM_PASSWORD=')
PY
  )
  chown root:netcore-alert "$ENV_FILE"
  chmod 0640 "$ENV_FILE"
fi
python3 - "$CONFIG" <<'PY'
import sys, tomllib
with open(sys.argv[1], 'rb') as stream:
    tomllib.load(stream)
PY

STAMP="$(date -u +%Y%m%dT%H%M%SZ)-$$"
BACKUP="/var/backups/netcore-alert-service/${STAMP}"
STAGE="$(mktemp -d /usr/local/lib/.netcore-alert-service.XXXXXX)"
install -d -m 0700 "$BACKUP"
cp -a "$CONFIG" "$ENV_FILE" "$BACKUP/"
[[ ! -e "$UNIT_PATH" ]] || cp -a "$UNIT_PATH" "$BACKUP/"
[[ ! -d "$APP_DIR" ]] || cp -a "$APP_DIR" "$BACKUP/app"
cp -a "${SERVICE_DIR}/." "$STAGE/"
chown -R root:root "$STAGE"
chmod 0755 "$STAGE"
WAS_ACTIVE=0
systemctl is-active --quiet "$UNIT" && WAS_ACTIVE=1
CHANGED=0
rollback() {
  local result=$?
  trap - ERR
  if [[ "$CHANGED" == 1 ]]; then
    echo 'Dienststatus zum Fehlerzeitpunkt:' >&2
    systemctl status "$UNIT" --no-pager -l >&2 || true
    journalctl -u "$UNIT" -n 40 --no-pager >&2 || true
    systemctl stop "$UNIT" || true
    if [[ -d "$BACKUP/app" ]]; then
      # Keep the unsuccessful release for diagnosis; never touch the database.
      [[ ! -d "$APP_DIR" ]] || mv "$APP_DIR" "$BACKUP/failed-app"
      cp -a "$BACKUP/app" "$APP_DIR"
    fi
    if [[ -f "$BACKUP/$UNIT" ]]; then cp -a "$BACKUP/$UNIT" "$UNIT_PATH"; fi
    systemctl daemon-reload || true
    if [[ "$WAS_ACTIVE" == 1 ]]; then systemctl start "$UNIT" || true; fi
  fi
  echo "Installation fehlgeschlagen. Sicherung: $BACKUP" >&2
  echo "Datenbank und bestehende Konfiguration wurden nicht zurueckgesetzt." >&2
  exit "$result"
}
trap rollback ERR

if [[ "$WAS_ACTIVE" == 1 ]]; then systemctl stop "$UNIT"; fi
CHANGED=1
# sqlite backup includes committed WAL content and also works for a custom DB path.
python3 - "$CONFIG" "$BACKUP/alerts.sqlite3" <<'PY'
import sqlite3, sys, tomllib
from pathlib import Path
with open(sys.argv[1], 'rb') as stream:
    path = Path(tomllib.load(stream)['storage']['database'])
if path.is_file():
    with sqlite3.connect(str(path)) as source, sqlite3.connect(sys.argv[2]) as target:
        source.backup(target)
PY
if [[ -d "$APP_DIR" ]]; then mv "$APP_DIR" "$BACKUP/replaced-app"; fi
mv "$STAGE" "$APP_DIR"
install -m 0644 "${SERVICE_DIR}/systemd/${UNIT}" "$UNIT_PATH"
systemctl daemon-reload
systemctl enable "$UNIT"
systemctl restart "$UNIT"
echo 'Warte auf den Start der Warnzentrale ...'
READY=0
for _ in {1..20}; do
  if python3 - "$CONFIG" <<'HEALTHCHECK'
import http.client, sys, tomllib, urllib.error, urllib.request
with open(sys.argv[1], 'rb') as stream:
    config = tomllib.load(stream)['server']
host = config.get('bind', '127.0.0.1')
if host in ('0.0.0.0', '::'): host = '127.0.0.1'
if ':' in host: host = '[' + host + ']'
# Type=simple returns before Python has opened the listening socket. Expected
# startup connection failures should retry quietly, not print a traceback.
# A local probe must not depend on the container's HTTP proxy configuration.
opener = urllib.request.build_opener(urllib.request.ProxyHandler({}))
try:
    with opener.open(f"http://{host}:{config.get('port', 8310)}/health/live", timeout=2) as result:
        sys.exit(0 if result.status == 200 else 1)
except (OSError, urllib.error.URLError, http.client.HTTPException):
    sys.exit(1)
HEALTHCHECK
  then READY=1; break; fi
  sleep 1
done
if [[ "$READY" != 1 ]]; then
  echo 'Warnzentrale nach 20 Startversuchen nicht erreichbar. Dienstprotokoll folgt.' >&2
  false # Trigger diagnostics and rollback without resetting the delivery ledger.
fi
systemctl is-active --quiet "$UNIT"
trap - ERR
printf 'Warnzentrale installiert. Sicherung: %s\n' "$BACKUP"
printf 'Konfiguration: %s\nZugangstoken: %s (wird nicht ausgegeben)\n' "$CONFIG" "$ENV_FILE"
printf 'Erstinstallation: SDS-Versand bleibt bis zur Freigabe in [delivery] deaktiviert.\n'
