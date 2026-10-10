# Transit – Grenze zu ETSI ISI

`netcore-transit-v1` ist das NetCore-interne regionale Envelope-Protokoll und **noch kein ETSI ISI**. Die vorhandenen Servicebezeichnungen `mobility`, `individual_call`, `group_call`, `sds`, `media` und `supplementary_service` beschreiben semantische Routingkategorien; sie implementieren keine jeweiligen ETSI-Stage-3-PDUs.

## Vorhanden und geplant

| Bereich | Heutiger Stand |
| --- | --- |
| Regionales Routing, Queues, TTL, Dedupe, Failover | Im Transit-Dienst vorhanden |
| HTTP-Peer-Transport | NetCore-nativ, ungeschütztes Open Lab |
| Fremde SwMI-Anbindung | Kein interoperabler ISI-Adapter im Transit-Paket |
| ISI-Signalisierung, Transport-/Sicherheitsprofile | Eigener geplanter Interworking-Ausbau |
| Standardisierte ISI-Media- und Supplementary-Service-Profile | Nicht durch die native Payload-Weiterleitung abgedeckt |

Die geplante Adaptergrenze soll standardisierte Fremdnetz-Signalisierung in das interne Ereignismodell übersetzen. Diese Beschreibung ist Architekturabsicht, kein bereits implementierter Adaptervertrag. Eine konkrete ISI-Anbindung benötigt ihre eigenen Codec-, Transport-, Sicherheits- und Interoperabilitätsnachweise.

Eine erfolgreiche Übergabe zwischen zwei NetCore-Labregionen belegt deshalb nur den nativen Transitpfad. Sie weist weder fremde SwMI-Kompatibilität noch standardkonformes Funkverhalten nach.

**Quellabgleich vom 9. Oktober 2026:** [Native Nachrichtenfelder](../../../system-backend/transit/src/protocol.rs), [Ingress-Prüfung](../../../system-backend/transit/src/state.rs) und [HTTP-Transport](../../../system-backend/transit/src/transport.rs).
