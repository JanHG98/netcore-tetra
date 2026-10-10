# Brainstorming: SWMI Foundation, Mobility, Core-LXC und Open-Lab-Ausbau

**Archivhinweis, 09.10.2026:** Diese Datei dokumentiert das im Titel beziehungsweise Quellenrahmen genannte Gespräch und dessen damalige Entscheidungen. Historische Kommandos, Planungen, Versionsangaben und Fehlerbilder bleiben erhalten; der heutige technische Stand steht in der [Gesamtroadmap](../roadmaps/gesamtroadmap.md) und in den [aktuellen Dienstanleitungen](../services/README.md). Die Archivaufnahme bestätigt keine spätere Implementierung oder Live-Abnahme.

## Zielbild und Festlegungen

- Ausbaufolge: Foundation → lokale Mobility-/Restore-Runtimes → zentrale Core-Dienste.
- Zeitkritische Funklogik bleibt in der TBS; TLMC/TLPD sind lokale Komponenten. Deploybare Dienste erhalten eigene WebUIs.
- Die frühe Phase verwendet `open_lab` ohne Tokens/TLS. Diese Festlegung gilt für die damaligen Dienste und nicht für später hinzugekommene Komponenten.
- Der Abgleich vom **06.10.2026** zeigt vorhandene Backends, aber Lücken der aktiven TBS-Policy-/Restore-Integration. Hardware-/On-Air-Abnahme bleibt offen.

## 1. Arbeitsstand

- **Thema:** Ausbau von NetCore-Tetra von typisierten TLMC/TLPD-Grundlagen über lokale Mobility-/Call-Restore-Runtimes bis zu den ersten zentralen LXC-Diensten `node-gateway`, `mobility-core`, `subscriber-core`, `group-core`, `call-control` und `media-switch`.
- **Erstellungsdatum dieser Notizen:** 2026-10-04.
- **Datum der erneuten Quellen- und Implementierungsprüfung:** 2026-10-06, Europe/Berlin. Der vorhandene Dateiname mit Erstellungsdatum bleibt erhalten; es wird kein zweiter Eintrag dieser Planung angelegt.
- **Repository:** `JanHG98/netcore-tetra`.
- **Zielbranch für dieses Archiv:** `Archiving`.
- **Am Prüfdatum untersuchter Branch-HEAD vor dieser Ergänzung:** [`dac5f0063e9fe06b62157c6b11a351a2ce476fdb`](https://github.com/JanHG98/netcore-tetra/commit/dac5f0063e9fe06b62157c6b11a351a2ce476fdb).
- **Erster Checkout dieser Fortsetzung:** `a76f0f5ae264e42aac2a244f3c2e32c09e7886bb`. Die anschließend übernommenen 40 Commits bis `dac5f00` ändern ausschließlich `Docs/archive/`; die hier geprüften Implementierungsdateien sind zwischen diesen beiden Ständen unverändert.
- **Provenienz der vorhandenen Archivfassung:** [`b2b4e5dcad4824597b403c2a397e46bbd7108a12`](https://github.com/JanHG98/netcore-tetra/commit/b2b4e5dcad4824597b403c2a397e46bbd7108a12). Sie nannte `875380e729ee45aa7cc89f48ccd0a32b4be0c770` als damaligen Prüfstand. Diese frühere Fassung bleibt in Git nachvollziehbar.

### 1.1 Quellenumfang und Grenzen

Die Entwicklungsnotizen unterscheiden drei Quellenebenen:

| Kürzel | Grundlage | Aussagegrenze |
|---|---|---|
| **C – Paketberichte** | Verfügbare Beschreibungen zu Core 1 A–D | Beschreiben den historischen Paketumfang, keine neu ausgeführten Tests. Frühe Foundation-/Mobility-Berichte fehlen. |
| **H – historische Dokumentation** | Vorfassung bei `b2b4e5d` | Sekundärquelle für Foundation/Mobility, Festlegungen, Verpackungsfehler und ZIP-Hashes; ZIP-Dateien wurden am 06.10.2026 nicht erneut geprüft. |
| **R – Repositoryprüfung** | Code, Konfiguration, Installer, Workflows und Strukturchecks bei `dac5f00` | Belegt vorhandene Implementierung und dokumentierte Prüfergebnisse, keine realen Funk-/LXC-Tests. |

C beschreibt die Paketfolge Subscriber Core → Group Core → Call Control → Media Switch. Der nachfolgend vorgesehene Recorder ist dort noch nicht geliefert. Die vier verlinkten Core-ZIPs und elf älteren Foundation-/Mobility-Artefakte waren für den Abgleich nicht lokal verfügbar. Überlieferte Hashwerte in Abschnitt 13 sind keine am 06.10.2026 neu berechneten Werte; Repository-Pfade belegen keine Bytegleichheit mit den ZIPs.

Die 25 PDF-Referenzen unter `sources/` wurden unverändert belassen. Eine neue Normenprüfung fand nicht statt; H nennt vor allem `en_30039202v030801p.pdf` als Grundlage. Eigenständige historische Bilder sind nicht verfügbar.

## 2. Statusbegriffe

Diese Dokumentation trennt bewusst fünf Ebenen:

| Status | Bedeutung |
|---|---|
| **Idee** | in der Planung vorgeschlagen, ohne verbindliche Umsetzung |
| **beschlossen/geplant** | ausdrücklich gewünscht oder als nächster Schritt vereinbart |
| **implementiert** | Quellcode ist am genannten Prüfcommit vorhanden; historische Paketbehauptungen werden zusätzlich mit C/H als Quelle eingeordnet. Vorhandene Runtime-Dateien sind kein Beleg für ihre Einbindung in den aktiven Funkpfad. |
| **getestet** | ein konkreter Test/Checker wurde tatsächlich ausgeführt und sein Ergebnis liegt vor; historische Testberichte und neue Strukturprüfungen werden getrennt geführt |
| **im Betrieb bestätigt** | auf realer TBS/LXC/Endgerät/Funkstrecke erfolgreich beobachtet |

Historisch wurden die Pakete als implementiert und statisch geprüft gemeldet. Der am Prüfdatum vorliegende Abgleich bestätigt viele Dateien und Backend-Funktionen, zeigt aber ausdrücklich fehlende aktive TBS-Integrationen. Ein erfolgreicher Rust-Build der historischen Pakete sowie reale TBS-/LXC-/On-Air-Abnahmen sind durch die zugänglichen Quellen nicht belegt. Die neuen tatsächlichen Prüfergebnisse stehen in Abschnitt 9.8.

## 3. Ziel und Ausgangslage

Der Ausbau setzte auf einem bereits weit fortgeschrittenen NetCore-Tetra-Codebestand auf. Zunächst fehlten belastbare, typisierte lokale SAP-Grundlagen für TLMC/TLPD und vollständige State Machines für Mobility- und Call-Restore-Pfade. Danach wurde der Fokus schrittweise auf einen Multi-Site-SWMI-Ausbau verschoben.

H überliefert folgende Festlegungen; C bestätigt die anschließende Paketfolge und die Open-Lab-Ausrichtung:

- vollständige Repository-ZIPs statt kleiner Patches;
- die lokale zeitkritische Funklogik in der TBS belassen;
- spätere Backend-Komponenten unter `system-backend/` jeweils als eigener Dienst strukturieren;
- für jeden tatsächlich deploybaren LXC-/VM-Dienst eine eigene WebUI vorsehen;
- zunächst eine **offene Testumgebung ohne Tokens, Login oder TLS** verwenden;
- den Ausbau schrittweise von Foundation → Mobility → zentrale Core-Dienste fortsetzen.

Die spätere Sicherheitsentscheidung „erstmal nicht mit Tokens arbeiten“ hatte Vorrang vor früheren allgemeinen RBAC-/Security-Ideen. Für die in dieser Entwicklungsphase eingeführten Backend-Dienste wurde deshalb `security.mode = "open_lab"`, `token_auth = false` und `tls = false` vorgesehen. Das war ausdrücklich ein **Labormodus**, keine Empfehlung für Produktion.

## 4. Architekturentscheidungen dieser Planung

### 4.1 Trennung Edge/TBS und zentraler Core

**Historisches Zielbild aus H und C; zusätzliche Integrationsgrenzen siehe Abschnitt 9.7:**

- PHY/UMAC/LLC/MLE/MM/CMCE-nahe Echtzeitpfade bleiben lokal in der TBS.
- TLMC und TLPD sind lokale Service-Access-Point-/Runtime-Komponenten und **keine eigenen LXC-Dienste**.
- Zentrale Dienste koordinieren Teilnehmer-, Gruppen-, Call- und Media-Zustände, übernehmen aber keine Air-Interface-Timer.
- `node-gateway` bildet den zentralen Transport zwischen TBS und Backend.
- Jeder deploybare Systemdienst besitzt eine eigene WebUI; `system-backend/shared` bleibt eine Bibliotheksstruktur und ist kein eigener Runtime-Container.

### 4.2 Geplanter bzw. in der Planung implementierter Core-Datenpfad

```text
TBS
  ↕ WS /ws/node
Node Gateway
  ↕ WS /ws/backend
  ├─ Mobility Core
  ├─ Subscriber Core
  ├─ Group Core
  ├─ Call Control
  └─ Media Switch
       ↕ Sprachframes / Routing
       └─ weitere TBS
```

### 4.3 Sicherheitsmodus der Testphase

Für die in der Planung aufgebauten LXC-Dienste galt:

```text
open_lab
kein Login
keine Tokens
keine Passwörter
kein TLS
kein mTLS
kein RBAC
isoliertes Testnetz erforderlich
```

Andere Security-Modi sollten in diesen frühen Diensten bewusst abgewiesen werden, statt nicht implementierte Sicherheit vorzutäuschen.

## 5. Chronologischer Entwicklungsstand dieser Planung

**Leseregel für diesen Abschnitt:** 5.1–5.10 werden aus H erhalten, weil die zugehörigen Originalberichte fehlen. 5.11–5.14 sind zusätzlich durch die zum Prüfdatum zugänglichen Paketberichte C belegt. „Implementiert“ und „getestet“ in dieser historischen Chronologie beschreiben die damaligen Paketberichte, nicht automatisch den zum Prüfdatum aktiven TBS-Code. Abweichende am Prüfdatum vorliegende Befunde in Abschnitt 9 haben für die Fortsetzung Vorrang. Insbesondere werden die alten ZIP-Hashes hier nicht erneut verifiziert.

### 5.1 SWMI Foundation 1 – Paket A: Inventur und WebUI-/Backend-Grundstruktur

Dieser Stand liegt im verfügbaren Verlauf nur als vorausgehender Projektkontext vor. Er bildete die Basis für die folgenden Pakete.

**Implementiert bzw. als Ausgangsbasis vorhanden:**

- Protokollinventur der vorhandenen PDU-/SAP-/State-Machine-Lücken;
- Grundstruktur `system-backend/` mit vorgesehenen Diensten;
- verbindliche WebUI-Anforderung für spätere deploybare Dienste;
- Dokumentation `Docs/BACKEND_WEBUI_STANDARD.md` und `Docs/BACKEND_WEBUI_SERVICE_MATRIX.md`;
- `system-backend/services.toml` mit `webui_defaults.required = true`;
- Shared-WebUI-Grundstruktur.

**Artefakte:**

- `netcore-tetra-swmi-foundation1-package-a.zip` – SHA-256 `6d6679b08febec7af936db9f2c76e12a9ff7cde532fb7033fd7c4bcb8ca08a32`.
- `netcore-tetra-swmi-foundation1-package-a-webui.zip` – SHA-256 `6e578e763c2963a08441ad1dc8c792cd284126e0f4257d263f954dc21cb92d62`.

### 5.2 SWMI Foundation 1 – Paket B: TLMC-/TLPD-Typfundament

**Implementiert:**

- gemeinsames Typmodul `crates/tetra-saps/src/common/mod.rs`;
- 27 zentrale gemeinsame Typen für Channel Change, Zellwahl, Measurement, QoS, Restore, LTPD-Kontext und Zustände;
- 18 TLMC-Primitive;
- 27 LTPD-Primitive;
- Einbindung sämtlicher Varianten in `SapMsgInner`;
- nicht-panischer Display-Fallback;
- `SsiType` und `TetraAddress` mit `PartialEq`, `Eq`, `Hash`;
- Korrektur des MS-seitigen SNDCP-Routings auf `TlpdSap` / `Sndcp`;
- neue Tests und statischer Checker `tools/check_swmi_foundation_types.py`.

**Getestet:** statischer Foundation-B-Checker sowie Protokollinventur. Ein lokaler Cargo-Build war in der Artefaktumgebung nicht möglich, weil dort keine Rust-Toolchain vorhanden war.

**Artefakt:** `netcore-tetra-swmi-foundation1-package-b.zip` – SHA-256 `2ff158c1e522c9e2b3fcafcbf68dbc1d408c828b871fb03312acc7ff34eee5e2`.

### 5.3 SWMI Foundation 1 – Paket C: TLMC Runtime

**Implementiert:**

- `crates/tetra-entities/src/umac/tlmc_runtime.rs`;
- Configure Request/Indication/Confirm;
- kantengetriggerter Ressourcenverlust und Recovery;
- Measurement und Monitor;
- Assessment-Listen;
- korrelierte Scan-, Cell-Read- und Select-Abläufe;
- Timeouts und negative Confirmations;
- defensives BS-Routing;
- read-only Diagnose-Snapshot für spätere WebUI/Node-Gateway-Nutzung.

**Bewusste Grenze in der Planung:** Der allgemeine physische SDR-Retune-Pfad war noch nicht vollständig hardwareunabhängig. Die Runtime konnte die benötigte `TmvConfigureReq` erzeugen, aber der Adapter bis zum realen SDR blieb ein klar benannter Integrationspunkt.

**Testgrenze:** Bei der sichtbaren finalen Paket-C-Prüfung wurde ein Python-Validierungslauf nach 60 Sekunden automatisch mit `KeyboardInterrupt` beendet. Das ZIP wurde dennoch erzeugt. Spätere Paketprüfungen führten den Paket-C-Static-Checker erfolgreich erneut aus. Daher gilt: statische C-Invarianten wurden später bestätigt; die damalige Behauptung einer vollständig erfolgreichen unabhängigen Endprüfung war in genau diesem einen sichtbaren Lauf zu stark formuliert.

**Artefakt:** `netcore-tetra-swmi-foundation1-package-c.zip` – SHA-256 `e877aec11e4298b76b6789356041aef9894402a12cf1f4d598cae337e51c360b`.

### 5.4 SWMI Foundation 1 – Paket D: TLPD Runtime

**Implementiert:**

- `crates/tetra-entities/src/mle/ltpd_runtime.rs`;
- lokale Context Registry für Teilnehmeradresse, Endpoint und Link;
- bidirektionales MLE-UNITDATA zwischen SNDCP und MLE/LLC;
- Configure, Connect, Disconnect, Reconnect, Break, Resume, Release, Cancel;
- Open/Close/Info/Busy/Idle/Enable/Disable;
- Layer-2-QoS und dynamische PDCH-Zuteilungen;
- Diagnose-Snapshots auf MLE- und SNDCP-Seite;
- Kopplung von TLMC-Ressourcenstatus an TLPD Break/Resume.

**Später ersetzt:** Paket D meldete einen Transfer anfangs nach LLC-Queue-Übergabe als erfolgreich. Diese Semantik wurde in Paket E ausdrücklich durch `TxReporter`-gestützte echte Abschlusszustände ersetzt.

**Artefakt:** `netcore-tetra-swmi-foundation1-package-d.zip` – SHA-256 `1d63fb16c2ac60c4391664918d4062e83e555453a301d979c4cac0ee993d9f64`.

### 5.5 SWMI Foundation 1 – Paket E: Robustheit und Abnahme

**Implementiert:**

- `TxReporter`-gestützte Transferabschlüsse;
- Duplicate-/Replay-Schutz für `RequestHandle`;
- Cancel-Semantik;
- Pending-Timeout nach 432 Timeslots;
- Cleanup bei Resource Loss, Disable, Close, Release und Call Release;
- negative Lifecycle-Transitionen;
- zusätzliche Diagnosezähler;
- wiederverwendbarer Zwei-Zellen-Harness.

**Getestet:** Paket-B- bis Paket-E-Static-Checker und Protokollinventur liefen erfolgreich. Rust-Kompilation blieb lokal unbestätigt.

**Artefakt:** `netcore-tetra-swmi-foundation1-package-e.zip` – SHA-256 `ff53d579664610eb4139f6bd50148edc60fe48d7aa1dba64306c6ea2ab2ce3d2`.

### 5.6 SWMI Mobility 1 – Paket A: MLE-Zellwechselbasis

**Implementiert:**

- Codecs und lokale Runtime für `U-PREPARE`, `D-NEW-CELL`, `D-PREPARE-FAIL`, `U-RESTORE`, `D-RESTORE-ACK`, `D-RESTORE-FAIL`, `U-CHANNEL-REQUEST`, `D-CHANNEL-RESPONSE`;
- Erhalt eingebetteter MM-/OTAR-/CMCE-SDUs;
- `crates/tetra-entities/src/mle/cell_change_runtime.rs`;
- Zustände für Prepare, Restore, Channel Request und Timeout;
- Zwei-Zellen-Testgrundlage.

**Historische Grenze:** CMCE besaß an diesem Punkt noch keine vollständige Restore-State-Machine. Daher wurde ein Restore zunächst konservativ mit `D-RESTORE-FAIL` beantwortet. Diese Grenze wurde unmittelbar im nächsten Paket ersetzt.

**Artefakt:** `netcore-tetra-swmi-mobility1-package-a.zip` – SHA-256 `dc295de64b5e6a7b36a10bedbe2d232b175f1ff84e542ba7e111d910b9245fb1`.

### 5.7 SWMI Mobility 1 – Paket B: vollständige CMCE Call-Restore-State-Machine

Wiederherstellung laufender Gruppen- **und** Einzelrufe ist als unmittelbar erforderlicher Ausbau festgelegt.

**Implementiert:**

- Restore laufender Gruppenrufe;
- Restore laufender Simplex- und Duplex-Individualrufe;
- Übernahme von Priorität, Call Origin, Floor Holder und T310-Bezug;
- Wiederverwendung bzw. Neuvergabe lokaler Call-IDs;
- Warteschlange bei belegten Traffic Channels;
- `RequestQueued`, `Granted`, `GrantedToOtherUser`, `NotGranted`;
- Floor Queue, Floor Release und spätere Sprecherübergabe;
- Duplicate-/Replay-Schutz und Cleanup;
- Support-Profil bewusst begrenzt auf unverschlüsselte TCH/S-Sprachdienste des vorhandenen NetCore-Modells.

**Bewusste Restgrenze:** netzweiter Transport des Restore Context zwischen physischen TBS war noch nicht vorhanden; dieser wurde für `mobility-core`/`call-control` vorgesehen.

**Artefakt:** `netcore-tetra-swmi-mobility1-package-b-call-restore.zip` – SHA-256 `853a38850849a8658000d335da980bb2ac5e7aa511dbd3dec6478faf32174e19`.

### 5.8 SWMI Mobility 1 – Paket C: MM Migration und Forward Registration

**Implementiert:**

- zweistufige Migration mit VASSI;
- `D-LOCATION-UPDATE-PROCEEDING`;
- zweite `U-LOCATION-UPDATE-DEMAND` zum Abschluss;
- Context Export/Import;
- Forward Registration aus `U-PREPARE`;
- Übernahme von Gruppenaffiliationen, Energy-Saving, Monitoring, Class of MS, TEI und Handle;
- Zuordnung lokaler VASSI zur Home-ISSI;
- Whitelist-/Admission-Prüfung gegen die Home-ISSI statt gegen die temporäre VASSI;
- Default-VASSI-Pool `0xE00000..0xEFFFFE`.

**Artefakt:** `netcore-tetra-swmi-mobility1-package-c-mm-mobility.zip` – SHA-256 `72201c4e73b9cf7396f7af532790882058177c8d687595180a7ba797a254ff22`.

### 5.9 SWMI Mobility 1 – Paket D: Node Gateway, erster echter LXC

Für die weitere Testphase wurde festgelegt: **in der Testumgebung vorerst keine Tokens**.

**Implementiert:**

- erster tatsächlich deploybarer Backend-LXC unter `system-backend/node-gateway/`;
- TBS-WebSocket `WS /ws/node`;
- Backend-WebSocket `WS /ws/backend`;
- vorhandenes Subprotokoll `netcore-control-room-node-v1` zur TBS-Kompatibilität;
- Hello/HelloAck, Heartbeat, Telemetrie, ACK/Response/Error;
- Duplicate-Node-Erkennung;
- REST-API, OpenAPI, Prometheus und eigene WebUI;
- systemd-Unit und Install-/Update-/Uninstall-Skripte;
- `security.mode = "open_lab"`, keine Tokenfelder, kein TLS.

**Historischer Standardport:** `8080`.

**Artefakt:** `netcore-tetra-swmi-mobility1-package-d-node-gateway-open-lab.zip` – SHA-256 `92f1f0deea93749d90f6f29b0487a2087bdee4977b83c26f2d049e6995c355c3`.

### 5.10 SWMI Mobility 1 – Paket E: Mobility Core

**Implementiert:**

- zweiter eigenständiger LXC `system-backend/mobility-core/`;
- zentrale Teilnehmerlage aus TBS-Telemetrie;
- dreistufiger Context Transfer: Quelle exportieren → Ziel importieren → Quelle bereinigen;
- Request-/Command-Korrelation;
- Timeout, Cancel und Fehlerzustände;
- eigene REST-API/WebUI/Metriken/OpenAPI;
- weiterhin `open_lab` ohne Token/TLS.

**Historischer Standardport:** `8090`.

**Artefakt:** `netcore-tetra-swmi-mobility1-package-e-mobility-core-open-lab.zip` – SHA-256 `fd2295a678948b987a088e060a3a606857b068072e410bbe06e4a2ac68c8b2b2`.

### 5.11 SWMI Core 1 – Paket A: Subscriber Core

**Implementiert:**

- zentraler Teilnehmerstammdatendienst;
- persistente atomare JSON-Datenbank;
- `allow_list` und `open_network`;
- explizites Deny-All bei leerer Allow-List;
- zentrale TBS-Admission-Policy;
- sofortiges Re-Registration/Trennen nicht mehr berechtigter Teilnehmer;
- langlebige `lokale VASSI → Home-ISSI`-Zuordnung;
- eigene WebUI, REST, Metriken, OpenAPI;
- `open_lab` ohne Tokens/TLS.

**Historischer Standardport:** `8100`.

**Artefakt:** `netcore-tetra-swmi-core1-package-a-subscriber-core-open-lab.zip` – SHA-256 `f564d24a6fe2bf0afd630c544ef7f6da31cd16d30cc23cceab21ff4404b54d25`.

### 5.12 SWMI Core 1 – Paket B: Group Core

**Implementiert:**

- zentrale GSSI-Stammdaten;
- Teilnehmermitgliedschaften, Auto-Attach und Locked-Markierung;
- Bereichsfilter pro TBS;
- versionierte TBS-Gruppenpolicy;
- lokale Gruppenattach-/DGNA-/Gruppenruf-/Notrufprüfung;
- zentrale Mindestpriorität und Class of Usage;
- korrelierte DGNA-Vorgänge;
- eigene WebUI/REST/Metriken/OpenAPI;
- `open_lab` ohne Tokens/TLS.

**Historischer Standardport:** `8110`.

**Fehler und Lösung:** Der erste finale Verpackungslauf scheiterte, weil `tools/protocol_inventory.py --check` einen veralteten Inventarstand meldete. Nach Aktualisierung/Neulauf wurde das Paket erneut validiert und erfolgreich erstellt.

**Artefakt:** `netcore-tetra-swmi-core1-package-b-group-core-open-lab.zip` – SHA-256 `7921ad118b22a104524d3d45832a94ed8d09e0fd215bc61a60b03f4ea711ff09`.

### 5.13 SWMI Core 1 – Paket C: Call Control

**Implementiert:**

- zentrale logische Gruppen- und Individualrufe;
- TBS-Call-Legs mit lokaler Call-ID, Timeslot und Carrier;
- Floor Request, Queueing, Release und Labor-Force-Floor;
- Teilnehmerinitiierte Call-Erkennung aus Telemetrie;
- mehrzellige Call-Restore-Koordination;
- Export/Import/Cleanup von Restore Context;
- persistente Call-/Leg-/Restore-Historie;
- eigene WebUI/REST/Metriken/OpenAPI;
- `open_lab` ohne Tokens/TLS.

**Historischer Standardport:** `8120`.

**Fehler und Lösung:** Der erste finale Validierungslauf konnte `/tmp/netcore-call-control-webui.js` wegen `PermissionError` nicht schreiben. Die Prüfung wurde anschließend mit einer Datei unter `/mnt/data` erneut ausgeführt und erfolgreich abgeschlossen. Das war ein Prüfpfadfehler, kein nachgewiesener Rust-Codefehler.

**Artefakt:** `netcore-tetra-swmi-core1-package-c-call-control-open-lab.zip` – SHA-256 `037726fc224725db7c5bac6dee28ad46382ff511c9329c7b0e13e97e7da4bff1`.

### 5.14 SWMI Core 1 – Paket D: Media Switch

**Implementiert:**

- erster netzweiter Sprachframepfad TBS → Node Gateway → Media Switch → Ziel-TBS;
- Routing bereits codierter TETRA-Speech-Service-0-Frames;
- 274 Nutzbits / 35 gepackte Bytes pro Frame;
- keine Transkodierung;
- Call-Control-Abgleich und Leg-Routing;
- begrenzte Jitter-Puffer;
- Duplicate-/Unknown-Stream-/Overload-Schutz;
- Stream-Mute, Session-Flush und Testframe-Injection;
- Diagnose-/Recorder-Tap-Anschlussstellen;
- eigene WebUI/REST/Metriken/OpenAPI;
- `open_lab` ohne Tokens/TLS.

**Historischer Standardport:** `8130`.

**Artefakt:** `netcore-tetra-swmi-core1-package-d-media-switch-open-lab.zip` – SHA-256 `4721caf2aa29ade70cc865cc6b362fc64fcaea1b776ce359fc58f82d434b93b7`.

### 5.15 Recorder: in der Planung nur als nächster Schritt begonnen

Nach dem Media Switch wurde als nächster LXC der **Recorder** angekündigt: Übernahme von Media-Taps/Sprachframes, Zusammensetzen netzweiter Calls, Metadaten und eigene offene WebUI.

Für den Recorder enthalten C und H noch keinen Implementierungsbericht oder ein geprüftes ZIP. **Historischer Entwicklungsstatus: als nächster Schritt geplant.** Spätere Repository-Implementierung wird in Abschnitt 9 getrennt bewertet.

Dieser Punkt ist wichtig, weil der am Prüfdatum geprüfte Repository-Stand inzwischen bereits einen Recorder enthält; siehe Abschnitt 9. Das ist eine spätere/anderweitige Repository-Weiterentwicklung und darf nicht rückwirkend als Ergebnis des hier sichtbaren Recorder-Schritts ausgegeben werden.

## 6. Relevante Dateien und technische Schnittstellen aus den Arbeitsnotizen

### 6.1 Lokale TBS-Runtimes

| Bereich | Relevante Datei/Komponente | Historischer Paketstatus laut H |
|---|---|---|
| TLMC | `crates/tetra-entities/src/umac/tlmc_runtime.rs` | implementiert |
| TLPD | `crates/tetra-entities/src/mle/ltpd_runtime.rs` | implementiert |
| MLE Cell Change | `crates/tetra-entities/src/mle/cell_change_runtime.rs` | implementiert |
| CMCE Call Restore | `crates/tetra-entities/src/cmce/call_restore_runtime.rs` | implementiert |
| MM Mobility | `crates/tetra-entities/src/mm/mobility_runtime.rs` | implementiert |
| Zwei-Zellen-Harness | `crates/tetra-entities/tests/common/two_cell.rs` | implementiert |

### 6.2 Backend-LXC und Ports

| Dienst | Historischer Port | Hauptschnittstelle | Sicherheitsmodus in der Planung |
|---|---:|---|---|
| Node Gateway | 8080 | `/ws/node`, `/ws/backend`, `/api/v1` | open_lab |
| Mobility Core | 8090 | Node-Gateway-Backend-WS, REST | open_lab |
| Subscriber Core | 8100 | REST + Node-Gateway-Backend-WS | open_lab |
| Group Core | 8110 | REST + Node-Gateway-Backend-WS | open_lab |
| Call Control | 8120 | REST + Node-Gateway-Backend-WS | open_lab |
| Media Switch | 8130 | REST + Node-Gateway-Backend-WS | open_lab |
| Recorder | 8140 | in der Planung nur geplant | open_lab geplant |

### 6.3 Wichtige Protokoll-/Datenparameter

- TBS-Kompatibilitätsprotokoll: `netcore-control-room-node-v1`.
- Node Gateway: `WS /ws/node` für TBS, `WS /ws/backend` für Backend-Dienste.
- LTPD/robuste Transfers: Timeout 432 Timeslots.
- MM-Migration: temporärer lokaler VASSI-Pool standardmäßig `0xE00000..0xEFFFFE`.
- Media Switch: TETRA Speech Service 0, 274 Nutzbits, auf 35 gepackte Bytes pro transportiertem Frame aufgerundet.
- Foundation-E-Replay-Schutz: kürzlich abgeschlossene Handles bleiben für eine Hyperframe-Dauer geschützt.

### 6.4 Subscriber Core: im zugänglichen Paket A festgehaltene Details

**Historischer Paketumfang (C):** Teilnehmer anlegen/ändern/löschen; Suche nach ISSI, Name und Organisation; Home-MCC/Home-MNC, Gerätebezeichnung, TEI, Registrierungserlaubnis, Rufpriorität, Notruf-/SDS-/Paketdatenberechtigung und vorbereitete Standardgruppen. JSON-Import/-Export, CSV-Export, beobachtete Funkgeräte, Synchronisationsstatus und Ereignisprotokoll gehören zur eigenen WebUI. Ein gespeichertes Berechtigungsfeld allein belegt noch keine Durchsetzung im jeweiligen Dienst.

Die Datenbank liegt unter `/var/lib/netcore-subscriber-core/subscribers.json`, die Sicherung unter demselben Pfad mit `.bak`. Schema-Version, globale Revision, Teilnehmerrevision und Erstellungs-/Änderungszeiten werden gespeichert. R bestätigt Schreiben einer temporären Datei und `fs::rename`; die vorgeschaltete Sicherung mit `fs::copy` ignoriert ihren Rückgabefehler. „Atomare Ersetzung“ ist deshalb keine vollständige Bestätigung eines fehlerfreien Backups oder eines stromausfallsicheren Speichervorgangs.

| Zugangspolitik | Historisch festgelegte Semantik |
|---|---|
| `allow_list` | Nur Profile mit `enabled=true` und `registration_allowed=true`; leere freigegebene Liste bedeutet **deny all**. |
| `open_network` | Alle ISSIs dürfen sich registrieren; Stammdaten bleiben erhalten. |
| `disconnect_unauthorized=true` | Laut Paketbericht sollten bereits registrierte, neu unberechtigte Geräte getrennt und zur erneuten Registrierung gezwungen werden. Die am Prüfdatum vorliegende TBS-Integration bestätigt dies nicht, siehe 9.7. |

Der historische Datenweg war Subscriber Core → `WS /ws/backend` → Node Gateway → `WS /ws/node` → MM. Vertrag: `SubscriberAccessPolicyApply` / `SubscriberAccessPolicyApplied`, mit Revision, Handle, erlaubten Home-ISSIs, Policy-Modus und Disconnect-Vorgabe. Sync-Zustände: `Pending`, `Requested`, `Applied`, `Failed`, `TimedOut`, `Offline`, `Unsupported`; dazu gewünschte/bestätigte Revision, Request-ID, Command-ID, Zeit und Timeout.

Die Home-ISSI bleibt bei Migration maßgeblich. Beispiel aus C: Home-ISSI `2260575`, lokale VASSI `0xE00000`. `home_issi_by_local_issi` überlebt im Runtime-Modell das Entfernen kurzlebiger Migrationstransaktionen; Detach/Context-Entfernung sollen sie bereinigen. Das bedeutet Langlebigkeit innerhalb dieses Modells, keinen hier nachgewiesenen Erhalt über einen Prozessneustart. C berichtet außerdem die Korrektur eines Gruppenaffiliationszugriffs auf einen nur beim Location Update vorhandenen Migration Context. Die historische fehlerhafte Codeversion ist nicht zugänglich.

**Am Prüfdatum vorliegende Abgrenzung:** Backend-Datenmodell, Persistenz und Policy-Erzeugung sind vorhanden. Die TBS meldet `subscriber_policy=false`; deshalb erzeugt der Core für diese TBS einen `Unsupported`-Status. Die historische Zusage eines vollständig gesicherten Umgangs mit verspäteten Antworten ist für den am Prüfdatum vorliegenden Handler zu weitgehend, siehe 9.7.

### 6.5 Group Core: im zugänglichen Paket B festgehaltene Details

Gruppenprofile enthalten GSSI, Name/Beschreibung, `enabled`, `attach_allowed`, `dgna_allowed`, `call_allowed`, `sds_allowed`, `emergency_allowed`, `call_priority`, `class_of_usage`, `area_nodes` und Notizen. Die in der Planung beispielhaft genannte GSSI `15501` („Hintergrundgruppe“) sollte SDS zulassen, Sprache und Notruf sperren, mit Priorität `0` und Class of Usage `4`. Das ist ein Planungsbeispiel, keine verifizierte am Prüfdatum vorliegende Gruppenkonfiguration.

Mitgliedschaften verknüpfen ISSI/GSSI mit `allowed`, `auto_attach`, `locked` und Notizen. Bei erzwungenen Mitgliedschaften ist ein freigegebener Datensatz erforderlich. Auto-Attach sollte registrierten Geräten bei der Policy-Synchronisation Gruppen per DGNA zuweisen. **`locked` wurde historisch nur gespeichert/transportiert; die dazugehörige Rollen-/Operatorlogik war geplant.** Eine weitergehende Sperrwirkung ist damit nicht belegt.

`area_nodes=[]` bezeichnet globale Gruppen; ansonsten liefert die Policy-Erzeugung einer TBS nur Gruppen ihres Bereichs. Datenbank: `/var/lib/netcore-group-core/groups.json` und `.bak`, mit Schema-/Datenbank-/Datensatzrevisionen und Zeiten. Wie beim Subscriber Core sind temporäre Datei und Rename vorhanden; ein fehlgeschlagenes Backup-Copy wird im am Prüfdatum vorliegenden Persistenzpfad nicht weitergereicht.

Verträge: `GroupAccessPolicyApply` / `GroupAccessPolicyApplied` und `GroupDgnaApply` / `GroupDgnaApplied`. DGNA-Zustände: `Pending`, `Requested`, `Applied`, `Failed`, `TimedOut`, `Cancelled`. Ein Vorgang führt Vorgangs-ID, Node-ID, ISSI, GSSI, Attach/Detach, Force, Request-/Command-ID und Ergebnis. Der am Prüfdatum vorliegende Group-Core-Handler prüft sowohl Command-Zuordnung als auch `desired_revision == revision` und protokolliert verwaiste Antworten.

Historisch beschriebene lokale Prüfungen:

- MM-Affiliation: Gruppe bekannt/aktiv, Attach erlaubt, Teilnehmer zugelassen.
- DGNA: Teilnehmer registriert, GSSI technisch gültig, Gruppe/DGNA freigegeben, Mitgliedschaft und Node-Fähigkeit vorhanden.
- Gruppenruf: `call_allowed`, lokale Affiliation, Notruferlaubnis und Mindestpriorität. Beispiel: angefordert `3`, Mindestwert `7` → effektive Priorität `7`; bei daraus resultierender Notrufpriorität trotzdem Notrufberechtigung prüfen.
- `force=true` sollte Gruppen-/Mitgliedschaftspolicy umgehen; Registrierung, gültige GSSI und DGNA-Unterstützung bleiben erforderlich. Dies war ein Operator-Override im offenen Labormodus, kein RBAC-geschütztes Recht.

**Am Prüfdatum vorliegende Abgrenzung:** Die Backend-Verwaltung und Policy-Erzeugung sind vorhanden. Der aktive TBS-Konstruktor setzt `group_policy=false`, und MM behandelt die zentralen `GroupAccessPolicyApply`-/`GroupDgnaApply`-Kommandos nicht. Der vorhandene lokale `Dgna`-Pfad ist davon zu unterscheiden. Die historischen Aussagen über lokale Policy-Durchsetzung dürfen nicht als am Prüfdatum vorliegender End-to-End-Nachweis fortgeschrieben werden.

### 6.6 Call Control: Call-Legs, Floor und Restore

Call Control ist Eigentümer des **logischen netzweiten Calls**. Ein lokales TBS-Leg besitzt Node-ID, lokale Call-ID, Operation-ID, Zustand, Timeslot, Carrier, Usage, Floor Holder, wartende ISSI, Command-ID und Restore-Markierung. Ein Gruppenruf kann über mehrere unterschiedliche lokale Timeslots laufen; eine logische Call-ID ersetzt keine lokale Funkressourcenkennung.

| Ebene | Zustände aus C, in R als Enumerationen vorhanden |
|---|---|
| Logischer Call | `Starting`, `Partial`, `Active`, `Releasing`, `Ended`, `Failed`, `Interrupted` |
| TBS-Leg | `Requested`, `Starting`, `Active`, `Releasing`, `Ended`, `Failed`, `TimedOut`, `Offline` |
| Restore | `ExportQueued`, `ExportRequested`, `ImportQueued`, `ImportRequested`, `Ready`, `Completed`, `Cancelled`, `Failed`, `TimedOut` |

Historische Zielauswahl: explizite TBS-Liste, anschließend beobachtete Teilnehmer mit passender GSSI-Affiliation, danach geeignete online Call-Control-Nodes als Fallback. Individualrufe führen Calling/Called ISSI, Simplex-/Duplex-Auswahl und Priorität. Der am Prüfdatum vorliegende Stand enthält zusätzlich eine Mobility-Core-Konfiguration; in der Beispielkonfiguration sind `allow_local_fallback=false` und `accept_stale_route=false`. Die alte Fallback-Beschreibung ist daher keine uneingeschränkte am Prüfdatum vorliegende Routinggarantie.

Floor Control umfasst Anforderung, Warteschlange, Freigabe und Übertragung auf aktive Legs. `force=true` erfordert zusätzlich `[calls].allow_operator_force_floor=true`. Auf dem am Prüfdatum vorliegenden Stand kommen Media-Route-Ready und Revisionsprüfung hinzu; der historische HTTP-Polling-Stand allein beschreibt diesen Ablauf nicht mehr vollständig.

Historischer Restore-Ablauf: Quelle exportiert Context → Ziel importiert Context → Call Control erzeugt Ziel-Platzhalter → echte Zieltelemetrie liefert das restaurierte Leg → Call-ID/Funkressourcen werden übernommen → temporärer Ziel-Context wird mit separat korreliertem Kommando entfernt. Eine reine Importbestätigung sollte den Restore **nicht** als abgeschlossen markieren. Scheitert nur das Cleanup, bleibt der bereits restaurierte Ruf bestehen; Timeout/Cancel/Fehler sollen Pending Commands und Platzhalter bereinigen. R enthält diese zentrale Zustandslogik, bestätigt aber keine aktive TBS-Restore-Unterstützung.

Zentrale Kommandos aus C: `CallControlGroupStart`, `CallControlIndividualStart`, `CallControlRelease`, `CallControlFloorRequest`, `CallControlFloorRelease`, `CallControlExportRestoreContext`, `CallControlImportRestoreContext`, `CallControlRemoveRestoreContext`. Antworten: `CallControlLegStarted`, `CallControlLegReleased`, `CallControlFloorChanged`, `CallControlRestoreContextExported`, `CallControlRestoreContextImported`, `CallControlRestoreContextRemoved`.

Persistenz: `/var/lib/netcore-call-control/calls.json` und `.bak`. Calls, Legs, Floor und Restore werden gespeichert. Beim Neustart werden zuvor laufende Calls im am Prüfdatum vorliegenden Ladepfad als `Interrupted` markiert; damit wird unbekannter Funkzustand nicht als sicher weiter aktiv ausgegeben. In der Beispielkonfiguration stehen 30 s Command-Timeout, 45 s Restore-Timeout und 2 s Reconcile-Intervall. Diese Werte sind Vorgaben im Repository, keine aus einem LXC gelesene Laufzeitkonfiguration.

### 6.7 Media Switch: Frameformat, Puffer und spätere Änderungen

C beschreibt ausschließlich **TETRA Speech Service 0**, 274 Nutzbits in 35 gepackten Bytes. 35 Bytes sind der auf ganze Bytes gerundete Transportcontainer, nicht 280 Sprach-Nutzbits. Der Media Switch soll die codierten Frames unverändert weiterreichen; Transkodierung, PCM-/WAV-Decoding, weitere Sprachdienste und E2EE-Verarbeitung gehörten nicht zum historischen Paket D.

Der Datenweg führt vom UMAC-Uplink über begrenzte nicht blockierende In-Process-Queues zum Node-Worker, zum Node Gateway und zum Media Switch; zurück über Gateway/Node-Worker in einen aktiven UMAC-Downlink-Circuit. C beschrieb Verteilung an andere aktive Legs desselben logischen Calls; lokale Wiedergabe der Quell-TBS bleibt im lokalen UMAC-Pfad. R bestätigt die Bridge-Einbindung und `try_send`/`try_recv`, aber keine gemessene Funklatenz.

Das Gateway-Abonnement lautet `{"kind":"subscribe","topics":["media_frames"]}`. C sieht keine automatische Sprachframe-Flut an Subscriber/Group/Mobility/Call Control und keine History-Einträge pro Sprachframe vor. Zu den beschriebenen Schutzmaßnahmen gehören Sequenz-/Duplikaterkennung, begrenzte Queues, Abwurf unbekannter/offline/nicht mediafähiger Streams und Reset bei neuer TBS-Session. R ergänzt eine wichtige Bindung: `MediaDownlinkFrame.session_id` bezeichnet die **Ziel-Leg-Operation-ID**; die UMAC-Prüfung soll alte Frames nach Wiederverwendung eines Bearers abweisen.

| Parameter/Kopplung | Paket D laut C | Beispielkonfiguration/Code bei R |
|---|---|---|
| Call-Control-Transport | HTTP `GET /api/v1/calls`, alle 2 s | WebSocket `/ws/media`; HTTP-Reconcile alle 15 s als Fallback/Sicherheitsnetz |
| Media-Bereitschaft | im zugänglichen Pakettext kein RouteReady-Handshake | revisionsgebundenes `POST /api/v1/media/route-ready` |
| Framezeit | 60 ms | 60 ms |
| Jitterpuffer | 3 Frames Startwert, höchstens 12 | adaptiv, Start 2, Minimum 1, Maximum 12 Frames |
| Adaptive Grenzen | nicht im Pakettext | 18 ms Schwelle zum Erhöhen; 120 stabile Frames zum Senken |
| Kaltstart | nicht im Pakettext | 5 Frames, höchstens 600 ms alt |
| Verarbeitung/Session | 256 Frames/Tick; 30 s Idle | dieselben Beispielwerte |
| Diagnose-Tap | 256 Metadateneinträge | weiterhin 256, ohne Payload |
| Recorder-Vollframe-Ring | Recorder nur vorbereitet | 20.000 Frames; `/api/v1/recorder/taps?after=<seq>&limit=<n>` |
| Grenzen | global begrenzt, ohne alle Zahlen im Text | 10.000 Sessions, 50.000 Streams, 100.000 Pending Frames |

Historische Verwaltungsaktionen: `POST /api/v1/sessions/{session_id}/mute` mit `node_id`, `logical_ts`, `muted`; `/flush` leert den Puffer; `/inject` nimmt einen bereits gepackten 35-Byte-Frame an, optional auf Ziel-TBS/Timeslot begrenzt. Vollständige WAV-/MP3-Dateiwiedergabe war damit noch nicht implementiert. Der Metadaten-Tap führte Session, Quell-TBS/-Timeslot, Sequenz, Zielanzahl, Payloadgröße und Injected-Markierung. Der spätere Recorder verwendet den gesonderten Vollframe-Tap; langsames Polling kann dessen Ring überlaufen lassen, was als `dropped_before` sichtbar wird.

### 6.8 REST-Oberflächen, Betriebsdateien und Abhängigkeiten

Alle vier Core-Dienste aus C besitzen `/`, `/health/live`, `/health/ready`, `/api/v1/status`, `/api/v1/nodes`, `/api/v1/events`, `/api/v1/config`, `/metrics` und `/openapi.json`. Die am Prüfdatum vorliegenden HTTP-Handler bestätigen diese Grundflächen. Health/Readiness sind Dienstprüfungen; ein erfolgreicher HTTP-Status ersetzt keine Multi-TBS-/Funkabnahme.

| Dienst | Weitere relevante REST-Routen aus C |
|---|---|
| Subscriber Core | `GET/POST /api/v1/subscribers`; `GET/PUT/DELETE /api/v1/subscribers/{issi}`; `GET /api/v1/observed`; `GET /api/v1/syncs`; `POST /api/v1/sync`; `POST /api/v1/import`; `GET /api/v1/export.json`, `/api/v1/export.csv` |
| Group Core | `GET/POST /api/v1/groups`; `GET/PUT/DELETE /api/v1/groups/{gssi}`; `GET/POST /api/v1/memberships`; `DELETE /api/v1/memberships/{issi}/{gssi}`; `GET /api/v1/affiliations`; `GET /api/v1/syncs`; `POST /api/v1/sync`; `GET/POST /api/v1/dgna`; `POST /api/v1/dgna/{id}/cancel`; JSON-Import/-Export |
| Call Control | `GET /api/v1/participants`; `GET /api/v1/calls`, `/api/v1/calls/{logical_call_id}`; `POST /api/v1/calls/group`, `/api/v1/calls/individual`; `POST /api/v1/calls/{logical_call_id}/release`, `/floor`, `/floor/release`; `GET/POST /api/v1/restores`; `POST /api/v1/restores/{restore_id}/cancel` |
| Media Switch | `GET /api/v1/sessions`, `/api/v1/sessions/{session_id}`, `/api/v1/streams`, `/api/v1/buffers`, `/api/v1/taps`; `POST /api/v1/sessions/{session_id}/mute`, `/flush`, `/inject`; `POST /api/v1/gateway/ping` |

R enthält zusätzlich Call Controls `/ws/media`, `/api/v1/media/route-ready`, `/api/v1/events/netcore` und den Media-Switch-Vollframe-Tap. Maßgeblich für genaue Payloads und Fehlercodes sind die Handler `system-backend/<dienst>/src/http.rs` und Modelle in `state.rs`; die hier inventarisierten Routen wurden nicht live aufgerufen.

Deploybare Dienste liegen unter `system-backend/<dienst>/` mit `Cargo.toml`, `src/{main,config,protocol,state,gateway,http}.rs`, Beispiel-TOML, systemd-Unit, `install/{install,update,uninstall}.sh` und Dienstdokumentation; Media Switch ergänzt `src/call_control.rs`. TBS-Kommandotypen liegen in `crates/tetra-entities/src/net_control/commands.rs`, das Node-Protokoll in `net_control_room/protocol.rs`, die Bridge in `net_media/mod.rs`.

| Dienst | Unit / Binary | Konfiguration | Zustandsablage |
|---|---|---|---|
| Subscriber Core | `netcore-subscriber-core.service` / `/usr/local/bin/netcore-subscriber-core` | `/etc/netcore/subscriber-core.toml` | `/var/lib/netcore-subscriber-core/subscribers.json` + `.bak` |
| Group Core | `netcore-group-core.service` / `/usr/local/bin/netcore-group-core` | `/etc/netcore/group-core.toml` | `/var/lib/netcore-group-core/groups.json` + `.bak` |
| Call Control | `netcore-call-control.service` / `/usr/local/bin/netcore-call-control` | `/etc/netcore/call-control.toml` | `/var/lib/netcore-call-control/calls.json` + `.bak` |
| Media Switch | `netcore-media-switch.service` / `/usr/local/bin/netcore-media-switch` | `/etc/netcore/media-switch.toml` | begrenzte Session-/Puffer-/Tap-Laufzeitdaten; keine hier belegte persistente Session-Datenbank |

Die systemd-/Installationsvorlagen setzen Linux mit systemd, Rust-Buildumgebung und den Dienstbenutzer `netcore` voraus. Der geprüfte Subscriber-Installer lädt außerdem `system-backend/shared/install/lxc-network.sh` und ruft `netcore_configure_lxc_endpoint` auf. Platzhalter wie `10.0.1.XX` in Beispielkonfigurationen sind vor Installation durch die tatsächlichen Dienstadressen zu ersetzen; sie sind keine bestätigten Zielhosts.

## 7. Deployment-, Build- und Reparaturabläufe

Die Paketdokumentationen und H sahen wiederholt den folgenden Clean-Deploy-Ablauf vor. **Historische Anleitung, in dieser Fortsetzung nicht ausgeführt; keine bestätigte Installation.** Er ist vor einer Wiederverwendung an vorhandene Konfigurationen, Datenbestände, aktive Units und den tatsächlich gewählten Softwarestand anzupassen:

1. laufenden Dienst stoppen;
2. Laufzeitkonfiguration sichern;
3. vollständiges Repository bzw. Paket ersetzen;
4. alte `target/`-Artefakte und veraltete Binaries entfernen;
5. `cargo clean`;
6. statische Checker und gezielte Cargo-Tests ausführen;
7. Release-Binary neu bauen;
8. systemd installieren/aktualisieren;
9. Dienst starten und Logs/Health-Endpunkte kontrollieren;
10. bei Fehlern auf gesicherten Stand zurückrollen.

Historische Prüfbefehle aus den Arbeitsnotizen bzw. den Paketdokumenten. Die 13 Strukturchecker und die Inventur wurden bei der erneuten Prüfung ausgeführt, mit den in Abschnitt 9.8 aufgeführten, überwiegend negativen Ergebnissen:

```bash
python3 -S tools/check_swmi_foundation_types.py
python3 -S tools/check_tlmc_runtime.py
python3 -S tools/check_ltpd_runtime.py
python3 -S tools/check_foundation_acceptance.py
python3 -S tools/check_mle_cell_change.py
python3 -S tools/check_cmce_call_restore.py
python3 -S tools/check_mm_mobility.py
python3 -S tools/check_node_gateway.py
python3 -S tools/check_mobility_core.py
python3 -S tools/check_subscriber_core.py
python3 -S tools/check_group_core.py
python3 -S tools/check_call_control.py
python3 -S tools/check_media_switch.py
python3 -S tools/protocol_inventory.py --check
```

Vorgesehene Rust-Prüfungen, im damaligen Artefaktcontainer **nicht lokal ausgeführt**:

```bash
cargo fmt --all -- --check
cargo test -p tetra-saps
cargo test -p tetra-entities
cargo test -p netcore-node-gateway
cargo test -p netcore-mobility-core
cargo test -p netcore-subscriber-core
cargo test -p netcore-group-core
cargo test -p netcore-call-control
cargo test -p netcore-media-switch
cargo clippy --all-targets -- -D warnings
```

Die GitHub-Actions-Workflows wurden deshalb als eigentliche Rust-Kompilations-/CI-Stufe vorgesehen. Ein vorhandener Workflow im Repository ist kein Nachweis, dass genau der zugehörige Commit erfolgreich auf GitHub Actions lief, sofern kein Run geprüft wurde.

### 7.1 Historische Installation und am Prüfdatum vorliegender Prüfbedarf

C schlug für Subscriber Core direkt `sudo system-backend/subscriber-core/install/install.sh` vor, für Group Core, Call Control und Media Switch zunächst das Kopieren der jeweiligen Beispielkonfiguration nach `/etc/netcore/`, deren Bearbeitung und anschließend denselben Installerpfad. Danach sollten `systemctl status`, `journalctl` und lokale `curl`-Abfragen Erfolg prüfen. **Für keinen dieser Befehle liegt in den verfügbaren Entwicklungsnotizen ein erfolgreicher Zielsystemlauf vor.**

Beispiel für die in der Planung vorgesehenen Diagnosebefehle, hier nur dokumentiert:

```bash
systemctl status netcore-media-switch --no-pager
journalctl -u netcore-media-switch -n 150 --no-pager
curl http://127.0.0.1:8130/health/live
curl http://127.0.0.1:8130/health/ready
curl http://127.0.0.1:8130/api/v1/status
curl http://127.0.0.1:8130/api/v1/sessions
```

Vor Wiederverwendung ist die tatsächliche Bind-Adresse zu prüfen: eine durch den Netzwerkhelfer auf die LXC-IP angepasste Bindung muss nicht auf `127.0.0.1` erreichbar sein. Vor dem historischen `cp` muss `/etc/netcore/` existieren; vorhandene Konfigurationen sollen nicht durch unbereinigte Beispiele ersetzt werden.

R zeigt beim Subscriber-Installer: Dienst stoppen, altes Binary löschen und bestimmte Release-Artefakte entfernen **vor** dem Neubau. Ein Buildfehler kann deshalb ein gestopptes Ziel ohne vorheriges Binary hinterlassen. Die oben erhaltene Clean-Build-Reihenfolge ist kein ungeprüft wiederzuverwendender Reparaturbefehl. Für eine spätere Betriebsänderung zuerst den genauen Commit, Binary, Unit, Konfiguration und Datenbank einschließlich Berechtigungen sichern, Build/Kompatibilität getrennt prüfen und einen getesteten Rückweg bereithalten. Auf knappen TBS-Systemen darf `cargo clean` nicht ohne konkreten Grund den vorhandenen Buildbestand beseitigen. Diese Fortsetzung führt keinen solchen Eingriff durch.

Die historischen vollständigen Abläufe bleiben unter `Docs/SWMI_CORE_1_PACKAGE_A_APPLY.md` bis `Docs/SWMI_CORE_1_PACKAGE_D_APPLY.md` auffindbar. Ihre Existenz ist keine Bestätigung einer Installation oder eines funktionierenden Rollbacks.

## 8. Fehler, Diagnose und daraus resultierende Regeln

Die historischen Fehler in 8.2–8.4 stammen aus H; die vollständigen damaligen Toollogs sind zum Prüfdatum nicht zugänglich. Sie werden als damaliger Bericht erhalten, nicht als in dieser Fortsetzung neu beobachtete Fehler. Die am Prüfdatum vorliegenden Checkerbefunde stehen in 9.8.

### 8.1 Keine Rust-Toolchain in der Artefaktumgebung

Mehrfach wurde ausdrücklich festgestellt, dass `cargo`/`rustc` im damaligen Ausführungscontainer nicht verfügbar waren. Daraus folgt:

- Static Checker und Syntaxprüfungen sind belastbar als solche;
- ein echter Rust-Build ist für diese Paketläufe **nicht** bestätigt;
- Aussagen wie „kompiliert“ oder „läuft“ wären ohne CI-/Zielsystemnachweis unzulässig.

### 8.2 Paket-C-Prüfung mit Timeout

Der unabhängige Recheck des Paket-C-ZIPs wurde im sichtbaren Lauf durch die 60-Sekunden-Grenze des sichtbaren Python-Runtimes unterbrochen. Spätere Gesamtpakete führten den C-Checker erfolgreich aus. Diese spätere Bestätigung ersetzt jedoch keinen dokumentierten echten Cargo-Build.

### 8.3 Group-Core-Inventar war zunächst nicht aktuell

Der erste Group-Core-Verpackungslauf endete mit `CalledProcessError` bei `tools/protocol_inventory.py --check`. Nach Korrektur/Neugenerierung wurde die Prüfung erneut gestartet und erfolgreich abgeschlossen. Funktionierende Lösung: Inventar vor finalem Packaging synchronisieren und danach den fertigen ZIP-Inhalt erneut prüfen.

### 8.4 Call-Control-WebUI-Check scheiterte zuerst an `/tmp`

Der erste Validierungsversuch schlug mit `PermissionError: /tmp/netcore-call-control-webui.js` fehl. Die WebUI-Prüfung wurde auf einen beschreibbaren Pfad unter `/mnt/data` umgestellt und danach erfolgreich mit `node --check` ausgeführt.

### 8.5 Offener Labormodus ist absichtlich unsicher

Die fehlende Authentisierung ist kein Bug, sondern eine ausdrückliche Testphasenentscheidung. Gleichzeitig wurde mehrfach dokumentiert, dass diese Dienste ausschließlich in einem isolierten Testnetz betrieben werden dürfen.

## 9. Erneut geprüfter Repository-Stand am 2026-10-06

Dieser Abschnitt beschreibt **nicht den historischen Planungsstand**, sondern R auf `Archiving` bei `dac5f0063e9fe06b62157c6b11a351a2ce476fdb`. Die früheren Readme-/Existenzbefunde wurden durch die unten angegebenen Codepfade und tatsächlich ausgeführten Strukturprüfungen präzisiert.

### 9.1 Quellcode der zentralen Runtimes ist vorhanden

Folgende Dateien wurden direkt im Branch `Archiving` gefunden:

- `crates/tetra-entities/src/umac/tlmc_runtime.rs`;
- `crates/tetra-entities/src/mle/ltpd_runtime.rs`;
- `crates/tetra-entities/src/mle/cell_change_runtime.rs`;
- `crates/tetra-entities/src/cmce/call_restore_runtime.rs`;
- `crates/tetra-entities/src/mm/mobility_runtime.rs`.

Damit sind die Runtime-Quelldateien **im Repository vorhanden**. Ihre Integration in aktive MM-/MLE-/CMCE-/UMAC-Instanzen ist separat zu prüfen; mehrere historische Adapter fehlen im am Prüfdatum vorliegenden Stand (Abschnitt 9.7). Dies ist kein vollständiger Funktions- oder Betriebsnachweis.

### 9.2 LXC-Crates sind zum Prüfdatum im Workspace vorhanden

`Cargo.toml` des geprüften Branches führt unter anderem als Workspace-Member:

- `system-backend/node-gateway`;
- `system-backend/mobility-core`;
- `system-backend/subscriber-core`;
- `system-backend/group-core`;
- `system-backend/call-control`;
- `system-backend/media-switch`;
- `system-backend/recorder`;
- sowie zahlreiche später hinzugekommene Dienste.

Die `Cargo.toml`-Dateien dieser sieben Dienste wurden im Branch direkt gefunden.

### 9.3 Open-Lab-Konfiguration zum Prüfdatum

`system-backend/services.toml` enthält weiterhin für Node Gateway, Mobility Core, Subscriber Core, Group Core, Call Control, Media Switch und Recorder die offenen Laborparameter mit HTTP, ohne TLS und ohne Token-Authentisierung. Außerdem ist `webui_defaults.required = true` gesetzt.

Wichtig: Der **zusätzliche gesamte Repository-Stand ist nicht mehr vollständig tokenfrei**. Beispielsweise enthält `services.toml` inzwischen einen späteren `alert-service` mit `security_mode = "token"` und `token_auth = true`. Die Open-Lab-Festlegung bezog sich auf die damalige Testphase und die hier eingeführten Dienste; sie darf nicht pauschal auf alle später hinzugekommenen Services übertragen werden.

### 9.4 Media Switch ist zum Prüfdatum weiterentwickelt

Historischer Planungsstand: Call Control wurde vom Media Switch zunächst zyklisch über `/api/v1/calls` abgeglichen.

Geprüfter Branchstand: `system-backend/media-switch/src/call_control.rs` verarbeitet den ereignisgesteuerten Call-Control-WebSocket; `system-backend/call-control/src/http.rs` stellt `/ws/media` sowie `POST /api/v1/media/route-ready` bereit. Die README nennt das Subprotokoll `netcore-call-control-media-v1`. Zusätzlich existieren revisionsgebundene Route-Ready-ACKs und ein Kaltstart-Vorpuffer. Call Control prüft die Media-Bereitschaft vor der Floor-Vergabe. Das ist Quellcodebefund, keine gemessene Echtzeitgarantie.

Das ist eine **spätere Weiterentwicklung** und ersetzt den älteren Polling-Schwerpunkt des Entwicklungspakets.

### 9.5 Recorder ist zum Prüfdatum implementiert, obwohl er in der Planung noch offen war

Im Branch existiert inzwischen `system-backend/recorder/` als Workspace-Crate. Die gelesene README beschreibt:

- Polling des replay-fähigen Vollframe-Taps `GET /api/v1/recorder/taps?after=<seq>&limit=<n>`;
- verlustfreie Ablage der 35-Byte-TETRA-ACELP-Frames in `audio.tacelp`;
- `frames.jsonl`, Metadaten und SHA-256-Integritätsdateien;
- Recovery unfertiger `.part`-Aufnahmen;
- Retention, Legal Hold, Löschen und TAR-Export;
- eigene WebUI auf Port `8140`;
- weiterhin `open_lab`.

Dieser zusätzliche Repository-Befund ist **nicht** als in dieser Entwicklungsphase fertiggestellter Recorder zu kennzeichnen. In der Planung wurde der Recorder nur als nächster Schritt angekündigt; ein entsprechender Implementierungsbericht liegt für diese Phase nicht vor.

### 9.6 Branchbezug und überholte Vergleichswerte

Die Erstfassung nannte `main` bei `7137e0dd69877e1b604bf89148fd8b6b590c1a97` und einen Abstand von 47 Commits vor/1 hinter `main`. Das sind **historische Vergleichswerte aus H**, keine am Prüfdatum vorliegenden Zahlen. Der Abgleich vom 06.10.2026 bezieht sich auf `Archiving`; ein neuer Vergleich mit `main` liegt nicht vor.

### 9.7 Wichtige Implementierungsbehauptungen gegen aktiven Code geprüft

**Der stärkste am Prüfdatum vorliegende Unterschied zum historischen Paketstand liegt an der TBS-Grenze.** Der tatsächlich verwendete Capability-Konstruktor `ControlRoomNodeCapabilities::from_stack_config` setzt:

```rust
subscriber_policy: false,
group_policy: false,
call_control: true,
call_restore_context: false,
media_bridge: cfg.control_room.as_ref().is_some_and(|control| control.enabled),
```

Nachweis am Prüfcommit: [TBS-Capabilities, `protocol.rs`](https://github.com/JanHG98/netcore-tetra/blob/dac5f0063e9fe06b62157c6b11a351a2ce476fdb/crates/tetra-entities/src/net_control_room/protocol.rs#L119). Testfixtures mit `call_restore_context=true` ändern diesen produktiven Konstruktor nicht. Capability-Felddefinitionen allein sind ebenfalls kein Aktivierungsnachweis.

| Historische Behauptung C/H | Befund R | Konsequenz für die Fortsetzung |
|---|---|---|
| Subscriber-Policy wird bis MM angewendet, leere Liste sperrt alle | Backend erzeugt Policy und `Unsupported` bei fehlender Fähigkeit. `MmBs::tick_start` behandelt auf dem Kontrollkanal nur `Dgna`; Subscriber-Policy-/Mobility-Kommandos fallen in den Unsupported-Zweig. | Zentrale Datenverwaltung ist vorhanden; lokale Durchsetzung der zentralen Policy ist für diese TBS nicht bestätigt. |
| Group Core setzt Affiliation, DGNA und Gruppenrufrechte auf der TBS durch | `group_policy=false`; MM hat keine Handler für `GroupAccessPolicyApply`/`GroupDgnaApply`. Die historischen Policy-Prüfmarker fehlen auch im am Prüfdatum vorliegenden Gruppenruf-Setup. | Gruppenprofile/Revisionen und zentrale Vorgänge sind vorhanden; aktive Durchsetzung separat implementieren und testen. Lokale DGNA-Unterstützung ist kein Beleg für den zentralen Gruppenpolicy-Vertrag. |
| Vollständige Mobility-/TLMC-/TLPD-Integration | Die separaten Runtime-Dateien sind vorhanden; historische Adapterfelder und Aufrufe fehlen in aktiven `umac_bs.rs`, `mle_bs.rs` und `mm_bs.rs`. | Architektur-/Modellstand nicht mit angeschlossenem RF-Pfad gleichsetzen. |
| Mehrzelliger Call Restore vollständig | Zentrale Restore-State-Machine, Platzhalter und Cleanup sind vorhanden. TBS meldet `call_restore_context=false`; CMCE behandelt die Export-/Import-/Remove-Restore-Kommandos nicht. Call Control prüft die Fähigkeit bei der Zielvalidierung. | Aktiver TBS-Restore ist nicht belegt; Backend-Code und ein nomineller Restore-Endpunkt genügen nicht. |
| Zentrale Call-/Floor-Steuerung | `call_control=true`; CMCE behandelt Group/Individual Start, Release, Floor Request/Release und antwortet mit den passenden Responses. | Diese Handler sind tatsächlich eingebunden. Sie dürfen wegen des Startbanners `MAIN-COMPAT` nicht pauschal als bloße Telemetrie bezeichnet werden. Funk-/Buildnachweis bleibt offen. |
| Nicht blockierende TBS-Media-Bridge | `main.rs` erzeugt die Bridge und verbindet UMAC; `net_media/mod.rs` verwendet begrenzte Kanäle und `try_send`/`try_recv`. UMAC behandelt den Downlink und erzeugt Uplinkframes. | Aktive Bridge ist als Code belegt, einschließlich der Bindung an die Ziel-Leg-Operation. Reale Sprachqualität und Lastverhalten wurden nicht getestet. |
| Jede verspätete Subscriber-Antwort ist zuverlässig abgefangen | Bei vorhandener Command-ID wird ein passender Sync gesucht. Ohne Command-ID erfolgt die Zuordnung nach Node; vor `Applied` fehlt ein zusätzlicher Vergleich mit `desired_revision`. | Die uneingeschränkte historische Garantie ist nicht bestätigt. Veraltete/fehlkorrelierte Antworten gezielt testen und den Handler härten. |

Konkrete Codebelege am selben Prüfcommit:

- [MM-Kontrollkanal und Unsupported-Zweig](https://github.com/JanHG98/netcore-tetra/blob/dac5f0063e9fe06b62157c6b11a351a2ce476fdb/crates/tetra-entities/src/mm/mm_bs.rs#L1959), ergänzend die Routingzuordnung in `crates/tetra-entities/src/net_control/worker.rs`: Weiterleitung an MM ersetzt keinen dortigen Handler.
- [Aktive CMCE-Call-/Floor-Kommandos](https://github.com/JanHG98/netcore-tetra/blob/dac5f0063e9fe06b62157c6b11a351a2ce476fdb/crates/tetra-entities/src/cmce/cmce_bs.rs#L62) und [Call-Control-Zielvalidierung](https://github.com/JanHG98/netcore-tetra/blob/dac5f0063e9fe06b62157c6b11a351a2ce476fdb/system-backend/call-control/src/state.rs#L1512).
- [Subscriber-Core-Sync-Erzeugung](https://github.com/JanHG98/netcore-tetra/blob/dac5f0063e9fe06b62157c6b11a351a2ce476fdb/system-backend/subscriber-core/src/state.rs#L711) und [Response-Verarbeitung](https://github.com/JanHG98/netcore-tetra/blob/dac5f0063e9fe06b62157c6b11a351a2ce476fdb/system-backend/subscriber-core/src/state.rs#L797).
- [Media-Bridge-Format und Kanäle](https://github.com/JanHG98/netcore-tetra/blob/dac5f0063e9fe06b62157c6b11a351a2ce476fdb/crates/tetra-entities/src/net_media/mod.rs) sowie [UMAC-Bridge-Einbindung](https://github.com/JanHG98/netcore-tetra/blob/dac5f0063e9fe06b62157c6b11a351a2ce476fdb/crates/tetra-entities/src/umac/umac_bs.rs#L93).
- [Am Prüfdatum vorliegende Media-Switch-Konfiguration](https://github.com/JanHG98/netcore-tetra/blob/dac5f0063e9fe06b62157c6b11a351a2ce476fdb/system-backend/media-switch/config/media-switch.example.toml) und [Call-Control-Anbindung](https://github.com/JanHG98/netcore-tetra/blob/dac5f0063e9fe06b62157c6b11a351a2ce476fdb/system-backend/media-switch/src/call_control.rs).

### 9.8 Tatsächlich ausgeführte Prüfungen dieser Fortsetzung

Ausführung am 2026-10-06 im separaten Windows-Checkout auf `Archiving` bei `dac5f00`. Aufrufschema für die ersten 13 Zeilen: `python -B -X utf8 -S tools/<checker>.py`. `-B` verhindert neue Python-Cachedateien; die vorhandenen Checker wurden unverändert ausgeführt. Es handelt sich überwiegend um Existenz-/Textprüfungen, nicht um Rust-Ausführung.

| Prüfung | Exitcode | Beobachtetes Ergebnis und Einordnung |
|---|---:|---|
| `check_swmi_foundation_types.py` | 1 | Meldet fehlendes `PartialEq, Eq, Hash` für `SsiType`. Gegenprüfung: Derive ist in `crates/tetra-core/src/address.rs` tatsächlich vorhanden. Der Checker betrachtet nur 160 Zeichen vor der Deklaration; eingefügte Kommentare verdrängen die Derive-Zeile aus diesem Fenster. Nachgewiesene Checker-Schwäche, kein damit bewiesener Traitfehler. |
| `check_tlmc_runtime.py` | 1 | Vermisst UMAC-Adapter `tlmc: TlmcRuntime`, Snapshot, `rx_tlmc_prim` und `ServiceNotSupported`-Marker. Runtime-Datei allein genügt nicht. |
| `check_ltpd_runtime.py` | 1 | Vermisst MLE-Adapter, Runtime-Tick, Primitive-/Inbound-Verarbeitung und Resource-Break/Resume-Anbindung. |
| `check_foundation_acceptance.py` | 0 | Marker für TxReporter-Abschluss, Replay/Cancel/Timeout, Lifecycle-Diagnose und Zwei-Zellen-Harness vorhanden. Kein ausgeführter Rust-Test. |
| `check_mle_cell_change.py` | 1 | Vermisst MLE-BS-Einbindung von Cell-Change-Runtime, Prepare/Restore/Channel Request, Restore-Indication und Tick. |
| `check_cmce_call_restore.py` | 1 | Vermisst die historische lokale Restore-Prozedur einschließlich Gruppen-/Individual-Restore, Ressourcen-/Call-ID-Übernahme und Queued-Restore-Behandlung. |
| `check_mm_mobility.py` | 1 | Vermisst in `mm_bs.rs` unter anderem `send_d_location_update_proceeding`, `rx_lmm_mle_prepare_ind`, `provide_migration_context`, `take_forward_context` und `GrantPrepare`. |
| `check_node_gateway.py` | 0 | Dienststruktur, WebUI/REST, Node-/Backend-WebSocket, Health-Matrix, Fallback-Verteilung und Open-Lab-Marker vorhanden. |
| `check_mobility_core.py` | 1 | Vermisst MM-Antwortmarker `MobilityContextExported` und WebUI-Text `OFFENER TESTMODUS`. Fehlender UI-Text allein wäre kein Funktionsnachweis; der MM-Befund wurde zusätzlich im aktiven Handler geprüft. |
| `check_subscriber_core.py` | 0 | Benötigte Begriffe/Dateien vorhanden. Der Checker sucht über mehrere Dateien und belegt weder `subscriber_policy=true` noch aktive MM-Policy-Durchsetzung. |
| `check_group_core.py` | 1 | Bricht bereits wegen fehlender `.github/workflows/swmi-core-group.yml` ab; nachfolgende Checks nicht ausgeführt. |
| `check_call_control.py` | 1 | Bricht bereits wegen fehlender `.github/workflows/swmi-core-call-control.yml` ab; nachfolgende Checks nicht ausgeführt. |
| `check_media_switch.py` | 1 | Bricht bereits wegen fehlender `.github/workflows/swmi-core-media-switch.yml` ab; nachfolgende Checks nicht ausgeführt. |
| `protocol_inventory.py --check` | 1 | Zehn generierte Inventar-/Matrixdateien stimmen nicht mit der Neuberechnung überein. Es wurde nur geprüft, nichts neu generiert. |
| TOML-Parsing | 0 | `system-backend/services.toml` und die sieben Beispielkonfigurationen Node Gateway, Mobility, Subscriber, Group, Call Control, Media Switch, Recorder erfolgreich gelesen. |

**Ergebnis:** 3 von 13 historischen Komponenten-/Foundation-Checkern bestanden; 10 scheiterten. Mit der zusätzlichen Inventur sind 11 Prüfaufrufe negativ. Diese Zahlen sind keine Fehlerquote der Funkimplementierung: Ein Teil betrifft alte Prüferwartungen, ein Teil reale Integrationslücken. Acht lesbare TOML-Dateien belegen Syntax, nicht funktionierende Zieladressen.

Die als veraltet gemeldeten Dateien sind `Docs/SWMI_FOUNDATION_1_INVENTORY.md`, `Docs/ETSI_CONFORMANCE_MATRIX.md`, `Docs/SAP_PRIMITIVE_MATRIX.md`, `Docs/IMPLEMENTATION_GAPS.md`, `Docs/STATE_MACHINE_INVENTORY.md` sowie `Docs/generated/{protocol_inventory.json,pdu_inventory.csv,sap_inventory.csv,gap_inventory.csv,state_inventory.csv}`. Sie liegen außerhalb des autorisierten Änderungsbereichs und wurden nicht geändert.

Im am Prüfdatum vorliegenden `.github/workflows/` existieren andere Workflows: unter anderem referenziert `phy-slotter-tests.yml` Tests für Call Control, Media Switch, `central_control` und `central_media`; `alert-service-tests.yml` enthält Call-Control-Tests. Daraus folgt weder, dass CI vollständig fehlt, noch dass ein bestimmter Run erfolgreich war. Ein am Prüfdatum vorliegender Lauf wurde in dieser Archivfortsetzung nicht verifiziert.

### 9.9 Historische Prüfberichte aus den vier zugänglichen Paketberichte

Diese Zahlen sind Aussagen aus C und dienen der Wiederauffindbarkeit der damaligen Übergabestände. Original-ZIPs und vollständige Prüflogs fehlen; sie wurden nicht als am Prüfdatum vorliegende Erfolgsmeldungen übernommen.

| Paket | In der Planung genannter Umfang | In der Planung genannte Prüfungen / Grenze |
|---|---|---|
| Core A / Subscriber | 795 Dateien, 26 TOML-Dateien | ZIP-Integrität, erneutes Entpacken, Foundation-/Mobility-Checker, Deny-All, Home-ISSI/VASSI, Open Lab und WebUI-Vorgaben als bestanden gemeldet; kein Cargo-Build. |
| Core B / Group | 816 Dateien, 28 TOML-, 16 Python-, 15 Shell- und 14 relevante Rust-Dateien | Syntax bzw. lexikalische Prüfung, WebUI-JavaScript mit Node, ZIP-/Inventurprüfung als bestanden gemeldet; kein Cargo-Build. |
| Core C / Call Control | 840 Dateien, 30 TOML-, 17 Python-, 13 YAML-, 18 Shell- und 16 relevante Rust-Dateien | Routing, Korrelation, CMCE-Integration und Restore-Cleanup sowie Syntax-/ZIP-/Inventurprüfung als bestanden gemeldet; kein Cargo-Build. |
| Core D / Media Switch | 865 Dateien, 32 TOML-Dateien, 21 Shellskripte | Media-Bridge, Subscription, Call-Abgleich, Jitter, Duplicate-Schutz, Mute/Injection, WebUI-JavaScript und ZIP-/Inventurprüfung als bestanden gemeldet; kein Cargo-Build. |

Paket D nannte als zusätzliche CI-Befehle `cargo fmt --all -- --check`, `cargo test -p tetra-entities --test test_media_bridge`, `cargo test -p netcore-media-switch`, `cargo build --release -p netcore-media-switch` und `cargo clippy -p netcore-media-switch --all-targets -- -D warnings`. Diese Liste dokumentiert den damaligen vorgesehenen Prüfumfang, nicht ausgeführte Befehle dieser Fortsetzung.

## 10. Erreichter Entwicklungs- und Betriebsstand

### Als Quellcode vorhanden; aktive Integration unterschiedlich weit

- typisierte TLMC-/TLPD-Modelle;
- TLMC- und TLPD-Runtimes;
- Foundation-Robustheit und Zwei-Zellen-Testharness;
- MLE-Zellwechselbasis;
- lokale CMCE-Call-Restore-State-Machine;
- MM-Migration und Forward Registration;
- Node Gateway;
- Mobility Core;
- Subscriber Core;
- Group Core;
- Call Control;
- Media Switch;
- im geprüften Repo zusätzlich Recorder.

Diese Aufzählung belegt Dateien, Modelle und Dienste. Sie bestätigt ausdrücklich nicht, dass die zentrale Subscriber-/Group-Policy oder der mehrzellige Mobility-/Restore-Pfad derzeit bis zum aktiven TBS-Stack durchgängig funktioniert. Die Capability-Werte und fehlenden Handler in Abschnitt 9.7 schränken die historischen Implementierungsbehauptungen wesentlich ein.

### Tatsächlich neu geprüft und historisch als geprüft berichtet

- **Neu ausgeführt:** 13 vorhandene Python-Strukturchecker, davon 3 erfolgreich und 10 fehlgeschlagen; zusätzlich die Protokollinventur mit negativem Ergebnis. Ursachen und Grenzen stehen in Abschnitt 9.8.
- **Neu ausgeführt:** Einlesen von acht TOML-Dateien (Dienstemanifest und die sieben Beispielkonfigurationen von Node Gateway bis Recorder), erfolgreich.
- **Nur C/H:** frühere TOML-/YAML-/Shell-/JavaScript-/ZIP-Prüfberichte. Diese wurden nicht erneut an den historischen Paketen ausgeführt; die ZIP-Binärdaten fehlen.
- **Nicht neu ausgeführt:** Cargo-Build, Rust-Unit-/Integrationstests, Browserprüfung und GitHub-Actions-Run-Abnahme. Lokal sind inzwischen `cargo` und `rustc` auffindbar; die historische Aussage „keine Rust-Toolchain“ gilt nur für die damalige Artefaktumgebung.

### Nicht im Betrieb bestätigt

- kein realer Proxmox-LXC-Start dieser neuen Dienste in dieser Entwicklungsphase;
- kein erfolgreicher `cargo build` im Artefaktcontainer;
- kein nachgewiesener GitHub-Actions-Run für die einzelnen Entwicklungspakete;
- kein echter Zwei-TBS-On-Air-Handover mit Funkgeräten;
- kein realer Gruppen-/Individualruf-Restore über zwei physische TBS;
- kein echter Sprachframe-Dauertest zwischen mehreren TBS;
- keine Last-/Latenz-/Jitter-Abnahme;
- keine produktive Security-/RBAC-Abnahme.

## 11. Verworfene oder ersetzte Ansätze

| Früherer Ansatz | Späterer Stand | Grund |
|---|---|---|
| TLPD-Erfolg bereits nach LLC-Queue | `TxReporter`-gestützter Abschluss in Foundation E | Queue-Übernahme ist kein belastbarer Sende-/ACK-Nachweis |
| Call Restore bei Mobility A konservativ mit `D-RESTORE-FAIL` | vollständige lokale CMCE-Restore-State-Machine in Mobility B | laufende Gruppen-/Einzelrufe sollten tatsächlich wiederhergestellt werden |
| nur lokale Restore-Logik | Mobility Core + Call Control koordinieren Context Transfer | physische Multi-TBS-Wiederherstellung benötigt zentrale Koordination |
| Call-Control-Abgleich im Media Switch primär per Polling | geprüfter Branch: ereignisgesteuerter `/ws/media`-Pfad, HTTP nur Fallback | geringere Latenz und konsistenter Routinggraph |
| zukünftige Token-/RBAC-Idee für erste LXCs | ausdrücklicher `open_lab` ohne Tokens/TLS | Testphase zunächst vollständig offen |
| Recorder als nächster Entwicklungspaket | in den verfügbaren Entwicklungsnotizen keine Fertigstellung; geprüftes Repo enthält ihn | spätere Repository-Entwicklung ist separat zu bewerten |
| historische TBS-Fähigkeiten `subscriber_policy`, `group_policy`, `call_restore_context` aktiv | am Prüfdatum vorliegender Konstruktor setzt alle drei auf `false` | vorhandene Backend- und Runtime-Dateien belegen keine aktive Integration |
| sämtliche historischen Checker bestanden | am Prüfdatum vorliegende Wiederholung: 3/13 bestanden, Inventur veraltet | geänderter Code, fehlende Workflow-Dateien und mindestens ein fragiler Textchecker |

## 12. Offene Aufgaben und Roadmap-Kandidaten

### 12.1 Aus dieser Entwicklungsphase unmittelbar offen

1. **Aktive TBS-Integration klären und herstellen:** Subscriber-/Group-Policy, Mobility und Restore gegen Capability-Werte, Handler und tatsächlichen Funkpfad prüfen. Ein bloßes Umschalten der Fähigkeiten auf `true` ist keine Implementierung. Subscriber-Response-Korrelation und Revisionen absichern.
2. **Reale Rust-Kompilation und CI-Abnahme:** historische Strukturchecker reparieren, relevante Adaptertests ergänzen, Inventur in einem gesonderten Implementierungsauftrag aktualisieren und alle betroffenen Crates auf Linux/ARM64 bauen und testen. Die 11 negativen Struktur-/Inventurprüfungen sind zunächst zuzuordnen.
3. **LXC-Deploymentstand erheben und gezielt abnehmen:** installierte Commits/Binaries, Konfigurationen und Datenbanken von Node Gateway bis Recorder sichern und gegen den ausgewählten Stand prüfen. Erst danach installieren oder aktualisieren; fehlender Betriebsnachweis bedeutet nicht, dass auf den realen Hosts keine Dienste existieren.
4. **Ende-zu-Ende-Multi-Site-Test:** zwei physische TBS mit realen MS, Registration/Migration, Restore und Media-Routing prüfen.
5. **RF-Retune-/Channel-Change-Adapter abschließen und mit realem SDR testen.**
6. **Latenz- und Jittermessung:** Sprachpfad TBS A → Node Gateway → Media Switch → TBS B unter Last vermessen.
7. **Call-Restore-Dauer- und Fehlerfälle:** Quell-TBS-Verlust, Ziel-TBS-Verlust, doppelte Restore-Nachrichten, verspätete ACKs, Floor-Wechsel während Restore.
8. **Persistenz/Recovery der zentralen Dienste auf echtem LXC testen**, insbesondere nach Prozess- und Host-Neustart.
9. **Security später nachziehen:** TLS/mTLS, zentrale Authentisierung, RBAC, Audit; offene Laborpfade nicht als Produktionsmodus übernehmen.

### 12.2 Durch geprüften Repo-Stand neu einzuordnen

- Recorder ist nicht mehr nur Roadmap-Kandidat, sondern repository-seitig implementiert; jetzt fehlen Build-, Deployment- und echte Aufzeichnungsabnahme.
- Media Switch verwendet zum Prüfdatum eine modernere eventgetriebene Call-Control-Kopplung; historische Polling-Dokumentation sollte bei zukünftigen Betriebsanleitungen nicht mehr als Primärpfad verwendet werden.
- Das Repository enthält inzwischen weitere Backend-Dienste außerhalb dieser Planung. Deren Security-Modi und Abhängigkeiten müssen jeweils separat bewertet werden; die Open-Lab-Entscheidung dieser Planung ist kein globaler Dauerstandard.

### 12.3 Sinnvolle nächste technische Priorität nach dieser Entwicklungsphase

Die technische Priorität sollte nicht sofort auf noch mehr neue Dienste springen, sondern zunächst auf eine **echte Integrationsabnahme des bereits gebauten Multi-Site-Kerns**:

1. am Prüfdatum vorliegender Branch/Commit auswählen und vollständigen Workspace bauen;
2. Node Gateway + zwei TBS verbinden;
3. fehlende Mobility-/Policy-/Restore-Adapter mit gezielten Tests integrieren, anschließend Mobility/Subscribers/Groups synchronisieren;
4. Gruppenruf über beide TBS aufbauen;
5. Media Switch RouteReady/Sprachframes prüfen;
6. laufenden Ruf über Mobility/Call Restore auf Ziel-TBS übernehmen;
7. Recorder parallel passiv mitschneiden lassen;
8. Fehlerpfade, Restart und Recovery reproduzierbar dokumentieren.

## 13. Artefakte aus den Arbeitsnotizen

Die folgenden ZIP-Namen und SHA-256-Werte sind **historisch überliefert**. Für Core A–D stimmen H und die vollständig gelesenen Paketberichte C überein. Foundation/Mobility stammen allein aus H. **Keines dieser ZIPs war in dieser Fortsetzung als Binärdatei zugänglich; kein Hash wurde erneut berechnet.** Die gegenteilige Präsens-Aussage der Erstfassung wird hiermit ausdrücklich korrigiert, ohne ihren damaligen Prüfbericht rückwirkend zu widerlegen.

| Artefakt | SHA-256 |
|---|---|
| `netcore-tetra-swmi-foundation1-package-a.zip` | `6d6679b08febec7af936db9f2c76e12a9ff7cde532fb7033fd7c4bcb8ca08a32` |
| `netcore-tetra-swmi-foundation1-package-a-webui.zip` | `6e578e763c2963a08441ad1dc8c792cd284126e0f4257d263f954dc21cb92d62` |
| `netcore-tetra-swmi-foundation1-package-b.zip` | `2ff158c1e522c9e2b3fcafcbf68dbc1d408c828b871fb03312acc7ff34eee5e2` |
| `netcore-tetra-swmi-foundation1-package-c.zip` | `e877aec11e4298b76b6789356041aef9894402a12cf1f4d598cae337e51c360b` |
| `netcore-tetra-swmi-foundation1-package-d.zip` | `1d63fb16c2ac60c4391664918d4062e83e555453a301d979c4cac0ee993d9f64` |
| `netcore-tetra-swmi-foundation1-package-e.zip` | `ff53d579664610eb4139f6bd50148edc60fe48d7aa1dba64306c6ea2ab2ce3d2` |
| `netcore-tetra-swmi-mobility1-package-a.zip` | `dc295de64b5e6a7b36a10bedbe2d232b175f1ff84e542ba7e111d910b9245fb1` |
| `netcore-tetra-swmi-mobility1-package-b-call-restore.zip` | `853a38850849a8658000d335da980bb2ac5e7aa511dbd3dec6478faf32174e19` |
| `netcore-tetra-swmi-mobility1-package-c-mm-mobility.zip` | `72201c4e73b9cf7396f7af532790882058177c8d687595180a7ba797a254ff22` |
| `netcore-tetra-swmi-mobility1-package-d-node-gateway-open-lab.zip` | `92f1f0deea93749d90f6f29b0487a2087bdee4977b83c26f2d049e6995c355c3` |
| `netcore-tetra-swmi-mobility1-package-e-mobility-core-open-lab.zip` | `fd2295a678948b987a088e060a3a606857b068072e410bbe06e4a2ac68c8b2b2` |
| `netcore-tetra-swmi-core1-package-a-subscriber-core-open-lab.zip` | `f564d24a6fe2bf0afd630c544ef7f6da31cd16d30cc23cceab21ff4404b54d25` |
| `netcore-tetra-swmi-core1-package-b-group-core-open-lab.zip` | `7921ad118b22a104524d3d45832a94ed8d09e0fd215bc61a60b03f4ea711ff09` |
| `netcore-tetra-swmi-core1-package-c-call-control-open-lab.zip` | `037726fc224725db7c5bac6dee28ad46382ff511c9329c7b0e13e97e7da4bff1` |
| `netcore-tetra-swmi-core1-package-d-media-switch-open-lab.zip` | `4721caf2aa29ade70cc865cc6b362fc64fcaea1b776ce359fc58f82d434b93b7` |

Diese Artefakte wurden **nicht** neu hochgeladen. Die Dateiverweise verwenden historische `sandbox:/mnt/data/`-Pfade und sind hier keine abrufbaren Downloadquellen. Eine spätere Nacharchivierung muss Originaldateien eindeutig dieser Entwicklungsphase zuordnen und ihre Bytes gegen die überlieferten Hashwerte prüfen.

## 14. Relevante Repository-Dateien und Dokumente

H führte die folgende Dateiliste für ihren damaligen Abgleich an. Bei R wurden insbesondere Workspace/Manifest, die sechs Core-Dienste, Recorder, ihre APIs/Beispielkonfigurationen, die aktiven TBS-Handler und die Checker erneut untersucht. Die Paketdokumente bleiben historische Ergänzungen, nicht eigenständige Testnachweise:

- `Cargo.toml`;
- `system-backend/services.toml`;
- `system-backend/node-gateway/README.md`;
- `system-backend/mobility-core/README.md`;
- `system-backend/subscriber-core/README.md`;
- `system-backend/group-core/README.md`;
- `system-backend/call-control/README.md`;
- `system-backend/media-switch/README.md`;
- `system-backend/recorder/README.md`;
- `Docs/SWMI_FOUNDATION_1_PACKAGE_B.md`;
- `Docs/SWMI_FOUNDATION_1_PACKAGE_C.md`;
- `Docs/SWMI_FOUNDATION_1_PACKAGE_D.md`;
- `Docs/SWMI_FOUNDATION_1_PACKAGE_E.md`;
- `Docs/SWMI_MOBILITY_1_PACKAGE_A.md`;
- `Docs/SWMI_MOBILITY_1_PACKAGE_B.md`;
- `Docs/SWMI_MOBILITY_1_PACKAGE_C.md`;
- `Docs/SWMI_MOBILITY_1_PACKAGE_D_NODE_GATEWAY.md`;
- `Docs/SWMI_MOBILITY_1_PACKAGE_E_MOBILITY_CORE.md`;
- `Docs/SWMI_CORE_1_PACKAGE_A_SUBSCRIBER_CORE.md`;
- `Docs/SWMI_CORE_1_PACKAGE_B_GROUP_CORE.md`;
- `Docs/SWMI_CORE_1_PACKAGE_C_CALL_CONTROL.md`;
- `Docs/SWMI_CORE_1_PACKAGE_D_MEDIA_SWITCH.md`;
- `crates/tetra-entities/src/umac/tlmc_runtime.rs`;
- `crates/tetra-entities/src/mle/ltpd_runtime.rs`;
- `crates/tetra-entities/src/mle/cell_change_runtime.rs`;
- `crates/tetra-entities/src/cmce/call_restore_runtime.rs`;
- `crates/tetra-entities/src/mm/mobility_runtime.rs`.

## 15. Normquellen und Anhänge

H nennt 25 ETSI-PDFs und ordnet den TLMC/TLPD/MLE/MM/CMCE-Pfaden vor allem `en_30039202v030801p.pdf` (ETSI EN 300 392-2 V3.8.1) zu. Die 25 lokalen PDF-Dateien unter `sources/` sind weiterhin sichtbar. Ihre Inhalte und Versionsangaben wurden in dieser Fortsetzung nicht neu fachlich geprüft. Weitere Dateinamen betreffen unter anderem EN 300 392-1, mehrere Teile von EN 300 392-3, EN 300 392-5/-7/-9/-10/-11/-12, EN 300 394-1 und EN 300 395-2 sowie ES-/TS-100812/200812/300812-Dokumente.

Die PDF-Anhänge sind Normquellen; sie beweisen keine konkrete Implementierung oder erfolgreiche Funkabnahme. Wo in der Planung auf Normen Bezug genommen wurde, wurde dies als Grundlage für Datenmodelle und Prozeduren verwendet, nicht als alleiniger Implementierungsnachweis.

## 16. Bilder

In den verfügbaren Planungsunterlagen wurden keine eigenständigen Bilder oder Screenshots gefunden. Es wurde deshalb kein Bild nach `Docs/archive/` hochgeladen und kein Bild aus anderen Projektphasen übernommen. Sollten weitere historische Bilddateien existieren, fehlt dafür hier die eindeutige Quelle.

## 17. Schlussstand

Die zugänglichen Paketberichte und die erhaltene historische Archivfassung beschreiben den Aufbau eines Multi-Site-Modells: zeitkritische Funklogik bleibt auf der TBS, zentrale Dienste koordinieren Teilnehmer, Gruppen, Calls und Medien. Die Backend-Dienste und viele Runtime-Modelle sind am Prüfcommit vorhanden; Recorder und Media-Kopplung wurden später erweitert.

Der am Prüfdatum vorliegende TBS-Stand belegt jedoch wesentliche Grenzen bei zentraler Teilnehmer-/Gruppenpolicy, Mobility und Restore. Deshalb müssen zuerst aktive Adapter, Capability-Ankündigungen und Tests konsistent werden. Darauf folgen ein belegter Rust-/Linux-/ARM64-Build sowie eine reproduzierbare Abnahme mit realen LXC-Instanzen, zwei TBS und Funkgeräten. Erst solche Nachweise rechtfertigen „im Betrieb bestätigt“.
