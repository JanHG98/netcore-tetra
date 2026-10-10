# Transit – Architektur und Zustellung

Jede Region besitzt eine Transit-Instanz. Lokale Core-Dienste geben semantische Ereignisse an deren Submit-API ab; Transit bestimmt die Zielregionen und erzeugt pro Zielregion ein Session-Leg. Der eigene Peer-Transport übergibt native Envelopes an die nächste Region. Das ist **noch kein ETSI ISI**.

| Grenze | Verhalten |
| --- | --- |
| Lokaler Core → Transit | `POST /api/v1/transit/submit`, Routingentscheidung und persistente Outbound-Queue |
| Transit → Peer | HTTP-Heartbeat und `netcore-transit-v1`-Envelope; Retry/Failover bei Fehlern |
| Peer → Transit | Protokoll-, TTL-, Hop-, Trace- und Dedupe-Prüfung |
| Transit → lokaler Core | Persistente Local Delivery, anschließend expliziter Consumer-ACK |

## Zustellung und Persistenz

Envelopes tragen Ursprung, vorherigen Hop, Zielregion, Adressen, Service/Operation, Session-/Korrelations-ID, Priorität, TTL, Trace und Payload. Die Annahme durch einen Peer bestätigt lediglich dessen Transit-Ingress. Die lokale Anwendung erfolgt getrennt über die Local-Delivery-API.

`storage.database_path` speichert Peers, Routen, Teilnehmer-/Gruppenregionen, Sessions, Queues, Dedupe-Einträge und Ereignisse. Ein Export oder Metadatenbackup enthält entsprechend auch die gespeicherten Payloads und ist keine anonymisierte Betriebsübersicht.

## Hintergrundarbeit und Modi

Der Transportworker prüft die Wartung etwa alle fünf Sekunden und sendet im authoritative-Modus Heartbeats beziehungsweise gequeuete Envelopes. Die Beispielwerte sind fünf Sekunden Heartbeat-Intervall, 20 Sekunden Peer-Timeout und drei Sekunden Retry-Backoff.

`shadow` verhindert ausgehenden Peer-Transport; er deaktiviert keine erreichbaren Ingress- oder Managementendpunkte. Ein Policywechsel ist bei Transit keine Laufzeit-API-Aktion, sondern eine Konfigurationsänderung mit Neustart.

**Quellabgleich vom 9. Oktober 2026:** [state.rs](../../../system-backend/transit/src/state.rs), [transport.rs](../../../system-backend/transit/src/transport.rs) und [http.rs](../../../system-backend/transit/src/http.rs). Weiter: [Routing](routing-und-failover.md).
