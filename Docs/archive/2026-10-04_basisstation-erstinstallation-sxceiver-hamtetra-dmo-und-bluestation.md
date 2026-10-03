# Abschlussdokumentation: SXceiver-Erstinstallation, HamTetra-DMO und Wechsel zu BlueStation

> **Historisches Chatarchiv – keine unmittelbar ausführbare Installationsanleitung.** Dieses Dokument bewahrt die tatsächlichen Beobachtungen, Benutzerentscheidungen und offenen Fragen des Chats. Mehrere damalige Assistentenantworten enthielten unbelegte oder falsche Hardwareannahmen, Befehle und Protokollerklärungen. Diese sind ausdrücklich zurückgezogen und dürfen nicht durch Übernahme in das Archiv zu vermeintlich bewährten Lösungen werden.

## 1. Metadaten und Auswertungsumfang

| Feld | Wert |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Erste Inbetriebnahme mit SXceiver, GPIO-/SPI-Zugriffsfehler, HamTetra-DMO-Repeater und anschließend vom Benutzer gemeldeter Installationserfolg mit tetra-bluestation |
| Archiv-ID | `basisstation-erstinstallation-sxceiver-hamtetra-dmo-und-bluestation` |
| Eröffnungsnachricht | „so, neuer Versuch der Basisstation“ |
| Ursprünglicher Chattitel | Nicht als verlässliches Metadatenfeld verfügbar. Die Eröffnungsnachricht dient nur zur Zuordnung, nicht als behaupteter Originaltitel. |
| Originaler Chatlink | Nicht verfügbar; kein Link rekonstruiert oder erfunden. |
| Historische zeitliche Einordnung | Die Terminalausgabe zeigt `Aug 20 22:52`, ohne Jahresangabe. Eine ergänzende Verlaufssuche ordnet den Einstieg dem 20.08.2025 zu. Das ist ein ergänzendes Retrieval-Metadatum, kein vollständiger datierter Chatexport. |
| Erstellung der Zusammenfassung | **2026-10-04**, Datumsbezug Europe/Berlin |
| Zielrepository | `JanHG98/netcore-tetra` |
| Ausschließlicher Schreibbranch | **`Archiving`** |
| Geprüfter Repository-Stand vor Archivierung | **`55f910820262fbe04ef7be0e586f9646c292bc32`** |
| Zugehöriger Root-Tree | `5edd71ab996547435f74712ecef89ab2921cdfd4` |
| Ablage dieser Datei | `Docs/archive/2026-10-04_basisstation-erstinstallation-sxceiver-hamtetra-dmo-und-bluestation.md` |
| Archivindex | `Docs/archive/README.md` |
| Archivierungscommit | Der tatsächliche Speichercommit wird durch die Git-Dateihistorie und die Abschlussmeldung ausgewiesen. Der oben genannte SHA ist ausdrücklich der **geprüfte Ausgangsstand**, nicht ein vorab behaupteter Archivierungscommit. |

### 1.1 Verwendete Evidenzebenen

- **H – Historischer Chat:** die hier zugänglichen Benutzer- und Assistentennachrichten einschließlich der eingefügten Terminalausgaben. Benutzerbestätigungen werden mit ihrer tatsächlichen Aussageweite erhalten.
- **R – Heutiges Zielrepository:** gezielt gelesene Dateien und Verzeichnisbäume des oben fixierten `Archiving`-Commits. Dateiinhalte belegen Quellcode oder Konfiguration, nicht deren Installation auf Jans Rechner.
- **U – Zusätzlich geprüfte Upstream-Quellen:** HamTetra/osmo-tetra-dmo und MidnightBlueLabs/tetra-bluestation. Diese erklären konkrete Fehler der damaligen Antworten, sind aber kein nachträglicher Beweis für die Version des historischen lokalen Builds.
- **A – Bereitgestellte Anhänge:** 25 ETSI-PDFs aus dem zugänglichen Projekt-/Dateikontext. Inventar und ausgewählte relevante Stellen wurden geprüft; keine vollständige Normenkonformitätsprüfung durchgeführt.

Die Quellenschlüssel werden in Abschnitt 15 aufgelöst. Quellen aus anderen NetCore-Chats werden nicht als Testergebnisse dieses Chats ausgegeben. Insbesondere gehören spätere Dual-Carrier-, Dashboard-, GPIO-Verkabelungs- oder Deployment-Arbeiten nicht automatisch zu diesem frühen Installationsversuch.

### 1.2 Grenzen der Auswertung

Der sichtbare Verlauf ist ausgewertet. Nicht verfügbar sind ein vollständiger Roh-Export mit allen Original-Metadaten, der externe DeepSeek-Dialog, die damals tatsächlich ausgeführten Installationsschritte für BlueStation, ein Abbild von `srv-tmo-bs01` sowie eventuell außerhalb des sichtbaren Verlaufs liegende Backups. Frühere generische Dateiverweise im Chat enthalten keine ausreichend identifizierbaren Backup-Inhalte. Daraus wird keine vermeintliche frühere Architektur rekonstruiert.

Es gab keinen Zugriff auf das laufende Funkgerät oder den Host, keine SSH-Sitzung zu Jan, keinen Senderstart und keinen heutigen Hardwaretest. Der Quellcodeabgleich erfolgte über den GitHub-Connector. Ein lokaler Cloneversuch scheiterte an der DNS-Auflösung der Arbeitsumgebung; dies ist kein Befund über Jans Installation und kein Ausfall des funktionierenden GitHub-Connectorzugriffs.

## 2. Ergebnis in einem Absatz

Jan wollte die **Softwareinstallation seiner SXceiver-basierten Funkstation** fortsetzen, nicht neue Hardware auswählen. Auf `jan@srv-tmo-bs01` lag unter `~/HamTetra` ein ausführbares `osmo-tetra-dmo/src/hamtetra_main2`. Zunächst scheiterte dessen Geräteinitialisierung am Zugriff auf `gpiochip0`, anschließend am Öffnen des SPI-Geräts. Jan bestätigte, dass der vorgeschlagene Gegenversuch als root läuft. Danach zeigte er Geräte mit Gruppenrechten für `gpio` und `spi` sowie entsprechende im Benutzerkonto eingetragene Gruppen; ein abschließender erfolgreicher Start ohne root ist im Chat jedoch nicht dokumentiert. Für den DMO-Versuch nannte er **419.99375 MHz, MCC 1, MNC 333 und GSSI 2000**. Die danach behaupteten DMO-DCC-Einstellungen und Zusatzflags waren unbelegt. Den abschließenden Installationserfolg meldete Jan nach Hilfe durch **DeepSeek**, das ihn auf **MidnightBlueLabs/tetra-bluestation** gebracht hatte. Welche Funktionen dieser erfolgreiche Aufbau tatsächlich erfüllte, bleibt mangels Konfiguration und Testprotokoll offen. [H01–H09]

## 3. Statusmodell und belastbarer Abschlussstand

In diesem Archiv bedeuten die Statusbegriffe:

| Status | Bedeutung |
|---|---|
| **Idee** | Erwähnter Vorschlag ohne nachgewiesene Beauftragung oder Umsetzung. |
| **Beschlossen/geplant** | Ausdrückliche Benutzerfestlegung oder autorisierter nächster Arbeitsschritt; noch kein Umsetzungsbeweis. |
| **Implementiert** | Konkretes Artefakt beziehungsweise Quellcode ist in einer benannten Quelle nachgewiesen. Eine nur vorgeschlagene Datei ist nicht implementiert. |
| **Getestet** | Ein bestimmter Versuch mit beobachtetem oder ausdrücklich berichteten Ergebnis ist dokumentiert. Die Grenzen des Versuchs bleiben sichtbar. |
| **Im Betrieb bestätigt** | Der Benutzer hat einen laufenden Zustand ausdrücklich gemeldet. Ohne Messdaten umfasst dies nur seine Aussage, nicht automatisch Dauerbetrieb oder alle Funktionen. |
| **Unbestätigt / zurückgezogen** | Nicht nachgewiesen beziehungsweise als falsche oder ungeprüfte Anleitung nicht weiterzuverwenden. |

| Gegenstand | Historischer Status | Nachweis und Grenze |
|---|---|---|
| SXceiver statt LimeSDR | **Beschlossen und als verwendete Hardware benannt** | Mehrfach ausdrücklich von Jan korrigiert. |
| Versionsmeldung `Hardware version 1.2` | **Getestet / Ausgabe vorhanden** | Geräteinitialisierung erreicht die Versionsausgabe; kein alleiniger Beweis für funktionierenden SPI- oder RF-Datenpfad. |
| HamTetra-Binary vorhanden und gestartet | **Implementiert auf dem damaligen Host / Startversuche getestet** | Pfad und Fehlerausgaben vorhanden. Buildherkunft und Commit unbekannt. |
| Zugriff als root | **Getestet nach Benutzerbericht** | „1 als root läuft“; keine vollständige Startausgabe oder Funkabnahme dazu. |
| Geräte `root:gpio` und `root:spi`, Modus `0660` | **Getestet / Ausgabe vorhanden** | Zustand zum Zeitpunkt von `ls -l` belegt. Persistenz nach Neustart nicht belegt. |
| Gruppeneinträge für `jan` | **Getestet / Ausgabe vorhanden** | `groups jan` belegt Konto-Gruppen, nicht sicher die Gruppen des schon laufenden Prozesses. |
| Erfolgreicher HamTetra-Start als normaler Benutzer nach Reparatur | **Unbestätigt** | Kein abschließendes Startprotokoll. |
| DMO-Gruppe mit MCC 1, MNC 333, GSSI 2000 | **Beschlossen/geplant** | Benutzerwerte; keine nachgewiesene Übernahme in Quellcode oder Geräteprogrammierung. |
| DMO-Repeaterfunktion auf 419.99375 MHz | **Gewünschter Versuch** | Mode-1-Aufruf besprochen, kein belegter Repeater-Funktionstest. |
| BlueStation installiert und zum Laufen gebracht | **Im Betrieb bestätigt, nur nach Benutzerbericht** | Externe Hilfe durch DeepSeek ausdrücklich benannt; Betriebsart, Revision und Leistungsumfang nicht dokumentiert. |
| Autostart, Health-Check, API-/SDS-/Voice-Integration | **Ideen des Assistenten** | Kein damaliger Auftrag beziehungsweise Erfolgsbeleg für die vorgeschlagenen Artefakte. |
| Moderne NetCore-Rust-Struktur und mitgeführter SoapySX-Treiber | **Heute im Zielrepository nachgewiesen** | Statischer Befund aus R02–R08, getrennt vom historischen Host. |

## 4. Historischer Ablauf mit Quellenankern

### H01 – Der Benutzer korrigiert die Aufgabenstellung

Auf „so, neuer Versuch der Basisstation“ antwortete der Assistent zunächst mit Hardwarekonzeption: Orange Pi/NUC, LimeSDR, Gehäuse, Versorgung und Zusatzfunktionen. Jan präzisierte nacheinander:

> „ne, die ganzen installation“
>
> „ich meine den ganzen spaß mit sxceiver und so“
>
> „ich nutze den sxceiver, kein Lime“
>
> „wir nutzen osmotetra“

**Festlegung:** Ausgangspunkt ist die vorhandene SXceiver-Hardware mit der tatsächlich verwendeten Software. Die Alternativhardware und fiktiven Node-Pakete waren kein Beschluss des Benutzers. Die spätere konkrete Ausgabe identifiziert HamTetra beziehungsweise dessen `osmo-tetra-dmo`-Teil genauer als die anfängliche Kurzbezeichnung „osmotetra“.

### H02 – Erster dokumentierter GPIO-Fehler

```text
jan@srv-tmo-bs01:~/HamTetra$ osmo-tetra-dmo/src/hamtetra_main2 sx 0
[INFO] Hardware version 1.2
SoapySDRDevice_make failed (0): cannot open GPIO device gpiochip0: Permission denied
```

Die Zeile ist als historische Eingabe erhalten, **nicht als empfohlener neuer Startbefehl**. Im heute überprüften Upstream-Parser wäre die `0` hier die Frequenz und nicht der Betriebsmodus; siehe Abschnitt 8.1. Der genaue historische Binary-Stand ist nicht nachgewiesen. [U03]

Der Assistent schlug root als Gegenprobe, Gruppenberechtigungen, udev-Regeln, neue Anmeldung und optional ACLs vor. Für diesen ersten GPIO-Fehler allein wurde kein vollständiges Abschlussprotokoll geliefert. Dass der nächste Versuch einen SPI-Fehler meldete, dokumentiert einen anderen sichtbaren Initialisierungsfehler, aber nicht die genaue dazwischen ausgeführte Reparaturfolge.

### H03 – Zweiter dokumentierter Fehler: SPI öffnen

```text
jan@srv-tmo-bs01:~/HamTetra$ osmo-tetra-dmo/src/hamtetra_main2 sx 419.99375 0
[INFO] Hardware version 1.2
SoapySDRDevice_make failed (0): Failed to open SPI
Done
```

Der Assistent schlug als ersten Gegenversuch denselben Aufruf mit `sudo` vor. Es folgten zahlreiche weitere Möglichkeiten, obwohl noch keine systematische Geräte-/Versionsprüfung vorlag: SPI aktivieren, Kernelmodule, alternative Chip-Selects, USB-SPI-Adapter, eine vermeintliche Umgebungsvariable und Python-Tests. Diese Möglichkeiten wurden nicht als tatsächlich ausgeführte Ursachenbehebung belegt.

### H04 – Root-Gegenprobe erfolgreich gemeldet

Jan antwortete:

> „1 als root läuft“

Gemeint war der erste vorgeschlagene Gegenversuch zum SPI-Fehler. Dies ist ein wertvoller diagnostischer Hinweis auf einen Berechtigungs- oder Ausführungskontextunterschied. Es rechtfertigt **keinen Neuaufbau der Hardware**, beweist aber auch nicht, dass nur eine bestimmte udev-Datei ursächlich war oder dass schon Sprachverkehr funktionierte.

### H05 – Geräte- und Gruppenzustand nach den Reparaturvorschlägen

```text
jan@srv-tmo-bs01:~/HamTetra$ ls -l /dev/spidev* /dev/gpiochip* 2>/dev/null
groups jan
crw-rw---- 1 root gpio 254, 0 Aug 20 22:52 /dev/gpiochip0
crw-rw---- 1 root gpio 254, 1 Aug 20 22:52 /dev/gpiochip1
crw-rw---- 1 root gpio 254, 2 Aug 20 22:52 /dev/gpiochip2
crw-rw---- 1 root gpio 254, 3 Aug 20 22:52 /dev/gpiochip3
crw-rw---- 1 root gpio 254, 4 Aug 20 22:52 /dev/gpiochip4
crw-rw---- 1 root spi  153, 1 Aug 20 22:52 /dev/spidev0.0
crw-rw---- 1 root spi  153, 2 Aug 20 22:52 /dev/spidev0.1
crw-rw---- 1 root spi  153, 0 Aug 20 22:52 /dev/spidev10.0
jan : jan adm dialout cdrom sudo audio video plugdev games users input render netdev gpio spi i2c
```

**Belegt:** Die aufgeführten Character Devices existieren; ihre Gruppen haben Lese-/Schreibrechte; das Benutzerkonto ist in `gpio`, `spi`, `audio`, `dialout` und `i2c` eingetragen.

**Nicht belegt:** Dass alle vorgeschlagenen udev-Dateien genau so angelegt wurden, welche Regel den Zustand erzeugte, ob eine neue Sitzung begonnen wurde, welche Gruppen der laufende Prozess tatsächlich hatte oder ob der nicht privilegierte Start anschließend erfolgreich war. Die damals folgende Aussage des Assistenten, weitere Fehler könnten jetzt nicht mehr an Rechten liegen, war deshalb zu weitgehend. [L01]

### H06 – Repeaterfrage und Benutzerparameter

Jan fragte, ob der Aufruf mit abschließender `1` für einen Repeater nicht zusätzlich MCC, MNC und GSSI benötigt. Danach nannte er:

```text
MCC 1 MNC 333 GSSI 2000
```

Der Assistent antwortete zunächst mit TMO-Zellparametern und einer erfundenen erweiterten CLI. Jan korrigierte:

> „das ist ne dmo gruppe“

**Gültige Benutzerfestlegung:** Die Kennungen gehören zum besprochenen DMO-Gruppenkontext. Daraus folgt weder eine TMO-Zellkonfiguration noch die Bedeutung von GSSI 2000 als Repeateradresse.

### H07 – Unbelegte DCC-Erklärung

Der Assistent erfand beziehungsweise behauptete für den konkreten Aufbau einen „DMO Colour Code“, Wertebereich 0–15, allgemeine Herstellervorgaben und CLI-/INI-Felder. Jan entgegnete:

> „dcc kenn ich nicht“

Es gab keine Konfigurationsansicht eines Endgeräts, keinen Normbeleg und keinen Parsernachweis für diese Aussagen. Die erneute Empfehlung, einfach `DCC = 0` zu setzen, wird ausdrücklich zurückgezogen. Der Benutzer hat keinen DCC gewählt oder bestätigt.

### H08 – Extern erreichter Installationserfolg

Jan berichtete:

> „deine konkurrenz hat es geschafft mir zu helfen es zum laufen zu bekommen. Deepseek hat das Repo https://github.com/MidnightBlueLabs/tetra-bluestation gefunden und mir beim installieren geholfen“

Dies ist die maßgebliche letzte Erfolgsmeldung. Der erfolgreiche Weg wurde **nicht durch die davor vorgeschlagenen erfundenen Befehle dieses Assistenten nachgewiesen**, sondern nach Benutzerangabe mit Hilfe von DeepSeek gefunden. BlueStation ist als tatsächlich herangezogenes Projekt zu bewahren. Der externe Installationsdialog selbst fehlt.

### H09 – Nur noch vorgeschlagene Folgeprüfungen

Der Assistent fragte anschließend nach mindestens etwa 30 Minuten Laufzeit, CPU-Last, Overflows/XRUNs, Endgeräteverhalten, Gruppenruf, Audio und Reichweite. Er schlug API-Anbindung, SDS/Voice, Autostart und Monitoring vor. Auf diese Fragen liegt keine weitere historische Antwort vor. Sie sind **keine bestandenen Tests** und keine vom Benutzer zugesagte Roadmap.

## 5. Endgültige Anforderungen und historische Parameter

### 5.1 Tatsächlich vom Benutzer vorgegeben

| Parameter / Anforderung | Historischer Wert | Bedeutung / Einschränkung |
|---|---|---|
| Aufgabe | Installation und Inbetriebnahme | Kein neuer Hardwareentwurf, kein Bookletauftrag. |
| RF-Hardware | SXceiver | LimeSDR ausdrücklich ausgeschlossen. |
| Gemeldete Hardwareversion | 1.2 | Aus Startausgabe; nicht als verifizierter kompletter Hardwareinventarsatz behandeln. |
| Benutzer / Host | `jan` / `srv-tmo-bs01` | Aus Shellprompt. Der Hostname beweist keine TMO-Betriebsart. |
| Arbeitsverzeichnis | `~/HamTetra` | Absoluter Pfad `/home/jan/HamTetra` war später im Assistentenbeispiel angenommen, nicht durch `pwd` belegt. |
| Tatsächlich gestartetes Programm | `osmo-tetra-dmo/src/hamtetra_main2` | Im Arbeitsverzeichnis aufgerufen. |
| Hardwareargument | `sx` | Im historischen Befehl und im geprüften Upstream vorhanden. |
| Besprochene Frequenz | **419.99375 MHz** | Entspricht 419993750 Hz; historische Vorgabe, keine Freigabe oder heutige Betriebsbestätigung. |
| Mode-Argumente | `0` bei dokumentiertem Fehler; `1` für besprochenen Repeater | Bedeutung im überprüften Upstream: Monitor / DMO-Repeater. Lokaler Versionsabgleich fehlt. |
| MCC | **1** | Vom Benutzer vorgegeben; nicht ersatzlos wegen DMO verwerfen. |
| MNC | **333** | Vom Benutzer vorgegeben. |
| GSSI | **2000** | Vom Benutzer ausdrücklich als DMO-Gruppe eingeordnet. |
| Erfolgreich verwendetes Folgeprojekt | `MidnightBlueLabs/tetra-bluestation` | Installationserfolg gemeldet; Revision, Konfiguration und Betriebsart nicht genannt. |

### 5.2 Nicht festgelegt oder nicht bestätigt

Keine belegte Betriebssystemversion, CPU-Architektur oder genaue Raspberry-Pi-Generation; keine bewiesene virtuelle Maschine oder Containerinstallation; keine vom Benutzer festgelegten `Cell ID`, `DCC`, manuellen Zeitschlitze, TX-Leistung, Hangtime oder Kryptoparameter. Die Assistentenannahme „Ubuntu x86_64, wie bei dir Standard“ war nicht durch diesen Chat gedeckt.

Es gibt keine belegte Zuordnung von `/dev/spidev0.1` oder `/dev/spidev10.0` zum SXceiver, keinen bestätigten USB-SPI-Adapter, keinen Nachweis einer seriellen Verbindung über `/dev/ttyUSB0` und keine festgelegte serielle Baudrate. Die Existenz mehrerer Geräteknoten allein bestimmt nicht deren elektrische Zuordnung.

Es wurde kein TMO-Uplink von 409.99375 MHz durch den Benutzer angefordert. Diese Zahl stammte aus der falschen Umdeutung des DMO-Versuchs. Die Frequenzwerte im Archiv ersetzen weder einen gültigen Betriebs-/Frequenzplan noch einen kontrollierten Testaufbau.

## 6. Architektur und Abhängigkeiten: drei getrennte Ebenen

### 6.1 Historisch direkt sichtbar

```text
Benutzer jan auf srv-tmo-bs01
  -> ~/HamTetra/osmo-tetra-dmo/src/hamtetra_main2
  -> Geräteanlage über SoapySDR (sichtbarer Fehlerpräfix)
  -> Zugriff auf GPIO / SPI
  -> SXceiver, gemeldete Hardwareversion 1.2
```

Diese Kette ist durch Prompt und Fehlerausgaben begrenzt belegt. Welche SoapySX-Version dynamisch geladen wurde, welche Bibliotheken verlinkt waren und welche Device-Tree-Konfiguration aktiv war, wurde nicht ermittelt.

### 6.2 Heute nachvollzogener HamTetra-Referenzaufbau

Der zusätzlich gelesene HamTetra-Snapshot bindet `libosmocore`, `osmo-tetra-dmo`, `suo` und `liquid-dsp` als Submodule ein. Das Hauptprogramm `hamtetra_main2.c` verbindet Protokoll-/Timinglogik mit `libsuo`, einem DPSK-Empfänger und einem PSK-Sender. Für `sx` wird SoapySDR mit dem Treiber `sx` verwendet. Dies ist ein softwaredefinierter Signalpfad und kein Beleg für ein Board, das selbständig ganze TETRA-Rufe, SDS, FEC und MAC abarbeitet. [U01–U03]

Der im aktuellen NetCore-Repository mitgeführte SoapySX-Treiber steuert SX1255-Register über SPI, GPIO-Leitungen über die Linux-GPIO-Character-Device-Schnittstelle und I/Q-Samples über I²S/ALSA. Die SoapySDR-Anwendung sieht komplexe Samples, nicht eine erfundene serielle SDS-CLI. Die bisherige Behauptung, mit SXceiver falle die SDR-Schicht weg, ist damit für den überprüften Aufbau nicht haltbar. [R06–R08]

### 6.3 BlueStation und heutiges NetCore

BlueStation wird im geprüften Upstream-README als experimenteller FOSS-TETRA-Stack mit Basisstations-Downlink, Anbindung passend programmierter Mobilgeräte, Gruppenanmeldung, teilweiser Sprachunterstützung und optionalem Brew beschrieben. Das README kennzeichnet den Stand als Alpha. Daraus lässt sich **keine automatische DMO-Repeaterfähigkeit** ableiten. [U05]

Das heutige Zielrepository enthält einen Rust-Workspace mit `bluestation-bs`, TETRA-Crates und zahlreichen Backend-Komponenten. Das ist ein späterer Projektzustand. Er darf nicht rückwirkend als damals bereits installierte NetCore-Node-Architektur ausgegeben werden. Ebenso sind moderne Dateien namens `security-core`, `sds-router` oder `mobility-core` allein kein Beleg für vollständige Protokollimplementierung oder Betriebsabnahme. [R02–R05]

## 7. Fehlerdiagnose und tatsächlich tragfähige Erkenntnisse

### 7.1 GPIO: explizite Zugriffsverweigerung

`Permission denied` beim Öffnen von `gpiochip0` ist der konkrete historische Fehler. Die erfolgreiche Versionsausgabe davor zeigt nur, dass ein früherer Initialisierungsschritt eine Versionsangabe liefern konnte. Im heute gelesenen SoapySX-Code wird die HAT-Information aus `/proc/device-tree/hat/product_id` und `product_ver` gelesen; dieser Zugriff ist vom späteren SPI-/GPIO-Zugriff getrennt. [H02; R07]

Die damalige Aussage, das Programm benutze zwingend `libgpiod`, war aus dem Log nicht nachweisbar. Der heute mitgeführte Code verwendet direkt Linux-GPIO-v2-ioctls. Ein älterer installierter Treiber kann anders aufgebaut gewesen sein. Diese Versionsgrenze ist relevant, weil die historische Fehlermeldung nicht wortgleich mit allen heutigen Fehlerpfaden ist. [R07; R08]

### 7.2 SPI: Öffnen ist nicht Datenübertragung

Der aktuelle Treiber wirft `Failed to open SPI`, wenn `open(spidev_path, O_RDWR)` einen Fehler liefert. Ein fehlgeschlagener SPI-Transfer hat einen separaten Fehlertext. Der gezeigte Fehler belegt somit im überprüften Code einen Fehler beim **Öffnen**, nicht automatisch einen verdrahteten, aber elektrisch defekten Bus. Der Code enthält an dieser Stelle weiterhin einen TODO für eine genauere Fehlerausgabe. [R07]

Jans root-Gegenprobe macht Rechte beziehungsweise Ausführungskontext zum vorrangigen Prüfpfad. Sie rechtfertigt keine Schlussfolgerung, dass ein USB-Adapter gekauft, ein anderes SPI-Gerät ausprobiert oder eine Frequenz geändert werden müsse. Ein Vergleich der tatsächlich geladenen Bibliothek und der Prozessgruppen bleibt sinnvoll, weil root und Benutzer nicht zwingend in jedem Punkt dieselbe Umgebung verwenden.

### 7.3 Warum die letzte Gruppenliste den Fehler nicht abschließend ausschließt

`groups jan` fragt die für das Konto gespeicherten Gruppen ab. Die Gruppen einer bereits laufenden Sitzung können davon abweichen. Für deren Prüfung sind `id` oder `groups` **ohne Benutzerargument** maßgeblich. Eine frische Anmeldung und ein danach dokumentierter Start waren im Chat nicht mehr zu sehen. [L01]

Deshalb lautet der Abschluss nicht „alle Rechte behoben und Nicht-root-Betrieb getestet“, sondern: **Dateirechte und Kontogruppen sahen passend aus; abschließender Prozess- und Startnachweis fehlt.**

### 7.4 Historisch vorgeschlagener Reparaturablauf und seine Grenzen

Die folgenden Schritte wurden vom Assistenten vorgeschlagen; ihre vollständige Ausführung ist nicht als Befehlsprotokoll vorhanden:

```bash
sudo groupadd -f spi
sudo groupadd -f gpio
sudo usermod -aG spi,gpio,dialout jan
```

Danach sollten udev-Regeln für `spidev*` und `gpiochip*` mit Gruppe `spi` beziehungsweise `gpio` und Modus `0660` angelegt sowie udev neu geladen werden. Frühe Varianten nannten zusätzlich `i2c`. Die spätere Ausgabe ist mit einem erfolgreichen Setzen solcher Rechte vereinbar, beweist aber nicht, welche Variante angewendet wurde.

Die seinerzeit ergänzte rekursive Änderung unter `/sys/class/gpio` wird **nicht als bewährte Lösung übernommen**. Sie ist kein präziser Nachweis oder Ersatz für die Berechtigung der hier betroffenen `/dev/gpiochip*`-Knoten. Vorhandene Regeln müssen bei einer Fortsetzung zuerst gelesen werden; die drei vorgeschlagenen Dateinamen dürfen nicht unbesehen zusätzlich übereinandergestapelt werden.

Vorgeschlagene Regeldateien waren:

| Pfad | Historischer Zweck | Status |
|---|---|---|
| `/etc/udev/rules.d/60-gpio.rules` | GPIO-Gruppenzugriff | Anlage/Inhalt nicht nachgewiesen. |
| `/etc/udev/rules.d/60-spi.rules` | SPI-Gruppenzugriff | Anlage/Inhalt nicht nachgewiesen. |
| `/etc/udev/rules.d/60-spi-gpio.rules` | Kombinierte spätere Variante | Anlage/Inhalt nicht nachgewiesen; nicht automatisch parallel zu den Einzeldateien verwenden. |

`newgrp spi` und `newgrp gpio` wurden ebenfalls vorgeschlagen, aber nicht mit anschließendem `id` und Startprotokoll belegt. Eine temporäre ACL mittels `setfacl` blieb ein Vorschlag. Ein Reboot-/Persistenztest fehlt.

### 7.5 Sinnvoller nächster Diagnoseumfang – neu empfohlen, nicht ausgeführt

Zunächst nur Informationen erheben, ohne die Funkhardware aktiv zu initialisieren:

```bash
# In einer frisch angemeldeten Sitzung von jan:
id
cat /etc/os-release
uname -m
pwd
ls -l /dev/gpiochip* /dev/spidev* 2>/dev/null

# Im tatsächlich vorhandenen HamTetra-Verzeichnis:
git rev-parse HEAD
git status --short
git submodule status
```

Danach vorhandene udev-Regeln, den wirklichen Servicebenutzer, die geladene SoapySX-Bibliothek und deren Quellstand gezielt vergleichen. Vor einer Veröffentlichung von Diagnosedaten Zugangsdaten und sensible Konfigurationsteile entfernen. Ein SoapySDR-`--probe` ist bereits eine Geräteinitialisierung und wird nicht mit einer rein lesenden Dateiauflistung gleichgesetzt.

## 8. Heutiger Abgleich der HamTetra-Befehle

> **Referenz, nicht historischer Versionsnachweis:** Geprüft wurde `rats-ry/HamTetra` bei `d0eddfc3bec3b65b2bcc63734d84f24a7ff77c9f` und dessen eingetragener Submodulstand `tejeez/osmo-tetra-dmo` bei `c630336d118075d6bba736be928ece39447decd3`. Es ist nicht bewiesen, dass Jans lokales Binary aus genau diesen Ständen gebaut wurde. [U01–U04]

### 8.1 Nachgewiesene Argumente und besonders kritischer Einheitenfehler

Der gelesene Parser erwartet:

```text
hamtetra_main2 HARDWARE FREQUENCY [MODE]
HARDWARE sx = SXceiver
MODE 0 = DMO monitor
MODE 1 = DMO repeater; zugleich Standard ohne MODE
FREQUENCY = TETRA-Signalmittenfrequenz in MHz
```

Der Parser multipliziert die Frequenzeingabe intern mit `1e6`. Für diese Version bedeutet daher:

| Historisch besprochene Form | Bedeutung im geprüften Quellstand |
|---|---|
| `hamtetra_main2 sx 419.99375 0` | SXceiver, 419.99375 MHz, DMO-Monitor. |
| `hamtetra_main2 sx 419.99375 1` | SXceiver, 419.99375 MHz, DMO-Repeater. Potenziell sendender Betrieb. |
| `hamtetra_main2 sx 0` | Frequenz 0 MHz und Standardmodus 1; **nicht** „SXceiver im Monitorbetrieb“. Nicht erneut so verwenden. |
| Frequenzargument `419.99375e6` | Falsche Größenordnung für diesen MHz-Parser, da intern nochmals mit `1e6` multipliziert wird. |

Zusatzargumente wie `--mcc`, `--mnc`, `--gssi`, `--cellid`, `--dmo-dcc` oder `--txpower` werden in dem gelesenen `main()` nicht als solche ausgewertet. Der Code liest die ersten Positionsargumente; angehängte vermeintliche Optionen schaffen keine zusätzliche Konfiguration. Das ist problematischer als ein garantiert sichtbarer Syntaxfehler, weil eine erfundene Option möglicherweise ohne Wirkung bleibt. [U03]

Ein Aufruf **ohne Argumente** gibt in dieser Version die Hilfe aus und endet vor der Hardwareinitialisierung. Auch dies ist vor einer Verwendung an einem anderen lokalen Fork gegen dessen Source zu prüfen.

### 8.2 MCC/MNC und Repeateradresse existieren im Referenzcode

Die Datei `src/hamtetra_config.h` enthält im geprüften Upstream:

```c
#define REP_MCC 244
#define REP_MNC 2
#define REP_ADDRESS 1099
#define DN232 3
#define DN233 1
#define DN253 2
#define DT254 2
```

Diese Werte sind **Upstream-Konstanten, keine Benutzerwerte und keine Empfehlung für Jans Aufbau**. Insbesondere ist `REP_ADDRESS` eine Repeateradressierung mit einem im Kommentar beschriebenen 10-Bit-Luftschnittstellenfeld, nicht die GSSI einer Rufgruppe. GSSI 2000 darf deshalb nicht einfach als Repeateradresse übernommen werden. [U04]

Der Befund widerlegt die damalige pauschale Aussage, MCC/MNC hätten bei diesem DMO-Repeater grundsätzlich keine Rolle. Er beantwortet noch nicht vollständig, wie Jans gewünschte Gruppe im konkreten lokalen Fork behandelt, gefiltert oder weitergeleitet wurde. Dazu wären dessen Gruppen-/PDU-Pfade und die Endgeräteprogrammierung zu prüfen. Ein neuer Patch der Konstanten wurde in diesem Archivierungsauftrag nicht durchgeführt.

### 8.3 Signalpfad und Offset im Referenzcode

Der `sx`-Zweig setzt unter anderem `driver=sx`, Zeitstempelverwendung, RX-/TX-Antennenbezeichner und eine Sample-Rate von 150000 Samples/s. Er verwendet einen internen Offset von 25000 Hz zwischen SDR-Mitte und TETRA-Signal; RX- und TX-Mitte werden dafür gemeinsam aus der gewünschten Frequenz abgeleitet. Dieser **DSP-Offset ist kein TMO-Duplexabstand**. Die dort mit `TBD` kommentierten Gainwerte sind keine im Chat gemessene Ausgangsleistung in dBm. [U03]

### 8.4 Tatsächlich vorhandener Installationspfad, nachträglich wiedergefunden

Das geprüfte HamTetra-README beschreibt für SXceiver auf Raspberry Pi OS die Installation von SoapySX und danach den Build mehrerer Submodule. Es nennt unter anderem:

```text
git submodule init
git submodule update
install/suo_dependencies.sh
install/build_liquiddsp.sh
install/build_suo.sh
install/osmo_dependencies.sh
install/build_osmocore.sh
install/build_osmotetra.sh
```

Das sind reale Referenzpfade im gefundenen HamTetra-Projekt und ein deutlich belastbarerer Ausgangspunkt als die zuvor erfundene `sxceiver`-CLI. Das README verwendet teilweise die historische Repository-Adresse `OH2NXX/HamTetra` und nennt Raspberry Pi OS 12 als dort getestete Umgebung. Weder diese README-Testangabe noch die vorhandenen Skriptnamen sind ein Beweis für Jans Betriebssystem, die damals ausgeführte Installationsfolge oder einen heute erfolgreichen Neuaufbau. [U01; U02]

## 9. Korrekturregister: nicht als Lösung weiterverwenden

| Frühere Aussage oder Vorschlag | Bewertung bei Archivierung | Konsequenz für die Fortsetzung |
|---|---|---|
| Orange Pi/NUC/LimeSDR als vorhandenes beziehungsweise gewähltes Setup | Vom Benutzer nicht vorgegeben; Lime ausdrücklich korrigiert. | SXceiver und tatsächlichen Host inventarisieren. |
| Ubuntu x86_64 sei Jans Standard für diesen Aufbau | Im Chat nicht belegt. | OS und Architektur erheben, nicht aus früheren Assistentenannahmen übernehmen. |
| SXceiver sei bereits ein vollständiges TETRA-natives Modem; die SDR-Schicht entfalle | Für den überprüften SoapySX-/HamTetra-Pfad falsch. | SPI/GPIO/I²S, SoapySDR und Host-Signalverarbeitung getrennt betrachten. |
| `github.com/netcore-tetra/sxceiver.git` sowie `sxceiver`, `sxceiver-cli` als fertige verwendete Software | Nicht durch einen Build, einen Dateifund oder einen tatsächlichen Repositorybezug dieses Chats belegt. | Nicht als Abhängigkeit, Dienst oder Installationsweg ausgeben. |
| `/dev/ttyUSB0`, 115200 Baud als SXceiver-Anbindung | Erfundenes Beispiel statt Bestandsaufnahme. | Keine serielle Schnittstelle unterstellen. |
| Stock-osmo-tetra liefere die gezeigte Komplett-BS mit `tetra-mux`, `tetra-mgr`, `osmo-tetra-sds` und `osmo-tetra-voice` | Im Chat nicht verifiziert; kein solcher lokaler Aufruf gezeigt. | Die erfundene Toolchain und zugehörigen Konfigurationen nicht weiterverwenden. |
| Mode `1` sei zugleich TMO-Basisstation mit Einbuchung, Cell-ID und Duplexpaar | Widerspricht dem geprüften DMO-Parser. | DMO-Repeater und TMO-Basisstation als verschiedene Betriebsziele dokumentieren. |
| Für DMO seien MCC/MNC grundsätzlich irrelevant | Für den geprüften Repeatercode falsch beziehungsweise unzulässig pauschal. | Benutzerwerte erhalten und ihre jeweilige Funktion prüfen. |
| DMO benötige hier zwingend einen „DCC 0–15“, Herstellerstandard 0 oder Beispiel 7 | Ohne Norm-, Codeplug- oder Parserbeleg behauptet. | Diese Konfiguration vollständig zurückziehen; keine DCC-Werte nachtragen. |
| Manuell `TS1` und `--slot 1` seien passende Pflichtparameter | Kein Nachweis eines solchen CLI-Feldes; DMO-Timing nicht auf dieses Beispiel reduzieren. | Betriebs-/Repeaterverfahren im realen Stack und Endgerät prüfen. |
| Erweiterte CLI mit `--mcc`, `--dmo-gssi`, `--dmo-dcc`, `--txpower` usw. | Im geprüften `main()` nicht implementiert. | Keine unverifizierten Flags an einen Funkstart anhängen. |
| `419.99375e6` passe als Frequenz in den gezeigten HamTetra-Aufruf | Beim geprüften MHz-Parser falsch. | Einheiten aus dem wirklichen Parser übernehmen. |
| 419.99375/409.99375 MHz seien das notwendige DMO-Duplexpaar | Aus einer TMO-Verwechslung entstanden; zusätzlich im Text mit widersprüchlichem Vorzeichen erklärt. | Nicht Teil der bestätigten DMO-Anforderung. |
| `scrambling = off` sei eine generische Einstellung für einen unverschlüsselten Test | Kein Konfigurationsvertrag nachgewiesen; Scrambling und Verschlüsselung nicht gleichsetzen. | Keine solchen Felder in reale Konfigurationen übertragen. |
| Weitere SPI-Fehler könnten nach `groups jan` nicht mehr an Rechten liegen | Prozessgruppen wurden nicht geprüft. | `id` in frischer Sitzung und tatsächlichen Prozesskontext prüfen. |
| `SXCVR_SPI` wähle den SPI-Pfad | Im gelesenen aktuellen SoapySX-Konstruktor nicht verwendet; der Pfad ist dort fest hinterlegt. | Kein wirkungsloses Exportieren oder blindes Durchprobieren als Lösung verkaufen. |
| Durchprobieren aller SPI-Geräte sei ein geeigneter automatischer Health-Check | Keine elektrische Zuordnung oder gefahrlose Initialisierungsfolge belegt. | Ein Health-Check muss bekannte Geräte prüfen, nicht unbekannte Peripherie umkonfigurieren. |
| `SupplementaryGroups=gpio,dialout` sei die passende systemd-Schreibweise | Kommaseparierte Aufzählung nicht als korrektes Gruppenlistenbeispiel übernehmen. | Benötigte Gruppen als getrennte Listeneinträge, z. B. durch Leerzeichen, und gegen die reale Unit prüfen. |
| BlueStation sei pauschal ein fertiger DMO-Repeater-/BS-Ersatz | Das geprüfte README beschreibt Basisstationsfunktionen und Alpha-Grenzen, keinen solchen umfassenden Nachweis. | Den von Jan gemeldeten Erfolg anerkennen, aber dessen Modus und Testumfang nicht hinzuerfinden. |
| Bestimmte BlueStation-Funktionen wie Auth, Handover oder SDS seien sämtlich nicht vorhanden | Kein umfassender damaliger oder heutiger Feature-Audit durchgeführt. | Funktionsaussagen nur anhand konkreter Revisionen und Tests treffen. |

Die in den frühen Antworten genannten fiktiven INI-/YAML-Beispiele werden nicht als gültige Konfiguration wiederabgedruckt. Ihre wesentlichen falschen Annahmen und Dateinamen bleiben zur Wiedererkennung in diesem Register und im Pfadverzeichnis erhalten.

## 10. Getrennter heutiger Stand im Zielrepository

### 10.1 Prüfmethode

Alle folgenden R-Befunde beziehen sich auf `Archiving` bei `55f910820262fbe04ef7be0e586f9646c292bc32`. Geprüft wurden Root-/ausgewählte Unterverzeichnisbäume und die in Abschnitt 15 genannten Dateien, nicht jedes Modul des gesamten Projekts. Es wurden **keine** Cargo-Tests, Builds, CI-Läufe oder On-Air-Tests ausgeführt. Die nachfolgende Bestandsaufnahme ist deshalb keine Release- oder Betriebsfreigabe.

### 10.2 Nachgewiesene aktuelle Komponenten

| Komponente | Beobachtung im Quellstand | Abgrenzung |
|---|---|---|
| Rust-Workspace | Core-/Config-/PDU-/Entity-Crates, `bins/bluestation-bs` und zahlreiche Backend-Pakete eingetragen. | Nicht die frühere lose Node-/osmotetra-Pseudokonfiguration. |
| Basisstationsprogramm | Paket und Binary `bluestation-bs`, Einstieg `src/main.rs`. | Nicht `sxceiver`, `tetra-mgr` oder `hamtetra_main2`. |
| Build-Funktionen | Im Paket manifestierte Standardfeatures `asterisk`, `recording`, `audio-player`. | Nicht der Nachweis, mit welchen Features eine installierte Binary gebaut wurde. |
| SoapySX | C++-Treiber unter `sxxcvr-main/SoapySX/SoapySX.cpp`. | Mitgeführter Quellstand; geladene Bibliothek auf Jans Rechner unbekannt. |
| Treiberbeispiele/-tests | Python-Beispiele und Tests für Stream-/Timestamp-/Gain-Themen im SoapySX-Baum vorhanden. | Keine Ausführung oder Testabnahme dokumentiert. |
| Updateablauf | `install/update-basisstation.sh` vorhanden. | Spezialisierter aktueller NetCore-Updateweg, keine nachträgliche Bestätigung eines historischen HamTetra-Installers. |
| Projekt-README | Nennt NetCore v1.9.0, Warnfunktionen und eine zentrale SIP-Anbindung mit lokalem Fallback. | Dokumentierte spätere Projektbeschreibung, nicht in diesem Chat nachgewiesener Live-Zustand. |

Quellen: [R01–R08].

### 10.3 Aktuelle RF-/Netzkonfiguration ist nicht die historische DMO-Konfiguration

Aus der gelesenen `config.toml` wurden nur die für diesen Vergleich relevanten, nicht geheimen Werte übernommen:

```toml
config_version = "0.6"
stack_mode = "Bs"
service_name = "tetra"

[phy_io]
backend = "SoapySdr"

[phy_io.soapysdr]
tx_freq = 418000000
rx_freq = 408000000
sample_rate = 600000
tx_center_freq = 418012500
rx_center_freq = 408012500

[net_info]
mcc = 901
mnc = 1510

[cell_info]
freq_band = 4
main_carrier = 720
secondary_carrier = 721
duplex_spacing = 0
freq_offset = 0
reverse_operation = false
location_area = 1
colour_code = 1
timezone = "Europe/Berlin"
```

**Dies ist ein dokumentierender Auszug, keine vollständige oder zu übernehmende Betriebsdatei.** Er belegt einen heutigen BS-/Dual-Carrier-Konfigurationskontext mit anderen Netzkennungen und Frequenzen. Die damals genannten DMO-Werte `1 / 333 / 2000` bleiben in ihrem historischen Zusammenhang bestehen. Die beiden Kontexte dürfen nicht über dieselbe ungeprüfte Startzeile vermischt werden. [R05]

In der gelesenen Datei gibt es außerdem veraltete beziehungsweise widersprüchliche Inline-Kommentare: Der Kommentar bei `main_carrier` passt nicht zum dortigen Zahlenwert, und der Kommentar zu `duplex_spacing` spricht von 5 MHz, während die expliziten TX-/RX-Werte 10 MHz auseinanderliegen. Dieses Archiv bewertet nicht abschließend die aktuelle Ableitungslogik; es markiert den notwendigen späteren Abgleich zwischen Frequenzwerten, ausgestrahlten Zellparametern und Kommentaren. Änderungen daran wären außerhalb dieses Auftrags.

Die optionale explizite Geräteauswahl ist in dem betrachteten Konfigurationsabschnitt nur als kommentiertes Beispiel zu sehen. Aus diesem Auszug folgt also nicht, dass dort bereits aktiv `device = "driver=sx"` gesetzt ist. `colour_code` der BS-Konfiguration ist kein Nachweis für die damalige behauptete DMO-DCC-Option.

### 10.4 SoapySX: konkrete Pfade und Verhalten

Im gelesenen Konstruktor sind fest angegeben:

| Funktion | Wert im aktuellen Treiber |
|---|---|
| SPI-Gerät | `/dev/spidev0.0` |
| GPIO-Gerät | `/dev/gpiochip0` |
| Reset-GPIO | 5 |
| RX-GPIO | Für Hardware 1.0: 13; sonst 23 |
| TX-GPIO | Für Hardware 1.0: 12; sonst 22 |
| ALSA RX | `hw:CARD=SX1255,DEV=1` |
| ALSA TX | `hw:CARD=SX1255,DEV=0` |
| Soapy-Streamformat im betrachteten Setup | `CF32` |
| HAT-Identifikation | `/proc/device-tree/hat/product_id`, `/proc/device-tree/hat/product_ver` |

Die GPIO-Zahlen sind hier **Quellcodewerte**, kein in diesem Chat elektrisch geprüfter Pinplan. Insbesondere die Zuordnung von Linux-GPIO-Chips zu einem bestimmten Pi-/Kernelstand muss zum tatsächlichen Gerät passen. [R07; R08]

Der Konstruktor ignoriert seine Geräteargumente mit `(void)args`; eine Auswertung der damals vorgeschlagenen Variable `SXCVR_SPI` ist dort nicht vorhanden. Der SPI-Pfad kann in diesem Code folglich nicht allein durch die vorgeschlagenen Shell-Exports auf `.1` oder `10.0` umgestellt werden. Der SPI-Open-Fehler enthält weiterhin keine aussagekräftige `errno`-/Pfadangabe. Dies ist ein konkreter kleiner Roadmap-Kandidat, nicht ein im Archiv behobener Bug.

Das SoapySX-README beschreibt Build und Probe über `SoapySDRUtil --probe=driver=sx`. EEPROM-Schreiben wird dort für bestimmte unbeschriebene Prototypen erwähnt. Aus Jans vorhandener Versionsmeldung ergibt sich **kein Auftrag**, das EEPROM vorsorglich neu zu schreiben. [R06]

### 10.5 Aktueller NetCore-Updater: bereits vorhandene Teilantwort auf frühere Ideen

`install/update-basisstation.sh` enthält einen realen Ablauf für die heutige Basisstation:

1. Systemd-/Werkzeugprüfung und Ermittlung des Buildbenutzers; Cargo wird bevorzugt in dessen Rustup-Umgebung gefunden, nicht nur in roots `PATH`.
2. Unit-Ermittlung aus Konfiguration beziehungsweise bekannten Namen und Ermittlung der tatsächlich gestarteten `bluestation-bs`-Binary über Prozess-/Unitinformationen.
3. Aufruf zweier benannter Config-Parser-Tests und eines Releasebuilds für `bluestation-bs`.
4. Konfigurationsmigration unter Erhalt von Eigentümer, Gruppe und Modus sowie Lesbarkeitsprüfung für den Servicebenutzer.
5. Binarybackup, gezielter Austausch, Neustart und Status-/Logprüfung mit Rollbackpfad.

Der Scriptstandard für die Konfiguration ist `/etc/netcore/config.toml`; Backups liegen unter `/var/backups/netcore-tetra`. Als Unit-Kandidaten erscheinen `tetra.service`, `bluestation.service`, `tetra-bluestation.service` und `bluestation-bs.service`. Die Konfiguration dieses Snapshots nennt `service_name = "tetra"`. Kein dieser Namen ist dadurch als damals tatsächlich laufende Unit auf `srv-tmo-bs01` belegt. [R05; R09]

Die im Script stehenden Tests `media_library_top_level_section_parses` und `media_library_unknown_field_is_rejected` wurden bei der Archivierung **nicht ausgeführt**. Gleiches gilt für den Build. Der Updater enthält weitere TTS-/Piper-spezifische Migrationen und ist deshalb kein neutraler universeller Reparaturbefehl für den alten DMO-Aufbau. Er zeigt aber, dass „Autoupdate/Backup/Health-Prüfung“ im heutigen Projekt nicht pauschal als völlig neu zu bauende Idee behandelt werden sollte.

## 11. Befehls-, Datei- und Schnittstellenregister

### 11.1 Historische Befehle mit Ausführungsstatus

| Befehl / Ablauf | Status | Ergebnis oder Einschränkung |
|---|---|---|
| `osmo-tetra-dmo/src/hamtetra_main2 sx 0` | **Tatsächlich im Chat ausgeführt** | GPIO-Zugriff verweigert; Argumentbedeutung problematisch, nicht wiederholen. |
| `osmo-tetra-dmo/src/hamtetra_main2 sx 419.99375 0` | **Tatsächlich im Chat ausgeführt** | `Failed to open SPI`, danach `Done`. |
| Gegenprobe mit `sudo` für den vorherigen Start | **Erfolg vom Benutzer bestätigt** | Läuft als root; exakte Ausgabe und Funktionsumfang fehlen. |
| `ls -l /dev/spidev* /dev/gpiochip* 2>/dev/null` | **Tatsächlich ausgeführt** | Geräteknoten und Rechte in H05 vollständig bewahrt. |
| `groups jan` | **Tatsächlich ausgeführt** | Kontogruppenliste in H05. |
| Start mit abschließender `1` | **Besprochen** | Repeaterfrage, kein zugehöriges Erfolgs-/Fehlerprotokoll. |
| `groupadd`, `usermod`, udev-Regeln und Reload/Trigger | **Vorgeschlagen; genauer Ablauf unbestätigt** | Spätere Rechte passen zum Zielzustand, aber kein lückenloses Reparaturprotokoll. |
| `newgrp`, erneute Anmeldung, Reboot | **Vorgeschlagen** | Kein belegt erfolgreicher Folgestart aus neuer Sitzung. |
| ACL mittels `setfacl` | **Vorgeschlagen** | Nicht als dauerhafte oder tatsächlich angewendete Lösung belegt. |
| `modprobe spidev`, Pi-`dtparam=spi=on` | **Vorgeschlagen** | Kein fehlendes SPI-Gerät als Ursache nachgewiesen; root-Erfolg spricht gegen vorschnellen Neuaufbau. |
| `gpiodetect`, `gpioinfo`, `lsmod`, `dmesg`, `lsusb` | **Angefordert/vorgeschlagen** | Keine zugehörigen Ausgaben im Chat. |
| Python-/periphery-Tests | **Vorgeschlagen** | Nicht ausgeführt belegt; damaliger Paketname/Installationsweg nicht verifiziert. |
| `export SXCVR_SPI=...` | **Unbelegt, nicht als Lösung übernehmen** | Im überprüften SoapySX-Pfad keine solche Auswahl nachgewiesen. |
| `systemctl enable --now hamtetra.service` | **Vorgeschlagen** | Weder Unitdatei noch Servicezustand tatsächlich vorgelegt. |
| BlueStation-Installation durch DeepSeek-Anleitung | **Erfolg gemeldet** | Die genauen Befehle fehlen und werden nicht nacherfunden. |

### 11.2 Historische und heutige Pfade strikt unterscheiden

| Pfad / Name | Einordnung |
|---|---|
| `~/HamTetra/osmo-tetra-dmo/src/hamtetra_main2` | Historisch tatsächlich verwendeter relativer Programmpfad. |
| `/dev/gpiochip0` bis `/dev/gpiochip4` | Historisch aufgelistete Geräte. Nur die konkrete benötigte Zuordnung ist zu prüfen. |
| `/dev/spidev0.0`, `/dev/spidev0.1`, `/dev/spidev10.0` | Historisch aufgelistete Geräte; nicht gleichbedeutend mit drei geeigneten SXceiver-Anschlüssen. |
| `/etc/systemd/system/hamtetra.service` | Damals nur vorgeschlagen. |
| `/etc/sxceiver/config.yaml`, `hamtetra.conf`, `cell.cfg`, `devices.conf` | Frühe unbelegte Beispielkonfigurationen, kein Bestandsnachweis. |
| `sxceiver.service`, `osmotetra.service`, Dienst `node` | Vorgeschlagene Namen aus unzureichend belegten Architekturen; keine tatsächlich nachgewiesenen Dienste. |
| `/var/log/syslog`, `journalctl -u ...` | Allgemeine damalige Diagnosevorschläge; keine hier vorgelegten Servicejournal-Auszüge. |
| `sxxcvr-main/SoapySX/SoapySX.cpp` | Heutiger gezielt geprüfter Treiber im Zielrepository. |
| `sxxcvr-main/SoapySX/test/` und `sxxcvr-main/example/` | Heutige Test-/Beispielartefakte, nicht in diesem Auftrag gestartet. |
| `bins/bluestation-bs/`, `crates/`, `config.toml` | Heutige NetCore-Struktur, nicht der historische HamTetra-Baum. |
| `/etc/netcore/config.toml`, `/var/backups/netcore-tetra` | Pfadstandards im heute geprüften Updater, nicht automatisch Live-Pfade. |
| `/etc/flowstation/config.toml`, `/usr/bin/bluestation-bs` | Im gelesenen Paketmanifest enthaltene Paketpfade; nicht mit dem Updaterstandard gleichsetzen. |

### 11.3 Ports, Protokolle und Zugriffskontexte

Historisch ist **kein tatsächlich belegter TCP-/UDP-Dienstport** dokumentiert. Die frühere WebUI-Adresse mit Port `8443` war ein Assistentenbeispiel, kein gefundenes Interface. Es gibt keinen Nachweis eines REST-Endpunkts, eines Managementsockets oder eines aktiven VPN-Tunnels dieses Aufbaus.

Belegt beziehungsweise im heutigen Code nachvollzogen sind Linux-Character-Device-Zugriffe, SPI, GPIO, SoapySDR sowie im überprüften Treiber ALSA/I²S für komplexe Samples. DMO-Repeaterbetrieb und TMO-Basisstationsbetrieb sind getrennte Luftschnittstellen-/Betriebskontexte. Brew-, SIP-, REST- oder Monitoring-Anbindungen dürfen nur mit ihren jeweils vorhandenen späteren Quellen beschrieben werden, nicht aus den damaligen Versprechen abgeleitet werden.

## 12. Tests, Ergebnisse und fehlende Abnahmen

| Test / Nachweis | Durchgeführt? | Ergebnis | Nicht abgedeckt |
|---|---|---|---|
| Programmstart als `jan`, erster dokumentierter Aufruf | Ja, Terminalausgabe | GPIO-`Permission denied`. | Erfolgreicher RF-Pfad. |
| Programmstart als `jan`, mit Frequenz und Mode 0 | Ja, Terminalausgabe | SPI-Open-Fehler. | Elektrische Busfunktion, RF, Audio. |
| Root-Gegenprobe | Nach Benutzerbericht ja | Läuft. | Dauer, verwendete Frequenz tatsächlich am Ausgang, DMO-Repeater, SDS/Voice. |
| Geräte-/Kontogruppenliste | Ja, Terminalausgabe | Geräte vorhanden, Gruppenrechte gesetzt. | Prozessgruppen, Persistenz und späterer Nicht-root-Start. |
| BlueStation-Inbetriebnahme | Nach Benutzerbericht ja | Zum Laufen gebracht. | Installationsrezept, Revision, DMO/TMO, Endgerätemodelle und Ende-zu-Ende-Funktionen. |
| Gruppenruf GSSI 2000 | Nicht belegt | Offen. | Kein Sprach-/Signalisierungsprotokoll vorhanden. |
| SDS Senden/Empfangen | Nicht belegt | Offen. | Weder konkrete gültige CLI noch Empfangsquittung dokumentiert. |
| Mehr als 30 Minuten stabiler Betrieb | Nur vorgeschlagen | Offen. | Keine CPU-/Underrun-/Abbruchdaten. |
| Reboot/Autostart ohne root | Nicht belegt | Offen. | Keine Unit-/Journal-/Gruppenprüfung. |
| Reichweite, Antennenabstimmung, Ausgangsleistung | Nicht belegt | Offen. | Keine Messungen und kein HF-Aufbau dokumentiert. |
| Heutiger statischer Codeabgleich | Ja, gezielte Dateien | Argumente, Treiberpfade, aktuelle BS-Konfiguration und Updater geprüft. | Kein vollständiger Feature-/Security-Audit. |
| Heutige Cargo-/Treiber-/Hardwaretests | Nein | Nicht Bestandteil des Archivierungslaufs. | Keine technische Abnahme aus dem Speichern der Dokumentation ableiten. |

Für eine spätere Abnahme sind Modus, Source-Commit, Binary-Hash, Treiberversion, OS/Kernel, Endgerätekonfiguration und Erfolgskriterium zusammen zu protokollieren. DMO-Repeaterdurchleitung und TMO-Einbuchung sind unterschiedliche Testfälle. Ein „Programm läuft“ darf nicht als Nachweis beider Fälle gelten.

## 13. Ideen, verworfene Ansätze und Roadmap-Kandidaten

### 13.1 Relevante noch offene Arbeit

Die folgenden Prioritäten sind **neu vorgeschlagene Arbeitsreihenfolge für die Fortsetzung**, keine im historischen Chat bereits vereinbarte Termin- oder Prioritätenplanung. Es werden hier keine Issues, PRs, Codeänderungen oder Roadmapdateien außerhalb des Archivs angelegt.

| ID | Priorität | Aufgabe | Ursprung / Status | Abhängigkeit und Abschlusskriterium |
|---|---|---|---|---|
| SX-01 | P0 | Historisch beziehungsweise aktuell tatsächlich funktionierende Installation sichern: Repository, Commit, Submodule, Binary-/Treiber-Hashes, OS, Startpfad. | Auswertungslücke; **offen**. | Nur bereinigte Konfiguration ohne Zugangsdaten archivieren; rekonstruierbarer Buildstand liegt vor. |
| SX-02 | P0 | Betriebsziel eindeutig trennen: alter HamTetra-DMO-Repeater oder heutige NetCore-TMO-Basisstation. | Benutzerkorrektur DMO versus späterer BlueStation-Erfolg; **offen**. | Benannte Binary, Modus und passender Endgerätekanal dokumentiert. |
| SX-03 | P0 | Nicht-root-Zugriff abschließen: echte Prozessgruppen, aktive Regeln und Gerätezuordnung prüfen. | Früherer Fehler; **teilweise beobachtet, nicht abschließend getestet**. | Start als vorgesehener Dienstbenutzer nach frischer Anmeldung und Neustart belegt. |
| SX-04 | P0 | Benutzerwerte 1/333/2000 richtig auf DMO-Netz-/Gruppen-/Repeaterparameter abbilden, sofern der alte DMO-Pfad weiter genutzt wird. | Explizite Benutzerwerte; **beschlossen, Umsetzung unbestätigt**. | Gruppenidentität und Repeateradresse getrennt; keine fiktiven Flags oder DCC-Felder. |
| SX-05 | P1 | Reproduzierbare SXceiver-Installation mit dokumentierten realen Abhängigkeiten und Versionen. | Installeridee; reale Upstream-Referenzen heute gefunden. | Neuaufbau aus festgehaltenem Stand getestet; keine Vermischung mehrerer SoapySDR-/Treiberinstallationen. |
| SX-06 | P1 | Fehlertexte des SoapySX-Open-Pfads um betroffenen Gerätepfad und Systemfehler ergänzen. | Konkreter heutiger Codebefund; **Idee**. | Fehlertests unterscheiden fehlendes Gerät, Zugriffsverweigerung und Transferfehler nachvollziehbar. |
| SX-07 | P1 | Sichere Preflight-/Health-Prüfung bekannter Devices und Bibliotheken. | Damaliger kleiner Health-Check-Vorschlag; **Idee**. | Keine automatische Initialisierung unbekannter SPI-Geräte; klare Diagnose ohne unbeabsichtigten Senderstart. |
| SX-08 | P1 | Reale Unit, Arbeitsverzeichnis, Benutzer/Gruppen, Neustartverhalten und Journalausgabe dokumentieren. | Autostartvorschlag; **unbestätigt**. | Kein dauerhaft privilegierter Betrieb nur als ungeprüfter Ersatz für Rechtebehebung; Neustart-/Fehlerfalltest protokolliert. |
| SX-09 | P1 | Getrennte DMO-/TMO-Testmatrix mit Gruppenruf, Audio, gegebenenfalls SDS, Dauerlast und Wiederanlauf. | Frühere Testvorschläge; **offen**. | Ergebnisse mit genauer Revision und Endgeräten statt pauschalem „läuft“. |
| SX-10 | P1 | Heutige Frequenz-/Zellparameter und widersprüchliche Konfigurationskommentare abgleichen. | Heutiger R05-Befund; **offen**. | Parser-/Broadcastableitung und freigegebener Frequenzplan stimmen überein; nicht im Archivauftrag verändern. |
| SX-11 | P2 | APIs, SDS/Voice, Monitoring, Logging und Backup mit vorhandenen NetCore-Komponenten zusammenführen. | Assistentenideen; heute teilweise entsprechende Artefakte vorhanden. | Bestand vor Neubau prüfen; Ende-zu-Ende-Funktion und Recovery nachweisen. |
| SX-12 | P2 | Den erfolgreichen externen BlueStation-Installationsweg nachdokumentieren. | Benutzererfolg, Rezept fehlt. | Freigegebene Befehlsfolge, Konfiguration und Revision ohne Geheimnisse als zusätzlicher Beleg vorhanden. |

SX-04 ist nur dann erneut als Implementierungsaufgabe nötig, wenn der historische DMO-Pfad tatsächlich weiterverfolgt wird. Die Archivierung soll keinen heute erfolgreichen TMO-Aufbau auf alte Versuche zurücksetzen. Die damaligen Gerätezugriffsprobleme sind als historisch unvollständig abgeschlossen zu kennzeichnen, nicht ohne aktuellen Fehlerbeleg als heute fortbestehende Störung.

### 13.2 Kleinere Nebenideen, die nicht verloren gehen sollen

Der Assistent bot automatische Installer unter Namen wie `setup-node.sh`, `setup-sxceiver.sh` und `setup-osmotetra.sh`, ein Level-0/1-Installationshandbuch beziehungsweise A4-Booklet, einen GPIO-/SPI-Health-Check, eine systemd-Unit, Logrotation, Temperatur-/TX-Last-Telemetrie, Konfigurationsbackup und einen Deploymentbericht an. **Keines dieser Artefakte wurde in diesem Chat tatsächlich geliefert oder vom Benutzer abschließend beauftragt.** Heute vorhandene ähnliche Funktionen sind zuerst zu prüfen, bevor diese Ideen als neue Entwicklungsarbeit übernommen werden. [H01–H09; R09]

Die API-Idee umfasste SDS/Voice aus einer App oder Zentrale; dazu kamen Vorschläge für SNR/BER-/Temperatur-/TX-Last-Anzeigen. Es wurden keine konkreten Messdatenquellen, APIs oder Portverträge erarbeitet. Auch der Vorschlag, BlueStation genauer auf Schwachstellen zu untersuchen, wurde hier nicht als vollständiger Audit ausgeführt.

### 13.3 Aus dem Auftrag herausgefallene oder ersetzte Ansätze

Die anfänglichen Ideen zu Orange Pi/NUC, LimeSDR/LimeSuite, LTE/5G, GPS, SSD, modularen Nodes, 19-Zoll-/Outdoor-Gehäusen, IP65, Lüfter-/Notabschaltung, 230-V-USV, 12/24-V-/Solarbetrieb und Mesh-VPN waren **unaufgeforderte Konzeptvorschläge im falschen Aufgabenverständnis**. Sie sind keine Hardware-BOM und keine Anforderungen dieses Chats.

Gleiches gilt für vermeintliche ZKN-/Lighthouse-Registrierung, Hardware-Fingerprint, Whitelist, TLS-Verteilung, Honeypot, vorgeschlagenes Secure Boot und feste Partitionsgrößen. Diese Themen wurden nicht ausgearbeitet und nicht als Bestandteil der erfolgreichen Inbetriebnahme belegt. Eine spätere separate Verwendung solcher Ideen ist möglich, darf aber nicht als damaliger Beschluss dargestellt werden.

Der Wechsel zu BlueStation ist der letzte benutzerseitig gemeldete praktische Lösungsweg. Das historische HamTetra-Verzeichnis muss deshalb nicht gelöscht oder erneut aufgebaut werden. Vor Änderungen an einer funktionierenden Installation steht deren Sicherung und eindeutige Identifikation.

## 14. Anhänge und Bildarchiv

### 14.1 Verfügbare Dateien und tatsächliche Sichtung

Im zugänglichen Projekt-/Dateikontext liegen **25 PDFs mit insgesamt 8061 physischen PDF-Seiten und 55265473 Bytes**. Die Seitenzahl enthält Überschneidungen zwischen Einzeldokumenten und `ETSI.pdf` und bezeichnet **keinen Umfang einmaliger oder vollständig fachlich ausgewerteter Normenseiten**.

Dateinamen, Seitenzahl, Deckblattkennung und SHA-256 wurden erfasst. Inhaltlich wurden für diesen Chat besonders die Identitäts-/Gruppenkontexte und die DMO-Gruppentabelle in `es_20081202v020401m.pdf`, Seiten 99–100, geprüft, einschließlich gerenderter Tabellen-/Seitenansicht. Dort sind Netzwerkadressverweis und GSSI getrennte Einträge. Dies unterstützt die vorsichtige Behandlung der Benutzerkennungen; es liefert keine HamTetra-CLI und keinen Nachweis einer auf dem Host implementierten TSIM-Funktion. [A07]

Die bereitgestellten Dateien sind als Projektquellen zugänglich. Ihre heutige Verfügbarkeit beweist nicht, dass jede Datei schon während des historischen Installationsversuchs vorhanden war. Insbesondere werden die Dokumente mit Draft-Kennung 2026 nicht rückwirkend als damalige Implementierungsgrundlage ausgegeben. Eine Prüfung, ob jede Ausgabe heute die neueste oder endgültig verabschiedete Fassung ist, war nicht Gegenstand dieser Archivierung.

### 14.2 Inhaltsinventar

| ID | Datei | Kennung / Stand laut Dokument | Themenbezug | Seiten |
|---|---|---|---|---:|
| A01 | `en_3003920308v010401p.pdf` | EN 300 392-3-8 V1.4.1, 2020-04 | ISI Generic Speech Format Implementation | 22 |
| A02 | `en_30039209v010701p.pdf` | EN 300 392-9 V1.7.1, 2020-04 | Allgemeine Anforderungen an Zusatzdienste | 46 |
| A03 | `ts_10081201v020205p.pdf` | TS 100 812-1 V2.2.5, 2003-10 | UICC, physikalische/logische Eigenschaften | 8 |
| A04 | `en_3003921201v010202p.pdf` | EN 300 392-12-1 V1.2.2, 2007-08 | Call Identification, Stage 3 | 56 |
| A05 | `en_3003920304v010301p.pdf` | EN 300 392-3-4 V1.3.1, 2010-08 | ISI Short Data Service | 28 |
| A06 | `en_3003921117v010102p.pdf` | EN 300 392-11-17 V1.1.2, 2002-01 | Include Call, Stage 2 | 18 |
| A07 | `es_20081202v020401m.pdf` | **Final draft** ES 200 812-2 V2.4.1, 2005-08 | TSIM-Anwendung; DMO-Gruppen auf S. 99–100 gezielt betrachtet | 139 |
| A08 | `en_3003921114v010101p.pdf` | EN 300 392-11-14 V1.1.1, 2002-07 | Late Entry, Stage 2 | 23 |
| A09 | `es_20081201v020205p.pdf` | ES 200 812-1 V2.2.5, 2003-12 | UICC, physikalische/logische Eigenschaften | 8 |
| A10 | `en_300812v020101p.pdf` | EN 300 812 V2.1.1, 2001-12 | SIM-ME-Schnittstelle, Sicherheitsaspekte | 156 |
| A11 | `en_3003921101v010201p.pdf` | EN 300 392-11-1 V1.2.1, 2004-01 | Call Identification, Stage 2 | 44 |
| A12 | `en_3003921006v010401p.pdf` | EN 300 392-10-6 V1.4.1, 2006-08 | Call Authorized by Dispatcher | 20 |
| A13 | `en_3003921018v010301p.pdf` | EN 300 392-10-18 V1.3.1, 2003-10 | Barring of Outgoing Calls | 17 |
| A14 | `en_3003921216v010400a.pdf` | **DRAFT** EN 300 392-12-16 V1.4.0, 2026-03 | Pre-emptive Priority Call, Stage 3 | 67 |
| A15 | `en_30039201v010601p.pdf` | EN 300 392-1 V1.6.1, 2020-04 | General network design; Adressierung und Identitäten | 182 |
| A16 | `ets_30039214e01v.pdf` | **Final draft** prETS 300 392-14, 1997-09 | PICS-Proforma; keine ausgefüllte NetCore-Konformitätserklärung | 61 |
| A17 | `en_30039207v030501p.pdf` | EN 300 392-7 V3.5.1, 2019-07 | Security | 216 |
| A18 | `en_30039401v030301p.pdf` | EN 300 394-1 V3.3.1, 2015-04 | Funk-Konformitätsprüfung; keine Messberichte dieses Aufbaus | 169 |
| A19 | `en_3003920313v010201p.pdf` | EN 300 392-3-13 V1.2.1, 2020-04 | Transportunabhängiger ISI-Gruppenruf | 191 |
| A20 | `en_30039502v010303p.pdf` | EN 300 395-2 V1.3.3, 2025-02 | TETRA-Sprachcodec | 94 |
| A21 | `en_3003920303v010301p.pdf` | EN 300 392-3-3 V1.3.1, 2011-11 | ISI Group Call | 251 |
| A22 | `en_30039205v020701p.pdf` | EN 300 392-5 V2.7.1, 2020-04 | Peripheral Equipment Interface, V+D und DMO | 320 |
| A23 | `en_3003920315v010500a.pdf` | **Draft** EN 300 392-3-15 V1.5.0, 2026-04 | Transportunabhängiges ISI Mobility Management | 380 |
| A24 | `en_30039202v030801p.pdf` | EN 300 392-2 V3.8.1, 2016-08 | V+D Air Interface | 1445 |
| A25 | `ETSI.pdf` | Sammeldatei; erstes Deckblatt EN 300 812 V2.1.1 | Überlappender umfangreicher Normenbestand; Zusammensetzung nicht vollständig abgeglichen | 4100 |

Die bloße Bereitstellung dieser Normen begründet keine zusätzlichen Benutzeranforderungen an ISI, TSIM, PPC, PEI oder alle Zusatzdienste. Kein Dokument ersetzt die Prüfung des tatsächlich ausgeführten Codes.

### 14.3 Prüfsummen zur späteren eindeutigen Zuordnung

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

### 14.4 Bilder: Ergebnis der Verfügbarkeitsprüfung

Im sichtbaren historischen Chat sind keine eigenständigen Aufbau-, Terminal- oder generierten Entwurfsbilder enthalten. Die Suche nach eigenständigen Bilddateien auf der verfügbaren Dateifläche sowie die Prüfung der gemounteten Dateien ergaben **keine PNG-, JPEG-, WebP- oder GIF-Chatbilder**.

Deshalb wird kein leerer Bildordner angelegt und kein Ersatzbild erzeugt. Die eingebetteten ETSI-Deckblätter und Normgrafiken werden nicht als Fotos dieses Aufbaus umetikettiert. Die PDFs wurden als Quellen inventarisiert, nicht erneut als angebliche Bildanhänge in das Repository kopiert. Sollten außerhalb des zugänglichen Verlaufs doch Originalbilder existieren, müssten diese später ausdrücklich identifiziert und unter demselben Archivbereich ergänzt werden; für deren Vorhandensein gibt es hier keinen Nachweis.

## 15. Quellen, Dateinachweise und weiterführende Zuordnung

### 15.1 Historische Quellen

**H01–H09** bezeichnen die in Abschnitt 4 erhaltenen Ereignisse und Benutzeräußerungen dieses Chats. Der Chatlink ist nicht verfügbar. Der dortige Wechsel von allgemeinen Assistentenvorschlägen zu tatsächlichen Benutzerlogs ist absichtlich sichtbar gehalten.

### 15.2 Zielrepository – fixierter Prüfstand

Alle R-Links zeigen auf denselben geprüften Commit, nicht auf eine bewegliche Branchansicht.

| Schlüssel | Quelle | Geprüfter Inhalt |
|---|---|---|
| R01 | [Repository-Prüfstand](https://github.com/JanHG98/netcore-tetra/tree/55f910820262fbe04ef7be0e586f9646c292bc32) | Rootstruktur und gezielte Unterbäume. |
| R02 | [README.md](https://github.com/JanHG98/netcore-tetra/blob/55f910820262fbe04ef7be0e586f9646c292bc32/README.md) | Vollständig gelesene kurze Projektbeschreibung. |
| R03 | [Cargo.toml](https://github.com/JanHG98/netcore-tetra/blob/55f910820262fbe04ef7be0e586f9646c292bc32/Cargo.toml#L1-L115) | Workspace und Paketzuordnung; nicht alle weiteren Manifestabschnitte. |
| R04 | [bins/bluestation-bs/Cargo.toml](https://github.com/JanHG98/netcore-tetra/blob/55f910820262fbe04ef7be0e586f9646c292bc32/bins/bluestation-bs/Cargo.toml#L1-L135) | Programmname, Standardfeatures und betrachtete Paketpfade. |
| R05 | [config.toml](https://github.com/JanHG98/netcore-tetra/blob/55f910820262fbe04ef7be0e586f9646c292bc32/config.toml#L1-L200) | Betriebsart, RF-/Netz-/Zellwerte, Fallbackbeschreibung und Kommentarwidersprüche. |
| R06 | [sxxcvr-main/README.md](https://github.com/JanHG98/netcore-tetra/blob/55f910820262fbe04ef7be0e586f9646c292bc32/sxxcvr-main/README.md) | Tatsächlicher SoapySX-Build-/Probehinweis. |
| R07 | [SoapySX.cpp, Anfang und Gerätezugriff](https://github.com/JanHG98/netcore-tetra/blob/55f910820262fbe04ef7be0e586f9646c292bc32/sxxcvr-main/SoapySX/SoapySX.cpp#L1-L290) | HAT-ID, SPI-Open-/Transferfehler, Linux-GPIO-Zugriff. |
| R08 | [SoapySX.cpp, ALSA und Konstruktor](https://github.com/JanHG98/netcore-tetra/blob/55f910820262fbe04ef7be0e586f9646c292bc32/sxxcvr-main/SoapySX/SoapySX.cpp#L330-L850) | Gelesene Teilbereiche 330–610 und 650–850; feste Gerätepfade, I/Q-Stream, Argumentbehandlung. |
| R09 | [install/update-basisstation.sh](https://github.com/JanHG98/netcore-tetra/blob/55f910820262fbe04ef7be0e586f9646c292bc32/install/update-basisstation.sh) | Vollständig gelesener Updater; keine Ausführung. |
| R10 | [Bestehender Archivindex vor diesem Eintrag](https://github.com/JanHG98/netcore-tetra/blob/55f910820262fbe04ef7be0e586f9646c292bc32/Docs/archive/README.md) | Vorhandene 21 Archiveinträge; zu erhalten, nicht neu zu bewerten oder zu überschreiben. |

Zusätzliche Blob-Identifikatoren der wichtigsten gelesenen Dateien:

```text
Cargo.toml                                      fb22c40ed26b08778f7b256b0aad87588ad6574f
bins/bluestation-bs/Cargo.toml                   db65cf2714bcfda02987b3e5e0fea81837afb073
config.toml                                     3f54ad275b956699dd7b38de1b7d2c0711133c00
sxxcvr-main/SoapySX/SoapySX.cpp                   7102014714bab6797a6e2a9c9e6f8bec75d398ac
install/update-basisstation.sh                   16083c6868fd3b7f16e41f50b6c84704cd5b9a64
Docs/archive/README.md vor diesem Archiveintrag  def5491853451fd7e700e87f864ccfdd84c9c493
```

### 15.3 Zusätzlich geprüfte Upstream-/Werkzeugquellen

| Schlüssel | Quelle | Aussageweite |
|---|---|---|
| U01 | [rats-ry/HamTetra README bei d0eddfc3…](https://github.com/rats-ry/HamTetra/blob/d0eddfc3bec3b65b2bcc63734d84f24a7ff77c9f/README.md) | SXceiver-/HamTetra-Referenzinstallation, Submodule und Beschränkung anderer Konfiguration auf Sourceänderungen. Nicht Jans belegter lokaler Commit. |
| U02 | [HamTetra .gitmodules](https://github.com/rats-ry/HamTetra/blob/d0eddfc3bec3b65b2bcc63734d84f24a7ff77c9f/.gitmodules) | Tatsächliche Abhängigkeitsquellen. Der zugehörige `osmo-tetra-dmo`-Gitlink wurde auf `c630336d118075d6bba736be928ece39447decd3` geprüft. |
| U03 | [hamtetra_main2.c bei c630336d…](https://github.com/tejeez/osmo-tetra-dmo/blob/c630336d118075d6bba736be928ece39447decd3/src/hamtetra_main2.c) | Gelesener vollständiger Hauptprogrammtext: Positionsargumente, MHz-Umrechnung, Monitor-/Repeatermodus und SX-Signalpfad. |
| U04 | [hamtetra_config.h](https://github.com/tejeez/osmo-tetra-dmo/blob/c630336d118075d6bba736be928ece39447decd3/src/hamtetra_config.h) und [hamtetra_mac.c](https://github.com/tejeez/osmo-tetra-dmo/blob/c630336d118075d6bba736be928ece39447decd3/src/hamtetra_mac.c#L1-L200) | Vollständiger betrachteter Konfigurationsheader und erster MAC-Abschnitt; Repeaterkennungen und DMO-Präsenzlogik. Kein vollständiger Gruppenrouting-Audit. |
| U05 | [MidnightBlueLabs/tetra-bluestation README](https://github.com/MidnightBlueLabs/tetra-bluestation/blob/09d4e0d9a0b8cf6c881e77353db325df9a4715aa/README.md) | Frisch gelesener `main`-Stand `09d4e0d9a0b8cf6c881e77353db325df9a4715aa`; Alpha-Basisstationsbeschreibung. Nicht der unbekannte historische Installationscommit. |
| L01 | [GNU Coreutils: groups invocation](https://www.gnu.org/s/coreutils/manual/html_node/groups-invocation.html) | Unterscheidung zwischen Kontogruppen bei Benutzerargument und Gruppen des aktuellen Prozesses ohne Benutzerargument. |

Die vom BlueStation-README verlinkte [BlueStation-Dokumentationswiki](https://github.com/MidnightBlueLabs/tetra-bluestation-docs/wiki) ist eine sinnvolle nachfolgende Referenz, wurde aber nicht vollständig als Installations- und Funktionshandbuch geprüft. Es wird kein externer PR und kein historischer Entwicklungscommit dieses Chats behauptet.

### 15.4 Verwandte Archive – ausdrücklich andere Chats

Im vorhandenen Index stehen unter anderem bereits:

- [Spätere Dual-Carrier-Portierung und SXceiver-Hotfixes](2026-10-03_flowstation-dualcarrier-portierung-sxceiver-hotfixes.md).
- [Spätere Bearer-/ACK-/Release- und Secondary-Control-Arbeiten](2026-10-03_flowstation-dualcarrier-bearer-ack-release-und-secondary-control.md).
- [Spätere Basisstations-ISSI- und Systemidentität](2026-10-03_basisstation-issi-eigentuemer-und-systemidentitaet.md).

Diese Links dienen der Navigation zur späteren Projektgeschichte. Deren vollständige Inhalte und Testergebnisse wurden in diesem Archivierungslauf nicht erneut geprüft und werden nicht als Beleg für die frühe HamTetra-Inbetriebnahme verwendet.

## 16. Übergabe und Abschlussbedingungen

**Gesichert für die Fortsetzung:** ursprüngliche Aufgabenpräzisierungen; SXceiver statt Lime; tatsächliche Host-/Programmpfade; beide Fehlerbilder; root-Erfolg in seiner begrenzten Aussageweite; vollständige letzte Geräte-/Gruppenliste; DMO-Frequenz und Benutzerkennungen; Wechsel zu BlueStation mit angemessener Zuordnung des extern erreichten Erfolgs; korrigierte heutige Parser-/Treiberbefunde; getrennte aktuelle NetCore-Bestandsaufnahme; offene Aufgaben und Quelleninventar.

**Weiterhin fehlend:** Originaltitel/-link, vollständige historische Installationsfolge, damalige Commits und Bibliotheksversionen, tatsächlich aktivierte Regeln/Unit, abschließender Nicht-root-Nachweis, reproduzierbarer BlueStation-Installationsstand sowie DMO-/TMO-/Sprach-/SDS-/Dauerbetriebsabnahmen. Es sind keine eigenständigen Chatbilder verfügbar. Die Normen wurden nicht vollständig durchgearbeitet und nicht als neue Bilddateien hochgeladen.

**Schreibgrenze dieses Auftrags:** ausschließlich diese Markdown-Datei und der zugehörige Eintrag in `Docs/archive/README.md` auf `Archiving`. Keine Änderungen an RF-Konfiguration, Treiber, Diensten, aktiven Roadmaps, anderen Archiveinträgen oder anderen Branches; kein Merge und kein Force-Push. Vor Veröffentlichung ist der aktuelle Branchstand erneut abzugleichen. Die Abschlussmeldung muss den tatsächlichen Speichercommit und die erfolgreiche Rückleseprüfung nennen, nicht einen nur geplanten Commit.

Die wichtigste praktische Übergabe lautet: **Nicht die damals erfundenen Befehle weiter reparieren, sondern den wirklich funktionierenden Source-/Binary-/Treiberstand identifizieren und darauf aufbauen.**
