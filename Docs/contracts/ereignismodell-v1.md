# NetCore Event Model v1

**Quellbezug:** gemeinsamer Vertrag und Schemas unter `system-backend/shared/`, Stand `main` (`c3ccdb4`, 09.10.2026). Phasenangaben sind Einführungshistorie; Ausführungsberechtigungen und optionale Felder richten sich nach dem konkreten Dienst.

## Zweck

`netcore-event-v1` ist das gemeinsame, transportneutrale Ereignisformat für Backend-Dienste. Die gemeinsame Semantik wird unabhängig vom MQTT-Transport definiert. Der vorhandene IoT Gateway bildet die Ereignisse bereits auf MQTT ab.

## Verbindliche Regeln

- `event_type` folgt `domain.action_name`, ausschließlich klein geschrieben.
- `event_id` ist pro Ereignis neu und global eindeutig.
- `source.service` benennt den Diensttyp, `source.instance` die konkrete Instanz.
- `sequence` ist innerhalb einer Instanz monoton steigend, aber nicht global.
- `deduplication_key` ist für Wiederanlauf und mindestens-einmalige Zustellung vorgesehen.
- `correlation_id` verbindet Ereignisse eines gemeinsamen Vorgangs.
- `causation_id` verweist auf das unmittelbar auslösende Ereignis oder Kommando.
- `subject` bezeichnet das primäre Fachobjekt; weitere IDs liegen im `payload`.
- `payload` bleibt fachlich typisiert, ist im Grundvertrag aber JSON-offen.
- MQTT-Topic, QoS, Retain und Brokerzustand gehören nicht in dieses Schema.

## Kompatibilität

Bestehende lokale Ereignislisten bleiben für die WebUIs erhalten. Die ersten migrierten Dienste liefern zusätzlich:

```http
GET /api/v1/events/netcore?limit=100
```

Die lokalen Ereignisdatensätze enthalten außerdem das Feld `canonical`, das dasselbe `netcore-event-v1`-Objekt enthält. Dadurch bleiben alte Darstellungen funktionsfähig, während neue Verbraucher bereits das gemeinsame Modell nutzen.

## Erste Produzenten

- Node Gateway
- Mobility Core
- Call Control
- SDS Router

## Heutige Nutzung

Der IoT Gateway bildet die Ereignisse auf MQTT-Topics ab. Command/Ack-Nachrichten besitzen eigene Verträge; sie sind keine beliebigen Ereignis-Payloads. Die folgenden Phasenabschnitte dokumentieren die eingeführten Domänen des aktuellen gemeinsamen Katalogs.


## Phase 8: Alarm- und Hardwareereignisse

Der gemeinsame Katalog umfasst nun zusätzlich `hardware.*`, `rf.*` und `alarm.*`. Der Alarm Workflow nutzt `correlation_id` für die Alarmakte, `causation_id` für das auslösende Ereignis und retained MQTT-Zustände ausschließlich als Transportabbildung außerhalb dieses Vertrags.

## Phase 9: Strukturierte Aufträge

Der Ereigniskatalog enthält zusätzlich `task.*`. Das primäre Fachobjekt verwendet `subject.type = "task"`; der aktuelle Auftrag wird separat als retained Zustand veröffentlicht.

## Phase 10: Asset- und Geräteverwaltung

Der Katalog enthält zusätzlich `asset.*`, `person.*`, `assignment.*` und `maintenance.*`. Subscriber Core und Mobility Core bleiben autoritativ für Netzfreigabe und Serving-TBS; der Asset-Dienst veröffentlicht lediglich seinen physischen Bestand, Zuordnungen und Wartungsvorgänge.


## Phase 11 – SIP Switch

Der gemeinsame Katalog enthält nun `sip.*`. Routingentscheidungen verwenden `subject.type = "sip_call"`; `payload.node_id`, `payload.endpoint`, `payload.issi` und `payload.reason` machen die Mobility-Core-Entscheidung nachvollziehbar. Der SIP Switch veröffentlicht keine SIP-Kennwörter oder vollständigen Asterisk-Konfigurationen als Ereignis.
