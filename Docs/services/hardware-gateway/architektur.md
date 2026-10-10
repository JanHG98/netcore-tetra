# Hardwareüberwachung: Architektur

```text
Sensoren / Kontakte / Edge-I/O
        │ MQTT oder HTTP
        ▼
NetCore Hardware Gateway
        ├── Geräte-Heartbeat
        ├── Threshold-/Alarmbewertung
        ├── persistenter Zustand
        ├── WebUI/API
        └── MQTT State + netcore-event-v1
                 │
                 ▼
         IoT Gateway / Home Assistant
```

Phase 6 schaltet absichtlich keine echten Ausgänge. Aktorsteuerung wird erst nach einem Hardware-Treiber- und Policy-Abnahmeschritt freigegeben.

**Quellabgleich: 9. Oktober 2026.** [Quellcode](../../../system-backend/hardware-gateway) · [Konfigurationsvorlagen](../../../system-backend/hardware-gateway/config).
