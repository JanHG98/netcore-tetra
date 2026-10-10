# Brainstorming: SXceiver/SoapySX auf Trixie – libgpiod-Buildreparatur

**Archivhinweis, 09.10.2026:** Diese Datei dokumentiert das im Titel beziehungsweise Quellenrahmen genannte Gespräch und dessen damalige Entscheidungen. Historische Kommandos, Planungen, Versionsangaben und Fehlerbilder bleiben erhalten; der heutige technische Stand steht in der [Gesamtroadmap](../roadmaps/gesamtroadmap.md) und in den [aktuellen Dienstanleitungen](../services/README.md). Die Archivaufnahme bestätigt keine spätere Implementierung oder Live-Abnahme.

> **Historisches Projektarchiv, keine pauschale Anleitung für zusätzliche Neuinstallationen.** Belegt sind der erfolgreiche Bau und die Installation des damaligen SoapySX-Moduls sowie dessen Enumeration mit `SoapySDRUtil --find`. Ein erfolgreicher Hardware-Probe-, RX/TX- oder TETRA-Betriebstest ist in dieser Entwicklungsphase nicht dokumentiert. Der zum Prüfdatum im Zielrepository enthaltene Treiber hat den damaligen libgpiod-Abhängigkeitspfad bereits durch direkte Linux-GPIO-v2-Aufrufe ersetzt. Historie und geprüfter Quellcodebefund werden deshalb ausdrücklich getrennt.

## Zielbild und Festlegungen

- Externen SXceiver/SoapySX-Treiber auf vorhandenem **Trixie/ARM64-Pi** nativ installieren; Container und eigener Treiberfork waren ausgeschlossen.
- Historische Reparatur: **libgpiod v1.6.4** unter `/opt/gpiod-v1`, `autoconf-archive`, C++-Bindings und passende Include-/Linkerpfade.
- Build, Installation von `libSXSupport.so` und Enumeration mit `SoapySDRUtil --find` sind protokolliert.
- Der Repository-Treiber nutzt inzwischen Linux-GPIO-v2 direkt. Hardware-Probe, RX/TX und Betrieb auf dem Ziel-Pi bleiben unbestätigt.

## 1. Arbeitsstand, Zuordnung und Quellenumfang

| Feld | Wert |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Native Installation von SXceiver/SoapySX auf einem frisch installierten Raspberry Pi mit Trixie/ARM64; Reparatur des libgpiod-v1/v2-Buildkonflikts |
| Archiv-ID | `sxceiver-soapysx-trixie-libgpiod-build-reparatur` |
| Eindeutiger Einstieg | Terminalprompt `jan@srv-tmo-bs01:~/sxxcvr/SoapySX/build`, `make`, Fehler bei `SoapySX.cpp:407`: `gpiod::line` ist kein Typ |
| Historische Datierung | Im sichtbaren Terminalverlauf keine vollständige Datierung. Eine ergänzende Verlaufssuche ordnet passende Nachrichten dem 06.11.2025 zu; dies ist nur ein Retrieval-Metadatum, keine vollständig datierte Entwicklungshistorie. |
| Erstellung | **2026-10-04**, Datumsbezug **Europe/Berlin** |
| Zielrepository | `JanHG98/netcore-tetra` |
| Geprüfter Ausgangscommit des Zielbranches | **`972201a03bd1351aad5ed70fe9fc0d57d777b6bb`** |
| Zugehöriger Root-Tree | `8bdbcb15cfbcd809655134de4fdce73ad844f375` |
| Archivdatei | `Docs/archive/2026-10-04_sxceiver-soapysx-trixie-libgpiod-build-reparatur.md` |
| Archivindex | `Docs/archive/README.md` |

### 1.1 Quellen- und Evidenzmodell

**H – Historische Entwicklungsnotizen:** historische Reparaturnotizen und Terminalausgaben. Die Anker H01–H11 in Abschnitt 5 benennen konkrete Stationen. Eine vorgeschlagene Kommandozeile ohne passende Ausgabe ist kein Ausführungsnachweis.

**R – Repository-Prüfung vom 04.10.2026:** gezielte Lesezugriffe auf Dateien des oben fixierten `Archiving`-Commits. Eine Code-Suche auf dem GitHub-Standardbranch diente ausschließlich zur Dateisuche; die hier geprüften Implementierungsbefunde wurden anschließend am fixierten Zielbranch-Commit gelesen. Es wurde kein anderer Branch verändert.

**W – Zusätzliche öffentliche Primärquellen:** Herstelleranleitung, Debian-Paket-/Releaseinformationen, CMake-, libgpiod-, Linux-man-pages- und C++-Dokumentation. Diese dienen dem ausdrücklich gewünschten geprüften Faktenabgleich. Sie werden nicht als Beweis für den damaligen lokalen Checkout ausgegeben.

**A – Dateikontext:** 25 zugängliche ETSI-Projekt-PDFs. Inventar, Titelseiten und Seitenzahlen wurden geprüft; die thematische Suche lieferte keine für diesen Buildfehler hilfreiche Fundstelle. Es wurde keine vollständige technische Quellenprüfung aller Normenseiten vorgenommen. Abschnitt 13 nennt die Dateien und ihre begrenzte Relevanz.

### 1.2 Grenzen und nicht verfügbare Informationen

Dokumentiert sind Compilerfehler, Reparaturschritte, erfolgreicher Build und `--find`-Ausgabe. Nicht verfügbar sind das damalige Pi-Systemabbild, der historische SoapySX-Commit, ein `ldd`-Ergebnis am richtigen Modul und das Ergebnis von `SoapySDRUtil --probe`.

Der Abgleich vom 04.10.2026 hatte keinen Zugriff auf `srv-tmo-bs01`; Hardware-, Reboot- und Dauerbetriebstests wurden nicht wiederholt. [SXceiver-Erstinstallation, HamTetra-DMO und BlueStation](2026-10-04_basisstation-erstinstallation-sxceiver-hamtetra-dmo-und-bluestation.md) dokumentiert eine separate Installation und belegt keinen Funkbetrieb nach dieser Reparatur.

## 2. Ziel, Ausgangslage und Abschlussbefund

Installiert wurde der **fremden SXceiver-Treiber aus `tejeez/sxxcvr` nach der offiziellen Herstelleranleitung**. Das System war ein frisch installierter Raspberry Pi. Die umgangssprachliche Angabe „x64“ wurde durch die tatsächlichen Paket- und Configure-Ausgaben präzisiert: **ARM64/AArch64**, nicht AMD64/x86-64. Als Distribution wurde Trixie bestätigt; die APT-Ausgabe zeigt Debian-Trixie- und Raspberry-Pi-Trixie-Quellen. [H02–H05]

Der damalige SoapySX-Code verwendete die **C++-API von libgpiod 1.x**, während das System **libgpiod 2.2.1** bereitstellte. Die Reparatur erfolgte nativ: libgpiod **v1.6.4** wurde separat unter **`/opt/gpiod-v1`** aufgebaut; entscheidend waren das zuvor fehlende Paket **`autoconf-archive`**, das Einschalten der **C++-Bindings mit `--enable-bindings-cxx`** und ein sauberer SoapySX-Build mit ausdrücklich gesetzten Include-/Linkerpfaden. [H01, H06–H10]

Der abschließende Terminalprotokoll belegt:

```text
[100%] Linking CXX shared module libSXSupport.so
[100%] Built target SXSupport
-- Installing: /usr/local/lib/SoapySDR/modules0.8/libSXSupport.so
-- Set non-toolchain portion of runtime path of "/usr/local/lib/SoapySDR/modules0.8/libSXSupport.so" to "/opt/gpiod-v1/lib"
-- Installing: /usr/share/wireplumber/main.lua.d/60-pipewire-do-not-use-i2s.lua

Found device 0
  driver = sx
  label = sx
```

**Belastbares Ende:** Buildproblem gelöst; Modul installiert und durch SoapySDR enumerierbar. **Nicht belegt:** tatsächliche GPIO-/SPI-/ALSA-Initialisierung, funktionierender Empfang oder Sender, Einsatz in NetCore-Tetra, ein bestimmter Dienst, ein Neustarttest oder Dauerbetrieb. Das frühere pauschale „Läuft“ wird auf diese nachgewiesene Aussageweite begrenzt. [H10–H11]

## 3. Endgültige Anforderungen und Entscheidungen

| Anforderung/Entscheidung | Status | Begründung und Abgrenzung |
|---|---|---|
| Raspberry Pi und Trixie weiterverwenden | **Beschlossen und im Verlauf verwendet** | Kein Neuaufsetzen auf Bookworm als abschließender Lösungsweg. |
| Keine Docker-/Container-Lösung | **Ausdrücklich beschlossen** | Docker und Container waren für diese Installation ausgeschlossen. Daraus folgt keine allgemeine technische Unmöglichkeit von Containern; es ist die verbindliche Vorgabe der dokumentierten Prüfung. |
| Fremden SoapySX-Code nicht selbst auf eine andere GPIO-Bibliotheks-API portieren | **Anforderung** | Die Installation des externen Treibers sollte repariert werden; ein eigener Treiberfork war nicht vorgesehen. |
| libgpiod v1 parallel statt System-Downgrade | **Gewählter historischer Lösungsweg** | Separater Präfix `/opt/gpiod-v1`; die normalen Trixie-Pakete mussten dafür nicht durch Bookworm-Pakete ersetzt werden. |
| libgpiod-C++-Bindings mitbauen | **Notwendige Korrektur des Lösungswegs** | Der reine C-Build lieferte kein nutzbares lokales `libgpiodcxx` für SoapySX. |
| Suchpfade für Compiler und Linker ausdrücklich setzen | **Implementiert auf dem damaligen Host / Build getestet** | Der abschließende Build mit `CXXFLAGS`, `LDFLAGS` und CMake-Runtimepfaden war erfolgreich. |
| Laufzeitpfad `/opt/gpiod-v1/lib` im Modul hinterlegen | **Installationseinstellung bestätigt** | CMake meldet den gesetzten Pfad. Die vollständige dynamische Bibliotheksauflösung wurde nicht mit dem richtigen `ldd`-Aufruf nachgewiesen. |
| Dynamisches oder statisches Einbinden von libgpiod im finalen Modul | **Nicht abschließend festgestellt** | Der falsche Dateiglob und das anschließende `echo` sind kein Linkage-Nachweis. |
| Hardware-Probe nach dem Build | **Vorgeschlagener nächster Test, offen** | Keine zugehörige Terminalausgabe nach dem Vorschlag vorhanden. |
| Dokumentation in `Archiving`, nur unter `Docs/archive/` | **Ausdrücklich autorisierter Dokumentationslauf** | Keine Änderung an Treiber, Installern, Systemdiensten oder anderen Roadmap-Dateien. |

Die Begriffe werden in diesem Dokument streng verwendet: **Idee** bedeutet Vorschlag ohne Umsetzungsbeleg; **beschlossen/geplant** bedeutet Festlegung, nicht automatisch Umsetzung; **implementiert** setzt ein benanntes Artefakt voraus; **getestet** setzt ein bestimmtes beobachtetes Ergebnis voraus; **im Betrieb bestätigt** erfordert einen tatsächlichen Betriebsbericht. Für den Funkbetrieb gibt es in dieser Entwicklungsphase keine solche Bestätigung.

## 4. Architektur, Abhängigkeiten und technische Parameter

### 4.1 Historischer Build- und Laufzeitpfad

Der alte C++-Treiber wurde als **SoapySDR-Modul `SXSupport`** gebaut. Die passenden libgpiod-v1-C++-Header mussten beim Übersetzen vor den inkompatiblen System-v2-Headern ausgewählt werden. Der Linker musste die zu diesen Headern passende Bibliothek finden. Anschließend lud SoapySDR das installierte Modul. Buildauswahl und spätere dynamische Bibliothekssuche sind zwei unterschiedliche Prüfschritte. [H01, H08–H11; W04–W06]

Der Präfix `/opt/gpiod-v1` war eine zusätzliche lokale Installation. `sudo make install` für SoapySX veränderte dagegen tatsächlich die globalen SoapySDR-Modul- und WirePlumber-Verzeichnisse. Die frühere Formulierung „kein Systemeingriff“ war insofern zu weitgehend: Kein Distributionsdowngrade, aber durchaus lokale Systeminstallation.

### 4.2 Belegte historische Umgebung

| Parameter | Beobachteter Wert / Evidenz |
|---|---|
| Host und Benutzer | `srv-tmo-bs01`, `jan` |
| Architektur | APT-Pakete `arm64`; Configure: `aarch64-unknown-linux-gnu` |
| Distribution | Trixie; Raspberry-Pi-spezifische Paketquelle und Paketrevisionen vorhanden |
| Compiler | GNU C++ **14.2.0** |
| CMake | Installiertes Paket **3.31.6-2** |
| GNU Make | Installiertes Paket **4.4.1-2** |
| pkg-config | Installiertes Paket **1.8.1-4** |
| libgpiod aus APT | `libgpiod-dev` und `libgpiod3`, **2.2.1-2** in der damaligen Ausgabe |
| Separat geklonte libgpiod-Version | Tag **`v1.6.4`** |
| libgpiod-Commit im Clone-Log | **`4a3c5d6c7ad524c8dd74f13c3ad064aa572b8037`**; aus der überlieferten Git-Ausgabe |
| SoapySDR-Systempakete | `libsoapysdr-dev`, `libsoapysdr0.8`, `soapysdr-tools`, `python3-soapysdr`; **0.8.1-5+b2** |
| ALSA-Entwicklungspaket | `libasound2-dev`, **1.2.14-1+rpt1** |
| Buildtyp | `Release`, von CMake als Standard gesetzt |
| Modul-/Treiberkennung | Buildziel `SXSupport`, Datei `libSXSupport.so`, SoapySDR-Treiber `sx` |
| Pi-Modell, SXceiver-Hardwareversion, Kernelversion | In dieser Entwicklungsphase nicht belastbar festgestellt. Keine Übernahme eines Pi-5- oder HW-1.2-Werts aus anderen Projektentwürfen. |

Die sichtbare APT-Aktualisierung verwendete `archive.raspberrypi.com/debian trixie`, `deb.debian.org/debian trixie`, `trixie-updates` und `deb.debian.org/debian-security trixie-security`. Dies belegt nicht jede lokale `.list`- oder `.sources`-Datei und auch nicht, ob frühere vorgeschlagene Änderungen daran tatsächlich ausgeführt wurden. [H05]

### 4.3 Pfade, Dateien und Werkzeuge

| Pfad / Objekt | Rolle | Belegstatus |
|---|---|---|
| `~/libgpiod` bzw. `/home/jan/libgpiod` | Clone und Build der zusätzlichen v1-Bibliothek | Historisch vorhanden und für den C-Build benutzt |
| `/opt/gpiod-v1` | Separater Installationspräfix | Historische Installationsausgabe vorhanden |
| `/opt/gpiod-v1/include/gpiod.h` | C-Header | Im zunächst unvollständigen v1-Installationslog nachgewiesen |
| `/opt/gpiod-v1/include/gpiod.hpp` | Benötigter C++-Header | Prüfung vorgeschlagen; kein eigener `ls`-Output archiviert |
| `/opt/gpiod-v1/lib/libgpiod.a` | Zunächst statisch gebaute C-Bibliothek | Installation ausdrücklich protokolliert |
| `/opt/gpiod-v1/lib/libgpiodcxx.*` | Benötigte C++-Bibliothek | Finaler Build spricht für passende Verfügbarkeit; konkrete endgültige Dateiliste fehlt |
| `/opt/gpiod-v1/lib/pkgconfig/libgpiod.pc` | C-Paketmetadaten | Im ersten erfolgreichen C-only-Installationslog vorhanden |
| `/opt/gpiod-v1/lib/pkgconfig/libgpiodcxx.pc` | C++-Paketmetadaten | Fehlten zunächst in der Installation; nach Korrektur nicht separat als Ausgabe nachgewiesen |
| `~/sxxcvr/SoapySX/SoapySX.cpp` | Historischer Treiberquellcode | Compilerlogs und späterer erfolgreicher Bau |
| `~/sxxcvr/SoapySX/build` | Historisches CMake-Buildverzeichnis | Mehrfach verwendet; final neu angelegt |
| `/usr/include/gpiod.hpp`, `/usr/include/gpiodcxx/line.hpp` | Inkompatible System-v2-C++-Header beim alten Build | Include-Stack des Compilerfehlers |
| `/usr/local/lib/SoapySDR/modules0.8/libSXSupport.so` | Tatsächlich installiertes Modul | CMake-Installationsausgabe |
| `/usr/share/wireplumber/main.lua.d/60-pipewire-do-not-use-i2s.lua` | Mitinstallierte Audiokonfiguration | Dateikopie bestätigt; Wirksamkeit im laufenden WirePlumber nicht getestet |
| `SoapySDRUtil` | Enumeration und vorgeschlagener Hardware-Probe | Nach Paketinstallation vorhanden; `--find` erfolgreich |

In dieser Entwicklungsphase wurde kein konkreter NetCore-Dienstname, keine systemd-Unit, keine API, kein TCP-/UDP-Port, kein VPN, keine Brokeradresse und keine Radiofrequenz konfiguriert. Die frühere Beispiel-Unit `dein-service-name.service` war ein Platzhalter und ist kein existierender Dienstnachweis. Der generische Empfangsbefehl mit `dein_programm`, 100 MHz und 2 MS/s war ebenfalls kein getesteter SXceiver-Aufruf.

### 4.4 GPIO-Details: historischer Codeausschnitt versus zusätzliche Implementierung

Die historischen Compilerzeilen nennen Reset-Offset **5**, RX-Offset **13** bei `hwversion == 0x0100`, sonst **23**, sowie TX-Offset **12** bei `0x0100`, sonst **22**. Das sind Codewerte; daraus folgt keine in dieser Entwicklungsphase gemessene Belegung oder identifizierte Hardwareversion. [H01]

Der zusätzliche Repository-Code R02 verwendet dieselbe bedingte Offsetzuordnung mit `hat_info.product_ver`. Er öffnet `/dev/gpiochip0` und `/dev/spidev0.0`, setzt nur Reset als **Open Source** und nutzt für RX/TX normale Outputs. Seine Initialwerte sind Reset **0**, RX **1**, TX **1**. Die am Prüfdatum vorliegende SPI-Transfergeschwindigkeit im Code beträgt **10 MHz**; ALSA verwendet `hw:CARD=SX1255,DEV=1` für Capture und `hw:CARD=SX1255,DEV=0` für Playback. Diese Werte stammen aus der geprüften statischen Prüfung, nicht aus einer historischen Hardwaremessung.

## 5. Historischer Ablauf und Fehleranalyse

### H01 – Initialer Compilerfehler: v1-Quellcode gegen v2-Header

`make` scheiterte bereits beim Übersetzen von `SoapySX.cpp`. Die primäre Meldung war, dass `gpiod::line` keinen Typ bezeichnet; der Include-Stack zeigte an dieser Stelle einen Namespace. Weitere Fehler betrafen `chip.get_line()`, `line_request::DIRECTION_OUTPUT` und `FLAG_OPEN_SOURCE`. Fehler über unbekannte Member `gpio_reset`, `gpio_rx` und `gpio_tx` waren weitgehend Folgen der ungültigen Memberdeklaration, nicht jeweils eigenständige Hardwarefehler.

Betroffene Stellen im damaligen Log: Member um Zeile **407**, `init_gpio()` um **475–487**, `reset_chip()` um **496**, Konstruktor um **559–561**, RX/TX-Setting um **1312–1321**. Diese Zeilennummern gelten nur für den damaligen Quellstand.

`sudo make install` nach einem fehlgeschlagenen Build repariert keine API-Inkompatibilität. Eine Installation ist erst nach einem erfolgreich gebauten Artefakt sinnvoll; im späteren Ablauf startete `make install` das Ziel nochmals mit.

### H02 – Herstelleranleitung und falsche Plattformannahmen

Ausgangspunkt war `https://sxceiver.com/doc/getting-started` auf einem frisch installierten Pi mit 64-Bit-System. Frühere Entwürfe unterstellten dennoch einen x86-Server beziehungsweise Bookworm mit Backports. Diese Annahmen waren nicht belegt und wurden durch die tatsächlichen Trixie-/ARM64-Ausgaben ersetzt.

Die zum Prüfdatum gelesene Herstellerseite ist keine konservierte Kopie der damaligen Seite. Sie darf deshalb nicht zur Behauptung verwendet werden, dass exakt dieselbe Abhängigkeitsliste schon damals dort stand. [W01]

### H03 – Nicht vorhandenes Bookworm-APT-Ziel

Der vorgeschlagene Paketaufruf mit `-t bookworm` scheiterte mit:

```text
E: The value 'bookworm' is invalid for APT::Default-Release as such a release is not available in the sources
```

Anschließend wurde **Debian Trixie** bestätigt. Auch der Versuch mit `libgpiod-dev=1.6.* libgpiod2` fand keine passenden Kandidaten. Die von APT genannten `:armhf`-Alternativen belegen keine passende Bibliothekslösung für dieses ARM64-System.

Die damaligen Empfehlungen zum Umschreiben sämtlicher Quellen auf Bookworm sind ausdrücklich verworfen. Ein Austausch von Distributionsnamen in APT-Dateien ist kein vollständig durchgeführter oder konsistenter System-Downgrade. Es gibt keinen Beleg, dass ein solcher Downgrade hier erfolgreich vorgenommen wurde.

### H04 – Native Lösung als verbindliche Vorgabe

Der externe SXceiver-Code sollte unverändert bleiben; eine Containerlösung war ausgeschlossen. Die weitere Lösung musste daher **auf dem vorhandenen Pi ohne Container und ohne eigene GPIO-Portierung** funktionieren.

Die Nebenwege `$HOME/gpiod-v1`, Bookworm-Container, Chroot und nspawn blieben Vorschläge beziehungsweise wurden verworfen. Der tatsächlich weiterverfolgte Präfix war `/opt/gpiod-v1`.

### H05 – Erster v1-Build scheitert an fehlendem Autoconf-Makropaket

Die Installation der grundlegenden Werkzeuge und der Clone von libgpiod v1.6.4 liefen. `./autogen.sh` brach jedoch mit dem Hinweis auf ein unaufgelöstes `AX_`-Makro und das fehlende **GNU autoconf-archive** ab. Daher wurden keine verwendbaren Makefiles für diesen Versuch erzeugt.

Die nachfolgenden Meldungen „No targets specified and no makefile found“ und „No rule to make target 'install'“ waren **Folgefehler dieses Abbruchs**. Weitere Fehler kamen hinzu, weil `~/sxxcvr/SoapySX` zu diesem Zeitpunkt in der gezeigten Umgebung fehlte: Der fehlgeschlagene `cd` ließ die Shell im falschen Verzeichnis; CMake suchte anschließend in `/home/jan/libgpiod` nach einer nicht vorhandenen `CMakeLists.txt`.

Auch `SoapySDRUtil` fehlte zunächst. Danach wurden die SoapySDR-/ALSA-Abhängigkeiten einschließlich `libgpiod-dev 2.2.1-2` installiert und `tejeez/sxxcvr` geklont. Der anschließende Standardbuild wiederholte erwartbar den ursprünglichen v1/v2-Konflikt.

**Lehre:** Nach einem fehlgeschlagenen Konfigurationsschritt oder `cd` nicht mit unabhängigen Folgekommandos weitermachen. Ein späterer Reparaturablauf sollte bei Fehlern abbrechen und jede Voraussetzung prüfen; ein angehängtes `sudo make install` ist kein Ersatz dafür.

### H06 – Autoconf funktioniert, aber nur die C-Bibliothek wird installiert

Nach dem Hinweis auf `autoconf-archive` lief die Autotools-Konfiguration im nächsten geposteten Versuch durch. Die tatsächliche Paketinstallation dieses einzelnen Zusatzpakets ist nicht als eigener Logblock sichtbar; der vorherige Makrofehler trat aber nicht mehr auf.

Der verwendete Aufruf enthielt `--disable-tools --enable-static --disable-shared`, jedoch **kein `--enable-bindings-cxx`**. Die Ausgabe dokumentiert die Installation von `gpiod.h`, `libgpiod.a` und `libgpiod.pc`. Unter `bindings` wurde der C++-Unterbau nicht tatsächlich gebaut/installiert. Dass Configure eine Datei `bindings/cxx/libgpiodcxx.pc` erzeugte, bedeutete nicht, dass das C++-Binding damit auch installiert war.

### H07 – pkg-config meldet weiterhin System-v2

Trotz `PKG_CONFIG_PATH=/opt/gpiod-v1/lib/pkgconfig` ergab die Prüfung:

```text
pkg-config --modversion libgpiodcxx
2.2.1

pkg-config --cflags --libs libgpiodcxx
-lgpiodcxx
```

Das ist konsistent mit einem fehlenden lokalen C++-Binding: Der ergänzte Suchpfad enthielt keinen passenden installierten C++-Paketdatensatz; das Werkzeug fand weiter den Systemdatensatz. Ein erneutes schlichtes `cmake ..` und `make` verwendete weiterhin die inkompatiblen Header.

Es fehlte also nicht bloß „noch ein Pfad“, sondern zunächst das **richtige C++-Artefakt**.

### H08 – Entscheidende Ergänzung: C++-Bindings aktivieren

Der Reparaturvorschlag wurde auf `--enable-bindings-cxx` korrigiert. Zusätzlich wurde ein isolierter pkg-config-Suchpfad vorgeschlagen. Die vollständige Ausgabe des anschließend korrigierten libgpiod-C++-Builds und ein danach erfolgreiches `pkg-config --modversion` mit 1.6.4 sind nicht gepostet.

Dass der folgende SoapySX-Build mit dem lokalen Include-/Bibliothekspfad durchlief, ist ein starker indirekter Beleg für die behobene Buildumgebung. Es ersetzt dennoch weder eine genaue endgültige Dateiliste unter `/opt/gpiod-v1` noch die ausstehende Prüfung aller Laufzeitbibliotheken.

### H09 – Sauberer SoapySX-Build mit expliziten Flags

Das alte SoapySX-Buildverzeichnis wurde entfernt; die Neukonfiguration setzte `CXXFLAGS`, `LDFLAGS`, `PKG_CONFIG_LIBDIR`, `CMAKE_PREFIX_PATH`, Build-RPATH und Install-RPATH. Dieser Versuch kompilierte und linkte erfolgreich.

CMake meldete zugleich, dass `CMAKE_INCLUDE_PATH` und `CMAKE_LIBRARY_PATH` nicht verwendet wurden. Diese Warnung ist kein Beleg dafür, dass sämtliche angegebenen Optionen ignoriert wurden. Der entscheidende Unterschied im erfolgreichen Kommando waren insbesondere die direkten Compiler-/Linkerflags. CMake-Umgebungsflags werden bei der Erstkonfiguration in Cachewerte übernommen; daher war der neue Buildbaum wichtig. [W04–W05]

### H10 – Installation erfolgreich, Lock-Warnung bleibt

Das Modul wurde als **`libSXSupport.so`** unter `modules0.8` installiert; der konfigurierte Laufzeitpfad wurde protokolliert. Zusätzlich wurde die WirePlumber-Lua-Datei kopiert.

Der Compiler warnte weiter bei der damaligen Zeile **657**:

```cpp
std::scoped_lock(stream->mutex);
```

Die wiederholte Einstufung dieser Warnung als „nur kosmetisch“ war falsch. Ein unbenanntes temporäres Lockobjekt schützt den nachfolgenden Code nicht über einen beabsichtigten längeren Scope. Der Build wird dadurch nicht zwingend verhindert, die Synchronisationswirkung ist aber relevant. Ein benanntes RAII-Lock hält den Mutex bis zum Ende seiner Lebensdauer. Ob daraus auf dem damaligen Host ein beobachteter Fehler entstand, ist unbekannt. [W07]

### H11 – Falscher ldd-Dateiname, erfolgreiche Enumeration

Der vorgeschlagene Glob `libSoapySX*.so` traf die installierte Datei nicht. `ldd` meldete eine nicht vorhandene Datei. Die Pipeline

```text
ldd ... | grep -i gpiod || echo "statisch gelinkt"
```

gab anschließend trotzdem „statisch gelinkt“ aus. Diese Ausgabe wurde allein durch den fehlgeschlagenen Filterpfad ausgelöst und beweist **keine statische Einbindung**.

Danach lieferte `SoapySDRUtil --find` den Eintrag `driver = sx`, `label = sx`. Ein korrekter `ldd`-Aufruf auf `libSXSupport.so` und ein `--probe` wurden erst als letzte Prüfaufgabe vorgeschlagen; ihre Ergebnisse fehlen. Das ist der offene Endpunkt dieser Planung.

## 6. Wichtige Befehle mit Ausführungsstatus

### 6.1 Nachweislich ausgeführte Paketinstallation und Quellbeschaffung

Die folgende Paketliste ist durch die Installation beziehungsweise die Bestätigung bereits vorhandener Pakete belegt. `libgpiod-dev` ist dabei ausdrücklich das damalige **System-v2-Paket**, nicht die separat gebaute v1:

```bash
sudo apt-get install -y --no-install-recommends \
  git make g++ cmake libsoapysdr-dev libasound2-dev \
  libgpiod-dev soapysdr-tools python3-soapysdr

git clone "https://github.com/tejeez/sxxcvr.git"
```

Ebenfalls belegt ist der Clone von libgpiod mit dem Tag `v1.6.4`. Die Git-Meldung über **detached HEAD** ist bei einem Checkout eines Tags kein Buildfehler. Der historische SoapySX-Clone wurde hingegen nicht durch einen in der Planung festgehaltenen Commit gepinnt. [H05–H06]

### 6.2 libgpiod-v1-Reparatur: vorgeschlagener abschließender Buildschritt

**Status:** notwendige Korrektur vorgeschlagen; kompletter Erfolgslog dieses Teilblocks fehlt. Nicht als separat vollständig getesteten Installationsblock ausgeben.

```bash
cd ~/libgpiod
# Zuvor war ein Bereinigen des alten Configure-/Buildzustands vorgeschlagen.
./autogen.sh --prefix=/opt/gpiod-v1 \
             --disable-tools \
             --enable-bindings-cxx
make -j"$(nproc)"
sudo make install
```

Voraussetzung der erfolgreichen Autotools-Generierung war die Behebung des fehlenden `autoconf-archive`. Der frühere statische C-only-Aufruf ohne C++-Bindings ist nicht der vollständige Reparaturschritt.

Für einen späteren **alten** Checkout wären zunächst `gpiod.hpp`, die C++-Bibliothek und `libgpiodcxx.pc` im lokalen Präfix zu prüfen. Ein isolierter Einzeltest kann beispielsweise mit `PKG_CONFIG_PATH=` und `PKG_CONFIG_LIBDIR=/opt/gpiod-v1/lib/pkgconfig` erfolgen. Ein global exportiertes `PKG_CONFIG_LIBDIR` kann dagegen andere Systempaketdatensätze ausblenden und sollte nicht unbemerkt in beliebige Folge-Builds übernommen werden.

### 6.3 Historischer SoapySX-Build: erfolgreich ausgeführt

**Status:** Der folgende CMake-Aufruf und die anschließenden Build-/Installationsbefehle sind durch die vollständige Erfolgsausgabe belegt. Zuvor wurde ein neues `~/sxxcvr/SoapySX/build` angelegt. Der Block gilt für den damaligen alten Quellstand; er ist keine Empfehlung, einen geprüften Treiber unnötig an libgpiod v1 zu binden.

```bash
cd ~/sxxcvr/SoapySX/build

CXXFLAGS="-isystem /opt/gpiod-v1/include" \
LDFLAGS="-L/opt/gpiod-v1/lib" \
PKG_CONFIG_LIBDIR=/opt/gpiod-v1/lib/pkgconfig \
cmake -DCMAKE_PREFIX_PATH=/opt/gpiod-v1 \
      -DCMAKE_LIBRARY_PATH=/opt/gpiod-v1/lib \
      -DCMAKE_INCLUDE_PATH=/opt/gpiod-v1/include \
      -DCMAKE_BUILD_RPATH=/opt/gpiod-v1/lib \
      -DCMAKE_INSTALL_RPATH=/opt/gpiod-v1/lib \
      ..

make -j"$(nproc)"
sudo make install
sudo ldconfig
```

`CMAKE_LIBRARY_PATH` und `CMAKE_INCLUDE_PATH` wurden laut Ausgabe nicht benutzt. Ihr Vorhandensein im historischen Erfolgsblock macht sie nicht nachträglich zu wirksamen Ursachen. Ebenso ersetzt ein gesetztes pkg-config-Verzeichnis keine Compileroption, wenn das betreffende CMake-Projekt pkg-config dafür gar nicht auswertet.

### 6.4 Korrigierte nächste Prüfungen – nicht in der Planung ausgeführt

Die folgenden Diagnoseaufrufe sind **noch offen**. Sie verändern nicht den Treiberquellcode. Der Hardware-Probe initialisiert allerdings das Gerät und darf nicht parallel zu einer laufenden Funkanwendung erfolgen.

```bash
# Genau die im Installationslog genannte Datei prüfen:
test -f /usr/local/lib/SoapySDR/modules0.8/libSXSupport.so && \
  ldd /usr/local/lib/SoapySDR/modules0.8/libSXSupport.so

# ELF-Abhängigkeiten und tatsächlichen RPATH/RUNPATH anzeigen:
readelf -d /usr/local/lib/SoapySDR/modules0.8/libSXSupport.so

# Enumeration ist noch kein Hardwaretest:
SoapySDRUtil --find="driver=sx"

# Erst bei exklusiv freiem Gerät:
SoapySDRUtil --probe="driver=sx"
```

Bei der Quellenprüfung: keine `not found`-Abhängigkeiten übersehen; bei einem alten v1-Build die tatsächlichen libgpiod-/libgpiodcxx-Pfade nachvollziehen. **Keine libgpiod-Zeile** ist zum Prüfdatum auch mit einem Treiber ohne diese Abhängigkeit vereinbar und deshalb selbst am richtigen Modul nicht automatisch „statisch gelinkt“.

Die CMake-Bezeichnung RPATH sagt allein noch nicht, ob das ELF `DT_RPATH` oder `DT_RUNPATH` enthält. `DT_RUNPATH` gilt nicht automatisch für alle transitiven Abhängigkeiten. Deshalb auch den C++-Wrapper und dessen C-Bibliothek auflösen, statt aus dem Installationsbanner vollständige Laufzeitunabhängigkeit von der Shell abzuleiten. [W06]

Ergänzende Bestandsaufnahme für eine Fortsetzung, ebenfalls nicht ausgeführt:

```bash
cat /etc/os-release
uname -m
uname -r
dpkg --print-architecture

git -C "$HOME/sxxcvr" rev-parse HEAD
git -C "$HOME/sxxcvr" status --short
git -C "$HOME/libgpiod" rev-parse HEAD
sha256sum /usr/local/lib/SoapySDR/modules0.8/libSXSupport.so
```

Die Pfade gelten nur, soweit sie auf dem später untersuchten Host weiterhin existieren. Keine Zugangsdaten oder komplette sensitive Konfigurationen für diese Bestandsaufnahme übernehmen.

## 7. Tests, Ergebnisse und Nachweisgrenzen

| Prüfung | Tatsächliches Ergebnis | Reichweite |
|---|---|---|
| Alter SoapySX-Standardbuild gegen Systemheader | **Fehlgeschlagen, mehrfach protokolliert** | Belegt API-Konflikt, nicht defekte Hardware. |
| APT-Installation v1 aus Bookworm-Ziel | **Fehlgeschlagen** | Zielrelease beziehungsweise Paketversion nicht verfügbar. |
| Erster libgpiod-Autotools-Lauf | **Fehlgeschlagen** | `autoconf-archive` fehlte. |
| Nächster libgpiod-v1-C-only-Build | **Erfolgreich** | Nur C-Artefakte belegt; noch keine ausreichende SoapySX-C++-Abhängigkeit. |
| pkg-config nach C-only-Installation | **2.2.1 / System-C++-Bibliothek** | Lokale v1-C++-Installation war dadurch gerade nicht bestätigt. |
| SoapySX nach C++-Korrektur und neuem Buildbaum | **Erfolgreich gebaut und installiert** | Compiler-/Linkschritt und Kopie des Moduls bestätigt. |
| ldd mit `libSoapySX*.so` | **Fehlgeschlagen: Datei nicht vorhanden** | Nachfolgendes „statisch gelinkt“ ist ein Fehlindikator. |
| `SoapySDRUtil --find` | **Erfolgreicher Eintrag `sx`** | Modulregistrierung/Enumeration; kein RX/TX-Nachweis. |
| ldd am richtigen Modul / readelf | **Nicht dokumentiert** | Finaler Linkage- und Loaderpfad offen. |
| `SoapySDRUtil --probe` | **Nur vorgeschlagen** | Kein Nachweis für GPIO, SPI, Chipreset, Clock-Erkennung oder ALSA. |
| Empfang, Sender, TETRA-Ruf/SDS, Reboot, Dauerlauf | **Nicht dokumentiert** | Keine entsprechende Betriebsfreigabe ableitbar. |
| Zusätzliche Repository-Prüfung | **Statische Sichtprüfung relevanter Dateien** | Kein geprüfter C++-Build, kein Python-Hardwaretest, kein CI-Erfolg behauptet. |

Der zusätzliche `findDevice()`-Code enthält ausdrücklich noch den Hinweis, dass die tatsächliche Gerätepräsenz geprüft werden müsste, und liefert den `sx`-Eintrag ohne eine solche Prüfung zurück. Damit ist die Trennung zwischen Enumeration und Geräteinitialisierung besonders wichtig. Dieser zusätzliche Quellcodebefund ist kein nachträglich rekonstruierter Hash des historischen Moduls. [R02]

## 8. Zusätzlich geprüfter Repository-Stand vom 04.10.2026

### 8.1 Der ursprüngliche libgpiod-Konflikt ist im zum Prüfdatum mitgeführten Treiber strukturell beseitigt

In `sxxcvr-main/SoapySX/SoapySX.cpp` sind eigene Klassen **`GpioChip`** und **`GpioLine`** enthalten. Sie verwenden `<linux/gpio.h>`, `gpio_v2_line_request`, `GPIO_V2_GET_LINE_IOCTL` und `GPIO_V2_LINE_SET_VALUES_IOCTL`. Der geprüfte GPIO-Pfad ist somit **direkter Linux-Kernel-UAPI-Zugriff**, nicht eine Portierung auf die libgpiod-v2-C++-Bindings. [R02]

` sxxcvr-main/SoapySX/CMakeLists.txt` setzt die expliziten zusätzlichen Gerätebibliotheken auf **`asound`**; eine libgpiod-Linkabhängigkeit wird dort nicht mehr aufgeführt. Auch die Abhängigkeitsliste in `sxxcvr-main/README.md` nennt kein libgpiod-Entwicklungspaket mehr. [R01, R03]

**Bedeutung:** Der alte `/opt/gpiod-v1`-Workaround ist nicht automatisch Voraussetzung für den am Prüfdatum geprüften Treiber. Ob der Pi diesen neueren Code verwendet, muss erst über Checkout, Modulherkunft und Probeausgabe festgestellt werden. Das Archiv aktualisiert oder ersetzt den installierten Treiber nicht.

### 8.2 Die konkrete Lock-Stelle ist im geprüften Quellcode korrigiert

`closeStream()` enthält im geprüften Stand ein benanntes Lock:

```cpp
std::scoped_lock lock(stream->mutex);
```

Damit ist der konkrete historische Ausdruck an dieser Stelle im Repository ersetzt. Das belegt eine Codekorrektur, nicht die Installation dieses Codes auf `srv-tmo-bs01` und auch keinen umfassenden Nachweis der Thread-Sicherheit des gesamten Treibers. [R02]

### 8.3 Versionsdiagnose ist zum Prüfdatum im Code vorgesehen

Der Treiber führt `SoapySX_tag` und `SoapySX_commit` und gibt in `getHardwareInfo()` die Schlüssel **`soapysx_tag`**, **`soapysx_commit`** sowie **`hardware_version`** zurück. Beim Erzeugen einer Geräteinstanz wird die Version geloggt. Diese Funktionen können eine spätere Source-/Binary-Zuordnung unterstützen; eine solche Ausgabe liegt aus dem historischen Abschluss noch nicht vor. [R02]

Die HAT-Informationen werden im geprüften Code aus `/proc/device-tree/hat/product_id` und `product_ver` gelesen. Der Fallback bei nicht lesbarer Identifikation ist eine **Annahme des Codes**, keine gemessene Hardwareversion. Auch eine Versionsmeldung allein ersetzt keinen Streamtest.

### 8.4 Buildverpackung des eingebetteten Quellbaums gesondert prüfen

Das zusätzliche CMake-Projekt generiert `version.cpp` über `version.sh` und nennt **`../.git/index`** als Abhängigkeit. Der geprüfte Repository-Tree führt `sxxcvr-main` als normalen Unterbaum, nicht als Git-Submodul mit einem mitgelieferten eigenen `.git`-Verzeichnis. Zudem ist `SoapySX/version.sh` im Tree als Modus **`100644`** gelistet, während CMake das Skript direkt als Kommando adressiert. [R01, R04]

Daraus ergeben sich **statisch erkannte Verpackungsrisiken** beim direkten Build aus der eingebetteten Kopie: fehlender erwarteter Git-Index, fehlendes Ausführungsbit beziehungsweise eine unpassende Versionsherkunft. Das ist kein in der Quellenprüfung reproduzierter Buildfehler. Ein eigenständig korrekt ausgechecktes Upstream-Repository kann sich anders verhalten. Hier wurde weder ein neuer Clone installiert noch das CMake-Projekt verändert.

### 8.5 Audiokonfiguration und vorhandene Tests

CMake installiert die Lua-Datei weiterhin standardmäßig; dafür existiert die Option **`INSTALL_PIPEWIRE_CONF`**. Ob die auf dem Pi vorhandene WirePlumber-Version dieses Konfigurationsformat lädt, ist nicht geprüft. Kopiert bedeutet nicht angewendet. [R01]

Der geprüfte Unterbaum enthält `SoapySX/test/test.py`, `test_gains.py`, `test_linked_streams.py` und `test_timestamps.py` sowie Beispiele und Device-Tree-Quellen. Deren Vorhandensein ist **kein Testlauf**. Inhaltliche Vollprüfung und Hardwareausführung dieser Tests gehörten nicht zur Dokumentation. [R04]

### 8.6 Keine behauptete Umsetzung weiterer NetCore-Funktionen

In dieser Entwicklungsphase wurden keine NetCore-Rust-Dateien, kein Dashboard, kein Installer und keine systemd-Unit geändert. Die geprüften Dateien `sxxcvr-main/...` sind zusätzliche Repository-Befunde. Ihre Herkunft beziehungsweise ein konkreter Übernahme-PR für die GPIO-Änderung wurde nicht in der Git-Historie ermittelt; deshalb wird kein entsprechender Fix-Commit oder PR erfunden.

## 9. Korrekturregister: frühere Aussagen nicht ungeprüft übernehmen

| Frühere Aussage / Empfehlung | Gültige Einordnung |
|---|---|
| Trixie sei weiterhin Testing | **Falsch für den maßgeblichen Zeitraum.** Debian 13 wurde am 09.08.2025 veröffentlicht; zum Prüfdatum wird Trixie als Stable geführt. [W02] |
| Pi mit „x64“ sei ein x86-Server | **Falsch.** Die tatsächliche Ausgabe ist ARM64/AArch64. [H05–H06] |
| Frisches Raspberry Pi OS müsse Bookworm/v1 bedeuten | **Unbelegt.** Release und Paketversion müssen aus dem konkreten System ermittelt werden. |
| Debian 12/Bookworm liefere standardmäßig libgpiod v2 | **Falsch.** Die geprüfte Bookworm-Paketseite führt 1.6.3; Trixie führt 2.2.1. Paketrevisionen von zum Prüfdatum nicht in historische Logs zurückschreiben. [W03] |
| Der Paketname `libgpiod2` bedeute API-Version 2 | **Falsch.** Debian führt diesen Runtime-Paketnamen bei der 1.6.x-Reihe; Paket-/ABI-Suffix und API-Hauptversion sind nicht dasselbe. [W03] |
| Quellen auf Bookworm umschreiben und danach sei das System vollständig zurückgestuft | **Zurückgezogen.** Eine Änderung der Bezugsquellen ist kein konsistenter System-Downgrade; nicht erneut ausführen. |
| v1-Build ohne `--enable-bindings-cxx` genüge für SoapySX | **Falsch für diesen Treiber.** Der erfolgreiche C-only-Build löste die fehlende C++-Abhängigkeit nicht. [H06–H08] |
| CMake-Suchpfade oder pkg-config allein erzwingen immer Header und Linkage | **Zu pauschal.** Das Projekt muss diese Suchmechanismen benutzen; hier waren zwei CMake-Variablen ausdrücklich ungenutzt. [H09] |
| `PKG_CONFIG="pkg-config --static"` erzwinge statisches Linken | **Nicht nachgewiesen und kein verlässlicher Schalter für dieses Projekt.** Es ersetzt keine tatsächliche Archive-Auswahl und keinen Linkeraufruf. |
| Statische libgpiod-Archive könnten ohne weitere Prüfung in jedes Shared-Modul | **Unvollständiger Vorschlag.** PIC-Eignung, transitive Abhängigkeiten und Linkreihenfolge müssten geprüft werden. In der Planung kein erfolgreicher finaler Static-Link-Nachweis. |
| Keine gpiod-Ausgabe aus einer fehlgeschlagenen ldd-Pipeline beweise statisches Linken | **Falsch.** Erst Datei, Exitstatus und ELF-Abhängigkeiten prüfen; zum Prüfdatum kann libgpiod auch komplett entfallen sein. [H11, R01–R02] |
| Die `scoped_lock`-Warnung sei kosmetisch | **Falsch.** Lebensdauer und damit Schutzbereich des Lockobjekts sind relevant; am Prüfdatum vorliegende Stelle im Repository korrigiert. [R02, W07] |
| Die damaligen v2-Skizzen seien ein fertiger Minimalpatch | **Zurückgezogen.** `gpiod::request`, vereinfachte `request_lines(...)`-Aufrufe und untypisierte Integerwerte bilden keinen belastbaren vollständigen Patch. Die dokumentierte C++-API verwendet unter anderem `line_request`, `request_builder` und typisierte `line::value`-Werte. [W08] |
| Alle drei GPIOs könnten beim Portieren dieselben Open-Source-Einstellungen erhalten | **Nicht übernehmen.** Der vorhandene Treiber unterscheidet Reset von RX/TX; auch Defaultpegel und Resetzeiten müssen aus dem passenden Originalstand erhalten werden. [R02] |
| `--find` bedeute vollständig funktionierende Hardware | **Falsch.** Es ist Enumeration; der zusätzliche `findDevice()` prüft die tatsächliche Präsenz gerade noch nicht. [R02] |
| Der korrekt angezeigte Install-RPATH garantiere jede spätere Dienstumgebung | **Zu weitgehend.** ELF-Tag, direkte/transitive Abhängigkeiten und tatsächliche Dienstumgebung sind separat zu prüfen. [W06] |

Die Korrekturen verändern nicht nachträglich die dokumentierten Installationsschritte. Insbesondere waren fehlende Werkzeuge, der anfangs ausgelassene C++-Buildschalter und der falsche `ldd`-Dateiname Mängel der damaligen Anleitung, nicht ein belegter Bedienfehler.

## 10. Verworfene oder nicht bestätigte Ansätze

**Bookworm-Paketdowngrade beziehungsweise Quellenwechsel:** mehrfach vorgeschlagen und in Teilen mit APT versucht, aber nicht erfolgreich als Lösung belegt; riskante generische Quellenumschreibung zurückgezogen.

**Docker, nspawn, Chroot:** vorgeschlagene Alternativen, vom Benutzer nicht gewünscht; keine Ausführung dokumentiert. Sie werden nicht als weitere Projektanforderung oder Roadmap-Aufgabe übernommen.

**Eigene libgpiod-v2-Portierung:** nur Entwurfsskizzen, nicht als Ziel festgelegt. Kein Patch, Commit oder Test daraus entstanden. Der zum Prüfdatum vorhandene direkte Kernel-UAPI-Pfad ist davon ausdrücklich zu unterscheiden.

**Installation unter `$HOME/gpiod-v1` mit `LD_LIBRARY_PATH`:** früher Vorschlag, später durch `/opt/gpiod-v1` mit eingebettetem Laufzeitpfad ersetzt. Kein dauerhaft eingerichteter Wrapper oder Service-Environment nachgewiesen.

**Statische Variante:** C-only-Archiv zunächst tatsächlich erzeugt; spätere vollständige statische C++-Einbindung nur als Alternative besprochen, nicht nachgewiesen. Deshalb keine Behauptung „null Laufzeitabhängigkeiten“.

**Purge von `libgpiod-dev`/Runtime-Paketen, Holds und Löschen von `/opt/gpiod-v1`:** vorgeschlagene Eingriffe, ohne vollständigen Nachweis ihres endgültigen Zustands. Für die erfolgreiche lokale Headerauswahl war kein nachgewiesener System-Purge erforderlich. Aus dem Archiv keine neuen Löschbefehle ableiten.

## 11. Offene Aufgaben und Roadmap-Kandidaten

In den historischen Planungsunterlagen wurde **keine formale P0/P1-Priorisierung** vereinbart. Die folgende Reihenfolge ist ein Vorschlag aus der Quellenprüfung. Sie bleibt vollständig in diesem Archiv und ist kein bereits implementierter Installer oder genehmigter Codeänderungsauftrag.

| Reihenfolge | Aufgabe | Status / Abhängigkeit / Abnahmekriterium |
|---|---|---|
| 1 | Tatsächlich installierten SoapySX-Quell- und Modulstand sichern | **Offen.** Checkout-SHA, lokaler Diff, Modulhash und vorhandene Versionsausgabe erfassen. Alten externen Clone nicht mit `sxxcvr-main` gleichsetzen. |
| 2 | Laufzeitabhängigkeiten am richtigen Modul prüfen | **In der Planung vorgeschlagen, nicht getestet.** `ldd` und `readelf`; keine fehlenden Bibliotheken; bei altem Build konsistente v1-Header-/C++-/C-Zuordnung. Erst danach über Entfernen einer alten v1-Installation nachdenken. |
| 3 | Hardwareinitialisierung prüfen | **In der Planung vorgeschlagen, offen.** Exklusiver `--probe=driver=sx`; GPIO, SPI, Chipreset, Takt und ALSA erfolgreich öffnen. Fehlerausgabe vollständig sichern. |
| 4 | Rechte und Audiokonfiguration bei tatsächlichem Bedarf reparieren | **Bedingte Diagnoseidee.** Geräte-/Gruppenrechte und eingesetzte WirePlumber-Version feststellen. Ein root-Gegenversuch wäre nur Diagnose, keine automatische Dauerlösung. Keine in dieser Entwicklungsphase schon bestätigten Rechtefehler behaupten. |
| 5 | Native, reproduzierbare Installation für den zum Prüfdatum ausgewählten Treiberstand festlegen | **Roadmap-Kandidat aus geprüftem Abgleich.** Upstream-/eingebetteten Quellstand und Herkunft festlegen; Versionsskript, `.git/index`-Annahme und Dateimodus prüfen; sauberer ARM64-Build ohne unnötige Alt-Abhängigkeit. |
| 6 | Stream- und Funkabnahme, anschließend Reboot/Dauerbetrieb | **Noch nicht begonnen beziehungsweise nicht dokumentiert.** RX/TX-Streaming und gegebenenfalls NetCore-Tetra unter definierten Bedingungen testen. Ein Enumerationsresultat ersetzt diese Abnahme nicht. |
| 7 | Präzise Fehlerdiagnose für künftige Installationsanleitungen | **Roadmap-Kandidat.** Betriebssystem/Architektur zuerst prüfen; fehlende Tools erkennen; erster Fehler stoppt den Ablauf; Modulpfad aus Installation übernehmen; keinen falschen Static-Link-Erfolg aus `grep` erzeugen. |
| 8 | Upstream-Rückmeldung bei weiterbestehendem Problem | **Nur bei reproduzierbarem Befund sinnvoll.** Historische Fehlannahmen nicht als am Prüfdatum vorliegende Fehler melden; der am Prüfdatum vorliegende Code hat GPIO-Abhängigkeit und konkrete Lockstelle bereits geändert. |

Weitere kleine Nebenideen aus den Arbeitsnotizen waren ein CMake-Schalter für beide libgpiod-APIs, ein Startwrapper mit `LD_LIBRARY_PATH`, ein systemd-Drop-in und ALSA-/GPIO-Schnellchecks. Sie wurden nicht umgesetzt. Der Dual-API-Schalter ist durch den geprüften direkten UAPI-Pfad als Standardlösung überholt; Wrapper und Drop-in bleiben unbeauftragte Alternativen, keine offenen Muss-Anforderungen.

## 12. Quellen, Repository-Dateien, Branches und Commits

### 12.1 Historische Primärbelege

H01–H11 sind die in Abschnitt 5 beschriebenen Terminalprotokolle und festgelegten Installationsvorgaben. Die Entwurfsbehauptungen werden nur als Vorschläge oder zur Dokumentation des Korrekturbedarfs verwendet. Frühere in der Planung sichtbare Web-Zitiermarker ohne hier nachvollziehbaren Abruf werden nicht als überprüfte Quellen übernommen.

Historische Fremdquellen:

- SoapySX-Quelle: `https://github.com/tejeez/sxxcvr.git`; **damaliger Commit unbekannt**.
- libgpiod-Quelle: `https://git.kernel.org/pub/scm/libs/libgpiod/libgpiod.git`; Tag **v1.6.4**, im Terminalprotokoll genannter Commit **4a3c5d6c7ad524c8dd74f13c3ad064aa572b8037**.
- Keine historische NetCore-Codeänderung, kein dazu belegter NetCore-Commit und kein PR aus dieser Entwicklungsphase.

### 12.2 Zusätzliche Repository-Belege, unveränderlich auf Ausgangscommit referenziert

| Schlüssel | Datei / Quelle | Gegenstand |
|---|---|---|
| R00 | [Geprüfter Ausgangscommit](https://github.com/JanHG98/netcore-tetra/commit/972201a03bd1351aad5ed70fe9fc0d57d777b6bb) | Referenzstand von `Archiving` vor diesem Dokumentationslauf |
| R01 | [sxxcvr-main/SoapySX/CMakeLists.txt](https://github.com/JanHG98/netcore-tetra/blob/972201a03bd1351aad5ed70fe9fc0d57d777b6bb/sxxcvr-main/SoapySX/CMakeLists.txt) | Bibliotheken, Modulziel, Versionsgenerierung und WirePlumber-Installation |
| R02 | [sxxcvr-main/SoapySX/SoapySX.cpp](https://github.com/JanHG98/netcore-tetra/blob/972201a03bd1351aad5ed70fe9fc0d57d777b6bb/sxxcvr-main/SoapySX/SoapySX.cpp) | GPIO-UAPI, SPI/ALSA, Lock, Version und Enumeration |
| R03 | [sxxcvr-main/README.md](https://github.com/JanHG98/netcore-tetra/blob/972201a03bd1351aad5ed70fe9fc0d57d777b6bb/sxxcvr-main/README.md) | Am Prüfdatum vorliegende mitgeführte Abhängigkeiten und Probeaufruf |
| R04 | [sxxcvr-main-Tree](https://api.github.com/repos/JanHG98/netcore-tetra/git/trees/d5f1e6898b647fcc8ceaa9f1a89b1db165ca53c3?recursive=1) | Vollständig gelesener Unterbaum, Dateimodi und vorhandene Testdateien |
| R05 | [Bisheriger Archivindex](https://github.com/JanHG98/netcore-tetra/blob/972201a03bd1351aad5ed70fe9fc0d57d777b6bb/Docs/archive/README.md) | Erhalt bestehender Einträge; Abgrenzung anderer Projektentwurfs |

Zusätzliche Inhaltsidentifikatoren: R01 Blob `3b56ad4f4e2ad750b9490a7bb1895fc6e95d529c`; R02 Blob `7102014714bab6797a6e2a9c9e6f8bec75d398ac`; R03 Blob `1871c1a6d94adcaeacf8513a44515df2418e0f51`. Das sind **Git-Blob-SHAs**, keine erfundenen Fix-Commits.

### 12.3 Zusätzlich geprüfte öffentliche Quellen

Abruf und Einordnung jeweils **04.10.2026**; zusätzliche Dokumentation nicht mit einer archivierten historischen Fassung verwechseln.

- **W01 – SXceiver-Hersteller:** [Getting started](https://sxceiver.com/doc/getting-started). Am Prüfdatum vorliegende native Installationsanleitung; zusätzliche Paketliste ohne libgpiod-dev. Kein historischer Webseiten-Snapshot.
- **W02 – Debian:** [Trixie Release Information](https://www.debian.org/releases/trixie/). Veröffentlichung von Debian 13 am 09.08.2025 und zusätzliche Stable-Einordnung.
- **W03 – Debian-Pakete:** [Bookworm libgpiod-dev](https://packages.debian.org/bookworm/libgpiod-dev) und [Trixie libgpiod-dev](https://packages.debian.org/trixie/libgpiod-dev). API-Reihen 1.6.3 beziehungsweise 2.2.1; am Prüfdatum vorliegende Revisionssuffixe sind nicht die historischen Installationsstände.
- **W04 – CMake:** [CXXFLAGS](https://cmake.org/cmake/help/latest/envvar/CXXFLAGS.html) und [LDFLAGS](https://cmake.org/cmake/help/latest/envvar/LDFLAGS.html). Übernahme der Umgebungsflags bei der Konfiguration.
- **W05 – CMake:** [INSTALL_RPATH](https://cmake.org/cmake/help/latest/prop_tgt/INSTALL_RPATH.html). Unterschied zwischen Build- und Installationsruntimepfad.
- **W06 – Linux man-pages:** [ld.so(8)](https://man7.org/linux/man-pages/man8/ld.so.8.html). ELF-Suchreihenfolge und direkte versus transitive `DT_RUNPATH`-Wirkung.
- **W07 – C++-Arbeitsentwurf:** [Class template scoped_lock](https://eel.is/c++draft/thread.lock.scoped). Besitzdauer des Locks und Unlock im Destruktor.
- **W08 – libgpiod-Dokumentation:** [C++ GPIO line request, v2.3](https://libgpiod.readthedocs.io/en/v2.3/cpp_line_request.html) und [Bindings](https://libgpiod.readthedocs.io/en/master/bindings.html). API-Begriffe und Aktivierung der C++-Bindings; kein getesteter Ersatzpatch für den historischen Checkout. Die direkt angefragte v2.2.1-Dokumentations-URL und der kernel.org-Webabruf von `configure.ac` v1.6.4 waren nicht lesbar und werden nicht als erfolgreich geprüfte Belege ausgegeben.

## 13. Anhänge und Bilder

### 13.1 Einordnung der verfügbaren PDFs

Die Dateischnittstelle meldete **25 Dateien, sämtlich `source_kind=project`**, und keine eigenständigen Benutzer-Uploads dieser Planung. Alle 25 PDFs waren in der Arbeitsumgebung als Dateien zugänglich. Titelseiten und Seitenzahlen wurden direkt am Material geprüft. Die für die Buildfrage ausgeführte Dateisuche lieferte nur fachfremde TETRA-Normenstellen, keine tragfähige Installationsdiagnose.

Die Normen sind Projekt-Referenzmaterial zu TETRA, nicht Nachweise für Linux-Paketversionen, CMake-Konfiguration oder GPIO-Bibliothekskompatibilität. Sie werden hier inventarisiert, aber weder als vollständig geprüft noch als in dieser Entwicklungsphase implementierte Protokollfunktionen ausgegeben.

| Datei | Kennung / Gegenstand laut Titelseite | PDF-Seiten |
|---|---|---:|
| `en_3003920308v010401p.pdf` | EN 300 392-3-8 V1.4.1, 2020-04; Generic Speech Format Implementation | 22 |
| `en_30039209v010701p.pdf` | EN 300 392-9 V1.7.1, 2020-04; General requirements for supplementary services | 46 |
| `ts_10081201v020205p.pdf` | TS 100 812-1 V2.2.5, 2003-10; UICC physical and logical characteristics | 8 |
| `en_3003921201v010202p.pdf` | EN 300 392-12-1 V1.2.2, 2007-08; Call Identification, stage 3 | 56 |
| `en_3003920304v010301p.pdf` | EN 300 392-3-4 V1.3.1, 2010-08; ISI Short Data Service | 28 |
| `en_3003921117v010102p.pdf` | EN 300 392-11-17 V1.1.2, 2002-01; Include Call, stage 2 | 18 |
| `en_3003921114v010101p.pdf` | EN 300 392-11-14 V1.1.1, 2002-07; Late Entry, stage 2 | 23 |
| `es_20081202v020401m.pdf` | **Final draft** ES 200 812-2 V2.4.1, 2005-08; TSIM application characteristics | 139 |
| `es_20081201v020205p.pdf` | ES 200 812-1 V2.2.5, 2003-12; UICC physical and logical characteristics | 8 |
| `en_300812v020101p.pdf` | EN 300 812 V2.1.1, 2001-12; SIM-ME interface/security aspects | 156 |
| `en_3003921101v010201p.pdf` | EN 300 392-11-1 V1.2.1, 2004-01; Call Identification, stage 2 | 44 |
| `en_3003921006v010401p.pdf` | EN 300 392-10-6 V1.4.1, 2006-08; Call Authorized by Dispatcher, stage 1 | 20 |
| `en_3003921018v010301p.pdf` | EN 300 392-10-18 V1.3.1, 2003-10; Barring of Outgoing Calls, stage 1 | 17 |
| `en_3003921216v010400a.pdf` | **DRAFT** EN 300 392-12-16 V1.4.0, 2026-03; Pre-emptive Priority Call, stage 3 | 67 |
| `en_30039201v010601p.pdf` | EN 300 392-1 V1.6.1, 2020-04; General network design | 182 |
| `ets_30039214e01v.pdf` | **FINAL DRAFT prETS** 300 392-14, 1997-09; PICS proforma | 61 |
| `en_30039207v030501p.pdf` | EN 300 392-7 V3.5.1, 2019-07; Security | 216 |
| `en_30039401v030301p.pdf` | EN 300 394-1 V3.3.1, 2015-04; Radio conformance testing | 169 |
| `en_3003920313v010201p.pdf` | EN 300 392-3-13 V1.2.1, 2020-04; Transport-independent ISI Group Call | 191 |
| `en_30039502v010303p.pdf` | EN 300 395-2 V1.3.3, 2025-02; TETRA codec | 94 |
| `en_3003920303v010301p.pdf` | EN 300 392-3-3 V1.3.1, 2011-11; ISI Group Call | 251 |
| `en_30039205v020701p.pdf` | EN 300 392-5 V2.7.1, 2020-04; Peripheral Equipment Interface | 320 |
| `en_3003920315v010500a.pdf` | **Draft** EN 300 392-3-15 V1.5.0, 2026-04; Transport-independent ISI Mobility Management | 380 |
| `en_30039202v030801p.pdf` | EN 300 392-2 V3.8.1, 2016-08; Air Interface | 1445 |
| `ETSI.pdf` | 4100-seitige Sammeldatei; erste Norm/Titelseite EN 300 812 V2.1.1 | 4100 |

Der historische Umfang und die genaue Zusammenstellung von `ETSI.pdf` wurden nicht vollständig abgeglichen. Aus der Existenz der 2026er Entwürfe folgt nicht, dass sie dem mutmaßlich 2025 geführten Installationsgespräch schon vorlagen. Sie sind Teil des **zum Prüfdatum verfügbaren Projektkontexts**.

### 13.2 Bildauftrag

Im zugänglichen eigentlichen Installationsverlauf sind **keine eigenständigen Fotos, Screenshots oder erzeugten Bilder** vorhanden. Die geposteten Compiler- und Shellausgaben liegen als Text vor. Auch die Dateiinventur und die anfängliche Arbeitsverzeichnisprüfung lieferten nur die 25 PDFs, keine separaten Bilddateien dieser Planung.

Die automatisch dargestellten ETSI-Titelseiten und in den Normen enthaltenen Abbildungen werden nicht als historische Originalbilder umetikettiert. Deshalb gibt es für diese Planung **keine eigenständigen Bilder zum Hochladen**. Die allgemeinen Projekt-PDFs werden für diese thematisch begrenzte Builddokumentation nicht vollständig dupliziert. Eventuell außerhalb des zugänglichen Verlaufs vorhandene Originalbilder bleiben eine ausdrücklich benannte Quellenlücke; es werden weder Ersatzbilder erzeugt noch Bilddateien aus anderen Projektphasen als Originale ausgegeben.

## 14. Übergabe und Fortsetzungspunkt

Für die Fortsetzung zuerst feststellen, **welcher Treiber tatsächlich installiert ist**, dann **das richtige Modul mit `ldd`/`readelf` prüfen** und danach **`SoapySDRUtil --probe=driver=sx` bei freier Hardware** auswerten. Nicht erneut bei Bookworm-Quellen, Docker oder einer selbst geschriebenen libgpiod-Portierung beginnen.

Der historische Erfolg bleibt erhalten: **Der alte SoapySX-Build wurde auf Trixie/ARM64 mit einer separaten libgpiod-v1-Umgebung zum erfolgreichen Bauen und Enumerieren gebracht.** Der zusätzliche Repository-Befund bleibt getrennt: **Der mitgeführte Treiber benötigt diesen libgpiod-Pfad nicht mehr und enthält bereits die konkrete Lockkorrektur.** Ob dieser neuere Stand auf dem Ziel-Pi angekommen ist und dort korrekt sendet beziehungsweise empfängt, ist weiterhin unbestätigt.
