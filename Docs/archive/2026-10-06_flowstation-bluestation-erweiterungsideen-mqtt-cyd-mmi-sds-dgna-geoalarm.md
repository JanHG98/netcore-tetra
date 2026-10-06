# Technische Abschlussdokumentation: Ideen zu Flowstation Bluestation

## 1. Metadaten und Einordnung

| Feld | Wert |
|---|---|
| Projekt | NetCore-Tetra / `JanHG98/netcore-tetra` |
| Thema | Öffentliche FlowStation-/BlueStation-Erweiterungen als mögliche Vorbilder für NetCore: Home Assistant, ESP32-Display, MS-MMI, SDS-Automation, DGNA und GeoAlarm |
| Ursprünglicher Chattitel | Ideen zu Flowstation Bluestation |
| Chat-ID | `6ab41782-2230-83eb-9a4d-22ed9be29690` |
| Chatlink | [Originalchat in ChatGPT](https://chatgpt.com/c/6ab41782-2230-83eb-9a4d-22ed9be29690) |
| App-Verweis | `chatgpt-conversation://6ab41782-2230-83eb-9a4d-22ed9be29690` |
| Erstellungsmetadatum des Chats | 23.09.2026, 20:16:35.035 Uhr, Europe/Berlin; vom Gesprächszugriff als `createdAt` geliefert |
| Sichtbare Fachrunde | 23.09.2026, 20:16:10.110–20:20:13.230 Uhr, Europe/Berlin |
| Sichtbare Archivierungsrunde | 06.10.2026, 08:20:25.607–08:21:20.253 Uhr, Europe/Berlin |
| Erstellung dieser Dokumentation | 06.10.2026, Europe/Berlin |
| Geprüfter Zielbranch | Bereits vorhandener Branch `Archiving` |
| Geprüfter Repository-Basiscommit | [`9fc9cc282023afe28f967ca1e236add012209935`](https://github.com/JanHG98/netcore-tetra/commit/9fc9cc282023afe28f967ca1e236add012209935) |
| Basiscommit-Metadatum | 06.10.2026, 08:18:02 +02:00; `docs(archive): document Pi 5 AI HAT 2 and SXceiver concept` |
| Ablage | `Docs/archive/2026-10-06_flowstation-bluestation-erweiterungsideen-mqtt-cyd-mmi-sds-dgna-geoalarm.md` |
| Umfang dieses Auftrags | Diese Archivdatei und ein zusätzlicher Eintrag in `Docs/archive/README.md`; keine Implementierung, kein Deployment, kein Merge, kein Force-Push |

**Ergebnis des Fachchats:** Eine begründete Sammlung öffentlich beschriebener Erweiterungen und möglicher Anschlussideen für NetCore. Der Chat enthält keinen Beschluss zur Umsetzung eines dieser Funktionspakete, keinen dazugehörigen Code-Patch und keine bestätigte Hardwareabnahme. Das eigenständige ESP32-Display und die BlueStation-MMI wurden vom damaligen Assistenten besonders hervorgehoben; eine vom Nutzer festgelegte Priorisierung ist daraus nicht abzuleiten.

Der oben genannte Commit ist der vor der Archivänderung geprüfte Code- und Dokumentationsstand. Er ist nicht der Veröffentlichungscommit dieser Datei. Den tatsächlichen Archivcommit liefert die Git-Historie der Datei; die Abschlussmeldung des Archivierungsauftrags nennt dessen Hash und die anschließende Remote-Prüfung. Eine selbstreferenzielle Commitnummer wird hier nicht erfunden.

## 2. Quellenumfang, Chronologie und Auswertungslücken

### 2.1 Tatsächlich zugänglicher Verlauf

Ausgewertet wurden der bereitgestellte, zunächst gekürzte Gesprächsvorschaublock, der danach über `read_thread` nachgeladene Verlauf und der aktuelle Fortsetzungsauftrag. Der abschließende vollständige Abruf lieferte zwei abgeschlossene Dialogrunden mit insgesamt vier Textnachrichten:

1. **Fachfrage:** Der Nutzer fragte nach Ideen und Funktionen anderer Personen, die auf FlowStation oder BlueStation aufbauen und möglicherweise mit ChatGPT entwickelt werden.
2. **Fachantwort:** Der Assistent erklärte die fehlende Einsicht in fremde Chats und beschrieb stattdessen sechs öffentliche Entwicklungsbeispiele. Er stellte Anwendungen für NetCore vor und begrenzte die Aussage ausdrücklich auf Dokumentationsrecherche ohne Funkhardwaretest oder Prüfung des damaligen NetCore-Branches.
3. **Archivierungsauftrag:** Der Nutzer verlangte eine ausführliche technische Abschlussdokumentation, die Trennung historischer Aussagen vom aktuellen Repository-Stand, die Übernahme zugänglicher Chatbilder, einen erhaltenen Archivindex und die Veröffentlichung ausschließlich in `Docs/archive/` auf `Archiving`.
4. **Übergabeantwort:** Der Assistent erklärte, einen Work-Chat für den Archivierungsauftrag angelegt zu haben. Diese Übergabeantwort ist kein Nachweis für einen bereits erfolgten Commit oder Push.

Der aktuelle Auftrag bekräftigt die Pfad- und Branchgrenze sowie das Verbot von Merge und Force-Push. Er verlangt ausdrücklich die Kennzeichnung fehlender Metadaten oder nicht verfügbarer Bilder. Der Nutzer archiviert den ursprünglichen Chat nach eigener Prüfung selbst; dieser Auftrag ändert dessen Archivstatus nicht.

Der letzte Abruf meldete `page.hasMore = false`, `page.nextCursor = null` und `attachments = []`. Beide zurückgelieferten Runden wurden vollständig gelesen. Das belegt die Auswertung des vom Werkzeug bereitgestellten Verlaufs, nicht einen unabhängigen vollständigen Export aller jemals existierenden Gesprächsversionen.

### 2.2 Fehlende oder begrenzt verfügbare Belege

- **Bilder und Anhänge:** Im zurückgelieferten Chat sind keine Bildnachrichten, Binärdateien oder herunterladbaren Anhänge verfügbar. Daher wurden keine historischen Chatbilder eingecheckt. Screenshots in externen Projekt-READMEs sind externe Referenzbilder und kein Ersatz für fehlende Chatbilder. Ob außerhalb des sichtbaren Verlaufs weitere Bilder existierten, ist nicht nachweisbar.
- **Historische Quellenverweise:** Die Antwort enthält bei DGNA und GeoAlarm lediglich interne Quellenmarker mit den Nummern 6 und 7. Deren ursprüngliche Ziel-URLs und damalige Quellenversionen wurden nicht geliefert. Die unten genannten Primärquellen wurden am 06.10.2026 separat geöffnet; sie rekonstruieren keine unsichtbare historische Recherche.
- **Chat-Metadaten:** Titel, ID und technische Zeitstempel sind verfügbar. Modellkennung, Zeitzone der ursprünglichen Oberfläche, vollständige Bearbeitungs-/Verzweigungshistorie und ein originaler Export fehlen. Die hier verwendete Zeitdarstellung wurde nach Europe/Berlin umgerechnet. Dass der Startzeitstempel der ersten Runde etwas vor dem `createdAt`-Wert liegt, wird unverändert als Metadatenabweichung erhalten.
- **Frühere Projektideen:** Die Fachantwort verweist auf ein bereits mit SXceiver belegtes Pi-System, ein geplantes Dispatch-System, eigene MQTT-Daten, Simulation und die Idee eines Funkgeräts aus Pi, SDR und Display. Die dazugehörigen ursprünglichen Nutzerfestlegungen stehen nicht im zugänglichen Fachdialog. Sie sind hier Kontextbezüge der Antwort und keine neu belegten Anforderungen.
- **Versionen und Betrieb:** Der Fachchat nennt keine geprüften NetCore-Commits, PRs, Gerätestände, Buildlogs, Paketversionen oder konkreten Betriebsprotokolle. Auch installierte Zielsysteme und aktuelle Konfigurationen wurden bei dieser Archivierung nicht abgefragt.

## 3. Ziel, Ausgangslage und verbindliche Entscheidungen

Die eigentliche technische Frage war, welche außerhalb von NetCore sichtbaren Erweiterungen als Inspiration dienen könnten. Die Antwort ersetzte die nicht belegbare Annahme über fremde ChatGPT-Nutzer durch öffentlich nachprüfbare Projektbeispiele. Sie enthielt ausdrücklich keine Behauptung, private Entwicklungsarbeiten anderer Nutzer einsehen zu können.

Die folgenden Grenzen und Präferenzen sind auseinanderzuhalten:

| Aussage | Einordnung |
|---|---|
| Öffentliche Projekte als Beispiele verwenden; keine vermeintlichen Einblicke in fremde Chats | Inhaltliche Klarstellung der Fachantwort |
| Anzeigen, Automationen und Bedienoberflächen möglichst an vorhandene Schnittstellen anbinden | Architekturpräferenz des Assistenten; kein bestätigter Implementierungsbeschluss |
| ESP32-Display und BlueStation-MMI besonders interessant | Empfehlung des Assistenten; keine verbindlich vereinbarte Reihenfolge |
| NetCore-Kompatibilität muss geprüft werden | Ausdrücklich genannte offene Voraussetzung |
| Archiv ausschließlich auf `Archiving` unter `Docs/archive/` erstellen und indexieren | Verbindlicher Nutzerauftrag |
| Bestehende Archive erhalten; keine Änderungen an Code, Roadmap oder anderen Pfaden | Verbindlicher Nutzerauftrag |
| Commit und normale Veröffentlichung ausführen; kein Merge, kein Force-Push | Autorisierte Archivierungsarbeit |
| Ideen, Planung, Code, Tests und Betriebsbestätigung trennen | Verbindliche Nachweisregel für diese Dokumentation |

Es gibt im Fachdialog keine später korrigierte Funktionsentscheidung. Die späteren Nachrichten betreffen die Archivierung. Die beim heutigen Repository-Abgleich erkannten Unterschiede werden in Abschnitt 6 gesondert dokumentiert und nicht rückwirkend in die Fachantwort hineingelesen.

## 4. Vollständige technische Ideensammlung des Fachchats

### 4.1 Home Assistant als Funknetzübersicht

**Historisch genannt:** `Nisbo/flowstation-mqtt` als separate MQTT-Bridge und `Nisbo/ha-flowstation` als zugehörige Home-Assistant-Karte.

Der beschriebene Datenumfang umfasst Zeitschlitze, aktive Gespräche, registrierte Funkgeräte, Gruppenzugehörigkeit, RSSI, Last Heard und SDS-Verlauf. Automatische MQTT-Discovery und getrennte Zustandsanzeigen für Bridge, FlowStation und Brew sollen die Übersicht und Fehlerzuordnung erleichtern.

Der wesentliche Architekturgedanke ist ein zusätzlicher Verbraucher der vorhandenen WebSocket-/HTTP-Schnittstellen. Für NetCore wurde eine Übernahme von Darstellungsprinzipien oder eine Anpassung an eigene MQTT-Daten vorgeschlagen. Ein Austausch der bestehenden NetCore-MQTT-Anbindung wurde nicht beschlossen.

**Status des Chat-Ergebnisses:** Idee für NetCore; extern dokumentiertes Beispiel. Keine Installation der Bridge/Karte, keine Prüfung einer tatsächlichen Home-Assistant-Entität und kein End-to-End-Test in diesem Chat.

### 4.2 Eigenständiges ESP32-Display / FlowStation CYD

**Historisch genannt:** Ein ESPHome-Beispiel im MQTT-Projekt für ein 2,8-Zoll-ESP32-Touchdisplay, erreichbar über WLAN/MQTT und für die Anzeige unabhängig von Home Assistant.

Die dargestellten Informationen sind Frequenzen, Zeitschlitze, Gesprächspartner und angemeldete Geräte. Bei einem Gespräch wechselt das Display in die Gesprächsansicht und erhöht die Helligkeit; im Leerlauf zeigt es die Geräteliste. SDS ist in diesem kleinen Layout ausdrücklich nicht vorgesehen.

Als Einsatzort wurden Rackfront und Arbeitsplatz genannt. Die eigenständige WLAN/MQTT-Verbindung würde für die Datenanzeige keine weiteren GPIO am SXceiver-Pi benötigen. Daraus folgt keine allgemeine Aussage über freie Pins einer konkreten SXceiver-Hardware.

**Übertragungsidee:** Ein NetCore-Display auf Basis derselben Bedienidee, aber mit einer bewusst definierten Zuordnung der NetCore-MQTT-Daten zu den Anzeigefeldern.

**Status:** Hervorgehobene Idee. Kein ausgewähltes oder beschafftes Board, kein NetCore-ESPHome-Projekt, kein Flashvorgang und kein Gerätetest im Chat.

### 4.3 BlueStation-MMI als Bedienoberfläche eines eigenen Funkgeräts

**Historisch genannt:** `misadeks/tetra-bluestation-mmi`, eine native Oberfläche aus Rust und Slint für den BlueStation-MS-Modus. Genannt wurden Gruppenordner, Scanlisten, Einzel-/Gruppenrufe, Kontakte, PTT, bidirektionale ACELP-Sprache und SDS-Unterhaltungen.

Die Oberfläche läuft getrennt vom MS-Funkstack und kommuniziert über WebSockets. Der Chat beschreibt variable Displaygrößen, Touch-/Tastenbedienung und Pi-Kioskbetrieb. Dies wurde mit dem Kontextbild eines Funkgeräts aus Pi, SDR und Display verbunden.

Ein Simulator kann nach der beschriebenen Architektur den Stack gegenüber der Oberfläche ersetzen. Die Antwort begrenzte dies ausdrücklich: Eine bedienbare MMI ohne RF-Hardware beweist weder einen vollständigen virtuellen Funkkanal noch Mehrzellenbetrieb oder Handover.

**Status:** Hervorgehobene Idee und extern beschriebene Anwendung. Kein MMI-Build, keine geprüfte Kombination aus NetCore-MS und externer MMI, keine Audio-/PTT-Abnahme.

### 4.4 SDS-Gateway für Skripte, Meldungen und Automationen

**Historisch genannt:** `SilentServices/flowstation-sds-cli`, ein eigenständiges Werkzeug am FlowStation-Dashboard-WebSocket.

Die beschriebenen Merkmale sind SDS-Versand aus Shellskripten und Automationen, Aufteilung längerer Texte auf mehrere Nachrichten mit Teilkennzeichnungen wie `[1/3]`, Berücksichtigung der Zeichenkodierung bei der Längenberechnung, ein Dry-Run-Modus und ein Home-Assistant-Beispiel.

Als konkrete Nebenideen wurden drei Betriebsereignisse vorgeschlagen:

- USV wechselt auf Batteriebetrieb.
- Die VPN-Verbindung zu einer Außenstelle fällt aus.
- Ein Deployment schlägt fehl.

Der Mehrwert wurde in einer brauchbaren Eingangsschnittstelle für andere Programme gesehen. Der Chat behauptete weder, dass NetCore bislang grundsätzlich keine SDS senden könne, noch dass das externe Skript bereits gegen NetCore funktioniere.

**Status:** Idee für eine Automationsschnittstelle. Keine im Chat ausgeführte SDS, kein erfolgreicher Dry-Run und keine überprüfte Funkzustellung.

### 4.5 DGNA und temporäre Arbeitsgruppen

**Historisch genannt:** FlowStation führt Dynamic Group Number Assignment als umgesetzt auf. Über das Dashboard sollen Gruppen über die Luftschnittstelle zugewiesen und wieder entzogen werden können.

**NetCore-Idee:** Ein Dispatch-Ablauf legt eine temporäre Arbeitsgruppe an und weist sie ausgewählten Geräten zu. Die Antwort unterschied diese Orchestrierung ausdrücklich von der einzelnen DGNA-Funktion. DGNA allein belegt keine vollständige Einsatz-, Benutzer- oder Berechtigungsverwaltung.

Offen blieben Gruppenlebenszyklus, Auswahlregeln, Rücknahme der Zuweisung, Fehlerrückmeldung und die Einbindung in vorhandene zentrale Dienste.

**Status:** NetCore-Idee auf Grundlage einer extern beschriebenen Funktion; kein im Chat ausgeführter DGNA-Auftrag.

### 4.6 GeoAlarm, Pager, Telegram und Snom/Asterisk

**Historisch genannt:** FlowStation wertet TETRA-LIP-Positionen aus. Beim Eintritt eines zugelassenen Geräts in einen konfigurierten Radius können SDS, Telegram-Meldungen und TPG2200-Call-Outs ausgelöst werden. Ergänzend wurden ein HTTP-Auslöser für TPG2200 sowie Nachrichtenanzeigen auf Snom-Telefonen über Asterisk genannt.

Die damalige Antwort hielt ausdrücklich fest, dass die in der betrachteten FlowStation-Dokumentation vorhandenen MeshCom-Schalter in diesem Build keine Funktion hatten.

**NetCore-Idee:** Aus dem Erreichen eines Standorts entsteht ein nachvollziehbares Ereignis im Leitstand. Die Antwort nennt drei besonders wichtige Prüfpunkte: veraltete Positionen, Mehrfachauslösungen und eindeutige Herkunft einer Meldung.

**Status:** Idee für einen standortabhängigen Leitstandsablauf. Keine geprüfte NetCore-Geofence-Konfiguration, kein realer LIP-Empfang, kein ausgelöster Pager und kein Telefon-/Telegram-Zustellnachweis aus diesem Chat.

## 5. Architektur und Abhängigkeiten der Ideen

### 5.1 Daten- und Steuerungswege

Die folgende Übersicht ordnet den historischen Vorschlägen ihre technischen Grenzen zu. Sie ist keine Aussage über eine in diesem Chat installierte Gesamtanlage.

| Anwendungsfall | Daten-/Steuerungsweg | Vor einer Umsetzung zu klären |
|---|---|---|
| Funkübersicht in Home Assistant | FlowStation-WebSocket/HTTP → externe Bridge → MQTT/Discovery → HA-Karte | NetCore-Datenschema, Entitäten, Zustandsalter und Bedeutung der Verfügbarkeitsanzeige |
| Eigenständiges Rackdisplay | MQTT-Broker → ESPHome/ESP32 → Anzeige/Touch | Topics und JSON-Felder, Mehrträgeranzeige, Offlineverhalten, konkrete Hardwarevariante |
| Eigenes Funkgerät | MS-Stack ↔ separate MMI über Control-/Telemetry-WebSockets | Anwendungsprotokollversion, Ruf-/PTT-Zustände, Audio, Provisionierung, echte RF-Anbindung |
| Automatische SDS | Externe Anwendung → SDS-Eingang → zuständige TBS → Endgerät | Routingautorität, Textkodierung, Segmentierung, Wiederholung, Annahme versus Empfang |
| Temporäre Gruppen | Dispatch-Ablauf → Gruppenverwaltung/DGNA → TBS → Funkgerät | Mitgliedschaften, Freigaben, Rücknahme und nachgewiesene Anwendung |
| Standortalarm | Positionsquelle → GeoAlarm-/Ereignislogik → SDS/Pager/Telegram/Snom | Aktualität, Quellenkennung, Eintritt versus Wiederholung, Zustellnachweis |

Die MMI und das MQTT-Display erfüllen unterschiedliche Aufgaben: Die MMI bedient einen MS-Stack einschließlich Steuerung; das CYD-Beispiel ist eine kompakte Anzeige von Basisstations-/Netzstatus. Ihr Einsatz wurde nicht als gemeinsame, bereits entworfene Hardwareplattform beschlossen.

### 5.2 Separat nachgelesene öffentliche Referenzen am 06.10.2026

Die folgenden Informationen ergänzen die historische Antwort. Es handelt sich um am Archivierungstag abrufbare Dokumentation, nicht um reproduzierte Builds oder Zusicherungen einer NetCore-Kompatibilität. Externe Branches wurden nicht als unveränderliche Commitstände archiviert.

- Die [FlowStation-MQTT-Bridge](https://github.com/Nisbo/flowstation-mqtt) beschreibt Python, einen laufenden FlowStation-Dienst, MQTT und Home Assistant für den HA-Anwendungsfall. Als Vorgaben nennt sie `ws://127.0.0.1:8080/ws`, MQTT-Port `1883` und den Topic-Präfix `flowstation`. Die [HA-Karte](https://github.com/Nisbo/ha-flowstation) liest den Sensor der Bridge statt selbst eine FlowStation-Verbindung zu öffnen.
- Das [CYD-README](https://raw.githubusercontent.com/Nisbo/flowstation-mqtt/main/display-examples/flowstation-cyd/README.md) konkretisiert das Beispiel auf ESP32-2432S028-kompatible Hardware, 320×240 Pixel, ST7789V-kompatible Anzeige und XPT2046-Touch. Es nennt ESPHome, 2,4-GHz-WLAN, einen erreichbaren Broker und Bridge-Version 1.1.0 oder neuer. Das Beispiel abonniert unter anderem `flowstation/bts`, `flowstation/slot/1` bis `slot/4` und `flowstation/registered`. Die aktualisierte Dokumentation bestätigt den Verzicht auf SDS im kleinen Layout. Diese Daten sind keine Auswahl einer konkreten NetCore-Hardware.
- Das [MMI-README](https://github.com/misadeks/tetra-bluestation-mmi) beschreibt die **MMI als WebSocket-Server** und den Stack oder Simulator als verbindungsaufbauenden Client. Genannt werden Control `9102`, Telemetry `9101`, die Subprotokolle `bluestation-control-v1`/`bluestation-telemetry-v1` sowie UTF-8-JSON in binären WebSocket-Frames. Das README spricht von `bluestation-ms-interface-2`; dieser Dokumentationswert ersetzt keine Prüfung der tatsächlich verwendeten Programmstände.
- Das [SDS-CLI-README](https://github.com/SilentServices/flowstation-sds-cli) nennt Bash, Python 3, GNU-`base64` und `websocat`. Der dortige Standard von `MAX_SDS_BYTES=140` ist eine konfigurierbare Interoperabilitätsgrenze des Hilfsprogramms, kein allgemeines TETRA-SDS-Maximum. Ebenso sind `/etc/flowstation-sds.conf` und `flowstation_send_sds.sh` externe Projektnamen, keine im Chat angelegten NetCore-Dateien.
- Die separat geöffnete [FlowStation-Projektbeschreibung](https://github.com/razvanzeces/flowstation) führt DGNA als vorhanden auf und beschreibt GeoAlarm mit TETRA-LIP sowie die dort wirkungslosen MeshCom-Schalter. Dies bestätigt eine heutige öffentliche Dokumentationsaussage über dieses externe Projekt; es belegt nicht den NetCore-Betriebszustand.

Die vier ausdrücklich benannten Repositorynamen und die zusätzlich aufgefundene FlowStation-Primärquelle werden als Quellen erhalten. Die ursprünglichen internen Quellenmarker 6/7 lassen sich dadurch nicht versionsgenau wiederherstellen.

## 6. Heutiger NetCore-Stand am geprüften Basiscommit

Dieser Abschnitt beruht auf einem frischen Checkout des vorhandenen Remote-Branches `Archiving`. Die Prüfung beschränkte sich auf einschlägige Dokumentation, Konfigurationsbeispiele, Quelltexte, Suchtreffer und Git-Metadaten. Es wurden keine Dienste gestartet und keine Testsysteme angesprochen.

### 6.1 MQTT und Home Assistant sind bereits eigene NetCore-Komponenten

Unter [`system-backend/iot-gateway/`](../../system-backend/iot-gateway/README.md) ist ein eigener IoT Gateway vorhanden. Das README beschreibt `netcore-event-v1`, `netcore-command-v1`, Policy-Prüfung und `netcore-command-ack-v1`. Der Quelltext enthält HTTP-Routen, MQTT-Verbindung/Subscriptions und Discovery-Erzeugung; die Funktion ist damit über eine bloße Roadmap-Erwähnung hinaus im Repository angelegt.

Relevante Nachweise:

- [`docs/mqtt-contract.md`](../../system-backend/iot-gateway/docs/mqtt-contract.md): NetCore-Topicvertrag, Retain-Regeln und HA-Discovery.
- [`config/iot-gateway.example.toml`](../../system-backend/iot-gateway/config/iot-gateway.example.toml): API-Bind `0.0.0.0:8240`, MQTT `127.0.0.1:1883`, Präfix `netcore/v1`, QoS 1, Ereignisse nicht retained und Zustände retained.
- [`src/http.rs`](../../system-backend/iot-gateway/src/http.rs), [`src/mqtt.rs`](../../system-backend/iot-gateway/src/mqtt.rs) und [`src/state.rs`](../../system-backend/iot-gateway/src/state.rs): Status-/Topic-/HA-Routen, MQTT-Verarbeitung und Discovery-Aufträge.

**Folgerung:** Die fremde Bridge, ihre HA-Karte und das CYD-Beispiel erwarten andere Topics und Payloads. Die bloße Verwendung von MQTT macht sie nicht direkt kompatibel. Für NetCore ist ein Feld- und Zustandsmapping erforderlich. Ob alle vom CYD erwarteten Werte bereits in geeigneter Form vorliegen, wurde nicht vollständig ermittelt.

Eine Suche nach den konkreten externen Projekt-/Produktnamen außerhalb des Archivs ergab MMI-Verweise, aber keinen entsprechenden Nisbo-/SDS-CLI-/CYD-/ESPHome-Nachweis. Das ist ein begrenzter Suchbefund, kein Beweis gegen anders benannte Eigenentwicklungen.

### 6.2 MS-Stack vorhanden, MMI extern referenziert, Dokumentationsversionen widersprüchlich

[`ms-mode/README.md`](../../ms-mode/README.md) und [`ms-mode/docs/MS_GETTING_STARTED.md`](../../ms-mode/docs/MS_GETTING_STARTED.md) verweisen ausdrücklich auf `misadeks/tetra-bluestation-mmi`. Die Einstiegshilfe beschreibt Control-Port `9102` und Telemetry-Port `9101` sowie die Verbindung vom Stack zur MMI.

Beim Versionsabgleich wurden drei unterschiedliche Angaben sichtbar:

| Fundstelle | Tatsächlich gelesene Angabe | Einordnung |
|---|---|---|
| [`ms-mode/examples/ms-interface/README.md`](../../ms-mode/examples/ms-interface/README.md) | `bluestation-ms-interface-1` | Ältere Dokumentationsangabe; widerspricht der heutigen Codekonstante |
| [`ms-mode/crates/tetra-entities/src/management/mod.rs`](../../ms-mode/crates/tetra-entities/src/management/mod.rs) | `MS_INTERFACE_SCHEMA_VERSION = "bluestation-ms-interface-6"` | Im geprüften NetCore-Quelltext gesetzter Anwendungs-Schemastand |
| Externes MMI-README, am 06.10.2026 gelesen | `bluestation-ms-interface-2` | Externe Dokumentationsangabe; kein geprüfter MMI-Laufzeitwert |

[`ms-mode/docs/MS_MODE.md`](../../ms-mode/docs/MS_MODE.md) nennt ebenfalls Interface-6 im SDS-/DTMF-Kontext. Diese konkrete Dokumentationsabweichung ist ein Fortsetzungsrisiko: Der Abgleich muss auf gepinnten Codeversionen und `GetInterfaceVersion` beruhen. Die WebSocket-Subprotokolle mit Suffix `-v1` sind eine andere Versionsebene und dürfen nicht mit dem Anwendungs-Schema gleichgesetzt werden.

**Nicht belegt:** Dass die heute externe MMI mit dem eingecheckten NetCore-MS-Stand inkompatibel ist. Ebenso wenig belegt ist ihre Kompatibilität. Unterschiedliche README-Angaben allein entscheiden das nicht.

Die Schema-Dokumentation wird im Rahmen dieses Archivauftrags nicht außerhalb von `Docs/archive/` repariert.

### 6.3 SDS Router als vorhandene Anschlussstelle für Automationen

Der [SDS Router](../../system-backend/sds-router/README.md) ist als zentraler Dienst vorhanden. Dokumentiert sind Individual-/Gruppen-SDS, pre-coded Status, Routing, TTL/Retry, Store-and-forward, Duplikaterkennung und eine Application-Outbox mit ACK/NACK.

Im [HTTP-Quelltext](../../system-backend/sds-router/src/http.rs) sind `GET/POST /api/v1/messages` sowie weitere Verwaltungsrouten implementiert. Die dortige API-Beschreibung nennt Idempotenz und trennt ausdrücklich TBS-Annahme von einem Empfangsnachweis des Handsets.

**Folgerung für die Chatidee:** Eine Automationsschnittstelle sollte zunächst den vorhandenen SDS Router und den tatsächlichen lokalen/zentralen Routingmodus prüfen. Das externe FlowStation-CLI darf nicht ungeprüft als alternative Sendeautorität eingebaut werden. Die quellseitige README nennt dafür `[control_room].central_sds_routing` als Umschalter zwischen lokaler und zentraler SDS-Logik.

Eine CLI-seitige Textsegmentierung mit `[1/n]`, echte SDS-Verkettung und die verlustfreie Weiterleitung bereits codierter SDS sind verschiedene Funktionen. Eine Gleichwertigkeit dieser Verfahren wurde nicht nachgewiesen.

### 6.4 DGNA-Bausteine und Gruppenverwaltung existieren

Der [Group Core](../../system-backend/group-core/README.md) enthält Gruppenstammdaten, Mitgliedschaften, Affiliationen und DGNA-Verfolgung. [`src/http.rs`](../../system-backend/group-core/src/http.rs) implementiert unter anderem `GET/POST /api/v1/dgna`; [`src/state.rs`](../../system-backend/group-core/src/state.rs) enthält die zugehörige Zustandsverwaltung.

Die [DGNA-Dokumentation](../../system-backend/group-core/docs/dgna.md) nennt Node-ID, ISSI, GSSI und Attach/Detach sowie die Korrelation von Node-Gateway-Request-ID, TBS-Command-ID und `GroupDgnaApplied`. Der vorhandene Operator-Override `force=true` umgeht laut Dokumentation Gruppen-/Mitgliedschaftsfreigaben, aber nicht Teilnehmerregistrierung, technische GSSI-Gültigkeit oder die DGNA-Fähigkeit der TBS.

**Abgrenzung:** Der aktuelle Code bietet damit eine Anschlussstelle für temporäre Dispatch-Gruppen. Ein vollständiger Gruppenlebenszyklus im Dispatch-System und eine erfolgreiche Übernahme durch reale Endgeräte sind damit noch nicht nachgewiesen.

Der [Task-Workflow-Dienst](../../system-backend/task-workflow/README.md) dokumentiert bereits strukturierte Aufträge, SDS-Benachrichtigung und MQTT-Ereignisse. Seine bloße Existenz belegt keine Implementierung der in diesem Chat skizzierten temporären Gruppenzuweisung.

### 6.5 GeoAlarm einschließlich angebundenem MeshCom-Eingang vorhanden

Relevante heutige Quellen sind:

- [`crates/tetra-config/src/bluestation/sec_geoalarm.rs`](../../crates/tetra-config/src/bluestation/sec_geoalarm.rs): Konfiguration und Validierung.
- [`crates/tetra-entities/src/net_geoalarm/mod.rs`](../../crates/tetra-entities/src/net_geoalarm/mod.rs): Positionsverarbeitung, Radiusentscheidung und Weiterleitung.
- [`crates/tetra-entities/src/net_meshcom/mod.rs`](../../crates/tetra-entities/src/net_meshcom/mod.rs): MeshCom-Eingang mit Aufruf von `send_meshcom_position_with_via`.
- [`bins/bluestation-bs/src/main.rs`](../../bins/bluestation-bs/src/main.rs): Start des GeoAlarm-/MeshCom-Workers und Zuführung von TETRA-LIP über `send_tetra_lip`.
- [`crates/tetra-entities/src/net_dashboard/server.rs`](../../crates/tetra-entities/src/net_dashboard/server.rs): TPG2200-ActionURL, DGNA-Dispatcher und Snom-Konfigurationsrouten.

**Wichtige Abweichung vom externen historischen Beispiel:** Die Aussage über wirkungslose MeshCom-Schalter bezog sich auf den betrachteten FlowStation-Build. Im geprüften NetCore-Code ist inzwischen ein MeshCom-Positionspfad angebunden. Daraus folgt weder dessen Aktivierung in einer realen Konfiguration noch ein erfolgreicher MeshCom-Empfang.

Zwei Details sind für die Fortsetzung besonders relevant:

1. Der Worker berechnet `inside && (!was_inside || cooldown_elapsed)`. Ein weiteres Positionsupdate innerhalb des Radius kann nach Ablauf des Cooldowns erneut auslösen; die Logik ist somit nicht auf ein einmaliges Eintrittsereignis beschränkt. Ein erneuter Eintritt erfüllt bereits die erste Alternative. Der Zeitpunkt der letzten Alarmierung wird bei mindestens einem erfolgreichen Weiterleitungspfad gesetzt.
2. Die betrachtete interne `GeoAlarmUpdate`-Struktur enthält Quelle, Breite und Länge, aber keinen Messzeitpunkt der ursprünglichen Position. Eine belastbare Ende-zu-Ende-Prüfung des Positionsalters ist dadurch an dieser Stelle nicht belegt. Mögliche Prüfungen weiter oben im Eingangspfad wurden nicht vollständig untersucht.

Der vorhandene TPG2200-HTTP-Auslöser prüft seine aktivierte Konfiguration und einen dedizierten Token. Tokenwerte wurden nicht gelesen, übernommen oder dokumentiert. Die Open-Lab-Einstellung anderer Dienste darf nicht pauschal auf diesen Endpunkt übertragen werden.

## 7. Relevante Schnittstellen, Pfade und Parameter

Die Zahlen und Pfade dieser Tabelle sind separat gelesene Repository- beziehungsweise externe Dokumentationswerte. Der historische Fachdialog legte keine konkreten NetCore-Hosts, IPs, Funkfrequenzen oder Netzkennungen fest.

| Bereich | Wert oder Pfad | Quelle und Grenze |
|---|---|---|
| NetCore IoT Gateway | TCP `8240`; `system-backend/iot-gateway/` | Konfigurationsbeispiel/README; kein aktiver Listener geprüft |
| MQTT | Beispielbroker `127.0.0.1:1883`, Topic-Präfix `netcore/v1` | NetCore-IoT-Beispiel; keine produktive Adresse |
| NetCore Ereignisse | `netcore/v1/events/<domain>/<action>` | MQTT-Vertrag; Ereignisse nicht retained |
| NetCore Zustände | `netcore/v1/state/<subject-type>/<subject-id>` | MQTT-Vertrag; Zustände retained |
| Befehle/Quittungen | `netcore/v1/commands/#`, `netcore/v1/acks/<command-id>` | Eigener NetCore-Vertrag; kein FlowStation-Payloadversprechen |
| HA Discovery | `homeassistant/<component>/netcore_tetra/<object-id>/config`; `homeassistant/status` | NetCore-MQTT-Vertrag |
| IoT-Persistenz | `/var/lib/netcore-iot-gateway/`, unter anderem Outbox und Command-Ledger | Im README dokumentiert, Dateisystem eines Live-LXC nicht geprüft |
| SDS Router | TCP `8150`; `GET/POST /api/v1/messages` | README und HTTP-Quelltext |
| Zentrale SDS-Umschaltung | `[control_room].central_sds_routing` | TBS-Konfiguration; tatsächlicher Zielwert unbekannt |
| Group Core | TCP `8110`; `GET/POST /api/v1/dgna` | Beispielkonfiguration und HTTP-Quelltext |
| Task Workflow | TCP `8280`; `system-backend/task-workflow/` | Repository-README; keine automatische DGNA-Kopplung belegt |
| MS-MMI Control | TCP `9102`; `[control]` | MS-Einstiegshilfe und externe MMI-Dokumentation |
| MS-MMI Telemetry | TCP `9101`; `[telemetry]` | Gleiche Quellen; Stack baut Verbindung zur MMI auf |
| MS-Anwendungsschema | `bluestation-ms-interface-6` | NetCore-Codekonstante; tatsächlichen Prozess gesondert befragen |
| GeoAlarm | `[geoalarm]`, unter anderem Radius, Cooldown, Quelle und Weiterleitungsziele | Konfigurations-/Worker-Code; keine Live-Parameter festgelegt |
| TPG2200-Auslöser | `/api/action/tpg2200`, `[tpg2200_action]` | Dashboard-Code; aktivierte Konfiguration und Token nötig |
| Externe FlowStation-Bridge | Vorgabe `ws://127.0.0.1:8080/ws`, Topic-Präfix `flowstation` | Externe Dokumentation; nicht mit NetCore-Core-Ports gleichsetzen |
| Externes CYD-Beispiel | `display-examples/flowstation-cyd/flowstation-cyd.yaml` | Datei im Nisbo-Projekt; nicht im Archiv installiert |

Die gelesenen NetCore-Core-Dokumentationen verwenden teilweise ausdrücklich einen isolierten Open-Lab-Betrieb ohne Login, Token und TLS. Dies ist eine Eigenschaft der betreffenden Beispiele, keine in diesem Fachchat getroffene Sicherheitsentscheidung und keine pauschale Beschreibung aller NetCore-Endpunkte.

## 8. Entwicklungs-, Test- und Betriebsstand

| Gegenstand | Idee | Beschlossen/geplant | Implementiert | Getestet | Im Betrieb bestätigt |
|---|---|---|---|---|---|
| NetCore-Übernahme der HA-Karte/Bridge | Im Chat vorgeschlagen | Kein Umsetzungsbeschluss | Konkrete Übernahme nicht nachgewiesen; eigener IoT Gateway vorhanden | Keine Integrationsprüfung | Nein |
| NetCore-CYD-/ESP32-Display | Besonders empfohlen | Keine Board-/Firmwareentscheidung | Kein spezifischer Implementierungsnachweis in dieser Prüfung | Kein Flash-/Displaytest | Nein |
| Nutzung der BlueStation-MMI mit NetCore-MS | Besonders empfohlen | Keine verbindliche Version-/Hardwarewahl | MS-Stack und externe Referenz vorhanden; MMI-Kombination nicht abgenommen | Kein Build-/Protokoll-/Audiotest | Nein |
| Betriebsalarme per SDS | Drei konkrete Beispiele genannt | Kein verbindlicher Automationsplan | SDS Router vorhanden; genannte Automationen nicht nachgewiesen | Keine Ende-zu-Ende-Zustellung | Nein |
| Temporäre Dispatch-Gruppen | Vorgeschlagen | Gruppenlebenszyklus offen | DGNA-/Group-Core-Bausteine vorhanden; vollständiger Ablauf unbelegt | Kein realer Geräteversuch | Nein |
| Standortereignisse im Leitstand | Vorgeschlagen | Semantik/Aktualität offen | GeoAlarm und Eingangs-/Ausgangspfade im Code vorhanden | Keine Positions-/Pager-/Telefonabnahme | Nein |
| Diese Abschlussdokumentation | Vom Nutzer angefordert | Pfad, Branch und Umfang ausdrücklich festgelegt | Archivdatei erstellt und Index ergänzt | Lokale Datei-, Link- und Pfadprüfung bestanden; Remote-Ergebnis in der Abschlussmeldung | Keine Aussage über Funkbetrieb |

„Implementiert“ in der Tabelle bedeutet ausschließlich, dass passende Quelltexte vorgefunden wurden. Es bestätigt keinen erfolgreichen Build, keine vollständige Funktionsabdeckung und keinen installierten Stand. „Nein“ bei Betriebsbestätigung bedeutet fehlenden Nachweis aus diesem Chat und dieser Archivprüfung, nicht die Behauptung, dass die Funktion nirgendwo betrieben wird.

## 9. Befehle, Abläufe und Fehler

### 9.1 Historischer Fachchat

Es wurden keine Installations-, Reparatur-, Deployment- oder Sendebefehle protokolliert. Die Antwort erwähnte einen Dry-Run-Modus des externen SDS-Werkzeugs, führte ihn aber nicht aus. Es gibt weder einen historischen Fehlerbericht mit Diagnose noch eine im Chat erfolgreich angewandte Reparatur.

Die in externen READMEs enthaltenen Installationsanweisungen sind keine in diesem Chat ausgeführten Abläufe. Sie wurden nicht zu einer ungeprüften NetCore-Installationsanleitung zusammenkopiert.

### 9.2 Tatsächliche Schritte der Archivvorbereitung

Die folgenden Schritte wurden im aktuellen Archivierungsauftrag tatsächlich ausgeführt:

1. Verfügbaren Chat nachladen und die fehlende Paginierung/Anhangsliste prüfen.
2. Existenz und Head von `refs/heads/Archiving` direkt am Remote prüfen.
3. Einen separaten Checkout des vorhandenen Branches anlegen.
4. Nach vorhandenem Archiv mit dieser Chat-ID/diesem Titel suchen; kein eindeutiger bestehender Eintrag gefunden.
5. Vorhandenen Index lesen und die einschlägigen Repository-Dateien prüfen.
6. Remote unmittelbar vor dem Schreiben erneut abrufen; Basiscommit unverändert.
7. Ausschließlich diese neue Archivdatei schreiben und den zugehörigen Indexeintrag ergänzen.
8. Alle 30 lokalen Dateiverweise der Dokumentation erfolgreich auflösen; den bisherigen Indexinhalt unverändert erhalten und genau eine neue Zeile feststellen. Die geänderten beziehungsweise neuen Dateien entsprechen exakt den zwei erlaubten Archivpfaden. Die lokale Whitespace-Prüfung bestand.

Beim frischen Windows-Checkout trat für ein bereits vorhandenes langes Archiv-Asset der Fehler `Filename too long` auf. Ursache war die Pfadlängenbehandlung in Git für Windows. Im ausschließlich für diesen Auftrag neu angelegten Checkout wurde `core.longpaths=true` gesetzt und der Checkout aus `HEAD` wiederhergestellt. Anschließend war der Arbeitsbaum sauber. Kein existierendes Archiv wurde umbenannt oder gekürzt; diese lokale Git-Einstellung ist keine Änderung einer versionierten Repository-Datei.

### 9.3 Prüf- und Veröffentlichungsablauf

Diese Befehle beschreiben den vorgesehenen Abschlussablauf im separaten Checkout. Die Commit-/Push-Ergebnisse und der resultierende Hash sind anhand der Git-Historie und der Abschlussmeldung zu prüfen; die Befehlsliste allein ist kein Ausführungsnachweis.

```bash
git branch --show-current
git rev-parse HEAD
git status --short --branch
git fetch origin Archiving
git diff --name-only
git diff --check
git add -- Docs/archive/2026-10-06_flowstation-bluestation-erweiterungsideen-mqtt-cyd-mmi-sds-dgna-geoalarm.md Docs/archive/README.md
git diff --cached --name-only
git diff --cached --check
git commit -m "docs(archive): document FlowStation and BlueStation extension ideas"
git push origin HEAD:refs/heads/Archiving
git ls-remote origin refs/heads/Archiving
git show --stat --oneline HEAD
git status --short --branch
```

Vor dem Commit darf die Liste der Änderungen ausschließlich die zwei vorgesehenen Archivpfade enthalten. Falls der Remote-Branch zwischenzeitlich weiterläuft, müssen dessen Änderungen erhalten und die eigenen Archivänderungen auf dem neuen Stand erneut geprüft werden. Ein abgewiesener Push rechtfertigt keinen Force-Push und keinen Merge.

Nach dem Push sind der Remote-Hash und beide Dateien am veröffentlichten Commit erneut zu lesen und mit den lokalen Git-Objekten beziehungsweise Inhalten zu vergleichen.

## 10. Prüfungen und ihre Grenzen

| Prüfung | Ergebnis bzw. Aussagegrenze |
|---|---|
| Sichtbarer Verlauf | Zwei vollständig lesbare Dialogrunden; keine weitere Seite gemeldet |
| Bild-/Anhangsverfügbarkeit | Leere Anhangsliste und keine Bildnachrichten; kein Bildimport möglich |
| Bestehender Remote-Branch | `Archiving` vorhanden; Basis `9fc9cc282023afe28f967ca1e236add012209935` live geprüft |
| Kollisionsprüfung Archiv | Kein eindeutiger Treffer auf Chat-ID/Titel; neue themenspezifische Datei erstellt |
| Repository-Implementierungen | Quelltext-/Dokumentationsprüfung für IoT, SDS Router, Group Core, MS und GeoAlarm |
| Öffentliche Referenzen | Primär-READMEs am 06.10.2026 gelesen; keine externe Versionsfixierung oder Installation |
| Build/Unit-/Integrationstests | Für die Funktionsideen in diesem Auftrag nicht ausgeführt |
| Live-LXC/MQTT/HA/SDR/Funkgeräte | Nicht angesprochen; keine Betriebs- oder Empfangsbestätigung |
| Lokale Archivverifikation | Alle 30 lokalen Dateiverweise gültig; bestehender Index unverändert, genau eine neue Zeile; ausschließlich die beiden erlaubten Archivpfade geändert; Whitespace-Prüfung bestanden |
| Veröffentlichung und Remote-Verifikation | Nach Commit/Push werden Branch-Hash und beide gespeicherten Dateien zurückgelesen; der konkrete Veröffentlichungsnachweis und Commit stehen in der Abschlussmeldung |

Die Archivänderung erfordert keine Veränderung oder Inbetriebnahme der Laufzeitdienste. Ein erfolgreicher Markdown-/Git-Abgleich ist kein Test des Funknetzes.

## 11. Abgegrenzte, nicht beschlossene und überholte Annahmen

- **Fremde ChatGPT-Projekte:** Keine personenbezogenen oder privaten Entwicklungsinformationen verfügbar; die Fachantwort stützt sich auf öffentliche Beispiele.
- **Direkte Austauschbarkeit:** Weder die FlowStation-MQTT-Nachrichten noch der Dashboard-WebSocket wurden als kompatibel mit NetCore abgenommen.
- **Weitere Protokollanbindungen als Selbstzweck:** Der Assistent hob eigenständige Anzeige und MMI hervor; daraus folgt kein generelles Verbot weiterer SIP-, Telegram- oder MQTT-Arbeiten.
- **Simulation gleich Handover:** Ein Simulator am MMI-Vertrag ersetzt keinen bewiesenen RF-/Timing-/Mehrzellenpfad.
- **DGNA gleich Einsatzverwaltung:** Einzelne Gruppenzuweisung und vollständiger Dispatch-Gruppenlebenszyklus bleiben getrennte Anforderungen.
- **MeshCom generell ohne Funktion:** Für den untersuchten externen FlowStation-Build dokumentiert, auf den heutigen NetCore-Code nicht pauschal übertragbar.
- **Interface-1 als aktueller MS-Vertrag:** Durch die heutige Codekonstante Interface-6 für diesen Repository-Stand überholt. Die alte Dokumentation bleibt als erkannte Abweichung benannt.
- **Archivdelegation gleich Veröffentlichung:** Die historische Übergabeantwort belegt keine abgeschlossene Speicherung; dafür sind die tatsächlichen Git- und Remote-Ergebnisse maßgeblich.

## 12. Offene Aufgaben und Roadmap-Kandidaten

Die folgende Reihenfolge ist ein Vorschlag aus der Archivbewertung. Sie ist keine im ursprünglichen Fachchat vereinbarte Roadmap und ändert keine Roadmap-Datei außerhalb dieses Verzeichnisses.

| Kandidat | Konkreter nächster Schritt | Voraussetzung und Nachweis für Abschluss |
|---|---|---|
| Gemeinsame Kompatibilitätsbasis | NetCore- und externe Projektcommits pinnen; installierten TBS-/Core-/MS-Stand erfassen | Reproduzierbare Matrix aus Code, Konfiguration und Schnittstellenschema |
| MQTT-/HA-Darstellung | Erwartete Bridge-/Kartenfelder gegen NetCore-Ereignisse und Zustände abbilden | RSSI, Gruppen, Gespräche, Zeitschlitze, Last Heard, SDS und Verfügbarkeiten mit realen Daten prüfen |
| ESP32-Rackdisplay | Konkrete CYD-Hardware und Displaycontroller festlegen; NetCore-MQTT-Mapping und Firmwareprototyp erstellen | Offline-/Reconnect-/veraltete-Daten-Verhalten, Anrufwechsel, Helligkeit und Mehrträgerdarstellung abnehmen |
| MMI-Prototyp | Gepinnte MMI gegen tatsächliches `GetInterfaceVersion` und NetCore-MS testen; Dokumentationsabweichung separat korrigieren | Control/Telemetry, Kontakte, Gruppen/Scanlisten, PTT, SDS und Audio zuerst im Simulator, danach mit SDR/Endgerät nachweisen |
| SDS-Automationszugang | Bestehenden Router als Sendeweg festlegen; USV-, VPN- und Deployment-Ereignisse konkret abbilden | Kodierung/Segmentierung, Idempotenz, TTL, Retry und Empfang auf dem Zielgerät prüfen |
| Temporäre Gruppen | Lebenszyklus von Anlage, Zuweisung, Rückmeldung und Rücknahme über Group Core spezifizieren | Nur geeignete/erlaubte Teilnehmer, Offlinefälle, Fehlermeldung und reale DGNA-Übernahme belegen |
| Standortabhängiger Leitstandsablauf | Positionszeit/-herkunft und Eintritts-/Wiederholungsregeln definieren; vorhandenen GeoAlarm verwenden | Veraltete/mehrfache Positionsmeldungen, Quellenwechsel, Radiusgrenze und tatsächliche Ausgangszustellung testen |
| Simulation | MMI-Simulator von virtuellem Funkmedium und Mehrzellenorchestrierung abgrenzen | Gesonderte Tests für Registrierung, Reselection/Handover und Sprachkontinuität statt Ableitung aus sichtbarer UI |
| Historische Belege | Bei später verfügbarem Originalexport Quellenmarker und mögliche Bilder nachtragen | Nur eindeutig diesem Chat zuordenbare, tatsächlich verfügbare Originale ergänzen |

Die konkrete Ausführung dieser Funktionsarbeiten war nicht Teil des Archivauftrags. Besonders das MQTT-Mapping und die MS-Schemaversion sind vor einer Portierung zu klären. Bereits vorhandene NetCore-Komponenten bieten dabei Anschlussstellen; deren Codepräsenz ersetzt keine Abnahme.

## 13. Quellen- und Fortsetzungsregister

### 13.1 Gespräch und öffentliche Primärquellen

- [Originalchat „Ideen zu Flowstation Bluestation“](https://chatgpt.com/c/6ab41782-2230-83eb-9a4d-22ed9be29690); zwei zugängliche Runden, Fachdialog vom 23.09.2026, Archivauftrag vom 06.10.2026.
- [Nisbo/flowstation-mqtt](https://github.com/Nisbo/flowstation-mqtt).
- [Nisbo/ha-flowstation](https://github.com/Nisbo/ha-flowstation).
- [FlowStation CYD – separat gelesene Dokumentation](https://raw.githubusercontent.com/Nisbo/flowstation-mqtt/main/display-examples/flowstation-cyd/README.md).
- [misadeks/tetra-bluestation-mmi](https://github.com/misadeks/tetra-bluestation-mmi).
- [SilentServices/flowstation-sds-cli](https://github.com/SilentServices/flowstation-sds-cli).
- [razvanzeces/flowstation](https://github.com/razvanzeces/flowstation), separat aufgefundene Primärquelle für DGNA/GeoAlarm; historische Quellenmarker bleiben unaufgelöst.

### 13.2 Repository und verwandte Archive

- [Geprüfter Repositorystand auf Commit `9fc9cc282023afe28f967ca1e236add012209935`](https://github.com/JanHG98/netcore-tetra/tree/9fc9cc282023afe28f967ca1e236add012209935).
- [Archivindex](README.md).
- [Home Assistant, MQTT und SDS-Diagnose](2026-10-05_home-assistant-proxmox-mqtt-sds-diagnose.md).
- [Virtuelle TBS, Funkgeräte und SimAir](2026-10-05_virtuelle-tbs-funkgeraete-simair-handover-testlabor.md).
- [NetCore Dispatch / IDECS](2026-10-05_netcore-dispatch-idecs-arbeitsplatz-rbac-audio-und-adaptive-ui.md).
- [Dashboard, Wiki und DGNA-Gruppenanzeige](2026-10-03_basisstation-dashboard-deutsch-integrationen-wiki-dgna-gruppennamen.md).
- [SDS-Services, Gateways und Bot-Architektur](2026-10-03_flowstation-sds-services-gateways-und-bot-architektur.md).
- [SXceiver GPIO, Sensorik und TFT](2026-10-05_sxceiver-gpio-sensorik-tft-und-modularer-hardwareausbau.md).

Die verwandten Archive sind Fortsetzungsverweise auf vorhandene Dateien. Ihre Inhalte werden nicht als Teil dieses ursprünglichen Fachdialogs ausgegeben. Für diesen Chat sind keine historischen Implementierungscommits oder PR-Nummern belegt. Zugangsdaten und nicht verfügbare Anhänge wurden nicht ergänzt.
