# Technische Abschlussdokumentation: Codex-Auftrag für WAP/IP-Mehrgerätebetrieb und Korrektur der Logdiagnose

> **Zentraler Archivbefund:** Gewünscht ist ein echter, stabiler WAP/IP-Dienst für mehrere TETRA-Endgeräte. Im historischen Chat wurden dafür Entwicklungsaufträge formuliert, aber keine Implementierung oder erfolgreiche Endgeräteabnahme nachgewiesen. Die damalige Schlussfolgerung aus `packet_data_flag: false` war zu weitgehend: Die vorgelegte Nachricht lässt sich anhand ihres Präfixes als **MLE / D-NWRK-BROADCAST** einordnen. Sie ist kein Nachweis dafür, dass die gesamte Basisstation keinen Paketdatendienst unterstützt. Im heute geprüften NetCore-Quellstand existiert bereits umfangreiche SNDCP-/WAP-/IP-Gateway- und Mehrgeräte-Ressourcenlogik. Diese beiden Zeitstände dürfen nicht vermischt werden.

## 1. Metadaten, Geltungsbereich und Nachweisstufen

| Feld | Wert |
|---|---|
| Projekt | NetCore-Tetra |
| Archiv-Repository | `JanHG98/netcore-tetra` |
| Zielbranch | Ausschließlich `Archiving` |
| Ablage dieser Datei | `Docs/archive/2026-10-04_codex-wap-ip-mehrgeraetebetrieb-und-logdiagnose.md` |
| Thema | Codex als Implementierungswerkzeug; stabiler WAP/IP-Dienst auf mehreren TETRA-Geräten; Packet Data/SNDCP; Bewertung eines LLC-/UMAC-Logs |
| Ursprünglicher Chattitel | Nicht zuverlässig verfügbar. Der Titel dieser Datei ist ein beschreibender Archivtitel. |
| Chatlink | Nicht verfügbar; kein Link rekonstruiert oder erfunden. |
| Eindeutiger Gesprächsanker | Erste sichtbare Nutzerfrage: „wie gut ist dein Codex?“ mit Verweis auf `MidnightBlueLabs/tetra-bluestation`; später der fünfzeilige Logauszug mit `llc_bs_ms.rs:806` und `dltime: 3/20/01/1`. |
| Erstellungsdatum | **2026-10-04**, Datum in `Europe/Berlin` |
| Historische Datierung | Eine ergänzende Verlaufssuche ordnet die Zielfestlegung dem 09.04.2026 und den Logauszug dem 19.04.2026 zu. Im unmittelbar sichtbaren Gespräch fehlen vollständige Zeitstempel; diese Einordnung ersetzt keinen Chat-Export. |
| Zu Beginn gelesener Archivbranch | `68d41aa4a039563ccc3feca6b486b5cd91f75998` |
| Zugehöriger Archiv-Tree | `50246e4565c5abb51e34ccc53b2ae6aef3d86abd` |
| Zusätzlich ermittelter aktueller NetCore-`main` | `7137e0dd69877e1b604bf89148fd8b6b590c1a97` |
| Zugehöriger `main`-Tree | `e68558c4df13d3d8b56df8c3611ac682b02889c1` |
| Gelesener Upstream | `MidnightBlueLabs/tetra-bluestation`, `main`, Commit `09d4e0d9a0b8cf6c881e77353db325df9a4715aa` |
| Historisch laufendes Repository / Branch / Binary | Nicht aus dem vorgelegten Log bestimmbar. Der Upstream-Link ist keine eindeutige Identifikation der tatsächlich installierten Binärdatei. |
| Speichercommit dieser Dokumentation | Der tatsächlichen Git-Dateihistorie und der Abschlussmeldung zu entnehmen. Nicht mit dem oben genannten geprüften Ausgangscommit verwechseln. |
| Freigabe | Keine neue Betriebs-, Interoperabilitäts- oder ETSI-Konformitätsfreigabe. Dieser Auftrag verändert keine Funk- oder Dienstimplementierung. |

### 1.1 Statusbegriffe

- **Idee:** besprochene Möglichkeit, ohne nachgewiesene Umsetzung.
- **Beschlossen/geplant:** ausdrücklich gewünschtes Ziel oder vereinbartes Arbeitspaket; noch kein Codebeleg.
- **Implementiert:** konkret benannte Logik ist im bezeichneten Quellstand vorhanden. Damit ist nicht automatisch bewiesen, dass sie baut oder mit einem Terminal funktioniert.
- **Getestet:** eine konkret benannte Prüfung wurde mit nachvollziehbarem Ergebnis durchgeführt. Ein Testplan oder vorhandener Testcode genügt nicht.
- **Im Betrieb bestätigt:** die betreffende Funktion ist durch reale Betriebsbeobachtung oder eine ausdrückliche Betreiberbestätigung belegt. Eine erfolgreiche Initialisierung, ein Logeintrag oder eine Konfiguration allein reichen nicht.

**Ein Wunsch im Chat, ein generierter Prompt, ein vorhandener Codepfad, ein erfolgreicher Build und ein erfolgreicher Funkgerätetest sind unterschiedliche Nachweisstufen.**

### 1.2 Umfang der Auswertung

Berücksichtigt wurden die vier sichtbaren historischen Nutzeranliegen und die zugehörigen Assistentenantworten: Machbarkeit, vollständiger Codex-Auftrag, Logdiagnose und nachgeschärfter Codex-Auftrag. Hinzu kommen der Archivierungsauftrag, die in dieser Sitzung zugänglichen 25 PDF-/Projektquellen sowie die ausdrücklich separat vorgenommenen heutigen Repository- und Normenprüfungen.

Die PDFs wurden vollständig als Dateien inventarisiert, aber **nicht vollständig inhaltlich Seite für Seite auditiert**. Vertieft geprüft wurden die für WAP/IP, SNDCP, Basic Link und das Packet-Data-Flag unmittelbar relevanten Stellen. Eine automatische Bereitstellung von Projekt-PDFs beweist nicht, dass jedes Dokument im historischen WAP-Gespräch einzeln hochgeladen oder besprochen wurde.

Andere Projektchats sind keine stillschweigend übernommenen Umsetzungsnachweise. Ein bereits vorhandenes, thematisch verwandtes Archiv wird nur als separate Fortsetzungsquelle verlinkt; es wird nicht überschrieben.

## 2. Ziel, Ausgangslage und Verlauf

### 2.1 Verbindliches Nutzerziel

Der Nutzer präzisierte ausdrücklich:

> „echter WAP/IP-Dienst auf mehreren Geräten stabil“

Das ist das maßgebliche Ziel. Ein einzelner erfolgreicher Seitenaufruf auf einem einzigen Gerät, ein SDS-Bot oder ein isolierter Parser wäre keine vollständige Erfüllung. Die Nutzeranfragen zielten darauf, Codex einen ausreichend vollständigen Implementierungsauftrag zu geben, nicht lediglich eine grobe Ideensammlung zu erhalten.

Nicht festgelegt wurden konkrete Endgerätemodelle, Firmwarestände, Gerätezahlen, Browserprofile, Dienstadressen, Durchsatz-/Latenzziele, Abnahmedauer oder ein Featurebranch. Diese Angaben bleiben offen.

### 2.2 Chronologie des sichtbaren Chats

| Stufe | Inhalt | Ergebnis und Grenze |
|---|---|---|
| A: Machbarkeit | Frage nach Codex und WAP-Erweiterung von `MidnightBlueLabs/tetra-bluestation` | Der Assistent hielt das grundsätzlich für machbar, aber für ein größeres Stack-/Interop-Projekt. Das war eine qualitative Einschätzung, kein Benchmark und kein ausgeführter Code-Audit. |
| B: Zielschärfung | Nutzer wählte ausdrücklich stabilen echten Mehrgeräte-WAP/IP-Betrieb und verlangte einen vollständigen Codex-Auftrag | Ein umfangreicher Phasenprompt wurde geliefert: Bestandsaufnahme, Architektur, Implementierung, Interoperabilität, Tests und Dokumentation. |
| C: Alternative Arbeitsweise | Der Assistent schlug zusätzlich einen reinen Analyseauftrag und einen anschließenden Implementierungsauftrag vor | Nur vorgeschlagene Aufteilung; keine nachgewiesene Codex-Ausführung und kein bestätigter Entwicklungsfortschritt. |
| D: Laufzeitbeobachtung | Nutzer zeigte fünf DEBUG-Zeilen und berichtete, dass ihm von der Basisstation für WAP noch nichts angeboten werde | Ein Ausschnitt aus dem Downlink-Sendeweg wurde dokumentiert. Ein erfolgreicher Paketdaten- oder Browserdialog ist nicht enthalten. |
| E: Historische Diagnose | Der Assistent deutete `packet_data_flag: false`, Broadcast-Adresse und fehlende Zuteilungsfelder als starke Hinweise auf einen fehlenden Paketdatenpfad | Diese Diagnose war in dieser Sicherheit nicht durch den Ausschnitt gedeckt. Die Korrektur steht in Abschnitt 6. |
| F: Zweiter Codex-Auftrag | Nutzer bat erneut um einen Auftrag, der die fehlende Funktion einbauen solle | Ein zweiter Prompt fokussierte den vermeintlich falschen Datenpfad und einen minimalen Labor-Paketdatenpfad. Seine technische Ausgangsannahme muss korrigiert werden. |
| G: Archivierung | Verlauf, Quellen und heutige Abweichungen sichern | Nur Archivdokumentation und Index; keine Implementierung, kein Deployment und kein Merge. |

### 2.3 Einordnung der früheren Codex-Bewertung

Die Einschätzung „geeignet für Analyse, Implementierung und Tests“ war eine Empfehlung des Assistenten. Im Chat wurden weder ein konkretes Codex-Modell noch dessen Version, Ausführungsumgebung, Berechtigungen oder Arbeitsresultate nachgewiesen. Eine spätere Fortsetzung darf daraus keine Zusage ableiten, dass ein einzelner Prompt ohne Iterationen und reale Endgerätetests interoperablen Funkbetrieb erzeugt.

Die ursprüngliche Einordnung von BlueStation als Alpha-Basis wird durch die eingesehene Upstream-README gestützt. Die weitergehende Aussage, wie viel Arbeit ein stabiler Mehrgerätebetrieb genau erfordert, blieb eine Schätzung. Eine belastbare Aufwandsschätzung setzt die konkrete Ausgangsversion und den tatsächlich zu unterstützenden Geräte-/Protokollumfang voraus. [S1, S2]

## 3. Anforderungen und Entscheidungen

| ID | Anforderung / Entscheidung | Status und Herkunft |
|---|---|---|
| R1 | Echter WAP/IP-Dienst über die TETRA-Infrastruktur | **Beschlossen/geplant**, ausdrücklich vom Nutzer gewünscht. |
| R2 | Stabiler Betrieb mit mehreren Endgeräten | **Beschlossen/geplant**; Zahl und Modelle nicht benannt. |
| R3 | Nicht bei einem Proof of Concept oder SDS-Ersatz stehen bleiben | Aus der expliziten Zielschärfung ableitbar. Ein PoC darf ein Zwischenschritt, aber nicht die Endabnahme sein. |
| R4 | Codex soll alle für den gewählten Umfang notwendigen Schichten berücksichtigen | **Beschlossen/geplant** als Entwicklungsauftrag; Ausführung im Chat nicht belegt. |
| R5 | Vor Änderungen Bestand und Lücken bestimmen | Im Agentenprompt vorgeschlagene Arbeitsweise, nicht historisch ausgeführt. |
| R6 | Control Plane, Nutzdaten, Ressourcen, Lebenszyklus und Fehlerbehandlung gemeinsam betrachten | Im Agentenprompt vorgesehen; technische Ausgestaltung offen. |
| R7 | Mehrgerätebetrieb, konkurrierende Sessions, Paketverluste, Duplikate, Timeouts und Wiederanmeldung testen | Im Agentenprompt vorgesehen; keine entsprechenden Testresultate im Chat. |
| R8 | Kleine logische Commits, verständliche Rust-APIs, Fehlerbehandlung, Formatierung und Clippy | Vorgeschlagene Engineering-Regeln; keine historischen Commits nachgewiesen. |
| R9 | Annahmen und echte Blocker dokumentieren, unfertige Teile nicht als fertig ausgeben | Vorgeschlagene und für die Fortsetzung wichtige Qualitätsanforderung. |
| R10 | Aktuelle Archivierung nur auf `Archiving` und nur unter `Docs/archive/` | **Ausdrücklich autorisierter Umfang dieses Archivauftrags.** Kein Auftrag für Änderungen am Funkstack. |

### 3.1 Keine stillschweigende Herabstufung des Ziels

Der zweite Assistentenprompt sprach von einer „minimalen“ Laborlösung. Der Nutzer hat sein zuvor ausdrücklich gewähltes Ziel eines stabilen Mehrgeräte-WAP/IP-Dienstes jedoch nicht zurückgenommen. Deshalb gilt:

**Minimaler testbarer Datenpfad = möglicher Meilenstein. Stabiler Mehrgerätebetrieb = weiterhin Endziel.**

Die Abnahmekriterien der damaligen Prompts waren dafür zu schwach: „Repo baut, Tests grün, Datenpfad vorhanden, Grenzen dokumentiert“ kann einen Entwicklungsmeilenstein beschreiben, aber keine vollständige Endgeräte-Interoperabilität beweisen.

### 3.2 Grenzen der historischen Agentenprompts

Die Prompts begrenzten die vorgeschlagene Arbeit auf ein autorisiertes Labornetz und schlossen unautorisierte Netznutzung, Sicherheitsumgehung, Identitätsfälschung, Angriffe, fremde Geheimnisse sowie Kryptographie-/Key-Handling-Arbeiten aus. Diese Formulierungen waren vom Assistenten eingefügte Grenzen des jeweiligen Agentenauftrags, keine gesonderte umfassende Architekturentscheidung des Nutzers für das gesamte NetCore-Projekt.

Die Bereitstellung von Security- oder SIM-Spezifikationen ist ebenfalls keine automatische Freigabe zur Entwicklung aller darin beschriebenen Funktionen. Für diesen Archivauftrag werden weder Zugangsdaten noch Schlüssel übernommen.

## 4. Architektur: historischer Entwurf und heutige fachliche Einordnung

### 4.1 Historisch vorgesehene Komponenten

Der erste Prompt sollte folgende Bereiche untersuchen und, soweit erforderlich, ergänzen:

1. Broadcast/Sync, Registrierung, Gruppenaufschaltung und Mobility/Control Plane.
2. Abgrenzung bereits vorhandener Sprach- und SDS-Funktionen vom Paketdatendienst.
3. MLE, LLC, UMAC, Scheduler, Fragmentierung, SAPs, PDU-Typen und Uplink.
4. Paketdaten-Kontext-/Session-Lebenszyklus einschließlich Aufbau, Freigabe, Wiederanmeldung und Recovery.
5. Individuelle Teilnehmerzuordnung, parallele Kontexte und faire beziehungsweise begrenzte Ressourcenvergabe.
6. Übergang zwischen Funkdatenpfad und lokalem IP-/WAP-Testdienst.
7. Konfiguration, Timer, Wiederholungen, Telemetrie und Diagnose.
8. Tests und dokumentierte Endgeräte-Interoperabilität.

Das war ein Anforderungskatalog. Der Chat enthielt noch keinen verabschiedeten detaillierten Modulvertrag oder vollständigen Zustandsautomaten.

### 4.2 Fachliche Präzisierung aus den beigefügten Quellen

Die frühere Formulierung „LLC-/SNDCF-/User-Plane-ähnliche Datenpfade“ war unpräzise. Für die hier relevante TETRA-Paketdatenfunktion ist die konkrete Bezeichnung **SNDCP – Subnetwork Dependent Convergence Protocol** zu verwenden. Die hochgeladene EN 300 392-2 V3.8.1 beschreibt in Abschnitt 28 die Paketdatenfunktion, in 28.1 die SNDCP-Kontexte und in 28.3.4 die unteren Linkdienste. [N1]

Die Norm zeigt auf Seite 1022, Abbildung 28.3, WAP oberhalb von UDP/IP und SNDCP. Seite 1023, Abbildung 28.4, ordnet darunter MLE, LLC, MAC und die Luftschnittstellenschicht 1 ein. Die tatsächliche IP-Routing-/Relaying-Implementierung der SwMI und der Anschluss externer Netze liegen außerhalb des dort festgelegten Funkprotokollumfangs. [N1, S. 1021–1023]

Ein zweckmäßiges Schichtenbild für die Fortsetzung ist damit:

```text
TETRA-Endgerät                                 NetCore / SwMI

WAP-Anwendung / Browser                        WAP-Dienst oder IP-Ziel
UDP/IP                                         lokaler Adapter / IP-Routing
SNDCP, PDP-Kontexte        <--- SN-PDUs --->     SNDCP, Kontext- und Teilnehmerzuordnung
MLE                                            MLE
LLC                                            LLC
MAC / Funkzugriff                              UMAC / Scheduler / Ressourcen
Funk / AI-1               <--- Luft --->        LMAC / PHY / SDR
```

Dieses Bild ist eine heutige fachliche Einordnung, kein im historischen Chat bereits implementiertes Systemdiagramm.

### 4.3 Drei getrennte Fragen bei „Die Basisstation bietet kein WAP an“

**Dienstankündigung:** Welche Paketdaten-/LLC-Fähigkeiten werden tatsächlich über die Luftschnittstelle angekündigt? Maßgeblich sind dafür die zugehörigen Service-Informationen, nicht ein beliebiges `packet_data_flag` einer einzelnen SAP-Nachricht. Die Norm nennt für CA-Zellen unter anderem die „BS service details“; die aktuelle Code-Suche zeigt ebenfalls eine gemeinsame Profilentscheidung für Ankündigung und Laufzeitannahme. [N1; S8]

**Datentransport:** Sendet das Endgerät einen SNDCP-Aufbauwunsch, wird ein gültiger Kontext ausgehandelt, funktionieren die erforderlichen Linkdienste und gelangen Nutzdaten in beide Richtungen über den Funkpfad?

**Anwendung:** Sind Browser, Zieladresse, Port, Protokollprofil und Inhalte miteinander kompatibel? Ein funktionierender IP-Pfad ist nicht automatisch ein vollständiges WAP-Gateway für jedes Geräteprofil.

Der historische Ausschnitt beantwortet diese drei Fragen nicht. Er enthält keine vollständige Service-Ankündigung, keinen Kontextaufbau und keine Browsertransaktion.

### 4.4 Basic Link ist kein Gegenbeweis für Paketdaten

Die Tabellen 28.16 und 28.17 auf Seiten 1081–1082 der beigefügten EN 300 392-2 ordnen zahlreiche SNDCP-Steuer-PDUs ausdrücklich dem Basic Link zu. Beispielsweise laufen Kontextaktivierung und -deaktivierung über den bestätigten Basic-Link-Dienst. `SN-DATA` wird in Tabelle 28.16 den Advanced Links zugeordnet; für `SN-UNITDATA` existiert eine profil-/datenklassenabhängige Auswahl. Die Tabellen wurden bei der Archivprüfung auch als gerenderte Seiten angesehen. [N1]

Daraus folgt: **Nicht „Basic Link abschaffen“, sondern die für die jeweilige PDU und das ausgehandelte Profil vorgesehenen Linkdienste korrekt implementieren.** Eine künstliche vollständige Trennung aller Paketdaten vom Basic Link wäre kein sachgerechtes Entwicklungsziel.

### 4.5 WAP/IP und WAP über SDS unterscheiden

Der Nutzer hat hier ausdrücklich die IP-Variante gewählt. Die beigefügte Norm enthält daneben in Abschnitt 29.5.8 eine WAP-Einordnung im SDS-Anwendungskontext. Das widerlegt eine pauschale Aussage „WAP braucht immer und ausschließlich IP“, macht aber SDS nicht zum Ersatz für das hier gewählte Ziel. Geräteprofile und Transportvarianten müssen separat dokumentiert werden. [N1, S. 1220]

## 5. Historischer Laufzeitbefund

### 5.1 Betreiberbeobachtung

Der Nutzer schrieb sinngemäß, dass von der Basisstation für WAP noch nichts angeboten werde. Das ist der einzige konkrete negative Betriebsbefund dieses Chats. Es fehlt eine genaue Beschreibung der Geräteanzeige, des Browsermenüs, einer Fehlermeldung, des ausgewählten Profils und des Zeitpunkts eines Verbindungsversuchs.

### 5.2 Vollständiger verfügbarer Logauszug

Der folgende Ausschnitt wird als technische Primärbeobachtung erhalten. Er enthält keine Passwörter, Tokens oder Schlüssel:

```text
DEBUG               [entities/llc] llc_bs_ms.rs:806:          rx_prim: SapMsg { sap: TlaSap, src: Mle, dest: Llc, dltime:     3/20/01/1, msg: TlaTlUnitdataReqBl(TlaTlUnitdataReqBl { main_address: TetraAddress { ssi: 16777215, ssi_type: Gssi, encrypted: false }, link_id: 0, endpoint_id: 0, tl_sdu: BitBuffer { <0 ^0 >78 ^101010000000000000000000110100011110101110011001110001000011010111111111111000 }, stealing_permission: false, subscriber_class: 0, fcs_flag: false, air_interface_encryption: None, packet_data_flag: false, n_tlsdu_repeats: 0, data_class_info: None, req_handle: 0, chan_alloc: None, tx_reporter: None }) }
DEBUG               [entities/llc] llc_bs_ms.rs:208:       -> BlUdata { has_fcs: false } sdu ^0010101010000000000000000000110100011110101110011001110001000011010111111111111000
DEBUG               [entities/llc] llc_bs_ms.rs:763:          submitting udata msg to umac: TmaUnitdataReq(TmaUnitdataReq { req_handle: 0, pdu: BitBuffer { <0 ^0 >82 ^0010101010000000000000000000110100011110101110011001110001000011010111111111111000 }, main_address: TetraAddress { ssi: 16777215, ssi_type: Gssi, encrypted: false }, link_id: 0, endpoint_id: 0, stealing_permission: false, subscriber_class: 0, air_interface_encryption: None, stealing_repeats_flag: None, data_category: None, chan_alloc: None, tx_reporter: None })
DEBUG               [entities/umac] bs_sched.rs:419:          dl_enqueue_tma: ts  enqueueing 1 PDU MacResource { fill_bits: true, pos_of_grant: 0, encryption_mode: 0, random_access_flag: false, length_ind: 16, addr: Some(TetraAddress { ssi: 16777215, ssi_type: Gssi, encrypted: false }), event_label: None, usage_marker: None, power_control_element: None, slot_granting_element: None, chan_alloc_element: None } SDU ^0010101010000000000000000000110100011110101110011001110001000011010111111111111000
DEBUG               [entities/umac] bs_frag.rs:75:         -> MacResource { fill_bits: true, pos_of_grant: 0, encryption_mode: 0, random_access_flag: false, length_ind: 16, addr: Some(TetraAddress { ssi: 16777215, ssi_type: Gssi, encrypted: false }), event_label: None, usage_marker: None, power_control_element: None, slot_granting_element: None, chan_alloc_element: None } sdu 0010101010000000000000000000110100011110101110011001110001000011010111111111111000
```

### 5.3 Was der Ausschnitt tatsächlich belegt

Die sichtbare interne Übergabekette lautet:

```text
Mle -> TlaSap / TlaTlUnitdataReqBl -> Llc
    -> BlUdata ohne FCS
    -> TmaUnitdataReq -> UMAC-Scheduler
    -> MacResource -> MAC-Fragmentierungs-/Ausgabepfad
```

Eine 78-Bit-TL-SDU wird in eine 82-Bit-LLC-PDU eingebettet. Die angezeigte Ziel-SSI beträgt `16777215`, also `0xFFFFFF`. Die gezeigte Nachricht enthält keine Kanalzuteilung und keine Slot-Granting-Erweiterung. Der Ausschnitt reicht bis zum internen Enqueue-/Ausgabepfad; er beweist nicht einmal für diese Nachricht einen erfolgreichen Empfang auf einem realen MS.

`dltime: 3/20/01/1` ist eine interne TDMA-Zeitdarstellung, kein Kalenderdatum. Die historischen Dateizeilen `806`, `208`, `763`, `419` und `75` identifizieren ohne Source-/Buildversion keine heutigen Codezeilen.

## 6. Ausdrückliche Korrektur der historischen Diagnose

### 6.1 Heute nachgeprüfte Einordnung des PDU-Präfixes

Die TL-SDU beginnt mit:

```text
101 010 ...
```

In den konkret gelesenen Upstream-Enums gilt:

- `MleProtocolDiscriminator::Mle = 5`, also `101`;
- `MlePduTypeDl::DNwrkBroadcast = 2`, also `010`;
- der SNDCP-Discriminator wäre `4`, also `100`.

Damit ist die Nachricht anhand des vorliegenden Präfixes als **MLE / D-NWRK-BROADCAST**, nicht als direkt discriminator-markierte SNDCP-PDU, einzuordnen. [S3, S4]

Der aktuelle Upstream-Code in `mle/components/broadcast.rs` erzeugt für seinen Netzwerkzeit-Broadcast genau die auffällige Kombination aus `Gssi 0xFFFFFF`, `link_id: 0`, `endpoint_id: 0`, `packet_data_flag: false` und `chan_alloc: None`. `mle_bs.rs` löst den Broadcast bei Multiframe 20, Frame 1 und Timeslot 1 aus. Das passt zusätzlich zu `3/20/01/1`. [S5, S6]

**Nachweisgrenze:** Die Zuordnung des PDU-Typs ist anhand der Bits und Enums nachvollziehbar. Die konkrete historische Binärversion und der vollständige Zeitwert wurden nicht rekonstruiert. Die Übereinstimmung mit dem heutigen Netzwerkzeitpfad ist ein starkes Indiz für dessen Herkunft, aber kein Binary-Provenienznachweis.

### 6.2 Korrekturtabelle

| Historische Aussage | Bewertung bei der Archivprüfung | Konsequenz |
|---|---|---|
| `packet_data_flag: false` sei der „Holzhammer“ gegen vorhandenen WAP-/Packet-Data-Support | **Zu weitgehend.** Das Flag klassifiziert die einzelne übergebene SDU; es ist kein globaler Dienstschalter. | Nicht pauschal auf `true` setzen. Zuerst Nachrichtentyp und Aufrufer prüfen. |
| `TlaTlUnitdataReqBl` / Basic Link spreche grundsätzlich gegen echten Paketdatenbetrieb | **Falsch verallgemeinert.** Basic-Link-Dienste werden auch von SNDCP verwendet. | PDU-/profilgerechte Linkauswahl statt Abschaffung oder Umgehung des Basic Link. |
| Die Broadcast-Adresse beweise, dass individuelle Datensessions fehlen | **Nicht ableitbar.** Sie charakterisiert diese Nachricht; individuelle Kontexte könnten daneben existieren. | Broadcast und individuellen Sessionverlauf getrennt untersuchen. |
| `chan_alloc: None` und fehlende Grants belegten fehlende Ressourcenverwaltung | **Nicht für das Gesamtsystem ableitbar.** Diese konkrete Broadcast-Nachricht muss keinen neuen Bearer zuteilen. | Zuteilungsereignisse und deren Lebenszyklus im vollständigen Trace prüfen. |
| Der komplette Paketdatenpfad fehle oder werde nicht benutzt | **Aus fünf Zeilen nicht beweisbar.** Der heute gelesene reine Upstream hat tatsächlich einen SNDCP-Stub; NetCore besitzt dagegen umfangreichen Code. | Codebefund an Repository und Commit binden, nicht aus einem Broadcastlog generalisieren. |
| „Warum landet alles im generischen Unitdata-Pfad?“ sei ziemlich sicher das Hauptproblem | **Unbelegte Verallgemeinerung.** Gezeigt wurde nur eine Nachricht, nicht „alles“. | Problem wieder ergebnisoffen untersuchen. |
| Ein `packet_data_flag: true` würde als zentraler Erfolgsnachweis genügen | **Nein.** Ein gesetztes Flag ersetzt weder Kontextaufbau noch IP-/WAP-Antwort oder Endgeräteabnahme. | Ende-zu-Ende-Kriterien verwenden. |

### 6.3 Bedeutung des Flags und der Adresse in den beigefügten Normen

EN 300 392-2 V3.8.1, Abschnitt **20.2.4.50**, Seite **603**, beschreibt das Packet-Data-Flag als Unterscheidung zwischen Paketdaten aus SNDCP und anderer Signalisierung, etwa für die Sendepriorisierung durch LLC. Abschnitt **28.3.5.5.5**, Seite **1119**, unterscheidet beim MS ausdrücklich `SN-DATA`/`SN-UNITDATA` von den übrigen PDUs. Das ist etwas anderes als „Paketdatendienst der Zelle aktiviert/deaktiviert“. [N1]

EN 300 392-1 V1.6.1, Abschnitt **7.7.8**, Seiten **37–38**, reserviert die aus 24 Einsen bestehende SSI für Informationen an alle MS. Die benachbarte Regel 7.7.7 für unadressierte Broadcast-MAC-PDUs darf nicht mit jeder adressierten Broadcast-Signalisierungsnachricht gleichgesetzt werden. [N2]

### 6.4 Tatsächlich neu ausgeführte Bitprüfung

Bei der Archivierung wurde mit Python geprüft:

```text
Länge der kopierten TL-Bitfolge: 78
Länge der kopierten LLC-Bitfolge: 82
Erste drei TL-Bits: 101 = 5
Nächste drei TL-Bits: 010 = 2
Erste vier LLC-Bits: 0010
LLC-Bitfolge ohne diese vier Bits == TL-Bitfolge: True
```

Das bestätigt die interne Konsistenz der beiden kopierten Bitfolgen und stützt die Präfixanalyse. Es ist **kein** kompletter PDU-Decoderlauf, kein CRC-/FEC-Test, kein Funkmitschnitt und kein WAP-Test.

## 7. Zusätzlich geprüfter Repository-Stand vom 04.10.2026

### 7.1 Prüfmethode und Branchgrenze

Die Codeprüfung erfolgte lesend über den GitHub-Connector, mit gezielten Dateibereichen am festgehaltenen Archivcommit. Ergänzend wurden die aktuellen `main`-Referenzen und Suchtreffer ermittelt. Es fand kein vollständiger Audit aller Module oder Branches statt.

Der GitHub-Vergleich `main` gegen den zu Beginn gelesenen Archivcommit meldete `diverged`, 23 Commits voraus und einen Commit zurück; Merge-Basis war `2fe2a1939a8795db3816d45973282781dae856f0`. Diese Angaben beschreiben die Historie, nicht die Gleichheit jedes Codepfads. `Archiving` wird deshalb nicht pauschal als Alias von `main` behandelt. [S9]

Ein lokaler Clone-Versuch scheiterte mit `Could not resolve host: github.com` und Exitstatus 128. Ein zusätzlicher Tarball-Downloadversuch kam ebenfalls nicht zustande. Deshalb wurden kein vollständiger lokaler Checkout, kein Cargo-Build und keine komplette Testsuite behauptet. Der Connector erlaubte dennoch die hier belegten gezielten Lesezugriffe.

### 7.2 Reiner BlueStation-Upstream

Am Upstream-Commit `09d4e0d9a0b8cf6c881e77353db325df9a4715aa` enthält `crates/tetra-entities/src/sndcp/sndcp_bs.rs` einen Stub: `rx_prim` prüft den SAP und ruft `unimplemented_log!("sndcp not implemented")` auf. Das ist ein direkter Codebeleg für die Lücke dieses konkret geprüften Upstream-Standes. [S2]

In dem gelesenen Abschnitt von `mle_bs.rs` baut der SNDCP-Discriminator-Zweig zwar ein `LtpdMleUnitdataInd`, verpackt es aber mit `Sap::LcmcSap` und `dest: TetraEntity::Cmce`. Diese Kombination wäre bei einem Ausbau des reinen Upstreams ausdrücklich zu prüfen. Sie ist ein statischer Codebefund, kein in diesem Lauf reproduzierter Fehler mit Funkgerät. [S6]

Die aktuell abgerufene Release-Liste beginnt mit **`v0.5.10-09d4e0d`**, veröffentlicht am **09.07.2026**. Die frühe Chatnennung einer damals neuesten Veröffentlichung vom 29.03.2026 wird daher nicht als heutiger Stand übernommen. Ihre damalige zeitpunktbezogene Richtigkeit wurde nicht rückwirkend rekonstruiert. [S10]

### 7.3 NetCore ist heute nicht mehr dieser Stub

Am geprüften NetCore-Archivcommit wurden unter anderem folgende konkrete Strukturen und Logik gelesen:

| Bereich | Direkt sichtbarer Codebefund | Nicht dadurch bewiesen |
|---|---|---|
| Teilnehmer-/Kontextzustand | `ContextTable`, Teilnehmer-Routen, Kontextschlüssel, Zustandstabellen-Anbindung | Vollständigkeit aller standardisierten Zustandsübergänge |
| Mehrgeräte-Ressourcen | `pdch_bearers: HashMap<u32, PdchBearer>`; Wiederverwendung eines Bearers pro ISSI; begrenzte Neuvergabe | Stabiler Parallelbetrieb auf realen Geräten |
| Slotbesitz | `TimeslotOwner::Sndcp`; gemeinsamer Allocator; Freigabe mit Besitzerprüfung | RF-Kontinuität und konfliktfreie Funktion in allen Ruf-/Recovery-Fällen |
| Sprachreserve | `reserved_voice_slots` begrenzt die Zulassung neuer Datenbearer | Harte Präemption bereits laufender Paketdaten oder eine vollständige Prioritätspolitik |
| Carrierwahl | Präferenzfolgen `[5,6,7,2,3,4]` beziehungsweise `[2,3,4,5,6,7]`; Mapping logischer Slots 5–7 auf Luftslots 2–4 | Multislot-Bündelung für ein einzelnes Endgerät |
| SNDCP-Ausgabe | `queue_ltpd_to`: SNDCP -> `TlpdSap` -> MLE, individuelle Adresse, Link-/Endpoint-ID, bestätigter/unbestätigter Dienst und optionale Zuteilung | Dass alle nachgelagerten LLC-/MAC-Wege korrekt arbeiten |
| IP-Daten | IPv4-Parsing, UDP-/TCP-Konstanten, Fragmentierungs-/Reassembly-Anbindung | Allgemeine IPv6- oder Kompressionsunterstützung |
| Gateway | Linux-TUN-Konfigurations- und Gatewaycode; Routing-/Firewall-/NAT-Parameter | Dass TUN, Routen und Firewall im laufenden Netz tatsächlich eingerichtet sind |
| WAP | `wap_ip.rs`: WTP-/WSP-Adapter, Endpoint-/Policy-Typen und WML-/XHTML-Anbindung | Vollständige Unterstützung sämtlicher WAP-1.x-/2.0-Profile oder jedes Browsers |
| Diagnose | Kontext-/Gateway-/PDCH-Telemetrie, LTPD-Zustand, Transferzähler, ControlEndpoint-Anbindung | Tatsächliche Live-Anbindung an einen laufenden Packet Core oder Control Room |
| Begrenzungen | Antwortcache mit 30 Sekunden TTL und Obergrenze 256; Kontext- und Reassembly-Grenzen | Fehlerfreiheit, Leakfreiheit oder bestandene Dauerlasttests |

Diese Befunde stammen aus gezielt gelesenen Quellbereichen, insbesondere `sndcp_bs.rs` Zeilen 1–150, 240–420 und 600–945, `packet_gateway.rs` Zeilen 1–115 sowie `wap_ip.rs` Zeilen 1–155. [S11–S13]

### 7.4 Konkrete Profilentscheidung und ein neuer Prüfpunkt

Die aktuelle `main`-Codesuche fand in `sec_cell.rs` die gemeinsame Profilbedingung:

```rust
self.sndcp_service && (self.wap_ip.enabled || self.packet_data_gateway.enabled)
```

In `umac_bs.rs` wird dieses Prädikat für die Profilankündigung herangezogen; im am Archivcommit gelesenen SNDCP-Code ruft `profile_enabled()` dieselbe benannte Funktion auf. Das ist der relevante Einstieg für eine heutige Konfigurations-/Ankündigungsprüfung. Es ist nicht gleichbedeutend mit der tatsächlich geladenen Konfiguration eines laufenden Senders. [S8, S11]

**Neuer Review-Kandidat, kein bestätigter Betriebsfehler:** Das am Archivcommit gelesene `queue_ltpd_to` setzt `packet_data_flag: true` im gemeinsamen Sendepfad. Bei der Fortsetzung sollte geprüft werden, ob die Klassifikation für Steuer-PDUs und Nutzdaten an dieser Stelle die beabsichtigte SAP-Semantik erfüllt. Die zuvor erläuterte Normstelle zum MS darf dabei nicht ohne Prüfung der Rollen und weiteren Verarbeitung als pauschaler SwMI-Bugnachweis angewendet werden. Weder ein blindes globales `true` noch ein blindes globales `false` ist eine sachgerechte Reparatur. [S11; N1]

### 7.5 Dokumentationsdrift ist bereits sichtbar

`Docs/SNDCP_COMPLETE.md` trägt im Inhalt den Stand 21.07.2026. Es beschreibt unter anderem einen einzelnen PDCH auf Hauptcarrier-TS2 und führt allgemeines TUN-/NAT-Routing als noch nicht angekündigte Funktion auf. Der heute gelesene Code enthält dagegen einen dynamischen PDCH-Pool und einen TUN-Gatewaypfad. [S14; S11, S12]

Daher sind die Aussagen dieser älteren Dokumentationsstufe nicht ungeprüft als vollständige Beschreibung des aktuellen Codes zu übernehmen. Auch das Wort „complete“ im Dateititel beweist weder heutige Vollständigkeit noch eine Endgeräteabnahme. Die Bereinigung dieser Dokumentationsdrift ist ein Roadmap-Kandidat, aber **nicht** Teil der hier ausschließlich unter `Docs/archive/` erlaubten Änderungen.

### 7.6 Verwandtes Archiv ausdrücklich getrennt halten

Bereits vorhanden ist:

[WAP/SNDCP, IP-Gateway, Multi-PDCH und Control Room – separates Archiv vom 03.10.2026](2026-10-03_wap-sndcp-ip-gateway-multi-pdch-und-control-room.md)

Dessen eingesehener Anfang behandelt einen anderen, späteren Verlauf mit ZIP-Auslieferungen, einem R1-Compilerfix, Multi-PDCH, Dashboard und Legacy-WAP. Es ist **nicht** die Abschlussdatei dieses Codex-/Logdiagnose-Chats. Seine historischen Buildangaben werden hier nicht zu eigenen Testergebnissen umetikettiert. Die Datei bleibt unverändert. [S15]

## 8. Relevante Dateien, Schnittstellen und Parameter

### 8.1 Historisch genannte Workspace-Bausteine

Die frühere Antwort nannte `tetra-core`, `tetra-saps`, `tetra-pdus`, `tetra-config`, `tetra-entities` sowie `bluestation-bs`, `bluestation-control`, `bluestation-telemetry` und `pdu-tool`. Das waren Strukturhinweise zur Analyse, kein Nachweis eines vollständig funktionierenden Datendienstes.

Die maßgeblichen im Log sichtbaren Schnittstellen sind `TlaSap`, `TlaTlUnitdataReqBl`, `BlUdata`, `TmaUnitdataReq` und `MacResource`. Für den heutigen SNDCP-Pfad kommen unter anderem `TlpdSap`, `LtpdMleUnitdataReq`, `ContextKey`, `ContextTable`, `PacketGateway` und `ControlEndpoint` hinzu.

### 8.2 Gezielt geprüfte Repository-Dateien

Alle NetCore-Angaben dieser Tabelle beziehen sich, soweit nicht anders angegeben, auf den in Abschnitt 1 bezeichneten Archivcommit. Blob-SHAs sind Inhaltsobjekte, keine zusätzlichen Feature-Commits.

| Datei | Gelesener Bereich / Zweck | Ermittelter Blob-SHA |
|---|---|---|
| `crates/tetra-entities/src/sndcp/sndcp_bs.rs` | Ausgewählte Bereiche 1–150, 240–420, 600–945; Zustand, Profil, Ressourcen, LTPD-Ausgabe | `c4260e16c9305efd14c01b38dff5fc6172f3c583` |
| `crates/tetra-entities/src/sndcp/packet_gateway.rs` | 1–115; TUN-Konzept, Konfiguration, Validierung, Konstanten | `53847d5bbe256a11ca7eacfc1ddf3a470d3d2267` |
| `crates/tetra-entities/src/sndcp/wap_ip.rs` | 1–155; WTP/WSP-Adapter, Typen und Größenbegrenzung | `ee5d5b52f297629d7a23f5454b612445f43a4756` |
| `crates/tetra-config/src/bluestation/sec_cell.rs` | 1–200; WAP-Konfiguration, Defaults und Gateway-Typen | `dfaf4bbaa609da79506f5b4adf73b061698cd65e` |
| `Docs/SNDCP_COMPLETE.md` | Protokollprofil, historische Grenzen und vorgeschlagene Prüfkommandos | `692cce7653ac0ac5368ea059c591cde23f8c944d` |
| `Docs/archive/README.md` | Vorhandenen Index gelesen; vorhandene Einträge erhalten | Ausgangsblob `e748626e12f9e003dd6d0110dcb0d96c081a61ca` |
| Separates WAP-/Multi-PDCH-Archiv | Anfang zur Abgrenzung der Gespräche gelesen | Keine Gleichsetzung mit diesem Chat |

Weitere durch aktuelle Codesuche gefundene Ansatzpunkte sind `sndcp/protocol.rs`, `sndcp/state.rs`, `sndcp/ip.rs`, `sndcp/fragment.rs`, `sndcp/wap_portal.rs`, `sndcp/mod.rs`, `umac/umac_bs.rs`, `net_control_room/protocol.rs`, `Docs/WAP_INTEGRATION.md` und `Docs/wap-port-spec.md`. Ihre Nennung ist **kein vollständiger Audit dieser Dateien**. Auch ein gleichnamiger SNDCP-Pfad unter `ms-mode/` wurde gefunden; Suchtreffer dort dürfen nicht mit dem Basisstationspfad im Hauptworkspace verwechselt werden.

### 8.3 Heute gelesene WAP-Defaults – keine bestätigte Live-Konfiguration

Aus `CfgWapIp::default()` am Archivcommit: [S16]

| Parameter | Quellcode-Default |
|---|---|
| `enabled` | `false` |
| `address` | `10.0.0.1` |
| `port` | `9200` im WAP-over-IPv4/UDP-Pfad |
| `response_ttl` | `32` |
| `dynamic_pool_prefix` | `10.0.0` |
| `dynamic_pool_first_host` / `dynamic_pool_last_host` | `2` / `254` |
| `allow_static_ipv4` | `true` |
| `accept_empty_probe`, `accept_root_path`, `accept_status_path`, `accept_status_wml_path` | jeweils `true` |
| `max_request_payload_bytes` | `1024` |
| `assume_pdch_ready_after_data_transmit` | `false` |
| `pdu_priority_max` | `4` |
| `ready_timer_code` / `standby_timer_code` / `response_wait_timer_code` | `8` / `4` / `7` |
| `mtu_code` | `2` |
| `network_default_data_priority` | `4` |
| `max_contexts_per_issi` / `max_total_contexts` | `4` / `64` |
| `strict_source_address` | `true` |

Timer- und MTU-Codes sind codierte Protokollwerte; sie werden hier nicht ohne Tabellenabgleich in Sekunden beziehungsweise Bytes umgedeutet. Das Poolpräfix ist eine Quellcodevorgabe, kein Nachweis für Jans tatsächlich verwendetes LAN oder Funkteilnehmernetz.

Der DTO-Kommentar nennt den Konfigurationsblock `[cell_info.wap_ip]`; unbekannte Schlüssel werden dort durch `deny_unknown_fields` zurückgewiesen. Zur Laufzeit werden die kompilierten Werte über `cfg.cell.wap_ip` und `cfg.cell.packet_data_gateway` verwendet. Ein Port im WAP-Adapter ist nicht automatisch ein auf Linux mit `ss` sichtbarer Listening-Socket; der konkrete Paketpfad ist zu beachten.

### 8.4 Weitere konkrete Quellparameter

`packet_gateway.rs` definiert unter anderem die nftables-Tabelle `netcore_tetra`, die iptables-Ketten `NETCORE_TETRA_FWD` und `NETCORE_TETRA_NAT` sowie den Zustandspfad `/run/netcore-tetra/packet-gateway-sysctls`. Der gelesene Konfigurationstyp enthält Interface, Präfixlänge, MTU, automatische Konfiguration, IPv4-Forwarding, verwaltete Weiterleitung, Regeln für unaufgefordert eingehenden Verkehr, NAT-Modus, Firewallbackend und externes Interface. [S12]

`wap_ip.rs` enthält eine `WSP_SDU_CAP` von 545. Das ist eine lokale Codebegrenzung, keine in diesem Archiv behauptete universelle TETRA- oder WAP-MTU. [S13]

Die historische Unterhaltung legte **keine** Service-Unit, keinen Installationspfad, kein Deployment-Ziel, keine reale Dienst-IP, keine DNS-Konfiguration und keine konkrete Funkfrequenz für diesen WAP-Test fest. Solche Werte dürfen nicht aus anderen Projektchats ergänzt und als hier getestet ausgegeben werden.

## 9. Befehle, Arbeitsabläufe und tatsächlicher Ausführungsstand

### 9.1 Historisch tatsächlich ausgeführt

Im sichtbaren historischen Gespräch wurde kein Shell-Befehl, Build, Patch, Commit, Push, Installations- oder Reparaturlauf mit Ergebnis dokumentiert. Der Nutzer lieferte Laufzeitlogs, aber nicht den zugehörigen Startbefehl.

Die Assistentenantworten enthielten ausformulierte Arbeitsaufträge, keine nachgewiesenen Codex-Ausführungen. Ein fertiger Codex-CLI-Befehl wurde angeboten, aber im verfügbaren Verlauf nicht mehr geliefert. Es gibt deshalb keine belastbar zu archivierenden CLI-Flags oder eine angeblich erfolgreiche Agentensitzung.

### 9.2 Historisch vorgeschlagene Suchbegriffe

```text
packet_data_flag
TlaTlUnitdataReqBl
TmaUnitdataReq
chan_alloc
slot_granting_element
chan_alloc_element
sndcp
llc
pdu
service
bearer
attach
packet data
wap
```

Zusätzlich sollten Zustandsautomaten für Session/Attach/Detach/Release, Retry-/Timeout-Logik, Uplink, Slot-/Channel-Management und SAP-Routing untersucht werden. Die Suche nach diesen Begriffen war ein Vorschlag; ein Treffer oder Nichttreffer allein wäre keine vollständige Funktionsprüfung.

### 9.3 Im aktuellen Repository dokumentierte, hier nicht ausgeführte Prüfkommandos

`Docs/SNDCP_COMPLETE.md` nennt: [S14]

```bash
cargo test -p tetra-core timeslot_alloc
cargo test -p tetra-config
cargo test -p tetra-entities sndcp --features runtime
cargo build --release \
  -p bluestation-bs \
  -p netcore-control-room \
  -p netcore-control-room-operator \
  --features bluestation-bs/asterisk
```

**Status:** aus einer vorhandenen Repository-Dokumentation übernommene Prüfkommandos; nicht während dieser Archivierung erfolgreich ausgeführt. Vor Verwendung sind aktueller Workspace, Toolchain und Features zu bestätigen. Auch erfolgreiche Ergebnisse dieser Kommandos würden keine Funkgeräteabnahme ersetzen.

### 9.4 Sinnvoller Wiederaufnahmeablauf – neu vorgeschlagen, nicht durchgeführt

1. Tatsächlich installierten Repository-/Commit-/Buildstand und Binärhash erfassen; aktive, bereinigte Konfiguration sichern.
2. Ein konkretes Testgerät mit Modell, Firmware, Browser-/Packet-Data-Profil und sichtbarer Fehlermeldung dokumentieren.
3. Logs vom Start beziehungsweise der Registrierung bis nach einem gezielt ausgelösten Browserzugriff erfassen, einschließlich Uplink und Dienstankündigung.
4. Zuerst Service-Ankündigung und Endgeräteanforderung, dann Kontext-/Linkaufbau, anschließend IP- und WAP-Antwort nachweisen.
5. Nach einem reproduzierbaren Einzelgerätetest mindestens zwei echte Geräte parallel prüfen; abschließende Stückzahl und Modellmatrix separat festlegen.
6. Fehlerfälle und Dauerbetrieb mit Logs und Ressourcenbeobachtung abnehmen.

Dieser Ablauf ist keine Erlaubnis, im Archivbranch neue Funkfunktionen zu implementieren. Dafür ist ein eigener Entwicklungsauftrag mit bewusst gewähltem Ausgangsstand erforderlich.

## 10. Fehlerbild, Ursachen und verbleibende Probleme

### 10.1 Was gesichert ist

Gesichert ist die Betreiberbeobachtung „noch kein WAP-Angebot“ zusammen mit dem beschriebenen Downlink-Broadcastlog. Gesichert ist ferner, dass die historische Antwort daraus eine zu starke systemweite Diagnose ableitete.

**Keine funktionierende Reparatur ist in diesem Chat nachgewiesen.** Es gibt weder eine bestätigte Änderung an `packet_data_flag` noch einen nachfolgenden erfolgreichen Browseraufruf oder eine erfolgreiche Mehrgeräteprüfung.

### 10.2 Offene Ursachen – keine davon durch den Ausschnitt bewiesen

| Mögliche Fehlergrenze | Erforderlicher Nachweis |
|---|---|
| Falscher oder alter Binary-/Konfigurationsstand | Commit/Version, Binärhash, aktive Konfigurationsdatei und Startparameter |
| Paketdatendienst nicht angekündigt | Decodierter vollständiger Service-Broadcast / SYSINFO-Pfad und dessen Konfigurationsprädikat |
| Gerät fordert den Dienst nicht an | Geräteprofil, Browserstatus, Uplink-Trace beim Verbindungsversuch |
| SNDCP-Aktivierung abgelehnt oder nicht verarbeitet | `SN-ACTIVATE...`-Dialog, Ablehnungsursache, MLE/SAP-Routing |
| LLC-/PDCH-Übergang fehlerhaft | Linkaufbau, Request/Response, Ressourcenbesitz, Timer und Rückkehr auf Control Channel |
| IP-Zuordnung oder Routing fehlerhaft | Ausgehandelte Adresse, Kontextzuordnung, Hin-/Rückpfad und gegebenenfalls TUN-/Routingzustand |
| WAP-Profil oder Inhalt inkompatibel | Tatsächliche Requests/Responses, Zieladresse/Port, WTP/WSP- beziehungsweise Geräteprofil |
| Mehrgeräte-Kollisionen oder Ressourcenreste | Paralleltrace mit eindeutiger Teilnehmer-/Kontextkorrelation und Ressourcenbilanz |

Die heutige Upstream-Stubbeobachtung kann erklären, warum dieser konkrete reine Upstream keinen vollständigen Dienst liefert. Sie identifiziert aber nicht rückwirkend die Ursache in einer unbekannten historischen Laufzeitversion und erst recht nicht automatisch im heutigen NetCore-Fork.

## 11. Tests und ihre Grenzen

### 11.1 Historischer Nachweisstand

| Prüfung | Ergebnis im historischen Chat |
|---|---|
| Codex hat den Auftrag tatsächlich ausgeführt | Nicht belegt |
| Rust-Build nach WAP-Änderungen | Nicht belegt |
| Unit-/Integrations-/Property-Tests | Nur gefordert, keine Ergebnisse |
| Vollständiger PDU-Replay | Nicht belegt |
| End-to-End mit einem realen TETRA-Gerät | Kein Erfolg dokumentiert |
| Stabiler Mehrgerätebetrieb | Nicht belegt |
| Recovery nach Funkabbruch oder Neustart | Nicht belegt |
| WAP- oder ETSI-Konformität | Nicht belegt |

### 11.2 Während der Archivierung tatsächlich geprüft

| Prüfung | Ergebnis | Grenze |
|---|---|---|
| GitHub-Lesezugriff und Branchidentität | Zielbranch und Ausgangscommit gelesen; `main` und Upstream getrennt ermittelt | Keine Aussage über installierte Systeme |
| Vorhandener Archivindex | Vorhandene Einträge gelesen; anderes WAP-Archiv erkannt | Kein Zusammenführen unterschiedlicher Gesprächshistorien |
| Ziel-Dateiname vor Schreiben | Abruf meldete `404 Not Found` | Keine vorhandene gleichnamige Zusammenfassung überschreiben |
| Gezielt gelesene Codebereiche | Upstream-Stub, Broadcastpfad, NetCore-Zustands-/Gateway-/Ressourcenlogik gefunden | Kein vollständiger Stack-Audit |
| Bitfolgenprüfung | 78/82 Bits; `0010`-Präfix; Restgleichheit; `101/010`-Einordnung | Kein On-Air-Test |
| PDF-Inventar | 25 Dateien erfolgreich geöffnet, Seitenzahlen und SHA-256 berechnet | Kein vollständiger semantischer Normenaudit |
| PDF-Abbildungen / Tabellen | Seiten 1022, 1081 und 1082 lokal gerendert und angesehen | Keine vollständige visuelle Prüfung aller PDFs |
| Lokaler Repository-Clone | Fehlgeschlagen: DNS-Auflösung von GitHub nicht möglich | Daher kein lokaler Gesamtbuild |
| Abschlussprüfung der Git-Ablage | Ergebnis und tatsächlicher Commit werden in der Abschlussmeldung angegeben | Kein vorweggenommenes Testergebnis in dieser Datei |

Bei der abschließenden erneuten Abfrage des beweglichen Archivrefs und des Index traten vorübergehend GitHub-Connector-ReadTimeouts auf. Eine Branchsuche bestätigte weiterhin `Archiving`. Der zuletzt erfolgreich gelesene Index und sein Blob-SHA sind oben festgehalten. Schreiboperationen müssen diese Fassung mit Konfliktschutz berücksichtigen; ein Timeout ist kein Beleg, dass ein Commit stattgefunden hat.

### 11.3 Weiterhin erforderliche Abnahmematrix

Die historischen Prompts verlangten bereits Unit-, Integrations-, Property-orientierte und Fixture-/Replay-Tests sowie einen lokalen End-to-End-Harness. Für das tatsächliche Nutzerziel müssen diese durch eine reale Interop-Matrix ergänzt werden.

Zu erfassen sind mindestens Endgerätemodell, Firmware, aktiviertes Browser-/Datendienstprofil, getesteter Code-/Konfigurationsstand, Kontextaufbau, Adresszuweisung, beidseitiger Datentransfer, konkreter Seiteninhalt, Parallelbetrieb und Recovery. Die Zielwerte für Laufzeit, Fehlerrate, Antwortzeit und Teilnehmerzahl sind noch zu vereinbaren.

Wichtige Fehlerfälle bleiben: konkurrierende Aufbauten, unterschiedliche Retries, Duplikate, Paketverlust, verspätete beziehungsweise ungeordnete Daten, unvollständige PDUs, unbekannte Kontexte, wiederholte Freigaben, Ressourcenerschöpfung, Funkabbruch, Wiederanmeldung, Neustart und Koexistenz mit Sprach-/SDS-Betrieb. Ein Simulations- oder In-Memory-Harness kann diese Prüfungen vorbereiten, aber nicht die reale Funkstrecke ersetzen.

## 12. Verworfene, ersetzte und weiter gültige Ansätze

**Zu verwerfen als Diagnosegrundlage:** ein globaler Schluss aus `packet_data_flag: false`; die Gleichsetzung Basic Link = kein Packet Data; die Annahme, jede relevante Nachricht müsse eine Kanalzuteilung enthalten; die Aussage, sämtliche Daten würden aufgrund dieses Ausschnitts im falschen Pfad landen.

**Nicht als aktuelle Implementierungsstrategie übernehmen:** den heutigen NetCore-Fork so behandeln, als sei er noch der reine Upstream-Stub; vorhandene SNDCP-/Gatewaykomponenten erneut parallel bauen; alte Promptpfade oder alte Dokumentationsgrenzen ungeprüft zur aktuellen Architektur erklären.

**Weiter gültig:** erst den konkreten Ausgangsstand verstehen; Normbezug an PDU-/SAP-/Timerentscheidungen angeben; in begrenzten, testbaren Schritten arbeiten; erwartete Fehlerfälle explizit behandeln; tatsächliche Endgeräteinteroperabilität messen; Teilstände ehrlich kennzeichnen.

**Nicht ausreichend als Endergebnis:** nur SDS, nur ein lokaler Testdienst ohne Funktransport, nur ein einzelnes Gerät, nur ein gesetztes Flag oder nur grüne Parsertests. Diese Dinge können nützliche Meilensteine sein, erfüllen aber nicht allein den festgelegten Zielumfang.

## 13. Offene Aufgaben und Roadmap-Kandidaten

Die folgende Reihenfolge ist eine **bei dieser Archivierung vorgeschlagene Priorisierung**. Im historischen Chat wurden keine verbindlichen Termine oder vollständigen Prioritätsstufen vereinbart. Es wurden keine externen Roadmap-Dateien oder Issues geändert.

| Priorität | Arbeitspaket | Abhängigkeit / Abnahmekriterium | Status |
|---|---|---|---|
| P0 | Tatsächlichen Laufzeitstand identifizieren | Repository, Branch/Commit, Binärhash, Konfiguration und Geräteprofil bekannt | Offen |
| P0 | Historische Fehlannahmen aus dem Codex-Auftrag entfernen | Broadcast nicht als globalen Paketdatentest verwenden; Basic-Link-Regeln korrekt berücksichtigen | In dieser Dokumentation korrigiert; Agentenfortsetzung noch offen |
| P0 | Bestehenden NetCore-Code statt pauschalem Neubau bewerten | GAP-Matrix mit Codebeleg, Normstelle, Test und Restlücke | Offen |
| P0 | Ankündigung und Aktivierung nachweisen | Service-Informationen und konkreter SNDCP-Aufbaudialog im korrelierten Trace | Offen |
| P1 | Einen reproduzierbaren End-to-End-WAP/IP-Test herstellen | Echtes MS, gültiger Kontext, bidirektionale Nutzdaten und sichtbare Antwort | Offen / Betriebsnachweis fehlt |
| P1 | Mehrgerätebetrieb und Kontexttrennung prüfen | Mindestens zwei parallele reale Geräte als erster Nachweis; endgültige Matrix festlegen | Offen |
| P1 | Ressourcen-/Recovery-Abnahme | Keine verwaisten Slots/Kontexte nach Fehlern, Zeitabläufen und Neustarts | Offen |
| P1 | Packet-Data-Flag-Semantik im gemeinsamen NetCore-Sendepfad prüfen | Steuer-/Nutzdaten und MS-/SwMI-Rollen anhand der konkreten SAP-Verträge unterscheiden | Neuer Review-Kandidat |
| P1 | TUN-/Routing-/Firewall- und WAP-Profilgrenzen abnehmen | Vorhandenen Code mit tatsächlicher Umgebung prüfen, nicht nur Konfigurationsfelder zählen | Offen |
| P2 | Dokumentationsdrift bereinigen | Ein-Slot-/kein-TUN-Aussagen mit aktuellem Code und Capability-Profil abgleichen | Offen; außerhalb dieses Archivauftrags |
| P2 | Replay-/Fehlerlast-/Dauertests und reproduzierbare Ergebnisablage | Exakte Versionen, Testdaten, Logs und Pass/Fail-Kriterien | Geplant in früheren Prompts, nicht belegt |
| P2 | Observability vervollständigen | Korrelation zwischen Teilnehmer, Kontext, Link, Ressource, IP-/WAP-Transaktion und Fehlerursache | Geplant; heutiger Teilcode vorhanden |
| Nach Bedarf | Weitere Geräteprofile, Optimierungen und optionale Protokollfunktionen | Erst nach belastbarer Kernabnahme und zusätzlicher Umfangsentscheidung | Idee |

### 13.1 Kleine, weiterhin relevante Wünsche und Nebenideen

Erhalten bleiben insbesondere: konservative Defaults; konfigurierbare Timer und Retries; klare Modul-/Dateiliste für Änderungen; nachvollziehbare öffentliche Rust-Typen; robuste Fehlerbehandlung ohne stille Panics in relevanten Produktivpfaden; möglichst wenige neue Abhängigkeiten; Logs und Metriken für Zustandsübergänge; idempotente Behandlung doppelter Anforderungen; saubere Ressourcenfreigabe; dokumentierte Recovery- und Neustartfälle; Replay-Fixtures; Interop-Checklisten; explizite Blocker und minimale nächste Arbeitspakete.

Die frühere Idee eines zweistufigen Codex-Auftrags bleibt sinnvoll als Organisation: zunächst Bestands-/GAP-Analyse, anschließend Umsetzung. Sie wurde aber nicht als bereits erledigte Phase nachgewiesen. Ein noch zu formulierender CLI-Aufruf darf nicht mit einer historisch erfolgreich ausgeführten Terminalanweisung verwechselt werden.

## 14. Archiv der damaligen Codex-Arbeitsaufträge

Dieser Abschnitt erhält den technischen Umfang der Prompts in zusammengefasster Form. Er ist **keine neue Ausführungsanweisung für den Archivbranch**.

### 14.1 Erster umfangreicher Auftrag

Ausgangspunkt war `MidnightBlueLabs/tetra-bluestation`. Ziel war ein stabiler, interoperabler TETRA-Paketdatenpfad für echten WAP/IP-Betrieb mit mehreren Geräten, nicht nur ein PoC.

**Phase 1 – Bestandsaufnahme:** Workspace, Crates, Binaries, Protokollschichten, SAPs, PDU-Typen, Timer, Zustandsautomaten, Konfiguration und Telemetrie lesen. Broadcast/Sync, Registrierung, Mobility, Sprache, SDS, Packet Data, Link-/Nutzdatenpfade, Slot-/Kanalverwaltung, Uplink und Mehrgeräteverhalten inventarisieren. GAP-Report mit konkreten Dateien und Lücken erstellen.

**Phase 2 – Architektur:** fehlende Komponenten und Module, Control-/Nutzdatenpfad, Kontext-/Bearer-Lebenszyklus, Konfiguration, Fehlerbehandlung, Timer, Logging und Metriken dokumentieren. Bestehende Crates bevorzugen, unnötige neue Abstraktionen vermeiden.

**Phase 3 – Implementierung:** Kontext-/Session-Aufbau und -Abbau, Ressourcenvergabe, Fragmentierung/Reassembly soweit notwendig, Funk-IP-/WAP-Anbindung, konservative Defaults, konfigurierbare Retries/Timer sowie Recovery ergänzen. Kleine logisch getrennte Commits und nachvollziehbare Änderungen vorsehen.

**Phase 4 – Interoperabilität:** nicht nur den Erfolgsfall implementieren. Unterschiedliche Geräte-/Retry-Verhalten, parallele Sessions, Verluste, Reihenfolgenprobleme, Wiederanmeldung, Duplikate, Ressourcenreste, Neustarts und unerwartete PDUs berücksichtigen.

**Phase 5 – Tests:** Unit-, Integrations-, Property-orientierte und Replay-/Fixture-Tests; lokaler End-to-End-Harness; Interop-Checklisten für mehrere Gerätetypen. Ungeklärte Spezifikations- oder Implementierungsblocker offen ausweisen und Teilstände nicht als Vollimplementierung darstellen.

**Phase 6 – Dokumentation:** Build-/Laborbetrieb, tatsächliche Funktionen, Annahmen, Grenzen, relevante Logs/Metriken und Fehlersuche beschreiben.

Nach größeren Schritten sollten Verständnis, Änderungen, Risiken, Testergebnisse und bewusste Lücken berichtet werden. Als zusätzliche Alternative wurde zuerst ein reiner Analyseauftrag mit priorisierter Roadmap, Risikoanalyse und betroffenen Dateien und danach ein Auftrag zur Umsetzung der ersten Phasen vorgeschlagen.

### 14.2 Zweiter, logbezogener Auftrag

Der zweite Prompt verlangte erneut Repoanalyse, GAP-Report und Zielarchitektur, fokussierte aber besonders `packet_data_flag`, `TlaTlUnitdataReqBl`, `TmaUnitdataReq`, Kanal-/Slotzuteilung, MLE/LLC/UMAC und Session-/Retry-/Timerpfade.

Er sollte erklären, warum vermeintlich Paketdaten über einen generischen Broadcast-/Basic-Link-Pfad liefen, und danach einen minimalen dokumentierten Labor-Paketdaten-/IP-Pfad bauen: Auswahl des Datenpfads, Sessionaufbau/-abbau, Ressourcen, Fragmentierung, Teilnehmertrennung, Recovery und Telemetrie. Gefordert waren Unit-/Integrations-/Replay-/End-to-End-Tests einschließlich Flag-Auswahl, Aufbau/Freigabe, Timeouts, Duplikaten, unvollständigen PDUs, Wiederanmeldung und Neustart.

Ein Zusatzprompt wiederholte die Logfelder und verlangte die exakte Lokalisierung der vermeintlich fehlenden Paketdaten-Auswahl.

**Für die Fortsetzung zu ersetzen ist die vorweggenommene Fehlerursache.** Richtig ist ein ergebnisoffener Auftrag: den Broadcast korrekt einordnen, den tatsächlich vorhandenen SNDCP-Pfad prüfen, Lücken mit Code und vollständigen Laufzeitdaten belegen und erst danach gezielt ergänzen. Das Nutzerziel bleibt stabiler Mehrgeräte-WAP/IP-Betrieb.

### 14.3 Damals vorgeschlagene Dokumentpfade

| Promptfamilie | Vorgeschlagene Pfade |
|---|---|
| WAP/IP-Gesamtauftrag | `docs/wap_ip_gap_analysis.md`, `docs/wap_ip_design.md`, `docs/wap_ip_lab_setup.md`, `docs/wap_ip_troubleshooting.md`, `docs/wap_ip_interop_matrix.md` |
| Logbezogener Paketdatenauftrag | `docs/packet_data_gap_analysis.md`, `docs/packet_data_design.md`, `docs/packet_data_lab_setup.md`, `docs/packet_data_troubleshooting.md`, `docs/packet_data_known_limits.md` |

Diese kleingeschriebenen `docs/`-Pfade waren Vorschläge der historischen Prompts. Der Chat belegt keine damalige Erstellung. Sie werden durch diesen Archivauftrag nicht angelegt. Die aktuelle Archivvorgabe lautet ausdrücklich **`Docs/archive/`**; Groß-/Kleinschreibung ist zu beachten.

## 15. Anhänge und PDF-Quelleninventar

### 15.1 Umfang und Zuordnung

In der Arbeitsumgebung waren **25 PDF-Dateien mit zusammen 8.061 Dateiseiten** verfügbar: 24 Einzeldateien mit zusammen 3.961 Seiten und die 4.100-seitige Zusammenstellung `ETSI.pdf`. Das ist eine Dateiseitenzählung, keine Zahl einzigartiger Normseiten; Überschneidungen sind möglich und wurden nicht vollständig dedupliziert.

Alle Dateien ließen sich zur Inventarisierung öffnen. Erfasst wurden Dateiname, Seitenzahl, Titelseite und SHA-256. Eine vollständige inhaltliche Auswertung sämtlicher Normen oder eine Prüfung ihrer heute jeweils neuesten Fassung wurde nicht vorgenommen. Die unten genannten Versionen bezeichnen die **bereitgestellten Dokumente**, nicht pauschal die aktuellsten ETSI-Ausgaben.

### 15.2 Inventar und Relevanz

| Datei | Dokument laut Titelseite | Seiten | Bedeutung / Auswertungsumfang |
|---|---|---:|---|
| `en_30039201v010601p.pdf` | EN 300 392-1 V1.6.1, 2020-04; General network design | 182 | Direkt relevant: Adressierung/Broadcast; gezielte Prüfung von 7.7.7–7.7.8. |
| `en_30039202v030801p.pdf` | EN 300 392-2 V3.8.1, 2016-08; Air Interface | 1445 | Hauptquelle für Packet-Data-Flag, SNDCP, Linkauswahl und WAP-Abgrenzung; gezielte Abschnittsprüfung. |
| `en_30039205v020701p.pdf` | EN 300 392-5 V2.7.1, 2020-04; PEI | 320 | Referenz für externe Terminal-/IP-Anbindung; kein Nachweis eines eingebauten Browserprofils. Titel/Inhaltsübersicht berücksichtigt. |
| `en_30039207v030501p.pdf` | EN 300 392-7 V3.5.1, 2019-07; Security | 216 | Kontextquelle; keine Kryptographie-/Schlüsselimplementierung aus diesem Archivauftrag. |
| `en_30039401v030301p.pdf` | EN 300 394-1 V3.3.1, 2015-04; Conformance testing, Radio | 169 | Mögliche spätere Funkprüfgrundlage; hier kein Konformitätstest durchgeführt. |
| `ets_30039214e01v.pdf` | Final draft prETS 300 392-14, 1997-09; PICS proforma | 61 | Historische Referenz für strukturierte Implementierungserklärungen; nicht als aktuelle vollständige SNDCP-Checkliste verwendet. |
| `en_3003920308v010401p.pdf` | EN 300 392-3-8 V1.4.1, 2020-04; ISI Generic Speech Format | 22 | ISI-/Sprachtransport, nicht die fehlende WAP-Funkimplementierung; inventarisiert. |
| `en_3003920304v010301p.pdf` | EN 300 392-3-4 V1.3.1, 2010-08; ANF-ISISDS | 28 | SDS-Interworking; kein Ersatz für den gewählten IP-Dienst. |
| `en_3003920303v010301p.pdf` | EN 300 392-3-3 V1.3.1, 2011-11; ANF-ISIGC | 251 | Gruppenruf-/ISI-Hintergrund; keine neue Anforderung aus dem WAP-Chat. |
| `en_3003920313v010201p.pdf` | EN 300 392-3-13 V1.2.1, 2020-04; transportunabhängiges ANF-ISIGC | 191 | Gruppenruf-/ISI-Hintergrund; inventarisiert. |
| `en_3003920315v010500a.pdf` | Draft EN 300 392-3-15 V1.5.0, 2026-04; ANF-ISIMM | 380 | Im bereitgestellten Dokument ausdrücklich Entwurf; nicht als abschließend verabschiedete Spezifikation ausgegeben. |
| `en_30039209v010701p.pdf` | EN 300 392-9 V1.7.1, 2020-04; supplementary services | 46 | Zusatzdienst-Rahmen; inventarisiert. |
| `en_3003921006v010401p.pdf` | EN 300 392-10-6 V1.4.1, 2006-08; CAD, Stage 1 | 20 | Dispatcherfreigabe; kein Auftrag zur Umsetzung aus diesem Chat. |
| `en_3003921018v010301p.pdf` | EN 300 392-10-18 V1.3.1, 2003-10; BOC, Stage 1 | 17 | Rufsperre; inventarisiert. |
| `en_3003921101v010201p.pdf` | EN 300 392-11-1 V1.2.1, 2004-01; Call Identification, Stage 2 | 44 | Zusatzdienst-Hintergrund; inventarisiert. |
| `en_3003921201v010202p.pdf` | EN 300 392-12-1 V1.2.2, 2007-08; Call Identification, Stage 3 | 56 | Zusatzdienst-Hintergrund; inventarisiert. |
| `en_3003921114v010101p.pdf` | EN 300 392-11-14 V1.1.1, 2002-07; Late Entry, Stage 2 | 23 | Rufbeitritt; keine Mehrgeräte-WAP-Abnahme. |
| `en_3003921117v010102p.pdf` | EN 300 392-11-17 V1.1.2, 2002-01; Include Call, Stage 2 | 18 | Titelseitenversion übernommen; inventarisiert. |
| `en_3003921216v010400a.pdf` | DRAFT EN 300 392-12-16 V1.4.0, 2026-03; PPC, Stage 3 | 67 | Entwurf; mögliche spätere Prioritäts-/Koexistenzreferenz, keine hier vereinbarte vollständige PPC-Implementierung. |
| `en_30039502v010303p.pdf` | EN 300 395-2 V1.3.3, 2025-02; TETRA codec | 94 | Sprachcodec, kein WAP-Transport; inventarisiert. |
| `en_300812v020101p.pdf` | EN 300 812 V2.1.1, 2001-12; SIM-ME | 156 | Karten-/Terminalreferenz; keine pauschale Voraussetzung für jedes WAP-Gerät abgeleitet. |
| `ts_10081201v020205p.pdf` | TS 100 812-1 V2.2.5, 2003-10; UICC physical/logical characteristics | 8 | Karteninterface; Titel/Scope berücksichtigt. |
| `es_20081201v020205p.pdf` | ES 200 812-1 V2.2.5, 2003-12; UICC physical/logical characteristics | 8 | Eigenständige ES-Ausgabe; nicht mit der TS-Datei zusammengeworfen. |
| `es_20081202v020401m.pdf` | Final draft ES 200 812-2 V2.4.1, 2005-08; TSIM application | 139 | Im bereitgestellten Dokument Final draft; inventarisiert. |
| `ETSI.pdf` | Zusammenstellung; beginnt mit EN 300 812 V2.1.1 | 4100 | Keine einzelne zusätzliche Norm. Vollständige Reihenfolge und Deckung mit den Einzeldateien nicht auditiert. |

### 15.3 Reproduzierbare SHA-256-Prüfsummen

Die folgenden Hashes wurden aus den tatsächlich verfügbaren lokalen Dateien berechnet. Sie sind keine Bestätigung einer vollständigen normativen Inhaltsprüfung.

```text
9434dad1e7bc80ca39b0edadd8e3b9995fda5ae5f5c3dd565af05708d5059e38  ETSI.pdf
788722e566098957bea1a9a5096e9dae1eace1da9f5d38c530c80230e5b9b9cb  en_30039201v010601p.pdf
3f07b1e4ad73fabc16277a900006fc19844b3a882bbc2bc1d74d78f6156daf28  en_30039202v030801p.pdf
94ca61038b3ff826e6adcfde56e00473dd996e1a47f9f84e7b9d2aa6c3133fd2  en_3003920303v010301p.pdf
8a38cc6238c6da726e4f07d7d377b2c827447a1448a60ee5ef2a126cb3a0ef9d  en_3003920304v010301p.pdf
4d993a25e35da7b1f2291ae281cb3ea5b3a7b702ab7eb3233bdf430a2c70909d  en_3003920308v010401p.pdf
b47d63853f83a6a4adda08c0e325c8ca13d358e2f8840bfba4bce430e600b1bd  en_3003920313v010201p.pdf
e959709667192e22d2dfeea51b48f9a579a4c107d952999056b7fad93a6d2100  en_3003920315v010500a.pdf
10aaf78988ae4080c3dfe90165ee7f4f3fe8eda402e09fc1e6fa0a0ad890758d  en_30039205v020701p.pdf
df47af9a642b6eba7d7d5cf5fb7aa3f961e9e03bb912316bd2d189ce47344e08  en_30039207v030501p.pdf
cad45938bcdf2fa4e8063d4aa9e7d7c06c45f729d3c55dc033e00c27af9f2e06  en_30039209v010701p.pdf
32b6ee43602ef88a0b5c209b1c69ad35ba8b2e660502257c2dfaa6205a346523  en_3003921006v010401p.pdf
4cc10bcd94d39201538639cefb529809729563f13f87cfe6ae52b44c253b0b8a  en_3003921018v010301p.pdf
852c17ea566757e80ecf0d2718ae7839d9e218aa4384b63d1277ed7e25d86c69  en_3003921101v010201p.pdf
ba1882df71dbb84a0c4f0f7dcea1dc8df968044457da03df9290a50e8821df32  en_3003921114v010101p.pdf
69ce800e352b1ecfc337416fd7b54c1bdf2d9270171af519ac2f5acc1ba85ca6  en_3003921117v010102p.pdf
4d5b56a188fb73c417d67b27da619cad2e49efb3b57306fd4ae51e140c018018  en_3003921201v010202p.pdf
c0ee7859f448c4072befc25905c29b751b5151d3590a880a2d654309a3ba6c02  en_3003921216v010400a.pdf
2d9c327e5ccf1476335e1b7a4f58a0ca7afc93911c30e8aad5ed7dc9ccf3276a  en_30039401v030301p.pdf
ac716ca18082cc0fd2ee768a14f03deb48fa78a77e83751b1a6b319c7fc2106a  en_30039502v010303p.pdf
196effeaae473a98244c89538e1562daa58bb1d74cbfe7de78e58ceeafbad18b  en_300812v020101p.pdf
346dc545049a7397ca86831731bbc05e96f61ff047d6e7f6b7de56da3aa408e9  es_20081201v020205p.pdf
330f045a908eefd249fd7450e5b71bd08e8a7b20ce5b86dc9b87c9138a5c7268  es_20081202v020401m.pdf
2b703f3ebb883c90dab40e1b3d1a634a6acf9c9a20d468fd108ea02226c2bf2c  ets_30039214e01v.pdf
96df75f5ddf1c3ae4ac75f796ee97babca68fa81aca22f39ba0e0658bd57eed1  ts_10081201v020205p.pdf
```

### 15.4 Bilder und tatsächlicher Ablageumfang

Im historischen Gespräch und unter den vorgefundenen Originaldateien war **kein eigenständiges Chatbild** vorhanden. Die sichtbaren Bilder im Quellenkontext sind gerenderte PDF-Seiten. Es wurden deshalb keine Bilder anderer Chats übernommen und keine fehlenden Screenshots rekonstruiert.

Für die heutige Quellenprüfung wurden drei Seiten der vorhandenen Air-Interface-PDF lokal gerendert. Das sind neu erzeugte Arbeitsansichten, keine historischen Chatbilder. Sie werden nicht als solche ausgegeben.

Die PDF-Dateien selbst werden hier als Quellen mit Version, Seitenzahl und Hash inventarisiert, nicht als vollständige Binärkopien ins Repository aufgenommen. Die Archivdatei bewahrt ihre relevanten Aussagen und Fundstellen. Damit ist transparent: **Dieses Archiv enthält keinen vollständigen Spiegel aller bereitgestellten PDF-Dateien.**

## 16. Quellen, Referenzen und Wiederaufnahmehinweise

### 16.1 Historische Primärquelle

**[H1]** Der in dieser Sitzung sichtbare Gesprächsverlauf, insbesondere die ausdrückliche Zielschärfung des Nutzers und der in Abschnitt 5 unverändert erhaltene Logauszug. Die Codex-Prompts sind historische Assistentenvorschläge, keine Ausführungsprotokolle.

### 16.2 Bereitgestellte normative Quellen

**[N1]** `en_30039202v030801p.pdf`, ETSI EN 300 392-2 V3.8.1 (2016-08): insbesondere 20.2.4.50/S. 603; 28.0–28.1/S. 1021–1023; 28.3.4.1–28.3.4.2/S. 1081–1082; 28.3.5.5.5/S. 1119; 29.5.8/S. 1220. Die Aussagen zu Service-Ankündigungen wurden ergänzend über gezielte Textsuche im selben Dokument geprüft. Die PDF-Version ist durch den Hash in Abschnitt 15 fixiert.

**[N2]** `en_30039201v010601p.pdf`, ETSI EN 300 392-1 V1.6.1 (2020-04): 7.7.7–7.7.8/S. 37–38, unadressierte Broadcast-MAC-PDUs und reservierte Broadcast-SSI. Weitere bereitgestellte Quellen und ihre Auswertungsgrenzen stehen im Inventar.

### 16.3 Repository-Quellen der zusätzlichen heutigen Prüfung

- **[S1]** [BlueStation-Repository und README](https://github.com/MidnightBlueLabs/tetra-bluestation): Alpha-Einordnung; die README ist keine vollständige Fähigkeitsprüfung.
- **[S2]** [Upstream-SNDCP-Stub am geprüften Commit](https://github.com/MidnightBlueLabs/tetra-bluestation/blob/09d4e0d9a0b8cf6c881e77353db325df9a4715aa/crates/tetra-entities/src/sndcp/sndcp_bs.rs).
- **[S3]** [Upstream MLE Protocol Discriminator](https://github.com/MidnightBlueLabs/tetra-bluestation/blob/09d4e0d9a0b8cf6c881e77353db325df9a4715aa/crates/tetra-pdus/src/mle/enums/mle_protocol_discriminator.rs).
- **[S4]** [Upstream MLE Downlink PDU Types](https://github.com/MidnightBlueLabs/tetra-bluestation/blob/09d4e0d9a0b8cf6c881e77353db325df9a4715aa/crates/tetra-pdus/src/mle/enums/mle_pdu_type_dl.rs).
- **[S5]** [Upstream Netzwerkzeit-Broadcast](https://github.com/MidnightBlueLabs/tetra-bluestation/blob/09d4e0d9a0b8cf6c881e77353db325df9a4715aa/crates/tetra-entities/src/mle/components/broadcast.rs).
- **[S6]** [Upstream MLE: Routing und Broadcast-Timing](https://github.com/MidnightBlueLabs/tetra-bluestation/blob/09d4e0d9a0b8cf6c881e77353db325df9a4715aa/crates/tetra-entities/src/mle/mle_bs.rs), gelesene Bereiche 1–200 und 280–Dateiende.
- **[S8]** [NetCore-Konfigurationsprädikat im ermittelten `main`](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/crates/tetra-config/src/bluestation/sec_cell.rs) und [UMAC-Verwendung](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/crates/tetra-entities/src/umac/umac_bs.rs); gezielte Codesuchtreffer, kein Vollread dieser beiden `main`-Dateien.
- **[S9]** [Vergleich des ermittelten `main` mit dem geprüften Archiv-Ausgangscommit](https://github.com/JanHG98/netcore-tetra/compare/7137e0dd69877e1b604bf89148fd8b6b590c1a97...68d41aa4a039563ccc3feca6b486b5cd91f75998).
- **[S10]** [Upstream-Release v0.5.10-09d4e0d](https://github.com/MidnightBlueLabs/tetra-bluestation/releases/tag/v0.5.10-09d4e0d), Release-Metadaten am 04.10.2026 abgefragt.
- **[S11]** [NetCore SNDCP-Basisstation am Archiv-Ausgangscommit](https://github.com/JanHG98/netcore-tetra/blob/68d41aa4a039563ccc3feca6b486b5cd91f75998/crates/tetra-entities/src/sndcp/sndcp_bs.rs).
- **[S12]** [NetCore Packet Gateway am Archiv-Ausgangscommit](https://github.com/JanHG98/netcore-tetra/blob/68d41aa4a039563ccc3feca6b486b5cd91f75998/crates/tetra-entities/src/sndcp/packet_gateway.rs).
- **[S13]** [NetCore WAP/IP-Adapter am Archiv-Ausgangscommit](https://github.com/JanHG98/netcore-tetra/blob/68d41aa4a039563ccc3feca6b486b5cd91f75998/crates/tetra-entities/src/sndcp/wap_ip.rs).
- **[S14]** [Ältere SNDCP-Profildokumentation im geprüften Baum](https://github.com/JanHG98/netcore-tetra/blob/68d41aa4a039563ccc3feca6b486b5cd91f75998/Docs/SNDCP_COMPLETE.md).
- **[S15]** [Separates Archiv des späteren WAP-/IP-/Multi-PDCH-Chats](2026-10-03_wap-sndcp-ip-gateway-multi-pdch-und-control-room.md).
- **[S16]** [NetCore WAP-Konfiguration und Defaults am Archiv-Ausgangscommit](https://github.com/JanHG98/netcore-tetra/blob/68d41aa4a039563ccc3feca6b486b5cd91f75998/crates/tetra-config/src/bluestation/sec_cell.rs), Zeilen 1–200 gelesen.

Die früheren Antworten verlinkten zusätzlich allgemeine TETRA-Informationen bei Wikipedia, eine Motorola-/PEI-Seite bei hamtetra und das Upstream-`Cargo.toml`. Diese damaligen Verweise sind kein Beleg für konkrete Endgerätekompatibilität oder den damals installierten Code. Die heutige technische Korrektur stützt sich auf die bereitgestellten ETSI-Quellen und die bezeichneten Primärdateien.

### 16.4 Noch fehlende Informationen für eine belastbare Fortsetzung

Es fehlen weiterhin der ursprüngliche Chattitel und Chatlink, ein vollständiger datierter Chat-Export, die historische und aktuelle installierte Binärversion, die aktive Konfiguration, konkrete Endgerätedaten, vollständige Uplink-/Downlink-Traces, Codex-Ausführungsprotokolle und reale Interop-/Dauerlastnachweise. Der vollständige Inhalt aller 8.061 PDF-Dateiseiten wurde nicht ausgewertet.

Der nächste Entwicklungsauftrag sollte deshalb **nicht** „alles neu bauen, weil `packet_data_flag` falsch ist“ lauten. Er sollte vom vorhandenen NetCore-Stand ausgehen, den fehlenden realen Dienstnachweis systematisch lokalisieren und die tatsächlich verbleibenden Lücken bis zur Mehrgeräteabnahme schließen.

---

**Abschluss des technischen Inhalts:** Historisches Ergebnis = Ziel und Agentenaufträge plus negativer Betreiberbefund, keine nachgewiesene Reparatur. Zusätzlicher heutiger Befund = korrigierte Broadcastdiagnose und bereits vorhandener NetCore-Paketdatenquellcode. Offenes Endziel = nachvollziehbar getesteter, stabiler WAP/IP-Betrieb mit mehreren realen TETRA-Endgeräten.
