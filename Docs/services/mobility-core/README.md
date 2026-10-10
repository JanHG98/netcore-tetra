# Mobility Core

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../system-backend/mobility-core/src/state.rs) · [src/http.rs](../../../system-backend/mobility-core/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

## Zweck

Der Mobility Core ist der zentrale LXC-Dienst für Teilnehmerlage, Migrationen und MM-Context-Transfers zwischen mehreren NetCore-TBS.

## Aktueller Funktionsumfang

- Verbindung zum offenen Backend-WebSocket des Node Gateway
- automatische Erfassung verbundener TBS
- zentrale Teilnehmerlage aus Registration-, Group-, Energy-Saving- und RSSI-Telemetrie
- dreistufige Context-Transfer-Orchestrierung im Core für eine kompatible TBS:
  1. Export auf der Quell-TBS
  2. Import auf der Ziel-TBS
  3. Bereinigung auf der Quell-TBS
- eindeutige Transfer-IDs und Command-Korrelation
- Timeouts, Fehlerzustände und Abbruch vor abgeschlossenem Zielimport
- eigene REST-API, Metriken und OpenAPI-Beschreibung
- eigene Verwaltungs-WebUI

## WebUI

```text
http://<LXC-IP>:8090/
```

Die Oberfläche zeigt:

- Verbindung zum Node Gateway
- bekannte und aktive TBS
- Serving Node je Teilnehmer
- Gruppen, Energy-Saving-Mode und RSSI
- aktive und abgeschlossene Transfers
- Transferphasen und Fehler
- Ereignisprotokoll
- manuellen Transferstart und kontrollierten Abbruch

## Offener Testmodus

Diese Ausbaustufe arbeitet absichtlich ohne Tokens, Login, Passwörter, mTLS oder HTTPS.

```toml
[security]
mode = "open_lab"
allow_remote_management = true
```

Der Dienst darf nur in einem isolierten Testnetz betrieben werden. Andere Security-Modi werden beim Start abgewiesen.

## Fehlende aktive TBS-Anbindung

Die Kommandos `MobilityExportContext`, `MobilityImportContext` und `MobilityRemoveContext` werden vom [Worker](../../../crates/tetra-entities/src/net_control_room/worker.rs) an MM weitergereicht. Der aktive [MM-BS-Handler](../../../crates/tetra-entities/src/mm/mm_bs.rs) behandelt im Kontrollpfad nur `Dgna` und ignoriert diese Mobility-Kommandos als unsupported. Das separate Modul `mm/mobility_runtime.rs` ist deshalb kein Beleg für angeschlossenen Ende-zu-Ende-Kontexttransfer. Die nachfolgend beschriebene Core-Orchestrierung und Mock-Bestätigung sind von diesem fehlenden aktiven TBS-Pfad zu unterscheiden.

## Architekturgrenze

Der Mobility Core koordiniert den zentralen MM-Kontext. Die zeitkritischen Air-Interface-Verfahren, MLE-Zellwechsel und CMCE-Call-Restore-State-Machines bleiben in der jeweiligen TBS.

Teilnehmerlage, Transfer-History und Ereignisse werden aktuell nur im Speicher gehalten. Nach Neustart rekonstruiert der Dienst beobachtbare Teilnehmer aus TBS-Telemetrie; laufende Transfers sind dadurch nicht persistent wiederaufnehmbar. Persistenz, gegenseitige Dienstauthentisierung, TLS, RBAC und Audit-Signaturen bleiben offen.

## Kanonischer Teilnehmer-Routenresolver

Der implementierte Resolver stellt `GET /api/v1/subscribers/{issi}/route` bereit. Die Antwort enthält Serving-TBS, Location Area, Registrierungszustand, TBS-Verbindungszustand und eine monotone `route_generation`. Call Control verwendet diese Route für Individualrufe.

## Gemeinsames Ereignismodell (MQTT Phase 2)

Der Dienst behält `GET /api/v1/events` für die bestehende WebUI bei. Jeder lokale Datensatz enthält zusätzlich `canonical`. Für neue Verbraucher steht ausschließlich das gemeinsame Format unter `GET /api/v1/events/netcore?limit=100` bereit. Das Wire-Schema ist `netcore-event-v1`; MQTT-Topics, QoS und Retain-Regeln werden vom vorhandenen IoT Gateway verarbeitet; diese HTTP-Ereignis-API veröffentlicht selbst keine MQTT-Nachrichten.
