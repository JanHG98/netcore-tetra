# Dienstkatalog

Die [aktuelle Open-Lab-Inventoryvorlage](https://github.com/JanHG98/netcore-tetra/blob/main/deploy/open-lab/inventory.example.toml) listet **24 deploybare Backend-Dienste**. Die Tabelle nennt jeweils den fachlichen Zweck und den TCP-Beispielport für Management/API/WebUI. Die reale Installation kann andere Hosts oder Ports verwenden. Installationsdateien, Endpunkte und Tests liegen unter dem verlinkten Dienstordner.

| Dienst | Aufgabe und Datenhoheit | Port | Quellort |
|---|---|---:|---|
| Node Gateway | TBS-/Backend-Sessions, Heartbeats, Service-Matrix | 8080 | [node-gateway](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/node-gateway) |
| Mobility Core | Zell- und Teilnehmerstandort, Serving-TBS, Kontextwechsel | 8090 | [mobility-core](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/mobility-core) |
| Subscriber Core | Profile, Gerätefreigabe und Teilnehmerregeln | 8100 | [subscriber-core](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/subscriber-core) |
| Group Core | GSSI, Mitgliedschaft, Affiliation und DGNA-Policy | 8110 | [group-core](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/group-core) |
| Call Control | Rufzweige, Floor, Priorität und Rufzustand | 8120 | [call-control](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/call-control) |
| Media Switch | codierte Sprachframes und Medienrouting | 8130 | [media-switch](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/media-switch) |
| Recorder | zentrale Aufnahmen, Suche und Aufbewahrung | 8140 | [recorder](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/recorder) |
| SDS Router | SDS-Routing, Warteschlangen und Zustellberichte | 8150 | [sds-router](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/sds-router) |
| Packet Core | PDP-/SNDCP-Kontexte, NSAPI und Datenfluss | 8160 | [packet-core](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/packet-core) |
| IP Gateway | TUN, IP-Leases, NAT, Firewall, DNS, WAP | 8170 | [ip-gateway](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/ip-gateway) |
| Security Core | Authentisierung, Security-Policy und Audit | 8180 | [security-core](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/security-core) |
| KMF | Schlüssel-Lebenszyklus, Rotation und OTAR-Grenze | 8190 | [kmf](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/kmf) |
| Transit | Regionen, Peers, Routen und Failover | 8200 | [transit](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/transit) |
| Observability | Metriken, Logs, Traces und Alarme | 8210 | [observability](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/observability) |
| Application Gateway | Connectoren, Webhooks und Zustellung | 8220 | [application-gateway](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/application-gateway) |
| Media Library | Audio-Assets, TTS-Import, Vorschau und Playout-Jobs | 8230 | [media-library](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/media-library) |
| IoT Gateway | MQTT, HA-Discovery, Homematic und Command-Ledger | 8240 | [iot-gateway](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/iot-gateway) |
| Hardware Gateway | Edge-I/O, Rack-Sensoren und physische Ausgänge | 8250 | [hardware-gateway](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/hardware-gateway) |
| RF Monitor | HF-Telemetrie, Temperaturen und Grenzwerte | 8260 | [rf-monitor](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/rf-monitor) |
| Alarm Workflow | Alarmzustand, Eskalation und Quittierung | 8270 | [alarm-workflow](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/alarm-workflow) |
| Task Workflow | Aufträge, Zuweisungen, SDS/WAP-Aktionen | 8280 | [task-workflow](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/task-workflow) |
| Asset Management | Funkgeräte, Ausgabe, Rückgabe, Wartungsakte | 8290 | [asset-management](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/asset-management) |
| SIP Switch | PBX-Trunk und Routing zur aktuellen TBS | 8300 | [sip-switch](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/sip-switch) |
| Control Room | Operatoren, Arbeitsplatzansicht, Befehle | 9010 | [control-room](https://github.com/JanHG98/netcore-tetra/tree/main/system-backend/control-room) |

## Daneben betreibbare Komponenten

| Komponente | Rolle | Einordnung |
|---|---|---|
| `bluestation-bs` | Funkkante mit eigenem Dashboard | kein LXC-Eintrag des 24er-Inventories |
| Provisioning Core | Teilnehmer/Gruppen per Subscriber/Group Core anlegen | zusätzlicher Dienst, Beispielport 8125; [[Provisioning]] |
| NetCore Directory | Namen, Bezeichnungen, Statusgruppen und lokale Metadaten | Python/SQLite, Beispielport 8095; [[NetCore-Directory]] |
| NetCore Piper | TTS-WAV-Erzeugung | separater HTTP-Dienst, Beispielport 5005; [[Audio-Zentrale]] |
| lokaler TBS-Asterisk und vorhandene PBX | SIP-/RTP-Edge und Telefonanlage | Rollen getrennt vom zentralen SIP Switch; [[SIP-und-Brew]] |
| Brew-Server | optionaler externer TETRA/Brew-Peer | eigene Konfiguration und Ruf-/SDS-Grenze; [[SIP-und-Brew]] |

**24 Einträge im Inventory sind keine 24 auf einem Host laufenden Prozesse.** Die Vorlage sieht einen Dienst pro LXC vor. [`system-backend/services.toml`](https://github.com/JanHG98/netcore-tetra/blob/main/system-backend/services.toml) führt darüber hinaus Provisioning Core und zeigt Ziel-/Kontraktinformationen; für die konkrete Bereitstellung zählen das gerenderte Inventory und die installierten TOML-Dateien. Die ältere [17-Dienst-Anleitung](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/NetCore-Tetra-Komplettguide.md) ist eine ältere Ausbauphase.

Alle Dienste stellen im Open-Lab-Paket Fach-WebUIs und APIs bereit; die [WebUI-Matrix](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/BACKEND_WEBUI_SERVICE_MATRIX.md) beschreibt Verwaltungsaktionen. [[Bedienoberflaechen]] trennt die Oberflächen, [[Netzwerk-und-Ports]] die Transportschicht und [[Security-and-Operations]] die Netzgrenze.
