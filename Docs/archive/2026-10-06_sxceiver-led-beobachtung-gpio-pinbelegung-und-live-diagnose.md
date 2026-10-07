# Brainstorming: NetCore-Tetra – SXceiver: LED-Beobachtung, GPIO-Pinbelegung und ausstehende Live-Diagnose

Stand der Repository- und Quellenprüfung: **06.10.2026**. Historische Ergebnisse beziehen sich auf die jeweils genannten Daten und Commits.

## 1. Projektstand und Geltungsbereich

| Feld | Wert |
| --- | --- |
| Thema | Zuordnung der am SXceiver-/GPIO-Aufbau leuchtenden LEDs; Unterschied zwischen Pegelanzeige und tatsächlicher Pinbelegung; Abgleich mit SoapySX und Vorbereitung der Live-Diagnose |
| Sichtbarer fachlicher Verlauf | 03.10.2026; Prüfdurchlauf vom 06.10.2026 |
| Erstellungsdatum dieser Zusammenfassung | **2026-10-06**, Europe/Berlin |
| Zielrepository | [JanHG98/netcore-tetra](https://github.com/JanHG98/netcore-tetra) |
| Ablagebranch | **Archiving** |
| Archivpfad | **Docs/archive/** |
| Geprüfter Archivstand bei Beginn | `000fad98419f6db1d8d3815f8c7c916375b02a41` |
| Zusätzlich geprüfter Produktstand | `main@9116c15d645458f99e236712b67a1ad970432791` |
| Geprüfter SXceiver-Upstream | `tejeez/sxxcvr main@9705147dd8c189625071f3f163ea56119bda4a05` |
| Nachweisumfang | Beobachtung am Aufbau, historischer Beratungsstand und geprüfte statische Quellprüfung; **keine neue Messung oder Abnahme am Raspberry Pi** |

### Abgrenzung zu bereits vorhandenen Archiven

Die [Dokumentation zu SXceiver-GPIO, Sensorik, TFT und modularem Hardwareausbau](2026-10-05_sxceiver-gpio-sensorik-tft-und-modularer-hardwareausbau.md) enthält dieselbe LED-Liste und einen weitergehenden Hardware-Entwurf. Die vorliegenden Notizen konzentrieren sich auf die unmittelbare Beobachtung, Pinzuordnung und ausstehende Live-Diagnose.

Der weitergehende Hardwareentwurf bleibt als eigener Planungsstand erhalten. Seine zusätzlichen Festlegungen werden nicht ohne Beleg auf die unmittelbar beobachtete Pinbelegung übertragen.

Ebenfalls verwandt ist das [Archiv zum GPIO-Breakout und zur Jumpersteuerung](2026-10-05_basisstation-gpio-breakout-jumper-steuerung-sxceiver.md). Beide älteren Zusammenfassungen sind Sekundärquellen, keine Originalfotos und keine Messprotokolle.

### Verbindliche Statusbegriffe

| Status | Bedeutung in dieser Dokumentation |
| --- | --- |
| **Idee** | Vorgeschlagene Möglichkeit ohne nachgewiesene Umsetzung |
| **Beschlossen/geplant** | Vereinbarter oder als nächster Schritt vorgeschlagener Arbeitsumfang; die Urheberschaft wird angegeben |
| **Implementiert** | In konkret benanntem Repository-Code vorhanden |
| **Getestet** | Durch einen beschriebenen Test mit Ergebnis nachgewiesen; statische Prüfung und Hardwaretest werden getrennt |
| **Im Betrieb bestätigt** | Aussage über den realen Aufbau mit ausdrücklich benanntem Nachweis; eine LED-Beobachtung bestätigt nur die Beobachtung selbst |

## 2. Ziel, Ausgangslage und behandelte Themen

Beobachtet wurden leuchtende LEDs an den unten aufgeführten beschrifteten Anschlüssen des SXceiver-/GPIO-Aufbaus. Die Frage war, welche Anschlüsse der SXceiver tatsächlich verwendet. LED-Leuchten allein belegt die Pinbelegung nicht.

Der konkrete Anlass war die weitere Pinplanung für eine NetCore-Tetra-Basisstation. Vor zusätzlichen GPIO-Funktionen muss zwischen Versorgung, Busleitungen, dedizierten SXceiver-Steuerleitungen, durch das Betriebssystem reservierten Anschlüssen und nur möglicherweise freien Pins unterschieden werden.

Behandelt wurden:

- Übertragung der Boardbeschriftungen auf BCM-GPIO-Nummern und physische Positionen des 40-Pin-Headers.
- Aussagegrenzen einer LED-Anzeige bei Pull-ups, Low-Pegeln und seriellen Signalen.
- Tatsächlicher Standard-SXceiver-Datenpfad über SPI0 und I²S.
- Reset sowie versionsabhängige RX-/TX-Steuerleitungen.
- Fehlende, aber trotzdem benötigte Pins in der LED-Liste.
- Lesende Diagnose mit `pinctrl` und `gpioinfo`.
- Abgrenzung einer aus Quellen abgeleiteten Arbeitsmatrix von einer elektrisch freigegebenen Pinliste.

Nicht Gegenstand einer nachgewiesenen Umsetzung waren ein neues PCB, eine Sensorinstallation, eine Displayanbindung, ein GPIO-Treiberumbau oder ein Deployment.

## 3. Bisherige Beobachtung und Diagnose

### 3.1 Unmittelbare Beobachtung am Aufbau

Die Liste behält die Reihenfolge der Beobachtung bei. „Links“ und „rechts“ beziehen sich auf die Ansicht des Boards. Sie bildet keine vollständige Geometrie gegenüberliegender Headerkontakte ab.

**Links:**

~~~text
+3V3
SDA
SCL1
IO4
IO22
+3V3
IO6
IO19
~~~

**Rechts:**

~~~text
+5V
+5V
IO23
IO8
IO7
IO20
IO21
~~~

Nachweisstufe: **im realen Aufbau am Aufbau beobachtetes Leuchten**. Nicht mitgeliefert wurden Spannungsmesswerte, Strommesswerte, Oszillogramme, ein Schaltplan des LED-Breakouts, LED-Helligkeiten oder eine vollständige Auflistung der dunklen LEDs.

### 3.2 Reaktion und fachliche Korrektur

Fachliche Einordnung:

1. Eine leuchtende LED ist kein hinreichender Beleg, dass der SXceiver diesen GPIO verwendet.
2. Eine dunkle LED ist kein hinreichender Beleg, dass ein Anschluss frei ist.
3. SDA/SCL können durch Pull-ups High sein, ohne dass gerade Daten übertragen werden.
4. Versorgungsspannung an einem Pin ist kein Nachweis des Stromverbrauchs des SXceivers an diesem Pin.
5. Der offizielle Treiber benötigt weitere Anschlüsse, die in der beobachteten Liste fehlen.

Die Standardbelegung wurde für Hardwareversion 1.2 eingeordnet. Als lesende Diagnose sind zwei Befehle vorgeschlagen:

~~~bash
sudo pinctrl get
sudo gpioinfo
~~~

### 3.3 Endpunkt des zugänglichen Verlaufs

Ausgaben dieser Befehle liegen **nicht vor**. Es gibt daher keine bestätigte Auflösung, welche GPIO auf dem konkreten Pi durch den Kernel, SoapySX, andere Overlays oder weitere Software beansprucht werden.

Die im Projektkontext genannte SXceiver-Version 1.2 war die Arbeitsannahme. Eine neu ausgelesene HAT-Kennung oder fotografisch belegte Boardrevision liegt in diesem sichtbaren Verlauf nicht vor.

## 4. Zuordnung der beobachteten Anschlüsse

Die folgende Tabelle verwendet **BCM-GPIO-Nummern**. Die Spalte „physischer Pin“ bezeichnet die Position am üblichen Raspberry-Pi-40-Pin-Header, nicht eine weitere GPIO-Nummer. Die geometrische Übertragung auf das konkrete Breakout bleibt mangels Originalbild zu kontrollieren.

| Beschriftung aus der Beobachtung | BCM | Physischer Pin | Einordnung für den Standard-SXceiver |
| --- | ---: | ---: | --- |
| +3V3, oberer Eintrag links | – | 1 | 3,3-V-Versorgung; LED zeigt keine exklusive Nutzung durch SXceiver |
| SDA | 2 | 3 | I²C1 SDA; High kann durch Pull-up entstehen |
| SCL1 | 3 | 5 | I²C1 SCL; High kann durch Pull-up entstehen |
| IO4 | 4 | 7 | Im geprüften SoapySX-Code keine direkte Reset-/RX-/TX-Steuerfunktion |
| IO22 | 22 | 15 | TX-Control für HW 1.1/1.2 bzw. den Nicht-1.0-Zweig |
| +3V3, zweiter Eintrag links | – | 17 | Zweiter 3,3-V-Versorgungsanschluss |
| IO6 | 6 | 31 | Im geprüften SoapySX-Code keine direkte Reset-/RX-/TX-Steuerfunktion |
| IO19 | 19 | 35 | I²S/PCM Frame Sync |
| +5V, erster Eintrag rechts | – | 2 | 5-V-Versorgung |
| +5V, zweiter Eintrag rechts | – | 4 | Zweiter 5-V-Versorgungsanschluss |
| IO23 | 23 | 16 | RX-Control für HW 1.1/1.2 bzw. den Nicht-1.0-Zweig |
| IO8 | 8 | 24 | SPI0 CE0; Auswahl des SXceiver-SPI-Geräts |
| IO7 | 7 | 26 | SPI0 CE1; vom geprüften SXceiver-Treiber nicht als dessen Chip Select geöffnet; eine Kernel-/Overlay-Reservierung bleibt möglich |
| IO20 | 20 | 38 | I²S/PCM Dateneingang des Pi |
| IO21 | 21 | 40 | I²S/PCM Datenausgang des Pi |

**Keine Freigabe aus Negativbefunden:** Dass GPIO4, GPIO6 oder GPIO2/3 nicht als direkte Steuerleitungen im SoapySX-Code vorkommen, beweist weder fehlende Leiterbahnverbindungen auf dem HAT noch Abwesenheit anderer Verbraucher.

Die frühere Kurzform „LED zeigt Pegel“ muss zudem auf das unbekannte Anzeigeboard eingeschränkt werden: Ohne dessen Schaltung sind Polarität, Pufferung und Belastung nicht verifiziert. Bei schnellen Signalen kann die sichtbare Helligkeit zeitlich gemitteltes Verhalten wiedergeben. Ein konkreter High-/Low-Messwert wurde nicht erfasst.

## 5. Vollständiger relevanter SXceiver-Signalumfang

### 5.1 Arbeitsmatrix für HW 1.2

| Funktion | BCM-GPIO | Physischer Pin | In der Beobachtungsliste enthalten? | Konsequenz für Erweiterungen |
| --- | ---: | ---: | --- | --- |
| SX Reset | 5 | 29 | Nein | Reservieren |
| SPI0 CE0 | 8 | 24 | Ja | Reservieren |
| SPI0 MISO | 9 | 21 | Nein | Reservieren |
| SPI0 MOSI | 10 | 19 | Nein | Reservieren |
| SPI0 SCLK | 11 | 23 | Nein | Reservieren |
| I²S/PCM Bittakt | 18 | 12 | Nein | Reservieren |
| I²S/PCM Frame Sync | 19 | 35 | Ja | Reservieren |
| I²S/PCM DIN | 20 | 38 | Ja | Reservieren |
| I²S/PCM DOUT | 21 | 40 | Ja | Reservieren |
| SX TX-Control | 22 | 15 | Ja | Reservieren |
| SX RX-Control | 23 | 16 | Ja | Reservieren |

Insbesondere **GPIO5, GPIO9, GPIO10, GPIO11 und GPIO18** fehlen in der LED-Liste, gehören aber zum Standardpfad. Damit widerlegt die Codeprüfung die Verwendung der LED-Liste als vollständiges Belegungsinventar.

Versorgung und Masse sind gesondert zu erfassen. Die HAT-ID-Anschlüsse GPIO0/1 an den physischen Pins 27/28 sind außerdem nicht mit GPIO2/3 an Pins 3/5 zu verwechseln. Der konkrete EEPROM-/HAT-Anschluss gehört in die spätere vollständige 40-Pin-Prüfung; diese Diagnose hat keine freie Nutzung der ID-Pins festgestellt.

### 5.2 Versionsabhängigkeit der Steuerleitungen

| Erkannte bzw. konfigurierte HAT-Version | TX-Control | RX-Control | Reset |
| --- | --- | --- | --- |
| 1.0 / `0x0100` | BCM12 / Pin 32 | BCM13 / Pin 33 | BCM5 / Pin 29 |
| 1.1 / `0x0101` | BCM22 / Pin 15 | BCM23 / Pin 16 | BCM5 / Pin 29 |
| 1.2 / `0x0102` | BCM22 / Pin 15 | BCM23 / Pin 16 | BCM5 / Pin 29 |

Der Code prüft ausdrücklich auf `0x0100`; alle anderen Werte gelangen in den Zweig für 22/23. Das ist eine Beschreibung der implementierten Bedingung, **keine automatische Kompatibilitätszusage für unbekannte künftige Hardwareversionen**.

### 5.3 Bei der geprüften Prüfung präzisiert: zwei verschiedene Versionsdefaults

Es sind zwei unterschiedliche Defaults zu unterscheiden:

- **Laufender SoapySX-Treiber:** `read_hat_info()` setzt bei nicht lesbarer HAT-Kennung die Annahme **1.1 / 0x0101**.
- **EEPROM-Makefile:** Wenn keine Version vorliegt bzw. vorgegeben ist, wird **1.2 / 0x0102** verwendet.

Beide führen in diesen Quellen zu RX23/TX22, ersetzen aber nicht die Feststellung der realen Boardrevision. Die verkürzte Aussage „Default ist 1.2“ ist nur für das Makefile korrekt.

### 5.4 Bei der geprüften Prüfung präzisiert: RX/TX-Control ist keine einfache Funkaktivitätsanzeige

Der Treiber setzt beim Initialisieren RX-Control und TX-Control auf 1. Für die Einstellung `PA` gilt:

| Modus | TX-Control | RX-Control |
| --- | ---: | ---: |
| `ON` | 1 | 0 |
| `OFF` | 0 | 1 |
| `AUTO` | 1 | 1 |

Daher beweist das Leuchten von IO22 bzw. IO23 **nicht**, dass in diesem Moment entsprechend gesendet oder ein Nutzsignal empfangen wird. Die Treiberstellung und die tatsächliche RF-Aktivität sind unterschiedliche Beobachtungsebenen.

Reset wird als Output mit `GPIO_V2_LINE_FLAG_OPEN_SOURCE` angefordert. `reset()` setzt ihn kurz auf 1 und anschließend auf 0. Der Anschluss bleibt funktional belegt, obwohl ein dauerhafter High-Zustand gerade nicht vorgesehen ist.

## 6. Architektur, Schnittstellen und Abhängigkeiten

Die RF-Anwendung greift über SoapySDR und dessen SX-Treiber auf den SX1255 zu. Die für die Pinfrage relevanten Wege sind:

| Ebene / Komponente | Aufgabe und Schnittstelle |
| --- | --- |
| Raspberry Pi mit 40-Pin-Header | Physische Versorgung, GPIO und alternative Pin-Funktionen |
| SXceiver-HAT / SX1255 | RF-Transceiver; konkrete Revision bestimmt die Control-Pins |
| GPIO-Breakout / LED-Anzeige | Sichtbarer Zwischen- oder Anzeigeaufbau; Hersteller, Schaltung und Belastung in diesem Verlauf nicht bestätigt |
| SoapySDR / SoapySX | Anwendungszugriff und Treibersteuerung |
| SPI0 | Registerzugriff über `/dev/spidev0.0` |
| GPIO-Character-Device | Reset/RX/TX über `/dev/gpiochip0` im geprüften Quellstand |
| I²S / ALSA | Digitaler I/Q-Datenpfad |
| Device-Tree-Overlay | Aktiviert `i2s_clk_consumer`, Soundkarte und `spi0` |
| HAT-Informationen | `/proc/device-tree/hat/product_id` und `product_ver` |

Weitere konkret im Code vorhandene Parameter:

- Erwartete HAT-Produkt-ID: `0x1255`.
- ALSA Capture: `hw:CARD=SX1255,DEV=1`.
- ALSA Playback: `hw:CARD=SX1255,DEV=0`.
- Soundkartenname im Overlay: `SX1255`.
- I²S-Konfiguration: zwei Slots zu je 32 Bit.
- Dummy-Codecs `linux,spdif-dir` und `linux,spdif-dit` dienen der ALSA-Anbindung. Daraus folgt kein zusätzlicher physischer S/PDIF-Port am HAT.
- GPIO-Consumer-Namen: `SX reset`, `SX RX`, `SX TX`.
- Das Makefile beschreibt TX/RX beim HAT-Setup als `OUTPUT DOWN`; das ist nicht mit jedem späteren Laufzeitpegel gleichzusetzen.

Die tatsächliche Zuordnung von `gpiochip0` zum Header muss zum verwendeten Pi und Kernel passen. Aus dem Quellpfad allein wurde keine aktuelle Zuordnung auf Jans Zielgerät nachgewiesen.

Für diese lokale Pinprüfung wurden **keine TCP-/UDP-Ports, Funkfrequenzen, MCC/MNC, ISSI oder neuen systemd-Dienste festgelegt**. Solche Werte aus anderen Projektunterlagen werden hier nicht als Ergebnis dieser Untersuchung übernommen.

## 7. Erreichter Entwicklungs- und Betriebsstand

| Gegenstand | Status | Nachweis und Grenze |
| --- | --- | --- |
| Genannte LEDs leuchten | **Im Betrieb beobachtet** | Betreiberbericht; keine eigene Messung oder Bildprüfung |
| GPIO5 als Reset | **Implementiert, statisch geprüft** | SoapySX-Quellcode; kein aktueller Live-Readback |
| RX/TX 23/22 für HW 1.1/1.2 | **Implementiert, statisch geprüft** | SoapySX plus EEPROM-Makefile |
| SPI0- und I²S-Pfad | **Implementiert, statisch geprüft** | Treiber und Overlay |
| Erklärung der LED-Aussagegrenzen | **Fachliche Einordnung** | Keine elektrische Abnahme des Anzeigeboards |
| `pinctrl`-/`gpioinfo`-Diagnose | **Geplant, als Diagnosevorschlag** | Noch keine Ausgabe im zugänglichen Verlauf |
| Verbindliche Liste freier GPIO | **Offen** | Reale Konfiguration und Verdrahtung fehlen |
| Zusätzliche Sensorik / Anzeige / Aktoren | **Idee bzw. angrenzende Planung** | Keine Umsetzung in diesem sichtbaren Verlauf |
| Neue Hardwaretests oder On-Air-Abnahme | **Nicht getestet** | Kein Pi-/HAT-Zugriff in dieser Archivierung |

Es wurde für diesen Entwicklungsstand kein Firmware-/Treiberfix vorgenommen. Die Archivierung verändert ausschließlich Dokumentation und Index.

## 8. Zusätzlich geprüfter Repository-Stand am 06.10.2026

### 8.1 Reproduzierbare Prüfbasis

Die Dateien wurden aus dem unveränderlichen Archivcommit gelesen. Die Git-Baumdaten des aktuellen `main` und des Upstreams wurden anschließend auf identische Blob-IDs geprüft.

| Datei in NetCore-Tetra | Blob-SHA in Archiving und main sowie im entsprechenden Upstream-Pfad |
| --- | --- |
| `sxxcvr-main/SoapySX/SoapySX.cpp` | `7102014714bab6797a6e2a9c9e6f8bec75d398ac` |
| `sxxcvr-main/dts/Makefile` | `d19151a17302bea4be05c53c1b64f12bb6714231` |
| `sxxcvr-main/dts/sx1255_raspberrypi.dts` | `1775e48b0566cdcee258f63765a5cef42880fb63` |

Die identischen Git-Blobs belegen für **diese drei Dateien** denselben Inhalt in allen drei geprüften Ständen. Daraus wird keine Gleichheit der gesamten Branches, aller Build-Abhängigkeiten oder der installierten Binärdateien abgeleitet.

Die historische Quellenprüfung vom 03.10.2026 hatte bereits den Upstream-Commit `9705147dd8c189625071f3f163ea56119bda4a05` erfasst. Die erneute Prüfung bestätigt denselben Upstream-Stand für die relevanten Dateien.

Wichtige Fundstellen in `SoapySX.cpp` sind `read_hat_info()`, der Konstruktor `SoapySX(...)`, die Klasse `GpioLine`, `reset()` und `writeSetting("PA", ...)`.

### 8.2 Ergebnis des historischen/aktuellen Vergleichs

| Historische Aussage | Geprüfter Befund |
| --- | --- |
| Standard-SXceiver benötigt GPIO5 für Reset | Bestätigt |
| HW 1.2 verwendet TX22/RX23 | Bestätigt; zusätzlich HW-1.0-Abweichung dokumentiert |
| SPI-Gerät ist `/dev/spidev0.0` | Bestätigt |
| I²S/SPI0 gehören zum Daten-/Steuerpfad | Bestätigt |
| GPIO4/6 bzw. SDA/SCL sind durch ihr Leuchten belegt | Als Schlussfolgerung weiterhin unzulässig |
| GPIO7 ist automatisch frei | Nicht festgestellt; Treiber nutzt CE0, übrige Reservierungen müssen live geprüft werden |
| HAT-Default ist pauschal Version 1.2 | Präzisiert: Laufzeit-Fallback 1.1, Makefile-Default 1.2 |
| Leuchtende TX-/RX-Leitungen zeigen aktuellen Funkbetrieb | Nicht ableitbar; unter anderem `AUTO` setzt beide auf 1 |
| Vollständige freie Pinmatrix liegt vor | Weiterhin nicht belegt |

### 8.3 Angrenzender Hardware-Gateway-Stand

Auf `main@9116c15d645458f99e236712b67a1ad970432791` sind ein Hardware Gateway und ein Beispiel-Edge-Agent vorhanden. Sie sind mögliche spätere Integrationspunkte, **kein Nachweis, dass die für diesen Entwicklungsstand betrachteten Pins bereits für Sensorik verwendet werden**.

Die README nennt:

- HTTP/WebUI-Port **8250**.
- `POST /api/v1/telemetry` sowie Status-, Geräte-, Ereignis- und Health-Endpunkte.
- MQTT-Eingang `netcore/v1/hardware/<device-id>/telemetry`.
- Normalisierte Zustände unter `netcore/v1/state/hardware/<device-id>`.
- Standardmäßig vollständig deaktivierte Hardware-Ausgänge.

Der tatsächlich gelesene Beispielagent `examples/edge-agent/netcore-edge-agent.py` liest die CPU-Temperatur aus `/sys/class/thermal/thermal_zone0/temp`, sofern vorhanden. Ohne diese Datei erzeugt er einen Zufallswert. Feuchte 45,0 %, Versorgung 12,2 V und mehrere Kontakte sind im Beispiel fest eingetragen; `outputs` ist leer.

**Folge:** Die Beispieltelemetrie ist kein Messnachweis für einen externen Sensor, ein echtes Spannungsmessmodul oder freie SXceiver-GPIO. Vor einem Hardwarepilot sind reale Erfassung und Kennzeichnung der Datenqualität nötig.

### 8.4 Geprüfte Gesamtprioritäten

Die gelesene zentrale `ROADMAP.md` auf main nennt **Z01.1** als ersten Gesamtprojektschritt. Reale Rack-Sensoren und RF-Messhardware sind dem weiteren praktischen Ausbau **Z08** zugeordnet.

Die Pinprüfung ist eine notwendige Vorarbeit für einen solchen Hardwarepilot. Die unten genannten lokalen Schritte ändern die übergeordnete Reihenfolge nicht. Es wurde nichts an `ROADMAP.md` oder anderen Dateien außerhalb des Archivs geändert.

## 9. Befehle und Diagnoseablauf

### 9.1 Tatsächlich im historischen Entwurf vorgeschlagen, nicht nachweislich ausgeführt

Auf dem Raspberry Pi bei laufender Basisstation:

~~~bash
sudo pinctrl get
sudo gpioinfo
~~~

**Status:** Nur vorgeschlagen. Keine erfolgreiche Ausführung, Fehlermeldung oder Ausgabe des Zielgeräts überliefert.

Zweck:

- `pinctrl get`: aktuelle Pin-Funktionen und Pegel einsehen.
- `gpioinfo`: GPIO-Chips, Line-Metadaten, Richtung und gemeldete Consumer einsehen.
- Beide Ausgaben zusammen mit der HAT-Revision und den Quellen auswerten.

Ein in `gpioinfo` als unbenutzt erscheinender Anschluss ist allein keine Freigabe: eine alternative Peripheriefunktion, externe Verdrahtung oder eine nur zeitweise Nutzung muss ebenfalls berücksichtigt werden. Auch ein Momentanpegel dokumentiert nicht alle Betriebszustände.

### 9.2 Ergänzung dieses Archivlaufs: sinnvoller Nachweisrahmen

Die folgenden **zusätzlichen Vorschläge wurden weder damals noch in diesem Archivlauf auf der TBS ausgeführt**:

~~~bash
date --iso-8601=seconds
uname -r
tr -d '\0' < /proc/device-tree/model
tr -d '\0' < /proc/device-tree/hat/product_id
tr -d '\0' < /proc/device-tree/hat/product_ver
sudo pinctrl get
sudo gpioinfo
~~~

Fehlende Dateien oder nicht vorhandene Werkzeuge sind als Diagnoseergebnis zu notieren. Insbesondere darf aus einem nicht lesbaren `product_ver` keine echte Boardrevision behauptet werden.

Der nächste Auswertungsschritt sollte enthalten:

1. Modell/Revision des Pi, OS/Kernel, HAT-Aufdruck und ausgelesene HAT-Kennung festhalten.
2. Installierte SoapySX-Version bzw. Quell-/Buildherkunft zuordnen.
3. Den laufenden Betriebszustand angeben: Basisstationsdienst aktiv, RX/TX-/PA-Einstellung bekannt.
4. Pinmux, Consumer und die obige Arbeitsmatrix vergleichen.
5. Erst anschließend die tatsächlichen elektrischen Verbindungen des konkreten HAT/Breakouts abgleichen und Kandidaten für Zusatzfunktionen freigeben.

Ein `SoapySDRUtil --probe` wurde in der damaligen Recherche zwar in fremden Quellen erwähnt, aber **nicht** als einer der beiden abschließend angeforderten Schritte ausgeführt. Es ist hier kein Bestandteil der parallelen Diagnose am bereits vom Basisstationsdienst geöffneten SDR.

Es wurden keine GPIO-Schreibbefehle, EEPROM-Neuprogrammierungen, Installationen, Dienststopps oder Neustarts angeordnet oder ausgeführt. Ein funktionsfähiger HAT wird aus der LED-Beobachtung allein nicht als reparaturbedürftig eingestuft.

## 10. Fehlerbild, Ursachen und offene Probleme

| Punkt | Beobachtung bzw. Problem | Einordnung / Lösung |
| --- | --- | --- |
| Schluss von LED auf Nutzung | LED-Liste wurde als möglicher Nutzungsumfang gemeldet | Fachlich korrigiert: Pegel-/Anzeigezustand und funktionale Belegung getrennt behandeln |
| Fehlende belegte Pins | Reset, mehrere SPI-Leitungen und I²S-Takt fehlen in der Liste | Mit Quellenabgleich ergänzt; kein Defekt daraus abgeleitet |
| SDA/SCL leuchten | Erklärung durch Pull-ups möglich | Plausible Erklärung, keine Messung am konkreten Board |
| IO4/IO6 leuchten | Ursache im realen Aufbau unbekannt | SoapySX allein erklärt keine direkte Steuerfunktion; Konfiguration und Verdrahtung prüfen |
| IO7 leuchtet | Zweiter SPI-Chip-Select kann im Ruhezustand High sein | Weder SXceiver-Nutzung noch freie Verfügbarkeit dadurch bewiesen |
| Kein Live-Readback | Ausgaben fehlen | Vorgeschlagene lesende Diagnose nachholen |
| Unbekannte Anzeigeschaltung | Originalfoto/Schaltplan fehlen | LED-Polarität und elektrische Belastung nicht abschließend bewertbar |

Ein tatsächlich aufgetretener Hardwarefehler, eine Pin-Kollision oder eine durch Zusatzhardware verursachte Funkstörung wurde in dem dokumentierten Aufbau **nicht nachgewiesen**. Es gibt entsprechend keinen hier erfolgreich ausgeführten Reparaturablauf.

Im älteren Sensorik-/TFT-Archiv wird zusätzlich eine frühere Verwechslung mit GPIO25 als Reset beschrieben. Diese frühere Originalaussage ist im hier sichtbaren Verlauf nicht enthalten. Unabhängig davon belegt die geprüfte Quellprüfung **GPIO5**; der sekundär überlieferte GPIO25-Vorschlag darf nicht als gültige SXceiver-Vorgabe übernommen werden.

## 11. Tests, Prüfungen und ihre Grenzen

| Prüfung | Tatsächliches Ergebnis | Aussagegrenze |
| --- | --- | --- |
| Beobachtung am Aufbau der LEDs | Konkrete Liste liegt vor | Keine Messung, kein verifiziertes Bild, kein zeitlicher Signalverlauf |
| Historische Prüfung offizieller Quellen | Reset/SPI/I²S/RX/TX zugeordnet | Standardhardware und Quellen, keine Live-Konfiguration |
| Erneutes Lesen des Zielbranches | Archiving und vorhandener Index zugänglich | Belegt Repositoryzugriff, nicht TBS-Zugriff |
| Geprüfte Quellprüfung der drei SX-Dateien | GPIO5, SPI0.0, RX/TX-Versionszweige und Overlay bestätigt | Kein Build und kein Hardwaretest |
| Vergleich der Git-Blob-IDs | Drei Dateien in Archiving/main/Upstream identisch | Keine Vollrepo- oder Installationsgleichheit |
| Prüfung des Beispiel-Edge-Agenten | CPU-Temperatur plus feste/simulierte weitere Werte | Kein Nachweis externer Hardwaremessung |
| `pinctrl` und `gpioinfo` auf der TBS | **Nicht durchgeführt / kein Ergebnis verfügbar** | Freigabe zusätzlicher Pins bleibt offen |
| Elektrische, thermische, EMV- oder On-Air-Abnahme | **Nicht durchgeführt** | Keine Belastungs-, Stör- oder Betriebssicherheitszusage |

Für reine Archivdokumentation wurde kein neuer Software-/RF-Testlauf gestartet. Ein solcher würde die konkret fehlenden Hardwarebelege nicht ersetzen.

## 12. Ersetzte Annahmen und verbleibende Ideen

### Bei der bisherigen Diagnose fachlich ersetzt

- „Leuchtend“ als Synonym für „vom SXceiver benutzt“.
- „Nicht in der LED-Liste“ als Synonym für „frei“.
- Versorgungsspannung als Nachweis eines bestimmten Verbrauchers.
- Ungeprüfte Freigabe von SDA/SCL, GPIO4, GPIO6 oder GPIO7.

### Noch relevante Ideen und Wünsche mit Herkunft

| Idee / Wunsch | Herkunft und Status |
| --- | --- |
| Zusätzliche Funktionen an wirklich freien GPIO | Ziel der weiteren Pinplanung; noch keine freigegebene Verdrahtung |
| I²C-Nutzung von SDA/SCL | Möglicher Ausbaupfad nach Prüfung; keine in diesem Verlauf bestätigte Freigabe |
| Sensorik, Aktoren, passive Beschaltung, TFT/LCD | Im verlinkten älteren Hardwarearchiv ausgearbeitet; hier als angrenzende Planung referenziert |
| Modularer Aufbau aus Fertigmodulen statt sofortiger eigener PCB | Als spätere Projektentscheidung im älteren Archiv überliefert; Originalnotizen in diesem sichtbaren Verlauf fehlen |
| RF-Leistungs-/Reflexionsmessung, Lüfter-/Temperatur-/Versorgungsüberwachung | Angrenzende Projektideen; keine Umsetzung für diesen Entwicklungsstand |
| Abgleich derselben LED-Liste zwischen Archiven | Dokumentationsaufgabe: bei später verfügbarem Quellenbeleg die gemeinsame Herkunft klären und Doppelungen gezielt konsolidieren |

Keine dieser Ideen wird allein durch ihre Aufnahme als implementiert oder in Betrieb bestätigt markiert.

## 13. Roadmap-Kandidaten und konkrete nächste Schritte

Die folgenden Kennungen sind **lokale Archiv-Arbeitspunkte**, keine neu eingeführten zentralen Roadmap-IDs. Sie werden ausschließlich hier erfasst.

| Reihenfolge | Aufgabe | Abhängigkeit | Abnahmekriterium |
| --- | --- | --- | --- |
| G01 | Reales Pi-Modell, SXceiver-Revision und installierten Treiberstand erfassen | Zugriff auf die betroffene TBS | Modell, HAT-Kennung/Aufdruck und Buildherkunft nachvollziehbar dokumentiert |
| G02 | `pinctrl get` und `gpioinfo` bei bekanntem Betriebszustand auslesen | G01; Tools vorhanden | Vollständige, datierte Ausgaben mit Betriebszustand liegen vor |
| G03 | Vollständige Pinmatrix erstellen | G02, konkrete HAT-/Breakout-Verschaltung | Je Pin: physische Nummer, BCM, Funktion, Consumer, Pull/Bootzustand, elektrische Verbindung, Reservierung/Freigabe |
| G04 | Dunkle und leuchtende Pins gleichermaßen bewerten | G03 | Keine Freigabe allein wegen LED-Zustand oder `unused` |
| G05 | Erweiterungskandidaten SDA/SCL und GPIO4/6/7 gezielt klären | G03/G04 | Nutzen und Konflikte des konkreten Aufbaus belegt; keine generische freie Liste |
| G06 | Originalfoto nachreichen und archivieren | Zugriff auf eindeutig zugehörige Originaldatei | Original unter Docs/archive/assets/ abgelegt, Quelle/Zuordnung dokumentiert |
| G07 | Danach kleinen Sensor-/Anzeige-Pilot planen | Freigegebene Pins; geprüfter Z08-Kontext beachten | Echte Messwerte, Fehlerverhalten und störungsfreier RF-Betrieb nachgewiesen |

**Unmittelbar nächster Schritt:** G01/G02, also Hardwarestand feststellen und die bereits angeforderten lesenden Ausgaben liefern. Ein neuer PCB-Entwurf ist dafür keine Voraussetzung.

Weitere Prioritäten, Termine, Bauteilbestellungen oder ein verbindliches Displayinterface wurden im sichtbaren Verlauf nicht vereinbart.

## 14. Bilder, Anhänge und offene Belege

### 14.1 Ursprüngliches Boardbild

Ein Boardbild ist in den frühen Notizen erwähnt, aber die originale Bilddatei und ein eindeutiger Dateibeleg fehlen.

Für die Quellenprüfung vom 06.10.2026 wurden folgende Zugänge geprüft:

- Die synchronisierten Projektquellen enthalten die bereitgestellten ETSI-PDFs, aber kein zugehöriges Boardfoto.
- Die Suche nach „SXceiver“ bzw. „GPIO Breakout“ lieferte kein eindeutig zugehöriges Originalbild.
- Die zeitlich eingegrenzte Liste hochgeladener Bilder um den sichtbaren Beginn der Arbeitsphase enthielt keinen eindeutig zuordenbaren SXceiver-/LED-Nachweis.

- Auch das bereits vorhandene verwandte Sensorik-/TFT-Archiv dokumentiert ein fehlendes Originalbild.

**Bildnachweis offen:** Das Originalfoto konnte nicht wiederhergestellt werden. Die schriftliche LED-Liste ist erhalten; andere Produktbilder oder Transkriptionen ersetzen das Originalfoto nicht.

### 14.2 Bereitgestellte PDFs

Für den Projektkontext sind 25 ETSI-/TETRA-PDF-Dateien bereitgestellt, darunter `ETSI.pdf` und `en_30039202v030801p.pdf`. Sie wurden für diese herstellerspezifische Pi-/SXceiver-Pinprüfung nicht inhaltlich ausgewertet und nicht dupliziert. Keine technische Aussage dieser Pinzuordnung wird auf eine angeblich gelesene Normpassage gestützt.

### 14.3 Vollständigkeitsgrenze

Grundlagen sind die schriftliche LED-Liste, die historische Pinzuordnung, die damaligen Quellenbefunde und der abgegrenzte Repository-Abgleich vom 06.10.2026.

Nicht verfügbar bzw. nicht nachgewiesen sind:

- das Originalfoto einschließlich seiner exakten Boardorientierung;
- gegebenenfalls zusätzliche frühere oder spätere Arbeitsnotizen;
- die tatsächlichen Diagnoseausgaben vom Pi;
- installierte Treiberversion, konkrete Pinmux-/Overlay-Konfiguration und bestätigte reale HAT-Revision;
- Schaltplan/elektrische Eigenschaften des LED-Breakouts;
- Nachweis einer Sensor-/Displayinstallation oder einer Freigabe weiterer Pins.

Die Quellenlücken lassen keine vollständige Rekonstruktion des Aufbaus und keine Hardwareabnahme zu.

## 15. Quellen und Querverweise

### Unmittelbare Quellen

- **Q1:** Beobachtungsliste der leuchtenden Boardanschlüsse vom 03.10.2026; in Abschnitt 3 in der ursprünglichen Reihenfolge erhalten.
- **Q2:** Technische Pinzuordnung vom 03.10.2026 und Diagnosevorschlag `sudo pinctrl get` / `sudo gpioinfo`.

### Geprüfte Repository-Quellen

- [Geprüfter Archiving-Ausgangscommit](https://github.com/JanHG98/netcore-tetra/commit/000fad98419f6db1d8d3815f8c7c916375b02a41).
- [Geprüfter main-Commit](https://github.com/JanHG98/netcore-tetra/commit/9116c15d645458f99e236712b67a1ad970432791).
- [SoapySX.cpp im geprüften main](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/sxxcvr-main/SoapySX/SoapySX.cpp).
- [EEPROM-Makefile im geprüften main](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/sxxcvr-main/dts/Makefile).
- [Device-Tree-Overlay im geprüften main](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/sxxcvr-main/dts/sx1255_raspberrypi.dts).
- [Hardware-Gateway-README](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/system-backend/hardware-gateway/README.md).
- [Beispiel-Edge-Agent](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/system-backend/hardware-gateway/examples/edge-agent/netcore-edge-agent.py).
- [Zentrale ROADMAP.md, geprüfter Stand](https://github.com/JanHG98/netcore-tetra/blob/9116c15d645458f99e236712b67a1ad970432791/ROADMAP.md).

### Hersteller-/Upstream-Quellen

- [SXceiver-Spezifikationen](https://sxceiver.com/doc/specs): SX1255-Anbindung über SPI/I²S; am 06.10.2026 erneut abgerufen.
- [Upstream tejeez/sxxcvr, geprüfter Commit](https://github.com/tejeez/sxxcvr/tree/9705147dd8c189625071f3f163ea56119bda4a05).
- [Raspberry-Pi-Hardwaredokumentation](https://www.raspberrypi.com/documentation/computers/raspberry-pi.html#gpio): GPIO-Grundlagen, Versorgung und Pull-ups; am 06.10.2026 erneut aufgerufen.
- [Raspberry-Pi-SPI-Dokumentation](https://www.raspberrypi.com/documentation/computers/raspberry-pi.html#serial-peripheral-interface-spi): physische/BCM-Zuordnung der SPI-Signale.

### Verwandte Archive

- [SXceiver-GPIO, Sensorik, TFT und modularer Hardwareausbau](2026-10-05_sxceiver-gpio-sensorik-tft-und-modularer-hardwareausbau.md).
- [GPIO-Pass-Through, Breakout und Jumpersteuerung](2026-10-05_basisstation-gpio-breakout-jumper-steuerung-sxceiver.md).
- [SoapySX-/Trixie-/libgpiod-Buildreparatur](2026-10-04_sxceiver-soapysx-trixie-libgpiod-build-reparatur.md).

Ein GPIO-Fehler ist für den betrachteten Aufbau nicht nachgewiesen; ein entsprechender Reparatur-PR oder eine neue Pinbelegung liegen nicht vor.
