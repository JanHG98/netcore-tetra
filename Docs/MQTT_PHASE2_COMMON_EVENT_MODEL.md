# MQTT-Vorbereitung Phase 2 – Gemeinsames NetCore-Ereignismodell

## Status

Umgesetzt im Branch `MQTT` auf Basis von Phase 1 (Mobility Core als Routing-Wahrheit).

## Ziel

Die Kerndienste veröffentlichen Ereignisse nicht mehr ausschließlich in ihren historisch gewachsenen lokalen Formaten. Zusätzlich steht ein gemeinsamer, transportneutraler Vertrag bereit:

```text
netcore-event-v1
```

MQTT ist in dieser Phase bewusst noch nicht eingebaut. Der spätere IoT Gateway kann die kanonischen Ereignisse übernehmen, ohne Fachbedeutung aus vier unterschiedlichen JSON-Formaten erraten zu müssen.

## Gemeinsamer Vertrag

```json
{
  "schema": "netcore-event-v1",
  "event_id": "uuid",
  "event_type": "subscriber.route_changed",
  "source": {
    "service": "netcore-mobility-core",
    "instance": "Mobility-Core"
  },
  "timestamp": "2026-07-31T19:30:00.000Z",
  "sequence": 14,
  "correlation_id": "optional-uuid",
  "causation_id": "optional-uuid",
  "severity": "info",
  "subject": {
    "type": "subscriber",
    "id": "4010001"
  },
  "payload": {},
  "deduplication_key": "netcore-mobility-core:Mobility-Core:14",
  "labels": {
    "legacy_kind": "subscriber_route_changed"
  }
}
```

Die Instanzkennung wird in dieser Reihenfolge ermittelt:

1. `NETCORE_INSTANCE_ID`
2. `HOSTNAME`
3. `/etc/hostname`
4. technischer Dienstname als Fallback

## Migrierte Produzenten

### Node Gateway

- `node.connected`
- `node.disconnected`
- `node.message_received`
- `node.command_queued`
- `service.state_changed`
- Dependency-Verbindungsereignisse

### Mobility Core

- `subscriber.registered`
- `subscriber.route_changed`
- `subscriber.detached`
- `mobility.transfer_created`
- `mobility.transfer_completed`
- `mobility.transfer_failed`
- `mobility.transfer_timed_out`
- Dependency-Verbindungsereignisse

Die Registrierungslogik unterscheidet nun eine echte Erst-/Wiederanmeldung von einem Wechsel der Serving-TBS. Route-Events enthalten `previous_node`, `serving_node` und `route_generation`.

### Call Control

- `call.requested`
- `call.started`
- `call.release_requested`
- `call.released`
- `call.failed`
- `floor.requested`
- `floor.release_requested`
- `floor.changed`
- `call.restore_requested`
- `call.restored`
- `call.restore_failed`
- `call.media_route_ready`

### SDS Router

- `sds.created`
- `sds.received`
- `sds.retry_scheduled`
- `sds.delivery_accepted`
- `sds.delivery_retry`
- `sds.cancelled`
- `sds.expired`
- `sds.duplicate`
- `sds.requeued`
- `sds.deleted`
- `sds.acknowledged`
- SDS-Routenereignisse
- Teilnehmeran-/abmeldung

## API-Kompatibilität

Der bestehende Endpunkt bleibt erhalten:

```http
GET /api/v1/events?limit=100
```

Er liefert weiterhin die bisherigen lokalen Felder. Jeder Datensatz enthält zusätzlich:

```json
"canonical": { "schema": "netcore-event-v1" }
```

Neue Integrationen verwenden direkt:

```http
GET /api/v1/events/netcore?limit=100
```

Dieser Endpunkt existiert bei Node Gateway, Mobility Core, Call Control und SDS Router.

## Dateien

```text
system-backend/shared/contracts/src/event.rs
system-backend/shared/contracts/schemas/netcore-event-v1.schema.json
system-backend/shared/contracts/examples/netcore-event-subscriber-route-changed.json
system-backend/shared/contracts/EVENT_MODEL_V1.md
system-backend/shared/service-common/src/lib.rs
tools/check_event_model.py
```

## Abnahme

Statisch:

```bash
python3 tools/check_event_model.py
```

Rust:

```bash
cargo test -p netcore-contracts -p netcore-service-common
cargo check \
  -p netcore-node-gateway \
  -p netcore-mobility-core \
  -p netcore-call-control \
  -p netcore-sds-router
```

Runtime je Dienst:

```bash
curl -fsS http://DIENST-IP:PORT/api/v1/events/netcore?limit=5 \
  | python3 -m json.tool
```

## Bewusste Nicht-Ziele

- noch kein MQTT-Broker
- noch keine Topic-Zuordnung
- noch kein Retain/QoS
- noch keine Command-/Ack-Ausführung
- noch kein dauerhafter zentraler Eventbus

Diese Punkte folgen in Phase 3 beziehungsweise Phase 4.
