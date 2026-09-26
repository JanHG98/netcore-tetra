# NetCore IoT Gateway – Phase 5

Der IoT Gateway verbindet `netcore-event-v1` mit MQTT, verarbeitet `netcore-command-v1` über Default-Deny-Policies, quittiert mit `netcore-command-ack-v1` und ergänzt jetzt Home Assistant sowie Homematic IP.

## OPEN LAB

- keine WebUI-Anmeldung;
- keine API-Tokens;
- keine MQTT-Credentials;
- kein TLS;
- Home Assistant MQTT Discovery ist standardmäßig aktiv;
- Zustände ausgewählter Home-Assistant-Entitäten dürfen importiert werden;
- reale Home-Assistant- und Homematic-Schreibzugriffe bleiben standardmäßig aus;
- virtuelle `lab-*`-Aktoren bleiben der sofort nutzbare Testpfad.

## Architektur

```text
NetCore-Dienste ── netcore-event-v1 ──► IoT Gateway ──► MQTT
                                                ├────► Home Assistant Discovery
Home Assistant / HmIP Access Point ── MQTT ─────┤
CCU3 / RaspberryMatic ── XML-RPC (optional) ────┘

Home Assistant Command Topics
        └─► netcore-command-v1 ─► Policy ─► virtueller Aktor

Direkte CCU-Schreibbefehle
        └─► Policy + allow_writes + writable ─► XML-RPC setValue
```

## Home Assistant

Der Gateway veröffentlicht Discovery-Konfigurationen für:

- Gateway-Verfügbarkeit;
- Gesundheit der Eventquellen;
- virtuelle Lab-Relais, -Lichter und -Taster;
- explizit konfigurierte Homematic-Datenpunkte.

Er hört auf `homeassistant/status` und veröffentlicht die Discovery-Daten nach einem Home-Assistant-Neustart erneut.

## Homematic IP

### Access Point

Der Access Point wird über Home Assistant angebunden. Ausgewählte Entitäten werden per Automation an den State-Ingress gesendet:

```text
netcore/v1/integrations/homeassistant/state
```

### CCU3 / RaspberryMatic

Optional kann der Gateway explizit konfigurierte Datenpunkte direkt über XML-RPC pollen. Der Modus ist standardmäßig deaktiviert. HmIP-Datenpunkte nutzen in der Beispielkonfiguration Port 2010.

Direkte Schreibzugriffe benötigen gleichzeitig:

1. `homematic.allow_writes = true`;
2. `writable = true` am Datenpunkt;
3. eine aktive Allow-Policy;
4. ein Ziel, das die Policy tatsächlich trifft.

## Ports

- WebUI/API/Metrics: TCP 8240
- optional lokaler Mosquitto: TCP 1883

## Installation oder Update

```bash
cd /opt/netcore-tetra/system-backend/iot-gateway
chmod 755 install/*.sh
./install/update.sh
```

Der Installer ergänzt fehlende Phase-5-Konfigurationsblöcke und überschreibt bestehende Broker-, Backend- oder CCU-Adressen nicht.

## Wichtige Endpunkte

```text
GET  /api/v1/status
GET  /api/v1/home-assistant
GET  /api/v1/home-assistant/entities
GET  /api/v1/homematic/datapoints
GET  /api/v1/policies
POST /api/v1/actions/home-assistant-discovery
POST /api/v1/actions/homematic-poll-now
POST /api/v1/test/homeassistant-state
```

## Persistenz

```text
/var/lib/netcore-iot-gateway/outbox/
/var/lib/netcore-iot-gateway/dedup.json
/var/lib/netcore-iot-gateway/command-inbox.ndjson
/var/lib/netcore-iot-gateway/command-ledger.json
/var/lib/netcore-iot-gateway/command-audit.ndjson
/var/lib/netcore-iot-gateway/virtual-device-state.json
/var/lib/netcore-iot-gateway/external-entity-state.json
/var/lib/netcore-iot-gateway/homematic-datapoint-state.json
```
