# Brainstorming: WERMA-Signalleuchte – Relais-Pi, API und Bootanzeige

> **Ergebnis der Planung:** Vorhanden ist eine WERMA-Leuchte mit Rot, Orange, Grün, Blau und Weiß. Ausdrücklich übernommen wurden die Farbbedeutungen mit Prioritäten und die Architektur eines **eigenen Raspberry Pi mit Relaiskarten und API-Steuerung**. REST/FastAPI, optionale MQTT-Anbindung, TTL, Blinkmuster, Override und sehr frühe Bootanzeige wurden als technische Ausgestaltung diskutiert.
>
> **Nachweisstand:** Im verfügbaren Fachverlauf wurde weder eine fertige Anwendung geliefert noch eine Installation, ein Hardwaretest oder ein produktiver Betrieb bestätigt. Der damalige Python-Kern ist ein unvollständiges, fehlerhaftes Prinzipbeispiel. Am 04.10.2026 wurden relevante Repository-Komponenten geprüft; vorhandene Telemetrie, Health, IoT-Kommandos und Hardware-Gateway sind Integrationsbausteine, aber kein Nachweis einer fertig implementierten WERMA-Ausgangssteuerung.

## Zielbild und Festlegungen

- Fünffarbige WERMA-Leuchte auf **separatem Raspberry Pi mit Relaiskarten**; API-Steuerung aus Basisstation/Zentrale.
- Festgelegt sind Farbbedeutungen und Prioritäten. REST/MQTT sollen dieselbe Zustandsmaschine nutzen.
- TTL, Blinkmuster, Override und frühe Bootanzeige sind Entwurfsbestand; 24 V und konkrete GPIOs sind noch zu verifizieren.
- Das Python-Prinzipbeispiel ist fehlerhaft. Offen sind funktionsfähige Treiber/API, GPIO-Übergabe beim Boot und Hardwareabnahme.

## 1. Arbeitsstand und Quellenbasis

| Feld | Inhalt |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Fünffarbige WERMA-Leuchte; eigener Pi mit Relais; REST/MQTT; Priorität/TTL; Blinkmuster; Override; frühe Bootanzeige und Bereitschaftsübergabe |
| Historische Datierung | Eine zusätzliche Kontextsuche ordnet die konkrete Farb-/Pi-Abstimmung und Bootentwurf dem **17.10.2025 UTC / 18.10.2025 Europe/Berlin** zu. Die verfügbaren Planungsnotizen enthalten keine vollständigen Originalzeitstempel; dies ist keine Prüfung eines vollständigen Exports. |
| Erstellungsdatum dieser Dokumentation | **2026-10-04**, Europe/Berlin; lokales Ausführungsdatum geprüft. |
| Zielrepository | [JanHG98/netcore-tetra](https://github.com/JanHG98/netcore-tetra) |
| Geprüfter Eingangscommit von Archiving | **11136de497f939bf3056dbccc4004a58ca99f344** |
| Tree dieses Commits | **2574924c5585cc0ad490a91738e5ce44fa6ff89b** |
| Zusätzlich lesend geprüfter Default-Branch | **main** bei **7137e0dd69877e1b604bf89148fd8b6b590c1a97** |
| Tree von main | **e68558c4df13d3d8b56df8c3611ac682b02889c1** |
| Ablage | **Docs/archive/2026-10-04_werma-signalleuchte-api-relais-pi-und-bootanzeige.md** |
| Index | **Docs/archive/README.md** |

### 1.1 Zugänglicher Verlauf

Die Planung umfasst fünf WERMA-Farben, ein Prioritätsmodell und die Festlegung auf einen **separaten Raspberry Pi mit Relaiskarten und API-Steuerung**. Das Implementierungsbriefing enthält GPIO-Beispiel, Endpunkte, Konfiguration, Treiberabstraktion, Zustandsmaschine, systemd-Dienst und Akzeptanzkriterien.

Als Erweiterung wurden Bootanzeige und deren Übergabe an den Lampendienst mit Device-Tree-GPIO-Hog, systemd-early und `rc.local` untersucht. Die Farb-/Pi-Festlegung lässt sich dem **17.10.2025 UTC / 18.10.2025 Europe/Berlin** zuordnen; vollständige Originalzeitstempel fehlen. Ein zusätzlicher Umsetzungs- oder Testbericht liegt nicht vor.

### 1.2 Quellenlücken und Anhänge

Nicht verfügbar beziehungsweise nicht belegt sind:

- Ein vollständiger Entwurfsbestand mit verlässlichen Zeitstempeln.
- WERMA-Artikelnummer, Typenschild, Anschlussplan, tatsächliche Versorgungsspannung, Lastströme und interne Blinkfunktion.
- Pi-Modell, Betriebssystemversion, Relaiskartentyp, Eingangsschaltung, active-high/active-low und echte Verdrahtung.
- Installierte Anwendung, Konfiguration, systemd-Status, GPIO-Line-Belegung und API-/MQTT-Laufzeitdaten.
- Ein tatsächlich geliefertes Skeleton-ZIP, ein Device-Tree-Overlay oder eine kompilierte DTBO-Datei.
- Ein spezifischer historischer WERMA-Implementierungscommit oder eine zugehörige PR.
- Eigenständige Bilder dieser Planung. Die verfügbaren Planungsunterlagen enthalten keine Bilder; die bereitgestellten lokalen Anhänge sind 25 PDFs, keine Bilddateien.

Die 25 PDFs wurden lokal geöffnet beziehungsweise über Textauszug/Metadaten geprüft. **Deckblätter und Seitenzahlen** wurden erfasst; die Dokumente wurden **nicht vollständig normativ durchgearbeitet**. Ihre Themen betreffen TETRA, ISI, SIM/UICC, Zusatzdienste, Luftschnittstelle, Security, Codec und Konformität. Sie sind keine gerätespezifischen WERMA-/Relais-Anschlussunterlagen. Das Deckblatt von ETSI.pdf identifiziert nur den ersten enthaltenen Standard; bei 4100 Seiten wird das Gesamt-PDF nicht als einzelner 156-seitiger Standard behandelt.

Aus den PDFs wurden keine projektspezifischen Lampenfarben, GPIO-Pins oder Bootverfahren als normativ vorgeschrieben abgeleitet. TETRA-Funkrufpriorität, beispielsweise PPC, ist nicht die lokale Anzeigepriorität 90/70/60/50/40.

**Bildarchiv:** Für diese Planung konnten keine eigenständigen Bilder nach Git übernommen werden, weil keine entsprechenden Bildquellen verfügbar sind. Bilder aus anderen Projektarchiven wurden nicht dieser Planung zugeschrieben. Eingebettete Normgrafiken wurden nicht als vermeintliche Originalbilder extrahiert.

### 1.3 Statusbegriffe

| Status | Verwendung in dieser Dokumentation |
|---|---|
| **Idee** | Vorgeschlagen oder erfragt; keine ausdrückliche endgültige Umsetzungsauswahl. |
| **Beschlossen/geplant** | Expliziter Planungsziel beziehungsweise übernommene Anforderung; noch kein Code- oder Betriebsnachweis. |
| **Implementiert** | An einem benannten Repository-Stand als Code/Konfiguration vorhanden; noch kein Funktions- oder Deploymentnachweis. |
| **Getestet** | Ein bestimmter Test wurde tatsächlich durchgeführt und sein Ergebnis erfasst. |
| **Im Betrieb bestätigt** | Konkreter erfolgreicher Betrieb ist durch Laufzeitdaten oder eine entsprechende Betriebsbestätigung belegt. |

Eine Empfehlung ist kein automatisch verbindlicher Projektbeschluss. Ein beschriebenes Akzeptanzkriterium ist kein bestandener Test.

## 2. Ziel, Ausgangslage und behandelte Themen

Die vorhandene WERMA-Leuchte soll den Zustand der NetCore-Tetra-Basisstation sichtbar anzeigen. Die Anzeige soll fachlich nachvollziehbar sein und per API aus der Basisstation, einer Zentrale beziehungsweise weiteren autorisierten Komponenten gesteuert werden können.

Die später ausdrücklich gewählte Trennung ist:

- Die Basisstation beziehungsweise zentrale Dienste liefern Statusereignisse oder API-Aufträge.
- Ein **eigener Pi mit Relaiskarten** betreibt die Lampensteuerung.
- Die tatsächliche Ausgabe und Blinktaktung laufen auf diesem I/O-Pi.
- Das Lampenverhalten wird über Priorität, TTL und definierte Zustände geregelt.

Dies vermeidet eine Abhängigkeit der physischen Lampenausgänge von frei verfügbaren GPIOs des SDR-/Basisstations-Pi. Die Planung liefert allerdings keinen Vergleichstest zur Ausfallisolation und keinen Nachweis einer sicherheitsgerichteten Anzeige.

Behandelt wurden: Farbmodell, automatische Trigger, REST, optionale MQTT-Steuerung, Authentifizierung, Rollen, manuelle Eingriffe, Konfliktauflösung, GPIO-Treiber, Konfiguration, systemd-Betrieb, Implementierungsübergabe und sehr frühe Bootsignalisierung.

## 3. Verlauf und endgültige Festlegungen

| Reihenfolge | Aussage/Änderung | Verbindlichkeit und Ergebnis |
|---|---|---|
| Ausgang | WERMA mit Rot, Orange, Grün, Blau, Weiß realistisch integrieren. | Planungsziel; Lampenbesitz und Farben genannt. |
| API-Frage | API-Steuerung wird gewünscht/geprüft. | Später ausdrücklich Teil der gewählten Lösung. |
| Erste API-Skizze | Lokaler Node-Dienst; REST/MQTT; JWT/TLS; TTL; Lock-Stack; Fernsteuerung. | Entwurf. Begriffe Node/Lighthouse/Watchtower sind keine hier eingerichteten Dienste. |
| Farbmodell | Fünf Farben mit konkreten Prioritäten und Blinkmustern. | Als Farb-/Prioritätsmodell festgelegt. |
| Hardwarekorrektur | Eigener Pi mit Relaiskarten, API-Steuerung. | **Beschlossen/geplant**; ersetzt die anfängliche Interpretation eines GPIO-Dienstes direkt auf der Funk-Node. |
| Übergabeformat | Zusammenfassung soll die Implementierung ermöglichen. | Briefing geliefert; keine Implementierung dadurch nachgewiesen. |
| Spätere Bootfrage | Beim Booten früh einschalten; nach fertigem Start wieder ausschalten. | **Gewünschte Erweiterung**. Keine abschließende Auswahl zwischen Device Tree, systemd und rc.local. |

### 3.1 Ausdrücklich übernommenes Farbmodell

| Farbe | API-Key | Bedeutung | Historisches Default-Verhalten | Priorität |
|---|---|---|---|---:|
| Rot | red | Kritisch/Alarm/Störung, z. B. Überhitzung, Watchdog oder TX-Fehler | Dauerlicht bis zur Aufhebung/Quittierung; schnelles Hard-Fault-Blinken nur als Zusatzidee | **90** |
| Orange | orange | TX aktiv / Sprachruf / Sendung | Blinken beziehungsweise Rechteckpuls **300 ms an / 300 ms aus** | **70** |
| Blau | blue | OTA, Synchronisation, Wartung | Langsam blinken **800 ms an / 800 ms aus** | **60** |
| Grün | green | Normalbetrieb/Healthy; in der frühen Beschreibung zusätzlich VPN-Verbindung | Dauerlicht | **50** |
| Weiß | white | Boot/Initialisierung | Zunächst Dauerlicht mit **5000 ms TTL**; später Wunsch „bis fertig gestartet“ | **40** |

**Grundregel:** Rot > Orange > Blau > Grün > Weiß. Die höher priorisierte Anzeige soll die niedriger priorisierte verdrängen; Rot soll die anderen Farben abschalten.

Die Bedeutungen sind projektspezifisch. Der Name **orange** bleibt in dieser Entwicklungsphase erhalten. Das abweichende gelb/blue/white-Modell eines anderen Rack-Entwurfs wird nicht stillschweigend hier hineingemischt.

### 3.2 Begründungen und noch offene Semantik

- Rot vermittelt einen Zustand mit unmittelbarem Klärungsbedarf und soll nicht gleichzeitig durch grünes „OK“ widersprochen werden.
- Orange zeigt Aktivität; Blau eine vorübergehende Betriebsphase; Grün den gesunden Normalzustand; Weiß den noch laufenden Start.
- TTL soll veraltete, nicht erneuerte Anzeigen begrenzen und nach Ablauf den weiterhin gültigen Grundzustand freigeben.
- Ein manueller Override soll gezielte Bedienung ermöglichen. Seine Priorität **100** wurde im Implementierungsbriefing vorgeschlagen, aber nicht separat hinsichtlich Alarmunterdrückung geklärt.
- MQTT soll optional bleiben; API-Steuerung ist gesetzt, die technische Ausführung mit FastAPI/Uvicorn ist der historische Implementierungsvorschlag.

**Widerspruch beim Booten:** Ein festes Weiß von fünf Sekunden beweist keinen fertigen Start. Die spätere Frage verlangt eine Bereitschaftsabhängigkeit. Daher darf die 5-s-TTL nicht als endgültiges Kriterium für Bootabschluss übernommen werden. Ob sie als zusätzlicher Kurztest oder Fallback bleiben soll, ist offen.

**Widerspruch bei Rot/Override:** Ein Override mit 100 kann Rot mit 90 verdecken; zugleich fordert der Entwurf „Rot blockiert alles bis quittiert“. Vor Umsetzung ist zu entscheiden, welche Eingriffe einen aktiven Alarm verdecken dürfen. Quittierung, Fehlerbeseitigung, Override-Ende und globales Ausschalten sind verschiedene Aktionen.

**Widerspruch bei TX:** Das Farbschema nennt sowohl „TX aktiv“ als auch „Sprachruf“. TETRA-Sendeträger/MCCH, ein bestehender Call, momentanes Sprechrecht und echte Sprachframes sind verschiedene Zustände. Ein dauernd sendender Träger darf nicht versehentlich dauernd die Aktivitätsanzeige auslösen, wenn Orange eigentlich Sprachaktivität bedeuten soll.

### 3.3 Historische TTL-Empfehlungen und ihre Grenzen

| Zustand | Frühe Empfehlung | Einordnung für die spätere Umsetzung |
|---|---|---|
| Weiß | 5000 ms | Kurzer Start-/Initialisierungshinweis; nach der späteren Bootfrage kein ausreichendes Readinesskriterium. |
| Orange | 5–10 s nach Ende; andere Beispiele beginnen mit TTL 10000 ms bei tx_start. | Unterschied zwischen Nachlauf und Lease während eines Calls nicht abschließend festgelegt. Bei längeren Calls ist Renewal oder ein explizites tx_end nötig. |
| Blau | Bis zum Ende des OTA-/Sync-Jobs | Explizites Beenden oder erneuerte Lease erforderlich; unbegrenztes veraltetes Blau bei verlorener Endmeldung vermeiden. |
| Grün | Ohne TTL | Historische Daueranzeige; Datenfrische und Verlust der Health-Quelle trotzdem prüfen. |
| Rot | Ohne TTL, bis quittiert beziehungsweise Fehler aufgehoben | Alarmursachen und Quittierung getrennt definieren; fehlende TTL muss wirklich unbefristet sein. |

Die 5–10-s-Angabe für Orange ist erhaltene historische Empfehlung, keine bereits validierte Hangtime des Funkstacks und keine vereinbarte feste Nachlaufentscheidung.

## 4. Architektur, Komponenten und Abhängigkeiten

### 4.1 Geplante Komponenten

| Komponente | Aufgabe | Nachweisstand dieser Planung |
|---|---|---|
| Basisstation / Statusadapter | Health-, TX-, Wartungs- und Bootereignisse auf Lampenaufträge abbilden | Integration geplant; Implementierung nicht belegt. |
| Separater Raspberry Pi | Lampendienst und GPIO/Relais-Ausgabe | Ausdrücklich gewählte Architektur, Hardwarestand unbekannt. |
| FastAPI + Uvicorn | REST-API unter /api/lamp | Historischer Vorschlag; kein fertiger Dienst geliefert. |
| State-Engine | Zustände, Priorität, TTL, Muster, Override und Rückfall | Spezifiziert; damaliger Kern unvollständig und fehlerhaft. |
| LampDriver | Einziger zuständiger physischer GPIO-/Relaisbesitzer | Vorgeschlagene Abstraktion; kein konkreter Hardwaretreiber. |
| Optionaler MQTT-Client | Befehle empfangen, Zustandsänderungen veröffentlichen | Vorzusehen, nicht aktiv eingerichtet. |
| JWT-Prüfung / TLS / Rollen | Steuerzugriff begrenzen | Sicherheitsentwurf; kein Implementierungsnachweis. |
| systemd | Start, Neustart und definierter Shutdown | Beispielunit vorgeschlagen, nicht installiert. |
| Frühe Bootanzeige | Weiß setzen, später an Lampendienst übergeben | Gewünschte Erweiterung mit ungeklärter GPIO-Übergabe. |
| Mock-Backend | Softwareprüfung ohne Relais/Leuchte | Im Briefing gefordert, nicht als fertiges Paket geliefert. |

Der geplante Datenweg lautet: **NetCore-Statusquelle → REST oder optional MQTT → State-Engine auf Relais-Pi → ein GPIO-Treiber → Relaisausgänge → WERMA-Farbkanäle**.

REST und MQTT sollten dieselbe Zustandsmaschine bedienen. Separate konkurrierende GPIO-Schreiber für REST, MQTT und Blinkthreads würden das Prioritätsmodell unterlaufen. Ein solcher gemeinsamer Schreiber ist ein geprüfter Implementierungshinweis, kein in der Planung nachgewiesener Code.

### 4.2 Historisch gewünschte Treiberabstraktion

Vorgeschlagene Datei: **relay_driver.py**, Klasse **LampDriver**.

~~~python
# Historischer Interface-Entwurf; keine vorhandene Umsetzung.
class LampDriver:
    def set_state(self, state: dict[str, int]) -> None:
        """Alle fünf logischen Farbausgänge setzen."""

    def pattern(self, colors, on_ms, off_ms, ttl_ms=None):
        """Historisch gewünschter Mustereinstieg."""
~~~

Als Backends wurden **RPi.GPIO oder gpiozero** sowie ein Mock genannt. Für das tatsächlich gewählte Pi-/OS-Modell ist die Backend-Kompatibilität zu prüfen; diese Alternativen sind keine bestätigte Installationsauswahl.

**Geprüfter Prüfhinweis:** Die Musterhoheit sollte bei der State-Engine beziehungsweise einem einzelnen Ausgabetaktgeber liegen. Ein unabhängig weiterlaufendes driver.pattern darf weder Rot übergehen noch nach /off erneut Ausgänge einschalten. Driver-Aufrufe benötigen außerdem definierte Fehlerbehandlung und einen kontrollierten Stopp.

### 4.3 Statusquellen und automatische Trigger

| Historischer Trigger | Vorgesehene Anzeige | Vor Umsetzung zu präzisieren |
|---|---|---|
| on_boot | Weiß, ursprünglich TTL 5000 ms | Boot des I/O-Pi oder der entfernten TBS? Wann endet Initialisierung tatsächlich? |
| vpn_up und health_ok | Grün | Ist VPN immer erforderlich? Relevante Health-Domänen und Datenfrische festlegen. |
| tx_start | Orange, blinkend, Beispiel-TTL 10000 ms | Call-/Sprechrecht-/Sprachframe-/Trägerbezug festlegen; Renewal und tx_end definieren. |
| ota_start / sync_start | Blau, langsam blinkend | Wartungszustand, relevante Jobs und tatsächliches Ende eindeutig identifizieren. |
| ota_end / sync_end | Blau entfernen | Andere aktive Ursachen und Rückfall erhalten. |
| critical_alarm | Rot; andere Farben aus | Temperatur-/Watchdog-/SDR-Fehlerquellen, Alarmlebenszyklus und Quittierung festlegen. |
| alarm_cleared | Rot entfernen; Rückfall auf weiterhin gültigen Zustand | Alle relevanten Alarmquellen berücksichtigen, nicht bloß die letzte Meldung. |

Die frühen Beispiele nannten **Temperatur > 70 °C** als Automation. Dieser Wert wurde nicht als geprüfter Grenzwert für Pi, SDR, Rack oder Leuchte übernommen. Bestehende Repository-Beispielschwellen sind ebenfalls keine Freigabe für die konkrete Hardware.

## 5. Hardware und technische Parameter

### 5.1 Gesichert versus angenommen

**Festgehaltene Ausgangslage:** fünf WERMA-Farben; separater Pi mit Relaiskarten.

**Nur aus dem Implementierungsbriefing:** **24 V DC** aus separatem Netzteil. Typ und Betriebsspannung sind nicht bestätigt. 24 V bleibt eine **Planungsannahme** bis zur Prüfung des Typenschilds und Anschlussplans.

Die vorgeschlagene Signalkette war Pi-GPIO → Relaiskarte → Versorgungseingang je Farbe. Sie ist kein fertiger Verdrahtungsplan. Gemeinsam geschalteter Plus/Minus, Kontaktart, Steuereingangspegel und Versorgung der Relaiskarte fehlen.

### 5.2 Historisches GPIO-Beispiel

Das Briefing bezeichnete die Nummern ausdrücklich als **BCM** und konfigurierbar:

| Farbkanal | Beispiel-BCM-GPIO | Einordnung |
|---|---:|---|
| Rot | 17 | Ungeprüfter Default |
| Orange | 27 | Ungeprüfter Default |
| Grün | 22 | Ungeprüfter Default |
| Blau | 5 | Ungeprüfter Default |
| Weiß | 6 | Ungeprüfter Default |

Diese Werte sind keine geprüfte Verdrahtung. BCM-Nummern, physische Headerpositionen und Linux-GPIO-Chip-/Line-Offsets dürfen nicht gleichgesetzt werden. Die Beispiele mit gpiochip0 sind ebenfalls keine bestätigte Gerätezuordnung.

**Active-low:** In der Bootantwort wurde das Umkehren der Pegel bei active-low erwähnt. Die ursprüngliche YAML enthielt dafür keinen eigenen Parameter. Polarität muss vor Umsetzung ausdrücklich und einheitlich für Start, Laufzeit, Shutdown und Mock festgelegt werden.

### 5.3 Blinken über Relais

Relaiskarten sind als Ausgangstreiber festgelegt. Diese Vorgabe bleibt erhalten; ein Halbleiterausgang wurde in dieser Entwicklungsphase nicht als endgültiger Ersatz beschlossen.

Für mechanische Relais ist die Schalthäufigkeit technisch relevant:

- 300/300 ms entsprechen einem vollständigen An-/Aus-Zyklus alle 0,6 s: rechnerisch **6000 Zyklen pro Stunde** bei Dauerblinken.
- 800/800 ms entsprechen 1,6 s je Zyklus: rechnerisch **2250 Zyklen pro Stunde** bei Dauerblinken.
- Ein Zyklus enthält zwei Zustandswechsel. Last, Kontaktlebensdauer, Geräusch und vorhandene interne Blinkfunktionen sind am tatsächlichen Relais-/Leuchtentyp zu prüfen.

Dies sind Berechnungen aus den festgelegten Zeiten, keine Lebensdauer- oder Belastbarkeitsmessungen.

Ein Softwarebefehl allein bestätigt nicht, dass der Kontakt oder die Leuchte tatsächlich geschaltet hat. „Kommandiert“ und „physisch rückgemeldet“ sind im künftigen Status zu unterscheiden, sofern keine Rückmeldung vorhanden ist.

## 6. REST-API: historischer Vertrag

**Basis:** /api/lamp. Die früher genannten /lamp-Routen werden durch den späteren, zusammenhängenden /api/lamp-Vertrag als Übergabeziel ersetzt. Ein Beispielpfad mit /api/lamp ist keine bestehende Repository-Route.

| Methode/Route | Vorgeschlagene Funktion | Offene Details |
|---|---|---|
| GET /api/lamp | Ausgabezustand und aktive Locks liefern | Gewünschter vs. tatsächlicher Ausgang, dominante Ursache, Restlaufzeit, Override und Datenfrische. |
| POST /api/lamp | Direkte Farb-/Zustandsanforderung | Patch oder vollständiger Zustand? false als Abschalten oder Freigeben? Mehrere Ursachen/Sender? |
| POST /api/lamp/pattern | Blinkmuster starten | Validierung, Zeitgrenzen, Stopp und Wechselwirkung mit Prioritäten. |
| POST /api/lamp/off | Alles aus, Locks löschen | Berechtigung und Umgang mit noch aktivem kritischem Alarm. |
| POST /api/lamp/override | Manueller Override, vorgeschlagen Priorität 100 | Alarmübersteuerung, maximale Dauer, Berechtigung. |
| DELETE /api/lamp/override | Nur Override beenden; Rückfall auf Auto/Locks | Früherer Beispielcode löschte stattdessen alle Locks; zu korrigieren. |

### 6.1 Historische Payloads

~~~json
{
  "red": false,
  "orange": true,
  "ttl_ms": 10000,
  "priority": 70,
  "source": "api"
}
~~~

~~~json
{
  "pattern": "blink",
  "colors": ["blue"],
  "on_ms": 800,
  "off_ms": 800,
  "ttl_ms": 15000,
  "priority": 60
}
~~~

Bei POST /api/lamp wurden die fünf Farbwerte als optionale bool/None-Felder beschrieben. TTL und priority waren optional, source ein freier String mit Default api. Der erste Python-Entwurf setzte request.priority pauschal auf 50, obwohl für einzelne Farben höhere Defaults gewünscht sind. Die spätere Config sah farbabhängige Prioritäten vor, ohne deren Anwendung zu implementieren.

### 6.2 Authentifizierung und Zugriff

Historisch vorgesehen:

- JWT als Bearer-Token, optional abschaltbar über Config.
- Issuer **lamp.local**, Audience **lamp-api**.
- Frühe Empfehlung: TLS sowie Viewer/Operator/Admin, zusätzlich beispielhaft sys.core.master.
- Leichtes Rate-Limit: **10 Requests pro Sekunde**.
- Debounce für häufige Pattern-Updates.
- Manuelle API-Aufträge sollen als Eingriffsquelle von automatischen Zustandsursachen unterscheidbar sein.

Nicht spezifiziert wurden Signaturalgorithmus, Schlüsselbereitstellung/Rotation, zentrale Ausstellerintegration, Claim-/Rollenzuordnung, maximale TTL, zulässige Prioritäten und Rate-Limit-Geltungsbereich. sys.core.master ist durch seine Erwähnung keine nachgewiesene Rolle der geprüften Software.

**Geprüfter Prüfhinweis:** source und priority aus einer Payload sind keine vertrauenswürdige Identität/Berechtigung. Erlaubte Prioritäten und Eingriffe müssen serverseitig an den authentifizierten Aufrufer gebunden sein. Frei vergebbare Priorität 100 kann das übernommene Alarmmodell aushebeln.

Die historischen cURL-Beispiele ohne Authorization-Header widersprechen jwt_required: true. Es waren vereinfachte Anwendungsbeispiele, keine erfolgreich ausgeführten Requests.

### 6.3 Beispielaufrufe zur späteren Abnahme

**Nur geplante API-Nutzung; hier nicht gegen einen realen Lampendienst ausgeführt.** Der Platzhalter wird lokal durch ein gültiges Token ersetzt und ist keine Zugangsdatenquelle.

~~~bash
# Orange: 10 Sekunden 300/300-ms-Blinken.
curl -X POST 'http://<lampen-pi>:8080/api/lamp/pattern' \
  -H 'Authorization: Bearer <gueltiges-JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"pattern":"blink","colors":["orange"],"on_ms":300,"off_ms":300,"ttl_ms":10000,"priority":70}'

# Rot: unbefristeter Alarm, falls fehlende TTL so implementiert wird.
curl -X POST 'http://<lampen-pi>:8080/api/lamp' \
  -H 'Authorization: Bearer <gueltiges-JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"red":true,"priority":90}'

# Weiß: historischer 5-s-Hinweis, kein Beweis eines vollständigen Bootabschlusses.
curl -X POST 'http://<lampen-pi>:8080/api/lamp' \
  -H 'Authorization: Bearer <gueltiges-JWT>' \
  -H 'Content-Type: application/json' \
  -d '{"white":true,"ttl_ms":5000,"priority":40}'
~~~

Port **8080/TCP** stammt aus der vorgeschlagenen Uvicorn-Unit. Die früher genannte HTTPS-Adresse hatte keine konfigurierte TLS-Terminierung dahinter. HTTP und HTTPS sind daher Entwurfsalternativen, kein bestätigtes Betriebsprofil.

## 7. MQTT und bestehender NetCore-Vertrag

### 7.1 Historischer Lampenentwurf

| Topic | Zweck |
|---|---|
| nct/<node-id>/lamp/cmd | Direkte Steuerung |
| nct/<node-id>/lamp/pattern | Mustersteuerung |
| nct/<node-id>/lamp/state | Zustandsänderungen veröffentlichen |

~~~json
{
  "set": {"orange": true},
  "ttl_ms": 10000,
  "priority": 70
}
~~~

Die erste Skizze verwendete teilweise 3000 ms statt 10000 ms; dies waren Beispiele, kein geänderter fester TX-Wert. Historisch sollten Statusänderungen publiziert werden. MQTT war **optional und in der Config deaktiviert**.

Nicht festgelegt: node-id des I/O-Pi versus beobachtete TBS, QoS, Retain, Last Will, Client-Authentifizierung, Command-ID, Deduplizierung, ACK, Reconnect, Uhr-/TTL-Behandlung und Verhalten gegenüber verspäteten Nachrichten.

### 7.2 Geprüfter Repository-Abgleich

Der vorhandene IoT-Vertrag nutzt inzwischen **netcore/v1**, unter anderem:

~~~text
netcore/v1/events/<domain>/<action>
netcore/v1/state/<subject-type>/<subject-id>
netcore/v1/commands/#
netcore/v1/acks/<command-id>
~~~

Nach der gelesenen Vertragsdokumentation sind Discovery-/Zustandsnachrichten retained, Aktionsbefehle nicht retained; Befehle laufen durch ein persistentes Command-Ledger. [R07]

Die Beispielkonfiguration enthält QoS 1, Command-Policy mit default_deny, Verbot retained Commands, Default-TTL 30 s, Maximal-TTL 300 s und Lifecycle-ACKs. Das sind **vorhandene IoT-Gateway-Beispielwerte**, keine hier nachträglich beschlossenen Lampenparameter. [R08]

**Abweichung:** Historische nct-Topics und zusätzliche netcore/v1-Topics sind nicht identisch. Vor Lampenimplementierung muss ein Adapter oder ein einheitlicher neuer Vertrag beschlossen werden. Die Dokumentation ändert keinen MQTT-Vertrag.

Der vorhandene Befehl **virtual.relay.set** aktualisiert einen virtuellen Gerätezustand. Der gelesene Handler greift nicht auf GPIO zu und liefert keine physische Lampenbestätigung. Ein ACK für dieses virtuelle Kommando beweist kein geschaltetes WERMA-Relais. [R06]

## 8. State-Engine: Anforderungen, Fehler und notwendige Präzisierungen

### 8.1 Historischer Lock-Stack

Vorgeschlagen war sinngemäß:

~~~text
[{exp_ms, priority, source, colors{...}}, ...]
~~~

Im Beispielcode hießen die internen Schlüssel exp, prio, src und colors. Die Beschreibung sah vor:

1. Abgelaufene Locks entfernen.
2. Locks absteigend nach Priorität ordnen.
3. Je Farbe den ersten gesetzten Wert wählen.
4. Ergebnis an den Treiber ausgeben.

**Konflikt:** Punkt 3 ermöglicht unterschiedliche Gewinner je Farbe. Damit können Rot und Grün gleichzeitig an sein oder Blau und Grün parallel erscheinen. Das widerspricht den Anforderungen „Rot schaltet andere aus“ und „Blau 60 verdrängt Grün 50“. Für die Umsetzung muss die **globale Anzeigenpriorität** gegenüber dem früheren per-Farbe-Mischen maßgeblich sein.

Bei einer blinkenden dominanten Anzeige soll die Pause nicht automatisch die niedrigere Farbe sichtbar machen, sofern ein solches Wechselmuster nicht ausdrücklich gewählt wird. Die Priorität gehört zum aktiven Zustand, nicht nur zum momentanen ON-Pegel.

### 8.2 Fehler des früheren Python-Prinzipbeispiels

| Befund | Konkrete Ursache im sichtbaren Code | Auswirkung |
|---|---|---|
| Fehlende TTL ist nicht unbefristet | exp = now + (ttl_ms or 0), während reconcile nur exp == 0 als unbefristet behandelt | Ohne TTL erzeugte Locks sind sofort beziehungsweise beim nächsten Vergleich abgelaufen. |
| Abgelaufene Ausgänge bleiben an | new = state.copy(); bei fehlendem Gewinner bleibt der alte Wert | Weiß kann nach TTL weiterhin leuchten, selbst wenn reconcile erneut aufgerufen wird. |
| Abgelaufene Locks bleiben im Speicher | alive ist nur eine lokale Liste; locks wird nicht bereinigt | Wachstum und irreführende Lock-Anzahl. |
| Kein zeitgesteuerter Ablauf | reconcile wird nur bei Requests aufgerufen; kein Timer/Worker | TTL allein erzeugt keinen späteren Schaltvorgang. |
| Priorität nicht global | Erste Werte je Farbe statt dominanter Gesamtanzeige | Rot unterdrückt Grün/Orange/Blau nicht zuverlässig. |
| Override-Ende zerstört Basiszustände | override_off ruft lamp_off auf | Alle Locks werden gelöscht; kein Rückfall auf Auto. |
| Muster nur vorgetäuscht | /pattern liefert lediglich JSON; keine Taktung | Erfolgreiche Antwort ohne blinkende Lampe. |
| GPIO fehlt | apply_outputs enthält pass | Kein physischer Schaltweg. |
| JWT/Rate-Limit fehlen | Keine implementierte Dependency/Middleware/Prüfung | Die Sicherheitsanforderung existiert nur im Text. |
| Pattern-Payload nicht sauber modelliert | pattern: str, colors: list[str], Timing als Funktionsparameter statt gemeinsames Requestmodell | Der gezeigte JSON-Objektvertrag wird dadurch nicht vollständig als Body definiert; kein HTTP-Verhalten getestet. |
| Keine kontrollierte Nebenläufigkeit | Globale Listen und dicts ohne zentralen seriellen Eigentümer | Race-/Ordnungsprobleme bei parallelen Requests und späteren Patternthreads möglich. |
| Ungeklärtes Ausschalten/Freigeben | Neue false-Werte werden nur als weitere Locks angehängt | Alte Ursachen bleiben bestehen oder werden nur verdeckt; Quittierungssemantik fehlt. |
| Konfiguration nicht angebunden | Kein YAML-Laden im Kern | Pin-, Priority-, Security- und MQTT-Defaults werden nicht wirksam. |

Diese Fehler sind **nicht durch diese Archivarbeit im Anwendungsrepository behoben** worden. Sie sollen verhindern, dass das Prinzipbeispiel später als fertige Implementation kopiert wird.

### 8.3 Zusätzliche Implementierungsleitplanken, noch nicht beschlossen oder umgesetzt

- Ein serialisierter Scheduler/State-Owner führt Zustandsänderungen, Expiry und Blinkphasen zusammen.
- Ausgabe bei jeder Berechnung aus definierten Defaults neu bilden; abgelaufene Ursachen tatsächlich entfernen.
- Fehlende TTL eindeutig als unbefristet behandeln; TTL 0, negative TTL und Maximalwerte ausdrücklich definieren.
- Für lokale Laufzeiten monotone Zeit verwenden; externe Zeitstempel und MQTT-Frische getrennt prüfen.
- Ursachen nach authentifiziertem Sender/Zweck identifizieren; Wiederholungen erneuern/upserten statt unbegrenzt anhängen.
- Mehrere aktive rote Ursachen getrennt verwalten. Eine Quittierung darf nicht beliebig den verbleibenden Fehler entfernen.
- Override nur als eigene Ebene entfernen; gültige Autozustände erhalten.
- Gleichstandregeln, maximal zulässige Payloadgröße, Farbenkombinationen und Tick-/Jittergrenzen definieren.
- Keine beliebige Dauerblockierung eines kritischen Zustands durch einen frei vergebenen Client-Prioritätswert.
- Netzwerkverlust, Prozessabsturz, Relaisfehler und Shutdown benötigen definierte Ausgangszustände.
- Status soll Grundzustand, dominante Ursache, TTL, Override, gewünschte Ausgabe und verfügbare Hardware-Rückmeldung auseinanderhalten.

Dies sind technische Prüfhinweise des Prüflaufs, keine behauptete nachträgliche Freigabe eines geänderten API-Vertrags.

## 9. Dateien, Config und Systemdienst des Briefings

### 9.1 Historisch vorgesehene Dateien

| Pfad/Name | Zweck | Nachweisstand |
|---|---|---|
| /opt/lamp_api/main.py | FastAPI-Anwendung | Vorgeschlagen, keine installierte Datei belegt. |
| /opt/lamp_api/relay_driver.py | GPIO-/Relaisabstraktion | Vorgeschlagen, kein fertiger Treiber geliefert. |
| /opt/lamp_api/config.yaml | Laufzeitconfig | Beispiel geliefert, kein Ladepfad implementiert. |
| /etc/systemd/system/lamp_api.service | API-Dienst | Unit vorgeschlagen, nicht eingerichtet nachgewiesen. |
| colors.yaml | Früher angebotene separate Farbdefaults | Angebot, durch das spätere config.yaml-Briefing als Strukturidee abgelöst; keine Datei geliefert. |
| werma_boot.dtbo | In der ersten Bootantwort angebotener Overlayname | Nur Angebot; Binärdatei nicht vorhanden. |
| werma-boot.dts | Später genannter Overlay-Quellname | Nur Angebot; Quelltext nicht geliefert. |
| /etc/rc.local | Späte Bootalternative | Vorgeschlagen; keine Änderung nachgewiesen. |

Das angebotene Skeleton-ZIP wurde im verfügbaren Verlauf nicht geliefert. Unter /opt/lamp_api beschriebene Zielpfade sind keine tatsächlich im Git vorhandenen Pfade.

### 9.2 Historische YAML, unverändert als Entwurf erhalten

~~~yaml
pins: { red: 17, orange: 27, green: 22, blue: 5, white: 6 }
priority: { red: 90, orange: 70, blue: 60, green: 50, white: 40 }
patterns:
  blink_fast: { on_ms: 300, off_ms: 300 }
  blink_slow: { on_ms: 800, off_ms: 800 }
security:
  jwt_required: true
  jwt_issuer: "lamp.local"
  jwt_audience: "lamp-api"
mqtt:
  enabled: false
  broker: "localhost"
  base_topic: "nct"
~~~

Nicht enthalten waren unter anderem GPIO-Backend, active-low, GPIO-Chip, node-id, Bind/Port, MQTT-Port und Zugriffsdatenbereitstellung, Grundzustands-/Heartbeatregel und konkrete TLS-Konfiguration. Das Beispiel enthält keine realen Zugangsdaten.

### 9.3 Historische systemd-Unit der API

**Vorgeschlagen, nicht installiert oder gestartet.**

~~~ini
[Unit]
Description=WERMA Lamp API
After=network.target

[Service]
WorkingDirectory=/opt/lamp_api
ExecStart=/usr/bin/python3 -m uvicorn main:app --host 0.0.0.0 --port 8080
Restart=always
User=pi

[Install]
WantedBy=multi-user.target
~~~

Fehlende Umsetzungsteile: Paket-/Interpreterumgebung, Konfigurationsladen, tatsächlicher Benutzer, GPIO-Rechte, Shutdown-Cleanup, Readiness-/Health-Semantik und festgelegtes Fehlerverhalten. User=pi ist ein historischer Beispielwert; ein existierender Benutzer wurde nicht geprüft.

network.target garantiert keine betriebsbereite Verbindung zur entfernten TBS und kein aktives VPN. Auch ein gestarteter Uvicorn-Prozess beweist noch keine valide Statusquelle oder funktionierende Relais.

Ein venv, ein eigener Dienstbenutzer, explizite GPIO-Berechtigungen und eine definierte Restart-/Shutdown-Strategie sind noch auszuarbeiten; sie wurden in der Planung nicht fertig bereitgestellt.

## 10. Bootanzeige: ursprüngliche Ansätze und zusätzliche Prüfung

### 10.1 Wunsch und Geltungsbereich

Die gewünschte Bootanzeige soll früh eingeschaltet und nach dem Start abgeschaltet beziehungsweise an den Lampendienst übergeben werden. Drei Ebenen wurden vorgeschlagen; systemd-early war der bevorzugte wartbare Ansatz.

**Offene Kernfrage bei zwei Rechnern:** Soll Weiß den Boot des **Relais-Pi** oder den der **Basisstation** darstellen?

Ein lokaler GPIO auf dem Relais-Pi kann dessen eigenen Start anzeigen. Ein Kernel-Hook der entfernten TBS kann deren fremde GPIOs nicht direkt ohne funktionsfähigen Transport schalten. Ein früher API-Aufruf benötigt bereits laufende Netzwerk- und API-Komponenten. Diese Trennung wurde im ursprünglichen Entwurf nicht geklärt.

Für eine TBS-Bootanzeige wären daher ein definierter Remote-Lebenszyklus/Heartbeat, eine unabhängige elektrische Verbindung oder eine andere separat zu planende Signalisierung erforderlich. Welche Variante gewünscht ist, bleibt offen.

### 10.2 Ansatz A: Device-Tree-GPIO-Hog

Historischer Vorschlag: einen GPIO als gpio-hog mit output-high initialisieren; danach soll Userspace ausschalten. Ein DTBO und ein Eintrag in /boot/config.txt wurden angeboten, aber nicht geliefert.

**Technische Korrektur:** Ein GPIO-Hog setzt nicht nur einen Startwert. Er **fordert die GPIO-Line während des Controller-Probes an und hält sie im Kernel**. Ein normaler GPIO-Character-Device-Treiber im Userspace kann denselben bereits reservierten Ausgang nicht automatisch übernehmen. Die Aussage „Hog setzen, danach Userspace übernimmt einfach“ ist deshalb unvollständig. Linux-Binding und gpiod_hog-Implementation belegen die Reservierung. [E01][E02]

Ein solcher Ansatz benötigt eine ausdrücklich getestete Freigabe-/Übergabestrategie oder eine andere GPIO-Verwaltung. Eine fertige Übergabelösung wurde in der Planung nicht ausgearbeitet.

Auch output-high ist nur unter bekannter Polarität richtig. Bei active-low, vorgeschalteten Invertierungen oder abweichendem elektrischen Design darf daraus kein pauschales „Weiß an“ abgeleitet werden.

### 10.3 Ansatz B: systemd-early mit gpioset

Historisch vorgeschlagene Installation:

~~~bash
# Nur vorgeschlagen; auf Jans Pi nicht nachweislich ausgeführt.
sudo apt-get update
sudo apt-get install -y gpiod
~~~

Historische Unit **01-lamp-boot-on.service**:

~~~ini
# Historischer Entwurf mit unten beschriebenen Problemen.
[Unit]
Description=WERMA Boot Lamp ON (early)
DefaultDependencies=no
Before=sysinit.target
After=local-fs-pre.target

[Service]
Type=oneshot
ExecStart=/usr/bin/gpioset --mode=signal gpiochip0 17=0 27=0 22=0 5=0 6=1

[Install]
WantedBy=sysinit.target
~~~

Historische Unit **99-lamp-boot-done.service**:

~~~ini
# Historischer Entwurf, keine freigegebene funktionsfähige Übergabe.
[Unit]
Description=WERMA Boot Lamp OFF when system ready
After=network-online.target multi-user.target
Wants=network-online.target

[Service]
Type=oneshot
ExecStart=/usr/bin/gpioset --mode=signal gpiochip0 6=0 22=1

[Install]
WantedBy=multi-user.target
~~~

Historische Aktivierungsvorschläge:

~~~bash
# Nicht nachweislich auf dem Zielsystem ausgeführt.
sudo systemctl daemon-reload
sudo systemctl enable 01-lamp-boot-on.service
sudo systemctl enable 99-lamp-boot-done.service
~~~

Wesentliche Probleme:

1. **Laufender gpioset-Prozess und Type=oneshot:** Der historische --mode=signal-Modus hält den Prozess am Leben. Eine oneshot-Unit gilt erst nach Prozessende als gestartet. Zusammen mit Before=sysinit.target kann der frühe Dienst die nachfolgenden Startschritte blockieren, solange gpioset weiterläuft. RemainAfterExit repariert diesen laufenden Startprozess nicht. [E03][E04]
2. **Exklusive Line-Besitzer:** Der frühe gpioset-Aufruf fordert alle fünf Lines an, auch ausgeschaltete. Ein zweiter gpioset oder der API-Treiber kann Weiß/Grün nicht gleichzeitig anfordern, solange der erste Prozess Eigentümer ist. Eine ausdrücklich koordinierte Übergabe fehlt.
3. **Versionsabhängige CLI:** --mode=signal und die Positionssyntax entsprechen dem älteren Toolmodell. Die am Prüfdatum gelesene libgpiod-Dokumentation beschreibt gpioset mit --chip/-c und anderen Optionen. Installierte Version und --help müssen vor Anwendung geprüft werden; v1-/v2-Kommandos nicht mischen. [E03]
4. **Ausgang nach Prozessende:** Laut libgpiod ist das Halten des Pegels nach Ende des anfordernden Prozesses nicht garantiert. Ein oneshot-Kurzaufruf ist damit keine pauschale dauerhafte Bootanzeige. [E03]
5. **Geräte-/Dateisystembereitschaft:** DefaultDependencies=no und frühe Startreihenfolge sichern nicht allein, dass GPIO-Character-Device, Binary und zugehörige Rechte bereits verfügbar sind.
6. **Keine echte Anwendungsbereitschaft:** multi-user.target und network-online.target beweisen nicht, dass SDR, TBS, Statusdaten und Lampentreiber gesund sind. Grün darf nicht allein aufgrund erreichter Targets gesetzt werden.
7. **After=lamp_api.service reicht nicht als Übergabe:** Die nachträglich empfohlene zusätzliche Reihenfolge führt keinen GPIO-Owner-Wechsel durch und prüft keine Lampen-/TBS-Readiness.
8. **Unitnummern sind keine Startlogik:** Die Namen 01-/99- bewirken ohne korrekte Abhängigkeiten keine belastbare frühe/späte Startreihenfolge.

**Präzisierung zur Target-Reihenfolge:** Aus After=multi-user.target plus WantedBy=multi-user.target wird hier **kein zwingend reproduzierter Ordering-Cycle** behauptet. Der offline geprüfte systemd-255-Abhängigkeitsentwurf mit Hardware-Stubs wurde ohne Zyklusdiagnose akzeptiert. Der gelesene systemd-v255-Code vermeidet widersprechende automatische Target-Reihenfolgen. Das ändert nichts an den realen Problemen mit dem dauerhaft laufenden oneshot-gpioset und der fehlenden GPIO-Übergabe. [E05]

### 10.4 Ansatz C: rc.local

Historischer Vorschlag:

~~~bash
# Historische Alternative; nicht ausführen, ohne die Prozesssemantik zu reparieren.
/usr/bin/gpioset --mode=signal gpiochip0 6=1
sleep 5
/usr/bin/gpioset --mode=signal gpiochip0 6=0 22=1
exit 0
~~~

Mit einem bis zum Signal laufenden ersten gpioset erreicht das Script den sleep- und Ausschaltteil nicht normal. Außerdem kommt rc.local später als ein expliziter früher Startpfad. Dieser Entwurf wurde nicht als funktionsfähiger Fallback bestätigt.

### 10.5 Weiterer geprüfter Prüfhinweis, kein historischer Beschluss

Die offizielle Raspberry-Pi-Dokumentation beschreibt **gpio=...=op,dh** beziehungsweise op,dl in config.txt als Boot-Vorkonfiguration. Diese kann später durch Device-Tree-Pinctrl oder GPIO-Nutzung überschrieben werden und wirkt erst nach einer Startverzögerung. Sie ist eine zu prüfende Alternative zu einem dauerhaft reservierenden Hog, kein hier getesteter Ersatz und keine Garantie für Anzeige ab Einschalten der Versorgung. [E06]

Zu prüfen ist auch der tatsächliche config.txt-Pfad: moderne Raspberry-Pi-OS-Installationen können eine andere Bootmount-Struktur als den historischen /boot/config.txt-Pfad haben. Ein Zielsystem wurde nicht erreicht.

**Planungsrichtung für eine Fortsetzung:** Ein einzelner zuständiger GPIO-Besitzer soll die Ausgabe durchgehend halten; Bootzustand und Betriebszustand werden über definierte Ereignisse umgestellt. Eine Firmware-Vorkonfiguration oder ein sehr früher Hilfsprozess benötigt eine nachgewiesene Übergabe an diesen Besitzer. Der konkrete Mechanismus bleibt auszuwählen und zu testen.

## 11. Historischer Entwicklungs-/Betriebsstand und geprüfter Repository-Stand

### 11.1 Stand am Ende der Planungsphase

| Gegenstand | Historischer Status |
|---|---|
| Lampenbesitz/Farben | Als Ausgangslage dokumentiert. |
| Farbbedeutungen/Prioritäten | **Beschlossen/geplant**. |
| Separater Pi mit Relais/API | **Beschlossen/geplant**. |
| FastAPI-/MQTT-/TTL-Design | **Entwurf/geplant**, keine fertige Installation. |
| Python-Kern | unvollständiges Prinzipbeispiel mit Platzhaltern und Fehlern. |
| config.yaml und Dienstunit | Im Entwurf beschrieben, nicht als installierte Dateien nachgewiesen. |
| Implementierung | Gewünschter späterer Arbeitsauftrag; nur Briefing geliefert. |
| Frühe Bootanzeige | **Idee/gewünschte Erweiterung**, konkrete Auswahl offen. |
| Physische Ansteuerung | Keine bestätigte Implementation. |
| Funktionstests/API-/GPIO-/MQTT-Tests | Keine in der Planungsphase dokumentierten Ergebnisse. |
| Produktiver Betrieb | **Nicht bestätigt**. |

### 11.2 Methode und Grenzen der geprüften Repository-Prüfung

Die GitHub-Referenz **refs/heads/Archiving** und der zugehörige Commit/Tree wurden gelesen. Zusätzlich wurde **main** nur lesend geprüft. Beide rekursiven Trees waren vollständig geliefert, jeweils truncated=false.

- Archiving: 2884 Tree-Einträge am Eingangscommit.
- main: 2819 Tree-Einträge am geprüften Default-Branch-Commit.
- Ein Blobvergleich zeigt außerhalb Docs/archive/ nur **Docs/Control_Room/Readme.md** als Abweichung.
- Die hier betrachteten Code-/Konfigurationsdateien haben in beiden geprüften Ständen identische Blob-SHAs.

Es existiert keine AGENTS.md in den gelieferten vollständigen Trees. Kein anderer Branch wurde schreibend verwendet.

Geprüft wurden Dateinamen und gezielt Quelltexte der relevanten Komponenten. Es wurde **kein gesamter Workspace kompiliert**, keine vollständige Volltextanalyse jedes Repository-Blobs durchgeführt und keine beliebige Branch-/PR-Historie auf WERMA durchsucht. Negative Aussagen gelten deshalb für die dokumentierten Prüfpunkte.

### 11.3 Tatsächlich vorhandene Integrationsbausteine

| Bereich | Geprüfter Befund | Bedeutung für WERMA |
|---|---|---|
| Basisstations-Fanout | telemetry-fanout verteilt Ereignisse u. a. an Dashboard, Alerts/Snom und Control-Room-/Netztelemetrie. [R01] | Vorhandener Einstieg für Adapter; kein GPIO-Lampentreiber. |
| TelemetryEvent | GroupCallStarted/Ended, GroupCallSpeakerChanged, IndividualCallStarted/Ended, TsVoiceActivity, TxVisual/TxQuality, SdrHealth, SysHealth, BrewConnected, EmergencyAlarm/Cancel. [R02] | Geeignete Quellen, fachliche Abbildung noch zu definieren. |
| Control-Room-HTTP | Node-/Call-/Health-APIs vorhanden; in der gelesenen Routingfunktion keine /api/lamp- oder /api/indicator-Routen. [R03] | Statusabruf möglich; Lampensteuerung nicht bereits vorhanden. |
| Control-Room-Readiness | /health/ready antwortet HTTP 200; JSON-status kann ready oder degraded sein. [R03] | HTTP 200 allein darf kein Grün auslösen. |
| Control-Room-Auth | Node/Viewer/Operator/Admin; vorhandene HTTP-/WS-Authentifizierung u. a. mit Basic/Bearer. [R04] | Bearer ist nicht automatisch JWT; kein belegter JWT-Vertrauensvertrag zum neuen Pi. |
| Hardware-Gateway | Geräte-/Sensor-Ingress, Alarme, MQTT-Zustände, Heartbeat-Watchdog, Metriken. [R05] | Bestehender Telemetrie-/Registry-Baustein. |
| Hardware-Gateway-Ausgänge | outputs wird als Telemetriefeld gespeichert; outputs_enabled im Status gezeigt; POST-Ingress /api/v1/telemetry. [R05] | Kein physischer Ausgangstreiber oder Lampen-Schaltendpunkt nachgewiesen. |
| IoT-Gateway | Command-Policy/Ledger/ACK, virtuelle Relais/Lichter und Integrationen. [R06][R07][R08] | Vorhandene Steuerinfrastruktur prüfen; virtual.relay.set schaltet nur virtuellen Zustand. |

Der direkte Dateinamensabgleich beider Trees lieferte keinen eigenen lamp_api-Dienst, keinen WERMA-Treiberpfad und keine werma-boot.dts/DTBO. In Archiving existiert zusätzlich die **andere WERMA-Rack-Archivdatei**, die selbst kein Implementierungsnachweis ist.

### 11.4 Hardware-Gateway: konkrete zusätzliche Parameter

Aus der gelesenen Beispielconfig und dem Handler: [R05][R09]

| Parameter/Pfad | Beispielwert/Befund |
|---|---|
| HTTP-Bind | 0.0.0.0:8250 |
| MQTT-Broker | 127.0.0.1:1883 |
| MQTT-Prefix | netcore/v1 |
| Default-Laufzeitconfig | /etc/netcore/hardware-gateway.toml |
| Zustandsdatei | /var/lib/netcore-hardware-gateway/state.json |
| Ereignislog | /var/lib/netcore-hardware-gateway/events.ndjson |
| heartbeat_timeout_secs | 30 |
| stale_after_secs | 20 in der Beispielconfig; nicht mit echter Lampen-Frischeprüfung gleichsetzen. |
| outputs_enabled | false in der Beispielconfig; im Handler lediglich Statusausgabe, kein belegter Aktivierungsweg für GPIO. |
| Telemetrie-Ingress | POST /api/v1/telemetry |
| Auslese-APIs | GET /api/v1/status, /api/v1/devices, /api/v1/events |
| MQTT-Telemetrie-Ingress | netcore/v1/hardware/+/telemetry |
| MQTT-Gerätestatus | netcore/v1/state/hardware/<device_id> |
| Security-Modus | open_lab; Codehinweis „keine Anmeldung, kein TLS“ |

Die Hardware-Gateway-Healthhandler geben in den gelesenen Routen status=ok aus; das ist keine Bestätigung funktionsfähiger WERMA-Kontakte. Für den geplanten JWT-Lampendienst ist dieses OPEN-LAB-Verhalten keine bereits erledigte Sicherheitsintegration.

Andere Beispielschwellen dieses Gateways (Temperatur 40/55 °C, Feuchte 75/90 %, Versorgung 11,5/10,8 V) gehören zu seinem allgemeinen Beispielprofil. Sie ersetzen weder den früher genannten 70-°C-Entwurf noch die Hardwareprüfung einer möglicherweise mit 24 V betriebenen Leuchte.

### 11.5 Relevante am Prüfdatum vorliegende Dateien und Blob-Identitäten

| Datei | Blob-SHA am geprüften Eingangsstand |
|---|---|
| bins/bluestation-bs/src/main.rs | 2e37254ce396a99765f499b211454436456693fa |
| crates/tetra-entities/src/net_telemetry/events.rs | e98ce67c8690fad54bf81c36b87b0511527ad18a |
| bins/netcore-control-room/src/http.rs | d05ea89c6c176a156bd55211d42a36ef010b5182 |
| bins/netcore-control-room/src/auth.rs | 80fee6d001297ec36128a6a447ac1ce76a96fdfb |
| system-backend/hardware-gateway/src/netcore_hardware_gateway.py | f24d9c4d52b3960026d6e69cad937b7faa6d3694 |
| system-backend/iot-gateway/src/command.rs | af395c7e804a1cf9e3436dfab40705a0db369037 |
| system-backend/iot-gateway/docs/mqtt-contract.md | 22c0b14911632fb6a7167f54af4eb3f4b7e2225f |

Es wurde keine konkrete Implementierungs-PR für die Leuchte nachgewiesen. Der erfasste main-Commit gehört zu PR #59, einer UI-/Designintegration; daraus wird keine Lampenfunktion abgeleitet.

## 12. Fehler, Diagnose, Lösungen und Tests

### 12.1 Historisch beobachtete Fehler

In der Planungsphase wurden keine echten Installationsfehler, Relaisfehler, API-Fehlermeldungen oder Messergebnisse vorgelegt. Deshalb existiert keine historisch als erfolgreich bestätigte Reparatur. Die oben genannten Fehler stammen aus dem geprüften Code-/Entwurfsreview und aus isolierten Prüfungen des sichtbaren Beispiels.

Die damals suggerierten automatischen TTL-/Prioritäts-/Bootübergänge dürfen nicht als „hat funktioniert“ archiviert werden.

### 12.2 Tatsächlich durchgeführte Prüfungen im Prüflauf

| Prüfung | Tatsächliches Ergebnis | Grenzen |
|---|---|---|
| Branch-/Commit-/Tree-Abruf | Archiving und main erreichbar; Commit-/Tree-IDs erfasst; vollständige Trees. | Momentaufnahme, kein Zugriff auf Zielgeräte. |
| Archivbestand | Vorhandener README-Index gelesen; andere WERMA-Rack-Datei als anderer Projektentwurf identifiziert. | Keine stillschweigende Umdeutung ihrer Farbentscheidungen. |
| Code-/Konfigurationsreview | Vorhandene Integrationsbausteine und fehlende Lampen-Schaltstellen an den aufgeführten Dateien nachvollzogen. | Kein kompletter Build, keine vollständige Analyse aller Blobs/Branches. |
| PDF-Inventar | Alle 25 lokalen PDFs mit pdftotext/pdfinfo geprüft; Seitenzahlen erfasst. | Deckblatt-/Bestandsprüfung, keine vollständige Normauswertung. |
| Bildbestand | Keine eigenständigen Bilddateien in den bereitgestellten Anhängen gefunden. | Ein früheres nicht übergebenes Bild wäre damit nicht ausgeschlossen. |
| Historischer Python-Kern | Vier gezielte Verhaltensprüfungen reproduzierten die unten genannten Fehler. | Nur reconcile/off/Expiry-Kern; deterministische Fake-Uhr; Treiberausgabe durch No-op ersetzt; keine REST-/GPIO-/MQTT-Prüfung. |
| Originale systemd-Units, offline | systemd-analyze verify mit systemd 255 (255.4-1ubuntu8.17): **Exit 1**, weil /usr/bin/gpioset im Prüfcontainer nicht vorhanden ist. | Nicht als Zielgeräte-/GPIO-Fehler interpretieren; keine Unit installiert/gestartet. |
| Isoliertes systemd-Targetmodell | gpioset durch /usr/bin/true ersetzt, Target-Wants als Modell des enable-Schritts ergänzt: **Exit 0**, keine Zyklusdiagnose; Debugprüfung bestätigte beide eingereihten Jobs. | Nur offline Abhängigkeitsprüfung mit Stubs, kein Boot-/Funktionsnachweis. |
| Primärquellenprüfung | Linux-Hog-Reservierung, libgpiod-Pegel-/Prozesssemantik, systemd-oneshot und Target-Sonderverhalten geprüft. | Nicht auf das unbekannte installierte Pi-/OS-System getestet. |

### 12.3 Ergebnisse der vier Python-Kernprüfungen

Die relevanten historischen Funktionen wurden mit denselben Schlüssel-/Auswahlregeln isoliert ausgeführt; externe Ausgänge wurden nicht geschaltet.

| Fall | Beobachtung | Nachgewiesenes Problem |
|---|---|---|
| Grün 50 plus Rot 90, beide unbefristet | red=1 **und** green=1 | Per-Farbe-Auswahl erfüllt globale Alarmunterdrückung nicht. |
| Weiß mit exp=6000, Uhr von 1000 auf 7000 ms, erneutes reconcile | white bleibt 1; ursprüngliche Lock-Liste enthält weiter einen Eintrag | Kein sauberer Rückfall und keine echte Lock-Bereinigung. |
| Fehlende TTL, exp=now+(None or 0) bei 1000 ms | exp=1000; green bleibt 0 | Fehlende TTL wird nicht als unbefristet gespeichert. |
| Grüner Auto-Lock plus blauer Override, danach override_off | Alle Farben 0; Lock-Liste leer | Override-Ende löscht Autozustand. |

Alle vier Fehler wurden reproduziert. Das ist **kein Bestehen der Akzeptanzkriterien**, sondern ein Nachweis, dass der historische Kern korrigiert werden muss. Eine reparierte Anwendung wurde im Rahmen des Dokumentationslaufs nicht geschrieben.

### 12.4 Historische Akzeptanzkriterien, noch offen

| Kriterium | Gewünschtes Ergebnis | Nachweisstand |
|---|---|---|
| Rot/Priorität | Rot blockiert andere Farben bis definierter Alarmaufhebung. | Nicht erfüllt durch den geprüften historischen Kern. |
| TTL Weiß | Weiß nach ca. 5 s aus beziehungsweise Rückfall, für die spätere Bootanforderung zusätzlich Readiness. | Historischer Kern fehlerhaft; Zielgerät nicht getestet. |
| Orange-Muster | 300/300 ms, Stopp nach TTL oder /off. | Nicht implementiert/getestet nachgewiesen. |
| Blau versus Grün | Blau 60 sichtbar, Grün 50 verdrängt. | Historischer per-Farbe-Ansatz widerspricht globaler Unterdrückung. |
| REST-Auth | Ohne gültiges JWT 401, mit gültigem JWT autorisierte Antwort. | Keine JWT-Implementation, kein ausgeführter HTTP-Test. |
| MQTT | Command steuert, State wird veröffentlicht. | Nur optional geplant; keine End-to-End-Prüfung. |
| Override | Nach DELETE Rückfall auf weiter gültige Autozustände. | Fehler im Kern reproduziert. |
| Früher Boot | Weiß früh an, nach tatsächlicher Readiness aus; definierter Handover. | Nicht als reales Bootverfahren getestet. |

Die frühere Kriterienformulierung „Rot blockiert bis override_off/red:false“ vermischt zwei Aktionen. Override-Off darf nicht automatisch einen aktiven Alarm quittieren. Der Vertrag muss vor Implementierung berichtigt werden.

### 12.5 Noch erforderliche Abnahme

Nach Implementierung sind unter anderem echte Prioritäts-/Mehrursachen-, Ablauf-, Timer-/Jitter-, Concurrency-, Auth-, Rate-Limit-, Restore-/Restart-, MQTT-Dedupe-/Retain- und Hardwaretests erforderlich.

Besonders relevant sind: Alarm während TX/OTA/Override; TTL ohne weiteren API-Request; Stoppen eines Musters ohne spätes Wiederanschalten; API-/MQTT-Verbindungsverlust; Lampen-Pi-/TBS-Neustart in unterschiedlicher Reihenfolge; active-low-Startzustände; GPIO-Busy-/Driverfehler; Wiederanlauf mit alten Zustandsdaten.

Das sind nächste Tests, **keine in dieser Entwicklungsphase bereits bestandenen Prüfungen**.

## 13. Ersetzte, verworfene und unbestätigte Ansätze

| Ansatz/Aussage | Historischer Kontext | Archivierte Einordnung |
|---|---|---|
| GPIO-Dienst direkt auf der Node | Erste API-Skizze | Durch separaten Relais-Pi als Zielarchitektur ersetzt. |
| /lamp ohne /api | Frühe Routenbeispiele | Späteres Briefing verwendet /api/lamp. |
| colors.yaml separat | Frühes Angebot | Nicht geliefert; späterer zusammenhängender Config-Entwurf heißt config.yaml. |
| Per-Farbe-Lock-Gewinner | Früher Kern / Engine-Beschreibung | Unvereinbar mit globaler Farbpriorität; vor Umsetzung zu ersetzen/korrigieren. |
| Kein TTL-Wert = unbefristet im Kern | Textanforderung | Code rechnet exp=now und erfüllt dies nicht. |
| Override-Ende = alles aus | Früher Beispielcode | Widerspricht gewünschtem Rückfall; als fehlerhaft archiviert. |
| Fünf Sekunden = Boot fertig | Frühe Boot-Triggeridee | Durch späteren Readinesswunsch unzureichend. |
| GPIO-Hog danach einfach im Userspace schalten | Frühe Kernelantwort | Übergabe fehlt, Kernel reserviert Line. |
| Zwei signal-haltende gpioset-OneShots | Früher systemd-Entwurf | Nicht als funktionsfähiger Bootablauf übernehmen. |
| rc.local mit signal-gpioset und sleep | Späte Alternative | Erster Aufruf blockiert die nachfolgenden Schritte. |
| „Keine Flackerei“ durch --mode=signal | Frühere Aussage | Hält nur während Prozesslaufzeit; kein Handover-/Bootmessnachweis. |
| RPi.GPIO oder gpiozero beliebig | Treiberempfehlung | Backend muss zum konkreten Pi/OS passen, noch nicht ausgewählt/getestet. |
| MQTT/REST-Code oder DTBO als schon vorhanden | Angebot/Briefing | Keine gelieferten ausführbaren Artefakte und kein Betriebsnachweis. |

Keine der Korrekturen ändert die ausdrücklich übernommene Farbreihenfolge oder den Wunsch nach einem eigenen Pi mit Relaiskarten.

## 14. Offene Ideen, Wünsche und Roadmap-Kandidaten

### 14.1 Historisch genannte Nebenideen

Die kleinen Nebenideen bleiben erhalten, ohne sie als Umsetzung zu behaupten:

- Lokale und zentrale API-Steuerung; Bedienung auch per App.
- WebUI mit Farb-/Pattern-Buttons und Livezustand.
- Statusübertragung per WebSocket oder MQTT.
- Einbindung in ITTT-/Automationslogik.
- SDS/API-Brücke, beispielsweise ein Lampenkommando über SDS.
- Hard-Fault-Schnellblinken von Rot.
- Blau bei OTA oder „Shadow-Sync“.
- Farbspezifische Defaults und Enums/Constants.
- Anpassung an Viewer/Operator/Admin beziehungsweise vorhandene zentrale Rechte.
- systemd-Dienstdatei und fertige GPIO-Abstraktion.
- Skeleton-Codebasis als ZIP.
- Kernel-/Device-Tree-Bootanzeige mit angebotenem DTS/DTBO.
- Optional Grün beim Bootabschluss.

Für SDS/API-Brücke, App, WebSocket, ITTT, Shadow-Sync und Hard-Fault-Muster wurden keine am Prüfdatum vorliegenden Implementierungsstellen oder fertigen Verträge dieser Planung nachgewiesen. Sie sind **Ideen**, nicht automatisch Teil der ersten Abnahme.

### 14.2 Prioritäten und Abhängigkeiten der Fortsetzung

Es gab keine nutzerseitig vereinbarte Lieferfrist, Sprintzuordnung oder Roadmap-Priorität außerhalb der Lampen-Prioritätszahlen. Die folgende Arbeitsreihenfolge ist ein **Vorschlag des Archiv-Reviews**, keine nachträglich erfundene Zusage.

| Reihenfolge | Konkreter nächster Schritt | Abhängigkeit/Ergebnis |
|---:|---|---|
| 1 | WERMA- und Relaismodell, Spannung/Polarität und Pi/OS erfassen. | Voraussetzung für sichere konkrete Pins, Backend und Ausgangsinitialisierung. |
| 2 | Semantik festlegen: Orange, Health/Grün, Boot von TBS oder I/O-Pi, Quittierung, Override und false/off. | Verhindert widersprüchliche Anzeigen und Aktionen. |
| 3 | Bestehende NetCore-Komponenten wiederverwenden: Quelle/Adapter, Hardware-Gateway-Telemetrie, IoT-Vertrag und Auth auswählen. | Entscheidung gegen unbeabsichtigte parallele Registry-/Command-Systeme. |
| 4 | State-Engine mit einem Ausgabe-Owner, globaler Priorität, wirklicher Expiry und Override-Rückfall implementieren. | Mock-Tests vor Last-/Relaisbetrieb. |
| 5 | FastAPI-Requestmodelle, validierte Rechte/Prioritäten und Auth konfigurieren. | Vereinbarter JSON-Vertrag, dokumentiertes Sicherheitsprofil. |
| 6 | Konfigurierbaren Relay-/GPIO-Treiber und systemd-Lebenszyklus umsetzen. | Hardware-/Polaritätskenntnis; Verhalten bei Driverfehler/Shutdown. |
| 7 | NetCore-Health/TX/Wartung anbinden und die Datenfrische prüfen. | Nur echte beobachtete Zustände steuern Grün/Orange/Rot. |
| 8 | Bootpfad mit Owner-Übergabe und expliziter Anwendungsreadiness implementieren. | Einheitliche GPIO-Verantwortung; eigenständiger Reboot-/Ausfalltest. |
| 9 | Optional MQTT auf am Prüfdatum vorliegenden NetCore-Vertrag abbilden. | Topic-/Command-ID-/ACK-/Retain-/TTL-Entscheidung und zentrale Policy. |
| 10 | Physische und End-to-End-Abnahme; Bedien-/Betriebsdokumentation. | Erst danach „getestet“ bzw. „im Betrieb bestätigt“ vergeben. |
| Später | WebUI/App, SDS-Brücke, zusätzliche Muster und Automationskomfort. | Auf funktionierender Kernsteuerung aufbauen. |

Die Reihenfolge ist ein Arbeitsvorschlag für die noch ausstehende Implementierung und Abnahme.

## 15. Quellen, Anhänge und verwandte Archive

### 15.1 Arbeitsgrundlagen

- Farb-/Prioritätsmodell, API-Implementierungsbriefing und Bootanzeige-Entwurf.
- Ergänzende Quellenfragmente zur Festlegung auf einen eigenen Pi und zur historischen Datierung.
- Technische Parameter und Quellenbelege; keine Zugangsdaten.

### 15.2 Repository-Quellen, auf den geprüften Commit fixiert

- [R01: Basisstations-Fanout][R01]
- [R02: Telemetrieereignisse][R02]
- [R03: Control-Room-HTTP/Readiness/Routing][R03]
- [R04: Control-Room-Authentifizierung und Rollen][R04]
- [R05: Hardware-Gateway-Code][R05]
- [R06: IoT-Kommandos/virtuelle Relais][R06]
- [R07: MQTT-Vertragsdokumentation][R07]
- [R08: IoT-Beispielkonfiguration][R08]
- [R09: Hardware-Gateway-Beispielkonfiguration][R09]

Zusätzlich gelesen: Hardware-/IoT-Gateway-READMEs, IoT-http.rs/mqtt.rs/model.rs/state.rs, Control-Room-state.rs/ws.rs, Health-Types/-Config sowie Docs/MQTT_PHASE4_COMMAND_ACK_POLICY_OPENLAB.md. Daraus wird kein zusätzlicher Lampen-Betriebsnachweis abgeleitet.

**Verwandtes, aber anderer Projektentwurf:** [WERMA-Racksignalisierung, BPI-R4 Pro und Rack-Architektur](2026-10-03_werma-racksignalisierung-bpi-r4-pro-und-rack-architektur.md). Dort stehen andere Farbideen und ein zentraler Indicator-/Rack-Agent-Entwurf. Diese Datei wird unverändert bewahrt. Bei einer späteren gemeinsamen Implementation sind die unterschiedlichen Entwürfe ausdrücklich zu konsolidieren; keiner wird durch bloße Archivnähe als globale Entscheidung behandelt.

### 15.3 Externe technische Primärquellen

Am 04.10.2026 gelesen; externe master-/Dokumentationsseiten können später geändert werden:

- [E01: Linux v6.12 GPIO Device-Tree-Binding mit Hog-Anforderung][E01]
- [E02: Linux v6.12 gpiod_hog und gpiochip_request_own_desc][E02]
- [E03: Offizielle libgpiod-gpioset-Dokumentation][E03]
- [E04: Offizielle systemd.service-Quellmanpage, Type=oneshot][E04]
- [E05: systemd v255 Target-Default-Reihenfolge][E05]
- [E06: Raspberry-Pi-Dokumentation, config.txt GPIO][E06]

Die Freedesktop-HTML-Manpages waren bei einem zusätzlichen Abruf nicht zugänglich (HTTP 403); für systemd wurde die offizielle Projektquelle gelesen. Ein zunächst vermuteter Linux-Binding-YAML-Pfad war nicht vorhanden; die Prüfung erfolgte an der tatsächlich abrufbaren v6.12-GPIO-Dokumentation und Implementation. Das sind Recherchegrenzen, keine beobachteten Anlagenfehler.

### 15.4 Anhangsinventar

Seitenzahlen sind tatsächlich per PDF-Metadaten erfasst. Titel/Versionen stammen aus Deckblättern. Für diesen Lampenentwurf handelt es sich um allgemeinen Projekt-Referenzbestand, nicht um einen Nachweis einer dort spezifizierten Lampenhardware.

| Datei | Seiten | Identifikation am Deckblatt |
|---|---:|---|
| en_3003920308v010401p.pdf | 22 | EN 300 392-3-8 V1.4.1; ISI Generic Speech Format Implementation |
| en_30039209v010701p.pdf | 46 | EN 300 392-9 V1.7.1; General requirements for supplementary services |
| ts_10081201v020205p.pdf | 8 | TS 100 812-1 V2.2.5; SIM-ME/UICC physical and logical characteristics |
| en_3003921201v010202p.pdf | 56 | EN 300 392-12-1 V1.2.2; Call Identification, stage 3 |
| en_3003920304v010301p.pdf | 28 | EN 300 392-3-4 V1.3.1; ANF-ISISDS |
| en_3003921117v010102p.pdf | 18 | EN 300 392-11-17 V1.1.2; Include Call |
| en_3003921114v010101p.pdf | 23 | EN 300 392-11-14 V1.1.1; Late Entry |
| es_20081202v020401m.pdf | 139 | Final draft ES 200 812-2 V2.4.1; TSIM application |
| es_20081201v020205p.pdf | 8 | ES 200 812-1 V2.2.5; TSIM-ME/UICC physical and logical characteristics |
| en_300812v020101p.pdf | 156 | EN 300 812 V2.1.1; SIM-ME interface/security aspects |
| en_3003921101v010201p.pdf | 44 | EN 300 392-11-1 V1.2.1; Call Identification, stage 2 |
| en_3003921006v010401p.pdf | 20 | EN 300 392-10-6 V1.4.1; Call Authorized by Dispatcher |
| en_3003921018v010301p.pdf | 17 | EN 300 392-10-18 V1.3.1; Barring of Outgoing Calls |
| en_3003921216v010400a.pdf | 67 | Draft EN 300 392-12-16 V1.4.0; Pre-emptive Priority Call |
| en_30039201v010601p.pdf | 182 | EN 300 392-1 V1.6.1; General network design |
| ets_30039214e01v.pdf | 61 | Final draft prETS 300 392-14; PICS proforma |
| en_30039207v030501p.pdf | 216 | EN 300 392-7 V3.5.1; Security |
| en_30039401v030301p.pdf | 169 | EN 300 394-1 V3.3.1; Conformance testing/radio |
| en_3003920313v010201p.pdf | 191 | EN 300 392-3-13 V1.2.1; Transport-independent ANF-ISIGC |
| en_30039502v010303p.pdf | 94 | EN 300 395-2 V1.3.3; TETRA codec |
| en_3003920303v010301p.pdf | 251 | EN 300 392-3-3 V1.3.1; ANF-ISIGC |
| en_30039205v020701p.pdf | 320 | EN 300 392-5 V2.7.1; Peripheral Equipment Interface |
| en_3003920315v010500a.pdf | 380 | Draft EN 300 392-3-15 V1.5.0; Transport-independent ANF-ISIMM |
| en_30039202v030801p.pdf | 1445 | EN 300 392-2 V3.8.1; Air Interface |
| ETSI.pdf | 4100 | Sammel-/Groß-PDF; beginnt mit EN 300 812 V2.1.1, übriger Gesamtinhalt nicht vollständig geprüft |

Die PDFs werden für diesen Lampenentwurf nicht als angebliche WERMA-Entwicklungsartefakte ins Git dupliziert. Die ursprünglichen bereitgestellten Quellen bleiben unverändert.

## 16. Wiederaufnahme

Bei Fortsetzung zuerst Abschnitt 3, 8, 10, 11 und 14 lesen: Sie enthalten die übernommenen Entscheidungen, Fehler des früheren Beispielcodes, Bootübergabeprobleme, vorhandene Integrationsbausteine und konkrete nächste Schritte.

[R01]: https://github.com/JanHG98/netcore-tetra/blob/11136de497f939bf3056dbccc4004a58ca99f344/bins/bluestation-bs/src/main.rs
[R02]: https://github.com/JanHG98/netcore-tetra/blob/11136de497f939bf3056dbccc4004a58ca99f344/crates/tetra-entities/src/net_telemetry/events.rs
[R03]: https://github.com/JanHG98/netcore-tetra/blob/11136de497f939bf3056dbccc4004a58ca99f344/bins/netcore-control-room/src/http.rs
[R04]: https://github.com/JanHG98/netcore-tetra/blob/11136de497f939bf3056dbccc4004a58ca99f344/bins/netcore-control-room/src/auth.rs
[R05]: https://github.com/JanHG98/netcore-tetra/blob/11136de497f939bf3056dbccc4004a58ca99f344/system-backend/hardware-gateway/src/netcore_hardware_gateway.py
[R06]: https://github.com/JanHG98/netcore-tetra/blob/11136de497f939bf3056dbccc4004a58ca99f344/system-backend/iot-gateway/src/command.rs
[R07]: https://github.com/JanHG98/netcore-tetra/blob/11136de497f939bf3056dbccc4004a58ca99f344/system-backend/iot-gateway/docs/mqtt-contract.md
[R08]: https://github.com/JanHG98/netcore-tetra/blob/11136de497f939bf3056dbccc4004a58ca99f344/system-backend/iot-gateway/config/iot-gateway.example.toml
[R09]: https://github.com/JanHG98/netcore-tetra/blob/11136de497f939bf3056dbccc4004a58ca99f344/system-backend/hardware-gateway/config/hardware-gateway.example.toml
[E01]: https://github.com/torvalds/linux/blob/v6.12/Documentation/devicetree/bindings/gpio/gpio.txt
[E02]: https://github.com/torvalds/linux/blob/v6.12/drivers/gpio/gpiolib.c
[E03]: https://libgpiod.readthedocs.io/en/master/gpioset.html
[E04]: https://github.com/systemd/systemd/blob/main/man/systemd.service.xml
[E05]: https://github.com/systemd/systemd/blob/v255/src/core/target.c
[E06]: https://www.raspberrypi.com/documentation/computers/config_txt.html#gpio
