# Dienstkatalog

Die [aktuelle Open-Lab-Inventoryvorlage](../../deploy/open-lab/inventory.example.toml) listet **26 Backend-Rollen auf `main`**. Z01.1–Z01.3 sind nach der dokumentierten Quell-/CI-Abnahme integriert; die konkreten Anlagenprüfungen laufen unter Z01.4. [Aktuelle Gesamtroadmap](../roadmaps/gesamtroadmap.md). Die Tabelle nennt jeweils den fachlichen Zweck und den TCP-Beispielport für Management/API/WebUI. Die reale Installation kann andere Hosts oder Ports verwenden. Installationsdateien, Endpunkte und Tests liegen unter dem verlinkten Dienstordner.

| Dienst | Aufgabe und Datenhoheit | Port | Quellort |
|---|---|---:|---|
| Node Gateway | TBS-/Backend-Sessions, Heartbeats, Service-Matrix | 8080 | [node-gateway](../../system-backend/node-gateway) |
| Mobility Core | Zell- und Teilnehmerstandort, Serving-TBS, Kontextwechsel | 8090 | [mobility-core](../../system-backend/mobility-core) |
| Subscriber Core | Profile, Gerätefreigabe und Teilnehmerregeln | 8100 | [subscriber-core](../../system-backend/subscriber-core) |
| Group Core | GSSI, Mitgliedschaft, Affiliation und DGNA-Policy | 8110 | [group-core](../../system-backend/group-core) |
| Call Control | Rufzweige, Floor, Priorität und Rufzustand | 8120 | [call-control](../../system-backend/call-control) |
| Media Switch | codierte Sprachframes und Medienrouting | 8130 | [media-switch](../../system-backend/media-switch) |
| Recorder | zentrale Aufnahmen, Suche und Aufbewahrung | 8140 | [recorder](../../system-backend/recorder) |
| SDS Router | SDS-Routing, Warteschlangen und Zustellberichte | 8150 | [sds-router](../../system-backend/sds-router) |
| Packet Core | PDP-/SNDCP-Kontexte, NSAPI und Datenfluss | 8160 | [packet-core](../../system-backend/packet-core) |
| IP Gateway | TUN, IP-Leases, NAT, Firewall, DNS, WAP | 8170 | [ip-gateway](../../system-backend/ip-gateway) |
| Security Core | Authentisierung, Security-Policy und Audit | 8180 | [security-core](../../system-backend/security-core) |
| KMF | Schlüssel-Lebenszyklus, Rotation und OTAR-Grenze | 8190 | [kmf](../../system-backend/kmf) |
| Transit | Regionen, Peers, Routen und Failover | 8200 | [transit](../../system-backend/transit) |
| Observability | Metriken, Logs, Traces und Alarme | 8210 | [observability](../../system-backend/observability) |
| Application Gateway | Connectoren, Webhooks und Zustellung | 8220 | [application-gateway](../../system-backend/application-gateway) |
| Media Library | Audio-Assets, TTS-Import, Vorschau und Playout-Jobs | 8230 | [media-library](../../system-backend/media-library) |
| IoT Gateway | MQTT, HA-Discovery, Homematic und Command-Ledger | 8240 | [iot-gateway](../../system-backend/iot-gateway) |
| Hardware Gateway | Edge-I/O, Rack-Sensoren und physische Ausgänge | 8250 | [hardware-gateway](../../system-backend/hardware-gateway) |
| RF Monitor | HF-Telemetrie, Temperaturen und Grenzwerte | 8260 | [rf-monitor](../../system-backend/rf-monitor) |
| Alarm Workflow | Alarmzustand, Eskalation und Quittierung | 8270 | [alarm-workflow](../../system-backend/alarm-workflow) |
| Task Workflow | Aufträge, Zuweisungen, SDS/WAP-Aktionen | 8280 | [task-workflow](../../system-backend/task-workflow) |
| Asset Management | Funkgeräte, Ausgabe, Rückgabe, Wartungsakte | 8290 | [asset-management](../../system-backend/asset-management) |
| SIP Switch | PBX-Trunk und Routing zur aktuellen TBS | 8300 | [sip-switch](../../system-backend/sip-switch) |
| Control Room | Operatoren, Arbeitsplatzansicht, Befehle | 9010 | [control-room](../../system-backend/control-room) |
| Alert Service | Warnquellen, Meldungen und kontrollierte Zustellung; Token-Verwaltung | 8310 | [alert-service](../../system-backend/alert-service) |
| Deployment Core | Discovery, Rollout und Imagebuilder auf voller Ubuntu-VM | 8320 | [deployment-core](../../system-backend/deployment-core) |

## Daneben betreibbare Komponenten

| Komponente | Rolle | Einordnung |
|---|---|---|
| `bluestation-bs` | Funkkante mit eigenem Dashboard | kein Backend-Eintrag des 26er-Inventories |
| Provisioning Core | Teilnehmer/Gruppen per Subscriber/Group Core anlegen | zusätzlicher Dienst, Beispielport 8125; [Provisioning und Datenhoheit](teilnehmer-und-gruppen-anlegen.md) |
| NetCore Directory | Namen, Bezeichnungen, Statusgruppen und lokale Metadaten | Python/SQLite, Beispielport 8095; [Namen und Metadaten im NetCore Directory](namen-und-metadaten-im-directory.md) |
| NetCore Piper | TTS-WAV-Erzeugung | separater HTTP-Dienst, Beispielport 5005; [Audio-Zentrale](audio-aufnahmen-und-tts.md) |
| lokaler TBS-Asterisk und vorhandene PBX | SIP-/RTP-Edge und Telefonanlage | Rollen getrennt vom zentralen SIP Switch; [SIP, lokaler Asterisk und Brew](telefonie-sip-und-brew.md) |
| Brew-Server | optionaler externer TETRA/Brew-Peer | eigene Konfiguration und Ruf-/SDS-Grenze; [SIP, lokaler Asterisk und Brew](telefonie-sip-und-brew.md) |

**26 Einträge im Inventory sind keine 26 laufenden oder abgenommenen Instanzen.** Deployment-Core ist eine volle Ubuntu-VM; andere Backendrollen folgen ihren jeweiligen LXC-/Betriebsverfahren. `system-backend/services.toml` und Inventory werden einschließlich Ports/Security-Modus gemeinsam geprüft; tatsächliche Bereitstellung richtet sich nach gerendertem Inventory und installierten TOMLs. Der Agentenkatalog enthält zusätzlich TBS und den optionalen Provisioning Core, während Controller-/VM-Update einen eigenen Pfad haben. Ältere 17-/24-Dienst-Anleitungen sind historische Momentaufnahmen.

Die Dienste stellen Fach-WebUIs und APIs bereit; Alert-Verwaltung bleibt tokenpflichtig, übrige Open-Lab-Netzgrenzen gelten weiter; die [WebUI-Matrix](../design/dienstoberflaechen-funktionsmatrix.md) beschreibt Verwaltungsaktionen. [Bedienoberflächen und Rollen](bedienoberflaechen-und-zustaendigkeiten.md) trennt die Oberflächen, [Netzwerk, Ports und Protokolle](netzwerk-und-ports.md) die Transportschicht und [Sicherheit und Betrieb](sicherheit-im-betrieb.md) die Netzgrenze.

## Quellen zur Pflege dieser Seite

[Dienstregistry mit Sicherheitsmodi](../../system-backend/services.toml) · [Inventory mit Units und Abhängigkeiten](../../deploy/open-lab/inventory.example.toml).
