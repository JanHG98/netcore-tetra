#!/usr/bin/env bash
set -euo pipefail
if [[ ${EUID} -ne 0 ]]; then
  echo "Bitte als root ausführen." >&2
  exit 1
fi
apt-get update
DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends mosquitto mosquitto-clients
install -d -m 0755 /etc/mosquitto/conf.d
cat > /etc/mosquitto/conf.d/netcore-openlab.conf <<'MOSQUITTO'
# NetCore OPEN LAB – keine Benutzer, keine Passwörter, kein TLS.
listener 1883 0.0.0.0
allow_anonymous true
persistence true
persistence_location /var/lib/mosquitto/
MOSQUITTO
systemctl enable mosquitto.service
systemctl restart mosquitto.service
systemctl --no-pager --full status mosquitto.service
