# MQTT Phase 3 – IoT Gateway im OPEN LAB

## Ziel

Phase 3 ergänzt `system-backend/iot-gateway/` als eigenständigen LXC-Dienst. Der Dienst übernimmt die vier in Phase 2 eingeführten `netcore-event-v1`-Quellen, prüft und dedupliziert die Ereignisse und veröffentlicht sie über MQTT.

Diese Stufe ist absichtlich offen:

- keine WebUI-Anmeldung;
- keine API-Tokens;
- keine MQTT-Benutzernamen oder Passwörter;
- kein TLS;
- eingehende Commands werden nur gespeichert und **niemals ausgeführt**.

Der Dienst verweigert den Start, wenn `mqtt.execute_commands = true` gesetzt wird. Sichere Command-, Ack- und Policy-Ausführung folgt erst in Phase 4.

## Laufzeitpfad

```text
Node Gateway ─────┐
Mobility Core ────┤
Call Control ─────┼─ netcore-event-v1 ─► IoT Gateway ─► MQTT Broker
SDS Router ───────┘                       │
                                         ├─ persistente Outbox
                                         ├─ Event-ID-Deduplizierung
                                         ├─ retained Subject-States
                                         └─ Command-Beobachtung ohne Ausführung
```

## Ports

- WebUI/API/Metrics: TCP 8240
- lokaler Open-Lab-Mosquitto: TCP 1883

## MQTT Topics

```text
netcore/v1/events/<domain>/<action>
netcore/v1/state/<subject-type>/<subject-id>
netcore/v1/state/services/iot-gateway
netcore/v1/commands/#
```

Eventtopics sind standardmäßig nicht retained. Subject- und Servicezustände sind retained. QoS 0 und 1 werden unterstützt; die Beispielkonfiguration verwendet QoS 1.

## Installation

```bash
cd /opt/netcore-tetra/system-backend/iot-gateway
chmod 755 install/*.sh
./install/install.sh
```

Die Erstinstallation installiert standardmäßig Mosquitto auf demselben LXC. Für einen bereits vorhandenen Broker:

```bash
INSTALL_LOCAL_MQTT_BROKER=0 ./install/install.sh
```

Anschließend die vier Quelladressen setzen:

```bash
./install/configure-openlab.sh \
  NODE_GATEWAY_IP \
  MOBILITY_CORE_IP \
  CALL_CONTROL_IP \
  SDS_ROUTER_IP
```

Ein optionales fünftes Argument setzt den MQTT-Broker.

## Prüfung

```bash
source /etc/netcore/lxc-network.env
curl -fsS "${NETCORE_WEBUI_URL}api/v1/status" | python3 -m json.tool
curl -fsS "${NETCORE_WEBUI_URL}api/v1/sources" | python3 -m json.tool
curl -fsS "${NETCORE_WEBUI_URL}api/v1/topics" | python3 -m json.tool
```

Lokalen Broker beobachten:

```bash
mosquitto_sub -h 127.0.0.1 -p 1883 -v -t 'netcore/v1/#'
```

Testpublikation:

```bash
curl -fsS -X POST \
  -H 'Content-Type: application/json' \
  -d '{"payload":{"message":"Phase 3 funktioniert"}}' \
  "${NETCORE_WEBUI_URL}api/v1/test/publish" \
  | python3 -m json.tool
```

Command-Beobachtung testen:

```bash
mosquitto_pub -h 127.0.0.1 -p 1883 \
  -t 'netcore/v1/commands/test/demo' \
  -m '{"command_id":"openlab-test","requested_state":true}'

curl -fsS "${NETCORE_WEBUI_URL}api/v1/commands" | python3 -m json.tool
```

Im Status müssen `command_execution_enabled` und `commands_executed` weiterhin `false` beziehungsweise `0` bleiben.
