# Brainstorming: Virtuelle TBS, Funkgeräte und SimAir-Testlabor

## 1. Rahmen und Aussagegrenzen

| Merkmal | Wert |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Vollwertige virtuelle TETRA-Basisstationen und Funkgeräte mit Web-MMI, Positionssimulation und Mehrzellen-/Call-Restore-Tests |
| Historischer Entwurfsstand | 2026-09-11 |
| Notizstand | 2026-10-05, Europe/Berlin |
| Historisch referenziertes Repository/Branch | JanHG98/netcore-tetra, Branch mqtt |
| Historischer Anhang | netcore-tetra-mqtt.zip |
| SHA-256 des ausgewerteten ZIP-Anhangs | 150bdeb4622ee3e7a15bb817ef354228d6e0f0546c9ad2e5b79feeb612bd53d4 |
| ZIP-Git-Metadaten | Keine .git-Historie enthalten; aus dem ZIP allein ist daher kein exakter historischer Commit ableitbar. |
| Am Prüfstand 05.10.2026 geprüfter Runtime-Branch | main |
| Am Prüfstand 05.10.2026 geprüfter main-Commit | 9116c15d645458f99e236712b67a1ad970432791 |
| Vor dem Schreiben geprüfter Archiving-Commit | 1e264140796a4b1875ecf6f75aa088669f6d8108 |
| Vor dem Schreiben geprüfter Archiving-Tree | 0c9bab792d26add85b2f82bfdd2a4691fe44d45f |
| Ablage | Docs/archive/2026-10-05_virtuelle-tbs-funkgeraete-simair-handover-testlabor.md |

### 1.1 Arbeitsgrundlage

Der historische Stand stammt aus `netcore-tetra-mqtt.zip`: 2.586 Einträge einschließlich `ms-mode/`, Mobility-/Call-Control-Dokumenten und Zwei-Zellen-Tests. Das ZIP enthält keine Git-Historie und kann keinem exakten historischen Commit zugeordnet werden.

Der Branch `mqtt` war am **2026-10-05** nicht mehr auffindbar. Historischer ZIP-Stand und `main@9116c15d645458f99e236712b67a1ad970432791` werden daher getrennt bewertet.

Relevante geprüfte Dateien wurden gegen main@9116c15d645458f99e236712b67a1ad970432791 geprüft. Dazu gehören insbesondere:

- ms-mode/docs/MS_MODE.md
- ms-mode/crates/tetra-pdus/src/phy/traits/rxtx_dev.rs
- crates/tetra-pdus/src/phy/traits/rxtx_dev.rs
- crates/tetra-entities/src/cmce/subentities/sds_bs.rs
- crates/tetra-entities/tests/test_two_cell_call_restore.rs
- system-backend/mobility-core/docs/context-transfer.md
- system-backend/call-control/docs/call-restore.md
- misc/brew-server/src/position.rs
- Cargo.toml
- bins/bluestation-bs/src/main.rs
- Docs/archive/2026-10-05_android-tetra-modul-virtueller-airlink-und-realtime-core.md

### 1.2 Offene Nachweise

Eigenständige Bilder des Simulationsentwurfs liegen nicht vor. Projektgrafiken im ZIP sind allgemeine Quellen, keine Belege eines laufenden Testlabors.

Bei der Bestandsaufnahme wurden keine Rust-Builds, Unit-/Integrationstests, RF-Messungen, WebUI-Interaktionen oder Live-Handover ausgeführt. Hinweise auf Tests und Hardwarevalidierung stammen aus Repository-Dokumentation oder vorhandenem Testcode; sie belegen keine vollständige Simulatorabnahme.

---

## 2. Statusmodell

Diese Dokumentation verwendet die folgenden Belegstufen strikt:

| Status | Bedeutung |
|---|---|
| Idee | Erwogene Funktion oder Architektur ohne verbindliche Umsetzung. |
| Beschlossen/geplant | Als Ziel gewünscht oder in einer späteren Projektplanung ausdrücklich als Zielarchitektur festgelegt; noch kein Implementierungsnachweis. |
| Implementiert | Quellcode oder eingecheckte Konfiguration im geprüften Repository vorhanden. |
| Getestet | Ein konkreter Test ist im Repository vorhanden oder im Projekt dokumentiert. „Test vorhanden“ bedeutet nicht, dass er bei der Bestandsaufnahme ausgeführt wurde. |
| Im Betrieb bestätigt | Ein realer RF-/Hardware-/Livebetrieb ist explizit dokumentiert. Auch das wurde bei der Bestandsaufnahme nicht erneut nachgestellt. |
| Überholt/präzisiert | Ein früherer Vorschlag wurde durch spätere Projektentscheidungen oder den geprüften Repository-Befund ersetzt beziehungsweise enger gefasst. |

## 3. Ziel und Ausgangslage des Arbeitsstands

Ziel sind virtuelle Basisstationen und Funkgeräte für realistische Ende-zu-Ende-Szenarien mit NetCore-Tetra. Eine reine Dashboard-Simulation genügt dafür nicht. Für virtuelle Funkgeräte war ausdrücklich eine Weboberfläche mit PTT, Display, D-Pad und möglichst vollständiger Bedienbarkeit gewünscht.

Als Zieltests wurden insbesondere genannt:

- Positions- und Ortungsdaten,
- Bewegung von Teilnehmern,
- Zellwechsel,
- Übergabe beziehungsweise Wiederherstellung laufender Gespräche zwischen TBS,
- später möglichst umfassende Nutzung der virtuellen Funkgeräte wie eines realen Endgeräts.

Die Machbarkeitseinschätzung ist positiv: Im historischen `mqtt`-Stand bestanden bereits MS-Mode, getrennte Core-Dienste und Mehrzellen-/Restore-Bausteine. Der Simulator ist bisher ein technischer Entwurf; eine Implementierung war noch nicht beauftragt.

## 4. Zentrale fachliche Entscheidung des Arbeitsstands

Die wichtigste Empfehlung war, keine „Fake-Funkgeräte“ zu bauen, die über spezielle Test-REST-Endpunkte direkt Zustände in NetCore schreiben. Stattdessen sollen virtuelle Teilnehmer und virtuelle Basisstationen dieselben Protokollzustände und möglichst dieselben Stack-Komponenten nutzen wie reale Geräte.

Historischer Architekturgrundsatz:

> Virtualisiert wird die Transport-/Luftschnittstellenkante. MLE, MM, CMCE, LLC und MAC sollen nicht durch Test-Shortcuts umgangen werden.

Damit sollte ein virtueller Teilnehmer beispielsweise eine Registrierung als echte U-LOCATION-UPDATE-DEMAND-/Downlink-Antwort-Sequenz durchlaufen, statt direkt als „registered=true“ in einer Datenbank angelegt zu werden. Dasselbe gilt für Gruppenaffiliation, PTT/Floor, SDS, LIP und Call Restore.

Dieser Grundsatz bleibt weiterhin sinnvoll und wird durch die geprüfte Codebasis gestützt.

## 5. Historischer Architekturvorschlag: VirtualRxTxDev und SimAir

### 5.1 Ursprünglicher Vorschlag

Ansatz: ein zusätzlicher virtueller PHY-Pfad, sinngemäß:

~~~text
PhyBackend::SoapySdr
PhyBackend::Virtual
~~~

Dazu wurden folgende logische Komponenten vorgeschlagen:

- VirtualRxTxDev
- SimAir
- SimEndpoint
- SimCell
- SimSubscriber

SimAir sollte keine abstrakten „Call started“-Events austauschen, sondern auf Slot-/Burstebene arbeiten. Der vorhandene RxTxDev-Vertrag war dafür als Integrationskante vorgesehen.

Im historischen ZIP existiert der Trait RxTxDev bereits mit rxtx_timeslot(). Im aktuellen main existiert derselbe Grundvertrag im Root-Workspace weiterhin; der separate ms-mode-Unterbaum besitzt zusätzlich MS-spezifische Hooks für RF-Pfad, TX-Air-Time, RSSI und Laufzeit-Retuning.

### 5.2 Empfohlener Datenvertrag

Für eine deterministische Simulation sollte ein virtueller Slot mindestens folgende Informationen tragen:

- Richtung beziehungsweise Quell-/Zielendpunkt,
- Carrier oder Carrier-Nummer,
- TdmaTime,
- TrainingSequence,
- Full-slot-/Subslot-Bits,
- simulierte Empfangsfeldstärke/RSSI,
- optional Fehler-/Loss-/Delay-Metadaten.

Die reale Protokolldekodierung und -zustandsmaschine sollte oberhalb dieser Kante weiterlaufen.

### 5.3 Zeitmodell

Der MS-Mode dokumentiert einen wesentlichen Unterschied:

- Eine Basisstation ist sendeseitig Timing-Master.
- Eine Mobile Station wird aus dem empfangenen Downlink getaktet und gewinnt TdmaTime aus der Synchronisation zurück.
- Uplink erfolgt zeitlich passend zur Downlink-Referenz; im dokumentierten MS-Pfad gilt unter anderem die feste UL/DL-Slotrelation.

Eine virtuelle Luftschnittstelle darf deshalb nicht nur Nachrichten „irgendwann“ zustellen. Für reproduzierbare Tests braucht sie ein explizites Simulationszeitmodell. Empfohlen ist ein zentraler SimClock beziehungsweise eine deterministische TDMA-Ereignisschleife, die Slots in definierter Reihenfolge verteilt.

Ein möglicher Ablauf:

~~~text
SimClock tick
  -> TBS erzeugt Downlink-Slot
  -> SimAir verteilt ihn an erreichbare MS-Endpunkte
  -> MS dekodiert und aktualisiert Zustand
  -> MS plant zulässigen Uplink
  -> SimAir liefert Uplink an passende TBS
  -> TBS verarbeitet den realen Stackpfad
~~~

Für reine CI-Tests kann ein VirtualRxTxDev als Adapter auf SimAir sinnvoll bleiben.

## 6. Spätere Präzisierung: nicht nur exklusives Virtual-Backend

Der historische Vorschlag „SoapySdr ODER Virtual“ wurde in einem späteren, am Prüfstand 05.10.2026 bereits archivierten Projektentwurf präzisiert. Die dort festgehaltene Zielarchitektur verlangt einen virtuellen Airlink parallel zum RF-Pfad und trennt ihn vom netzweiten Realtime-Control-/Media-Pfad.

Daraus folgt für eine geprüfte Fortsetzung:

- Ein exklusives PhyBackend::Virtual ist weiterhin nützlich für vollständig offline laufende CI-/Laborsimulation.
- Für die langfristige Produktarchitektur sollte der virtuelle Airlink zusätzlich parallel zu SoapySDR funktionieren können.
- Eine reale TBS darf dann gleichzeitig echte RF-Teilnehmer und virtuelle Teilnehmer sehen.
- Der Airlink darf nicht mit der TBS-zu-TBS-Call-/Media-Verteilung vermischt werden.

Der verwandte Archivstand ist dokumentiert unter:

[Android-TETRA-Modul, virtueller Airlink und Realtime-Core](2026-10-05_android-tetra-modul-virtueller-airlink-und-realtime-core.md)

Damit ist der ursprüngliche Either-or-Backend-Vorschlag **überholt/präzisiert**, nicht die Idee der Slot-/Burst-basierten Virtualisierung selbst.

## 7. Virtuelles Funkgerät und Web-MMI

### 7.1 Bedienkonzept

Für das virtuelle Funkgerät wurde eine echte MMI statt eines Entwicklerformulars vorgeschlagen. Der historische Entwurf umfasste:

- Display,
- PTT,
- D-Pad,
- Softkeys,
- grüne/rote Gesprächstaste,
- Nummerntastatur,
- Lautstärke,
- Emergency-Taste,
- Gruppen- und Ordnerauswahl,
- Kontakte,
- SDS-Postfach,
- Statusmeldungen,
- Engineering-Overlay.

Das Engineering-Overlay sollte unter anderem MCC, MNC, LA, Carrier, Colour Code, RSSI, Registrierungszustand, Call-ID, Timeslot und Floor State anzeigen.

### 7.2 Schnittstelle zum Stack

Die UI soll nicht eigene Call- oder Registrierungslogik erfinden. Sie soll die vorhandenen Control-/Telemetry-SAPs nutzen. Der ms-mode-Unterbaum dokumentiert dafür bereits WebSocket + JSON + TLS + argon2 sowie TNMM-, TNSDS- und Call-Control-Kommandos.

Der aktuelle ms-mode-Stand beschreibt insbesondere:

- TNMM Registration/Deregistration,
- Group Attach/Detach,
- TNSDS Unitdata/Status,
- SDS-TL Delivery-/Read-Reports,
- Tncc Setup/Tx/DTMF,
- manuelle Zellwahl und Cell Scan,
- Telemetrie für Scanresultate,
- Sprachframes zwischen Stack und UI.

### 7.3 Audio

Der MS-Mode legt den ACELP-Vocoder bewusst außerhalb des Stacks. Downlink-TCH/S wird als Sprachframe an die UI gemeldet; Uplink-Sprache kommt von der UI zurück.

Für eine Browser-MMI passt deshalb:

~~~text
Browser-Mikrofon
  -> WebAudio/PCM
  -> ACELP-Vocoder im UI-/Companion-Prozess
  -> MS-SpeechFrame
  -> echter MS-Stack
  -> virtuelle oder reale Luftschnittstelle
~~~

Im frühen Entwurf war die Radio-WebUI eine noch zu bauende Komponente. Der geprüfte ms-mode-Text verweist jedoch bereits auf das externe Companion-Projekt misadeks/tetra-tn-web-ui für portable-radio UI, Codeplug, Call Control und ACELP. Für eine Fortsetzung sollte daher zuerst geprüft werden, welche MMI-Funktionen dort bereits vorhanden sind, statt eine zweite UI von Null zu entwickeln.

## 8. Aktueller MS-Mode-Befund

### 8.1 Historischer und geprüfter Funktionsstand

Die Datei ms-mode/docs/MS_MODE.md liegt am Prüfstand 05.10.2026 unverändert mit demselben Blob-SHA wie im geprüften historischen Stand vor. Sie dokumentiert unter anderem:

**Im Repository als hardware-validiert markiert:**

- Downlink-Synchronisation,
- RX-getriebene TDMA-Zeit,
- BSCH/MAC-SYNC und Scrambling-Ableitung,
- Uplink-PHY,
- Runtime-Uplink-Retuning,
- initiale Zellwahl,
- ITSI Attach Registration,
- Group Affiliation,
- Group Call mit Floor in beide Richtungen,
- Individual-/Duplex-Call,
- STCH Talker Identity,
- Uplink/Downlink TCH/S.

**Als software-getestet markiert:**

- Radio-Link-Failure-Grundpfad,
- manuelle Cell Survey,
- CampOnCell/Register-to-cell,
- Teile von Deregistration und Reject Handling,
- SDS/Status und SDS-TL,
- DTMF,
- Control-/Telemetry-WebSocket,
- weitere Management-Funktionen.

**Explizit deferred beziehungsweise out of scope:**

- Neighbor Scanning und vollständige Multi-Cell Reselection,
- vollständige RLF-Measurement-Infrastruktur,
- Authentication/OTAR/Air-Interface-Encryption im MS-Mode,
- DMO,
- Packet Data im damaligen MS-Mode,
- in-stack ACELP.

### 8.2 Wichtige Integrationsnuance des geprüften main

Der aktuelle Top-Level-Cargo-Workspace listet ms-mode/ **nicht** als Workspace-Member. Er baut die Root-Pakete unter crates/, bins/ und system-backend/. Der Unterbaum ms-mode/ ist ein eigener vollständiger Stand mit eigenen Cargo-Dateien.

Gleichzeitig sind Teile von MS-Unterstützung inzwischen auch im Root-Baum sichtbar, beispielsweise StackMode::Ms in Konfiguration/Tests und MS-bezogene UMAC/MLE-Pfade. Der top-level bins/bluestation-bs/src/main.rs enthält im geprüften Stand jedoch keinen build_ms_stack-/StackMode::Ms-Startpfad; der vollständige dokumentierte MS-Runtime-Aufbau liegt im separaten ms-mode-Unterbaum.

Daraus folgt ein P0-Architekturpunkt: Vor der Simulatorimplementierung muss festgelegt werden, welcher MS-Stack der führende Integrationsstand ist und wie er in den Hauptworkspace konsolidiert wird. Andernfalls drohen zwei auseinanderlaufende Implementierungen.

## 9. Virtuelle Basisstation

Architekturprinzip: keine eigenständige „Fake TBS“ mit abweichender Logik. Eine virtuelle TBS soll möglichst dieselbe TBS-Implementierung verwenden wie eine physische TBS und lediglich den Luftschnittstellenadapter austauschen beziehungsweise um einen virtuellen Airlink ergänzen.

Das ist weiterhin die bevorzugte Architektur:

~~~text
gleiche TBS-Logik
  -> MLE/MM/CMCE/LLC/MAC identisch
  -> PHY-/Airlink-Kante austauschbar oder parallel
      - SoapySDR
      - Virtual Airlink
~~~

So werden dieselben Protokollpfade, Ressourcenvergaben, Floor-Regeln und Restore-Abläufe geprüft, statt nur Backend-APIs zu testen.

## 10. Position, Ortung und Bewegungsmodell

### 10.1 Gewünschtes Teilnehmermodell

Ein virtueller Teilnehmer kann mindestens folgende Simulationsdaten erhalten:

- ISSI,
- latitude/longitude,
- Geschwindigkeit,
- Kurs/Heading,
- Fix-Status,
- Accuracy,
- Route/Waypoint-Zustand.

Die Position kann manuell auf einer Karte verschoben oder entlang einer GPX-/Szenarioroute gefahren werden.

### 10.2 Position soll das Funkmodell beeinflussen

Die Position ist nicht nur für die Karte gedacht. Sie soll zugleich die Empfangsbedingungen je Zelle bestimmen. Für einen ersten sinnvollen Simulator reicht ein einfaches deterministisches Link-Budget-Modell:

~~~text
RSSI(TBS_i, MS_j) =
  tx_reference
  - path_loss(distance)
  + optional shadowing
  + scenario offsets
~~~

Zusätzliche Effekte können später zugeschaltet werden:

- harte Coverage-Grenzen,
- Fading,
- Paket-/Burstverlust,
- künstliche Interferenz,
- TBS-Ausfall,
- verzögerte Frames,
- definierte Hysterese.

Für reproduzierbare CI-Tests sollte Zufall nur mit festem Seed verwendet werden.

### 10.3 LIP statt direktem Datenbank-Write

Der historische Entwurf empfahl, simulierte Positionen als echte TETRA-LIP-SDS zu senden und sie nicht direkt in eine Control-Room-Datenbank einzutragen.

Der geprüfte Root-Code bestätigt, dass LIP-Payloads bereits dekodiert werden können. In crates/tetra-entities/src/cmce/subentities/sds_bs.rs wird PID 0x0A erkannt und eine plausible WGS84-Position dekodiert. Zusätzlich enthält misc/brew-server/src/position.rs Decoder für kurze und Motorola-artige lange LIP-Berichte.

Eine repositoryweite Suche fand dagegen keinen eindeutig benannten encode_lip_position-Pfad. Damit gilt:

- LIP-Dekodierung: **implementiert**.
- Ein sauberer generischer LIP-Encoder für den virtuellen MS: **nicht als eigener klarer Baustein nachgewiesen**.
- End-to-End-Weitergabe binärer LIP-Daten besitzt in vorhandenen Kommentaren historisch unterschiedliche Annahmen und muss vor Nutzung im Simulator einmal praktisch verifiziert werden.

Roadmap-Folge: LIP-Encoding als MS-Funktion implementieren oder vorhandene SDS-Bitcodec-Bausteine dafür erweitern und anschließend vom virtuellen GPS bis zur Control-Room-Karte testen.

## 11. Mehrzellenbetrieb, Mobility und Call Restore

### 11.1 Mobility Context Transfer

Der geprüfte Mobility-Core dokumentiert einen dreistufigen Transfer:

1. MobilityExportContext auf der Quell-TBS,
2. MobilityImportContext auf der Ziel-TBS,
3. MobilityRemoveContext auf der Quell-TBS.

Übertragen werden derzeit MM-Daten wie Home-ISSI, Registrierungszustand, Gruppen, Energy-Saving-Mode/Monitoring Window, Class of MS, Layer-2-Handle und TEI.

Die Quelle wird erst nach bestätigtem Import entfernt. Aktive CMCE-Calls werden nicht als MM-Kontext transportiert, sondern über die Call-Restore-Logik behandelt.

Status: **implementiert/dokumentiert**, bei der Bestandsaufnahme nicht live ausgeführt.

### 11.2 Call Restore

system-backend/call-control/docs/call-restore.md dokumentiert bereits einen netzweiten Restore-Ablauf mit den Zuständen:

- ExportQueued
- ExportRequested
- ImportQueued
- ImportRequested
- Ready
- Completed
- Cancelled
- Failed
- TimedOut

Ein Platzhalter-Leg auf der Ziel-TBS verhindert einen zweiten logischen Call; abgeschlossen wird erst bei beobachtetem echtem Ziel-Leg.

### 11.3 Vorhandene Zwei-Zellen-Tests

crates/tetra-entities/tests/test_two_cell_call_restore.rs enthält aktuell Tests für unter anderem:

- laufenden Gruppenruf mit Floor-/Priority-Kontext auf Zielzelle,
- Individual-Simplex-Restore ohne Floor-Diebstahl,
- Call-ID-Kollision und Remapping,
- überlastete Zielzelle mit Queue statt Channel Allocation,
- idempotenten Replay desselben Restore,
- Listener-Restore während ein anderer Teilnehmer spricht,
- Duplex-Individual-Restore.

Damit existiert bereits eine gute Protokollbasis für einen späteren echten SimAir-End-to-End-Test.

### 11.4 Noch fehlende Medienkontinuität

Die Call-Restore-Dokumentation sagt ausdrücklich, dass Media Frames in diesem Paket noch nicht zwischen Zellen transportiert werden; das soll der Media Switch übernehmen.

Das ist für das Ziel „laufendes Gespräch beim Zellwechsel“ der wichtigste verbleibende Integrationspunkt. Ein Restore kann signalisierungsseitig korrekt sein und trotzdem eine hörbare Lücke erzeugen.

Deshalb sollte der Simulator messen:

- Signalling-Handover-Latenz,
- Media-Gap,
- verlorene/duplizierte Sprachframes,
- Logical-Call-ID-Kontinuität,
- Floor-Holder-Kontinuität,
- Route-/Leg-Generation,
- Restore-Fehler und Timeouts.

## 12. Automatische Cell Reselection

Dies ist die zentrale funktionale Lücke für einen realistischen bewegten Teilnehmer.

Der ms-mode-Stand bietet:

- initiale Zellwahl,
- manuelle Cell Survey,
- CampOnCell,
- manuelle Registrierung auf gewählter Zelle,
- grundlegende RLF-Erkennung.

Neighbor Scanning, vollständige C1/C2/C3-basierte Überwachung und Multi-Cell Reselection sind dagegen explizit als deferred markiert.

Damit kann eine erste Mehrzellen-Simulation bereits deterministisch mit manuellem Zellwechsel gebaut werden. Für einen realistischen „Fahrzeug fährt von Zelle A nach B“-Test muss danach die automatische Reselection vervollständigt werden.

## 13. Vorschlag für einen stufenweisen Simulatoraufbau

### Phase A — Offline-Simulationskern

**Beschlossen/geplant als Roadmap-Kandidat, noch nicht implementiert:**

- gemeinsamer SimClock,
- SimAir/Airlink-Transport auf Slot-/Burstebene,
- VirtualRxTxDev-Adapter,
- eine TBS + ein MS,
- Registrierung,
- Group Attach,
- SDS,
- Group Call/PTT,
- deterministische Logs/Trace.

Akzeptanzkriterium: derselbe MLE/MM/CMCE-Pfad läuft wie bei realer RF-Nutzung; keine direkten Test-Hintertüren in Subscriber-/Call-State.

### Phase B — Web-MMI

- vorhandene ms-mode-Control-/Telemetry-Schnittstelle nutzen,
- vorhandenes Companion-UI zuerst evaluieren,
- Display/D-Pad/PTT/Softkeys/SDS/Gruppen ergänzen,
- WebAudio + ACELP anbinden,
- Engineering-Overlay,
- mehrere virtuelle Geräte parallel in Tabs/Frames.

Akzeptanzkriterium: ein Nutzer kann einen virtuellen Teilnehmer ohne Entwicklerkonsole vollständig registrieren, Gruppe wählen, PTT nutzen, SDS senden/empfangen und Zustände beobachten.

### Phase C — Zwei Zellen, zunächst manuell

- zwei TBS mit unterschiedlichen Zellparametern,
- definierte Serving Cell,
- manuelles CampOnCell oder erzwungener RF-Level-Wechsel,
- Mobility Context Transfer,
- Call Restore,
- Assertions für Logical Call und Floor.

Akzeptanzkriterium: Gruppen-/Individual-Call bleibt logisch erhalten und Ziel-Leg wird korrekt übernommen.

### Phase D — Position und automatisches Funkmodell

- Karte/Route,
- RSSI je Zelle aus Position,
- Neighbor Measurements,
- Hysterese,
- automatische Reselection,
- LIP-SDS-Erzeugung.

Akzeptanzkriterium: Zellwechsel entsteht aus Funkbedingungen, nicht aus einem „handover“-Testknopf.

### Phase E — Media-Handover

- Media-Switch-Weiterleitung,
- Umschalten auf neues Leg,
- Gap-/Loss-Messung,
- Fehlerfälle und Recovery.

Akzeptanzkriterium: definierte maximale Sprachunterbrechung und keine Doppelwiedergabe.

### Phase F — Scenario Runner und Skalierung

- deklarative Szenarien,
- 10/50/100+ virtuelle MS,
- mehrere TBS,
- TBS-Ausfälle,
- Loss/Delay/Interferenz,
- reproduzierbare Seeds,
- maschinenlesbare Testreports.

## 14. Beispiel eines Szenariorunners

Der folgende Entwurf stammt konzeptionell aus den Entwicklungsnotizen und ist **nicht implementiert**:

~~~yaml
scenario: moving_group_call

tbs:
  - id: tbs-a
    position: [52.39, 9.61]
  - id: tbs-b
    position: [52.40, 9.68]

radios:
  - issi: 4010001
    group: 12001
    route: route-1

events:
  - at: 5s
    radio: 4010001
    action: ptt
  - at: 20s
    tbs: tbs-a
    action: disable

assert:
  - call_not_released
  - serving_cell: tbs-b
  - logical_call_unchanged
  - floor_holder: 4010001
  - max_media_gap_ms: 500
  - latest_position_received: true
~~~

Für eine reale Implementierung müssen Schema, Zeitbasis, Fehlermodell und Assertion-Semantik versioniert werden.

## 15. Relevante Schnittstellen und Protokolle

| Bereich | Relevante Schnittstelle | Einordnung |
|---|---|---|
| Virtuelle Luftschnittstelle | Slot-/Burst-Vertrag über RxTxDev beziehungsweise später parallelen Airlink | Geplant |
| RF | SoapySDR | Implementiert |
| MS-Steuerung | Control-/Telemetry-WebSocket | Im ms-mode dokumentiert |
| Registrierung/Mobility | MLE/MM/TNMM | Implementierte Bausteine |
| Call Control | CMCE/TNCC | Implementierte Bausteine |
| SDS | TNSDS, SDS-TL | MS-Mode dokumentiert |
| Position | LIP über SDS PID 0x0A | Decoder implementiert, Encoder für Simulator offen |
| Audio | TCH/S 274-Bit-Sprachblock, ACELP außerhalb Stack | Implementierte Grundlage |
| Multi-TBS-Kontext | MobilityExport/Import/RemoveContext | Implementiert/dokumentiert |
| Call Restore | Restore Context + Ziel-Leg-Korrelation | Implementiert/dokumentiert |
| Netzweite Medien | Media Switch | Für vollständige Handover-Kontinuität noch zu integrieren |

Neue TCP-/UDP-Ports wurden noch nicht festgelegt. Bestehende Ports anderer Komponenten sind keine automatische Vorgabe für den Simulator.

## 16. Relevante Dateien und Pfade

### 16.1 Aktueller main-Stand

- Cargo.toml — Top-Level-Workspace; ms-mode/ ist kein Workspace-Member.
- crates/tetra-pdus/src/phy/traits/rxtx_dev.rs — Root-RxTxDev-Vertrag mit Carrier-/RSSI-Bezug.
- ms-mode/crates/tetra-pdus/src/phy/traits/rxtx_dev.rs — erweiterter MS-RxTxDev-Vertrag mit set_rf_path, tx_air_time, ms_tx_lookahead, dl_rssi_dbfs und Runtime-Retuning.
- ms-mode/docs/MS_MODE.md — vollständige MS-Mode-Architektur und Featurestatus.
- crates/tetra-entities/src/cmce/subentities/sds_bs.rs — SDS-/LIP-Dekodierung.
- misc/brew-server/src/position.rs — zusätzliche Position-/LIP-Decoder.
- system-backend/mobility-core/docs/context-transfer.md — zentraler MM-Kontexttransfer.
- system-backend/call-control/docs/call-restore.md — Mehrzellen-Call-Restore.
- crates/tetra-entities/tests/test_two_cell_call_restore.rs — Zwei-Zellen-Restore-Tests.

### 16.2 Historischer ZIP-Anhang

Der ZIP-Anhang enthält diese Pfade ebenfalls beziehungsweise den damaligen Stand. Besonders relevant:

- netcore-tetra-mqtt/ms-mode/docs/MS_MODE.md
- netcore-tetra-mqtt/ms-mode/crates/...
- netcore-tetra-mqtt/system-backend/mobility-core/docs/context-transfer.md
- netcore-tetra-mqtt/system-backend/call-control/docs/call-restore.md
- netcore-tetra-mqtt/crates/tetra-entities/tests/test_two_cell_call_restore.rs

## 17. Befehle, Installation und Deployment

Ein ausgeführter Installations-/Deploymentablauf für den Simulator ist noch nicht dokumentiert.

Es gab ausschließlich Architektur- und Datenflussvorschläge. Deshalb existiert hier kein „funktionierender Installationsbefehl“, der später ungeprüft wiederverwendet werden sollte.

Für die künftige Implementierung sollte zunächst ein in-process Testtarget entstehen. Deployment in LXC/VM/Kubernetes ist nachrangig; erst muss das Simulationsprotokoll stabil sein.

## 18. Fehler, Diagnose und verbleibende Probleme

Im ursprünglichen Entwurf trat kein konkreter Runtimefehler auf. Die geprüfte Gegenprüfung zeigt jedoch mehrere Architektur- beziehungsweise Reifeprobleme:

1. **Historischer mqtt-Branch nicht mehr vorhanden.**\
   Der alte Link ist kein reproduzierbarer Branchstand mehr. Das ZIP ist deshalb für diesen Arbeitsstand die wichtigste historische Momentaufnahme.

2. **Kein VirtualRxTxDev/SimAir im aktuellen main gefunden.**\
   Repositoryweite Suche nach VirtualRxTxDev, SimAir und PhyBackend::Virtual lieferte keine Implementierung.

3. **MS-Mode-Konsolidierung offen.**\
   Der vollständige MS-Unterbaum liegt separat unter ms-mode/ und ist nicht Mitglied des Top-Level-Cargo-Workspace. Teile des Root-Baums kennen StackMode::Ms, der Root-Binary-Startpfad ist aber nicht als vollständiger MS-Runtimepfad nachgewiesen.

4. **Automatische Neighbor-Relection fehlt.**\
   Ohne sie ist ein positionsgetriebener Zellwechsel nicht realistisch.

5. **Media-Handover ist noch nicht vollständig.**\
   Call Restore koordiniert Signalisierung, aber das aktuelle Restore-Dokument weist die Übertragung der Media Frames zwischen Zellen noch dem Media Switch als späteren Schritt zu.

6. **LIP-Sendepfad für Simulator nicht nachgewiesen.**\
   Decoder sind vorhanden; ein klarer Encoderbaustein wurde nicht gefunden.

7. **Historischer Either-or-PHY-Vorschlag wurde später präzisiert.**\
   Für ein hybrides Testlabor soll Virtual Air parallel zu RF möglich sein.

8. **Skalierung und Determinismus sind noch ungetestet.**\
   Es gibt keine Belege für 50/100/500 virtuelle Teilnehmer, keine Simulator-Lasttests und keinen deterministischen Handover-Benchmark.

## 19. Durchgeführte Tests und ihre Grenzen

### 19.1 Bei der Bestandsaufnahme tatsächlich durchgeführt

- Repositorydateien statisch gelesen.
- Branchstatus main/Archiving geprüft.
- Nichtvorhandensein des mqtt-Branches am Prüfstand 05.10.2026 geprüft.
- Historisches ZIP programmgesteuert inventarisiert und relevante Dateien gelesen.
- Repositoryweite Suchen nach VirtualRxTxDev/SimAir/Virtual Backend und LIP-Encoder durchgeführt.

Keine Builds und keine ausführbaren Tests wurden gestartet.

### 19.2 Vorhandene Testnachweise im Repository

- MS_MODE.md kennzeichnet mehrere MS-Funktionen als software-tested oder hardware-validated.
- test_two_cell_call_restore.rs besitzt mehrere konkrete Mehrzellen-Restore-Tests.
- Root-Tests enthalten MS-bezogene StackMode::Ms-Testfälle.

Diese vorhandenen Tests bestätigen einzelne Protokollbausteine, aber **nicht** den entworfenen vollständigen SimAir, die Browser-MMI, positionsgetriebene Reselection oder eine nahtlose Ende-zu-Ende-Mediaübergabe.

## 20. Verworfene oder ersetzte Ansätze

### 20.1 Direkte Fake-Backendzustände

**Verworfen:** WebUI -> spezieller POST-Endpunkt -> Teilnehmer künstlich registriert.

Grund: Damit würden MLE/MM/CMCE/MAC-Pfade umgangen und der Simulator hätte geringe Aussagekraft für reale RF-Probleme.

### 20.2 Eigene Fake-TBS-Logik

**Verworfen:** separate vereinfachte VirtualTBS mit eigener Call-/Mobility-Logik.

Grund: Zwei Implementierungen würden auseinanderlaufen. Die virtuelle TBS soll dieselbe Runtime-/Protokolllogik nutzen.

### 20.3 Ausschließliches PhyBackend::Virtual

**Präzisiert:** Für offline CI weiterhin sinnvoll; langfristig zusätzlich paralleler Virtual Airlink neben SoapySDR.

### 20.4 Position direkt in Kartenbackend schreiben

**Nicht empfohlen:** Das würde SDS/LIP und Routing nicht testen.

Besser: virtueller MS erzeugt reguläre Positionstelegramme; NetCore dekodiert und verarbeitet sie auf normalem Weg.

## 21. Roadmap-Kandidaten aus diesem Arbeitsstand

Diese Punkte sind Vorschläge für die Umsetzung und noch keine beschlossene Termin- oder Implementierungsroadmap.

| Priorität | Kandidat | Abhängigkeiten | Erfolgskriterium |
|---|---|---|---|
| P0 | Führenden MS-Stack festlegen und ms-mode/Main-Workspace konsolidieren | aktueller Cargo-Workspace, ms-mode | ein eindeutiger produktiver MS-Runtimepfad |
| P0 | Virtual-Airlink-Vertrag festlegen | RxTxDev, TdmaTime, Carrier, TrainingSequence | versionierter Slot-/Burst-Vertrag |
| P0 | Deterministischen SimClock/SimAir bauen | Virtual-Airlink-Vertrag | 1 TBS + 1 MS reproduzierbar |
| P0 | Registrierung, Group Attach, SDS und PTT über SimAir | MS/TBS-Stack | keine Test-Shortcuts |
| P1 | Bestehendes portable-radio UI evaluieren/erweitern | Control/Telemetry/Voice | vollständige virtuelle MMI |
| P1 | Zwei-TBS-Test mit manuellem Zellwechsel | Mobility Core, Call Control | Call/Floor erhalten |
| P1 | Neighbor Measurement + automatische Cell Reselection | MS MLE/RLF | Zellwechsel durch RF-Modell |
| P1 | LIP-Encoder im virtuellen MS | SDS/Bitcodec | GPS -> LIP -> TBS -> Karte |
| P1 | Media-Switch-Handover und Gap-Messung | Call Restore, Media Switch | definierte max. Sprachlücke |
| P2 | Parallelbetrieb RF + virtuelle Teilnehmer | SoapySDR + Airlink-Mux | reale und virtuelle MS gleichzeitig |
| P2 | Scenario Runner | alle obigen Komponenten | YAML/JSON-Szenarien + Assertions |
| P2 | Fault Injection | SimAir | Loss/Delay/TBS-Ausfall reproduzierbar |
| P2 | Lasttest 50/100/500 MS | Scheduler, UI/headless clients | Ressourcen- und Latenzprofile |

## 22. Konkrete nächste Schritte

Die sinnvollste Fortsetzung aus geprüfter Sicht ist:

1. **Keinen UI-Code zuerst schreiben.** Zuerst den Airlink-/SimClock-Vertrag festlegen.
2. **Entscheiden, welcher MS-Stack führend wird.** Nested ms-mode und Root-Main dürfen nicht parallel auseinanderentwickelt werden.
3. **Minimalen Headless-End-to-End-Test bauen:** eine virtuelle TBS, ein virtueller MS, Registrierung, Gruppenattach, SDS, Group PTT.
4. **Danach zweite Zelle hinzufügen**, zunächst mit explizitem CampOnCell, um Mobility Context Transfer und Call Restore deterministisch zu testen.
5. **Anschließend RSSI-/Positionsmodell und automatische Reselection** implementieren.
6. **Parallel vorhandenes Companion-UI erweitern**, statt eine neue MMI ohne Wiederverwendung zu beginnen.
7. **Media-Handover schließen und instrumentieren**, erst danach das Wort „nahtlos“ für laufende Sprache verwenden.
8. **Scenario Runner und Lasttests** erst auf dem stabilen Airlink aufbauen.

## 23. Relevante verwandte Archivdokumente

- [Android-TETRA-Modul, virtueller Airlink und Realtime-Core](2026-10-05_android-tetra-modul-virtueller-airlink-und-realtime-core.md) — spätere Präzisierung auf parallelen virtuellen Airlink und getrennte Realtime-Control-/Media-Pfade.
- [Foundation/Mobility/Core-LXC-Projektnotizen](2026-10-04_swmi-foundation-mobility-core-lxc-open-lab.md) — verwandter Mobility-/Restore-Kontext.
- [Gesprächssimulator und lokale PTT-Sprechstelle](2026-10-03_gespraechssimulator-issi-und-lokale-mikrofon-ptt-sprechstelle.md) — anderer Simulatorbegriff: netzseitige Gesprächsszenarien, nicht vollständige virtuelle MS.

Die verwandten Unterlagen ergänzen den Simulatorentwurf. Maßgeblich bleibt das Ziel eines vollständig bedienbaren virtuellen Funkgeräts.

## 24. Abschlussbewertung

Die Machbarkeitseinschätzung des ursprünglichen Entwurfs bleibt bestehen: Ein vollwertiges virtuelles TETRA-Testlabor ist mit NetCore-Tetra technisch sehr gut realisierbar, weil bereits MS-Protokollbausteine, eine abstrahierte RX/TX-Kante, Mobility Context Transfer, Call Restore, SDS/LIP-Dekodierung und Mehrzellentests existieren.

Der entscheidende Unterschied zwischen „Demo“ und „brauchbarem Prüflabor“ ist die Integrationskante. Der Simulator sollte auf Slot-/Burstebene oder einer äquivalenten Airlink-Schnittstelle einspeisen und die echten Protokollzustände darüber laufen lassen.

Seit dem ursprünglichen Entwurf hat sich die Zielarchitektur präzisiert: Virtual Air soll langfristig parallel zu RF möglich sein, und der Airlink ist von zentralem Call-/Media-Routing zu trennen. Gleichzeitig ist noch kein SimAir/VirtualRxTxDev im aktuellen main nachgewiesen. Automatische Reselection, ein sauberer LIP-Sendepfad und die vollständige Media-Kontinuität beim Zellwechsel bleiben die wichtigsten technischen Lücken.

Für die Umsetzung stehen damit Architektur, Anschlussstellen und offene Integrationspunkte fest. Das vollständige SimAir-Testlabor bleibt zu bauen und abzunehmen.
