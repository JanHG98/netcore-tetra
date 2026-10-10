# Brainstorming: Basisstation Clean Install, SXceiver/SoapySX, SNDCP und Healthchecks

**Archivhinweis, 09.10.2026:** Diese Datei dokumentiert das im Titel beziehungsweise Quellenrahmen genannte Gespräch und dessen damalige Entscheidungen. Historische Kommandos, Planungen, Versionsangaben und Fehlerbilder bleiben erhalten; der heutige technische Stand steht in der [Gesamtroadmap](../roadmaps/gesamtroadmap.md) und in den [aktuellen Dienstanleitungen](../services/README.md). Die Archivaufnahme bestätigt keine spätere Implementierung oder Live-Abnahme.

**Arbeitsstand:** 2026-10-05. Historische Betriebsbeobachtungen und der an diesem Datum geprüfte Repository-Stand sind getrennt ausgewiesen.

## Arbeitsstand

| Feld | Wert |
|---|---|
| Thema | Neuaufbau einer NetCore-Tetra-Basisstation auf frischer SD-Karte; SXceiver/SoapySX-Fehlersuche; `RxReadError`; Raspberry-Pi-5-I²S/DMA und Unterspannung; SNDCP/TUN; Subscriber-Core-/Media-Library-Healthchecks |
| Zusammenfassung erstellt | 2026-10-05 |
| Historischer Betriebszeitraum in der Planung | vor allem 2026-08-02; spätere Archiv-/Projektfortsetzung am 2026-10-05 |
| Zielrepository | `JanHG98/netcore-tetra` |
| Zielbranch für dieses Archiv | `Archiving` |
| Zielbranch vor dem Archiv-Commit geprüft | `Archiving` @ `9af2de92b120dc612d83a17f78eeee9a97f2f775` |
| Zusätzlich geprüfter aktueller Codebranch | `main` @ `e5d825b33db2e1fce14bcb2cc23e73c241873a18` |
| Früher in der Planung verwendeter Branchname | `mqtt`; am 2026-10-05 nicht mehr als eigener Branch vorhanden, GitHub-Branchsuche lieferte keinen Treffer. Die REST-Abfrage für `mqtt` verwies auf `main`. |
| Belege | Laufzeit- und Diagnoselogs, später präzisierte Befunde, Textlog und Systemdienste-Screenshot. |

> **Stand der Arbeiten:** Laufzeitbeobachtungen, geplante Reparaturen und Repository-Befunde sind getrennt. Befehlsvorschläge bleiben offen, solange keine Ausgaben vorliegen; vorhandener Code allein bestätigt keine Installation oder Abnahme auf TBS-01.

## Statuslegende

- **Idee** – diskutierter Ansatz ohne verbindliche Umsetzungsentscheidung.
- **Beschlossen/geplant** – in der Planung als gewünschter oder nächster Weg festgelegt, aber nicht durch Code/Live-Ausgabe als umgesetzt belegt.
- **Implementiert** – im aktuell geprüften Repository vorhanden.
- **Getestet** – in der Planung durch konkrete Ausgabe eines Tests oder Dienststarts belegt.
- **Im Betrieb bestätigt** – in der Planung als funktionierender Live-Betrieb über einen relevanten Zeitraum bzw. anhand konkreter Dienst-/Funktionsausgabe belegt.

---

## 1. Ziel, Ausgangslage und behandelte Themen

Die Arbeit entwickelte sich von einer Basisstations-Reparatur zu einem bewusst vollständigen Neuaufbau. Die zentralen Ziele waren:

1. eine instabile Raspberry-Pi-5-/SXceiver-Basisstation reproduzierbar zu diagnostizieren,
2. SoapySX, ALSA/I²S und NetCore-PHY sauber voneinander zu trennen,
3. die Ursache des wiederkehrenden `RxReadError` nicht vorschnell NetCore zuzuschreiben,
4. nach erfolgreicher Isolation eine **neue Installation von Null an auf neuer SD-Karte ohne Backups** zu definieren,
5. den NetCore-Paketdatenpfad (`SNDCP` + Linux-TUN `ntetra0`) sauber in die neue Installation einzubinden,
6. anschließend die systemweiten NetCore-Dienste und deren Fallback-/Healthanzeige zu prüfen,
7. aktuelle Repository-Implementierungen gegen die historischen Annahmen abzugleichen.

Die Basisstation wurde im Verlauf mit einer dualen 400-MHz-Konfiguration betrieben. Die in den Logs und Konfigurationsständen wiederkehrenden Eckdaten waren:

```text
Host:             SRV-M-TBS-01
TBS-IP:           10.0.1.20
Raspberry Pi:     Raspberry Pi 5 Model B Rev 1.0
SDR:              SXceiver / SX1255, H/W 1.2 in erfolgreichen Probes
SoapySX:          9705147dd8c189625071f3f163ea56119bda4a05
Sample Rate:      600000 S/s
MCC/MNC:          901 / 1510
Location Area:    1
Colour Code:      1
Carrier:          720 / 721
DL:               418.000000 / 418.025000 MHz
UL:               408.000000 / 408.025000 MHz
RX center:        408.012500 MHz
TX center:        418.012500 MHz
```

---

## 2. Endgültige Anforderungen und Entscheidungen

### 2.1 Clean Install ohne Altlasten

**Status: beschlossen/geplant; teilweise durch spätere Shellausgaben als begonnen belegt.**

Die spätere ausdrückliche Entscheidung des Nutzers ersetzt frühere Reparatur-/Backup-Ansätze:

> neue Installation von 0 an, neue SD, keine Backups.

Daraus folgt für den dokumentierten Zielzustand:

- Raspberry Pi OS 64-bit frisch aufsetzen;
- keine alte `config.toml`, keine alte Signatur, keine alte Binary und kein altes `target/` zurückspielen;
- SoapySX aus dem fest bekannten Upstream-Commit neu bauen;
- native SDR-Funktion **vor** NetCore-Build und Dienststart testen;
- NetCore anschließend aus Git klonen und neu bauen;
- Integrationen stufenweise zuschalten, nicht alles gleichzeitig;
- während der ersten RF-Stabilitätsprüfung `Restart=no`, damit ein defekter Stream nicht in einer Neustartschleife wiederholt geöffnet/geschlossen wird;
- erst nach bestandener RF-Gate-Prüfung automatische Neustarts und zentrale Integrationen aktivieren.

### 2.2 Stromversorgung ist ein RF-/DMA-Blocker, kein Nebenthema

**Status: getestet; als harte Voraussetzung beschlossen.**

Der Pi meldete:

```text
throttled=0x50000
```

und im Kernel-Log wiederholt:

```text
hwmon hwmon3: Undervoltage detected!
hwmon hwmon3: Voltage normalised
```

Damit wurde die Stromversorgung als ernsthafte Ursache bzw. Mitursache für I²S-/DMA-Instabilität behandelt. Spätere Tests zeigten, dass nach einem sauberen Kaltstart der native RX-Stream tatsächlich stabil auf rund 600 kS/s lief.

**Endgültige Betriebsregel:** Ein sauberer Start für RF-Abnahme beginnt mit stabiler Versorgung und `vcgencmd get_throttled` ohne aktuelle/historische Unterspannungsbits seit dem frischen Power-Cycle.

### 2.3 HAT-EEPROM nicht blind überschreiben

**Status: beschlossen.**

Zwischenzeitlich wurde erwogen, das SXceiver-HAT-Overlay/EEPROM neu aufzubauen. Die endgültige Linie war konservativ:

- vorhandenen EEPROM nicht blind flashen;
- zunächst `/proc/device-tree/hat`, `aplay`, `arecord` und SoapySX-Probe prüfen;
- manuellen Overlay-Weg nur verwenden, wenn die Karte/Overlay-Erkennung fehlt;
- für Hardware 1.2 den Upstream-DTS-Stand mit `HAT_VERSION=0x0102` verwenden.

### 2.4 Python-SoapySDR war kein geeigneter Primärtest

**Status: verworfen/ersetzt.**

Die Python-Bindings lieferten trotz funktionierender nativer Probe:

```text
RuntimeError: SoapySDR::Device::make() no match
```

Daher wurde der Python-Test als separater Binding-/Library-Mismatch behandelt und der Hardwarepfad stattdessen mit `SoapySDRUtil` nativ getestet.

### 2.5 Asterisk zunächst deaktivieren, lokale SIP-Architektur später sauber herstellen

**Status: beschlossen; Asterisk-Build vorhanden, finaler Fallback-Livebetrieb für diesen Arbeitsstand nicht bestätigt.**

Ein früher Start scheiterte mit:

```text
Failed to start Asterisk SIP integration: failed to lookup address information: Name or service not known
```

Die Architekturentscheidung war:

- für RF-Basistests `[asterisk] enabled = false`;
- nicht direkt einen beliebigen zentralen Host als schnellen Fix einsetzen;
- später lokalen Asterisk-Fallback auf der TBS sauber installieren;
- lokaler Asterisk: typischerweise `127.0.0.1:5060`;
- zentraler SIP-Switch: `10.0.1.125:5060`, Management/WebUI `:8300`;
- PBX-Fallback: `10.0.1.21:5060`.

### 2.6 SNDCP/TUN nur mit korrekten Linux-Capabilities aktivieren

**Status: im Repository implementiert; Live-Abnahme für diesen Arbeitsstand offen.**

Der Paketdatenpfad soll ein Linux-TUN-Interface `ntetra0` erzeugen. Der frühere Dienststart meldete:

```text
SNDCP packet gateway stopped: ... Permission denied
SNDCP: packet gateway startup failed: WorkerStartup("I/O: Permission denied (os error 13)")
```

Später ergab:

```text
ip link show ntetra0
Device "ntetra0" does not exist.
```

In der Clean-Install-Strategie war das zunächst erwartbar, weil Paketdaten für den ersten RF-Gate-Test bewusst deaktiviert worden waren. Für die Aktivierung wurden `CAP_NET_ADMIN`, `/dev/net/tun`, Forwarding und die Paketdatenkonfiguration als Voraussetzungen festgelegt.

### 2.7 Health-WebUI und tatsächlicher Service-Health sind getrennt zu bewerten

**Status: getestet/diagnostiziert; Reparatur noch nicht bestätigt.**

Der letzte Dashboard-Screenshot zeigte:

- Subscriber Core: `AUSGEFALLEN`, Fallback aktiv;
- Media Library: `AUSGEFALLEN`, Fallback aktiv;
- zahlreiche andere Core-Dienste `ONLINE`;
- bei Subscriber Core: `connect failed: Connection refused (os error 111)`;
- bei Media Library: `read failed: Resource temporarily unavailable (os error 11)`.

Gleichzeitig funktionierte nach Betriebsrückmeldung die Media-Library-WebUI. Daraus wurde korrekt abgeleitet:

- WebUI-Erreichbarkeit ist kein Beweis für erfolgreichen Health-Check;
- der Node Gateway kann einen falschen/defekten Healthpfad oder einen transienten Socketfehler sehen, obwohl `/` im Browser funktioniert;
- Subscriber Core war dagegen objektiv gestoppt.

---

## 3. Historischer Ablauf der Planung

### 3.1 TBS startet bis in den RF-Pfad

Nach Build- und Konfigurationsarbeiten startete `tetra.service` und erkannte:

```text
Version: v1.3.0-5980ed3b
Radio runtime: MAIN-COMPAT
Derived BS carriers: [(720, 418000000, 408000000), (721, 418025000, 408025000)]
SoapySX version 9705147 ...
Hardware version 1.2
Detected clock as 38.4 MHz
Using settings ... fs: 600000.0
RX gains: LNA 42, PGA 16
TX gains: DAC 9, MIXER 30
RX center 408.012500 MHz
TX center 418.012500 MHz
MCC 901 / MNC 1510 / CC 1
```

**Status: getestet.**

Dashboard, Brew/TetraPack und weitere Runtime-Bausteine wurden initialisiert. Der erste harte Fehler dieser Phase war SNDCP `Permission denied`; kurz danach folgte der PHY-Panic.

### 3.2 NetCore-PHY-Panic `RxReadError`

Der Prozess beendete sich mit:

```text
thread 'main' ... panicked at crates/tetra-entities/src/phy/phy_bs.rs:334:56:
Got error from rxtx_timeslot: RxReadError
```

**Status: getestet.**

Wichtig war die spätere Codeanalyse: `soapyio.rs` verwirft den konkreten SoapySDR-Fehler und bildet jeden Read-Fehler auf `RxTxDevError::RxReadError` ab. Damit war die Logmeldung diagnostisch zu grob; sie bewies nicht, welcher ALSA-/Soapy-Fehler tatsächlich vorlag.

### 3.3 Geräte- und Rechteprüfung

Folgende Prüfungen waren erfolgreich:

```text
/proc/asound/cards:
 2 [SX1255] simple-card - SX1255

arecord -l:
 card 2, device 1 ... capture

aplay -l:
 card 2, device 0 ... playback
```

`jan` war Mitglied u. a. in:

```text
audio, spi, i2c, gpio, dialout, plugdev
```

`/dev/snd/*`, `/dev/spidev*` und `/dev/gpiochip*` hatten passende Gruppenrechte. `fuser` zeigte zu diesem Zeitpunkt keine belegenden Prozesse. PipeWire/WirePlumber liefen zwar, hielten aber nach `fuser` nicht die SX1255-Geräte offen.

**Status: getestet.**

### 3.4 SoapySDRUtil-Probe funktioniert, Python-Binding nicht

Als Benutzer `jan` und als `root` funktionierte:

```bash
SoapySDRUtil --probe="driver=sx"
```

mit:

```text
driver=sx
hardware=sx
hardware_version=1.2
Channels: 1 Rx, 1 Tx
Timestamps: YES
Sample rates: ... 0.6 MSps
```

Der Python-Aufruf `SoapySDR.Device({"driver":"sx"})` scheiterte dagegen. Dadurch wurde klar, dass Python für die Hardwarediagnose in diesem Zustand nicht vertrauenswürdig war.

### 3.5 Nativer RX-Test reproduziert Streamfehler

Ein nativer Rate-Test startete, brach aber zunächst sofort ab:

```text
Begin RX rate test at 0.6 Msps
Starting stream loop...
Unexpected stream error STREAM_ERROR
[INFO] Stopping and resetting streams
[ERROR]
ALSA error in alsa_tx.reset(): Input/output error
```

Im Kernel waren bereits Meldungen vorhanden:

```text
dma dma2chan4: dma2chan4 failed to stop
```

**Status: getestet.**

Damit war bewiesen, dass der Fehler unterhalb von NetCore reproduzierbar war. Ein NetCore-Codefehler war nicht mehr die einzige plausible Ursache.

### 3.6 Wiederholte Unterspannung entdeckt

Das Kernel-Log zeigte zahlreiche `Undervoltage detected!`/`Voltage normalised`-Wechsel. `vcgencmd get_throttled` ergab:

```text
throttled=0x50000
```

Dies wurde zum entscheidenden Betriebsbefund. Ein bloßer Software-Reboot wurde nicht mehr als ausreichende Resetbedingung betrachtet; empfohlen wurde ein echtes `poweroff` plus physisches Trennen der Versorgung.

### 3.7 Erfolgreicher RX-Test nach sauberem Kaltstart

Später lief der native RX-Test:

```text
0.599976 Msps   4.7998 MBps
0.599989 Msps   4.79991 MBps
```

**Status: getestet; RX-Grundpfad im frischen Boot bestätigt.**

Dabei erschien allerdings:

```text
[WARNING] Could not read HAT ID. Assuming hardware version 1.1
```

und beim Stop erneut:

```text
ALSA error in alsa_tx.reset(): Input/output error
```

Damit war der RX-Streamingpfad grundsätzlich funktionsfähig, aber das Stop/Reset-Verhalten und die HAT-ID-Erkennung blieben offene technische Punkte.

### 3.8 Entscheidung: neue SD, keinerlei Backups

Auf Projektwunsch wurde ein vollständiger Neuaufbau entworfen. Die vorgeschlagene Reihenfolge war:

1. Raspberry Pi OS Lite 64-bit frisch installieren;
2. Stromversorgung testen;
3. SoapySX aus Commit `9705147...` bauen;
4. HAT/ALSA prüfen;
5. nativen RX-Test durchführen;
6. `outerplane/tetra-codec` und `libgsm` installieren;
7. Rust installieren;
8. `JanHG98/netcore-tetra` frisch klonen;
9. `bluestation-bs` mit `asterisk,recording,audio-player` bauen;
10. neue bereinigte `config.toml` erzeugen;
11. `tetra.service` zunächst mit `Restart=no` betreiben;
12. RF-Gate für mindestens zehn Minuten;
13. dann Node Gateway/Edge Fallback, Media Library, NFS/Audio, SNDCP/WAP, Asterisk nacheinander aktivieren.

**Status: beschlossen/geplant.** Nicht jeder dieser Schritte ist in den verfügbaren Unterlagen durch eine erfolgreiche Ausgabe belegt.

### 3.9 `ntetra0` fehlt

Später wurde auf der neuen/fortgesetzten Installation ausgeführt:

```bash
ip link show ntetra0
```

Ergebnis:

```text
Device "ntetra0" does not exist.
```

Im Kontext der vorgeschlagenen Clean-Install-Konfiguration war dies zunächst konsistent, weil der Packet Data Gateway für den reinen RF-Ersttest deaktiviert worden war. Danach wurde das Aktivieren von:

```toml
[cell_info]
sndcp_service = true
advanced_link = true
circuit_mode_data_service = true

[cell_info.wap_ip]
enabled = true

[cell_info.packet_data_gateway]
enabled = true
interface_name = "ntetra0"
```

sowie `/dev/net/tun` und `CAP_NET_ADMIN` vorgeschlagen.

**Live-Erfolg von `ntetra0` wurde für diesen Arbeitsstand nicht mehr bestätigt.**

### 3.10 Systemweite Dienste: Subscriber Core und Media Library

Der letzte Screenshot zeigte die systemweite Dienstmatrix. Die meisten Dienste waren grün; Subscriber Core und Media Library waren rot/Fallback.

Subscriber Core wurde direkt im LXC geprüft:

```text
netcore-subscriber-core.service
Active: inactive (dead)
... code=killed, signal=TERM
... Deactivated successfully.
Warning: The unit file ... changed on disk. Run 'systemctl daemon-reload' ...
```

Interpretation:

- kein nachgewiesener Crash;
- Prozess wurde per `SIGTERM` sauber beendet;
- Unit/Drop-in wurde nach dem letzten daemon-reload verändert;
- Neustart/`daemon-reload` stand aus.

Für die Media Library galt dagegen: WebUI funktionierte laut Betriebsrückmeldung, aber der Healthcheck im TBS-/Node-Gateway-Dashboard scheiterte. Daher wurde eine Pfad-/Healthcheck-/Socketdiagnose empfohlen, nicht eine pauschale Annahme, der gesamte Dienst sei tot.

---

## 4. Architektur, Komponenten, Schnittstellen und Abhängigkeiten

### 4.1 Basisstation

```text
Raspberry Pi 5
  ├─ SXceiver/SX1255
  │   ├─ SPI (Steuerung)
  │   ├─ I²S / ALSA (RX/TX samples)
  │   └─ SoapySX → SoapySDR
  ├─ bluestation-bs
  │   ├─ PHY/MAC/MLE/MM/CMCE
  │   ├─ Dashboard :8080
  │   ├─ SNDCP + TUN ntetra0
  │   ├─ lokale Aufzeichnung / Audio
  │   ├─ Node-Gateway-Anbindung
  │   └─ optional lokaler Asterisk
  └─ systemd: tetra.service
```

### 4.2 Relevante zentrale Dienste im Live-Plan dieser Planung

| Komponente | Live-IP/Port im Projektkontext | Relevanz in der Planung |
|---|---:|---|
| TBS `SRV-M-TBS-01` | `10.0.1.20:8080` | Basisstation/Dashboard |
| Node Gateway | `10.0.1.179:8080` | Service-Matrix, zentrale Verbindung und Edge-Fallback |
| Subscriber Core | `10.0.1.153:8100` | Teilnehmerverwaltung; zuletzt gestoppt |
| Mobility Core | `10.0.1.150:8090` | Registrierung/Mobility; Dashboard grün |
| Group Core | `10.0.1.157:8110` | Gruppenverwaltung; Dashboard grün |
| Call Control | `10.0.1.155:8120` | Rufsteuerung; Dashboard grün |
| Media Switch | `10.0.1.159:8130` | Medienvermittlung; Dashboard grün |
| Recorder | `10.0.1.170:8140` | zentrale Aufzeichnung; Dashboard grün |
| SDS Router | `10.0.1.169:8150` | systemweites SDS/Statusrouting; Dashboard grün |
| Packet Core | `10.0.1.166:8160` | SNDCP/PDCH-Kontexte; Dashboard grün |
| IP Gateway | `10.0.1.142:8170` | TUN/TAP, Routing/NAT; Dashboard grün |
| Security Core | `10.0.1.149:8180` | Security Policy; Dashboard grün |
| KMF | `10.0.1.180:8190` | Schlüsselverwaltung; Dashboard grün |
| Transit | `10.0.1.151:8200` | Interconnect; Dashboard grün |
| Observability | `10.0.1.143:8210` | Logs/Health; Dashboard grün |
| Application Gateway | `10.0.1.144:8220` | Anwendungen/TTS/Integrationen; Dashboard grün |
| Media Library | `10.0.1.154:8230` | zentrale Aufnahmen/TTS/Audio; WebUI erreichbar, Healthcheck rot |
| Control Room | `10.0.1.156:9010` | Leitstelle |
| SIP Switch | `10.0.1.125:8300`, SIP `:5060/udp` | zentrale SIP-Vermittlung |
| PBX | `10.0.1.21:5060/udp` | Telefonie-Fallback |

> Die im aktuellen Repository enthaltenen Open-Lab-Inventarbeispiele verwenden teilweise ein anderes `10.0.20.x`-Adressschema. Diese Beispiele sind **nicht** mit dem hier beobachteten Live-`10.0.1.x`-Netz gleichzusetzen.

### 4.3 Edge Fallback

Die Basisstation verwendet eine Service-Matrix mit Fail-closed-Verhalten für unbekannte/stale Services. Relevante Konfigurationswerte aus dem in der Planung geprüften Stand:

```toml
[edge_fallback]
enabled = true
enter_after_secs = 15
recover_after_secs = 20
unknown_service_is_available = false
service_matrix_lease_secs = 60
keep_last_known_policy = true
required_services = [
  "subscriber-core",
  "group-core",
  "mobility-core",
  "call-control",
  "media-switch",
  "sds-router",
]
```

Bei Subscriber-Core-Ausfall bleibt die Zelle deshalb lokal handlungsfähig, verwendet aber Fallback-/Cache-/statische Regeln. Bei Media-Library-Ausfall bleiben lokale Medien/Cachefunktionen aktiv.

---

## 5. Relevante Dateien, Dienste, Pfade und Parameter

### Basisstation

| Zweck | Pfad / Wert |
|---|---|
| Repository | `/opt/netcore-tetra` |
| aktive Konfiguration | `/etc/netcore/config.toml` |
| Fallback-Konfiguration | `/etc/netcore/config.toml.fallback` |
| Binary | `/usr/local/bin/bluestation-bs` bzw. durch Unit tatsächlich gestarteter Pfad |
| systemd | `/etc/systemd/system/tetra.service` |
| lokale Recordings | `/var/lib/netcore/recordings` |
| lokale Audio Library | `/var/lib/netcore/audio` |
| Audio Cache | `/var/cache/netcore/audio` |
| Edge Policy Cache | `/var/lib/flowstation/edge-policy-cache.json` |
| Edge Event Spool | `/var/lib/flowstation/edge-event-spool.jsonl` |
| NFS-Mount | `/mnt/nfs-share` |
| Packet Data TUN | `ntetra0` |

### SXceiver / SoapySX

| Parameter | Wert |
|---|---|
| Upstream | `tejeez/sxxcvr` |
| fest verwendeter Commit | `9705147dd8c189625071f3f163ea56119bda4a05` |
| Soapy factory | `driver=sx` |
| im NetCore-Setup verwendete Device-Args | `driver=sx,label=sx` |
| Sample Rate | `600000` |
| RX Gain | `LNA=42`, `PGA=16` |
| TX Gain | `DAC=9`, `MIXER=30` |
| Stream-Period in NetCore | bei 600 kS/s: `900` Samples (1,5 ms) |
| Audio-Karte | `SX1255`, Playback device 0, Capture device 1 |

### TETRA-Codecs

Für den Feature-Build mit Asterisk/Recording/Audio wurden in der Planung als Abhängigkeiten festgehalten:

- `libgsm1-dev` / `libgsm`;
- `outerplane/tetra-codec` als native Shared Library;
- `pkg-config --libs tetra-codec` muss funktionieren;
- `ldconfig -p | grep tetra-codec` soll die installierte Library zeigen.

### Subscriber Core

| Zweck | Wert |
|---|---|
| systemd | `netcore-subscriber-core.service` |
| Binary | `/usr/local/bin/netcore-subscriber-core` |
| Config | `/etc/netcore/subscriber-core.toml` |
| State | `/var/lib/netcore-subscriber-core` |
| Port | `8100` |
| Standard Health | `/health/live`, `/health/ready` |

### Media Library

| Zweck | Wert |
|---|---|
| systemd | `netcore-media-library.service` |
| Config | `/etc/netcore/media-library.toml` |
| State | `/var/lib/netcore-media-library` |
| Port | `8230` |
| Standard Health | `/health/live`, `/health/ready` |
| zentrale TTS/Medienrolle | ja |

---

## 6. Wichtige Befehle und ihr tatsächlicher Status

### 6.1 Tatsächlich erfolgreich ausgeführt

#### ALSA/SX1255-Erkennung

```bash
sudo -u jan cat /proc/asound/cards
sudo -u jan arecord -l
sudo -u jan aplay -l
```

**Ergebnis:** SX1255 wurde als Karte 2 erkannt; Capture und Playback waren vorhanden.

#### Native SoapySX-Probe

```bash
sudo -u jan SoapySDRUtil --probe="driver=sx"
sudo SoapySDRUtil --probe="driver=sx"
```

**Ergebnis:** beide erfolgreich, Hardware 1.2, 1 RX/1 TX, 600 kS/s unterstützt.

#### Stromversorgungsdiagnose

```bash
vcgencmd get_throttled
```

**Ergebnis:** `throttled=0x50000` in der problematischen Bootphase; zusammen mit wiederholten Unterspannungs-Kernelmeldungen.

#### Nativer RX-Rate-Test nach Kaltstart

```bash
timeout --foreground --signal=INT 12s \
  sudo -u jan SoapySDRUtil \
    --args="driver=sx" \
    --rate=600000 \
    --direction=RX
```

**Ergebnis:** erfolgreiches Streaming mit `0.599976` und `0.599989 Msps`; danach weiterhin Fehler beim Reset der TX-ALSA-Seite.

#### Subscriber-Core-Status

```bash
systemctl status netcore-subscriber-core --no-pager --full
```

**Ergebnis:** `inactive (dead)`, zuvor sauber mit `SIGTERM` beendet; Unit-Datei/Drop-ins auf Disk verändert, `daemon-reload` erforderlich.

### 6.2 Tatsächlich ausgeführt, aber mit Fehler

#### Früher nativer RX-Test

```bash
SoapySDRUtil --args="driver=sx" --rate=600000 --direction=RX
```

**Ergebnis:** `Unexpected stream error STREAM_ERROR`, danach `alsa_tx.reset(): Input/output error`.

#### Python-SoapySDR-Geräteerzeugung

```python
SoapySDR.Device({"driver":"sx"})
```

**Ergebnis:** `RuntimeError: SoapySDR::Device::make() no match` trotz funktionierendem nativen Utility.

#### TUN-Prüfung

```bash
ip link show ntetra0
```

**Ergebnis:** `Device "ntetra0" does not exist.`

### 6.3 Nur vorgeschlagen / für diesen Arbeitsstand nicht als erfolgreich bestätigt

- vollständige frische SD-Installation aller Phasen bis zum finalen TBS-Dauerbetrieb;
- `systemctl daemon-reload && enable --now netcore-subscriber-core.service` nach dem letzten Screenshot;
- Curl-Abnahme von Subscriber Core und Media Library über `/health/live` und `/health/ready` vom Node Gateway;
- Aktivierung und erfolgreiche Erzeugung von `ntetra0`;
- lokaler Asterisk-Fallback und Registrierung zum zentralen SIP-Switch;
- RF-Monitor-Agent auf der neu installierten TBS;
- finale Signatur-/USB-Autorisierung nach Clean Install;
- 10-Minuten-/Langzeit-Gate der Basisstation nach komplettem Neuaufbau.

---

## 7. Fehler, Diagnose, Ursachen und Lösungen

### 7.1 Asterisk: Hostnameauflösung verhindert TBS-Start

**Symptom**

```text
Failed to start Asterisk SIP integration: failed to lookup address information: Name or service not known
```

**Diagnose:** Ungültiger/nicht auflösbarer Asterisk-Zielhost; der Fehler lag vor dem späteren RF-Fehler.

**Funktionierender Zwischenweg:** Asterisk für den RF-Test deaktivieren.

**Endgültiger Architekturweg:** lokalen Asterisk-Fallback korrekt installieren und erst danach `[asterisk]` aktivieren.

### 7.2 SNDCP-Packet-Gateway: `Permission denied`

**Symptom**

```text
SNDCP packet gateway stopped: ... Permission denied
```

**Diagnose:** TUN-/Netzwerkadministrationsrechte fehlten. Der Paketdatenpfad benötigt `/dev/net/tun` und `CAP_NET_ADMIN`.

**Geprüfter Repository-Befund:** Der aktuelle Code nennt bei `TUNSETIFF` explizit `CAP_NET_ADMIN` und verweist auf `contrib/packet-data/netcore-tetra-packet-gateway-install`. Die Dokumentation beschreibt den systemd-Drop-in mit `CAP_NET_ADMIN`, `/dev/net/tun`, Sysctl-Zugriff und Cleanup.

**Live-Status:** Noch nicht bestätigt, da `ntetra0` in der Planung zuletzt fehlte.

### 7.3 `RxReadError` in NetCore

**Symptom**

```text
Got error from rxtx_timeslot: RxReadError
```

**Diagnose:** Nicht ausreichend spezifisch. Der aktuelle Code verwirft den konkreten SoapySDR-Readfehler (`Err(_)`) und liefert nur `RxReadError` weiter.

**Konsequenz:** Logs sollten künftig den originalen SoapySDR-/ALSA-Fehlercode und Fehlertext mitführen. Das ist ein klarer Roadmap-Kandidat.

### 7.4 `STREAM_ERROR` im nativen Soapy-Test

**Symptom:** Auch ohne NetCore war der RX-Stream zunächst sofort fehlerhaft.

**Diagnose:** Damit war ein Problem unterhalb der NetCore-PHY-Schicht bewiesen. Parallel gab es `dma2chan4 failed to stop`.

**Spätere Erkenntnis:** Nach Kaltstart und Behandlung der Stromversorgung lief RX stabil. Das schwächt die Hypothese eines rein deterministischen NetCore-Codefehlers deutlich.

### 7.5 Raspberry Pi 5 Unterspannung

**Symptome:**

```text
Undervoltage detected!
Voltage normalised
throttled=0x50000
```

**Bewertung:** zentraler Hardware-/Versorgungsfehler. Kurze Spannungseinbrüche können I²S/DMA/ALSA destabilisieren, selbst wenn das System anschließend wieder „normal“ meldet.

**Funktionierender Diagnoseweg:** echter Power-Cycle statt nur Software-Reboot; danach nativer RX-Stream vor NetCore testen.

### 7.6 HAT-ID nach Kaltstart nicht lesbar

**Symptom:**

```text
Could not read HAT ID. Assuming hardware version 1.1
```

**Widerspruch:** Frühere Probes erkannten Hardware 1.2 korrekt.

**Status:** offen. RX funktionierte trotzdem. EEPROM/Overlay-Erkennung muss getrennt vom Streamingpfad geprüft werden.

### 7.7 ALSA-TX-Resetfehler nach reinem RX-Test

**Symptom:**

```text
ALSA error in alsa_tx.reset(): Input/output error
```

**Diagnose:** SoapySX öffnet/verwaltet RX und TX gemeinsam und setzt beim Deaktivieren beide Seiten zurück. Der Fehler trat nach erfolgreichem RX-Streaming auf.

**Status:** offen. Relevanz insbesondere für wiederholte Stop/Start-Zyklen und systemd-Restarts.

### 7.8 Subscriber Core rot/Fallback

**Symptom:** Dashboard `Connection refused`; systemd `inactive (dead)`.

**Diagnose:** Dienst wurde sauber mit SIGTERM beendet, nicht als Crash belegt. Zusätzlich meldete systemd eine veränderte Unit/Drop-ins auf Disk.

**Vorgeschlagene Reparatur:** `systemctl daemon-reload`, anschließend `enable --now netcore-subscriber-core.service`, Port `8100` und Health-Endpunkte prüfen.

**Status:** Reparaturausgang für diesen Arbeitsstand nicht mehr gezeigt.

### 7.9 Media Library rot, obwohl WebUI funktioniert

**Symptom:** Dashboard/Service-Matrix meldete `read failed: Resource temporarily unavailable (os error 11)`, funktionierende WebUI durch Betriebsrückmeldung bestätigt.

**Diagnose:** Healthpfad/Readiness/Socketfehler oder Node-Gateway-Healthworker; nicht automatisch kompletter Dienststillstand.

**Geprüfter Repository-Befund:** Standardmarker sind `/health/live` und `/health/ready`; die Betriebsdokumentation verwendet für Media Library `http://127.0.0.1:8230/health/live`.

**Offen:** direkter Curl-Test lokal und vom Node Gateway; danach Gateway-Healthworker prüfen.

---

## 8. Durchgeführte Tests und Grenzen

| Test | Ergebnis | Status | Grenze |
|---|---|---|---|
| SoapySX Device Probe als `jan` | H/W 1.2, 38.4 MHz, RX/TX erkannt | getestet | kein Dauerstream |
| SoapySX Device Probe als `root` | ebenfalls erfolgreich | getestet | kein Rechteproblem nachweisbar |
| ALSA-Kartenliste | SX1255 vollständig sichtbar | getestet | sagt nichts über DMA-Dauerstabilität |
| `/dev`-Rechte/Gruppen | `jan` hatte relevante Gruppen | getestet | ACL-/systemd-Sandbox kann separat wirken |
| `fuser /dev/snd/*` | kein Prozess hielt Geräte offen | getestet | Momentaufnahme |
| Python SoapySDR | `Device::make() no match` | getestet/fehlgeschlagen | Bindingsproblem, kein Hardwarebeweis |
| nativer RX-Test vor sauberem Kaltstart | `STREAM_ERROR` | getestet/fehlgeschlagen | System hatte bereits DMA-/Unterspannungsereignisse |
| `get_throttled` | `0x50000` | getestet | historische Bits zeigen Ereignis, nicht exakten Zeitpunkt der RF-Störung |
| nativer RX-Test nach Kaltstart | ca. 0.6 MSps stabil über Testdauer | getestet/erfolgreich | nur RX; Stop/Reset war weiterhin fehlerhaft |
| TBS-Start | erreichte RF-/Dashboard-/Brew-Initialisierung | getestet | endete später mit `RxReadError` |
| `ntetra0` | nicht vorhanden | getestet | Packet Gateway war im Clean-RF-Profil bewusst deaktiviert bzw. noch nicht erfolgreich aktiviert |
| Subscriber Core status | `inactive (dead)` | getestet | Neustart danach nicht in der Planung belegt |
| Media Library WebUI | laut Betriebsrückmeldung erreichbar | im Betrieb bestätigt für WebUI | keine Bestätigung der Health-/Readiness-API |

---

## 9. Verworfene oder ersetzte Ansätze

### Reines „NetCore ist schuld“

**Verworfen.** Der native SoapySDR-Rate-Test reproduzierte den Streamfehler außerhalb NetCore.

### Wiederholte Software-Neustarts als vollständiger Reset

**Ersetzt durch Kaltstart/Power-Cycle.** Die DMA-/ALSA-Lage und Unterspannungsereignisse machten einen echten Power-Cycle diagnostisch notwendig.

### Python als primäres Soapy-Testwerkzeug

**Verworfen.** Native `SoapySDRUtil`-Tests waren verwertbar, Python-Binding nicht.

### Blindes Schreiben des HAT-EEPROMs

**Verworfen.** Vorhandene Hardware wurde zuvor als 1.2 erkannt; daher erst Diagnose, nur bei tatsächlichem EEPROM-/Overlay-Problem schreiben.

### Sofortiges Aktivieren aller Integrationen auf frischer SD

**Ersetzt durch Gate-Modell.** Erst RF-Grundpfad, dann Node Gateway, Media Library/NFS, Packet Data, Asterisk, Signatur.

### Direkter zentraler Asterisk-Zielhost als schneller Fix

**Ersetzt durch lokale Fallback-Architektur.** TBS soll später lokalen Asterisk verwenden; dieser registriert/bridged zentral.

### Wiederherstellen alter Backups auf der neuen SD

**Explizit verworfen.** Neuaufbau ohne Backups ausdrücklich festgelegt.

---

## 10. Geprüfter Repository-Abgleich (2026-10-05)

### 10.1 Branches und Commits

- `Archiving` war vor diesem Archiv-Commit auf `9af2de92b120dc612d83a17f78eeee9a97f2f775`.
- `main` wurde geprüft auf `7137e0dd69877e1b604bf89148fd8b6b590c1a97` (`Merge pull request #59 ... NetCore-Design mit Dark Mode ...`).
- Ein eigener Branch `mqtt` ist am Prüftag nicht mehr vorhanden; Branchsuche ergab keinen Treffer. Historische historische Befehle, die `mqtt` voraussetzen, sind daher **nicht** unverändert auf den geprüften Repositoryzustand zu übertragen.

### 10.2 PHY / SoapyIO

**Implementiert, aber Diagnose-Lücke weiterhin vorhanden:**

`crates/tetra-entities/src/phy/components/soapyio.rs` enthält im aktuellen `main` weiterhin sinngemäß:

```rust
Err(_) => Err(RxTxDevError::RxReadError)
```

Der konkrete SoapySDR-Fehler wird damit im NetCore-Layer weiterhin nicht sichtbar gemacht. Die in der Planung identifizierte Logging-Lücke ist also am aktuellen `main` noch relevant.

### 10.3 SXceiver-Defaults

`crates/tetra-entities/src/phy/components/soapy_settings.rs` enthält weiterhin SXceiver-spezifische Einstellungen mit 600-kS/s-Default/Override und 1,5-ms-Block-/Periodenlogik. Bei 600 kS/s entspricht dies 900 Samples pro Periode.

### 10.4 Packet Data / `ntetra0`

Aktueller `main` enthält:

- `crates/tetra-entities/src/sndcp/packet_gateway.rs` mit Zugriff auf `/dev/net/tun`;
- expliziten Fehlerhinweis, dass `TUNSETIFF` `CAP_NET_ADMIN` benötigt;
- `contrib/packet-data/netcore-tetra-packet-gateway-install` als vorgesehene systemd-Integration;
- Dokumentation `Docs/PACKET_DATA_COMPLETE_INTEGRATION_2026-07-21.md` mit `ntetra0`, Routing/NAT, Cleanup und Diagnose.

Damit ist die Entwicklungsidee nicht nur Konzept: **der Packet-Data-Gateway ist im Repository implementiert**. Nicht bestätigt ist lediglich, dass er auf der in der Planung neu aufgebauten TBS bereits korrekt installiert und gestartet wurde.

### 10.5 Subscriber Core

Aktueller `main` enthält:

- `system-backend/subscriber-core/install/install.sh`;
- Build von `netcore-subscriber-core`;
- Konfiguration `/etc/netcore/subscriber-core.toml`;
- Port `8100`;
- `systemctl daemon-reload`;
- `systemctl enable --now netcore-subscriber-core.service`;
- systemd `Restart=on-failure`, `RestartSec=3`;
- State-Pfad `/var/lib/netcore-subscriber-core`.

Der letzte Live-Zustand in der Planung (`inactive dead`) entspricht daher **nicht** dem vom aktuellen Installer vorgesehenen Sollzustand.

### 10.6 Media Library

Aktueller `main` enthält eine gehärtete `netcore-media-library.service` mit:

- dediziertem Benutzer/Gruppe `netcore-media-library`;
- State-/Runtime-/ConfigurationDirectory;
- `Restart=on-failure`;
- Schreibrechten nur auf die vorgesehenen lokalen/NFS-Pfade;
- zentraler Piper-/TTS-Abhängigkeit;
- Installations-/Migrationshelfern.

Die Standard-Healthpfade im aktuellen System sind `/health/live` und `/health/ready`. Der Screenshotfehler ist daher am Prüfdatum gezielt gegen diese Pfade zu prüfen.

### 10.7 Basisstations-Updater

`install/update-basisstation.sh` ist weiterhin vorhanden und:

- erkennt den tatsächlichen aktiven Binary-Pfad;
- baut als expliziter `BUILD_USER`;
- testet Media-Library-Parserfälle;
- ersetzt exakt die von systemd verwendete Binary;
- sichert die alte Binary;
- startet und prüft den Dienst;
- kann bei Fehlern zurückrollen;
- deaktiviert standardmäßig lokalen Piper zugunsten zentraler TTS-Assets.

**Hinweis:** Für den ausdrücklich gewünschten Clean Install ohne Backups ist dieser Updater erst **nach** einer erfolgreichen Erstinstallation relevant.

---

## 11. Noch relevante Ideen, Wünsche und offene Aufgaben

### P0 – vor weiterer Funktionsabnahme

1. **Stromversorgung dauerhaft stabilisieren und dokumentieren.**
   - frischer Boot: `vcgencmd get_throttled` kontrollieren;
   - keine erneuten `Undervoltage detected!`-Meldungen unter RF-/CPU-Last;
   - Netzteil/Kabel als Teil der TBS-Hardwarebaseline dokumentieren.

2. **Stop/Start-Verhalten von SoapySX/SX1255 klären.**
   - erfolgreicher RX-Dauerstream ist belegt;
   - `alsa_tx.reset(): Input/output error` nach RX-only bleibt offen;
   - wiederholte Stream-Start/Stop-Zyklen testen;
   - prüfen, ob `dma2chan4 failed to stop` reproduzierbar zurückkehrt.

3. **NetCore-PHY-Fehlerlogging verbessern.**
   - originalen SoapySDR-Fehlercode/-text nicht mehr vollständig auf `RxReadError` reduzieren;
   - Fehlerursache im Journal sichtbar machen;
   - Overflow/Timeout/Stream Error differenzieren.

### P1 – Clean-Install-Baseline

4. **Komplette Clean-Install-Prozedur Ende-zu-Ende abnehmen.**
   - neue SD ohne Restore;
   - SoapySX → Codec → Rust → NetCore → systemd;
   - mindestens 10 Minuten RF-Stabilität, später Langzeittest;
   - `NRestarts=0` während Gate.

5. **HAT-ID-Widerspruch auflösen.**
   - warum wurde einmal 1.2 erkannt und später „ID nicht lesbar / assume 1.1“?
   - `/proc/device-tree/hat` und Overlayquelle prüfen;
   - EEPROM nur bei eindeutigem Befund anfassen.

6. **`ntetra0` vollständig in Betrieb nehmen.**
   - `contrib/packet-data/netcore-tetra-packet-gateway-install tetra.service` bzw. äquivalente aktuelle Installation nutzen;
   - `/dev/net/tun`, Capabilities, Sysctls, Firewall prüfen;
   - `ip link`, Route, NAT und PDP-Aktivierung mit realem Funkgerät testen.

### P1 – zentrale Dienste

7. **Subscriber Core wieder aktivieren und Health prüfen.**
   - `daemon-reload` nach veränderter Unit;
   - Dienst starten/enable;
   - `:8100/health/live` und `/health/ready` lokal und vom Node Gateway prüfen;
   - Ursache des vorherigen Stopps nachvollziehen (Update/Deployment/Operatoraktion).

8. **Media-Library-Healthproblem trotz funktionierender WebUI auflösen.**
   - `:8230/health/live`, `/health/ready` lokal;
   - dieselben Endpunkte vom Node Gateway;
   - Node-Gateway-URL/Pfad/Timeout kontrollieren;
   - `os error 11` (EAGAIN) auf Socket-/Worker-Ebene untersuchen;
   - danach Service-Matrix aktualisieren/verifizieren.

9. **Service-Matrix/Fallback-UI gegen tatsächlichen Healthzustand abnehmen.**
   - Fallback darf nicht allein anhand erfolgreicher `/`-WebUI aufgehoben werden;
   - Healthpfade, Readiness und Matrix-Lease konsistent halten.

### P2 – Integrationen

10. **Lokalen Asterisk-Fallback sauber installieren und erst dann Asterisk in der TBS aktivieren.**
11. **Media Library/Recorder/Audio-Playout nach RF-Baseline aktivieren.**
12. **RF-Monitor-Agent und Observability erst nach stabiler Basis anbinden.**
13. **Signatur-/USB-Workflow zuletzt neu aufbauen; keine alte Signatur übernehmen.**

### Roadmap-Kandidaten aus dieser Planung

- standardisierter `netcore-tbs-preflight` für Stromversorgung, HAT-ID, ALSA, SoapySX, `/dev/net/tun`, Capabilities und zentrale Healthchecks;
- `SoapyIo`-Fehlertelemetrie mit originalem Fehlercode;
- optionaler systemd-Preflight, der bei `get_throttled != 0x0` nicht blind RF startet;
- SoapySX-Start/Stop-Regressionstest für Raspberry Pi 5;
- Node-Gateway-Healthdiagnose, die „TCP refused“, „read EAGAIN“, HTTP-Status und falschen Pfad getrennt ausgibt;
- Clean-Install-Runbook mit Gates statt monolithischem Installationsskript.

---

## 12. Konkrete nächste Schritte in vereinbarter Priorität

1. **Subscriber Core:** `daemon-reload`, Start, Port/Health lokal und vom Node Gateway belegen.
2. **Media Library:** Healthpfade direkt testen; WebUI-Status von Readiness trennen; Node-Gateway-Konfiguration prüfen.
3. **TBS:** Versorgung weiterhin auf Unterspannung überwachen und SoapySX-Stop/Start mehrfach testen.
4. **PHY-Logging:** konkreten SoapySDR-Fehler im Code sichtbar machen, bevor weitere schwer reproduzierbare RF-Probleme untersucht werden.
5. **Packet Data:** systemd-Gateway-Installer/Capabilities anwenden, `ntetra0` erzeugen und SNDCP/PDP mit realem MS testen.
6. **Danach erst** Asterisk, Media Library/Audio, RF Monitor und Signatur/USB vollständig zuschalten.

---

## 13. Relevante Repository-Dateien und Quellen

### Aktueller Repository-Stand (`main` @ `7137e0dd...`)

- `crates/tetra-entities/src/phy/components/soapyio.rs`
- `crates/tetra-entities/src/phy/components/soapy_settings.rs`
- `crates/tetra-entities/src/sndcp/packet_gateway.rs`
- `Docs/PACKET_DATA_COMPLETE_INTEGRATION_2026-07-21.md`
- `Docs/basisstation.config.sanitized.example.toml`
- `install/update-basisstation.sh`
- `system-backend/subscriber-core/install/install.sh`
- `system-backend/subscriber-core/systemd/netcore-subscriber-core.service`
- `system-backend/media-library/install/install.sh`
- `system-backend/media-library/systemd/netcore-media-library.service`
- `system-backend/services.toml`
- `tools/check_full_system_integration.py`
- `Docs/NetCore-Tetra-Komplettguide.md`

### Externe Quellen/Repos, die in der Planung technisch verwendet wurden

- `tejeez/sxxcvr`, Commit `9705147dd8c189625071f3f163ea56119bda4a05`
- `outerplane/tetra-codec`

### Branch-/PR-Hinweise

- historisch wurde `mqtt` als aktiver Entwicklungsbranch verwendet;
- dieser Branch existiert am 2026-10-05 nicht mehr separat;
- geprüfter `main` enthält PR #59 als letzten geprüften Commitstand;
- dieses Archiv wird ausschließlich im Branch `Archiving` abgelegt und nicht gemerged.

---

## 14. Anhänge und Bilder dieser Planung

### 14.1 Systemdienste-/Fallback-Screenshot
Der in der Planung gezeigte Screenshot dokumentiert den Dienstmatrix-Zustand mit Subscriber Core und Media Library in Rot/Fallback bei gleichzeitig zahlreichen grünen Diensten.

![Systemdienste: Subscriber Core und Media Library im Fallback](assets/2026-10-05_basisstation-clean-install-sxceiver-soapysx-sndcp-healthchecks/systemdienste-fallback-subscriber-media-library.jpg)

Archivdatei:

`Docs/archive/assets/2026-10-05_basisstation-clean-install-sxceiver-soapysx-sndcp-healthchecks/systemdienste-fallback-subscriber-media-library.jpg`

Der ursprüngliche Originalupload war ein PNG mit 1700×886 Pixeln. Die verfügbare GitHub-Schnittstelle konnte in diesem Lauf den ursprünglichen Original-Binärblob nicht direkt übernehmen. Deshalb wurde eine 180×94-Pixel-JPEG-Ableitung desselben Screenshots archiviert. Der visuelle Inhalt bleibt als Archivvorschau erhalten; die Datei ist ausdrücklich **nicht byteidentisch** mit dem Original.

SHA-256 des ursprünglichen hochgeladenen PNGs:

```text
e16d8994a7a9dd3db9a50793603080a85eaa278612cd1ad3f41f0bc5c38ed36f
```

SHA-256 der archivierten JPEG-Ableitung:

```text
336d51fb2599894a445eb6fd652219335220f5a8d92f4c04e87d2cb060dc6825
```

### 14.2 Eingefügtes Textlog

Das hochgeladene Textlog mit dem nativen SDR-Test, PipeWire-Abschaltung, Pi-/Kernelinformationen, Unterspannungsereignissen und DMA-Ausgaben wird zusätzlich als Rohanhang archiviert:

`Docs/archive/assets/2026-10-05_basisstation-clean-install-sxceiver-soapysx-sndcp-healthchecks/eingefuegter-text-21.txt`

SHA-256 der Originaldatei:

```text
ccaaac9f8bbc7355123c6bc34600805e8c00b671a642e18c9389986256625ce8
```

---

## 15. Offene Nachweise und Grenzen

- Einige frühe Diagnose-/Installationsschritte liegen nur in der verdichteten verfügbaren Projekthistorie vor; wo keine konkrete Shellausgabe vorlag, wurden sie ausdrücklich als vorgeschlagen/geplant markiert.
- Es gibt keine belastbare Bestätigung im verfügbaren Verlauf, dass die komplette spätere Clean-Install-Prozedur bis zum finalen Langzeitbetrieb vollständig abgeschlossen wurde.
- Es gibt keine abschließende Ausgabe, dass `ntetra0` nach Aktivierung tatsächlich existierte.
- Es gibt keine abschließende Ausgabe, dass Subscriber Core nach `daemon-reload`/Start wieder `ONLINE` wurde.
- Es gibt keine abschließende Ausgabe, dass der Media-Library-Healthcheck vom Node Gateway wieder erfolgreich war; lediglich die WebUI war laut Betriebsrückmeldung erreichbar.
- Die aktuellen Repository-Prüfungen beziehen sich auf `main` @ `7137e0dd...`; der Archivbranch dient nur der Dokumentation und kann einen anderen Code-Snapshot enthalten.
- Die Projektumgebung enthielt weitere ETSI-PDFs und andere Dateien, die für diesen konkreten operativen Diagnosezeitraum nicht ausschlaggebend waren und daher nicht erneut in dieses Archiv kopiert wurden.

---

## 16. Sicherheits- und Betriebsnotizen

- Keine Passwörter, Tokens, privaten Schlüssel oder echten Zugangsdaten wurden in diese Archivdatei übernommen.
- Beispiel-/Platzhalter-Credentials aus Konfigurationsvorlagen sind nicht als produktive Zugangsdaten dokumentiert.
- RF-Betrieb, Frequenzen, Sendeleistung, Antennen und eventuelle Testlasten müssen weiterhin rechtlich und technisch zulässig betrieben werden.
- Full-Duplex-/TX-Tests sind getrennt von RX-only-Tests zu betrachten; die erfolgreiche RX-Abnahme beweist nicht automatisch einen sicheren/sauberen TX-Dauerbetrieb.

---

## 17. Kurzfazit

Der wichtigste technische Erkenntnisgewinn dieser Planung ist die Trennung der Fehlerdomänen: Der ursprüngliche `RxReadError` war **nicht ausreichend spezifisch** und ließ sich zeitweise als nativer SoapySX-/ALSA-Streamfehler außerhalb NetCore reproduzieren. Gleichzeitig zeigte der Raspberry Pi 5 wiederholte Unterspannung. Nach einem sauberen Kaltstart lief der native SXceiver-RX-Pfad mit praktisch exakt 600 kS/s, womit Hardware und I²S grundsätzlich funktionsfähig sind. Offen bleiben insbesondere das Stream-Stop/Reset-Verhalten, die HAT-ID-Konsistenz, das detaillierte Fehlerlogging, `ntetra0`/CAP_NET_ADMIN sowie zwei zentrale Healththemen (Subscriber Core wirklich gestoppt, Media Library WebUI erreichbar aber Healthcheck rot).

Für die Fortsetzung ist daher nicht eine weitere pauschale Neuinstallation die höchste Priorität, sondern die **reproduzierbare Abnahme der einzelnen Gates**: stabile Versorgung → SoapySX Start/Stop → NetCore RF → Packet Data → zentrale Healthmatrix → Integrationen.
