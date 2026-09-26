# Architektur

```text
Subscriber Core ─┐
Mobility Core  ──┼─ read-only reconcile ─► Asset Management
Task Workflow  ◄─┘                         ├─ Assets
MQTT Broker    ◄───────────────────────────┤─ Personen
                                          ├─ Ausgaben
                                          └─ Wartung
```

Der Dienst schreibt keine Teilnehmerprofile und keine Mobility-Routen zurück. Dadurch entstehen keine konkurrierenden Wahrheiten.
