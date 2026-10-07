# Brainstorming: WAP, SNDCP, IPv4-Gateway, Multi-PDCH und Control Room

**Stand der Notizen und ergänzenden Prüfungen: 2026-10-03.** Historische Entwürfe, nachgewiesene Umsetzung und ausgeführte Tests sind jeweils getrennt gekennzeichnet.

**Arbeitsrichtung:** WAP/SNDCP zu einem allgemeinen IPv4-Paketdatenpfad ausbauen. Letzter ausgewählter Umfang sind Multi-PDCH, Paketdaten-Dashboard/Control Room und Legacy-WAP über SDS. R1-Compilerreparatur ist bestätigt; Endgeräte- und Parallelbetriebsabnahme stehen aus.

## 1. Kontext und Nachweisstufen

| Feld | Wert |
|---|---|
| Projekt | NetCore-Tetra |
| Repository | `JanHG98/netcore-tetra` |
| Thema dieser Planung | Vergleich mit Nexus-BS/BlueStation/FlowStation; Ausbau von WAP über SNDCP zu allgemeinem IPv4-Paketdatenbetrieb; R1-Compilerkorrektur; dynamischer Multi-PDCH-Pool, Paketdaten-Dashboard, Control-Room-Anbindung und Legacy-WAP über SDS Type 4 |
| Erstellungsdatum dieser Zusammenfassung | **2026-10-03** |
| Zu Beginn gelesener Archivbranch | `fd6c77b1eb4e1c656daf40ac1d4b406a2ca41249` |
| Zugehöriger Archiv-Tree | `f87070a3d4db3345d846324d11455ee7b5ae81f2` |
| Zusätzlich geprüfter `main` | **`7137e0dd69877e1b604bf89148fd8b6b590c1a97`** |
| Zugehöriger `main`-Tree | `e68558c4df13d3d8b56df8c3611ac682b02889c1` |
| Letzter konkret bestätigter Betreiber-Erfolg im Entwurf | Nach dem R1-Compile-Fix: „okay er lief jetzt fehlerfrei durch.“ Das bestätigt einen erfolgreichen Compiler-/Buildlauf, nicht sämtliche Tests oder On-Air-Funktion. |
| Letztes im Entwurf ausgeliefertes Featurepaket | `netcore-tetra-multi-pdch-dashboard-legacy-wap-2026-07-21.zip` |
| Betriebsfreigabe | **Keine allgemeine Betriebs-, Interoperabilitäts- oder ETSI-Konformitätsfreigabe aus diesem Planungsstand ableitbar.** |

**Datierung:** Die historischen Pakete und Begleitdateien tragen durchgehend `2026-07-21`. Einzelne Nachrichten haben im hier verfügbaren Verlauf keinen belastbaren Zeitstempel. Die Dateinamen werden unverändert als Artefaktbezeichnungen übernommen; sie beweisen nicht den Versandzeitpunkt aller Nachrichten. Die Archivdatei verwendet ausdrücklich das tatsächliche Erstellungsdatum 2026-10-03.

### 1.1 Statusbegriffe

- **Idee:** diskutierte Möglichkeit, ohne konkrete Freigabe zur Umsetzung.
- **Beschlossen/geplant:** gewünschter oder ausdrücklich ausgewählter Umfang. Das ist noch kein Code- oder Betriebsnachweis.
- **Implementiert:** die betreffende Logik ist in einem bezeichneten ZIP oder einem konkret geprüften Repository-Commit vorhanden. Das Wort wird hier nicht mit erfolgreicher Endgerätefunktion gleichgesetzt.
- **Getestet:** Für jede Prüfung sind Gegenstand und Ergebnis benannt. Historische Angaben aus Entwicklungsberichten und zusätzlich ausgeführte Prüfungen bleiben getrennt.
- **Im Betrieb bestätigt:** tatsächliche, vom Betreiber bestätigte Funktion des betreffenden Dienstes oder Funkpfads. Für den neuen WAP-/IP-/Multi-PDCH-Funkpfad fehlt eine solche Bestätigung im zugänglichen Verlauf.

Ein erfolgreicher Build, ein Unit-Test, eine plausible PDU und ein auf einem realen Terminal bestätigter Datentransfer sind vier unterschiedliche Nachweisstufen.

### 1.2 Quellen- und Auswertungsgrenzen

Ausgewertet wurden der zugängliche Planungsverlauf, die konkret benannten und im Arbeitsbereich vorhandenen ZIP-/Patch-/Markdown-/Prüfsummen-Anhänge sowie gezielte aktuelle Repository-Lesezugriffe. Die letzte Featureauslieferung wurde zusätzlich lokal entpackt und in zentralen Dateien gelesen. Alle acht vorhandenen ZIP-Pakete wurden erneut auf CRC-/Archivintegrität geprüft; die letzte Patch-Anwendung wurde erneut ausgeführt.

Die umfangreichen ETSI-Anhänge wurden **nicht vollständig Seite für Seite auditiert**. Für die Einordnung wurden insbesondere die überlieferten SNDCP-/SDS-Referenzen und die WAP-Abgrenzung in EN 300 392-2, Abschnitt 29.5.8, herangezogen. Die 4.100-seitige Datei `ETSI.pdf` ist eine Zusammenstellung, kein zusätzliches unabhängiges Konformitätszeugnis.

**Fehlende Nachweise:** Vollständige historische Build-/Test-Rohlogs, installierte TBS-Binärdatei samt Hash, aktive Konfiguration, RF-Mitschnitte und Endgerätetest der letzten Erweiterungsstufe. Gleichnamig ersetzte Dateien waren am 2026-10-03 nur in der letzten verfügbaren Fassung lesbar; ältere Versionen sind daraus nicht rekonstruierbar.

Frühere Bezeichnungen wie „komplett“, „vollständig“ und „fertig“ sind keine Abnahmekriterien. Maßgeblich sind konkrete Codezweige, begrenzte Profile und vorhandene Testnachweise.

## 2. Ergebnis in komprimierter Form

Die ZIP-basierten Ausbaustufen führen vom lokalen WAP-Statusdienst zu einem umfangreicheren Paketdatenpfad: SNDCP-/PDP-Kontexte und IPv4/UDP-WAP, zusätzliche SNDCP-PDUs, Zustände und Timer, schließlich Linux-TUN-Gateway mit Routing/NAT und Fragmentbehandlung. Ein Compilerabbruch erforderte den R1-Fix. **Nur für diesen R1-Ausgangsstand wurde anschließend ein fehlerfreier Buildlauf bestätigt.**

Danach wurden drei Komponenten ausgewählt und gemeinsam ausgeliefert: dynamischer Pool mehrerer Ein-Slot-PDCHs, Paketdatenanzeige im Basisstations-Dashboard und Control Room sowie Legacy-WAP-SDS-Generator. Ihr Code ist auch im Repository-Prüfstand vom 2026-10-03 vorhanden. Ein anschließender Build- oder Betriebsnachweis dieser letzten Erweiterung fehlt.

Der ergänzende Repository-Stand ist nicht mehr identisch mit den historischen Paketen: Unter anderem existieren inzwischen eine Packet-Core-Control-Anbindung und ausgelagerte Dashboard-Assets. Deshalb darf ein historisches Komplett-ZIP nicht als pauschales Update über den geprüften Baum kopiert werden.

**Wichtigster neuer Archivbefund:** Die damalige Aussage, der letzte Patch und das letzte Komplett-ZIP ergäben exakt denselben Baum, ist in dieser Allgemeinheit nicht reproduzierbar. Alle 660 Dateien des Ergebnis-ZIPs stimmen byteweise mit dem erneut gepatchten Baum überein; dort bleiben aber vier zusätzliche Wiki-Dateien mit Unicode-Bindestrich erhalten. Details stehen in Abschnitt 14.

## 3. Ausgangslage und ursprünglicher Auftrag

Vergleichsquellen für das eigene Repository:

| Quelle | Rolle im Entwurf |
|---|---|
| `JanHG98/netcore-tetra` | Eigenes Zielprojekt; zunächst hochgeladener Arbeitsstand als Änderungsbasis |
| `invictus737/nexus-bs` | Hauptvergleich, besonders WAP/SNDCP, Betriebsstabilität und weitere mögliche Portierungen |
| `MidnightBlueLabs/tetra-bluestation` | Gemeinsame Abstammung beziehungsweise schlankerer Protokollunterbau |
| `misadeks/flowstation` | Vergleich von Ruf-, SDS-, Dashboard- und Integrationsfunktionen |
| `ea5gvk/flowstation` | Zusätzlich betrachteter FlowStation-Zweig |

WAP war unmittelbar als Ergänzung vorgesehen; weitere Funktionen sollten einzeln entschieden werden. Der freigegebene Umfang wurde anschließend auf SNDCP, allgemeine Paketdaten und die drei zuletzt ausgewählten Komponenten erweitert.

Der erste Arbeitsstand ist `netcore-tetra-wap.zip`. Der weitergeführte Stand `netcore-tetra-wap(1).zip` bildet die konkrete Grundlage für Multi-PDCH, Dashboard und Legacy-WAP. Historische Vergleiche müssen diese Eingangsartefakte verwenden statt einen nachträglich angenommenen `main`-Stand.

### 3.1 Aussagekraft des historischen Repository-Vergleichs

Die damaligen Quellen zeigten bei Nexus einen opt-in terminalseitigen WAP/IP-Pfad über SNDCP sowie einen davon getrennten Legacy-SDS-Pfad. BlueStation wurde als Alpha-Basis mit Registrierung, Gruppenaufschaltung, partieller Sprache und optionalem Brew beschrieben. Die beiden betrachteten FlowStation-READMEs hatten beim Abruf denselben Blobinhalt und listeten unter anderem Gruppen-/Einzelrufe, SDS, HMD, DGNA, Dashboard, Notruf-Präemption und Integrationen auf.

Der Vergleich ist keine vollständige Funktions- oder Qualitätsabnahme aller Zweige. Identische READMEs belegen keine identischen Codebäume; breite Einordnungen wie „alles bereits vorhanden“ sind nicht für jede Funktion nachgewiesen. Ein erneuter vollständiger Upstreamvergleich wurde am Prüfdatum nicht durchgeführt.

### 3.2 Lizenz- und Provenienzentscheidung

Die eingesehenen Nexus-BS-Dateien waren gemischt lizenziert: historische Apache-2.0-Anteile und eigene Änderungen/Ergänzungen unter PolyForm Noncommercial. Laut Entwicklungsbeschreibung wurde die NetCore-Erweiterung deshalb eigenständig anhand von Port-Vertrag, Protokollfeldern und Testvektoren erstellt; eine unabhängige Provenienzprüfung ist damit nicht ersetzt.

**Status:** dokumentierte Vorgehensabsicht beziehungsweise historische Herkunftsbehauptung. Eine unabhängige Clean-room- oder Lizenzprüfung ist damit nicht erbracht; im Verlauf wurden auch Nexus-Quelldateien gelesen. Vor einer entsprechenden verbindlichen Aussage oder Weiterverteilung ist eine gesonderte Provenienzprüfung erforderlich. Lizenzhinweise und Credits vorhandener Dateien dürfen nicht entfernt werden. Diese Archivierung ändert keine Lizenz und kopiert keinen Upstreamcode in den Produktivbaum.

## 4. Chronologie und endgültige Entscheidungen

| Stufe | Auslöser und Entscheidung | Auslieferung beziehungsweise Ergebnis | Nachweisgrenze |
|---|---|---|---|
| A: WAP | WAP ergänzen; weitere Funktionen einzeln entscheiden | WAP-Cleanroom-ZIP und Patch; lokaler Browserpfad über SNDCP/IPv4/UDP/WTP/WSP; fester Hauptcarrier-TS2 | Syntax-/Vektorprüfungen laut Entwicklungsbericht, Rust-Build und RF-Test nicht belegt |
| B: SNDCP | Möglichst vollständiger SNDCP-Ausbau ausgewählt | Weitere PDU-Familien, primäre/sekundäre Kontexte, SNEI, QoS-/IE-Codecs, Timer, Paging/Reconnect, Cache und Lebenszyklusbereinigung | Begrenzt auf IPv4-/Ein-Slot-Profil; nicht sämtliche optionalen SNDCP-/Funkfunktionen |
| C: allgemeine IP-Paketdaten | Router/TUN, Routing, Fragmentierung und NAT ergänzen | IPv4-Gateway, Kernel-Routing/NAT, PCO-/DNS-Behandlung, Downlinkpufferung und Betriebshelfer | Vollständiger Build nicht belegt; anschließend reale Compilerfehler |
| D: R1 | Fünf Rust-Diagnosen aus dem Zielbuild | Konstanten, Fehlerenum, SNEI-Helper und Result-Typ korrigiert; R1-ZIP/Patch | Fehlerfreier Lauf bestätigt; genauer Befehl und vollständiges Log fehlen |
| E: weitere Planung | Fehlende Funktionen ermitteln | Kandidatenliste; Tests, Multi-PDCH und Paketdatenanzeige priorisieren | Keine pauschale Freigabe sämtlicher Themen |
| F: letzte drei Features | Multi-PDCH, Dashboard/Control Room und Legacy-WAP auf weitergeführtem ZIP-Stand | Gemeinsames ZIP und Patch gegen `netcore-tetra-wap(1).zip` | Anschließender Build-/Betriebsnachweis fehlt |
| G: diese Archivierung | Technischen Verlauf sichern, geprüften Stand getrennt prüfen | Archivdokument und Index ausschließlich im Branch `Archiving` | Kein neues Featuredeployment, kein Merge, keine Änderungen an Betriebssystem/Firewall/RF |

### 4.1 Endgültiger Umfang des letzten Featurepakets

**Beschlossen und in den Quellen vorhanden:** mehrere dynamisch vergebene PDCH-Bearer, Paketdaten-Telemetrie und Benutzeroberflächen, SDS-Type-4-Legacy-WAP-Generator.

**Nicht Teil des abschließend nachgewiesenen Umfangs:** gebündelte Multislot-Datenübertragung für ein einzelnes Funkgerät, TEDS/QAM, ein vollständiger fairer Airtime-Scheduler, harte Verdrängung laufender Datenbearer durch Notrufe, allgemeiner WAP-Push-Server mit nachgewiesener Endgeräteinteroperabilität, vollständige kryptographische Netzauthentifizierung oder kommerziell belastbare Konformität.

Die vorgeschlagene aggressive Präemption wurde in der gelieferten Stufe durch **Admission Control mit reserviertem Sprach-Headroom** ersetzt. Implementiert ist „ein PDCH pro ISSI“ als Mehrteilnehmerlösung; mehrere Slots pro ISSI bleiben eine Erweiterungsidee.

## 5. Architektur und Datenwege

### 5.1 Zuständigkeiten

| Ebene/Komponente | Aufgabe | Wichtige Abgrenzung |
|---|---|---|
| PHY/LMAC/UMAC | Funk, Burst-/Kanalverarbeitung, tatsächliche Slot- und Carrierführung | Eine SNDCP-Kontexttabelle allein implementiert diese Funkfunktionen nicht |
| Gemeinsamer `TimeslotAllocator` | Ressourcenbesitz für `Brew`, `Cmce` und `Sndcp`; Belegung und Freigabe | Lokale Ressourcenexklusivität ist nicht gleich Ende-zu-Ende-Funktionsnachweis |
| LLC/MLE/LTPD | Signalisierungs- und Paketdatentransport zwischen Funksteuerung und SNDCP | Acknowledged/Unacknowledged-Service und Carrier-Routen müssen tatsächlich zusammenpassen |
| SNDCP-Entity | PDP-/NSAPI-Zustände, Protokoll-PDUs, Timer, Paging, Bearerbindung, Auswahl des lokalen WAP- oder IP-Pfades | Unterstützte Profile begrenzen den nutzbaren Dienst |
| Lokaler WAP/IP-Dienst | Kleine Statusressourcen mit WSP/WTP über UDP; WML/XHTML | Kein allgemeiner HTTP-/TCP-Proxy und keine bewiesene vollständige WAP-Suite |
| Lokales Packet Gateway | TUN-Transport und Verwaltung optionaler Host-Routen, Weiterleitung und NAT-Regeln | TCP/UDP/ICMP/Routing/NAT werden wesentlich vom Linux-Kernel übernommen |
| Telemetrie/Control Room | Aggregierte Sicht auf Gateways, Kontexte und Bearer; Bedien- und Diagnoseoberflächen | Snapshot oder „queued“ bestätigt keinen Funkempfang |
| Legacy-WAP-Modul | WML erzeugen, Größenprüfung, PID-/SDS-TL-Hülle und Einspeisung in vorhandenen Raw-SDS-Pfad | Vom SNDCP-WAP-Browserpfad unabhängig |

### 5.2 Uplink des allgemeinen IPv4-Pfades

```text
MS / TETRA-Endgerät
  -> MAC / LLC / MLE-Paketdatenpfad
  -> SN-DATA oder SN-UNITDATA
  -> ISSI + NSAPI + aktiver PDP-Kontext
  -> Quelladress-, Längen-, MTU- und IPv4-Prüfungen
  -> gegebenenfalls Fragment-Reassembly
  -> lokaler WAP-Endpunkt ODER TUN ntetra0
  -> Linux-IP-Stack
  -> lokaler Dienst / geroutetes Netz / optional NAT-Uplink
```

### 5.3 Downlink und ruhender Teilnehmer

```text
Linux-IP-Stack / Route auf ntetra0
  -> Ziel-IPv4 -> Teilnehmer und Kontextfamilie
  -> NSAPI-Auswahl anhand Flow-/Port-/DiffServ-Regeln oder Primärkontext
  -> READY: Daten senden
     STANDBY: begrenzt puffern, SN-PAGE anstoßen
  -> nach erfolgreichem Beareraufbau Warteschlange bedienen
  -> IPv4-Fragmentierung entsprechend ausgehandelter MTU, sofern zulässig
  -> SN-UNITDATA / unterer Paketdatentransport
  -> zugeteilter Carrier und Timeslot
  -> MS
```

Die Existenz von Paging-, Queue- und Sendezweigen beweist nicht, dass ein konkretes Terminal alle Übergänge unterstützt. Insbesondere sind Queueannahme, LLC-Bestätigung, SDS-Zustellbericht und Darstellung einer WAP-Seite unterschiedliche Ereignisse.

### 5.4 Steuer- und Telemetriepfad

```text
SNDCP-Zustand / PacketGateway-Zähler
  -> TelemetryEvent mit Gateway-, Kontext- und Bearerwerten
  -> lokales Dashboard / letzter Snapshot / WebSocket
  -> Node-/Control-Room-Verbindung
  -> Control-Room-State und HTTP-API
  -> Operator-CLI und native UI

Legacy-WAP-Formular oder API
  -> Eingabevalidierung und vorhandene Autorisierungsprüfung
  -> WML- und SDS-Type-4-Payload
  -> vorhandener Raw-SDS-/Control-Pfad
  -> Ziel-ISSI
```

Der konkrete Authentifizierungsmodus muss jeweils aus dem tatsächlich eingesetzten Dienst gelesen werden. Ein mitgesendetes Feld `operator_id` ist für sich genommen kein Nachweis der Identität des HTTP-Aufrufers.

## 6. SNDCP- und WAP/IP-Ausbau

### 6.1 Protokollfamilien

Die SNDCP-Ausbaustufe ergänzte einen Katalog der Haupt-PDU-Typen 0–13 einschließlich der im Code behandelten Untertypen:

| Familie | Im historischen Entwurf beschriebene Formen |
|---|---|
| ACTIVATE PDP CONTEXT | Demand, Accept und Reject; statische/dynamische IPv4-Adresse und sekundärer Kontext |
| DEACTIVATE PDP CONTEXT | Demand und Accept; einzelner NSAPI oder gesamte Teilnehmerkontextfamilie |
| Nutzdaten | SN-UNITDATA und SN-DATA mit NSAPI/PCOMP/DCOMP/N-PDU |
| DATA TRANSMIT | Request und Response einschließlich passender Ressourcenablehnungen |
| END OF DATA | Bearer beenden beziehungsweise in STANDBY überführen |
| RECONNECT | Wiederaufnahme und Ressourcenanforderung |
| PAGE | Request/Response zum erneuten Aktivieren eines ruhenden Teilnehmers |
| NOT SUPPORTED | Behandlung nicht unterstützter Protokolltypen |
| DATA PRIORITY | Acknowledgement, Information und Request |
| MODIFY | Request, Accepted/Rejected Response, Availability und Usage |

**Einordnung:** Codec-/Dispatch-Abdeckung ist von vollständigen Prozeduren, Timern, Wiederholungen und interoperabler Funkübertragung zu unterscheiden. Die damalige Formulierung „wire-vollständig“ wurde nicht durch eine vollständige unabhängige Bit-/Prozedurprüfung aller Profile untermauert.

### 6.2 Kontextmodell

Vorgesehen und im gelieferten Quellbaum vorhanden sind NSAPI 1–14, mehrere primäre und sekundäre Kontexte pro ISSI, gemeinsamer IPv4-Besitz einer Primär-/Sekundärfamilie, SNEI-Verwaltung, Grenzen je Teilnehmer und insgesamt sowie statische beziehungsweise dynamische Adressvergabe mit Kollisionsprüfung. Die strikte Quelladressprüfung soll Nutzdaten an die zugeteilte Teilnehmeradresse binden.

Als fortsetzungsrelevante Invarianten sind zu erhalten:

1. Ein sekundärer Kontext darf nicht unabhängig vom Primärkontext eine widersprechende IP-Adresse erhalten.
2. Reaktivierung oder Entfernen eines Primärkontexts muss seine abhängigen Kontexte berücksichtigen.
3. Ein Funk-Bearer gehört in der letzten Stufe einer ISSI; mehrere NSAPIs dieser ISSI teilen ihn.
4. Ressourcen-, Kontext-, Routen- und Cachebereinigung müssen denselben Teilnehmerlebenszyklus verfolgen.
5. Ein aus der Tabelle entfernter Teilnehmer darf keinen Timeslot unbemerkt blockieren oder alte Antworten unter neuer Kontextidentität erhalten.

### 6.3 Zustände, Timer und Bereinigung

Die historischen Stufen beschrieben READY/STANDBY, CONTEXT_READY pro NSAPI sowie Verfügbarkeits-/Nutzungszustände wie suspended/quiescent. Das anfängliche Problem, dass ein nicht ordnungsgemäß beendeter TS2 bis Registrierung oder Neustart blockiert bleiben konnte, sollte durch Timer und Lebenszyklusereignisse abgelöst werden.

| Ereignis | Beabsichtigtes Verhalten der späteren Stufen |
|---|---|
| READY-Ablauf | SN-END OF DATA anstoßen, betroffene Kontexte nach STANDBY, Bearer freigeben |
| STANDBY-Ablauf | Teilnehmerkontexte beziehungsweise Kontextfamilie entfernen |
| END OF DATA | Nur den zu diesem Teilnehmer gehörenden Bearer freigeben |
| PDP-Deaktivierung | Gewünschten Kontext und gegebenenfalls abhängige Kontexte korrekt entfernen |
| MM-Deregistrierung / Kick / T351-Drop | PDP/SNEI/Routen/Cache und PDCH-Ressource bereinigen |
| Daten für STANDBY-Teilnehmer | Downlink begrenzt speichern, Paging auslösen, nicht grenzenlos puffern |

Der Standardbezug für READY/STANDBY wurde im Entwurf insbesondere mit EN 300 392-2, Kapitel 28, hergestellt. Er ersetzt keine Prüfung aller realen Nachrichtenfolgen. Für die letzte Erweiterung fehlt hier ein On-Air-Nachweis für jeden genannten Abbruch- und Übergangspfad.

### 6.4 Wiederholungsbehandlung

Der Response-Cache wurde mit **30 Sekunden Lebensdauer und höchstens 256 Einträgen** beschrieben und ist im geprüften SNDCP-Code mit entsprechenden Konstanten sichtbar. Er soll identische wiederholte Kontrollanforderungen nicht mehrfach zustandsverändernd ausführen, etwa bei ACTIVATE oder TRANSMIT.

Dieser Cache ist **nicht automatisch** ein vollständiger WTP-Session-/Retransmission-Cache, ein LLC-ARQ-Nachweis oder eine garantierte Ende-zu-Ende-Deduplizierung. Diese Ebenen müssen getrennt getestet werden.

### 6.5 QoS und optionale IEs

Beschrieben wurden symmetrische/asymmetrische QoS-Strukturen, Background Data Class, CONTEXT_READY-Timer, Minimum Peak Throughput, Mean/Mean Active Throughput, Delay- und Reliability-Class, automatische und spezifizierte Filter, Ports/Portbereiche/DiffServ, Scheduled Access sowie Type-2-/Type-3-/Type-4-Optional-IEs und rohe unbekannte Bitfolgen.

Der Codec kann einen Wunsch verstehen und trotzdem dessen Nutzung ablehnen. Insbesondere bedeutet ein decodierter Multislot- oder Scheduled-Access-Request nicht, dass der MAC diese Betriebsart bereitstellt. Akzeptierte QoS-Parameter und tatsächlich durchgesetzte Airtime-/Durchsatzpolitik sind separat zu belegen.

### 6.6 Lokaler WAP/IP-Dienst

Der Ausbau umfasste IPv4/UDP, WTP Invoke/Result/ACK/Abort und WSP Connect/Resume/GET, absolute und relative URLs sowie lokale XHTML-/WML-Statusantworten. Der Dienst ist als opt-in Statusendpunkt konzipiert. Die historischen ausgelieferten Betreiber-TOMLs aktivierten ihn jedoch bereits.

Historische WAP-Profilwerte:

```text
Gateway-Adresse:  10.0.0.1
Gateway-Port:     9200 / UDP
Statusressourcen: /, /status.xhtml, /status.wml
Weitere Pfade:   /status entsprechend Policy/Implementierung prüfen
Homepage-Beispiel: http://10.0.0.1:9200/
```

Die frühere pauschale Profilbezeichnung „connectionless“ bei gleichzeitig vorhandenem WTP/WSP-Connect ist **keine universell verifizierte Codeplug-Anweisung**. Transportmodus, Port und Browsereinstellungen müssen pro Terminal/Firmware zum real unterstützten WTP-/WSP-Verfahren passen.

`max_request_payload_bytes = 1024` ist eine Eingabegrenze des Dienstes und kein Beweis, dass über den Funkpfad jedes 1.024-Byte-WAP-Paket unter der ausgehandelten MTU transportiert wird. Beim 576-Byte-N-PDU verbleiben ohne IPv4-Optionen nach IPv4-/UDP-Headern 548 Byte; WTP/WSP benötigen daraus zusätzlich Platz.

### 6.7 Bewusst begrenzte Profile

Als nicht vollständig bereitgestellt wurden genannt: IPv6-PDP, Mobile IPv4, RFC-1144/VJ- und RFC-2507-Headerkompression, DCOMP, Paketdaten-AIE, Enhanced-/Multislot-PDCH und Scheduled Access. Nicht unterstützte Anforderungen sollten mit passenden Ursachen abgewiesen werden, statt Leistungsfähigkeit vorzutäuschen. „Alle PDUs vorhanden“ und „alle optionalen Dienste verfügbar“ dürfen nicht gleichgesetzt werden.

Die Motorola-/Dimetra-CHAP-Success-Kompatibilitätsantwort wurde erhalten. Sie ist ausdrücklich **keine eigenständige Passwortprüfung und keine TETRA-Netzauthentifizierung**.

## 7. Allgemeines IPv4-Gateway und Hostintegration

### 7.1 Implementierungsaufteilung

Die allgemeine Paketdatenstufe ergänzte `sndcp/packet_gateway.rs`, `sndcp/fragment.rs` und die Anbindung im SNDCP-Entity. Ein Linux-TUN-Interface transportiert rohe IP-Pakete. Der Linux-IP-Stack übernimmt die allgemeinen Transport- und Routingfunktionen; NetCore muss dazu keinen vollständigen TCP-Stack nachbauen.

TAP/Ethernet/ARP waren für diesen konkreten SNDCP-Raw-IP-Pfad nicht vorgesehen. Die Adresszuweisung erfolgt über PDP-Aktivierung. DNS wurde über PPP-IPCP in Protocol Configuration Options mit den DNS-Optionen 129 und 131 ergänzt. **Kein DHCP-Dienst ist implementiert.** Die historische Aussage, DHCP setze grundsätzlich Ethernet voraus, wird hier nicht als allgemeine technische Regel übernommen; ausschlaggebend ist die gewählte Projektarchitektur.

### 7.2 Weiterleitung und NAT

Beschriebene Betriebsarten sind lokaler Zugriff, gerouteter Betrieb ohne NAT und Masquerading/NAPT über den Host. Conntrack und Port-/Adressübersetzung werden dem Kernel beziehungsweise der Host-Firewall überlassen. Mobile-zu-Mobile-Verkehr soll ebenfalls über diesen IP-Pfad geroutet werden können.

Die vorgesehenen eigenen Firewallobjekte heißen:

```text
nftables: table ip netcore_tetra
iptables: NETCORE_TETRA_FWD
iptables: NETCORE_TETRA_NAT
```

Die historischen Vorgaben erlauben ausgehenden Verkehr und zugehörigen Rückverkehr; `allow_unsolicited_inbound = false` soll neue ungefragte Weiterleitungen zum Teilnehmernetz begrenzen. **Das ist kein Nachweis einer vollständigen Host-Firewall oder einer Sperre aller Managementdienste im INPUT-Pfad.** Bestehende nftables-/iptables-Regeln, Chain-Reihenfolge, Routing und andere Dienste müssen gemeinsam betrachtet werden.

### 7.3 Fragmentierung und Reassembly

Im Paket beschrieben und als Implementierung vorhanden: IPv4-Version/IHL/Gesamtlänge/Prüfsumme, DF/MF/Fragmentoffset, Out-of-order-Reassembly, Behandlung identischer Duplikate, Ablehnung widersprüchlicher beziehungsweise überlappender Fragmente, Speicher- und Zeitlimits, Berücksichtigung kopierpflichtiger Optionen in Folgefragmenten und Neuberechnung der Headerprüfsumme.

| Grenze | Historischer Wert |
|---|---:|
| Unvollständige Reassembly-Datagramme | 128 |
| Gesamter Reassembly-Speicher | 4.194.304 Byte |
| Reassembly-Lebensdauer | 30 Sekunden |
| Downlinkpakete je Kontext | 64 |
| Downlinkbytes je Kontext | 262.144 Byte |
| Downlink-Pufferlebensdauer | 30 Sekunden |
| Paging-Wiederholungsintervall | 5 Sekunden |

Ein zu großes DF-Paket wird nicht einfach unerlaubt fragmentiert. **Ob jede Fehlerstelle ein korrektes ICMP-„Fragmentation needed“/PMTU-Verhalten bis zum Sender erzeugt, ist in diesem Planungsstand nicht Ende-zu-Ende nachgewiesen.** Das muss geprüft werden, statt aus TUN-MTU und lokalem Fehlerenum automatisch funktionierende PMTUD abzuleiten.

### 7.4 Auswahl sekundärer NSAPIs

Die beschriebene Reihenfolge ist: automatisch gelernte symmetrische Flows, statische Portfilter, Portbereiche, DiffServ-/DSCP-Regeln und anschließend Primärkontext als Fallback. Historische Limits: 300 Sekunden Lebensdauer automatischer Filterbindungen und maximal 4.096 Bindungen.

Zu prüfen bleiben unter anderem Filterkollisionen, Priorität konkurrierender Regeln, Fragmente ohne Transportheader, Antworten nach Kontextwechsel und abgelaufene Flowbindungen. Filtererkennung ist keine pauschale Erlaubnis für fremde Quelladressen.

### 7.5 systemd, Rechte und Bereinigung

Betriebshelfer im Paket:

```text
contrib/packet-data/netcore-tetra-packet-gateway-install
contrib/packet-data/netcore-tetra-packet-gateway-uninstall
contrib/packet-data/netcore-tetra-packet-gateway-status
contrib/packet-data/netcore-tetra-packet-gateway-cleanup
```

Vorgeschlagener Dienstname: `tetra.service`. Vorgesehene Runtime-Ablage: `/run/netcore-tetra/packet-gateway-sysctls`. Der Cleanup-Helfer wird nach Installationskonzept unter `/usr/local/libexec/netcore-tetra-packet-gateway-cleanup` verwendet. TUN ist nicht persistent; die Lebensdauer ist an den Dateideskriptor gebunden.

Der vorgeschlagene Drop-in setzt beziehungsweise ergänzt `AmbientCapabilities=CAP_NET_ADMIN`, `PrivateDevices=no`, `ProtectKernelTunables=no`, `RuntimeDirectory=netcore-tetra` und einen `ExecStopPost`-Cleanup. Ein vorhandenes restriktives `CapabilityBoundingSet` wurde nicht pauschal überschrieben; `CAP_NET_ADMIN` muss darin gegebenenfalls zusätzlich zulässig sein.

Gespeichert/wiederhergestellt werden nach dem beschriebenen Konzept unter anderem `net.ipv4.ip_forward`, TUN-bezogenes `rp_filter` und `send_redirects`. Wiederherstellung ist vom tatsächlichen Cleanup-Ablauf und zwischenzeitlichen Hoständerungen abhängig. „Crash-sicher“ aus der historischen Beschreibung ist **keine Garantie für Stromausfall, Hostabsturz oder beliebige parallele Firewallverwaltung**.

Die Hilfen und Fähigkeiten betreffen die Host-Sicherheit. Erfolgreiche Installation und Laufzeitfunktion sind nicht mit realen Hostlogs bestätigt.

## 8. Dynamischer Multi-PDCH-Pool

### 8.1 Ressourcenmodell

Die fixe Reservierung von Hauptcarrier-TS2 wird in der letzten Stufe durch die Suche nach einem freien Traffic-Slot im gemeinsamen Allocator ersetzt. Besitzer bleiben unterscheidbar: `TimeslotOwner::Brew`, `::Cmce`, `::Sndcp`.

| Logischer Slot | Physischer Träger | Air-Timeslot |
|---:|---|---:|
| 2 | Hauptcarrier | 2 |
| 3 | Hauptcarrier | 3 |
| 4 | Hauptcarrier | 4 |
| 5 | Sekundärcarrier | 2 |
| 6 | Sekundärcarrier | 3 |
| 7 | Sekundärcarrier | 4 |

TS1 beider Carrier ist außerhalb dieses Pools. **Acht angezeigte logische Funk-Timeslots bedeuten hier nicht acht frei vergebbare Paketdatenkanäle.** Die C2-TS1-Control-/Guard-Rolle darf bei späteren Portierungen nicht aus einer alten Traffic-only-Annahme überschrieben werden.

`air_ts(5..=7)` bildet mit `logical_ts - 3` auf TS2–TS4 ab. Der SNDCP-Code verwendet für die Sekundärroute den internen Hint `-2`; das ist ein Implementierungsmarker, kein auf der Luftschnittstelle zu sendender Carrierwert. Die reale Carrierzahl kommt aus der Konfiguration.

### 8.2 Vergabe und Grenzen

Mit `prefer_secondary_carrier=true` lautet die bevorzugte Reihenfolge im Dual-Carrier-Fall `[5, 6, 7, 2, 3, 4]`, andernfalls `[2, 3, 4, 5, 6, 7]`. Ein vorhandener Bearer derselben ISSI wird wiederverwendet; neue Teilnehmer erhalten nur freie Ressourcen.

Sei `C` die Traffic-Kapazität (3 oder 6), `R` der konfigurierte Sprach-Headroom und `M` die maximale PDCH-Zahl:

```text
Poolgrenze = C - min(R, C)
M = 0:   zulässige PDCH-Zahl = Poolgrenze
M > 0:   zulässige PDCH-Zahl = min(M, Poolgrenze)

Zusätzliche Bedingung für eine neue Vergabe:
aktuell freie Traffic-Slots > min(R, C)
```

Bei `R=1`, `M=0` ergeben sich **höchstens zwei PDCHs bei Single Carrier und fünf bei Dual Carrier**. Bereits laufende Sprach-/Brew-Ressourcen können die aktuell nutzbare Anzahl weiter reduzieren. Ein Wert `max_pdch_bearers` ist daher kein garantierter gleichzeitiger Durchsatz oder eine zugesicherte Zahl verbundener MS.

### 8.3 Freigabe

Zu jedem Bearer werden ISSI, logischer Slot sowie Zeitinformationen geführt. Die Freigabe adressiert den konkret zugewiesenen Slot mit Besitzer `Sndcp`; Timer, END, Deaktivierung und Deregistrierung dürfen nicht pauschal TS2 oder einen anderen Teilnehmer freigeben.

Am Prüfstand nachgewiesene Funktionsnamen sind `traffic_slot_capacity`, `pdch_capacity`, `air_ts`, `carrier_hint`, `carrier_num`, `pdch_allocation`, `quit_allocation`, `reserve_pdch`, `touch_pdch`, `pdch_for_issi` und `release_pdch`. Der Allocator bietet `allocate_preferred_with_capacity` und `free_count_with_capacity`.

### 8.4 Was dieser Pool nicht leistet

Kein harter Notruf-Preempt eines bestehenden Datenbearers ist durch die gelieferte Headroom-Lösung implementiert. Der Guard begrenzt neue Datenzulassungen; er kann weder unendlich viele Notrufe ermöglichen noch einen bereits durch Sprache belegten letzten Slot freizaubern.

Ebenso wenig folgt aus „dynamisch“ bereits gewichtete Fairness, zeitliche Rotation zwischen mehr wartenden ISSIs als Slots, garantierte Dienstgüte oder Mehrslot-Bündelung für ein MS. Diese weitergehenden Punkte bleiben Kandidaten und brauchen eigene Ablauf- und Lasttests.

## 9. Paketdaten-Dashboard und Control Room

### 9.1 Datenmodell und Anzeige

Die letzte Erweiterung stellte Gateway-, PDP- und Bearer-Snapshots bereit. Im geprüften Repository sind unter anderem `PacketDataGatewayTelemetry`, `PacketDataContextTelemetry`, `PdchBearerTelemetry` und der lokale `PacketDataDashboardSnapshot` mit `last_packet_data` vorhanden.

| Ansicht | Vorgesehene Inhalte |
|---|---|
| Gateway | Aktivität/Verfügbarkeit, Interface, IPv4/Prefix, Paket-/Bytezähler UL/DL, Drops, I/O-Fehler und Warteschlangenzustand |
| Kontext | ISSI, NSAPI, SNEI, IPv4, PDP-Zustand, MTU und Priorität |
| Bearer | ISSI, zugehörige NSAPIs, Carrier, Air-TS und logischer Slot |
| Kapazität | Aktive Bearer, Poolkapazität, freie Traffic-Slots, reservierter Sprach-Headroom |
| Node-Übersicht | Gatewaystatus sowie Zahl der Kontexte/Bearer und Kapazität je TBS |

Das sind Laufzeitdaten beziehungsweise letzte Snapshots. Es ist daraus weder dauerhaftes Accounting noch vollständig implementierte Anzeige aller früher vorgeschlagenen Werte wie Conntrack-Auslastung, jede individuelle Dropursache oder jede Pagingstatistik abzuleiten.

### 9.2 Schnittstellen

| Ebene | Schnittstelle | Historischer beziehungsweise am 2026-10-03 belegter Umfang |
|---|---|---|
| Basisstations-Dashboard | `GET /api/packet-data` | Lokaler Paketdaten-Snapshot; im finalen ZIP implementiert |
| Basisstations-WebSocket | Event `type=packet_data` | Gateway-, Kontext- und Bearerwerte |
| Control-Room-Core | `GET /api/packet-data` | Aggregation über Nodes; Route am 2026-10-03 im Code vorhanden |
| Control-Room-Core | `GET /api/nodes/{node_id}/packet-data` | Node-bezogene Sicht |
| Control-Room-Core | `POST /api/nodes/{node_id}/commands/legacy-wap` | WML/SDS-Type-4-Sendekommando |
| Operator-CLI | `packet-data [--node …]` | Abfrage der allgemeinen oder Node-bezogenen API |
| Operator-CLI | `legacy-wap …` | Sendekommando für den Legacy-Pfad |
| Native UI | Seite „Paketdaten“ | Gateway-/Kontext-/Beareransicht und Legacy-WAP-Formular |

Das im Entwurf skizzierte WebSocket-Beispiel

```json
{"type":"packet_data","gateway":{},"contexts":[],"bearers":[]}
```

ist ein Schemabeispiel, kein vollständiger garantierter Vertrag aller HTTP-Antworten. HTTP kann einen Wrapper wie `packet_data` und zusätzliche Metadaten verwenden. Beim Anschluss neuer Clients müssen konkrete Serializer und tatsächliche Antworten am gewählten Commit geprüft werden.

### 9.3 Pfade und Buildabgrenzung

Der Control-Room-Core liegt unter `bins/netcore-control-room/`. Die Operator-CLI befindet sich im betrachteten Quellbaum unter **`system-backend/control-room/operator/`**, auch wenn ihr Paket-/Binaryname `netcore-control-room-operator` lautet. Die native UI hat ihr eigenes Manifest `system-backend/control-room/ui/Cargo.toml` und wurde separat gebaut beziehungsweise als separat zu bauen beschrieben.

Ein aktualisierter Rust-Server allein ersetzt keine bereits ausgelieferte native UI-Binärdatei. TBS, Core, Operator und UI müssen bei geänderten Serialisierungs-/Telemetry-Enums kompatibel zusammen eingesetzt werden. Historische Aussagen über binär stabile Erweiterungen sind keine Ersatzprüfung des konkreten Bitcode-Vertrags.

## 10. Legacy-WAP über SDS Type 4

### 10.1 Tatsächlich vorhandener Generator

Das Modul `crates/tetra-entities/src/legacy_wap.rs` führt zwei Varianten:

| Variante | Beginn des vollständigen Type-4-Payloads | Verbleibendes Bytebudget bei 255 Byte insgesamt |
|---|---|---:|
| `wdp` | `04` + Anwendungsnutzlast | 254 |
| `sds_tl` | `84 00 <message_reference>` + Anwendungsnutzlast | 252 |

Die Bezeichnungen im Code sind `WAP_WDP_PROTOCOL_ID`, `WAP_SDS_TL_PROTOCOL_ID`, `SDS_TYPE4_MAX_BYTES=255`, `SDS_TL_NO_REPORT_FLAGS=0x00` und `LegacyWapTransport::{Wdp,SdsTl}`. `build_type4_payload` lehnt leere beziehungsweise zu große Nutzlasten ab.

`render_compact_wml` erzeugt eine WML-1.1-Karte mit Titel, Nachricht und optionalem Link; XML-Sonderzeichen werden escaped. `build_compact_wml_type4` prüft zuerst, ob das feste Gerüst aus XML, Titel und URL überhaupt passt, und sucht anschließend eine passende Länge des Nachrichtentexts. Es wird nicht mitten in einem UTF-8-Zeichen, XML-Tag oder URL abgeschnitten. Ein zu großes fixes Gerüst wird abgelehnt, nicht durch Beschädigen des Markups passend gemacht.

Der erzeugte Payload soll den vorhandenen Raw-SDS-Type-4-Pfad nutzen, ohne den Protokoll-Identifier doppelt einzufügen. Die hier genannte 255-Byte-Grenze ist die verwendete byteausgerichtete Implementierungsgrenze für den vollständigen Type-4-Inhalt, nicht ein beliebiges WML-Seitenbudget zusätzlich zum Header.

### 10.2 Grenzen und notwendige Interoperabilitätsprüfung

**PID + rohe WML-Karte ist nicht automatisch ein interoperabler WAP-Push-Stack.** EN 300 392-2, Abschnitt 29.5.8, definiert als minimale standardisierte WAP-Information den Protocol Identifier und verweist für weitere WAP-Aspekte auf WAP 2.0. Damit ist die PID-Zuordnung allein kein Beleg für vollständige WDP-/WSP-/Push-/WBXML-Codierung.

Im gelieferten Modul wird die Anwendungsnutzlast mit einer PID- beziehungsweise minimalen SDS-TL-Hülle versehen. Ob das Zielterminal rohe WML annimmt, einen Port-/WDP-Header, WSP Push/Service Indication, WBXML oder einen anderen herstellerspezifischen Envelope erwartet, muss separat belegt werden. Es gibt im Entwurf keinen erfolgreichen Terminalmitschnitt, der automatisches Öffnen oder Anzeigen dieser WML-Karten bestätigt.

Daher lautet der belastbare Status: **Generator und Sendeschnittstelle implementiert; reale Legacy-WAP-Browserkompatibilität offen.** Der unabhängig vorhandene SNDCP/WTP/WSP-Statuspfad bleibt davon getrennt.

### 10.3 Beispielanforderung, kein Ausführungsnachweis

Historisch wurde folgendes Muster vorgeschlagen; Node, Teilnehmer, Rechte und erreichbaren Dienst vor Verwendung prüfen:

```json
{
  "operator_id": "jan",
  "dest_issi": 4010002,
  "source_issi": 4010001,
  "title": "NetCore",
  "message": "Statusseite öffnen",
  "url": "http://10.0.0.1:9200/",
  "transport": "wdp"
}
```

Für die andere Hülle: `"transport":"sds_tl"` und beispielsweise `"message_reference":1`. Die Adressen/ISSIs sind historische Beispiele; daraus entsteht weder ein endgültiger Nummernplan noch ein Beleg für tatsächlich registrierte Geräte.

## 11. Konfiguration und technische Parameter

### 11.1 Historisches WAP-/SNDCP-Profil

Die folgenden Werte dokumentieren die beschriebene und ausgelieferte Profilkonfiguration. Sie sind **keine neu auf eine TBS angewendete Konfiguration**:

```toml
[cell_info.wap_ip]
enabled = true
address = "10.0.0.1"
port = 9200
response_ttl = 32
dynamic_pool_prefix = "10.0.0"
dynamic_pool_first_host = 2
dynamic_pool_last_host = 254
allow_static_ipv4 = true
accept_empty_probe = true
accept_root_path = true
accept_status_path = true
accept_status_wml_path = true
max_request_payload_bytes = 1024
assume_pdch_ready_after_data_transmit = false
pdu_priority_max = 4
ready_timer_code = 8
standby_timer_code = 4
response_wait_timer_code = 7
mtu_code = 2
network_default_data_priority = 4
max_contexts_per_issi = 4
max_total_contexts = 64
strict_source_address = true
```

Die Timer-/MTU-Werte wurden im damaligen Paket als READY 10 s, STANDBY 300 s, RESPONSE WAIT 5 s und MTU 576 Byte erläutert. Bei künftigen Änderungen immer die Codetabelle und ausgehandelte Antwort gemeinsam prüfen, nicht nur die Kommentarwerte. `assume_pdch_ready_after_data_transmit=false` ist eine explizite Policy, kein fehlender Timer.

Frühere Begleitbeispiele setzten außerdem `sndcp_service=true` und `advanced_link=true` unter `[cell_info]`. Im geprüften `config.rs` ist die Profilaktivierung ausdrücklich an `wap_ip.enabled || packet_data_gateway.enabled` gekoppelt. Die Annahme, SNDCP beziehungsweise das allgemeine Gateway könne nur zusammen mit dem lokalen WAP-Server aktiviert werden, ist damit nicht als ergänzende Vorgabe zu übernehmen.

### 11.2 Historisches Packet-Gateway-Profil und letzte Ergänzungen

```toml
[cell_info.packet_data_gateway]
enabled = true
interface_name = "ntetra0"
prefix_len = 24
auto_configure = true
enable_ipv4_forwarding = true
managed_forwarding = true
allow_unsolicited_inbound = false
nat_mode = "masquerade"
firewall_backend = "auto"
dns_servers = ["1.1.1.1", "9.9.9.9"]
channel_capacity = 256
downlink_queue_packets_per_context = 64
downlink_queue_bytes_per_context = 262144
downlink_queue_ttl_secs = 30
page_retry_secs = 5
fragment_reassembly_timeout_secs = 30
fragment_reassembly_max_datagrams = 128
fragment_reassembly_max_bytes = 4194304
automatic_filter_ttl_secs = 300
automatic_filter_max_bindings = 4096
max_pdch_bearers = 0
reserved_voice_slots = 1
prefer_secondary_carrier = true
```

Im gelesenen Konfigurationscode werden `max_pdch_bearers` auf 0–6 und `reserved_voice_slots` auf 0–5 begrenzt. NAT, Firewall und Forwarding müssen zum lokalen oder gerouteten Betriebsmodell passen. Ein pauschaler Umbau des Hosts zum Internetrouter ist kein Bestandteil der festgelegten Erweiterung.

### 11.3 Dienste, Adressen, Protokolle und Abhängigkeiten

| Element | Bedeutung / Stand |
|---|---|
| `tetra.service` | Historisch verwendeter systemd-Dienstname; reale Unit und ExecStart vor Installation prüfen |
| `bluestation-bs` | Basisstations-Binary |
| `netcore-control-room` | Control-Room-Core-Binary |
| `netcore-control-room-operator` | Operator-CLI-Binary |
| `ntetra0` | Nicht persistentes Linux-TUN-Interface nach Profil |
| `10.0.0.1/24` | Historisches Gateway/Teilnehmernetz, keine Aussage über die tatsächlich aktive Hostroute |
| `10.0.0.2`–`10.0.0.254` | Historischer dynamischer Pool |
| UDP 9200 | WAP-Gateway-Endpunkt im beschriebenen Profil |
| HTTP 9010 | In Control-Room-Beispielen verwendeter API-Port; Konfiguration und Bindadresse separat prüfen |
| TCP/UDP/ICMP über IPv4 | Allgemeine Hostnetzfunktionen, kein eigener vollständiger TCP-Code im WAP-Modul |
| SDS Type 4 / PID 0x04 und 0x84 | Legacy-WAP-Hüllen |
| Bitcode / Serde | Serialisierung und JSON-Strukturen in den gelesenen Telemetrie-/Control-Datentypen |
| Linux, `/dev/net/tun`, `CAP_NET_ADMIN` | Voraussetzungen der lokalen TUN-/Hostkonfiguration |
| `iproute2`, `nftables`/`iptables`, `tcpdump` | Vorgeschlagene Host- und Diagnosewerkzeuge |
| Rust/Cargo, SDR-/SoapySDR-Umgebung | Projektbuild und Hardwareintegration; genaue Zieltoolchain dieser Planung nicht aufgezeichnet |
| Feature `bluestation-bs/asterisk` | In den vollständigen Buildbeispielen aktiviert; native Codec-/Telefonieabhängigkeiten bleiben zusätzlich erforderlich |

Die Beispielhostadresse `10.0.1.25:9010` und der Node-String `SRV-M_TBS-01` waren im Verlauf Befehlsbeispiele, keine unabhängig bestätigten Live-Endpunkte dieser Planung. Den Node-String nicht stillschweigend mit anders geschriebenen Hostnamen gleichsetzen.

## 12. Compilerfehler und R1-Reparatur

### 12.1 Tatsächlich gemeldete Fehler

Gemeldeter Fehlerkomplex im Zielbuild von `tetra-entities`:

| Diagnose | Historische Fundstelle | Ursache |
|---|---|---|
| `E0432` | `sndcp/wap_ip.rs:3` | Import `super::ip::IPV4_UDP_HEADER_BYTES` ohne entsprechende Definition |
| `E0425` | `sndcp/sndcp_bs.rs:893` | Aufruf von `snei_optional_section(...)`, aber Helper nicht vorhanden/im Scope |
| `E0599` | `sndcp/wap_ip.rs:281`, Enum in `ip.rs:8` | Variante `IpError::UnsupportedProtocol` fehlte |
| `E0282` | `sndcp/packet_gateway.rs:469` | Closure-Result aus `Ok(())` nicht eindeutig typisierbar |
| `E0283` | `packet_gateway.rs:488` | Folgeambiguität bei `result?`, unter anderem durch `From<io::Error>` und Identitätskonvertierung |

Die Zeilennummern bezeichnen den damaligen Baum. Durch spätere Kommentare und Erweiterungen sind sie keine stabilen geprüften Sprungmarken.

### 12.2 Korrekturen

In `sndcp/ip.rs` wurden die gemeinsam benötigten Konstanten und die fehlende Enumvariante ergänzt:

```rust
pub const IPV4_HEADER_BYTES: usize = 20;
pub const UDP_HEADER_BYTES: usize = 8;
pub const IPV4_UDP_HEADER_BYTES: usize = IPV4_HEADER_BYTES + UDP_HEADER_BYTES;
// zusätzliche Variante in IpError:
// UnsupportedProtocol(u8)
```

Der SNEI-Helper wurde ergänzt:

```rust
fn snei_optional_section(snei: Option<u16>) -> String {
    match snei {
        Some(snei) => format!("11{snei:016b}0"),
        None => "0".to_string(),
    }
}
```

Die nftables-Setup-Closure wurde explizit auf `Result<(), GatewayError>` festgelegt. Das rohe WAP-Antwortbudget verwendet anschließend die gemeinsame Konstante `576 - IPV4_UDP_HEADER_BYTES`, statt auf eine fehlende beziehungsweise inkonsistente Headerdefinition zuzugreifen.

### 12.3 Ergebnis und verbleibende Reichweite

Nach dem R1-Paket wurde ein fehlerfreier Lauf bestätigt. Dies ist der konkrete Erfolg für den gemeldeten Compilerblock. Konstanten, Enumvariante, Helper und explizite Closure-Typisierung sind auch im geprüften Repository nachweisbar.

Nicht belegt sind dadurch sämtliche Unit-Tests, der gesamte Windows-/UI-Build, TUN-Firewallbetrieb oder irgendeine WAP-Seite auf einem realen MS. Ebenso wenig überträgt sich diese Bestätigung automatisch auf das danach geänderte Multi-PDCH-Paket.

Die ursprünglichen Syntaxprüfungen hatten diese Fehler nicht entdeckt. Daraus folgt die konkrete Prozessanforderung: **Rust-Typcheck/Build dürfen vor künftigen Auslieferungen nicht durch Tree-sitter-Syntaxprüfung ersetzt werden.**

## 13. Historische Build-, Deployment- und Diagnoseabläufe

### 13.1 Status der Befehle

Die nachstehenden Befehle wurden im Entwurf als Vorgehen vorgeschlagen. Für sie liegt keine lückenlose Ausführungshistorie vor. Ein einzelner R1-Erfolg wurde bestätigt, ohne genaues Kommando, Rust-Version, Featureauflösung und Binärhash. Dieses Archiv führt keine der Dienst-, Firewall- oder Sendebefehle aus.

### 13.2 Vor jedem erneuten Einspielen

```bash
cd ~/netcore-tetra
git status --short
git rev-parse HEAD
systemctl cat tetra.service
```

Damit zuerst tatsächlichen Quellstand, lokale Änderungen und die von systemd verwendete Binärdatei ermitteln. Konfigurationen, etwa `config.toml` und `config.toml.fallback`, sowie lokale Assets separat sichern. Archivbranch und geprüfter Produktivbranch sind nicht automatisch der installierte Stand.

Historisch vorgeschlagen waren `git apply --check <passender-patch>` und erst danach `git apply <passender-patch>`. Jeder Patch ist an seine konkrete Vorstufe gebunden. Ein Patch gegen den alten WAP- oder SNDCP-Baum ist kein beliebiges Update für den geprüften `main`.

### 13.3 Typcheck und Tests

```bash
cargo check -p tetra-core
cargo check -p tetra-config
cargo check -p tetra-entities --features runtime
cargo check -p netcore-control-room
cargo check -p netcore-control-room-operator

cargo test -p tetra-core timeslot_alloc
cargo test -p tetra-config
cargo test -p tetra-entities sndcp --features runtime
cargo test -p tetra-entities legacy_wap --features runtime
```

Diese Filter sind gezielte Prüfungen, keine vollständige Workspace-Abnahme. Für aktuelle Änderungen zusätzlich alle betroffenen Testtargets und Featurekombinationen bestimmen. `cargo check` und ein vollständiger Build sind getrennte Prüfschritte; ein Check führt nicht alle abschließenden Codegenerierungs-/Linkschritte aus.

### 13.4 Build

```bash
cargo build --release \
  -p bluestation-bs \
  -p netcore-control-room \
  -p netcore-control-room-operator \
  --features bluestation-bs/asterisk

cargo build --release \
  --manifest-path system-backend/control-room/ui/Cargo.toml
```

Die native UI wurde als separates Projekt behandelt. Vor Verteilung Buildplattform und Zielplattform beachten. Der Asterisk-Build setzt eine passende native Codec-/Telefonieumgebung voraus; das Hinzufügen eines Cargo-Flags installiert diese nicht automatisch.

### 13.5 TUN-/Hostintegration und Diagnose

Historische Installationsbeispiele:

```bash
sudo apt install -y iproute2 nftables iptables tcpdump
test -c /dev/net/tun
sudo contrib/packet-data/netcore-tetra-packet-gateway-install tetra.service
sudo systemctl daemon-reload
```

Vorgeschlagene Kontrollen nach einem bewusst freigegebenen Start:

```bash
sudo contrib/packet-data/netcore-tetra-packet-gateway-status ntetra0
ip -details address show ntetra0
ip -4 route show dev ntetra0
sudo journalctl -u tetra.service -n 500 --no-pager
sudo tcpdump -ni ntetra0
```

`tcpdump` und Logausgaben können Teilnehmer- und Inhaltsdaten enthalten. Keine vollständigen Payloads, Zugangsdaten oder Schlüssel unkontrolliert in öffentliche Issues oder Archive übernehmen. Die Hoständerungen und der Neustart gehören in einen geplanten Testablauf, nicht in diese reine Archivierung.

### 13.6 Control-Room-Abfragebeispiele

```bash
netcore-control-room-operator --api http://127.0.0.1:9010 packet-data
netcore-control-room-operator --api http://127.0.0.1:9010 \
  packet-data --node <tatsaechliche-node-id>
```

Der CLI-Platzhalter muss vor Ausführung ersetzt werden; spitze Klammern sind keine fertige Shellsyntax. Für Legacy-WAP wurden `legacy-wap`, `--node`, `--dest-issi`, `--title`, `--message`, `--url`, `--transport wdp` beziehungsweise `--transport sds_tl --message-reference 1` vorgeschlagen. Erst mit freigegebenem Zielgerät und geklärtem Payloadprofil senden.

### 13.7 Nicht als Standardablauf weiterverwenden

Frühere Reparaturvorschläge enthielten `rm -rf target` und `cargo clean`, zuletzt auch Löschen von `~/netcore-tetra` nach Sicherung und Ersatz durch ZIP. **Dies sind unbestätigte historische Ansätze, keine notwendige Standardreparatur.** Das Löschen des gesamten Repositorys kann Git-Historie, lokale Änderungen und neuere Funktionen verlieren.

Änderungen gegen bekannten Commit integrieren, Git-Metadaten und Konfiguration bewahren, vor einem Dienststopp separat bauen und tatsächliche Binärdatei kontrolliert ersetzen. Alte Komplettfassungen von `html.rs` dürfen ausgelagerte UI-Assets nicht zurücksetzen. Eine operative Neuinstallation wurde bei der Prüfung nicht vorgenommen.

## 14. Tests, Artefaktprüfung und Evidenz

### 14.1 Historische Angaben aus Entwicklungsberichten

| Paketstufe | Damals genannte Prüfungen | Was daraus ausdrücklich nicht folgt |
|---|---|---|
| WAP | 447 Rust-Dateien syntaktisch geparst; TOML-/Struct-/Vektorprüfung; Patch-/ZIP-Roundtrip | Kein Rust-Typcheck, Linkergebnis oder RF-Nachweis |
| SNDCP | 451 Rust-Dateien; TOML; PDU-/ACTIVATE-/UNITDATA-Vektoren; Tests ergänzt; Patchprüfung | Testcode vorhanden heißt nicht erfolgreich `cargo test` ausgeführt |
| Packet Data | 453 Rust-Dateien; 19 TOMLs; vier Shellhelfer mit `sh -n`; systemd-Analyse; IPv4/PCO/Fragment-/Subnetzvektoren | Danach traten reale Compilerfehler auf; diese Stufe war nicht vollständig gebaut |
| R1 | Wieder Syntax-/Artefaktprüfung; anschließend Betreiber-Erfolg | Keine nachträgliche pauschale Bestätigung aller Protokoll-/Hostfunktionen |
| Multi-PDCH/UI/Legacy | 454 Rust-Dateien; 19 TOMLs; zwei JavaScript-Blöcke mit Node.js; Diff-/ZIP-/Prüfsummen-/Roundtripprüfung laut Entwicklungsbericht | Cargo-Build und anschließender Betreiber-Build nicht belegt |

Die damalige Einschränkung lautete wiederholt: Rust-Toolchain im Arbeitscontainer nicht verfügbar, Download-/DNS-Probleme. Die Formulierung „getestet“ in den alten Abschlussantworten muss deshalb auf die jeweils tatsächlich behauptete statische Prüfung begrenzt bleiben.

### 14.2 Neu ausgeführte Archivprüfungen am 2026-10-03

Tatsächlich in dieser Archivierung ausgeführt, ausschließlich in lokalen Prüfarbeitskopien:

- Alle acht vorhandenen ZIPs inventarisiert, SHA-256 berechnet und mit ZIP-CRC-Prüfung ohne Archivfehler gelesen.
- Letztes Feature-ZIP entpackt, 660 reguläre Dateien, darunter 454 Rust- und 19 `.toml`-Dateien erfasst.
- Alle 19 TOML-Dateien dieses letzten Pakets mit Python `tomllib` erfolgreich geparst. Das ist keine vollständige NetCore-Schemavalidierung.
- Alle 22 Dateireferenzen aus den fünf verfügbaren SHA-256-Manifesten erfolgreich gegen die vorhandenen Bytes geprüft.
- Letzten Feature-Patch gegen eine frische Extraktion von `netcore-tetra-wap(1).zip` mit `git apply --check` erfolgreich geprüft und mit `git apply` erfolgreich angewendet.
- Ergebnisbaum mit dem finalen ZIP dateiweiser per SHA-256 verglichen; alle gemeinsamen 660 Dateien identisch, keine Datei des Ergebnis-ZIPs fehlt im gepatchten Baum.
- Zentrale historische Quellmodule gelesen sowie wichtige Aussagen gezielt gegen den geprüften GitHub-Commit geprüft.

**Nicht neu ausgeführt:** Cargo-Check, Cargo-Tests, Release-Build, Rust-Syntaxanalyse sämtlicher Dateien, Node-JavaScriptprüfung, Shell-/systemd-Funktionstests, Start der TBS, Firewalländerung, TUN-Livetest oder Funkübertragung. Ein read-only Clone-Versuch scheiterte an DNS; der Live-Quellzugriff war über den GitHub-Connector möglich. Das verhindert die hier dokumentierten Remote-Lese-/Schreibzugriffe nicht.

### 14.3 Abweichung beim letzten Patch-Roundtrip

Das finale ZIP besitzt 810 ZIP-Einträge einschließlich Wurzelverzeichnis; ohne die Wurzel sind es 809. Die alte Zahl „809 Einträge“ kann daher eine solche Zählweise meinen. **Die Aussage eines vollständig identischen Baums wird dadurch jedoch nicht richtig:**

| Vergleich | Neu gemessen |
|---|---:|
| Reguläre Dateien im finalen ZIP | 660 |
| Reguläre Dateien nach Patch auf hochgeladenes `(1).zip` | 664 |
| Abweichende Inhalte unter gemeinsamen Pfaden | 0 |
| Nur im finalen ZIP vorhandene Dateien | 0 |
| Nur im gepatchten Ausgangsbaum vorhandene Dateien | 4 |

Die vier zusätzlichen Pfade sind:

```text
wiki/Device‐Groups.md.md
wiki/NetCore‐Directory.md.md
wiki/Status‐Messages.md.md
wiki/Systemd‐Service.md.md
```

Der Bindestrich in diesen vier Namen ist Unicode **U+2010**. Im finalen ZIP verbleiben stattdessen die bereits im Ausgangs-ZIP zusätzlich vorhandenen Dateinamensformen mit dem wörtlichen Bestandteil `#U2010`, zum Beispiel `wiki/Device#U2010Groups.md.md`. Der Patch enthält keine Löschoperationen für die vier Unicode-Pfade.

Folgerung: Der Featurecode und sämtliche Dateien des gelieferten Ergebnis-ZIPs stimmen beim neuen Vergleich überein; es besteht eine zusätzliche **Wiki-Dateinamens-/Paketierungsabweichung**, kein in diesem Vergleich gefundener abweichender Rust-Dateiinhalt. Diese Archivierung korrigiert die damalige pauschale Roundtripbehauptung, löscht aber keine Wiki-Dateien und verändert keinen Patch. Eine künftige Bereinigung muss Links, Namen und Versionshistorie bewusst berücksichtigen.

### 14.4 Historische Referenzvektoren

Zur Wiederaufnahme der Tests sind folgende im Verlauf beziehungsweise den Begleitdateien angeführten Werte wichtig. Sie werden als historische Testanker erhalten, nicht als am 2026-10-03 erneut vollständig unabhängig zertifizierte Vektoren:

```text
ACTIVATE ACCEPT, 70 signifikante Bits, byteweise aufgefüllt:
02 90 8E 82 80 00 38 80 10

SN-UNITDATA-Kopf und IPv4-Anfang:
42 00 45 00 00 14
Nicht: 42 45 ...; zwischen NSAPI und IPv4 liegen PCOMP/DCOMP.

IPv4/UDP-Referenzdatagramm:
4500001f00070000201116c4c0000201c0000202c00023f0000b0000776170

WTP Result / WSP ConnectReply:
12 93 cc 02 01 08 00 03 80 84 21 03 81 84 21

IPCP-DNS-Configure-Request:
01 07 00 0a 81 06 00 00 00 00

IPCP-DNS-Configure-NAK:
03 07 00 10 81 06 01 01 01 01 83 06 09 09 09 09
```

Ein genannter Fragmenttest teilte ein 1.220-Byte-Datagramm bei MTU 300 in Paketlängen 300/300/300/300/100. Byteanzahl, Headerlängen, Kopieroptionen und Out-of-order-Zusammensetzung müssen in reproduzierbaren ausführbaren Tests zusammen geprüft werden.

## 15. Geprüfter Repository-Stand – getrennt vom historischen Ergebnis

### 15.1 Geprüfte Revisionen und Branchbeziehung

Der aktuelle `main` wurde auf **`7137e0dd69877e1b604bf89148fd8b6b590c1a97`** festgehalten. Dieser Commit ist der Merge von **PR #59**, `feat/netcore-dashboard-design`, mit dem Thema NetCore-Design/Dark Mode für Basisstation und Dienst-WebUIs. Das ist Kontext für spätere UI-Änderungen und **kein nachgewiesener ursprünglicher Feature-PR dieser Planung**.

Der zu Beginn gelesene `Archiving`-Head war **`fd6c77b1eb4e1c656daf40ac1d4b406a2ca41249`**. GitHub meldete gegenüber `main` eine divergierte Historie mit 15 vorausliegenden und einem fehlenden Commit; der gemeinsame Ausgangspunkt war `2fe2a1939a8795db3816d45973282781dae856f0`. Dessen Tree entspricht dem gelesenen `main`-Tree. Die Vergleichsdateiliste enthielt nur zusätzliche Dokumentation: `Docs/Control_Room/Readme.md` sowie Archivdateien und Index. **Die in diesem Audit betrachteten Produktivquellen unterscheiden sich dadurch nicht zwischen diesen beiden gepinnten Zuständen.**

Die vorhandene Datei außerhalb des Archivverzeichnisses stammt aus vorherigen Änderungen und gehört nicht zu diesem Auftrag. Für den Archivcommit wird der Branch vor dem Schreiben erneut geladen; fremde Archivänderungen sind zu bewahren.

### 15.2 Befundmatrix

| Thema | Geprüfter Code-/Dateibefund | Verhältnis zum historischen Entwurf |
|---|---|---|
| Dynamische SNDCP-Slots | `TimeslotOwner::Sndcp`, bevorzugte Vergabe, Headroom, ISSI-Bearermap und konkrete Freigabe vorhanden | Letzte Featureidee nicht mehr nur ZIP-Behauptung; Quellimplementation nachweisbar |
| Slotabbildung | Hauptträger 2–4, Sekundär 5–7 -> Air-TS 2–4, interner Secondary-Hint vorhanden | Bestätigt den begrenzten Ein-Slot-Pool, nicht Multislot/TEDS |
| R1-Konstanten und Fehlerenum | Headergrößen 20/8/28 und `UnsupportedProtocol(u8)` in `sndcp/ip.rs` vorhanden | Gemeldete konkrete Import-/Enumfehler inzwischen im Repository adressiert |
| R1-SNEI/Result | `snei_optional_section` vorhanden; nftables-Closure ausdrücklich `Result<(), GatewayError>` | Konkrete Reparaturen sichtbar; keine Aussage über vollständigen Build dieses geprüften Commits |
| WAP/Gateway-Aktivierung | `config.rs` berücksichtigt WAP oder Packet Gateway | Starre Kopplung an ausschließlich WAP nicht als aktuelle Anforderung behandeln |
| Packet Data in Control Room | HTTP-Routen, Operator-Abfrage und native UI-Polling von `/api/packet-data` vorhanden | UI-/API-Code ist vorhanden; kein aktueller E2E-Betriebsnachweis |
| Legacy-WAP | Generator, PID-/TL-Hüllen und Control-Room-Submit-Funktion vorhanden | Standard-/Terminalkompatibilität bleibt eine separate offene Prüfung |
| Lokales Dashboard | Snapshot-Datentyp vorhanden; `html.rs` bindet `ui/`-Assets ein | Alte Komplettdatei mit eingebettetem HTML ist als Updatebasis überholt |
| Paketdaten-Control | SNDCP verarbeitet nun Deactivate/Modify/Wake/EndOfData aus der Control Plane | Spätere Erweiterung gegenüber dem historischen letzten Featurepaket |
| Eigenständiger Packet Core | `system-backend/packet-core/`, Dokumentation Paket G, zugehörige Control-Zuordnung vorhanden | Ergänzende Netzarchitektur ist weiterentwickelt; nicht als in diesem Planungsstand damals erledigt darstellen |

### 15.3 Später hinzugekommene Packet-Core-Control-Anbindung

Der am 2026-10-03 gelesene SNDCP-Code besitzt `process_control_commands`. Nachweisbare Kommandofamilien sind:

```text
PacketDataContextDeactivate
PacketDataContextModify
PacketDataWake
PacketDataEndOfData
PacketDataActionResult
```

Sie werden in der Control-Verarbeitung zur SNDCP-Entity geroutet. Deactivate prüft unter anderem lokale Route und Kontextvorhandensein und meldet das Queuen eines SN-DEACTIVATE. Wake bereinigt/prüft die NSAPI-Auswahl und stößt Paging an.

**Besonders wichtige aktuelle Grenze:** Beim gelesenen Modify-Pfad werden Availability/Pause-Änderungen über entsprechende Nachrichten signalisiert, während Priorität, MTU und Resume nach dem ausdrücklichen Ergebnistext als lokale Policyänderungen verbleiben. Ein erfolgreicher API-Response ist daher nicht automatisch eine vollständig über Funk ausgehandelte neue MTU oder Priorität.

Die ergänzende Datei `Docs/SWMI_CORE_1_PACKAGE_G_PACKET_CORE.md` beschreibt einen eigenständigen Dienst unter `system-backend/packet-core/` mit Management-WebUI auf Port **8160**, langlebiger Netzsicht und dem Edge-Protokoll **`netcore-packet-edge-v1`**. Die Dokumentation nennt außerdem Shadow-/Authoritative-Modus, Persistenz, API/OpenAPI/Metrics, Pool-/Mobility-Anchor- und Outbox-Funktionen. Diese weitergehenden Aussagen wurden hier nicht sämtlich in ihren Laufzeitpfaden abgenommen; konkret geprüft wurde insbesondere die Control-Zuordnung und der lokale SNDCP-Empfang.

Paket G grenzt den zentralen Kontextdienst von einem späteren **`ip-gateway` / LXC 09** ab. Das bedeutet nicht, dass das in diesem Planungsstand gebaute lokale TUN-Gateway verschwunden wäre. Bei der Fortsetzung sind lokaler Funk-/Gatewaypfad und zentrale Kontext-/Adressautorität ausdrücklich auseinanderzuhalten.

Die Paket-G-Dokumentation kennzeichnet den Dienst als **`open_lab` ohne Login, Token und TLS**. Das ist ein wichtiges geprüftes Sicherheitsrisiko und kein im Entwurf erteilter Auftrag, ihn ungeschützt außerhalb eines abgegrenzten Labors erreichbar zu machen. Kontextänderungen und sensible Netzwerkzustände dürfen nicht allein aufgrund eines bestehenden HTTP-Ports als geschützt gelten.

### 15.4 Spätere Dashboard-Umstrukturierung

Im geprüften `crates/tetra-entities/src/net_dashboard/html.rs` werden eingebunden:

```text
ui/dashboard.html
ui/login.html
ui/netcore.css
ui/netcore.js
ui/netcore-rf.css
ui/netcore-rf.js
ui/netcore-login.js
ui/netcore-logo.png
```

Das ist eine konkrete Abweichung zur alten ZIP-Datei mit eingebetteten Dashboard-Blöcken. Änderungen an Paketdatenformularen und Ereignisbehandlung sind jetzt gegen diese Struktur zu planen. Der vorhandene UI-Testfixtureeintrag für `/api/packet-data` zeigt eine Prüfgrundlage, aber noch keinen bestandenen Lauf oder echte Daten vom Funkgerät.

### 15.5 Was die ergänzende Prüfung nicht behauptet

Kein vollständiger Audit sämtlicher Runtime-Zweige, kein aktueller Workspace-Build, kein neuer Upstreamvergleich und keine Hardwareprüfung wurden durchgeführt. Die Codepräsenz beseitigt nicht die offene Frage nach tatsächlicher Mehrteilnehmerübertragung auf beiden Carriern, korrekt rückgeführten Zuständen, sicherer IP-Freigabe und terminalgerechtem Legacy-WAP-Envelope.

## 16. Ersetzte, verworfene oder zu relativierende Aussagen

| Frühere Aussage / Ansatz | Abschließende Einordnung |
|---|---|
| Fester TS2 für SNDCP | In der letzten Featurestufe durch dynamischen Pool ersetzt |
| Funkabriss hält TS2 bis Registrierung/Neustart | Später durch Timer-/Lebenszyklusbereinigung adressiert; RF-Abnahme der Bereinigung fehlt |
| Nur lokale WAP-Statusseite, kein allgemeines IP | Durch Packet-Gateway-Stufe erweitert |
| „SNDCP komplett“ beziehungsweise „alles vollständig“ | Nur für ausdrücklich eingeschränktes Profil als Entwicklungsziel verständlich; kein Nachweis aller Optionen/ETSI-Prozeduren |
| Multi-PDCH bedeutet mehrere Slots pro Teilnehmer | Nicht zutreffend für die gelieferte Stufe: ein Slot pro ISSI, mehrere NSAPIs gemeinsam |
| Notrufe räumen laufende Daten automatisch ab | In der gelieferten Stufe nicht umgesetzt; Headroom statt harter Präemption |
| Acht angezeigte Timeslots sind acht PDCHs | Nicht im implementierten Pool: drei beziehungsweise sechs Traffic-Slots, davon Headroom abziehen |
| `operator_id` ist Authentifizierung | Keine solche Garantie; vorhandenen Auth-/RBAC-Pfad separat prüfen |
| WAP-PID mit WML genügt jedem Terminal | Nicht nachgewiesen; vollständigen WDP/WSP-/Push-Envelope und Codeplug prüfen |
| R1-Build bestätigt spätere Multi-PDCH-Stufe | Zeitlich nicht zulässig; die Bestätigung ging der letzten Änderung voraus |
| SHA/CRC beweisen korrekte Funkfunktion | Belegen Artefaktidentität beziehungsweise Lesbarkeit, keine Protokollfunktion |
| Letzter Patch und ZIP vollständig bytegleich | Neuer Audit: 660 gemeinsame Dateien gleich, vier zusätzliche Unicode-Wiki-Pfade nur im gepatchten Baum |
| `main` steht aktuell bei `41d9254` | Historische Aussage; am 2026-10-03 geprüfter `main` ist `7137e0dd…` |
| `rm -rf`/Komplett-ZIP ist bevorzugter Updateweg | Historischer unbestätigter Vorschlag; gegenüber geprüftem Quellbaum nicht blind wiederholen |
| Clean-room-Label beweist Lizenzfreiheit | Unabhängige Provenienz-/Lizenzprüfung nicht durchgeführt |
| Allgemeines DHCP-/IPv6-/Multicast-Verhalten aus kurzen Roadmapbemerkungen | Nicht als technisch universelle Regel übernehmen; Profil, Bearer und Endgerät bestimmen den konkreten Aufwand |

## 17. Vollständiges offenes Ideen- und Aufgabenregister

Die Auswahl der letzten drei Features umfasst nicht sämtliche späteren Ausbauideen. Die Statusangaben beschreiben den historischen Lieferstand; andere Revisionsstände können einzelne Punkte zusätzlich enthalten.

| ID | Thema | Status aus diesem Planungsstand / ergänzende Einordnung | Nutzen, offene Abhängigkeit oder Grenze |
|---|---|---|---|
| R01 | R1-Stand identifizieren, committen, taggen und sichern | Empfohlen; konkreter damaliger Push/Tag nicht nachgewiesen | Build mit Commit, Features, Toolchain und Binärhash verbinden; ergänzende Quellen nicht auf alten Stand zurücksetzen |
| R02 | On-Air-Testmatrix und automatisierte Regression | Beschlossen als notwendige Qualitätsaufgabe, Ausführung nicht belegt | Reale PDP-/WAP-/IP-/Multi-PDCH-Funktion, Fehlerpfade und Wiederanlauf absichern |
| R03 | Dynamischer Multi-PDCH-Pool | Ausgewählt; ZIP und geprüfter Quellcode vorhanden | Mehrere ISSIs, Carrierzuordnung, Freigabe und Parallelbetrieb abnehmen |
| R04 | Paketdaten-Dashboard und Control-Room-Integration | Ausgewählt; Quellcode vorhanden | Live-Polling/WebSocket, Node-Zuordnung, veraltete Snapshots und UI-Binaries prüfen |
| R05 | Legacy-WAP über SDS Type 4 | Ausgewählt; Generator/Sendepfad vorhanden | Tatsächlichen Envelope, PID-/TL-Grenzen, UTF-8 und Endgeräteanzeige prüfen |
| R06 | Fairer dynamischer Scheduler / Backpressure | Weitergehende Idee; durch bevorzugte Erstvergabe nicht vollständig erfüllt | Wartende MS, begrenzte Queues, Airtime-Anteile und Verhungern unter Dauerlast untersuchen |
| R07 | Harte Sprach-/Notruf-Präemption von PDCH | Idee; im letzten Paket ausdrücklich nicht umgesetzt | Sichere Suspend-/END-/Reallocation-Verfahren und Rückkehr nach Sprachruf definieren |
| R08 | Multislot für ein MS / Enhanced PDCH | Idee; nicht aus mehreren Ein-Slot-Bearern ableitbar | MAC, Kanalallokation, Endgerätecapabilities und Durchsatzabnahme |
| R09 | Teilnehmerprofile/APN-ähnliche Policies | Idee, nicht durch die drei letzten Featureaufträge pauschal freigegeben | ISSI-Profil, lokale/Internet-Freigabe, erlaubte Netze/Protokolle/Ports, statische IP, DNS, NAT vs Routing |
| R10 | Rate Limits, Quoten und Accounting | Idee | Bandbreite, Volumen, maximale Sitzungsdauer, parallele Flows, dauerhaftes Accounting; Telemetriezähler allein reichen nicht |
| R11 | Erweiterte Paketdatenbedienung | Idee; am 2026-10-03 einzelne Control-Befehle hinzugekommen | Kontext deaktivieren/modifizieren, Paging, END, Queue leeren, IP neu vergeben, sperren, Capture; je Aktion lokale Policy vs Air-Signalisierung trennen |
| R12 | Erweiterte Diagnoseanzeige | Idee, teilweise Snapshotbasis vorhanden | Dropursachen, Pagingversuche, Fragmentzähler, DNS-/NAT-/Routing-/Conntrack-Zustand; Datenschutz und Messkosten berücksichtigen |
| R13 | SNDCP-/WTP-Zustände und Wiederholungen vervollständigen | Teilweise implementierte Kernlogik; zusätzliche Prüfung offen | Request-/Response-Timer, Session-/Retransmission-Cache, ACK-/Abort-Semantik und Duplikate getrennt validieren |
| R14 | SDS-TL komplettieren | Weiterer Kandidat, keine komplette Abnahme in diesem Planungsstand | Segmentierung/Reassembly, persistentes Store-and-forward, Zustellberichte über Neustart, Retry/Expiry, Ende-zu-Ende-IDs und Duplicate Detection |
| R15 | SDS-Adressierungs-/Binärdienste | Idee | Externe Nummern/TSI-/DM-MS-Adressierung, Priorität, große Binärdaten/Dateiübertragung, API/Webhooks und Message History |
| R16 | Standardgerechter WAP Push / Service Indication | Idee beziehungsweise noch offener Teil von Legacy-Interoperabilität | Nicht mit roher WML-SDS-Ausgabe gleichsetzen; WDP/WSP/WBXML und Zielbrowser abstimmen |
| R17 | Parrot-/Echo-Einzelrufdienst | Im Vergleich vorgeschlagen, nicht als letzter Auftrag ausgewählt | Service-ISSI für Aufnahme/Wiedergabe, Audio-/Codec-/Latenztest; von EchoLink und anderen Echo-/Parrotdiensten unterscheiden |
| R18 | Native systemd-Readiness/Watchdog | Vorschlag, nicht in diesem Planungsstand vollständig nachgewiesen | `READY=1` erst nach realem RF-/MCCH-/Scheduler-/Dienststatus; `WATCHDOG=1` nicht bloß Prozesslebt-Signal |
| R19 | Debian-Paket, Migration, Rollback | Idee | `.deb`, Konfigurationsschema-Versionen, Capabilities-/Serviceinstallation, Upgrade und Rückfall ohne manuelles Löschen |
| R20 | TETRA-Authentifizierung/AIE | Großer Kandidat; nicht durch CHAP-Success oder ISSI-Whitelist abgedeckt | MS-/Infrastruktur-/gegenseitige Authentifizierung, Zustände, sichere Provisionierung, passende Algorithmen und Schlüsselablage |
| R21 | Schlüsselmanagement/OTAR/TSIM | Idee im Sicherheitsausbau | DCK/CCK/SCK/GCK, Security Classes, Rotation, Sperrverhalten und Audit; reale Endgeräte-/Lizenz-/Provenienzanforderungen prüfen |
| R22 | Mehrzellenbetrieb, Handover und Roaming | Weiterer großer Kandidat; nicht hier komplett umgesetzt | Nachbarzellen, Reselection, C1/C2, Cell Change, Call Restore und laufender Gruppenruf beim Zellwechsel |
| R23 | Paketdatenmobilität / gemeinsamer Core / ISI | Idee; am 2026-10-03 Packet-Core-Bausteine sichtbar | PDP-Kontext-/IP-/Bearerübergabe, gemeinsame Teilnehmer-/Gruppen-/Rufzustände, Mobility Anchor; lokal vs zentral eindeutig zuordnen |
| R24 | Supplementary Services | Breites Kandidatenfeld, keine pauschale Fehlendbehauptung für am 2026-10-03 | CLIP/CLIR/COLP/COLR, Forwarding/Waiting/Hold/Completion, Short Number, Area Selection, Access Priority und externe Nummern |
| R25 | Leitstellennahe Rufdienste | Besonders interessant im damaligen Vorschlag, nicht ausgewählt | CAD, Include Call, Late Entry, Call Retention, Priority/PPC, Barring, Ambience/Discreet Listening; Berechtigungen und Protokollumfang vor Umsetzung klären |
| R26 | TEDS/QAM/Link Adaptation | Nachgelagerte Idee | Augmented/Extended Channel Allocation, Bandbreiten/Modulation, PHY/LMAC/UMAC, SDR und kompatible MS gemeinsam entwickeln |
| R27 | IPv6-PDP und Mobile IPv4 | Niedriger priorisierte optionale Profile | Endgeräteunterstützung, Header-/MTU- und Mobilitätsmodell konkret prüfen; im bisherigen Profil abgelehnt |
| R28 | RFC-1144/RFC-2507 und DCOMP | Niedriger priorisiert | Kompressionskontexte, Aushandlung, Fehlerbehandlung und reale Gegenseite; nicht nur Negotiation-Bits setzen |
| R29 | Multicast-/Broadcast-Fan-out | Idee | Abbildungsmodell auf Funkbearer und Lastgrenzen definieren; keine pauschale Aussage, jede mögliche TETRA-Gruppendatenlösung müsse identisch funktionieren |
| R30 | Packet Core sicher integrieren | Zusätzlicher Prüfbefund vom 2026-10-03; keine historische Implementierung | `open_lab` isolieren; Kontext-/Adressvergabe, Edge-Korrelation und IP-Gateway abstimmen |
| R31 | Artefakt-/Wiki-Dateinamensbereinigung | Neuer geprüfter Auditpunkt | Vier Unicode-/`#U2010`-Aliaspaare und Links bewusst prüfen; Paketmanifest/Dateimengen reproduzierbar machen |
| R32 | Lizenz-/Provenienzprüfung und Vergleich erneut pinnen | Offen | Nexus-Lizenzgrenzen, eigene Implementierung/Attribution und tatsächliche Upstreamdifferenzen revisionsbezogen prüfen |

Die früheren Beispiel-TOMLs zu „internet“, „local-only“ oder „telemetry“-Teilnehmerprofilen waren Entwurfsbeispiele. Schlüssel wie `rate_limit_kbit` sind dadurch **nicht automatisch gültige Felder der aktuellen NetCore-Konfiguration**. Vor Übernahme Schema und vorhandene spätere Policy-Dienste prüfen.

## 18. Konkrete Fortsetzung und Priorität

### P0 – belastbaren Ausgangspunkt und Schutzgrenzen herstellen

Den tatsächlich eingesetzten Source-/Binary-Stand mit Commit, Rust/Cargo-Version, Features, Zielplattform, Unit-ExecStart und Konfigurationshash festhalten. Die ZIP-Kette dieser Planung nicht mit einem geprüften Deployment verwechseln. Den aktuellen Build gegen den aktuellen Code durchführen, nicht nur das historische R1-Ergebnis wiederverwenden. Alle Managementpfade, insbesondere `open_lab` des späteren Packet Core, nur im vorgesehenen abgeschotteten Umfeld betreiben.

### P1 – die drei zuletzt beauftragten Komponenten abnehmen

| Testblock | Mindestens zu belegen |
|---|---|
| Einzelner Paketdatenteilnehmer | ACTIVATE, Adresse/MTU/Timer, lokaler WAP-Pfad, ein definierter IP-Dienst, END und erneute Aktivierung |
| Mehrere Teilnehmer | Zwei unabhängige ISSIs, getrennte PDCH-Slots, gleiche/verschiedene Kontextfamilien, kein Slot-Doppelbesitz |
| Dual Carrier | Echte sekundäre Funkübertragung und korrekte Rückroute, nicht nur Carrieranzeige im Dashboard |
| Sprache parallel | Belegter CMCE/Brew-Slot wird nicht durch SNDCP vergeben; Headroom unter wechselnder Last korrekt |
| Grenzen/Abbruch | Volle Zelle, Paging-Timeout, Deregistrierung, T351/Kick, verlorenes END, Queueablauf und Slotwiederverwendung |
| IP-Gateway | Route, TUN, DNS-PCO, TCP/UDP/ICMP-Rückweg, NAT/routed getrennt, Host-INPUT-/FORWARD-Grenzen |
| Fragmente | MTUgrenze, DF, ICMP/PMTU-Verhalten, Out-of-order, Duplikat, Überlappung, Speicher-/Timeoutgrenze |
| Telemetrie/Control Room | Richtige Node-/ISSI-/NSAPI-Zuordnung, Counterrichtung, Snapshotalter, WebSocket/HTTP und native UI konsistent |
| Legacy-WAP | PID04/84, Längen-/Headergrenzen, kein doppelter PID, definierter Zielbrowser/Codeplug, tatsächliche Anzeige mit passendem Envelope |

Zu jedem Test Ergebnis, erwartete/erhaltene PDUs, Geräte-/Firmwareversion und Quellenrevision dokumentieren. Ein HTTP-200/„queued“ oder ein grünes Buildfenster ersetzt diese Beobachtungen nicht.

### P2 – aus den Befunden gezielt erweitern

Erst nach der Abnahme die weitergehenden Scheduler-/Policy-/Zustell- und Sicherheitsideen auswählen. Der Multi-PDCH-Pool braucht bei Fairness/Präemption eine eigene Zustands- und Prioritätsspezifikation. Legacy-WAP braucht vor weiteren UI-Effekten den Nachweis des richtigen Envelopes. Die ergänzende Packet-Core-Anbindung verlangt eine klar definierte Kontext-/IP-Autorität und kein zweites unkoordiniertes Adressmanagement.

### P3 – große optionale Protokollprogramme

Authentifizierung/AIE/OTAR, Mehrzellenübergaben, vollständige Supplementary Services und TEDS bleiben eigenständige Arbeitspakete. Eine ZIP-Auslieferung allein belegt deren Vollständigkeit nicht. Unmittelbarer Abnahmeschwerpunkt sind die drei zuletzt ausgewählten Komponenten.

## 19. Anhänge und Artefaktidentität

### 19.1 ZIP-Inventar, neu berechnete Werte

Die SHA-256-Werte identifizieren die am 2026-10-03 lesbaren Anhänge. Ihre Binärfassungen müssen separat erhalten bleiben; temporäre Downloadlinks ersetzen keinen dauerhaften Artefaktbestand.

| ZIP-Datei | Bytegröße | ZIP-Einträge inkl. Verzeichnissen | Reguläre Dateien | Rust / TOML |
|---|---:|---:|---:|---:|
| `netcore-tetra-wap.zip` | 1678369 | 748 | 619 | 444 / 19 |
| `netcore-tetra-wap-cleanroom-2026-07-21.zip` | 1707483 | 750 | 621 | 447 / 19 |
| `netcore-tetra-sndcp-complete-2026-07-21.zip` | 1770395 | 755 | 626 | 451 / 19 |
| `netcore-tetra-packet-data-complete-2026-07-21.zip` | 1791132 | 771 | 638 | 453 / 19 |
| `netcore-tetra-packet-data-complete-r1-2026-07-21.zip` | 1782420 | 772 | 639 | 453 / 19 |
| `netcore-tetra-packet-data-compile-fix-r1-files-2026-07-21.zip` | 36573 | 12 | 6 | 4 / 0 |
| `netcore-tetra-wap(1).zip` | 1775246 | 811 | 661 | 453 / 19 |
| `netcore-tetra-multi-pdch-dashboard-legacy-wap-2026-07-21.zip` | 1806030 | 810 | 660 | 454 / 19 |

```text
b0d4e57637b21f2ce029092f8942eec2cd3dbaec1cb484450561130e915c50d1  netcore-tetra-wap.zip
01585a224f907154cb5ff2be2d65e462e92ad748618aaffce977f2e4d58aa5cd  netcore-tetra-wap-cleanroom-2026-07-21.zip
a8af6fa93c739c20fef5857d72d080142878cbda5b65fc8c1c4484fe8613e6a2  netcore-tetra-sndcp-complete-2026-07-21.zip
f1a584977111e488cac3fa44ebc43af261f7f38af0394323aafc72bc1aa86640  netcore-tetra-packet-data-complete-2026-07-21.zip
f16cb4582b022d92104b261a351e750db0fa9c507bb9d9d16315b96761939305  netcore-tetra-packet-data-complete-r1-2026-07-21.zip
e4b18ca6f3d2718494114f1fa60ab19b379f117cae58f30ce62247e2f6302255  netcore-tetra-packet-data-compile-fix-r1-files-2026-07-21.zip
0355ffd207b32a5ea4fc80352504e591a23c7e163a1e05b5770ab54cff64067a  netcore-tetra-wap(1).zip
fa47f03166cba792c68c28dbea50f760ad251eb92a45b39c7b256f29872e7af6  netcore-tetra-multi-pdch-dashboard-legacy-wap-2026-07-21.zip
```

### 19.2 Patchkette

| Patch | Vorgesehene Basis |
|---|---|
| `netcore-tetra-wap-cleanroom-2026-07-21.patch` | Erster hochgeladener WAP-Arbeitsstand |
| `netcore-tetra-sndcp-complete-2026-07-21.patch` | Vorheriger WAP-Cleanroom-Stand |
| `netcore-tetra-packet-data-complete-2026-07-21.patch` | Vorheriger SNDCP-Complete-Stand |
| `netcore-tetra-packet-data-compile-fix-r1-2026-07-21.patch` | Vorheriger Packet-Data-Complete-Stand |
| `netcore-tetra-multi-pdch-dashboard-legacy-wap-2026-07-21.patch` | Weitergeführter Eingangsstand `netcore-tetra-wap(1).zip` |

Für den letzten Patch wurde SHA-256 `0dda62711709e1b4d094d5f0987a1591e687627ebf156954e87e72a0089302b9` gegen das Manifest bestätigt. Übrige Manifestreferenzen wurden ebenfalls geprüft; nur die letzte Patchkette wurde am 2026-10-03 neu angewendet und verglichen.

### 19.3 Begleitdokumente

Lesbare historische Markdown-Anhänge:

```text
WAP_INTEGRATION_2026-07-21.md
WAP_VALIDATION_2026-07-21.md
REPO_COMPARISON_2026-07-21.md
SNDCP_COMPLETE_INTEGRATION_2026-07-21.md
SNDCP_COVERAGE_2026-07-21.md
SNDCP_VALIDATION_2026-07-21.md
PACKET_DATA_COMPLETE_INTEGRATION_2026-07-21.md
PACKET_DATA_COVERAGE_2026-07-21.md
PACKET_DATA_SECURITY_2026-07-21.md
PACKET_DATA_VALIDATION_2026-07-21.md
PACKET_DATA_COMPILE_FIX_R1_2026-07-21.md
MULTI_PDCH_PACKET_DATA_LEGACY_WAP_2026-07-21.md
MULTI_PDCH_PACKET_DATA_LEGACY_WAP_VALIDATION_2026-07-21.md
```

Im finalen ZIP liegen Dokumente teilweise unter anderen Namen, insbesondere `Docs/WAP_INTEGRATION.md`, `Docs/SNDCP_COMPLETE.md`, `Docs/PACKET_DATA_GATEWAY_2026-07-21.md` und `Docs/wap-port-spec.md`. Beim Patchvergleich Dokumentnamen und Revision beachten. Die Begleittexte sind Entwicklungsberichte, keine unabhängigen Abnahmeprotokolle.

### 19.4 ETSI-Anhänge

| Datei | Verwendung / Einordnung |
|---|---|
| `en_30039202v030801p.pdf` | EN 300 392-2 V3.8.1, Air Interface, 1.445 Seiten; SNDCP Kapitel 28, SDS Kapitel 29, WAP 29.5.8 |
| `en_30039201v010601p.pdf` | Allgemeines TETRA-Netzdesign |
| `en_30039205v020701p.pdf` | PEI; relevant für Endgeräte-/Peripherieintegration |
| `en_30039207v030501p.pdf` | Security; Referenz für separat zu planende Authentifizierung/Schlüsselverwaltung |
| `en_30039209v010701p.pdf` | Allgemeine Supplementary-Service-Anforderungen |
| `en_3003921201v010202p.pdf` | Call Identification, Stage 3 |
| `en_3003921101v010201p.pdf` | Call Identification, Stage 2 |
| `en_3003921117v010102p.pdf` | Include Call, Stage 2 |
| `en_3003921114v010101p.pdf` | Late Entry, Stage 2 |
| `en_3003921006v010401p.pdf` | Call Authorized by Dispatcher, Stage 1 |
| `en_3003921018v010301p.pdf` | Barring of Outgoing Calls, Stage 1 |
| `en_3003921216v010400a.pdf` | PPC Stage 3, Draft V1.4.0 (2026-03); nicht als endgültig veröffentlichte Norm ausgeben |
| `en_3003920308v010401p.pdf` | ISI Generic Speech Format |
| `en_3003920304v010301p.pdf` | ANF-ISISDS |
| `en_3003920303v010301p.pdf` | ANF-ISIGC |
| `en_3003920313v010201p.pdf` | Transportlayerunabhängiges ANF-ISIGC |
| `en_3003920315v010500a.pdf` | Transportlayerunabhängiges ANF-ISIMM, Draft V1.5.0 (2026-04) |
| `en_30039401v030301p.pdf` | Radio-Conformance-Tests; Vorlage, kein durchgeführter Testnachweis |
| `en_30039502v010303p.pdf` | Full-Rate-TETRA-Codec |
| `ets_30039214e01v.pdf` | PICS-Proforma, Final Draft 1997; nicht automatisch PICS der eigenen Implementierung |
| `en_300812v020101p.pdf` | SIM-ME-Schnittstelle |
| `ts_10081201v020205p.pdf` | UICC physikalische/logische Eigenschaften |
| `es_20081201v020205p.pdf` | Entsprechende ES-UICC-Grundlage |
| `es_20081202v020401m.pdf` | TSIM-Anwendung, Final Draft |
| `ETSI.pdf` | 4.100-seitige Zusammenstellung; teilweise Überschneidung mit Einzeldateien |

Diese Dokumente waren als Anhänge verfügbar. Daraus folgt nicht, dass sämtliche darin spezifizierten Dienste in NetCore implementiert oder alle Tabellen in diesem Planungsstand geprüft wurden.

## 20. Revisionsfeste Repository-Quellen und Verweise

Die folgenden Links verweisen bewusst auf den geprüften Commit, nicht auf einen künftig beweglichen Branch. Sie belegen Dateiinhalt beziehungsweise konkrete aktuelle Teilbefunde, keinen Livebetrieb.

| Gegenstand | Quelle |
|---|---|
| Geprüfter `main` / PR-59-Merge | [Commit 7137e0dd](https://github.com/JanHG98/netcore-tetra/commit/7137e0dd69877e1b604bf89148fd8b6b590c1a97) |
| Geladener Archiv-Ausgangspunkt | [Commit fd6c77b1](https://github.com/JanHG98/netcore-tetra/commit/fd6c77b1eb4e1c656daf40ac1d4b406a2ca41249) |
| Dokumentations-/Branchvergleich | [Gepinnter Vergleich](https://github.com/JanHG98/netcore-tetra/compare/7137e0dd69877e1b604bf89148fd8b6b590c1a97...fd6c77b1eb4e1c656daf40ac1d4b406a2ca41249) |
| Gemeinsamer Allocator | [timeslot_alloc.rs](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/crates/tetra-core/src/timeslot_alloc.rs) |
| SNDCP, neuer Control-Pfad und Bearerpool | [sndcp_bs.rs](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/crates/tetra-entities/src/sndcp/sndcp_bs.rs) |
| IP-Primitiven / R1 | [ip.rs](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/crates/tetra-entities/src/sndcp/ip.rs) |
| Lokales TUN-/Hostgateway | [packet_gateway.rs](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/crates/tetra-entities/src/sndcp/packet_gateway.rs) |
| Konfigurationsvalidierung | [config.rs](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/crates/tetra-config/src/bluestation/config.rs) |
| Konfigurationstypen und Defaults | [sec_cell.rs](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/crates/tetra-config/src/bluestation/sec_cell.rs) |
| Legacy-WML und SDS-Hülle | [legacy_wap.rs](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/crates/tetra-entities/src/legacy_wap.rs) |
| Telemetrie-Datentypen | [events.rs](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/crates/tetra-entities/src/net_telemetry/events.rs) |
| Lokaler Snapshot | [Dashboard-State](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/crates/tetra-entities/src/net_dashboard/state.rs) |
| Ergänzende Asset-Einbindung | [html.rs](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/crates/tetra-entities/src/net_dashboard/html.rs) |
| Control-Room-API | [http.rs](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/bins/netcore-control-room/src/http.rs) |
| Operator-CLI | [Operator main.rs](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/system-backend/control-room/operator/src/main.rs) |
| Native UI | [UI main.rs](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/system-backend/control-room/ui/src/main.rs) |
| Neue Control-Zuordnung | [Control worker.rs](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/crates/tetra-entities/src/net_control/worker.rs) |
| Packet-Core-Aktionszuordnung | [Packet Core state.rs](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/system-backend/packet-core/src/state.rs) |
| Späterer Packet Core / open_lab | [Paket G](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/Docs/SWMI_CORE_1_PACKAGE_G_PACKET_CORE.md) |
| Übernommene historische Featurebeschreibung | [Multi-PDCH-Dokument](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/Docs/MULTI_PDCH_PACKET_DATA_LEGACY_WAP_2026-07-21.md) |
| Übernommene R1-Beschreibung | [R1-Dokument](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/Docs/PACKET_DATA_COMPILE_FIX_R1_2026-07-21.md) |
| Historischer Port-Vertrag | [wap-port-spec.md](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/Docs/wap-port-spec.md) |
| UI-Prüfgrundlage | [test_dashboard_ui.mjs](https://github.com/JanHG98/netcore-tetra/blob/7137e0dd69877e1b604bf89148fd8b6b590c1a97/tools/test_dashboard_ui.mjs) |

### 20.1 Historische Repository-Referenzen

Der damalige NetCore-Codeabruf verwendete unter anderem `41d925457bed3cc0f1207d509642f3e003303010` sowie einen früheren Suchtreffer auf `7c3aea85733eb9a9c8cb45e965a8cd172f93414a`. Sichtbare historische Commitmeldungen waren außerdem `e1b66f77aef402edc011b620d506e720f83be0a8` („Format WAP port before compiling“) und `1c11426e0695bf911cc1bf4385e68a5b4803fb4b` („Make WAP bootstrap observable from pull requests“).

Diese Referenzen dokumentieren die damalige Recherche. Sie wurden nicht als eindeutige Commitbasis jeder ZIP-Stufe nachgewiesen. Zwei frühere Git-Tree-Schreibversuche endeten sichtbar mit HTTP 422 („Invalid tree info“ beziehungsweise „GitRPC::BadObjectState“). Daraus ist kein erfolgreicher damaliger Commit/Push ableitbar. Die damaligen Lieferantworten beschrieben ausdrücklich Dateipakete statt eines direkt geänderten `main`.

Nexus wurde auf `ae234dd905629a2c0b6c8dd44f327aa21a1f8f97` betrachtet; relevante Dateien waren `README.md`, `crates/tetra-entities/src/sndcp/{mod,transfer,pdp,unitdata,wap_ip}.rs` und `crates/tetra-entities/src/net_control/commands.rs`. Die Pfade dienen als historische Vergleichsanker. Aktuelle Lizenz- und Upstreamstände vor einer neuen Portierung erneut prüfen.

### 20.2 Externe Primärreferenzen

- [Nexus-BS, historisch betrachtete Revision](https://github.com/invictus737/nexus-bs/tree/ae234dd905629a2c0b6c8dd44f327aa21a1f8f97)
- [BlueStation](https://github.com/MidnightBlueLabs/tetra-bluestation)
- [misadeks/flowstation](https://github.com/misadeks/flowstation)
- [ea5gvk/flowstation](https://github.com/ea5gvk/flowstation)
- [ETSI EN 300 392-2 V3.8.1](https://www.etsi.org/deliver/etsi_en/300300_300399/30039202/03.08.01_60/en_30039202v030801p.pdf), insbesondere Kapitel 28 und 29.5.8. Versionsgebundene Referenz, keine Behauptung, es sei die jüngste verfügbare Ausgabe.
- [Linux TUN/TAP-Dokumentation](https://docs.kernel.org/networking/tuntap.html): historische Architekturreferenz für den Raw-IP-Hostpfad.
- [RFC 791](https://www.rfc-editor.org/rfc/rfc791.html): historische IPv4-/Fragmentreferenz.
- [RFC 1877](https://www.rfc-editor.org/rfc/rfc1877.html): historische IPCP-DNS-Referenz.
- [RFC 3022](https://www.rfc-editor.org/rfc/rfc3022.html): historische NAT/NAPT-Referenz.
- [Cargo Book: cargo check](https://doc.rust-lang.org/cargo/commands/cargo-check.html): Abgrenzung Typcheck gegenüber abschließender Codegenerierung/Build.

### 20.3 Verwandte bereits vorhandene Archive

Für die Weiterarbeit sind besonders die Archive zu [DualCarrier-Ressourcen/ACK/Release](2026-10-03_flowstation-dualcarrier-bearer-ack-release-und-secondary-control.md), [Control Room, RBAC und Directory](2026-10-03_control-room-windows-ui-rbac-status-tableau-directory-api.md), [Dashboard/Wiki/DGNA](2026-10-03_basisstation-dashboard-deutsch-integrationen-wiki-dgna-gruppennamen.md) und [SDS-Services/Gateways/Bot-Architektur](2026-10-03_flowstation-sds-services-gateways-und-bot-architektur.md) relevant. Diese Links ersetzen nicht die eigene Code-/Testprüfung und übertragen keine Betriebsbestätigung aus einem anderen Thema auf den hier behandelten Paketdatenpfad.

## 21. Arbeitsstand und nächster Schwerpunkt

Entscheidungen, Lieferstände, bestätigte R1-Reparatur und ergänzender Repository-Abgleich bleiben getrennt. Nächster Schwerpunkt ist die reproduzierbare Abnahme der drei letzten Komponenten auf einem eindeutig identifizierten Produktstand.

Offen bleiben insbesondere Endgeräte-Interoperabilität, Parallelbetrieb, tatsächlich installierte Fassung und aktive Konfiguration.
