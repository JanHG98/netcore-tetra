# Architektur des IoT Gateways – Phase 5

```text
Event Sources ─► Poller ─► Schema/Dedup ─► persistente Outbox ─► MQTT
MQTT Commands ─► Command Ledger ─► Policy ─► Adapter/Sandbox ─► Ack
MQTT Discovery ◄──────────────── Home Assistant Adapter
HA Entity State ────────────────► normalisierter External-State Store
CCU XML-RPC ────────────────────► Homematic-Datapoint Store
```

Die Adapter verwenden denselben Command-/Ack-/Policy-Pfad. Home Assistant oder Homematic erhalten keinen Sonderweg an der Policy vorbei.
