# Technische Abschlussdokumentation: FlowStation-DualCarrier, Bearer-Zuordnung, ACK, Release und Secondary-Control

> **Historischer Entwicklungsverlauf mit gesonderter Repository-Prüfung, keine Freigabe eines Betriebsstands.** Dieser Chat behandelt die Weiterentwicklung von zunächst drei Traffic-Ressourcen zu optionalem DualCarrier-Betrieb, die Fehlerfolge der Pakete v1 bis v2.8 und die zugehörige WebUI. Die letzte ausdrückliche Benutzerentscheidung reserviert Carrier 2 / Air-TS1 wieder für einen Steuerkanal. Das ausgelieferte Paket v2.8 setzt dafür `SecondaryBcchNoMcch` und sechs logische Traffic-Bearer ein. Ein anschließender erfolgreicher Funk- oder Lasttest dieses letzten Pakets ist nicht dokumentiert. Der am 03.10.2026 geprüfte Repository-Code enthält wichtige spätere Änderungen und darf nicht durch diese historischen Ersatzdateien überschrieben werden.

## 1. Metadaten und Abgrenzung

| Merkmal | Festgestellter Stand |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Optionale Nutzung beider Carrier; Ressourcenverwaltung; Asterisk/Brew-Kompatibilität; logische versus physische Timeslots; Absturz beim vierten Ruf; ACK-Routing; Gruppenruf-Release; Hangtime; Dashboard; Reservierung von Secondary-TS1 |
| Ursprünglicher Chattitel | Nicht eindeutig zugänglich. Der Titel dieses Dokuments ist eine redaktionelle Themenbeschreibung. |
| Ursprünglicher Chatlink / Chat-ID | Nicht verfügbar; nicht rekonstruiert oder erfunden. |
| Historische Zeitangaben | Die bereitgestellten Betriebslogs tragen `Jun 29` und `Jun 30`. Ein zugehöriger Commit ist auf den 29.06.2026 datiert. Das stützt den Zusammenhang mit Juni 2026, ersetzt aber keine vollständigen Zeitstempel sämtlicher Chatnachrichten. |
| Erstellungs- und Prüftag | **03.10.2026** |
| Historisch vom Benutzer verlinktes Repository | `JanHG98/flowstation` |
| Heute aufgelöstes Repository | Die GitHub-Abfrage für `JanHG98/flowstation` liefert `JanHG98/netcore-tetra`, Repository-ID `1281497427`. Die Namen werden nicht als zwei unabhängig geprüfte heutige Repositories behandelt. |
| Ausschließlicher Schreibbranch | **`Archiving`** |
| Für die Quellcodeprüfung eingefrorener Archiving-Commit | **`311462808d6ebe49b0b426bec11f381d6867f676`** |
| Tree dieses Prüfstands | `fcc52a09bec493da059f26e04efcf42566ab0f33` |
| Zusätzlich aufgelöster main-Commit | **`6aa9be8f74ab731f72dc133a5f8e90c5018c626d`** |
| Vor dem Schreiben erneut geladener Archiving-Stand | `f8f1e001779cf564982c83abad4f5fa6b7637cd5`, Tree `3d0e2b1f328f602964712375dc323e716ccdf22b` |
| Zwischenzeitliche Änderung | Gegenüber dem eingefrorenen Prüfstand wurde ausschließlich `Docs/Control_Room/Readme.md` ergänzt. Diese fremde Änderung wird unverändert bewahrt. |
| Archivdatei | `Docs/archive/2026-10-03_flowstation-dualcarrier-bearer-ack-release-und-secondary-control.md` |
| Umfang dieses Archivauftrags | Diese Dokumentation und ein zusätzlicher Indexeintrag in `Docs/archive/README.md`; keine Änderung von Produktivcode, Konfiguration, Wiki, Releases oder anderen Branches. |

Der ursprüngliche Repo-Upload `flowstation-beta.zip` enthält den ZIP-Kommentar `86648c2a766bcc5298d1eeb158606e131192a5ac`. Dieser Commit wurde heute tatsächlich aufgelöst: Nachricht `netcore`, Commitdatum 28.06.2026, 21:30:20 UTC. Ein ZIP-Kommentar allein beweist nicht die bytegenaue Übereinstimmung jeder enthaltenen Datei mit diesem Commit. [H1]

Im späteren Startlog steht `Version: v1.3.0-d00c4e03`. Der Kurzbezeichner wurde heute zu **`d00c4e03849898aa137e0f1de09a2897cc76d695`** aufgelöst: `Update llc_bs_ms.rs`, 29.06.2026, 23:35:02 UTC. Der Commit enthält die hier diskutierte Erweiterung des ACK-Routings um den logischen `link_id`. Damit ist diese konkrete historische Quellcodeänderung auch im Git nachgewiesen. Ein Startbanner schließt zusätzliche lokale, nicht committete Änderungen am Build nicht aus. [H2, L42]

Die Paketbezeichnungen **v2, v2.1 bis v2.8 sind Bezeichnungen dieser Chat-Auslieferungen**, nicht automatisch Git-Tags, Produktversionen oder vollständige Release-Stände. Für die übrigen Pakete wird keine exakte historische Commit- oder PR-Zuordnung behauptet.

### 1.1 Verhältnis zu einem bereits vorhandenen Archiv

Die Datei [FlowStation-DualCarrier-Portierung, SXceiver und Hotfixes 001–009](2026-10-03_flowstation-dualcarrier-portierung-sxceiver-hotfixes.md) dokumentiert einen **anderen, vorgelagerten Chat**: erste Portierung, SDR-Konfiguration und Hotfixes vom 28.06. Dieser Eintrag wurde vor dem Schreiben geprüft und nicht überschrieben. Die vorliegende Dokumentation setzt thematisch bei der anschließenden Ressourcen-, ACK-, Release- und Belegungsproblematik an. Aussagen aus dem anderen Archiv werden nicht als zusätzliche Benutzerbestätigung dieses Chats umgedeutet.

### 1.2 Belegstufen

| Kennzeichnung | Bedeutung |
|---|---|
| **Idee** | Diskutierter Ansatz ohne verbindliche Beauftragung oder Implementierungsnachweis. |
| **Beschlossen/geplant** | Ausdrücklicher Benutzerwunsch beziehungsweise festgelegtes Ziel; noch kein Umsetzungsbeweis. |
| **Implementiert – Artefakt** | In den heute zugänglichen Ersatzdateien oder ZIP-Bytes vorhanden. Kein Beweis für Einspielen, Kompilieren oder Betrieb. |
| **Implementiert – Repository** | Im angegebenen, unveränderlich referenzierten Git-Commit im Code nachgewiesen. |
| **Getestet** | Tatsächlich ausgeführte Prüfung mit nachvollziehbarem Ergebnis. Art und Reichweite des Tests werden genannt. |
| **Im Betrieb bestätigt** | Konkrete Benutzerbeobachtung oder Laufzeitlog für genau das beobachtete Verhalten. Kein pauschales Gütesiegel für andere Funktionen. |
| **Hypothese / offen** | Technisch begründeter Prüfpunkt ohne reproduzierten Nachweis als Ursache des beobachteten Fehlers. |

Die acht bereitgestellten Logdateien und zwölf ZIP-Dateien waren bei dieser Auswertung als vollständige Dateien zugänglich. Die Logtexte wurden durchsucht und relevante Ereignisfolgen im Zusammenhang geprüft; die vielen wiederholten PHY-Burst-Zeilen sind keine jeweils eigenständigen Funktionstests. Manche Dateien beginnen oder enden bereits mitten in einer Nachricht; eine vollständige Datei ist deshalb nicht automatisch ein vollständiges Journal des Testlaufs.

**Datenschutz und Geheimnisse:** Keine Passwörter, Tokens, privaten Schlüssel, Authentifizierungsheader oder vollständigen geheimnishaltigen Konfigurationsdateien werden übernommen. Interne Hostnamen, Funkidentitäten, Ports und Frequenzen bleiben als erforderlicher technischer Kontext erhalten. Aus Git-Metadaten werden keine privaten E-Mail-Adressen übernommen.

## 2. Ziel und Ausgangslage

Jan meldete zu Beginn, dass die Geräte im verlinkten Fork nur Carrier 1 verwendeten und Carrier 2 ignorierten. Gefordert war keine Ablösung vorhandener Integrationen, sondern eine durchgängige Kenntnis der verfügbaren Carrier:

- Carrier 2 darf nur verwendet werden, wenn er im WebUI beziehungsweise in der effektiven Konfiguration aktiviert ist.
- Bei deaktiviertem Carrier 2 muss der bisherige Einträgerbetrieb erhalten bleiben.
- Asterisk muss weiter funktionieren; ebenso dürfen die bestehenden Call-Control- und Medienpfade nicht durch den Umbau auseinanderlaufen.
- Ausgeliefert werden sollen **vollständige Dateien mit vollständigen Repository-Pfaden**, keine Patch-Dateien.
- Die WebUI soll die tatsächlich belegten Ressourcen beider Carrier anzeigen.
- Rufaufbau, Sprechrecht, ACK, Hangtime und Rufabbau müssen auch bei mehreren gleichzeitigen Belegungen funktionieren, ohne andere Geräte oder den gesamten Dienst zu verlieren.

Die erste Lösung griff nur in UMAC ein. Das reichte nicht: Eine andere RF-Frequenz für einen bereits vorhandenen Timeslot schafft noch keine zusätzlichen unabhängig verwalteten Traffic-Ressourcen. Der historische Allocator, die oberen Rufzustände und die unteren physischen Timeslot-Arrays mussten gemeinsam angepasst werden. [A0–A4, L34, L37]

**Zentrale Lehre dieses Chats:** Der Schlüssel einer Funkressource muss über alle beteiligten Komponenten eindeutig bleiben. `Air-TS2` auf Carrier 720 ist nicht dieselbe Ressource wie `Air-TS2` auf Carrier 721. Ein teilweiser Umbau, bei dem eine Schicht logische TS5–TS7 liefert und eine andere nur physische TS1–TS4 erwartet, führt nicht nur zu Anzeigeproblemen, sondern zu Fehlrouting und im dokumentierten Fall zu einem Prozessabsturz.

## 3. Endgültige Anforderungen und letzte ausdrückliche Entscheidung

### 3.1 Zuletzt beschlossenes Ressourcenmodell

Der Benutzer nahm den zwischenzeitlichen Wunsch nach vier Traffic-Timeslots auf Carrier 2 ausdrücklich zurück und verlangte zuletzt auch dort auf **Air-TS1 einen Control Channel**. Seine Begründung war die Vermutung, dass Funkgeräte und Basisstation bei voller Auslastung nicht mehr hinterherkämen.

Diese Begründung ist als **Benutzerhypothese zur Stabilisierung**, nicht als bewiesene Kapazitätsgrenze der Geräte, festzuhalten. Die Entscheidung für die Reservierung ist dagegen verbindlich für den Abschluss dieses Chats.

Das letzte ausgelieferte Paket **v2.8** setzt folgendes Modell um:

| Carrier | Physischer Air-Timeslot | Logische Traffic-ID | Rolle im letzten Paket |
|---|---:|---:|---|
| 720 / Carrier 1 | 1 | Keine Traffic-ID; Common-Control-Kontext | MCCH / Control |
| 720 / Carrier 1 | 2 | 2 | Traffic |
| 720 / Carrier 1 | 3 | 3 | Traffic |
| 720 / Carrier 1 | 4 | 4 | Traffic |
| 721 / Carrier 2 | 1 | Nicht als Traffic zugeteilt | Secondary-BCCH-/Control-/Guard-Rolle im Modus `SecondaryBcchNoMcch` |
| 721 / Carrier 2 | 2 | 5 | Traffic |
| 721 / Carrier 2 | 3 | 6 | Traffic |
| 721 / Carrier 2 | 4 | 7 | Traffic |

Damit stehen im letzten Modell **drei Traffic-Ressourcen pro Carrier, insgesamt sechs**, zur Verfügung. Bei deaktiviertem Secondary bleiben drei. Die Zahlen 720/721 sind die konkreten Testparameter; die Zuordnung muss die konfigurierten Carrierwerte verwenden und darf diese Zahlen nicht fest in den Medienpfad codieren. [A11, R1, R2, R3]

### 3.2 Steuerkanal, Broadcast und reservierter Slot sind nicht dasselbe

Der Wortlaut „Carrier 2 TS1 Control Channel“ beschreibt die Benutzeranforderung. Die nachgewiesene Implementierung wählt `SecondaryBcchNoMcch`, reserviert den Slot und bezeichnet ihn in der Oberfläche als Control/Guard. **Das ist kein Nachweis eines zweiten MCCH und auch keine vollständige Abnahme eines zusätzlichen, von den MS benutzten Common-SCCH.**

Für eine behauptete Entlastung des allgemeinen Steuerverkehrs müssten unter anderem die angekündigte Kanalrolle, Auswahl durch das Endgerät, zulässiger Uplink-Zugang, Grants, Paging, ACK und Rückkehr zum Hauptsteuerkanal zusammenpassen. Nur einen Timeslot nicht mehr für Sprache zu vergeben oder ihn mit SYNC/SYSINFO zu füllen, belegt diese vollständige Funktion nicht.

Die heutige begrenzte Sichtung der angehängten Air-Interface-Spezifikation bestätigt die notwendige begriffliche Trennung: EN 300 392-2 V3.8.1, Abschnitt 19.3.2.1.1, beschreibt verschiedene Kombinationen aus MCCH, Common-SCCH und Assigned-SCCH. Abschnitt 16.10.45 beschreibt den SCCH-Informationsparameter zur Auswahl eines Common-Control-Kanals. Daraus wird hier **keine vollständige Konformitätsbewertung** des Codes abgeleitet. [N1]

### 3.3 Kapazität bedeutet nicht automatisch Anzahl etablierter Gespräche

Die sechs Ressourcen können auch durch Rufaufbau-Reservierungen belegt sein. Ein lokaler Einzelruf mit getrennten Funkstrecken ist außerdem nicht zwingend mit einer einzelnen Ressource gleichzusetzen. Im heutigen Repository konkurriert zusätzlich SNDCP um denselben Allocator. Deshalb müssen die Begriffe **frei, reserviert, RF-Circuit offen, Sprache aktiv, Hangtime und Release ausstehend** getrennt werden. [L42, R1, R3]

### 3.4 Verbindlicher Erhalt bestehender Funktionen

Asterisk-Kompatibilität bleibt eine Anforderung. Dass Logs `AsteriskEntity: media ready` zeigen, belegt den entsprechenden Aufbaupfad, nicht die vollständige bidirektionale Sprachqualität, DTMF, alle Rufrichtungen oder zuverlässiges Auflegen unter Last. Dass Brew eine Affiliation erhält, ist wiederum kein Test eines Brew-Sprachrufs. EchoLink wurde zwar in Kompatibilitätszusagen genannt, war im späteren Startlog aber deaktiviert. [L35, L38, L42]

## 4. Chronologie der Pakete und Korrekturen

Die Reihenfolge ist die Reihenfolge des zugänglichen Dialogs. Die Artefaktkürzel verweisen auf das vollständige Manifest in Abschnitt 14.

| Stufe | Anlass und behaupteter beziehungsweise implementierter Eingriff | Ergebnis und Beleggrenze |
|---|---|---|
| Ausgangsupload A0 | `flowstation-beta.zip` als Basis für komplette Ersatzdateien. | Bytes zugänglich; ZIP-Kommentar auf einen existierenden Commit aufgelöst. |
| v1 / A1 | Nur `umac_bs.rs`: pro physischem Timeslot den Traffic-Carrier merken, Carrierwahl signalisieren, DL-Medien gegebenenfalls von Main auf Secondary remappen. | Carrier-2-Bursts und Remapping sichtbar, aber `NoCircuitFree` bleibt. Keine vollständige Ressourcen-Erweiterung. |
| v2 / A2 | Neun Dateien: Allocator und CMCE auf sechs logische Bearer; Channel-Allocation-Hint; UMAC-Zuordnung; SDS und Defragmentierung einbeziehen. | Konzeptioneller Umbau im ZIP vorhanden. Compiler meldet fehlenden Typnamen `Todo` im PDU-Modul. |
| v2.1 / A3 | `use tetra_core::Todo;` in `pdu.rs`. | ZIP-Unterschied zu v2 auf diesen Import beschränkt. Spätere Prozesse starten, aber kein vollständiges erfolgreiches `cargo check`-Protokoll angehängt. |
| Registrierungslog L35 | ITSI-Attach, Gruppen-Affiliation und Brew-Affiliation. | Normaler Registrierungsverkehr beobachtet; kein Carrier-2-Ruf und kein Asterisk-End-to-End-Test in diesem Ausschnitt. |
| Vierter Ruf / L37 | Obere Ebene vergibt `ts=5`, laufende UMAC verwendet noch alte physische Zuordnung. | `invalid ts 5`, Panic in unterem CircuitMgr und anschließend neue Prozess-ID. Dienstabsturz ist belegt. |
| v2.2 / A4 | Zehn Dateien, darunter gehärteter unterer `umac/subcomp/circuit_mgr.rs`; konsistenter logischer Mapper und Startmarker. | Folgelog L38 zeigt Öffnen von Secondary-Air-TS2/3/4 und Asterisk-Media-Ready. Der frühere Panic erscheint dort nicht. |
| Erste UI-Erweiterung / A5 | `html.rs` zeigt zusätzlich TS5–TS8; Backend nutzt weiterhin nur TS5–TS7. TS8 wird als Spare dargestellt. | Anzeige und reale Kapazität widersprechen sich aus Benutzersicht. Keine zusätzliche Ressource durch eine Kachel. |
| Releaseproblem / L39 | Benutzer meldet auf Carrier 2 festhängendes Funkgerät. Gruppenruf wird serverseitig geschlossen. | Ein reiner MCCH-Release erreicht einen noch auf C2 zugeteilten Teilnehmer möglicherweise nicht. Außerdem zeigt der Log bereits ACKs mit verlorenem Carrierkontext. |
| v2.3 / A6 | Normaler Gruppen-Release: zwei FACCH/STCH-Sendungen auf dem zugeteilten Bearer plus MCCH-Fallback, danach verzögerter physischer Close. | Änderung im PDU-Artefakt vorhanden. Keine abschließende Geräteabnahme. |
| Lastproblem / L40 | Benutzer meldet erneut Geräteverluste bei nahezu voller Belegung. | RX `link_id=7`, LLC-ACK aber `ts=4`, Carrier 720: konkreter Routingfehler nachgewiesen. |
| v2.4 / A7 | LLC speichert und verwendet den logischen `link_id`; für TS5–TS7 Air-TS minus 3 und Carrier-Hint `-2`. | Artefakt geprüft; zugehörige Änderung heute im historischen Commit `d00c4e03849898aa137e0f1de09a2897cc76d695` nachgewiesen. |
| Releasefolge / L41 | Zwei STCH-Blöcke werden vor dem Schließen abgearbeitet. | Der Log belegt Drain und Close, nicht eine durch diese Wiederholungen verursachte Überlastung. |
| v2.5 / A8 | Konservativer normaler Gruppen-Release: auf C2 einmal FACCH/STCH ohne MCCH-Fallback; auf C1 nur MCCH. | Im Artefakt vorhanden. Die damalige Überlastungsbegründung war nicht bewiesen. Dieser Stand bleibt im letzten ZIP v2.8 enthalten, ist aber nicht der heutige Repository-Stand. |
| Letzter TS / L42 | Benutzer sieht scheinbar ungenutzten letzten Slot. | Log zeigt TS2–TS7 als Rufaufbau-Reservierungen; danach Congestion. Nicht als sechs etablierte Sprachrufe belegt. |
| v2.6 / A9 | Phantom-/Spare-TS8 aus UI entfernen; nur logische TS1–TS7 zeigen. | Reine Anzeigeanpassung, keine neue RF-Kapazität. |
| v2.7 / A10 | Nach Benutzerhinweis alle vier Air-Slots auf Secondary für Traffic freigeben; `TrafficOnly`, Kapazität sieben. | Logische TS5–TS8 entsprechen nun Air-TS1–TS4. Im Paket umgesetzt, kein nachfolgender bestätigter Test dieses Modells. |
| v2.8 / A11 | Letzte ausdrückliche Benutzerkorrektur: C2 Air-TS1 ebenfalls für Steuerung vorsehen. | Rückkehr zu sechs Ressourcen und Mapping minus 3; `SecondaryBcchNoMcch`. Paket vorhanden, keine anschließende Funk-/Lastabnahme im Chat. |

### 4.1 Warum v2/v2.1 und der Panic-Log nicht als identischer Stand gelten dürfen

Die heute untersuchten ZIPs v2 und v2.1 enthalten bereits den neuen logischen Mapper. Der beim vierten Ruf laufende Prozess in L37 loggt dagegen noch die aus v1 stammenden Funktionen `remember_traffic_carrier` beziehungsweise `selected carrier ... ts 5`. Der Quelltext des laufenden Builds und die ausgegebenen Ersatzdateien waren somit nicht durchgängig deckungsgleich.

Das stützt die Diagnose **gemischter oder nicht aktualisierter Build-/Deployment-Stand**. Ob ein ZIP nur teilweise kopiert, ein anderes Binary gestartet oder ein älterer Build installiert wurde, lässt sich aus dem Verlauf nicht entscheiden. Es wäre falsch, einen bestimmten Bedienfehler des Benutzers als erwiesen darzustellen. Der spätere Marker war ein Diagnosehilfsmittel, aber kein Ersatz für eine überprüfte Build- und Dateiversion.

### 4.2 Geänderte Dateien beim letzten Mappingwechsel

Die letzte Antwort verlinkte vier besonders wichtige Dateien. Der tatsächliche Vergleich **v2.7 → v2.8** betrifft jedoch acht Dateien: Allocator, CMCE-PDU, SDS, LLC, UMAC, unterer CircuitMgr, Scheduler und Dashboard-HTML. Wer nur die vier prominent verlinkten Dateien übernimmt, riskiert erneut ein Mischmodell. Das letzte vollständige ZIP enthält insgesamt 13 Quellcodedateien. [A10, A11]

## 5. Architektur, Ressourcenidentität und Schnittstellen

### 5.1 Ebenen und Verantwortlichkeiten

```text
WebUI-Konfiguration
  └─ dual_carrier_enabled + secondary_carrier
      └─ effektive CfgCellInfo::secondary_carrier
          ├─ PHY / SDR: konfigurierte RF-Träger, Modulation und Empfang
          ├─ UMAC: pro Carrier ein Scheduler
          └─ CMCE / TimeslotAllocator: freigeschaltete logische Ressourcen

Rufsteuerung:  CMCE ↔ MLE ↔ LLC ↔ UMAC ↔ LMAC ↔ PHY
Sprachpfad:   Medienintegration ↔ TMD-SAP / UMAC ↔ LMAC ↔ PHY
Telemetrie:   Call-/Voice-/End-Ereignisse → Dashboard-Zustand → WebUI
```

Diese Darstellung beschreibt die im Code und in den Logs sichtbaren Zuständigkeiten. Sie ist kein Nachweis, dass jede Ende-zu-Ende-Kombination getestet wurde. [A0, A11, L35, L38, L40, R2–R8]

**TimeslotAllocator:** Eigentum und Belegung logischer Bearer. Im finalen historischen Modell TS2–TS7; im Einträgerbetrieb nur TS2–TS4. Die Kapazität ist von der effektiven Carrierkonfiguration abhängig.

**CMCE-CircuitMgr:** Rufbezogene Ressourcen, Call-ID, Usage, Richtung, Zuweisung und Freigabe. Die erweiterten Arrays sind logisch zu interpretieren. Ein reservierter Call muss nicht bereits einen offenen unteren RF-Circuit besitzen.

**UMAC:** Übersetzt zwischen logischer Traffic-ID und konkretem Paar aus Carrier und physischem Air-Timeslot; verarbeitet Channel Allocation, Signalisierung, Sprechrechtswechsel und Medien. Vor dem Zugriff auf den pro Carrier arbeitenden unteren CircuitMgr darf nur ein physischer Timeslot 1–4 übergeben werden.

**BsChannelScheduler und unterer CircuitMgr:** Arbeiten jeweils für genau einen Carrier auf physischen Air-Slots. Ihre Arrays bleiben vier Slots groß. Ein acht Elemente großes globales Array ist kein Ersatz für die Auswahl des richtigen Carrier-Schedulers.

**LLC:** Muss bei Antworten den von UMAC gelieferten Link-/Bearer-Kontext erhalten. Die Ableitung aus einem Zeitstempel-Timeslot allein verliert die Frequenzzuordnung.

**Asterisk/Brew und weitere Medienpfade:** Verwenden die logische Ressource weiterhin als Schlüssel. Die historische Integration vermied dadurch einen flächendeckenden Umbau aller Bridge-APIs. Voraussetzung ist eine einheitliche Interpretation dieses Schlüssels in beiden Richtungen, einschließlich Auflegen und neuer Belegung.

### 5.2 Definitive Zuordnungsregeln des letzten Pakets

```text
logical_ts 2..4:
  carrier = main_carrier
  air_ts  = logical_ts

logical_ts 5..7:
  carrier = secondary_carrier  // nur bei effektiv aktiviertem Secondary
  air_ts  = logical_ts - 3

Secondary-Uplink auf air_ts 2..4:
  logical_ts = air_ts + 3
```

Das zwischenzeitliche v2.7-Modell verwendete dagegen `logical_ts - 4` und TS5–TS8. Diese Regeln dürfen nicht innerhalb eines Builds gemischt werden.

Carrier-2-Air-TS1 ist im letzten Modell **kein logischer Traffic-TS5**. Im Dashboard kann eine Control-Kachel den lokalen Slotwert 1 tragen; ihre eindeutige Identität ergibt sich dann erst zusammen mit dem Carrier.

### 5.3 Channel Allocation und der Sentinel `-2`

`CmceChanAllocReq` trägt unter anderem Usage, Carrier-Hint, eine physische Timeslot-Bitmaske `[bool; 4]`, Allocation-Type und UL-/DL-Zuordnung. Der Carrier-Hint ist als `Option<Todo>` typisiert.

`Todo` ist in der historischen Codebasis ein Alias für `i32`. Der Fehler in v2 war der fehlende Import in `pdu.rs`, nicht das Fehlen jeder Definition dieses Typs. Der Name ist unglücklich, rechtfertigt aber nicht die Behauptung, die Carrier-Zuordnung sei dadurch automatisch nur ein unimplementierter Platzhalter.

`Some(-2)` ist ein **interner Sentinel zwischen CMCE und UMAC** für den aktiv konfigurierten Secondary. Auf dem Air Interface muss daraus eine reale Carrier-Nummer werden. L40 zeigt die Auflösung in `ChanAllocElement { carrier_num: 721, ... }`. Das vier Elemente umfassende Timeslot-Feld enthält weiterhin physische Air-Slots; logische TS5–TS7 werden nicht als fünftes bis siebtes Bit gesendet. [A0, A2, A3, L40, R3]

Eine spätere Verbesserung wäre ein expliziter Carrier-/Bearer-Datentyp statt mehrfach verteilter Sentinel- und Bereichslogik. Das ist ein **Roadmap-Kandidat**, kein bereits im Chat beauftragter Komplettumbau.

### 5.4 Signalisierungs- und Kontrollnachrichten

Relevante im Verlauf sichtbare Nachrichten und Übergänge:

- Rufaufbau: `U-SETUP`, `DCallProceeding`, `DSetup`, `DConnect`, `NetworkCircuitSetupAccept`, `NetworkCircuitConnectRequest`, `NetworkCircuitMediaReady`.
- Sprechrecht: `UTxCeased`, `DTxCeased`, `FloorGranted`, `FloorReleased`, Hangtime und UL-Inaktivität.
- Rufabbau: `DRelease`, `CallEnded`, `CallControl::Close`, verzögerter UMAC-Close nach STCH-Drain.
- LLC: `BlData`, `BlUdata`, `BlAck`, kombinierte beziehungsweise quittierte Nachrichten; TMA-/TLA-/LCMC-SAP-Primitiven.
- MAC: `MacAccess`, `MacData`, `MacEndHu`, `MacResource`, Grants, Usage-Marker und Channel Allocation.
- Registrierung: `ULocationUpdateDemand`, `DLocationUpdateAccept`, Gruppen-Attach und `MmSubscriberUpdate`.

FACCH/STCH nutzt Signalisierung auf dem zugewiesenen Traffic-Bearer. Der Code und die Logs zeigen dabei beispielsweise 124 STCH-Bits und einen TCH-Block mit 274 Bits. Eine Logmeldung über das Erzeugen oder Finalisieren eines Blocks ist noch kein Empfangsnachweis durch das Endgerät. [L39–L41, R5]

### 5.5 Zeitstempel nicht mit logischer Ressource verwechseln

Die PHY-Logzeit enthält den TDMA-Timeslot der Verarbeitung. Die LLC-Verarbeitung berücksichtigt eine Verschiebung zwischen Uplink und Downlink. Deshalb ist etwa ein PHY-Zeitstempel mit letztem Wert `2` nicht ohne Weiteres die Aussage „Air-TS2 wurde zugeteilt“: Für die Diagnose müssen Carrier, Umrechnung, `link_id`, Channel Allocation und Circuit gemeinsam betrachtet werden.

Der entscheidende Fehler in L40 ist nicht allein ein unterschiedlicher Zeitstempel, sondern die vollständige Kombination **eingehender logischer Link 7 → ACK-Ziel 4 ohne Secondary-Kontext → ausgesendete Carrier-Nummer 720**. [L40, R4]

## 6. Fehlerbilder, Diagnose und tatsächliche Aussagekraft

### 6.1 Carrier 2 erscheint im PHY, trotzdem `NoCircuitFree`

**Beobachtung:** L34 enthält regelmäßig Bursts mit `carrier=721` und das v1-DL-Remapping von 720 auf 721. Ein neuer Gruppenruf von ISSI 5102 zu GSSI 15201 wird trotzdem mit `NoCircuitFree` und `CongestionInInfrastructure` abgewiesen.

**Nachgewiesener Architekturfehler:** Die reine UMAC-Carrierauswahl erweitert nicht den oberen Ressourcenpool. Ein pro physischem Timeslot gespeicherter Carrier kann auch nicht gleichzeitig beide Carrier auf demselben Air-Slot repräsentieren.

**Umgesetzter Ansatz:** Ab v2 eindeutige logische Bearer-IDs, kapazitätsabhängiger Allocator und Umrechnung erst an der Grenze zu den Carrier-Schedulern. Der Ansatz ist im letzten Paket und im heutigen Repository vorhanden. Die erste Ein-Datei-Lösung ist überholt. [A1–A4, R1–R3]

### 6.2 Compilerfehler `E0425: cannot find type Todo`

Der vom Benutzer vollständig angegebene Fehler betrifft `pdu.rs`, damals Zeilen 64 und 79: Konstante `SECONDARY_CARRIER_HINT: Todo` und Rückgabetyp `Option<Todo>`.

**Korrektur v2.1:** `use tetra_core::Todo;`. Der ZIP-Vergleich bestätigt die Änderung. Ein vom Compiler vorgeschlagenes `impl<Todo>` wäre hier nicht die beabsichtigte Lösung, weil ein existierender konkreter Alias importiert werden sollte.

**Testgrenze:** Keine vollständige erfolgreiche Cargo-Ausgabe nach diesem Import im Chat. Spätere gestartete Prozesse zeigen, dass nachfolgende Stände auf der Anlage ausführbar waren; die genaue Zusammenstellung ist wegen des anschließend beobachteten Mischstands separat zu prüfen. [A0, A2, A3, L35, L37]

### 6.3 Vierter Ruf wirft alle Teilnehmer ab

L37 enthält um **00:09:46.263**:

```text
UMAC: invalid ts 5 while remembering carrier 720
UMAC: selected carrier 720 for new traffic circuit on ts 5 ...
thread 'main' ... panicked at
crates/tetra-entities/src/umac/subcomp/circuit_mgr.rs:25:30
```

Kurz danach läuft ein neuer Prozess: PID 16187 wird durch PID 16867 ersetzt. Das ist ein belegter Prozessabsturz mit Neustart, nicht nur eine normal abgewiesene neue Verbindung.

**Ursache des konkreten Panics:** Ein logischer Timeslot 5 erreicht eine nur für physische Slots ausgelegte untere Struktur. **Korrektur v2.2:** Umrechnung vor dem unteren Zugriff und Bounds-Checks in dessen CircuitMgr. Das spätere Beispiel L38 enthält keine Wiederholung dieses Panics und zeigt Secondary-Circuit-Open für logische TS5, TS6 und TS7. Das belegt Fortschritt für diesen Fehler, keine generelle Absturzfreiheit. [L37, L38, A4, R6]

### 6.4 Funkgerät bleibt nach Gruppenruf auf Carrier 2 hängen

L39 zeigt Gruppenruf `call_id=9` auf logischem TS5 / Carrier 721 / Air-TS2. Um **00:55:36.577** wird der Close verzögert, um **00:55:36.591** werden UL und DL geschlossen. Der Benutzer meldete dennoch ein festhängendes Gerät.

Ein abgeschlossener serverseitiger Close beweist nicht, dass das Endgerät seinen Release empfangen hat. Der damalige MCCH-only-Gruppenrelease auf Carrier 1 war deshalb ein sinnvoller Prüfpunkt. v2.3 ergänzte Release-Signalisierung auf dem tatsächlich zugeteilten Traffic-Bearer.

**Wichtige Einschränkung:** Derselbe Log enthält bereits ein Carrier-Kontextproblem bei ACKs: FACCH-Versuch auf logischem TS2 / Carrier 720 neben der eigentlichen Signalisierung auf logischem TS5 / Carrier 721. Es wäre daher zu eng, das Festhängen ausschließlich dem Release-Pfad zuzuschreiben. [L39, insbesondere rohe Zeilen 594–595, 716–735 und 834–842]

### 6.5 ACK auf falschem Carrier bei nahezu voller Belegung

L40 ist der stärkste Beleg für einen unabhängigen Signalisierungsfehler. Bei einem Gruppenruf auf **logischem TS7 / Carrier 721 / Air-TS4** liefert UMAC ein `TmaUnitdataInd` mit `link_id: 7`. LLC erzeugt jedoch anschließend:

```text
auto-ack for ssi: 5102, n: 1, ts: 4
FACCH stealing on logical ts 4 / carrier 720 air ts 4 skipped ...
ChanAllocElement { ... carrier_num: 720, ... }
```

Parallel wird die eigentliche `DTxCeased`-Meldung korrekt auf logischem TS7 / Carrier 721 eingereiht. Es folgen wiederholte U-TX-CEASED-Nachrichten und ein `RoamingLocationUpdating` von ISSI 5102.

**Nachgewiesen:** Der Carrierkontext geht im ACK-Rückweg verloren. **Nicht nachgewiesen:** Dass genau dieser Fehler jedes gemeldete „Kicken“ aller Geräte verursachte oder dass die BS dabei erneut abstürzte.

**v2.4:** `schedule_outgoing_ack` bekommt den logischen `link_id`; ACK-TS5–TS7 werden auf Air-TS2–TS4 zurückgerechnet und mit Secondary-Hint weitergegeben. Diese Änderung ist im ZIP, im historischen Commit H2 und weiterhin im heutigen Code vorhanden. [L40, A7, H2, R4]

### 6.6 Die frühere Aussage „zu viele Release-Wiederholungen“ war nicht belegt

L41 zeigt einen normalen Ablauf eines verzögerten Close:

| Zeitpunkt | Ereignis |
|---|---|
| 01:47:08.937 | `Deferred Both circuit close for logical ts 7` |
| 01:47:08.950 | Zwei Release-Blöcke auf C2 / Air-TS4 eingereiht; zusätzlicher MCCH-Release |
| 01:47:08.994 | Erster FACCH/STCH-Block finalisiert |
| 01:47:09.051 | Zweiter Block finalisiert; DL- und UL-Circuit geschlossen |

Vom Deferred-Close bis zum physischen Close liegen in diesem Ausschnitt **114 ms**. Die wiederholten `waiting for FACCH/STCH drain`-Zeilen sind während dieses Ablaufs erwartbare Zustandsbeobachtungen. Der Ausschnitt enthält keinen Nachweis einer dadurch verursachten CPU-, Queue- oder Funküberlastung und keinen Nachweis, dass ein fremdes Gerät durch diese drei Nachrichten entfernt wurde.

Die anschließende Reduktion auf einen Release in v2.5 war ein **implementierter Versuch**, dessen behauptete Ursache und erfolgreiche Wirkung nicht belegt wurden. Sie wird deshalb nicht als bewiesener Stabilitätsfix archiviert. Das heutige Repository verwendet beim normalen Gruppenrelease wieder zwei FACCH-Sendungen plus MCCH-Fallback. Auch daraus folgt nicht automatisch eine neue Fehlfunktion. [L41, A6, A8, A11, R3]

### 6.7 Scheinbar ungenutzter letzter TS: UI-Artefakt und Reservierung unterscheiden

Die erste UI-Erweiterung zeigte TS8 als Spare, obwohl der Allocator nur TS2–TS7 vergeben konnte. Das war eine irreführende Kapazitätsdarstellung und wurde mit v2.6 entfernt.

L42 zeigt zusätzlich einen anderen wichtigen Sachverhalt: Die sechs `forwarding U-SETUP over Brew`-Zeilen belegen **reservierte Rufaufbauten**, nicht sechs erfolgreich etablierte Sprachverbindungen:

| Zeitpunkt | Call-ID | Logische Ressource | Ziel / Pfad |
|---|---:|---:|---|
| 18:08:20.551 | 4 | 2 | Brew, Ziel 2091201 |
| 18:08:24.348 | 5 | 3 | Brew, Ziel 2091201 |
| 18:08:36.135 | 6 | 4 | Brew, Ziel 2091201 |
| 18:08:42.028 | 7 | 5 | Brew, Ziel 2091201 |
| 18:08:45.088 | 8 | 6 | Brew, Ziel 2091201 |
| 18:08:53.985 | 9 | 7 | Brew, Ziel 2091201 |

Danach werden Gruppenrufversuche mit `NoCircuitFree` abgewiesen. Bei der späteren Freigabe erscheinen für diese Ressourcen `No Dl circuit to close` und `No Ul circuit to close`. In der bereitgestellten Datei sind für diese sechs Aufbauten keine passenden UMAC-Open-/Media-Ready-Belege enthalten. Erst ein späterer Gruppenruf wird um 18:09:20.958 auf logischem TS2 tatsächlich unten geöffnet.

**Folgerung:** Ein voll belegter Allocator kann bei fehlgeschlagenen oder wartenden externen Rufaufbauten korrekt keine neue Ressource liefern, obwohl die RF-Anzeige nicht sechs aktive Sprachkanäle zeigt. Ob die Reservierungen jeweils rechtzeitig zurückgenommen wurden, ist anhand der vollständigen Rufzustandsfolge zu testen. Die damalige pauschale Interpretation „sechs Gespräche laufen“ wird hier ausdrücklich eingeschränkt. [L42, rohe Zeilen 441, 470, 627, 659, 697, 807, 867–944, 1000–1459 und 1524–1525]

### 6.8 Weitere beobachtete Meldungen ohne bewiesenen Ursachenzusammenhang

Im Startlog L42 stehen ALSA-Probe-Fehler, ein anfänglicher Sample-Verlust und ein zu spät produzierter erster TX-Block. Die Basisstation startet danach weiter. Diese Meldungen dürfen nicht ohne zeitlichen und messtechnischen Zusammenhang als Erklärung für alle späteren Geräteverluste dienen.

`NoCircuitFree` ist eine Ressourcenentscheidung, `UItsiDetach` eine Teilnehmermeldung, `RoamingLocationUpdating` ein Mobilitäts-/Registrierungsereignis und ein Rust-Panic ein Prozessfehler. Sie sind nicht austauschbare Bezeichnungen für „gekickt“.

## 7. Erreichter historischer Stand und Testgrenzen

| Gegenstand | Tatsächlich sichtbarer Nachweis | Nicht daraus ableitbar |
|---|---|---|
| Beide RF-Träger konfiguriert | Abgeleitete Carrier 720 und 721 im Startlog. | Erfolgreicher Ruf über jeden Slot. |
| Registrierung / Affiliation | ITSI-Attach, Accept und Gruppen-Affiliation in L35/L42. | Asterisk-Sprachqualität oder Lastfestigkeit. |
| Zusätzliche Bearer | L38: Call-ID 9, 10, 11 auf logischen TS5, TS6, TS7; Circuit-Open auf C2-Air-TS2/3/4. | Jeder denkbare Rufmodus und jedes Endgerät funktioniert. |
| Asterisk-Aufbau | In L38 korrespondierende `AsteriskEntity: media ready`-Meldungen. | Vollständige Audio-, DTMF-, Gegenrichtungs- und Release-Abnahme. |
| Früher Panic | L37 enthält Panic und neue PID. L38 enthält diesen Fehler nicht. | Generell crashfreier Dauerbetrieb. |
| Gruppen-Release | L39/L41 enthalten Deferred-Close und physischen Close. L41 zeigt abgearbeitete STCH-Blöcke. | Endgerät hat jeden Release tatsächlich empfangen und ist sauber zurückgekehrt. |
| ACK-Carrierkorrektur | Quellcode A7/H2/R4 vorhanden. | Vollständige Nachher-Lastserie mit allen Slots und ACK-Verlustszenarien. |
| Kapazitätsgrenze | TS2–TS7-Reservierungen und Congestion in L42. | Sechs gleichzeitig etablierte Sprachverbindungen in genau diesem Test. |
| TS8-Modell v2.7 | Quellcodepaket vorhanden. | Erfolgreicher siebter Traffic-Ruf; kein passender Nachtest im Verlauf. |
| Secondary-Control v2.8 | Letzte Benutzerentscheidung und entsprechendes 13-Dateien-Paket. | Vollständiger Common-SCCH-Betrieb oder bestätigte Entlastung der Endgeräte. |
| Deaktivierter Carrier 2 | Implementierte Kapazitäts-/Konfigurationspfade. | Dokumentierte Aus-/Ein-Umschaltmatrix unter Last. |
| EchoLink und sonstige Integrationen | Bestehende Komponenten erwähnt; EchoLink im späteren Startlog deaktiviert. | Erfolgreiche Regression dieser Integrationen. |

### 7.1 Konkrete Secondary-Aufbauereignisse aus L38

Um **00:29:02.167** wird Asterisk-Call 9 mit `ts=5` weitergeleitet; anschließend werden DL und UL auf Carrier 721 / Air-TS2 geöffnet und Media-Ready gemeldet. Um **00:29:18.359** ist die entsprechende Folge für Call 10 / TS6 / Air-TS3 sichtbar; um **00:29:18.528** für Call 11 / TS7 / Air-TS4. Die maßgeblichen Fundstellen sind die rohen Dateizeilen 490, 512–514, 1748, 1771–1773, 1811 und 1835–1837.

Das ist stärker als ein bloßer PHY-Empfang auf 721 und belegt, dass die erweiterte Rufsteuerung tatsächlich Secondary-Ressourcen öffnete. Es bleibt schwächer als eine durch den Benutzer bestätigte verständliche Sprachverbindung in beiden Richtungen über eine längere Lastserie.

### 7.2 Heute zur Archivierung tatsächlich ausgeführte Prüfungen

- Die **zwölf ZIP-Dateien** wurden geöffnet, ihre Dateilisten und SHA-256-Werte erfasst und mit `ZipFile.testzip()` auf CRC-Fehler geprüft: **kein CRC-Fehler**.
- Relevante Quelltextstände wurden verglichen, insbesondere v1/v2/v2.1/v2.2 sowie v2.7/v2.8 und das letzte Release-Verhalten in v2.8.
- Die **zwei eingebetteten JavaScript-Blöcke** aus dem historischen v2.8-`html.rs` wurden extrahiert und mit `node --check` geprüft: **beide Exitcode 0**.
- Die acht Logdateien wurden auf Rufaufbau, Zuweisung, ACK-Routing, Panic, Freigabe und Kapazitätsfehler untersucht.
- Aktuelle GitHub-Dateien wurden an einem festen Commit gelesen; alter Repository-Name und zwei historische Commitanker wurden aufgelöst.

**Nicht ausgeführt:** Rust-/Cargo-Compile, Rust-Unit-Tests, Browser-Interaktion, SDR-/Endgerätetest, Asterisk-/Brew-Audiotest, Last- oder Langzeittest. In der verfügbaren lokalen Umgebung war keine Rust-Toolchain vorhanden. Ein lokaler Git-Clone scheiterte am DNS-Zugriff auf GitHub; der GitHub-Connector funktionierte für die Repository-Prüfung. Die JavaScript-Syntaxprüfung ist kein Browser- oder Belegungstest.

## 8. Dienste, Konfiguration und technische Betriebsparameter

Die Werte dieses Abschnitts sind **historisch beobachtete Testparameter**, keine heute remote ausgelesene Live-Konfiguration. [L42]

### 8.1 Funk und SDR

| Parameter | Beobachteter Wert |
|---|---|
| Hauptträger | Carrier 720; DL 418.000000 MHz; UL 408.000000 MHz |
| Sekundärträger | Carrier 721; DL 418.025000 MHz; UL 408.025000 MHz |
| Trägerabstand | 25 kHz zwischen den beiden beobachteten Trägern |
| SDR-TX-Mitte | 418.012500 MHz, expliziter Center-Override |
| SDR-RX-Mitte | 408.012500 MHz, expliziter Center-Override |
| Sample-Rate | 600000 Samples/s |
| Frequenzkorrektur | 0.00 ppm im Log |
| Gerät / Treiber | SXceiver; `driver=sx, label=sx`; SoapySX |
| Hardware | Version 1.2; erkannter Takt 38.4 MHz |
| SoapySX-Version | `9705147dd8c189625071f3f163ea56119bda4a05` |
| RX-/TX-Kanal | Jeweils 0 |
| Antennenbezeichnung | RX beziehungsweise TX |
| RX-Gain | LNA 42.0, PGA 16.0 |
| TX-Gain | DAC 9.0, MIXER 30.0 |
| Treiberargumente | RX und TX jeweils `period=900` |
| Hardwarezeit | `use_get_hardware_time=true` |
| Netzidentität | MCC 901, MNC 1510, Colour Code 1 |
| Scrambling-Code im Log | 3779454471; nicht als geheimer Kryptoschlüssel zu interpretieren |

Die explizite SDR-Mitte liegt zwischen den beiden Carriern. Sie darf nicht mit der logischen Main-Carrier-Frequenz verwechselt werden. Aus den Gain-Werten wird weder eine tatsächliche abgestrahlte Leistung noch eine konkrete HF-Qualität abgeleitet.

### 8.2 Systeme und Schnittstellen

| Gegenstand | Historischer Wert / Hinweis |
|---|---|
| Basisstationshost | `SRV-M-TBS-01` |
| Benutzer-Arbeitsverzeichnis | `/home/jan/flowstation`, im Dialog `~/flowstation` |
| Tatsächlich belegter systemd-Dienst | **`tetra.service`** |
| Prozess-/Binaryname im Journal | `bluestation-bs` |
| Konfigurierter Service-Control-Name | `tetra` |
| Dashboard | HTTP auf `0.0.0.0:8080`; Basic Auth aktiv; anonyme Read-only-Übersicht im Log aktiv |
| Brew-Verbindung | WebSocket `ws://10.0.1.22:8081` |
| Asterisk | SIP-Integration aktiv; erfolgreiche Routingbeispiele für Wählfolge `91201` → SIP-Ziel `201` |
| Unbekanntes lokales Ziel | ISSI 2091201 wird in mehreren Tests über Brew geroutet; kein identischer Nachweis wie beim expliziten SIP-Ziel `201` |
| SIP-/RTP-Ports | Im ausgewerteten Verlauf nicht belastbar angegeben; keine Standardports als Istwerte ergänzt |
| SDS-Protokolldatei | `/home/jan/flowstation/sds_log.json`; 26 geladene Einträge im Startlog |
| Health Monitor | Intervall 300 s; Watchdog-Restart im Startlog ausgeschaltet |
| Weitere gestartete Komponenten | Snom-Notify-Worker und Telegram-Alerter im Log; keine Abnahme ihrer Funktionen |
| Deaktivierte Integrationen im Startlog | EchoLink, MeshCom, DAPNET und GeoAlarm |

Testidentitäten im Verlauf: ISSI 5102, 2010002 und 2020001–2020006; Gruppen GSSI 15201 und 15501. Diese Angaben dienen der Zuordnung der Logs, nicht der Festlegung eines neuen NetCore-Nummernplans.

### 8.3 DualCarrier-Konfigurationssemantik

Das heute geprüfte Dashboard-Modul beschreibt die Trennung zwischen gespeicherter Carrier-Nummer und Betriebsfreigabe:

```toml
[cell_info]
main_carrier = 720
secondary_carrier = 721
dual_carrier_enabled = true
```

Das ist ein **Schemaauszug ohne Zugangsdaten**, keine vollständige Betriebskonfiguration. Bei `dual_carrier_enabled = false` bleibt die gespeicherte Secondary-Nummer erhalten, die effektive `CfgCellInfo::secondary_carrier` soll aber `None` sein. Der vorhandene Code beschreibt den Umschalter als Konfigurationsänderung mit kontrolliertem Neustart, nicht als Live-Umbau aller aktiven Scheduler. Fehlt der Schalter, ist der dokumentierte Kompatibilitätsdefault `true`; ohne konfigurierte Secondary-Nummer entsteht dennoch kein zweiter aktiver Carrier. [R8]

Deshalb sind **UI-Schalter, gespeicherte TOML-Datei, effektive Startkonfiguration und tatsächlich laufendes Binary** getrennt zu prüfen. Ein Service-Neustart beim Umschalten beendet laufende Rufe; das wäre nicht dasselbe Fehlerbild wie ein unerwarteter Panic unter unveränderter Konfiguration.

## 9. Relevante Dateien und Paketabhängigkeiten

### 9.1 Vollständige Pfadliste des letzten Ersatzdateienpakets

```text
crates/tetra-core/src/timeslot_alloc.rs
crates/tetra-entities/src/cmce/components/circuit_mgr.rs
crates/tetra-entities/src/cmce/subentities/cc_bs/pdu.rs
crates/tetra-entities/src/cmce/subentities/cc_bs/procedures/setup.rs
crates/tetra-entities/src/cmce/subentities/cc_bs/procedures/individual.rs
crates/tetra-entities/src/cmce/subentities/cc_bs/procedures/isi.rs
crates/tetra-entities/src/cmce/subentities/sds_bs.rs
crates/tetra-entities/src/llc/llc_bs_ms.rs
crates/tetra-entities/src/umac/umac_bs.rs
crates/tetra-entities/src/umac/subcomp/bs_defrag.rs
crates/tetra-entities/src/umac/subcomp/circuit_mgr.rs
crates/tetra-entities/src/umac/subcomp/bs_sched.rs
crates/tetra-entities/src/net_dashboard/html.rs
```

| Datei / Gruppe | Bedeutung für diesen Chat |
|---|---|
| `timeslot_alloc.rs` | Eindeutiges Eigentum der logischen Bearer, Kapazität drei oder sechs; zwischenzeitlich sieben in v2.7. |
| CMCE `components/circuit_mgr.rs` | Logische Rufressourcen, größere Arrays, kapazitätsabhängige Vergabe. Nicht mit dem gleichnamigen unteren UMAC-Modul verwechseln. |
| CMCE `pdu.rs` | Carrier-Hint, physische Timeslot-Bitmaske, Kapazitätshilfe, Gruppen-/Einzelruf-Release. |
| `setup.rs`, `individual.rs`, `isi.rs` | Aufbau und Zuordnung der verschiedenen lokalen beziehungsweise Netz-/Asterisk-/Brew-Rufpfade. |
| `sds_bs.rs` | Signalisierungs-/SDS-Pfade dürfen Traffic-Bearer auf C2 nicht als C1-Slots behandeln. Eine Paketänderung ist kein SDS-Funktionstest. |
| `llc_bs_ms.rs` | Logischer Link im ACK-Rückweg; zentrale Korrektur v2.4. |
| `umac_bs.rs` | Zuordnung Carrier/Air-TS, Auswahl des Schedulers, Medien, Floor, Hangtime, Inaktivität und verzögerter Close. |
| `bs_defrag.rs` | Trennung von Reassembly-Zuständen für zusätzliche logische Ressourcen. |
| UMAC `subcomp/circuit_mgr.rs` | Pro Carrier vier physische Slots, Validierung vor Arrayzugriffen. |
| `bs_sched.rs` | Traffic versus Signalisierung, Stealing, Broadcast, Secondary-Modus und physische Slotproduktion. |
| Historisches `net_dashboard/html.rs` | Damals großer Rust-String mit eingebettetem HTML/JS; Carrier-/TS-Kacheln und Belegung. |

### 9.2 Weitere Anschlussstellen für eine spätere Fortsetzung

Relevante weitere Pfade im Repository beziehungsweise ursprünglichen Upload sind `crates/tetra-core/src/tetra_common.rs`, `crates/tetra-config/src/bluestation/sec_cell.rs`, `crates/tetra-config/src/bluestation/sec_phy_soapy.rs`, `crates/tetra-saps/src/lcmc/fields/chan_alloc_req.rs`, `crates/tetra-saps/src/control/call_control.rs`, die PHY-/LMAC-Komponenten und die Medienintegrationen `net_asterisk`, `net_brew`, `net_echolink`.

Im heutigen Archiving-Stand liegen UI-Inhalte zusätzlich unter:

```text
crates/tetra-entities/src/net_dashboard/ui/dashboard.html
crates/tetra-entities/src/net_dashboard/ui/netcore-rf.js
crates/tetra-entities/src/net_dashboard/ui/netcore-rf.css
crates/tetra-entities/src/net_dashboard/ui/netcore.js
crates/tetra-entities/src/net_dashboard/ui/netcore.css
crates/tetra-entities/src/net_dashboard/dual_carrier.rs
```

Nicht jede dieser Anschlussstellen wurde in diesem Archivauftrag vollständig auditiert. Die tatsächlich gelesenen aktuellen Kerndateien und ihre Git-Blobs sind in Abschnitt 11 aufgeführt.

## 10. Befehle, Deployment und Reparatur: ausgeführt oder nur vorgeschlagen?

### 10.1 Historisch vorgeschlagene Build-/Einspielbefehle

Im Dialog wurden wiederholt folgende Abläufe vorgeschlagen:

```bash
cd ~/flowstation
unzip -o /pfad/zu/<jeweiliges-komplette-dateien-paket>.zip
cargo check
cargo build --release
```

Beim v2.2-Wechsel zusätzlich:

```bash
grep -nE 'remember_traffic_carrier|selected carrier' \
  crates/tetra-entities/src/umac/umac_bs.rs
cargo clean -p tetra-entities
cargo build --release
```

**Status:** Dies waren Anweisungen des Assistenten, keine von ihm auf Jans Anlage ausgeführten Befehle. Die E0425-Ausgabe belegt einen fehlgeschlagenen Compileversuch. Laufzeitlogs belegen gestartete Folgestände, aber nicht lückenlos, mit welchem Buildbefehl und welcher installierten Dateikombination jeder Prozess erstellt wurde. Eine komplette erfolgreiche Cargo-Ausgabe und ein verifiziertes Installationsziel fehlen.

**Heutige Einordnung:** Die obigen ZIP-Befehle sind historische Reproduktionsinformationen. **Nicht als Empfehlung verstehen, v2.8 über den heutigen Branch zu entpacken.** Aktuelle Quelltexte sind weiterentwickelt; insbesondere die UI-Dateistruktur wurde geändert. `cargo clean` stellt außerdem nicht sicher, dass ein Service anschließend genau das neu erzeugte Binary startet.

### 10.2 Tatsächlich sichtbarer Diagnosebefehl

Im Benutzer-Log L37 ist dieser Aufruf enthalten:

```bash
journalctl -u tetra.service -f | \
  grep -E 'carrier=721|carrier=720|U-SETUP|D-SETUP|accepting|rejecting|NoCircuitFree|channel|traffic|circuit|CMCE|UMAC'
```

**Status:** Benutzerseitiger Aufruf und Ausgabe sind dokumentiert. Die früher vom Assistenten vorgeschlagenen Varianten mit `-u bluestation-bs` waren nicht als korrekter Unitname belegt. Für diesen historischen Host ist `tetra.service` der beobachtete Dienstname.

Der Filter kann entscheidende Fehlermeldungen ausschließen, wenn sie kein passendes Wort enthalten. Für einen neuen Reproduktionstest sollte deshalb zunächst ein **ungefilterter** Ausschnitt gesichert und erst die Kopie gefiltert werden.

### 10.3 Neue, nicht ausgeführte Prüfbefehle für die Fortsetzung

Die folgenden Befehle sind **Vorschläge aus dieser Abschlussprüfung**, keine in diesem Chat bereits ausgeführten Anlagenaktionen:

```bash
# Auf der betroffenen Basisstation: Source- und Dienstidentität aufnehmen.
cd ~/flowstation
git status --short
git rev-parse HEAD
systemctl show tetra.service \
  -p MainPID -p ExecStart -p FragmentPath -p NRestarts -p ExecMainStatus

# Ungefilterte Logdatei eines klar begrenzten Testfensters sichern.
journalctl -u tetra.service --since '15 minutes ago' \
  --no-pager -o short-iso-precise > "$HOME/tetra-dualcarrier-test.log"
```

Vor Weitergabe der Ausgaben Geheimnisse und gegebenenfalls Zugangsdaten in Startargumenten redigieren. Der tatsächliche Installationspfad des vom Dienst gestarteten Binaries muss aus `ExecStart` beziehungsweise der Unit ermittelt werden; er wird hier nicht erfunden.

Geeignete Suchanker in einer **Kopie** des Logs:

```bash
grep -E 'logical ts|auto-ack|NoCircuitFree|FACCH|Deferred|Closed|panicked|Main process exited|RoamingLocationUpdating|No .* circuit to close' \
  "$HOME/tetra-dualcarrier-test.log"
```

Ein neuer Lasttest muss zu jeder Beobachtung Build-Commit, lokale Änderungen, effektive Carrierkonfiguration und beteiligte MS-/Firmwarestände festhalten. Ein sichtbarer Mapper-Marker allein reicht nicht.

### 10.4 Historische Startmarker

```text
UMAC dual-carrier logical-timeslot mapper v2.2 active
UMAC dual-carrier logical-timeslot mapper v2.8 active (C2 TS1 control/guard, traffic TS5-TS7)
```

Der heutige Code enthält weiterhin den v2.8-Marker, obwohl zahlreiche spätere Änderungen vorhanden sind. Die Zeile ist damit ein Hinweis auf die Mapper-Familie, **kein belastbarer vollständiger Versionsnachweis**. [L42, R2]

## 11. Gesondert überprüfter Repository-Stand vom 03.10.2026

Die folgenden Befunde stammen aus der **heutigen statischen Quellcodeprüfung**, nicht aus nachträglich erfundenen historischen Tests. Referenz ist `Archiving@311462808d6ebe49b0b426bec11f381d6867f676`. Der unmittelbar vor dem Schreiben nachgeladene Commit `f8f1e001779cf564982c83abad4f5fa6b7637cd5` änderte an diesen Dateien nichts.

Der Vergleich mit `main@6aa9be8f74ab731f72dc133a5f8e90c5018c626d` zeigte beim anfänglichen Archiving-Prüfstand zehn zusätzliche Commits. Dazu gehören **auch UI- und Serviceänderungen**, nicht nur Archive. Die hier relevanten RF-/CMCE-/LLC-Kerndateien waren in dieser Differenz nicht als geändert aufgeführt; die Dashboard-Dateistruktur dagegen schon. Die Branches dürfen daher nicht pauschal als identisch behandelt werden. [R0]

### 11.1 Ergebnisübersicht

| Bereich | Heute im Repository nachgewiesen | Verhältnis zum historischen Abschluss |
|---|---|---|
| Allocator | TS2–TS7, Kapazität drei/sechs, C2-Air-TS1 nicht als Traffic; zusätzlicher Eigentümer SNDCP. | Grundmodell v2.8 erhalten, zusätzliche gemeinsame Nutzung. |
| CMCE-Hilfsfunktionen | `SECONDARY_CARRIER_HINT=-2`, Air-TS-Umrechnung minus 3, Kapazität nach effektiv vorhandenem Secondary. | Entspricht dem finalen Mapping. |
| UMAC | Ein Scheduler pro Carrier, `SecondaryBcchNoMcch`, logische/physische Umrechnung, v2.8-Marker. | Keine Rückkehr zum siebten Traffic-Bearer. |
| Unterer CircuitMgr | Validierung physischer TS1–TS4 vor Arrayzugriffen. | Schutz gegen den konkret beobachteten Typ von TS5-Zugriff weiterhin vorhanden. |
| LLC-ACK | Logischer `link_id` bleibt erhalten, Secondary-Hint und physische Bitmaske. | v2.4-Grundkorrektur vorhanden. |
| Normaler Gruppenrelease | Zweimal FACCH/STCH plus MCCH-Fallback. | **Abweichung vom finalen ZIP**, das noch die konservative v2.5-Variante enthält. |
| Secondary-Hangtime/Broadcast | Idle-Unterdrückung berücksichtigt tatsächliche Allocation und obligatorische BSCH-/BNCH-Zeitpunkte. | Wichtige spätere Änderung gegenüber v2.8-ZIP. |
| Scheduler-Prioritäten | Grants und in Resource integrierte Grants vor Hintergrundsignalisierung; Behandlung von Fragmenten und ACKs. | Weiterentwickelter Last-/Signalisierungspfad. |
| UI | `html.rs` bindet externe Build-Assets ein; RF-Ansicht spiegelt die Carrier-/Air-TS-Daten der Belegungskacheln. | Alte monolithische Ersatzdatei ist nicht mehr kompatibler Zielstand. |
| Betriebsabnahme | Kein Zugriff auf Jans laufendes SDR-/MS-System in diesem Auftrag. | Nicht durch Repository-Lektüre ersetzt. |

### 11.2 Konkreter später geänderter Fehlerpfad: Secondary wird fälschlich still

Im **historischen v2.8-ZIP**, `bs_sched.rs` um Zeilen 1215–1254, werden `dl_is_traffic` und `ul_is_traffic` während Hangtime beziehungsweise Frame 18 auf false gesetzt. Der frühe Secondary-Idle-Zweig verwendet anschließend genau diese Traffic-Flags:

```text
Secondary/TrafficOnly-Modus
AND !dl_is_traffic
AND !ul_is_traffic
→ leerer Slot, keine DL-Blöcke
```

Dadurch unterscheidet dieser Zweig nicht ausreichend zwischen **unbelegt** und **zugeteilt, aber gerade Signalisierung/Broadcast statt Sprache erforderlich**. Das ist ein im alten Artefakt nachvollziehbarer problematischer Codepfad. Seine Rolle als alleinige Ursache eines bestimmten Geräteverlusts wurde in diesem Archivauftrag nicht durch einen RF-Test reproduziert.

Im **heutigen Code** verwendet derselbe Bereich dagegen Allocation-Flags und schützt obligatorische Broadcast-Zeitpunkte:

```text
!dl_allocated && !ul_allocated
&& !ts.is_mandatory_bsch()
&& !ts.is_mandatory_bnch()
```

Damit wird der frühe Silence-Zweig für bereits zugeteilte Bearer und die genannten obligatorischen Broadcast-Zeitpunkte vermieden. Zusätzlich wird für AACH/BBK dieselbe Hangtime-/Traffic-Entscheidung verwendet, die vor dem Entnehmen des letzten FACCH-Elements getroffen wurde. Das verhindert in diesem Pfad eine widersprüchliche Klassifikation von Nutzblock und Access-Assignment. **Status: statisch geändert; keine heutige Funkabnahme.** [A11, R5]

### 11.3 Heutiger Release-Pfad

`release_group_call()` sendet im aktuellen PDU-Modul bei vorhandenem aktivem Call zwei Release-PDUs mittels `build_sapmsg_stealing`, danach einen normalen MCCH-Fallback. Diese Logik gilt nicht nur für C2. Weitere Sonderfälle wie das explizite Aufheben eines Emergency-Originator-Zustands besitzen eigene Nachrichtenfolgen.

Der aktuelle Einzelruf-Release ist ebenfalls nicht mit dem normalen Gruppenrelease identisch: Für aktive lokale Beine sind wiederholte FACCH-Releases vorgesehen; im Rufaufbau erfolgt MCCH-Signalisierung. Die alte v2.5-Regel darf deshalb nicht pauschal als allgemeine Vorgabe für alle heutigen Disconnects angewandt werden. [R3]

### 11.4 Offener Lifecycle-Prüfpunkt: verzögerter Close und schnelle Wiederverwendung

Der heutige CMCE-Gruppenrelease gibt die logische Ressource frei, während UMAC den physischen Close noch bis zum STCH-Drain verzögern kann. Die gelesene UMAC-Struktur `PendingCircuitClose` enthält DL-/UL-Flags je logischem Slot, aber keinen sichtbaren Generationsbezug zu der alten Belegung.

Daraus ergibt sich ein **Prüfkandidat**: Kann eine schnelle Neuvergabe desselben logischen Slots erfolgen, bevor der alte Close abgeschlossen ist, und könnte dieser dann die neue Belegung beeinflussen? Der Archivauftrag hat diesen Ablauf nicht reproduziert und erklärt ihn nicht nachträglich zur bewiesenen Ursache des historischen Lastfehlers.

Ein geeigneter Entwurf wäre eine explizite Freigabebestätigung oder eine mit Call-/Usage-/Generationskennung verknüpfte Ressourcenfreigabe. Die konkrete Lösung muss mit der bestehenden Queue- und Zustandsmaschinenlogik entwickelt werden. [R2, R3]

### 11.5 Weitere heutige Anschlussstellen

**UL-Inaktivität:** Ein UL-fähiger Circuit allein aktiviert im heutigen UMAC nicht mehr automatisch den Stuck-Transmitter-Timer. Lokales `FloorGranted` startet die Erwartung von Uplink-Sprache; Remote-Floor und Hangtime behandeln sie anders. Dies ist wichtig, damit Netz-/Shared-Calls ohne lokalen Sprecher nicht fälschlich auslaufen. Der hier betrachtete Code ist weiterentwickelt gegenüber den frühen pauschalen Aussagen im Chat. [R2]

**Gemeinsamer Allocator:** SNDCP kann über eine Präferenzreihenfolge Secondary-Ressourcen bevorzugen und teilt das Eigentumsmodell mit Voice. Drei Tests sind im Source vorhanden: `sndcp_reservation_blocks_voice_until_release`, `preferred_allocation_can_keep_main_carrier_free` und `multiple_sndcp_bearers_share_allocator_with_voice`. **Vorhandene Testfunktionen sind nicht in diesem Auftrag ausgeführte Tests.** [R1]

**Legacy-Vergabe:** Der CMCE-CircuitMgr enthält neben kapazitätsabhängigen Aufrufen weiterhin Legacy-Helfer, deren freie Suche nur TS2–TS4 umfasst. Das Vorhandensein dieser Helfer beweist nicht, dass der heutige Haupt-Rufpfad falsch ist; bei neuen Aufrufern muss aber die korrekte Variante gewählt werden. [R9]

**Carrier-Fallbacks:** Einige UMAC-Helfer fallen bei unbekanntem Carrier oder nicht vorhandenem Secondary auf den Primary zurück. Ob dies für jeden Eingangsweg erwünscht ist, sollte geprüft werden. Eine fehlgeschlagene Secondary-Anforderung darf nicht still eine bereits belegte Primary-Ressource treffen. Dies ist ein Hardening-Kandidat, kein hier reproduzierter Fehler im normalen Deaktivierungspfad. [R2]

**LLC-Zustandsidentität:** Der aktuelle Kommentar zur ACK-Integration weist darauf hin, dass Teile des Basic-Link-Zustands weiterhin nach SSI statt nach dem Paar aus Teilnehmer und Link geführt werden. Für parallele Signalisierungskontexte desselben Teilnehmers wäre zu prüfen, ob eine feinere Zuordnung erforderlich ist. [R4]

**Medien:** Der aktuelle UMAC besitzt zusätzlich eine zentrale Medienanbindung mit Session-Bindung an die logische Ressource, Prüfung auf aktiven DL-Circuit und ausstehenden Close sowie einem begrenzten Drain-Budget von 16 Frames pro Tick. Diese heutige Ergänzung gehörte nicht zur historischen Asterisk-Abnahme dieses Chats. [R2]

### 11.6 Heutige WebUI: keine alten Komplettdateien darüberkopieren

Im geprüften Archiving-Stand besteht `crates/tetra-entities/src/net_dashboard/html.rs` nur noch aus Build-Asset-Einbindungen, beispielsweise `include_str!("ui/dashboard.html")`. Die neue RF-Ansicht in `ui/netcore-rf.js` liest `data-carrier`, `data-ts` und `data-air-ts` der existierenden Timeslot-Kacheln und gruppiert diese nach Carrier. Sie verändert dadurch nicht die Ressourcenvergabe.

Die historische vollständige `html.rs` aus v2.8 würde diese Architektur zurücksetzen. Ein heutiger Anzeigefix muss an der aktuellen Telemetrie und den getrennten UI-Assets ansetzen. **Keine der historischen UI-Dateien wurde in diesem Archivauftrag in Produktivcode übernommen.** [R7]

### 11.7 Identifikatoren der tatsächlich geprüften aktuellen Kerndateien

Alle folgenden Blob-SHAs gehören zum eingefrorenen Archiving-Prüfstand:

| Datei | Git-Blob-SHA |
|---|---|
| `crates/tetra-core/src/timeslot_alloc.rs` | `21994bb67733df5c68922a026ae793c09d5a0e0c` |
| `crates/tetra-entities/src/umac/umac_bs.rs` | `6106362325b75dab5dc2d8bc9a08b6b07ce9469c` |
| `crates/tetra-entities/src/cmce/subentities/cc_bs/pdu.rs` | `4f13c6d9368af9824fb6c4923306bbc5ea2cfc39` |
| `crates/tetra-entities/src/llc/llc_bs_ms.rs` | `90f6bbfda3499308857f76ff11efe0db37a457cb` |
| `crates/tetra-entities/src/umac/subcomp/bs_sched.rs` | `4214f2ad580e0e563ded76b445785f56f6d08ed3` |
| `crates/tetra-entities/src/umac/subcomp/circuit_mgr.rs` | `a7379b294502849d4e593963e14aebcd2dd498d5` |
| `crates/tetra-entities/src/cmce/components/circuit_mgr.rs` | `747356ce0b6a8800f6f24d64aa51dfa124d60d7e` |
| `crates/tetra-entities/src/net_dashboard/html.rs` | `caf9cd6932346141021ed2f4ef62fbc79562fece` |
| `crates/tetra-entities/src/net_dashboard/ui/netcore-rf.js` | `deb39f2d637ed5dc7b0701c9ab34e836398b3a5f` |
| `crates/tetra-entities/src/net_dashboard/dual_carrier.rs` | `765f3c3ef8da26a9e1e51b60ec2364a161bf80aa` |

Dies ist eine gezielte Prüfung der wichtigen Aussagen dieses Chats, kein vollständiger Audit aller Dateien des Repositories, aller Nebenbranches oder der `ms-mode`-Kopie des Stacks.

## 12. Verworfene, ersetzte und nicht bewiesene Ansätze

| Ansatz / Aussage | Einordnung am Abschluss |
|---|---|
| Ein-Datei-UMAC-Remapping schafft vollständigen DualCarrier-Betrieb. | Durch die folgenden Ressourcen- und Indexfehler widerlegt; überholt. |
| Nach Beheben des `Todo`-Imports bleiben wahrscheinlich nur noch kleine Typfehler, kein Konzeptproblem. | Zu selbstsichere damalige Einschätzung. Die späteren Laufzeitfehler zeigen weitere Schicht- und Deploymentprobleme. |
| Registrierung und Affiliation beweisen Asterisk-/Brew-Kompatibilität. | Nicht ausreichend; nur die beobachteten Registrierungs-/Affiliate-Pfade sind bestätigt. |
| Sechs logische Ressourcen bedeuten sechs gleichzeitig etablierte Gespräche. | Zu pauschal; Setup-Reservierungen, mehrere Funkbeine und heute Paketdaten berücksichtigen. |
| C2-Air-TS1 müsse grundsätzlich immer unbenutzt beziehungsweise Control sein. | Keine allgemeingültig nachgewiesene TETRA-Grenze. Hier ist die Reservierung eine konkrete letzte Projektentscheidung. |
| Phantom-TS8 als freier Spare-Slot in einem Sechs-Bearer-Modell. | Als irreführende Anzeige ersetzt. |
| Sieben Traffic-Bearer durch `TrafficOnly` auf C2 inklusive Air-TS1. | Als v2.7-Artefakt vorhanden, aber durch die letzte Benutzerentscheidung und v2.8 ersetzt. |
| Zwei FACCH-Releases plus MCCH seien nachweislich ein Überlastungsgewitter. | Durch L41 nicht belegt; nicht als gesicherte Fehlerursache übernehmen. |
| Ein Release ist stets stabiler als wiederholte Zustellung. | Nicht nachgewiesen; v2.5 war ein Versuch. Heutiger Code verwendet wieder Wiederholungen. |
| Ein reservierter, als CTRL beschrifteter TS1 ist automatisch ein vollständig genutzter zusätzlicher Steuerkanal. | Nicht nachgewiesen; Broadcast, reservierte Ressource und SCCH-Prozeduren getrennt prüfen. |
| Ein Startmarker identifiziert das gesamte installierte Release. | Unzureichend; der Marker v2.8 ist auch im heutigen weiterentwickelten Code vorhanden. |

## 13. Offene Aufgaben und Roadmap-Kandidaten

Die folgende Priorisierung ist eine **aus dieser Archivprüfung abgeleitete Empfehlung**. Historisch ausdrücklich vereinbart sind der Erhalt der Integrationen, das optionale Carrier-Verhalten, korrekte UI-Belegung und zuletzt die Secondary-TS1-Reservierung zugunsten der Stabilität. Neue Tickets, Zuständigkeiten, Termine oder verbindliche Sprintzusagen werden hier nicht erfunden.

### P0 – Verifizierbaren Ausgangspunkt und belastbare Reproduktion herstellen

**DC-01: Source, Build und laufendes Binary zusammenführen.** Den heute tatsächlich eingesetzten Commit, lokale Änderungen, den Installationspfad, effektive Konfiguration und die Service-Startzeit erfassen. Abhängigkeit für alle weiteren Tests. Abnahmekriterium: Ein Testlog lässt sich eindeutig einer Dateikombination zuordnen; keine Mischung aus v2.7- und v2.8-Mapping.

**DC-02: Secondary-Control präzise spezifizieren.** Festlegen, ob die abschließende Benutzeranforderung durch BCCH/SYNC plus reserviertem Slot erfüllt werden soll oder ob zusätzlicher, wirklich von MS verwendeter Steuerverkehr vorgesehen ist. Bei letzterem Rolle, Ankündigung, Auswahl, Uplink-Zugang und Rückkehr anhand des Air-Interface-Protokolls ausarbeiten. Abnahmekriterium: Nicht nur eine CTRL-Kachel, sondern eine nachvollziehbare, spezifizierte Nachrichtenfolge und passender Endgerätetest.

**DC-03: Vorhandenes Hangtime-/Broadcast-Hardening nicht zurücksetzen.** Den aktuellen `dl_allocated`-/`ul_allocated`-Pfad einschließlich obligatorischer BSCH/BNCH und AACH-Konsistenz im reproduzierbaren Test bestätigen. Das alte v2.8-Paket nicht als vermeintlich sichere Rückfallversion über den aktuellen Scheduler kopieren.

### P1 – Lifecycle und Interoperabilität unter Last absichern

**DC-04: Release und Wiederverwendung derselben Ressource.** Schnelle Folge aus PTT-Ende, Hangtime, Release, STCH-Drain und neuer Belegung prüfen; insbesondere mögliche alte Deferred-Closes gegen neue Calls absichern. Auch Abbruch eines noch nicht unten geöffneten externen Rufaufbaus testen. Kein unbewiesenes Rennen als bereits bestätigten Defekt behandeln.

**DC-05: ACK-Routing für alle sechs Bearer.** RX-Link, Carrier-Hint, physische Bitmaske und tatsächlichen Scheduler korrelieren. Gleichzeitige Belegung gleicher Air-TS auf beiden Carriern ist der wesentliche Regressionstest. ACK-Verlust, Wiederholung und kombinierte ACK-/Daten-Nachrichten berücksichtigen.

**DC-06: Integrationsmatrix Asterisk/Brew.** Lokaler Gruppenruf, lokaler Einzelruf, Asterisk-Ruf in beiden Richtungen, Brew-Ruf, Abbruch während Setup, Auflegen von beiden Seiten, Timeout, DTMF soweit im jeweiligen Pfad unterstützt und Netzunterbrechung testen. Medienfluss und saubere Ressourcenfreigabe getrennt protokollieren. Weitere vorhandene Integrationen erst nach ausdrücklicher Aktivierung sinnvoll testen.

**DC-07: Disabled-Pfad und ungültige Bearer.** UI aus/ein mit dokumentiertem Neustartverhalten; ohne Secondary keine TS5–TS7-Zuteilung. Ungültige IDs dürfen keinen Panic auslösen und keine andere Carrier-Ressource verändern. Fehlende oder unbekannte Carrier nicht unbemerkt auf einen belegten Main-Slot umleiten.

**DC-08: Volllast und Ressourcenknappheit.** Belegung von einer bis sechs Ressourcen, danach weitere Anforderung. Erwartung: kontrollierte Ablehnung oder spezifizierte Prioritätsentscheidung, kein Prozessneustart und kein Eingriff in unabhängige Calls. Reservierungen und offene RF-Circuits dabei getrennt zählen; heutige SNDCP-Belegung mit berücksichtigen.

### P2 – Wartbarkeit, Diagnose und Anzeige

**DC-09: Ressourcenidentität zentralisieren.** Gemeinsame, getestete Abbildung von logischer Bearer-ID zu Carrier/Air-TS statt wiederholter Bereichsprüfungen in CMCE, LLC, SDS, UMAC und JavaScript. Längerfristig typisierte Kennungen für physische Timeslots, logische Bearer und Carrier-Hints erwägen. Bei Defragmentierung und teilweise empfangenen STCH-Blöcken auch die Carrier-/Link-Trennung prüfen.

**DC-10: UI zeigt Zustände statt nur Aktivität.** Freie Kapazität, Setup-Reservierung, offener Circuit, aktive Sprache, Hangtime und Pending-Release unterscheiden. Für beide Carrier physische Air-Slots anzeigen, die logische ID ergänzen und den reservierten C2-Air-TS1 nicht als freie Traffic-Ressource zählen. Die aktuelle ausgelagerte UI-Architektur verwenden.

**DC-11: Lastdiagnose messbar machen.** Queue-Tiefe je Carrier/Slot, Bearer-Eigentümer, Release-Latenz, Inaktivitätsereignisse, Grants/ACK-Wiederholungen, verlorene Samples und TX-Deadline-Verletzungen korrelieren. Nicht dauerhaft jeden PHY-Burst auf INFO ausgeben, ohne die Wirkung auf die Testbedingungen zu messen.

**DC-12: Reproduzierbare Auslieferung.** Für spätere Fixes vollständige Dateien weiterhin mit manifestiertem Basiscommit und Prüfsummen liefern; idealerweise CI-Prüfungen und ein eindeutig versioniertes Gesamtpaket. Einzelfile-Links dürfen nicht still verschiedene Mappinggenerationen vermischen.

### Historische Nebenidee ohne Freigabe

Es wurde vorgeschlagen, einen Traffic-Bearer für Gruppenruf/Emergency/Priorität zu reservieren, damit Asterisk-/Einzelrufe nicht alle Ressourcen belegen. Beispiel war eine Reserve auf dem letzten Secondary-Traffic-Bearer. **Status: Idee**, nicht als beschlossene Policy und nicht als in diesem Chat implementierte Prioritätslogik behandeln. Eine Umsetzung müsste mit dem heutigen gemeinsamen Allocator und vorhandener Pre-emption abgestimmt werden; konkrete Klassen, Ausnahmen und Kapazitätskosten sind noch festzulegen.

### Empfohlene Abnahmematrix für die nächste Fortsetzung

| Test | Erwarteter Nachweis |
|---|---|
| Carrier 2 deaktiviert | Effektiv ein Carrier, drei Ressourcen, keine Phantom-C2-Belegung. |
| Carrier 2 aktiviert | Sechs Ressourcen mit der finalen minus-3-Zuordnung. |
| Gleichzeitiger C1-Air-TS2- und C2-Air-TS2-Ruf | Medien, ACK und Close bleiben getrennt. |
| Alle C2-Traffic-Slots einzeln und parallel | Logische IDs 5/6/7 passen zu Air-TS2/3/4; Sprache und Control stimmen. |
| Hangtime über Multiframe-/Frame-18-Grenze | Erforderliche Signalisierung und Broadcasts bleiben erhalten. |
| Gruppenrelease auf beiden Carriern | Endgeräte kehren zurück; Ressourcen werden frei; keine fremden Calls betroffen. |
| Sofortige Wiederbelegung nach Release | Kein alter Close oder altes Media-Frame trifft den neuen Call. |
| Abgebrochener externer Rufaufbau | Reservierung wird frei, auch ohne vorherigen RF-Circuit-Open. |
| Siebte Ressourcenanforderung im finalen Sechs-Bearer-Modell | Kontrollierte Knappheitsbehandlung ohne Neustart. |
| Unterschiedliche MS-Fähigkeiten / Firmware | Pro Gerät dokumentierte Ergebnisse statt pauschaler Herstellerannahme. |
| Normaler Einträgerbetrieb nach Rückschaltung | Vorhandene Asterisk-/Brew-/SDS-Pfade regressionsfrei im definierten Testumfang. |

Diese Matrix wurde hier **nicht ausgeführt**.

## 14. Quellen- und Anhangsmanifest

Die Originalanhänge werden durch diesen Auftrag **nicht zusätzlich in Git hochgeladen**. Das Archiv enthält deren Namen, Prüfsummen, relevante Beobachtungen und Auszüge. Für eine spätere bitgenaue Reproduktion müssen die Original-ZIPs beziehungsweise Journalausschnitte weiterhin verfügbar sein. Das Archivieren des Chats wird hier nicht mit einer gesicherten dauerhaften Ablage aller Originalbytes gleichgesetzt.

### 14.1 ZIP-Artefakte

Alle Größen beziehen sich auf die heute zugänglichen Dateien. Dateizahl zählt reguläre ZIP-Einträge ohne Verzeichnisse. Zwölf Archive wurden geprüft; A11 enthält 13 Quellcodedateien.

| Kürzel | Dateiname | Bytes | Dateien | Einordnung |
|---|---|---:|---:|---|
| A0 | `flowstation-beta.zip` | 1228359 | 433 | Ursprünglicher Source-Upload; nicht als heutiges Deployment verwenden. |
| A1 | `flowstation-dual-carrier-fix-complete-files.zip` | 20615 | 1 | Erster UMAC-only-Versuch. |
| A2 | `flowstation-dual-carrier-fix-v2-complete-files.zip` | 91925 | 9 | Sechs logische Bearer, vor Importkorrektur. |
| A3 | `flowstation-dual-carrier-fix-v2_1-complete-files.zip` | 91949 | 9 | Importkorrektur. |
| A4 | `flowstation-dual-carrier-fix-v2_2-complete-files.zip` | 93458 | 10 | Konsistenter Mapper und physischer CircuitMgr-Schutz. |
| A5 | `flowstation-webui-ts5-8-fix-complete-files.zip` | 131359 | 1 | Erste UI-Erweiterung mit Spare-TS8. |
| A6 | `flowstation-dual-carrier-release-fix-v2_3-complete-files.zip` | 16135 | 1 | Mehrfacher Traffic-Release plus MCCH. |
| A7 | `flowstation-dual-carrier-ack-fix-v2_4-complete-files.zip` | 10063 | 1 | LLC-ACK-Carrierkontext. |
| A8 | `flowstation-dual-carrier-release-conservative-v2_5-complete-files.zip` | 16109 | 1 | Konservativer normaler Gruppenrelease. |
| A9 | `flowstation-webui-ts1-7-fix-v2_6-complete-files.zip` | 131368 | 1 | Spare-TS8 entfernen. |
| A10 | `flowstation-dual-carrier-ts8-v2_7-complete-files.zip` | 255635 | 13 | Sieben Traffic-Ressourcen; später verworfen. |
| A11 | `flowstation-dual-carrier-secondary-ts1-control-v2_8-complete-files.zip` | 255604 | 13 | Letztes Paket: C2-TS1 reserviert, sechs Ressourcen. |

SHA-256 der Archive:

```text
A0  9fa3691e67407e170f0d0e217e15e8ba4950ca85e341104871295e14487bf4b3
A1  840285142db408e6535a826ee2e9e9d9b24f71c4f8a8ebd5f27ce3597bcb0083
A2  72cdf3c6dc2c72906692d9613c483e818f2f2e30757fbd0b6b775a7b0b90a8b3
A3  f15045b0ac5c1a9a28001074dea3d92f9bcd9976e396eb7c385b1ad40eb3dfa8
A4  56eb27d54ca7955713896385f2a1f315ecdfa8b6b47ee06eb801483a6c456619
A5  1dd13309bae8a355e7197567e132ecec843b31b1aa9d7b7b2058548343f5f70d
A6  ae92af3192a4fdeb7970c06bc43c932bea66c9c665c3f043ee59df79d7c9abdf
A7  1625ca09306ef4ff386a7866bf5b5d6ab4726479e40cf8b15c6becd1babd87fb
A8  1026b78b94f05a11fd936e8e36373747a9032658ef9a5fbdc54e6d5b5bd21b47
A9  0757780da34574eed0fb318b7e489124877641d1b3a718b7b63fe118f6eb3303
A10 b2d436f1980f3cc23d4b3335f3af3381a0022333015fbc36075e791b0fda0577
A11 a12062cfeff87b8a421d77ef79d62f6204e176cf85e90141e1f526623bec7947
```

Zusätzlich waren zahlreiche der vollständigen Rust-Dateien einzeln im Chat bereitgestellt. Teilweise wurden dabei Pfade früherer Versionen erneut verwendet. Für die Versionsrekonstruktion wurden deshalb vorzugsweise die klar bezeichneten ZIP-Inhalte verwendet, nicht allein eine frühere Downloadbeschriftung. Ein im Chat ausgegebener Dateilink belegt für sich weder einen Git-Commit noch ein Deployment.

### 14.2 Betriebslogs

Die hier genannten Zeilenzahlen beziehen sich auf die lokal zugänglichen Textdateien, nicht auf Zeilen des Rust-Codes und nicht auf verkürzte Suchausschnitte.

| Kürzel | Datei | Bytes | Zeilen | Hauptbeitrag |
|---|---|---:|---:|---|
| L34 | `Eingefügter Text(34).txt` | 134034 | 522 | Carrier-2-PHY/Remapping, aber `NoCircuitFree`. |
| L35 | `Eingefügter Text(35).txt` | 42657 | 124 | Registrierung und Affiliation, kein Traffic-Nachweis. |
| L37 | `Eingefügter Text(37).txt` | 309666 | 1392 | Gefiltertes Journal, alter Mapper, TS5-Panic und Neustart. |
| L38 | `Eingefügter Text(38).txt` | 1063886 | 4744 | Secondary-Circuit-Open auf TS5/6/7 und Asterisk-Media-Ready. |
| L39 | `Eingefügter Text(39).txt` | 384971 | 1355 | C2-Gruppenruf, Hangtime/Release, falscher ACK-Kontext. |
| L40 | `Eingefügter Text(40).txt` | 57944 | 157 | Logischer Link7, ACK auf C1-TS4, Wiederholung und Roaming-Refresh. |
| L41 | `Eingefügter Text(41).txt` | 11244 | 33 | Zwei STCH-Releases, Queue-Drain, Close nach 114 ms. |
| L42 | `Eingefügter Text(42).txt` | 547282 | 1619 | Startparameter, sechs Brew-Setup-Reservierungen, Congestion und Freigabe. |

SHA-256 der Logs:

```text
L34 6a702f776866b69699938af69b8b9e3e8815f68441321c491a7bbc0bebebc918
L35 993531a41dafeee0453678e8f4dbf47ab8175fea78ed54ab495f0c62fbd72a69
L37 3f5a9052adbe694e4cc48135526051bb9c9f9ced3e15debf81937e70f96318e0
L38 d91573f3a7541e48b181b1d5b8bf14f65246887ee490f1e116a085fd68cf78bb
L39 214d709bb292ad5221734c64f77ada233949912e600be430390cab90d500aa52
L40 2fb34f1c43455c9107b12c03c6824003fb1becd2f39692b266c799607d9c2525
L41 7dbac9d4140decb523c05a70b53b5fa9d33ec459e605473bcd66df2dfdab0335
L42 d3d8a5a54b1d28f9e551aa81f3b024c11d08704e7a9eebf8a8c26ffeaf528013
```

Die Nummerierung der Anhänge ist nicht fortlaufend. Daraus wird nicht auf zusätzliche, tatsächlich im Chat vorhandene, aber fehlende Tests geschlossen.

### 14.3 Verfügbare ETSI-Unterlagen

Die nachstehenden PDFs waren im Projekt-/Gesprächskontext verfügbar. Ihre bloße Anwesenheit bedeutet nicht, dass sie während der historischen Fixfolge bereits geprüft oder die Implementierung gegen sie abgenommen wurde. **Für dieses Archiv wurden Titel/Versionen beziehungsweise Dokumentidentitäten erfasst und nur die ausdrücklich genannten Air-Interface-Fundstellen gezielt inhaltlich geprüft.** Die Sammlung wurde nicht vollständig neu gelesen. Entwürfe werden nicht als verabschiedete Endfassung bezeichnet; ein aktueller ETSI-Statusabgleich war nicht Gegenstand dieser Archivierung.

| Datei | Dokument / Rolle |
|---|---|
| `en_30039201v010601p.pdf` | EN 300 392-1 V1.6.1, General network design. |
| `en_30039202v030801p.pdf` | **N1:** EN 300 392-2 V3.8.1 (2016-08), Air Interface, 1445 Seiten; für Control-/SCCH-Abgrenzung relevant. |
| `en_3003920303v010301p.pdf` | EN 300 392-3-3 V1.3.1, ISI Group Call. |
| `en_3003920304v010301p.pdf` | EN 300 392-3-4 V1.3.1, ISI Short Data Service. |
| `en_3003920308v010401p.pdf` | EN 300 392-3-8 V1.4.1, Generic Speech Format Implementation. |
| `en_3003920313v010201p.pdf` | EN 300 392-3-13 V1.2.1, transportunabhängiger ISI Group Call. |
| `en_3003920315v010500a.pdf` | **Draft** EN 300 392-3-15 V1.5.0 (2026-04), ISI Mobility Management. |
| `en_30039205v020701p.pdf` | EN 300 392-5 V2.7.1, Peripheral Equipment Interface. |
| `en_30039207v030501p.pdf` | EN 300 392-7 V3.5.1, Security. |
| `en_30039209v010701p.pdf` | EN 300 392-9 V1.7.1, allgemeine Anforderungen an Supplementary Services. |
| `en_3003921006v010401p.pdf` | EN 300 392-10-6 V1.4.1, Call Authorized by Dispatcher. |
| `en_3003921018v010301p.pdf` | EN 300 392-10-18 V1.3.1, Barring of Outgoing Calls. |
| `en_3003921101v010201p.pdf` | EN 300 392-11-1 V1.2.1, Call Identification, Stage 2. |
| `en_3003921114v010101p.pdf` | EN 300 392-11-14 V1.1.1, Late Entry. |
| `en_3003921117v010102p.pdf` | EN 300 392-11-17 V1.1.2 laut Titelseite, Include Call. |
| `en_3003921201v010202p.pdf` | EN 300 392-12-1 V1.2.2, Call Identification, Stage 3. |
| `en_3003921216v010400a.pdf` | **Draft** EN 300 392-12-16 V1.4.0 (2026-03), Pre-emptive Priority Call. |
| `en_30039401v030301p.pdf` | EN 300 394-1 V3.3.1, Radio-Conformance-Tests. |
| `en_30039502v010303p.pdf` | EN 300 395-2 V1.3.3 (2025-02), TETRA-Speech-Codec. |
| `en_300812v020101p.pdf` | EN 300 812 V2.1.1 laut Titelseite, SIM-ME. |
| `es_20081201v020205p.pdf` | ES 200 812-1 V2.2.5, UICC physische/logische Eigenschaften. |
| `es_20081202v020401m.pdf` | **Final draft** ES 200 812-2 V2.4.1, TSIM-Anwendung. |
| `ets_30039214e01v.pdf` | **Final draft** prETS 300 392-14, September 1997, PICS-Proforma. |
| `ts_10081201v020205p.pdf` | TS 100 812-1 V2.2.5, UICC-Schnittstelle. |
| `ETSI.pdf` | 4100-seitige Zusammenstellung; keine einzelne zusätzliche Normversion. |

**Gezielt herangezogene Fundstellen N1:** Abschnitt 19.3.2.1.1, gedruckte Seiten 579–580, Control-Channel-Betriebsformen; Abschnitt 16.10.45, gedruckte Seite 412, SCCH-Informationsparameter. Diese Fundstellen ersetzen keinen vollständigen Entwurf des beabsichtigten Secondary-Control-Verfahrens.

## 15. Unveränderliche Repository-Quellen

Die folgenden Links referenzieren bewusst den geprüften Commit und nicht einen später beweglichen Branch. Die Links dienen der Fortsetzung und Nachprüfung; Git-Blobs stehen zusätzlich in Abschnitt 11.7.

- **R0 – Repository-Prüfstand:** [Archiving-Prüfcommit](https://github.com/JanHG98/netcore-tetra/commit/311462808d6ebe49b0b426bec11f381d6867f676), [Vergleich mit dem aufgelösten main-Stand](https://github.com/JanHG98/netcore-tetra/compare/6aa9be8f74ab731f72dc133a5f8e90c5018c626d...311462808d6ebe49b0b426bec11f381d6867f676).
- **R1 – Allocator:** [crates/tetra-core/src/timeslot_alloc.rs](https://github.com/JanHG98/netcore-tetra/blob/311462808d6ebe49b0b426bec11f381d6867f676/crates/tetra-core/src/timeslot_alloc.rs).
- **R2 – UMAC:** [crates/tetra-entities/src/umac/umac_bs.rs](https://github.com/JanHG98/netcore-tetra/blob/311462808d6ebe49b0b426bec11f381d6867f676/crates/tetra-entities/src/umac/umac_bs.rs); geprüft insbesondere Initialisierung/Mapping, zentrale Medien, Circuit-Open/-Close, Pending-Close, Inaktivität und Floor-Steuerung.
- **R3 – CMCE-PDU:** [crates/tetra-entities/src/cmce/subentities/cc_bs/pdu.rs](https://github.com/JanHG98/netcore-tetra/blob/311462808d6ebe49b0b426bec11f381d6867f676/crates/tetra-entities/src/cmce/subentities/cc_bs/pdu.rs); geprüft Carrier-Hilfsfunktionen und Release-Pfade.
- **R4 – LLC:** [crates/tetra-entities/src/llc/llc_bs_ms.rs](https://github.com/JanHG98/netcore-tetra/blob/311462808d6ebe49b0b426bec11f381d6867f676/crates/tetra-entities/src/llc/llc_bs_ms.rs); geprüft ACK-Scheduling und ACK-Ausgabe.
- **R5 – Scheduler:** [crates/tetra-entities/src/umac/subcomp/bs_sched.rs](https://github.com/JanHG98/netcore-tetra/blob/311462808d6ebe49b0b426bec11f381d6867f676/crates/tetra-entities/src/umac/subcomp/bs_sched.rs); geprüft Prioritäten, Signalisierungsqueue, Slotfinalisierung, Hangtime, Broadcast und BBK.
- **R6 – Physischer CircuitMgr:** [crates/tetra-entities/src/umac/subcomp/circuit_mgr.rs](https://github.com/JanHG98/netcore-tetra/blob/311462808d6ebe49b0b426bec11f381d6867f676/crates/tetra-entities/src/umac/subcomp/circuit_mgr.rs).
- **R7 – UI:** [Build-Asset-Einbindung html.rs](https://github.com/JanHG98/netcore-tetra/blob/311462808d6ebe49b0b426bec11f381d6867f676/crates/tetra-entities/src/net_dashboard/html.rs) und [RF-Ansicht netcore-rf.js](https://github.com/JanHG98/netcore-tetra/blob/311462808d6ebe49b0b426bec11f381d6867f676/crates/tetra-entities/src/net_dashboard/ui/netcore-rf.js).
- **R8 – DualCarrier-Schalter:** [crates/tetra-entities/src/net_dashboard/dual_carrier.rs](https://github.com/JanHG98/netcore-tetra/blob/311462808d6ebe49b0b426bec11f381d6867f676/crates/tetra-entities/src/net_dashboard/dual_carrier.rs), insbesondere dokumentierte Neustartsemantik und Konfigurationsrepräsentation.
- **R9 – Logischer CMCE-CircuitMgr:** [crates/tetra-entities/src/cmce/components/circuit_mgr.rs](https://github.com/JanHG98/netcore-tetra/blob/311462808d6ebe49b0b426bec11f381d6867f676/crates/tetra-entities/src/cmce/components/circuit_mgr.rs).
- **H1 – ZIP-Kommentar aufgelöst:** [86648c2a766bcc5298d1eeb158606e131192a5ac](https://github.com/JanHG98/netcore-tetra/commit/86648c2a766bcc5298d1eeb158606e131192a5ac).
- **H2 – Historischer ACK-Fix:** [d00c4e03849898aa137e0f1de09a2897cc76d695](https://github.com/JanHG98/netcore-tetra/commit/d00c4e03849898aa137e0f1de09a2897cc76d695).

Für die übrigen Chat-Pakete wurden keine exakten Commit-/PR-Zuordnungen nachgewiesen. Auch wenn ähnliche Funktionen im heutigen Code vorhanden sind, ist das keine nachträgliche Bestätigung, wann genau jedes ZIP eingecheckt oder auf dem Funkrechner installiert wurde.

## 16. Auswertungslücken und Übergabe

**Zugänglich und ausgewertet:** Der hier verfügbare Dialog einschließlich späterer ausdrücklicher Korrekturen; die acht zugehörigen Logdateien; der ursprüngliche Source-ZIP und elf Fix-/UI-ZIPs; relevante aktuelle Repository-Dateien; begrenzte einschlägige ETSI-Fundstellen. Die Prüfsummen ermöglichen eine spätere Zuordnung der konkret verwendeten Anhänge.

**Nicht verfügbar beziehungsweise nicht belegt:** Ursprünglicher Chattitel und Chatlink; vollständige Zeitstempel jeder Nachricht; genaue Source-/Binary-Zuordnung jedes historischen Testprozesses; vollständige Build- und Installationsprotokolle; Ende-zu-Ende-Audio- und Dauerlastabnahme; Nachtest der letzten Pakete v2.7/v2.8; vollständige MS-Modell-/Firmwareliste; vollständiges ungefiltertes Anlagenjournal außerhalb der eingesandten Ausschnitte; eine komplette SCCH-/ETSI-Konformitätsprüfung.

Ein Teil der frühen Assistentenantworten verwendete zu weitgehende Erfolgsaussagen. Für die Fortsetzung gilt daher folgende belastbare Zusammenfassung:

1. **Die Ressourcen- und Carrier-Awareness wurde tatsächlich erweitert.** Secondary-Circuit-Open und Asterisk-Media-Ready sind historisch für logische TS5–TS7 sichtbar.
2. **Der TS5-Panic und das ACK-Routing ohne Carrierkontext sind konkrete, belegte Fehler.** Ihre jeweiligen Korrekturen sind in Artefakten und heute im relevanten Code nachweisbar; der ACK-Fix zusätzlich in einem historischen Commit.
3. **Die letzte gewünschte Kapazität ist sechs Traffic-Bearer, mit reserviertem C2-Air-TS1.** Der vorangegangene Sieben-Bearer-Versuch ist überholt. Ein vollständig genutzter zusätzlicher Steuerkanal ist dadurch noch nicht abgenommen.
4. **Release-Zuverlässigkeit und Laststabilität sind nicht abschließend bestätigt.** Die Behauptung, drei Release-Sendungen hätten die Überlastung bewiesen, darf nicht übernommen werden.
5. **Der heutige Repository-Stand ist der Ausgangspunkt weiterer Arbeiten.** Er enthält relevante Hangtime-/Broadcast-Verbesserungen, wiederholte Release-Zustellung und eine neue UI-Struktur. Historische Komplettdateien sind Vergleichsmaterial, keine aktuelle Einspielanleitung.

Der Chat selbst wird durch diesen Auftrag nicht archiviert; das übernimmt der Benutzer nach Prüfung der gespeicherten Dokumentation.
