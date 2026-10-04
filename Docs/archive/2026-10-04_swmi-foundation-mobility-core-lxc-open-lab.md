# Abschlussdokumentation: SWMI Foundation, Mobility, Core-LXC und Open-Lab-Ausbau

## 1. Metadaten

- **Thema:** Ausbau von NetCore-Tetra von typisierten TLMC/TLPD-Grundlagen über lokale Mobility-/Call-Restore-Runtimes bis zu den ersten zentralen LXC-Diensten `node-gateway`, `mobility-core`, `subscriber-core`, `group-core`, `call-control` und `media-switch`.
- **Ursprünglicher Chattitel:** im verfügbaren Chatkontext nicht zuverlässig vorhanden.
- **Chatlink:** nicht verfügbar.
- **Erstellungsdatum dieser Abschlussdokumentation:** 2026-10-04.
- **Repository:** `JanHG98/netcore-tetra`.
- **Zielbranch für dieses Archiv:** `Archiving`.
- **Vor dem Schreiben geprüfter Branch-HEAD:** `875380e729ee45aa7cc89f48ccd0a32b4be0c770` (`docs(archive): document historical groups and Zebra-Test with evidence gaps`).
- **Default-Branch zum Prüfzeitpunkt:** `main`, Commit `7137e0dd69877e1b604bf89148fd8b6b590c1a97`.
- **Branchvergleich zum Prüfzeitpunkt:** `Archiving` und `main` waren divergiert; `Archiving` lag 47 Commits vor und 1 Commit hinter `main`. Es wurde für diesen Auftrag **nicht gemergt**.
- **Zulässiger Schreibbereich dieses Auftrags:** ausschließlich `Docs/archive/`.

### 1.1 Auswertungsumfang und Grenzen

Ausgewertet wurden der in dieser Sitzung zugängliche Chatverlauf, der bereitgestellte Projekt-/Verlaufskontext, die weiterhin lokal vorhandenen erzeugten ZIP-Artefakte sowie gezielte Readbacks des GitHub-Repositories. Nicht jeder historische Zwischenprompt liegt als vollständige Rohtranskription vor; mehrere ältere Turns waren in einer kompakten Verlaufzusammenfassung verfügbar. Deshalb wird bei Punkten, die nur aus dieser Zusammenfassung stammen, keine wortgetreue Transkription behauptet.

Die erzeugten ZIP-Dateien der Entwicklungsstufen sind im Arbeitsbereich weiterhin vorhanden und ihre SHA-256-Werte wurden am 2026-10-04 erneut lokal geprüft. Ein vorhandenes ZIP ist jedoch **kein Nachweis**, dass sein Inhalt in einen produktiven Branch gemergt, auf einer TBS gebaut oder in einem LXC betrieben wurde.

Die im Projektkontext bereitgestellten ETSI-PDFs sind als Normquellen verfügbar. In diesem Chat wurde insbesondere `en_30039202v030801p.pdf` (ETSI EN 300 392-2 V3.8.1) als Air-Interface-Grundlage herangezogen. Eine vollständige Neuprüfung sämtlicher 25 PDF-Anhänge war für die Archivierung nicht erforderlich.

Es wurden in diesem Chat **keine eigenständigen Bilddateien oder Screenshots** hochgeladen. Daher gibt es keine Chatbilder, die zusätzlich nach `Docs/archive/` kopiert werden könnten. Die vorhandenen PDFs sind Normdokumente und keine Chatbilder.

## 2. Statusbegriffe

Diese Dokumentation trennt bewusst fünf Ebenen:

| Status | Bedeutung |
|---|---|
| **Idee** | im Chat vorgeschlagen, ohne verbindliche Umsetzung |
| **beschlossen/geplant** | vom Nutzer ausdrücklich gewünscht oder als nächster Schritt vereinbart |
| **implementiert** | Quellcode/Artefakt wurde erzeugt oder ist im geprüften Repository vorhanden |
| **getestet** | ein konkreter Test/Checker wurde tatsächlich ausgeführt und sein Ergebnis liegt vor |
| **im Betrieb bestätigt** | auf realer TBS/LXC/Endgerät/Funkstrecke erfolgreich beobachtet |

Wichtig: Die meisten Arbeitsschritte dieses Chats erreichten **implementiert** und statisch **getestet**. Ein echter Rust-Build in der Artefaktumgebung sowie reale TBS-/LXC-/On-Air-Abnahmen wurden in diesem Chat nicht durchgeführt und dürfen daher nicht als bestätigt gelten.

## 3. Ziel und Ausgangslage

Der Chat setzte auf einem bereits weit fortgeschrittenen NetCore-Tetra-Codebestand auf. Zunächst fehlten belastbare, typisierte lokale SAP-Grundlagen für TLMC/TLPD und vollständige State Machines für Mobility- und Call-Restore-Pfade. Danach wurde der Fokus schrittweise auf einen Multi-Site-SWMI-Ausbau verschoben.

Der Nutzer wollte ausdrücklich:

- vollständige Repository-ZIPs statt kleiner Patches;
- die lokale zeitkritische Funklogik in der TBS belassen;
- spätere Backend-Komponenten unter `system-backend/` jeweils als eigener Dienst strukturieren;
- für jeden tatsächlich deploybaren LXC-/VM-Dienst eine eigene WebUI vorsehen;
- zunächst eine **offene Testumgebung ohne Tokens, Login oder TLS** verwenden;
- den Ausbau schrittweise von Foundation → Mobility → zentrale Core-Dienste fortsetzen.

Die spätere Sicherheitsentscheidung „erstmal nicht mit Tokens arbeiten“ hatte Vorrang vor früheren allgemeinen RBAC-/Security-Ideen. Für die in diesem Chat eingeführten Backend-Dienste wurde deshalb `security.mode = "open_lab"`, `token_auth = false` und `tls = false` vorgesehen. Das war ausdrücklich ein **Labormodus**, keine Empfehlung für Produktion.

## 4. Architekturentscheidungen dieses Chats

### 4.1 Trennung Edge/TBS und zentraler Core

**Beschlossen und in den Paketen konsequent verfolgt:**

- PHY/UMAC/LLC/MLE/MM/CMCE-nahe Echtzeitpfade bleiben lokal in der TBS.
- TLMC und TLPD sind lokale Service-Access-Point-/Runtime-Komponenten und **keine eigenen LXC-Dienste**.
- Zentrale Dienste koordinieren Teilnehmer-, Gruppen-, Call- und Media-Zustände, übernehmen aber keine Air-Interface-Timer.
- `node-gateway` bildet den zentralen Transport zwischen TBS und Backend.
- Jeder deploybare Systemdienst besitzt eine eigene WebUI; `system-backend/shared` bleibt eine Bibliotheksstruktur und ist kein eigener Runtime-Container.

### 4.2 Geplanter bzw. im Chat implementierter Core-Datenpfad

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

Für die im Chat aufgebauten LXC-Dienste galt:

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

## 5. Chronologischer Entwicklungsstand dieses Chats

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

**Bewusste Grenze im Chat:** Der allgemeine physische SDR-Retune-Pfad war noch nicht vollständig hardwareunabhängig. Die Runtime konnte die benötigte `TmvConfigureReq` erzeugen, aber der Adapter bis zum realen SDR blieb ein klar benannter Integrationspunkt.

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

Der Nutzer verlangte ausdrücklich, dass die Wiederherstellung laufender Gruppen- **und** Einzelrufe nicht weiter auf später verschoben wird.

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

Hier änderte der Nutzer die Sicherheitspriorität ausdrücklich: **in der Testumgebung vorerst keine Tokens**.

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

### 5.15 Recorder: im Chat nur als nächster Schritt begonnen

Nach dem Media Switch wurde als nächster LXC der **Recorder** angekündigt: Übernahme von Media-Taps/Sprachframes, Zusammensetzen netzweiter Calls, Metadaten und eigene offene WebUI.

Der Nutzer erteilte danach jedoch den Archivierungsauftrag. **In diesem Chat wurde vor der Archivierung kein Recorder-Paket erzeugt und kein Recorder-ZIP zurückgegeben.** Deshalb lautet der Chatstatus für den Recorder: **beschlossen/geplant, aber in diesem Chat nicht abgeschlossen**.

Dieser Punkt ist wichtig, weil der heute geprüfte Repository-Stand inzwischen bereits einen Recorder enthält; siehe Abschnitt 9. Das ist eine spätere/anderweitige Repository-Weiterentwicklung und darf nicht rückwirkend als Ergebnis des hier sichtbaren Recorder-Schritts ausgegeben werden.

## 6. Relevante Dateien und technische Schnittstellen aus dem Chat

### 6.1 Lokale TBS-Runtimes

| Bereich | Relevante Datei/Komponente | Status im Chat |
|---|---|---|
| TLMC | `crates/tetra-entities/src/umac/tlmc_runtime.rs` | implementiert |
| TLPD | `crates/tetra-entities/src/mle/ltpd_runtime.rs` | implementiert |
| MLE Cell Change | `crates/tetra-entities/src/mle/cell_change_runtime.rs` | implementiert |
| CMCE Call Restore | `crates/tetra-entities/src/cmce/call_restore_runtime.rs` | implementiert |
| MM Mobility | `crates/tetra-entities/src/mm/mobility_runtime.rs` | implementiert |
| Zwei-Zellen-Harness | `crates/tetra-entities/tests/common/two_cell.rs` | implementiert |

### 6.2 Backend-LXC und Ports

| Dienst | Historischer Port | Hauptschnittstelle | Sicherheitsmodus im Chat |
|---|---:|---|---|
| Node Gateway | 8080 | `/ws/node`, `/ws/backend`, `/api/v1` | open_lab |
| Mobility Core | 8090 | Node-Gateway-Backend-WS, REST | open_lab |
| Subscriber Core | 8100 | REST + Node-Gateway-Backend-WS | open_lab |
| Group Core | 8110 | REST + Node-Gateway-Backend-WS | open_lab |
| Call Control | 8120 | REST + Node-Gateway-Backend-WS | open_lab |
| Media Switch | 8130 | REST + Node-Gateway-Backend-WS | open_lab |
| Recorder | 8140 | im Chat nur geplant | open_lab geplant |

### 6.3 Wichtige Protokoll-/Datenparameter

- TBS-Kompatibilitätsprotokoll: `netcore-control-room-node-v1`.
- Node Gateway: `WS /ws/node` für TBS, `WS /ws/backend` für Backend-Dienste.
- LTPD/robuste Transfers: Timeout 432 Timeslots.
- MM-Migration: temporärer lokaler VASSI-Pool standardmäßig `0xE00000..0xEFFFFE`.
- Media Switch: TETRA Speech Service 0, 274 Nutzbits = 35 Bytes pro transportiertem Frame.
- Foundation-E-Replay-Schutz: kürzlich abgeschlossene Handles bleiben für eine Hyperframe-Dauer geschützt.

## 7. Deployment-, Build- und Reparaturabläufe

Die Paketdokumentationen sahen wiederholt denselben Clean-Deploy-Grundsatz vor:

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

Beispiele aus dem Chat bzw. den Paketdokumenten:

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

Vorgesehene Rust-Prüfungen, im Chat-Artefaktcontainer **nicht lokal ausgeführt**:

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

## 8. Fehler, Diagnose und daraus resultierende Regeln

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

## 9. Heutiger zusätzlich geprüfter Repository-Stand

Dieser Abschnitt beschreibt **nicht den historischen Chatstand**, sondern den am 2026-10-04 gezielt geprüften Stand des Zielbranches `Archiving` vor dieser Archivänderung.

### 9.1 Quellcode der zentralen Runtimes ist vorhanden

Folgende Dateien wurden direkt im Branch `Archiving` gefunden:

- `crates/tetra-entities/src/umac/tlmc_runtime.rs`;
- `crates/tetra-entities/src/mle/ltpd_runtime.rs`;
- `crates/tetra-entities/src/mle/cell_change_runtime.rs`;
- `crates/tetra-entities/src/cmce/call_restore_runtime.rs`;
- `crates/tetra-entities/src/mm/mobility_runtime.rs`.

Damit sind die Kernpfade des Foundation-/Mobility-Ausbaus heute **im Repository vorhanden**. Das ist weiterhin kein Betriebsnachweis.

### 9.2 LXC-Crates sind heute im Workspace vorhanden

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

### 9.3 Open-Lab-Konfiguration heute

`system-backend/services.toml` enthält weiterhin für Node Gateway, Mobility Core, Subscriber Core, Group Core, Call Control, Media Switch und Recorder die offenen Laborparameter mit HTTP, ohne TLS und ohne Token-Authentisierung. Außerdem ist `webui_defaults.required = true` gesetzt.

Wichtig: Der **heutige gesamte Repository-Stand ist nicht mehr vollständig tokenfrei**. Beispielsweise enthält `services.toml` inzwischen einen späteren `alert-service` mit `security_mode = "token"` und `token_auth = true`. Die Nutzerentscheidung dieses Chats „erstmal alles offen“ bezog sich auf die damalige Testphase und die hier eingeführten Dienste; sie darf nicht pauschal auf alle später hinzugekommenen Services übertragen werden.

### 9.4 Media Switch ist heute weiterentwickelt

Historischer Chatstand: Call Control wurde vom Media Switch zunächst zyklisch über `/api/v1/calls` abgeglichen.

Heutiger Branchstand: Die Readmes beschreiben inzwischen einen ereignisgesteuerten Call-Control-WebSocket `ws://<call-control>:8120/ws/media` mit Subprotokoll `netcore-call-control-media-v1`. HTTP dient nur noch als Fallback. Zusätzlich existieren revisionsgebundene Route-Ready-ACKs und ein Kaltstart-Vorpuffer.

Das ist eine **spätere Weiterentwicklung** und ersetzt den älteren Polling-Schwerpunkt des Chatpakets.

### 9.5 Recorder ist heute implementiert, obwohl er im Chat noch offen war

Im Branch existiert inzwischen `system-backend/recorder/` als Workspace-Crate. Die gelesene README beschreibt:

- Polling des replay-fähigen Vollframe-Taps `GET /api/v1/recorder/taps?after=<seq>&limit=<n>`;
- verlustfreie Ablage der 35-Byte-TETRA-ACELP-Frames in `audio.tacelp`;
- `frames.jsonl`, Metadaten und SHA-256-Integritätsdateien;
- Recovery unfertiger `.part`-Aufnahmen;
- Retention, Legal Hold, Löschen und TAR-Export;
- eigene WebUI auf Port `8140`;
- weiterhin `open_lab`.

Dieser heutige Repository-Befund ist **nicht** als in diesem Chat fertiggestellter Recorder zu kennzeichnen. Im Chat wurde der Recorder nur als nächster Schritt angekündigt; die Archivierungsanweisung kam vor einer entsprechenden Implementierungsantwort.

### 9.6 Branchabweichung zu `main`

`Archiving` war beim Prüfzeitpunkt 47 Commits vor und 1 Commit hinter `main`. Daher ist dieser Abschnitt ausdrücklich ein Befund des Zielbranches `Archiving`. Es wurde keine automatische Angleichung an `main` vorgenommen, da der Auftrag ausschließlich die Archivdokumentation im vorhandenen Branch verlangte.

## 10. Erreichter Entwicklungs- und Betriebsstand

### Implementiert / repository-seitig belegt

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
- im heutigen Repo zusätzlich Recorder.

### Getestet

- zahlreiche paketbezogene Python-Static-Checker;
- Protokollinventur;
- TOML-/YAML-/Shell-/JavaScript-Syntaxprüfungen in den jeweiligen Paketläufen;
- ZIP-Integrität und unabhängiges Entpacken für die späteren Pakete;
- Hashwerte der weiterhin lokalen Chat-ZIPs wurden für dieses Archiv erneut geprüft.

### Nicht im Betrieb bestätigt

- kein realer Proxmox-LXC-Start dieser neuen Dienste in diesem Chat;
- kein erfolgreicher `cargo build` im Artefaktcontainer;
- kein nachgewiesener GitHub-Actions-Run für die einzelnen Chatpakete;
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
| Call-Control-Abgleich im Media Switch primär per Polling | heutiger Branch: ereignisgesteuerter `/ws/media`-Pfad, HTTP nur Fallback | geringere Latenz und konsistenter Routinggraph |
| zukünftige Token-/RBAC-Idee für erste LXCs | ausdrücklicher `open_lab` ohne Tokens/TLS | Nutzer wollte Testumgebung zunächst vollständig offen |
| Recorder als nächster Chatbaustein | Chat endete vor Implementierung; heutiges Repo enthält ihn inzwischen | Archivierungsauftrag unterbrach die geplante Fortsetzung |

## 12. Offene Aufgaben und Roadmap-Kandidaten

### 12.1 Aus diesem Chat unmittelbar offen

1. **Reale Rust-Kompilation und CI-Abnahme:** alle Foundation-/Mobility-/Core-Crates mit aktuellem Toolchain-Stand bauen und testen.
2. **LXC-Deployment tatsächlich durchführen:** Node Gateway, Mobility Core, Subscriber Core, Group Core, Call Control, Media Switch und Recorder auf den vorgesehenen Zielsystemen installieren.
3. **Ende-zu-Ende-Multi-Site-Test:** zwei physische TBS mit realen MS, Registration/Migration, Restore und Media-Routing prüfen.
4. **RF-Retune-/Channel-Change-Adapter abschließen und mit realem SDR testen.**
5. **Latenz- und Jittermessung:** Sprachpfad TBS A → Node Gateway → Media Switch → TBS B unter Last vermessen.
6. **Call-Restore-Dauer- und Fehlerfälle:** Quell-TBS-Verlust, Ziel-TBS-Verlust, doppelte Restore-Nachrichten, verspätete ACKs, Floor-Wechsel während Restore.
7. **Persistenz/Recovery der zentralen Dienste auf echtem LXC testen**, insbesondere nach Prozess- und Host-Neustart.
8. **Security später nachziehen:** TLS/mTLS, zentrale Authentisierung, RBAC, Audit; offene Laborpfade nicht als Produktionsmodus übernehmen.

### 12.2 Durch heutigen Repo-Stand neu einzuordnen

- Recorder ist nicht mehr nur Roadmap-Kandidat, sondern repository-seitig implementiert; jetzt fehlen Build-, Deployment- und echte Aufzeichnungsabnahme.
- Media Switch verwendet heute eine modernere eventgetriebene Call-Control-Kopplung; historische Polling-Dokumentation sollte bei zukünftigen Betriebsanleitungen nicht mehr als Primärpfad verwendet werden.
- Das Repository enthält inzwischen weitere Backend-Dienste außerhalb dieses Chats. Deren Security-Modi und Abhängigkeiten müssen jeweils separat bewertet werden; die Open-Lab-Entscheidung dieses Chats ist kein globaler Dauerstandard.

### 12.3 Sinnvolle nächste technische Priorität nach diesem Chat

Die technische Priorität sollte nicht sofort auf noch mehr neue Dienste springen, sondern zunächst auf eine **echte Integrationsabnahme des bereits gebauten Multi-Site-Kerns**:

1. aktueller Branch/Commit auswählen und vollständigen Workspace bauen;
2. Node Gateway + zwei TBS verbinden;
3. Mobility/Subscribers/Groups synchronisieren;
4. Gruppenruf über beide TBS aufbauen;
5. Media Switch RouteReady/Sprachframes prüfen;
6. laufenden Ruf über Mobility/Call Restore auf Ziel-TBS übernehmen;
7. Recorder parallel passiv mitschneiden lassen;
8. Fehlerpfade, Restart und Recovery reproduzierbar dokumentieren.

## 13. Artefakte aus dem Chat

Die folgenden ZIPs sind im aktuellen Arbeitsbereich noch vorhanden; ihre SHA-256-Werte wurden für diese Archivierung erneut lokal berechnet:

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

Diese Artefakte wurden **nicht** als Teil dieses Archivauftrags in GitHub hochgeladen, weil der Auftrag ausschließlich die Abschlussdokumentation und zugehörige Chatbilder unter `Docs/archive/` verlangt. Sie bleiben historische Build-/Übergabeartefakte.

## 14. Relevante Repository-Dateien und Dokumente

Für den heutigen Abgleich wurden unter anderem folgende Dateien im Branch `Archiving` direkt gelesen bzw. ihre Existenz geprüft:

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

Im Projektkontext waren 25 ETSI-PDFs verfügbar. Für die hier entwickelten TLMC/TLPD/MLE/MM/CMCE-Pfade war vor allem die Air-Interface-Spezifikation `en_30039202v030801p.pdf` (ETSI EN 300 392-2 V3.8.1) relevant. Weitere bereitgestellte Dateien waren u. a. EN 300 392-1, mehrere Teile von EN 300 392-3, EN 300 392-5, EN 300 392-7, EN 300 392-9, EN 300 392-10, EN 300 392-11, EN 300 392-12, EN 300 394-1, EN 300 395-2 sowie ETSI/ES/TS-300812-Dokumente.

Die PDF-Anhänge sind Normquellen; sie beweisen keine konkrete Implementierung oder erfolgreiche Funkabnahme. Wo im Chat auf Normen Bezug genommen wurde, wurde dies als Grundlage für Datenmodelle und Prozeduren verwendet, nicht als alleiniger Implementierungsnachweis.

## 16. Bilder

Im für diesen Chat verfügbaren Verlauf wurden keine eigenständigen Bilder oder Screenshots gefunden. Es wurde deshalb kein Bild nach `Docs/archive/` hochgeladen und kein Bild aus anderen Projektchats übernommen. Sollte der ursprüngliche Chat außerhalb des zugänglichen Verlaufs doch Bilder enthalten haben, fehlt dafür hier die eindeutige Quelle.

## 17. Schlussstand

Der Chat hat NetCore-Tetra von einer lokalen SAP-/State-Machine-Grundlage zu einem klar getrennten Multi-Site-Modell geführt: zeitkritische Funklogik bleibt auf der TBS, während zentrale Dienste Teilnehmer, Gruppen, Calls und Medien koordinieren. Die wichtigsten Architekturentscheidungen – lokale TLMC/TLPD/MLE/MM/CMCE-Zuständigkeit, WebUI je deploybarem LXC und expliziter Open-Lab-Modus – sind im heutigen Repository weiterhin deutlich erkennbar.

Der entscheidende verbleibende Schritt ist nicht noch mehr Konzeptcode, sondern eine reproduzierbare Ende-zu-Ende-Abnahme mit echtem Rust-Build, realen LXC-Instanzen, zwei physischen TBS und realen Funkgeräten. Erst danach kann aus „implementiert“ belastbar „im Betrieb bestätigt“ werden.

## 18. Archivierungsnachweis

Diese Datei wurde ausschließlich für `Docs/archive/` erstellt. Der zugehörige Indexeintrag wird in `Docs/archive/README.md` ergänzt. Es werden keine Implementierungsdateien verändert, kein Force-Push durchgeführt und der Branch nicht gemergt.
