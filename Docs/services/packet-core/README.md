# NetCore Packet Core

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../system-backend/packet-core/src/state.rs) · [src/http.rs](../../../system-backend/packet-core/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Der Packet Core ist der zentrale SwMI-Dienst für **SNDCP-Kontexte, Packet-Data-Zustände und Mobility Anchoring**. Er übernimmt die langlebige Netzsicht oberhalb der lokalen TBS-SNDCP-Instanz; PHY, MAC, LLC, lokale PDCH-Zuteilung und die konkrete Air-PDU bleiben bewusst an der TBS.

## Aktueller Umfang

- PDP-Kontexte und NSAPI 1 bis 14
- primäre und sekundäre Kontexte
- READY, STANDBY, RESPONSE-WAITING, SUSPENDED und Deactivation
- Data Transmit Request/Response als zentrale Zustandsereignisse
- Reconnect, Modify, End of Data und Deactivation
- dynamische IPv4-Adressvergabe mit stabiler Wiederverwendung
- Mobility Anchor und Umzug eines Kontextes zwischen TBS
- Packet Priority und Flow-Control-Grenzen
- Downlink-Queue mit Paket-, Byte- und TTL-Limits
- Fragmentierung sowie überlappungssichere Reassembly
- PDCH-/Bearer-Sicht aus TBS-Telemetrie
- Node-Gateway-Steuerung für Deactivate, Modify, Wake/Page und End of Data
- persistente JSON-Datenbank mit Backup
- REST-API, OpenAPI, Prometheus-Metriken, Health und eigene WebUI
- systemd-Unit sowie Install-, Update- und Uninstall-Skripte

## Shadow und Authoritative

Standard ist bewusst:

```toml
[packet]
mode = "shadow"
```

Dabei bleibt die lokale TBS entscheidend. Der Packet Core importiert Telemetrie, führt seine zentrale State Machine parallel und macht Abweichungen sichtbar. Für gezielte Labortests kann auf `authoritative` umgestellt werden; dann beantwortet der Dienst das versionierte Edge-Protokoll `netcore-packet-edge-v1` mit konkreten Aktionen.

## Saubere Edge/Core-Grenze

Die TBS behält:

- SNDCP-PDU-Encoding und -Decoding,
- MLE-/LLC-Primitiven,
- lokale TDMA-Zeit,
- PDCH-Zuteilung und Timeslot-Freigabe,
- Air-Interface-nahe Retry- und Response-Cache-Logik,
- lokale Packet-Gateway-Funktion bis zur späteren Migration.

Der Packet Core hält:

- netzweite PDP-/NSAPI-Sicht,
- Timer- und Kontextzustände,
- Adress- und Anchor-Zuordnung,
- Prioritäts-/Flow-Control-Policy,
- TBS-übergreifende Recovery- und Mobility-Entscheidungen.

## Noch ausdrücklich nicht enthalten

TUN, Routing, NAT, Firewall, DNS, WAP-Testserver und Packet Capture liegen im **bereits vorhandenen separaten Dienst `ip-gateway`**. Der Packet Core erzeugt deshalb keine künstliche Layer-2-Domäne und behauptet auch nicht, bereits der zentrale Internet-Gateway zu sein.

## Open-Lab-Betrieb

Der aktuelle Dienst läuft absichtlich ohne Benutzerkonten, Token und TLS. Jeder Client mit Netzzugriff kann Kontexte lesen, Zustände ändern, Teilnehmer pagen, Kontexte deaktivieren und N-PDUs einspeisen. Nur in einem isolierten Testnetz einsetzen.

Standardzugriff:

```text
http://<LXC-IP>:8160/
```

## Schnellstart

```bash
cargo run -p netcore-packet-core -- --no-config --bind 0.0.0.0:8160
```

Mit Konfiguration:

```bash
sudo install -d /etc/netcore
sudo cp system-backend/packet-core/config/packet-core.example.toml /etc/netcore/packet-core.toml
cargo run -p netcore-packet-core -- --config /etc/netcore/packet-core.toml
```

## API-Auswahl

```text
GET    /api/v1/status
GET    /api/v1/nodes
GET    /api/v1/contexts
GET    /api/v1/contexts/{id}
POST   /api/v1/contexts/{id}/wake
POST   /api/v1/contexts/{id}/end-of-data
POST   /api/v1/contexts/{id}/modify
POST   /api/v1/contexts/{id}/deactivate
GET    /api/v1/bearers
GET    /api/v1/actions
POST   /api/v1/actions/{id}/ack
POST   /api/v1/edge/events
POST   /api/v1/downlink
GET    /api/v1/reassemblies
GET    /api/v1/npdu-outbox
DELETE /api/v1/npdu-outbox/{id}
GET    /api/v1/events
GET    /api/v1/export.json
GET    /metrics
GET    /health/live
GET    /health/ready
GET    /openapi.json
```

Die Themenanleitungen liegen neben diesem Überblick; Start und Netzkonfiguration stehen in [Bereitstellung im LXC](bereitstellung-im-lxc.md), die API-Aufrufe in [Laborbeispiele](tests/api-beispiele-im-labor.md).

## Aktuelle Grenze des zentralen Datenpfads

Das HTTP-Edge-Protokoll ist eine implementierte Referenzschnittstelle. Die Node-Gateway-Bridge bildet derzeit nur Deactivate, Modify, Page/Wake und End of Data auf TBS-Kommandos ab; Aktivierungsantworten und N-PDU-Fragmente werden nicht über diese Bridge vollständig zum Funkstack transportiert. Ein Wechsel auf `authoritative` allein ersetzt daher noch nicht die lokale TBS-Paketdatenanbindung. Den vollständigen Weg von Funkgerät über Packet Core und IP Gateway erst durch einen eigenen Ende-zu-Ende-Labortest belegen.

Außerdem verwenden die Beispielkonfigurationen unterschiedliche Pools: Packet Core `10.44.0.0/24`, IP Gateway `10.0.0.0/24`. Vor Kopplung Adresspool, TUN-Netz/Gateway, DNS-Bind und Firewall-CIDRs konsistent setzen.

## Arbeitsverzeichnis

Alle `cargo`- und `system-backend/...`-Befehle in diesem Überblick werden im Repository-Root ausgeführt, beispielsweise nach `cd /opt/netcore-tetra`. Direkte Starts mit `/etc/netcore/...` benötigen passende Schreibrechte auf die konfigurierten State-Verzeichnisse; für den dauerhaften LXC-Betrieb den Installer und dessen Benutzer `netcore` verwenden.
