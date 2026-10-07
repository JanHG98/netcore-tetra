# Brainstorming: FlowStation-DualCarrier-Portierung, SXceiver und Hotfixes 001–009

> **Historisches Ergebnis, kein aktuelles Deployment-Handbuch.** Gegenstand ist die erste Übernahme des zweiten Carriers in den FlowStation-Fork und die anschließende Fehlersuche am SXceiver. Nach Hotfix 006 wurde das Ende der Gesprächsschleife im Betrieb bestätigt. Eine vollständige Abnahme unabhängiger Gespräche auf dem zweiten Carrier wurde nicht dokumentiert. Der am 03.10.2026 überprüfte Repository-Code ist wesentlich weiterentwickelt und weicht insbesondere bei Ressourcenvergabe, Secondary-Control und TX-Timing vom damaligen Stand ab.

## 1. Rahmen und Quellenstand

| Merkmal | Wert |
|---|---|
| Projekt | NetCore-Tetra |
| Thema dieses Vorhabens | FlowStation-Upstreamvergleich; DualCarrier-Portierung ohne Verlust eigener Integrationen; Konfiguration 418/408 MHz; SXceiver; Hotfixes; Dashboard/Telemetrie; Git-Push und Versionierung |
| Historischer Zeitraum | 28.06.2026; die Betriebslogs tragen `Jun 28`. Das Jahr wird zusätzlich durch den am 03.10.2026 aufgelösten Commit des Tags `v1.3.0` vom 28.06.2026 gestützt. |
| Erstellungs- und Prüftag dieses Archivs | **03.10.2026** |
| Historischer Fork | `JanHG98/flowstation`, überwiegend lokaler Arbeitsbranch `main` |
| Aktuelles Archiv-Repository | `JanHG98/netcore-tetra` |
| Auflösung des alten Repository-Namens am 03.10.2026 | Die GitHub-Abfrage für `JanHG98/flowstation` liefert `JanHG98/netcore-tetra`, Repository-ID `1281497427`. Der alte Name wird nicht als zweites unabhängiges aktuelles Repository behandelt. |
| Geprüfter Branch | **`Archiving`** |
| Archiving bei Beginn der Prüfung | `15c3f9f8dc8313dc6efdc2039ab86ad8719015ff`, Tree `9dafb6acb37a8142791700bf1595d920962703b3` |
| Zusätzlich geprüfter main-Stand | `6aa9be8f74ab731f72dc133a5f8e90c5018c626d` |
| Am 03.10.2026 aufgelöster historischer Tag | `v1.3.0` → `7834f46748f3205ce6d3e6e1480345c6bdf27bca` |
| Historischer Upstreambezug | `razvanzeces/flowstation`; damalige Suchergebnisse verwiesen unter anderem auf `ca8e901e49bdfee9f303406e534735d37093166d`. Das ist ein damaliger Quellenanker, keine nachgewiesene gemeinsame Merge-Basis. |
| Upstream-HEAD bei Abfrage vom 03.10.2026 | `0f4faa98b1abe9ec295cf7a94a7a1fa4b3a869b8`, Branch `main`; nur ergänzend aufgelöste Metadaten, kein vollständiger erneuter Upstream-Audit. |
| Ablage | `Docs/archive/2026-10-03_flowstation-dualcarrier-portierung-sxceiver-hotfixes.md` |

**Belegstufen in diesem Dokument:**

- **Idee:** diskutierter Ausbau ohne Umsetzungsauftrag oder Nachweis.
- **Beschlossen/geplant:** ausdrücklicher Wunsch beziehungsweise vereinbarter nächster Schritt; noch keine Aussage über vorhandenen Code.
- **Implementiert, statisch belegt:** in einem bezeichneten ZIP oder einem festgehaltenen Git-Commit im Quelltext vorhanden. Das bedeutet weder erfolgreich gebaut noch im Funkbetrieb geprüft.
- **Getestet:** ein tatsächlich ausgeführter Test mit nachvollziehbarem Ergebnis. Nur vorgeschlagene `cargo`-Befehle zählen nicht.
- **Im Betrieb bestätigt:** konkrete Betriebsbeobachtung oder Laufzeitlog für den bezeichneten Stand. Diese Bestätigung gilt nur für das beobachtete Verhalten, nicht pauschal für alle Funktionen.

Grundlagen sind die Konfigurations- und Loganhänge, zwei Dashboard-Abbildungen sowie die ausgelieferten Archive, insbesondere der vollständige Hotfix-009-Stand. Frühere Diagnoseergebnisse sind teilweise nicht erhalten. Die ETSI-Unterlagen wurden nicht vollständig neu geprüft; gezielte Fundstellen stehen in Abschnitt 12 als fachliche Einordnung vom 03.10.2026.

**Zugangsdaten:** Die ursprüngliche Konfiguration enthielt Zugangsdaten. Keine Passwörter, Tokens, privaten Schlüssel, Authentifizierungswerte oder vollständigen geheimnishaltigen Konfigurationsdateien werden in dieses Archiv übernommen. Interne Hostnamen, IP-Adressen, Ports und Funkparameter bleiben als technische Betriebskontextdaten erhalten.

## 2. Ziel, Ausgangslage und zentrale Schlussfolgerung

Ziel war, DualCarrier aus `razvanzeces/flowstation` in den erweiterten Fork zu übernehmen, ohne eigene Funktionen durch einen pauschalen Dateiaustausch oder unkontrollierten Merge zu verlieren. Zunächst wurden vollständige Dateipfade und konkrete Änderungen angefordert, anschließend ein direkt anwendbarer Patch und nach dessen Scheitern komplette Ersatzdateien beziehungsweise ZIP-Pakete.

Zu erhalten waren insbesondere NetCore-Directory-Anbindung, Brew/Brew2, EchoLink, MeshCom, SDS-/Status-Kommandos, Motorola-TPG2200-bezogene Funktionen, Asterisk/SIP und die vorhandenen Dashboard- und Gateway-Erweiterungen. Ein Teil dieser Integrationen war in der hochgeladenen Betriebskonfiguration ausgeschaltet. **Vorhandener Code, aktivierter Dienst und erfolgreich getestete Integration sind deshalb drei verschiedene Aussagen.**

Der erste Ansatz reduzierte den Umbau fälschlich auf `sec_cell.rs`, ein neues Dashboard-Modul und zwei HTTP-Routen. Tatsächlich müssen Konfiguration, Frequenzableitung, SDR-Tuning, Modulatoren/Demodulatoren, SAP-Primitiven, LMAC, UMAC, Ressourcenvergabe, Rufsteuerung und Telemetrie zusammenpassen. Die nachfolgenden Fehler machten diese Abhängigkeiten sichtbar.

**Belastbares historisches Ergebnis:** Zwei Carrier wurden konfiguriert und vom Stack abgeleitet; das SDR startete mit 600 kS/s und den expliziten Mittenfrequenzen. Auf dem Hauptträger wurde ein Gespräch auf TS2 zugewiesen. Nach dem Hamming-Dedupe war das Ende der Schleife im Betrieb bestätigt. Für eine unabhängige, vollständig funktionierende Traffic-Zuweisung auf Carrier 721 fehlt in den Arbeitsnotizen der Nachweis. Die geprüfte Nachprüfung des alten ZIPs und des Tags `v1.3.0` zeigt zudem einen damals noch auf drei Traffic-Slots begrenzten Allocator.

**Belastbares geprüftes Ergebnis:** Der aktuelle Code besitzt inzwischen einen sechs Ressourcen umfassenden logischen Allocator und weitergehende Carrier-Zuordnungen. Er betreibt den Secondary im Modus `SecondaryBcchNoMcch` und lässt zugewiesene Kontrollsignalisierung auf dessen Traffic-Slots zu. Der damalige Traffic-only-Zwischenstand darf daher nicht als aktuelle Sollarchitektur zurückgespielt werden. Details und Grenzen stehen in Abschnitt 11. [R1–R8]

## 3. Chronologie und Entscheidungsentwicklung

| Phase | Inhalt und Änderung | Nachweis / Einordnung |
|---|---|---|
| Ausgangsvergleich | Eigenen Fork mit Upstream vergleichen; Eigenfunktionen erhalten. | Ausdrücklicher Auftrag. Die anfängliche Einschätzung des Änderungsumfangs war zu klein. |
| Erster Patch | `flowstation-dualcarrier-netcore.patch`; vorgeschlagen: eigener Branch, `git apply --3way`, `cargo check`. | Benutzer meldete, dass der Patch nicht funktionierte. Kein erfolgreicher Patch-Test belegt. |
| Erste Ersatzdateien | Config, Dashboard, API und PHY-Frequenzliste als vollständige Dateien. | ZIP vorhanden; kein vollständiger DualCarrier-End-to-End-Port. |
| Weitere Dateianforderung | Zunächst `umac_bs.rs`, danach umfangreiche SAP-, PDU-, MAC-, PHY- und CMCE-Dateiliste. | Zeigt nachträglich erkannten Abhängigkeitsumfang. Angeforderte Dateien sind nicht automatisch geänderte Dateien. |
| Vollständiger Repo-Upload | `flowstation-main(1).zip`; Stage-2-Paket und vollständiger neuer Source-Stand. | Lokale Archivbytes zugänglich; Stage 2 enthält 18 Rust-Dateien plus README, nicht die gesamte zuvor angeforderte Liste. |
| Hotfix 001 | Zwei Rust-Compilerfehler: alter Bool-Reset nach HashMap-Umbau und fehlendes `mut` beim BitBuffer. | Fehler durch Compilerlog belegt; korrigierte Dateien ausgeliefert. Spätere Starts belegen, dass Folgestände gebaut wurden, nicht eine komplette Testserie. |
| Frequenzfestlegung | Zunächst 419 MHz besprochen, ausdrücklich auf **418 MHz / Main 720** korrigiert. | Verbindliche spätere Benutzerkorrektur. Secondary 721, Abstand 25 kHz. |
| Hotfix 002 | `tx_center_freq`/`rx_center_freq` im Parser und echten SoapyIO-Tuning ergänzen. | Vorher Unknown-Fields-Abbruch; später Laufzeitmeldung mit expliziten Center-Overrides. |
| Hotfix 003 | Dashboard-Writer verwechselt Arrayzeile `[0, 90],` mit TOML-Tabelle. | UI meldete `invalid array`, Zeile 152. Writer geändert; anschließend nächste, fachliche Validierungsstufe erreicht. |
| Sample-Rate | Explizite Rate erforderlich. Vorgeschlagene 1,8 MS/s wurden vom SoapySX-Treiber abgewiesen; Korrektur auf **600 kS/s**. | `Unsupported sample rate` mit Prozessabbruch; spätere Logs zeigen `fs: 600000.0` und laufenden Stack. |
| Hotfix 004 | Zusätzliche Timeslot-Reihe für Carrier 721. Secondary TS1 zunächst statisch `BCCH ACTIVE`. | Screenshot bestätigt Darstellung, nicht korrekte unabhängige Ressourcenbelegung. |
| Hotfix 005 | Exakte RX-Burst-Deduplizierung im PHY nach Bits, Slot/Subslot und RSSI. | Benutzer meldete weiterhin Schleife. Ansatz reichte in der beobachteten Situation nicht. |
| Hotfix 006 | Deduplizierung über Hamming-Distanz statt nur identischer Rohbits. | Benutzer: **„Aber die schleife ist jetzt weg :-)“**. Hauptträger-Gesprächszuweisung sichtbar; keine vollständige Zweiträger-Abnahme. |
| Dritter Carrier | Idee eines dritten Trägers, etwa 719 zusätzlich zu 720/721; zuerst Hardening. | Idee, nicht implementiert. Fähigkeiten des einzelnen MS wurden in der damaligen Begründung zu pauschal interpretiert. |
| Hotfix 007 | Verbesserte RX-Logs, hartes Secondary-Uplink-Gating, Carrier-Zuordnung in Telemetrie und Dashboard. | Umsetzungsauftrag und ZIP. Optional konfigurierbarer Filter beziehungsweise echte MS-fähigkeitsabhängige Entscheidung nicht belegt. |
| Hotfix 008 | Secondary bewusst auf `TrafficOnly`; BCCH-Anzeige entfernen; Idle-Slots leer. | Quelltextpaket vorhanden; direkt danach zeigt Benutzerlog ständig fehlenden BBK auf Secondary. |
| Hotfix 009 | All-empty-Slots nicht mehr als fehlerhafte Pakete behandeln, sondern überspringen. | ZIP und am 03.10.2026 aufgelöster Release-Commit vorhanden. Kein anschließender erfolgreicher Last-/Funk-Test in den Arbeitsnotizen dokumentiert. |
| Version | Intern zunächst v0.3.8/v0.4.0 vorgeschlagen. Verbindlicher Produktstand **v1.2.1**; daraus Vorschlag **v1.3.0**. | v0.x-Vorschläge überholt. Codename „Ghostbuster“ nur Vorschlag. |
| Git-Push | `GH007` wegen privater Commit-E-Mail; Noreply-/Amend-Anleitung; weiterer Push ohne sichtbare Ablehnung. | Letzter Push-Ausschnitt abgeschnitten, daher kein vollständiger damaliger Remote-HEAD-Abgleich. Tag `v1.3.0` am 03.10.2026 tatsächlich nachgewiesen. |

## 4. Historische Anforderungen und zuletzt vereinbarter Stand

### 4.1 Erhalt bestehender Funktionen

Gezielter Port statt Ersetzen ganzer Upstream-Dateien war die zentrale Anforderung. Besonders `config.rs`, `parsing.rs`, Dashboard `mod.rs`/`server.rs` und die Integrationen enthielten Fork-Erweiterungen. Ein bloßes Kopieren der Upstream-Version konnte diese entfernen.

Die Projektentscheidung, vollständige Dateien zu erhalten, war eine **Auslieferungsform**, keine Freigabe, eigene Funktionen ungeprüft zu überschreiben. Alle späteren ZIPs waren als auf dem gelieferten Stand aufbauende Ersatzdateien gedacht. Eine vollständige, automatisierte Regression aller Eigenfunktionen wurde nicht gezeigt.

### 4.2 Konfiguration und Umschaltung

- Ein Hauptträger und optional genau ein Sekundärträger.
- `secondary_carrier` behält die konfigurierte Nummer auch bei ausgeschaltetem DualCarrier.
- `dual_carrier_enabled = false` soll im effektiven Stack zu `secondary_carrier = None` führen.
- Bei fehlendem Schalter gilt im implementierten Modell `true`; ein vorhandener Secondary kann daher bereits ohne explizite Aktivierungszeile wirksam werden.
- Der WebUI-Schalter ist ein **Konfigurationswechsel mit geplantem Service-Neustart**, kein Hot-Reconfigure des aktiven Funkstacks.
- Ein fehlerhafter vorgeschlagener TOML-Inhalt soll vor dem Schreiben beziehungsweise Neustart abgewiesen werden.
- Explizite RX-/TX-Mittenfrequenzen sind unabhängig von der logischen Hauptträgerfrequenz; die Main-Felder bleiben 418/408 MHz.

Diese Semantik ist im geprüften Dashboard-Modul weiterhin statisch nachvollziehbar. Sie bestätigt nicht, dass die damalige Benutzerdatei nach jedem Versuch unverändert geblieben ist. [R9]

### 4.3 Historisches Secondary-Hardening

Zuletzt wurde in den Arbeitsnotizen beschlossen, den Primary für MCCH/Common-Control zu nutzen und den Secondary nicht als zweiten allgemeinen Random-Access-/Common-Control-Träger zu behandeln. Hotfix 007 sperrte Non-main-Uplink ohne zugewiesenen Traffic; Hotfix 008 setzte den Secondary-Scheduler auf `TrafficOnly` und erzeugte im Idle leere Slots.

**Wichtige Abgrenzung:** Die Formulierung „solange die Endgeräte `concurrent_multicarrier: false` melden“ wurde nicht als pro Endgerät ausgewertete Bedingung umgesetzt. Der alte Filter war global. Auch eine absolute Aussage „auf Secondary darf kein BCCH laufen“ lässt sich aus dieser Capability nicht ableiten. Der geprüfte Stand hat diese Vereinfachung bereits wieder verlassen; siehe Abschnitte 11 und 12.

### 4.4 Anzeige und Telemetrie

Gewünscht waren zwei eindeutig getrennte Carrier-Reihen mit jeweils vier physischen Timeslots, Live-Call- und Voice-Anzeige auf dem tatsächlich zugehörigen Carrier sowie keine gegenseitige Überschreibung von beispielsweise C720/TS2 und C721/TS2. Eine statische Kachel oder ein `running_active`-Flag allein sollte nicht als Funkmessung oder Kapazitätsnachweis gelten.

Die ersten Erweiterungen zeichneten lediglich zusätzliche Slots. Carrier-fähige Ereignisse und Zustandsverwaltung kamen erst später. Die geprüften logischen TS5–TS7 sind eine weitere Entwicklung und müssen von den vier physischen Slots pro Carrier unterschieden werden.

## 5. Architektur, Schichten und Abhängigkeiten

Relevanter Verarbeitungspfad:

```text
config.toml
  -> TOML/DTO/StackConfig + Validierung
  -> Haupt-/Sekundärfrequenzliste und SDR-Mittenfrequenzen
  -> SoapySDR / SoapySX / SXceiver

CMCE / Ressourcenvergabe / CallControl
  -> UMAC-Scheduler je Carrier
  -> TMV-Primitive, einzelne Slots oder Slot-Batch
  -> LMAC: logische Kanäle, BBK/AACH, Fehlerkorrektur, Scrambling
  -> TP-Primitive mit Carrier-Zuordnung
  -> PHY: Burstaufbau, Modulatoren, gemeinsamer SDR-IQ-Strom

SDR-IQ-Empfang
  -> Demodulatoren je Frequenz / Carrier
  -> RX-Kandidaten und Dedupe
  -> LMAC-Klassifikation + Secondary-Zulässigkeitsprüfung
  -> UMAC / LLC / MLE / MM / CMCE

Telemetrieereignisse
  -> Dashboard-Zustand / JSON / WebSocket
  -> Carrier- und Timeslotanzeige
```

Die Namen beschreiben den in den Arbeitsnotizen und den geprüften Quellen sichtbaren Ablauf. Sie behaupten keinen vollständig ETSI-konformen Gesamtstack.

### 5.1 Warum Carrier-Zuordnung durchgängig sein muss

Ein physischer Timeslot `2` ist bei zwei Trägern nicht mehr eindeutig. Entweder alle betroffenen Ebenen arbeiten konsequent mit `(carrier_num, air_ts)`, oder höhere Ebenen verwenden einen eindeutigen logischen Bearer-Schlüssel und übersetzen kontrolliert an der Funkgrenze. Halbe Umbauten führen zu falsch freigegebenen Ressourcen, überschriebenen Calls, falscher Audiozuordnung oder falsch angezeigter Aktivität.

Das ursprüngliche Stage-2-Paket führte `carrier_num` in mehreren PHY-/SAP-Strukturen ein, hatte aber die komplette höhere Ressourcenvergabe noch nicht entsprechend umgebaut. Am 03.10.2026 verwendet das Repository in wesentlichen höheren Pfaden logische Bearer-IDs 2–7; UMAC und Dashboard besitzen dafür explizite Übersetzungsfunktionen. [R3, R4]

### 5.2 Schnittstellen und Datenstrukturen

Für die Portierung relevant waren insbesondere:

- `TmvUnitdataReqSlot`, `TmvUnitdataReqSlots`, `TmvUnitdataInd`, `TmvConfigureReq` zwischen UMAC und LMAC.
- `TpUnitdataReqSlot`, `TpUnitdataReqSlots`, `TpUnitdataInd` zwischen LMAC und PHY.
- `RxSlotBits`, `TxSlotBits` und `RxTxDev::rxtx_timeslot` zur Geräte-/DSP-Anbindung.
- `TmdCircuitDataReq` und `TmdCircuitDataInd` für Traffic-Daten.
- Rufsteuerungs-, Circuit- und Kanalzuweisungsstrukturen in CMCE, CallControl und PDU-Feldern.
- `GroupCallStarted`, `IndividualCallStarted`, `TsVoiceActivity` für Carrier-bezogene Telemetrie.

Nicht jedes im damaligen Upstream gefundene `carrier_num`-Feld wurde tatsächlich in das ausgelieferte Fork-ZIP übernommen. Die angeforderte Datei- oder Symbolmenge darf daher nicht mit einer abgeschlossenen Umsetzung gleichgesetzt werden.

### 5.3 Abhängigkeiten

Historisch sichtbar sind ein Rust/Cargo-Workspace mit `tetra-core`, `tetra-config`, `tetra-saps`, `tetra-pdus`, `tetra-entities` und `bluestation-bs`, SoapySDR, SoapySX, systemd und Linux auf dem Pi-/SXceiver-System. Für den verwendeten Asterisk-Build wurde `--features asterisk` genannt; die Konfigurationsdokumentation verweist auf die native TETRA-Codec-Bibliothek für diese Integration. Eine erfolgreiche Installation jeder nativen Abhängigkeit wurde in dieser Entwicklungsphase nicht einzeln protokolliert.

Brew, Asterisk, Directory, Snom, Telegram und weitere NetCore-Dienste sind Integrationspartner; sie dürfen bei Carrier-Umbauten nicht stillschweigend aus Modullisten oder Konfigurationen verschwinden. Der geprüfte Workspace ist um zahlreiche zentrale Dienste erweitert. Eine Untersuchung aller dieser Dienste gehört nicht zu diesem Archiv-Audit.

## 6. Funkparameter, Konfiguration und lokaler Betrieb

### 6.1 Korrigierte Frequenzberechnung

Der in den Arbeitsnotizen gelesene `FreqInfo`-Code verwendet für den Downlink:

```text
DL_Hz = freq_band * 100000000 + carrier * 25000 + freq_offset
UL_Hz = DL_Hz - Duplexabstand   bei reverse_operation = false
```

Für `freq_band = 4`, `freq_offset = 0` und Duplexindex `0` ergibt sich ein Abstand von **10 MHz**, nicht die irrtümlich kommentierten 5 MHz.

| Carrier | BS sendet / DL | BS empfängt / UL | Rolle im historischen Aufbau |
|---:|---:|---:|---|
| 720 | 418,000000 MHz | 408,000000 MHz | Hauptträger |
| 721 | 418,025000 MHz | 408,025000 MHz | Sekundärträger |

`721` bedeutet also 418,025 MHz, nicht 419 MHz. Die zwischenzeitlich gerechneten Carrier 760/761 für 419,000/419,025 MHz wurden durch Jans ausdrückliche Korrektur verworfen. Die Alternative Secondary 719 unterhalb des Hauptträgers wurde nur als Option genannt, nicht als endgültige Betriebswahl. Historischer Codeanker: `crates/tetra-core/src/freqs.rs`, `FreqInfo::get_freqs` und Duplex-Tabelle.

### 6.2 Historisch zuletzt verwendeter RF-Konfigurationsausschnitt

**Nur ein Ausschnitt für die bestehenden Tabellen, keine vollständige Ersatzkonfiguration:**

```toml
[phy_io.soapysdr]
tx_freq = 418000000
rx_freq = 408000000
sample_rate = 600000
tx_center_freq = 418012500
rx_center_freq = 408012500

[cell_info]
freq_band = 4
main_carrier = 720
secondary_carrier = 721
dual_carrier_enabled = true
duplex_spacing = 0
freq_offset = 0
reverse_operation = false
```

Zusätzliche bestehende Tabellen oder Einstellungen dürfen dabei nicht gelöscht und Tabellenköpfe nicht doppelt eingefügt werden. Insbesondere Zugangsdaten gehören in die lokale Konfiguration und nicht in dieses Archiv.

Die Mitten liegen jeweils 12,5 kHz zwischen den Trägerfrequenzen. Die 600-kS/s-Einstellung ist im gezeigten SoapySX-Betrieb belegt. Das allein beweist weder die erforderliche Nachbarkanalselektivität noch ausreichende Aussteuerungsreserve, spektrale Reinheit oder gleichzeitige Gesprächskapazität. Die historische generische Empfehlung `sample_rate = 1800000` war für das verwendete Gerät beziehungsweise den Treiber nicht verwendbar.

### 6.3 Hardware- und Laufzeitparameter aus den Logs

| Parameter | Beobachteter Wert |
|---|---|
| Host | `SRV-M-TBS-01` |
| Projektverzeichnis | `/home/jan/flowstation` |
| Prozess/Binaryname | `bluestation-bs` |
| systemd-Unit | `tetra.service` |
| Service-Auswahl | `service_name=tetra`, im Startlog bestätigt |
| SDR | SXceiver, Hardwareversion 1.2 |
| Soapy-Treiber | `driver=sx`, Hardware-/Driver-Key `sx` |
| SoapySX-Codekennung | `9705147dd8c189625071f3f163ea56119bda4a05` |
| Erkannter Takt | 38,4 MHz |
| RX-/TX-Sample-Rate im laufenden korrigierten Start | 600000 Samples/s |
| RX-/TX-Kanal | jeweils 0 |
| Antennen-Auswahl | RX=`RX`, TX=`TX` |
| RX-Gainstufen im Log | LNA 42,0; PGA 16,0 |
| TX-Gainstufen im Log | DAC 9,0; MIXER 30,0 |
| Stream-Argument `period` | RX und TX jeweils `900` bei 600 kS/s |
| Netz-/Zellparameter | MCC 901, MNC 1510, LA 1, Colour Code 1 |
| Zeit-/SYNC-Konfiguration im Anhang | `Europe/Berlin`, `system_code = 1` |

Die Gainangaben werden nur als Treibereinstellungen dokumentiert, nicht als kalibrierte Sendeleistung. Die im Dashboard sichtbare Systemleistung ist kein Nachweis einer HF-Ausgangsleistung. Auch `ppm_err = 0` im Log ersetzt keine Frequenzmessung.

### 6.4 Dienste, Ports und Protokolle im historischen Setup

| Funktion | Ziel / Port / Protokoll | Beleg und Grenze |
|---|---|---|
| Basisstations-WebUI | `0.0.0.0:8080`, HTTP; Dashboard-Livefeed über WebSocket | Listener und Login im Log, Zugangswerte ausgelassen. |
| DualCarrier-Verwaltung | `GET /api/dualcarrier`, `POST /api/dualcarrier`, JSON | Implementierter Dashboard-Pfad; POST plant Neustart. |
| Betriebs-/Carrierinfo | `GET /api/btsinfo` | Als Einfügestelle und Anzeigequelle in den Arbeitsnotizen relevant. |
| Brew-Backhaul | `10.0.1.22:8081`, `ws://`, TLS im Anhang aus | Verbindung bestätigt; Versionserkennung v0, kein vollständiger Medien-/SDS-Nachweis. |
| NetCore Directory | `http://10.0.1.23:8095`, u. a. `/api/devices`; Timeout 2000 ms | Im Log Abruffehler und Rückfall auf lokale `devices.json`. |
| Asterisk-Gegenstelle | `10.0.1.21:5060`, SIP | Integration aktiviert; Gesprächstests separat erforderlich. |
| Lokale SIP-Anbindung | Kontaktadresse `10.0.1.20`, Bind-Port 5062 | Konfigurationswert, nicht gesondert gemessener Netzfluss. |
| RTP-Bereich | 30000–30100; Codec-Konfiguration PCMU | Konfiguriert; keine Paketmitschnitte zur Abnahme. |
| Rufpräfixe | ausgehend `91`, Strip aktiv; eingehend `T` | Bestehende Routingfunktion bewahren. `service_numbers = ["*"]` war breit konfiguriert, keine neue Empfehlung. |
| SDS-Kommandos | historisches Steuerziel ISSI 4010001 | Status 33001–33006 für restart, shutdown, kick_all, ip, temp, info; Zugriffsberechtigungen lokal. |
| Wetterdienst | historischer Service ebenfalls ISSI 4010001 | Bestandteil des damaligen Setups, keine geprüfte globale Nummernplanentscheidung. |
| Health | Samplerintervall 300 s; Watchdog-Neustart aus | Startlog und Konfiguration. |
| Recovery | proaktiver und reaktiver Modus im Anhang ausgeschaltet | Nicht nachträglich als im Betrieb getestete Recovery interpretieren. |

Brew und Asterisk waren aktiviert; Snom- und Telegram-Worker wurden gestartet. EchoLink, MeshCom, DAPNET und GeoAlarm meldeten im gezeigten Lauf deaktiviert. „Worker gestartet“ beziehungsweise „Integration enabled“ beweist noch keine erfolgreiche Nachrichten- oder Audiozustellung.

## 7. Fehler, Diagnose und Reparaturgrenzen

### 7.1 Unvollständiger Erstport und nicht anwendbarer Patch

Der handgeschriebene Patch wurde ausdrücklich als nicht funktionierend zurückgemeldet. In der sichtbaren Patchfassung standen künstliche Index-Platzhalter und unzuverlässige Hunk-Angaben. `git apply --3way` ist kein Ersatz für einen korrekten Diff und eine tatsächlich passende Basis. Eine jetzt ausgeführte rein lesende Strukturprüfung mit `git apply --numstat` meldete `corrupt patch at line 11`; es wurde kein Patch angewendet. Das Verfahren wurde historisch durch komplette Ersatzdateien und anschließend den vollständigen Repo-Upload ersetzt.

Die später erkannte Notwendigkeit von UMAC-, LMAC-, PHY-, SAP- und Call-Control-Änderungen widerlegt die erste Aussage, es handele sich im Kern nur um einen kleinen Dashboard-/Config-Port. Diese erste Einschätzung ist ausdrücklich **überholt**.

### 7.2 Hotfix 001: Rust-Typ- und Mutabilitätsfehler

Gemeldet wurden:

```text
error[E0308]: expected HashMap<(u16, u8), bool>, found bool
self.blk2_stolen = false;

error[E0596]: cannot borrow bbk_bits as mutable
```

Der alte globale Bool-Reset in `lmac_bs.rs::tick_start` passte nicht mehr zum Carrier-/Timeslot-bezogenen HashMap-Zustand. Der zweite Fehler entstand, weil `BitBuffer::to_bitarr` den internen Lesezustand verändert und deshalb einen veränderlichen Buffer benötigt. Ausgeliefert wurden die Entfernung der falschen Zuweisung und `let Some(mut bbk_bits) = prim.bbk else { ... }`.

Das war ein Compilerfix, kein Nachweis korrekter Lebensdauer aller Stealing-Flags. Der geprüfte LMAC behandelt den zweiten gestohlenen Halbslot gezielter burstbezogen; siehe Abschnitt 11.

### 7.3 Hotfix 002: Center-Felder und tatsächlicher SDR-Pfad

Die primäre Konfiguration scheiterte zuerst mit:

```text
Unrecognized fields: phy_io.soapysdr::["rx_center_freq", "tx_center_freq"]
```

Zusätzlich fehlte `/home/jan/flowstation/config.toml.fallback`. Damit war keine gültige Startkonfiguration vorhanden. Das vorübergehende Auskommentieren der Center-Felder war nur ein Workaround. Verbindlich gefordert ist ein Fix, der die Werte **akzeptiert und nutzt**.

Die spätere Auslieferung erweiterte DTO/Config-Zuordnung, Validierung und `soapyio.rs`. Der Benutzerlog zeigte anschließend:

```text
SDR centers: RX 408.012500 MHz / TX 418.012500 MHz (explicit center override)
```

Damit ist der Override-Pfad im beobachteten Lauf belegt. Vorher war RX im Legacy-Verhalten bei 407,980000 MHz, TX bei 418,000000 MHz zu sehen. Der Unterschied zwischen logischer Trägerfrequenz und tatsächlicher SDR-Mitte ist daher kein bloßes UI-Detail.

### 7.4 Hotfix 003: TOML-Array durch WebUI-Writer beschädigt

Der Fehler trat beim Versuch auf, DualCarrier im Dashboard zu aktivieren:

```text
resulting config does not parse: TOML parse error at line 152, column 1
152 | dual_carrier_enabled = true
invalid array
expected `]`
```

Im damaligen zeilenorientierten Writer genügte eine mit `[` beginnende Zeile, um einen neuen Tabellenabschnitt anzunehmen. Die Arrayzeile innerhalb von `local_ssi_ranges` wurde daher fälschlich als Abschnittsgrenze behandelt und fehlende Konfigurationsschlüssel wurden in das Array eingefügt.

Die Korrektur unterscheidet Tabellenköpfe wie `[cell_info]` oder `[[cell_info.neighbor_cells_ca]]` von Arrayzeilen wie `[0, 90],`. Die API prüft den vorgeschlagenen Inhalt vor dem eigentlichen Schreiben. Deshalb beweist die UI-Fehlermeldung zunächst einen fehlerhaften **Entwurf**, nicht automatisch eine bereits auf Platte beschädigte Datei. Der damals vorsorglich vorgeschlagene Restore aus `.dualcarrier.bak` war nicht als tatsächlich ausgeführt bestätigt.

Die geprüfte Lösung bleibt ein begrenzter Zeilenscanner und kein vollständiger formatbewahrender TOML-Parser. Mehrzeilige Strings, zitierte Tabellen und andere gültige TOML-Formen benötigen zusätzliche Tests beziehungsweise eine robuste Parser-/Writer-Lösung. [R9]

### 7.5 Fehlende und nicht unterstützte Sample-Rate

Nach dem Arrayfix griff die Validierung:

```text
dual carrier requires phy_io.soapysdr.sample_rate to be set so the secondary carrier can be proven to fit the SDR passband
```

Eine auskommentierte Zeile zählt nicht als gesetzte Rate. Der Geräte-Default 600 kS/s war im Runtime-Log sichtbar, stand dem Config-Validator aber ohne expliziten Wert nicht als zugesicherte Konfiguration zur Verfügung.

Die folgende Empfehlung von 1,8 MS/s war gerätespezifisch falsch. Der Log zeigte die beiden korrekt abgeleiteten Carrier und Center-Werte, danach aber:

```text
Failed to set RX sample rate: Other: Unsupported sample rate
Failed to open SDR device: Other: Unsupported sample rate
status=101
```

Mit `sample_rate = 600000` startete der gezeigte Folgelauf. Die Meldung „DualCarrier selbst ist durch“ war dennoch zu weitgehend: Der erfolgreiche Konfigurations-/Gerätestart beweist nicht den Gesprächspfad auf dem zweiten Carrier.

### 7.6 Doppelte RX-Verarbeitung und Gesprächsschleifen

Die Anhänge `Eingefügter Text(30).txt` und `(31).txt` enthalten eng aufeinanderfolgende beziehungsweise gleichzeitige Kandidaten auf 720 und 721, doppelte `MacAccess`, doppelte ACK-Verarbeitung und konkurrierende Fragment-/Grant-Abläufe. Besonders aussagekräftig sind zwei Rufanlagen für denselben Ursprung und dieselbe Gruppe:

```text
12:39:15.828: ISSI 5102 -> GSSI 15201 -> ts=2 call_id=4 usage=4
12:39:15.828: ISSI 5102 -> GSSI 15201 -> ts=3 call_id=5 usage=5

12:50:29.970: ISSI 5102 -> GSSI 15201 -> ts=2 call_id=4 usage=4
12:50:29.970: ISSI 5102 -> GSSI 15201 -> ts=3 call_id=5 usage=5
```

Das passt zu Jans Beobachtung: TS2 sendet, TS3 leuchtet sehr kurz auf, danach Schleife. Die konkrete Doppelanlage ist in `(30)` Zeilen 234/240 und `(31)` Zeilen 219/225 belegt. Sie ist aussagekräftiger als allein ein zweites PHY-Log mit einer anderen Carrier-Nummer.

Die damalige Diagnose lautete Nachbarkanal-Ghost beziehungsweise Doppeldecodierung desselben Uplinks durch zwei Demodulatoren. Die Logs stützen doppelte Weiterverarbeitung; **eine HF-/IQ-Messung, die die physikalische Ursache abschließend beweist, fehlt**. Die etwa 10 dB schwächere zweite RSSI-Kopie ist ein Hinweis, keine vollständige Ursachenbestimmung.

Hotfix 005 verglich rohe Bursts exakt. Das reichte nicht. Hotfix 006 verglich Hamming-Distanzen; danach war das Ende der Schleife im Betrieb bestätigt. Die Erklärung, zwei unterschiedliche Rohbitfolgen könnten nach Fehlerkorrektur zum gleichen MAC-Inhalt führen, ist plausibel, aber in den Arbeitsnotizen nicht durch gespeicherte Rohbitpaare samt Decoder-Test nachgewiesen.

### 7.7 Residuale Carrier-721-Meldungen nach Hotfix 006

Der letzte kurze Anhang `(32)` zeigt im `ChanAllocElement` `carrier_num: 720` und `ts_assigned: [false, true, false, false]`, also **C720/TS2**. Parallel erscheinen teils PHY-Meldungen mit Carrier 721.

Die damalige Antwort wertete diese pauschal als harmlose Rohkandidaten vor der Filterung. Das war nicht ausreichend abgesichert. Je nach Hotfixstand lag die betreffende Meldung bereits im Kandidaten-/Weiterleitungspfad nach der Dedupe-Entscheidung. Ein solcher Eintrag beweist weder einen echten Ruf auf 721 noch automatisch einen erfolgreich verworfenen Ghost.

Richtig ist die engere Bewertung: Die Betriebsbeobachtung „Schleife weg“ bleibt ein positiver Betriebshinweis. Ob unerwünschte Kopien noch bis LMAC/UMAC gelangen, muss anhand expliziter Drop-/Forward-Logs, Kanalzuweisung und Folgereaktionen geprüft werden. Hotfix 007 adressierte diese Unklarheit durch getrennte Logstufen und zusätzliches LMAC-Gating.

### 7.8 Hotfixes 007–009: Hardening, Traffic-only und leere Slots

Hotfix 007 beschränkte Secondary-Uplink auf `PhysicalChannel::Tp`. Das reduzierte unbeabsichtigte Common-Control-Verarbeitung, konnte aber legitime zugewiesene Signalisierung in Hangtime-/Retake-Phasen ausschließen. Eine nur nach MS-Capability aktivierte Ausnahme war nicht eingebaut.

Hotfix 008 schaltete den Secondary-Scheduler auf `TrafficOnly`. Im Idle wurden alle drei Bestandteile eines Slots leer: kein BBK, kein Block 1, kein Block 2. LMAC erwartete weiterhin BBK und protokollierte in jedem Timeslot:

```text
LMAC: batched slot missing bbk on carrier=721 ts=1/2/3/4
```

Hotfix 009 unterschied daraufhin vollständig leere Slots von fehlerhaft unvollständigen Slots: All-empty wurde mit TRACE übersprungen, nichtleere Slots ohne BBK/Block 1 blieben WARN und wurden verworfen.

**Am 03.10.2026 geprüfte Auditkorrektur zur damaligen Erklärung:** Das war nicht nur ein Logproblem. Das Überspringen von Carrier-Einträgen war mit dem damaligen positionsabhängigen DSP-`zip` und der zeitlichen Fortschaltung mehrerer Modulatoren abzugleichen. Auch Signalisierung auf dem Secondary während Hangtime oder Frame-18-Sonderfällen darf nicht durch eine vereinfachte Traffic-only-Bedingung verloren gehen. Abschnitt 10 beschreibt diese offenen Punkte des alten Codes; Abschnitt 11 die inzwischen sichtbaren Weiterentwicklungen.

### 7.9 Weitere beobachtete Meldungen

ALSA-/Pulse-/HDMI-Probe-Fehler traten beim Start neben der SDR-Gerätesuche auf. Im gezeigten erfolgreichen Ablauf wurde danach SoapySX geöffnet; sie waren nicht der belegte Auslöser der Sample-Rate-Abweisung oder der doppelten Rufanlage. Ebenso sind einzelne Startmeldungen `Lost -1200 samples` und `Too late to produce TX block 0` kein ausreichender Beweis für eine dauerhaft gestörte Funkstrecke, dürfen bei wiederholtem Auftreten unter Last aber nicht ignoriert werden.

Der Directory-Abruf scheiterte mindestens einmal und fiel auf `devices.json` zurück. Das ist ein eigener verbleibender Integrationspunkt, nicht die belegte Ursache der Carrier-Schleife. Eine erfolgreiche spätere Directory-Verbindung ist in den Arbeitsnotizen nicht nachgereicht.

## 8. Dateien und tatsächlich ausgelieferte Änderungsflächen

Alle nachfolgenden Pfade sind repositoryrelativ; im damaligen Arbeitsverzeichnis steht `/home/jan/flowstation/` davor.

### 8.1 Konfiguration und Frequenzableitung

```text
Cargo.toml
Cargo.lock
crates/tetra-core/src/freqs.rs
crates/tetra-core/src/timeslot_alloc.rs
crates/tetra-config/src/bluestation/config.rs
crates/tetra-config/src/bluestation/parsing.rs
crates/tetra-config/src/bluestation/sec_cell.rs
crates/tetra-config/src/bluestation/sec_phy.rs
crates/tetra-config/src/bluestation/sec_phy_soapy.rs
crates/tetra-entities/tests/common/default_stack.rs
```

`freqs.rs` wurde zur Berechnung geprüft; nicht jede oben genannte Datei wurde in dieser Entwicklungsphase geändert. `timeslot_alloc.rs` ist gerade wegen seiner damals fehlenden Erweiterung relevant. `config.toml` ist lokale Betriebsdatei; `example_config/config.toml` ist eine Vorlagen-/Dokumentationsfläche und nicht automatisch mit lokalen geheimen Einstellungen identisch.

### 8.2 Dashboard und Telemetrie

```text
crates/tetra-entities/src/net_dashboard/mod.rs
crates/tetra-entities/src/net_dashboard/dual_carrier.rs
crates/tetra-entities/src/net_dashboard/server.rs
crates/tetra-entities/src/net_dashboard/state.rs
crates/tetra-entities/src/net_dashboard/html.rs
crates/tetra-entities/src/net_telemetry/events.rs
```

Im am 03.10.2026 geprüften `Archiving`-Stand liegt die eigentliche Oberfläche zusätzlich beziehungsweise statt des großen Rust-Strings unter:

```text
crates/tetra-entities/src/net_dashboard/ui/dashboard.html
crates/tetra-entities/src/net_dashboard/ui/login.html
crates/tetra-entities/src/net_dashboard/ui/netcore.css
crates/tetra-entities/src/net_dashboard/ui/netcore.js
crates/tetra-entities/src/net_dashboard/ui/netcore-rf.css
crates/tetra-entities/src/net_dashboard/ui/netcore-rf.js
crates/tetra-entities/src/net_dashboard/ui/netcore-login.js
```

Ein altes vollständiges `html.rs` aus Hotfix 004, 007 oder 008 würde diese spätere Struktur ersetzen und ist deshalb keine geeignete geprüfte Installationsanweisung. [R10]

### 8.3 Tatsächliche Stage-2-Rust-Dateien

Das Stage-2-Changed-Files-ZIP enthält diese 18 Rust-Dateien sowie eine README:

```text
crates/tetra-entities/tests/test_umac_bs.rs
crates/tetra-entities/tests/test_umac_ms.rs
crates/tetra-entities/src/lmac/lmac_bs.rs
crates/tetra-entities/src/lmac/lmac_ms.rs
crates/tetra-entities/src/lmac/components/errorcontrol.rs
crates/tetra-entities/src/net_asterisk/entity.rs
crates/tetra-entities/src/net_brew/entity.rs
crates/tetra-entities/src/net_echolink/mod.rs
crates/tetra-entities/src/phy/phy_bs.rs
crates/tetra-entities/src/phy/components/demodulator.rs
crates/tetra-entities/src/phy/components/soapy_dev.rs
crates/tetra-entities/src/umac/umac_bs.rs
crates/tetra-entities/src/umac/subcomp/bs_sched.rs
crates/tetra-pdus/src/phy/traits/rxtx_dev.rs
crates/tetra-saps/src/sapmsg.rs
crates/tetra-saps/src/tmd/mod.rs
crates/tetra-saps/src/tmv/mod.rs
crates/tetra-saps/src/tp/mod.rs
```

### 8.4 Zusätzliche Analyse-/Portierungspfade

In den Arbeitsnotizen wurden außerdem die folgenden Dateien für einen vollständigen Port angefordert beziehungsweise als Abhängigkeiten genannt. **Das ist keine Behauptung, dass Stage 2 sie alle verändert hat:**

```text
crates/tetra-saps/src/tma/mod.rs
crates/tetra-saps/src/control/call_control.rs
crates/tetra-pdus/src/umac/fields/channel_allocation.rs
crates/tetra-pdus/src/umac/pdus/mac_resource.rs
crates/tetra-pdus/src/cmce/structs/cmce_circuit.rs
crates/tetra-entities/src/umac/subcomp/circuit_mgr.rs
crates/tetra-entities/src/umac/subcomp/bs_defrag.rs
crates/tetra-entities/src/cmce/components/circuit_mgr.rs
crates/tetra-entities/src/cmce/subentities/cc_bs/routes/ra.rs
crates/tetra-entities/src/cmce/subentities/cc_bs/pdu.rs
crates/tetra-entities/src/cmce/subentities/cc_bs/procedures/setup.rs
crates/tetra-entities/src/cmce/subentities/cc_bs/procedures/group.rs
crates/tetra-entities/src/cmce/subentities/cc_bs/procedures/individual.rs
crates/tetra-entities/src/cmce/subentities/cc_bs/procedures/uplink.rs
crates/tetra-entities/src/cmce/subentities/cc_bs/procedures/isi.rs
crates/tetra-entities/src/cmce/subentities/cc_bs/state/mod.rs
crates/tetra-entities/src/cmce/subentities/cc_bs/lifecycle.rs
crates/tetra-entities/src/cmce/subentities/cc_bs/timers.rs
crates/tetra-entities/src/cmce/subentities/sds_bs.rs
```

`setup.rs`, `individual.rs` und `isi.rs` tauchen später tatsächlich im Hotfix-007-Archiv auf. `soapyio.rs` kam im Center-Frequency-Fix hinzu. Der Änderungsnachweis muss jeweils dem bezeichneten Paket beziehungsweise Commit entnommen werden, nicht der Wunschliste.

## 9. Befehle, Deployment und Git-Veröffentlichung

### 9.1 Historisch vorgeschlagener ZIP-Ablauf

Beispiel des in den Arbeitsnotizen wiederholt vorgeschlagenen Verfahrens:

```bash
cd /home/jan/flowstation
unzip flowstation-dualcarrier-empty-slot-hotfix-009.zip
cp -av flowstation-dualcarrier-empty-slot-hotfix-009/* .
cargo check
cargo build --release --features asterisk
sudo systemctl restart tetra.service
journalctl -u tetra.service -n 100 --no-pager
```

**Status:** Die Befehle waren Empfehlungen. Anschließend lagen mehrfach Compiler- beziehungsweise Laufzeitlogs vor; damit sind konkrete Fehlversuche und Starts belegt. Es liegt aber nicht für jede ZIP-Stufe eine vollständige Ausgabe mit erfolgreichem `cargo check`, Build-Abschluss und exakt installierter Binary vor. Die Hotfixnummern sind Paketbezeichnungen, keine automatisch erstellten Git-Tags.

**Nicht als geprüfter Blind-Rollout benutzen:** Die alten Dateien passen nicht mehr ungeprüft zum aktuellen Repository. Vor einem geprüften Update muss der aktuelle Source-Commit feststehen und der tatsächliche `ExecStart` der Unit geprüft werden. `cargo check` ersetzt kein Release-Binary; ein Service-Neustart lädt nur dann den neuen Build, wenn er tatsächlich den gebauten beziehungsweise installierten Pfad verwendet.

### 9.2 Sicherer Fortsetzungsablauf — neu empfohlene Prüfung, nicht ausgeführt

```bash
cd /home/jan/flowstation

git status --short
git branch --show-current
git rev-parse HEAD
systemctl show tetra.service -p ExecStart -p WorkingDirectory
systemctl cat tetra.service

# Gezielt das tatsächlich verwendete Binary und dessen Features bauen.
# Voraussetzung: passender Toolchain-/Native-Dependency-Stand und bewusste Source-Auswahl.
cargo check -p bluestation-bs --features asterisk
cargo build --release -p bluestation-bs --features asterisk
```

Danach den gebauten Pfad mit `ExecStart` vergleichen; gegebenenfalls über den vorhandenen Installationsweg ausliefern, nicht einen unbekannten `/usr/local/bin`-Pfad erfinden. Erst nach erfolgreichem Build und abgestimmtem Zeitpunkt neu starten und die **neue** Startsequenz prüfen:

```bash
sudo systemctl restart tetra.service
journalctl -u tetra.service --since '2 minutes ago' --no-pager
```

Ein Neustart unterbricht laufende Gespräche. Die Schritte sind historische Abläufe, kein neu ausgeführter Wartungslauf.

### 9.3 Fallback und Wiederherstellung

Relevante lokale Pfade waren:

```text
/home/jan/flowstation/config.toml
/home/jan/flowstation/config.toml.fallback
/home/jan/flowstation/config.toml.dualcarrier.bak
/home/jan/flowstation/devices.json
/home/jan/flowstation/sds_log.json
```

Eine Fallback-Datei sollte erst aus einer nachweislich startfähigen, gesicherten Konfiguration erzeugt werden. Der historische Befehl `cp config.toml config.toml.fallback` wurde vorgeschlagen, aber seine erfolgreiche Ausführung nicht bestätigt. Eine `.dualcarrier.bak` ist nicht automatisch ein garantierter funktionierender Stand. Vor einem Restore müssen Inhalt, Zeitpunkt und aktuelle Konfiguration geprüft werden; ein blindes Zurückkopieren kann neuere Einstellungen verlieren.

Die Dateien können Zugangsdaten oder Betriebsdaten enthalten. Sie sind nicht zusammen mit einem „alles hochladen“-Befehl ungeprüft zu veröffentlichen.

### 9.4 Was „alles nach Git pushen“ tatsächlich bedeutete

Historisch verwendet wurden die Befehle `git add -A`, `git diff --cached --name-only`, `git commit`, `git push origin main` sowie einen separaten Tag-Push. Der wesentliche Ablauf war:

```bash
git status
git add -A
git diff --cached --name-only
git diff --cached
# Erst nach Prüfung auf lokale Betriebsdateien und Geheimnisse committen.
git commit -m "netcore: add dual carrier traffic-only mode"
git push origin main
```

Diese historische Commitnachricht ist keine geprüfte Stabilitätszusage. Der damals ebenfalls vorgeschlagene Zusatz „stable“ war mangels Zweiträger-Abnahme nicht gerechtfertigt.

Ein Push überträgt Commits beziehungsweise angegebene Referenzen, nicht beliebige uncommittete oder ignorierte Dateien und nicht automatisch den laufenden Build. Ein sauberer Arbeitsbaum und identische Commit-IDs nach einem Fetch sind getrennt zu prüfen:

```bash
git fetch origin
git status --short
git rev-parse HEAD
git rev-parse origin/main
```

Das Entfernen einer bereits getrackten Konfiguration mit `git rm --cached` schützt nur zukünftige Stände; frühere veröffentlichte Inhalte bleiben in der Historie. Eine gesonderte Prüfung und gegebenenfalls Rotation bereits veröffentlichter Geheimnisse bleibt nötig. Dieser Dokumentation verändert diese Historie nicht.

Der letzte Benutzer-Push-Ausschnitt endete bei einer abgeschnittenen `To https://github.com/...`-Zeile. Die übliche erfolgreiche Ref-Aktualisierung und ein Hashvergleich waren dort nicht sichtbar. Der historische Erfolg kann deshalb nicht allein aus diesem Ausschnitt als vollständige Synchronität bestätigt werden. Am 03.10.2026 ist immerhin der Releaseanker `v1.3.0` im Remote nachgewiesen. [R2, G2]

### 9.5 GH007 und private Commit-E-Mail

Ein Push wurde ausdrücklich mit `GH007: Your push would publish a private email address` abgelehnt. Vorgeschlagen wurde, die repositorylokale Git-Identität auf die eigene GitHub-Noreply-Adresse umzustellen und den noch unveröffentlichten eigenen Commit neu zu schreiben:

```bash
git config user.name "JanHG98"
git config user.email "<EIGENE_GITHUB_NOREPLY_ADRESSE>"
git commit --amend --reset-author --no-edit
git push origin main
```

Für mehrere unveröffentlichte eigene Commits wurde ein interaktives Rebase vorgeschlagen. **Nicht pauschal fremde Autoren überschreiben und nicht veröffentlichte gemeinsame Historie zwangsläufig umschreiben.** Die genaue Zahl betroffener lokaler Commits und die wirklich ausgeführten Korrekturbefehle fehlen in den Arbeitsnotizen. Es wurde kein Force-Push angefordert. GitHubs E-Mail-Schutz sollte nicht allein zur Umgehung des Fehlers ausgeschaltet werden. [G1]

### 9.6 Produktversion, interne Version und Tag

Verbindlicher bestehender Produktstand war **v1.2.1**. Der daraus abgeleitete nächste Vorschlag war **v1.3.0**; v0.3.8/v0.4.0 waren vorherige, überholte Vorschläge anhand der internen v0.3.7-Anzeige. „Ghostbuster“ war lediglich ein Namensvorschlag.

Am 03.10.2026 existiert `refs/tags/v1.3.0` und zeigt direkt auf Commit `7834f46748f3205ce6d3e6e1480345c6bdf27bca` vom 28.06.2026, 11:32:40 UTC. Committext: `Implement empty slot skipping logic in LMAC`. Der Parent ist `56f216764a0b3085b49c45b3305e4ccb5e92363f`, passend zum Buildpräfix aus dem Hotfix-008-Log.

Der Ref verweist auf einen **Commit**, also auf einen Lightweight-Tag und nicht auf das in den Arbeitsnotizen vorgeschlagene annotierte Tagobjekt. Das ist kein Fehler, aber ein Unterschied zwischen vorgeschlagenem Befehl und überprüftem Ergebnis. Eine GitHub-Release-Seite, Release-Assets, ein Produktversions-Bump in allen Dateien oder das Deployment dieses exakten Tags wurden hier nicht nachgewiesen. [R2]

## 10. Am 03.10.2026 geprüfte Nachprüfung des historischen Auslieferungsstandes

Dieser Abschnitt enthält **neue statische Auditbefunde über den alten Stand**, keine damals bestätigten Testergebnisse.

### 10.1 Drei-Slot-Allocator trotz DualCarrier-Unterbau

Im vollständigen `flowstation-main-dualcarrier-hotfix-009-nosensitive.zip` hatte `crates/tetra-core/src/timeslot_alloc.rs` weiterhin `owners: [Option<TimeslotOwner>; 3]` und akzeptierte nur TS2–TS4. Der gleiche Drei-Slot-Aufbau ist im am 03.10.2026 gelesenen Git-Tag `v1.3.0` direkt sichtbar. [R2]

Weitere alte ZIP-Befunde: `CmceCircuit` und `CallControl` hatten keinen durchgehend ergänzten Carrier-Schlüssel; wesentliche UMAC-Open-/Close-/Floor-Pfade arbeiteten weiterhin mit dem primären `channel_scheduler` und vier Einträgen pro Zustand. Die Stage-2-Auslieferung vergrößerte somit nicht schon automatisch die unabhängige höhere Rufkapazität.

**Folgerung:** Die damalige Formulierung „vollständiger DualCarrier-Port“ war nicht ausreichend gedeckt. Acht gezeichnete Slots, zwei abgeleitete Frequenzen und zwei DSP-Kanäle sind kein Beweis für sechs oder sieben unabhängig belegbare Traffic-Ressourcen. Das erklärt, weshalb die fehlende reale C721-Abnahme als wesentliche Lücke und nicht nur als kosmetischer Nachtest zu führen ist.

### 10.2 Leere Slots und positionsabhängige DSP-Verarbeitung

Im alten Stand wurden vollständig leere Slots in LMAC aus dem Batch entfernt. In `soapy_dev.rs` war die TX-Verarbeitung gleichzeitig positionsabhängig:

```rust
for (modulator, tx_slot) in self.modulators.iter_mut().zip(tx_slot) {
    // ...
}
```

Entfernte Einträge beeinflussen damit nicht nur das Log: Ein Modulator kann keinen passenden Slot mehr erhalten oder seine zeitliche Fortschaltung auslassen. Falls Einträge an anderer Stelle fehlen, ist auch eine falsche Zuordnung nach Reihenfolge möglich. Das ist ein statisch erkennbares Risiko; eine konkrete so verursachte Funkstörung nach Hotfix 009 ist in den Arbeitsnotizen nicht gemessen.

Der geprüfte Code verwendet dagegen Carrier-Lookup und explizite Stille pro Modulator; diese Weiterentwicklung darf beim Wiederaufnehmen nicht durch alte ZIPs ersetzt werden. [R7]

### 10.3 Grenzen des Dedupe

Die Hamming-Heuristik in Hotfix 006 und den Folgeständen benutzt:

| Trainingstyp | Grenze für Hamming-Distanz |
|---|---|
| `ExtendedTrainSeq` | `max(bit_len / 8, 24)` |
| `NormalTrainSeq1` / `NormalTrainSeq2` | `max(bit_len / 8, 40)` |
| Sonstige | `max(bit_len / 10, 16)` |

Verglichen werden gleiche Subslot-Kennung, gleicher Trainingstyp und gleiche Rohbitlänge innerhalb eines empfangenen Batches. Die stärkere RSSI-Kopie wird bevorzugt. Das ist ein **heuristischer Schutz**, kein beweisbar verlustfreier Mehrträger-Decoder.

In der untersuchten Vergleichsfunktion fehlen eine explizite Prüfung auf verschiedene Carrier, eine Prüfung ihres tatsächlichen Frequenzabstands, ein Vergleich der einzelnen `rx_slot.time`-Werte und eine Prüfung nach dekodiertem CRC-/Adress-/Rufkontext. Dass zwei echte unabhängige Bursts nie verworfen würden, wurde daher zu stark behauptet. Auch die Annahme „unabhängige Bursts unterscheiden sich ungefähr zur Hälfte“ ersetzt keinen Test mit strukturähnlichen Kontrollpaketen oder ähnlichen Sprachdaten.

Diese zentralen Heuristikgrenzen sind im geprüften gelesenen PHY-Code weiterhin erkennbar. Es wurde in den geprüften Pfaden kein konfigurierbarer Schwellwert oder abschaltbarer Betriebsmodus nachgewiesen. [R5]

### 10.4 Hardes Traffic-Gating und Signalisierung

Der alte Filter `secondary && pchan != Tp -> drop` berücksichtigt nicht, dass ein zugewiesener Bearer neben Sprachdaten Signalisierung während Hangtime, Retake, Release oder Stealing benötigt. Ebenso kann ein vereinfachtes „kein aktiver Traffic -> vollständig leer“ im Scheduler Kontrollphasen beeinflussen.

Es wäre falsch, das allein mit `concurrent_multicarrier = false` zu begründen. Der aktuelle LMAC erlaubt zusätzlich `Cp` auf Secondary-TS2–TS4; Secondary-TS1 bleibt für Uplink-Zugriff geschlossen. Das ist eine am 03.10.2026 überprüfte Codeänderung gegenüber dem historischen Hardening. [R6]

## 11. Zusätzlich überprüfter Repository-Stand vom 03.10.2026

### 11.1 Branches und Prüfmethode

Die Codeprüfung wurde an festen Commit-IDs vorgenommen. `main` stand auf `6aa9be8f74ab731f72dc133a5f8e90c5018c626d`; der bei Beginn gelesene Archivbranch auf `15c3f9f8dc8313dc6efdc2039ab86ad8719015ff`. Der Vergleich zeigte `Archiving` neun Commits voraus, nicht zurück. Er enthielt bereits Änderungen aus anderen Arbeiten, unter anderem UI-Auslagerung und Dienst-WebUI-Änderungen — **also nicht nur Archivdateien**.

Die hier geprüften RF-/Config-/Allocator-Dateien waren zwischen diesen beiden festgehaltenen Ständen nicht geändert. Dashboard `html.rs` und `server.rs` gehörten dagegen zu den abweichenden Dateien; die neue UI-Struktur und Timeslotlogik wurden deshalb explizit am Archivbranch-Commit gelesen.

Eine vollständige Quellcode- oder Sicherheitsprüfung aller Dienste, aller aktuellen Branches und aller Upstream-Änderungen wurde nicht durchgeführt. Ein versuchter direkter Git-Clone in die Arbeitsumgebung scheiterte an DNS; die erforderlichen Repository-Dateien konnten über die API gelesen werden. Rust/Cargo stand in der damaligen Prüfumgebung nicht zur Verfügung; es wurden keine Builds oder Funkversuche ausgeführt.

### 11.2 Ergebnisübersicht

| Teilbereich | Am 03.10.2026 statisch belegt | Unterschied / verbleibende Grenze |
|---|---|---|
| Konfiguration | Ein optionaler Secondary; Center-Felder im DTO und in `CfgSoapySdr`; PPM-korrigierte Center-Helper. | Weiterhin kein allgemeines N-Carrier-Konfigurationsschema. [R8] |
| Frequenz-/Passbandprüfung | Main/Secondary müssen verschieden sein; Hauptfrequenzen müssen zu `FreqInfo` passen; DualCarrier verlangt explizites `sample_rate`. | Prüfung der Trägermitten innerhalb `sample_rate / 2`, kein vollständiger Nachweis von Kanalbreite, Filterreserve oder tatsächlich unterstützten Geräteraten. [R8] |
| Ressourcenvergabe | Sechs logische Ressourcen mit IDs 2–7; `Sndcp` zusätzlich zu Brew/CMCE als Eigentümer. | Eine Codekapazität ist keine Bestätigung von sechs gleichzeitigen Funkgesprächen. [R3] |
| UMAC | Secondary-Scheduler, Übersetzung logical/air, Zustandsarrays mit acht Plätzen; TS8 unbenutzt/reserviert. | Nicht mit acht Traffic-Bearern verwechseln. [R4] |
| Secondary-Downlink | **`SecondaryBcchNoMcch`**, Secondary-TS1 für Control/Guard reserviert. | Historisches `TrafficOnly` ist nicht mehr aktueller Modus. [R4] |
| Secondary-Uplink | Secondary-TS2–TS4 mit `Tp` **oder `Cp`** zulässig; Secondary-TS1 nicht als allgemeiner Uplink zugelassen. | Keine per-MS-Capability-Verzweigung in dieser Funktion. [R6] |
| Dedupe/Logging | Hamming-Heuristik; Rohkandidaten TRACE, Forwarding TRACE, Dedupe-Entscheidungen DEBUG mit Parametern. | Heuristik-/Konfigurationsgrenzen aus Abschnitt 10 bleiben. [R5] |
| Stealing-Zustand | LMAC entfernt/verbraucht `blk2_stolen` burstbezogen. | Codeprüfung, kein nachgestellter Stealing-/Hangtime-Regressionstest. [R6] |
| TX-Verarbeitung | Modulatoren werden anhand `carrier_num` bedient; fehlender Carrier erhält Stille mit gleichem Zeitbezug; alle Teilblöcke werden vorbereitet. | Echte RF-Kontinuität muss gemessen werden. [R7] |
| TX-Diagnose | `TX continuity` mit Lead, Skip-Zählern und Hardware-Statuszählern im Code. | Keine aktuellen Live-Zähler vom Zielsystem vorhanden. [R7] |
| Dashboard/Telemetrie | Ereignisse mit Carrier, logische TS-Normalisierung, eigene Carrier-Schlüssel, C2-TS1 als Steuerung. | Konfigurierte Rollen werden teils statisch angezeigt; nicht jede Kachel ist eine Messung. [R10, R11] |
| WebUI-Writer | Arrayzeilen-Fix enthalten; Secondary-Nummer bleibt beim Abschalten erhalten. | Begrenzter TOML-Scanner; Backupfehler werden ignoriert; Schreiben nicht atomar. [R9] |
| Historischer Releaseanker | `v1.3.0` existiert und enthält alten Drei-Slot-Allocator. | Tag ist kein Beleg für vollständige damalige DualCarrier-Kapazität. [R2] |

### 11.3 Am 03.10.2026 geprüfte Zuordnung der logischen und physischen Slots

| Logischer Bearer / Anzeige | Physische Ressource bei historischem Carrierpaar 720/721 |
|---:|---|
| TS1 auf Main | C720 / Air-TS1, MCCH/Common-Control |
| 2 | C720 / Air-TS2 |
| 3 | C720 / Air-TS3 |
| 4 | C720 / Air-TS4 |
| Secondary-TS1 | C721 / Air-TS1, Control/Guard; keine Traffic-Ressource des Allocators |
| 5 | C721 / Air-TS2 |
| 6 | C721 / Air-TS3 |
| 7 | C721 / Air-TS4 |
| 8 | Kein freigegebener Traffic-Bearer im gelesenen Modell |

Die konkreten Carrierzahlen kommen aus der Konfiguration; das Mapping ist nicht fest auf 720/721 begrenzt. `air_ts_for_logical(5..=7)` zieht drei ab; die Gegenrichtung addiert drei für Secondary-Air-TS2–TS4. Die CMCE-Carrier-Hint-Auflösung kennt außerdem `Some(-2)` als interne Secondary-Anforderung, während nichtnegative Werte echte Carrier bezeichnen. Diese interne Kennung darf nicht ungeprüft als übertragene Carrier-Nummer verwendet werden. [R4]

Die aktuelle UI zeigt Secondary-Slots als `[1,5,6,7]` mit Erläuterung der Air-TS im Tooltip. Sie normalisiert auch lower-layer-Ereignisse, die bereits als Secondary plus physischem TS2–TS4 ankommen. Die Schlüssel enthalten Carrier und logischen TS. Damit unterscheidet sich die geprüfte Anzeige bewusst von der früheren rein physischen `TS1..TS4`-Reihe. [R10]

### 11.4 Am 03.10.2026 korrigierte beziehungsweise noch offene Altannahmen

**Inzwischen auf Codeebene weiterentwickelt:** größerer Allocator, logische Bearer-Zuordnung, Secondary-Control/Guard, Zulassen zugewiesener Cp-Signalisierung, Carrier-bezogene TX-Auswahl, explizite Stille und TX-Kontinuitätsdiagnose. Eine erneute Anwendung der alten Hotfixfolge wäre ein möglicher Rückschritt.

**Noch zu prüfen:** Die Dedupe-Prüfung unterscheidet im gelesenen Kandidatenvergleich nicht explizit verschiedene Carrier und speichert keinen individuellen Empfangszeitstempel. Der UMAC-Helper für unbekannte Carrier fällt mit Fehlerlog auf den Primary zurück; ob an allen Eingängen stattdessen ein kontrolliertes Verwerfen erforderlich ist, bleibt eine Review-Aufgabe. Alte Kommentare wie „Defaults to the main carrier for current CMCE allocation logic“ in Telemetrie-Strukturen sind nicht alleiniger Beweis der tatsächlichen geprüften Aufrufwerte.

**Wichtig für Konfigurationsmigration:** Die historische Config setzte `sndcp_service=true` bei `advanced_link=false`. Der geprüfte Validator verlangt bei aktivem SNDCP einen aktivierten WAP/IP- oder Packet-Data-Gateway-Pfad; aktive Packet-Profile verlangen `advanced_link=true`. Ein unverändertes Übernehmen der ganzen alten Konfiguration kann daher am 03.10.2026 scheitern, auch wenn die RF-Werte korrekt sind. Das ist ein späterer Funktions-/Validierungsstand, kein neuer Fehler des historischen Center-Frequency-Fixes. [R8]

## 12. Fachliche Klarstellungen zu früheren Aussagen

Diese Klarstellungen beruhen auf am 03.10.2026 gezielt gelesenen ETSI-Fundstellen und der aktuellen Codeprüfung. Sie werden **nicht rückwirkend als damals ausgeführte Normenprüfung ausgegeben**.

### 12.1 Concurrent Multicarrier ist nicht normale Trägerumschaltung

`concurrent_multicarrier: false` beschreibt nicht schlicht „das Funkgerät kann keinen zweiten Träger benutzen“. Die Norm behandelt gleichzeitigen Mehrträgerbetrieb, mehrere gleichzeitige Kanäle auf einem Carrier und Multislot als unterschiedliche Fähigkeiten. Eine Replace-Kanalzuweisung kann das MS auf eine andere Ressource verschieben; das ist nicht dasselbe wie zwei Carrier gleichzeitig für unabhängige Dienste zu verwenden.

Deshalb ist die damalige Schlussfolgerung „dritter Carrier bringt wegen concurrent=false grundsätzlich keine Kapazität“ zu pauschal. Zusätzliche Zellressourcen können für verschiedene Teilnehmer nutzbar sein, ohne dass jeder Teilnehmer gleichzeitig mehrere Träger bedienen muss. Ob die konkreten Geräte und der konkrete Stack die Zuweisung und Rückkehr korrekt beherrschen, bleibt praktisch zu testen. Quellen: EN 300 392-2 V3.8.1, Klauseln 23.5.4.1/23.5.4.2, insbesondere Seiten 928–929. [E1]

### 12.2 BCCH, MCCH und „Traffic-only“ nicht gleichsetzen

Für den beschriebenen Conventional-Access-Betrieb liegt der MCCH in Normal Control Mode auf Slot 1 des Hauptträgers. Daraus folgt nicht, dass jede Broadcast-/Synchronisationsfunktion auf einem zweiten Träger verboten wäre. Ebenso bedeutet Broadcast-Unterstützung nicht automatisch, dass ein zweiter allgemein zugänglicher MCCH mit Random Access eröffnet wird.

Die aktuelle Wahl `SecondaryBcchNoMcch` drückt genau diese notwendige Unterscheidung aus. Zugewiesener Verkehr benötigt außerdem Kontrollsignalisierung für seinen Lebenszyklus. Die historische Forderung „keine Common-Control-Ghosts auf Secondary“ bleibt als Ziel sinnvoll, ihre Umsetzung durch ein generelles Unterdrücken aller Secondary-Kontrollanteile war aber zu grob. Quellen: EN 300 392-2 V3.8.1, Klausel 4.11.1.1, Seite 71, plus geprüfte UMAC-/LMAC-Implementierung. [E1, R4, R6]

### 12.3 Diskontinuierlicher Secondary und DSP-Zeitachse

Die gelesene Normstelle zu D-CT erlaubt unter den dort beschriebenen Bedingungen diskontinuierliche Downlink-Übertragung auf anderen phase-modulierten Trägern. Eine fehlende HF-Aussendung ist jedoch nicht gleichbedeutend damit, einen Modulator softwareseitig ohne Zeitfortschritt auszulassen. Die gemeinsame DSP-Zeitachse und korrekte Burstrampen müssen erhalten bleiben. Das ist der Grund, leere Slots, explizite Stille und Carrier-Zuordnung zusammen zu prüfen. [E1, R7]

Eine ETSI-Konformitätsbewertung des gesamten Forks, eine Freigabe des RF-Aufbaus oder eine Prüfung regulatorischer Nutzungsvoraussetzungen wurde in dieser Entwicklungsphase nicht durchgeführt.

## 13. Tests, Beobachtungen und Abnahmegrenzen

| Prüfung / Beobachtung | Ergebnis und belastbare Reichweite |
|---|---|
| Anwenden des ersten Patches | Benutzer meldete Misserfolg. |
| Rust-Kompilierung eines frühen Stage-2-Standes | E0308 und E0596 tatsächlich protokolliert. |
| Folgestände starten als Dienst | Durch spätere Banner/Initialisierung belegt; vollständige Buildprotokolle fehlen. |
| Center-Felder werden akzeptiert | Expliziter Center-Override im Startlog bestätigt. |
| Zwei Carrier werden berechnet | `[(720,418000000,408000000),(721,418025000,408025000)]` protokolliert. |
| Sample-Rate 1,8 MS/s | SoapySX lehnte ab; kein zulässiger Betriebswert für diesen gezeigten Lauf. |
| Sample-Rate 600 kS/s | In späteren laufenden Starts bestätigt. |
| WebUI-Zweiträgerdarstellung | Screenshots vorhanden; zunächst statisch/teilweise irreführend. |
| Gesprächsversuch vor Dedupe | Doppelte Call-IDs, TS2/TS3-Zuweisungen, ACK-/Fragmentprobleme. |
| Hotfix 005 | Benutzer meldete weiterhin Fehler. |
| Hotfix 006 | Benutzer bestätigt Ende der Schleife; Log zeigt C720/TS2-Allocation. |
| Unabhängiger Ruf auf C721 | Kein eindeutiger Betriebsnachweis in den Arbeitsnotizen. |
| Gleichzeitige unterschiedliche Rufe C720/TS2 und C721/TS2 | Nicht dokumentiert. |
| Volle Kapazität, Duplex, Retake, Hangtime, Release, Frame 18 | Keine systematische Abnahmematrix ausgeführt/nachgewiesen. |
| Hotfix 008 | Neuer Missing-BBK-Spam tatsächlich beobachtet. |
| Hotfix 009 | Datei-/Commitnachweis; kein nachgereichter erfolgreicher Funk-/Lasttest. |
| `cargo test -p tetra-config` / `cargo test -p tetra-entities dual_carrier` | In den Arbeitsnotizen vorgeschlagen, keine erfolgreichen Testausgaben. |
| Am 03.10.2026 vorhandene Tests in den Quellen | Teilweise gesichtet, unter anderem Writer-/Allocator-Tests; nicht ausgeführt. |
| Am 03.10.2026 geprüfter Repository-Stand | Gepinnte Quelltext-/Branch-/Tagprüfung, kein Deployment- oder CI-Testbericht. |
| Vollständiger Erhalt aller Integrationen | Ziel und teilweise Codevorhandensein, keine vollständige Regression. |

Die wiederholten damaligen „fertig“, „stabil“ oder „Carrier hat bestanden“-Formulierungen sind keine zusätzliche Belegstufe. Insbesondere das Ende einer beobachteten Schleife darf nicht zur abgeschlossenen Zweiträger-Abnahme hochgestuft werden.

## 14. Verworfene und ersetzte Ansätze

| Früherer Ansatz | Späterer Stand / Grund |
|---|---|
| Nur Config und Dashboard ergänzen | Unvollständig; der Port betrifft mehrere Protokollschichten und Ressourcenmodelle. |
| Handgeschriebener Patch mit `--3way` als Allheilmittel | Benutzer konnte ihn nicht anwenden; Ersatzdateien wurden verwendet. |
| 419-MHz-Plan mit 760/761 | Durch Benutzer ausdrücklich auf 418 MHz, 720/721 korrigiert. |
| Center-Felder einfach weglassen | Nur Zwischenlösung; danach Parser und tatsächliches Tuning erweitert. |
| Generische 1,8-MS/s-Einstellung | Vom SoapySX-Treiber abgewiesen; 600 kS/s im gezeigten Aufbau. |
| Jede `[`-Zeile als TOML-Abschnitt | Beschädigt Arrays; echtere Header-Erkennung eingeführt, vollständige TOML-Robustheit weiter offen. |
| Nur bit-identische Ghosts entfernen | Reichte bei den gezeigten Empfangskopien nicht; Hamming-Heuristik. |
| Zusätzliche UI-Slots ohne Carrier-Zustand | Irreführende Belegung; Carrier-/TS-Schlüssel und später logisches Mapping. |
| Carrier-721-Log automatisch als harmless vor Dedupe werten | Ohne genaue Logstelle nicht begründet; Roh-, Drop- und Forward-Ereignisse unterscheiden. |
| `concurrent_multicarrier=false` als Verbot jeder Secondary-Nutzung | Fachlich zu pauschal; Zuweisung und Gleichzeitigkeit getrennt betrachten. |
| Secondary global nur `Tp`, kein `Cp` | Zu grob für zugewiesene Signalisierung; aktueller Code erlaubt Tp/Cp auf TS2–TS4. |
| Secondary vollständig Traffic-only ohne BCCH | Historischer Zwischenstand; am 03.10.2026 `SecondaryBcchNoMcch`, TS1 Control/Guard. |
| Leere Secondary-Slots nur aus TX-Liste streichen | Muss Zeitachse und Carrierbindung berücksichtigen; geprüfter DSP ergänzt Stille und Lookup. |
| Neue Produktversion aus internem v0.3.7 ableiten | Bestehender Produktstand v1.2.1; daraus v1.3.0-Vorschlag. |
| E-Mail-Privacy für Push ausschalten | Nicht erforderlich; eigene unveröffentlichte Commit-Metadaten sauber korrigieren. |

## 15. Offene Aufgaben, Ideen und Roadmap-Kandidaten

Die Prioritäten unten sind **redaktionelle Fortsetzungsempfehlungen** aus diesem Archiv. Bereits in den Arbeitsnotizen ausdrücklich vereinbart war: zunächst DualCarrier-Hardening und eindeutige Telemetrie, dann erst über einen dritten Carrier nachdenken. Kein Roadmap-Dokument außerhalb dieses Archivs wurde geändert.

### P0 — Reproduzierbarer Iststand und echte Zweiträger-Abnahme

1. Source-Commit, Buildfeatures, installierte Binary, `ExecStart` und lokale Konfiguration des Zielsystems zusammen erfassen. Keine Gleichsetzung von GitHub-main und laufendem Gerät ohne Prüfung.
2. Einen unabhängigen Ruf tatsächlich auf Secondary-Air-TS2–TS4 beziehungsweise geprüfte logische TS5–TS7 zwingen oder durch gezielte Belegung erreichen. Allocation, UL, DL, Empfang am zweiten MS und Release gemeinsam belegen.
3. Gleichzeitige verschiedene Rufe auf gleichem Air-TS verschiedener Carrier testen; Dedupe darf den zweiten echten Ruf nicht unterdrücken und Audio darf nicht zwischen Gesprächen wechseln.
4. Hangtime, gleicher/anderer Sprecher beim Retake, STCH/FACCH, Frame 18, Rufabbau, Timeout, Wiederregistrierung und Rückkehr zum MCCH auf beiden Carriern prüfen.
5. TX-Kontinuität, tatsächliches Spektrum und Signalqualität unter Last messen. Am 03.10.2026 geprüfte `TX continuity`-Zähler zusammen mit Air-/Endgerätelogs sichern.

**Abhängigkeit:** belastbarer aktueller Source-/Binary-Stand, passender Treiber, geeignete Funkmess-/Testumgebung und mindestens die für das Szenario erforderlichen Endgeräte. Erfolgskriterium sind nachvollziehbare Medien- und Steuerpfade, nicht allein zusätzliche Dashboardkacheln.

### P1 — Dedupe und Signalisierungsgrenzen präzisieren

- Dedupe nur mit explizitem Carrier-/Zeitkontext durchführen; gleiche Träger und nicht benachbarte Kandidaten nicht versehentlich als „adjacent“ behandeln.
- Rohbit-Heuristik gegen CRC-/PDU-/Ressourceninformation abwägen und echte gleichartige gleichzeitige Pakete als Gegenbeispiele testen.
- Gewünschte optionale härtere Filterung als dokumentierte, getestete Betriebsoption ausarbeiten; keine blind erhöhten Hamming-Schwellen.
- Drop-/Forward-/Discard-Zähler pro Carrier und Slot vorsehen; Empfangskandidaten, gültige MAC-Pakete und echte Call-Belegung getrennt anzeigen.
- Gründe für Rx-Spiegelungen beziehungsweise Nachbarkanalempfang untersuchen: Kanalfilter, Pegel, Aussteuerung, I/Q-/LO-Effekte und Treiberpfad. Keine Ursache ohne Messung festschreiben.
- Zulässigen Secondary-Uplink an zugewiesene Ressourcen und Protokollzustand binden. Eine Capability-basierte Richtlinie erst mit der richtigen Bedeutung von `concurrent_multicarrier` formulieren.

### P1 — Konfigurationswriter und Passbandvalidierung

- Array-Reproduktion als echter TOML-Roundtrip-Test, einschließlich Untertabellen, `[[...]]`, Inline-Kommentaren, mehrzeiligen Strings, fehlendem Schluss-Newline und mehrfachen Umschaltungen.
- Formatbewahrenden TOML-Editor oder robuste Parserlösung statt bloßer Zeilenheuristik erwägen.
- Backupfehler auswerten, atomar schreiben und konkurrierende Dashboard-/manuelle Änderungen schützen; nur validierte, konsistente Inhalte übernehmen.
- Passbandprüfung um volle belegte Kanalbreite und Reserve erweitern; finite/range-validierte Werte und die tatsächlich vom Treiber unterstützten RX-/TX-Raten berücksichtigen.
- PPM-korrigierte reale Center und Legacy-RX-Versatz konsistent mit dem Validator abgleichen. Die momentane reine Trägermittenprüfung ist kein vollständiger Bandbreitennachweis.

### P1 — Telemetrie, Ressourcen und Regression

- Für alle lokalen, Brew/Brew2-, Asterisk- und EchoLink-Rufpfade die korrekte Carrier-/logical-/air-TS-Zuordnung prüfen; alte Kommentare und Default-Carrier-Annahmen bereinigen.
- UI-Reconnect/Snapshot, Rufende, Sprecherwechsel und gleichzeitig gleiche Air-TS testen. Statische „AKTIV“-Controlrollen klar von gemessener Funkaktivität unterscheiden.
- Änderungen serialisierter Telemetrieereignisse auf Kompatibilität alter JSON-/Bitcode-Verbraucher prüfen; vorhandene Derives allein garantieren keine Protokollkompatibilität.
- SDS, Directory, Snom, Telegram, DAPNET/TPG und deaktivierte, aber zu erhaltende Integrationen regressionsprüfen. Directory-Verbindungsfehler gesondert klären.
- Historisch nicht ausreichende Allocator-/Call-Control-Portierung als erledigt auf **Codeebene** kennzeichnen, aber die geprüfte End-to-End-Abnahme offenlassen.

### P2 — Dritter Carrier / allgemeines Mehrträgermodell

Die Idee bleibt erhalten: zusätzlich etwa Carrier 719 mit DL 417,975 MHz und UL 407,975 MHz. Für 719/720/721 läge eine geometrische Mitte bei 418/408 MHz. Alternativ wurde 720/721/722 als denkbare Anordnung erwähnt. Keine der Varianten wurde aufgebaut oder freigegeben.

Dafür wären statt eines einzelnen `Option<u16>` beispielsweise `additional_carriers = [719, 721]` oder `secondary_carriers = [...]` denkbar. Beide Namen sind **Schemaideen**, keine am 03.10.2026 akzeptierten TOML-Schlüssel. Zu erweitern wären mindestens Config/Validierung, Ressourcen-ID-Modell, Scheduler, CMCE-/Bridge-Zuordnung, PHY-/DSP-Zeitachse, Dedupe, Telemetrie und UI.

Symmetrie um eine Center-Frequenz ist nur eine geometrische Eigenschaft, kein Beleg für die beste Hardwarekonfiguration; insbesondere liegen bei manchen Anordnungen Nutzträger auf der SDR-Mitte. Ein dritter Carrier bleibt von der erfolgreichen DualCarrier-Abnahme, Lastreserve und RF-Messung abhängig.

### P2 — Release- und Betriebsqualität

Reproduzierbare Builds, zielhardwarebezogene Tests, eindeutig installierbare Artefakte, dokumentierte Migration und Rückfallstand sollten einen nächsten Release begleiten. Interne Cargo-/Banner-Version, Produktversion, Tag und Release-Asset sind getrennt zu führen. Der tatsächliche `v1.3.0`-Tag bleibt historischer Anker, keine geprüfte Stabilitätsempfehlung.

## 16. Quellen, Referenzen, Anhänge und Lücken

### 16.1 Zitierbare Repository-Anker

Die Links sind auf die gelesenen Commits festgelegt, damit zukünftige Änderungen die Aussagen dieses Archivs nicht unbemerkt umdeuten. Zeilenangaben beziehen sich auf die jeweilige Quellfassung; Funktionsnamen sind zusätzlich angegeben.

- **[R1] Aktuelle Prüfstände:** [main-Commit 6aa9be8](https://github.com/JanHG98/netcore-tetra/commit/6aa9be8f74ab731f72dc133a5f8e90c5018c626d), [Archiving-Ausgangscommit 15c3f9f](https://github.com/JanHG98/netcore-tetra/commit/15c3f9f8dc8313dc6efdc2039ab86ad8719015ff). Die Auflösung des alten Forknamens wurde über die GitHub-Repository-API geprüft.
- **[R2] Historischer Releaseanker:** [Commit 7834f46](https://github.com/JanHG98/netcore-tetra/commit/7834f46748f3205ce6d3e6e1480345c6bdf27bca), [alter Allocator im Tagziel](https://github.com/JanHG98/netcore-tetra/blob/7834f46748f3205ce6d3e6e1480345c6bdf27bca/crates/tetra-core/src/timeslot_alloc.rs). Der Tag-Ref wurde gesondert über `git/ref/tags/v1.3.0` aufgelöst.
- **[R3] Am 03.10.2026 geprüfter Allocator:** [timeslot_alloc.rs](https://github.com/JanHG98/netcore-tetra/blob/6aa9be8f74ab731f72dc133a5f8e90c5018c626d/crates/tetra-core/src/timeslot_alloc.rs), insbesondere logische Zuordnung und `allocate_any_with_capacity`/`allocate_preferred_with_capacity`.
- **[R4] Am 03.10.2026 geprüfter UMAC:** [umac_bs.rs](https://github.com/JanHG98/netcore-tetra/blob/6aa9be8f74ab731f72dc133a5f8e90c5018c626d/crates/tetra-entities/src/umac/umac_bs.rs), gelesener Kernbereich Zeilen 40–270: Konstruktor, `SecondaryBcchNoMcch`, logische Übersetzungen und Carrier-Hints.
- **[R5] Am 03.10.2026 geprüfter PHY/Dedupe:** [phy_bs.rs](https://github.com/JanHG98/netcore-tetra/blob/6aa9be8f74ab731f72dc133a5f8e90c5018c626d/crates/tetra-entities/src/phy/phy_bs.rs), `rx_tpsap_prim`, Kandidatenvergleich und Weiterleitung, Zeilen 285–555.
- **[R6] Am 03.10.2026 geprüfter LMAC:** [lmac_bs.rs](https://github.com/JanHG98/netcore-tetra/blob/6aa9be8f74ab731f72dc133a5f8e90c5018c626d/crates/tetra-entities/src/lmac/lmac_bs.rs), `accepts_uplink`, `rx_tp_prim` und Batchbehandlung, gelesener Bereich 325–580.
- **[R7] Am 03.10.2026 geprüfter TX-DSP:** [soapy_dev.rs](https://github.com/JanHG98/netcore-tetra/blob/6aa9be8f74ab731f72dc133a5f8e90c5018c626d/crates/tetra-entities/src/phy/components/soapy_dev.rs), `TxDsp`, `modulate_tx_block`, `TX continuity`, gelesener Bereich 420–655.
- **[R8] Konfiguration:** [sec_phy_soapy.rs](https://github.com/JanHG98/netcore-tetra/blob/6aa9be8f74ab731f72dc133a5f8e90c5018c626d/crates/tetra-config/src/bluestation/sec_phy_soapy.rs) sowie [config.rs](https://github.com/JanHG98/netcore-tetra/blob/6aa9be8f74ab731f72dc133a5f8e90c5018c626d/crates/tetra-config/src/bluestation/config.rs), `bs_phase_mod_carriers`, `frequencies_fit_center`, `validate`, gelesener Kernbereich 175–345.
- **[R9] WebUI-Writer:** [dual_carrier.rs](https://github.com/JanHG98/netcore-tetra/blob/6aa9be8f74ab731f72dc133a5f8e90c5018c626d/crates/tetra-entities/src/net_dashboard/dual_carrier.rs), `table_name`, `compute_toml`, `write_dual_carrier`, gelesener Bereich 1–265.
- **[R10] UI im Archivbranch:** [html.rs als Asset-Loader](https://github.com/JanHG98/netcore-tetra/blob/15c3f9f8dc8313dc6efdc2039ab86ad8719015ff/crates/tetra-entities/src/net_dashboard/html.rs), [ui/dashboard.html](https://github.com/JanHG98/netcore-tetra/blob/15c3f9f8dc8313dc6efdc2039ab86ad8719015ff/crates/tetra-entities/src/net_dashboard/ui/dashboard.html), Ereignisse um 6200–6340 und TS-Visualizer um 6390–6590.
- **[R11] Telemetrietypen:** [net_telemetry/events.rs](https://github.com/JanHG98/netcore-tetra/blob/6aa9be8f74ab731f72dc133a5f8e90c5018c626d/crates/tetra-entities/src/net_telemetry/events.rs), `GroupCallStarted`, `IndividualCallStarted`, `TsVoiceActivity`, gelesener Bereich 1–200.
- **[R12] Ergänzende geprüfte Dokumentation:** [wiki/Dual-Carrier.md](https://github.com/JanHG98/netcore-tetra/blob/6aa9be8f74ab731f72dc133a5f8e90c5018c626d/wiki/Dual-Carrier.md). Der dortige Testplan ist kein Testergebnis; Wiki-Datei im Hauptrepository ist nicht automatisch Nachweis eines separaten GitHub-Wiki-Pushs.
- **[R13] Upstream-Metadaten:** [Upstream-HEAD 0f4faa9](https://github.com/razvanzeces/flowstation/commit/0f4faa98b1abe9ec295cf7a94a7a1fa4b3a869b8). Die dort genannte PR #46 ist ein Metadatenbefund des geprüften Upstreams, nicht der in dieser Entwicklungsphase nachgewiesene Portierungs-PR. Ein eigener PR für die damalige Hotfixfolge ist nicht belegt.
- **[G1] GitHub-E-Mail-Schutz:** [Blocking command-line pushes that expose your personal email address](https://docs.github.com/en/account-and-profile/how-tos/email-preferences/blocking-command-line-pushes-that-expose-your-personal-email-address), ergänzend am Prüftag konsultiert.
- **[G2] Git-Push-Semantik:** [git-push](https://git-scm.com/docs/git-push), ergänzend am Prüftag konsultiert.

### 16.2 Historische Log- und Konfigurationsanhänge

| Anhang | Umfang | Bedeutung |
|---|---:|---|
| `Eingefügter Text(28).txt` | 835 Zeilen | Historische Betriebs-Konfiguration; geheimnishaltig, nur selektiv ohne Zugangsdaten ausgewertet. |
| `Eingefügter Text(29).txt` | 83 Zeilen | Start um 12:02, noch nur Carrier 720; Default-Rate und Legacy-Center sichtbar. |
| `Eingefügter Text(30).txt` | 382 Zeilen | Start um 12:39 und doppelte Ruf-/ACK-/Fragmentverarbeitung; zwei Calls TS2/TS3. |
| `Eingefügter Text(31).txt` | 358 Zeilen | Fehler nach exaktem Dedupe, erneute doppelte Rufanlage um 12:50. |
| `Eingefügter Text(32).txt` | 38 Zeilen | Nach positiver Rückmeldung: C720/TS2 zugewiesen, weiterhin einzelne/parallele C721-PHY-Erkennungen. |

Direkt in Nachrichten stehen zusätzlich der Compilerfehlerblock, Unknown-Fields-/Fallback-Fehler, die 1,8-MS/s-Abweisung, die laufenden Missing-BBK-Meldungen nach Hotfix 008 und die Git-Push-Ausgaben. Diese Nachrichtenausschnitte sind Teil der historischen Belegbasis, besitzen aber keine eigene neue Rohlogdatei im Repository.

Die beiden Screenshots zeigen einmal die frühere Hauptträger-only-Timeslotanzeige trotz aktiviertem DualCarrier und später zwei Reihen mit `MCCH` beziehungsweise `BCCH` auf TS1. Die Abbildungen sind UI-Belege, keine RF-Messprotokolle. Sie werden nicht in dieses Git-Archiv kopiert.

### 16.3 Quellpakete und Hotfixartefakte

Die ZIPs wurden in den Arbeitsnotizen als komplette Ersatzdateien oder kumulative Source-Pakete bereitgestellt. Die Dateien sind für diese Nachprüfung zugänglich; sie werden hier **inventarisiert, nicht erneut als empfohlene Installation veröffentlicht**. Der Zusatz `nosensitive` ist ein damaliger Paketname, keine nachträgliche vollständige Geheimnis-/Sicherheitszertifizierung des gesamten Inhalts.

| Artefakt | Dateien im ZIP | SHA-256 |
|---|---:|---|
| `flowstation-centerfreq-hotfix-002.zip` | 5 | `6b0728553c81ef7ff1cbc0b5bc17f3f4e62ca7a592889b5a524dfa2d93b5a7f8` |
| `flowstation-dashboard-dualcarrier-timeslots-hotfix-004.zip` | 2 | `874550f7aa1795ad7b404333d26666c26860bdd0540d60d96ecf19b42925c880` |
| `flowstation-dualcarrier-empty-slot-hotfix-009.zip` | 1 | `d0c50a93a2fc81d4737a76de595cb847a435f4300fdb334649201c6dd3507dd2` |
| `flowstation-dualcarrier-fixed-files.zip` | 9 | `c5fbc48287a82a6ee53f1bd04f9f7be707f4b0e85a727d1959272bd48f557ee1` |
| `flowstation-dualcarrier-hardening-hotfix-007.zip` | 11 | `14df2fdb251d2c2e8ed3e8e127f45da5fec160f0fda991d89b60a1f64278de32` |
| `flowstation-dualcarrier-hotfix-001.zip` | 2 | `327f9e1d28e43dcf6408f5a77ece0bd60cb3b8355a18d44b1684dcf9c5692aa4` |
| `flowstation-dualcarrier-rx-dedup-hotfix-005.zip` | 1 | `60630cc1ef010fe687f855552e99085c8f482203c7a81e2889d47034d80faf68` |
| `flowstation-dualcarrier-rx-hamming-dedup-hotfix-006.zip` | 2 | `add95400d0ce4c778c3f89799808afee39764db0ad1d21ccc1ace78127f9ad30` |
| `flowstation-dualcarrier-stage2-changed-files.zip` | 19 | `8e405732dad8b43e6de8507d34ef1615491914f4a8a3911fbc3ddccb2af9b6a1` |
| `flowstation-dualcarrier-trafficonly-hotfix-008.zip` | 3 | `20807b7c6835dcc4276047240ebad85d7bf0a15c7629764e162656d6c8f0dda1` |
| `flowstation-dualcarrier-webui-array-hotfix-003.zip` | 2 | `6f1011ff8e5dd0605a52b43730bb57cf87474a9131789797d7c5ddbbeec17e0d` |

Kumulative beziehungsweise ursprüngliche Source-Pakete im zugänglichen Verlauf:

```text
flowstation-main(1).zip
flowstation-main-dualcarrier-hotfix-001.zip
flowstation-main-dualcarrier-hotfix-002-nosensitive.zip
flowstation-main-dualcarrier-hotfix-003-nosensitive.zip
flowstation-main-dualcarrier-hotfix-004-nosensitive.zip
flowstation-main-dualcarrier-hotfix-005-nosensitive.zip
flowstation-main-dualcarrier-hotfix-006-nosensitive.zip
flowstation-main-dualcarrier-hotfix-007-nosensitive.zip
flowstation-main-dualcarrier-hotfix-008-nosensitive.zip
flowstation-main-dualcarrier-hotfix-009-nosensitive.zip
flowstation-main-dualcarrier.zip
```

Prüfanker des hier genauer untersuchten historischen Gesamtstands: `flowstation-main-dualcarrier-hotfix-009-nosensitive.zip`, 431 Dateien, SHA-256 `988fdff9b91ce67bc8a5573009e62a0d855d598b38f14fc9c741fa16ebf4f552`. Die Hashes identifizieren die am 03.10.2026 zugänglichen Artefaktbytes; sie beweisen keine damalige Installation.

Zusätzlich waren die ursprünglichen Einzeldateien `config.rs`, `default_stack.rs`, `html(3).rs`, `mod(1).rs`, `sec_cell.rs`, `server(3).rs`, `soapy_dev.rs` und `umac_bs.rs` sowie der nicht erfolgreich angewendete `flowstation-dualcarrier-netcore.patch` verfügbar. Die ursprünglich vorgeschlagenen Namen `flowstation-dualcarrier-input.tgz` und `flowstation-dualcarrier-stage2-input.tgz` sind Verpackungs-/Anforderungsnamen aus dem Dialog; sie werden nicht als nachgewiesene zusätzliche Archivbytes ausgegeben.

### 16.4 ETSI-Unterlagen

**[E1] Für die geprüfte begrenzte Klarstellung gezielt benutzt:** `en_30039202v030801p.pdf`, ETSI EN 300 392-2 V3.8.1 (2016-08), 1445 PDF-Seiten; insbesondere gedruckte Seite 71 sowie 928–929 zu MCCH/Übertragungsmodi und Kanalzuweisung/Capabilities. Diese Fassung ist ein bereitgestellter Quellenstand; ihr Status als neueste Norm wurde nicht behauptet.

Als weiterer Projekt-Referenzbestand verfügbar, aber für diesen Dokumentation **nicht vollständig inhaltlich neu geprüft**:

```text
en_30039201v010601p.pdf
en_3003920308v010401p.pdf
en_3003920304v010301p.pdf
en_3003920303v010301p.pdf
en_3003920313v010201p.pdf
en_3003920315v010500a.pdf
en_30039205v020701p.pdf
en_30039207v030501p.pdf
en_30039209v010701p.pdf
en_3003921006v010401p.pdf
en_3003921018v010301p.pdf
en_3003921101v010201p.pdf
en_3003921114v010101p.pdf
en_3003921117v010102p.pdf
en_3003921201v010202p.pdf
en_3003921216v010400a.pdf
ets_30039214e01v.pdf
en_30039401v030301p.pdf
en_30039502v010303p.pdf
en_300812v020101p.pdf
ts_10081201v020205p.pdf
es_20081201v020205p.pdf
es_20081202v020401m.pdf
ETSI.pdf
```

Der Bestand enthält unter anderem Air-Interface-, Codec-, RF-Test-, ISI-, Supplementary-Service- und SIM/UICC-Dokumente sowie Entwurfsfassungen. `ETSI.pdf` ist ein umfangreicher Sammelbestand mit 4100 Seiten, keine einzelne zusätzliche einheitliche Normfassung. Das Vorhandensein dieser PDFs bedeutet nicht, dass die historischen Hotfixes gegen sämtliche Dokumente geprüft wurden.

### 16.5 Verbleibende Offene Nachweise

1. Einzelne frühe Diagnoseausgaben und Arbeitsschritte sind nicht erhalten.
2. Keine vollständigen erfolgreichen Cargo-/CI-Protokolle für jeden Hotfix und keine durchgängige Zuordnung von lokalem Git-HEAD, gebauter und installierter Binary.
3. Keine dokumentierte unabhängige C721-Sprachverbindung, keine echte gleichzeitige Zweiträger-Lastabnahme, keine IQ-/Spektrums-/RF-Konformitätsmessung.
4. Kein ausdrücklich nachgereichter erfolgreicher Betriebslog nach Hotfix 009. Die Tag-Existenz wurde erst in der Quellenprüfung unabhängig geprüft.
5. Der historische letzte Push-Ausschnitt ist unvollständig; geprüfte Remote-Belege ersetzen keinen damaligen vollständigen lokalen Dateiabgleich.
6. Der geprüfte Audit ist auf die beschriebenen Codepfade und Referenzstellen begrenzt. Er bestätigt keine vollständige Regression, keinen aktuellen Gerätebetrieb und keine vollständige Auswertung aller ETSI-PDFs.
7. Historische ZIPs sind in dieser Sitzung lesbar, aber nicht Bestandteil der beiden hier neu abgelegten Git-Dateien. Für eine langfristige Aufbewahrung der Originalartefakte wäre ein gesonderter, auf Geheimnisse geprüfter Auftrag nötig.

## 17. Abschluss und Wiederaufnahme

Die Notizen dokumentieren einen frühen, iterativen DualCarrier-Port mit nachvollziehbaren Fehlversuchen und einer bestätigten Verbesserung gegen die beobachtete Gesprächsschleife. Er darf nicht als Nachweis gelesen werden, dass der alte Stand bereits jede Zweiträgerfunktion vollständig implementierte oder abgenommen war.

Für eine Fortsetzung ist **der am 03.10.2026 überprüfte beziehungsweise dann erneut zu prüfende Repository-Code** die Ausgangsbasis, nicht das letzte historische ZIP. Zuerst Source/Binary/Config eindeutig zuordnen, anschließend Secondary-Gespräch und gleichzeitige Nutzung nachweisen, dann Dedupe-, Signalisierungs-, Writer- und RF-Grenzen gezielt härten. Ein dritter Carrier bleibt ein nachgelagerter Ausbaukandidat.
