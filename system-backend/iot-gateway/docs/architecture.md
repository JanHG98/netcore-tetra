# Architektur

```text
Node Gateway ───────┐
Mobility Core ──────┤ GET /api/v1/events/netcore
Call Control ───────┤
SDS Router ─────────┘
         │
         ▼
 NetCore IoT Gateway
 ├─ Vertrag prüfen
 ├─ event_id deduplizieren
 ├─ Topic ableiten
 ├─ Datei-Outbox
 ├─ MQTT 3.1.1 / QoS 0-1
 ├─ retained Subject-State
 ├─ Command-Inbox (nur Beobachtung)
 └─ WebUI/API/Metrics
         │
         ▼
       Broker
         ├─ Home Assistant (spätere Phase)
         ├─ Homematic-Adapter (spätere Phase)
         ├─ Edge-I/O
         └─ Automationen
```

Der Gateway verändert Eventpayloads nicht. MQTT ist ein Transportadapter und
keine zweite fachliche Datenbank. Die Phase-2-Produzenten bleiben Besitzer der
jeweiligen Wahrheit.

Die aktuelle Polling-Schnittstelle ist bewusst einfach und mit den bestehenden
Diensten kompatibel. Ein späterer interner Eventbus kann denselben Vertrag
verwenden, ohne Topics oder Verbraucher zu ändern.
