# Brew Server Integration: Protokolladapter, Core-Zuständigkeiten und Repository-Abgleich

## 1. Metadaten und Lesart

| Feld | Wert |
|---|---|
| Projekt | NetCore-TETRA, `JanHG98/netcore-tetra` |
| Ursprünglicher Chattitel | **Brew Server Integration** |
| Ursprungsdialog | [ChatGPT-Chat](https://chatgpt.com/c/6aae887e-08b0-83eb-8a9d-1b3bfb0401ed), ID `6aae887e-08b0-83eb-8a9d-1b3bfb0401ed` |
| App-Verweis | `chatgpt-conversation://6aae887e-08b0-83eb-8a9d-1b3bfb0401ed` |
| Zugängliche Gesprächszeitpunkte | 19.09.2026, 15:05 und 15:07 Uhr; 05.10.2026, 23:25 und 23:26 Uhr, jeweils Europe/Berlin |
| Erstellung dieser Abschlussdokumentation | **2026-10-05**, Europe/Berlin |
| Zielbranch | Bestehender Branch **`Archiving`** |
| Geprüfter Repository-Stand | [`7eb95f9aff36c25d26da4978da9c1fe77ee0a72d`](https://github.com/JanHG98/netcore-tetra/tree/7eb95f9aff36c25d26da4978da9c1fe77ee0a72d), vor dem Archivcommit |
| Historisch diskutierter Stand | Branchname `mqtt`, fest referenzierter Commit [`c128a83cd3eef463de1e7e0c909af1fff4b84b36`](https://github.com/JanHG98/netcore-tetra/commit/c128a83cd3eef463de1e7e0c909af1fff4b84b36) |
| Historischer Vergleichspunkt | `v1.8.0` → `f0a4ae39ba8d7d54597732de09bf2025ba45b403` |
| Zusätzlich abgefragter Upstream | [`ysamouhos/brew-server` bei `b9ada00097d608e5ebd3f92c89857b9efff7f9ae`](https://github.com/ysamouhos/brew-server/tree/b9ada00097d608e5ebd3f92c89857b9efff7f9ae) |
| Umfang des aktuellen Auftrags | Dokumentation und Index ausschließlich unter `Docs/archive/`; Veröffentlichung auf `Archiving` ohne Force-Push oder Merge |

Der **geprüfte Repository-Stand** ist die fachliche Quellenbasis und nicht der Commit, der dieses Dokument anschließend veröffentlicht. Der tatsächliche Archivcommit ist über die Dateihistorie und die Abschlussmeldung der Fortsetzung nachzuvollziehen. Eine eigene Commit-ID wird nicht vor ihrer Erstellung erfunden.

Dieses Dokument unterscheidet ausdrücklich:

- **Idee:** diskutierte Möglichkeit ohne Umsetzungsentscheidung.
- **Beschlossen/geplant:** verbindlicher Archivauftrag beziehungsweise als solche gekennzeichnete abschließende Architektur-Empfehlung; eine Empfehlung des Assistenten ist noch kein Implementierungsauftrag des Nutzers.
- **Implementiert:** im angegebenen Commit durch Quelltext oder Konfiguration belegt.
- **Getestet:** ein tatsächlich berichteter oder in dieser Fortsetzung ausgeführter Test, mit seiner jeweiligen Grenze.
- **Im Betrieb bestätigt:** konkrete Beobachtung im Testnetz; hier nur die ausdrücklich dem Nutzerbericht zugeordneten Telefonieversuche, keine heutige unabhängige Betriebsabnahme.

## 2. Ergebnis und wichtigste Korrekturen

Das abschließende fachliche Ergebnis des Chats lautet: **NetCore soll seine eigenen Core-Zuständigkeiten behalten. `brew-server` ist als Upstream-Referenz für Protokoll, Interoperabilität und ausgewählte Medienbausteine interessant. Ein zusätzlicher, gleichzeitig für dieselben Rufe, Teilnehmer und Gruppen zuständiger Brew-Core wurde zuletzt ausdrücklich nicht empfohlen.**

Ein späterer Brew-Adapter kann fremde oder virtuelle Basisstationen anbinden. Er soll Nachrichten in vorhandene NetCore-Schnittstellen übersetzen und keine konkurrierende Rufsteuerung, Teilnehmerverwaltung, Gruppenverwaltung, SDS-Routinginstanz oder SIP-Vermittlung etablieren. Ob dafür ein neuer Dienst erforderlich ist, blieb offen.

Der Repository-Abgleich liefert eine wesentliche Ergänzung zum Gespräch: **Ein separater Rust-Brew-Server war bereits im diskutierten Commit `c128a83...` vorhanden**, dort mit Paketversion `0.5.0`. Im geprüften heutigen `Archiving`-Stand liegt er weiterhin unter `misc/brew-server/`, inzwischen mit Paketversion `1.9.0`. Er besitzt tatsächlich eigene Teilnehmer-, Gruppen-, Ruf-, Floor-, SDS- und SIP-Zustände. Seine bloße Ablage im Repository beweist weder seine aktive Verwendung noch eine Integration als reiner NetCore-Adapter.

Deshalb sind drei Dinge auseinanderzuhalten: die letzte Empfehlung dieses Chats, der vorhandene eigenständige Server und eine noch zu prüfende gemeinsame Betriebsarchitektur. Aus diesem Archiv folgt weder ein Auftrag, den vorhandenen Server zu entfernen, noch die Bestätigung, dass eine konfliktfreie Integration bereits umgesetzt wäre.

## 3. Quellenumfang und Auswertungslücken

Der Ursprungsdialog wurde mit `read_thread`, `turnLimit=10` und `maxOutputCharsPerItem=20000` gelesen. Zurückgegeben wurden **vier Gesprächsrunden mit vier Nutzernachrichten und drei Assistentenantworten**. `hasMore=false` und `nextCursor=null` boten keine weitere Seite an. Alle zurückgegebenen technischen Nachrichten lagen unter dem Zeichenlimit und wurden ausgewertet. Der verkürzte Vorschautext war nicht die alleinige Grundlage.

Die vier Runden umfassen die ursprüngliche Integrationsfrage, den Einwand gegen doppelte Dienstzuständigkeiten, die vom Nutzer eingebrachte `mqtt`-Releasebeschreibung mit anschließender korrigierter Empfehlung und den Archivauftrag. Hinzu kommt der aktuelle Auftrag zur vollständigen Speicherung und Verifikation auf `Archiving`.

Folgende Grenzen bleiben bestehen:

- Der Zugriff belegt den vollständig **zurückgegebenen** Verlauf. Nicht zurückgegebene Alternativantworten, gelöschte Nachrichten oder interne historische Werkzeugausgaben lassen sich daraus nicht rekonstruieren.
- Historische Antworten enthalten `chatgpt-content-reference`-Marker. Deren vollständige damalige Quellenzuordnung wurde nicht mitgeliefert. Aussagen über BlueStation, FlowStation, Nexus-BS und ETSI-Dokumente werden deshalb als Gesprächsinhalt erhalten und nicht als hier vollständig nachgeprüfte Fremdimplementierungen oder Normnachweise ausgegeben.
- `attachments=[]`; in den zurückgegebenen Nachrichten waren keine technisch zugänglichen Bilddateien oder Bildverweise enthalten. **Es wurden keine Chatbilder hochgeladen.** Eventuell außerhalb dieses Abrufs vorhandene Originalbilder bleiben eine ausdrücklich benannte Lücke. Die im Chat enthaltenen Textdiagramme sind keine Bildanhänge.
- Keine Betriebslogs, Paketmitschnitte, Audioaufzeichnungen oder Geräteabbilder begleiten die berichteten Telefonietests. Keine reale TBS, PBX oder LXC wurde für diesen Archivauftrag kontaktiert oder verändert.
- Die schreibgeschützten synchronisierten Projektquellen wurden nicht verändert. Fremde Bilder oder andere Chats wurden nicht als Ersatzmaterial für diesen Chat übernommen.

## 4. Ziel, Ausgangslage und Verlauf der Architekturentscheidung

### 4.1 Ursprüngliche Frage und erste Empfehlung

Der Nutzer fragte, ob sich [ysamouhos/brew-server](https://github.com/ysamouhos/brew-server) in NetCore-Tetra einpflegen lasse und ob das sinnvoll sei. Die erste Antwort bewertete das Projekt sehr positiv als möglichen zusätzlichen Core-/Gateway-Baustein. Genannt wurden Brew v1, Multi-BS, Registrierung, Gruppen- und Individualrufe, SDS, LIP/Positionsdaten, Telemetrie, BS-Control und SIP einschließlich ACELP↔G.711.

Schon diese Antwort wollte NetCore-Dienste nicht durch den fremden Core ersetzen. Sie schlug einen eigenen `brew-gateway` vor, der Subscriber-, Mobility-, Group-, Call-, Media-, SDS- und Observability-Dienste nutzt. Die Formulierung, den Server als eigenen Dienst zu integrieren, war jedoch weitergehend als die später festgehaltene Richtung und ist **durch die folgenden Korrekturen überholt**.

Als langfristiger Nutzen wurden eine gemeinsame Southbound-Schnittstelle für NetCore-TBS, BlueStation, FlowStation, Nexus-BS und virtuelle TBS sowie dieselben Core-Tests gegen reale und simulierte Basisstationen genannt. Die erwähnten Ortsnamen für ein mögliches Multi-Site-Netz waren illustrative Beispiele, keine bestätigte Standortinventur.

### 4.2 Einwand des Nutzers: doppelte Zuständigkeiten

Der Nutzer wies ausdrücklich darauf hin, dass bereits viele Dienste auf Call Control und die bestehende Verteilung ausgerichtet seien. Daraufhin wurde die Empfehlung eingeengt: **keinen weiteren Core mit eigener fachlicher Wahrheit danebenstellen**, sondern gezielt Komponenten und Erkenntnisse übernehmen.

Als Konfliktbeispiel diente ein Gruppenruf: Zwei Instanzen könnten gleichzeitig Rufzustand, Floor, Release, Late Entry, Ziel-TBS und Ressourcenfreigabe entscheiden. Das Gespräch beschrieb als mögliche Folge, dass die Leitstelle einen Ruf als beendet anzeigt, während Media Switch weiterleitet und eine zweite TBS noch einen Slot belegt. **Das war ein hypothetisches Fehlerszenario, kein belegter Fehlerbericht aus Jans Netz.**

Die zweite Antwort stellte außerdem infrage, ob ein neuer Container überhaupt nötig sei. Zuerst sollten vorhandener TBS-Brew-Pfad und NetCore-Backhaul gegen Upstream verglichen werden.

### 4.3 Letzte fachliche Antwort nach der `mqtt`-Releasebeschreibung

Nach dem vom Nutzer gelieferten Stand `c128a83...` wurde die Empfehlung nochmals ausdrücklich festgelegt:

1. `brew-server` nicht als zusätzliche laufende Core-Autorität in NetCore integrieren.
2. Upstream als Referenz verfolgen und geeignete Implementierungen gezielt portieren.
3. Brew bei Bedarf als Schnittstelle zu Fremdsystemen am Rand der Architektur einsetzen.
4. Eine Kompatibilitätsmatrix erstellen, statt pauschal Serverfunktionen zu übernehmen.
5. `brew-server` als sechstes Vergleichsrepository neben `flowstation`, `tetra-bluestation`, `nexus-bs`, `nexus-bs2` und `bost-flowstation` aufnehmen.

**Status:** letzte Architektur-Empfehlung des Assistenten, vom späteren Archivauftrag nicht durch einen Implementierungsauftrag ergänzt. Es gibt in diesem Chat keinen nachgewiesenen neuen Adapter, keine eingerichtete Vergleichsautomation und keinen dadurch entstandenen Feature-PR.

## 5. Vom Nutzer eingebrachtes `mqtt`-Releasebild

Die folgende Übersicht erhält die Releasebeschreibung als **historische Nutzeraussage über den damaligen Beta-/Vorabstand**. Sie beschreibt keine in diesem Archivauftrag neu entwickelten Funktionen.

| Thema | Im Chat angegebener Stand |
|---|---|
| MQTT/IoT | IoT-Gateway mit gemeinsamem Ereignismodell, Befehlen, Quittierungen und konfigurierbaren Freigaberegeln |
| Home Assistant/Homematic | MQTT Discovery, Übernahme ausgewählter Zustände, optionale CCU3-/RaspberryMatic-Anbindung über XML-RPC, Beispiele und Installationshilfen |
| Mobility und zentrale Rufe | Erweiterte Teilnehmer-Routenauflösung, zentrale Rufkommandos und begrenzte TETRA-Sprachframe-Brücke für MAIN-COMPAT |
| Zentraler SIP-Switch | Vermittlung zwischen bestehender PBX und TBS, Zielauflösung über die vom Mobility Core gemeldete Serving-TBS, numerische und `T`-präfixierte Ziele |
| Lokaler SIP-Fallback | Überwachter Wechsel zwischen zentralem Weg und direkter PBX-Anbindung; eine aktiv gehaltene externe Registrierung pro TBS |
| Anruferkennung und Rufende | Erhalt der ISSI auf dem Telefoniepfad, überarbeitete eingehende Rufnummernsignalisierung und SIP-/Q.850-Ursachenübersetzung |
| RF/DSP und Diagnose | Korrekturen an DQPSK-Phasenanpassung, TX-Trägerzuordnung, Signalkontinuität und TETRA-Netzzeit; synchronisierte Dienstinformationen und zusätzliche Diagnosewerte |
| Lastverhalten | Recording-I/O und SIP-Medienverarbeitung außerhalb des zeitkritischen Funkpfads, begrenzte Netzwerkverarbeitung, priorisierte Zugriffsbestätigungen und Grants |
| Gruppenrufe | Hangtime-Signalisierung auf dem zweiten Träger, Sprecherfreigabe und Rufabbau durch den ursprünglichen Rufbesitzer |
| Verwaltung | Hardware-/RF-Monitoring, Alarm- und Aufgabenabläufe, Asset-Verwaltung, Weboberflächen und WAP-Formulare |
| Installation/Tests | Erweiterte Installations-, Update- und Reparaturwerkzeuge; Schutz vor kollidierenden SIP-Rollen; Trixie-Asterisk-Hilfe; zusätzliche Regressionstests und CI-Prüfungen |

**Im damaligen Testbetrieb laut Nutzer bestätigt:** Funkgerät↔Telefon in beiden Richtungen; Anzeige der Funkgeräte-ISSI am Telefon; eingehende Anruferanzeige am **Sepura SC20 mit V10.24** bei passend gesetzter **FreePBX-Outbound-CID**. Der Chat liefert dafür keine Rohprotokolle oder erneute unabhängige Reproduktion. Diese Beobachtungen belegen insbesondere keine Brew-Fremd-BS-Interoperabilität.

Ausdrücklich genannte Grenzen: Beta, kein vollständiges Handover laufender Gespräche zwischen Zellen, SIP weiter über den lokalen Medien-/Codec-Pfad, keine Migration laufender SIP-Dialoge bei Fallback, OPEN LAB für isolierte Testnetze. Zentraler SIP-Switch und TBS-Fallback gehören auf getrennte Hostrollen. Konfiguration und Dienstzustand sind vor Umstellungen zu sichern; TBS und Backend benötigen zusammenpassende Versionen für den zentralen Medienpfad.

Die Aussage **141 zusätzliche Commits gegenüber `v1.8.0`** wurde jetzt durch Git bestätigt. Der Endcommit trägt die Nachricht `Merge pull request #48 from JanHG98/mqtt-fix/sip-cli-full-tsi`; dies ist die Identifikation eines historischen Commits, kein in diesem Archivauftrag ausgeführter Merge und kein hier ausgewerteter vollständiger PR-/CI-Verlauf.

## 6. Zuständigkeitsmodell und Schnittstellengrenzen

### 6.1 Fachliche Eigentümer

| Bereich | Im Gespräch vorgesehene NetCore-Zuständigkeit | Grenze eines künftigen Brew-Adapters |
|---|---|---|
| Teilnehmerstammdaten und Zulassung | Subscriber Core | Keine zweite authoritative Teilnehmerverwaltung |
| Registrierung, Aufenthaltsort, Serving-TBS | Mobility Core, mit Teilnehmer-/Gruppenlage | Registrierung übersetzen und melden, keine widersprüchliche Mobility-Wahrheit erzeugen |
| Gruppen und Affiliationen | Group Core | Gruppenereignisse übertragen, keine konkurrierende Gruppenpolitik |
| Netzweite logische Rufe | Call Control | Setup/Grant/Release übersetzen, keinen zweiten netzweiten Rufautomaten betreiben |
| Lokale Funkprozeduren | TBS-CMCE/UMAC | Lokale Funkressourcen und zeitkritische Prozeduren bleiben Aufgabe der TBS |
| Medienweiterleitung | Media Switch und zugeordnete TBS-Rufzweige | Medien zuordnen und transportieren; keine unabhängige Freigabe trotz beendeten Calls |
| SDS | SDS Router; nachgelagerte Anwendungen | Meldungen und Reports übertragen, keine eigene fachliche Zielentscheidung parallel zum Router |
| SIP/PBX | SIP-Switch, lokaler TBS-Asterisk und vorhandene PBX | Keinen weiteren SIP-Master für dieselben Rufe einsetzen |
| Telemetrie/Bedienung | Observability/Control Room und vorhandener Kommandopfad | Status und unterstützte Befehle abbilden |

Die Chatformel „Call Control ist die einzige Call-Wahrheit“ ist zu präzisieren: Im heutigen Repository besitzt Call Control die **netzweiten logischen Calls und deren Koordination**. Lokale CMCE-, Funkkanal- und Floor-Prozeduren verbleiben ausdrücklich auf der TBS. Ein lokaler Rufzustand ist damit nicht automatisch eine unzulässige zweite netzweite Autorität.

### 6.2 Zielbild aus der Diskussion

```text
Eigene NetCore-TBS                 Fremde / virtuelle Brew-TBS
        |                                      |
        | NetCore-Backhaul                      | Brew
        |                                      v
        |                         optionaler Protokolladapter
        |                                      |
        +------------ vorhandene NetCore-Verträge --------+
                             |
           Node Gateway / normalisierter Transport
                             |
        +--------------------+--------------------+
        |                    |                    |
 Mobility/Subscriber    Call Control         Group/SDS
                             |
                        Media Switch

SIP separat nach vorhandener Rollenverteilung:
Funkgerät -> TBS-Codec-Bridge -> lokaler Asterisk
          -> zentraler SIP-Switch -> bestehende PBX
```

**Status des Adapterzweigs: Idee/geplante Option.** Das Diagramm ist kein Nachweis eines vorhandenen Adapters oder einer bereits festgelegten Deployment-Topologie.

„Keine eigene Business-Logik“ bedeutet in der Diskussion keine konkurrierende fachliche Autorität. Für eine spätere Umsetzung wären transportbezogene Session-, UUID-, Sequenz- und Pufferzuordnungen trotzdem zu definieren. Diese technische Präzisierung ist eine aus dem Zielbild abgeleitete Aufgabe, kein im Chat bereits implementierter Vertrag.

### 6.3 Illustrative Chat-API gegenüber echter API

Das Gespräch verwendete ein absichtlich sinngemäßes Beispiel `GROUP_CALL_REQUEST`, GSSI `1001`, ISSI `4010001`, `source_bs=TBS-01`, gefolgt von `POST /calls` mit Feldern `type`, `group`, `originator` und `source_bs`. Auch `CALL_GRANT` war dort eine illustrative Rückmeldung. **Diese Schreibweisen sind keine hier verifizierte Brew-Wire-Spezifikation und kein kopierfähiger NetCore-API-Vertrag.**

Der heutige Quelltext definiert stattdessen unter anderem:

- `POST /api/v1/calls/group`, Eingabetyp `GroupCallInput`: `gssi`, `source_issi`, `priority`, `target_nodes`.
- `POST /api/v1/calls/individual`, `IndividualCallInput`: `calling_issi`, `called_issi`, `simplex`, `priority`, optional `target_node`.
- `POST /api/v1/calls/{logical_call_id}/release` sowie Floor-Anforderung und -Freigabe unter dem jeweiligen Call.
- `POST /api/v1/media/route-ready` und `WS /ws/media` für den revisionsgebundenen Medienabgleich.
- Node Gateway: `WS /ws/node`, `WS /ws/backend`, `POST /api/v1/nodes/{id}/commands`.

Das Vorhandensein dieser Endpunkte ersetzt keine definierte Übersetzung eines funkseitig ausgelösten Brew-Ereignisses. Vor einer Umsetzung sind Ereignisrichtung, Idempotenz, Call-ID-/UUID-Zuordnung und Kommandorückmeldungen festzulegen.

## 7. Historischer Codeabgleich am exakt genannten Commit

Die historischen Objekte wurden unabhängig vom heutigen Branch geladen und direkt geprüft:

| Prüfung | Ergebnis |
|---|---|
| `v1.8.0` auflösen | `f0a4ae39ba8d7d54597732de09bf2025ba45b403` |
| `git rev-list --count v1.8.0..c128a83...` | **141** |
| Diff von `central_control.rs` gegen `v1.8.0` | **483 hinzugefügte, 0 entfernte Zeilen**; die Chatangabe „rund 480“ ist bestätigt |
| `system-backend/call-control/` | Bereits vorhanden |
| `crates/tetra-entities/src/net_brew/` | Bereits vorhanden |
| `misc/brew-server.py` | Bereits vorhanden |
| `misc/brew-server/` | Bereits vorhanden, `Cargo.toml` nennt **0.5.0** |

Die damalige Rust-Server-README beschrieb bereits eigene Registrierung/Affiliation, Gruppenrouting mit Prioritätsübernahme, SDS-Routing, experimentelle Private-/Simplex-Routen, Telemetrie, Control und getrenntes Dashboard. Die erste Antwort sprach dagegen über einen weitergehenden Upstream-Stand einschließlich Version-1.0-Codec/SIP-Funktionen. **Diese Upstream-Aussage darf nicht rückwirkend als Funktionsumfang der damals eingebetteten Version 0.5.0 gelesen werden.**

Die historische [CENTRAL_NETWORK_ROLLOUT.md](https://github.com/JanHG98/netcore-tetra/blob/c128a83cd3eef463de1e7e0c909af1fff4b84b36/Docs/CENTRAL_NETWORK_ROLLOUT.md) beschreibt präziser:

- vorhandene PBX und lokalen TBS-Codecpfad; getrennten TETRA-Framepfad `TBS ↔ Node Gateway ↔ Media Switch` unter Call Control;
- begrenzte, nicht blockierende TBS-Kanäle, höchstens 16 Downlink-Frames und je acht Steuerkommandos pro Control-Endpunkt und Tick;
- Downlink-Zuordnung über die Operation-ID des **Zielrufzweigs**, Annahme nur am aktiv gebundenen Traffic-Slot, sofortiges Entfernen der Zuordnung beim Schließen;
- keine falsche Capability-Ankündigung für nicht in MAIN-COMPAT integrierte Restore-/Policy-Funktionen;
- `edge_media`, keine zentrale SIP-Transcodierung und kein Verschieben laufender SIP-Dialoge;
- Backend-/TBS-Versionen müssen zusammenpassen; zuerst Call Control/Media Switch, danach TBS; vor Neustarts laufende Testgespräche beenden.

Diese Parameter wurden aus dem historischen Dokument gelesen, nicht unter Last oder an Funkhardware nachgemessen.

## 8. Zusätzlich geprüfter heutiger Repository-Stand

### 8.1 NetCore-Backhaul, Call Control und Medien

Die Root-Workspace-Liste enthält Node Gateway, Mobility Core, Subscriber Core, Group Core, Call Control, Media Switch, SDS Router, Recorder, Transit, Observability, Application Gateway und IoT Gateway. Der separate Rust-Brew-Server hat eine eigene `Cargo.toml` mit eigenem `[workspace]` und ist darin als eigenständig gebautes Legacy-Paket gekennzeichnet. `system-backend/services.toml` und das Open-Lab-Inventory führen ihn nicht als regulären Core-Dienst auf.

`central_control.rs` passt zentrale Kommandos an die lokale MAIN-COMPAT-CMCE an. Operation-UUIDs werden geprüft und vorhandenen Rufzweigen zugeordnet; Gruppen-/Individualrufe, Release und Floor-Operationen greifen auf lokale Rufverfahren zurück. Das ist eine konkrete Implementierungsgrundlage, jedoch kein Beleg für vollständigen Handover oder fremde Brew-TBS.

Call Control und Media Switch dokumentieren heute einen ereignisgetriebenen Abgleich über `ws://<call-control>:8120/ws/media`, Subprotokoll `netcore-call-control-media-v1`. Die Ereignisse `call_created`, `leg_ready`, `floor_changed`, `call_updated`, `call_released` enthalten revisionsbehaftete Snapshots. Ein Operator-Floor setzt aktive lokale Call-IDs/Slots und ein passendes RouteReady-ACK voraus. HTTP bleibt ein Fallback.

Media Switch transportiert gepackte **35-Byte-TETRA-ACELP-Frames**, dokumentiert einen adaptiven Jitterpuffer mit **1–12 Frames**, Startwert **2**, sowie **5 Frames** Kaltstart-Vorpuffer. Der Recorder liest einen begrenzten Replay-Tap asynchron; das bestätigt keinen zentralen ACELP↔G.711-SIP-Transcoder in diesem Dienst.

### 8.2 Separater Brew-Server ist tatsächlich ein zustandsführender Server

Die geprüfte Paketversion lautet **1.9.0**. Sie ist nicht mit einer bestätigten Installation oder einem nachgewiesenen exakten Upstream-Importcommit gleichzusetzen. Der lokale Build verwendet Rust 2021, unter anderem Tokio/Axum und einen durch `build.rs` mit einem C-Compiler gebauten Codec unter `third_party/tetra-codec/`.

Konkrete Codebefunde:

- `src/state.rs` enthält `subscribers`, `group_clients`, `calls`, `group_floor`, `sds_routes` und optionale SIP-Handles.
- `src/router.rs` bearbeitet `SUB_REGISTER`, `SUB_AFFILIATE`, Gruppenrufe, SDS und Weiterleitung an die eigene SIP-Brücke.
- `src/sip/` besitzt eigene Signalisierungs-, Routing-, Dialog- und Medienbausteine.
- `src/transcode/` enthält ACELP-, G.711- und Transcoding-Tasks.
- `src/position.rs`, `src/aprs.rs`, `src/store.rs` und `src/federation.rs` sind vorhanden.
- In den untersuchten Serverquellen wurde kein Anschluss an NetCore-Endpunkte wie `/api/v1/calls` oder `/ws/backend` gefunden. Worttreffer auf „call-control“ betreffen dort unter anderem Brew-Protokollnachrichten und beweisen keinen Anschluss an den NetCore-Dienst.

Damit ist die Sorge vor überlappenden Zuständigkeiten durch Code plausibel. **Nicht belegt** sind ein tatsächlich aufgetretener Konflikt, der aktuelle Deploymentmodus oder eine bereits vorgenommene Trennung der jeweiligen Rufdomänen.

### 8.3 Weitere Präzisierungen gegenüber dem Gespräch

| Gesprächsaussage/Skizze | Heutiger Befund und Konsequenz |
|---|---|
| „transit / vorhandener Gateway“ als möglicher TBS-Einstieg | Node Gateway ist der dokumentierte TBS-/Backend-Einstieg. Transit vermittelt zwischen Core-Regionen über `netcore-transit-v1`; seine Existenz belegt keinen Brew-Adapter. |
| LIP als zukünftiger Location-Baustein | `sds_bs.rs` enthält bereits PID-`0x0A`-/LIP-Positionsdekodierung; im separaten Server liegen weitere Position-/APRS-Bausteine. Ein kompletter gemeinsam genutzter Location-Service ist daraus nicht abzuleiten. |
| `net_brew` sei vorhanden | Bestätigt; Parser-/Builder-Tests für v0/v1, Mnemonic, Sprachframe, SHORT_TRANSFER, SDS, GROUP_IDLE und SDS_REPORT liegen vor. Nicht gleichbedeutend mit geprüfter Interoperabilität aller genannten Fremdsysteme. |
| SIP dürfe keinen zweiten Master erhalten | Der reguläre Weg ist weiterhin zentraler SIP-Switch plus lokale Edge-Rolle. Der separate Brew-Server bringt zusätzlich eigene SIP-Funktionen mit; Aktivierung und Konfliktfreiheit wurden nicht live geprüft. |
| Sechstes Vergleichsrepo/Kompatibilitätsmatrix dauerhaft aufnehmen | Im untersuchten `.github`-, `tools`- und relevanten Dokumentationsumfang wurde keine diesem Chat eindeutig zuordenbare Brew-Vergleichsmatrix oder eingerichtete Upstream-Automation gefunden. Das ist ein begrenzter Suchbefund, keine Aussage über alle privaten/externalen Arbeitsabläufe. |

### 8.4 Upstream-Schnappschuss

Der anonym gelesene Upstream-HEAD war `b9ada00097d608e5ebd3f92c89857b9efff7f9ae`. Dessen [Cargo.toml](https://github.com/ysamouhos/brew-server/blob/b9ada00097d608e5ebd3f92c89857b9efff7f9ae/Cargo.toml) nennt **1.14.0**, während NetCores eingebettetes Paket **1.9.0** nennt. Die [README an diesem Commit](https://github.com/ysamouhos/brew-server/blob/b9ada00097d608e5ebd3f92c89857b9efff7f9ae/README.md) verweist inzwischen auch auf einen Active-/Standby-HA-Modus. Das ist ein zusätzlicher heutiger Quellenbefund; HA wurde im ursprünglichen Gespräch nicht als NetCore-Anforderung beschlossen und hier nicht erprobt.

Dieser Abgleich prüft Metadaten und ausgewählte Dokumentation. Er ist **kein vollständiger Source-Diff**, kein automatisches Upgrade und keine Empfehlung, den neueren Server als weiteren Core zu starten. Die Versionsdifferenz begründet lediglich einen offenen, auf feste Commits zu stützenden Vergleich.

## 9. Relevante Dateien, Dienste, Ports und Protokolle

Die Ports sind geprüfte **Beispiel-/Defaultwerte im Repository**, keine Feststellung aktuell erreichbarer Listener.

| Komponente | Dateien/Anknüpfungspunkte | Port/Vertrag |
|---|---|---|
| TBS-Brew | `crates/tetra-entities/src/net_brew/{protocol,worker,entity}.rs` | Brew über WebSocket; konkrete Gegenstelle konfigurationsabhängig |
| Lokale zentrale Rufanbindung | `crates/tetra-entities/src/cmce/subentities/cc_bs/central_control.rs` | Operation-UUID, lokale Call-ID, Timeslot, Floor |
| Node Gateway | `system-backend/node-gateway/` | TCP 8080; `/ws/node`, `/ws/backend`, `/api/v1` |
| Mobility Core | `system-backend/mobility-core/` | TCP 8090; Serving-TBS-/Teilnehmerlage |
| Subscriber Core | `system-backend/subscriber-core/` | TCP 8100 |
| Group Core | `system-backend/group-core/` | TCP 8110 |
| Call Control | `system-backend/call-control/src/{http,state,media_ws}.rs` | TCP 8120; `/api/v1/calls/*`, `/ws/media` |
| Media Switch | `system-backend/media-switch/` | TCP 8130; codierte Sprachframes, RouteReady, Recorder-Tap |
| SDS Router | `system-backend/sds-router/` | TCP 8150 |
| Transit | `system-backend/transit/` | TCP 8200; `netcore-transit-v1`, regionale Peer-/Delivery-Verträge |
| Observability | `system-backend/observability/` | TCP 8210 |
| IoT Gateway | `system-backend/iot-gateway/` | TCP 8240; gemeinsames NetCore-Ereignismodell/MQTT-Anbindung |
| SIP-Switch | `system-backend/sip-switch/` | HTTP TCP 8300, SIP-Beispiel UDP 5060, `edge_media` |
| TBS-SIP-Fallback | `system-backend/sip-switch/tbs-fallback/` | Native Bridge zum lokalen Asterisk `127.0.0.1:5060`; externe Registrierung wechselt |
| Separater Rust-Brew-Server | `misc/brew-server/`, eigene TOML/Cargo-/Docker-Dateien | TCP 9000, `/brew` bzw. `/brew/`, `/healthz`, Subprotokoll `brew` |
| Brew-Telemetrie | `misc/brew-server/src/telemetry.rs` | TCP 9001, `bluestation-telemetry-v2` |
| Brew-Control | `misc/brew-server/src/control.rs` | TCP 9002, `bluestation-control-v1` |
| Brew-Dashboard | `misc/brew-server/src/dashboard.rs` | TCP 9003 |
| Brew-eigenes SIP | `misc/brew-server/src/sip/`, `src/config.rs` | Eigenständige optionale SIP-Rolle; Default ebenfalls UDP 5060 |
| Älterer Python-Brew-Server | `misc/brew-server.py` | Weiterhin als separate Alternative vorhanden; aktive Installation nicht untersucht |

Weitere relevante Pfade: `/etc/netcore/call-control.toml`, `/etc/netcore/sip-switch.toml`, `/var/lib/netcore-call-control/calls.json` und `.bak`; Unit `netcore-call-control.service`, `netcore-sip-switch.service` und TBS-seitig `netcore-tbs-sip-failover.service`. Die tatsächliche Brew-Unit und Installationsablage wurden in diesem Chat nicht belegt und werden nicht erfunden.

Das offene NetCore-Labormodell darf nicht pauschal auf den separaten Server übertragen werden: Brew hat eigene Auth-/TLS-Konfigurationsstrukturen. Es wurden keine Zugangswerte, Tokens, privaten Schlüssel oder vollständigen Betriebs-Konfigurationsdateien in dieses Archiv übernommen.

## 10. Fehlerbilder, Diagnose und verworfene Ansätze

| Punkt | Einordnung | Ergebnis bzw. verbleibende Arbeit |
|---|---|---|
| Doppelte Call Owner | Architektur-Risiko, kein beobachteter Incident | Rufdomäne und Autorität eindeutig festlegen; Adapter darf keine parallele netzweite Entscheidung treffen |
| Ruf beendet, Medien/Slot bleiben aktiv | Illustratives Fehlerszenario | Release-/Timeout-/Reconnect-/Slot-Reuse-Verhalten später integriert testen |
| Zwei SIP-Master, doppelte Registrierung | Im Gespräch befürchteter Konflikt | Vorhandenen zentralen Weg und exklusiven Fallback beibehalten; optionalen Brew-SIP-Modus separat bewerten |
| Im Gespräch angeführte SIP-Folge bis `481 Call/Transaction Does Not Exist` | Beispiel, kein bereitgestellter Mitschnitt | Keine tatsächliche Ursache oder erfolgreiche Reparatur dieses Fehlers im Chat nachgewiesen |
| Monolithischer Server zusätzlich zum entkoppelten RF-/Backend-Pfad | Verworfene Integrationsrichtung für dieselben Verantwortlichkeiten | Protokoll-/Codec-Bausteine gezielt beurteilen; Blockierung und Rückstau im Funkpfad verhindern |
| Neuer Gateway-Container ohne nachgewiesene Lücke | Hinterfragte Idee | Bestehenden Backhaul zuerst untersuchen; Dienstgrenze erst danach entscheiden |
| Upstream-Funktionen und eingebettete Serverversion verwechselt | Bei Archivprüfung erkannte Quellenunschärfe | Historisch 0.5.0, heutiger NetCore-Snapshot 1.9.0, abgefragter Upstream 1.14.0 getrennt dokumentiert |

Der Chat enthält keine ausgeführte Installation, Reparatur, Deinstallation oder Migration des Brew-Servers. Auch die Formulierung „nicht integrieren“ ist kein nachträglicher Nachweis, dass bestehender Code oder ein bestehender Betrieb entfernt worden wären.

## 11. Befehle und Abläufe mit Ausführungsstatus

### 11.1 Für diese Archivprüfung tatsächlich ausgeführt

In einem getrennten lokalen Checkout wurde der vorhandene Branch anonym gelesen, ohne Zugangsdaten aus Dateien oder Umgebungen auszulesen. Relevante lesende Prüfungen waren:

```powershell
git -c credential.helper= ls-remote https://github.com/JanHG98/netcore-tetra.git refs/heads/Archiving
git -c credential.helper= clone --single-branch --branch Archiving https://github.com/JanHG98/netcore-tetra.git netcore-archiving-brew-20261005
git -c credential.helper= fetch --no-tags origin c128a83cd3eef463de1e7e0c909af1fff4b84b36 refs/tags/v1.8.0:refs/tags/v1.8.0
git rev-parse 'v1.8.0^{commit}'
git rev-list --count v1.8.0..c128a83cd3eef463de1e7e0c909af1fff4b84b36
git diff --numstat v1.8.0 c128a83cd3eef463de1e7e0c909af1fff4b84b36 -- crates/tetra-entities/src/cmce/subentities/cc_bs/central_control.rs
git status --short --branch
```

Hinzu kamen gezielte Quelltext-/Dateisuchen, das Lesen der genannten READMEs und Beispiel-Portdefinitionen sowie ein erneuter Abruf von `Archiving` vor dem Schreiben. Dies sind **Repository-Prüfungen**, keine Deployment- oder Funktests.

### 11.2 Nur dokumentiert, hier nicht ausgeführt

Die Server-README nennt `cargo run --release -- brew-server.toml`, `docker compose up --build` und den lokalen HTTP-Healthcheck. Diese Befehle würden einen eigenständigen Brew-Server starten beziehungsweise prüfen; sie realisieren keinen NetCore-Adapter. Sie wurden für dieses Archiv nicht ausgeführt.

Vorhandene Testeinstiege sind `cargo test --locked --manifest-path misc/brew-server/Cargo.toml dashboard::`, die `net_brew`-Tests im TBS-Crate und statische Prüfer wie `tools/check_call_control.py` und `tools/check_cmce_call_restore.py`. Ihre Existenz ist festgestellt; es wurde hier kein neuer grüner Cargo-/CI-/Hardware-Lauf erzeugt oder behauptet.

Die historischen Rollout-Befehle mit `git switch mqtt` und `git pull --ff-only origin mqtt` gehören zum damaligen Dokument. Sie werden **nicht als heutige Updateanweisung übernommen**. Für eine spätere Installation sind ein aktuell bestätigter Zielref, der bestehende Hostzustand und Sicherungen erforderlich. `Archiving` ist hier Dokumentationsziel, kein aus diesem Chat abgeleiteter Installationsref.

## 12. Entwicklungs-, Test- und Betriebsstand

| Gegenstand | Status und Nachweisgrenze |
|---|---|
| Architekturberatung | Im Chat durchgeführt; spätere Einschränkungen ersetzen die erste weitergehende Empfehlung |
| NetCore-Core-Dienste/zentraler TBS-Steuerpfad | Im historischen und heutigen Repository vorhanden; keine Implementierung allein aufgrund der Chatantwort behauptet |
| Separater Rust-Brew-Server | Im Repository implementiert, historisch und heute; aktiver Betrieb und Integrationsmodus dieses Chats offen |
| Konfliktfreier Brew-Adapter zu NetCore | Idee/Option; kein diesem Chat zuordenbarer Implementierungs- oder Abnahmenachweis |
| Upstream-Kompatibilitätsmatrix | Vorgeschlagen; nicht als ausgeführtes Arbeitsergebnis nachgewiesen |
| Parser-/Builder-Regressionen | Zehn `#[test]`-Fälle in `net_brew/protocol.rs` gefunden, unter anderem v0/v1, Mnemonic und SDS; in dieser Fortsetzung nicht ausgeführt |
| CI-Konfiguration | `service-ui-tests.yml` enthält Rust-Service-Tests und einen getrennten Brew-Dashboard-Test; vorhandene Workflowdatei ist kein erfolgreicher Lauf |
| Funk↔Telefon, ISSI und SC20-CLI | Im damaligen Testnetz laut Nutzer erfolgreich; keine erneute heutige Verifikation |
| Multi-BS mit fremden Brew-TBS | Nicht durch gemeinsame Testprotokolle bestätigt |
| Mid-Call-Handover/Restore | Historisch ausdrücklich unvollständig; heutige Restore-/Capability-Dateien sind keine Live-Abnahme |
| SIP-Fallback | Im Repository mit drei Fehlprüfungen und 30 Sekunden stabiler Rückkehr dokumentiert; aktive Dialoge werden nicht migriert; hier kein Ausfalltest |
| Vollständiger Upstream-Diff/Codec-Übernahme | Offen; Versions-/Metadatenvergleich ersetzt keine Portierung oder Testreihe |
| Dieser Archivauftrag | Dokumentation/Index und deren Git-Verifikation; keine Änderung von Laufzeitcode oder Dienstkonfiguration |

## 13. Offene Aufgaben, Ideen und nächste Schritte

Die Reihenfolge unten ist eine **aus dem letzten Gesprächsstand abgeleitete Fortsetzungsempfehlung**. Im Chat wurden keine Verantwortlichen, Termine oder nummerierten Prioritäten verbindlich vereinbart.

1. **Vorhandenen Brew-Betrieb klären.** Verwendete Python-/Rust-Variante, Build-Commit, angeschlossene TBS, eigener oder geteilter Teilnehmer-/Rufbereich, Aktivierung von SIP und tatsächlich verwendete Ports feststellen. Aus dem Repository keine Laufzeitkonfiguration ableiten.
2. **Zuständigkeitsvertrag dokumentieren.** Netzweiter logischer Call, lokale CMCE-Verantwortung, Gruppen-/Teilnehmerrouten, Floor, Release, Timeout und Ressourcenfreigabe müssen eindeutig bleiben. Bestehender separater Betrieb darf nicht ungeprüft umgebaut werden.
3. **Kompatibilitätsmatrix anlegen.** Pro Brew-Nachricht beziehungsweise Funktion erfassen: gepinnter Upstream-Stand, NetCore-Stand, vorhandene Unterstützung, fehlende Funktion, robustere Upstream-Lösung, portierbarer Teil, Konflikt mit vorhandener Zuständigkeit und zugehöriger Testfall.
4. **Vergleichsquellen konkretisieren.** Die im Chat genannten fünf Vergleichsrepos und `ysamouhos/brew-server` mit überprüften URLs/Refs erfassen. Eine regelmäßige Prüfung blieb eine Idee; es wurde keine Automation für spätere Läufe angelegt.
5. **Protokoll und Robustheit zuerst vergleichen.** Framing, Versionserkennung, Parsergrenzen, Call-/UUID-Zuordnung, Registration/Affiliation, SDS-Reports, Wiederverbindung und Fehlerbehandlung. Bestehende `net_brew`-Tests erweitern, wenn eine konkrete Portierung feststeht.
6. **Interoperabilitätslabor aufbauen.** Eigene, fremde und virtuelle TBS mit gleichen Core-Testfällen prüfen. Gruppen-/Individualrufe, Priorität/Floor, Late Entry, Release, Medienfluss, SDS, Mobility und Telemetrie separat abnehmen. Erfolgreiches Parsing ist noch kein erfolgreicher Funkruf.
7. **Medien-/Codec-Bausteine getrennt bewerten.** ACELP↔PCM/G.711, Latenz, Frameformat, Jitter, Last und Trennung vom Funkthread vergleichen; Quellprovenienz und mitgelieferte Lizenzhinweise je tatsächlich übernommenem Bestandteil dokumentieren. Keine automatische Übernahme des fremden SIP-/Call-State-Modells.
8. **LIP und Telemetrie weiterverwenden.** Bestehende Decoder und Observability-Verträge einbeziehen, fehlendes Mapping bestimmen. Nicht eine zweite Positions-/Statuswahrheit ungeplant parallel aufbauen.
9. **Adapter-Dienstgrenze erst bei belegter Lücke festlegen.** Bestehenden Node-Gateway-/Backhaul-Pfad prüfen; ein neues `brew-gateway` bleibt eine Option. Transit nicht ohne Vertragsprüfung als TBS-Adapter umdeuten.
10. **Gezielte Änderungen separat liefern.** Erst aus belegten Lücken kleine Feature-/Fix-PRs mit passenden Regressionen ableiten. Solche Codeänderungen, Installationen oder Roadmapdateien außerhalb `Docs/archive/` sind nicht Teil dieses Archivauftrags.

Als längerfristiges Ziel bleibt erhalten: Eine NetCore-SwMI soll eigene und fremde Brew-fähige TBS bedienen können, ohne die fachlichen Core-Dienste von der konkreten Basisstationsimplementierung abhängig zu machen. Das virtuelle TETRA-Labor und reale Multi-Site-Tests sind dafür komplementäre Ideen, keine bereits erreichte Betriebsfreigabe.

## 14. Quellen und technische Einstiegspunkte

Alle relativen Repository-Links beziehen sich fachlich auf den in Abschnitt 1 genannten Prüfcommit. Beim Lesen auf einem später veränderten Branch können die Zieldateien weiterentwickelt sein; für reproduzierbare Vergleiche den festen Prüfcommit verwenden.

| Quelle | Relevanz |
|---|---|
| [Ursprungsdialog](https://chatgpt.com/c/6aae887e-08b0-83eb-8a9d-1b3bfb0401ed) | Anforderungen, Korrekturen, Nutzerrückmeldung und offene Ideen |
| [Historischer Vergleich v1.8.0 bis c128a83](https://github.com/JanHG98/netcore-tetra/compare/v1.8.0...c128a83cd3eef463de1e7e0c909af1fff4b84b36) | 141 Commits; lokal mit Git nachgeprüft, unabhängig von der Web-Diff-Darstellung |
| [Historischer Server-Paketstand](https://github.com/JanHG98/netcore-tetra/blob/c128a83cd3eef463de1e7e0c909af1fff4b84b36/misc/brew-server/Cargo.toml) | Bereits eingebetteter Server 0.5.0 |
| [Historischer Rollout](https://github.com/JanHG98/netcore-tetra/blob/c128a83cd3eef463de1e7e0c909af1fff4b84b36/Docs/CENTRAL_NETWORK_ROLLOUT.md) | MAIN-COMPAT, Mediengrenzen, SIP-/Fallback-Rollen |
| [Root-Workspace](../../Cargo.toml), [Dienstregister](../../system-backend/services.toml), [Inventory](../../deploy/open-lab/inventory.example.toml) | Core-Komponenten und Deploymentrollen |
| [Brew-Paket](../../misc/brew-server/Cargo.toml), [Brew-README](../../misc/brew-server/README.md), [Serverzustand](../../misc/brew-server/src/state.rs), [Router](../../misc/brew-server/src/router.rs) | Eigenständiges Paket und tatsächlich eigene Fachzustände |
| [TBS-Brew-Protokoll](../../crates/tetra-entities/src/net_brew/protocol.rs) | Parser, Builder, v0/v1-/SDS-/Medientests |
| [Zentrale CMCE-Anbindung](../../crates/tetra-entities/src/cmce/subentities/cc_bs/central_control.rs) | Lokale Rufkommandos und Medienzuordnung |
| [Call-Control-README](../../system-backend/call-control/README.md), [HTTP-API](../../system-backend/call-control/src/http.rs), [Zustand/Eingaben](../../system-backend/call-control/src/state.rs) | Logische Rufzuständigkeit und echte API |
| [Media Switch](../../system-backend/media-switch/README.md), [Node Gateway](../../system-backend/node-gateway/README.md), [Transit](../../system-backend/transit/README.md) | Unterschiedliche Transport- und Vermittlungsrollen |
| [SIP-Switch](../../system-backend/sip-switch/README.md), [Fallback-Anleitung](../../system-backend/sip-switch/tbs-fallback/docs/installation-openlab.md) | Zentraler Weg, lokale Edge-Rolle, Ausfall-/Rückkehrgrenzen |
| [TBS-SDS/LIP](../../crates/tetra-entities/src/cmce/subentities/sds_bs.rs), [Brew-Position](../../misc/brew-server/src/position.rs), [Brew-Transcoding](../../misc/brew-server/src/transcode/mod.rs) | Vorhandene Decoder-/Codec-Bausteine |
| [CI-Testdefinition](../../.github/workflows/service-ui-tests.yml), [Call-Control-Prüfer](../../tools/check_call_control.py), [Restore-Prüfer](../../tools/check_cmce_call_restore.py) | Vorhandene Prüfmittel, keine hier behaupteten Laufresultate |
| [Gepinnter Upstream](https://github.com/ysamouhos/brew-server/tree/b9ada00097d608e5ebd3f92c89857b9efff7f9ae) | Begrenzter heutiger Metadaten-/README-Abgleich |

## 15. Archivumfang und Veröffentlichungskontrolle

Die Suche nach dem exakten Chattitel und der Gesprächs-ID ergab vor dem Schreiben keine eindeutig zugehörige Archivdatei. Daher wird dieses neue, datierte Dokument angelegt. Andere Chatarchive bleiben erhalten. Der bestehende `Docs/archive/README.md` erhält genau einen zusätzlichen Eintrag mit relativem Dateilink und offenen Aufgaben.

Die Veröffentlichung umfasst ausschließlich dieses Dokument und den Archivindex. Sie erfolgt über die vorhandene GitHub-Anbindung als ein Commit auf dem zuvor gelesenen Branch-Stand und eine Fast-Forward-Aktualisierung von `Archiving` mit `force=false`. Bei zwischenzeitlicher Änderung ist der neue Branch-Stand erneut zu lesen und der Index zu erhalten; ein fremder Commit darf nicht verdrängt werden.

Für die Abschlussmeldung sind der tatsächliche Archivcommit, dessen geänderte Pfade, der Remote-Branch und beide zurückgelesenen Dateien zu prüfen. Das Ergebnis dieser Veröffentlichungskontrolle wird in der Abschlussmeldung festgehalten. Die fachlich offenen Aufgaben und die fehlenden Bild-/Betriebsnachweise bleiben auch nach erfolgreicher Archivierung offen.
