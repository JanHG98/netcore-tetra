# Observability-LXC: Syslog, Discovery und tägliche NFS-Logarchivierung

Der Chat führte von der Idee einer täglichen Logkopie nach einer vollen LXC-Platte zu einer eigenen, in NetCore integrierten Syslog-Pipeline im bestehenden Observability-LXC. **Am 27.09.2026 waren der echte NFS-Mount in Observability und Media Library, ein Schreibtest mit der übersetzten Dienstidentität sowie ein Archivlauf mit einem Segment und ohne Fehler durch Betreiber-Ausgaben bestätigt.** Die Anbindung des ersten entfernten Journald-Senders auf CT 138 blieb im zugänglichen Verlauf der nächste, noch nicht bestätigte Schritt.

**Der heutige Repository-Stand ist davon zu trennen:** Die historische Implementierung ist im Commit `41e69acba89dc3bd9ea0e0679efef3bcecc7f7e6` erhalten. PR #57 wurde in den damaligen Branch `feature/openlab-discovery-deployment` übernommen. Die neue Syslog-/Discovery-Pipeline fehlt jedoch im am 06.10.2026 geprüften `main` und `Archiving`. Eine Archivierung dieses Wissens integriert die fehlenden Produktdateien nicht. Vor neuen Updates muss deshalb der tatsächlich installierte Stand mit der aktuellen Integrationsplanung abgeglichen werden.

## 1. Metadaten, Quellenumfang und Nachweisstufen

| Feld | Wert |
|---|---|
| Thema | Zentrale Syslog-Sammlung, Discovery-Adressen, begrenzte Logpuffer und tägliche NAS-Archivierung; Proxmox-/LXC-NFS-Reparatur |
| Ursprünglicher Chattitel | Nicht verfügbar; der Titel dieses Dokuments ist ein beschreibender Archivtitel |
| Ursprünglicher Chatlink / Chat-ID | Nicht verfügbar; die im Browserkontext genannte GitHub-Datei ist kein Chatlink |
| Historischer Betriebszeitpunkt | Terminalnachweise vom 27.09.2026; ursprünglicher Mountfehler im Journal vom 14.09.2026 |
| Erstellung und Quellenprüfung | 2026-10-06; Benutzerzeitzone Europe/Berlin |
| Repository | [JanHG98/netcore-tetra](https://github.com/JanHG98/netcore-tetra) |
| Ausschließlicher Schreibbranch | `Archiving` |
| Geprüfter Archiv-Ausgangscommit | [`5bcc293c123ca11d66bc7550d4160df354bfcde2`](https://github.com/JanHG98/netcore-tetra/commit/5bcc293c123ca11d66bc7550d4160df354bfcde2) |
| Zusätzlich nur lesend geprüfter Hauptzweig | [`main@9116c15d645458f99e236712b67a1ad970432791`](https://github.com/JanHG98/netcore-tetra/tree/9116c15d645458f99e236712b67a1ad970432791) |
| Historischer Implementierungscommit | [`41e69acba89dc3bd9ea0e0679efef3bcecc7f7e6`](https://github.com/JanHG98/netcore-tetra/commit/41e69acba89dc3bd9ea0e0679efef3bcecc7f7e6) |
| Historischer Feature-Branch | `feature/observability-syslog-discovery`; am Archivdatum kein vorhandener Remote-Branch mehr |
| Historischer PR | [#57: Observability: Discovery-Adressen, begrenzter Syslog-Puffer und tägliches Share-Archiv](https://github.com/JanHG98/netcore-tetra/pull/57) |
| Zielpfad | `Docs/archive/2026-10-06_observability-syslog-discovery-nfs-logarchivierung.md` |
| Index | [README.md](README.md) |
| Änderungsumfang dieses Auftrags | Abschlussdokumentation und Index unter `Docs/archive/`; keine Produkt-, NAS-, LXC-, Roadmap- oder main-Änderung |

Der Ausgangscommit bezeichnet den geprüften Bestand vor der Archivänderung. Der tatsächliche Archivcommit ist über die Git-Dateihistorie und die Abschlussmeldung auffindbar; er ist nicht mit dem historischen Produktcommit gleichzusetzen.

### 1.1 Zugängliches Material und Grenzen

Ausgewertet wurden die sichtbaren Benutzerbeiträge einschließlich der ausführlichen Terminalausgaben, eine komprimierte Übergabe früherer Assistenzarbeit, eine ergänzende Kontextsuche sowie direkt gelesene Repository-Dateien und PR-Metadaten. Frühere Assistenzantworten, vollständige ursprüngliche Toolprotokolle und sämtliche damaligen Buildlogs liegen nicht als vollständiger Chat-Export vor. Die ergänzende Kontextsuche lieferte für diesen Chat keinen zusätzlichen Sendererfolg, Chatlink oder zuordenbaren Bildanhang. Andere gefundene Projektchats werden nicht als Ausführungsbeleg dieses Chats behandelt.

Die technischen Aussagen zur historischen Implementierung wurden, soweit unten angegeben, erneut am festen Commit geprüft. Die damaligen Testzahlen sind als historische Angaben gekennzeichnet; neue Prüfungen dieses Archivlaufs werden separat aufgeführt. Der Archivlauf hatte keinen Zugriff auf das Live-LAN. Der seit dem 27.09. möglicherweise veränderte aktuelle Betriebszustand ist unbekannt.

Von 25 angekündigten PDF-Anhängen sind 23 Dateien lokal vorhanden. Zwei fehlen; siehe Abschnitt 13. Die PDFs wurden für diese NFS-/Syslog-Dokumentation nicht fachlich als TETRA-Normen ausgewertet. Es sind keine zugänglichen Originalbilder dieses Chats vorhanden. Ein früher erwähnter Testscreenshot ist nicht wieder verfügbar; er wurde nicht erfunden oder durch ein fremdes Bild ersetzt.

### 1.2 Statusbegriffe

| Status | Bedeutung in diesem Dokument |
|---|---|
| **Idee** | Erwogener Ansatz ohne belegten Umsetzungsbeschluss |
| **Beschlossen/geplant** | Gewünschter bzw. vereinbarter Ablauf; Ausführung noch nicht belegt |
| **Implementiert** | In einem ausdrücklich genannten Repository-Commit nachgewiesener Code |
| **Getestet** | Ein bestimmter Prüfablauf und sein Ergebnis sind belegt; Umfang und Umgebung werden genannt |
| **Im Betrieb bestätigt** | Betreiber-Ausgabe aus der realen Anlage belegt die konkret genannte Eigenschaft zu einem bestimmten Zeitpunkt |

Diese Stufen sind nicht austauschbar: Ein PR-Text oder eine Installationsanleitung belegt keinen Rollout; ein aktiver Dienst belegt keine Zustellung; ein lokaler Syslog-Test belegt keinen entfernten RELP-Sender.

## 2. Ziel, Ausgangslage und Entscheidungen

Auslöser war eine zuvor vollgelaufene LXC-Platte. Welcher Container, welche genaue Logdatei und welcher Umfang den ursprünglichen Vorfall verursachten, ist in diesem Verlauf nicht abschließend belegt. Der Nutzer wollte Logs automatisch, beispielsweise täglich, auf dem vorhandenen Share ablegen. Dort hatte er bereits `Logs` auf derselben Ebene wie `Recordings`, `Media-Library` und `TTS-Dateien` angelegt.

Danach kamen zentrale Syslog-Sammlung, die Alternative [inventor7777/syslog-flow](https://github.com/inventor7777/syslog-flow), die Frage „Deployment-VM oder eigener LXC?“ und die automatische Übernahme der Dienst-IP-Adressen hinzu. Der Nutzer beauftragte schließlich ein Update für den vorhandenen Observability-LXC. Die beobachtete Umsetzung war ein eigener NetCore-Integrationslayer auf rsyslog, Python und dem vorhandenen Rust-NMS.

| Anforderung / Entscheidung | Endstand und Begründung | Nachweisstufe |
|---|---|---|
| Logs zentral auf dem bestehenden Medien-Share archivieren | `Logs/NetCore` verwenden; andere Medienverzeichnisse erhalten | Beschlossen, historisch implementiert, erster Archivlauf im Betrieb bestätigt |
| Observability als eigener LXC | Bestehenden CT 136 nutzen; Empfang und Archivierung organisatorisch von der Deployment-VM trennen | Beschlossen; Betrieb von CT 136 bestätigt |
| Adressen aus der Deployment-VM übernehmen | Bekannte Adressen als Startwerte, danach regelmäßige Discovery; letzte gültige Ziele bei VM-Ausfall behalten | Historisch implementiert; dynamischer Wechsel im Live-LAN hier nicht bestätigt |
| Speicherproblem nicht nur auf den Collector verlagern | Lokale Segmente, Sender-/Receiver-Queues, UI-Vorschau und Archiv begrenzen | Historisch implementiert; Grenztests teilweise lokal, reale Langzeitwirkung offen |
| Verlässlicher Transport | Journald-Sender über RELP/TCP 20514; zusätzlich TCP/UDP 514 für kompatible Quellen | Historisch implementiert; lokaler TCP-514-Test bestätigt, entfernter RELP-Pilot offen |
| Archiv nur auf echtem Netzlaufwerk | Netzwerkdateisystem und Mountidentität prüfen; kein lokales Ersatzarchiv bei fehlendem NAS | Historisch implementiert; Mountreparatur und erster NAS-Lauf bestätigt |
| Löschung erst nach verifizierter Kopie | gzip-Archiv zurücklesen und SHA-256 gegen Quelldaten prüfen | Historisch implementiert und lokal testbar; Dienstlauf am NAS erfolgreich |
| NAS-VM nicht umkonfigurieren | NAS ist VM 100, startet laut Nutzer bereits automatisch als zweite VM nach AD | Ausdrückliche Nutzerkorrektur; NAS-Startreihenfolge unverändert lassen |
| Kopierbare Betriebsanweisungen | Kurze, eigenständige Befehle mit absoluten Pfaden; keine über SSH-Neuanmeldungen vorausgesetzten Variablen | Aus konkretem Kopier-/Sitzungsfehler abgeleitete Arbeitsvorgabe |
| `syslog-flow` als fertige Alternative | Im Chat vorgeschlagen; keine Installation oder abschließende belastbare Produktbewertung nachgewiesen | Idee, nicht gewählter Implementierungspfad |

Die ursprüngliche tägliche „Kopie der LXC-Logs“ wurde damit präzisiert: laufende Übertragung neuer Journalmeldungen an den Collector, tägliche Archivierung seiner Segmente und zusätzliche lokale Journalgrenzen. Bereits vorhandene alte Journale und beliebige Dateilogs werden dadurch nicht automatisch vollständig exportiert oder verkleinert.

## 3. Architektur der historischen Implementierung

Die folgenden Angaben beschreiben **Commit `41e69ac…`**, nicht die derzeit fehlende Integration in main.

### 3.1 Daten- und Steuerpfade

1. Auf einer Logquelle liest eine eigene rsyslog-Instanz `journald` über `imjournal`. Der persistente Cursor wird bei Neustarts wiederverwendet. Beim ersten Start werden nur neue Meldungen gelesen.
2. `omrelp` überträgt an Observability auf TCP 20514. Der Sender besitzt eine begrenzte persistente Queue. Für sonstige Syslog-Quellen lauscht der Collector auch auf TCP und UDP 514.
3. Die separate Receiver-Instanz übergibt strukturierte Meldungen über `omprog` an `log_store.py ingest`. Bestätigungen erfolgen über das omprog-Protokoll; lokale JSONL-Segmente enthalten Zeitstempel, Hostname, Quell-IP, Programm, Severity, Facility, Transport und Meldung.
4. Ein separater Preview-Prozess führt eine begrenzte SQLite-Ausgangsqueue zur vorhandenen NMS-Log-API. Der native Rust-Dienst stellt Suche und Status im WebUI auf Port 8210 bereit. Die Vorschau arbeitet unabhängig von der täglichen Archivierung.
5. Der systemd-Archiver versiegelt Segmente, komprimiert sie auf dem NAS, prüft die zurückgelesenen Daten und entfernt erst danach die jeweilige lokale Quelle. Ein eigener Archiv-Lock verhindert parallele Archivläufe. Langsame NAS-Zugriffe halten nicht den Rohdaten-Lock des Empfängers.
6. Die Deployment-VM liefert Discovery-Metadaten. Sie transportiert keine Logs. Ein Ausfall der VM soll den Empfang mit bekannten Adressen nicht stoppen.

Ein hängender `hard`-NFS-Zugriff bleibt ein Betriebsrisiko für den Archivprozess; die Trennung vermeidet dessen direkte Kopplung an den Receiver. Ein Timeout in einer systemd-Unit ist kein allgemeiner Beweis, dass jeder blockierte Kernel-I/O sofort beendet werden kann.

### 3.2 Discovery und Adresspflege

| Eigenschaft | Historischer Stand |
|---|---|
| Controller | `http://10.0.1.131:8320` |
| Controller-Status | `GET /api/v1/status` |
| Kennungen | `protocol=netcore.discovery.v1`, `role=controller`, `security_mode=open_lab`, `environment=netcore-openlab` |
| Intervall | 30 Sekunden; Sender-Timer startet nach 30 Sekunden und wiederholt nach inaktivem Dienst |
| Startinventar | 29 Maschinen in `openlab-hosts.json`, 24 konfigurierte NetCore-Metrikziele |
| Zuordnung | Dienstname, nicht abgeleitete Port- oder CT-Nummer |
| Ausfall / fehlendes Ziel / Mehrdeutigkeit | Bekannte Ziele bzw. Senderkonfiguration erhalten; Konflikte nicht willkürlich auflösen |
| Manuelles Target | `labels = { discovery = "manual" }`; `auto` erlaubt wieder Synchronisierung |
| Gesamtschalter | `[discovery].enabled = false` deaktiviert den NMS-Abgleich |
| Sender-Erstfallback | `10.0.1.143`, wenn noch keine gültige lokale Konfiguration vorhanden ist |
| Eingabegrenzen | Managementnetz `10.0.1.0/24`, HTTP/IPv4 und erwartete Controllerkennung; Anfrage 3 Sekunden, maximal 1 MiB |
| Sender-Konfigurationswechsel | Temporäre Datei, `rsyslogd -N1`, atomarer Austausch und Dienstneustart |

Die Inventardatei ersetzt keine gemessene aktuelle Hostinventur. Die im historischen Beispiel enthaltene PBX-Klassifikation als `LXC` ist in diesem Chat nicht durch eine zugehörige `pct config` bestätigt; vor einem späteren PBX-Rollout muss die tatsächliche Virtualisierungsart festgestellt werden. Für PBX, Brew und TBS werden nicht allein aus ihrer Existenz ungeprüfte `/metrics`-Endpunkte angenommen.

### 3.3 Protokolle, APIs und Grenzen

| Schnittstelle | Zweck / Reichweite |
|---|---|
| TCP/UDP 514, Collector | Syslog-Eingang; UDP ohne Empfangsbestätigung |
| TCP 20514, Collector | RELP-Eingang für den separaten Journald-Sender |
| HTTP 8210, Observability | Native NMS-Oberfläche und API |
| `GET /api/v1/logs` | Begrenzte Logsuche, u. a. Filter `contains`, `service`, `level`, `trace_id`, `limit` |
| `POST /api/v1/logs/ingest` | Native Logannahme, auch für den Preview-Bridgepfad |
| `GET /api/v1/syslog` | Receiver-/Archivstatus aus den Statusdateien |
| `GET /api/v1/discovery` | Status des Adressabgleichs |
| `POST /api/v1/discovery/sync` | Manueller Discovery-Abgleich |
| `GET /api/v1/targets/prometheus` | HTTP Service Discovery für optionales Prometheus |
| NFSv4.1 zum NAS | Am Host geprüfter und später produktiv eingebundener Export |

Das historische Sicherheitsmodell bleibt `open_lab`: keine Managementanmeldung und kein TLS für diese HTTP-/Logpfade. Die erlaubten Quellnetze sind kein Ersatz für Authentisierung. Der Receiver lauscht laut Konfiguration auf `0.0.0.0`; Quellprüfung erfolgt im Store. Die damalige Nutzung war auf das Managementnetz ausgerichtet. Eine spätere Absicherung bleibt ein eigener Entwicklungs- und Abnahmeschritt.

Die Websuche durchsucht die begrenzte Vorschau, nicht die vollständigen gzip-Archive. Persistente Queues und RELP verbessern die Zustellung, garantieren bei vollen Platten, langen Ausfällen oder Abstürzen jedoch weder Verlustfreiheit noch exakt einmalige Rohdatenspeicherung. Wiederholte Preview-IDs werden innerhalb der NMS-Aufbewahrung dedupliziert; Raw-Duplikate nach einem Crash bleiben möglich.

## 4. Reales Host-, Mount- und Rechteinventar aus dem Chat

### 4.1 Für diese Reparatur maßgebliche Systeme

| System | Proxmox-ID / Typ | Adresse | Bestätigte Eigenschaften |
|---|---|---|---|
| `SRV-H-PVE-01` | Proxmox-Host | `10.0.1.3` | Debian; Kernelmeldung `6.8.12-30-pve`; vorhandene NFS-Clientkonfiguration |
| NAS | VM **100** | `10.0.1.148` | Startet laut Nutzer als zweite VM nach dem AD-Server |
| Observability | CT **136** | `10.0.1.143/24` | Gateway `10.0.1.1`, `vmbr0`, Firewallflag 1; Ubuntu; 2 Kerne, 2048 MiB RAM, 2048 MiB Swap; Rootdisk **10 GiB**; unprivilegiert, nesting |
| Media-Library | CT **138** | `10.0.1.154/24` | Gateway `10.0.1.1`, `vmbr0`, Firewallflag 1; Ubuntu; 4 Kerne, 2048 MiB RAM, 2048 MiB Swap; Rootdisk 25 GiB; unprivilegiert, nesting |
| Deployment-VM | VM, ID hier nicht belegt | `10.0.1.131:8320` | Historischer Discovery-Endpunkt; kein hier gezeigter Live-Discovery-Test |

CT 136 ist nicht „CT 143“, und VM 100 ist nicht „NAS-IP .100“. Ebenso ist CT 131 im vorgelegten `pct list` der Packet Core, nicht die Deployment-VM mit IP `.131`.

### 4.2 Mountpfade

| Ebene | Pfad / Quelle |
|---|---|
| NAS-Export | `10.0.1.148:/mnt/MassStorage/SRV-M-TBS-01` |
| Proxmox-Mountpunkt | `/mnt/netcore-nfs/SRV-M-TBS-01` |
| LXC-Mountpunkt | `/mnt/nfs-share` |
| Bind-Mount-Eintrag | `mp0: /mnt/netcore-nfs/SRV-M-TBS-01,mp=/mnt/nfs-share,backup=0` |
| Archivbasis im LXC | `/mnt/nfs-share/Logs/NetCore` |
| Collector-Unterverzeichnis | `observability-10.0.1.143/YYYY-MM-DD/` |
| NAS-relative Archivlage | `Logs/NetCore/observability-10.0.1.143/` |

`backup=0` ist Teil des gezeigten Bind-Mount-Eintrags; daraus folgt kein Backup des NAS-Inhalts durch ein Containerbackup.

Zusätzlich waren auf dem Host bereits NFSv4.2-Mounts nach `/mnt/pve/Iso-Images` und `/mnt/pve/Media` vorhanden, vom selben NAS zu `/mnt/MassStorage/ISO-Images` bzw. `/mnt/MassStorage/Medien`. Diese Mounts waren funktionsfähig; der NetCore-Export hatte einen eigenen fehlgeschlagenen Mountvorgang.

### 4.3 UID-/GID-Abbildung und tatsächlich gesetzte Rechte

Im Observability-LXC:

```text
uid=999(netcore-observability) gid=989(netcore-observability) groups=989(netcore-observability)
uid_map: 0 100000 65536
gid_map: 0 100000 65536
```

Damit entspricht die Dienstidentität auf dem Host **UID 100999 / GID 100989**. Auch Media Library zeigte die UID-Abbildung `0 100000 65536`.

| Verzeichnis | Ausgangszustand auf dem NAS | Reparierter Zustand laut erfolgreichen Befehlen |
|---|---|---|
| Exportwurzel | Eigentümer/Gruppe `3000:3000`, Modus `0755` | Unverändert |
| `Logs` | `3000:3000`, Modus `0700` | Eigentümer 3000 erhalten; Gruppe **100989**, Modus **0750** |
| `Logs/NetCore` | Kein belastbarer ursprünglicher Rechtezustand | Eigentümer **100999**, Gruppe **3000**, Modus **2750** |
| `Media-Library`, `Recordings`, `TTS-Dateien` | `root:3000`, Modus `0777` in der Verzeichnisliste | Keine globale Rechteänderung beauftragt oder durchgeführt |

Die unterschiedliche Gruppe ist beabsichtigt festgehalten: Der Dienst kann `Logs` über GID 100989 betreten und besitzt `NetCore` selbst über UID 100999. Setgid auf `NetCore` dient der Gruppenvererbung 3000. Keine pauschale oder rekursive Eigentumsänderung über das ganze Medien-Share vornehmen.

### 4.4 Native NetCore-Container und historische IP-Startwerte

Die CT-Zuordnung stammt aus `pct list`, die folgenden IPs mit Ausnahme der separat gezeigten CT-Konfigurationen aus dem historischen `openlab-hosts.json`. Sie sind **keine am Archivdatum neu gemessene Netzliste**.

| CT | Rolle | Historische IP |
|---:|---|---|
| 123 | Node-Gateway | `10.0.1.179` |
| 124 | Mobility-Core | `10.0.1.150` |
| 125 | Subscriber-Core | `10.0.1.153` |
| 126 | Group-Core | `10.0.1.157` |
| 127 | Call-Control | `10.0.1.155` |
| 128 | Media-Switch | `10.0.1.159` |
| 129 | Recorder | `10.0.1.170` |
| 130 | SDS-Router | `10.0.1.169` |
| 131 | Packet-Core | `10.0.1.166` |
| 132 | IP-Gateway | `10.0.1.142` |
| 133 | Security-Core | `10.0.1.149` |
| 134 | KMF | `10.0.1.180` |
| 135 | Transit | `10.0.1.151` |
| 136 | Observability | `10.0.1.143` |
| 137 | Control-Room | `10.0.1.156` |
| 138 | Media-Library | `10.0.1.154` |
| 139 | Application-Gateway | `10.0.1.144` |
| 140 | Provisioning-Core | `10.0.1.106` |
| 141 | IoT-Gateway | `10.0.1.119` |
| 142 | Hardware-Gateway | `10.0.1.123` |
| 143 | RF-Monitor | `10.0.1.122` |
| 144 | Alarm-Workflow | `10.0.1.121` |
| 145 | Task-Workflow | `10.0.1.120` |
| 146 | Asset-Management | `10.0.1.124` |
| 147 | SIP-Switch | `10.0.1.125` |

Weitere historische Logquellen: Brew `10.0.1.22` auf CT 102, TBS/Pi `10.0.1.20`, PBX `10.0.1.21` und Deployment-VM `10.0.1.131`. Das beobachtete `pct list` enthält außerdem CT 149 `warn-control`; dieser ist nicht in der 29er-Startliste dieses Syslog-Commits enthalten. Sein Senderrollout wurde hier nicht bestätigt.

Die anderen gelisteten Hostcontainer waren 103 `CT-H-DEV-02`, 104 `CT-H-DASH-01`, 105 `CT-H-RPX-01`, 106 `CT-H-DOC-01`, 107 `CT-H-SFTP-01`, 108 `radio-id`, 111 `traefik`, 112 `CT-H-PXE-01`, 117 `CT-H-APP-02`, 118 `plex` und 120 `nomad`. CT 120 war gestoppt, die übrigen aufgelisteten Container liefen. Diese Liste ist keine Freigabe, alle fremden Anwendungen in einen NetCore-Bulkrollout einzubeziehen.

## 5. Historische Dateien, Dienste und Konfigurationen

Alle Quellpfade dieser Tabelle sind relativ zu `system-backend/observability/` im [historischen Baum](https://github.com/JanHG98/netcore-tetra/tree/41e69acba89dc3bd9ea0e0679efef3bcecc7f7e6/system-backend/observability).

| Quellpfad | Zweck / installierter Pfad |
|---|---|
| `docs/syslog-update.md` | Installations-, Speicher-, Diagnose- und Rücknahmeanleitung |
| `config/observability.example.toml` | Native Konfiguration; installiert als `/etc/netcore/observability.toml` |
| `config/openlab-hosts.json` | Startinventar; `/etc/netcore/openlab-hosts.json` |
| `config/syslog.example.json` | Collector-/Archivkonfiguration; `/etc/netcore/syslog.json` |
| `config/log-client.example.json` | Senderkonfiguration; `/etc/netcore/log-client.json` |
| `src/discovery.rs` | Validierung, periodischer Controllerabruf und Zielabgleich |
| `src/state.rs`, `src/http.rs`, `web-ui/index.html` | Persistenter Zustand, Log-Deduplikation, API und Status-/Logansicht |
| `install/update.sh`, `install/install.sh` | Build/Installation des NMS einschließlich Integration der Logging-Installation |
| `install/install-logging.sh` | rsyslog-Pakete, Collector-Dateien, Konfigurationsprüfung und systemd-Units |
| `install/install-log-client.sh` | Separater Senderinstaller; Journald-Limits und Clientunits |
| `install/seed-openlab.py` | Konfigurationsmigration mit Backup und Erhalt eigener Einstellungen |
| `install/rsyslog-apparmor.sh`, `logging/apparmor-netcore` | Ergänzung unterstützter AppArmor-Includes; kein generelles Abschalten |
| `logging/receiver.rsyslog.conf` | `/etc/netcore/receiver.rsyslog.conf`; eigene Receiverinstanz ohne Include der lokalen Standard-rsyslog-Konfiguration |
| `logging/log_store.py` | `/opt/netcore-observability/logging/log_store.py`; Modi Check/Ingest/Bridge/Archive |
| `logging/log_client.py` | `/opt/netcore-log-client/log_client.py`; Sender-Discovery und Generierung von `/etc/netcore/log-client.rsyslog.conf` |
| `stack/prometheus/prometheus.yml` | Optionale HTTP-SD-Zielkonfiguration; ausgelieferte Stackvorlage unter `/opt/netcore-observability/stack` |
| `tests/test_syslog.py`, `tests/test_syslog_wire.py` | Store-/Archiv-/Senderprüfungen und reale lokale rsyslog-Protokolltests |
| `tests/native-syslog-smoke.py`, `tests/browser-syslog.cjs` | Native API-/WebUI-Smokeprüfungen |

| systemd-Unit | Aufgabe |
|---|---|
| `netcore-observability.service` | Nativer NMS-Dienst, HTTP 8210 |
| `netcore-syslog.service` | Receiver unter `netcore-observability`; Capability `CAP_NET_BIND_SERVICE` für Port 514; Neustart bei Fehler |
| `netcore-syslog-preview.service` | SQLite-Vorschau zur NMS-API und Receiverstatus; Bridgezyklus 5 Sekunden |
| `netcore-syslog-archive.service` | Separater Oneshot; User/Group `netcore-observability`, `TimeoutStartSec=20min`, `UMask=0027`, `Nice=10`, I/O idle |
| `netcore-syslog-archive.timer` | `OnCalendar=*-*-* 02:15:00 UTC`, `RandomizedDelaySec=5min`, `Persistent=true` |
| `netcore-log-client.service` | Separate Journald-/RELP-Senderinstanz mit eigenem Journalcursor und Queue |
| `netcore-log-client-discovery.service` | Einmaliger Python-Abgleich der Senderadresse |
| `netcore-log-client-discovery.timer` | Regelmäßiger Senderabgleich |

Receiver und Preview haben restriktive systemd-Schreibpfade. Die Archivunit enthält bewusst kein `ProtectSystem=strict`; ihr Kommentar begründet dies mit der Sichtbarkeit nachträglich hinzugefügter Mounts. Sie verwendet dennoch unter anderem `ProtectHome`, `PrivateTmp`, `NoNewPrivileges` und die eigene Mount-/Verzeichnisprüfung. Aus diesem Detail folgt keine allgemeine Sicherheitsabnahme der gesamten Unit.

### 5.1 Speicherparameter

| Bereich | Historische Voreinstellung | Verhalten / Grenze |
|---|---:|---|
| Lokale Raw-Segmente | 2.147.483.648 Bytes = 2 GiB | Älteste ungearchivierte Segmente können bei Budgetdruck verworfen werden; Verlustzähler |
| Lokale Freiplatzreserve | 536.870.912 Bytes = 512 MiB | Teil der lokalen Begrenzung |
| Segmentgröße | 16.777.216 Bytes = 16 MiB | Segmentwechsel |
| Einzelrecord | 16.384 Bytes, maximal 8.192 Meldungszeichen | Kürzung markiert; UI-Meldung höchstens 4.096 Zeichen |
| Receiverqueue | 128 MiB plus Segment-/Verwaltungsaufwand | Diskqueue, begrenzt; Dateisegment 8 MiB |
| Senderqueue | 64 MiB plus Segment-/Verwaltungsaufwand | Diskqueue; Dateisegment 4 MiB; Retryintervall 5 Sekunden |
| Preview-Outbox | 5.000 Einträge | SQLite; alte Vorschauen können verworfen werden |
| Native Logsuche bei Standardmigration | 10.000 Einträge | Keine Volltextsuche über NAS-gzip-Dateien |
| Archiv dieses Collectors | 90 Tage oder 21.474.836.480 Bytes = 20 GiB | Älteste eigene Archive entfernen |
| NAS-Freiplatzreserve | 1.073.741.824 Bytes = 1 GiB | Andere Medien erhalten; keine globale Share-Bereinigung |
| Quelljournal persistent | `SystemMaxUse=256M`, `SystemKeepFree=512M`, `SystemMaxFileSize=32M` | Journald-Drop-in auf jeder installierten Quelle |
| Quelljournal flüchtig | `RuntimeMaxUse=64M`, `RuntimeKeepFree=64M` | Journald-Drop-in |

Für die Pipeline wurden zusätzlich zum 2-GiB-Rohpuffer ungefähr 512 MiB für Queue, SQLite und Metadaten eingeplant, zuzüglich NMS-Zustand und Freiplatzreserve. Die gezeigte Observability-Rootdisk ist nur 10 GiB groß. Diese Budgets begrenzen nicht automatisch Cargo-/Git-Dateien, andere Anwendungen, native Metrikdaten, `/var/log/syslog` oder sonstige Anwendungslogs.

### 5.2 Installierte Zustands- und Konfigurationspfade

```text
/etc/netcore/observability.toml
/etc/netcore/syslog.json
/etc/netcore/openlab-hosts.json
/etc/netcore/receiver.rsyslog.conf
/etc/netcore/log-client.json
/etc/netcore/log-client.rsyslog.conf
/etc/systemd/journald.conf.d/60-netcore-limits.conf
/var/lib/netcore-observability/state.json
/var/lib/netcore-observability/state.json.bak
/var/lib/netcore-observability/logs/
/var/lib/netcore-observability/logs/queue/
/var/lib/netcore-observability/logs/preview.sqlite
/var/lib/netcore-observability/logs/receiver-status.json
/var/lib/netcore-observability/logs/archive-status.json
/var/lib/netcore-observability/logs/archive.lock
/var/lib/netcore-log-client/
```

Historische relevante `syslog.json`-Werte:

```json
{
  "state_dir": "/var/lib/netcore-observability/logs",
  "inventory": "/etc/netcore/openlab-hosts.json",
  "allowed_networks": ["10.0.1.0/24", "127.0.0.0/8"],
  "collector_id": "observability-10.0.1.143",
  "archive_mount": "/mnt/nfs-share",
  "archive_directory": "Logs/NetCore",
  "archive_days": 90,
  "archive_max_bytes": 21474836480,
  "archive_min_free_bytes": 1073741824,
  "local_max_bytes": 2147483648,
  "local_min_free_bytes": 536870912,
  "segment_bytes": 16777216,
  "max_record_bytes": 16384,
  "preview_records": 5000,
  "nms_url": "http://127.0.0.1:8210"
}
```

`open_share()` prüft `/proc/self/mountinfo`: Der konfigurierte Mountpunkt muss als `nfs`, `nfs4` oder `cifs` erscheinen. Anschließend wird er mit `O_DIRECTORY|O_NOFOLLOW` geöffnet und die Gerätekennung mit dem Mount abgeglichen. Weitere Archivzugriffe nutzen Verzeichnisdeskriptoren und Symlinkschutz. Die Prüfung bindet sich **nicht an einen bestimmten NAS-Server oder Exportnamen**; die exakte Quelle wurde im Chat zusätzlich mit `findmnt` kontrolliert. Ein schlafender `autofs`-Mount allein erfüllt diese Prüfung nicht; das tatsächliche NFS muss sichtbar sein. Deshalb darf eine spätere Automountplanung nicht unterstellen, dass der Archiver jeden beliebigen Autofs-Zustand selbst aktiviert.

### 5.3 Migration, Abhängigkeiten und Rückweg

Voraussetzungen der historischen Anleitung: Debian 12 oder neuer bzw. Ubuntu 24.04 oder neuer, Python 3.11+ wegen `tomllib`, eine passende Rust-Toolchain für den Workspace sowie `rsyslog` und `rsyslog-relp`. Der native Dienst verwendet den vorhandenen Benutzer `netcore-observability`. Der Archivinstaller legt keinen NAS-Mount an, da er den tatsächlichen Export und die Proxmox-Rechteabbildung nicht kennt.

Die historische Migration sichert geänderte TOML-Dateien als `observability.toml.before-syslog-<Zeitstempel>`, ersetzt alte ausgelieferte Loopback-/`10.0.20.*`-Ziele und ergänzt fehlende Targets. Eigene URLs, Regeln, deaktivierte Targets und manuelle Discovery-Overrides sollen erhalten bleiben; auch persistente alte NMS-Ziele werden berücksichtigt. Die unveränderte Standard-Loggrenze 100.000 wird auf 10.000 umgestellt. Dateimodus und Eigentum werden erhalten. Ein individueller bestehender `/etc/prometheus`-Stand wird vom normalen Update nicht pauschal ersetzt; aktualisierte Vorlagen liegen separat im Installationspräfix.

Der damals dokumentierte Rückweg war **nur vorgeschlagen, nicht im LAN getestet**: zuerst Archivtimer, Preview und Receiver stoppen/deaktivieren, auf den Quellen Sender und dessen Discovery-Timer stoppen; anschließend vorherigen NMS-Build und gesicherte TOML wiederherstellen. Raw-Segmente, SQLite, Journalcursor und NAS-Archive für Diagnose behalten. Den Journald-Drop-in nur entfernen, wenn tatsächlich die früheren Grenzen wieder gelten sollen, und dann journald neu starten. Der frühere Rust-Deserializer ignoriert unbekannte optionale Discovery-Felder; dies allein ist keine vollständige Abnahme beliebiger Versionsrücksprünge. Die NFS-/Bind-Mount-Reparatur muss für eine reine Software-Rücknahme nicht automatisch zurückgebaut werden.

## 6. Historischer Ablauf und Fehlerdiagnose

### 6.1 Update und erste Mountdiagnose

Der Nutzer meldete, dass das Update auf dem damaligen Feature-Branch lief:

```bash
git fetch origin
git switch feature/observability-syslog-discovery
bash system-backend/observability/install/update.sh
```

**Status:** Installation vom Nutzer als laufend/funktionierend gemeldet; anschließend existierten Syslog-Archivdienst und Statusdatei im LXC. Ein `git rev-parse HEAD` der installierten Anlage wurde nicht gezeigt. Deshalb ist der genaue Live-Quellcommit nicht zusätzlich bewiesen. Die Befehle sind historische Dokumentation: Der Feature-Branch existiert am Archivdatum nicht mehr als Remote-Branch und ist keine unverändert ausführbare heutige Neuinstallationsanweisung.

Zunächst nahm der Nutzer an, Proxmox kenne das Share nicht, weil er es direkt in LXC einspiele. Die späteren Ausgaben korrigierten dies:

```text
Media-Library /mnt/nfs-share:
SOURCE rpool/ROOT/pve-1[/mnt/netcore-nfs/SRV-M-TBS-01]
FSTYPE zfs
```

`pct config 138` zeigte den Host-Bind-Mount. Auf dem Host löste `findmnt -T /mnt/netcore-nfs/SRV-M-TBS-01` zunächst nur die ZFS-Root `/` auf. Der NFS-Eintrag war bereits in `/etc/fstab`, aber nicht aktiv:

```fstab
10.0.1.148:/mnt/MassStorage/SRV-M-TBS-01 /mnt/netcore-nfs/SRV-M-TBS-01 nfs rw,vers=4.1,hard,_netdev,x-systemd.mount-timeout=30s,timeo=600,retrans=2 0 0
```

Das Journal des systemd-Mountunits zeigte am 14.09.2026 den Start um 21:46:48 und den Abbruch um 21:47:18: `Mounting timed out`, Prozess mit TERM beendet, `Failed with result 'timeout'`. **Bewiesen ist der 30-Sekunden-Timeout.** Dass der NAS-Dienst beim Hostboot noch nicht bereit war, ist eine plausible Erklärung, aber kein durch korrelierte NAS-/VM-Logs bewiesener Ursachenbefund.

### 6.2 Exporttest und echter Hostmount

Ein temporärer Mount mit `ro,vers=4.1,hard,retry=0` desselben Exports funktionierte. `findmnt` zeigte `nfs4`; die Verzeichnisliste enthielt `Logs`, `Media-Library`, `Recordings` und `TTS-Dateien`. Anschließend wurde wieder ausgehängt und der temporäre Mountpunkt entfernt. Das bewies die grundsätzliche Erreichbarkeit und Lesbarkeit des Exports am Testzeitpunkt, noch nicht die Schreibrechte des Observability-Kontos.

`du -xhd1` meldete für den ungemounteten lokalen Hostordner 512 Bytes. Zusätzlich wurde geprüft, dass er keine Einträge enthielt. Nur in diesem ungemounteten, leeren Zustand wurden Eigentümer `root:root` und Modus `000` gesetzt, um versehentliches Schreiben in den lokalen Ersatzordner zu verhindern. **Dieser Schritt darf nicht auf die gemountete NAS-Wurzel übertragen werden.**

Der fehlgeschlagene Mountunit wurde zurückgesetzt und gestartet. Danach zeigte `findmnt -M` exakt den gewünschten NAS-Export als `nfs4`. Das Problem lag damit nicht in einer grundsätzlich unmöglichen NFS-Nutzung des NAS.

### 6.3 Kopierfehler, SSH-Abbruch und falscher Testpfad

Ein längerer Ablauf wurde stückweise in die interaktive Root-Shell kopiert. Dabei waren `set -euo pipefail` und `NETCORE_SHARE` für mehrere Befehle vorausgesetzt. Aus `test` wurde `est`; der Fehler beendete durch `set -e` die SSH-Sitzung.

Nach erneutem Login war `NETCORE_SHARE` nicht mehr gesetzt. Ohne den früheren `set -u` expandierte `"$NETCORE_SHARE/Logs"` zu `/Logs`. Darauf folgten Fehler wie `chgrp: cannot access '/Logs'`. Ein späterer Verzeichnisanlagebefehl konnte trotzdem `/Logs/NetCore` lokal anlegen. Der dort erfolgreiche Schreibtest bewies keine NAS-Funktion. `pct set` erhielt ebenfalls eine leere Volumequelle und meldete:

```text
400 Parameter verification failed.
mp0: invalid format - format error
mp0.volume: property is missing and it is not optional
```

Das war ein Variablen-/Kopierfehler, kein Nachweis eines defekten NFS-Exports. Die erfolgreiche Reparatur entfernte die leeren lokalen Ordner mit `rmdir`, deaktivierte die interaktiven Fehleroptionen und verwendete ausgeschriebene Pfade.

### 6.4 Rechte, Bind-Mount und finaler Nachweis

Die Rechtekorrektur unter dem echten Export und der Schreib-/Lese-/Löschtest als UID 100999/GID 100989 liefen erfolgreich. CT 136 erhielt `mp0`; sein Shutdown/Start ist ausdrücklich in der Benutzer-Ausgabe enthalten. Die abschließenden Abfragen zeigten **in beiden CTs**:

```text
TARGET         SOURCE                                    FSTYPE
/mnt/nfs-share 10.0.1.148:/mnt/MassStorage/SRV-M-TBS-01      nfs4
```

Ein Neustart von CT 138 war im früheren Ablauf vorgeschlagen. Seine konkrete erfolgreiche Ausführung ist im sichtbaren Originalprotokoll nicht separat belegt; der abschließende echte NFS-Mount innerhalb von CT 138 dagegen schon. Diese beiden Aussagen dürfen nicht gleichgesetzt werden.

### 6.5 Archivlauf und Timer

Der Nutzer schickte lokal im Observability-LXC eine TCP-Syslog-Meldung, startete den Archivservice und las die Statusdatei. Ergebnis:

```json
{"last_attempt": "2026-09-27T16:18:04.149856+00:00", "archived_segments": 1, "error": null, "last_success": "2026-09-27T16:18:08.270585+00:00"}
```

Das Journal bestätigte den erfolgreichen Dienstabschluss um `16:18:10` UTC. Ein früherer Lauf um `16:09` steht ebenfalls im Journal, wird aber nicht ohne weiteren Statusinhalt als zusätzlicher Segmentnachweis gewertet. Die Timerliste zeigte einen nächsten Lauf am 28.09. um `02:16…`; die Ausgabe war gekürzt und der eingegebene Befehl enthielt einen versehentlich verdoppelten Teil.

Die vollständige Timerdefinition im historischen Code belegt **02:15 UTC plus bis zu fünf Minuten Zufallsverzögerung**. Das entspricht im September **04:15–04:20 MESZ**. Es handelt sich nicht um eine fest auf 02:15 deutscher Ortszeit gesetzte Aufgabe.

**Testgrenze:** Der Status bestätigt einen archivierten Abschnitt und einen erfolgreichen Serviceabschluss. Im Chat wurde kein manuelles Entpacken gezeigt, das genau den zuvor gesendeten Marker im NAS-Archiv sichtbar macht. Der Code führt eine interne Rückleseprüfung aus; diese ist von einer zusätzlich beobachteten manuellen Inhaltskontrolle zu unterscheiden. Ebenso bestätigt der Loopback-Test nicht TCP 20514 durch die Proxmox-/LXC-Firewall von einem anderen Container.

## 7. Wichtige Befehle und ihr tatsächlicher Status

Die folgenden Befehle werden als Betriebswissen aufbewahrt. Sie wurden im Archivauftrag **nicht erneut auf den Zielsystemen ausgeführt**. Vor einer Wiederholung müssen aktuelle IDs, Mounts, Rechte und installierte Software geprüft werden. Insbesondere sind Reparaturbefehle keine Aufforderung, den inzwischen funktionierenden Mount erneut umzubauen.

### 7.1 Diagnosen – im Chat ausgeführt

Auf dem **Proxmox-Host**:

```bash
pct list
pct config 136
pct config 138
findmnt -M /mnt/netcore-nfs/SRV-M-TBS-01 -o TARGET,SOURCE,FSTYPE
findmnt -t nfs,nfs4 -o TARGET,SOURCE,FSTYPE,OPTIONS
findmnt --fstab -t nfs,nfs4 -o TARGET,SOURCE,FSTYPE,OPTIONS
du -xhd1 /mnt/netcore-nfs/SRV-M-TBS-01
```

Im **Observability-LXC** wurden `id netcore-observability`, `cat /proc/self/uid_map` und `cat /proc/self/gid_map` ausgeführt. Im **Media-Library-LXC** wurden die UID-Abbildung und `findmnt -T /mnt/nfs-share` geprüft.

Der erfolgreiche temporäre Lesetest verwendete:

```bash
mount -v -t nfs -o ro,vers=4.1,hard,retry=0 10.0.1.148:/mnt/MassStorage/SRV-M-TBS-01 <temporärer-leerer-Mountpunkt>
```

`<temporärer-leerer-Mountpunkt>` ist hier bewusst ein Platzhalter, keine wörtlich auszuführende Shellzeile. Im Chat wurde ein mit `mktemp -d /mnt/netcore-nfs-test.XXXXXX` erzeugter Ordner benutzt und danach wieder entfernt.

### 7.2 Erfolgreiche abschließende Reparatur – auf Proxmox ausgeführt

Die leeren, versehentlich lokal erstellten Ordner wurden entfernt:

```bash
set +e
set +u
set +o pipefail
rmdir /Logs/NetCore /Logs
```

Anschließend auf dem zuvor als echt gemountet geprüften Export:

```bash
chgrp 100989 /mnt/netcore-nfs/SRV-M-TBS-01/Logs &&
chmod 0750 /mnt/netcore-nfs/SRV-M-TBS-01/Logs &&
install -d -o 100999 -g 3000 -m 2750 /mnt/netcore-nfs/SRV-M-TBS-01/Logs/NetCore
```

Der tatsächliche NAS-Test:

```bash
setpriv --reuid=100999 --regid=100989 --clear-groups sh -ec 'p=$(mktemp /mnt/netcore-nfs/SRV-M-TBS-01/Logs/NetCore/.nfs-test.XXXXXX); echo NAS-Test > "$p"; test "$(cat "$p")" = NAS-Test; rm "$p"; echo "NAS-Schreibtest OK"'
```

Ergebnis: **`NAS-Schreibtest OK`**. Die Shelloption `-e` ist hier auf die kurze Kind-Shell begrenzt und beendet nicht die übergeordnete SSH-Sitzung.

Der erfolgreiche CT-136-Bind-Mount und Neustart:

```bash
pct set 136 -mp0 /mnt/netcore-nfs/SRV-M-TBS-01,mp=/mnt/nfs-share,backup=0
pct shutdown 136 --timeout 60 && pct start 136
```

Die abschließenden Abfragen in beiden CTs:

```bash
pct exec 136 -- findmnt -T /mnt/nfs-share -o TARGET,SOURCE,FSTYPE
pct exec 138 -- findmnt -T /mnt/nfs-share -o TARGET,SOURCE,FSTYPE
```

Vorher war eine Sicherung von `/etc/pve/lxc/136.conf` nach `/root/136.conf.before-nfs.<Zeitstempel>` vorgeschlagen und im fehlerhaften Ablauf eingegeben worden. Der genaue erzeugte Dateiname und sein Inhalt wurden nicht separat ausgegeben; nicht als geprüften aktuellen Rücksicherungspunkt behandeln.

### 7.3 Erfolgreicher Archivtest – auf Proxmox ausgeführt

```bash
pct exec 136 -- logger --tcp --server 127.0.0.1 --port 514 --tag netcore-nfs-test "NetCore: Test der NAS-Archivierung"
pct exec 136 -- systemctl start netcore-syslog-archive.service
pct exec 136 -- cat /var/lib/netcore-observability/logs/archive-status.json
pct exec 136 -- journalctl -u netcore-syslog-archive.service -n 30 --no-pager
```

Der korrekt geschriebene Timer-Prüfbefehl lautet:

```bash
pct exec 136 -- systemctl list-timers netcore-syslog-archive.timer --no-pager
```

Der Chat enthält ein Timerergebnis, jedoch mit einem kopierbedingt veränderten Aufruf. Die hier normalisierte Zeile ist die lesbare Fortsetzungsfassung.

### 7.4 Boot-/Automount-Vorschlag – Anwendung nicht bestätigt

Weil der NAS selbst eine VM auf Proxmox ist, garantiert seine VM-Startposition allein nicht, dass NFS bereits bereit ist, wenn der Host seine fstab-Mounts abarbeitet. **Am NAS und an seiner Startreihenfolge sollte auf ausdrücklichen Wunsch nichts geändert werden.** Vorgeschlagen war ausschließlich eine Anpassung des bestehenden NetCore-fstab-Eintrags auf Proxmox, mit vorheriger Sicherung:

```bash
cp -an /etc/fstab /etc/fstab.before-netcore-automount
nano /etc/fstab
```

Vorgeschlagene Ersatzzeile, nicht zusätzlich zur bisherigen Zeile eintragen:

```fstab
10.0.1.148:/mnt/MassStorage/SRV-M-TBS-01 /mnt/netcore-nfs/SRV-M-TBS-01 nfs rw,vers=4.1,hard,_netdev,nofail,x-systemd.automount,x-systemd.idle-timeout=0,x-systemd.mount-timeout=120s,timeo=600,retrans=2 0 0
```

Danach vorgeschlagen:

```bash
systemctl daemon-reload
findmnt --fstab --raw -t nfs,nfs4 -o TARGET,OPTIONS
```

**Keine abschließende fstab-Ausgabe und kein Kaltstarttest liegen vor.** Der bereits funktionierende Mount sollte nicht nur zum Aktivieren dieser Idee ausgehängt werden. Ein fehlender NAS kann den Start eines Containers mit benötigtem Bind-Mount weiterhin verhindern. `nofail` ist kein Beleg dafür, dass jeder davon abhängige LXC startfähig bleibt. `x-systemd.idle-timeout=0` soll ein späteres automatisches Aushängen vermeiden. Zusammenspiel von Host-Automount, Bind-Mount und der strengen Archiv-Mountprüfung muss bei Wiederaufnahme real abgenommen werden.

### 7.5 Senderpilot auf Media Library – nur vorgeschlagen

Der nächste vereinbarte Ablauf war, zuerst CT 138 anzubinden. Dafür sollte das vorhandene Logging-Paket aus dem installierten Observability-Checkout verwendet werden. **Keiner der folgenden Schritte ist durch eine nachfolgende Benutzer-Erfolgsausgabe bestätigt.** Bei heutiger Wiederaufnahme zuerst sicherstellen, dass `/opt/netcore-tetra` in CT 136 tatsächlich noch den geprüften historischen Logging-Code enthält; das heutige main allein enthält diese Dateien nicht.

Alle Befehle waren für den **Proxmox-Host** vorgesehen, einzeln kopierbar:

```bash
pct exec 136 -- tar -C /opt/netcore-tetra/system-backend/observability -czf /root/netcore-logclient.tgz config install logging systemd
```

```bash
pct pull 136 /root/netcore-logclient.tgz /root/netcore-logclient.tgz
```

```bash
pct push 138 /root/netcore-logclient.tgz /root/netcore-logclient.tgz
```

```bash
pct exec 138 -- mkdir -p /opt/netcore-logclient-setup
```

```bash
pct exec 138 -- tar -xzf /root/netcore-logclient.tgz -C /opt/netcore-logclient-setup
```

```bash
pct exec 138 -- bash /opt/netcore-logclient-setup/install/install-log-client.sh
```

Nach erfolgreicher Installation:

```bash
pct exec 138 -- logger -t netcore-log-test "NETCORE-CLIENTTEST-138"
```

Nach ungefähr zehn Sekunden:

```bash
pct exec 136 -- curl -fsS 'http://127.0.0.1:8210/api/v1/logs?contains=NETCORE-CLIENTTEST-138&limit=5'
```

Das Abnahmekriterium ist der passende Eintrag mit plausibler Quelle, nicht nur `active` beim Sender. Der Installer installiert `python3`, `rsyslog` und `rsyslog-relp`, richtet den separaten Sender ein, schreibt die Journalgrenzen und startet `systemd-journald` neu. Er importiert keine alten Journale und keine beliebigen Dateilogs. Ein älterer optionaler HTTP-Journalforwarder sollte nicht parallel dieselben Meldungen senden.

Bei fehlendem Eintrag zuerst Senderdienst/Journal, Discovery-Ziel, TCP 20514, Receiverstatus und Preview prüfen. Der in beiden CT-Konfigurationen gesetzte Proxmox-Firewallflag macht die reine Loopback-Abnahme nicht zu einem Netzwerktest. Keine pauschale Firewallabschaltung als Reparatur vorsehen. AppArmor-Kompatibilität ist ebenfalls auf dem jeweiligen Ziel zu prüfen; der historische Installer bricht bei nicht unterstützter Include-/Reload-Situation ab, statt AppArmor abzuschalten.

## 8. Heutiger Repository-Befund vom 06.10.2026

### 8.1 Historische Veröffentlichung und Branchstatus

Die GitHub-PR-Metadaten bestätigen:

| Eigenschaft | Geprüfter Wert |
|---|---|
| PR | #57 |
| Erzeugt | `2026-09-27T15:03:53Z` |
| Zustand heute | `closed`, `merged=true`, `draft=false` |
| Headbranch / SHA | `feature/observability-syslog-discovery` / `41e69acba89dc3bd9ea0e0679efef3bcecc7f7e6` |
| Basebranch / SHA | `feature/openlab-discovery-deployment` / `17bd2751ce81af48cd41f3d90ab02f556548efdd` |
| Mergecommit | `8573b50a22287e62a30ad34beb2abd70b262d112` |
| Mergezeit | `2026-09-27T15:24:37Z` |
| Umfang | 1 Commit, 35 Dateien, 2.078 Ergänzungen, 87 Löschungen |

Eine frühere Bezeichnung als Draft beschreibt die Erstellungssituation, nicht den heutigen PR-Status. Die PR-Beschreibung enthält noch „Noch keine Installation im Live-LAN“; dieser damalige Text wird durch die späteren Benutzer-Ausgaben zum Mount-/Archivtest teilweise überholt. Er bleibt für nicht nachgewiesene Sender-/Discovery-Tests weiterhin keine positive Abnahme.

Die am Archivdatum gelesene Branchliste enthält `Archiving` und `main`. Beide historischen Feature-Branch-Namen fehlen. Der vollständige historische Commit ist trotzdem direkt abrufbar. Ein gemergter PR in einen Feature-Branch belegt keine Integration in main.

### 8.2 Direkter Vergleich der relevanten Produktdateien

Die Pfade `system-backend/observability/` und `tools/check_observability.py` sind zwischen dem geprüften `Archiving@5bcc293…` und `main@9116c15…` inhaltlich identisch. Daher gelten die folgenden aktuellen Befunde für beide geprüften Stände. Das ist keine Behauptung, dass die gesamten Branches identisch sind.

| Bereich | Historischer Syslog-Commit `41e69ac…` | Heutiges `Archiving` / `main` |
|---|---|---|
| Native NMS-Basis | Vorhanden | Vorhanden |
| Dienstadressen | 24 Starttargets mit realen Managementadressen; Controllerabgleich | 21 Beispieltargets: 17 Loopbackadressen, vier Ziele aus `10.0.20.*`; kein Discovery-Abschnitt |
| `src/discovery.rs` | Vorhanden | Fehlt |
| `config/openlab-hosts.json`, `syslog.example.json`, `log-client.example.json` | Vorhanden | Fehlen |
| `logging/` mit Receiver, Store und Sender | Vorhanden | Fehlt |
| Collector-/Archiv-/Sender-systemd-Units | Vorhanden | Fehlen; native NMS-Unit vorhanden |
| `install/install-logging.sh`, `install-log-client.sh`, `seed-openlab.py`, `rsyslog-apparmor.sh` | Vorhanden | Fehlen |
| `docs/syslog-update.md` | Vorhanden | Fehlt |
| `/api/v1/syslog`, `/api/v1/discovery` | Vorhanden | Fehlen |
| Native Logs API | Vorhanden, mit Preview-Anbindung/Deduplizierung | Vorhanden, ältere direkte Ingestion; neue UUID je Eintrag, keine entsprechende Preview-ID-Deduplizierung |
| Native Loggrenze | Standardmigration auf 10.000 | Beispielkonfiguration 100.000 Einträge und 604.800 Sekunden Alter |
| Journald-Hilfsagent | Älterer HTTP-Agent plus neuer separat zu installierender RELP-Agent | Nur älterer optionaler HTTP-Agent vorhanden |
| `install/update.sh` | Build plus Syslog-Integration | Baut nativen Rust-Dienst, kopiert klassische Stackvorlagen und konfiguriert den eigenen LXC-Endpunkt; kein neuer Archivinstaller |
| Neue Syslogtests / Browser-Smoke | Vorhanden | Fehlen; bestehender allgemeiner Observability-Checker/Referenztest vorhanden |

Die heutige Funktion zur Erkennung der **eigenen** LXC-IPv4 über den gemeinsamen `lxc-network.sh`-Helfer ist nicht dieselbe Funktion wie Discovery aller Dienstziele von der Deployment-VM. Diese ähnliche Bezeichnung darf die Integrationslücke nicht verdecken.

Der heute vorhandene `agents/journal_forwarder.py` startet `journalctl -f -n 0`, sendet HTTP-Batches von 25 Einträgen und verwendet ohne Anpassung `127.0.0.1:8210`. Er ist kein gleichwertiger Nachweis für den historischen RELP-Sender mit persistenter Diskqueue. Die klassischen Prometheus-/Grafana-/Loki-/Promtail-Vorlagen sind Altbestand; die neue historische Pipeline benötigt weder Loki noch Promtail für ihren Kernpfad.

Die letzte Änderung am heute vorhandenen Observability-Verzeichnis stammt aus `2fe2a1939a8795db3816d45973282781dae856f0` mit dem Betreff `Complete persistent dark mode across all NetCore WebUIs`. Diese neuere UI-Arbeit muss bei einer späteren Integration erhalten bleiben. Ein ungeprüftes Zurücksetzen auf das gesamte alte Verzeichnis wäre keine saubere Zusammenführung.

### 8.3 Einordnung in die aktuelle Gesamtplanung

Die nur lesend geprüfte [ROADMAP.md auf main](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/ROADMAP.md) führt als ersten Gesamtschritt **Z01.1: fehlende Deployment-/Syslog-Arbeit gegen aktuelles main abgleichen und den Integrationsplan vorbereiten**. Danach folgen Z01.2 kontrollierte Integration, Z01.3 konsistentes Inventory/Readiness/CI und Z01.4 Installation, Upgrade und Recovery einschließlich NAS-Ausfall und begrenzter Logpuffer.

Dort ist außerdem der spätere historische Feature-Stand `bbf039729b9b05f8d623b11195ca24a124f68d16` genannt. Dieser Archivauftrag hat den konkreten Syslog-Commit `41e69ac…` und die relevanten aktuellen Observability-Pfade geprüft; er behauptet keinen vollständigen Tip-Vergleich aller Deployment-, Imagebuilder- und VPN-Dateien und erklärt Z01.1 damit nicht pauschal für erledigt. Ergänzende Historie steht im [Deployment-/Discovery-Archiv](2026-10-05_deployment-vm-tbs-imagebuilder-und-auto-discovery.md).

## 9. Tests und erreichte Nachweisgrenzen

### 9.1 Historisch berichtete Entwicklungsprüfungen

Die ursprüngliche Assistenzübergabe und die überprüfte PR-Beschreibung berichten einen Release-Build, vier Rust-Tests, 17 Python-Tests einschließlich echter rsyslog-Protokoll-/Wiederanlaufprüfung, den bestehenden Paketcheck und Referenztest sowie NMS-API und Chromium-WebUI. Ferner wurde die Migration einer alten TOML einschließlich Backup und Wiederholung beschrieben.

Diese Angaben sind heute als **historische Testberichte** auffindbar, aber ihre vollständigen damaligen Build-/Testlogs wurden nicht wiedergewonnen. Sie sind weder neu ausgeführte Archivtests noch ein Beleg für 29 sendende Live-Systeme. Die historische Teststruktur umfasst 15 Store-/Clienttests und zwei Wiretests; echte lokale rsyslog-Prozesse belegen keine Produktionsfirewall oder NAS-Berechtigungen. Archivtests verwenden einen injizierten lokalen Share-Ersatz.

### 9.2 Durch Benutzer-Ausgaben belegte Tests im LAN

| Prüfung | Ergebnis | Grenze |
|---|---|---|
| Temporärer NFSv4.1-Read-only-Mount | Export erreichbar; erwartete vier Hauptordner sichtbar | Kein Dienstkonto-Schreibtest |
| Hostmount nach Reparatur | Echter Export, `nfs4` | Keine Kaltstartabnahme |
| Dienstidentität `100999:100989` | `NAS-Schreibtest OK` nach Schreiben, Lesen, Vergleichen und Löschen | Nur dieser Pfad/Zeitraum |
| Bind-Mount CT 136 | `pct set` und Shutdown/Start ohne Fehler; echtes NFS sichtbar | Nicht alle anderen CTs geprüft |
| Mount in CT 138 | Echtes NFS sichtbar | Kein separat dokumentierter erfolgreicher Neustartbefehl |
| Lokale TCP-Syslog-Injektion | Kein gemeldeter Fehler | Kein entfernter RELP-Ende-zu-Ende-Test |
| Manueller Archivlauf | Ein Segment, `error=null`, `last_success=2026-09-27T16:18:08.270585+00:00`; Unit erfolgreich | Keine manuelle Gzip-Markeranzeige; kein Dauerlauf |
| Archivtimer | Gelistet mit nächstem Termin | Kein Nachweis, dass der nächste nächtliche Termin später tatsächlich erfolgreich lief |

### 9.3 Neue Prüfungen im Archivauftrag

| Prüfung am 06.10.2026 | Ergebnis | Grenze |
|---|---|---|
| `PYTHONDONTWRITEBYTECODE=1 python3 tools/check_observability.py` im aktuellen Archiving-Checkout | Exit 0, `Observability static package check: OK` | TOML-/Quellmarker-, Shell-/JavaScript-Prüfungen und Aufruf des Referenzmodells; kein Rust-Build und kein Live-Dienst |
| Direkter Vergleich `Archiving@5bcc293…` gegen `main@9116c15…` für `system-backend/observability/` und `tools/check_observability.py` | `git diff --exit-code` Exit 0, keine Unterschiede | Nur die genannten relevanten Pfade, nicht gesamte Branchgleichheit |
| Export von `41e69ac…` in ein temporäres Testverzeichnis einschließlich Deployment-Core-Testabhängigkeiten; `python3 -m unittest discover -s …/system-backend/observability/tests -p test_syslog.py -v` | **15 Tests bestanden: 11 StoreTests und 4 ClientTests** | Lokale/injizierte Abhängigkeiten; keine echte NAS-, rsyslog-Wire- oder Browser-Abnahme |
| Historische Quellen, heutige relevante Dateien, PR-Metadaten, Branchliste, Mount-/Rechteangaben und vorhandener Archivindex | Gelesen und in dieser Dokumentation abgeglichen | Installierte Live-Dateien wurden nicht neu ausgelesen |

Beim anfänglich zu kleinen Testexport fehlten `common.py` bzw. `catalog.json` aus den Testabhängigkeiten. Ein vollständigerer Export behob diese lokale Vorbereitungslücke; es war keine Produktcodeänderung nötig. Die zwei zusätzlichen `test_syslog_wire.py`-Tests wurden heute nicht erneut ausgeführt. Ebenso wurden keine neuen Rust-/Browser-Buildtests, Live-Installationen, NAS-Zugriffe oder Funkprüfungen durchgeführt.

## 10. Ersetzte Ansätze und verbleibende Fehlerklassen

| Früherer Ansatz / Fehler | Endgültige Einordnung |
|---|---|
| „Share wird direkt im unprivilegierten LXC eingebunden; Proxmox kennt es nicht“ | Durch `pct config` und `findmnt` widerlegt: Hostmount plus Bind-Mount; zunächst darunterliegendes ZFS statt NFS |
| Tägliche Kopie allein löst volle LXC-Platten | Unzureichend: lokale Journale/Dateilogs brauchen Grenzen und Rotation; alte Dateien werden durch Übertragung nicht automatisch kleiner |
| Alles in der Deployment-VM betreiben | Für diese Umsetzung ersetzt durch separaten Observability-LXC, VM nur für Discovery |
| Eigenen Syslog-Protokollserver komplett neu bauen | Implementierung verwendet rsyslog als Protokollschicht und maßgeschneiderte NetCore-Speicherung/Integration |
| Alle Ziel-IPs dauerhaft nur statisch pflegen | Historisch ersetzt durch Startinventar plus Controllerabgleich und Cache |
| NAS-VM wegen Mounttimeout umkonfigurieren | Nutzerkorrektur: VM 100 startet schon nach AD; keine Änderung ihrer Startreihenfolge |
| Erfolgreicher Test unter `/Logs/NetCore` nach SSH-Neuanmeldung | Falscher lokaler Pfad durch verlorene Variable; verworfen und entfernt |
| `set -euo pipefail` direkt in interaktiver SSH-Shell für lange Kopiersequenzen | Für diesen Nutzer ungeeignet; Tippfehler trennte die Sitzung. Kurze eigenständige Befehle verwenden |
| `chmod 000` am Mountpfad ohne Zustandsprüfung | Nur für den vorab als leer und ungemountet geprüften lokalen Platzhalter gedacht; nicht auf NAS-Wurzel anwenden |
| Erfolgreicher Archive-Oneshot bedeutet vollständigen Flottenrollout | Falsch; Senderinstallation, Remote-RELP und alle Quellen müssen separat nachgewiesen werden |
| Gemergter PR bedeutet aktuelle main-Integration | Falsch; PR #57 ging in einen Feature-Branch, aktuelle Produktdateien fehlen |

Offen bleiben unter anderem Berechtigungs-/Mountabweichungen nach Boot, die tatsächliche tägliche Ausführung, Netzwerkfreigaben für Remote-RELP, AppArmor-Unterschiede zwischen Zielsystemen und die verbleibenden lokalen Dateilogs. Es gibt keinen Nachweis, dass diese Fehler aktuell auftreten; es sind noch nicht abgeschlossene Abnahmen bzw. bekannte Integrationsgrenzen.

## 11. Offene Aufgaben, Ideen und Roadmap-Kandidaten

Die folgende Liste bewahrt die Arbeiten dieses Chats und ordnet sie der heutigen Planung zu. Sie ändert keine Roadmap außerhalb von `Docs/archive/` und behauptet keine neue pauschale Rolloutfreigabe.

| ID | Aufgabe / Status | Abhängigkeit und konkretes Abnahmekriterium |
|---|---|---|
| OBS-01 | **Beschlossen/geplant:** aktuellen installierten Observability-Stand erfassen und vor Updates sichern | Git-SHA, installierte Dateien/Units, `/etc/netcore`-Konfiguration und vorhandene Archive prüfen; historischen Betriebsstand nicht versehentlich durch aktuelles main ohne Pipeline ersetzen |
| OBS-02 | **Beschlossen/geplant, heute Z01:** fehlende Syslog-/Discovery-Entwicklung kontrolliert integrieren | Vergleich aktuelles main ↔ historischer Feature-Stand, Erhalt neuer UI/Fachänderungen, Konfigurationsmigration, gezielte Tests und Rückweg; dieser Archivlauf ist nicht die vollständige Z01-Abnahme |
| OBS-03 | **Nächster historischer Betriebsschritt:** Senderpilot CT 138 | Passendes Paket, erreichbarer RELP-Port, erfolgreiche Installation; `NETCORE-CLIENTTEST-138` mit korrekter Quelle im NMS und anschließend im Archiv nachweisen |
| OBS-04 | **Geplant:** übrige native NetCore-LXCs anbinden | Erst nach Pilot; CT 123–147 anhand aktueller Liste einzeln erfassen, CT 138 nicht als ungeprüftes Neuziel behandeln; Erfolg/Fehler je Host protokollieren |
| OBS-05 | **Gewünschte Erweiterung:** Deployment-VM, TBS/Pi, Brew und PBX als Logquellen | Tatsächliche Plattform/Adressierung und Dienststartart prüfen; keine CT-ID für PBX oder VM erfinden; CT 149/Warn-Control bewusst ins aktuelle Inventar aufnehmen, falls gewünscht |
| OBS-06 | **Vorgeschlagen, unbestätigt:** Host-Automount und Kaltstartverhalten | Aktuelle fstab zuerst lesen; NAS-Startreihenfolge erhalten; verzögerte NAS-Bereitschaft, fehlender NAS und Wiederkehr prüfen; beide CTs müssen echten NFS sehen |
| OBS-07 | **Offen:** ursprüngliche volle Platte dauerhaft entschärfen | Größte Dateien/Verzeichnisse und Schreibquellen bestimmen; Journald-Limits überprüfen; gezielte logrotate-/imfile-Regeln für Dateilogs; keine wahllose Altdatenlöschung |
| OBS-08 | **Offen:** tägliche Archivierung und Retention im Dauerbetrieb abnehmen | Mehrere tatsächliche Timerläufe, Markernachweis, gzip-Lesbarkeit, lokales Entfernen nach Erfolg, Aufbewahrung nur eigener Archive und NAS-Freiplatz prüfen |
| OBS-09 | **Offen:** Ausfall-/Wiederanlaufprüfungen | Controller aus, Receiver neu starten, NMS aus, NAS aus/fehlerhafte Rechte, volle begrenzte Puffer; korrekte Verlustzähler und Wiederkehr ohne unbegrenztes Wachstum |
| OBS-10 | **Offen:** Discovery im realen Netz abnehmen | Gültiger Adresswechsel, Konflikt, ungültiger Controller, VM-Ausfall und manueller Target-Override; bisher nur Code-/Entwicklungstests, kein Live-Nachweis |
| OBS-11 | **Option:** bestehendes Prometheus auf HTTP-SD umstellen | Individuelle `/etc/prometheus`-Konfiguration erhalten; `promtool check config`, gezieltes Reload und echte Ziele prüfen; Update überschreibt diese Konfiguration nicht pauschal |
| OBS-12 | **Option:** vollständige Archivsuche / zusätzliche Inputs | Bestehende WebUI durchsucht nur Preview; breitere gzip-Suche oder gezielte Dateilog-/weitere Geräteinputs wären zusätzliche Arbeit, kein bereits vorhandenes Versprechen |
| OBS-13 | **Option:** NAS-seitige zusätzliche Quoten | Nur nach tatsächlichem Bedarf; Schutz von TTS/Recordings. Kein Auftrag, am funktionierenden NAS ungefragt Einstellungen zu ändern |
| OBS-14 | **Idee, nicht gewählt:** `syslog-flow` erneut fachlich vergleichen | Nur bei Bedarf; vorhandene Pipeline und Integrationskosten berücksichtigen; kein belegter Funktions-/Lizenzvergleich in diesem Chat |
| OBS-15 | **Spätere Betriebsarbeit:** Authentisierung/TLS und geschütztes Management | Mit zentraler IAM-/Betriebsplanung abstimmen; Open-Lab-Code nicht als bereits abgesicherten Produktionsdienst darstellen |
| OBS-16 | **Archivlücke:** Originalbild/Testprotokolle und fehlende Anhänge nachsichern | Nur echte zuordenbare Dateien ergänzen; ursprünglichen Chatlink/Titel nachtragen, falls verfügbar |

## 12. Konkrete Fortsetzung und Prioritäten

**Zum historischen Gesprächsende** war die Reihenfolge: CT-138-Senderpilot → eindeutigen Empfang prüfen → weitere Quellen ausrollen. Mount und erster Archivlauf waren bereits erfolgreich; eine erneute vollständige NAS-Reparatur wäre kein sinnvoller Start gewesen.

**Bei Wiederaufnahme nach dem heutigen Repository-Abgleich** kommt davor die Bestandsklarheit: installierten Quellstand und Konfiguration von CT 136 sichern, aktuelle Gesamtroadmap lesen und die Übernahmelücke nach Z01 bearbeiten. Der begrenzte Vergleich dieses Archivdokuments liefert dafür Belege, ersetzt aber weder den vollständigen Integrationsplan noch den Produktrollout. Ein bereits laufender historischer Dienst kann weiterhin existieren, obwohl die aktuellen Branches seinen Code nicht enthalten.

Danach den CT-138-Pilot mit dem überprüften Paket abschließen. Eine einzelne positive Logzeile mit eindeutiger Quelle ist das erste Netzwerk-Abnahmekriterium. Als zweites Kriterium genau diesen Marker im NAS-Archiv prüfen. Erst dann einen protokollierten Rollout auf weitere NetCore-Container und getrennt VM/Pi/Brew/PBX planen bzw. im weiter autorisierten Arbeitsumfang durchführen.

Boot-/NAS-Wiederkehr, mehrere echte Tagesarchive und die Kontrolle lokaler Plattenverbraucher bleiben notwendige Betriebsabnahmen. Der Nutzer hat NAS-Änderungen nicht als nächsten Schritt gewünscht. Die übergeordnete aktuelle Priorität Z01.1 bleibt erhalten; dieses Archiv startet keine zusätzlichen Automationen, Deployments oder Änderungen an der Anlage.

## 13. Anhänge, Bilder und Auswertungslücken

### 13.1 PDF-Anhänge

Die folgenden Namen wurden mit dem Auftrag bereitgestellt. „Vorhanden“ bedeutet, dass die Binärdatei im Arbeitsbereich vorlag, nicht dass sie für diese Dokumentation vollständig gelesen oder normativ ausgewertet wurde. Der konkrete Chat behandelt Syslog, Linux-Dienste, Proxmox und NFS; aus den TETRA-Standards wurden hierfür keine technischen Behauptungen abgeleitet. Die umfangreichen PDF-Dateien wurden nicht ohne fachlichen Zweck in das Git-Archiv dupliziert.

| Anhang | Verfügbarkeit / Nutzung |
|---|---|
| `en_3003920308v010401p.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `en_30039209v010701p.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `ts_10081201v020205p.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `en_3003921201v010202p.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `en_3003920304v010301p.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `en_3003921117v010102p.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `en_3003921114v010101p.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `es_20081202v020401m.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `es_20081201v020205p.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `en_300812v020101p.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `en_3003921101v010201p.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `en_3003921006v010401p.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `en_3003921018v010301p.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `en_3003921216v010400a.pdf` | **Nicht bereitgestellt; nicht gelesen. Erneutes Hochladen nötig, falls später relevant.** |
| `en_30039201v010601p.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `ets_30039214e01v.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `en_30039207v030501p.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `en_30039401v030301p.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `en_3003920313v010201p.pdf` | **Nicht bereitgestellt; nicht gelesen. Erneutes Hochladen nötig, falls später relevant.** |
| `en_30039502v010303p.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `en_3003920303v010301p.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `en_30039205v020701p.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `en_3003920315v010500a.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `en_30039202v030801p.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |
| `ETSI.pdf` | Vorhanden; keine fachliche Nutzung für NFS/Syslog |

### 13.2 Bilder

Im sichtbaren Chat liegen Terminalausgaben als Text vor, keine eingebetteten Bilddateien. Die komprimierte Übergabe erwähnt den damaligen Testscreenshot `observability-syslog-test.png`; die Datei war am Archivdatum weder im verfügbaren Arbeitsbestand noch als zuordenbarer Anhang verfügbar. Deshalb konnten **keine Originalbilder dieses Chats** hochgeladen werden. Die bereits im Repository vorhandenen Bilder anderer Chats bleiben unverändert und werden diesem Chat nicht zugerechnet. Eine nachträglich erzeugte Illustration würde den fehlenden Originalnachweis nicht ersetzen und wurde nicht angelegt.

### 13.3 Weitere ausdrücklich offene Belege

- Ursprünglicher Chattitel, Chatlink und vollständiger ungekürzter Altverlauf.
- Vollständige historische Build-, Python-, Rust- und Browser-Testprotokolle.
- Exakter installierter Git-SHA auf CT 136 am 27.09. und aktueller installierter Stand am Archivdatum.
- Abschließende `/etc/fstab` nach dem Automount-Vorschlag und Kaltstartnachweis.
- Installation/Ergebnis des CT-138-Senderpiloten und weiterer Quellen.
- Manuell sichtbarer Testmarker im gzip-Archiv sowie spätere tägliche Timererfolge.

Es wurden keine Passwörter, Tokens, privaten Schlüssel oder sonstigen Zugangsdaten übernommen. Interne Adressen, Dienstbenutzer und numerische Dateirechte sind technische Betriebsparameter des ausdrücklich beauftragten Repository-Archivs.

## 14. Quellen und Fortsetzungsreferenzen

### 14.1 Primärbelege dieses Chats

- Benutzer-Terminalausgaben zu `pct list`, `pct config 136/138`, `findmnt`, UID/GID-Mappings, NFS-Test, Rechtekorrektur und Archive-Service vom 27.09.2026.
- Benutzerkorrektur: NAS ist VM 100, startet bereits als zweite VM nach AD; keine entsprechende NAS-Umkonfiguration gewünscht.
- Letzte offene Assistenzanleitung: Senderpilot CT 138 mit Marker `NETCORE-CLIENTTEST-138`; keine nachfolgende Erfolgsausgabe verfügbar.
- Frühere Zusammenfassung der Entwicklungsarbeit; nur zusammen mit den explizit gekennzeichneten Repository-/PR-Prüfungen als Implementierungsbeleg verwendet.

### 14.2 Repository-Quellen mit festen Ständen

- [Historischer Implementierungscommit `41e69ac…`](https://github.com/JanHG98/netcore-tetra/commit/41e69acba89dc3bd9ea0e0679efef3bcecc7f7e6).
- [Historische Installations- und Betriebsanleitung](https://github.com/JanHG98/netcore-tetra/blob/41e69acba89dc3bd9ea0e0679efef3bcecc7f7e6/system-backend/observability/docs/syslog-update.md).
- [Historischer Store/Archiver](https://github.com/JanHG98/netcore-tetra/blob/41e69acba89dc3bd9ea0e0679efef3bcecc7f7e6/system-backend/observability/logging/log_store.py).
- [Historischer Sender](https://github.com/JanHG98/netcore-tetra/blob/41e69acba89dc3bd9ea0e0679efef3bcecc7f7e6/system-backend/observability/logging/log_client.py) und [Senderinstaller](https://github.com/JanHG98/netcore-tetra/blob/41e69acba89dc3bd9ea0e0679efef3bcecc7f7e6/system-backend/observability/install/install-log-client.sh).
- [Historisches Startinventar](https://github.com/JanHG98/netcore-tetra/blob/41e69acba89dc3bd9ea0e0679efef3bcecc7f7e6/system-backend/observability/config/openlab-hosts.json), [Archivkonfiguration](https://github.com/JanHG98/netcore-tetra/blob/41e69acba89dc3bd9ea0e0679efef3bcecc7f7e6/system-backend/observability/config/syslog.example.json) und [Timerdefinition](https://github.com/JanHG98/netcore-tetra/blob/41e69acba89dc3bd9ea0e0679efef3bcecc7f7e6/system-backend/observability/systemd/netcore-syslog-archive.timer).
- [PR #57](https://github.com/JanHG98/netcore-tetra/pull/57), Merge nach `feature/openlab-discovery-deployment`, nicht nach main.
- [Geprüfter aktueller Observability-Code](https://github.com/JanHG98/netcore-tetra/tree/9116c15d645458f99e236712b67a1ad970432791/system-backend/observability).
- [Aktuelle zentrale Roadmap mit Z01](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/ROADMAP.md).
- [Archiv: Deployment-VM, Imagebuilder und Discovery](2026-10-05_deployment-vm-tbs-imagebuilder-und-auto-discovery.md).
- [Archiv: Media Library, TTS und Medien-Share](2026-10-05_media-library-tts-archivierung-basisstations-playout-und-ip-gateway-routing.md).
- [Archiv: Containerupdates und spätere Betriebsdiagnose](2026-10-06_tbs-container-updates-katwarn-reparaturen-release-und-brancharchiv.md); eigener Chat, keine automatische Erweiterung des hier belegten Senderstands.

### 14.3 Externe Referenz aus der Ideenphase

[inventor7777/syslog-flow](https://github.com/inventor7777/syslog-flow) wurde vom Nutzer als Alternative genannt. Dieses Archiv enthält keine neue Produktbewertung und behauptet weder dessen Installation noch eine dokumentierte Ablehnung aufgrund bestimmter Funktionen. Maßgeblich für den hier bestätigten Betrieb ist die NetCore-Pipeline im historischen Commit und die konkret gezeigte NFS-/Archivabnahme.
