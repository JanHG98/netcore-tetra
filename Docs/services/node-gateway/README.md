# Node Gateway

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../system-backend/node-gateway/src/state.rs) · [src/http.rs](../../../system-backend/node-gateway/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

## Status

Zentraler Transportdienst für die NetCore-Basisstationen und Backend-Dienste. Eine systemd-Unit, Installationsskripte und die integrierte WebUI sind vorhanden; der erfolgreiche Start eines einzelnen Dienstes ist keine Gesamtabnahme des Netzes.

## Zweck

Der Node Gateway ist der zentrale Einstiegspunkt für NetCore-TBS-Instanzen. Er nimmt die bestehenden TBS-WebSocket-Verbindungen an, verwaltet Sessions und vermittelt Telemetrie und Kommandos für die vorhandenen zentralen Backend-Dienste.

## Umgesetzt

- kompatibler TBS-WebSocket unter `/ws/node`
- Aushandlung von `netcore-control-room-node-v1`
- Hello-, Heartbeat-, Telemetrie-, ACK-, Response- und Error-Verarbeitung
- Duplicate-Node-Erkennung mit kontrolliertem Austausch der alten Session
- Hello-Timeout und Nachrichtengrößenlimits
- In-Memory-Node- und Ereigniszustand
- Kommandotransport vom API-/Backend-Pfad zur TBS
- Backend-WebSocket unter `/ws/backend`
- REST-API unter `/api/v1`
- Prometheus-Metriken
- OpenAPI-Beschreibung
- eigene integrierte WebUI
- systemd-Unit und Installationsskripte

## Offener Testmodus

Diese Version verwendet ausdrücklich **keine Tokens**. Ebenso gibt es noch keine Benutzer, Passwörter, Zertifikate oder TLS-Verschlüsselung.

```toml
[security]
mode = "open_lab"
allow_remote_management = true
```

Andere Security-Modi werden abgewiesen, statt nicht vorhandene Sicherheit vorzutäuschen. Der LXC darf daher ausschließlich in einem isolierten Testnetz betrieben werden.

## Endpunkte

| Endpunkt | Funktion |
|---|---|
| `GET /` | Verwaltungs-WebUI |
| `WS /ws/node` | TBS-Verbindungen |
| `WS /ws/backend` | Mobility-, Call-, SDS- und Medien-Backend-Transport |
| `GET /api/v1/core-services` | überwachte Backend-Dienste und Ausfallzustände |
| `GET /api/v1/status` | Gateway-Übersicht |
| `GET /api/v1/nodes` | Nodes |
| `GET /api/v1/nodes/{id}` | Node-Details |
| `POST /api/v1/nodes/{id}/ping` | Application Ping |
| `POST /api/v1/nodes/{id}/disconnect` | Node trennen |
| `POST /api/v1/nodes/{id}/commands` | typisiertes TBS-Kommando |
| `GET /api/v1/events` | Ereignis-History |
| `GET /metrics` | Prometheus |
| `GET /openapi.json` | API-Beschreibung |
| `GET /health/live` | Liveness |
| `GET /health/ready` | Readiness |

## WebUI

Die WebUI zeigt:

- verbundene, getrennte und stale Nodes
- Stations-, Zell- und Carrierdaten
- Stackversion und Capabilities
- Heartbeat-, Telemetrie- und Response-Zähler
- letzte Gateway-Ereignisse
- Ping- und Disconnect-Aktionen
- gut sichtbare Warnung zum offenen Testmodus

## Build

```bash
cargo build --release -p netcore-node-gateway
```

## Start im Repo

```bash
target/release/netcore-node-gateway \
  --config system-backend/node-gateway/config/node-gateway.example.toml
```

## Grenzen dieses Pakets

- keine persistente Datenbank
- keine fachliche Teilnehmer-, Gruppen-, Mobility- oder Ruflogik
- kein Media-Transport
- noch keine abgesicherte Produktivbetriebsart
- der TBS-Transport nutzt weiterhin die vorhandenen Datentypen aus `crates/tetra-entities/src/net_control_room/protocol.rs`; gemeinsame Edge-Verträge liegen zusätzlich unter `system-backend/shared/contracts`, ersetzen diese Laufzeitanbindung jedoch nicht automatisch

## Gemeinsames Ereignismodell (MQTT Phase 2)

Der Dienst behält `GET /api/v1/events` für die bestehende WebUI bei. Jeder lokale Datensatz enthält zusätzlich `canonical`. Für neue Verbraucher steht ausschließlich das gemeinsame Format unter `GET /api/v1/events/netcore?limit=100` bereit. Das Wire-Schema ist `netcore-event-v1`; MQTT-Topics, QoS und Retain-Regeln werden vom vorhandenen IoT Gateway verarbeitet; diese HTTP-Ereignis-API veröffentlicht selbst keine MQTT-Nachrichten.

## Überwachung zentraler Dienste

Der Abschnitt `[service_monitor]` definiert Health-Ziele, Prüfintervall und Fehler-/Erholungsschwellen. Der Gateway verteilt die daraus berechnete Dienstzustandsmatrix an die TBS. Die Beispieladressen `10.0.20.*` müssen zur tatsächlichen LXC-Belegung passen. Der Zustand eines Dienstes wird getrennt von der allgemeinen Internetverbindung bewertet; Fallback-Beschreibungen ersetzen keine Prüfung der jeweiligen TBS-Funktion.

## Arbeitsverzeichnis

Alle `cargo`- und `system-backend/...`-Befehle in diesem Überblick werden im Repository-Root ausgeführt, beispielsweise nach `cd /opt/netcore-tetra`. Direkte Starts mit `/etc/netcore/...` benötigen passende Schreibrechte auf die konfigurierten State-Verzeichnisse; für den dauerhaften LXC-Betrieb den Installer und dessen Benutzer `netcore` verwenden.
