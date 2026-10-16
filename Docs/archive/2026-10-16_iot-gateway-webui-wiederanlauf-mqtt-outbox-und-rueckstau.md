# IoT-Gateway: Web-UI-Wiederanlauf, volle MQTT-Outbox und Rückstauabbau

**Technische Abschlussdokumentation, erstellt am 16.10.2026 (Europe/Berlin).** Dieses Archiv trennt das im Chat erreichte Ergebnis vom zusätzlich am 16.10.2026 geprüften Repository-Stand. Historische Kommandos sind mit ihrem Ausführungsstand gekennzeichnet. Eine Übernahme in `main`, ein erfolgreicher CI-Lauf und eine tatsächliche Installation auf dem Gateway sind unterschiedliche Nachweise.

Aktuelle fachliche Einstiege: [IoT-Gateway](../services/iot-gateway/README.md), [LXC-Installation und Reparatur](../services/iot-gateway/installation-im-lxc.md), [Gesamtroadmap](../roadmaps/gesamtroadmap.md). Der vorliegende Archivauftrag ändert ausschließlich Inhalte unter `Docs/archive/`; Roadmap-Kandidaten werden hier erfasst und nicht in die aktive Roadmap übertragen.

## 1. Metadaten, Quellenumfang und Nachweisstufen

| Feld | Stand |
|---|---|
| Thema | Ausgefallene IoT-Gateway-Web-UI, Wiederanlauf bei voller persistenter MQTT-Outbox, kleiner Git-Transfer, Versanddurchsatz und korrekte Behandlung von Rückdruck |
| Ursprünglicher Chattitel | Nicht als verlässliches Titelmetadatum zugänglich. Der sichtbare thematische Einstieg lautet: „iot gateway will nicht, Web Ui scheint down zu sein“. Dies wird nicht als nachgewiesener UI-Chattitel ausgegeben. |
| Chatlink | Im zugänglichen Verlauf nicht enthalten; nicht aus internen Aufgabenkennungen rekonstruiert. |
| Historischer Arbeitszeitraum | Betreiberlogs und Git-Aktivität vom 13.10.2026 UTC; die späteren Wiederanlauf- und Folgefixschritte liegen am 14.10.2026 in Europe/Berlin. |
| Erstellungs- und zusätzlicher Prüftag | 16.10.2026, Europe/Berlin |
| Repository | `JanHG98/netcore-tetra` |
| Zusätzlich geprüfter Entwicklungsbranch | `main` |
| Gepinnter Prüfausgangsstand | `8fb581b730f7a3833566c4bbb7c87875ad921e69` — Merge von PR #97, Commitdatum `2026-10-15T21:13:54Z` |
| Geprüfter Hauptfix für den Wiederanlauf | `b223ecc0f626e5dd8222d3edc0e969c774b8da21`, PR #83 |
| Geprüfter Folgefix für Durchsatz/Rückdruck | `0dffd40ea5687794b276b6dd4e54f2f645729746`, PR #88 |
| Zielbranch der Archivübernahme | **`main`**, vom Benutzer am 16.10.2026 ausdrücklich bestätigt. Geprüfte Ablagebasis: `8fb581b730f7a3833566c4bbb7c87875ad921e69`. Der frühere Branch `fix/iot-outbox-throughput-20261014` ist am Prüftag nicht mehr vorhanden. |
| Erwarteter Repository-Pfad | `Docs/archive/2026-10-16_iot-gateway-webui-wiederanlauf-mqtt-outbox-und-rueckstau.md` |

### 1.1 Verfügbare Primärquellen und Grenzen

Ausgewertet wurden die zugänglichen Chatnachrichten mit den eingefügten Shellblöcken und ihren Ausgaben, die erhaltenen technischen Fortsetzungsnotizen, die relevanten PRs und die heutigen Repository-Dateien. Der ursprüngliche Textanhang **`Eingefügter Text(20261013-222443).txt`** wurde am 16.10.2026 erneut vollständig gelesen: 83 Textzeilen, laut Dateimetadaten 12.529 Byte. Seine für die Diagnose maßgeblichen Inhalte sind im [archivierten Logtext](assets/2026-10-16_iot-gateway-webui-outbox/startabbruch-2026-10-13.txt) erhalten. Dieser ist eine UTF-8-Textübernahme des ausgelesenen Inhalts; eine gesonderte Behauptung über die Byteidentität der ursprünglichen Upload-Datei wird nicht getroffen.

Zusätzlich wurden der [bereits im Repository gespeicherte Betreiberbeleg](../services/iot-gateway/tests/evidence/wiederanlauf-2026-10-14.json), die PR-Mergezustände, die historischen GitHub-Actions-Ergebnisse und das Linux-Joblog des Folgefixes geprüft. Die Aussage „im Repository implementiert“ stützt sich daher auf gelesene Dateien und Git-Objekte, nicht allein auf frühere Assistentenaussagen.

**Auswertungslücken:** Es liegt kein verlustfreier Export sämtlicher früherer interner Toolausgaben und Zwischenfassungen vor. Frühere lokale Windows-Testläufe sind über den erhaltenen Arbeitsverlauf und PR-Beschreibungen dokumentiert; die Linux-CI wurde zusätzlich unmittelbar nachgelesen. Titel- und Chatlinkmetadaten konnten auch über den verfügbaren App-Kontext nicht gewonnen werden. Es gibt keinen späteren Betreiberoutput zur Installation von PR #88 und keine neue Verbindung zum privaten Gateway am Prüftag. Die vollständige aktuelle Hostkonfiguration, systemd-Drop-ins, laufende Prozessidentität, Brokerkonfiguration und Endabnehmerzustellung wurden nicht neu erhoben.

Dem IoT-Verlauf ist **kein Bildanhang** zuordenbar. Der frühere Diagnoseanhang ist eine Textdatei. Die zusätzlich angebotenen 25 ETSI-PDFs sind Normunterlagen ohne im verfügbaren Gespräch hergestellten Bezug zur hier behandelten HTTP-/MQTT-/Outbox-Störung; ihre Inhalte und eingebetteten Abbildungen wurden für diesen Abschluss nicht ausgewertet. Bilder aus anderen Projektchats werden nicht als Bilder dieses Chats übernommen. Ein Bild-Upload für diesen Chat ist deshalb nicht belegt beziehungsweise nicht möglich; es werden keine Ersatzbilder erfunden.

### 1.2 Statusbegriffe

| Stufe | Bedeutung |
|---|---|
| **Idee** | Erwogene Möglichkeit ohne nachgewiesenen Umsetzungsbeschluss. |
| **Beschlossen/geplant** | Als Vorgehen oder nächster Schritt festgelegt; noch kein Ausführungsnachweis. |
| **Implementiert** | Quelländerung beziehungsweise Datei am genannten Commit tatsächlich nachgewiesen. |
| **Getestet** | Konkreter lokaler, statischer, Echtprozess- oder CI-Test; Testumgebung und Grenzen bleiben benannt. |
| **Im Betrieb bestätigt** | Aussage ist durch Betreiberoutput oder ausdrückliche Betreiberbestätigung für diesen Host und diesen Zeitraum gedeckt. |
| **Offen/unbestätigt** | Die vorhandenen Belege reichen für die betreffende Aussage nicht aus. |

## 2. Ziel, Ausgangslage und Ergebnis dieses Chats

Ziel war, den nicht startenden IoT-Gateway und seine Web-UI auf **`10.0.1.119:8240`** wieder verfügbar zu machen, ohne den gespeicherten MQTT-Ausgang zu löschen. Nach der erfolgreichen Wiederherstellung wurde der weiterhin nahezu volle Ausgang untersucht. Daraus entstand ein zweiter Fix für MQTT-Durchsatz, Dateiauswahl und Rückdruck im Quellenpoller.

| Teilziel | Historischer Abschlussstand | Zusätzlich am 16.10.2026 geprüft |
|---|---|---|
| Ursache des Startabbruchs bestimmen | Volle Outbox verhindert das Einreihen zusätzlicher Startzustände; Prozess beendet sich vor MQTT und HTTP. | Originaljournal erneut gelesen; Startreihenfolge und Fehlerbehandlung im Code geprüft. |
| Wiederanlauf trotz voller/überfüllter Outbox | PR #83 implementiert und getestet; vom Betreiber installiert. | PR #83 ist gemergt; Korrektur im heutigen `main` vorhanden. |
| Git-Transfer zuverlässig klein halten | Partial Clone, Sparse Checkout und HTTP/1.1 funktionieren auf dem Host. | Anleitung und PR #85 übernommen; erfolgreiche Betreiberübertragung dokumentiert. |
| Web-UI wieder verfügbar | Betreiber bestätigt ausdrücklich „Web-UI geht wieder“. | Historischer Betriebsnachweis; keine neue Hostprüfung am 16.10.2026. |
| MQTT-Verbindung und Quellen | MQTT verbunden, Quittungen steigen; alle vier Quellen gesund. | Historischer Betreiberbeleg im Repository vorhanden. |
| Nachhaltiger Rückstauabbau | Trotz laufender Quittungen bleibt die Outbox fast voll. | Kein späterer erfolgreicher Abbau auf dem Diensthost nachgewiesen. |
| Durchsatz- und Rückdruckfix | PR #88 implementiert; 24 lokale Rust-Tests und zwei Echtprozessregressionen sowie relevante Linux-/Browserprüfungen bestanden. | PR #88 gemergt; alle drei zugehörigen Workflows erfolgreich. Betroffene Kern-Dateien im heutigen `main` byteinhaltlich über identische Git-Blob-SHAs zugeordnet. |
| Folgefix auf Gateway installieren | Updateblock bereitgestellt; keine Ausführung durch Betreiber dokumentiert. | Bleibt offen. Ein Merge ist kein Rolloutbeleg. |
| Home Assistant, Homematic und fachliche Ende-zu-Ende-Zustellung | Nicht vollständig abgenommen. | Keine neue Betriebsabnahme gefunden. |

## 3. Chronologie der Diagnose und Reparaturen

### 3.1 Ursprünglicher Ausfall und belegte Neustartschleife

Der Textanhang zeigt wiederholt folgende Abfolge:

```text
IoT Gateway WebUI/API=10.0.1.119:8240 MQTT=127.0.0.1:1883 prefix=netcore/v1 sources=4
Error: Custom { kind: Other, error: "outbox limit 20000 would be exceeded" }
netcore-iot-gateway.service: Main process exited, code=exited, status=1/FAILURE
netcore-iot-gateway.service: Failed with result 'exit-code'.
netcore-iot-gateway.service: Scheduled restart job, restart counter is at 976.
```

Die erste Zeile ist eine Konfigurationsmeldung vor dem tatsächlichen Listenerstart. Sie beweist keinen bereits geöffneten HTTP-Port. In `main.rs` wird zuerst `SharedGateway::new(...)` ausgeführt; erst anschließend werden Poller, MQTT-Worker, Homematic-Worker und HTTP-Server gestartet. Das zusätzliche Einreihen der drei virtuellen Gerätezustände konnte deshalb die gesamte Initialisierung beenden, bevor der vorhandene Rückstau überhaupt versendet werden konnte.

Der Neustartzähler bis 976 ist im Originaljournal belegt. Die Logzeilen `Discovery: 0 dependency fields resolved` wurden ebenfalls angezeigt, belegen aber keine Ursache des Startabbruchs. Die direkte fatale Fehlermeldung und die Startreihenfolge ordnen den Abbruch der Outbox-Kapazität zu.

Am Ende des Rohlogs scheitert `curl` gegen `127.0.0.1:8240`. Der konfigurierte Bind war `10.0.1.119:8240`. Ein alleiniger Loopback-Fehler wäre bei diesem Bind kein hinreichender Ausfallbeweis; die unabhängigen Prozessabbrüche belegen die eigentliche Störung.

### 3.2 Wiederanlauffix — PR #83

Der Fix legt ausstehende virtuelle Startzustände in `startup_virtual_states_pending: VecDeque<String>` ab. Bei der Initialisierung wird nur eingereiht, solange ein Platz vorhanden ist. Eine vollständig gefüllte oder bereits über dem neu konfigurierten Limit liegende Outbox darf den Gateway weiter starten lassen. Nach erfolgreich veröffentlichten und lokal entfernten Nachrichten werden die noch ausstehenden Startzustände nachgereicht.

Die Begrenzung zusätzlicher Nachrichten bleibt bestehen. Bereits gespeicherte Nachrichten werden nicht als Reparatur gelöscht. Echte Fehler beim Anlegen, Lesen oder Schreiben des persistenten Zustands bleiben Fehler; die Korrektur unterdrückt nicht pauschal sämtliche Initialisierungsprobleme.

Implementiert und durch PR #83 nachgewiesen an **`b223ecc0f626e5dd8222d3edc0e969c774b8da21`**. Damaliger Testumfang: 18 Rust-Tests einschließlich vier neuer Zustandsregressionen sowie ein Python-Echtprozesstest mit den Unterfällen 100 und 105 bereits gespeicherter Nachrichten bei einem konfigurierten Limit von 100.

### 3.3 Erster Updateversuch: Git-Transport bricht ab

Der Betreiber startete einen streng abbrechenden Bash-Block mit `set -Eeuo pipefail` und einem temporären Quellverzeichnis unter `/opt/netcore-update-sources/iot-outbox-fix.XXXXXX`. Bereits der vollständige flache Fetch scheiterte:

```text
error: RPC failed; curl 56 Recv failure: Connection reset by peer
error: 7070 bytes of body are still expected
fetch-pack: unexpected disconnect while reading sideband packet
fatal: early EOF
fatal: fetch-pack: invalid index-pack output
```

**Ausführungsstand:** Der Fehler lag vor `systemctl stop` und vor der Übernahme des Dienstes. Dieser Block hat den Dienst daher nicht erst durch einen ausgeführten Stop außer Betrieb genommen. Eine genaue Ursache des Transportresets — Netzpfad, Proxy, Gegenstelle oder anderer Faktor — wurde nicht bewiesen.

Der ursprüngliche spätere Updateraufruf enthielt außerdem nur `--apply`, ohne den obligatorischen `--commit`. Das war ein zusätzlicher Fehler im vorgeschlagenen Ablauf, aber kein in dieser Ausgabe beobachteter Updaterfehler: Wegen des früheren Fetchabbruchs wurde diese Zeile nicht erreicht. Beide Probleme wurden im nächsten Ablauf korrigiert.

### 3.4 Erfolgreicher kleiner Quellcheckout und native Übernahme

Der zweite Versuch kombinierte HTTP/1.1, `--filter=blob:none`, ein Promisor-Remote und Sparse Checkout. Ausgecheckt wurden `system-backend`, `crates`, `bins` und `tools/operator` samt den durch Cone Mode erforderlichen Wurzeldateien. Der genaue Commit wurde geprüft; Build und vorbereitete Installationsdateien entstanden vor dem Dienststopp.

**Im Betrieb erfolgreich ausgeführt:**

- Quellverzeichnis: `/opt/netcore-update-sources/iot-fix-small.2Ml8FO`.
- Metadaten-Pack: 128,80 KiB; nachgeladener Blob-Pack: 3,77 MiB; 1.189 aktualisierte Dateien.
- `HEAD` exakt `b223ecc0f626e5dd8222d3edc0e969c774b8da21`.
- Build-Pakete bereits vorhanden: `build-essential pkg-config libssl-dev libsqlite3-dev cmake git curl ca-certificates`; deshalb kein `apt-get update/install` in dieser Vorbereitung.
- `rustc 1.99.0 (b940084d7 2026-09-28)` und `cargo 1.99.0 (5f94df478 2026-08-27)` laut Betreiberoutput; Stable-Synchronisation unverändert, Updater setzt `RUSTUP_TOOLCHAIN=stable`.
- Nativer Release-Build des Gateways: 29,85 s. Die Warnung zur unbenutzten Funktion `execute_sandbox` war nicht fatal.
- Vorbereitungsplan: `/opt/netcore-update-sources/iot-fix-small.2Ml8FO/.netcore-prepared/iot-gateway.json`.
- Ziel: `/usr/local/bin/netcore-iot-gateway`.
- Neuer SHA-256: `32acbedd105aa1859db10f034af83fff6baf45c83a84512e99d6dbbb0bf3048d`.
- Vorheriger SHA-256: `e9d4e3a22cb2457ccb8d821269ff94de62c7022adcade753f557d860ea34e4c2`.
- Backup: `/var/backups/netcore-service-update/iot-gateway-5l57rpap`.
- Übernahmebeleg: `/var/lib/netcore-service-update/iot-gateway.json`.

Der Übernahmebeleg meldet `phase=inactive_updated`, `assets_complete=true`, `verified=true`, `active_before=inactive`, `MainPID=0` und zunächst `liveness=null`/`readiness=null`. Das ist mit dem Ablauf konsistent: Der Betreiber hatte den Dienst zwischen Vorbereitung und Übernahme manuell gestoppt. Die Dateiübernahme ist damit belegt; ein laufender Prozess war zu diesem Zeitpunkt noch nicht Gegenstand der Verifikation. Anschließend wurde der Dienst explizit gestartet.

Zwei anfängliche Verbindungsablehnungen wurden durch `curl --retry-connrefused` überbrückt. Danach antwortete `/health/live` mit `{"status":"live"}` und `/api/v1/status` mit einem laufenden Gateway. Diese beiden anfänglichen Ablehnungen sind kein Beleg eines erneuten dauerhaften Ausfalls.

### 3.5 Erste Statusaufnahme nach dem Start

Der Gateway meldete Version `1.3.0`, Phase 5 und Startzeit **`2026-10-13T23:12:22.806Z`**. MQTT war ab `23:12:22.827Z` verbunden, `mqtt_last_error=null`, Client-ID `netcore-iot-gateway`, Host `127.0.0.1`, Port 1883.

In dieser ersten Momentaufnahme standen 20.000 Nachrichten an, 0 Publish-Abschlüsse waren gezählt, 0 von 4 Quellen waren gesund, `events_seen=33`, `invalid_events=32` und `home_assistant_discovery_runs=0`. Diese unmittelbar nach dem Start gewonnenen Anfangswerte wurden ausdrücklich **nicht** als Beweis einer dauerhaften MQTT- oder Quellenstörung behandelt. Die späteren Messungen haben diese offene Einordnung konkretisiert.

Weitere gemeldete Betriebsparameter: `security_mode=open_lab`, `command_execution_enabled=true`, `command_execution_mode=open_lab_sandbox`, drei aktive Command-Policies, drei virtuelle Geräte, `home_assistant_enabled=true`, `homematic_enabled=false`, `homematic_mode=home_assistant_mqtt` und noch keine externen HA-Entitäten beziehungsweise direkten Homematic-Datenpunkte.

### 3.6 Drei Folgeproben und bestätigte UI-Wiederherstellung

Der Betreiber führte drei Statusproben im Abstand von jeweils zehn Sekunden aus:

| Probe | MQTT verbunden | Letzter MQTT-Fehler | `broker_publish_acks` | `outbox_pending` | Gesunde Quellen | `invalid_events` |
|---|---|---|---:|---:|---:|---:|
| 1 | Ja | `null` | 1.234 | 19.999 | 4 | 15.022 |
| 2 | Ja | `null` | 1.273 | 19.997 | 4 | 15.179 |
| 3 | Ja | `null` | 1.320 | 19.997 | 4 | 15.301 |

Alle vier Quellen — `call-control`, `mobility-core`, `node-gateway`, `sds-router` — meldeten `healthy=true` und `last_error=null`. Die gezeigten letzten zehn Ereignisse von `2026-10-13T23:18:47.449Z` bis `.601Z` waren `event.rejected` mit `reason="outbox limit 20000 would be exceeded"`; betroffen waren `node-gateway`, `mobility-core` und `sds-router`.

Es wurden somit **86 weitere Publish-Abschlüsse in 20 Sekunden**, also rund 4,3/s, gezählt. Gleichzeitig sank der Ausgang in dieser kleinen Stichprobe netto nur um zwei Einträge und blieb praktisch voll. Die Stichprobe beweist tatsächlichen Versandfortschritt, aber keinen nachhaltigen Abbau des Rückstaus unter dem laufenden Zufluss.

Die ausdrückliche Betreiberbestätigung **„Web-UI geht wieder“** schließt die ursprüngliche UI-Ausfallmeldung für diesen beobachteten Zustand. Sie ist keine vollständige Fachabnahme von Home Assistant, Homematic oder der Zustellung an alle MQTT-Abonnenten.

Der alte Poller zählte jeden Fehler beim Einreihen als `invalid_events`. Die gezeigten zehn Ablehnungen sind nachweislich Kapazitätsfehler. Daraus darf nicht abgeleitet werden, dass sämtliche 15.301 historischen Zählererhöhungen dieselbe Ursache oder umgekehrt tatsächlich einen Formatfehler hatten.

### 3.7 Folgefix für Versand und Rückdruck — PR #88

Die Codeprüfung fand drei konkrete Probleme:

1. Der MQTT-Worker prüfte Sendeaufträge und las dann blockierend am Socket mit 200-ms-Timeout. Nach einer PUBACK konnte der Outbox-Worker den nächsten Auftrag erst einreichen, nachdem der Verbindungsworker bereits wieder im Socket-Read wartete. Damit entstand häufig eine zusätzliche Wartezeit je Nachricht; die Größenordnung passt zu den gemessenen rund 4,3/s.
2. `next_outbox_message()` las und sortierte vor jeder Nachricht erneut das vollständige Outbox-Verzeichnis. Bei rund 20.000 Dateien war das unnötige wiederholte Arbeit.
3. `enqueue_source_state_locked()` schrieb Quellenstatus-Nachrichten ohne vorherige Kapazitätsprüfung. Der Poller unterschied zudem Vertrags-, Speicher- und Platzfehler nicht ausreichend.

Die Folgekorrektur wurde an **`0dffd40ea5687794b276b6dd4e54f2f645729746`** implementiert:

- Ein einzelner MQTT-Schreiber wird über einen begrenzten `sync_channel(32)` durch `Publish`, `Packet`, `ReaderError` oder `Wake` aufgeweckt. Neue Sendeaufträge müssen nicht auf einen Socket-Lesetimeout warten.
- Ein eigener `PacketReader` liest vollständige Pakete und reicht sie an diesen Kanal weiter. Bei vollem Kanal wartet er mit kontrolliertem `try_send` und Stopprüfung; der Veröffentlichungsaufruf kann bei vollem Kanal einen vorübergehenden Fehler zurückgeben, sodass die persistente Outbox später erneut sendet.
- Vor dem ersten Paketbyte gilt für den Leser ein 200-ms-Idle-Timeout; nach Paketbeginn sind fünf Sekunden für die restlichen, gegebenenfalls fragmentierten Bytes vorgesehen. Ein bloßes Verkürzen des alten Socket-Timeouts wäre kein gleichwertiger Fix für fragmentierte Pakete.
- Reader-Aufräumen setzt das Stopflag, führt Socket-Shutdown aus und wartet auf den Thread. Ein manueller Reconnect setzt das bestehende Flag und weckt den Verbindungsworker.
- Eine `VecDeque<PathBuf>` hält sortierte Gruppen von höchstens 256 Dateipfaden. Der gültige Kopf bleibt bis zur erfolgreichen Veröffentlichung und lokalen Entfernung stehen und wird bei einem Sendefehler erneut angeboten. Fehlende Dateien werden übersprungen; ungültiges JSON wird wie bisher protokolliert und durch Umbenennung auf `.invalid` isoliert.
- Quellenstatus-Nachrichten beachten jetzt das Outbox-Limit.
- `EnqueueOutcome` unterscheidet `Enqueued`, `Duplicate` und `Deferred`. Ein gültiges Ereignis ohne Platz wird nicht in die Duplikaterkennung aufgenommen.
- Der Quellenpoller beendet die aktuelle Gruppe beim ersten `Deferred`, protokolliert `event.deferred` mit `reason=outbox_capacity` und versucht die Annahme beim nächsten Poll erneut, **solange das Ereignis noch in der Historie des Quelldienstes enthalten ist**.
- `EventEnqueueError::Invalid` ist von `Storage` getrennt. Vertragsfehler zählen als ungültige Ereignisse; Speicherfehler melden einen Quellenfehler.
- `events_deferred` erscheint global und pro Quelle in der API sowie in der UI. Prometheus ergänzt `netcore_iot_gateway_events_deferred` und `netcore_iot_gateway_invalid_events`.

`events_deferred` zählt Zurückstellungsversuche. Wiederholte Zurückstellung desselben Ereignisses kann den Zähler mehrfach erhöhen. Die Änderung baut weder eine unbegrenzte zweite Warteschlange noch eine Garantie für bereits aus der Quellhistorie verschwundene Ereignisse ein.

**Historischer Abschluss:** PR #88 war angelegt, lokale Tests und die Linux-Service-/Browserprüfungen waren erfolgreich. Der allgemeine OpenLab-Workflow lief noch. Ein vollständiger Updateblock stand im PR. Eine Installation dieses Folgefixes auf `10.0.1.119` ist im Chat nicht belegt.

## 4. Architektur, Schnittstellen und Abhängigkeiten

### 4.1 Daten- und Steuerpfade

Der Rust-Dienst pollt die kanonischen HTTP-Ereignisquellen mit einem blockierenden `reqwest`-Client. Eingehende `netcore-event-v1`-Ereignisse werden validiert, anhand der Ereignis-UUID dedupliziert und als persistente `OutboundMessage`-JSON-Dateien abgelegt. Ein Ereignis ohne Subjekt benötigt einen Outbox-Platz; bei vorhandenem Subjekt kommen Ereignisnachricht und retained Zustandsnachricht hinzu, also zwei Plätze.

Der Outbox-Worker sendet über den MQTT-Verbindungsworker. Bei QoS 1 wartet der Veröffentlichungsaufruf auf die zugehörige PUBACK. Erst nach dem erfolgreichen Veröffentlichungsabschluss versucht der Gateway, die lokale Datei zu entfernen. Die implementierten Wiederholungen können erneute Zustellungen verursachen; ein genau-einmaliger Ende-zu-Ende-Effekt wird damit nicht behauptet. MQTT-Brokerquittung, lokales Entfernen und tatsächliche Verarbeitung durch einen HA-/anderen Abonnenten sind getrennte Schritte.

Web-UI/API, HTTP-Quellenpoller, MQTT-Verbindungsworker und Outbox-Worker teilen `SharedGateway`, intern `Arc<Mutex<GatewayState>>`. Das Aufschieben der Startzustände löst die frühere Abhängigkeit „vor dem Start aller Worker müssen alle Startmeldungen in die Outbox passen“ auf.

Der Gateway unterstützt außerdem `netcore-command-v1` und `netcore-command-ack-v1`, policygesteuerte virtuelle Laborgeräte, Home-Assistant-MQTT-Discovery und optionale Homematic-Anbindung. Der konkrete Chat repariert deren gemeinsame Dienstverfügbarkeit und Outbox-Verhalten; er implementiert keine neue Feldgeräte- oder Funkstreckenfunktion.

### 4.2 HTTP-Endpunkte

| Endpunkt | Rolle und Nachweisgrenze |
|---|---|
| `GET /` | Web-UI; vom Betreiber wieder erreichbar bestätigt, in Echtprozess- und Browserregressionen geprüft. |
| `GET /health/live` | Prozess-/HTTP-Liveness, unabhängig von einer vollständig abgearbeiteten Outbox. |
| `GET /health/ready` | Aktueller Code verlangt MQTT-Verbindung, mindestens eine aktivierte Quelle und Gesundheit aller aktivierten Quellen. Die Queuelänge ist kein Kriterium; auch eine volle Outbox kann daher mit Readiness vereinbar sein. Der Statuscode wurde auf dem Host nach dem Fix nicht separat mitgeliefert. |
| `GET /api/v1/status` | Gateway-, Verbindungs-, Quellen-, Outbox- und Ereigniszähler. |
| `GET /api/v1/sources` | Einzelne Quellen einschließlich Gesundheit, letztem Fehler und Zählern. |
| `GET /api/v1/events?limit=10` | Letzte interne Bridge-Ereignisse, im Chat zur Kapazitätsdiagnose genutzt. Das ist nicht derselbe HTTP-Eingang wie die kanonischen Quelldienst-Ereignisse. |
| `GET /api/v1/outbox` | Zusammenfassende Outbox-Dateiansicht. |
| `GET /api/v1/home-assistant` | HA-Adapterstatus; noch keine separate Hostabnahme im Chat. |
| `POST /api/v1/actions/reconnect` | Manueller MQTT-Reconnect; Code vorhanden, keine dokumentierte Betreiberausführung hier. |
| `POST /api/v1/actions/home-assistant-discovery` | Explizites erneutes Einreihen der HA-Discovery; kein Ausführungsnachweis in diesem Chat. |
| `POST /api/v1/test/publish` | Testnachricht; im Echtprozesstest bei voller Outbox HTTP 409, nach Wiederherstellung HTTP 202. Kein Hosttest mit realen Aktoren daraus abzuleiten. |
| `GET /metrics` | Prometheus-Textformat mit Verbindungs-, Outbox-, Quellen- und Ereigniskennzahlen. |

### 4.3 Standardquellen und MQTT-Themen

Die folgenden Quellen-URLs sind die am Prüftag gelesenen **Beispiel-/Defaultwerte**, keine neu abgefragten effektiven IP-Ziele des Diensthosts:

| Quelle | Default-URL |
|---|---|
| `node-gateway` | `http://node-gateway:8080/api/v1/events/netcore` |
| `mobility-core` | `http://mobility-core:8090/api/v1/events/netcore` |
| `call-control` | `http://call-control:8120/api/v1/events/netcore` |
| `sds-router` | `http://sds-router:8150/api/v1/events/netcore` |

Default-Polling: alle 2.000 ms, bis 500 Ereignisse pro Quelle, HTTP-Timeout 2.500 ms. Die Phase-2-APIs liefern neueste Einträge zuerst; der Poller kehrt die Gruppe für chronologische Verarbeitung um. `event_id` verhindert innerhalb des Dedup-Fensters erneutes Einreihen.

MQTT verwendet in der beobachteten Konfiguration den Präfix `netcore/v1`. Ereignisse werden unter `netcore/v1/events/...`, abgeleitete Zustände unter `netcore/v1/state/...` und Quellenstatus unter `netcore/v1/state/integrations/iot_gateway_sources/...` veröffentlicht. Commands liegen unter `netcore/v1/commands/#`. HA-Defaults sind `homeassistant` als Discovery-Präfix und `homeassistant/status` für den Birth-/Statuskanal. Das sind Code-/Beispielwerte; eine vollständige Broker-Topicinventur des Hosts liegt nicht vor.

### 4.4 Relevante Bibliotheken und Buildvoraussetzungen

Das Paket `netcore-iot-gateway` verwendet die gemeinsamen Workspace-Crates `netcore-contracts` und `netcore-service-common`; hinzu kommen unter anderem `clap`, `chrono`, `serde`/`serde_json`, `toml`, `uuid`, `tracing` und `reqwest 0.12` mit den Features `blocking`, `json`, `rustls-tls` ohne dessen Default-Features. Der Rust-Workspace benötigt auch beim Sparse Checkout die ausgewählten lokalen Paketverzeichnisse und die Wurzel-Manifeste/Lockdatei.

Das Modul implementiert die MQTT-Paketverarbeitung selbst über TCP. Der native Release-Build und der Python-Updater setzen eine funktionierende Rust/Cargo-Toolchain und die vom Updater geprüften nativen Buildpakete voraus. Der erfolgreiche Hostlauf änderte keine Brokerinstallation und nahm kein pauschales Betriebssystemupgrade vor.

## 5. Dateien, Pfade und technische Parameter

### 5.1 Host und systemd

| Parameter/Pfad | Einordnung |
|---|---|
| Host | `IoT-Gateway`, beobachtete Adresse `10.0.1.119` |
| Dienst | `netcore-iot-gateway.service` |
| HTTP im Betreiberlog | `10.0.1.119:8240`; Beispieldefault `0.0.0.0:8240` ist davon zu unterscheiden. |
| MQTT im Betreiberstatus | `127.0.0.1:1883`, Client-ID `netcore-iot-gateway` |
| Laufzeitprogramm | `/usr/local/bin/netcore-iot-gateway` |
| Standardkonfiguration | `/etc/netcore/iot-gateway.toml` |
| Unit im Quellbaum | `system-backend/iot-gateway/systemd/netcore-iot-gateway.service` |
| Standard-Unitinstallation | `/etc/systemd/system/netcore-iot-gateway.service` |
| Standard-Dienstidentität | `User=netcore-iot-gateway`, gleichnamige Gruppe; aktuelle Drop-ins auf dem Host nicht neu erhoben. |
| Startbefehl laut Unit | `/usr/local/bin/netcore-iot-gateway --config /etc/netcore/iot-gateway.toml` |
| Neustartregel laut Unit | `Restart=on-failure`, `RestartSec=3` |
| Unit-Schutzparameter | `NoNewPrivileges=true`, `PrivateTmp=true`, `ProtectSystem=strict`, `ProtectHome=true`, `ReadWritePaths=/var/lib/netcore-iot-gateway` |
| Zustandswurzel | Standard `/var/lib/netcore-iot-gateway` |
| Quellbereich für die Reparatur | `/opt/netcore-update-sources/`; erfolgreicher Checkout `iot-fix-small.2Ml8FO` |

Das ursprüngliche Journal zeigt zeitweise `python3` als Prozessnamen. Daraus wird hier keine alternative Python-Gatewayimplementierung abgeleitet. Der später dokumentierte Runtime-Austausch betrifft ein natives ELF-Binary; die tatsächliche laufende Prozess-/Wrapperidentität hätte gesondert geprüft werden müssen.

### 5.2 Persistente Dateien und Limits

Unterhalb der standardmäßigen Zustandswurzel verwendet die Beispielkonfiguration:

| Pfad/Parameter | Bedeutung / Standardwert |
|---|---|
| `outbox/` | Persistente MQTT-Nachrichten als JSON-Dateien |
| `dedup.json` | Ereignis-UUIDs zur Deduplizierung |
| `command-inbox.ndjson` | Command-Eingang |
| `command-ledger.json` | Persistentes Command-Ledger |
| `command-audit.ndjson` | Command-Audit |
| `virtual-device-state.json` | Zustände der virtuellen Laborgeräte |
| `external-entity-state.json` | Externe HA-Entitätszustände |
| `homematic-datapoint-state.json` | Homematic-Datenpunkte |
| `storage.outbox_limit` | 20.000 zusätzliche persistente Nachrichtenplätze; Bestand über dem Limit verhindert seit PR #83 nicht den Start. |
| `storage.dedup_limit` | 50.000 Ereignis-UUIDs |
| `storage.command_ledger_limit` | 50.000 Command-Einträge |
| `server.history_limit` | 1.000 interne Ereignisse |
| Konfigurationsvalidierung | Outbox-, Dedup- und Command-Ledger-Limit jeweils mindestens 100; einzelne Rust-Unit-Tests umgehen die Konfigurationsvalidierung bewusst für kleine Zustandsfälle. |
| MQTT-Beispielwerte | QoS 1, Keepalive 30 s, `clean_session=false`, Reconnect 3 s, Publish-Timeout 8 s, Ereignisse nicht retained, Zustände retained |

In der Hostaufnahme sind Phase 5, `open_lab` und `open_lab_sandbox` bestätigt. Es gibt keine Anmeldung, Tokens oder TLS in diesem Modus. Reale HA-/Homematic-Schreibzugriffe bleiben explizit zu aktivierende, gesonderte Pfade. Die Reparaturen ändern keine Aktorfreigaben und rechtfertigen keinen Rückschluss auf tatsächlich ausgeführte Feldgeräte-Commands.

### 5.3 Maßgebliche Repository-Dateien

| Datei | Rolle |
|---|---|
| [main.rs](../../system-backend/iot-gateway/src/main.rs) | Initialisierungsreihenfolge, Konfigurationsargumente und Workerstart |
| [state.rs](../../system-backend/iot-gateway/src/state.rs) | Persistenz, Kapazität, Startzustände, Outbox-Gruppen, Zähler und Zustandsregressionen |
| [mqtt.rs](../../system-backend/iot-gateway/src/mqtt.rs) | Publisher, Reader, begrenzter Ereigniskanal, PUBACK-Verarbeitung, Reconnect und Fragmenttest |
| [poller.rs](../../system-backend/iot-gateway/src/poller.rs) | HTTP-Quellenpolling, Chronologie, Dedupe-/Deferred-/Fehlerbehandlung |
| [model.rs](../../system-backend/iot-gateway/src/model.rs) | Serialisierbare Statusmodelle einschließlich `events_deferred` |
| [config.rs](../../system-backend/iot-gateway/src/config.rs) | Defaults, Pfade und Validierung |
| [http.rs](../../system-backend/iot-gateway/src/http.rs) | API, Liveness/Readiness und Actions |
| [index.html](../../system-backend/iot-gateway/web-ui/index.html) | Gateway-Web-UI mit separater Anzeige von Zurückstellungen |
| [iot-gateway.example.toml](../../system-backend/iot-gateway/config/iot-gateway.example.toml) | Default-/Beispielkonfiguration; kein Dump der effektiven Hostkonfiguration |
| [update-netcore-service.py](../../tools/operator/update-netcore-service.py) | Commitgebundene Buildvorbereitung, Dateiaustausch, Verifikation und Rollback |
| [test_iot_gateway_outbox.py](../../tools/test_iot_gateway_outbox.py) | Echte Gateway-Prozesse, HTTP-Testquelle und MQTT-Testbroker |
| [service-ui-tests.yml](../../.github/workflows/service-ui-tests.yml) | Rust-Serviceprüfungen, Browserprüfungen und tatsächlicher IoT-Echtprozesstest unter Linux |
| [installation-im-lxc.md](../services/iot-gateway/installation-im-lxc.md) | Gepflegte Wiederanlauf-, Transfer- und Rückdruckanleitung |
| [wiederanlauf-2026-10-14.json](../services/iot-gateway/tests/evidence/wiederanlauf-2026-10-14.json) | Strukturierter Betreiberbeleg einschließlich Folgeproben und offener Abnahme |

## 6. Wichtige Befehle und ihr tatsächlicher Ausführungsstand

### 6.1 Fehlgeschlagener Transport: nicht als funktionierende Reparatur wiederholen

Der folgende Kern des ersten Blocks wurde ausgeführt und scheiterte beim Fetch:

```bash
NC_REF=b223ecc0f626e5dd8222d3edc0e969c774b8da21
NC_SRC=$(mktemp -d /opt/netcore-update-sources/iot-outbox-fix.XXXXXX)
git -C "$NC_SRC" init -q
git -C "$NC_SRC" fetch --depth 1 https://github.com/JanHG98/netcore-tetra.git "$NC_REF"
```

Die danach vorgesehenen Stop-/Apply-/Startzeilen wurden wegen `set -Eeuo pipefail` nicht erreicht. Der dortige reine `--apply`-Aufruf ohne `--commit` ist überholt und wird nicht als funktionsfähige Updateranleitung übernommen.

### 6.2 Erfolgreich vom Betreiber ausgeführter Wiederanlaufblock für PR #83

Der folgende Ablauf ist historisch tatsächlich ausgeführt. Er ist auf den damaligen Fixcommit gepinnt und kein Auftrag, heute einen neueren installierten Stand zurückzusetzen:

```bash
(
  set -Eeuo pipefail
  NC_REF=b223ecc0f626e5dd8222d3edc0e969c774b8da21
  install -d -m 0755 /opt/netcore-update-sources
  NC_SRC=$(mktemp -d /opt/netcore-update-sources/iot-fix-small.XXXXXX)

  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" remote add origin https://github.com/JanHG98/netcore-tetra.git
  git -C "$NC_SRC" config http.version HTTP/1.1
  git -C "$NC_SRC" config remote.origin.promisor true
  git -C "$NC_SRC" config remote.origin.partialclonefilter blob:none
  git -C "$NC_SRC" fetch --depth 1 --filter=blob:none origin "$NC_REF"
  git -C "$NC_SRC" sparse-checkout init --cone
  git -C "$NC_SRC" sparse-checkout set system-backend crates bins tools/operator
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  [[ "$(git -C "$NC_SRC" rev-parse HEAD)" == "$NC_REF" ]]
  chmod 0755 "$NC_SRC"

  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service iot-gateway --commit "$NC_REF" --prepare --prepare-build-deps

  systemctl stop netcore-iot-gateway.service
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service iot-gateway --commit "$NC_REF" --apply
  systemctl reset-failed netcore-iot-gateway.service
  systemctl start netcore-iot-gateway.service

  curl --noproxy '*' --retry-connrefused --retry 30 --retry-delay 1 --max-time 5 \
    -fsS http://10.0.1.119:8240/health/live
  curl --noproxy '*' --max-time 5 -fsS http://10.0.1.119:8240/api/v1/status
)
```

Der tatsächliche `mktemp`-Pfad wurde `.../iot-fix-small.2Ml8FO`. Der Modus 0755 ermöglicht die Verzeichnisdurchquerung auch für die Dienstidentität. Die Befehle entfernen keine Outbox- oder Konfigurationsdateien.

### 6.3 Tatsächlich ausgeführte Folgeprobe

Der Betreiber verwendete folgenden Python-Kern; die Werte stehen in Abschnitt 3.6:

```python
import json, time
from urllib.request import urlopen

base = "http://10.0.1.119:8240"

def get(path):
    with urlopen(base + path, timeout=15) as response:
        return json.load(response)

keys = ("mqtt_connected", "mqtt_last_error", "broker_publish_acks",
        "outbox_pending", "sources_healthy", "invalid_events")

for i in range(3):
    status = get("/api/v1/status")
    print(json.dumps({"probe": i + 1, **{k: status.get(k) for k in keys}}))
    if i < 2:
        time.sleep(10)

print("QUELLEN:")
for source in get("/api/v1/sources"):
    print(json.dumps({k: source.get(k)
                     for k in ("id", "healthy", "last_error")}))

print("LETZTE EREIGNISSE:")
print(json.dumps(get("/api/v1/events?limit=10"), indent=2))
```

Für eine neue Abnahme ist zusätzlich `events_deferred` aufzunehmen. Diese Erweiterung ist eine hier dokumentierte nächste Prüfaufgabe, kein bereits vorliegender Messwert des Betreibers. Hinter einer Proxyumgebung sind direkte Hostzugriffe wie in den Curl-Prüfungen ausdrücklich ohne Proxy vorzusehen.

### 6.4 Bereitgestellter, auf dem Diensthost nicht nachgewiesen ausgeführter Folgefix-Block

Dieser Ablauf wurde in PR #88 bereitgestellt und als Bash-Syntax geprüft. **Er ist keine bestätigte Hostausführung.** Er setzt voraus, dass der damalige Checkout noch vorhanden ist. Vor einer heutigen Fortsetzung sind installierter Stand, Konfiguration und Quelle zu erfassen, um keinen eventuell bereits neueren Dienst auf einen älteren Commit zurückzusetzen.

```bash
(
  set -Eeuo pipefail
  NC_REF=0dffd40ea5687794b276b6dd4e54f2f645729746
  NC_SRC=/opt/netcore-update-sources/iot-fix-small.2Ml8FO

  git -C "$NC_SRC" fetch --depth 1 --filter=blob:none origin "$NC_REF"
  git -C "$NC_SRC" checkout --detach FETCH_HEAD
  [[ "$(git -C "$NC_SRC" rev-parse HEAD)" == "$NC_REF" ]]

  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service iot-gateway --commit "$NC_REF" --prepare --prepare-build-deps

  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service iot-gateway --commit "$NC_REF" --apply

  systemctl reset-failed netcore-iot-gateway.service
  systemctl start netcore-iot-gateway.service
  curl --noproxy '*' --retry-connrefused --retry 30 --retry-delay 1 --max-time 5 \
    -fsS http://10.0.1.119:8240/health/live
  curl --noproxy '*' --max-time 5 -fsS http://10.0.1.119:8240/api/v1/status
)
```

Die spätere Fassung lässt einen bereits laufenden Dienst bis zur geprüften Vorbereitung aktiv. Der Updater übernimmt im Apply-Schritt den nötigen Stop, Dateiaustausch und Neustart einschließlich Rollback bei Fehlern selbst. Der abschließende explizite Start deckt auch einen vor dem Update inaktiven Dienst ab. Dies ersetzt als Empfehlung den unnötigen eigenen Vorabstopp aus dem erfolgreichen historischen Block; dessen tatsächliches Ergebnis bleibt unverändert dokumentiert.

### 6.5 Entwicklungsprüfungen

```bash
cargo test --locked --offline -p netcore-iot-gateway
cargo build --locked --offline -p netcore-iot-gateway
python3 -B tools/test_iot_gateway_outbox.py
```

Diese Befehle wurden in der Entwicklungsumgebung ausgeführt. Die Linux-CI verwendet die entsprechenden `cargo ... --locked`-Befehle und baut das echte Binary vor `python3 tools/test_iot_gateway_outbox.py`. `--offline` setzt bereits vorhandene Cargo-Abhängigkeiten voraus und ist keine allgemeine Vorgabe für einen frischen Host.

## 7. Fehler, Einordnung und ersetzte Ansätze

| Befund/Ansatz | Diagnose und wirksame Behandlung | Grenze / endgültige Einordnung |
|---|---|---|
| Web-UI wirkt down | Gateway beendet sich vor Listenerstart durch volle Outbox. PR #83 verschiebt zusätzliche virtuelle Startzustände. | UI-Wiederherstellung ist auf dem Host bestätigt. Ursache der ursprünglichen Befüllung bleibt offen. |
| Hunderte systemd-Neustarts | Wiederholter identischer Initialisierungsfehler, Zähler bis 976. | Wiederholtes bloßes `systemctl start` hätte das Kapazitätsproblem nicht beseitigt. |
| `curl` gegen Loopback schlägt fehl | Bind im Log ist `10.0.1.119:8240`; mit tatsächlicher Adresse prüfen. | Loopback-Fehler allein ist kein Ausfallbeweis. |
| Git `curl 56` / Early EOF | Voller flacher Fetch abgebrochen; kleiner gezielter Fetch mit HTTP/1.1, Blobfilter und Sparse Checkout funktioniert. | Keine gesicherte Einzelursache des Netzresets; kein pauschales „GitHub war down“. |
| Updater ohne `--commit` | Unvollständiger ursprünglicher Aufruf; in der funktionierenden Fassung korrigiert. | Keine beobachtete Ausführung dieses fehlerhaften Apply-Aufrufs im ersten Block. |
| `inactive_updated`, `MainPID=0` | Erwarteter Beleg für Dateiaustausch nach manuellem Dienststopp. | Nicht als fehlgeschlagener Start oder bereits verifizierte aktive Prozessidentität ausgeben. |
| Zwei Curl-Connection-Refused nach Start | Durch vorhandene Wiederholungsoption überbrückt; danach Liveness und Status erfolgreich. | Normale Anlaufbeobachtung, kein nachgewiesener neuer Dauerfehler. |
| Anfangs 0 Quittungen / 0 gesunde Quellen | Direkt nach dem Start erfasste Momentaufnahme. | Spätere Proben zeigen Versand und vier gesunde Quellen. Frühere Unsicherheit dadurch präzisiert. |
| Rund 4,3 Publish-Abschlüsse/s bei voller Queue | 200-ms-Socketwartefenster plus wiederholte Vollsortierung; durch Kanal-Weckmechanismus und Dateigruppen korrigiert. | Tests zeigen Fixwirkung am Testbroker, noch keine Lastabnahme am Host. |
| Stark steigendes `invalid_events` | Alter Poller zählt auch Kapazitätsfehler. Neuer `Deferred`-Pfad und getrennte Fehlertypen. | Die zehn gezeigten Gründe sind bekannt; die gesamte historische Menge ist nicht vollständig klassifiziert. |
| Quellenstatus überschreitet Kapazität | Fehlende Prüfung in `enqueue_source_state_locked()` ergänzt. | Quellen können HTTP-seitig gesund bleiben, obwohl Ereignisse zurückgestellt werden. |
| Outbox löschen oder Limit pauschal erhöhen | Nicht als Reparatur durchgeführt oder beschlossen; vorhandene Nachrichten und Kapazitätsgrenze erhalten. | Die implementierte Lösung setzt am Start-/Versand-/Rückdruckverhalten an. |
| Nur MQTT-Read-Timeout weiter verkürzen | Kein gewählter Ersatz für das Aufwecken des Schreibers; nach Beginn eines fragmentierten Pakets wäre eine zu kurze Frist problematisch. | Eigenständiger Leser mit vollständigem Paket und getrennten Idle-/Paketfristen implementiert. |
| Dauernd sämtliche Ereignisse trotz voller Queue abweisen | Durch Stopp der jeweiligen Gruppe am ersten `Deferred` ersetzt. | Erneutes Polling hängt von der Quellhistorie ab; verlorene Historie wird nicht nachträglich rekonstruiert. |

## 8. Tests, Ergebnisse und Aussagegrenzen

### 8.1 Wiederanlauffix PR #83

Die Zustandsregressionen prüfen wiederholten Start mit voller, überfüllter und teilweise gefüllter Outbox, unveränderten Erhalt alter Dateien, Nachreichen virtueller Startzustände nach Quittierung sowie echte Speicherfehler. Ein zusätzlicher kleiner Test deckt das Nachreichen mit einem einzigen Outbox-Platz ab; solche internen Tests ersetzen nicht die Mindestwerte der öffentlich validierten Konfiguration.

Der Python-Echtprozesstest startet das tatsächlich gebaute Programm mehrfach mit 100 und 105 alten Einträgen bei Limit 100. Während der Broker noch nicht arbeitet, bleiben `/` und `/health/live` verfügbar; `/health/ready` ist negativ, die vorhandenen Dateien bleiben erhalten und manuelles zusätzliches Publish wird mit 409 abgewiesen. Nach Brokerwiederkehr werden alte Nachrichten und drei Startzustände abgearbeitet; Testpublish ist anschließend mit 202 möglich.

Die am 16.10.2026 erneut gelesenen historischen Workflows sind alle erfolgreich und dem exakten Fixcommit `b223ecc0...` zugeordnet:

| Workflow | Lauf |
|---|---|
| NetCore service WebUIs | [38478808790](https://github.com/JanHG98/netcore-tetra/actions/runs/38478808790) |
| OpenLab deployment and discovery | [38478808745](https://github.com/JanHG98/netcore-tetra/actions/runs/38478808745) |
| Deployment inventory and source gate | [38478809002](https://github.com/JanHG98/netcore-tetra/actions/runs/38478809002) |

### 8.2 Folgefix PR #88

**24 lokale Rust-Tests bestanden.** Neue relevante Regressionen prüfen:

- Gültige Ereignisse werden bei voller Queue zurückgestellt, weder als ungültig markiert noch vorzeitig dedupliziert; nach freiem Platz sind Annahme und anschließende Duplikatunterdrückung möglich.
- Erfolgreiche und fehlschlagende Quellenabfragen erzeugen keinen unkontrollierten zusätzlichen Outbox-Eintrag über dem Limit.
- Speicherfehler sind von Vertragsfehlern unterscheidbar.
- Dateigruppen erhalten Reihenfolge und Wiederholbarkeit des Kopfes, finden später ergänzte Nachrichten und verarbeiten mehr als eine Gruppe.
- Fehlende Dateien und defektes JSON verhindern die weitere Dateiauswahl nicht; defekte Dateien werden isoliert.
- Eine über mehr als 200 ms fragmentiert eintreffende PUBACK bleibt korrekt; der Reader beendet sich im Leerlauf innerhalb der getesteten Grenze.

Die zwei Python-Testmethoden prüfen das wirkliche Programm. Eine Methode umfasst die Unterfälle 100 und 105 alte Nachrichten und zwei Starts; die zweite betreibt zusätzlich eine echte lokale HTTP-Testquelle mit drei kanonischen Ereignissen. Bei voller Queue bleiben Quelle und HTTP-Dienst erreichbar, es wächst `events_deferred` statt `invalid_events`, die Queue bleibt begrenzt, und die Ereignisse werden nach Brokerwiederkehr einmalig in chronologischer Reihenfolge eingereiht und veröffentlicht.

| Umgebung | 100 alte Nachrichten + 3 Startzustände | 105 alte Nachrichten + 3 Startzustände | Gesamter Echtprozesstest |
|---|---:|---:|---|
| Historischer lokaler Windows-Lauf | 0,514 s | 0,534 s | 2 Testmethoden bestanden, protokollierte Gesamtdauer 6,631 s |
| Linux-CI, Job `115598069269` | 0,511 s | 0,460 s | 2 Testmethoden bestanden, Gesamtdauer 3,064 s |

Die Zeitmessung beginnt beim Aktivieren des Testbrokers und umfasst den beobachteten Abbau bis leer. Der Test verwendet 10 s Abbaufrist je Unterfall; der frühere Test hatte 45 s zugelassen. Die kurzen Zeiten belegen, dass die alte häufige Wartezeit pro Nachricht in diesen Fällen entfällt. Sie sind **keine Prognose** für den produktiven Datenträger mit 20.000 Dateien, laufendem Zufluss, anderer Brokerlatenz und weiteren Abnehmern.

**Nachträgliche Prüfung vom 16.10.2026:** Alle drei Workflows am unveränderten Folgefixcommit `0dffd40e...` sind erfolgreich:

| Workflow | Lauf | Abschluss laut GitHub |
|---|---|---|
| NetCore service WebUIs, Rust- und Browserjobs | [38485512609](https://github.com/JanHG98/netcore-tetra/actions/runs/38485512609) | `success`, aktualisiert `2026-10-13T23:41:32Z` |
| Deployment inventory and source gate | [38485512349](https://github.com/JanHG98/netcore-tetra/actions/runs/38485512349) | `success`, aktualisiert `2026-10-13T23:40:50Z` |
| OpenLab deployment and discovery | [38485512384](https://github.com/JanHG98/netcore-tetra/actions/runs/38485512384) | `success`, aktualisiert `2026-10-13T23:54:11Z` |

Die frühere Abschlussaussage „OpenLab-Imageworkflow läuft noch“ ist damit für den heutigen Repository-Status **überholt**, war aber am damaligen Nachrichtenzeitpunkt korrekt. Das spätere erfolgreiche Ende wird nicht rückwirkend als bereits damals geprüfter Befund ausgegeben.

### 8.3 Was nicht als getestet beziehungsweise im Betrieb bestätigt gilt

Nicht belegt sind die Folgefixinstallation auf dem Gateway, anhaltend fallende Queue bei tatsächlichem Quellenzufluss, dauerhafte Broker-/Netzausfall-Recovery auf dem Host, vollständige HA-Discovery nach der Sättigung, korrekte Verarbeitung bei allen MQTT-Abonnenten, reales Homematic-Schreiben oder ein Funkgerät-zu-HA-Ende-zu-Ende-Pfad. Auch die erfolgreiche Linux-CI ersetzt diese Tests nicht.

Der Archivauftrag selbst führt keine neuen Dienststarts, Hostupdates oder Aktorzugriffe aus. Die heutigen Prüfungen bestehen aus Repository-/PR-/CI-Abgleich und Dokumentprüfung; es wird kein neuer Gateway-Testlauf behauptet.

## 9. Zusätzlich geprüfter Repository-Stand am 16.10.2026

### 9.1 PRs, Commits und heutige Abweichungen

| PR | Zweck | Head-Commit | Bestätigter Merge-Commit / Mergezeit UTC |
|---|---|---|---|
| [#83](https://github.com/JanHG98/netcore-tetra/pull/83) | Start und Replay trotz voller MQTT-Outbox | `b223ecc0f626e5dd8222d3edc0e969c774b8da21` | `cfa366e41ad374943aa2156e6f3a66f2ba191cf8`, `2026-10-13T22:56:11Z` |
| [#85](https://github.com/JanHG98/netcore-tetra/pull/85) | Kleiner Quellcheckout und korrigierter Pflichtparameter `--commit` | `2f462d734ab26dab6064da0e0bb6ff4a6de4d69f` | `cbb8c87922369199a5178be7a360ec18e2543ff9`, `2026-10-13T23:08:54Z` |
| [#86](https://github.com/JanHG98/netcore-tetra/pull/86) | Betreiberbeleg zur tatsächlichen Wiederherstellung | `0a461bc0e46bbf04342f5967e9e136d0590b4da6` | `56991a99a894c48d74cf753fd063be798437d888`, `2026-10-13T23:18:20Z` |
| [#88](https://github.com/JanHG98/netcore-tetra/pull/88) | MQTT aufwecken, Outbox-Auswahl beschleunigen, Rückdruck korrekt behandeln | `0dffd40ea5687794b276b6dd4e54f2f645729746` | `8d70a7d7aca83bc3cb820962fd5fd4de31c263bf`, `2026-10-13T23:47:35Z` |

Alle vier PRs sind geschlossen und gemergt. Der letzte Chatabschluss bezeichnete PR #88 noch als bereitgestellten Folgefix. Heute ist seine **Repositoryintegration** bestätigt. Die **Installation auf dem Diensthost** ist dadurch weiterhin nicht nachgewiesen.

Beim aktuellen Branchinventar existiert der frühere Folgefixbranch nicht mehr. Historische Kommandos dürfen den Commit daher weiterhin adressieren, aber nicht voraussetzen, dass der Branchname noch aufgelöst wird. Nach Klärung der zunächst fehlenden Zielbranchangabe hat der Benutzer `main` ausdrücklich für die Archivübernahme benannt. Der gelöschte Fixbranch wird dafür nicht neu angelegt.

### 9.2 Dateiinhaltsvergleich gegen den Folgefix

Die folgenden Git-Blob-SHAs stimmen zwischen `0dffd40e...` und dem geprüften `main@8fb581b7...` überein. Damit sind die zentralen Korrekturen im heutigen Code erhalten; dies ist kein neuer Hosttest.

| Datei | Git-Blob-SHA an beiden Ständen |
|---|---|
| `system-backend/iot-gateway/src/state.rs` | `bf6452b1d02d29d38bb569bf7367b9ca66c622a0` |
| `system-backend/iot-gateway/src/mqtt.rs` | `b367d45cdf4be29fe08a4be64e7982e8d98f18e8` |
| `system-backend/iot-gateway/src/poller.rs` | `38bb23c93fdd48f4d534e4ef0d3ac3e8559834fd` |
| `system-backend/iot-gateway/src/model.rs` | `f7a1adccaac1bdec6b091977d5b5900d8764c791` |
| `system-backend/iot-gateway/web-ui/index.html` | `5c11fcb9598d7fb6eba4900d9e950d3dca5abd38` |
| `tools/test_iot_gateway_outbox.py` | `e713e149807848c30c736d90fcdf2ebecff70177` |

Die aktuelle Installationsanleitung enthält sowohl die erfolgreichen Hostschritte als auch die Folgekorrektur und ihre Abnahmegrenzen. Der strukturierte Betreiberbeleg enthält die drei Folgeproben und `live_acceptance_of_throughput_fix=false`. Die aktuelle Gesamtroadmap führt das Thema bei **Z01.4**, beschreibt die vier gesunden Quellen und steigenden Quittungen und lässt nachhaltigen Rückstauabbau, HA-Discovery und ursprüngliche Ursache ausdrücklich offen. Dieses Archiv ändert diese Roadmap nicht.

## 10. Offene Aufgaben, Wünsche und Roadmap-Kandidaten

Die Reihenfolge folgt dem konkreten Reparaturverlauf. Neue Aufgaben-IDs werden hier nicht als bereits vereinbarte Projekt-IDs ausgegeben; die vorhandene Zuordnung Z01.4 bleibt erhalten.

| Priorität im Fortsetzungsablauf | Aufgabe und Stufe | Abhängigkeit | Konkreter Abschlussnachweis |
|---|---|---|---|
| 1 | **Beschlossen/geplant:** tatsächlich installierten Commit, Binary-Hash, aktive Unit und Prozessidentität auf `10.0.1.119` erfassen. | Betreiberzugriff auf den Gateway; aktuelle Konfiguration und vorhandenen Updatebeleg lesen. | Aktueller Updaterbeleg, Binary-Hash, aktive PID/Startidentität und korrekte ExecStart-/Konfigurationszuordnung. |
| 2 | **Implementiert/getestet, Betrieb offen:** Folgefix PR #88 gezielt übernehmen, soweit noch nicht vorhanden. | Iststand aus Schritt 1; geprüfter Commit und erfolgreiche Buildvorbereitung. | Verifizierter Dateiaustausch, erfolgreicher Neustart, Liveness und separate Readiness. Kein unnötiger Rückschritt auf einen älteren Stand. |
| 3 | **Beschlossen/geplant:** Rückstau unter echtem Zufluss mehrfach messen. | Folgefix tatsächlich aktiv. | Zeitreihe aus `broker_publish_acks`, `outbox_pending`, `events_deferred`, `invalid_events`, `mqtt_last_error` und Quellenzählern über mehrere Pollzyklen; steigende Abschlüsse und nachhaltig abnehmende Queue. |
| 4 | **Offen:** Ursache der ursprünglichen Outbox-Befüllung bestimmen. | Historische Broker-/Gatewaylogs, Verbindungs-/Datenträgerbefunde und Quellenlast. | Nachvollziehbare Zuordnung zu Ausfall, fehlenden Quittungen, Eingangslast, Speicherproblem oder anderem konkret belegtem Auslöser. Der Startfix allein beantwortet diese Frage nicht. |
| 5 | **Geplant/offen:** HA-Discovery nach frei gewordenem Platz und tatsächliche Abonnentenverarbeitung abnehmen. | Freie Kapazität, Broker-/HA-Zugriff; gegebenenfalls explizite erneute Discovery. | Discovery-Lauf, sichtbar angelegte Entitäten und nachgewiesene Zustandsänderung am vorgesehenen Ziel. `mqtt_connected=true` reicht dafür nicht. |
| 6 | **Roadmap-Kandidat:** Rückdruck und Überlast im Dienstbetrieb länger beobachten. | Reale Quellenrate und Speicher-/Brokerleistung bekannt. | Belastbarer Dauer-/Recovery-Test einschließlich begrenzter Quellhistorie, nachvollziehbarer Deduplizierung und Verhalten bei Speicherfehlern. Kein bereits vereinbarter zusätzlicher Implementierungsumfang. |
| Nachgeordnet | **Neu abgeleitete Idee, im Chat nicht beschlossen:** weitergehende Optimierung der wiederholten Kapazitätszählung, Zusammenfassung wiederholter retained Quellenstatusmeldungen oder automatische Nachlieferung gescheiterter HA-Discovery. | Erst verbleibenden Flaschenhals beziehungsweise Bedarf messen. | Getrennte Spezifikation und Regressionen; keine solche Optimierung in PR #88 behaupten. |

Kleine, aber fortsetzungsrelevante Punkte: Monitor um `events_deferred` erweitern; dessen Versuchszählung in Auswertungen beachten; Liveness von Readiness und von leerer Outbox trennen; alte Fehlerzähler nicht als vollständige Formatfehlerstatistik lesen; Hosttests gegen die wirkliche Bind-Adresse und direkt ohne Proxy ausführen; die drei virtuellen Startzustände bei Replaytests mitrechnen. Ein pauschales Erhöhen des Limits oder Löschen gespeicherter Nachrichten ist keine vereinbarte Folgeaufgabe.

## 11. Quellen, Anhänge und Nachvollziehbarkeit

### 11.1 Primärbelege

- Zugänglicher Dialog mit den tatsächlich ausgeführten Bash-/Python-Blöcken und Betreiberantworten; kein auflösbarer Chatlink vorhanden.
- `Eingefügter Text(20261013-222443).txt`, vollständiger erneuter Lesezugriff am 16.10.2026; [archivierter Textinhalt](assets/2026-10-16_iot-gateway-webui-outbox/startabbruch-2026-10-13.txt).
- [Strukturierter Wiederanlaufbeleg](../services/iot-gateway/tests/evidence/wiederanlauf-2026-10-14.json) mit Build, Hashes, Receipt, Status und Folgeproben.
- PRs #83, #85, #86 und #88 sowie die oben verlinkten CI-Läufe. Linux-Echtprozesslog aus Job `115598069269` im Workflow `38485512609` am 16.10.2026 erneut gelesen.
- [Gepinnter heutiger Prüfstand](https://github.com/JanHG98/netcore-tetra/tree/8fb581b730f7a3833566c4bbb7c87875ad921e69), [Wiederanlauffix](https://github.com/JanHG98/netcore-tetra/commit/b223ecc0f626e5dd8222d3edc0e969c774b8da21), [Folgefix](https://github.com/JanHG98/netcore-tetra/commit/0dffd40ea5687794b276b6dd4e54f2f645729746).

### 11.2 Zusätzlich angebotene, hier nicht ausgewertete Normanhänge

Die Verfügbarkeit dieser Dateien wird als Quelleninventar erhalten. Es wird weder ihre vollständige Auswertung noch eine Ableitung der Gateway-Reparatur aus diesen Normen behauptet:

```text
en_3003920308v010401p.pdf
en_30039209v010701p.pdf
ts_10081201v020205p.pdf
en_3003921201v010202p.pdf
en_3003920304v010301p.pdf
en_3003921117v010102p.pdf
en_3003921114v010101p.pdf
es_20081202v020401m.pdf
es_20081201v020205p.pdf
en_300812v020101p.pdf
en_3003921101v010201p.pdf
en_3003921006v010401p.pdf
en_3003921018v010301p.pdf
en_3003921216v010400a.pdf
en_30039201v010601p.pdf
ets_30039214e01v.pdf
en_30039207v030501p.pdf
en_30039401v030301p.pdf
en_3003920313v010201p.pdf
en_30039502v010303p.pdf
en_3003920303v010301p.pdf
en_30039205v020701p.pdf
en_3003920315v010500a.pdf
en_30039202v030801p.pdf
ETSI.pdf
```

### 11.3 Datenschutz und Archivabgrenzung

Übernommen werden die für Fortsetzung und Nachweis benötigten Dienstnamen, privaten Netzadressen, Quellpfade, Commit-IDs, Prüfsummen und bereinigten technischen Befunde. Passwörter, Tokens, private Schlüssel oder sonstige Zugangsdaten gehören nicht in diese Dokumentation. Der Original-Logtext enthält keine erkannten Zugangsdaten. Zugangsdaten aus anderen Projektchats und dortige Bild-/Dateianhänge sind nicht Bestandteil dieses Archivs.

Der tatsächliche Commit der Archivübernahme ist nach dem Push über die Git-Historie dieser Datei und den Archivindex nachzuweisen. Die oben genannten Commits sind geprüfte Quell-/Fixstände und werden nicht als bereits erfolgter Archivcommit ausgegeben. Der Chat wird durch diesen Auftrag nicht geschlossen oder automatisch archiviert.
