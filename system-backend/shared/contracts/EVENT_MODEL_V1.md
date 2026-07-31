# NetCore Event Model v1

## Zweck

`netcore-event-v1` ist das gemeinsame, transportneutrale Ereignisformat für Backend-Dienste. Es wird bereits vor dem MQTT-IoT-Gateway eingeführt, damit MQTT später nur noch Ereignisse transportiert und nicht erst deren Bedeutung erfinden muss.

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

## Spätere Nutzung

Der IoT Gateway wird diese Ereignisse in Phase 3 auf MQTT-Topics abbilden. Command/Ack-Nachrichten erhalten in Phase 4 eigene Verträge und werden nicht als beliebige Ereignis-Payloads missbraucht.
