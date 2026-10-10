# Kontext-State-Machine

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../system-backend/packet-core/src/state.rs) · [src/http.rs](../../../system-backend/packet-core/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

| Auslöser | Implementierter Zustand / Wirkung |
|---|---|
| `activate_demand` in `shadow` | protokolliert Nachfrage; erzeugt keinen neuen zentralen Kontext |
| `activate_demand` in `authoritative` | akzeptierter Kontext direkt `standby`; Referenzaktion `activate_accept`, bei Kontextgrenze `activate_reject` |
| `context_activated` | Kontext wird als `standby` gespiegelt |
| `data_transmit_request` für vorhandene NSAPI | `ready`; READY- und Kontext-Timer werden gesetzt |
| Downlink bei `standby` / `quiescent`, manuelles Wake | `response_waiting`; Page/Wake-Aktion und Response-Timer |
| Response-Timer abgelaufen | `standby` mit Fehlerhinweis |
| READY-Timer abgelaufen | `standby`; in Authoritative End-of-Data-Aktion |
| Kontext-Timer in READY abgelaufen | `quiescent`, soweit die READY-Grenze nicht bereits vorher greift |
| Modify `availability=false`, Operator-Suspend | `suspended` |
| Modify `usage_active=false` | `quiescent` |
| Reconnect mit bekannten Kontexten und `data_to_send=true` | `ready` |
| Standby-Timer für Standby/Suspended/Quiescent abgelaufen | Kontext entfernt, Authoritative kann Deactivate-Aktion erzeugen |
| Node-Verlust bei `preserve_context_on_node_loss=true` | betroffene Kontexte `suspended`, nicht verfügbar |

`activating`, `deactivating` und `failed` sind zusätzliche Modellzustände; sie bilden keinen zwingenden Ablauf jedes Aktivierungsversuchs. API-Zustandsnamen werden als `snake_case` serialisiert.

Timer werden als absolute RFC-3339-Deadlines persistiert. Nach einem Neustart läuft die Auswertung weiter; der Dienst fällt also nicht durch einen Neustart in ein fiktives READY zurück.
