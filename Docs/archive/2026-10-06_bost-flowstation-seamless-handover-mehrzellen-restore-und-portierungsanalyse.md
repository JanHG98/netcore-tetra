# Bost-FlowStation Multi-Cell/Handover: technische Abschlussdokumentation und Portierungsanalyse

> **Archivstatus:** Dieser Text dokumentiert den gesamten für diesen Chat zugänglichen fachlichen Verlauf und trennt die damalige Analyse ausdrücklich vom am 2026-10-06 erneut geprüften Repository-Stand. Die Archivierung selbst implementiert keine Handover-Funktion in NetCore-Tetra.

## 1. Metadaten und Quellenumfang

| Feld | Wert |
|---|---|
| Thema | Bost-FlowStation: Multi-Cell, Zellwechsel, Call Restore, Late Entry und Übertragbarkeit auf NetCore-Tetra |
| Ursprünglicher Chattitel | Nicht verfügbar; dieser Archivtitel ist beschreibend |
| Ursprünglicher Chatlink / Chat-ID | Nicht verfügbar |
| Erstellungsdatum dieser Abschlussdokumentation | 2026-10-06 |
| Repository | https://github.com/JanHG98/netcore-tetra |
| Ausschließlicher Zielbranch | Archiving |
| Geprüfter Archiving-Ausgangscommit vor diesem Archivschreibvorgang | 0f3433bcf16befea50e565f7d472478890c37610 |
| Zusätzlich lesend geprüfter NetCore-Hauptzweig | main@9116c15d645458f99e236712b67a1ad970432791 |
| Ursprünglich im Chat geprüfter Bost-Snapshot | 3d476eea |
| Heute erneut geprüfter Bost-Hauptzweig | main@96671561dfc61af6612f087305ebbed15a7ba5d1 |
| Externes Vergleichsrepository | https://github.com/Aitorrio/bost-flowstation |
| Archivdatei | Docs/archive/2026-10-06_bost-flowstation-seamless-handover-mehrzellen-restore-und-portierungsanalyse.md |
| Archivindex | Docs/archive/README.md |
| Schreibumfang | Nur diese Archivdatei und der Archivindex; keine Produktdateien, keine Roadmap außerhalb des Archivs, kein Merge |

### 1.1 Zugänglicher Chatverlauf

Der zugängliche Verlauf dieses Chats besteht fachlich aus zwei Schritten:

1. Der Nutzer verwies auf Aitorrio/bost-flowstation und fragte, ob die dort vermutete Seamless-Handover-Implementierung für mehrere Zellen herausgelöst und an NetCore-Tetra angepasst werden kann.
2. Darauf folgte eine Quellcodeanalyse mit dem damaligen Ergebnis: gezielte Übernahme ja, jedoch im damals geprüften Bost-Snapshot kein hinreichend belegter vollständig integrierter Mehrzellen-Handover. Gefunden wurden Nachbarzellenausstrahlung, Late Entry, lokale Call-Restore-Logik sowie zwei wichtige Restore-Fixes.

Der heutige Repository-Abgleich zeigt, dass Bost nach dem damals geprüften Snapshot erheblich weiterentwickelt wurde. Diese spätere Entwicklung ist der wichtigste Nachtrag dieser Abschlussdokumentation.

### 1.2 Anhänge und Bilder

Im aktuellen Chat wurden keine eigenständigen Bilder, Screenshots oder Binäranhänge übermittelt. Deshalb gibt es für diesen Chat keine Chatbilder, die zusätzlich unter Docs/archive/ hätten eingecheckt werden können.

Im Projektkontext sind zahlreiche ETSI-PDFs vorhanden. Für dieses Thema sind insbesondere folgende bereits verfügbare Normen fachlich relevant:

- EN 300 392-11-14: Supplementary Service Late Entry.
- EN 300 392-3-3: ISI Group Call einschließlich Group Call Restoration.
- Draft EN 300 392-3-15: ISI Mobility Management einschließlich Unterstützung von Call Restoration bei Migration.
- EN 300 392-2: Air Interface, insbesondere MLE-/Zellreselektionsprozeduren.

Diese Normen wurden für diesen Archivlauf nur zur Einordnung der im Code verwendeten Begriffe herangezogen. Es wurde keine vollständige Konformitätsprüfung gegen alle normativen Muss-Anforderungen durchgeführt. Die PDFs wurden nicht erneut in Docs/archive/ dupliziert.

### 1.3 Nachweisstufen

| Status | Bedeutung in diesem Dokument |
|---|---|
| **Idee** | Im Chat erwogen, aber weder beschlossen noch im Repository nachgewiesen |
| **Beschlossen/geplant** | Als sinnvoller Projektweg festgelegt, aber noch nicht als Produktcode nachgewiesen |
| **Implementiert** | Im ausdrücklich genannten Repository/Commit als Code nachgewiesen |
| **Getestet** | Ein konkreter Test wurde ausgeführt und sein Ergebnis ist belegt |
| **Im Betrieb bestätigt** | Verhalten wurde an realen NetCore-TETRA-Systemen bzw. Funkgeräten im laufenden Betrieb nachgewiesen |

Quellcode mit Unit-Tests bedeutet in dieser Dokumentation nur, dass Tests im Repository vorhanden sind. Es bedeutet nicht automatisch, dass sie in diesem Archivlauf ausgeführt oder mit realen Funkgeräten bestätigt wurden.

---

## 2. Ziel, Ausgangslage und behandelte Fragestellung

Die Kernfrage war, ob sich die in Bost-FlowStation entwickelte Mehrzellen-/Handover-Logik technisch als Grundlage für NetCore-Tetra verwenden lässt.

Das Ziel war ausdrücklich **nicht**, den kompletten Fremdfork unbesehen zu mergen. Gesucht waren vielmehr:

- wiederverwendbare TETRA-Protokollbausteine,
- Zustandslogik für Zellwechsel und Rufwiederherstellung,
- Mechanismen für gruppenweite Sprachkontinuität über mehrere Zellen,
- Schutz gegen doppelte Sprechfreigaben und hängende Traffic-Slots,
- eine belastbare Trennung zwischen lokalem Call-Identifier und netzweiter Rufidentität,
- Hinweise darauf, welche Teile wegen der NetCore-Architektur neu adaptiert werden müssen.

Die ursprüngliche Antwort kam zu dem Schluss, dass eine gezielte Portierung sinnvoll ist. Dieser Grundsatz bleibt gültig. Die Einschätzung zum Umfang des bereits vorhandenen Bost-Handover-Codes muss jedoch durch den heutigen Stand deutlich nach oben korrigiert werden.

---

## 3. Wichtigste Korrektur gegenüber der ursprünglichen Chatantwort

### 3.1 Damaliger Stand: Bost bei 3d476eea

Im damals geprüften Snapshot 3d476eea war die Lage tatsächlich so, wie im Chat beschrieben:

- crates/tetra-entities/src/net_site/mod.rs existierte dort noch nicht.
- crates/tetra-entities/src/cmce/subentities/cc_bs/procedures/restoration.rs arbeitete mit dem lokal bekannten call_identifier.
- Ein unbekannter Ruf aus einer Schwesterzelle wurde nicht anhand der GSSI auf einen lokalen aktiven Gruppenruf abgebildet.
- crates/tetra-entities/src/mle/components/broadcast.rs konnte Nachbarzellen in D-NWRK-BROADCAST ausstrahlen, setzte die Cell-Re-Select-Parameter jedoch auf 0.
- Die zwei wichtigen Restore-Sicherheitsfixes aus c71c9ad waren bereits enthalten.

Damit war die damalige Aussage, dass Nachbarzellen + Restore + Late Entry allein noch keinen vollständigen Seamless-Handover belegen, für genau diesen Snapshot sachlich nachvollziehbar.

### 3.2 Heutiger Stand: Bost main@96671561dfc61af6612f087305ebbed15a7ba5d1

Der aktuelle Bost-Hauptzweig enthält dagegen eine explizite Multi-Cell-Site-Architektur mit eigener Mobility- und Handover-Logik. Insbesondere wurden Ende September 2026 mehrere Phasen ergänzt.

Damit ist die frühere pauschale Aussage **überholt**, Bost besitze keinen integrierten zellübergreifenden Handover-Pfad. Korrekt ist heute:

> **Bost besitzt inzwischen konkrete, zusammenhängende Multi-Cell-Mechanismen für Zellreselektion, Zielzellenvorbereitung, Registrierung, Call-Restore, gruppenweiten Medienpfad, standortweite Sprechrechtskoordination, Einzelrufe zwischen Zellen sowie SIP/Asterisk-Anbindung. Diese Mechanismen sind für NetCore sehr wertvolle Referenz- und Portierungskandidaten. Sie sind jedoch wegen der unterschiedlichen NetCore-Systemarchitektur nicht als blindes Komplett-Cherry-Pick zu behandeln.**

Die Bezeichnung seamless darf für NetCore erst nach gemessenen Ende-zu-Ende-Tests mit realen Funkgeräten verwendet werden. Der Quellcode zeigt Kontinuitätsmechanismen; eine gemessene maximale Audio-Unterbrechung wurde in diesem Chat nicht festgelegt oder nachgewiesen.

---

## 4. Historisches Ergebnis des Chats

### 4.1 Nachbarzellenausstrahlung

**Implementiert im damals geprüften Bost-Snapshot und in NetCore vorhanden:** D-NWRK-BROADCAST kann eine Liste von CA-Nachbarzellen enthalten. Der damalige Bost- und NetCore-Code in crates/tetra-entities/src/mle/components/broadcast.rs hatte denselben Blob 9bed0569dd928327298c3b1363147674be37d69c.

Der wichtige damalige Schluss bleibt gültig: Eine Nachbarzellenliste allein ist noch kein Rufhandover. Sie ermöglicht dem Mobilgerät nur, eine Zielzelle zu kennen und anhand des TETRA-Verhaltens eine Reselection vorzubereiten bzw. auszuführen.

### 4.2 Late Entry

**Implementiert im Bost-Kontext:** Late Entry ermöglicht einem Teilnehmer, in einen bereits laufenden Gruppenruf einzusteigen. Der Chat identifizierte dies als relevant für den Zellwechsel, weil ein Teilnehmer nach Ankunft in der Zielzelle wieder in einen dort bereits aktiven Gruppenruf einsteigen kann.

Die ETSI-Systematik bestätigt die konzeptionelle Trennung: Late Entry ist ein Mechanismus zum Beitritt zu einem bestehenden Point-to-Multipoint-Ruf. Es ist nicht identisch mit der vollständigen Mobility-/Call-Restore-Prozedur eines laufenden Zellwechsels.

### 4.3 Zwei Restore-Fixes aus c71c9ad51462bbc64fb7669f9b192d5f6f325d8b

Der Commit trägt die Nachricht:

    fix(cmce): fix floor and timeslot leaks, add group-call late entry

Er wurde im Chat als unmittelbarer Portierungskandidat identifiziert.

#### Fix A: keine doppelte Sprechfreigabe beim Restore eines Einzelrufs

**Implementiert in Bost, heute in NetCore main weiterhin nicht vollständig vorhanden.**

Beim Einzelruf darf ein U-CALL RESTORE nicht einfach erneut Sprechrecht vergeben, wenn der andere Teilnehmer bereits Sprecher ist. Bost prüft den bestehenden floor_holder bzw. ob derselbe Teilnehmer das Sprechrecht schon hält. Bei Simplex wird der interne Floor-Zustand passend gesetzt.

Ohne diese Prüfung besteht das Risiko zweier gleichzeitig als sendeberechtigt betrachteter Teilnehmer sowie eines inkonsistenten internen Sprechrechtszustands.

#### Fix B: Uplink-Inaktivitätsüberwachung nach Gruppenruf-Restore

**Implementiert in Bost, heute in NetCore main weiterhin nicht vollständig vorhanden.**

Wenn ein Gruppenruf-Restore dem wiederkehrenden Teilnehmer das Sprechrecht gibt, reicht D-CALL RESTORE allein nicht. Bost erzeugt zusätzlich den passenden Floor-Grant-Pfad bis in die darunterliegende Funksteuerung. Dadurch wird die Uplink-Inaktivitätsüberwachung aktiviert.

Ohne diese Meldung kann ein Teilnehmer nach erfolgreichem Restore schweigen, während der Traffic-Slot bis zu einem späteren absoluten Timeout belegt bleibt. Genau dieses Fehlerbild beschreibt c71c9ad.

### 4.4 Ursprüngliche Architekturentscheidung

**Beschlossen/geplant:** Keine komplette Übernahme des Fremdforks. Stattdessen sollten einzelne semantisch klar abgegrenzte Fixes und Protokollteile in NetCore integriert werden.

Diese Entscheidung bleibt auch nach dem heutigen Bost-Abgleich richtig. Neu ist lediglich, dass der Pool wiederverwendbarer Referenzlogik wesentlich größer geworden ist.

---

## 5. Aktueller Bost-Multi-Cell-Stand

### 5.1 Relevante Commit-Folge

| Commit | Inhalt | Bedeutung für NetCore |
|---|---|---|
| c71c9ad51462bbc64fb7669f9b192d5f6f325d8b | Restore-Floor-/Timeslot-Leak-Fixes und Group-Call Late Entry | Niedrig gekoppelte Sicherheits-/Stabilitätsfixes; zuerst portieren |
| c949a16a33524cac75e20dd96eef640fc3d120e1 | Multi-cell phase 4: Calls zwischen Zellen über SiteSwitch | Architekturreferenz für Gruppen-/Einzelruf und Medienkopie |
| da7f6263d5f8593a1273c6dcbca3c4c05ac90937 | Multi-cell phase 5: Mobility between cells | Kernreferenz für Reselection, alte Registrierung und Call Restore |
| 44be7e440a7d1a27866b69300c0978531e146202 | Multi-cell phase 6: Cells-API/UI | Verwaltungsmodell für Zellen |
| 65c38e9910124bd40676d9db6465e3d19da62725 | Asterisk/WX auf allen Zellen und Group SDS | Referenz für SIP/Medienrelay und standortinterne SDS |

### 5.2 SiteSwitch

Datei:

    crates/tetra-entities/src/net_site/mod.rs
    crates/tetra-entities/src/net_site/switch.rs

**Implementiert im aktuellen Bost.**

Die Architektur ist nicht nur ein kleiner Restore-Handler, sondern ein standortweiter Vermittler zwischen mehreren eigenständigen Zellstacks.

Vereinfacht:

    Funkgerät
       |
       v
    Zelle A / eigener Stack ----                                                                   SiteSwitch ---- Netzwerk/Brew
                                 /
    Zelle B / eigener Stack ----/
       |
       +---- SiteRelay ---- Asterisk/SIP

Wesentliche Aufgaben des SiteSwitch:

- Registrierung eines ISSI einer konkreten Zelle zuordnen.
- GSSI-Affiliations zellbezogen nachhalten.
- Netzrufsignalisierung nur an die tatsächlich beteiligten Zellen verteilen.
- Gruppenrufe auf mehrere Zellen auffächern.
- Uplink-Sprache eines lokalen Sprechers auf die Hörzellen kopieren.
- Downlink-Sprache aus dem Netz an die beteiligten Zellen verteilen.
- Standortweit nur einen Sprecher je Gruppe zulassen bzw. Prioritäten behandeln.
- Einzelrufe zwischen Teilnehmern verschiedener Zellen direkt verbinden.
- SDS zwischen lokalen Zellen direkt routen.
- lokale Call-Identifier gegenüber der Netzseite auf standortweit eindeutige Identifier abbilden.
- Netzlinkzustand auf die Schwesterzellen spiegeln.
- temporäre Session-/Handover-Zustände zeitlich bereinigen.

### 5.3 SiteDirectory

Datei:

    crates/tetra-entities/src/net_site/directory.rs

**Implementiert im aktuellen Bost.**

Der zentrale Zustand enthält im Wesentlichen:

- ISSI -> aktuelle CellId
- (CellId, ISSI) -> Menge der zugehörigen GSSIs

Bei einer Registrierung desselben ISSI auf einer anderen Zelle erkennt die Directory den Move. Die alte Zellzuordnung wird entfernt. Eine verspätete Deregistrierung der alten Zelle darf die bereits neue Registrierung nicht wieder löschen.

Diese Semantik ist für NetCore besonders wichtig, weil reale Mobilität asynchrone Meldungen erzeugt. Eine simple last-message-wins-Logik ohne Zellkontext wäre fehleranfällig.

### 5.4 Globaler und lokaler Call-Identifier

Datei:

    crates/tetra-entities/src/net_site/switch.rs

**Implementiert im aktuellen Bost.**

Bost verwendet eine CallIdMap:

- lokal: (CellId, u16 call_id)
- standortweit/netzseitig: eigener u16 Identifier
- Rückabbildung: global -> (CellId, lokaler call_id)

Damit ist genau das Problem adressiert, das im ursprünglichen Chat als Architekturvorgabe identifiziert wurde: Ein lokaler call_id darf nicht als global eindeutige Rufidentität behandelt werden.

Für eine verteilte NetCore-Architektur sollte dieser Gedanke eher noch konsequenter umgesetzt werden, z. B. mit einer stabilen internen UUID/Session-ID und separaten lokalen Air-Interface-Call-IDs.

### 5.5 Sprachpfad über mehrere Zellen

**Implementiert im aktuellen Bost.**

Bost kopiert Sprachframes zwischen Zellstacks. Da die SDRs bzw. Zellstacks eigene TDMA-Zeitbasen haben, werden Frames auf der Zielzelle in einem eigenen Jitter-/Playout-Puffer auf deren Traffic-Slot ausgegeben.

Relevante Parameter im aktuellen SiteSwitch:

| Parameter | Wert im Bost-Code |
|---|---:|
| Idle Session Timeout | 600 s |
| Handover Timeout | 30 s |
| Playout Idle Timeout | 5 s |

Diese Werte sind Bost-Implementierungsdetails, keine automatisch für NetCore geeigneten Zielwerte.

### 5.6 Standortweites Sprechrecht

**Implementiert im aktuellen Bost.**

Für Gruppenrufe koordiniert der SiteSwitch das Sprechrecht standortweit:

- Läuft derselbe Gruppenruf bereits mit Sprecher auf einer anderen Zelle, wird ein konkurrierender lokaler Grant nicht ungeprüft als zweiter Sprecher ans Netz weitergereicht.
- Höhere Priorität, beispielsweise ein entsprechend markierter Notruf, kann den laufenden Sprecher standortweit verdrängen.
- Die anderen Zellen werden als Replikate in denselben Gruppenruf aufgenommen.

Für NetCore ist dies wichtiger als bloßes Audio-Multicast. Ohne gemeinsame Floor-Arbitration hätte man zwar Sprache auf mehreren Zellen, aber keinen konsistenten TETRA-Rufzustand.

### 5.7 Einzelrufe zwischen Zellen

**Implementiert im aktuellen Bost.**

CellToCellCall verbindet die CMCE-Signalisierung zweier Zellstacks. Jede Seite behandelt die jeweils andere Seite funktional als Netzgegenstelle. Die Sprachpfade werden zwischen den lokalen Traffic-Circuits kopiert.

Das ist eine starke Referenz für NetCore, muss aber an dessen bestehende zentrale Call-/Media-Struktur angepasst werden, anstatt zwei konkurrierende Call-Control-Ebenen einzuführen.

### 5.8 SIP/Asterisk über SiteRelay

Datei:

    crates/tetra-entities/src/net_site/relay.rs

**Implementiert im aktuellen Bost.**

Der SiteRelay kapselt die Asterisk-Entität der Primärzelle und leitet:

- Rufsignalisierung anhand Teilnehmerstandort bzw. Session,
- RTP-/Sprachdaten anhand Carrier/Timeslot

an die passende Zelle weiter.

Das widerlegt die frühere Befürchtung, Bost habe im Mehrzellenmodell nur Gruppenruf-Funkpfade. Es gibt inzwischen auch einen konkreten Ansatz für PBX-/SIP-Erreichbarkeit über alle Zellen.

Für NetCore ist das vor allem ein Semantik- und Testreferenzpunkt. Ob der Code selbst übernommen wird, hängt davon ab, welcher SIP-/PBX-Pfad in NetCore zum Implementierungszeitpunkt tatsächlich führend ist.

---

## 6. Zellreselektion und Handover-Ablauf im aktuellen Bost

### 6.1 Automatische Schwesterzellen als Nachbarn

Datei:

    crates/tetra-config/src/bluestation/sec_cells.rs

**Implementiert im aktuellen Bost.**

Bost unterstützt zusätzliche Zellen über [[cells]]. Jede zusätzliche Zelle kann eine eigene SDR-Konfiguration und eigene Zellparameter besitzen. Schwesterzellen werden automatisch als Nachbarn in die jeweilige Nachbarliste aufgenommen.

Die Implementierung validiert unter anderem:

- eindeutige Cell-IDs,
- eindeutige Carrier,
- kompatible Funkparameter,
- nicht pro Zelle überschreibbare zentrale Nachbar-/Steuerungsparameter.

Dieses Modell setzt mehrere Zellstacks innerhalb einer Station/Prozessarchitektur voraus. NetCore kann die Semantik übernehmen, muss aber nicht zwingend das identische Konfigurationsformat wählen, wenn Zellen physisch auf getrennten TBS laufen.

### 6.2 Cell-Re-Select-Parameter

Dateien:

    crates/tetra-config/src/bluestation/sec_cell.rs
    crates/tetra-entities/src/mle/components/broadcast.rs

**Implementiert im aktuellen Bost.**

Bost besitzt inzwischen CfgCellReselect mit:

- slow_threshold_db
- fast_threshold_db
- slow_hysteresis_db
- fast_hysteresis_db

Die Werte werden in 2-dB-Schritten in das 16-Bit-Element Cell re-select parameters codiert und bei vorhandener Nachbarliste in D-NWRK-BROADCAST ausgesendet.

Im aktuellen NetCore main existiert zwar die Nachbarzellenausstrahlung, aber der geprüfte Broadcast-Code setzt cell_re_select_parameters weiterhin auf 0.

### 6.3 Announced Reselection: U-PREPARE -> D-NEW-CELL

Datei:

    crates/tetra-entities/src/mle/mle_bs.rs

**Implementiert im aktuellen Bost.**

Der heutige Bost-Code verarbeitet U-PREPARE:

1. MS benennt die gewünschte Nachbarzelle über cell_identifier_ca.
2. Bost prüft, ob die Zelle tatsächlich als Nachbar bekannt ist.
3. Bei unbekannter Zelle wird D-PREPARE-FAIL erzeugt.
4. Bei gültiger Zielzelle wird der SiteSwitch über SiteHandoverPrepare informiert.
5. Der Zielstack kann die Gruppenmitgliedschaften des MS bereits vorbereiten.
6. D-NEW-CELL weist den Zellwechsel an.

Für Type-1-announced-reselection kann zusätzlich die Registrierung vorab an die Zielzelle weitergereicht werden.

### 6.4 Forward Registration

**Implementiert im aktuellen Bost.**

Wenn U-PREPARE eine Registrierungs-SDU enthält:

1. Serving Cell hält D-NEW-CELL zunächst zurück.
2. SiteSwitch leitet SiteForwardRegistration an die Zielzelle.
3. Ziel-MLE reicht die SDU an das dortige MM weiter.
4. Die Antwort der Zielzelle wird zurücktransportiert.
5. Serving Cell kann das MM-Ergebnis in D-NEW-CELL einbetten.
6. Bei Timeout kann D-NEW-CELL ohne Zielantwort gesendet werden.

Bost verwendet hierfür aktuell:

- FORWARD_REGISTRATION_TIMEOUT = 3 s

Das ist ein konkreter, bereits implementierter Make-Before-Move-Baustein auf der Signalisierungsseite.

### 6.5 Vorbereitung laufender Gruppenrufe

**Implementiert im aktuellen Bost.**

Bei SiteHandoverPrepare ermittelt der SiteSwitch die Gruppen des Teilnehmers auf der Quellzelle und affiliiert ihn vorübergehend auf der Zielzelle. Dadurch kann die Zielzelle bereits in dort laufende bzw. standortweit laufende Gruppenrufe aufgenommen werden, bevor das MS physisch dort registriert ist.

Kommt das MS nicht an, wird die Vorbereitung nach HANDOVER_TIMEOUT, derzeit 30 s, zurückgenommen.

### 6.6 U-RESTORE und U-CALL RESTORE

**Implementiert im aktuellen Bost.**

Nach dem Zellwechsel kann das MS einen U-RESTORE mit eingebettetem U-CALL RESTORE senden.

Bost:

1. MLE nimmt U-RESTORE an.
2. Die enthaltene CMCE-SDU wird an CMCE übergeben.
3. CMCE bearbeitet U-CALL RESTORE.
4. Die CMCE-Antwort wird wieder in D-RESTORE-ACK bzw. D-RESTORE-FAIL verpackt.

Für die Antwort auf den eingebetteten Call Restore existiert im Bost-Code derzeit ein RESTORE_ANSWER_TIMEOUT von 10 s.

### 6.7 Mapping eines fremden Call-Identifiers

Aktuelle Datei:

    crates/tetra-entities/src/cmce/subentities/cc_bs/procedures/restoration.rs

**Implementiert im aktuellen Bost.**

Wenn der vom MS mitgebrachte call_identifier auf der Zielzelle unbekannt ist, prüft Bost im site-linked-Modus:

- Ist other_party_ssi vorhanden?
- Entspricht diese GSSI einem lokal aktiven Gruppenruf?
- Wenn ja, wird der fremde Call-Identifier auf den lokalen Call-Identifier dieses aktiven Gruppenrufs abgebildet.

Das ist genau der zellübergreifende Restore-Schritt, der im alten 3d476eea-Snapshot noch fehlte.

Wichtig: Die GSSI-basierte Zuordnung ist für Gruppenrufe geeignet. Für andere Rufarten braucht NetCore eine eindeutigere, zentral geführte Session-/Rufidentität.

### 6.8 Abschluss des Zellwechsels

**Implementiert im aktuellen Bost.**

Registriert sich das MS auf der neuen Zelle:

- SiteDirectory aktualisiert ISSI -> neue CellId.
- alte Gruppenaffiliations werden dort entfernt,
- die alte Zelle erhält intern eine Deregistrierung zum Aufräumen,
- diese alte Deregistrierung wird nicht fälschlich ans externe Netz weitergereicht,
- verspätete Cleanup-Meldungen der alten Zelle dürfen die neue Registrierung nicht löschen.

Damit ist nicht nur der Call-Restore, sondern auch die Teilnehmer-Ortslogik Teil des Multi-Cell-Designs.

---

## 7. Aktueller NetCore-Stand am 2026-10-06

Geprüft wurde main@9116c15d645458f99e236712b67a1ad970432791. Die folgenden Punkte sind reine Repository-Prüfungen, keine Aussage über eventuell abweichende, lokal installierte Builds.

### 7.1 Direkter Vergleich

| Funktion | Bost main@96671561dfc61af6612f087305ebbed15a7ba5d1 | NetCore main@9116c15d645458f99e236712b67a1ad970432791 |
|---|---|---|
| Neighbor Cells in D-NWRK-BROADCAST | Implementiert | Implementiert |
| konfigurierbare Cell-Re-Select Threshold/Hysteresis | Implementiert | Im geprüften Pfad nicht vorhanden; Broadcast setzt 0 |
| [[cells]] Multi-Cell-Konfiguration | Implementiert | Entsprechende sec_cells.rs nicht vorhanden |
| SiteDirectory ISSI -> Cell | Implementiert | Entsprechendes net_site-Modul nicht vorhanden |
| SiteSwitch | Implementiert | net_site/mod.rs nicht vorhanden |
| standortweite Call-ID-Abbildung | Implementiert | In diesem Pfad nicht vorhanden |
| gruppenweiter Multi-Cell-Medienpfad | Implementiert | Kein entsprechender SiteSwitch-Pfad gefunden |
| Zellwechsel-Cleanup alter Registrierung | Implementiert | Kein entsprechender SiteSwitch-Pfad gefunden |
| U-PREPARE/D-NEW-CELL BS-Prozedur | Implementiert | Im geprüften mle_bs.rs nicht entsprechend implementiert |
| Forward Registration zur Zielzelle | Implementiert | Im geprüften Pfad nicht vorhanden |
| U-RESTORE -> CMCE -> D-RESTORE ACK/FAIL | Implementiert | Im geprüften BS-Pfad nicht entsprechend vorhanden |
| sibling call_id -> lokale GSSI-Session | Implementiert | restoration.rs besitzt dieses Mapping nicht |
| Individual-Restore Floor-Holder-Schutz | Implementiert | Fehlt im geprüften restoration.rs |
| Restore -> UMAC/Floor-Grant-Watchdog | Implementiert | Fehlt im geprüften restoration.rs |
| Asterisk SiteRelay | Implementiert | Kein Bost-äquivalentes net_site/relay.rs vorhanden |

### 7.2 Aktuelle NetCore-Restore-Funktion

Datei:

    crates/tetra-entities/src/cmce/subentities/cc_bs/procedures/restoration.rs

Blob im geprüften main:

    c206b22750794be7c08633f1d67c22c4965d891b

**Implementiert, aber gegenüber Bost lückenhaft.**

Der aktuelle NetCore-Code:

- sucht Individual- und Gruppenruf anhand des lokalen call_id,
- besitzt kein sibling-cell-GSSI-Mapping,
- erteilt beim Individual-Restore bei gesetztem Request-Flag weiterhin unmittelbar TransmissionGrant::Granted,
- benachrichtigt nach einem Gruppen-Restore-Grant den Floor-/UMAC-Watchdog nicht über den in Bost ergänzten Pfad.

Damit sind die beiden c71c9ad-Sicherheitsfixes nach heutigem Vergleich weiterhin echte Portierungskandidaten.

### 7.3 Aktueller NetCore-Nachbarbroadcast

Datei:

    crates/tetra-entities/src/mle/components/broadcast.rs

Blob:

    9bed0569dd928327298c3b1363147674be37d69c

**Implementiert.**

Die Nachbarzellendaten können gesendet werden. Bei vorhandenen Nachbarn wird cell_re_select_parameters im geprüften Stand jedoch weiterhin mit 0 belegt. Bost ist hier inzwischen weiter.

### 7.4 Aktueller NetCore-MLE-BS-Pfad

Datei:

    crates/tetra-entities/src/mle/mle_bs.rs

Blob:

    d48545a08dd3460a6588b7a8b0c502e52dfffdc4

Im heute geprüften NetCore-Pfad sind die für Bost zentralen U-PREPARE-/U-RESTORE-BS-Abläufe nicht in derselben Form vorhanden. Gefundene DNewCell-, DPrepareFail- und DNwrkBroadcast-Zweige enthalten noch unimplemented_log-Marker.

### 7.5 Fehlende Bost-Site-Module

Im aktuellen NetCore main waren folgende exakten Bost-Pfade nicht vorhanden:

    crates/tetra-entities/src/net_site/mod.rs
    crates/tetra-config/src/bluestation/sec_cells.rs

Das ist ein klarer Hinweis, dass Bosts neuere Mehrzellenarchitektur noch nicht einfach bereits unter anderem Namen in diesen unmittelbaren Pfaden übernommen wurde.

### 7.6 Hinweis zu früherer NetCore-Architektursprache

Im ursprünglichen Chat wurde als Zielbild mit zentralen Komponenten wie Node Gateway, Call Control und Media Switch argumentiert. Beim heutigen Dateipfad-Abgleich existieren die damals sinngemäß genannten Pfade services/node-gateway, services/call-control und services/media-switch so nicht im aktuellen Repository.

Daraus folgt **nicht**, dass entsprechende Funktionen im Gesamtprojekt nicht existieren. Es bedeutet lediglich, dass die frühere Architekturformulierung nicht als Nachweis für exakt diese aktuellen Dateipfade verwendet werden darf. Vor der tatsächlichen Portierung muss die heute führende NetCore-Komponentenstruktur erneut konkret aufgelöst werden.

---

## 8. Endgültige technische Entscheidung für NetCore

### 8.1 Grundsatz

**Beschlossen/geplant:** Bost nicht komplett mergen. Die Multi-Cell-Entwicklung soll als Referenzimplementierung und Quelle gezielter Ports verwendet werden.

Begründung:

- NetCore besitzt eigene zusätzliche Funktionen und bereits abweichende Konfigurationen, beispielsweise erweiterte Brew-Zuordnungsregeln.
- Bosts SiteSwitch ist auf mehrere Zellstacks innerhalb derselben Prozess-/Stationsarchitektur zugeschnitten.
- NetCore plant bzw. besitzt darüber hinaus eine stärker verteilte Core-/TBS-Architektur.
- Ein Vollmerge würde Verantwortlichkeiten und Zustandsautomaten vermischen und spätere Wartung erschweren.
- Die niedrig gekoppelten CMCE-Fixes lassen sich deutlich risikoärmer separat übernehmen.

### 8.2 Priorität 0: Restore-Sicherheitsfixes

**Beschlossen/geplant, noch nicht implementiert in diesem Archivauftrag.**

Zuerst übernehmen:

1. Individual-Restore nur gewähren, wenn Sprechrecht frei oder bereits beim anfragenden Teilnehmer liegt.
2. Bei Simplex den internen Floor-Holder korrekt setzen.
3. Nach erfolgreichem Gruppen-Restore mit Grant denselben Downlink-/UMAC-Floor-Grant-Pfad auslösen wie bei regulärer Sprechfreigabe.
4. Regressionstests für Restore ergänzen.

Diese Änderungen sind auch ohne Mehrzellenbetrieb sinnvoll.

### 8.3 Priorität 1: Reselection-Parameter und Neighbor-Policy

**Beschlossen/geplant.**

Prüfen und adaptieren:

- CfgCellReselect,
- 16-Bit-Encoding der vier Parameter,
- D-NWRK-BROADCAST mit tatsächlichen Threshold/Hysteresis-Werten,
- automatische bzw. zentral erzeugte Nachbarbeziehungen.

Bosts Defaultwerte sollen nicht blind übernommen werden. Die Zielwerte müssen zum NetCore-RF-Layout und zu den eingesetzten Funkgeräten passen.

### 8.4 Priorität 2: MLE U-PREPARE/U-RESTORE

**Beschlossen/geplant.**

Die Bost-Prozeduren sind eine direkte Referenz für:

- U-PREPARE,
- D-PREPARE-FAIL,
- D-NEW-CELL,
- vorgezogene Registrierung,
- U-RESTORE,
- D-RESTORE-ACK/-FAIL.

Da NetCore denselben TETRA-Protokollstack verwendet bzw. davon abstammt, ist hier echte Codewiederverwendung wahrscheinlicher als beim SiteSwitch. Trotzdem sind aktuelle Datentypen und NetCore-spezifische Routingwege vor dem Port zu prüfen.

### 8.5 Priorität 3: zentrale Teilnehmer- und Rufidentität

**Beschlossen/geplant.**

Aus Bost übernehmen bzw. in NetCore-Form neu implementieren:

- ISSI -> serving cell,
- zellbezogene GSSI-Affiliation,
- Schutz gegen verspätete Deregistrierungen,
- lokale Air-Interface-call_id strikt von zentraler Session-ID trennen.

Für NetCore ist eine interne globale UUID/Session-ID vorzuziehen. Der lokale u16 call_identifier bleibt ein Air-Interface-/Zellkontext.

### 8.6 Priorität 4: Gruppenruf-Handover und Media

**Beschlossen/geplant.**

Zu übernehmen ist primär die Semantik:

- Zielzelle vor Ankunft als Gruppenhörer vorbereiten,
- bestehenden Gruppenruf dort aktivieren,
- Floor-Zustand zentral erhalten,
- Medien auf den Zielpfad legen,
- alten Pfad erst nach erfolgreicher Zielübernahme abbauen,
- Timeout/Rollback bei nicht abgeschlossenem Handover.

Bosts in-process VoiceJitterBuffer kann Referenz sein. Bei verteilten NetCore-TBS muss der Transport wahrscheinlich über den vorhandenen zentralen Medienpfad bzw. ein geeignetes Inter-TBS-Protokoll laufen.

### 8.7 Priorität 5: Einzelruf, Duplex und SIP/PBX

**Beschlossen/geplant.**

Separat abnehmen:

- Individual Simplex,
- Individual Duplex,
- Telefon-/SIP-Ruf,
- Medienkontinuität in beide Richtungen,
- DTMF und Rufabbau,
- Teilnehmerwechsel zwischen Zellen während eines laufenden SIP-Rufs.

Bosts SiteRelay ist eine wertvolle Referenz, aber nicht automatisch NetCores endgültige Implementierung.

---

## 9. Komponenten- und Portierungsmatrix

| Bost-Datei/Komponente | Heutige Funktion | Empfehlung für NetCore |
|---|---|---|
| cmce/.../restoration.rs | Floor-Schutz, UMAC-Benachrichtigung, sibling-call-id per GSSI | Gezielt portieren; zuerst Sicherheitsfixes, dann Multi-Cell-Mapping |
| mle/mle_bs.rs | U-PREPARE, D-NEW-CELL, Forward Registration, U-RESTORE | Protokollnah adaptieren |
| mle/components/broadcast.rs | Nachbarbroadcast + Cell-Re-Select-Parameter | Kleiner, gut isolierbarer Port |
| tetra-config/.../sec_cell.rs | Reselection-Threshold/Hysteresis | Datenmodell übernehmen/anpassen |
| tetra-config/.../sec_cells.rs | Mehrere lokale Zellen/SDRs | Semantik übernehmen; Konfigurationsmodell nur falls passend |
| net_site/directory.rs | ISSI-/GSSI-Ortszustand | Sehr guter Modellkandidat für zentrale Mobility-Daten |
| net_site/switch.rs / CallIdMap | lokale/global Call-ID, Rufrouting | Konzept übernehmen; Transport für verteiltes NetCore neu schneiden |
| net_site/switch.rs / GroupSession | Gruppenruf über mehrere Zellen | Zustandsautomat als Referenz |
| net_site/switch.rs / VoicePlayout | zeitbasierte Medienkopie | Als Timing-/Jitter-Referenz, nicht zwingend 1:1 |
| net_site/relay.rs | SIP/Asterisk über alle Zellen | Semantik und Testfälle übernehmen |
| SiteDirectory-Move-Cleanup | alte Registrierung aufräumen | Unbedingt als Race-/Ordering-Schutz berücksichtigen |

---

## 10. Protokolle, Schnittstellen und technische Parameter

### 10.1 TETRA-Nachrichten im Handover-Pfad

Relevante Nachrichten/Prozeduren:

- D-NWRK-BROADCAST
- U-PREPARE
- D-PREPARE-FAIL
- D-NEW-CELL
- U-RESTORE
- D-RESTORE-ACK
- D-RESTORE-FAIL
- U-CALL RESTORE
- D-CALL RESTORE
- MM Registration / Location Update als ggf. eingebettete SDU
- Floor Granted / Floor Released als interne Steuerereignisse

### 10.2 Adressen und Identitäten

- ISSI: Teilnehmeridentität und serving-cell-Ortung.
- GSSI: Gruppenidentität; im Bost-Gruppenrestore auch Schlüssel zum Auffinden des lokal passenden aktiven Gruppenrufs.
- call_identifier: lokal pro Zelle nicht als global eindeutig behandeln.
- interne Session-ID: für NetCore als standort-/netzweit stabile Identität vorsehen.

### 10.3 Bost-spezifische Timeouts

Diese Werte sind dokumentierte aktuelle Bost-Implementierungsparameter, keine bereits beschlossenen NetCore-Werte:

- Forward Registration: 3 s
- Restore Answer: 10 s
- Handover-Vorbereitung: 30 s
- Idle Site Session: 600 s
- Idle Playout Buffer: 5 s

### 10.4 Backhaul und Site-Link

Bost aktiviert wesentliche Multi-Cell-Semantik nur im site-linked-Kontext. Die Schwesterzellen werden intern über SiteSwitch/CellLink verbunden; die reale Netzwerkentität sitzt im Primärrouter.

NetCore muss diese Bedingung auf seine eigene Betriebsart abbilden. Ein Ausfall des zentralen Pfades darf nicht unkontrolliert lokale Rufzustände duplizieren oder hängen lassen.

---

## 11. Durchgeführte Prüfungen und Grenzen

### 11.1 In diesem Archivlauf tatsächlich geprüft

**Getestet im Sinne einer Repository-Inspektion, nicht im Funkbetrieb:**

- aktueller Archiving-Branch und dessen Ausgangscommit,
- aktueller NetCore-main-Commit,
- aktueller Bost-main-Commit,
- Existenz und Inhalt der genannten Bost-Dateien,
- Existenz bzw. Fehlen der genannten NetCore-Dateien,
- direkter Vergleich der Restore-Handler,
- direkter Vergleich des Neighbor-Broadcast-Pfads,
- Bost-Commit-Historie der Multi-Cell-Entwicklung,
- Vorhandensein der Bost-Unit-Tests im Quellcode.

### 11.2 Nicht ausgeführt

**Nicht getestet:**

- cargo test für Bost oder NetCore,
- Cross-Compilation,
- Installation auf Raspberry Pi/TBS,
- zwei reale gleichzeitig sendende TBS,
- HF-Reselection eines Sepura-/Motorola-/Hytera-Geräts,
- Audio-Unterbrechungszeit beim Zellwechsel,
- Gruppenruf während aktiver eigener PTT,
- Individual-/Duplex-Handover,
- SIP-Handover,
- Last-, Paketverlust-, Clock-Drift- oder Backhaul-Failure-Tests.

Daher ist weder der aktuelle Bost-Stand noch eine spätere NetCore-Portierung durch diesen Chat als **im Betrieb bestätigt** zu bewerten.

### 11.3 Bost-Unit-Tests als verfügbare Referenz

Im aktuellen Bost-SiteSwitch-/MLE-Code sind unter anderem Tests für folgende Fälle vorhanden:

- Network Group Call Fan-out und kopierte Sprache,
- eindeutige netzseitige Call-IDs,
- lokaler Gruppenruf auf anderen Zellen hörbar,
- nur ein Sprecher je Gruppe über mehrere Zellen,
- Individual Call zwischen zwei Zellen mit Sprache in beide Richtungen,
- Move eines Radios zwischen Zellen und Cleanup der alten Registrierung,
- höhere Rufpriorität über mehrere Zellen,
- angekündigter Handover bereitet Zielzelle vor,
- abgebrochener Handover wird zurückgerollt,
- abgeschlossener Handover wird nicht fälschlich zurückgerollt,
- Group SDS zwischen Zellen,
- Asterisk Relay auf Schwesterzellen,
- Forward Registration zur Ziel-MLE und Rückantwort,
- U-PREPARE zu gültiger/ungültiger Nachbarzelle,
- Forward Registration mit D-NEW-CELL bzw. D-PREPARE-FAIL.

Diese Tests wurden in diesem Archivlauf **nicht ausgeführt**; ihre Existenz ist aber ein wichtiger Portierungs- und Regressionstest-Katalog.

---

## 12. Empfohlene NetCore-Testmatrix nach der Portierung

**Beschlossen/geplant als Roadmap-Kandidat.**

### 12.1 Zellwechsel ohne Ruf

- unannounced reselection,
- announced reselection,
- Type-1 Forward Registration,
- Zielzelle nicht erreichbar,
- Zielzelle lehnt Registrierung ab,
- Teilnehmer kehrt zur alten Zelle zurück,
- verspätete Deregistrierung der alten Zelle.

### 12.2 Gruppenruf

Jeweils Wechsel:

- während reinem Empfang,
- in der Hangtime,
- unmittelbar vor eigener PTT,
- während eigener PTT,
- nach Floor Release,
- bei gleichzeitigem Sprecher auf anderer Zelle,
- bei Notruf-/Prioritätswechsel.

Zu messen:

- verlorene Sprachframes,
- Unterbrechungsdauer,
- doppelter Downlink,
- doppelter Sprecherzustand,
- hängende Timeslots,
- Restore-Erfolg/-Ablehnung,
- Cleanup der Quellzelle.

### 12.3 Individual Calls

- Simplex A -> B,
- Simplex B -> A,
- Duplex,
- Caller wechselt Zelle,
- Callee wechselt Zelle,
- beide wechseln nacheinander,
- Abbruch während Prepare/Restore.

### 12.4 SIP/PBX

- TETRA -> SIP,
- SIP -> TETRA,
- Zellwechsel während Ringing,
- Zellwechsel während Connected,
- bidirektionales Audio,
- DTMF,
- korrekter Release,
- PBX-/Backhaul-Ausfall während Zellwechsel.

### 12.5 Robustheit

- zwei Zellen mit unabhängigen SDR-Clocks,
- künstlicher Jitter,
- Paketverlust im Inter-TBS-Pfad,
- verzögerte Steuertelegramme,
- doppelte/verspätete Registrierungsereignisse,
- Call-ID-Kollision zwischen zwei Zellen,
- Neustart einer Zielzelle während Handover,
- Core-/Backhaul-Wiederkehr,
- wiederholtes Ping-Pong an der Zellgrenze.

**Kein fester Seamless-Grenzwert wurde in diesem Chat vereinbart.** Ein solcher Grenzwert muss vor Abnahme definiert und anschließend gemessen werden.

---

## 13. Wichtige Befehle und Arbeitsabläufe

In diesem Fachchat wurden keine Build-, Installations- oder Deployment-Kommandos auf einer TBS ausgeführt.

Für eine spätere kontrollierte Portierung sind folgende Arbeitsweisen sinnvoll; sie sind hier **nur vorgeschlagen und nicht ausgeführt**:

    git show c71c9ad51462bbc64fb7669f9b192d5f6f325d8b
    git show da7f6263d5f8593a1273c6dcbca3c4c05ac90937
    git diff <netcore-basis>..<portierungsbranch> -- crates/tetra-entities
    cargo test

Ein direktes git cherry-pick des kompletten Multi-Cell-Commitsets wird **nicht** als Standardweg empfohlen. Für die kleinen Restore-Fixes kann ein selektives Patchen sinnvoll sein; für SiteSwitch/Mobility soll zunächst das Zielmodell im aktuellen NetCore-Code festgelegt werden.

Die einzige in diesem Auftrag tatsächlich ausgeführte Änderung ist die Archivdokumentation im Branch Archiving.

---

## 14. Fehlerbilder, Ursachen und Lösungen

### 14.1 Doppelter Floor beim Individual Restore

**Fehlerbild:** Restore kann Sprechrecht vergeben, obwohl der Peer bereits spricht.

**Ursache:** fehlende Prüfung des aktuellen Floor-Owners.

**Bost-Lösung:** nur Grant, wenn Floor frei oder bereits beim wiederkehrenden Teilnehmer; Simplex-Floor-Zustand passend aktualisieren.

**NetCore heute:** Fix im geprüften main noch nicht vollständig vorhanden.

### 14.2 Hängender Traffic-Slot nach Group Restore

**Fehlerbild:** Restore erteilt Floor, Teilnehmer bleibt danach stumm, Traffic-Slot kann zu lange belegt bleiben.

**Ursache:** UMAC/UL-Inactivity-Pfad wird nach dem Restore-Grant nicht wie bei regulärer Floor-Vergabe aktiviert.

**Bost-Lösung:** Downlink TX-granted/FACCH und interne FloorGranted-Benachrichtigung auslösen.

**NetCore heute:** Fix im geprüften main noch nicht vorhanden.

### 14.3 Fremder Call-Identifier auf Zielzelle

**Fehlerbild:** MS bringt beim Zellwechsel die Call-ID der Quellzelle mit; Zielzelle kennt sie nicht und lehnt Restore ab.

**Historischer Bost-Snapshot:** kein Cross-Cell-Mapping.

**Aktuelle Bost-Lösung:** im site-linked-Modus aktiven Gruppenruf über other_party_ssi/GSSI finden und auf lokalen Call-Identifier abbilden.

**NetCore heute:** dieses Mapping fehlt im geprüften restoration.rs.

### 14.4 Alte Registrierung überschreibt neue Registrierung

**Fehlerbild:** Cleanup/Deregister der alten Zelle trifft verspätet ein und könnte den Teilnehmer zentral fälschlich abmelden.

**Bost-Lösung:** SiteDirectory kennt die aktuell gültige CellId; alte Deregistrierungen werden lokal verarbeitet und gegenüber dem Netz unterdrückt, wenn der Teilnehmer bereits woanders registriert ist.

### 14.5 Ursprüngliche Analyse unterschätzte den heutigen Bost-Stand

**Ursache:** Die Chatantwort prüfte den älteren Snapshot 3d476eea. Die maßgeblichen Multi-Cell-Phasen wurden später hinzugefügt.

**Korrektur:** Diese Abschlussdokumentation trennt strikt den historischen Snapshot vom aktuellen Bost main@96671561dfc61af6612f087305ebbed15a7ba5d1.

---

## 15. Verworfene oder ersetzte Ansätze

### 15.1 Komplettmerge des Fremdforks

**Verworfen.**

Grund: zu große Kopplung, abweichende NetCore-Konfigurationen und andere Zielarchitektur. Selektive Übernahme bleibt vorzuziehen.

### 15.2 Neighbor Broadcast als ausreichender Handover-Nachweis

**Verworfen.**

Neighbor Broadcast ist notwendig bzw. hilfreich für Reselection, löst aber weder Rufkontext, Floor, Medienpfad noch Registrierungscleanup.

### 15.3 Late Entry als vollständiger Ersatz für Call Restore

**Verworfen.**

Late Entry ist für Gruppenrufwiedereintritt wertvoll. Für echte Mobility müssen zusätzlich MLE/MM-/Call-Restore-Zustände und Medienkontinuität behandelt werden.

### 15.4 Lokalen u16 call_id als netzweite Identität verwenden

**Verworfen.**

Bosts heutige CallIdMap bestätigt die Notwendigkeit einer Kontext-/Global-ID-Schicht.

---

## 16. Lizenz und Übernahmebedingungen

Das aktuelle Bost-Repository enthält eine Apache License 2.0.

Für die geplante Übernahme bedeutet das grundsätzlich, dass Quellcode modifiziert und weiterverwendet werden kann, sofern die Lizenzbedingungen eingehalten werden. Bei einer tatsächlichen Codeübernahme sind insbesondere zu prüfen:

- Lizenztext beibehalten/bereitstellen,
- geänderte übernommene Dateien als geändert kennzeichnen,
- relevante Copyright-/Attributionshinweise erhalten,
- ein vorhandenes NOTICE entsprechend berücksichtigen,
- Herkunft der übernommenen Änderungen nachvollziehbar dokumentieren.

Dies ist eine technische Lizenznotiz und keine Rechtsberatung.

---

## 17. Offene Aufgaben und Roadmap-Kandidaten

### P0 – kurzfristig

- c71c9ad-Restore-Fixes gegen den aktuellen NetCore-main manuell portieren.
- Unit-Tests für Individual-/Group-Restore ergänzen.
- Restore-Grant bis in den UMAC-Inactivity-Pfad verifizieren.

### P1 – Reselection-Grundlage

- Bost CfgCellReselect gegen NetCore-Konfiguration modellieren.
- D-NWRK-BROADCAST Cell-Re-Select-Parameter implementieren.
- reale Endgeräte auf Threshold-/Hysteresis-Verhalten prüfen.

### P2 – MLE Mobility

- U-PREPARE/D-NEW-CELL/D-PREPARE-FAIL implementieren bzw. portieren.
- U-RESTORE/D-RESTORE-ACK/-FAIL implementieren.
- Forward Registration und Timeout-Verhalten ergänzen.

### P3 – zentrale Mobility-/Session-Daten

- ISSI -> serving TBS/cell.
- GSSI-Affiliations pro Zelle.
- globale Session-ID vs. lokale Air-Interface-call_id.
- Reihenfolge-/Race-Schutz für verspätete Deregistrierungen.

### P4 – Multi-Cell Group Call

- Zielzelle vor Restore in laufenden Gruppenruf aufnehmen.
- zentrale Floor-Arbitration.
- Media-Routing auf mehrere beteiligte TBS.
- Rollback bei fehlgeschlagenem Handover.

### P5 – Individual/SIP

- Einzelruf zwischen zwei Zellen.
- Duplex-Fälle.
- SIP/PBX-Kontinuität.
- Medienpfad und DTMF.
- Release-/Fehlerfälle.

### P6 – Abnahme

- Zwei-TBS-Lab.
- definierte RF-Überlappungszone.
- reproduzierbare Reselection.
- Zeitmessung der Audio-Lücke.
- Langzeit-/Ping-Pong-/Failure-Tests.
- reale Funkgeräte verschiedener Hersteller.

---

## 18. Relevante Quellen, Dateien und Commits

### Extern: Aitorrio/bost-flowstation

Repository:

    https://github.com/Aitorrio/bost-flowstation

Heute geprüfter Hauptzweig:

    96671561dfc61af6612f087305ebbed15a7ba5d1

Historisch im Chat geprüfter Snapshot:

    3d476eea

Wichtige Commits:

    c71c9ad51462bbc64fb7669f9b192d5f6f325d8b
    c949a16a33524cac75e20dd96eef640fc3d120e1
    da7f6263d5f8593a1273c6dcbca3c4c05ac90937
    44be7e440a7d1a27866b69300c0978531e146202
    65c38e9910124bd40676d9db6465e3d19da62725

Wichtige Dateien:

    crates/tetra-entities/src/cmce/subentities/cc_bs/procedures/restoration.rs
    crates/tetra-entities/src/mle/mle_bs.rs
    crates/tetra-entities/src/mle/components/broadcast.rs
    crates/tetra-entities/src/net_site/mod.rs
    crates/tetra-entities/src/net_site/directory.rs
    crates/tetra-entities/src/net_site/switch.rs
    crates/tetra-entities/src/net_site/relay.rs
    crates/tetra-config/src/bluestation/sec_cell.rs
    crates/tetra-config/src/bluestation/sec_cells.rs
    LICENSE

### NetCore-Tetra

Heute lesend geprüfter Hauptzweig:

    main@9116c15d645458f99e236712b67a1ad970432791

Relevante Dateien:

    crates/tetra-entities/src/cmce/subentities/cc_bs/procedures/restoration.rs
    crates/tetra-entities/src/mle/components/broadcast.rs
    crates/tetra-entities/src/mle/mle_bs.rs
    crates/tetra-config/src/bluestation/sec_cell.rs
    crates/tetra-config/src/bluestation/sec_brew.rs
    crates/tetra-entities/src/messagerouter.rs
    PHASE1-MOBILITY-ROUTING.md
    FAST_REALTIME_MEDIA.md
    ROADMAP.md

Die letztgenannten NetCore-Dokumente wurden als vorhandene Repository-Artefakte festgestellt; ihr kompletter Inhalt wurde in diesem Archivlauf nicht als Ersatz für die direkte Quellcodeprüfung verwendet.

---

## 19. Zusammenfassung des erreichten Stands

### Idee

- Bost als Quelle für einen NetCore-Seamless-Handover verwenden.

### Beschlossen/geplant

- gezielte Übernahme statt Vollmerge,
- zuerst Restore-Sicherheitsfixes,
- danach Reselection/MLE,
- dann zentrale Mobility-/Session-ID,
- anschließend Multi-Cell-Medien und Individual-/SIP-Fälle,
- finale Zwei-TBS-/RF-Abnahme mit Messung der Audio-Unterbrechung.

### Implementiert

**Bost:** Die aktuelle externe Implementierung enthält inzwischen einen umfangreichen Multi-Cell-SiteSwitch, Mobility-Vorbereitung, U-PREPARE/U-RESTORE, Call-ID-Mapping, Cross-Cell-Gruppen-/Einzelrufe und Asterisk-SiteRelay.

**NetCore:** Nachbarzellenausstrahlung und lokaler Call-Restore sind vorhanden; die hier untersuchten neueren Bost-Multi-Cell-Komponenten und die zwei Restore-Fixes sind im heutigen main nicht vollständig nachgewiesen.

### Getestet

In diesem Archivlauf wurde der Quellcode und die Commit-Historie geprüft. Die Bost-Repositories enthalten passende Unit-Tests, diese wurden hier jedoch nicht ausgeführt.

### Im Betrieb bestätigt

Für NetCore ist in diesem Chat kein realer Multi-Cell-Handover im Funkbetrieb bestätigt.

---

## 20. Nächster sinnvoller Entwicklungsschritt

Der risikoärmste nächste Schritt ist **nicht** der komplette SiteSwitch-Port.

Zuerst sollte ein kleiner Feature-Branch vom aktuellen NetCore-main entstehen, der ausschließlich die beiden c71c9ad-Restore-Fixes übernimmt und mit Unit-Tests absichert. Danach kann als zweite klar abgegrenzte Stufe die MLE-/Neighbor-Definition mit Cell-Re-Select-Parametern sowie U-PREPARE/U-RESTORE umgesetzt werden.

Erst wenn diese Protokollgrundlagen stabil sind, sollte entschieden werden, wie Bosts in-process SiteSwitch-Semantik auf NetCores tatsächliche verteilte TBS-/Core-Struktur abgebildet wird.

So bleibt die Portierung überprüfbar, bisectbar und rückrollbar – und aus einem sehr guten Upstream-Fund wird kein unwartbarer Komplettumbau.
