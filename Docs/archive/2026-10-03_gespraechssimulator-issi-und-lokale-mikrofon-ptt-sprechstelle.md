# Brainstorming: Gesprächssimulator mit wechselnden ISSIs und lokale Mikrofon-/PTT-Sprechstelle

## 1. Rahmen und Quellenstand

| Merkmal | Wert |
|---|---|
| Projekt | NetCore-Tetra |
| Historische Fachgespräche | 2026-07-20; anschließend Dokumentation am 2026-10-03 |
| Notizstand | **2026-10-03** (Europe/Berlin) |
| Geprüftes Repository | [JanHG98/netcore-tetra](https://github.com/JanHG98/netcore-tetra) |
| Geprüfter Branch | `Archiving` |
| Geprüfter Ausgangscommit | [57570d905244d5a166c948e54e129b3daf5bdb83](https://github.com/JanHG98/netcore-tetra/commit/57570d905244d5a166c948e54e129b3daf5bdb83) |
| Commitbetreff des Ausgangsstands | `docs(archive): document WERMA rack signalling and BPI-R4 Pro architecture` |
| Ablage | `Docs/archive/2026-10-03_gespraechssimulator-issi-und-lokale-mikrofon-ptt-sprechstelle.md` |

Der ursprüngliche Repository-Link war [JanHG98/flowstation](https://github.com/JanHG98/flowstation/). Er leitete bei der aktuellen Prüfung auf `JanHG98/netcore-tetra` weiter. Historische Bezeichnungen wie FlowStation, Bluestation, TBS und geprüfte NetCore-Pfade werden deshalb nicht als getrennte Implementierungen behandelt. Ein konkreter historischer Codecommit vom 2026-07-20 ist im ursprünglichen Entwicklungsstand nicht angegeben.

Grundlage sind zwei Machbarkeitsfragen und die zugehörigen Architekturentwürfe. Die Entwürfe wurden am Repository-Stand vom 03.10.2026 abgeglichen.

**Offene Nachweise:** ISSI-Liste und Audiodateien sind nicht als Artefakte verfügbar; Inhalte, Formate, Längen und konkrete Identitäten wurden nicht geprüft. RF-Mitschnitte, Laufzeitlogs, Messungen, Buildberichte, Implementierungs-PRs und Deployments für die Erweiterungen fehlen. Frühere Dual-Carrier-Probleme oder ein zweiter Pi für WERMA/Relais sind lediglich angrenzende Kontextideen.

Die Repositoryprüfung ist eine statische Prüfung des ausgecheckten Branches, keine Bestätigung einer laufenden Basisstation. Keine Zugangsdaten wurden in dieses Dokument übernommen.

## 2. Ergebnis und Statusmodell

**Beide Vorhaben sind Konzepte.** Der geprüfte Quellcode enthält wichtige wiederverwendbare Bausteine: netzinitiierte Gruppenrufe, einen Sprecherwechsel in CMCE, einen Datei-AudioPlayer sowie einen gemeinsamen TETRA-Sprachcodec und einen Asterisk-Medienpfad. Ein ausführbarer Gesprächssimulator und eine lokale USB-Mikrofon-/PTT-Sprechstelle wurden im geprüften Stand nicht gefunden.

Die Suche umfasste insbesondere `crates/`, `bins/`, `tests/` und `system-backend/`, außerdem eine repositoryweite Suche außerhalb der Archivtexte nach `conversation_simulator`, `ConversationScenario`, `play_scenario`, `OperatorRadio`, `net_operator_radio`, `ptt_backend` und `abort_on_real_transmission`. Sie ergab keine passende Implementierung. Zusätzlich wurden Entity-Aufzählung, Konfiguration, Startregistrierung und Audio-API gelesen. Diese Aussage gilt für den geprüften Branch und die beschriebenen Funktionen, nicht für unbekannte andere Branches oder externe lokale Änderungen.

| Status | Bedeutung in diesem Dokument |
|---|---|
| **Idee** | Im Gespräch erwogene Funktion oder Alternative ohne verbindliche Umsetzung. |
| **Beschlossen/geplant** | Eine ausdrücklich beschlossene Anforderung wird als solche benannt. Entwicklungsvorschläge gelten als Entwurf, solange keine verbindliche Festlegung belegt ist. |
| **Implementiert** | Der relevante Code ist im geprüften Commit vorhanden. Das sagt nichts über einen erfolgreichen Build oder Betrieb aus. |
| **Getestet** | Nur ein tatsächlich belegter Testlauf würde diesen Status begründen. Vorhandener Testcode wird gesondert als „Test vorhanden, hier nicht ausgeführt“ bezeichnet. |
| **Im Betrieb bestätigt** | Erfordert eine konkrete Betriebsbeobachtung oder einen passenden Nachweis. Für die beiden neuen Funktionen liegt keiner vor. |

| Gegenstand | Historischer Stand | Aktueller Befund | Test-/Betriebsnachweis |
|---|---|---|---|
| Gespräche mit wechselnden ISSIs und passenden Clips | Projektidee; Machbarkeit bejaht | Kein Szenario-Modus gefunden | Keiner |
| AudioPlayer als Grundlage | Erweiterungsansatz | Implementiert; eine globale Quell-ISSI, ein aktiver Auftrag | Testcode vorhanden; kein aktueller Testlauf bei der Quellenprüfung |
| CMCE-Sprecherwechsel bei gleicher GSSI | Als vorhanden beschrieben | Implementiert, einschließlich neuer `NetworkCallReady`-Rückmeldung | Komponenten-Test vorhanden; hier nicht ausgeführt |
| USB-Mikrofon und PTT am Pi | Machbarkeitsfrage bei fehlenden freien GPIO | Kein `OperatorRadio`-Modul, keine zugehörige Konfiguration/Entity gefunden | Keiner |
| Streaming-Sprachcodec und Asterisk-Transcoding | Als Echtzeit-Grundlage genannt | Implementiert; inzwischen eigener Codec-Worker sichtbar | Codec-/Worker-Tests vorhanden; hier nicht ausgeführt |
| Vollständige Dual-Carrier-Eignung des AudioPlayers | Historisch zu positiv dargestellt | Statische Unstimmigkeit zwischen logischem Slot und physischem Tick, siehe Abschnitt 5.6 | Kein RF-Test |
| TBS läuft auf einem Pi, keine freien GPIO | Ausdrückliche Nutzerangabe | Als Ausgangsbedingung übernommen | Keine aktuelle Hardwareinspektion; kein Nachweis des Betriebs neuer Funktionen |

## 3. Ziel, Ausgangslage und verbindliche Anforderungen

Machbarkeitsfrage: Kann die Basisstation vollständige Gespräche netzseitig simulieren? Jeder Beitrag soll zu einer passenden Audiodatei und einer anderen Quell-ISSI gehören. Eine Liste möglicher ISSIs wurde als vorhanden beschrieben. „Geheimer Modus“ bezeichnete den gewünschten Zugang bzw. die Sichtbarkeit der Funktion; eine konkrete Freischaltmethode wurde nicht festgelegt.

Die zweite Frage betrifft eine lokale Sprechstelle: Mikrofon und PTT direkt an der auf einem Raspberry Pi laufenden TBS, um aktiv auf einer Gruppe sprechen zu können. **Es sind keine GPIOs frei.** Eine Lösung muss deshalb ohne zusätzliche Pi-GPIO-Belegung auskommen.

Verbindlich aus dem Nutzerauftrag sind:

- die beiden Machbarkeitsziele und die GPIO-Randbedingung;
- Abgleich der Architekturideen mit den vorhandenen Codepfaden;
- ausschließlich Archivdatei und Index unter `Docs/archive/` auf `Archiving` ändern, bestehende Inhalte bewahren, committen, ohne Force-Push veröffentlichen und beide Dateien anschließend im Branch prüfen;
- Datum 2026-10-03 und keine Übernahme von Zugangsdaten.

Nicht verbindlich beschlossen wurden eine bestimmte USB-Hardware, finale ISSI/GSSI-Werte, API-Routen, Datenformate, Betriebsgrenzen, Prioritäten, ein Termin oder die tatsächliche Implementierung. Die folgenden Architektur- und MVP-Details bewahren die fachlichen Entwicklungsvorschläge für eine spätere Entscheidung.

## 4. Historischer Entwurf: Gesprächssimulator

### 4.1 Netzseitige Gespräche statt virtueller Mobilstationen

Der damalige Vorschlag war ein Conversation-/Scenario-Modus des vorhandenen AudioPlayers. Vorbereitete Sprachbeiträge sollen über CMCE und UMAC/LMAC als echte Gruppenrufaussendung übertragen werden. Die Quell-ISSI wird je Beitrag in der Signalisierung geändert.

Das ist keine vollständige Emulation von Mobilstationen: Die simulierten ISSIs führen dadurch keine MM-Registrierung aus und erzeugen keinen echten Uplink. Eigene RSSI-Werte, GPS-/Standortmeldungen, Zellwechsel, U-TX-Demand oder SDS-Verhalten entstehen nicht automatisch. Ein virtueller MS-Stack wäre ein eigenständiges, wesentlich größeres Vorhaben und wurde für dieses Ziel nicht empfohlen.

Die erwartete Anzeige der ISSI auf einem Endgerät und eine Namensauflösung aus dessen Telefonbuch waren Funktionsziele bzw. technische Erwartungen. In den Arbeitsnotizen wurde kein Endgerät gezeigt, das dieses Verhalten tatsächlich vorführte.

### 4.2 Szenariodateien und Datenmodell

Als Beispielverzeichnis wurde vorgeschlagen:

```text
/mnt/nfs-share/Simulationen/
└── Einsatzaufnahme/
    ├── scenario.json
    ├── 01_fahrzeug12.wav
    ├── 02_leitstelle.wav
    ├── 03_fahrzeug12.wav
    └── 04_leitstelle.wav
```

Das folgende historische Beispiel ist **kein am 03.10.2026 implementiertes Dateiformat**. Die Zahlen sind Beispielkennungen und keine bestätigte Zuteilung:

```json
{
  "name": "Fahrzeug 12 am Einsatzort",
  "target_gssi": 1001,
  "priority": 3,
  "steps": [
    {"source_issi": 4010112, "file": "01_fahrzeug12.wav", "pause_after_ms": 700},
    {"source_issi": 4010001, "file": "02_leitstelle.wav", "pause_after_ms": 450},
    {"source_issi": 4010112, "file": "03_fahrzeug12.wav", "pause_after_ms": 300},
    {"source_issi": 4010001, "file": "04_leitstelle.wav", "pause_after_ms": 0}
  ]
}
```

Die zuerst erzählte Dialogskizze verwendete für das Fahrzeug abweichend `4010101`; das ausführlichere JSON-Beispiel verwendete `4010112`. Das sind zwei Illustrationen, keine spätere bestätigte Korrektur eines Nummernplans.

Weitere Ideen waren zufällige Vorpausen, beispielsweise `pause_before_ms: [300, 900]`, Auswahl aus `random_file`, Wiederholungen über `repeat: 2` und eine Auswahlwahrscheinlichkeit `probability: 0.7`. Semantik, Seed, Validierung und Zusammenspiel dieser Felder blieben offen.

### 4.3 Vorbereiten, starten, Sprecher wechseln, beenden

Alle Clips sollten vor dem ersten Ruf vollständig decodiert und in TETRA-Sprachblöcke codiert werden. Damit würden Decoder-, Netzfreigabe- oder Speichermedienverzögerungen nicht erst mitten im Gespräch auftreten. Ein fehlerhafter Clip soll den Start verhindern, statt einen teilweise vorbereiteten Ruf zu beginnen.

Vorgeschlagener Ablauf:

1. Szenario, Dateien, Quellen und Zielgruppe prüfen; alle Beiträge vorbereiten.
2. `NetworkCallStart` mit erster `source_issi`, `dest_gssi` und Priorität senden.
3. `NetworkCallReady` abwarten und den zugewiesenen Bearer übernehmen.
4. Vorbereitete Blöcke TDMA-synchron senden.
5. Mit `NetworkCallEnd` den Sprecherbeitrag beenden, dann die geplante Pause abwarten.
6. Für dieselbe GSSI erneut `NetworkCallStart` mit der nächsten Quell-ISSI senden.
7. Nach dem letzten Beitrag den Ruf kontrolliert auslaufen lassen.

Die damalige Annahme war, dass bei Pausen unterhalb der Hangtime derselbe Gruppenruf und Verkehrskanal erhalten bleiben. Der CMCE-Zustandsautomat bietet dafür eine Grundlage; die geprüfte AudioPlayer-Schnittstelle setzt diesen Ablauf jedoch nicht als zusammenhängendes Szenario um. Freigabefrist, UUID-Zuordnung, tatsächlicher Floor-Besitz, Rufverlust und verspätete Rückmeldungen müssen ausdrücklich behandelt werden.

Als neuer Zustand wurde `ActiveConversation` mit Szenario-ID, Ziel-GSSI, Segmentliste, aktuellem Segment und Sprecher, Ruf-UUID, Ruf-ID, Timeslot, Phase und Pausenende skizziert. Genannte Phasen: `Preparing`, `StartingTurn`, `WaitingForCallReady`, `Playing`, `InterTurnPause`, `Finishing`, `Stopped`, `Failed`.

Vorgeschlagene Typen und Schnittstellen:

- `source_issi` je Auftrag in `AudioPlayerCommand::Play`, `PreparedAudio` und `AudioPlayerStatus`;
- `ConversationScenario`, `ConversationSegment`, `PreparedConversation`, `ConversationState`;
- `play_media_as(...)`, `play_scenario(...)`, `stop_scenario(...)`, `list_scenarios(...)`;
- Ablaufsteuerung in `net_audio_player/entity.rs`, Validierung und öffentliche Methoden in `service.rs`, Datentypen in `types.rs`.

Diese Bezeichner sind Entwürfe. Der aktuelle Befehl enthält weiterhin keine individuelle Quell-ISSI.

### 4.4 Grenzen, Bedienung und Schutzmechanismen

Historischer Konfigurationsentwurf, **vom geprüften Parser nicht als diese Funktion implementiert**:

```toml
[conversation_simulator]
enabled = false
scenario_directory = "/mnt/nfs-share/Simulationen"
max_segments = 50
max_total_duration_seconds = 600
abort_on_real_transmission = true
allowed_target_gssis = [1001, 1002]
allowed_source_issis = [4010001, 4010112, 4010113, 4010114]
```

Vorgeschlagen wurden 24-Bit-Validierung der ISSIs, Quell- und Ziel-Allowlists, auf das Szenarioverzeichnis begrenzte Dateizugriffe, maximale Segmentzahl und Gesamtdauer. Die erwähnte reale ISSI-Liste sollte Grundlage der Allowlist werden; sie liegt der Quellenprüfung vom 03.10.2026 nicht vor.

Als verborgener Einstieg wurden `/system/lab/conversations`, eine Tastenkombination, wiederholte Logoklicks oder ausschließlich die Control-Room-API erwogen. Keine Variante wurde ausgewählt oder implementiert. Die damalige Empfehlung verlangte zusätzlich API-Authentifizierung, standardmäßige Deaktivierung, sichtbaren Simulationsstatus für Administratoren, globalen Stop, Fehlerabbruch und eindeutige Kennzeichnung `source=simulation`. Versteckte Navigation sollte nicht die Zugriffskontrolle ersetzen.

Ein optionaler Abbruch bei realem Funkverkehr wurde vorgeschlagen. Wie ein echter PTT-Wunsch während einer netzseitigen Aussendung zuverlässig erkannt und priorisiert wird, wurde nicht spezifiziert. Das braucht ein konkretes CMCE-/Floor-Ereignis und darf nicht allein aus fehlendem Uplink-Audio abgeleitet werden.

Eine Simulation belegt reale Funkkapazität. Für den ersten Ausbau wurde maximal ein simuliertes Gespräch gleichzeitig empfohlen. Das wurde nicht als Reservierung aller sonstigen Rufarten oder als fertige globale Konfliktsteuerung beschrieben.

## 5. Gegenprüfung am aktuellen Repository: Simulator und AudioPlayer

### 5.1 Bestätigte Grundbausteine

In `net_audio_player/entity.rs` erstellt der Player einen netzinitiierten Gruppenruf mit Quell-ISSI aus seiner Konfiguration, Ziel aus dem Auftrag und Auftragpriorität. `AudioPlayerCommand::Play` und `PreparedAudio` besitzen keine Quell-ISSI. `AudioPlayerStatus` meldet ebenfalls keine Quell-ISSI je Auftrag. Die historische Einschränkung einer globalen Absenderkennung ist damit weiterhin richtig. [R1, R2]

`AudioPlayerHandle::play_resolved` akzeptiert neue Aufträge nur in `Idle` oder `Failed`. Andernfalls lautet der Fehler `an audio transmission is already active`. Zielkennungen werden auf 1 bis `0x00ff_ffff`, Prioritäten auf 0 bis 15 begrenzt. Die vorhandene Busy-Sperre betrifft den AudioPlayer-Auftrag; sie beweist keine umfassende Nichtunterbrechungsregel gegenüber fremden CMCE-Gruppenrufen. [R3]

Die Dateivorbereitung läuft in einem Thread `audio-player-prepare`. `media.rs` decodiert WAV bzw. MP3, wandelt auf 8-kHz-Mono-PCM um, ergänzt den letzten unvollständigen Block mit Nullen und codiert vor Rufbeginn alle Blöcke. Externe Dateien können zuvor lokal zwischengespeichert werden. Native WAV-Verarbeitung, lineares Resampling und ein ffmpeg-Fallback sind vorhanden. Der AudioPlayer enthält außerdem Recording-, TTS- und Media-Library-Anbindungen; diese stellen noch keinen Szenario-Player dar. [R2, R3, R4]

### 5.2 Sprecherwechsel, Signalisierung und Listener

`fsm_on_network_call_start` sucht einen bestehenden Ruf nach `dest_gssi`. Bei einem Treffer wird `fsm_group_on_network_call_start` aufgerufen; der bekannte Logtext `network call speaker change` existiert weiterhin. Die erlaubten Ausgangszustände sind `Transmitting` und `NoActiveSpeaker`. [R6, R7]

Der Wechsel setzt den Floor auf die neue Quell-ISSI, aktualisiert `brew_uuid` und gegebenenfalls den Netzursprung, sendet `D-TX GRANTED` über FACCH, meldet `RemoteFloorGranted` und antwortet der anfragenden Entity mit `NetworkCallReady` einschließlich `brew_uuid`, `call_id`, `ts` und `usage`. Das ist eine Implementierungsgrundlage für Sprecherwechsel, kein Nachweis einer fertigen Szenariosteuerung. [R7]

Bei einem neuen Gruppenruf steht die Quelle in `DSetup.calling_party_address_ssi`. Beim Wechsel verwendet `DTxGranted` das Feld `transmitting_party_address_ssi`. Damit ist die Identität in der Signalisierung belegt. Eine konkrete Endgeräteanzeige oder Telefonbuch-Namensauflösung muss trotzdem an den jeweiligen Geräten geprüft werden. [R6, R8]

Für den Start wird `has_listener(dest_gssi)` geprüft. Ohne Listener wird der neue Start beendet und gegebenenfalls bestehender ungehörter Gruppenverkehr entfernt. Die historische Kurzform „mindestens ein echter Listener“ ist zu präzisieren: Am 03.10.2026 gilt auch ein kürzlich deaffiliierter Listener innerhalb der implementierten Grace-Phase als ausreichend. Ein zum Prüfzeitpunkt tatsächlich hörendes Gerät wurde nicht nachgewiesen. [R6, R8]

Ein neuer Ruf reserviert einen P2MP-Circuit mit `TimeslotOwner::Cmce`, öffnet den UMAC-Pfad und erzeugt `GroupCallStarted`-Telemetrie. Der Simulator wäre somit echte Belastung des Sendepfads. Die aktuelle Telemetrie ordnet `AudioPlayer` über den Defaultfall als `local` ein; ein eigener Simulationsquellentyp ist nicht vorhanden. [R6, R20]

### 5.3 Hangtime, Freigabefrist und UUID-Lebenszyklus

Beim Ende eines sendenden Netzsprechers geht CMCE in Hangtime, setzt `brew_uuid=None`, signalisiert `D-TX CEASED` und gibt den Floor frei. `NetworkCallEnd` wird eingangs anhand der aktiven `brew_uuid` einem Ruf zugeordnet. Ein identisches zweites End-Kommando findet deshalb nicht automatisch denselben Ruf wieder. Ein Szenario darf diesen Ablauf nicht als einfachen idempotenten „Stop/Start“-Schalter behandeln. [R7, R9]

Der geprüfte AudioPlayer besitzt `Finishing` und eine konfigurierbare `group_release_guard_seconds`. Im Standard sind es sechs Sekunden; der Konfigurationsparser erlaubt fünf bis dreißig. Der Player bleibt damit nach einem beendeten Gruppenbeitrag zunächst beschäftigt, bis die passende Beendigung oder sein Abschlusszeitlimit verarbeitet wird. Der Kommentar nennt ausdrücklich die Vermeidung eines ungewollten UUID-Wechsels innerhalb eines noch bestehenden Gruppenrufs. [R2, R5]

**Folge für die Fortsetzung:** Vier unabhängige `/api/audio/play`-Aufrufe mit Pausen von 700/450/300 ms bilden den historischen Entwurf nicht ab. Ein Szenario muss die Beiträge innerhalb einer eigenen zusammenhängenden Sitzung koordinieren und die Guard-Logik für vollständige Aufträge von absichtlichen Sprecherwechseln unterscheiden. Die vorhandene Freigabefrist einfach zu entfernen wäre keine aus den Arbeitsnotizen abgeleitete Lösung.

CMCE verwendet `cell.hangtime_secs`; dessen Parserstandard ist fünf Sekunden, begrenzt auf 0 bis 300. Die Ablaufprüfung enthält außerdem eine Ausnahme für Notrufprioritäten. Eine ausfallsichere Szenariobegrenzung darf daher nicht allein auf normale Hangtime vertrauen. [R10, R24]

### 5.4 Zusätzliche Start- und Audiolatenzen

Nach vollständiger Vorbereitung wartet der geprüfte Gruppen-Player zunächst `GROUP_CALL_PREPARE_SETTLE = 1000 ms`. Der Start wird zusätzlich auf Timeslot 4 außerhalb der Frames 1, 17 und 18 gelegt. Die vorbereiteten Daten enthalten standardmäßig zwölf stille 60-ms-Blöcke vor dem Clip und drei danach, also 720 ms Vorlauf und 180 ms Nachlauf. [R2, R4, R5]

Diese Wartezeiten sind von einer im Szenario konfigurierten Gesprächspause zu unterscheiden. Würde jede Äußerung als neuer normaler Abspielauftrag behandelt, entstünden zusätzliche Verzögerungen. Bei einer Live-Sprechstelle müssen Freigabeton, Vorpuffer und tatsächliche Sendelatenz gesondert abgestimmt werden.

### 5.5 Pfadvalidierung: historischen Absolutanspruch einschränken

Bestätigt sind bereinigte relative Pfade, Kanonisierung, eine anschließende Prüfung auf Zugehörigkeit zum Medienverzeichnis, erlaubte WAV-/MP3-Endungen sowie Größen- und Dauergrenzen. Die Dateiliste überspringt Symlink-Einträge. [R3, R4]

Die historische Formulierung „keine Symlinks“ ist jedoch als generelle Aussage zu weitgehend: `resolve_media_file` kanonisiert den Zielpfad und prüft dessen Lage, verbietet aber in diesem Ablauf nicht explizit jede Symlink-Komponente. Ein innerhalb des erlaubten Roots aufgelöster Link ist damit nicht durch diese Prüfung grundsätzlich ausgeschlossen. Das ist keine hier nachgewiesene Ausbruchsmöglichkeit, sondern eine Präzisierung der vorhandenen Validierung. Für Szenarien muss die gewünschte Symlink-Regel ausdrücklich festgelegt werden.

### 5.6 Dual-Carrier: zwei historische Aussagen korrigieren

Der aktuelle Allocator hat sechs mögliche Traffic-Bearer: logisch 2/3/4 für Hauptträger-TS2/3/4 und logisch 5/6/7 für Sekundärträger-TS2/3/4. Sekundär-TS1 ist Control/Guard vorbehalten. Die historische Formulierung „TS1–8“ bzw. „TS5–8“ beschreibt **nicht** den geprüften Traffic-Slotplan. Ein älterer Kommentar im UMAC-TMD-Eingang erwähnt noch 5–8; maßgeblich sind die implementierten Mappingfunktionen und der Allocator. [R11, R12]

Beim AudioPlayer existiert zwar `carrier_for_logical_ts`, aber sein `playout` vergleicht den von CMCE übernommenen logischen Timeslot direkt mit `self.dltime.t`. Die Routerzeit läuft über `TdmaTime::add_timeslots` mit physischem `t=1..4`; `media_ready` übernimmt den Slot ohne Umrechnung. Für einen logisch zugewiesenen Slot 5, 6 oder 7 ist diese Gleichheit im normalen Routertakt nicht erfüllbar. [R2, R13, R14]

**Statischer Befund:** Der aktuelle AudioPlayer zeigt damit einen konkreten Verdacht auf ausbleibendes Playout auf dem zweiten Träger. Die frühere Annahme, sein TDMA-Versand könne praktisch unverändert übernommen werden und sei bereits vollständig korrekt, ist nicht ausreichend belegt. Vor Wiederverwendung sind physischer Tick und logischer Bearer sauber zu trennen und mit einem echten Secondary-Bearer zu testen. In diesem Auftrag wurde der Fehler weder durch einen RF-Test reproduziert noch behoben.

Asterisk setzt im erzeugten `TmdCircuitDataReq` weiterhin `carrier_num = main_carrier`. **Daraus folgt im geprüften Gesamtpfad aber nicht automatisch ein Sendefehler:** `UmacBs::rx_tmd_prim` berechnet den Carrier und den physischen Slot selbst aus `prim.ts`, statt dieses Carrierfeld für die Auswahl zu verwenden. Der historische Schluss „Asterisk muss deswegen auf TS5–8 falsch senden“ ist somit für den aktuellen Stand zu korrigieren. Das redundante Carrierfeld bleibt ein Konsistenzpunkt; eine Live-Abnahme beider Träger bleibt offen. [R11, R17]

## 6. Historischer Entwurf: lokale Mikrofon-/PTT-Sprechstelle

### 6.1 Architektur und Abgrenzung zum Dateiplayer

Empfohlen wurde eine eigene `OperatorRadioEntity` mit separater Zustandsmaschine. Die Begründung: Liveaufnahme, Geräteereignisse, kleine begrenzte Puffer, Rufaufbau, Freigabetöne und sichere Freigabe bei losgelassenem PTT benötigen einen anderen Lebenszyklus als die vollständig vorbereitete Dateiwiedergabe.

Vorgeschlagene neue Dateien, **im geprüften Stand nicht vorhanden**:

```text
crates/tetra-entities/src/net_operator_radio/
├── entity.rs
├── audio.rs
├── ptt.rs
├── service.rs
└── types.rs
```

Dazu sollte `TetraEntity::OperatorRadio` kommen. Ein solches Modul erfordert zusätzlich Registrierung im Hauptprogramm und Router, Konfiguration, Status-/Bedienanbindung und eine klare Zulassungsregel in CMCE. Am 03.10.2026 nimmt CMCE nur bestimmte Entities vom Brew-Inbound-Prädikat aus, darunter `AudioPlayer` und `Cmce`; eine neue Entity darf daher nicht allein durch das Senden von `NetworkCallStart` als fertig integriert gelten. [R6, R21, R22]

Vorgeschlagener Signalweg:

```text
USB-Audio → Aufnahme-/Resampling-/Codec-Worker → begrenzte TMD-Queue
                                                     ↓
USB-HID oder USB-Serial → PTT-Ereignisse → OperatorRadio-Zustandsmaschine
                                                     ↓
                                      CMCE: Ruf und Floor
                                                     ↓
                                      UMAC/LMAC: Funkaussendung
```

ALSA-Aufnahme darf den synchronen MessageRouter nicht blockieren. Ein erster Ansatz sah Encoding in der Entity vor; später folgte die vollständig ausgelagerte Aufnahme-/Resampling-/Encoding-Variante. Als geprüfter Planungshinweis ist die Worker-Variante passend zur bereits vorhandenen Asterisk-Auslagerung; die konkrete Ausführung bleibt zu implementieren und zu messen. [R14, R16]

### 6.2 PTT-Ablauf, Vorpuffer und Ende

Beim Drücken sollte die Aufnahme sofort beginnen, während ein kleiner Puffer die Rufaufbauzeit überbrückt. Parallel würde `NetworkCallStart` mit Operator-ISSI und gewählter GSSI gesendet. Nach `NetworkCallReady` sollte ein lokaler Freigabeton folgen und der gepufferte bzw. aktuelle Ton ausgesendet werden.

Genannt wurden 500–1000 ms Vorpuffer, im Konfigurationsbeispiel 750 ms. Das ist ein Entwurfswert, keine gemessene erforderliche Zeit oder zugesicherte Latenz. Für die Fortsetzung müssen Pufferobergrenze, Umgang mit zu frühem Sprechen, Verhalten bei verspäteter Freigabe und maximal zulässige Audioalterung festgelegt werden.

Beim Loslassen sollten ein letzter Block kontrolliert abgeschlossen, ein kurzer stiller Nachlauf gesendet und anschließend `NetworkCallEnd` ausgelöst werden. Ein vor `NetworkCallReady` losgelassener PTT, verspätete Ready-Meldungen und zwischenzeitlich verlorene Geräte wurden nicht vollständig ausprogrammiert. Sie gehören zur offenen Zustands- und Testarbeit.

Genannte Zustände: `Idle`, `PttRequested`, `WaitingForCallReady`, `Transmitting`, `Releasing`, `Busy`, `Failed`. Für eine `OperatorSession` wurden UUID, Quell-ISSI, Ziel-GSSI, Priorität, optionale Ruf-ID/Timeslot sowie `ptt_pressed` und `release_requested` vorgeschlagen.

### 6.3 Audiohardware ohne freie GPIO

Vorgeschlagen wurden USB-Headset, USB-Mikrofon, USB-Soundkarte mit Mikrofoneingang oder USB-Audiointerface. `hw:1,0` war ein ALSA-Beispiel, keine ermittelte Geräteadresse. Ebenso war 48 kHz, Mono, S16 als Aufnahmeformat mit Umwandlung auf 8 kHz, Mono, S16 ein Beispiel; die unterstützten Formate müssen am tatsächlichen Gerät geprüft werden.

Bei der Hardwareauswahl sind nicht kontrollierbare automatische Verstärkung, Noise Gate, Echoverarbeitung und Latenz zu prüfen. Es wurde kein konkretes Modell ausgewählt, gekauft oder getestet.

Der gemeinsame Codec erwartet 8-kHz-PCM: 240 Samples pro 30-ms-Codecframe, zwei Frames bzw. 480 Samples pro 60-ms-TMD-Sprachblock. Zwei mal 137 Sprachbits ergeben 274 Bits, gepackt in 35 Bytes. Das sind die Audio-/TMD-Parameter, keine Angabe der vollständigen physikalischen Funkburstgröße. [R15]

### 6.4 PTT-Varianten und Nebenideen

| Variante | Historischer Vorschlag | Offene Anforderungen |
|---|---|---|
| USB-HID | Bevorzugte lokale Lösung; beispielsweise `KEY_F13` DOWN/UP über evdev | Gerät identifizieren, Zugriff des Dienstes, Press/Release, Wiederanmeldung und Verbindungsverlust |
| USB-Mikrocontroller | RP2040 oder Arduino Pro Micro als HID-Encoder | Firmware, Entprellung, LED-/Schaltersignale, verlässliche Ausfallmeldung |
| USB-Serial | Nachrichten `PTT_DOWN`, `PTT_UP`, `CHANNEL 1001`; Beispiel 115200 Baud | Nachrichtenrahmen, Heartbeat/Watchdog, Startzustand, Reconnect und Trennungsabbruch |
| Netzwerk-PTT | I/O-Pi oder ESP32 über authentifizierte Verbindung zur TBS | Protokoll, Authentifizierung, Zeitgrenzen und Geräteausfall; kein Port festgelegt |
| Dashboard-PTT | Gedrückthalten-Taste für Tests/Fernbedienung | Fokusverlust, verlorenes Release-Ereignis, Verbindungsabbruch und Browser-Audioberechtigungen |

HID nutzt keinen freien GPIO am Pi. Ein externer Mikrocontroller kann eigene Pins für den Taster verwenden; das widerspricht der Pi-Randbedingung nicht. Ein Pfad unter `/dev/input/by-id/` wurde für eine stabile Gerätezuordnung vorgeschlagen, gegenüber dem nur beispielhaften `/dev/input/eventX`.

Für den Browser wurde ein Keepalive etwa alle 250 ms genannt. Der tatsächliche Lease-Ablaufwert wurde nicht festgelegt. Ein Keepalive-Intervall allein definiert noch nicht, nach welcher Ausfallzeit der Sender stoppen muss.

Weitere Hardwareideen: RX-, TX- und Besetzt-LED, Gruppen-/Kanalwahlschalter, optional Lautstärkeregler, Not-Aus und Handapparat-Erkennung. Die erwähnte Kopplung an einen zweiten WERMA-/Relais-Pi war eine Alternative, keine bestätigte Gerätearchitektur dieses Vorhabens.

### 6.5 Besetzte Gruppe, Priorität und Ausfallsicherheit

Ein naiver `NetworkCallStart` kann eine bereits sprechende Partei ersetzen. Das ist durch die geprüfte CMCE-Transition aus `Transmitting` weiterhin technisch relevant. Der Vorschlag lautete:

```toml
respect_active_floor = true
allow_preemption = false
```

Bei belegtem Floor sollte der normale PTT nur einen Busy-Ton auslösen. Eine spätere gesonderte Leitstellenübernahme mit `allow_preemption=true` und `preemption_priority=15` war eine **optionale Idee**, keine implementierte Berechtigungs- oder Verdrängungsregel. Ein Prioritätswert allein ersetzt keine Floor-Policy; Wechsel, Rechteprüfung und konkurrierende Ereignisse müssen atomar abgestimmt werden.

Weitere vorgeschlagene Regeln: höchstens 60 oder 120 Sekunden Sendedauer, Ende bei Trennung von PTT- oder Audiogerät, verlässliche Freigabe beim Loslassen, Stille bei Encoder-Unterlauf, Verwerfen zu alter Samples bei Überlauf, ISSI/GSSI-Allowlists, sichtbarer TX-Status sowie lokale Freigabe-/Besetzttöne. Keine dieser Regeln ist als `OperatorRadio`-Funktion bereits implementiert oder getestet.

### 6.6 Historische Beispielkonfiguration

**Nur Entwurf; nicht in die geprüfte produktive Konfiguration übernehmen und als funktionierendes Feature erwarten.**

```toml
[operator_radio]
enabled = true
audio_device = "hw:1,0"
playback_device = "hw:1,0"
source_issi = 4010001
default_gssi = 1001
priority = 5
ptt_backend = "hid"
ptt_device = "/dev/input/by-id/usb-NetCore_PTT-event-kbd"
ptt_key = "KEY_F13"
prebuffer_ms = 750
tail_silence_ms = 180
max_transmit_seconds = 120
respect_active_floor = true
allow_preemption = false
allowed_gssis = [1001, 1002, 1003]
```

Für die alternative serielle Variante wurden `ptt_backend="serial"`, `ptt_device="/dev/serial/by-id/usb-NetCore_PTT"` und `baud_rate=115200` genannt. Gerätepfade, Kennungen und Werte waren illustrative Platzhalter. Die Beispielaktivierung `enabled=true` ist keine Aussage über eine laufende Installation.

### 6.7 Empfang und empfohlener MVP

Lokales Mithören sollte später TETRA-Uplink über den Decoder als PCM an ALSA-Playback ausgeben. Dafür wurden Headset oder Handapparat empfohlen; bei Lautsprecherbetrieb sollte die Wiedergabe während PTT stummgeschaltet oder abgesenkt werden. Echo-/Rückkopplungsverhalten wurde nicht gemessen.

Als erster Ausbau vorgeschlagen: USB-Audio plus USB-HID, eine feste Quell-ISSI, eine auswählbare GSSI, ausschließlich Gruppenruf, Halten/Loslassen, Freigabe-/Busy-Ton, keine Verdrängung, maximal eine lokale Aussendung und Dashboardstatus. Der historische Wunsch nach „TS1–8“ ist bei der Umsetzung auf den geprüften Bearerplan aus Abschnitt 5.6 zu übertragen.

Nachgelagerte Ideen: Gruppenwahlschalter, RX-Monitoring, Handapparat, mehrere Operator-ISSIs, Remote-PTT, bewusste Leitstellenübernahme und Kopplung mit dem Gesprächssimulator. Eine ausdrücklich bestätigte Reihenfolge zwischen Simulator und Sprechstelle wurde nicht festgelegt.

## 7. Gegenprüfung am aktuellen Repository: Live-Audio und Integration

`TetraSpeechEncoder::push_pcm` sammelt PCM und gibt vollständige 60-ms-Blöcke zurück; `encode_complete_block` verarbeitet exakt 480 Samples. `TetraSpeechDecoder::decode_tmd_to_pcm` stellt die Gegenrichtung bereit. Diese Bausteine sind im Quellcode vorhanden. USB-Audioaufnahme und lokales ALSA-Playback werden dadurch noch nicht implementiert. [R15]

`AsteriskAudioTranscoder` verbindet PCMU und TETRA. Der geprüfte `MediaWorker` verwendet pro SIP-Dialog einen Thread `asterisk-codec`, begrenzte Kanäle mit Kapazität acht und eine maximale Audioalterung von 240 ms. Die Entity nutzt nichtblockierende Übergaben; zu alte Daten werden verworfen. Die 240 ms sind die Policy dieses Asterisk-Workers, kein automatisch geeigneter Vorpufferwert für Operator-PTT. [R16, R17]

Ein eingehender SIP-INVITE erzeugt einen `NetworkCircuitCall` mit individueller Kommunikation (`communication=0`) und Duplex (`duplex=1`). Das stützt die historische Abgrenzung: Ein vorhandener SIP-Client mit USB-Headset ist noch keine Gruppen-PTT-Sprechstelle. Gruppenmodus, Quell-ISSI, Ziel-GSSI, Halbduplex, Floor-Control und Busy-/Freigabeverhalten wären gesondert zu ergänzen. [R17]

Die Aussage „Asterisk beweist, dass Echtzeit-Audio funktioniert“ wird hier deshalb auf **vorhandene Echtzeit-Verarbeitung im Quellcode** und vorhandene Testfälle begrenzt. Ein am Pi erfolgreich laufender SIP-/RF-Versuch wurde bei der Quellenprüfung nicht durchgeführt.

Der Build verwendet Rust-Workspace-Edition 2024. Im Binary-Paket `bluestation-bs` sind `asterisk`, `recording` und `audio-player` Defaultfeatures. Auf Entity-Ebene aktiviert `asterisk` den `tetra-codec`; `audio-player` aktiviert `tetra-codec` und `recording`. Der Codec wird als native Bibliothek `tetra-codec` eingebunden; `build.rs` wertet bei aktiviertem Feature verfügbare `pkg-config`-Linkpfade aus. Diese Abhängigkeiten sind für spätere Builds relevant, wurden hier aber nicht installiert oder gebaut. [R15, R23]

## 8. Relevante Konfiguration, Schnittstellen, Pfade und Parameter

### 8.1 Vorhandene AudioPlayer-Konfiguration

Die folgende Tabelle unterscheidet Parserdefaults von der eingecheckten `config.toml`. Beides ist keine gelesene Live-Konfiguration des Pi. [R5, R25]

| Parameter | Parserdefault | Eingecheckte Konfiguration |
|---|---:|---:|
| `audio_player.enabled` | false | true |
| `source_issi` | 4010099 | 4010001 |
| `default_priority` | 5 | 5 |
| `directory` | `/var/lib/netcore/audio` | gleich |
| `cache_directory` | `/var/cache/netcore/audio` | gleich |
| `max_file_size_mb` | 100 | 100 |
| `max_duration_seconds` | 1800 | 1800 |
| `lead_in_silence_blocks` | 12 | 12 |
| `tail_silence_blocks` | 3 | 3 |
| `group_release_guard_seconds` | 6 | 6 |
| `individual_answer_timeout_seconds` | 30 | 30 |
| `ffmpeg_path` | `ffmpeg` | `ffmpeg` |

Das Größenlimit wird im Code mit 1024 × 1024 Bytes multipliziert. Der Timeoutparameter wird im Entity-Lebenszyklus auch auf einen noch ohne Timeslot wartenden Aufbau angewandt. Die hypothetischen Simulatorgrenzen 50 Beiträge/600 Sekunden sind davon unabhängig.

### 8.2 Vorhandene Nachrichten und HTTP-API

Interne Rufsteuerung läuft über `Sap::Control` und `CallControl`; Audiodaten über `Sap::TmdSap` und `TmdCircuitDataReq`. `NetworkCallStart` enthält UUID, Quell-ISSI, Ziel-GSSI und Priorität. `NetworkCallReady` liefert UUID, Ruf-ID, logischen Timeslot und Usage. `NetworkCallEnd` trägt die UUID. Das sind interne Nachrichten, keine in den Arbeitsnotizen festgelegten Netzwerkports für einen Simulator. [R19]

Vorhandene Dashboardrouten: `GET /api/audio/status`, `GET /api/audio/sources`, `GET /api/audio/browse`, `GET/HEAD /api/audio/preview`, `POST /api/audio/play` und `POST /api/audio/stop`. Das Play-JSON verwendet unter anderem `target_type`, `target_id`, `priority`, `source_type` und je nach Quelle `source_id`/`path` oder `recording_id`. Eine Quell-ISSI je Play-Aufruf wird dort nicht ausgewertet. Die vorhandenen TTS-Routen sind separat. [R18]

Ein künftiger Simulator-/Operator-Endpunkt, sein Rechtemodell und sein Protokoll sind noch festzulegen. Die alte beispielhafte Route `/system/lab/conversations` ist keine bestätigte geprüfte API.

### 8.3 Ports und Gerätepfade

| Gegenstand | Aus dem geprüften Code bzw. historischen Entwurf | Aussagegrenze |
|---|---|---|
| TBS-Dashboard | Parserdefault `0.0.0.0:8080` | Nicht am laufenden Pi verifiziert |
| Asterisk-Bridge SIP-Bind | Parserdefault `0.0.0.0:5062` | Vorhandene Bridge, nicht neue lokale PTT-API |
| SIP-Gegenstelle | Parserdefault `127.0.0.1:5060` | Keine bestätigte konkrete Betriebsgegenstelle |
| RTP-Portbereich | Parserdefault 30000–30100; PCMU, Payload Type 0, 8 kHz | Kein Portscan/Ende-zu-Ende-Test durchgeführt |
| USB-Audio | Entwurfsbeispiel `hw:1,0` | Gerät und unterstützte Formate unbekannt |
| HID-PTT | `/dev/input/eventX` bzw. stabiler `/dev/input/by-id/...`-Pfad | Entwurf; keine udev-/Dienstrechte umgesetzt |
| Serieller PTT | `/dev/serial/by-id/...`, Beispiel 115200 Baud | Entwurf; Protokoll und Watchdog offen |
| Szenarien | `/mnt/nfs-share/Simulationen` | Vorgeschlagener Pfad, kein verifizierter Mount |
| Neue Netzwerk-PTT-Schnittstelle | Kein Port/Transport verbindlich festgelegt | Offen |

Die Portwerte stammen aus den relevanten Konfigurationsdefaults [R26, R27]. Es wurden keine bestehenden Passwörter, Tokens, Schlüssel oder vollständigen authentifizierten URLs kopiert.

## 9. Fehlerbilder, Diagnose und bisherige Lösungen

Im historischen Entwicklungsstand wurde kein konkreter Implementierungsfehler durch Ausführung diagnostiziert und anschließend repariert. Die dort erwähnten abgeschnittenen Silben, verlorenen PTT-UP-Ereignisse, Geräteabbrüche, Audiopufferprobleme und Rückkopplungen waren **vorausgedachte Fehlerfälle**. Vorpuffer, Worker, Watchdog, Sendezeitlimit und Wiedergabedämpfung waren vorgeschlagene Gegenmaßnahmen, keine nachgewiesenen Reparaturen.

Die aktuelle statische Prüfung ergänzt folgende konkrete Punkte:

| Befund | Ursache bzw. Einordnung | Stand der Lösung |
|---|---|---|
| Mehrere schnelle Play-Aufrufe reichen nicht für ein Szenario | Globaler Auftragszustand und Gruppen-Freigabefrist | Zusammenhängende Szenario-Sitzung entwerfen; nicht implementiert |
| Wechselnde Quellen über bestehende Play-API fehlen | Quell-ISSI nur globale Konfiguration | Datenmodell/API/Status je Auftrag erweitern; nicht implementiert |
| Eine neue Netzquelle kann einen aktiven Sprecher ersetzen | CMCE erlaubt NetworkCallStart aus Transmitting | Floor-Policy und atomare Zulassung für Operator/Simulation fehlen |
| AudioPlayer-Secondary-Playout fraglich | Logische Slots 5–7 werden mit physischem Tick 1–4 verglichen | Statischer Befund; weder repariert noch im Funk reproduziert |
| Asterisk-Hauptträgerfeld wirkt historisch fehlerhaft | Am 03.10.2026 geprüfter UMAC-TMD-Pfad ignoriert es zur Trägerwahl und mappt den Slot selbst | Historische Schlussfolgerung korrigiert; Live-Test weiterhin offen |
| „Keine Symlinks“ war zu absolut | Browser filtert Links, Resolver prüft kanonisches Ziel | Aussage präzisiert; gewünschte Szenario-Regel offen |
| Ende/Neustart kann alte UUIDs betreffen | Floor-Ende entfernt aktive Brew-UUID; spätere Meldungen möglich | Sitzungs-/Generationsprüfung und Abbruchfälle spezifizieren |
| Simulierte Aktivität wäre nicht eindeutig gekennzeichnet | AudioPlayer wird am 03.10.2026 als local klassifiziert | Eigene Herkunftskennzeichnung vorgeschlagen, nicht implementiert |

Die aktuelle Quellenhistorie weist die relevanten Audio-Dateien in einem Import-Snapshot aus. Ohne den im historischen Entwicklungsstand benutzten Commit lässt sich nicht verlässlich datieren, wann jede geprüfte Schutzmaßnahme hinzugekommen ist. Der Befund vom 03.10.2026 datiert keine rückwirkende Fehlerbehebung.

## 10. Tests, Ergebnisse und Grenzen

### 10.1 Im Repository vorhandene Testfälle

Folgende Tests wurden als Quelltext geprüft, **nicht bei der Quellenprüfung ausgeführt**:

| Test/Datei | Was der Testcode abdeckt | Was er nicht belegt |
|---|---|---|
| `test_network_group_speaker_change_uses_remote_floor_grant`, `tests/test_cmce_bs.rs` | Zweiter Netzsprecher, RemoteFloorGranted und Vermeidung eines fälschlich lokalen FloorGranted | Anzeige auf echtem MS, kompletter Simulator, RF-Qualität |
| `test_network_group_start_uses_requested_priority_without_group_d_connect` | Gewünschte Rufpriorität und Signalisierung des Gruppenstarts | Betrieb unter Last |
| `test_network_group_start_emits_dense_initial_dsetup_burst` | Initiale D-SETUP-Wiederholungen | Akustisch vollständiger Empfang auf jedem Gerät |
| `a_tetra_block_is_sixty_milliseconds`, Asterisk-Audio | Konstante von 480 Samples | Laufzeitlatenz, USB-Capture oder erfolgreicher historischer Testlauf |
| `real_codec_preserves_twenty_ms_rtp_assembly_and_sixty_ms_tetra_blocks`, MediaWorker | Drei PCMU-Pakete mit je 160 Samples zu einem 35-Byte-TMD-Block und Rückweg zu 480 PCMU-Bytes | Gesamte SIP-/RF-/PTT-Kette |
| `stalled_codec_and_consumer_never_block_rf_or_replay_old_audio`, MediaWorker | Volle Queue, veraltete Ergebnisse und Kanaltrennung | Vollständige Ausfallbehandlung eines Operatorgeräts |
| AudioPlayer-`group_launch_gate_*` | Zulässiger Startslot und ausgeschlossene Frames | Secondary-Playout |
| Konfiguration `rejects_invalid_identity_and_priority`, `rejects_unsafe_rf_guard_values` | Wertevalidierung | Produktive Konfiguration oder Endgerätetest |

Quellen: [R2, R5, R16, R28]. Der ursprüngliche Satz, 60-ms-Blöcke seien „bereits getestet“, wird damit nur als vorhandener Testcode bestätigt. Ein datierter erfolgreicher Testlauf des historischen Entwicklungsstands fehlt.

### 10.2 Tatsächlich bei der Quellenprüfung vom 03.10.2026 durchgeführte Prüfungen

- Prüfung der verfügbaren Arbeitsgrundlagen und Anhänge.
- Live-Abfrage des Branches, separater Checkout von `Archiving`, Prüfung von HEAD und Remote-Stand.
- Thematischer Abgleich mit dem vorhandenen Archivindex.
- Statische Prüfung von Rufaufbau, Floor-Wechsel, Rufende, AudioPlayer-Auftrag/Vorbereitung/Playout, Pfadvalidierung, Codec/Worker, Slotmapping, API, Konfiguration und Startintegration.
- Gegenprüfung der historischen Implementierungsbehauptungen und Erfassung widersprechender Befunde.

Die Dokumentation enthält 13 geprüfte relative Links, 32 auf den Prüfcommit fixierte Quellverweise mit gültigen Pfaden und Zeilenangaben sowie 28 aufgelöste Quellenkürzel. Die Prüfungen auf typische Zugangsdatenmuster und fehlerhafte Ersatzzeichen blieben ohne Treffer. Diese Dokumentationsprüfung ist kein Funktionstest der vorgeschlagenen Erweiterungen.

**Nicht durchgeführt:** Cargo-Builds oder Rust-Tests, Pi-/USB-Zugriff, Aufnahme/Wiedergabe, PTT-Betätigung, RF-Aussendung, SIP-Anruf, Lasttest, Performance- oder Latenzmessung. Es wurden keine Dienste neu gestartet, keine Geräte neu konfiguriert und keine Schutzparameter produktiv verändert.

## 11. Befehle und Abläufe mit Ausführungsstatus

### 11.1 Erfolgreich ausgeführte Bestandsaufnahme

Die wesentlichen Git-Schritte waren:

```powershell
git ls-remote --heads https://github.com/JanHG98/netcore-tetra.git Archiving
git clone --single-branch --branch Archiving https://github.com/JanHG98/netcore-tetra.git output/netcore-tetra-archive-2026-10-03
git fetch origin Archiving
git rev-parse HEAD origin/Archiving
git status --porcelain=v1
git ls-tree -r --name-only HEAD Docs/archive
```

Die Befehle ab `git fetch` wurden im neu angelegten Checkout ausgeführt. Zum Prüfzeitpunkt zeigten lokales HEAD und Remote auf den in Abschnitt 1 genannten Commit; der Checkout war sauber. Zusätzlich wurden gezielte Dateisuchen und Quelltextlesungen durchgeführt. Ein `git push --dry-run origin HEAD:refs/heads/Archiving` meldete beim unveränderten Ausgangsstand `Everything up-to-date`; das allein ist kein Nachweis der späteren Veröffentlichung.

Im historischen Fachentwurf selbst sind keine ausgeführten Installations-, Deployment- oder Reparaturbefehle dokumentiert.

### 11.2 Nur vorgeschlagene spätere Verifikation

Nach Bereitstellung einer geeigneten Buildumgebung wären unter anderem diese gezielten Testläufe sinnvoll; **hier nicht ausgeführt und kein Erfolg behauptet**:

```sh
cargo test -p tetra-config rejects_unsafe_rf_guard_values
cargo test -p tetra-entities --features asterisk,audio-player --lib net_audio_player::entity::tests
cargo test -p tetra-entities --features asterisk --lib net_asterisk::media_worker::tests
cargo test -p tetra-entities --features asterisk,audio-player --test test_cmce_bs test_network_group_speaker_change_uses_remote_floor_grant
```

Die Tests setzen die Abhängigkeiten des jeweiligen Workspace-Standes und für Codecfeatures die native Codec-Bibliothek voraus. Diese Liste ist eine gezielte Auswahl, kein vollständiger Abnahmeplan.

Ein konkreter Installationsdienst, eine neue systemd-Unit, udev-Regeln, ALSA-Permissions oder ein Remote-PTT-Protokoll wurden für `OperatorRadio` noch nicht ausgearbeitet. Es gibt deshalb keinen belegten Installationsablauf, der bereits eine funktionierende Sprechstelle herstellen würde.

## 12. Verworfene, zurückgestellte und korrigierte Ansätze

- **Virtuelle Mobilstationen oder zweite Fake-Basisstation:** Für die reine Gesprächsdarstellung als unnötig aufwendig zurückgestellt; erst bei Registrierungs-, Uplink-, GPS- oder SDS-Emulation erneut relevant.
- **Liveaufnahme direkt im synchronen Router:** Als ungeeignet beschrieben; Aufnahme und vorzugsweise Encoding in einen Worker verlagern.
- **Live-PTT als bloße Datei-AudioPlayer-Erweiterung:** Für den Operatorbetrieb wurde eine eigene Entity empfohlen; der Simulator sollte dagegen den Player weiterverwenden.
- **Asterisk/SIP als schnelle Gruppen-PTT-Lösung:** Wegen individuellem Duplexpfad und fehlender Gruppen-/Floor-Bedienung zurückgestellt; kein endgültiges Verbot einer späteren SIP-Lösung.
- **Browser als primärer PTT:** Wegen Fokus-/Verbindungs-/Release-Risiken nicht bevorzugt; Test- und Fernbedienungsoption mit Lease blieb erhalten.
- **Netzwerk-PTT als erste lokale Racklösung:** Möglich, USB wurde bevorzugt; keine endgültige Hardwareentscheidung.
- **Verborgene Navigation als alleinige Zugriffskontrolle:** Im Entwurf ausdrücklich nicht ausreichend.
- **Unverändertes TDMA-Playout und pauschal TS1–8:** Durch geprüftes Slotmapping und AudioPlayer-Befund eingeschränkt.
- **Asterisk muss wegen main_carrier auf dem zweiten Träger scheitern:** Für den aktuellen UMAC-Pfad nicht ableitbar; siehe Abschnitt 5.6.
- **Vorhandene Testfunktion gleich erfolgreicher Test/Betrieb:** Nicht zulässige Gleichsetzung; hier konsequent getrennt.

## 13. Offene Aufgaben und Roadmap-Kandidaten

Die folgende Reihenfolge ist eine Empfehlung aus dem Repository-Abgleich, keine historische Projektpriorisierung.

| Reihenfolge | Aufgabe | Abhängigkeit / Abnahmekriterium |
|---|---|---|
| 1 | Reale ISSI-/GSSI-Liste, Audiodateien, Geräte und gewünschte Betriebsform festlegen | Identitäten und Assets verfügbar; keine Beispielnummern als Produktionsplan übernehmen |
| 2 | AudioPlayer-Slotvergleich und tatsächliches Secondary-Playout prüfen | Regressionstest für logische 5–7 sowie realer Test auf Haupt- und Sekundärträger |
| 3 | Gemeinsame Floor-/Ressourcenregeln für Simulation, Operator und übrige Audioquellen definieren | Aktiven Sprecher respektieren, erlaubte Übernahme separat autorisieren, Konkurrenz atomar entscheiden |
| 4 | Simulator-Datenmodell, Quelle je Segment und ganze Sitzung implementieren | Vollständiges Vorbereiten, eindeutige UUID-Lebenszyklen, Abbruch und begrenzte Dauer |
| 5 | Simulator-Bedienung, Zugriffsschutz, Herkunft und Stop ergänzen | Klare Status-/Auditdaten; eigener Simulationsursprung; vorhandene Medien-/Control-Room-Rechte abstimmen |
| 6 | USB-Audio und HID-PTT prototypisch anbinden | Stabile Gerätezuordnung, Rechte, Captureformat, nichtblockierende begrenzte Queue |
| 7 | Operatorzustände, Freigabeton, Busy, Tail und Fail-safe-Verhalten implementieren | Release in jeder Phase, Geräteverlust, Setup-Timeout und Sendezeitlimit zuverlässig |
| 8 | Beide Funktionen auf realer TBS/MS-Kombination abnehmen | Nachvollziehbarer Source-/Binary-/Konfigurationsstand und datierte Messungen |
| Später | RX-Monitoring, Handapparat, LEDs/Schalter, mehrere Operator-ISSIs, Remote-PTT, Szenariozufall und Kopplung | Stabiler Kernbetrieb und gesonderte Priorisierung |

Der spätere Abnahmeplan sollte mindestens umfassen: wechselnde ISSI-Anzeige und Name auf geeigneten Geräten; kein Listener und Grace-Phase; fehlende oder zu große Dateien; Pfad-/Allowlist-Ablehnung; Pause unter und über Hangtime; Stop in jeder Phase; echte lokale PTT-Anforderung während Simulation; belegte oder erschöpfte Bearer; Start/Ende ohne abgeschnittene Sprache; verlorene PTT-UP-Ereignisse; USB-Trennung/Wiederanmeldung; Encoderstau und zu alte Samples; Maximum-Sendezeit; physisch/logisch korrektes Playout beider Träger; keine falsche Registrierung oder RSSI-Telemetrie für simulierte Teilnehmer.

Für jede spätere Bestätigung sind verwendeter Commit, gebaute Binary, freigegebene Konfiguration ohne Geheimnisse, Hardware, Testzeit und beobachtetes Ergebnis zu erfassen. Bis dahin lautet der Entwicklungsstand der beiden Erweiterungen weiterhin **Idee mit ausgearbeiteten Planungsvorschlägen und vorhandenen Teilbausteinen**, ohne bestätigten Betrieb.

## 14. Quellen und technische Anker

Alle folgenden Repositorylinks sind auf den geprüften Commit fixiert. Zeilenangaben dienen der Wiederauffindbarkeit und beziehen sich auf diesen Snapshot.

| Kürzel | Quelle | Relevanz |
|---|---|---|
| R1 | [AudioPlayer types.rs](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-entities/src/net_audio_player/types.rs#L29-L127) | Zustände, Status, Play-Befehl und PreparedAudio |
| R2 | [AudioPlayer entity.rs](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-entities/src/net_audio_player/entity.rs#L26-L621) | Worker, Startgate, globaler Absender, Playout, Abschluss, Slotvergleich und Tests |
| R3 | [AudioPlayer service.rs](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-entities/src/net_audio_player/service.rs) | Busy-Prüfung 526–572; Symlinkfilter um 374; Resolver 1265–1311 |
| R4 | [AudioPlayer media.rs](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-entities/src/net_audio_player/media.rs#L138-L369) | Vorbereitung, Codecblöcke, Cache, ffmpeg und Stille |
| R5 | [AudioPlayer-Konfiguration](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-config/src/bluestation/sec_audio_player.rs) | Defaults, Guard, Identitäts-/Prioritätsprüfung und Tests |
| R6 | [CMCE: netzinitiierter Gruppenstart](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-entities/src/cmce/subentities/cc_bs/procedures/isi.rs#L777-L987) | Zulassung, Listener, Circuit, Quell-ISSI, Telemetrie und Ready |
| R7 | [CMCE: Gruppen-Transitionen](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-entities/src/cmce/subentities/cc_bs/procedures/group.rs#L287-L362) | Sprecherwechsel, UUID, RemoteFloorGranted und Hangtime |
| R8 | [CMCE: PDU-Hilfen](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-entities/src/cmce/subentities/cc_bs/pdu.rs) | has_listener 357–368; send_d_tx_granted_facch 1018–1042 |
| R9 | [CMCE: NetworkCallEnd-Routing](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-entities/src/cmce/subentities/cc_bs/routes/isi.rs#L156-L193) | Zuordnung über aktive UUID |
| R10 | [CMCE-Timer](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-entities/src/cmce/subentities/cc_bs/timers.rs#L674-L699) | Hangtime und Prioritätsausnahme |
| R11 | [UMAC der Basisstation](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-entities/src/umac/umac_bs.rs) | Slotmapping 196–220; TMD-Eingang 1637–1656 |
| R12 | [TimeslotAllocator](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-core/src/timeslot_alloc.rs#L35-L75) | Sechs Traffic-Bearer; Sekundär-TS1 als Control/Guard |
| R13 | [TdmaTime](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-core/src/tdma_time.rs#L42-L120) | Physischer Zeittakt 1–4 |
| R14 | [MessageRouter](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-entities/src/messagerouter.rs#L194-L285) | Synchrones Ticken und Zeitfortschritt |
| R15 | [Gemeinsamer Sprachcodec](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-entities/src/net_audio/codec.rs#L8-L191) | Sample-/Blockformat, Encoder, Decoder und native Bibliothek |
| R16 | [Asterisk MediaWorker](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-entities/src/net_asterisk/media_worker.rs#L1-L102) | Codec-Thread, Queue-Limits, Alterungsgrenze und Tests |
| R17 | [Asterisk entity.rs](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-entities/src/net_asterisk/entity.rs) | INVITE/NetworkCircuitCall 934–951; TMD-Versand 1301–1307 |
| R18 | [Dashboard-Audio-API](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-entities/src/net_dashboard/server.rs#L2876-L3047) | Status, Quellen, Browse, Preview, Play und Stop |
| R19 | [CallControl-Vertrag](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-saps/src/control/call_control.rs#L122-L141) | Interne Start-/Ready-/End-Nachrichten |
| R20 | [Telemetriequellen](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-entities/src/net_telemetry/events.rs#L17-L28) | Am 03.10.2026 geprüfte Herkunftszuordnung ohne simulation |
| R21 | [Entity-Aufzählung](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-core/src/tetra_entities.rs) | AudioPlayer/Asterisk vorhanden, OperatorRadio fehlt |
| R22 | [Startintegration](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/bins/bluestation-bs/src/main.rs#L454-L539) | Instanziierung abhängig von Feature und Konfiguration |
| R23 | [Entity-Features](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-entities/Cargo.toml#L60-L73), [Binary-Features](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/bins/bluestation-bs/Cargo.toml#L34-L46), [Buildskript](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-entities/build.rs), [Workspace](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/Cargo.toml#L243) | Compilefeatures und native Abhängigkeiten |
| R24 | [Zellkonfiguration](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-config/src/bluestation/sec_cell.rs#L715) | Hangtime-Default und Begrenzung |
| R25 | [Eingecheckter AudioPlayer-Abschnitt](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/config.toml#L667-L681) | Konkrete Repositorywerte, keine Live-Konfiguration |
| R26 | [Asterisk-Konfigurationsdefaults](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-config/src/bluestation/sec_asterisk.rs#L270-L303) | SIP- und RTP-Ports |
| R27 | [Dashboard-Konfiguration](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-config/src/bluestation/sec_dashboard.rs#L48-L49) | Bind-/Portdefault |
| R28 | [CMCE-Komponententests](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-entities/tests/test_cmce_bs.rs#L977-L1061), [Asterisk-Audiotests](https://github.com/JanHG98/netcore-tetra/blob/57570d905244d5a166c948e54e129b3daf5bdb83/crates/tetra-entities/src/net_asterisk/audio.rs#L135-L178) | Vorhandene Testdefinitionen, keine hier ausgeführten Läufe |

Weitere Archive im selben Branch behandeln [Dual-Carrier, Bearer, ACK und Release](2026-10-03_flowstation-dualcarrier-bearer-ack-release-und-secondary-control.md), [ISSI und Systemidentität](2026-10-03_basisstation-issi-eigentuemer-und-systemidentitaet.md) sowie [WERMA und Rack-Architektur](2026-10-03_werma-racksignalisierung-bpi-r4-pro-und-rack-architektur.md). Diese Verweise dienen der Navigation; sie ersetzen keinen in diesem Dokument fehlenden Betriebsnachweis und wurden durch die Dokumentation nicht geändert.

Der zugängliche ursprünglichen Entwicklungsstand nennt keine konkrete Implementierungs-PR und keinen tatsächlichen Implementierungscommit für Simulator oder lokale Sprechstelle. Entsprechende Nummern werden daher nicht ergänzt.
