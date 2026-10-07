# Brainstorming: GPIO-Pass-Through, Jumpersteuerung und SXceiver

**Arbeitsstand:** 2026-10-05. Historische Betriebsbeobachtungen und der an diesem Datum geprüfte Repository-Stand sind getrennt ausgewiesen.

## 1. Arbeitsstand

| Feld | Wert |
| --- | --- |
| Thema | Industriell anmutende Hardware-Modi für die TETRA-Basisstation über Jumper/DIP-Schalter; Zugriff auf freie Raspberry-Pi-GPIO trotz aufgestecktem SXceiver/SDR |
| Sichtbarer Planungsbeginn | 2026-09-12, Europe/Berlin |
| Zusammenfassung erstellt | 2026-10-05 |
| Zielrepository | JanHG98/netcore-tetra |
| Archivbranch vor diesem Archivcommit | Archiving@5e2386f849584ef48540743eb45eb875b346036c |
| Zusätzlich geprüfter aktueller Produktstand | main@9116c15d645458f99e236712b67a1ad970432791 |
| Branchvergleich beim Schreiben | Archiving → main: diverged; Archiving 68 Commits hinter und 5 Commits vor main |
| Ablage | ausschließlich Docs/archive/ |
| Prüfart | Hardwarekonzept und statische Repository-Prüfung; elektrische und On-Air-Abnahme offen |

Die Umsetzung wird nach Idee, Planung, vorhandenem Code und Testbelegen unterschieden. `Archiving` und `main` waren am Prüfdatum divergiert; Softwarebefunde beziehen sich deshalb gesondert auf den benannten `main`-Commit.

## 2. Ziel, Ausgangslage und behandelte Themen

Ausgangspunkt war die Idee, die NetCore-Tetra-Basisstation hardwareseitig stärker wie eine Industrie-/Telekommunikationssteuerung aufzubauen: Betriebsarten und Wartungszustände sollen nicht ausschließlich in einer Weboberfläche oder Konfigurationsdatei existieren, sondern optional über physische Jumper beziehungsweise DIP-Schalter sichtbar und eindeutig vorgegeben werden können.

Der erste Entwurf ging davon aus, dass der Raspberry-Pi-40-Pin-Header wegen des aufgesteckten SXceiver/SDR praktisch nicht erreichbar sei. Daraus entstand zunächst die Idee eines separaten Supervisor-Controllers per USB. Diese Annahme wurde präzisiert: Der SDR belegt voraussichtlich nicht jeden GPIO. Anschließend wurde ein stackbares Raspberry-Pi-Breakout-Board mit Pass-Through-Header und Schraubklemmen als wesentlich einfacherer Ansatz identifiziert.

Der finale Planungsstand ist damit kein eigener Supervisor als Startpunkt, sondern:

~~~text
SXceiver / SDR
      │
      │ 40-Pin Pass-Through
      ▼
GPIO-Breakout / Interposer
      │
      ├── seitliche Schraubklemmen für tatsächlich freie Pins
      ├── Jumper / DIP / Service-Schalter
      └── optional spätere LEDs / Sensorik / I²C-Erweiterung
      │
      ▼
Raspberry Pi
~~~

Das Breakout dient nur zum Herausführen der Signale. Es macht einen vom SXceiver verwendeten GPIO nicht frei.

## 3. Statusbegriffe dieser Dokumentation

- **Idee:** in der Diskussion vorgeschlagen, aber nicht verbindlich festgelegt.
- **Beschlossen/geplant:** als bevorzugter weiterer Weg festgehalten, aber noch nicht umgesetzt.
- **Implementiert:** im geprüften Repository tatsächlich als Code/Dokument/Hardwaredefinition vorhanden.
- **Getestet:** mit einem konkret beschriebenen Test nachgewiesen.
- **Im Betrieb bestätigt:** auf der realen Basisstation beziehungsweise realer Hardware dauerhaft oder reproduzierbar bestätigt.

## 4. Endgültige Anforderungen und Entscheidungen

### 4.1 Beschlossen/geplant: stackbares GPIO-Breakout als bevorzugter erster Hardwareweg

Der bevorzugte Aufbau ist ein Raspberry-Pi-Breakout-Board, das auf den 40-Pin-Header gesteckt wird und den Header nach oben für den SXceiver weiterführt. Die seitlichen Klemmleisten stellen die Pins zusätzlich zur Verfügung. Ein Freenove-Produktbild und eine Herstellerdarstellung zeigen den vorgesehenen Einsatz direkt auf einem Raspberry Pi.

Damit gilt die frühere Annahme „kein Zugriff auf Pi-GPIO wegen SDR“ als **überholt**. Korrekt ist: GPIO-Zugriff ist mechanisch möglich; elektrisch dürfen ausschließlich Pins verwendet werden, die der konkrete SXceiver in der konkreten Hardwareversion nicht nutzt beziehungsweise deren Mehrfachnutzung ausdrücklich zulässig ist.

### 4.2 Beschlossen/geplant: vor Verdrahtung eine echte Pinmatrix erstellen

Vor dem ersten Jumper muss die 40-Pin-Belegung der realen Kombination aus Raspberry Pi, SXceiver-HAT-Revision und aktuellem Treiber/Overlay festgehalten werden. Die Pinmatrix soll mindestens unterscheiden:

- Versorgung und Masse;
- vom SXceiver fest belegt;
- vom SXceiver versionsabhängig belegt;
- für SPI0 beziehungsweise I²S benötigt;
- für HAT-/EEPROM-Zwecke reserviert;
- potenziell frei;
- absichtlich für NetCore-Hardwarefunktionen reserviert.

Eine reine Durchschleifplatine ist kein Beleg dafür, dass eine Leitung frei ist.

### 4.3 Idee: physische Betriebsmodi über Jumper/DIP

In der Planung wurden unter anderem folgende Kandidaten vorgeschlagen. Es wurde **keine endgültige Bitbelegung** beschlossen:

- LOCAL / STANDALONE
- HYBRID
- COMMISSIONING
- SAFE MODE
- RECOVERY
- RF INHIBIT
- CONFIG LOCK
- SERVICE
- optional Node-ID beziehungsweise Hardwareprofil per Binär-DIP

Ein sinnvoller späterer Entwurf kann mehrere Modi binär kodieren, statt für jeden Modus einen eigenen GPIO zu verbrauchen. Drei Eingänge erlauben acht codierte Zustände. Separate sicherheits- oder wartungsrelevante Eingänge wie RF INHIBIT oder CONFIG LOCK sollten davon getrennt bewertet werden.

### 4.4 Idee: aktive-low Eingänge mit Pull-up

Als einfache Schaltung wurde vorgeschlagen:

~~~text
GPIO ----+---- interner/externer Pull-up nach 3,3 V
         |
       Jumper
         |
        GND
~~~

Damit wäre „Jumper offen = HIGH“ und „Jumper gesteckt = LOW“. Dies wurde in der Planung **nicht elektrisch aufgebaut oder getestet**. Für einen fertigen Entwurf müssen Bootzustände, Störfestigkeit, ESD, Kabellänge, Entprellung/Filterung und das Verhalten bei Floating/Defekt bewusst festgelegt werden.

### 4.5 Idee: RF INHIBIT besonders behandeln

RF INHIBIT wurde als besonders wertvoller Wartungsmodus herausgestellt. Zwei unterschiedliche Ebenen sind zu trennen:

1. **Software-Inhibit:** Jumperzustand wird von NetCore gelesen und der TX-Pfad wird softwareseitig nicht gestartet.
2. **Echter Hardware-Inhibit:** eine physische TX-/PA-Enable-Leitung wird unabhängig von Linux gesperrt.

Nur Variante 2 ist ein tatsächlicher Hardware-Lockout. Ob der aktuelle SXceiver beziehungsweise eine nachgeschaltete PA eine geeignete Enable-Leitung bereitstellt, wurde für diesen Arbeitsstand nicht nachgewiesen.

### 4.6 Beschlossen/geplant: Zusatzcontroller zunächst nicht erzwingen

Der zuerst vorgeschlagene RP2040/STM32-Supervisor per USB ist als **spätere Ausbauoption** erhalten, aber für den ersten Prototyp nicht mehr bevorzugt. Solange genügend bestätigte freie GPIO vorhanden sind, ist das direkte Breakout einfacher.

Ein Supervisor wird wieder interessant, wenn NetCore später unabhängig von Linux Hardware-Watchdog, Power-Cycle, Lüfterregelung, Messwerterfassung, galvanisch getrennte I/O oder einen echten RF-Hardware-Lockout benötigt.

### 4.7 Idee: I²C-Portexpander nur bei Bedarf

Ein MCP23017 oder vergleichbarer I²C-Portexpander wurde als Fallback vorgeschlagen, falls nur wenige Pi-GPIO sicher frei sind. Im aktuellen main wurde beim Archivieren **kein MCP23017-spezifischer Codefund** gefunden. Daher: Architekturidee, nicht Implementierung.

## 5. Architektur, Komponenten, Schnittstellen und Abhängigkeiten

### 5.1 Zielarchitektur des ersten Prototyps

~~~text
┌──────────────────────────────┐
│ SXceiver / SDR HAT           │
└──────────────┬───────────────┘
               │ 40-Pin
┌──────────────▼───────────────┐
│ Stackbares GPIO-Breakout     │
│                              │
│ Pass-Through 1:1             │
│ Schraubklemmen               │
│        │                     │
│        ├── Jumper/DIP        │
│        ├── Service-Taster    │
│        └── später LEDs/I²C   │
└──────────────┬───────────────┘
               │ 40-Pin
┌──────────────▼───────────────┐
│ Raspberry Pi / NetCore-TBS   │
└──────────────────────────────┘
~~~

### 5.2 Elektrische Schnittstellen

Für den diskutierten Ausbau sind primär 3,3-V-GPIO und GND relevant. 5-V-Leitungen des Pi sind **nicht** als GPIO-Pegel zu behandeln. Das Breakout führt Versorgungspins ebenfalls heraus; daraus folgt keine Freigabe, externe Signale ungeprüft anzuschließen.

Mögliche spätere Erweiterung:
- I²C für Portexpander/Sensorik;
- Status-LEDs;
- Temperatur-/Spannungsmessung;
- Lüftersteuerung;
- Watchdog/Power-Control;
- Service-/Recovery-Eingänge.

### 5.3 SXceiver-Abhängigkeiten im aktuell geprüften main

Die statische Prüfung von main@9116c15d645458f99e236712b67a1ad970432791 ergab:

- sxxcvr-main/dts/sx1255_raspberrypi.dts aktiviert I²S sowie SPI0 für den SX1255-Pfad.
- sxxcvr-main/dts/Makefile erzeugt HAT-EEPROM-Einstellungen und reserviert versionsabhängige Steuer-GPIO:
  - HAT-Version 0x0100: TX control BCM GPIO 12, RX control BCM GPIO 13.
  - andere Versionen; der aktuelle Default im Makefile ist 0x0102: TX control BCM GPIO 22, RX control BCM GPIO 23.
- Diese Angaben sind BCM-GPIO-Nummern; sie ersetzen noch keine vollständige physische 40-Pin-Matrix.
- Das Device-Tree-Overlay selbst dokumentiert nicht vollständig, welche physischen Headerpins in jeder Pi-/HAT-Kombination am Ende exklusiv belegt sind.

Die Konsequenz aus den Projektaufzeichnungen bleibt daher korrekt: **erst Pinout verifizieren, dann Klemmen nutzen.**

### 5.4 Bezug zum vorhandenen NetCore-Hardware-Stack

main enthält bereits system-backend/hardware-gateway mit MQTT-/HTTP-bezogener Hardwaretelemetrie und WebUI. Das ist ein sinnvoller vorhandener Integrationspunkt für einen späteren Jumper-/Sensorstatus. Es wurde in der Planung jedoch keine konkrete API-Anbindung implementiert.

Außerdem existiert ein Software-Health-Watchdog im TBS-Code. Dieser ist von einem zukünftigen **Hardware-Watchdog** ausdrücklich zu unterscheiden.

## 6. Erreichter Entwicklungs- und Betriebsstand

### Planungsstand

| Punkt | Status | Nachweis |
| --- | --- | --- |
| Industrielle Jumper-/DIP-Idee | Idee | Hardwarekonzept |
| Stackbares Breakout zwischen Pi und SXceiver | beschlossen/geplant | Produktbilder eines Raspberry-Pi-Pass-Through-Boards und seiner Montage |
| Konkretes Freenove-Board gekauft/eingebaut | nicht bestätigt | kein Einbau-/Bestellnachweis in der Planung |
| Exakte freie GPIO bestimmt | offen | in der Planung nicht vermessen |
| Jumper elektrisch aufgebaut | nicht implementiert | kein Hardwareaufbau |
| Jumper von NetCore eingelesen | nicht implementiert | kein Codeauftrag/Commit |
| RF INHIBIT | Idee | Software-/Hardwarevarianten diskutiert |
| MCP23017 | Idee | nur Fallbackvorschlag |
| USB-Supervisor | ersetzter erster Ansatz / spätere Option | durch Breakout-Konzept als Startpunkt abgelöst |
| Mechanischer Stack getestet | offen | nur Hersteller-/Produktdarstellung |
| On-Air- oder Dauerbetrieb | nicht getestet | kein entsprechender Test |

### Aktueller Repository-Stand, getrennt vom historischen Arbeitsstand

Auf main@9116c15d645458f99e236712b67a1ad970432791 ist die allgemeine Hardwareidee bereits in wiki/Hardware-und-RF.md und wiki/Roadmap.md verankert. Dort wird eine GPIO-/Rack-Platine mit Sensorik, Lüftern, Statusanzeigen, Watchdog und Stromversorgung als Ausbauziel geführt und ausdrücklich ein Schaltplan-/Pinout-Abgleich des SXceiver verlangt. Ein fertiger und geprüfter PCB-Stand wird dort nicht behauptet.

Die Suche auf main ergab am Prüfdatum:
- vorhandene SXceiver-HAT-/Overlay-Definitionen;
- vorhandenes Hardware-Gateway;
- vorhandenen Software-Watchdog;
- **keinen** spezifischen MCP23017-Fund;
- **keinen** spezifischen „RF inhibit“-Fund als implementierte Funktion.

Damit stimmt das aktuelle Repository mit dem Schluss der Planung überein: Die Idee passt zur Roadmap, ist aber noch kein implementiertes Jumper-Subsystem.

## 7. Relevante Dateien, Dienste, Protokolle und technische Parameter

| Element | Bedeutung für diese Planung | Status |
| --- | --- | --- |
| sxxcvr-main/dts/Makefile | HAT-Version und TX/RX-Control-GPIO | implementiert im geprüften main |
| sxxcvr-main/dts/sx1255_raspberrypi.dts | I²S-/SPI0-Aktivierung für SX1255 | implementiert im geprüften main |
| wiki/Hardware-und-RF.md | Hinweis auf HAT-Pinprüfung und geplante Leiterplatte | dokumentiert |
| wiki/Roadmap.md | GPIO-/Rack-Platine als Ausbauziel | geplant |
| system-backend/hardware-gateway/ | bestehender NetCore-Pfad für Hardware-/Racktelemetrie | Code vorhanden; Livebetrieb für diesen Arbeitsstand nicht geprüft |
| config.toml / health-Code | Software-Watchdog für TBS-Core | Code vorhanden; kein Hardware-Watchdog |
| GPIO-Pegel | 3,3 V Logik | Hardware-Randbedingung |
| SPI0 / I²S | vom SXceiver-Stack aktiviert | nicht für freie Jumper verplanen, bevor Pinmatrix bestätigt ist |
| BCM 22 / 23 | TX/RX-Control bei HAT-Versionen ungleich 0x0100 im aktuellen Makefile | versionsabhängig belegt |
| BCM 12 / 13 | TX/RX-Control bei HAT-Version 0x0100 | versionsabhängig belegt |

Keine neuen Ports, Daemons oder Netzwerkprotokolle wurden für diesen Arbeitsstand verbindlich eingeführt.

## 8. Wichtige Befehle und Abläufe

In der ursprünglichen Planung wurde **kein Installations-, Build-, Deployment- oder Reparaturbefehl auf der realen TBS ausgeführt**. Es gab lediglich Architektur- und Pseudokonfigurationsbeispiele.

Daraus folgt:
- kein erfolgreicher GPIO-Test ist belegt;
- kein systemd-Dienst für Jumper wurde erstellt;
- kein /dev/ttyACM0-Supervisor wurde eingerichtet;
- keine MQTT-Topics für diese Platine wurden produktiv bestätigt.

Der im frühen, inzwischen nachrangigen Supervisor-Entwurf genannte serielle USB-Pfad war ein Beispiel und darf nicht als vorhandenes Gerät dokumentiert werden.

## 9. Fehler, Diagnose, Ursachen und Lösungen

### 9.1 „Auf Pi-GPIO kann ich nicht zugreifen, weil dort der SDR steckt“

**Diagnose:** mechanisch stimmt das zunächst; der Header ist vom HAT belegt. Elektrisch folgt daraus aber nicht, dass alle GPIO belegt sind.

**Funktionierende konzeptionelle Lösung:** stackbares 40-Pin-Pass-Through-Breakout zwischen Pi und SXceiver. Der SXceiver bleibt aufgesteckt; freie Pins werden parallel über Schraubklemmen zugänglich.

**Restproblem:** Welche Pins tatsächlich frei sind, ist für die konkrete HAT-Revision noch nicht abschließend belegt.

### 9.2 Zu frühe Annahme eines separaten USB-Supervisors

Der erste Lösungsweg sprang direkt zu RP2040/STM32 per USB. Das ist technisch weiterhin sinnvoll für einen echten Supervisor, war aber unnötig komplex für das unmittelbare Ziel „ein paar Hardwaremodi/Jumper“.

**Korrektur:** zunächst vorhandene freie GPIO nutzen; I²C-Expander oder MCU erst bei nachgewiesenem Bedarf.

### 9.3 Zweifel, ob das Breakout tatsächlich stapelbar ist

Zunächst wurde als Risiko genannt, dass ein beliebiges Breakout nicht automatisch einen nach oben nutzbaren Header besitzt.

**Korrektur durch späteres Originalbild:** Das gezeigte Freenove-Raspberry-Pi-Board ist ausdrücklich für die Montage auf dem Pi gedacht und zeigt den weiterhin zugänglichen zentralen 40-Pin-Header. Damit ist die mechanische Grundidee plausibel. Die tatsächliche Bauhöhe mit dem SXceiver bleibt trotzdem zu prüfen.

## 10. Tests und Ergebnisse

### In der Planung tatsächlich erfolgt

1. **Visuelle Prüfung der Produktbilder:**\
   - erstes Bild: OSOYOO Breakout Board for Pico Series; nur als Stil-/Breakout-Beispiel relevant, nicht als Raspberry-Pi-HAT-Lösung;
   - zweites Bild: Freenove Raspberry-Pi-Breakout mit Schraubklemmen und zentralem 40-Pin-Header;
   - drittes Bild: Herstellerdarstellung des Freenove-Boards auf Raspberry Pi; bestätigt den vorgesehenen Stack-Einsatz.

2. **Keine elektrische Prüfung:** keine Durchgangsmessung, kein GPIO-read, kein Logic-Analyzer, kein Boot-/RF-Test.

### Zusätzlich am 2026-10-05 statisch geprüft

- aktueller main-Commit: 9116c15d645458f99e236712b67a1ad970432791;
- SXceiver-Device-Tree aktiviert I²S und SPI0;
- HAT-EEPROM-Makefile enthält versionsabhängige TX/RX-Control-Pins;
- Hardware-/RF-Wiki warnt vor ungeprüfter GPIO-Verwendung;
- GPIO-/Rack-Platine steht bereits als Ausbauziel in der Wiki-Roadmap;
- Hardware-Gateway ist als vorhandener Softwarebaustein im Repository vorhanden.

**Grenze:** Statische Repository-Prüfung beweist weder die elektrische Belegung des real aufgesteckten Boards noch einen laufenden Dienst auf der realen Basisstation.

## 11. Verworfene oder ersetzte Ansätze

### Als erster Schritt ersetzt: separater RP2040/STM32-Supervisor

Ursprünglich vorgeschlagen, weil der Pi-Header vermeintlich vollständig blockiert sei. Nach der Erkenntnis, dass ein Pass-Through-Breakout den Zugriff auf freie Leitungen ermöglicht, ist dieser Ansatz für den ersten Jumper-Prototyp unnötig.

Nicht vollständig verworfen: Ein unabhängiger Supervisor bleibt für spätere Watchdog-/Power-/Safety-Funktionen attraktiv.

### Nicht übernommen: blindes Verwenden sämtlicher herausgeführter Klemmen

Ein Breakout stellt Leitungen zugänglich dar, garantiert aber nicht deren Freiheit. Diese Fehlinterpretation ist ausdrücklich zu vermeiden.

### Nicht als Finaldesign übernommen: direkte Frequenz-/MCC-/MNC-Wahl per Jumper

In der Diskussion wurde empfohlen, keine konkreten Funkparameter hart über Jumper zu kodieren. Wenn überhaupt, sollten Jumper Hardware-/Betriebsprofile auswählen; eigentliche Netz- und RF-Parameter verbleiben in kontrollierter Konfiguration.

## 12. Bilder und Anhänge

Drei Produktbilder sind als **SVG-Archivtranskriptionen** unter `Docs/archive/assets/2026-10-05_gpio-breakout-jumper/` erhalten. Sie dokumentieren sichtbare Merkmale und Beschriftungen; die Originalraster wurden geprüft, konnten aber nicht direkt in den Git-Commit übernommen werden. Die SVGs sind **keine bitidentischen Kopien** der Rasterbilder.

Zur Nachvollziehbarkeit wurden die Original-Rasterdaten lokal geprüft:

| Bild | Originalabmessung | SHA-256 des Originalrasters | Archivasset |
| --- | ---: | --- | --- |
| OSOYOO Pico-Series Breakout | 2000 × 2000 PNG | 0719cc4856656ca8904bc623541aa7ace3683daf81cad2a98a459d0c15bae938 | [01_osoyoo-breakout-board.svg](assets/2026-10-05_gpio-breakout-jumper/01_osoyoo-breakout-board.svg) |
| Freenove Raspberry-Pi-Breakout, Produktansicht | 1441 × 1383 PNG | 6a22187581d9a38c5c906aa1444d4c3016cd4229892cfea975e31bb314761bbf | [02_freenove-breakout-board.svg](assets/2026-10-05_gpio-breakout-jumper/02_freenove-breakout-board.svg) |
| Freenove Breakout auf Raspberry Pi | 1443 × 1170 PNG | 5f504348ac539c58b211eb78290b680f7f3fabeb18500819bd1e964498bd4358 | [03_freenove-stacked-on-pi.svg](assets/2026-10-05_gpio-breakout-jumper/03_freenove-stacked-on-pi.svg) |

Andere im Projektkontext vorhandene ETSI-PDFs wurden für diesen GPIO-/Breakout-Planung nicht benötigt und deshalb nicht als zugehörige Anhänge dieses Archivs dupliziert.

## 13. Roadmap-Kandidaten aus dieser Planung

Die folgenden Kandidaten bauen auf einem bestätigten Breakout-Prototyp auf.

### P0 – vor jeder Verdrahtung

1. Konkrete SXceiver-Hardwareversion feststellen.
2. Vollständige BCM-/Physical-Pin-Matrix für Pi + Breakout + SXceiver erstellen.
3. SPI0, I²S, HAT-/EEPROM-Funktionen und TX/RX-Control eindeutig markieren.
4. Nur danach NetCore-freie GPIO reservieren.

### P1 – mechanischer und elektrischer Minimalprototyp

1. Pi → Breakout → SXceiver mit passenden Abstandshaltern aufbauen.
2. Bauhöhe, Kühlung, Headerkontakt, Schraubzug und Kollisionsfreiheit prüfen.
3. Einen einzigen bestätigten freien GPIO als Jumper-Eingang testen.
4. Danach zwei bis drei Eingänge für codierten Betriebsmodus ergänzen.
5. 3,3-V-Pegel, Bootzustand und Fehlerzustand reproduzierbar prüfen.

### P1 – Softwarevertrag

1. Festlegen, welche Jumper nur beim Boot gelesen werden und welche live wirken.
2. Hardwarezustand read-only in TBS-WebUI/Status anzeigen.
3. Ereignisse/Audit bei Hardwaremoduswechsel erfassen.
4. Vorhandenes Hardware-Gateway als Integrationspunkt prüfen, statt einen parallelen Telemetriestack zu erfinden.
5. Fallback definieren, wenn Jumperzustand und Softwarekonfiguration widersprechen.

### P2 – optionale Erweiterungen

- MCP23017 oder anderer I²C-Expander, falls freie GPIO knapp sind;
- Status-LEDs;
- Node-/Site-ID per DIP nur bei echtem Nutzen;
- Temperatur/Spannung/Lüfter;
- unabhängiger MCU-Supervisor;
- Hardware-Watchdog und Power-Cycle;
- physischer RF-/PA-Inhibit, falls die RF-Hardware dafür eine echte Enable-Schnittstelle bereitstellt.

### P3 – eigene NetCore-Platine

Erst nach bewährtem Breakout-Prototyp eine eigene TBS-Interposer-/Supervisor-PCB entwickeln. Sie sollte dann die tatsächlich gemessene Pinbelegung, EMV, Versorgung, Schutz, Testpunkte, Servicezugang und mechanische Befestigung berücksichtigen.

## 14. Konkrete nächste Schritte

Der unmittelbar nächste Schritt ist **nicht** das Anlöten von Jumpern, sondern die belastbare Pinbelegung:

1. reale SXceiver-HAT-Version auslesen/ablesen;
2. Repository-Makefile und Device-Tree mit dem realen Board abgleichen;
3. physische 40-Pin-Tabelle BCM ↔ Headerpin ↔ SXceiver-Funktion erstellen;
4. einen freien GPIO auswählen;
5. Breakout mechanisch montieren;
6. mit einem einzelnen Jumper den Boot- und Laufzeitzustand testen;
7. erst danach den Modusvertrag und die NetCore-Softwareintegration festlegen.

Abhängigkeit für alles Weitere ist die bestätigte freie Pinmenge. Sind genügend Pins frei, bleibt die Lösung direkt. Sind nur SDA/SCL sicher verfügbar, wird der I²C-Expander wieder relevant. Gibt es Anforderungen an unabhängigen Watchdog oder Power-Cycle, wird der Supervisor-MCU ein eigener Ausbaupfad.

## 15. Relevante Quellen und Repository-Bezüge

Geprüft am 2026-10-05:

- main@9116c15d645458f99e236712b67a1ad970432791
- Archiving vor diesem Archivcommit: 5e2386f849584ef48540743eb45eb875b346036c
- sxxcvr-main/dts/Makefile
- sxxcvr-main/dts/sx1255_raspberrypi.dts
- wiki/Hardware-und-RF.md
- wiki/Roadmap.md
- ROADMAP.md
- system-backend/hardware-gateway/src/netcore_hardware_gateway.py
- system-backend/hardware-gateway/web-ui/index.html
- config.toml sowie Health-/Watchdog-Code für die Abgrenzung Software- versus Hardware-Watchdog

Es wurde kein PR und kein früherer Commit als Beleg für eine bereits implementierte Jumperplatine gefunden beziehungsweise behauptet.

## 16. Offene Nachweise

- Die drei Rasterbilder sind verfügbar und wurden visuell ausgewertet; im Repository werden aus Connector-Gründen SVG-Archivtranskriptionen statt bitidentischer Binärkopien abgelegt.
- Die konkrete SXceiver-Hardwareversion der realen Basisstation wurde für diesen Arbeitsstand nicht eindeutig festgehalten.
- Keine Messung belegt die freie GPIO-Menge.
- Keine elektrische, thermische, mechanische oder RF-Abnahme des geplanten Stacks wurde durchgeführt.
