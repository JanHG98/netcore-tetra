# NetCore IoT Gateway – Phase 3

Der IoT Gateway ist die zentrale Brücke zwischen dem gemeinsamen
`netcore-event-v1`-Modell und MQTT. Er läuft als eigener LXC-Dienst auf Port
**8240** und besitzt eine eigene WebUI.

## Umfang dieser Phase

- Polling der vier Phase-2-Ereignisquellen:
  - Node Gateway
  - Mobility Core
  - Call Control
  - SDS Router
- Prüfung des gemeinsamen `netcore-event-v1`-Vertrags
- persistente Deduplizierung über `event_id`
- persistente Dateisystem-Outbox bei Broker-Ausfall
- MQTT 3.1.1 über TCP, QoS 0 oder 1
- MQTT Last Will und retained Servicezustand
- nicht-retained Eventtopics
- retained Zustände pro Event-Subject
- Topic Registry
- Command-Beobachtung unter `netcore/v1/commands/#`
- WebUI, REST-API, Health und Prometheus-Metriken
- optionaler lokaler Mosquitto-Broker

## Sicherheitsgrenze

Diese Stufe ist absichtlich `open_lab`:

- kein Login;
- keine Management-Tokens;
- kein MQTT-Benutzername oder Passwort;
- kein TLS;
- anonymer Mosquitto-Zugriff beim lokalen Testbroker.

**MQTT-Commands werden nur gespeichert und angezeigt. Sie werden niemals
abgearbeitet.** Command-Schema, Ack-Lifecycle, Policy, Idempotenz und
Aktorsteuerung folgen erst in Phase 4. Die Konfiguration verweigert den Start,
wenn `execute_commands = true` gesetzt wird.

## Standardtopics

```text
netcore/v1/events/<domain>/<action>
netcore/v1/state/<subject-type>/<subject-id>
netcore/v1/state/services/iot-gateway
netcore/v1/commands/#
```

Beispiele:

```text
netcore/v1/events/subscriber/route_changed
netcore/v1/events/node/disconnected
netcore/v1/state/subscribers/4010001
netcore/v1/state/nodes/TBS-01
```

Eventpayloads bleiben unverändert im gemeinsamen `netcore-event-v1`-Format.
Ein Ereignis mit Subject wird zusätzlich als retained State veröffentlicht.

## Installation

Auf einem frischen Debian-/Ubuntu-LXC mit dem vollständigen Repository:

```bash
cd /opt/netcore-tetra/system-backend/iot-gateway
chmod 755 install/*.sh
./install/install.sh
```

Der Erstinstaller installiert standardmäßig einen lokalen Mosquitto-Broker im
OPEN-LAB-Modus. Bei einem vorhandenen externen Broker:

```bash
INSTALL_LOCAL_MQTT_BROKER=0 ./install/install.sh
```

Updates installieren keinen Broker nach:

```bash
./install/update.sh
```

Danach `/etc/netcore/iot-gateway.toml` bearbeiten und die vier `source.url`
Werte auf die realen LXC-Adressen setzen.

## WebUI und API

```text
http://<iot-gateway-lxc>:8240/
```

Wichtige Endpunkte:

```text
GET  /api/v1/status
GET  /api/v1/sources
GET  /api/v1/topics
GET  /api/v1/events
GET  /api/v1/commands
GET  /api/v1/outbox
POST /api/v1/actions/poll-now
POST /api/v1/actions/reconnect
POST /api/v1/test/publish
GET  /health/live
GET  /health/ready
GET  /metrics
```

## Persistenz

```text
/var/lib/netcore-iot-gateway/
├── outbox/
├── dedup.json
└── command-inbox.ndjson
```

Eine Nachricht verlässt die Outbox erst nach einem MQTT-PUBACK bei QoS 1.
Nach einem Timeout bleibt sie liegen und wird später erneut gesendet. Damit ist
die Zustellung bewusst at-least-once; Verbraucher müssen weiterhin anhand der
`event_id` idempotent arbeiten.

### Schnellkonfiguration der Quellen

```bash
./install/configure-openlab.sh NODE_GATEWAY_IP MOBILITY_CORE_IP CALL_CONTROL_IP SDS_ROUTER_IP
```

Optional kann als fünftes Argument ein externer MQTT-Broker angegeben werden.

`/health/ready` liefert erst dann HTTP 200, wenn der MQTT-Broker verbunden ist
und alle aktivierten Ereignisquellen erfolgreich erreichbar sind. Die WebUI und
`/health/live` bleiben auch bei einer gestörten Quelle erreichbar.
