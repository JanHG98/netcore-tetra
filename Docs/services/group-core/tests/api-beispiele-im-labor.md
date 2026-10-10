# API-Beispiele im offenen Labor

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../../system-backend/group-core/src/state.rs) · [src/http.rs](../../../../system-backend/group-core/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Diese Aufrufe sind eine Bedien- und Prüfanleitung für den Group Core, kein Protokoll bereits ausgeführter Tests. Der Dienst muss laufen; `127.0.0.1:8110` funktioniert nur bei einem Listener auf Loopback oder allen Interfaces. Nach LXC-Installation die tatsächliche Adresse aus `/etc/netcore/lxc-network.env` verwenden. Platzhalter für Node-, Ruf- und Aufnahme-IDs durch vorhandene IDs aus den lesenden API-Antworten ersetzen. Schreibende Beispiele verändern den Laborzustand.

```bash
curl http://GROUP-CORE:8110/api/v1/status
curl -X POST http://GROUP-CORE:8110/api/v1/groups -H 'Content-Type: application/json' -d '{"gssi":15501,"name":"Status","enabled":true,"attach_allowed":true,"dgna_allowed":true,"call_allowed":false,"sds_allowed":true,"emergency_allowed":false,"call_priority":0,"class_of_usage":4,"area_nodes":[],"notes":"keine Sprache"}'
curl -X POST http://GROUP-CORE:8110/api/v1/memberships -H 'Content-Type: application/json' -d '{"issi":1234,"gssi":15501,"allowed":true,"auto_attach":true,"locked":false,"notes":""}'
```

Die Gruppe im Beispiel erlaubt SDS, aber keinen Sprach- oder Notruf. Nach POST die Policy-Synchronisation in `/api/v1/syncs` prüfen. Die gespeicherte SDS-Freigabe ist in diesem Quellstand keine gemeinsame Zugriffssperre des SDS Router.
