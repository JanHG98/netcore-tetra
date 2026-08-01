#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT=${REPO_ROOT:-$(cd "${SCRIPT_DIR}/../../.." && pwd)}
CONFIG=${CONFIG:-/etc/netcore/iot-gateway.toml}
INSTALL_LOCAL_MQTT_BROKER=${INSTALL_LOCAL_MQTT_BROKER:-1}

if [[ ${EUID} -ne 0 ]]; then
  echo "Bitte als root ausführen." >&2
  exit 1
fi

cd "${REPO_ROOT}"

if [[ "${INSTALL_LOCAL_MQTT_BROKER}" == "1" ]]; then
  echo "Installiere lokalen Mosquitto-Broker im OPEN-LAB-Modus ..."
  apt-get update
  DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends mosquitto mosquitto-clients
  install -d -m 0755 /etc/mosquitto/conf.d
  if [[ ! -e /etc/mosquitto/conf.d/netcore-openlab.conf ]]; then
    cat > /etc/mosquitto/conf.d/netcore-openlab.conf <<'MOSQUITTO'
# NetCore Phase 3 OPEN LAB – nur in einem isolierten Testnetz verwenden.
listener 1883 0.0.0.0
allow_anonymous true
persistence true
persistence_location /var/lib/mosquitto/
MOSQUITTO
  fi
  systemctl enable mosquitto.service
  systemctl restart mosquitto.service
  systemctl --no-pager --full status mosquitto.service
fi

systemctl stop netcore-iot-gateway.service 2>/dev/null || true
rm -f /usr/local/bin/netcore-iot-gateway
rm -rf target/release/netcore-iot-gateway target/release/deps/netcore_iot_gateway-*

cargo build --release -p netcore-iot-gateway

getent group netcore-iot-gateway >/dev/null || groupadd --system netcore-iot-gateway
id netcore-iot-gateway >/dev/null 2>&1 || useradd \
  --system \
  --gid netcore-iot-gateway \
  --home-dir /var/lib/netcore-iot-gateway \
  --shell /usr/sbin/nologin \
  netcore-iot-gateway

install -d -o netcore-iot-gateway -g netcore-iot-gateway -m 0750 \
  /var/lib/netcore-iot-gateway \
  /var/lib/netcore-iot-gateway/outbox
install -d -m 0755 /etc/netcore
install -m 0755 target/release/netcore-iot-gateway /usr/local/bin/netcore-iot-gateway

if [[ ! -f "${CONFIG}" ]]; then
  install -o root -g netcore-iot-gateway -m 0640 \
    system-backend/iot-gateway/config/iot-gateway.example.toml \
    "${CONFIG}"
fi

install -m 0644 \
  system-backend/iot-gateway/systemd/netcore-iot-gateway.service \
  /etc/systemd/system/netcore-iot-gateway.service

source "${SCRIPT_DIR}/../../shared/install/lxc-network.sh"
netcore_configure_lxc_endpoint "${CONFIG}" "iot-gateway" "8240"

chown -R netcore-iot-gateway:netcore-iot-gateway /var/lib/netcore-iot-gateway
systemctl daemon-reload
systemctl enable --now netcore-iot-gateway.service
systemctl --no-pager --full status netcore-iot-gateway.service

echo
echo "OPEN LAB: keine Anmeldung, keine Tokens, kein TLS und anonymer MQTT-Zugriff."
echo "WebUI: http://<LXC-IP>:8240/"
echo "MQTT:  <LXC-IP>:1883 (wenn lokaler Broker installiert wurde)"
echo "Vor dem Quelltest die vier source.url-Einträge in ${CONFIG} auf die echten LXC-Adressen setzen."
