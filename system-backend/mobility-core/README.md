# Mobility Core

## Zweck

Der Mobility Core ist der zentrale LXC-Dienst für Teilnehmerlage, Migrationen und MM-Context-Transfers zwischen mehreren NetCore-TBS.

## Aktueller Funktionsumfang

- Verbindung zum offenen Backend-WebSocket des Node Gateway
- automatische Erfassung verbundener TBS
- zentrale Teilnehmerlage aus Registration-, Group-, Energy-Saving- und RSSI-Telemetrie
- dreistufiger Context Transfer:
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

## Architekturgrenze

Der Mobility Core koordiniert den zentralen MM-Kontext. Die zeitkritischen Air-Interface-Verfahren, MLE-Zellwechsel und CMCE-Call-Restore-State-Machines bleiben in der jeweiligen TBS.

Eine spätere produktive Ausbaustufe ergänzt persistente Datenhaltung, gegenseitige Dienstauthentisierung, TLS, RBAC und Audit-Signaturen.

## Kanonischer Teilnehmer-Routenresolver

Phase 1 stellt `GET /api/v1/subscribers/{issi}/route` bereit. Die Antwort enthält Serving-TBS, Location Area, Registrierungszustand, TBS-Verbindungszustand und eine monotone `route_generation`. Call Control verwendet diese Route für Individualrufe.

## Gemeinsames Ereignismodell (MQTT Phase 2)

Der Dienst behält `GET /api/v1/events` für die bestehende WebUI bei. Jeder lokale Datensatz enthält zusätzlich `canonical`. Für neue Verbraucher steht ausschließlich das gemeinsame Format unter `GET /api/v1/events/netcore?limit=100` bereit. Das Wire-Schema ist `netcore-event-v1`; MQTT-spezifische Topic-, QoS- und Retain-Regeln folgen erst im IoT Gateway.
