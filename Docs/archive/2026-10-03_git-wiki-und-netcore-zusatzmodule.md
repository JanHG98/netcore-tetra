# Brainstorming: Git-Wiki, externe TETRA-Tools und NetCore-Zusatzmodule

**Archivhinweis, 09.10.2026:** Diese Datei dokumentiert das im Titel beziehungsweise Quellenrahmen genannte Gespräch und dessen damalige Entscheidungen. Historische Kommandos, Planungen, Versionsangaben und Fehlerbilder bleiben erhalten; der heutige technische Stand steht in der [Gesamtroadmap](../roadmaps/gesamtroadmap.md) und in den [aktuellen Dienstanleitungen](../services/README.md). Die Archivaufnahme bestätigt keine spätere Implementierung oder Live-Abnahme.

## Rahmen und Quellenstand

| Feld | Wert |
|---|---|
| Thema | Git-Wiki-Zusammenführung, deutsche Start-/Wiki-Dokumentation, externe TETRA-Tools und NetCore-eigene Zusatzmodule |
| Erstellungsdatum | 2026-10-03 |
| Zielrepository | `JanHG98/netcore-tetra` |
| Geprüfter Branch | `Archiving` |
| Branch-Stand am 03.10.2026 | `be8009663c79b11ca7dc057aff1d385427444e42` |
| Aktuell geprüfter `main`-Stand | `6aa9be8f74ab731f72dc133a5f8e90c5018c626d` (`docs: add central identity and RBAC roadmap`) |
| Ablage | ausschließlich `Docs/archive/` |
| Status dieser Datei | Entwicklungsnotizen; keine Implementierung der beschriebenen Wiki-Seiten oder Module im Produktivcode |

## Offene Nachweise und Belastbarkeit

Grundlage sind die erhaltenen Wiki- und Modulkonzepte. Frühere Entwurfsabschnitte sind teilweise nicht erhalten. Markdown-Seiten wurden zunächst als kopierfähige Dokumententwürfe ausgearbeitet; ihre Übernahme ins Repository war zum 03.10.2026 nicht bestätigt.

**Stand der Wiki-Seiten:** Ausgearbeitetes Markdown ist von einer veröffentlichten GitHub-Wiki- oder Repository-Datei zu unterscheiden.

Es wurden keine Passwörter, Tokens, privaten Schlüssel oder Zugangsdaten übernommen.

## Ziel und Ausgangslage

Ziel war, aus vorhandenen beziehungsweise als Vorbild genannten Dokumentationen eine eigene, deutschsprachige Wiki-/Dokumentationsstruktur für NetCore-Tetra aufzubauen.

Ausgangspunkt waren insbesondere:

- Flowstation-README als grober Ursprung für die Start-README.
- Tetra-BlueStation-Wiki als strukturelles Vorbild für Wiki-Seiten.
- Wunsch, die Namen Flowstation und BlueStation aus der eigenen Dokumentation herauszulösen und auf NetCore-Tetra anzupassen.
- Zunächst kopierfähige Markdown-Entwürfe erstellen; die Übernahme ins Repository folgt gesondert.
- späterer Wunsch nach Zusatzmodulen, erst vorhandene externe Tools, dann eigene NetCore-Module.

Festgelegte Linie: **README kurz halten, eigentliche Inhalte ins Wiki auslagern, externe Tools sauber von NetCore-eigenen Modulen trennen.**

## Behandelte Themen im Überblick

### Erstellt oder ausgearbeitet als Wiki-/Markdown-Seiten

Folgende Seiten wurden als kopierfähiges Markdown ausgearbeitet:

- Start-`README.md` für das eigene Repo, kurz und deutsch.
- Wiki-`Home`.
- `_Sidebar`.
- Einführung.
- Anforderungen.
- Abhängigkeiten und Build.
- Konfiguration.
- Betrieb.
- Weboberfläche.
- SDS-Services.
- Hardware und SDR.
- Troubleshooting.
- NetCore-Integration.
- Node-Struktur.
- ZKN-Anbindung.
- Watchtower.
- FAQ.
- Changelog.
- `_Footer`.
- Externe Tool-Seiten:
  - TetraEar.
  - SDR-Tetra-Plugin.
  - sdrpp-tetra-demodulator.
  - Tetra Receiver.
  - tetra-kit.
  - tetra-toolkit.
- NetCore-eigene Zusatzmodule:
  - TetraSDS-Router.
  - TetraWeather.
  - TetraStatus.
  - TetraDiag.
  - TetraAlert.
  - TetraPrint.
  - TetraGuard.
  - TetraLogger.
  - TetraBridge.

### Nur als Idee oder weiterer Vorschlag genannt

Als sinnvolle weitere Ergänzungen wurden vorgeschlagen, aber nicht mehr ausgearbeitet:

- `SDS-Nummernplan`.
- `ISSI-und-GSSI-Planung`.
- `TetraConfig`.
- `TetraUpdate`.
- `TetraBackup`.
- `TetraNodeAgent`.
- `TetraAPI`.
- `Betriebsarten`.
- `Backup-und-Restore`.
- `Release-Prozess`.
- `Versionsschema`.
- `API-Konzept`.
- `Systemd-Dienste`.
- `Log-und-Diagnosepfade`.
- `Laboraufbau`.
- `Feldaufbau`.
- `Demo-Betrieb`.
- `Wartung-und-Updates`.
- `Glossar`.
- `Architekturübersicht`.
- `Datenflüsse`.
- `Sicherheitsmodell`.
- `Berechtigungsmodell`.
- `Schnittstellen`.
- `Roadmap`.
- `Known-Issues`.
- `Designentscheidungen`.

## Endgültige Anforderungen und Entscheidungen

### README kurz halten

**Beschlossen/geplant:** Die Start-README soll nur als Einstieg dienen. Installation, Konfiguration, Betrieb und Details sollen ins Wiki.

Begründung: Die README soll nicht zu lang werden und nicht die spätere Wiki-Struktur duplizieren.

### Wiki-Seiten ohne künstliche Nummerierung

**Beschlossen/geplant:** Seitennamen sollen direkt und lesbar sein, ohne Präfixe wie `01-`, `02-` usw.

Beispiele:

```md
- [Home](Home)
- [Einführung](Einführung)
- [Anforderungen](Anforderungen)
- [Konfiguration](Konfiguration)
```

### Externe Tools und NetCore-Module trennen

**Beschlossen/geplant:** Es soll klar unterschieden werden zwischen real existierenden externen Projekten und frei entworfenen NetCore-eigenen Modulen.

Externe Tools wurden als solche dokumentiert:

- TetraEar.
- SDR-Tetra-Plugin.
- sdrpp-tetra-demodulator.
- Tetra Receiver.
- tetra-kit.
- tetra-toolkit.

NetCore-eigene Modulnamen wurden als eigene Konzepte deklariert:

- TetraSDS-Router.
- TetraWeather.
- TetraStatus.
- TetraDiag.
- TetraAlert.
- TetraPrint.
- TetraGuard.
- TetraLogger.
- TetraBridge.

Begründung: Vertrauenswürdigkeit, saubere Attribution und kein Vermischen von Fremdprojekten mit eigenen Ideen.

### Statusbegriffe sauber trennen

**Beschlossen/geplant:** Die Dokumentation soll ausdrücklich zwischen Idee, beschlossen/geplant, implementiert, getestet und im Betrieb bestätigt unterscheiden.

In den Arbeitsnotizen wurde für nahezu alle neuen NetCore-Module nur Konzept-/Wiki-Status erreicht. Eine Implementierung wurde nicht nachgewiesen.

### Labor- und Rechtsrahmen

**Beschlossen/geplant:** Alle SDR-/TETRA-Analysewerkzeuge und NetCore-Tetra-Funktionen werden als Labor-, Test- und Forschungsumgebung dokumentiert. Sendebetrieb und Analyse fremder Netze sollen nicht impliziert oder unterstützt werden.

Wiederkehrende Formulierung:

```text
Eigene Signale. Eigene Testumgebung. Eigene Verantwortung.
```

## Architektur, Komponenten und Schnittstellen

### Dokumentationsarchitektur

Die Wiki-Struktur wurde als mehrschichtiges Dokumentationsmodell aufgebaut:

1. **Start/Orientierung**
   - README kurz.
   - Wiki-Home als Inhaltsübersicht.
   - Sidebar als Navigation.
   - Footer mit Branding und Laborhinweis.

2. **Basisbetrieb**
   - Einführung.
   - Anforderungen.
   - Abhängigkeiten und Build.
   - Konfiguration.
   - Betrieb.
   - Weboberfläche.
   - Hardware und SDR.
   - Troubleshooting.

3. **NetCore-Betriebsmodell**
   - NetCore-Integration.
   - Node-Struktur.
   - ZKN-Anbindung.
   - Watchtower.

4. **Zusatzmodule**
   - SDS, Wetter, Status, Diagnose, Alerts, Print, Guard, Logger, Bridge.

5. **Externe Tools**
   - TetraEar, SDR#-Plugin, SDR++-Plugin, Tetra Receiver, tetra-kit, tetra-toolkit.

### NetCore-Zielarchitektur

Konzeptionell entstand folgende Struktur:

```text
TETRA-Endgeräte
      |
      | Luftschnittstelle / SDS / Sprache
      v
NetCore-Tetra Node / TBS
      |
      +--> lokale WebUI
      +--> TetraSDS-Router
      +--> TetraStatus
      +--> TetraDiag
      +--> TetraWeather
      +--> TetraAlert
      +--> TetraPrint
      +--> TetraGuard
      +--> TetraLogger
      +--> TetraBridge
      |
      v
Watchtower / ZKN / externe Dienste
```

### Watchtower, ZKN und TetraAlert

**Beschlossen/geplant:**

- Watchtower: technische Monitoring-Ebene.
- ZKN: operative Kontroll- und Netzinstanz.
- TetraAlert: Meldeschicht zwischen technischer Erkennung und menschlicher Reaktion.

Abgrenzung:

```text
Watchtower erkennt: Node gelb/rot.
TetraAlert erzeugt Meldungen mit Deduplizierung, Cooldown und Empfängern.
ZKN bewertet operativ und entscheidet über Eskalation.
```

### TetraSDS-Router als zentrale SDS-Schicht

**Beschlossen/geplant:** Der TetraSDS-Router wird als zentrale Vermittlungsschicht für SDS-Dienste entworfen.

Vorgeschlagene Servicebereiche:

| Bereich | Verwendung |
|---:|---|
| `40000–40009` | allgemeine Systemdienste |
| `40010–40019` | Node- und Hardwarestatus |
| `40020–40029` | Endgeräte- und Gruppeninformationen |
| `40030–40039` | ZKN-/Watchtower-/Alert-/Print-/Guard-/Logger-/Bridge-Dienste |
| `40040–40049` | Labor-, Test- und Debugdienste |
| `40050–40099` | Reserve |

Konkrete Service-Nummern aus den Arbeitsnotizen:

| Nummer | Modul/Dienst | Status |
|---:|---|---|
| `40000` | Help | geplant |
| `40001` | Echo | geplant |
| `40002` | Time | geplant |
| `40004` | TetraWeather | geplant |
| `40010` | TetraStatus | geplant |
| `40011` | TetraDiag | geplant |
| `40030` | ZKN | geplant |
| `40031` | Watchtower | geplant |
| `40032` | TetraAlert | geplant |
| `40033` | TetraPrint | geplant |
| `40034` | TetraGuard | geplant |
| `40035` | TetraLogger | geplant |
| `40036` | TetraBridge | geplant |

## Erreichter Entwicklungs- und Betriebsstand

### Historisch erarbeitet

**Als Markdown-Entwurf ausgearbeitet:**

- Ausführliche Markdown-Entwürfe für die oben genannten Wiki-Seiten.
- Einheitlicher Dokumentationsstil.
- Trennung externe Tools vs. eigene Module.
- Ein Modulnamens- und Nummerierungskonzept.
- Wiederverwendbare Beispielkonfigurationen als TOML-Konzepte.
- systemd-Ideen für mehrere Dienste.
- SDS-Beispielantworten.
- Checklisten, Testpläne und typische Fehlerbilder.

### Im Repository bestätigt

**Geprüft:**

- Das Repository `JanHG98/netcore-tetra` ist erreichbar und der Connector hat Schreibrechte.
- Der Archivbranch `Archiving` existiert.
- Zum Beginn der Quellenprüfung vom 03.10.2026 stand `Archiving` auf Commit `be8009663c79b11ca7dc057aff1d385427444e42`.
- `Docs/archive/README.md` existiert bereits und enthält mehrere Archivindex-Einträge.
- `main` stand bei Prüfung auf Commit `6aa9be8f74ab731f72dc133a5f8e90c5018c626d` mit Commit-Message `docs: add central identity and RBAC roadmap`.
- Die Haupt-README auf `main` nennt Stand `v1.9.0` und beschreibt NINA/KATWARN, eigene Warnmeldungen, zentrale SIP-Anbindung und lokalen Fallback.
- Eine Code-/Dateisuche nach den neuen Modulnamen wie `TetraSDS-Router`, `TetraWeather`, `TetraStatus`, `TetraDiag`, `TetraAlert`, `TetraPrint`, `TetraGuard`, `TetraLogger`, `TetraBridge` lieferte keine Treffer auf dem geprüften Standard-Suchstand. Daraus folgt: Die Module sind im Repository nicht als implementiert bestätigt.

### Nicht bestätigt

Nicht bestätigt wurden:

- Dass die erstellten Wiki-Seiten bereits ins GitHub-Wiki übertragen wurden.
- Dass die Module implementiert wurden.
- Dass SDS-Service-Nummern in laufender Software reserviert sind.
- Dass TetraEar oder andere externe Tools im NetCore-Repo integriert sind.
- Dass die vorgeschlagenen systemd-Dienste existieren.
- Dass die vorgeschlagenen Ports/APIs existieren.

## Relevante Dateien, Pfade, Dienste, Ports und Parameter

### Archivpfade

```text
Docs/archive/
Docs/archive/README.md
Docs/archive/2026-10-03_git-wiki-und-netcore-zusatzmodule.md
```

### Vorgeschlagene spätere lokale Pfade

Diese Pfade wurden als Konzepte vorgeschlagen, nicht als im Repo bestätigt:

```text
/opt/netcore-tetra/
/opt/netcore-tetra/config/
/opt/netcore-tetra/node-info/
/opt/netcore-tetra/status/
/opt/netcore-tetra/sds-router/
/opt/netcore-tetra/tetraweather/
/opt/netcore-tetra/tetrastatus/
/opt/netcore-tetra/tetradiag/
/opt/netcore-tetra/tetraalert/
/opt/netcore-tetra/tetraprint/
/opt/netcore-tetra/tetraguard/
/opt/netcore-tetra/tetralogger/
/opt/netcore-tetra/tetrabridge/
/opt/netcore-tetra/secrets/
```

### Vorgeschlagene systemd-Dienste

Nur Konzeptstatus:

```text
netcore-tetra.service
netcore-sds-router.service
tetraweather.service
tetrastatus.service
tetradiag.service
tetraalert.service
tetraprint.service
tetraguard.service
tetralogger.service
tetrabridge.service
```

### Vorgeschlagene lokale API-Ports

Nur Konzeptstatus:

| Port | Dienstidee |
|---:|---|
| `9010` | TetraStatus / Node-Status |
| `9011` | TetraDiag |
| `9020` | TetraWeather |
| `9032` | TetraAlert |
| `9033` | TetraPrint |
| `9034` | TetraGuard |
| `9035` | TetraLogger |
| `9036` | TetraBridge |

### Wiederkehrende technische Parameter

Historischer Konzeptstand:

```text
node_id = "srv-m-rpi-tbs01"
short_name = "TBS01"
service_number = 400xx
ascii_only = true
```

Für ISSI-Darstellung:

```text
UI-Darstellung: 04010001
Technische ISSI: 4010001
```

## Externe Tools: Status und Einordnung

### TetraEar

**Status:** Externes Tool, nicht NetCore-eigene Implementierung.

Inhalt der Wiki-Seite:

- Installation auf Debian/Raspberry Pi 5.
- Virtualenv.
- `pyrtlsdr==0.3.0` wegen `rtlsdr_set_dithering`-Problem.
- `setuptools<81` bei fehlendem `pkg_resources`.
- RTL-SDR-Test mit `rtl_test`.
- Codec-Installation über `scripts/install_tetra_codec.sh`, nicht über Python-Installer.
- Ersetzen von Windows-`.exe`-Dateien im Codec-Ordner durch Linux-Binaries mit denselben Dateinamen.
- Start mit `PYTHONNOUSERSITE=1 ./venv/bin/python -m tetraear`.

Wichtiger Startbefehl:

```bash
cd ~/TetraEar
PYTHONNOUSERSITE=1 ./venv/bin/python -m tetraear --no-gui
```

### SDR-Tetra-Plugin

**Status:** Externes SDR#-/Windows-Plugin.

Dokumentiert als:

- SDR# / SDRSharp Plugin.
- C#/.NET-5-Bezug laut Projektbeschreibung.
- Windows-orientiert.
- Experimentelles Analysewerkzeug, kein NetCore-Kernmodul.

### sdrpp-tetra-demodulator

**Status:** Externes SDR++-Plugin.

Dokumentiert als:

- TETRA-Demodulator-Plugin für SDR++.
- Downlink-Demodulation/Decoding.
- `libtalloc`-Abhängigkeit.
- Build über SDR++ Core Headers oder `SDRPP_MODULE_CMAKE`.
- ETSI-Codec-Patch-Script.
- Arch/AUR-Hinweis.

### Tetra Receiver

**Status:** Externes Tool.

Dokumentiert als:

- Empfänger für mehrere TETRA-Streams.
- UDP-Ausgabe an Decoder wie `tetra-rx` oder `tetra-kit`.
- TOML-/CLI-Konfiguration.
- Decimation-Konzept.
- Prometheus-Idee für Stream-Pegel.

### tetra-kit

**Status:** Externes Tool.

Dokumentiert als:

- Linux-orientiertes TETRA-Downlink-Decoder-/Recorder-Kit.
- GNU-Radio-Bezug.
- JSON-Ausgabe.
- Recorder für eigene unverschlüsselte Testsignale.
- Vergleich zu TetraEar, Tetra Receiver und SDR++-Plugin.

### tetra-toolkit

**Status:** Externes Tool.

Dokumentiert als:

- GNU-Radio-/GRC-Workflow.
- Bits per UDP an `127.0.0.1:1234`.
- Weiterverarbeitung mit `tetra-rx`.
- GSMTAP/Wireshark auf `127.0.0.1:4729`.
- Ubuntu-24.04-orientierte Anleitung.

## NetCore-eigene Module: Zusammenfassung

### TetraSDS-Router

**Status:** beschlossen/geplant als Konzeptmodul.

Aufgaben:

- SDS-Eingang auswerten.
- Zielnummer und Quell-ISSI prüfen.
- Berechtigungen prüfen.
- Dienst anhand Routing-Tabelle aufrufen.
- SDS-Antwort erzeugen.
- Logging und Metriken.

Vorgeschlagene Route:

```toml
[[services]]
number = 40010
name = "node-status"
handler = "http://127.0.0.1:9010/status"
enabled = true
permission = "internal"
timeout_ms = 1000
```

### TetraWeather

**Status:** geplant.

Aufgaben:

- Wetterabfrage per SDS an `40004`.
- Befehle `WETTER`, `WX`, `WARN`, `HELP`.
- DWD/API/lokale Sensorik als mögliche Quellen.
- Cache und Offline-Fallback.

Beispielantwort:

```text
Hannover
12C Regen
Wind W 18
Warnung keine
```

### TetraStatus

**Status:** geplant.

Aufgaben:

- kurzer Node-Status per SDS an `40010`.
- Befehle `STATUS`, `STAT`, `HEALTH`, `NODE`, `SERVICES`, `REG`, `CALLS`, `HELP`.
- Statuscache.
- Watchtower/ZKN-Datenquelle.

Beispielantwort:

```text
TBS01 OK
TX aktiv
RX aktiv
Reg 3
Calls 0
```

### TetraDiag

**Status:** geplant.

Aufgaben:

- technische Diagnose per SDS an `40011`, API und WebUI.
- CPU/RAM/Disk/Temp/SDR/WebUI/Services/Config/Logs.
- Fehlercodes wie `SDR_MISSING`, `TEMP_HIGH`, `WEB_OFF`.
- Diagnoseberichte.

Beispielantwort:

```text
TBS01 DIAG
CPU 18%
RAM 41%
Temp 42C
SDR OK
```

### TetraAlert

**Status:** geplant.

Aufgaben:

- Alerts sammeln, deduplizieren und priorisieren.
- SDS-Service `40032`.
- Befehle `ALERTS`, `ALARMS`, `WARN`, `LAST`, `ACK`, `HELP`.
- Cooldown, Eskalation, Quittierung, Resolved-Meldungen.
- Übergabe an ZKN, Watchtower, TetraPrint.

### TetraPrint

**Status:** geplant.

Aufgaben:

- Thermodrucker-Ausgabe für Startbons, Fehlerbons, Diagnosebons, Updatebons, Wartungsbons.
- SDS-Service `40033`.
- QR-Code-Ideen.
- ESC/POS, USB/TCP/CUPS als mögliche Anbindungen.
- Print-Level, Deduplizierung und Cooldown.

### TetraGuard

**Status:** geplant.

Aufgaben:

- ISSI-Whitelist.
- Rollenmodell.
- SDS-Berechtigungen.
- Config-Signaturen.
- USB-Config-Schutz.
- WebUI-/API-Schutz.
- Audit-Logs.
- Safe Mode.

### TetraLogger

**Status:** geplant.

Aufgaben:

- strukturierte Logs und Events.
- JSONL-Eventformat.
- Rotation, Aufbewahrung, Redaction.
- SDS-Service `40035` mit `LAST`, `ERR`, `WARN`, `ALERTS`, `EXPORT`.
- Unterstützung für TetraDiag, TetraAlert, TetraGuard, Watchtower, ZKN.

### TetraBridge

**Status:** geplant.

Aufgaben:

- kontrollierte externe Integration.
- HTTP APIs, Webhooks, Queue, Retry, Filter, Transformation.
- Status zu Watchtower, Alerts zu ZKN, externe APIs für SDS-Dienste.
- SDS-Service `40036`.
- TetraGuard-gesicherte Schreibaktionen.

## Befehle und Abläufe

### TetraEar-Installation

**Status:** vorgeschlagen/dokumentiert; in den Arbeitsnotizen nicht live ausgeführt.

Kompaktfassung:

```bash
sudo apt update
sudo apt upgrade -y
sudo apt install -y git python3 python3-venv python3-pip build-essential gcc g++ make wget curl unzip patch rtl-sdr librtlsdr-dev libportaudio2 portaudio19-dev

cd ~
git clone https://github.com/syrex1013/TetraEar.git
cd TetraEar

python3 -m venv venv
source venv/bin/activate

pip install --upgrade pip
pip install -r requirements.txt
pip uninstall -y pyrtlsdr
pip install "pyrtlsdr==0.3.0"
pip install "setuptools<81"

chmod +x scripts/install_tetra_codec.sh
./scripts/install_tetra_codec.sh

cd ~/TetraEar/tetraear/tetra_codec/bin
rm -f cdecoder.exe sdecoder.exe ccoder.exe scoder.exe
cp cdecoder cdecoder.exe
cp sdecoder sdecoder.exe
cp /tmp/tetra_codec_build/osmo-tetra/codec/c-code/ccoder ./ccoder.exe
cp /tmp/tetra_codec_build/osmo-tetra/codec/c-code/scoder ./scoder.exe
chmod +x cdecoder.exe sdecoder.exe ccoder.exe scoder.exe

cd ~/TetraEar
python -m tetraear.tools.verify_codec

PYTHONNOUSERSITE=1 ./venv/bin/python -m tetraear
```

### NetCore-Tetra Build-Seite

**Status:** vorgeschlagen/dokumentiert; nicht in dieser Entwicklungsphase ausgeführt.

Enthielt Debian-basierte Build-Abhängigkeiten, Rustup, Kannel/WAP-Konfiguration, SoapySX-Installation und Build über `cargo build --release`.

### Wiki-/Repo-Abläufe

**Status:** nur geplant.

- Wiki-Seiten sollten nach und nach kopiert und angepasst werden.
- Credits für Flowstation/BlueStation waren als spätere Ergänzung vorgesehen.
- Die Inhalte lagen als Markdown-Entwürfe vor und wurden nicht direkt ins Wiki geschrieben.

## Fehler, Diagnosen und Lösungen

### TetraEar: `undefined symbol: rtlsdr_set_dithering`

**Diagnose:** Neuere `pyrtlsdr` erwartet Symbol, das die Debian-`librtlsdr` im betrachteten Setup nicht liefert.

**Lösung:**

```bash
pip uninstall -y pyrtlsdr
pip install "pyrtlsdr==0.3.0"
```

Zusätzlich TetraEar immer mit `PYTHONNOUSERSITE=1 ./venv/bin/python ...` starten.

### TetraEar: `ModuleNotFoundError: No module named 'pkg_resources'`

**Diagnose:** altes Paket erwartet `pkg_resources`, modernes `setuptools` liefert es nicht passend.

**Lösung:**

```bash
pip install "setuptools<81"
```

### TetraEar: `Exec format error`

**Diagnose:** Windows-`.exe`-Dateien lagen im Codec-Ordner statt ARM-/Linux-Binaries.

**Lösung:** Linux-Binaries bauen und unter den erwarteten `.exe`-Namen ablegen.

### TetraEar: `PortAudio library not found`

**Lösung:**

```bash
sudo apt install -y libportaudio2 portaudio19-dev
sudo ldconfig
```

### Dokumentationsfehler: erfundene Pluginnamen

**Fehler:** Zunächst wurden viele Modulnamen als „Plugins“ vorgeschlagen, ohne klar zu trennen, ob sie real existieren.

**Korrektur:** Die tatsächliche Existenz der Namen wurde geprüft. Verbindliche Trennung:

- real existierende externe Tools.
- eigene NetCore-Modulnamen.

Diese Korrektur ist eine wichtige spätere Festlegung und ersetzt die frühere unsaubere Darstellung.

## Tests und Ergebnisse

### Historisch dokumentierte Tests

Keine Repository-Builds, keine Tetra-Funkversuche und keine Tool-Installationen wurden in dieser Entwicklungsphase tatsächlich ausgeführt.

### Repository-Prüfungen vom 03.10.2026

**Durchgeführt:**

- Repository-Metadaten abgefragt.
- Branch `Archiving` gesucht und gefunden.
- Branch-Commit von `Archiving` gelesen.
- `Docs/archive/` und `Docs/archive/README.md` gelesen.
- `main`-Branch-Commit gelesen.
- `README.md` auf `main` ausschnittweise gelesen.
- GitHub-Code-/Dateisuche nach neuen Modulnamen durchgeführt; keine Treffer.

**Grenzen:**

- GitHub-Code-Suche bezieht sich auf den Suchindex und wurde nicht als vollständiger Checkout mit Grep über alle Branches durchgeführt.
- GitHub-Wiki wurde nicht als separates Wiki-Repository geprüft.
- Canvas-Inhalte wurden aus den erhaltenen Entwürfen ausgewertet, nicht aus einer persistierten Wiki-Datei.

## Verworfene oder ersetzte Ansätze

### Lange README

**Verworfen:** Eine sehr ausführliche Start-README mit Installation, Konfiguration und Betrieb.

**Grund:** README soll kurz bleiben; Details gehören ins Wiki.

### Nummerierte Wiki-Seiten

**Verworfen:** `01-Home`, `02-Einführung` usw.

**Grund:** Direkte Seitennamen ohne künstliche Nummerierung sind verbindlich.

### Externe Tools als eigene Plugins darstellen

**Verworfen/korrigiert:** Fremdprojekte und eigene Ideen in einer Liste als „Plugins“ ohne Statusklarheit.

**Grund:** Gefahr falscher Attribution und Vertrauensverlust.

### TetraEar Python-Codec-Installer

**Verworfen für das dokumentierte Setup:**

```bash
python -m tetraear.tools.install_tetra_codec
```

**Grund:** Im dokumentierten Ausgangsstand verursachte dieser Weg Ärger. Stattdessen wurde der Linux-Installer aus `scripts/` empfohlen.

## Offene Aufgaben und Roadmap-Kandidaten

### Höchste Priorität

1. **SDS-Nummernplan erstellen.**
   - Verbindet TetraSDS-Router, TetraWeather, TetraStatus, TetraDiag, TetraAlert, TetraPrint, TetraGuard, TetraLogger und TetraBridge.
   - Sollte Konflikte, Reserven, Rechte und Antwortformate enthalten.

2. **ISSI-und-GSSI-Planung erstellen.**
   - Technische ISSI vs. UI-Darstellung klären.
   - Bereiche für Endgeräte, Dienste, Infrastruktur, Gruppen und Tests festlegen.

3. **TetraConfig ausarbeiten.**
   - Config-Verwaltung, Signaturen, Validierung, Versionierung, Rollback.

4. **TetraUpdate ausarbeiten.**
   - Git-Pull, Build, Restart, Healthcheck, Rollback, Release-Workflow.

5. **TetraBackup ausarbeiten.**
   - Configs, Logs, Node-Steckbriefe, Reports, SDS-Routen, Schlüssel-/Public-Key-Handling.

6. **TetraNodeAgent ausarbeiten.**
   - Heartbeat, lokale Healthchecks, Statusdateien, Watchtower-Anbindung.

### Weitere sinnvolle Wiki-Seiten

- API-Konzept.
- Betriebsarten.
- Backup-und-Restore.
- Release-Prozess.
- Versionsschema.
- Systemd-Dienste.
- Log-und-Diagnosepfade.
- Laboraufbau.
- Feldaufbau.
- Demo-Betrieb.
- Wartung-und-Updates.
- Glossar.
- Architekturübersicht.
- Datenflüsse.
- Sicherheitsmodell.
- Berechtigungsmodell.
- Schnittstellen.
- Roadmap.
- Known-Issues.
- Designentscheidungen.

### Konkrete nächste Schritte

1. Wiki-Seiten tatsächlich ins GitHub-Wiki oder in ein dokumentiertes Wiki-Quellverzeichnis übertragen.
2. Prüfen, ob GitHub-Wiki separat versioniert ist und ob ein eigenes Wiki-Repo genutzt werden soll.
3. Credits/Herkunftshinweise zu Flowstation, Tetra-BlueStation und externen Tools ergänzen.
4. Modulstatus-Seite anlegen: `Idee`, `geplant`, `implementiert`, `getestet`, `im Betrieb bestätigt`.
5. SDS-Service-Nummernplan verbindlich machen.
6. API-/Portplan mit echten Implementierungsständen abgleichen.
7. Prüfen, welche der vorgeschlagenen Module bereits teilweise durch vorhandene Komponenten in `main` abgedeckt sind, z. B. NINA/KATWARN, Alert-Service, SIP-Switch, RBAC-Roadmap.
8. Für externe Tools eine separate Lizenz-/Attributionsseite anlegen.
9. README im Repo erst anpassen, wenn Wiki-Grundstruktur wirklich steht.

## Relevante Quellen und Repository-Befunde

### Repository

- `JanHG98/netcore-tetra`.
- Branch `Archiving` am 03.10.2026: `be8009663c79b11ca7dc057aff1d385427444e42`.
- Branch `main` bei Prüfung: `6aa9be8f74ab731f72dc133a5f8e90c5018c626d`.
- `Docs/archive/README.md` war bereits vorhanden.
- `main/README.md` nennt `v1.9.0 · NINA/KATWARN, eigene Warnmeldungen und Funk-/SDS-Korrekturen`.
- `Docs/CENTRAL_IDENTITY_RBAC_ROADMAP.md` existiert laut Docs-Verzeichnis auf `main`.

### Externe Tool-Repositories

- `https://github.com/syrex1013/TetraEar.git`
- `https://github.com/vgpastor/SDR-Tetra-Plugin`
- `https://github.com/cropinghigh/sdrpp-tetra-demodulator`
- `https://github.com/tlm-solutions/tetra-receiver`
- `https://gitlab.com/larryth/tetra-kit`
- `https://github.com/Tim---/tetra-toolkit`

### Anhänge / Projektdateien

Im Projektkontext waren zahlreiche ETSI-PDFs verfügbar. Diese wurden in dieser Entwicklungsphase nicht inhaltlich als primäre Quelle für die Wiki-Seiten ausgewertet. Sie bleiben als mögliche spätere Norm-/Referenzbasis relevant, insbesondere für:

- Air Interface.
- Security.
- PEI.
- SDS / ANF-ISISDS.
- ISI Group Call / Mobility Management.
- Codec.
- PICS / Conformance.

## Schlussbewertung

Ergebnis ist eine erweiterte **Dokumentations- und Modulkonzeption** für NetCore-Tetra mit strukturierter Wiki-Roadmap:

- klare Navigation,
- externe Toolseiten,
- eigene Modulwelt,
- SDS-Servicebereiche,
- ZKN-/Watchtower-Trennung,
- Sicherheits-, Logging-, Alerting- und Bridge-Konzepte.

Die wichtigsten offenen Punkte sind jetzt nicht mehr Ideenfindung, sondern Konsolidierung:

1. Nummernplan festziehen.
2. ISSI/GSSI-Planung festziehen.
3. echte Wiki-Ablage schaffen.
4. Roadmap-Kandidaten priorisieren.
5. Repository-Code gegen die Konzepte mappen.
6. implementierte Komponenten klar von Konzeptmodulen trennen.

Bis dahin gilt: Die hier beschriebenen NetCore-Zusatzmodule sind **geplant/konzipiert**, nicht als implementiert bestätigt.
