# Abschlussdokumentation: Main-kompatibler RF-Pfad, Pi-Neuinstallation, SXceiver-Systemd-Fix und SWMI-Fallback

## 1. Metadaten

- **Projekt:** NetCore-Tetra
- **Repository:** `JanHG98/netcore-tetra`
- **Archivbranch:** `Archiving`
- **Archivpfad:** `Docs/archive/2026-10-05_main-kompatibler-rf-pfad-pi-neuinstallation-sxceiver-und-swmi-fallback.md`
- **Erstellungsdatum dieser Zusammenfassung:** 2026-10-05
- **Technischer Zeitraum des ausgewerteten Chats:** hauptsächlich 2026-07-29; Archivierung am 2026-10-05
- **Ursprünglicher Chattitel:** im zugänglichen Verlauf nicht zuverlässig verfügbar
- **Chatlink:** im zugänglichen Verlauf nicht verfügbar
- **Historisch bearbeiteter Branch:** `swmi` (im Chat aktiv; am 2026-10-05 nicht mehr als Branch vorhanden)
- **Historischer, im Chat zuletzt bestätigter SWMI-Snapshot:** Commit `60cb7438d6e82f4faa9cc814a752380e3acafc5e`
- **Historischer Main-Snapshot aus Chat-Anhang:** Commit `0b84cc0c8340b4a4ea34778fa5a2653a682f9ae7`
- **Aktuell geprüfter Repository-Stand am 2026-10-05:** Branch `main`, Commit `7137e0dd69877e1b604bf89148fd8b6b590c1a97`
- **Archivbranch vor diesem Dokument:** `Archiving`, Commit `cabafc0b270be4d87d18615735abe3155df8da1f`

### Auswertungsgrundlage und Lücken

Ausgewertet wurden der vollständig zugängliche Chatkontext, die späteren Korrekturen im selben Chat, der hochgeladene Laufzeitlog `Eingefügter Text(15).txt`, der historische Repository-Snapshot `netcore-tetra-main(3).zip` sowie der historische SWMI-Snapshot `netcore-tetra-swmi(9).zip`. Die ZIP-Metadaten identifizieren die Stände als `0b84cc0c...` beziehungsweise `60cb7438...`. Zusätzlich wurde der am 2026-10-05 live erreichbare Repository-Stand auf `main` und `Archiving` überprüft.

Nicht zugänglich beziehungsweise nicht sicher rekonstruierbar sind der ursprüngliche Chatlink und der ursprüngliche Chattitel. Im ausgewerteten Chat waren **keine eigenständigen Bildanhänge** enthalten; daher wurden für diesen Archiveintrag keine neuen Bilddateien angelegt. Die beiden Repository-ZIPs enthalten zahlreiche Projektdateien, sind aber keine Chatbilder und wurden nicht als Bildanhänge in `Docs/archive/` dupliziert.

Zugangsdaten, Passwörter, Tokens, private Schlüssel und sonstige Secrets werden in diesem Dokument bewusst nicht übernommen.

---

## 2. Statuslegende

Dieses Dokument unterscheidet konsequent:

- **Idee:** diskutierter Ansatz ohne verbindliche Umsetzung.
- **Beschlossen/geplant:** als Ziel festgelegt, aber nicht zwingend implementiert.
- **Implementiert:** durch Repository-Commit oder überprüften Source-Stand belegt.
- **Getestet:** durch Build-, Laufzeit- oder Diagnoseausgabe im Chat belegt.
- **Im Betrieb bestätigt:** vom Nutzer nach realem Betrieb ausdrücklich bestätigt.

Eine Aussage im Chat allein wird nicht als Repository-Nachweis gewertet.

---

## 3. Ziel und Ausgangslage

### Ziel

Das zentrale Ziel des Chats war, die bereits umfangreich ausgebaute SWMI-/Core-Architektur wieder so mit der Basisstation zu verbinden, dass die **lokal bewährte Funkstrecke** nicht durch zentrale Zustandsautomaten, Policies oder Service-Statuswechsel destabilisiert wird.

Konkret sollte der lokale RF-Pfad wieder das Verhalten des funktionierenden historischen Main-/No-Core-Stands erhalten:

```text
RF / lokale Zelle
  MM
  MLE
  CMCE
  UMAC/LMAC/LLC
  lokale Registrierung
  lokale Rufzustände
  lokale Floor-/Hangtime-/Release-Sequenz
        |
        +--> Telemetrie / Policy / Mobility / Media / SDS / Control
             über Node Gateway in die SWMI
```

Die SWMI sollte also **ergänzen und zentral koordinieren**, aber nicht mehr den normalen Single-Site-RF-Betrieb umformen.

### Ausgangslage

Vor der Korrektur traten insbesondere folgende Symptome auf:

- Sepura registrierte sich, begann aber wiederholt `RoamingLocationUpdating`.
- Senden beziehungsweise Rufaufbau verhielt sich unzuverlässig.
- Frühere SWMI-Anpassungen hatten MM-/CMCE-/UMAC-Verhalten gegenüber dem funktionierenden Main verändert.
- Der Versuch „v25“ setzte Common-SCCH für StayAlive-Geräte erzwungen auf Frame 18 / TS1.
- Beim Neuaufbau eines frischen Raspberry Pi traten zusätzlich Build- und Hardwarezugriffsfehler auf.
- Nach Wiederherstellung der lokalen Funkfunktion lief die TBS weiterhin im lokalen Fallback, obwohl der Node Gateway und die Core-Dienste grundsätzlich gesund waren.

---

## 4. Endgültige Anforderungen und Entscheidungen

### 4.1 Git ist die einzige Build-Quelle

**Status: beschlossen und im weiteren Chat angewendet**

Der Nutzer stellte ausdrücklich klar, dass der Build **direkt aus Git** erfolgen muss. ZIP-basierte Overlays, manuelles Kopieren einzelner Source-Bäume oder lokale Sonderstände wurden als unerwünscht verworfen.

Endgültiger Arbeitsstil:

```bash
git clone ...
git fetch ...
git switch ...
git reset --hard origin/<branch>
cargo build ...
```

Historische ZIPs dienen nur noch der Analyse beziehungsweise Rekonstruktion, nicht dem Deployment.

### 4.2 Lokaler RF-Pfad bleibt autoritativ

**Status: beschlossen, implementiert und später in Betrieb bestätigt**

Für den normalen lokalen Funkbetrieb bleiben MM/MLE/CMCE und die lokalen Rufzustände auf der TBS maßgeblich. Insbesondere sollen zentrale Dienste nicht:

- normale Location Updates künstlich neu aufbauen,
- Live-Teilnehmerzustände während harmloser Refreshes rekonstruieren,
- lokale Rufzustände durch eine zweite Call-FSM übersteuern,
- lokale Floor-/Hangtime-/Release-Sequenzen verändern,
- bei Node-Gateway-/Service-Matrix-Wechseln das RF-SYSINFO unnötig verändern.

Der Core bleibt für zentrale Policies, Telemetrie, Mobility, SDS, Media, Multi-Site-Funktionen und spätere zentrale Steuerung zuständig, darf aber die funktionierende lokale Zelle nicht destabilisieren.

### 4.3 StayAlive-Geräte bleiben auf dem normalen MCCH

**Status: spätere Korrektur; implementiert und heute weiterhin im Source vorhanden**

Die frühere v25-Hypothese war falsch. v25 hatte angenommen, bei `clch_needed || common_scch` müsse auch für `StayAlive` zwingend `Some(0x01)` für Frame-18-Common-SCCH gesetzt werden.

Später wurde anhand des funktionierenden Main-Pfads und des realen Funkverhaltens korrigiert:

- `StayAlive` überwacht den normalen MCCH.
- Common-SCCH auf Frame 18 wird nur bei tatsächlichen Energy-Economy-Modi zugewiesen.
- Für StayAlive bleibt `scch_information_and_distribution_on_18th_frame = None`.

Der heutige `main`-Stand enthält diese Logik weiterhin.

### 4.4 RF-SYSINFO darf nicht an Core-Service-Matrix-Churn hängen

**Status: implementiert**

Ein weiterer Fehler war, dass Node-Gateway-/Core-Service-Zustände den ausgestrahlten RF-Zellenvertrag beeinflussen konnten. Dies wurde so geändert, dass `system_wide_services` im normalen lokalen Betrieb nicht durch Gateway-Reconnects oder Service-Matrix-Lease-Wechsel flattert.

### 4.5 Edge-Fallback bleibt grundsätzlich aktiviert

**Status: beschlossen und im Source weiterhin vorhanden**

Fallback sollte **nicht deaktiviert** werden. Der gewünschte Zustand ist:

```text
lokaler RF-Pfad stabil
+ Node Gateway verbunden
+ frische, vollständig verfügbare Required-Service-Matrix
= EdgeFallbackMode::Online
```

Bei Gateway-/Core-Ausfall muss lokale Funkfunktion erhalten bleiben.

---

## 5. Chronologie der zentralen Codekorrekturen

### 5.1 Überholter v25-Ansatz

**Status: verworfen**

Der v25-Ansatz stellte für StayAlive-Terminals Common-SCCH auf Frame 18 / TS1 wieder her. Im Chat wurde zunächst angenommen, dies entspreche exakt dem letzten funktionierenden Stand. Diese Schlussfolgerung wurde später ausdrücklich zurückgenommen.

Wichtig für spätere Arbeiten: **v25 darf nicht wieder als „bekannt guter“ Funkvertrag übernommen werden.**

### 5.2 Commit `5fee15c276e96cd9e7b90e9ea4944c9f2662447d`

**Status: implementiert**

Commit-Titel:

> Restore exact main MM and local call setup on RF path

Wesentliche Wirkung:

- SWMI-MM-Pfad durch den bewährten Main-MM-Ablauf ersetzt.
- zentrale Group-Policy-Entscheidungen aus dem lokalen `U-SETUP`-Pfad entfernt.
- lokale Rufzulassung wieder durch lokale CMCE-Zustände bestimmt.
- MM-seitige Live-Core-Policy-Verarbeitung aus dem normalen RF-Pfad entfernt.
- Core-Dienste blieben im Repository und für zentrale Funktionen verfügbar.

### 5.3 Commit `863068a9f74877a123b2cc42468d747261450879`

**Status: implementiert**

Commit-Titel:

> Keep RF SYSINFO independent from Core service-matrix churn

Wesentliche Wirkung:

- RF-SYSINFO wieder von Core-Service-Matrix-Churn entkoppelt.
- bei konfiguriertem Brew kann dessen Connectivity weiterhin relevant sein;
- ansonsten bleibt die statische Zellkonfiguration maßgeblich.
- Media-Bridge-API blieb erhalten.

### 5.4 Commit `78257a4295dfac19f5d4e90817f156d3fc6a35c3`

**Status: implementiert und auf dem Pi als laufende Binary belegt**

Commit-Titel:

> Remove incorrect v25 common-SCCH profile markers

Wesentliche Wirkung:

- falsche v25-Dokumentation entfernt,
- falscher v25-Guard entfernt,
- Startup-Marker, der v25 als gewünschten Radiovertrag auswies, entfernt.

Der hochgeladene Laufzeitlog zeigt anschließend:

```text
Version: v1.3.0-78257a42
Radio runtime: MAIN-COMPAT (local MM/MLE/CMCE state machines)
```

Damit ist belegt, dass genau dieser Commit auf dem Pi gebaut und gestartet wurde.

### 5.5 Commit `60cb7438d6e82f4faa9cc814a752380e3acafc5e`

**Status: implementiert und anschließend im Betrieb bestätigt**

Commit-Titel:

> Allow SXceiver devices in packet-gateway systemd sandbox

Dieser Commit korrigierte einen systemd-Hardwarezugriffsfehler. Details siehe Abschnitt 9.

---

## 6. Architektur und Schnittstellen

### 6.1 TBS / lokaler RF-Stack

Relevante lokale Komponenten:

- PHY / SoapySDR / SoapySX
- LMAC
- UMAC
- LLC
- MLE
- MM
- CMCE
- SNDCP
- lokale Recorder-/Audio-/SIP-Bausteine

Wichtige Sourcepfade:

```text
bins/bluestation-bs/src/main.rs
crates/tetra-entities/src/mm/mm_bs.rs
crates/tetra-entities/src/mle/mle_bs.rs
crates/tetra-entities/src/cmce/
crates/tetra-entities/src/umac/umac_bs.rs
crates/tetra-entities/src/umac/subcomp/
crates/tetra-core/src/timeslot_alloc.rs
```

### 6.2 Node Gateway / SWMI

Die TBS verbindet sich zentral über:

```text
ws://10.0.1.179:8080/ws/node
```

Historisch genutzte Node-ID:

```text
SRV-M-TBS-01
```

Der Node Gateway bietet für Backend-Dienste:

```text
/ws/backend
```

und für Diagnose unter anderem:

```text
GET /health/ready
GET /api/v1/status
GET /api/v1/core-services
GET /api/v1/nodes
```

### 6.3 Edge-Fallback

Historisch konfigurierte Required Services:

```text
subscriber-core
group-core
mobility-core
call-control
media-switch
sds-router
```

Logik:

- Gateway nicht verbunden -> Degraded/Isolated
- Health-Matrix fehlt oder ist älter als Lease -> Degraded/Isolated
- Required Service nicht `Available` -> Degraded
- alles gesund -> zunächst `Recovering`
- nach `recover_after_secs` -> `Online`

Historische Werte:

```toml
[edge_fallback]
enabled = true
enter_after_secs = 15
recover_after_secs = 20
unknown_service_is_available = false
service_matrix_lease_secs = 60
```

### 6.4 Aktuelle Main-Service-Matrix (Repository-Beispiel, 2026-10-05)

Die heutigen Beispielports auf `main` sind:

| Dienst | Beispielport |
|---|---:|
| Node Gateway | 8080 |
| Mobility Core | 8090 |
| Subscriber Core | 8100 |
| Group Core | 8110 |
| Call Control | 8120 |
| Media Switch | 8130 |
| Recorder | 8140 |
| SDS Router | 8150 |
| Packet Core | 8160 |
| IP Gateway | 8170 |
| Security Core | 8180 |
| KMF | 8190 |
| Transit | 8200 |
| Observability | 8210 |
| Application Gateway | 8220 |
| Media Library | 8230 |
| IoT Gateway | 8240 |
| Hardware Gateway | 8250 |
| RF Monitor | 8260 |
| Alarm Workflow | 8270 |
| Task Workflow | 8280 |
| Asset Management | 8290 |
| SIP Switch | 8300 |
| Control Room | 9010 |

Diese Tabelle beschreibt den **heutigen Source-/Beispielstand**, nicht zwingend die realen IPs/Ports des historischen Labors.

---

## 7. Hardware- und RF-Parameter

### 7.1 SXceiver / SoapySX

Historisch verwendeter SoapySX-Commit:

```text
9705147dd8c189625071f3f163ea56119bda4a05
```

Im Laufzeitlog:

```text
SoapySX version 9705147 9705147dd8c189625071f3f163ea56119bda4a05
Hardware version 1.2
```

Geräteargument:

```text
driver=sx,label=sx
```

Der Treiber öffnet im verwendeten Stand direkt:

```text
/dev/spidev0.0
/dev/gpiochip0
ALSA CARD=SX1255
```

### 7.2 RF-Werte aus dem bestätigten Laufzeitlog

```text
Main Carrier:      720
Secondary Carrier: 721
DL:                418.000000 MHz
UL:                408.000000 MHz
Secondary DL:      418.025000 MHz
Secondary UL:      408.025000 MHz
```

Der Log zeigte:

```text
Derived BS carriers: [(720, 418000000, 408000000), (721, 418025000, 408025000)]
```

Historisch im Config-Snapshot:

```toml
tx_freq = 418000000
rx_freq = 408000000
sample_rate = 600000
tx_center_freq = 418012500
rx_center_freq = 408012500
```

Netzparameter:

```text
MCC 901
MNC 1510
Location Area 1
Colour Code 1
```

---

## 8. Frischer Raspberry Pi: Build- und Installationsablauf

### 8.1 Grundsystem

**Status: vorgeschlagen; wesentliche Teile anschließend durch erfolgreichen Build/Start indirekt bestätigt**

Vorgesehen war Raspberry Pi OS Lite 64-bit, Hostname `SRV-M-TBS-01`, Benutzer `jan`, SPI aktiviert.

Wichtige Paketgruppen:

```text
git
curl
build-essential
cmake
pkg-config
clang
libclang-dev
libssl-dev
libasound2-dev
alsa-utils
libsoapysdr-dev
soapysdr-tools
python3-soapysdr
ffmpeg
nftables
iproute2
nfs-common
```

### 8.2 SoapySX

**Status: gebaut und Laufzeitinitialisierung belegt**

Ablauf:

```bash
git clone https://github.com/tejeez/sxxcvr.git /opt/sxxcvr
cd /opt/sxxcvr
git checkout 9705147dd8c189625071f3f163ea56119bda4a05

cmake -S SoapySX -B SoapySX/build   -DCMAKE_BUILD_TYPE=Release   -DCMAKE_INSTALL_PREFIX=/usr/local

cmake --build SoapySX/build --parallel "$(nproc)"
sudo cmake --install SoapySX/build
sudo ldconfig
```

Prüfung:

```bash
SoapySDRUtil --probe="driver=sx,label=sx"
```

### 8.3 tetra-codec

**Status: gebaut; erfolgreicher finaler Link nach Installation von libgsm1-dev indirekt bestätigt**

Verwendeter Source:

```text
https://github.com/outerplane/tetra-codec.git
```

Im Installationsablauf wurde Commit `21d884064478d63306ec654378e666ae41503d00` verwendet.

### 8.4 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs   | sh -s -- -y --profile minimal --default-toolchain stable

source "$HOME/.cargo/env"
```

### 8.5 NetCore-Tetra ausschließlich über Git

**Status: vom Nutzer ausdrücklich gefordert und im funktionierenden Aufbau verwendet**

Historisch:

```bash
git clone   --branch swmi   --single-branch   https://github.com/JanHG98/netcore-tetra.git   /opt/netcore-tetra
```

Später wurde der Branch auf den jeweils gewünschten Commit zurückgesetzt.

Wichtig: Der Branch `swmi` existiert am 2026-10-05 nicht mehr; heutige Fortsetzung muss von `main` ausgehen oder einen neuen Arbeitsbranch von `main` erzeugen.

### 8.6 Build

Final funktionierender Buildaufruf:

```bash
CARGO_BUILD_JOBS=2 cargo build   --release   -p bluestation-bs   --features   "bluestation-bs/asterisk,bluestation-bs/recording,bluestation-bs/audio-player"
```

`--locked` wurde **verworfen**, weil der damalige Workspace eine Aktualisierung von `Cargo.lock` benötigte.

### 8.7 Installation der Binary

```bash
sudo install -m 0755   target/release/bluestation-bs   /usr/local/bin/bluestation-bs
```

Wichtige Pfade:

```text
Source:    /opt/netcore-tetra
Config:    /etc/netcore/config.toml
Fallback:  /etc/netcore/config.toml.fallback
Binary:    /usr/local/bin/bluestation-bs
Service:   tetra.service
```

---

## 9. Fehlerfall 1: Linker findet `-lgsm` nicht

### Symptom

```text
/usr/bin/ld: cannot find -lgsm: No such file or directory
collect2: error: ld returned 1 exit status
```

### Ursache

Der native `tetra-codec` wurde gelinkt und erwartete die GSM-Development-Bibliothek. Installiert war nicht die für den Linker benötigte Development-Datei.

### Funktionierende Lösung

**Status: angewendet; anschließend gelangte das System bis zum laufenden Basisstationsprozess**

```bash
sudo apt update
sudo apt install -y libgsm1-dev
sudo ldconfig
```

Danach konnte der Cargo-Build ohne vollständiges `cargo clean` fortgesetzt werden.

### Dauerhafte Lehre

`libgsm1-dev` gehört in die vollständige Neuinstallations-Paketliste.

---

## 10. Fehlerfall 2: `Failed to open SPI` trotz erkannter Hardware

### Symptom

Der hochgeladene Log zeigt mehrfach:

```text
Trying to open a device with arguments: driver=sx, label=sx
SoapySX version 9705147 ...
Hardware version 1.2
Skipping a SoapySDR device because opening failed: Other: Failed to open SPI
Failed to open SDR device: NotSupported: No supported devices found
```

Die Basisstation startete daraufhin per systemd immer wieder neu.

### Diagnose

Wichtig war die Reihenfolge:

1. SoapySX-Modul wurde geladen.
2. HAT-ID wurde gelesen.
3. Hardwareversion 1.2 wurde erkannt.
4. erst `open("/dev/spidev0.0", O_RDWR)` scheiterte.

Damit waren „Treiber fehlt“ und „HAT nicht erkannt“ ausgeschlossen.

### Tatsächliche Ursache

Das Packet-Data-Systemd-Drop-in enthielt bereits:

```ini
PrivateDevices=no
DeviceAllow=/dev/net/tun rw
```

Sobald `DeviceAllow=` aktiv ist, entsteht effektiv eine Geräte-Allowlist. Dadurch war der TUN-Zugriff erlaubt, aber der SXceiver wurde vom Service-Sandboxing ausgesperrt.

Betroffen waren:

```text
/dev/spidev0.0
/dev/gpiochip0
ALSA-Character-Devices
```

### Implementierte Lösung

Commit:

```text
60cb7438d6e82f4faa9cc814a752380e3acafc5e
```

Ergänzt wurden:

```ini
DeviceAllow=/dev/net/tun rw
DeviceAllow=/dev/spidev0.0 rw
DeviceAllow=/dev/gpiochip0 rw
DeviceAllow=char-alsa rw
```

### Betriebsbestätigung

**Status: im Betrieb bestätigt**

Nach dem Fix meldete der Nutzer:

- Basisstation läuft,
- Funkgerät verbindet sich,
- Sepura verbindet sich,
- Motorola verbindet sich,
- Hytera verbindet sich,
- SIP funktioniert wieder.

Damit ist der Systemd-Gerätezugriffsfix der stärkste bestätigte Betriebsmeilenstein dieses Chats.

### Heutiger Repository-Stand

Der Fix ist am 2026-10-05 weiterhin auf `main` vorhanden, einschließlich `spidev0.0`, `gpiochip0` und `char-alsa`.

---

## 11. Funkprotokoll-Korrektur: v25 Common-SCCH war falsch

### Früherer Ansatz

**Status: verworfen/überholt**

v25 setzte:

```rust
if class.clch_needed || class.common_scch {
    Some(0x01u64)
}
```

auch für StayAlive.

### Spätere Korrektur

Der funktionierende Main-Pfad behandelt StayAlive anders:

```text
StayAlive
-> ordinary MCCH
-> kein Frame-18 Common-SCCH-Eintrag
```

Nur echte Eg1..Eg3-Energy-Economy-Zuteilungen erhalten Common-SCCH.

### Heutige Verifikation

Der am 2026-10-05 geprüfte `main`-Code enthält weiterhin genau diese korrigierte Logik und den Debug-Hinweis:

```text
is StayAlive; keeping it on the ordinary MCCH instead of assigning frame-18 common-SCCH
```

Damit ist diese spätere Korrektur nicht nur historisch, sondern im aktuellen Source noch gültig.

---

## 12. Gewünschter Gruppenrufablauf

Der als Ziel festgelegte lokale Ablauf:

```text
PTT
-> U-SETUP
-> D-CALL-PROCEEDING
-> D-CONNECT

PTT release
-> U-TX CEASED
-> D-TX CEASED
-> Sepura verlässt TX-Anzeige sofort

nach Hangtime
-> D-RELEASE
-> Traffic Channel schließt
```

Historischer Standard für Hangtime:

```text
5 Sekunden
```

Dieser Ablauf sollte lokal bleiben und nicht durch eine zentrale zweite Call-FSM verändert werden.

---

## 13. Betriebsstand nach RF-/SPI-Reparatur

### Im Betrieb bestätigt

Nach Commit `60cb7438...`:

- SXceiver startet unter systemd.
- Sepura registriert.
- Motorola registriert.
- Hytera registriert.
- SIP funktioniert wieder.

### Nur teilweise beziehungsweise nicht abschließend bestätigt

Nicht eindeutig durch Langzeittest belegt:

- vollständiges Ende der zuvor beobachteten Sepura-Rereg-Schleifen über längere Laufzeit,
- vollständiger Mehrgeräte-Gruppenruf-Dauerlasttest,
- alle Floor-Retake-/Hangtime-Varianten,
- Multi-Site/Call-Restore mit angebundener SWMI.

---

## 14. SWMI-/Fallback-Diagnose am Ende des Chats

Dies ist der wichtigste **offene Punkt**, an dem der Chat endete.

### 14.1 Nutzerbeobachtung

Die Basisstation meldete weiterhin lokalen Fallback, obwohl der Node Gateway eingetragen war.

### 14.2 Diagnoseausgabe der TBS

Die vom Nutzer ausgeführte Diagnose zeigte:

```text
Control Room: False
Node Gateway: http://10.0.1.179:8080/ws/node
Node ID: SRV-M-TBS-01
Fallback aktiviert: True
Erforderliche Dienste:
subscriber-core, group-core, mobility-core, call-control, media-switch, sds-router
```

Der Node Gateway selbst meldete:

```text
known_nodes: 1
connected_nodes: 0
stale_nodes: 0
backend_clients: 8
monitored_services: 17
available_services: 17
degraded_services: 0
unavailable_services: 0
```

Für den Node:

```text
SRV-M-TBS-01 connected=False
```

Alle sechs Required Services waren `available` und antworteten mit HTTP 200:

```text
subscriber-core available
group-core      available
mobility-core   available
call-control    available
media-switch    available
sds-router      available
```

### 14.3 Schlussfolgerung

**Status: Diagnose belegt, Reparatur im Chat noch nicht durchgeführt**

Die Core-Infrastruktur war zu diesem Zeitpunkt gesund. Der unmittelbare Grund dafür, dass die TBS nicht online ging, war die lokale TBS-Konfiguration:

```toml
[control_room]
enabled = false
```

Damit startete der Control-Room-/Node-Gateway-Worker nicht beziehungsweise stellte keine aktive Node-Verbindung her. Das erklärt zugleich:

```text
connected_nodes: 0
SRV-M-TBS-01 connected=False
```

Der Nutzer hatte den Node Gateway zwar eingetragen, aber `enabled` war im tatsächlich gelesenen `/etc/netcore/config.toml` noch `false`.

### 14.4 Historischer Repository-Snapshot bestätigt die Ursache

Der hochgeladene SWMI-ZIP-Snapshot auf Commit `60cb7438...` enthält in `config.toml` ebenfalls:

```toml
[control_room]
enabled = false
host = "10.0.20.10"
port = 8080
endpoint_path = "/ws/node"
central_sds_routing = false
```

Der Laufzeitwert `host = 10.0.1.179` war lokal bereits angepasst, `enabled` jedoch nicht.

### 14.5 Was als nächster Reparaturschritt erforderlich gewesen wäre

**Status: offen; nicht mehr im Chat ausgeführt**

Auf der TBS:

```toml
[control_room]
enabled = true
host = "10.0.1.179"
port = 8080
use_tls = false
endpoint_path = "/ws/node"
node_id = "SRV-M-TBS-01"
```

Danach:

```bash
sudo systemctl restart tetra.service
```

und prüfen:

```bash
sudo journalctl -u tetra.service -f -o cat   | grep --line-buffered -E   'ControlRoom transport|hello accepted|core-service|edge fallback transition|service plane'
```

Erwarteter Ablauf:

```text
ControlRoom transport connected
ControlRoom hello accepted
... Recovering ...
... Online / central service plane healthy ...
```

### 14.6 Heutiger Repository-Stand

Im aktuellen `main`-Commit `7137e0dd...` steht die Beispielkonfiguration bereits auf:

```toml
[control_room]
enabled = true
host = "10.0.1.179"
port = 8080
use_tls = false
endpoint_path = "/ws/node"
node_id = "SRV-M-TBS-01"
central_sds_routing = true
```

Damit ist die **Source-Vorlage heute korrigiert**. Das beweist jedoch nicht, dass eine heute laufende TBS unter `/etc/netcore/config.toml` denselben Wert besitzt.

---

## 15. Aktueller Repository-Stand am 2026-10-05

### 15.1 Branchlage

Live geprüft wurden:

```text
Archiving
main
```

Der historische Branch `swmi` ist am 2026-10-05 nicht mehr vorhanden. Deshalb ist der Nutzerlink `.../tree/swmi` heute kein gültiger Fortsetzungsbranch mehr.

### 15.2 Aktueller Main-Commit

```text
7137e0dd69877e1b604bf89148fd8b6b590c1a97
```

Commit-Beschreibung:

```text
Merge pull request #59 from JanHG98/feat/netcore-dashboard-design
NetCore-Design mit Dark Mode für Basisstation und alle Dienst-WebUIs
```

### 15.3 Historische Fixes, die heute noch vorhanden sind

Live verifiziert:

- StayAlive bleibt auf ordinary MCCH.
- Common-SCCH wird für StayAlive nicht erzwungen.
- Systemd-Packet-Gateway-Drop-in erlaubt `/dev/spidev0.0`, `/dev/gpiochip0` und `char-alsa`.
- `control_room.enabled = true` in der aktuellen Main-Beispielkonfiguration.
- Edge-Fallback mit Required-Service-Matrix ist weiterhin vorhanden.
- Control-Room-Worker wartet nach Socket-Verbindung auf eine frische Service-Matrix und wechselt erst nach Hysterese auf `Online`.

### 15.4 Gegenüber dem historischen Chat weiterentwickelte Bereiche

Der aktuelle Main-Beispielstand enthält zusätzliche zentrale Dienste und Fallback-Einträge, unter anderem:

- IoT Gateway
- Hardware Gateway
- RF Monitor
- Alarm Workflow
- Task Workflow
- Asset Management
- SIP Switch

Diese waren im historischen Chat nicht alle Bestandteil der unmittelbaren Fehlersuche und sollten nicht rückwirkend als damals getestet interpretiert werden.

---

## 16. Relevante Konfigurations- und Runtime-Pfade

```text
/opt/netcore-tetra
/etc/netcore/config.toml
/etc/netcore/config.toml.fallback
/usr/local/bin/bluestation-bs
/etc/systemd/system/tetra.service
/etc/systemd/system/tetra.service.d/20-packet-data-gateway.conf
/usr/local/libexec/netcore-tetra-packet-gateway-cleanup
/var/lib/netcore/
/var/lib/flowstation/
```

Packet-Data/TUN:

```text
/dev/net/tun
```

SXceiver:

```text
/dev/spidev0.0
/dev/gpiochip0
ALSA SX1255
```

---

## 17. Relevante Befehle und ihr tatsächlicher Status

| Befehl/Ablauf | Status im Chat |
|---|---|
| Git-Clone des SWMI-Branches | **getestet/angewendet** |
| Cargo-Build mit `--locked` | **fehlgeschlagen**; Lockfile nicht synchron |
| Cargo-Build ohne `--locked` | **erfolgreich genug für laufende Binary belegt** |
| `libgsm1-dev` installieren | **Lösung angewendet; danach finaler Linkfehler überwunden** |
| SoapySX Commit `9705147...` bauen/installieren | **Laufzeitmodul und Hardwareerkennung belegt** |
| Packet-Gateway-Drop-in mit nur TUN | **fehlerhaft; blockierte SXceiver** |
| Drop-in mit SPI/GPIO/ALSA-Allowlist | **implementiert und in Betrieb bestätigt** |
| Sepura-Verbindung | **im Betrieb bestätigt** |
| Motorola-Verbindung | **im Betrieb bestätigt** |
| Hytera-Verbindung | **im Betrieb bestätigt** |
| SIP | **im Betrieb bestätigt** |
| SWMI Online | **nicht erreicht; Abschlussdiagnose zeigt control_room.enabled=false** |
| längerer Rereg-/Gruppenruf-Dauertest | **nicht abschließend belegt** |
| vorgeschlagenes Tag `working-pi-radio-sip-2026-07-29` | **nur vorgeschlagen; keine Ausführung belegt** |

---

## 18. Verworfene oder ersetzte Ansätze

### 18.1 ZIP-Overlay als Installationsweg

**Verworfen**, weil der Nutzer ausdrücklich Git-basierte Builds fordert.

### 18.2 v25 Frame-18-SCCH für StayAlive

**Verworfen**, weil dieser Ansatz den tatsächlich funktionierenden Main-Pfad falsch abbildete und später wieder entfernt wurde.

### 18.3 `cargo build --locked` im damaligen Workspace

**Verworfen für die Erstinstallation**, weil `Cargo.lock` nicht synchron zum Workspace war.

### 18.4 Fokus auf angeblich defekten SDR-Treiber beim SPI-Fehler

**Verworfen**, nachdem Log und Source zeigten, dass SoapySX geladen und Hardware 1.2 erkannt wurde. Ursache war systemd-Gerätefilterung.

### 18.5 Fallback durch Deaktivieren von Edge-Fallback „lösen“

**Nicht gewünscht.** Der Fallback-Mechanismus ist Bestandteil der Architektur; stattdessen muss die TBS sauber `Online` werden, wenn Gateway und Required Services verfügbar sind.

---

## 19. Offene Aufgaben und Roadmap-Kandidaten

### Priorität 1 – SWMI-Verbindung tatsächlich online bringen

**Status: offen**

1. auf der realen TBS `/etc/netcore/config.toml` prüfen:
   ```toml
   [control_room]
   enabled = true
   ```
2. TBS neu starten.
3. Node Gateway auf aktive TBS-Session prüfen.
4. Hello/HelloAck prüfen.
5. frische Service-Matrix prüfen.
6. Übergang `Recovering -> Online` nach Hysterese prüfen.

### Priorität 2 – Stabilität nach zentraler Anbindung erneut testen

**Status: geplant**

Mit aktivem Node Gateway und Online-Core:

- Sepura registriert und bleibt stabil,
- Motorola registriert und bleibt stabil,
- Hytera registriert und bleibt stabil,
- lokale Gruppenrufe senden/empfangen,
- SIP TETRA->SIP und SIP->TETRA,
- keine neue Rereg-Schleife durch Core-Status,
- kein SYSINFO-Flattern bei Backend-Reconnect,
- lokaler Ruf bleibt bei Backend-Ausfall bestehen.

### Priorität 3 – SWMI-Funktionen schrittweise scharfstellen

**Status: geplant**

Erst nach stabiler Online-Verbindung:

- Subscriber Policy,
- Group Policy,
- zentraler SDS Router,
- Media Switch,
- Multi-TBS Call Control,
- Mobility/Cell Change,
- Call Restore.

Jede Aktivierung muss gegen den bestätigten lokalen RF-Baseline-Test regressionsgetestet werden.

### Priorität 4 – Build-/Installationsdokumentation korrigieren

**Status: Roadmap-Kandidat**

Dauerhaft in offizielle Installationsdokumentation aufnehmen:

- Git-only-Workflow,
- `libgsm1-dev`,
- SoapySX Commit-/Hardwareprobe,
- systemd DeviceAllow für TUN + SPI + GPIO + ALSA,
- kein pauschales `--locked` bei nicht synchronisiertem Lockfile,
- Control-Room-Enable als explizite Inbetriebnahmeprüfung.

### Priorität 5 – „Known Good“-Abnahme automatisieren

**Status: Idee/Roadmap-Kandidat**

Ein maschinenlesbarer Preflight könnte prüfen:

```text
Git commit
Binary hash
config parse
control_room.enabled
Node Gateway reachability
Required-service matrix
SoapySX probe
SPI/GPIO/ALSA rights
systemd effective DeviceAllow
RF startup marker
```

Damit ließen sich genau die in diesem Chat aufgetretenen Fehler vor dem On-Air-Test erkennen.

---

## 20. Testmatrix für die nächste Fortsetzung

### RF-Baseline

```text
[ ] Sepura Attach
[ ] Motorola Attach
[ ] Hytera Attach
[ ] 15-30 min ohne unerwartete Reregs
[ ] Gruppenruf A->B/C
[ ] Gruppenruf B->A/C
[ ] Gruppenruf C->A/B
[ ] PTT Release / TX CEASED
[ ] Hangtime
[ ] D-RELEASE
```

### SIP

```text
[ ] TETRA -> SIP
[ ] SIP -> TETRA
[ ] Rufende
[ ] erneuter Ruf ohne Neustart
```

### SWMI

```text
[ ] ControlRoom transport connected
[ ] Hello accepted
[ ] Node Gateway connected_nodes >= 1
[ ] Required Services available
[ ] Matrix lease erneuert
[ ] Recovering
[ ] Online
[ ] Backend-Ausfall -> Degraded/Fallback ohne RF-Abbruch
[ ] Backend-Rückkehr -> Recovering -> Online
```

### Hardware/Systemd

```text
[ ] /dev/spidev0.0 sichtbar
[ ] /dev/gpiochip0 sichtbar
[ ] ALSA SX1255 sichtbar
[ ] SoapySDRUtil probe als Service-User
[ ] effective DeviceAllow enthält TUN/SPI/GPIO/ALSA
```

---

## 21. Relevante Repository-Dateien

Historisch und/oder heute relevant:

```text
config.toml
config.toml.fallback

bins/bluestation-bs/src/main.rs

crates/tetra-entities/src/mm/mm_bs.rs
crates/tetra-entities/src/mle/mle_bs.rs
crates/tetra-entities/src/cmce/
crates/tetra-entities/src/umac/umac_bs.rs
crates/tetra-entities/src/net_control_room/worker.rs

contrib/packet-data/netcore-tetra-packet-gateway-install
contrib/packet-data/netcore-tetra-packet-gateway-cleanup
contrib/systemd/tetra.service.d/20-packet-data-gateway.conf

system-backend/node-gateway/
system-backend/subscriber-core/
system-backend/group-core/
system-backend/mobility-core/
system-backend/call-control/
system-backend/media-switch/
system-backend/sds-router/
```

---

## 22. Relevante Commits und Quellen

### Historische Chat-Commits

- `5fee15c276e96cd9e7b90e9ea4944c9f2662447d` — Restore exact main MM and local call setup on RF path
- `863068a9f74877a123b2cc42468d747261450879` — Keep RF SYSINFO independent from Core service-matrix churn
- `78257a4295dfac19f5d4e90817f156d3fc6a35c3` — Remove incorrect v25 common-SCCH profile markers
- `60cb7438d6e82f4faa9cc814a752380e3acafc5e` — Allow SXceiver devices in packet-gateway systemd sandbox

### Historische Anhänge

- `netcore-tetra-main(3).zip`
  - Snapshot-Metadatum: `0b84cc0c8340b4a4ea34778fa5a2653a682f9ae7`
- `netcore-tetra-swmi(9).zip`
  - Snapshot-Metadatum: `60cb7438d6e82f4faa9cc814a752380e3acafc5e`
- `Eingefügter Text(15).txt`
  - systemd-/SoapySX-Fehlerlog mit `Failed to open SPI`
  - Laufzeitversion `v1.3.0-78257a42`
  - Hardwareversion 1.2
  - Dual-Carrier-Ableitung 720/721

### Externe Source-Abhängigkeiten

- SoapySX/SXceiver: `tejeez/sxxcvr`, Commit `9705147dd8c189625071f3f163ea56119bda4a05`
- native TETRA-Sprachcodec: `outerplane/tetra-codec`

### Heutiger Repository-Stand

- `main`: `7137e0dd69877e1b604bf89148fd8b6b590c1a97`
- historische `swmi`-Branch-Referenz: nicht mehr vorhanden
- Archivierung erfolgt ausschließlich in `Archiving`

---

## 23. Sicherheits- und Betriebsgrenzen

Der im Chat verwendete zentrale Core lief im Open-Lab-Modus. Für einen produktionsnahen Betrieb sind zusätzliche Sicherheitsmaßnahmen erforderlich. Diese Abschlussdokumentation enthält bewusst keine Zugangsdaten.

Wichtig für spätere Fortsetzung:

- Open-Lab-WebSockets nicht ins öffentliche Netz exponieren.
- zentrale Policy erst aktiv erzwingen, wenn Synchronisation und Recovery robust abgenommen sind.
- Hardwarezugriffe nicht durch Systemd-Hardening versehentlich erneut abschneiden.
- Core-Ausfälle dürfen den lokalen RF-Taktpfad nicht blockieren.

---

## 24. Kompakte Abschlussbewertung

Der Chat erreichte einen wichtigen stabilen Zwischenstand:

**Im Betrieb bestätigt:**

```text
Pi + SXceiver + MAIN-COMPAT RF
Sepura OK
Motorola OK
Hytera OK
SIP OK
```

Die entscheidenden technischen Korrekturen waren:

1. falschen v25-Common-SCCH-Ansatz zurücknehmen,
2. Main-MM und lokalen CMCE-Rufpfad wiederherstellen,
3. RF-SYSINFO von Core-Service-Churn entkoppeln,
4. fehlendes `libgsm1-dev` für den finalen Link ergänzen,
5. systemd-DeviceAllow für SPI/GPIO/ALSA korrigieren.

**Nicht abgeschlossen** wurde die Rückkehr von Local Fallback zu SWMI Online. Die Abschlussdiagnose ist jedoch eindeutig: Node Gateway und Required Services waren gesund, aber die tatsächlich gelesene TBS-Konfiguration hatte `control_room.enabled = false`, wodurch keine aktive Node-Verbindung bestand.

Für die Fortsetzung ist daher **keine neue RF-Rewrite-Runde** erforderlich. Der erste Schritt ist die saubere Aktivierung und Verifikation der bereits vorhandenen Node-Gateway-Verbindung bei unverändertem, bestätigtem RF-Baseline-Verhalten.
