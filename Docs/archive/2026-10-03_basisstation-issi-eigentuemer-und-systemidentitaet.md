# Brainstorming: Basisstations-ISSI, Eigentümer und Systemidentität

**Archivhinweis, 09.10.2026:** Diese Datei dokumentiert das im Titel beziehungsweise Quellenrahmen genannte Gespräch und dessen damalige Entscheidungen. Historische Kommandos, Planungen, Versionsangaben und Fehlerbilder bleiben erhalten; der heutige technische Stand steht in der [Gesamtroadmap](../roadmaps/gesamtroadmap.md) und in den [aktuellen Dienstanleitungen](../services/README.md). Die Archivaufnahme bestätigt keine spätere Implementierung oder Live-Abnahme.

## 1. Rahmen und Quellenstand

| Merkmal | Wert |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Nummerierung einer Basisstation und ihres logischen SDS-/Systemteilnehmers |
| Erstellungsdatum | 2026-10-03, Europe/Berlin |
| Repository | [JanHG98/netcore-tetra](https://github.com/JanHG98/netcore-tetra) |
| Geprüfter Branchs | `Archiving` |
| Geprüfter Stand von Archiving am 03.10.2026 | `2fe2a1939a8795db3816d45973282781dae856f0` |
| Zusätzlich geprüfter Stand von main | `6aa9be8f74ab731f72dc133a5f8e90c5018c626d` |
| Zuordnung | Sichtbarer Dialog: Klassen-/ISSI-Schema → Eigentümer 01 → ausdrückliche Auswahl `04010001` |

### Verfügbarkeit und Grenzen

Grundlage sind ein Ausschnitt der vorhandenen Nummerierungsdokumentation, die endgültige Adressfestlegung, 25 PDF-Anhänge und die relevanten Repository-Dateien. Die vollständige ursprüngliche Nummerierungsdokumentation fehlt.

Festgelegt ist die Anzeige `04010001`. Eine Installation, Konfigurationsänderung auf der realen Basisstation oder Funkmessung ist dafür nicht nachgewiesen. Benachbarte Projektideen sind nur insoweit relevant, wie sie unmittelbar an diese Systemadresse anknüpfen.

Die PDFs wurden sämtlich per Textauszug erfasst und nach Titel, Version und Thema eingeordnet. Die für die Nummerierung maßgeblichen Abschnitte aus EN 300 392-1 wurden gezielt gelesen. Das ist **keine vollständige inhaltliche Auswertung aller 25 Normen** und keine Prüfung auf die jeweils neueste veröffentlichte Fassung. Die Anhänge werden nicht in dieses Archiv kopiert. Zugangsdaten werden nicht übernommen.

Die aufgeführten SHAs bezeichnen die geprüften Quellstände.

## 2. Ziel, Ausgangslage und behandelte Themen

Ziel ist, die Basisstation konsistent in das vorhandene organisatorische Nummernschema einzuordnen. Der bereitgestellte Ausschnitt beschreibt folgende Felder:

| Feld | Breite | Bedeutung im gelieferten Schema |
|---|---:|---|
| D | 1 Stelle | Betriebsdomäne: 0 = Produktiv, 1 = Sonderbetrieb |
| K | 1 Stelle | Teilnehmerklasse |
| EE | 2 Stellen | Eigentümer / Organisation |
| NNNN | 4 Stellen | Laufende Nummer |

Der Ausschnitt weist darauf hin, dass die ISSI numerisch ist und eine achtstellige Darstellung mit führender Null lediglich eine Darstellungsoption sein kann. Behandelt wurden:

- Unterschied zwischen RF-Zelle, Management-/Node-ID und logischem SDS-Absender/-Empfänger.
- Einordnung der logischen Basisstationsidentität in Klasse 4 „Infrastruktur“.
- Integration von Jan als Eigentümer mit der vorhandenen Eigentümernummer 1, also `EE = 01`.
- Festlegung der Nummer für Basisstation 1.
- Skalierung auf weitere Basisstationen und mögliche zentrale Dienste.
- Umgang mit führenden Nullen, technischen SSI-Grenzen und dem früher diskutierten Wert `9999`.
- Vorschlag möglicher Konfigurationsfelder; diese wurden im Dialog nicht implementiert.

### Vorgegebene Teilnehmerklassen

Diese Tabelle dokumentiert das gelieferte organisatorische Schema. Die Klassen sind keine ETSI-definierten Berechtigungs- oder Gerätetypcodes.

| K | Zahlenblock ohne D | Klasse | Beschreibung aus den Arbeitsnotizen |
|---|---|---|---|
| 1 | 1 000 000–1 999 999 | Leitstelle (LST) | Voll berechtigte Steuer- und Bedienplätze |
| 2 | 2 000 000–2 999 999 | HRT | Handheld Radio Terminal |
| 3 | 3 000 000–3 999 999 | MRT | Mobile Radio Terminal |
| 4 | 4 000 000–4 999 999 | Infrastruktur | Gateways, Recorder, Kopplungen |
| 5 | 5 000 000–5 999 999 | Virtuell | Virtuelle Sprecher, Bots, KI-Agenten |
| 9 | 9 000 000–9 999 999 | Test / Übung | Im Schema als strikt getrennte Test-/Übungsteilnehmer beschrieben |

„Voll berechtigt“ und „strikt getrennt“ beschreiben organisatorische Ziele. Eine Nummer allein implementiert weder RBAC noch technische Netztrennung.

## 3. Endgültige Entscheidungen und Status

**Verbindliche Festlegung: `04010001` für die erste Basisstation.** Die Eigentümernummer ist ausdrücklich `01` für Jan; frühere Vorschläge mit Eigentümer `00` sind damit überholt.

| Festlegung | Endgültiger Wert | Status und Nachweis |
|---|---|---|
| Anzeige-/Schema-ID für Basisstation 1 | `04010001` | **Beschlossen:** ausdrücklich ausdrücklich gewählt |
| Technische numerische ISSI | `4010001` | **Beschlossen im Dialog**, als numerisches Gegenstück erklärt; im aktuellen Code an mehreren Stellen vorhanden |
| Produktivdomäne | `D = 0` | Bestandteil der gewählten Anzeige; keine eigenständige Isolation nachgewiesen |
| Klasse | `K = 4`, Infrastruktur | **Beschlossen** durch Auswahl der vorgeschlagenen Infrastrukturkennung |
| Eigentümer | `EE = 01`, Jan | **Beschlossen:** ausdrückliche Präzisierung |
| Laufende Nummer | `NNNN = 0001` | Bestandteil der ausgewählten Kennung für Basisstation 1 |
| Zusätzliche Felder `station_display_id` usw. | siehe Abschnitt 6 | **Idee/Vorschlag**, kein nachgewiesenes produktives Schema |
| Neue Nummer auf echter Station und Endgeräten eingerichtet | nicht belegt | **Nicht getestet / nicht im Betrieb bestätigt** |
| Weitere Basisstationen mit individuellen Nummern | siehe Abschnitt 9 | **Idee**, keine vollständige Zuteilung beschlossen |

Die ausgewählte Anzeige zerlegt sich in `0 / 4 / 01 / 0001`. Ohne vorangestelltes Darstellungsfeld bleibt `4 / 01 / 0001 = 4010001`.

### Begründung

Die logische Basisstationsadresse gehört organisatorisch zur Infrastruktur und soll nicht mit einem normalen HRT oder MRT kollidieren. Das Eigentümerfeld bewahrt die Zuordnung zu Jan. Die Anzeige passt in das bestehende Schema; die numerische Adresse lässt sich für SDS- und Statusfunktionen verwenden. Eine getrennte Node-ID ermöglicht Management und Dienstverbindungen unabhängig von der Funkteilnehmeradresse.

Die frühere Formulierung „Die Basisstation bekommt keine Teilnehmer-ISSI“ ist präziser zu lesen: **Eine RF-Zelle wird nicht durch die hier vergebene Service-ISSI identifiziert. Softwarefunktionen der Basisstation können sehr wohl als logischer Systemteilnehmer eine ISSI verwenden.** Es handelt sich nicht um ein normatives Verbot von Infrastruktur-ISSIs.

## 4. Technische Einordnung und Korrekturen

### 4.1 Anzeige, Zahl und Domäne

`"04010001"` ist eine Zeichenkette mit erhaltener führender Null. Dezimal interpretiert entspricht sie `4010001`. Die Luftschnittstelle transportiert keine führende Dezimalnull und kennt das projektspezifische Feldschema nicht.

Für TOML ist `station_issi = 4010001` die korrekte numerische Schreibweise. `station_issi = 04010001` wäre keine gültige TOML-Dezimalzahl mit führender Null. Für eine Anzeige muss ein unterstütztes Stringfeld oder eine UI-Formatierung verwendet werden.

Das im Dialog empfohlene Abtrennen von D ist eine konzeptionelle Empfehlung. Für Sonderbetrieb ist keine abschließende Umsetzung beschlossen. Wer `D = 1` nur als Verwaltungsfeld führt, erhält damit keine zweite Funkadresse und keine technische Isolation. Wer dagegen `14010001` als vollständige Dezimalzahl sendet, verwendet eine andere ISSI; D wäre dann numerisch Teil der Adresse. Diese Varianten dürfen nicht stillschweigend vermischt werden.

### 4.2 Normative Grundlage aus den Anhängen

EN 300 392-1 V1.6.1 (2020-04), Abschnitt 7.2.1, PDF-Seite 27, definiert TSI mit 48 Bit und SSI mit 24 Bit. Abschnitt 7.2.3 und 7.2.4, PDF-Seite 29, ordnet die ISSI als netzspezifischen Teil der ITSI ein und zeigt die Aufteilung MCC 10 Bit, MNC 14 Bit, SSI 24 Bit. Eine SSI ist innerhalb des betreffenden TETRA-Netzes eindeutig; derselbe Zahlenwert kann in anderen Netzen erneut vorkommen.

Der reine 24-Bit-Zahlenraum reicht von 0 bis `16 777 215`. Daraus folgt **nicht**, dass jeder Wert frei als reguläre individuelle Adresse vergeben werden darf. Abschnitt 7.7.8, PDF-Seiten 37–38, reserviert den Wert mit 24 gesetzten Bits für Gruppen-Broadcast. `4010001` liegt im Zahlenraum und ist nicht dieser Broadcastwert. Das ersetzt keine Prüfung lokaler Belegungen und Routingregeln.

Die pauschale frühere Aussage, ISSIs seien stets sechs- oder siebenstellig, ist technisch unvollständig: 24-Bit-Werte können auch acht Dezimalstellen besitzen. Acht Stellen sind nicht automatisch ungültig; ausschlaggebend sind Zahlenwert und zulässige Verwendung. Beispielsweise passt `14010001` numerisch in 24 Bit, `19010001` nicht. Ein vollständiges D/K-Schema für Sonderbetrieb muss deshalb gesondert geprüft werden.

### 4.3 RF- und Managementidentität

MCC/MNC bestimmen das Netz. Location Area, Colour Code, Carrier und gegebenenfalls weitere Zellparameter sind für die RF-Zuordnung relevant. Die in den Arbeitsnotizen beispielhaft genannte „Cell-/Node-ID“ darf dabei nicht als eine einheitliche standardisierte Luftschnittstellenkennung verstanden werden. Die NetCore-Node-ID ist im geprüften Code eine Managementzeichenkette.

Eine Class-4-Adresse ist eine interne Nummernkonvention, keine vom Standard aus der führenden Dezimalziffer abgeleitete Infrastrukturrolle. Auch Eigentümerschaft und Berechtigung entstehen nicht allein aus `EE = 01`.

## 5. Repository-Befund vom 03.10.2026

### 5.1 Prüfmethode

Der Branch `Archiving` wurde gelesen und lokal ausschließlich als dieser Branch ausgecheckt. Der vollständige Git-Dateibaum war verfügbar; unter `Docs/archive/` existierten beim ersten und beim erneuten Prüfen vor der Erstellung keine Dateien. Im Repository-Dateibaum wurde keine `AGENTS.md` gefunden.

Zusätzlich wurde `main` gelesen/geholt. Für die maßgeblichen Dateien `config.toml`, `sec_directory.rs`, `sds_bs.rs`, `tetra-core/src/address.rs`, die gemeinsamen Address-Contracts und `wiki/ISSI-and-GSSI.md` zeigten die geprüften Branchstände identische Inhalte. `net_dashboard/server.rs` unterscheidet sich durch UI-/Zugriffserweiterungen; die unten genannten Systemadressen beziehen sich auf den geprüften Archiving-Stand. Es wurde kein Branch gemergt oder außerhalb des Archivpfads verändert.

### 5.2 Nachgewiesene Implementierungsstellen

| Datei / Komponente | Statisch nachgewiesener Stand | Einordnung |
|---|---|---|
| `crates/tetra-entities/src/cmce/subentities/sds_bs.rs` | `DASHBOARD_ISSI: u32 = 4010001` | **Implementiert**, als feste lokale Systemadresse |
| dieselbe Datei, `route_rf_deliver` | Lokale Sonderbehandlung für Emergency-SDS, konfigurierten WX-Service und die Dashboard-ISSI vor zentralem SDS-Handoff | **Implementiert**; Laufzeit nicht geprüft |
| dieselbe Datei, `route_status_deliver` | U-STATUS an `4010001` wird an `handle_sds_command_status` übergeben; Rückkehr vor zentralem Statusrouting | **Implementiert**; Zugriff wird durch konfigurierte autorisierte ISSIs/Aktionen begrenzt |
| `crates/tetra-entities/src/net_dashboard/server.rs` | Dashboard-Aktion `sds` setzt Quelle `4010001`; Call-Out verwendet diesen Wert als Default bei fehlender/Null-Quelle | **Implementiert**, kein Nachweis einheitlicher Konfiguration aller Sender |
| `crates/tetra-config/src/bluestation/sec_directory.rs` | Vorhandenes Feld `bs_issi`, Default `4010001` im Directory-Konfigurationsmodell | **Implementiert**; eigenständiger Parameter |
| `crates/tetra-config/src/bluestation/sec_control_room.rs` | `node_id`, `station_name`, `endpoint_path`, `central_sds_routing` | **Implementiert** als Management-/Routingkonfiguration |
| `crates/tetra-entities/src/net_control_room/protocol.rs` | Explizite Node-ID hat Vorrang; sonst `tbs-{mcc}-{mnc}-la{location_area}-cc{colour_code}-c{main_carrier}` | **Implementiert**, unabhängig von ISSI |
| `Docs/PROVISIONING_CORE_COMPLETE_INSTALL.md` | Beispiel `node_id = "tbs-04010001"` | **Dokumentiert**, kein Nachweis einer laufenden Station mit dieser Node-ID |
| `system-backend/shared/contracts/src/address.rs` | `MAX_SSI = 0x00ff_ffff`; Konstruktor mit Rangeprüfung, numerische Darstellung | **Implementiert**; generische Prüfung ist keine vollständige Vergabepolicy |
| dieselbe Contract-Datei | Test `serde_is_numeric_and_roundtrips` mit `4_010_001` und JSON `"4010001"` | **Test vorhanden**, bei der Quellenprüfung nicht ausgeführt |
| `crates/tetra-core/src/address.rs` | `TetraAddress` speichert `ssi: u32`; Konstruktor prüft dort selbst keine 24-Bit-Grenze | **Implementiert**; keine pauschale Behauptung flächendeckender Validierung |
| `crates/tetra-pdus/src/cmce/pdus/d_sds_data.rs` | SSI-Feld wird mit 24 Bit gelesen/geschrieben | **Implementiert**, Grenzen vor Serialisierung weiterhin relevant |
| `wiki/ISSI-and-GSSI.md` | Organisatorisches D/K/EE/NNNN-Schema und getrennte System-ISSIs empfohlen | **Dokumentiert**, D-/Sonderbetriebssemantik noch präzisierungsbedürftig |

Eine gezielte Suche in `crates/`, `system-backend/`, `deploy/`, `wiki/` und `Docs/` fand keine Implementierung der exakten vorgeschlagenen Namen `station_display_id`, `station_issi`, `station_owner`, `station_class` oder `station_role`. Diese Suche ist ein Befund zum geprüften Stand, keine Aussage über externe Konfigurationen oder unveröffentlichte Branches.

### 5.3 Konfigurationswerte und relevante Schnittstellen

Die folgenden Werte stammen aus der versionierten `config.toml`; sie sind **kein Nachweis der tatsächlich ausgerollten Datei**:

| Parameter | Wert / Bedeutung |
|---|---|
| `[net_info].mcc` / `mnc` | `901` / `1510` |
| `[cell_info].location_area` / `colour_code` | `1` / `1` |
| `[wx_service].enabled` / `service_issi` | `true` / `4010001` |
| `[audio_player].enabled` / `source_issi` | `true` / `4010001` |
| `[control_room].node_id` / `station_name` | jeweils `"SRV-M-TBS-01"` |
| `[control_room].enabled` / `port` | `true` / `8080`; ausgehende Dienstverbindung, nicht mit lokalem Dashboard gleichsetzen |
| `[dashboard].bind` / `port` | `"0.0.0.0"` / `8080`; HTTP-Dashboard |
| `[netcore_directory].base_url` | `http://10.0.1.23:8095`; Directory über HTTP |
| `[netcore_directory].timeout_ms` | `2000` |
| Directory-`bs_issi` | Im betrachteten TOML-Block nicht explizit gesetzt; Code-Default `4010001` |
| Node-WebSocket-Pfad | Code-Default `/ws/node`; WS/WSS abhängig von `use_tls` |

Die bereinigte Beispieldatei `Docs/basisstation.config.sanitized.example.toml` verwendet bei MCC/MNC dagegen `1`/`333` als anzupassende Werte; sie ist nicht mit der versionierten Hauptkonfiguration oder dem Live-System gleichzusetzen. Ihre Node-ID ist `SRV-M_TBS-01` mit Unterstrich. Das dokumentierte Beispiel `tbs-04010001` ist eine weitere Darstellung und ersetzt keine dieser Werte automatisch.

### 5.4 Zusammenhang mit SDS, MQTT und Home Assistant

Der Code liest/loggt eine eingehende SDS, verarbeitet lokale Sonderfälle und erreicht erst anschließend gegebenenfalls das zentrale Routing. Ist WX aktiviert und die Zieladresse `4010001`, kann eine Nachricht als Wetteranfrage verarbeitet werden. SDS-TL-Reports werden dort gesondert absorbiert. Andere SDS an die Dashboard-Adresse werden lokal konsumiert; U-STATUS an dieselbe Adresse läuft durch den lokalen Befehlskanal.

Damit ist die generische Erwartung „SDS an Basisstationsadresse erscheint automatisch im zentralen SDS Router/MQTT/HA“ **nicht bestätigt**. Je nach Nachrichtentyp entstehen lokale Log-/Telemetry-Ereignisse, aber ein durchgängiger Nutzdatenpfad ist gesondert nachzuweisen.

`wiki/FAQ.md`, `wiki/Troubleshooting.md` und das versionierte Systemhandbuch erwähnen die aus dem Projektkontext bekannte offene SDS-/HA-Strecke an `4010001`. Die aktuelle Prüfung liefert einen konkreten möglichen Zusammenhang mit lokalem Routing; sie beweist weder die tatsächliche Ursache des damaligen Live-Fehlers noch dessen Behebung.

## 6. Historisch vorgeschlagene Konfiguration

Der Assistant schlug im Dialog vor:

```toml
# Historischer Entwurf: kein nachgewiesenes aktuelles NetCore-Schema.
station_display_id = "04010001"
station_issi = 4010001
station_owner = "01"
station_class = "infrastructure"
station_role = "base_station_sds_service"
```

**Status: Idee/Vorschlag.** Beschlossen ist die Nummer; die Implementierung dieser fünf Felder ist nicht belegt. Dieser Block darf nicht als geprüfte Installationsanleitung in die bestehende `config.toml` kopiert werden. Dass unbekannte Werte gegebenenfalls als Zusatzdaten geparst werden, würde keine Nutzung durch den Stack beweisen.

Für eine Fortsetzung ist zunächst das vorhandene Schema mit `netcore_directory.bs_issi`, dienstspezifischen Quellen und `control_room.node_id` zu berücksichtigen. Ein zentraler neuer Parameter müsste ausdrücklich entworfen, verdrahtet und auf alle hart codierten Stellen angewendet werden.

## 7. Befehle, Abläufe und tatsächlich erreichte Ergebnisse

Im historischen Nummerierungsdialog wurden keine Installations-, Deployment- oder Reparaturbefehle ausgeführt. Der TOML-Block war nur ein Vorschlag.

Bei Erstellung dieses Archivs wurden read-only Repository- und Anhangsprüfungen durchgeführt:

| Ablauf | Status / Ergebnis |
|---|---|
| GitHub-Branch- und Dateibaumabfrage für `Archiving` und `main` | **Erfolgreich ausgeführt**, oben genannte SHAs und Dateibäume gelesen |
| `git ls-remote` für beide Branches | **Erfolgreich ausgeführt**, Branch-SHAs bestätigt |
| `git clone --depth 1 --single-branch --branch Archiving https://github.com/JanHG98/netcore-tetra.git repo-archive` | **Erfolgreich ausgeführt**, isolierter Arbeitsstand auf Archiving |
| `git fetch origin main` | **Erfolgreich ausgeführt**; bei Single-Branch-Clone lag der geholte Stand in `FETCH_HEAD` |
| Erster Vergleich über `origin/main` | **Fehlgeschlagen**, Remote-Tracking-Ref durch Single-Branch-Clone nicht vorhanden |
| Anschließender Vergleich `git diff FETCH_HEAD HEAD -- <relevante Dateien>` | **Erfolgreich ausgeführt**, Branchunterschiede geprüft |
| `rg`-Suche nach Kennungen und vorgeschlagenen Feldern | **Erfolgreich ausgeführt**, Befunde in Abschnitt 5 |
| Python/`tomllib`-Auswertung ausgewählter Konfigurationsparameter | **Erfolgreich ausgeführt**, ohne Zugangsdaten in diese Datei zu übernehmen |
| `pdftotext -layout <PDF> <Textauszug>` für alle Anhänge | **Erfolgreich ausgeführt**, 25 Textauszüge verfügbar |
| Build, Cargo-Tests, Funk-/SDS-Test, Deployment, Zugriff auf Live-TBS | **Nicht ausgeführt** |

Ein fehlender `origin/main`-Ref im Prüfcheckout war ein lokales Git-Prüfproblem und kein Fehler des Produktivsystems. Der Vergleich über den zuvor geholten `FETCH_HEAD` löste es.

Der Dokumentation speichert ausschließlich Dokumentation. Er ist keine Migration von `9999` und kein Deployment der ausgewählten Nummer.

## 8. Fehler, Abweichungen, Tests und Grenzen

### 8.1 Überholte oder ungenaue Aussagen

| Frühere Aussage / Ansatz | Einordnung und endgültige Behandlung |
|---|---|
| `4000001` für die erste Basisstation | **Ersetzt** durch `4010001`, nachdem Jan Eigentümer 01 ausdrücklich ergänzte |
| `04000001` als Anzeige | **Ersetzt** durch ausdrücklich gewähltes `04010001` |
| EE 00 für NetCore/Core reservieren | **Idee**, keine verbindliche Reservierung belegt |
| `9999` „rauswerfen“ | **Empfehlung**, keine vollständige Migration oder Deploymentbestätigung |
| Achtstellige Kennungen generell problematisch | **Präzisiert:** Zahlenwert/Reservierungen sind ausschlaggebend, nicht die bloße Stellenzahl |
| D garantiert getrennte Betriebsdomänen | **Nicht belegt:** Verwaltungsdarstellung allein bewirkt keine technische Trennung |
| Jede Basisstation ohne Weiteres auf andere Service-ISSI umstellen | **Nicht implementiert:** hart codierte `4010001`-Stellen müssen berücksichtigt werden |
| Neue `station_*`-Felder als bestehende Konfiguration | **Nicht bestätigt:** nur historischer Entwurf |

### 8.2 Offene Inkonsistenz bei 9999

`crates/tetra-entities/tests/test_sds_bs.rs` enthält `test_u_status_command_ip_replies_to_authorized` und `test_u_status_command_unauthorized_no_reply`, die U-STATUS an `9999` senden. Die aktuelle Routingfunktion behandelt den lokalen Befehlskanal dagegen über `DASHBOARD_ISSI = 4010001`.

Das ist eine **statisch festgestellte Abweichung zwischen Testannahme und Implementierung**. In diesem Auftrag wurde nicht geprüft, ob oder wie die Tests im vollständigen Testaufbau fehlschlagen. Insbesondere beweist ein negativer Test an der alten Adresse nicht, dass die Autorisierung des geprüften Befehlskanals funktioniert.

Andere `9999`-Treffer gehören zu Testdaten, MNC-Werten, Node-IDs oder Dienstbeispielen. Ein pauschales Suchen/Ersetzen wäre fachlich falsch. Beispielsweise sind noch dienstspezifische Absender in generierten Task-Workflow-/Application-Gateway-Konfigurationen vorhanden; daraus folgt keine aktive Fehlkonfiguration.

### 8.3 Teststatus

- **Statisch überprüft:** Vorhandensein der Adresse, Codepfade, Konfigurationswerte, Schemanamen und 24-Bit-Felder.
- **Testcode vorhanden:** numerischer JSON-Roundtrip für `4010001`, generische SSI-Grenztests und SDS-Komponententests.
- **Nicht durchgeführt:** Ausführung dieser Tests, RF-SDS-Senden/Empfangen, Endgeräteprogrammierung, UI-Anzeigeprüfung auf laufendem Dienst.
- **Im Betrieb bestätigt:** Für die neue Anzeige bzw. eine abgeschlossene Adressmigration liegt in dieser Entwicklungsphase kein Nachweis vor.
- **Erreichter Stand:** Nummer organisatorisch beschlossen, technische Adresse bereits teilweise im Code verwendet; ganzheitliche Migration und mehrzellige Parametrisierung bleiben offen.

Bei der Archivvalidierung wurden zusätzlich alle 19 enthaltenen Markdown-Links auf verfügbare lokale Ziele bzw. referenzierte HTTPS-Quellen geprüft, die numerische Gleichheit von `04010001` und `4010001` bestätigt, die 24-Bit-Beispiele rechnerisch kontrolliert und die Ablehnung der führenden Null bei numerischer TOML-Schreibweise mit `tomllib` überprüft. Diese erfolgreichen Dokumentationsprüfungen sind keine Produkt- oder Funkfunktionstests.

## 9. Noch relevante Ideen und Roadmap-Kandidaten

### 9.1 Erweiterung der Nummernvergabe

Nach dem Eigentümerkorrigieren wurden folgende Beispiele vorgeschlagen:

| Logischer Teilnehmer | Vorgeschlagene ISSI | Vorgeschlagene Produktivanzeige | Status |
|---|---:|---|---|
| Jans TBS01 | 4010001 | 04010001 | **Beschlossen** |
| Jans TBS02 | 4010002 | 04010002 | **Idee**, individuelle Verwendung prüfen |
| Jans TBS03 | 4010003 | 04010003 | **Idee** |
| Weitere Stationen | fortlaufend im Block 401NNNN | 0 voranstellen | **Idee**, Zuteilungsregister erforderlich |

Vor der Eigentümerkorrektur wurden außerdem TBS10 mit `4000010`, Lighthouse `4000100`, Dashboard/API `4000200`, SDS-Gateway `4000300`, Recorder `4000400` und Bridge/Gateway `4000500` genannt. Diese **nicht freigegebenen Beispiele** beziehen sich auf EE 00 und sind keine endgültige Vergabeliste für Jan.

Eine mögliche Anpassung dieser Dienstnummern auf Eigentümer 01 ist offen. Nicht jeder Dienst benötigt eine eigene ISSI: Voraussetzung ist eine tatsächliche Rolle als Funk-/SDS-Systemteilnehmer. Rein interne HTTP-APIs, Recorder oder Weboberflächen erhalten nicht allein wegen ihrer Existenz eine Funkadresse.

### 9.2 Konkrete nächste Schritte

Die Reihenfolge ist eine technische Empfehlung aus der Archivprüfung; im Nummerierungsdialog wurden keine Termine oder verbindlichen Prioritäten vereinbart.

| Reihenfolge | Aufgabe | Status / Abhängigkeit / Abnahmekriterium |
|---|---|---|
| 1 | Aktuelle Live-Konfiguration und Endgeräteadressbücher mit `4010001` abgleichen | **Offen**; benötigt tatsächliche TBS-/Endgerätewerte; Anzeige und technische Adresse getrennt bestätigen |
| 2 | Vergabe und Eigentümer 01 verbindlich im Directory/Nummernregister dokumentieren | **Roadmap-Kandidat**; bestehende Belegungen und Servicezweck prüfen |
| 3 | Festlegen, welche Nachrichten an `4010001` lokal bleiben und welche zentral weitergegeben werden | **Roadmap-Kandidat**; WX, Dashboard-Reports, U-STATUS-Kommandos und HA-Nutzdaten voneinander abgrenzen |
| 4 | Tests für geprüfte Systemadresse und Autorisierung abgleichen | **Roadmap-Kandidat**; alte `9999`-Annahmen einzeln prüfen; positiver und negativer Test am tatsächlichen Befehlskanal |
| 5 | Basisstations-/Service-ISSI zentral konfigurierbar machen, falls pro TBS eigene Adresse gewünscht | **Idee**; bestehende Directory-Parameter, Senderdefaults, CMCE-Routing und Dashboard konsistent verdrahten |
| 6 | Produktivanzeige `04010001` in UI, Export und Inventar konsistent erhalten | **Beschlossener Darstellungswunsch**, Umsetzung nicht nachgewiesen; numerische API-/Funkwerte bleiben numerisch |
| 7 | Sonderbetriebsdomäne D abschließend definieren und Werte validieren | **Offen**; Verwaltung vs. eigene Funkadresse vs. separates Netz entscheiden |
| 8 | Mehrstationsvergabe und zentrale Serviceadressen festlegen | **Idee**; Netzweite Eindeutigkeit, lokale Sonderbehandlung und Routing über Brew/Core berücksichtigen |

Empfohlene spätere Abnahme: Eine normale SDS vom Funkgerät zu einem vorgesehenen Dienst, eine Antwort mit korrektem Absender, ein nicht destruktiver autorisierter Statusbefehl und ein abgelehnter unautorisierter Befehl müssen nachvollziehbar getestet werden. Für eine HA-Anbindung die Strecke TBS-Uplink → lokale Routingentscheidung → Node Gateway/SDS Router → IoT Gateway/MQTT → HA-Automation mit derselben Testnachricht korrelieren. Diese Abnahme ist **vorgeschlagen**, nicht ausgeführt.

## 10. Quellen und Referenzen

### 10.1 Repository

Alle nachfolgenden relativen Links beziehen sich auf den Branch/Commit, aus dem dieses Dokument betrachtet wird. Für die damalige Codeprüfung gilt der unveränderliche [geprüfte Archiving-Commit](https://github.com/JanHG98/netcore-tetra/tree/2fe2a1939a8795db3816d45973282781dae856f0). Zusätzlich geprüft: [main-Commit](https://github.com/JanHG98/netcore-tetra/tree/6aa9be8f74ab731f72dc133a5f8e90c5018c626d).

- [Versionierte Hauptkonfiguration](../../config.toml)
- [Bereinigte Beispielkonfiguration](../basisstation.config.sanitized.example.toml)
- [CMCE-/SDS-Basisstationslogik](../../crates/tetra-entities/src/cmce/subentities/sds_bs.rs)
- [Dashboard-Server](../../crates/tetra-entities/src/net_dashboard/server.rs)
- [Directory-Konfigurationsmodell](../../crates/tetra-config/src/bluestation/sec_directory.rs)
- [Control-Room-Konfigurationsmodell](../../crates/tetra-config/src/bluestation/sec_control_room.rs)
- [Management-Node-Identität](../../crates/tetra-entities/src/net_control_room/protocol.rs)
- [Gemeinsame SSI-Contracts und vorhandene Tests](../../system-backend/shared/contracts/src/address.rs)
- [Core-Adressdatentyp](../../crates/tetra-core/src/address.rs)
- [D-SDS-DATA-PDU](../../crates/tetra-pdus/src/cmce/pdus/d_sds_data.rs)
- [SDS-Komponententests mit Legacy-Adressannahmen](../../crates/tetra-entities/tests/test_sds_bs.rs)
- [ISSI-/GSSI-Wiki](../../wiki/ISSI-and-GSSI.md)
- [Provisioning-Beispiel mit tbs-04010001](../PROVISIONING_CORE_COMPLETE_INSTALL.md)
- [Systemhandbuch mit SDS-/HA-Prüfstrecke](../NetCore-Tetra-Systemhandbuch-2026-09-27.md)
- [FAQ](../../wiki/FAQ.md) und [Troubleshooting](../../wiki/Troubleshooting.md)

Es wurde kein thematisch zugehöriger Implementierungs-PR aus dem historischen Nummerierungsdialog übermittelt. Allgemeine PRs anderer Projektphasen werden nicht als Nachweis dieses Vorhabens geführt.

### 10.2 Anhangsinventar

Alle 25 Dateien waren zugänglich. „Erfasst“ bedeutet Titel-/Versionserfassung und Textextraktion; „gezielt geprüft“ bezeichnet die tatsächlich für Aussagen dieses Archivs gelesenen Abschnitte.

| Anhang | Identifizierter Inhalt / Fassung | Verwendung |
|---|---|---|
| `en_3003920308v010401p.pdf` | EN 300 392-3-8 V1.4.1, 2020-04; ISI Generic Speech Format | Erfasst; keine spezielle Nummernentscheidung daraus |
| `en_30039209v010701p.pdf` | EN 300 392-9 V1.7.1, 2020-04; allgemeine Supplementary Services | Erfasst |
| `ts_10081201v020205p.pdf` | TS 100 812-1 V2.2.5, 2003-10; UICC physisch/logisch | Erfasst |
| `en_3003921201v010202p.pdf` | EN 300 392-12-1 V1.2.2, 2007-08; Call Identification, Stage 3 | Erfasst |
| `en_3003920304v010301p.pdf` | EN 300 392-3-4 V1.3.1, 2010-08; ISI SDS | Erfasst |
| `en_3003921117v010102p.pdf` | EN 300 392-11-17 V1.1.2, 2002-01; Include Call, Stage 2 | Erfasst |
| `en_3003921114v010101p.pdf` | EN 300 392-11-14 V1.1.1, 2002-07; Late Entry, Stage 2 | Erfasst |
| `es_20081202v020401m.pdf` | Final Draft ES 200 812-2 V2.4.1, 2005-08; TSIM-Anwendung | Erfasst; Entwurfsstatus beachten |
| `es_20081201v020205p.pdf` | ES 200 812-1 V2.2.5, 2003-12; UICC physisch/logisch | Erfasst |
| `en_300812v020101p.pdf` | EN 300 812 V2.1.1, 2001-12; SIM-ME Security | Erfasst |
| `en_3003921101v010201p.pdf` | EN 300 392-11-1 V1.2.1, 2004-01; Call Identification, Stage 2 | Erfasst |
| `en_3003921006v010401p.pdf` | EN 300 392-10-6 V1.4.1, 2006-08; Call Authorized by Dispatcher | Erfasst |
| `en_3003921018v010301p.pdf` | EN 300 392-10-18 V1.3.1, 2003-10; Barring of Outgoing Calls | Erfasst |
| `en_3003921216v010400a.pdf` | Draft EN 300 392-12-16 V1.4.0, 2026-03; Pre-emptive Priority Call | Erfasst; Entwurfsstatus beachten |
| `en_30039201v010601p.pdf` | EN 300 392-1 V1.6.1, 2020-04; General Network Design | **Gezielt geprüft:** 7.2.1, 7.2.3, 7.2.4, 7.7.8 |
| `ets_30039214e01v.pdf` | Final Draft prETS 300 392-14, 1997-09; PICS Proforma | Erfasst; Entwurfsstatus beachten |
| `en_30039207v030501p.pdf` | EN 300 392-7 V3.5.1, 2019-07; Security | Erfasst |
| `en_30039401v030301p.pdf` | EN 300 394-1 V3.3.1, 2015-04; Radio Conformance Testing | Erfasst; keine Konformitätstests ausgeführt |
| `en_3003920313v010201p.pdf` | EN 300 392-3-13 V1.2.1, 2020-04; transportunabhängiger ISI Group Call | Erfasst |
| `en_30039502v010303p.pdf` | EN 300 395-2 V1.3.3, 2025-02; TETRA Codec | Erfasst |
| `en_3003920303v010301p.pdf` | EN 300 392-3-3 V1.3.1, 2011-11; ISI Group Call | Erfasst |
| `en_30039205v020701p.pdf` | EN 300 392-5 V2.7.1, 2020-04; PEI | Erfasst |
| `en_3003920315v010500a.pdf` | Draft EN 300 392-3-15 V1.5.0, 2026-04; transportunabhängiges ISI Mobility Management | Erfasst; Entwurfsstatus beachten |
| `en_30039202v030801p.pdf` | EN 300 392-2 V3.8.1, 2016-08; Air Interface | Erfasst; kein vollständiger AI-Konformitätsabgleich |
| `ETSI.pdf` | EN 300 812 V2.1.1, 2001-12; SIM-ME Security | Gleiche Norm/Fassung wie en_300812v020101p.pdf; PDFs nicht byteidentisch |

## 11. Fortsetzungshinweis

Für eine spätere Fortsetzung gilt: **Gewählte Anzeige `04010001`, technische Adresse `4010001`, Eigentümer Jan/01, Klasse Infrastruktur/4, erste Basisstation.** Die Nummer ist beschlossen. Der Stack verwendet `4010001` bereits an mehreren Stellen, aber eine durchgängig parametrierte Stationsidentität, die exakte achtstellige Anzeige überall und eine abgeschlossene Live-Migration sind damit nicht nachgewiesen. Zuerst den bestehenden lokalen Befehlskanal und die SDS-Routingausnahmen berücksichtigen, bevor Nummern geändert oder neue TBS-Adressen vergeben werden.
