# Architektur des IoT Gateways – Phase 4

## Komponenten

- Event-Poller für vier `netcore-event-v1`-Quellen
- MQTT-3.1.1-Client mit QoS 0/1 und Last Will
- persistente Publish-Outbox
- Event-Deduplizierung
- Command-Parser und Contract-Validierung
- Zeitfenster- und Retain-Prüfung
- persistente Command-Deduplizierung
- Policy-Engine mit Default Deny und Deny-Vorrang
- OPEN-LAB-Sandbox-Executor
- Command-Ack-Publisher
- persistenter virtueller Gerätezustand
- WebUI/API/Metrics

## Command-Sequenz

```text
MQTT PUBLISH command
  → MQTT-PUBACK an Publisher
  → JSON/Schema prüfen
  → command_id gegen Ledger prüfen
  → Retain und Zeitfenster prüfen
  → Policy auswerten
  → accepted/executing Ack in Outbox
  → Sandbox-Executor
  → virtuellen Zustand persistieren
  → succeeded/failed Ack in Outbox
  → terminales Ledger und Audit persistieren
```

Transport-PUBACK und Fach-Ack sind absichtlich getrennt.
