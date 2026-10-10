# Zentraler Context Transfer

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../system-backend/mobility-core/src/state.rs) · [src/http.rs](../../../system-backend/mobility-core/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

**Integrationsgrenze:** Der Core besitzt den unten beschriebenen Kommandofluss. Der aktive [TBS-MM-Handler](../../../crates/tetra-entities/src/mm/mm_bs.rs) verarbeitet diese Kommandos noch nicht; der [Worker](../../../crates/tetra-entities/src/net_control_room/worker.rs) routet sie lediglich an MM. Ein Mock-TBS-ACK oder das separate `mobility_runtime.rs` belegt keine aktive Übernahme am Funkstack.

Im Core-Vertrag läuft ein Transfer mit einer kompatiblen TBS in drei bestätigten Schritten:

1. `MobilityExportContext` auf der Quell-TBS,
2. `MobilityImportContext` auf der Ziel-TBS,
3. `MobilityRemoveContext` auf der Quell-TBS.

Die Quelle wird erst entfernt, nachdem die Ziel-TBS den Import bestätigt hat. Scheitert der Export oder Import, bleibt der ursprüngliche Kontext erhalten. Scheitert nur die abschließende Quellbereinigung, wird der Transfer als Fehler markiert und in der WebUI sichtbar.

Das Transfermodell enthält folgende MM-Daten:

- Home-ISSI,
- Registrierungszustand,
- Gruppen,
- Energy-Saving-Mode und Monitoring Window,
- Class of MS,
- letzter Layer-2-Handle,
- TEI.

Aktive CMCE-Calls werden weiterhin durch die bereits vorhandene Call-Restore-State-Machine behandelt; der Mobility Core transportiert in diesem Paket ausschließlich den MM-Teilnehmerkontext.

## Abbruch und Neustart

Der API-Abbruch ist vor bestätigtem Zielimport möglich. Sobald die Quellbereinigung eingereiht oder angefordert ist, lehnt `cancel` den Abbruch ab; bereits versandte Kommandos werden nicht rückwirkend zurückgenommen. `server.transfer_timeout_secs` begrenzt die Bearbeitung, im Beispiel auf 45 Sekunden. Transferzustände sind In-Memory-Daten und überleben keinen Dienstneustart. Vor einem erneuten manuellen Transfer deshalb den tatsächlich vorhandenen Kontext beider TBS prüfen.
