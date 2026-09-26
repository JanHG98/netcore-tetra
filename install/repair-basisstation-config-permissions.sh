#!/usr/bin/env bash
set -Eeuo pipefail

if [[ ${EUID} -ne 0 ]]; then
    echo "FEHLER: Bitte als root ausführen (sudo $0)." >&2
    exit 1
fi

UNIT="${UNIT:-tetra.service}"
CONFIG_PATH="${CONFIG_PATH:-/etc/netcore/config.toml}"

[[ -f "$CONFIG_PATH" ]] || { echo "FEHLER: $CONFIG_PATH fehlt." >&2; exit 1; }
systemctl cat "$UNIT" >/dev/null 2>&1 || { echo "FEHLER: Unit $UNIT fehlt." >&2; exit 1; }

SERVICE_USER="$(systemctl show "$UNIT" -p User --value)"
[[ -n "$SERVICE_USER" ]] || SERVICE_USER=root
SERVICE_GROUP="$(systemctl show "$UNIT" -p Group --value)"
[[ -n "$SERVICE_GROUP" ]] || SERVICE_GROUP="$(id -gn "$SERVICE_USER")"

install -d -m 0750 -o root -g "$SERVICE_GROUP" "$(dirname "$CONFIG_PATH")"
chown root:"$SERVICE_GROUP" "$CONFIG_PATH"
chmod 0660 "$CONFIG_PATH"

if [[ "$SERVICE_USER" == root ]]; then
    test -r "$CONFIG_PATH"
else
    runuser -u "$SERVICE_USER" -- test -r "$CONFIG_PATH"
fi

systemctl reset-failed "$UNIT" || true
systemctl start "$UNIT"
systemctl --no-pager --full status "$UNIT"

echo "OK: $CONFIG_PATH ist für $SERVICE_USER:$SERVICE_GROUP les- und schreibbar und $UNIT wurde gestartet."
