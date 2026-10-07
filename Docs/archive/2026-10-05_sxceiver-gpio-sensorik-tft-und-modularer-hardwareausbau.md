# Brainstorming: SXceiver-GPIO, Sensorik, TFT und modularer Hardwareausbau

## 1. Rahmen

| Feld | Wert |
| --- | --- |
| Thema | Reale GPIO-Belegung des SXceiver-HAT, sinnvolle Nutzung freier Raspberry-Pi-Pins, Sensorik/Aktoren/Passivelemente, lokales TFT/LCD und Entscheidung gegen eine frühe Eigen-PCB |
| Beginn der Entwicklungsnotizen | 2026-10-03 |
| Notizstand | 2026-10-05 |
| Zielrepository | JanHG98/netcore-tetra |
| Zielbranch | Archiving |
| Archivbranch vor der ursprünglichen Dokumentation | Archiving@ded2c14a713f65196e3f28a4640b4e317f84b59e |
| Zusätzlich geprüfter aktueller Produktstand | main@9116c15d645458f99e236712b67a1ad970432791 |
| Branchvergleich vor dem Schreiben | main...Archiving: divergiert; Archiving 71 Commits vor und 5 Commits hinter main |
| Ablage | ausschließlich Docs/archive/ |
| Nachweischarakter | Entwurfsstand + statische Repository-Prüfung; keine elektrische, thermische, EMV- oder On-Air-Abnahme |

**Wichtige Nachweisgrenze:** Aussagen aus den Entwicklungsnotizen sind nicht automatisch implementiert. Diese Dokumentation trennt konsequent zwischen **Idee**, **beschlossen/geplant**, **implementiert**, **getestet** und **im Betrieb bestätigt**.

## 2. Ziel, Ausgangslage und behandelte Themen

Ausgangspunkt ist die reale GPIO-Belegung des SXceiver und die sinnvolle Nutzung freier Raspberry-Pi-I/O. Priorität haben **Sensorik, Aktoren, passive Schutz-/Beschaltungselemente und lokale Anzeige**; Jumper und Konfigurationsschalter sind nur ein Teil des möglichen Ausbaus.

Daraus ergibt sich ein Hardware-Management-Konzept für die Basisstation:

- SXceiver bleibt primärer RF-/SDR-Baustein.
- Freie Pi-I/O sollen nach belastbarer Pinprüfung für lokale Hardwarefunktionen genutzt werden.
- Sensorik und langsame I/O sollen bevorzugt über einen gemeinsamen I²C-Bus und fertige Module angebunden werden.
- Ein lokales LCD/TFT ist ausdrücklich erwünscht.
- Eine eigene, komplexe PCB wurde am Ende **nicht** als kurzfristiger Weg gewünscht; der Prototyp soll modular aus fertigen Breakouts/Modulen aufgebaut werden.
- Eine spätere Integrationsplatine bleibt nur dann sinnvoll, wenn der modulare Aufbau stabil ist und mehrfach reproduziert werden soll.

Damit verschob sich die Priorität im Verlauf deutlich: von „welche Jumper legen wir auf freie GPIO?“ hin zu „wie machen wir aus den freien Ressourcen eine kleine, wartbare Hardware-Management-Schicht der TBS?“.

## 3. Relevante Beobachtungen am realen Aufbau

Am realen SXceiver-/Breakout-Aufbau wurde beobachtet, dass auf dem Board bei folgenden Beschriftungen Aktivität beziehungsweise LED-Anzeigen sichtbar waren:

### Linke Seite
- +3V3
- SDA
- SCL1
- IO4
- IO22
- +3V3
- IO6
- IO19

### Rechte Seite
- +5V
- +5V
- IO23
- IO8
- IO7
- IO20
- IO21

Diese Liste ist ein **Praxis-Hinweis**, aber kein elektrischer Beleg dafür, dass jeder genannte GPIO durch den SXceiver funktional belegt ist. LEDs auf einem Breakout oder eine sichtbare Pegelanzeige können Versorgung, Pull-Zustände, Bootzustände oder reale Busaktivität anzeigen. Deshalb wurde die Repository-Prüfung gegen den tatsächlichen SoapySX-/HAT-Code vorgenommen.

## 4. Korrektur der SXceiver-Pinannahmen

### 4.1 Frühere, inzwischen überholte Annahme

Die frühe generische SX1255-/M17-Belegung behandelte **GPIO25 als Reset**. Diese Annahme ist für den geprüften SXceiver-/SoapySX-Pfad überholt.

Das ist für den am 05.10.2026 geprüften SXceiver-/SoapySX-Stand **nicht korrekt** und gilt für diese Dokumentation als überholt.

### 4.2 Am Prüfstand 05.10.2026 im Repository statisch belegter Stand

main@9116c15d645458f99e236712b67a1ad970432791 enthält unter sxxcvr-main/ den relevanten SoapySX- und HAT-Code.

Aus sxxcvr-main/SoapySX/SoapySX.cpp ergibt sich:

- SPI-Gerät: /dev/spidev0.0
- GPIO-Chip: /dev/gpiochip0
- SX1255 Reset: **BCM GPIO5**
- RX-Control:
  - HAT 0x0100: **BCM GPIO13**
  - andere Versionen, damit insbesondere Default 0x0102: **BCM GPIO23**
- TX-Control:
  - HAT 0x0100: **BCM GPIO12**
  - andere Versionen: **BCM GPIO22**

Aus sxxcvr-main/dts/Makefile ergibt sich ergänzend:

- Default-HAT-Version: 0x0102
- HAT 0x0100: TX=GPIO12, RX=GPIO13
- sonst: TX=GPIO22, RX=GPIO23
- TX/RX-Control werden in der HAT-EEPROM-Konfiguration als Outputs mit Pull-Down angelegt.

Aus sxxcvr-main/dts/sx1255_raspberrypi.dts ergibt sich:

- I²S wird aktiviert.
- SPI0 wird aktiviert.
- der SX1255 wird aus Userspace über SPIDEV konfiguriert.

### 4.3 Daraus abgeleitete Pinmatrix für den aktuellen SXceiver-Pfad

Die Standardfunktionen von Raspberry Pi SPI0/I²S ergeben für den derzeitigen Aufbau folgende Arbeitsmatrix. Die physische Belegung muss am realen Pi/HAT weiterhin gegengeprüft werden.

| Funktion | BCM GPIO | Physischer 40-Pin-Pin | Status für NetCore-Erweiterungen |
| --- | ---: | ---: | --- |
| SPI0 CE0 | 8 | 24 | SXceiver – nicht verplanen |
| SPI0 MISO | 9 | 21 | SXceiver – nicht verplanen |
| SPI0 MOSI | 10 | 19 | SXceiver – nicht verplanen |
| SPI0 SCLK | 11 | 23 | SXceiver – nicht verplanen |
| I²S/PCM CLK | 18 | 12 | SXceiver – nicht verplanen |
| I²S/PCM FS | 19 | 35 | SXceiver – nicht verplanen |
| I²S/PCM DIN | 20 | 38 | SXceiver – nicht verplanen |
| I²S/PCM DOUT | 21 | 40 | SXceiver – nicht verplanen |
| SX Reset | 5 | 29 | SXceiver – nicht verplanen |
| SX TX control, HW 1.1/1.2 | 22 | 15 | SXceiver – nicht verplanen |
| SX RX control, HW 1.1/1.2 | 23 | 16 | SXceiver – nicht verplanen |
| SX TX control, HW 1.0 | 12 | 32 | versionsabhängig reserviert |
| SX RX control, HW 1.0 | 13 | 33 | versionsabhängig reserviert |

**Folge:** Für einen auf Hardwareversion 1.2 festgelegten Aufbau sind GPIO12/13 prinzipiell nicht durch die aktuelle SoapySX-Control-Logik belegt; für ein revisionsübergreifend kompatibles NetCore-Breakout sollte man sie trotzdem zunächst nicht fest verplanen.

GPIO0/1 beziehungsweise die ID-Pins des 40-Pin-Headers sollten wegen HAT-/EEPROM-Zwecken ebenfalls nicht als allgemeine NetCore-I/O eingeplant werden, solange die konkrete HAT-Verschaltung nicht verifiziert ist.

## 5. Kandidaten für tatsächlich sinnvolle freie I/O

Unter der Annahme des aktuellen HAT-Standes 1.2 und nach realer elektrischer Bestätigung sind folgende Gruppen besonders interessant:

- **GPIO2/3:** I²C-Backbone für Sensoren, ADC und I/O-Expander; Repository-Code des SXceiver nutzt diese Leitungen nicht als RF-Datenpfad.
- **GPIO14/15:** UART-Kandidaten für Service/Debug, nur wenn die serielle Konsole entsprechend behandelt wird.
- **GPIO6, 16, 17, 24, 25, 26, 27:** gute generische Kandidaten für Interrupts, Tacho, Taster oder dedizierte Steuersignale.
- **GPIO7:** SPI0 CE1; technisch möglicherweise frei, aber strategisch als zweite Chip-Select-Leitung besser zunächst reservieren.
- **GPIO12/13:** bei HAT 1.2 möglicherweise frei, bei HAT 1.0 aber TX/RX-Control; daher nicht als revisionssicher frei betrachten.
- **GPIO4:** möglicher allgemeiner GPIO beziehungsweise 1-Wire-Kandidat; erst nach Abgleich mit dem realen Board nutzen.

Das ist eine **Planungsmatrix**, keine bestätigte Freigabeliste. Der konkrete SXceiver-HAT, das Pass-Through-Board und die reale Pi-Version bleiben maßgeblich.

## 6. Endgültige Hardware-Richtung des Arbeitsstands

### 6.1 Beschlossen/geplant: Sensorik statt GPIO nur für Jumper zu „verbrauchen“

Festlegung: freie GPIO nicht überwiegend für Jumper oder Betriebsmodi verwenden. Der bevorzugte Nutzen ist lokale Zustands- und Hardwareüberwachung.

Als sinnvolle Sensorik wurden festgehalten:

| Messgröße | Geeigneter Ansatz | Schnittstelle | Status |
| --- | --- | --- | --- |
| Gehäuse-/Umgebungstemperatur | SHT4x/BME280/TMP117-Klasse | I²C | Idee/geplant |
| Luftfeuchte | SHT4x/BME280-Klasse | I²C | Idee/geplant |
| SXceiver-nahe Temperatur | TMP117 oder ähnlicher präziser Sensor | I²C | Idee/geplant |
| PA-/Kühlkörpertemperatur | digitaler Temperatursensor | I²C/1-Wire | Idee/geplant |
| Eingangsspannung/Strom/Leistung | INA226/INA260-Klasse | I²C | Idee/geplant |
| langsame analoge Messwerte | ADS1115-Klasse | I²C | Idee/geplant |
| Lüfterdrehzahl | Tacholeitung | direkter GPIO | Idee/geplant |
| Gehäuse-/Türkontakt | Reed/Mikroschalter | GPIO/Expander | Idee/geplant |
| externe Alarmkontakte | optisch/galvanisch aufbereitete Eingänge | GPIO/Expander | Idee/geplant |

Der Raspberry Pi besitzt keinen allgemeinen analogen GPIO-Eingang. Für Spannungen und Detektorsignale ist daher ein externer ADC erforderlich.

### 6.2 Beschlossen/geplant: I²C als modularer Management-Bus

Der bevorzugte modulare Ansatz ist ein I²C-Backbone, sofern GPIO2/3 am realen Aufbau bestätigt frei sind.

~~~text
Raspberry Pi
   │
   ├── SXceiver
   │    ├── SPI0
   │    ├── I²S
   │    ├── Reset
   │    └── TX/RX-Control
   │
   └── I²C Management Bus
        ├── Temperatur/Feuchte
        ├── Strom-/Spannungsmessung
        ├── ADC
        ├── optional MCP23017
        └── optionale weitere Sensoren
~~~

Vorteil: Viele langsame Managementfunktionen teilen sich nur zwei Pi-Leitungen. Direkte native GPIO bleiben für Funktionen sinnvoll, die Interrupts, Tacho, PWM, Watchdog-Heartbeat oder definierte Bootzustände benötigen.

**Zu prüfen:** Modul-Adressen, doppelte Pull-ups, Kabellänge, Buskapazität, EMV und Fehlerverhalten bei einem blockierten I²C-Teilnehmer.

### 6.3 Idee/geplant: Aktoren

Sinnvolle Aktoren beziehungsweise lokale Ausgänge:

- 4-Pin-Lüfterregelung; PWM nicht als Leistungsversorgung missbrauchen, sondern über geeignete Treiberstufe.
- Status-LEDs beziehungsweise Lightpipes.
- optional Summer für lokalen Service-/Fehlerhinweis.
- MOSFET-/Relaisausgänge für spätere externe Funktionen.
- unabhängiger Hardware-Watchdog beziehungsweise Powercontroller erst dann, wenn dafür ein klarer Betriebsbedarf definiert ist.

**Wichtig:** Die aktuelle Architektur von hardware-gateway schaltet laut Repository-Dokumentation in Phase 6 absichtlich noch **keine echten Ausgänge**. Aktorsteuerung ist dort erst nach Hardware-Treiber- und Policy-Abnahme vorgesehen. Eine Ausbauidee ist daher kein Nachweis einer bereits freigeschalteten Aktor-API.

### 6.4 Beschlossen/geplant: passive Schutz- und Serviceelemente mitdenken

Passive Elemente gehören ausdrücklich zum Ausbauumfang. Relevante Bausteine sind:

- externe Pull-ups/Pull-downs für definierte Boot-/Fehlerzustände;
- Serienwiderstände an exponierten digitalen Leitungen;
- RC-Filter/Entprellung bei mechanischen Kontakten;
- TVS-/ESD-Schutz an extern zugänglichen Leitungen;
- Abblock-/Stützkondensatoren nahe Modulen;
- Sicherung/PTC in geeigneten Niederspannungszweigen;
- Mess- und Testpunkte;
- saubere Masseführung und Trennung störender Lastpfade;
- bei 12/24-V-Feldsignalen geeignete Pegelanpassung/Optokoppler statt direkter Pi-Anbindung.

230-V-Komponenten wurden nur als allgemeines Projektziel erwähnt. Sie gehören **nicht** auf eine improvisierte GPIO-Modulplatte und erfordern eine gesondert geprüfte elektrische/mechanische Auslegung.

## 7. LCD/TFT als lokales TBS-Statusdisplay

### 7.1 Idee/geplant

Ein kleines lokales Display wurde ausdrücklich als sinnvolle Erweiterung eingebracht.

Diskutierte Größenordnung:

- etwa 2,4–3,5 Zoll;
- typischerweise 320×240 oder 480×320;
- Anzeige von Stationsname, Core-/Netzstatus, Frequenzen, Carrier/Timeslot-Status, Temperatur, Versorgung, Lüfter und später RF-Telemetrie/Alarme.

Mögliche Seiten:

1. Übersicht/Health
2. RF
3. Netzwerk/Core
4. Sensorik
5. Alarme
6. System/Service

### 7.2 Nicht als endgültig beschlossen: TFT direkt am SXceiver-SPI0

SPI0 wird vom SXceiver als /dev/spidev0.0 verwendet. Ein Display sollte deshalb **nicht ungeprüft** einfach auf denselben Bus gehängt werden. Selbst mit separatem Chip-Select sind Timing, Treiberverhalten, Buslast und EMV zu prüfen.

Für den Prototyp wurden als sauberere Wege bewertet:

- kleines HDMI-/DSI-/USB-Display, wenn mechanisch passend;
- separates SPI-Interface nur, wenn es auf dem verwendeten Pi unabhängig und stabil verfügbar ist;
- kleines I²C/OLED für reine Statusdaten, wenn die geringere Grafikleistung genügt.

Der entscheidende Gedanke ist: Die lokale Anzeige soll den RF-Pfad möglichst wenig beeinflussen.

### 7.3 Frühere MCU-Idee, später relativiert

Zwischenzeitlich wurde vorgeschlagen, Display, Sensorik, Lüfter und Watchdog über einen kleinen separaten Mikrocontroller zu führen. Das hätte den Vorteil, dass Hardwarezustand und Display auch bei abgestürztem Linux weiterlaufen.

Diese Richtung bleibt technisch attraktiv, wurde aber durch die spätere Festlegung **nicht als kurzfristiger Standardweg übernommen**, weil sie schnell in eine eigene Controller-/PCB-Entwicklung führt.

## 8. RF-Telemetrie: besonders wertvoller Ausbaupfad

Als besonders sinnvoll wurde eine echte physische RF-Telemetrie identifiziert, weil reine DSP-Werte vor dem Leistungsverstärker nicht dieselbe Aussage haben wie Messungen im realen Antennenpfad.

Geplante Messgrößen:

- Forward Power
- Reflected Power
- daraus Return Loss/VSWR
- PA-Temperatur
- Versorgungsspannung/-strom
- Lüfterzustand
- Gehäuse-/Umgebungstemperatur

Für Forward/Reflected Power ist ein Richtkoppler/Detektor mit geeigneter ADC-Anbindung erforderlich; der Pi selbst kann das analoge Detektorsignal nicht direkt messen.

### 8.1 Abgleich mit aktuellem Repository

Der aktuelle rf-monitor passt bereits sehr gut zu dieser Idee:

- Dienst: netcore-rf-monitor
- Port: 8260
- Architektur dokumentiert optionales externes Probe-Kommando für Richtkoppler/ADC/PA/Relaiskontakte.
- der Dienst kann VSWR/Return Loss ableiten, Alarmtransitionen/Heartbeat verwalten und Zustände per WebUI/API/Prometheus/MQTT bereitstellen.
- die Repository-Dokumentation weist ausdrücklich darauf hin, dass TBS-DSP-Werte **vor** dem PA liegen und reale Forward-/Reflected-/Antennenmessungen eine kalibrierte externe Messquelle benötigen.

Damit ist die Sensor-/RF-Idee dieser Planung **architektonisch bereits anschlussfähig**, aber die konkrete physische Sensorhardware aus diesem Arbeitsstand ist nicht implementiert.

## 9. Abgleich mit hardware-gateway

Der aktuelle Hauptbranch enthält:

- system-backend/hardware-gateway/
- Standardport 8250
- Konfiguration /etc/netcore/hardware-gateway.toml
- persistenter Zustand /var/lib/netcore-hardware-gateway/state.json
- Events /var/lib/netcore-hardware-gateway/events.ndjson

Die Architektur beschreibt:

~~~text
Sensoren / Kontakte / Edge-I/O
        │ MQTT oder HTTP
        ▼
NetCore Hardware Gateway
        ├── Geräte-Heartbeat
        ├── Threshold-/Alarmbewertung
        ├── persistenter Zustand
        ├── WebUI/API
        └── MQTT State + netcore-event-v1
                 │
                 ▼
         IoT Gateway / Home Assistant
~~~

Das ist ein geeigneter Integrationspunkt für Temperatur, Gehäusekontakte, Versorgung, Lüfterzustand und sonstige Rack-/Edge-I/O.

**Aktueller Repository-Hinweis:** Phase 6 schaltet absichtlich keine echten Ausgänge; Aktorsteuerung ist erst nach separater Treiber-/Policy-Abnahme vorgesehen.

## 10. Wichtigste endgültige Entscheidung: keine eigene PCB als Voraussetzung

Eine vollständige Eigen-PCB würde den kurzfristigen Prototyp unnötig aufwendig machen. Vorrang hat daher ein modularer Aufbau mit Fertigmodulen.

Daraufhin wurde der Architekturvorschlag bewusst vereinfacht.

### Beschlossen/geplant für den Prototyp

**Keine eigene NetCore-TBS-Management-PCB als Voraussetzung.**

Stattdessen:

- vorhandenes GPIO-Pass-Through-/Breakout weiterverwenden;
- fertige I²C-Sensormodule;
- fertiges INA-/ADC-Modul;
- optional fertiger MCP23017;
- fertige MOSFET-/Lüfter-/Relaismodule;
- fertiges Displaymodul;
- mechanische Montage auf Platte/DIN-Schiene beziehungsweise im Gehäuse;
- klare Steckverbinder statt sofortiger Integrationsplatine.

### Spätere PCB nur bei echtem Nutzen

Eine eigene Platine wird erst wieder attraktiv, wenn:

1. der modulare Aufbau funktional stabil ist;
2. reale Pinbelegung, Strompfade, EMV und thermisches Verhalten bekannt sind;
3. sich wiederkehrende identische TBS-Hardware abzeichnet;
4. Modulzahl/Verdrahtungsaufwand den Integrationsaufwand rechtfertigen.

Damit gilt eine frühere, früh skizzierte „NetCore TBS Management Controller Board“-PCB als **nicht verworfenes Langfristziel, aber ausdrücklich nicht als nächster Schritt**.

## 11. Erreichter Entwicklungs- und Betriebsstand

| Punkt | Status | Nachweis |
| --- | --- | --- |
| Reale SXceiver-Beobachtung am Breakout | beobachtet, aber nicht elektrisch verifiziert | Praxisangaben zu sichtbaren Pins/LEDs |
| GPIO25 als Reset | **überholt/falsch für aktuellen Repo-Stand** | geprüfte SoapySX-Prüfung zeigt GPIO5 |
| GPIO5 als SX reset | implementiert im aktuellen Code | SoapySX.cpp |
| GPIO22/23 als TX/RX bei aktuellem HAT-Default | implementiert im aktuellen Code/HAT-Setup | SoapySX.cpp, dts/Makefile |
| SPI0 + I²S für SXceiver | implementiert | Device Tree/SoapySX |
| vollständige reale Pinmatrix | noch offen | keine Durchgangs-/Logic-Analyzer-Abnahme |
| I²C-Sensorbus | beschlossen/geplant | Ausbauplanung |
| Temperatur-/Feuchte-/Power-Sensoren | Idee/geplant | Ausbauplanung |
| ADS1115-/ADC-Schicht | Idee/geplant | Ausbauplanung |
| Lüfterregelung/Tacho | Idee/geplant | Ausbauplanung |
| TFT/LCD | Idee/geplant | Ausbauplanung |
| externe RF-Power-/VSWR-Messung | Idee/geplant; Softwareziel bereits vorhanden | Ausbauidee + rf-monitor |
| hardware-gateway | implementiert im Repository | main |
| rf-monitor | implementiert im Repository | main |
| konkrete geplante Sensorhardware | nicht implementiert | kein entsprechender Treiber-/BOM-Commit |
| eigene Management-PCB | **vorerst nicht gewünscht** | spätere Festlegung |
| modularer Aufbau mit Fertigmodulen | beschlossen/geplant | finaler Arbeitsstand |
| elektrische/thermische/EMV-Abnahme | nicht getestet | kein Messprotokoll |
| On-Air-Einfluss der Zusatzhardware | nicht getestet | kein RF-Test |

## 12. Relevante Dateien, Dienste, Ports, Pfade und technische Parameter

| Element | Relevanz |
| --- | --- |
| sxxcvr-main/SoapySX/SoapySX.cpp | reale Reset-/TX-/RX-Control-GPIO, SPI0-Device |
| sxxcvr-main/dts/Makefile | HAT-Version und versionsabhängige Control-Pins |
| sxxcvr-main/dts/sx1255_raspberrypi.dts | aktiviert I²S und SPI0 |
| sxxcvr-main/dts/README.md | HAT-Versionen 1.0/1.1/1.2 und EEPROM-Ablauf |
| wiki/Hardware-und-RF.md | warnt vor freier GPIO-Planung ohne HAT-Pinout-Abgleich; nennt LCD/TFT, Watchdog, Temperatur/Spannung als Ausbauziele |
| wiki/Roadmap.md | GPIO-/Rack-Platine als Ausbauziel, aber nicht als fertig |
| system-backend/hardware-gateway/ | Edge-I/O/Rack-Sensorik |
| system-backend/rf-monitor/ | RF-/PA-/Antennen-Telemetrie |
| Hardware Gateway Port | 8250/TCP |
| RF Monitor Port | 8260/TCP |
| Hardware Gateway Config | /etc/netcore/hardware-gateway.toml |
| RF Monitor Config | /etc/netcore/rf-monitor.toml |
| Hardware Gateway State | /var/lib/netcore-hardware-gateway/state.json |
| Hardware Gateway Events | /var/lib/netcore-hardware-gateway/events.ndjson |
| RF Monitor State | /var/lib/netcore-rf-monitor/state.json |
| RF Monitor Events | /var/lib/netcore-rf-monitor/events.ndjson |
| SXceiver SPI | /dev/spidev0.0 |
| SX GPIO chip | /dev/gpiochip0 |

## 13. Wichtige Befehle und Abläufe

### 13.1 Noch nicht praktisch erprobt

Noch nicht praktisch erprobt sind Sensorinstallation, TFT-Anschluss, GPIO-Schaltung und Hardwaretreiber-Deployment. Ein erfolgreicher Installationsablauf für die neue Sensor-/Displayhardware fehlt.

### 13.2 Repository-seitig vorhandener HAT-/EEPROM-Ablauf

dts/README.md dokumentiert für den SXceiver-HAT unter anderem:

~~~bash
sudo apt-get install --no-install-recommends git make gcc device-tree-compiler alsa-utils
cd
git clone "https://github.com/raspberrypi/hats.git"
cd hats/eepromutils
make
sudo make install
~~~

Danach im dts-Verzeichnis:

~~~bash
make clean
make HAT_VERSION=0x0102
sudo make write_eeprom
sudo reboot
~~~

Prüfung:

~~~bash
ls -l /proc/device-tree/hat
aplay -L | grep SX1255
arecord -L | grep SX1255
~~~

Diese Befehle sind **Repository-Dokumentation**, ohne dokumentierte erfolgreiche Ausführung.

## 14. Fehler, Diagnose, Ursachen und Korrekturen

### 14.1 Zu starker Fokus auf Jumper

**Problem:** Die erste Beratung verwendete viele freie GPIO gedanklich für Betriebsarten/Jumper.

**Korrektur der Anforderung:** Sensorik, Aktoren und passive Elemente sind mindestens ebenso wichtig beziehungsweise sinnvoller.

**Finale Folge:** GPIO-Knappheit nicht durch viele Modusleitungen erzeugen; langsame Hardwarefunktionen bündeln, insbesondere über I²C.

### 14.2 Falscher Reset-Pin aus generischer SX1255-Referenz

**Problem:** GPIO25 wurde zunächst als SX1255-Reset angenommen.

**Ursache:** Übertragung einer generischen M17/SX1255-Referenz auf den konkreten SXceiver.

**Korrektur:** aktueller NetCore-/SoapySX-Code verwendet **GPIO5** als Reset. GPIO25 ist damit im aktuellen Softwarestand nicht der SXceiver-Reset.

### 14.3 „Komplette Management-PCB“ als zu großer Sprung

**Problem:** Aus Sensorik + TFT + Watchdog entstand schnell die Idee einer eigenen Controllerplatine.

**Korrektur:** Eine komplette PCB-Entwicklung soll für den aktuellen Prototyp vermieden werden.

**Finale Lösung:** Fertigmodule + Pass-Through-Breakout + saubere modulare Verdrahtung; eigene PCB erst später bei bewiesenem Nutzen.

### 14.4 TFT auf demselben SPI wie RF-Hardware

**Risiko:** Der SXceiver benutzt SPI0 bereits aktiv. Ein TFT erzeugt zusätzlich Bus- und EMV-Last.

**Lösungsidee:** Display zunächst unabhängig anbinden oder separaten Bus/Interface wählen; keine ungeprüfte gemeinsame Nutzung.

## 15. Tests und Ergebnisse

### Historische Beobachtungen

- Praxisbeobachtung am realen Board/Breakout wurde aufgenommen.
- Es erfolgte **keine** elektrische Freimessung der Pins.
- Es erfolgte **kein** GPIO-Readback-Test.
- Es erfolgte **kein** I²C-Scan mit angeschlossenen neuen Modulen.
- Es erfolgte **kein** Lüfter-/PWM-Test.
- Es erfolgte **kein** Displaytest.
- Es erfolgte **kein** RF-Stör-/Desensibilisierungstest mit Zusatzhardware.

### Zusätzlich bei der Bestandsaufnahme statisch geprüft

Am 2026-10-05 gegen main@9116c15d645458f99e236712b67a1ad970432791:

1. SoapySX.cpp – GPIO5 Reset, GPIO22/23 beziehungsweise GPIO12/13 für TX/RX-Control.
2. dts/Makefile – Default 0x0102, versionsabhängige Control-Pins.
3. sx1255_raspberrypi.dts – I²S und SPI0 aktiv.
4. wiki/Hardware-und-RF.md – GPIO-Pinprüfung sowie LCD/TFT, Watchdog, Temperatur-/Spannungsüberwachung als Ausbauziele.
5. wiki/Roadmap.md – GPIO-/Rack-Platine weiterhin nur Ausbauziel.
6. hardware-gateway – Sensor-/Kontakt-/Edge-I/O-Architektur vorhanden, echte Ausgänge noch absichtlich nicht freigegeben.
7. rf-monitor – externe Probe für Richtkoppler/ADC/PA/Relaiskontakte ausdrücklich vorgesehen.

**Grenze:** Diese Repository-Prüfung bestätigt Software-/Dokumentationsstand, nicht die reale elektrische Ausführung des SXceiver-HATs.

## 16. Verworfene oder ersetzte Ansätze

### Ersetzt: GPIO25 als SXceiver-Reset
Durch aktuellen Repository-Code widerlegt; korrekt ist GPIO5.

### Nachrangig: viele Jumper/Betriebsmodi auf direkten GPIO
Nicht grundsätzlich verboten, aber nicht mehr Schwerpunkt. Native GPIO sollen nicht unnötig durch Konfigurationsschalter verbraucht werden.

### Nachrangig: sofortiger separater MCU-Supervisor
Technisch attraktiv, aber im Prototyp nicht erforderlich. Wieder aufgreifen, wenn Linux-unabhängiger Watchdog/Power-Cycle/Display zwingend wird.

### Vorläufig verworfen: komplette eigene Management-PCB als nächster Schritt
Zu hoher Entwicklungsaufwand für den aktuellen Nutzen. Fertigmodule sind die bevorzugte Prototypstrategie.

### Nicht empfohlen: ungeprüftes SPI-TFT am SXceiver-SPI0
Erst nach klarer Bus-/Treiber-/EMV-Prüfung denkbar.

## 17. Bilder und ergänzende Unterlagen

Zum SXceiver-/GPIO-Aufbau ist ein Bild referenziert, dessen ursprüngliche Binärdatei nicht verfügbar ist. Die Pin-/LED-Beobachtung bleibt deshalb ein Praxisbericht, kein erneut visuell oder elektrisch geprüfter Nachweis. Bilder verwandter Archive sind nur ergänzende Referenzen und ersetzen das Original nicht.

Verwandte Bild- und Hardwareunterlagen finden sich in den unten verlinkten GPIO-, Baseboard- und SXceiver-Notizen.

## 18. Roadmap-Kandidaten aus diesem Arbeitsstand

Diese Punkte sind Kandidaten für spätere Roadmap-Arbeit. Eine Umsetzung außerhalb der Notizen ist daraus noch nicht entstanden.

### P0 – Pinbelegung sauber abschließen

1. reale SXceiver-HAT-Version auf der TBS eindeutig bestimmen;
2. 40-Pin-Matrix gegen SoapySX.cpp, HAT-EEPROM und reales Board abgleichen;
3. Reset GPIO5 sowie aktuelle TX/RX-Control-Pins verifizieren;
4. SPI0 und I²S als reserviert markieren;
5. GPIO2/3 als I²C-Kandidaten elektrisch bestätigen;
6. nur danach freie native GPIO verbindlich vergeben.

### P1 – minimaler modularer Hardware-Prototyp

1. Temperatur-/Feuchte-Sensor per I²C;
2. Spannungs-/Strommessung per INA226/INA260-Klasse;
3. externer ADC für langsame Analogwerte;
4. ein Lüfter mit sauberer Treiberstufe plus Tacho;
5. kleines lokales Display mit vom SXceiver möglichst unabhängiger Anbindung;
6. Hardware in hardware-gateway sichtbar machen.

### P1 – RF-Monitoring anbinden

1. Richtkoppler/Detektor auswählen;
2. Forward/Reflected analog messen;
3. ADC kalibrieren;
4. Werte an vorhandenen rf-monitor liefern;
5. VSWR/Return-Loss und Grenzwerte gegen Messgerät abgleichen;
6. Alarmübergänge/Heartbeat testen.

### P2 – passive Robustheit und Feld-I/O

- ESD/TVS;
- definierte Pull-Zustände;
- RC-Entprellung;
- Sicherung/PTC;
- Testpunkte;
- optisch/galvanisch aufbereitete externe Kontakte;
- EMV-gerechte Leitungsführung und mechanische Steckverbinder.

### P3 – eigene Integrationsplatine nur bei Bedarf

Erst nach einem stabilen modularen Aufbau prüfen, ob eine NetCore-TBS-Hardwareplatine Zeit und Fehlerquellen spart. Dann nicht bei Null anfangen, sondern die bewährten Modulblöcke integrieren.

## 19. Konkrete nächste Schritte

Der sinnvollste nächste reale Arbeitsschritt ist bewusst klein:

1. HAT-Version der realen SXceiver-Platine bestätigen.
2. Pinmatrix auf Basis von GPIO5 + SPI0 + I²S + versionsabhängig 22/23 beziehungsweise 12/13 finalisieren.
3. GPIO2/3 auf dem realen Stack als I²C prüfen.
4. **Einen** I²C-Temperatur-/Feuchte-Sensor anschließen und auslesen.
5. Messwert zunächst lokal loggen.
6. Danach Anbindung an hardware-gateway planen.
7. Erst anschließend Strommessung/ADC und Display ergänzen.
8. RF-Forward/Reflected als eigener Messpfad an rf-monitor anbinden.
9. Vor jedem dauerhaften Rackeinbau Stör-/Temperatur-/Boot-/Recovery-Test durchführen.

Damit entsteht Fortschritt ohne den Zwang, zuerst eine komplette Platine zu designen.

## 20. Relevante Quellen und Repository-Bezüge

Geprüft am 2026-10-05:

- main@9116c15d645458f99e236712b67a1ad970432791
- Archiving@ded2c14a713f65196e3f28a4640b4e317f84b59e vor der ursprünglichen Dokumentation
- sxxcvr-main/SoapySX/SoapySX.cpp
- sxxcvr-main/dts/Makefile
- sxxcvr-main/dts/sx1255_raspberrypi.dts
- sxxcvr-main/dts/README.md
- wiki/Hardware-und-RF.md
- wiki/Roadmap.md
- system-backend/hardware-gateway/docs/architecture.md
- system-backend/hardware-gateway/config/hardware-gateway.example.toml
- system-backend/rf-monitor/docs/architecture.md
- system-backend/rf-monitor/config/rf-monitor.example.toml
- wiki/Dienstkatalog.md
- verwandtes Archiv: Docs/archive/2026-10-05_basisstation-gpio-breakout-jumper-steuerung-sxceiver.md

Es wurde kein PR und kein Commit als Nachweis dafür gefunden oder behauptet, dass die hier diskutierten neuen Sensor-, Lüfter- oder TFT-Module bereits auf einer TBS installiert seien.

## 21. Offene Nachweise

- Die Originaldatei des referenzierten SXceiver-/GPIO-Bildes fehlt. Ein erneut visuell geprüfter oder bitidentisch gesicherter Bildnachweis liegt deshalb nicht vor.
- Die konkrete reale HAT-Version wurde in den Entwicklungsnotizen nicht abschließend bestätigt; das Repository verwendet 0x0102 als Default.
- Die Pinbelegung ist statisch aus Code/Device-Tree abgeleitet, aber nicht mit Durchgangsmessung oder Logic Analyzer am realen Board bestätigt.
- Keine neue Sensorik, kein Display und kein Aktor aus diesem Arbeitsstand wurde praktisch getestet.
- Keine EMV-, Temperatur-, Dauerlast- oder On-Air-Abnahme der geplanten Zusatzhardware liegt vor.
