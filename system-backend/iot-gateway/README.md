# NetCore IoT Gateway – Phase 4

Der IoT Gateway verbindet die kanonischen `netcore-event-v1`-Ereignisse mit MQTT und ergänzt in Phase 4 einen persistierten Command-/Ack-/Policy-Pfad.

## OPEN LAB

Diese Stufe ist weiterhin ausschließlich für ein isoliertes Testnetz gedacht:

- keine WebUI-Anmeldung;
- keine API-Tokens;
- keine MQTT-Benutzer oder Passwörter;
- kein TLS;
- `source.actor` ist nur selbst deklarierte Metadaten;
- reale Hardware-Executor sind nicht enthalten.

Trotz offener Transportebene ist die Command-Ausführung nicht beliebig: Die Konfiguration arbeitet mit `default_deny = true`. Mitgeliefert werden nur Allow-Policies für virtuelle `lab-*`-Aktoren.

## Datenpfade

```text
Node Gateway ─────┐
Mobility Core ────┤
Call Control ─────┼─ netcore-event-v1 ─► IoT Gateway ─► MQTT events/state
SDS Router ───────┘

MQTT commands/#
       │
       ▼
netcore-command-v1
       │ Schema / Zeit / Retain / Dublette / Policy
       ▼
OPEN-LAB Sandbox Executor
       │
       ├─ persistenter virtueller Zustand
       └─ netcore-command-ack-v1 → MQTT acks/<command-id>
```

## Mitgelieferte Sandbox-Commands

| Command | Target-Typ | Payload |
|---|---|---|
| `virtual.relay.set` | `virtual_relay` | `{"state":true}` |
| `virtual.light.set` | `virtual_light` | `{"on":true,"brightness":42}` |
| `virtual.button.press` | `virtual_button` | `{}` |

Die Standardpolicies erlauben nur Ziel-IDs, die mit `lab-` beginnen.

## Schutzmechanismen

- stabiles `command_id`;
- persistente Dublettensperre über Neustarts;
- `requested_at` und `expires_at`;
- globale und policy-spezifische TTL-Grenzen;
- Retained Commands standardmäßig verboten;
- Deny überstimmt Allow;
- Default Deny;
- Dry-Run ohne Zustandsänderung;
- persistentes Inbox-, Ledger- und Audit-Log;
- Acks über persistente MQTT-Outbox;
- virtuelle Zustände werden retained publiziert.

## Ports

- WebUI/API/Metrics: TCP 8240
- optional lokaler Mosquitto: TCP 1883

## Installation oder Update

```bash
cd /opt/netcore-tetra/system-backend/iot-gateway
chmod 755 install/*.sh
./install/update.sh
```

Der Installer migriert eine vorhandene Phase-3-Konfiguration, ohne die eingetragenen Backend- oder Broker-Adressen zu überschreiben.

## Wichtige Endpunkte

```text
GET  /api/v1/status
GET  /api/v1/topics
GET  /api/v1/policies
GET  /api/v1/commands
GET  /api/v1/commands/<uuid>
GET  /api/v1/virtual-devices
GET  /api/v1/outbox
POST /api/v1/test/command
```

## MQTT Topics

```text
netcore/v1/events/<domain>/<action>
netcore/v1/state/<subject-type>/<subject-id>
netcore/v1/commands/#
netcore/v1/acks/<command-id>
```

## Persistenz

```text
/var/lib/netcore-iot-gateway/outbox/
/var/lib/netcore-iot-gateway/dedup.json
/var/lib/netcore-iot-gateway/command-inbox.ndjson
/var/lib/netcore-iot-gateway/command-ledger.json
/var/lib/netcore-iot-gateway/command-audit.ndjson
/var/lib/netcore-iot-gateway/virtual-device-state.json
```

## Nächste Phase

Phase 5 ergänzt Home Assistant beziehungsweise Homematic IP als Adapter. Reale Aktionen dürfen erst nach expliziter Adapter- und Policy-Konfiguration entstehen; die OPEN-LAB-Sandbox wird nicht stillschweigend zu einem Hardware-Executor umgebaut.
