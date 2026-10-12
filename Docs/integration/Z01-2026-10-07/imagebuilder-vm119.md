# VM119 Imagebuilder – Buildnachweise, NVMe-Recovery und offene Abnahmen

## Aktueller Nachweisstand vom 11.10.2026

Der vollständige ARM64-NetCore-Build an `1595259a2a76abfc9eff08842409156473b585a7` ist technisch dokumentiert: Erfolgsauftrag `f141c06d4b2f48699486e149c4887b16`, Artefakt `2e2603beacd942dfaddc7119b85a4af0`, 2.043.954.184 Bytes, SHA-256 und `Fertig:`-Abschlussmarker. Die spätere Dateiprüfung fand dieses fertige Image nicht; der passende Softwarecache wurde bestätigt. Der erfolgreiche Build und die aktuelle Verfügbarkeit der Datei sind deshalb getrennte Befunde.

VM119/scsi0 wurde anschließend nach NVMe_OSData verschoben. Kopierabschluss, Konfiguration und Live-QEMU-Backing auf `/dev/zvol/rpool/data/vm-119-disk-0` sind belegt. Der separat angenommene NVMe-Recoveryauftrag ist `4ebe40edf4b443f592e7f7ad28ad0624` / Build `6a0a67657cce49118c89f5b9479db29d`. Der nachfolgende Windows-Prüfbeleg ordnet das heruntergeladene Recoveryartefakt jetzt dieser Build-ID und Commit1595259 zu; Größe und vollständige SHA-256 stimmen mit dem Manifest überein. Der separate SQLite-Endstatus wurde nicht erneut eingefügt.

Am 11.10.2026 besteht der Betreiber-Prüfblock am Windows-PC: `PASS` für Build6a0a6765, lokaler Dateiname und vollständige SHA-256 sind angegeben. Download / Manifestzuordnung / komprimierte Dateigröße / Byteintegrität sind damit bestanden. Anschließend besteht der physische Pi-OS-/SD-/Firstboot-/SSH-/lokale Agententest auf srv-m-tbs-03. NTP ist inzwischen synchronisiert. Der Modulrechtefehler wurde durch chmod0755 der beiden Soapy-Verzeichnisse repariert: sx ist als netcore PRESENT, Hardware1.2 erkannt. Der anschließende GPIO-Abbruch wurde auf den fehlenden festen Pfad gpiochip0 zurückgeführt. Der Betreiber identifiziert RP1 per ioctl eindeutig als gpiochip15 /54 Leitungen und bestätigt die vollständige Probe als netcore mit temporärem Alias: Hardware1.2 /38,4MHz /1Rx1Tx. Reparaturfortschritt und dauerhafte Quellkorrekturen stehen im [Rechtenachtrag](#soapysx-verzeichnisrechte-auf-dem-physischen-pi). Der geprüfte GPIO-Helfer ist inzwischen installiert; die Zuordnung und vollständige Probe bestehen auch nach echtem Pi-Neustart. Die lesende RF-/TBS-Konfigurationsprüfung ist ebenfalls inzwischen bestanden; Gateway-HTTP und frische Dienstmatrix sind ebenfalls vom Pi bestätigt:15/17 available, alle6 kritischen verfügbar. Der anschließende manuelle TBS-Start / TX-Verarbeitung / Gateway-WebSocket mit passender Identität bestehen ebenfalls; die Anlauf-Skips bleiben in den drei TX-Berichten konstant. Der automatische TBS-Boot / Gateway-Wiederkehr sind anschließend ebenfalls bestätigt; ein neuer TX-Deadline-Warnungsstrom verhindert die TX-Kontinuitätsabnahme. Die Folgeprobe am12.10. bestätigt den Produktionsstillstand über15s bei geringem CPU-Verbrauch und normalen Pi-Thermalwerten. Der Dienst ist nun bestätigt gestoppt/deaktiviert, BinarySHA zugeordnet und Journald-Suppression belegt. Diagnosepatch PR67 mit nativen Tests / Release-Build bestanden; gezielter Pi-Binary-Build folgt. Konkreter RX-/Slot-Auslöser und Reparatur, Dauerlauf / On-Air / fachliche Core-Funktionen / Controller / VPN bleiben offen; media-library / observability sind im Monitor unavailable. Kein neuer Build oder erneuter Disk-Move ist für die bisherige Pi-Reparatur erforderlich. Frühere DNS-, dpkg-, Initramfs-, SQLite- und I/O-Blocker stehen unten als Chronik der jeweiligen Läufe.

[Aktuelle Imagebuilder-Anleitung](../../services/deployment-core/README.md) · [Integrationsübersicht](README.md) · [Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Physischer Pi-Erststart bestanden

Der Betreiber liefert am 11.10.2026 die tatsächliche SSH-Ausgabe von `jan@srv-m-tbs-03`. Sie belegt den Start des zuvor gegen Manifest geprüften Recoveryimages auf SD:

| Prüfung | Tatsächlicher Befund |
| --- | --- |
| OS / Architektur | Debian GNU/Linux12 (bookworm), Kernel `6.12.109+rpt-rpi-2712`, `aarch64` |
| Host / LAN | `srv-m-tbs-03`, eth0 UP mit `10.0.1.20/24` |
| Rootdateisystem | `/dev/mmcblk0p2`, 29G gesamt, 3.1G benutzt, 25G verfügbar |
| Firstboot | `Result=success`, `active/exited`; `resize2fs` meldet bereits 7.658.624 Blöcke zu4KiB, keine weitere Erweiterung nötig |
| Discovery / SSH | Beide `Result=success`, `active/running` |
| Stationsprofil | SRV-M-TBS-03, MCC901 / MNC1510 / ISSI4010003 / LA3 / CC1 |
| Imagequelle / Overlay | Commit `1595259a2a76abfc9eff08842409156473b585a7`, Hostname `srv-m-tbs-03`, `overlay=eeprom` |
| Lokaler Agent | `http://127.0.0.1:8321/health/ready`: `ok=true`, `ready=true`, `security_mode=open_lab` |

Die Dateisystemgröße und der erfolgreiche Erweiterungsausgang bestätigen die SD-Erweiterung zusätzlich zum Dienststatus. Der Imager-Schreibprüfbericht wurde nicht separat eingefügt; der tatsächliche SD-Boot ist belegt. Das lokale Ready-Ergebnis belegt den Agenten, noch keine SXceiver-/Funkbereitschaft oder Sichtbarkeit im Controller. Der Nachweis steht strukturiert unter `operator_physical_pi_firstboot_acceptance` im [Betreiberbeleg](evidence/vm119-imagebuilder-success-2026-10-09.json).

Die Firstboot-Journalzeilen tragen `Apr27 21:48:57–58`. Diese historischen Bootzeitstempel können vor der Zeitsynchronisierung entstanden sein; eine aktuell falsche Uhr ist damit nicht belegt. Der nachfolgende Betreiberbefund vom 11.10.2026 zeigt die aktuelle Uhr korrekt in Europe/Berlin, NTP aktiv und `System clock synchronized: yes`.

### Erste Hardwareprüfung unter dem Dienstbenutzer: inzwischen bestanden

Der gebaute Quellstand aktiviert `tetra.service` und startet die Funkanwendung nach erfolgreicher Hardwareprobe. Für die separate Hardwareprüfung den Dienst zunächst stoppen und dessen Autostart deaktivieren:

```bash
sudo systemctl disable --now tetra.service
```

Falls der SXceiver noch nicht montiert ist, den Pi mit `sudo poweroff` herunterfahren, anschließend die Stromversorgung trennen, den HAT anschließen und über LAN wieder starten. Ist er bereits montiert, direkt mit der Probe fortfahren. Beim erneuten SSH-Verbindungsaufbau die aktuelle DHCP-Adresse verwenden; zuletzt war es `ssh jan@10.0.1.20`.

Auf dem Pi:

```bash
timedatectl status

sudo -u netcore timeout --kill-after=2s 30s \
  SoapySDRUtil --probe='driver=sx'
```

Die Probe läuft als `netcore`, um die Hardwarezugriffsrechte des TBS-Dienstbenutzers mitzuprüfen. Erwartet werden ein erkanntes SX-Gerät und erfolgreiche Probe ohne `No devices found` oder `[ERROR]`. Sie initialisiert die Hardware; eine On-Air-Abnahme oder vollständige Funkdienstbereitschaft folgt daraus noch nicht. Den TBS-Dienst bis zur Prüfung von Hardwarebefund und RF-Konfiguration deaktiviert lassen. Nach dieser Prüfung ist der Rückweg `sudo systemctl enable --now tetra.service`; dies startet die Basisstation tatsächlich.

Nach dem nächsten Boot kann `netcore-firstboot.service` aufgrund seiner bereits vorhandenen Done-Datei `inactive/dead` sein. Das allein ist kein Rückfall der bestandenen Erststartabnahme.

Geprüfte Dienstquellen am Imagecommit1595259: [tetra.service](https://github.com/JanHG98/netcore-tetra/blob/1595259a2a76abfc9eff08842409156473b585a7/system-backend/deployment-core/image/systemd/tetra.service) und [check-hardware.py](https://github.com/JanHG98/netcore-tetra/blob/1595259a2a76abfc9eff08842409156473b585a7/system-backend/deployment-core/image/check-hardware.py).

## SoapySX-Verzeichnisrechte auf dem physischen Pi

**Betreiberbefund vom 11.10.2026, Z01.4:** Dasselbe SoapySDRUtil meldet als `netcore` alle Modulpfade als fehlend und keine Factories; als root lädt es `/usr/local/lib/SoapySDR/modules0.8/libSXSupport.so` und meldet Factory `sx`. Lib0.8.1-3 / API0.8.0 / ABI0.8 sind bei beiden identisch. `namei -l` zeigt:

| Pfad | Besitzer / Gruppe | Modus |
| --- | --- | --- |
| `/usr/local/lib/SoapySDR` | root /973 |0700 |
| `/usr/local/lib/SoapySDR/modules0.8` | root /973 |0700 |
| `/usr/local/lib/SoapySDR/modules0.8/libSXSupport.so` | root /973 |0644 |

Die Ursache der fehlenden Registrierung als `netcore` ist damit bestätigt: Die beiden Elternverzeichnisse sind nur für root zugänglich. Die bisherige Probe erreicht noch keine Hardwareinitialisierung. Der ARM64-Builder vererbt seine Worker-Umask0077 in das Gastrezept; die ursprüngliche Treiberprüfung läuft ausschließlich als root. [Strukturierter Laufzeitbeleg](evidence/pi-soapysx-permissions-2026-10-11.json).

**Gezielte Reparatur auf dem vorhandenen Pi:**

```bash
sudo chmod 0755 \
  /usr/local/lib/SoapySDR \
  /usr/local/lib/SoapySDR/modules0.8

sudo -u netcore timeout --kill-after=2s 15s \
  SoapySDRUtil --check=sx

sudo -u netcore timeout --kill-after=2s 30s \
  SoapySDRUtil --probe='driver=sx'
```

Moduldatei, Eigentümer und Gruppe benötigen nach diesem Befund keine Änderung. Die nachfolgende Betreiber-Ausgabe bestätigt das chmod durch `Checking driver 'sx'... PRESENT` als netcore. Die Probe lädt SoapySX9705147, erkennt Hardware1.2 und endet mit `Failed to open GPIO`. Der Modulzugriff ist damit repariert; die Hardwareprobe war zu diesem Zeitpunkt noch offen. Der TBS-Dienst bleibt bis zur RF-/Core-Prüfung deaktiviert.

Der gepinnte Treiber öffnet in `GpioChip` fest `/dev/gpiochip0` mit `O_RDWR`. Die Fehlermeldung entsteht unmittelbar beim fehlgeschlagenen open, vor den GPIO-Line-Requests. Der nächste Betreiberbefund bestätigt den fehlenden Pfad: Root und netcore melden beide errno2 / FileNotFoundError; udev findet keinen gpiochip0. Die vorhandenen gpiochip11–16 sind root:gpio0660, netcore gehört zur GPIO-Gruppe; SPI0 ist root:spi0660. Quelle: [gepinnte SoapySX.cpp](https://github.com/tejeez/sxxcvr/blob/9705147dd8c189625071f3f163ea56119bda4a05/SoapySX/SoapySX.cpp).

**Physische GPIO-Zuordnung und SXceiver-Probe bestanden:** Der Betreiber liest mit GPIO_GET_CHIPINFO_IOCTL alle nativen Character-Devices aus. Das Ergebnis vom 11.10.2026:

| Knoten | Label | Leitungen |
| --- | --- | --- |
| gpiochip11 | gpio-brcmstb@107d517c00 |17 |
| gpiochip12 | gpio-brcmstb@107d517c20 |6 |
| gpiochip13 | gpio-brcmstb@107d508500 |32 |
| gpiochip14 | gpio-brcmstb@107d508520 |4 |
| gpiochip15 | pinctrl-rp1 |54 |
| gpiochip16 | 7inch-touchscreen-p |2 |

Genau ein nativer RP1-Treffer und fehlender gpiochip0 erlauben den temporären Alias `/dev/gpiochip0 -> /dev/gpiochip15`. Die anschließende Soapy-Probe als netcore besteht: gepinnter Commit9705147dd8c189625071f3f163ea56119bda4a05, Hardware1.2, Takt38,4MHz, 1Rx/1Tx, TimestampsYES, CF32 und saubere Uninitialisierung. Gemeldete Raten sind25/50/75/150/300/600kSps. Es werden noch keine Stream-/RF-/Core-Ergebnisse behauptet.

**Dauerhafter GPIO-Bootweg:** [gpio-compat.py](../../../system-backend/deployment-core/image/gpio-compat.py) enthält einen kleinen Installationsweg und einen Laufzeithelfer. Installation schreibt nur Helfer / `netcore-sx-gpio-compat.service` / `tetra.service.d/10-gpio-compat.conf` und greift nicht auf GPIO zu; abweichende lokale Dateien werden vor jedem Schreiben erkannt und unverändert abgelehnt. Laufzeit gilt für Pi5/CM5 und identifiziert genau einen nativen pinctrl-rp1-Controller mit54 Leitungen über ioctl. Auf das Erscheinen dieses Controllers wird höchstens15 Sekunden gewartet. Ein bestehender gpiochip0 ist nur erlaubt, wenn seine Dateiidentität zum geprüften RP1 gehört; ein anderer realer Knoten, falscher oder verwaister Alias bleibt unverändert und beendet den Dienst mit Fehler. Die Berechtigungen root:gpio0660 bleiben erhalten.

Der root-Oneshot startet nach dem Udev-Trigger und vor tetra.service; `Requires` und `After` im tetra-Drop-in sorgen dafür, dass der Alias bereits vor der Einrichtung des vorhandenen DeviceAllow-Filters bereitsteht. Der Helfer fordert keine GPIO-Leitungen an und ändert keine Pinwerte. Der gepinnte Soapy-Treiber bleibt unverändert. Der eigene Gerätepfad mit Treiberoption bleibt eine spätere Verfeinerung; für den kontrollierten Bootweg ist kein Treiber-Rebuild nötig. Das Gastrezept installiert und aktiviert den Helfer auch in neuen Images, ohne VM-Hardwareprobe. Auf dem vorhandenen Pi sind Installation / Aktivierung und die nachfolgende Neustartprobe inzwischen durch den Betreiber bestätigt. Bis zur nachfolgend belegten Konfigurationsprüfung und dem manuellen Start war tetra deaktiviert.

Sechs [GPIO-Regressionen](../../../system-backend/deployment-core/tests/test_gpio_compat.py) bestehen: ioctl und FD-Abschluss, eindeutige native RP1-Auswahl, Aliasanlage / Wiederholung, Erhalt kollidierender Pfade, Dateirechte / Installations-Vorprüfung / Dienstreihenfolge sowie Modell- / Root-Guard und begrenzter Wartepfad. Dateien und Symlinks sind reale temporäre Fixtures, Character-Device-Metadaten und ioctl-Antworten werden simuliert. Zusammen mit den bisherigen23 Fällen bestehen alle29 Deployment-Core-Tests; Bash-/Python-Syntax ist gültig. Diese lokalen Tests werden durch die folgende tatsächliche Pi-Neustartprobe ergänzt.

**Physische GPIO-Neustartabnahme bestanden, 11.10.2026:** Der Operatorblock lädt [Helferstand6b43cce](https://github.com/JanHG98/netcore-tetra/blob/6b43cce1c7ff8151833e2d6db3b68d8add7a16fb/system-backend/deployment-core/image/gpio-compat.py) und meldet die Prüfsumme `133c4d6d433c67ff5aee0726d487282faf19ad370d38da710fc3b883153d33e0` als OK. Installation und Aktivierung sind erfolgreich; tetra wird deaktiviert / gestoppt. Der Pi kündigt um22:08:15 CEST den echten Neustart an, die SSH-Verbindung endet, und der Betreiber meldet sich anschließend erneut auf10.0.1.20 an.

| Prüfung nach dem Neustart | Betreiberbefund |
| --- | --- |
| GPIO-Helfer | ExecMainStatus=0, ActiveState=active, SubState=exited |
| Erwarteter Treiberpfad | readlink -f /dev/gpiochip0 → /dev/gpiochip15 |
| Probe als netcore | Vollständig bestanden; SoapySX9705147 / Hardware1.2 /38,4MHz /1Rx1Tx |
| Abschluss | Uninitializing SoapySX ohne Fehler |

Der GPIO-Bootweg und die Hardwareinitialisierung sind auf diesem Pi abgenommen. Die Loginmeldung zum WLAN-Rfkill ist ein separater WLAN-Befund und widerspricht der LAN-/GPIO-Abnahme nicht. Die nachfolgende lesende RF-/TBS-Konfigurationsprüfung ist unten bestätigt; Gateway-HTTP und der anschließende manuelle TBS-Stream / Gateway-Handshake sind inzwischen ebenfalls bestätigt. Das generierte Stationsprofil enthält bereits belegte MCC901 / MNC1510 / ISSI4010003 / LA3 / CC1; Konfiguration und tatsächlicher Start stehen gesondert unten. Der folgende automatische TBS-Boot / Gateway-Wiederkehr sind inzwischen ebenfalls bestätigt; aktuell folgt die Diagnose neuer TX-Deadline-Ereignisse, danach Dauerlauf / On-Air / Controller / VPN. Die Pi-Reparatur bedeutet keine bereits ausgeführte VM119-Quellübernahme oder neue Imageausgabe.


**Lesende RF-/TBS-Konfigurationsprüfung bestanden, 11.10.2026:** Der Betreiber führt die reine Python-Vorschau als netcore aus. Sie liest `/etc/netcore/config.toml`, berücksichtigt vorhandene Laufzeitedits über `launch.merge_runtime_edits` und die gecachten bekannten Abhängigkeiten über `bindings.resolve_config`, ohne `launch.main`, Funkstack oder GPIO aufzurufen und ohne Dateien zu schreiben. Quellkonfiguration und nächste Startvorschau melden dieselben Werte:

| Bereich | Betreiberbefund |
| --- | --- |
| Stack / Backend | Bs / SoapySdr |
| Netz / Zelle | MCC901 / MNC1510 / LA3 / CC1 |
| Träger | main720 / secondary721, Band4 / Offset0 / Duplexindex0 / reverse=false |
| RF-Hauptträger | TX418.000MHz / RX408.000MHz |
| RF-Mitten / Sample | TX418.0125MHz / RX408.0125MHz /600kSps |
| Gerätewahl / Overrides | Kein explizites Gerät; keine Gain-, Kanal- oder Antennenüberschreibungen gemeldet |
| Gatewaytransport | enabled=true / host10.0.1.179 / port8080 / use_tls=false |
| Station / zentrale SDS | Node-ID und Stationsname SRV-M-TBS-03 / central_sds_routing=true |
| Discovery | Cache als netcore lesbar / keine neu gebundenen Felder |
| TBS-Dienst | User=netcore / inactive / disabled; ExecStart führt über launch.py |
| Gerätefilter | DevicePolicy=auto; char-alsa / gpiochip0 / spidev0.0 / net-tun jeweils rw |

`dual_carrier_enabled` ist nicht gesetzt und erscheint deshalb in der rohen Vorschau als null. Der gebaute [Zellparser](https://github.com/JanHG98/netcore-tetra/blob/1595259a2a76abfc9eff08842409156473b585a7/crates/tetra-config/src/bluestation/sec_cell.rs) verwendet dafür true; der Sekundärträger bleibt aktiv. Die [Frequenzumsetzung](https://github.com/JanHG98/netcore-tetra/blob/1595259a2a76abfc9eff08842409156473b585a7/crates/tetra-core/src/freqs.rs) ergibt die beiden TX-Träger418.000/418.025MHz und RX-Träger408.000/408.025MHz. Für Band4 / Duplexindex0 beträgt der Abstand10MHz; der alte 5MHz-Kommentar in der Beispielkonfiguration ist irreführend. Die12,5kHz versetzten Mitten sind für die benachbarten Träger passend und kein zusätzlicher Kontrollkanal.

Ohne individuelle Overrides setzt [SdrSettings](https://github.com/JanHG98/netcore-tetra/blob/1595259a2a76abfc9eff08842409156473b585a7/crates/tetra-entities/src/phy/components/soapy_settings.rs) beim späteren Start RX-/TX-Kanal0, Antennen RX/TX, RX-LNA42dB / PGA16dB und TX-DAC9dB / MIXER30dB. Die [Soapy-PHY](https://github.com/JanHG98/netcore-tetra/blob/1595259a2a76abfc9eff08842409156473b585a7/crates/tetra-entities/src/phy/components/soapyio.rs) wendet diese Werte tatsächlich an. Eine fehlende Geräteauswahl bedeutet automatische Auswahl des ersten erfolgreich geöffneten unterstützten SDR. Die erfolgreiche einzelne SX-Probe ersetzt dennoch keinen TBS-Streamingnachweis.

**Gateway-HTTP-Nachweis:** Das Gatewayziel aus der Konfiguration ist inzwischen vom Pi aus als laufender Node-Gateway bestätigt. Die [HTTP-Handler](https://github.com/JanHG98/netcore-tetra/blob/1595259a2a76abfc9eff08842409156473b585a7/system-backend/node-gateway/src/http.rs) liefern unter /health/live die Serviceidentität, unter /api/v1/status den Gatewaystatus und unter /api/v1/core-services die überwachte zentrale Dienstmatrix. /health/ready ist in diesem Stand ein unveränderliches ready=true und beweist keine Verfügbarkeit der zentralen Fachpfade. Die Matrix nach service / level / critical_for_edge / fallback_mode / checked_at / last_success_at filtern; keine freitextlichen Backendmeldungen oder Konfigurationen ausgeben. Die folgende HTTP-Matrix allein bestätigt keinen TBS-Start; der spätere manuelle Start / Gateway-Handshake ist gesondert darunter dokumentiert. `systemctl --no-pager` für weitere Blöcke verwenden, damit der Pager die SSH-Eingabe nicht abfängt.


**Gateway-HTTP und aktuelle Core-Matrix bestanden, 11.10.2026:** Der Betreiber ruft die beiden APIs tatsächlich vom Pi aus ab. /api/v1/status meldet service=netcore-node-gateway, node_path=/ws/node, known_nodes=2, connected_nodes=0, monitored_services=17, available_services=15, degraded_services=0, unavailable_services=2. /api/v1/core-services liefert generated_at=2026-10-11T20:18:37.962Z. Alle sechs critical_for_edge-Dienste sind available und um20:18:34.075/076Z zuletzt geprüft / erfolgreich: call-control, group-core, media-switch, mobility-core, sds-router und subscriber-core.

| Gemeldeter optionaler Ausfall | critical_for_edge | Letzter erfolgreicher Check | Fallback laut Matrix |
| --- | --- | --- | --- |
| media-library | false |2026-10-10T01:14:38.064Z | local_media_cache_and_playout |
| observability | false |2026-10-11T00:25:34.385Z | local_logs_and_health_continue |

Die beiden Ausfälle sind Folgeaufgaben mit eigener Diagnose; sie sind laut vorhandener Matrix keine kritischen Voraussetzungen für den Gateway-Handshake / TBS-Grundlauf. Der Monitorbefund ist kein Funktionsnachweis aller zentralen Dienste oder der vollständigen26er-Flotte. Bei dieser Abfrage vor dem manuellen TBS-Start sind keine Nodes verbunden; die konkrete spätere Verbindung SRV-M-TBS-03 steht im nächsten Befund.

### Manueller TBS-Start / Gateway-Verbindung bestanden

**Betreiberbefund vom 11.10.2026:** `systemctl start tetra.service` startet den Funkstack bei weiterhin deaktiviertem Autostart. Zwei Prüfungen mit jeweils zehn Sekunden Wartezeit bestätigen:

| Prüfung | Betreiberbefund |
| --- | --- |
| TBS-Prozess | active/running / Result=success / ExecMainStatus=0 / MainPID=2934 in beiden Prüfungen |
| Dienstinstanz | InvocationID=9bb74e1668d740099cf059e4cb9b9f0b in beiden Prüfungen / NRestarts=0 |
| Gateway | SRV-M-TBS-03 / connected=true / stale=false / error_count=0 |
| Frische Nachrichten | last_seen20:25:20.919Z→20:25:31.040Z / message_count59→135 / heartbeat_seq40→115 |
| Gemeldete Identität | Stationsname SRV-M-TBS-03 / MCC901 / MNC1510 / LA3 / CC1 / main720 / secondary721 / dual_carrier=true |
| Tatsächliche SX-Einstellungen |600kSps / RX-/TX-Kanal0 / AntennenRX/TX / RX LNA42dB+PGA16dB / TX DAC9dB+MIXER30dB / period900 / dev_args leer |
| Tatsächliche SDR-Mitten | RX408.012500MHz / TX418.012500MHz, explicit center override |
| ControlRoom | transport connected22:25:13.610 CEST / hello accepted22:25:13.636 CEST, OPEN LAB |

Die tatsächliche PHY aktiviert RX-/TX-Streams vor dem ControlRoom-Worker; der angenommene Handshake zusammen mit fortschreitenden TX-Blöcken bestätigt mehr als die frühere Konstruktorprobe oder ein alleiniger Type=simple-active-Status. Die Gateway-Heartbeat-Sequenz wird auch von Telemetrie und deren Replay erhöht;40→115 bedeutet keine75 Heartbeats. Quelle: [ControlRoom-Worker](https://github.com/JanHG98/netcore-tetra/blob/1595259a2a76abfc9eff08842409156473b585a7/crates/tetra-entities/src/net_control_room/worker.rs).

| TX-Bericht, CEST | tx_block | hw_block | lead_blocks | late_skip_events | late_skipped_blocks |
| --- | --- | --- | --- | --- | --- |
|22:25:18.728 |3333 |3317 |16 |5 |76 |
|22:25:23.728 |6666 |6651 |15 |5 |76 |
|22:25:28.728 |9999 |9984 |15 |5 |76 |

Die fünf Warnungen um22:25:13.857–.862 überspringen zunächst72, danach viermal einen TX-Block. Die kumulierten Zähler wachsen in den drei Folgekontrollen nicht weiter; das ist Anlauf-Aufholen im beobachteten Intervall. Die einmalige Meldung um22:25:13.856 lautet `Lost -1200 samples, skipping 900 more samples and 1 processing blocks`. Im [RX-/TX-Code](https://github.com/JanHG98/netcore-tetra/blob/1595259a2a76abfc9eff08842409156473b585a7/crates/tetra-entities/src/phy/components/soapy_dev.rs) ist Lost die vorzeichenbehaftete Differenz zwischen gemeldetem und erwartetem Samplezähler. Hier erwartet der erste DSP-Block1200, während [SoapyIo](https://github.com/JanHG98/netcore-tetra/blob/1595259a2a76abfc9eff08842409156473b585a7/crates/tetra-entities/src/phy/components/soapyio.rs) den ersten Zeitstempel auf0 normalisiert; die negative Meldung beschreibt die Startausrichtung, keinen Nachweis von1200 physisch verlorenen Samples. Weitere900 Samples werden bei dieser Ausrichtung verworfen.

Alle drei Berichte melden hw_status_supported=false und hw_status_events / hw_underflows / hw_time_errors / hw_other_errors jeweils0. Die Hardware-Statusabfrage wird vom Treiber nicht unterstützt; diese Nullwerte belegen keine Abwesenheit tatsächlicher Hardware-Unterläufe. Der manuelle Prozess-/Stream-/Gateway-Grundlauf ist in der begrenzten Beobachtung bestanden. Dauerlauf, RF-Qualität, Empfang durch Endgeräte und fachliche Core-Funktionen sind damit noch nicht abgenommen. [Strukturierter Betreiberbeleg](evidence/pi-soapysx-permissions-2026-10-11.json).

### Automatischer TBS-Boot / Gateway-Wiederkehr, neue TX-Timingstörung

**Betreiberbefund vom 11.10.2026 nach erneutem Pi-Neustart:** Der zuvor vorgesehene Autostart-/Bootblock ist ausgeführt. Die neue SSH-Ausgabe bestätigt GPIO-Helfer ExecMainStatus0 / active-exited und Aliasziel gpiochip15. Beide TBS-Prüfungen melden UnitFileState=enabled / active-running / Result=success / ExecMainStatus0 / MainPID780 / InvocationID138a645a94644ae0822ecb3c92a1b96f / NRestarts0. Der konkrete Gatewayeintrag bleibt connected=true / stale=false / error_count0, mit identischem Stationsnamen / MCC901 / MNC1510 / LA3 / CC1 / Träger720/721 und dual_carrier=true. last_seen steigt von2026-10-11T20:40:10.878Z auf20:40:20.787Z; message_count167→185, heartbeat_seq161→179. Automatischer Dienststart, stabiler Prozess und Gateway-Wiederkehr sind bestanden.

**TX-Kontinuität nicht bestanden:** `journalctl -b ... -n60 --grep=...` liefert ausschließlich60 neue Deadline-Warnungen zwischen22:40:20.838 und22:40:20.930 CEST. Die TX-Blöcke reichen von32039 bis32100;58 Ereignisse überspringen je einen und zwei Ereignisse je zwei Blöcke, insgesamt62. Die beobachtete Warnungsspanne beträgt nur92ms. Anders als die alte Anlaufgruppe bei Block0–75 sind diese Ereignisse deutlich später in der SDR-Zeitachse. Der Ausschnitt beweist einen TX-Timingfehler in diesem Intervall, aber weder einen minutenlangen Dauerfehler noch dessen Ursache. Durch die Warnungshäufung sind sämtliche regelmäßigen TX-continuity-Berichte aus dem begrenzten Tail verdrängt; kumulierte Zähler und RX-Verlauf des Bootlaufs fehlen noch.

Der gepinnte TX-Code löst diese Warnung bei weniger als zwei Blöcken Vorlauf aus und zieht den Sollzähler auf aktuellen Hardwareblock+2 vor. Das sind tatsächliche Software-Deadline-Ereignisse; ein Hardware-Unterlauf ist damit nicht unabhängig gemessen. Boot-/Gateway-PASS darf deshalb nicht als RF-/PHY-PASS gelten. Die frühere manuelle Teilabnahme beschreibt nur deren beobachtetes Intervall.

### TX-Produktionsstillstand über15s bestätigt, 12.10.2026

Die begrenzte Folgeprobe ist ausgeführt. MainPID771 / InvocationID8af0dbe98d1e4d38a9fdaeb12d33b439 bleiben während dieser Messung gleich; Dienst active-running und NRestarts0. Das ist eine andere Instanz als der zuvor dokumentierte Bootlauf780; es wird kein ununterbrochener nächtlicher Dauerlauf behauptet. Die angezeigte ExecMainStartTimestamp=Sun2026-10-11 22:40:35 CEST bei top-Uptime16min und aktueller Wanduhr10:12 ist ohne weitere Startchronik nicht als reale Laufdauer zu verwenden.

| Kontinuitätsbericht, CEST | tx_block | hw_block | lead_blocks | late_skip_events | late_skipped_blocks |
| --- | --- | --- | --- | --- | --- |
|2026-10-12 10:12:37.021 |653268 |653266 |2 |632402 |633302 |
|2026-10-12 10:12:47.020 |659934 |659932 |2 |639066 |639968 |
|2026-10-12 10:12:52.019 |663267 |663265 |2 |642393 |643301 |

Δtx_block=9999 und Δlate_skipped_blocks=9999 über14,998s; Δlate_skip_events=9991. Im gepinnten [TxDsp-Code](https://github.com/JanHG98/netcore-tetra/blob/1595259a2a76abfc9eff08842409156473b585a7/crates/tetra-entities/src/phy/components/soapy_dev.rs) wird block_count durch Skip-Aufholen und zusätzlich einmal unmittelbar vor jedem Transmit-Aufruf erhöht. Bei Übereinstimmung von laufendem Binary und Quellstand bleibt der normale Erzeugungs-/Transmit-Pfad in diesem Intervall deshalb ohne Fortschritt. lead2 ist der wiederholt aufgeholte Mindestvorlauf, kein gesunder stabiler TX-Abstand. Die späteren Warnungen bis10:12:56.753 zeigen weitere aufeinanderfolgende Skips; sie sind keine bloße Fortsetzung der ursprünglichen Anlaufwarnung. hw_status_supported bleibt false; physische Unterläufe und RF-Ausgang sind weiterhin nicht unabhängig gemessen.

| Last-/Thermalbefund | Ergebnis / Grenze |
| --- | --- |
| Unit-CPU-Regeln | CPUQuotaPerSecUSec=infinity; AllowedCPUs / CPUAffinity leer; Nice−10, normale Policy0 / Priorität0 |
| Tatsächliche Threads |17 Threads, TS / Nice−10; Hauptthread in zweiter top-Intervallprobe10% |
| Gesamtlast / Speicher |96,6% CPU idle in zweiter Probe; viel freier RAM, Swap nicht verwendet |
| cgroup CPU vorher→nachher | usage_usec122332602→125007552, Δ2674950µs; user Δ1455267µs / system Δ1219683µs |
| CPU-Controllergrenze | cpu.max fehlt; cpu.stat enthält keine Quota-Throttlefelder. Kein positiver Nachweis einer CPU-Quota-Drosselung |
| Pi | throttled=0x0 / temp46,6°C / ARM1600020224Hz. Taktwert allein beweist keine Frequenzbegrenzung |
| RX-Journalfilter | No entries für Lost samples / Skipping demodulation im aktuellen Boot; keine Garantie erhaltener früher Startzeilen oder erfolgreicher RX-Verarbeitung |
| Kernelabfrage | Aus gepostetem verkürztem Befehlsblock nicht separat als -k-Abfrage verifiziert; keine behauptete Kernel-Fehlerfreiheit |

Die Messung stützt keine CPU-/Thermal-/Quota-Korrektur. Quelle lässt vor der TX-Erzeugung zwei relevante Rückwege zu: TX mehr als60 Blöcke vor dem letzten RX-Block oder nicht verfügbare / veraltete TX-Slotdaten. Diese Zweige wurden auf dem Pi noch nicht getrennt gemessen; eine isoliert nachvollziehbare Stale-Slot-Fehlerklasse ist keine Hardware-Ursachenabnahme. Keine alleinige Modulator-Epochen-, dmin-, Rate- oder Gainänderung vornehmen. Der gepinnte [SoapySX-Treiber](https://github.com/tejeez/sxxcvr/blob/9705147dd8c189625071f3f163ea56119bda4a05/SoapySX/SoapySX.cpp) berechnet getHardwareTime aus TX-PCM-Sampleposition minus ALSA pcm_delay; RX-Zeitstempel stammen aus der RX-Sampleposition. Das ist Stream-Samplezeit, keine NTP-/Realtime-Uhr und auch kein direkter CLOCK_MONOTONIC-Aufruf. Ein Wanduhrsprung erklärt keine direkte Änderung dieses Samplezählers; fehlerhafte RX-/TX-Startausrichtung ist dadurch trotzdem nicht ausgeschlossen.

### Diagnosestopp / Binary / Journald bestätigt; Diagnosepatch geprüft

Der Betreiber führt den nächsten Block aus: /proc/771/exe löst auf /usr/local/bin/bluestation-bs auf, SHA-256 lautet e4180b8580a5e37431bd84a7f9796eb7ea17a2e8619a4ba338163c0f6b39698e. systemctl disable --now entfernt den Autostartlink; MainPID0 / inactive / dead / disabled ist bestätigt. Start-/SDR- und RX-Filter für Invocation8af0dbe98d1e4d38a9fdaeb12d33b439 liefern No entries. Die Journald-Abfrage zeigt neun Suppressed-Meldungen von tetra.service, je5050–5069 Meldungen, zusammen45513 in den neun gezeigten Fenstern. Fehlende Start-/RX-Treffer beweisen weder unveränderte Zeitachsen noch einen gesunden RX-Pfad; nicht jede fehlende konkrete Zeile kann einem bestimmten Suppressionsfenster zugeordnet werden.

**Diagnosequelle umgesetzt / geprüft:** [PR67](https://github.com/JanHG98/netcore-tetra/pull/67), voller Diagnosecommit29f891f186f0b85d1930ba38ea8ac38157bc78ad, Merge510ec6f0f50f43bc8b3e87b50077fed8ea81c11f. Neue Felder in den vorhandenen5s-Berichten:

| Feld | Bedeutung |
| --- | --- |
| tx_process_attempts / tx_write_attempts | Aufrufe der TX-Verarbeitung / zurückgekehrte Treiber-Schreibversuche einschließlich Fehler |
| tx_written_blocks | Erfolgreich beim Treiber abgegebene vollständige Blöcke; keine On-Air-Bestätigung |
| latest_rx_block / tx_rx_lead_blocks | Letzter RX-Verarbeitungsblock und Abstand zum vorgesehenen TX-Block |
| tx_slot_times | Gelieferte Trägernummern mit ihren TDMA-Zeiten, nur beim periodischen Bericht formatiert |
| tx_hw_ahead_returns / tx_rx_ahead_returns | Rückkehr wegen der unveränderten60-Block-Vorlaufgrenzen; im normalen Betrieb auch erwartete Wartepfade |
| tx_slot_not_ready_returns | Rückkehr, weil Modulationsdaten des nächsten Blocks fehlen oder die angebotenen Slots bereits veraltet sind |
| late_skip_events / late_skipped_blocks | Jeder Skip bleibt im kumulativen Zähler, auch wenn keine Einzelwarnung ausgegeben wird |

Die Einzelwarnung wird durch die SDR-Blockzeit auf höchstens einmal pro vorhandenen5s-Berichtsintervall begrenzt. Keine Wanduhrabfrage oder neue Allokation im heißen TX-Pfad; TDMA-/Sample-Arithmetik, dmin/dmax, FCFB-Prozessanzahl, Modulation, Schreiben und Fehlerweitergabe bleiben unverändert. Damit bleibt die nächste Ursache messbar, statt unter hunderten Einzelwarnungen je Sekunde zu verschwinden.

**Validierung:** Zwei tatsächliche Helperregressionen bestehen mit lokalem Rustcompiler; neun vollständige native Soapy-/Wellenformfälle und der vollständige BS-Release-Build bestehen in [Radio-CI38224342827](https://github.com/JanHG98/netcore-tetra/actions/runs/38224342827). Die vier neuen Fälle prüfen10000 Skip-Ereignisse mit vollständigen Zählern / begrenztem Warnungsstrom, fehlgeschlagene Writes ohne Erfolgszählung, den tatsächlichen Stale-Slot-Modulationspfad und Träger-/Zeitformatierung. Bestehende Single-/Dualcarrier- und Kontinuitäts-Wellenformen bleiben geprüft. [Quellgate38224342762](https://github.com/JanHG98/netcore-tetra/actions/runs/38224342762) und [OpenLab38224342791](https://github.com/JanHG98/netcore-tetra/actions/runs/38224342791) bestanden; unabhängiger Review ohne Blocker. Vollständiger Git-Vergleich bestätigt keine Änderung irgendeiner Rustdatei / Cargo-Manifest-/Lockdatei oder Codec-C-/Headerdatei zwischen1595259 undf81bdb2. Die neue Binarybasis enthält damit dieselbe bisherige Runtime plus den gezielten Diagnosepatch.

**Gezielter Pi-Stagingweg:** [pi-txdiag-build-stage.sh](../../../tools/operator/pi-txdiag-build-stage.sh), als normaler SSH-Nutzer mit vollem festen Diagnosecommit und voller alter BinarySHA aufrufen. Prüft Zielhost / ARM64, inactive / MainPID0 / disabled, vorhandene Build-Abhängigkeiten und alte Prüfsumme. Baut in einem isolierten Nutzercheckout mit der aus /usr/share/netcore-image/versions.json gelesenen Rustversion, Cargo --locked / unveränderten Standardfeatures und zwei Buildjobs; der Nutzer-Cargo-Cache bleibt für die anschließende gezielte Reparatur erhalten. Native Bibliotheken, Soapy/GPIO, bestehende Konfiguration und VM119-Rezept/Softwarecache werden nicht installiert oder geändert. Der normale Deployment-Updater wird hier nicht benutzt, weil er zusätzlich migriert und den Funkdienst startet.

Nach fokussierten Tests / Release-Build prüft der Helfer ELF-AArch64, aufgelöste Bibliotheken und Diagnosemarker. Root prüft den deaktivierten Dienst und alte SHA erneut, sichert das eine Originalbinary mit Manifest und setzt den verifizierten Kandidaten atomar ein; kein Start / Enable / automatischer RF-Rollback. 14 aktuelle [Staging-/Guard-/Resume-Regressionen](../../../system-backend/deployment-core/tests/test_txdiag_stage.py) bestehen mit echten temporären Dateien / Prüfsummen / Kopien / fsync / Rename; Dienstmetadaten und Root-Ownership sind simuliert. Fälle: falsche alte/neue SHA, aktiver Dienst, erneute Aktivierung während des Stagings, bestehender Symlink und erfolgreicher Austausch mit unverändertem Originalbackup / ohne RF-Startanforderung; fehlende dynamische Bibliotheken im tatsächlichen Bashguard blockiert; drei eingebettete Pythonprogramme und Bashsyntax gültig.

**Erster tatsächlicher Pi-Stagingversuch / korrigierter Sourceguard, 12.10.2026:** Download9311 Bytes / Prüfsumme17f6c38c81c1fe54687b8220d46010cd8438cfae42017ff0cf175c3868b6e943 OK. Klonen / Checkout des festen Diagnosecommits29f891f erfolgreich; Quellpfad /home/jan/netcore-txdiag-29f891f186f0.0s34kP/source. Der Sourceguard meldet Unexpected runtime source changes und beendet den Aufruf vor Rustup / Build / Installation und vor dem im aufrufenden Block folgenden Funkstart. Die ursprüngliche Binary und der deaktivierte Dienst werden durch diesen Versuch nicht geändert.

Der Guard hatte Shared-Pfade zusätzlich aufgenommen, aber die erlaubten historischen Dokumentationsumzüge dort noch nicht berücksichtigt. Vollständiger primärer Git-Baumvergleich bestätigt genau elf erfasste Pfadänderungen: zwei Soapy-Diagnoserustdateien und neun Markdowndateien. Neu exakt erlaubt: system-backend/shared/README.md, shared/contracts/README.md, shared/web-ui/README.md sowie die entfernten shared/contracts/{ASSET_MANAGEMENT_V1,COMMAND_MODEL_V1,EVENT_MODEL_V1,TASK_MODEL_V1}.md. Keine Rust-, Cargo-, Codec-, CSS-/JS-/HTML-Änderung daneben; keine Build-/include_str-/include_bytes-Abhängigkeit auf diese entfernten Dokumente. Kein pauschales Markdown-Wildcard und kein Entfernen von shared aus der Kontrolle.

Drei neue Sourceguardfälle führen den tatsächlichen eingebetteten Prüfcode mit der gemeldeten elfteiligen Pfadliste aus, lehnen zusätzliche Rust-/Cargo-/CSS-/nicht geprüfte Markdownänderungen ab und verlangen beide Diagnose-Dateien. Drei weitere Resume-Fälle prüfen den tatsächlichen Bashzweig: vorhandener sauberer Checkout am richtigen Commit akzeptiert; lokale Änderungen, fehlgeschlagene Git-Statusabfrage, falscher Commit und nicht isolierter Pfad abgelehnt. Gitantworten sind in diesen sechs Fällen simuliert; zusätzlich besteht derselbe Sourceguard mit dem tatsächlichen vollständigen Git-Vergleich1595259→29f891f. Zusammen mit den acht bisherigen Guards14 Tests / Bashsyntax bestanden. Der frische ältere OpenLab-Lauf38226568083 am Commitabf8a92e hat nur im Discovery-Testjob sechs Fehler: Mock der abstrakten pathlib.Path-Klasse erzeugt unter Python3.11.17 AttributeError _flavour, bevor die echten Stagingguards laufen. Die konkrete Plattformklasse ersetzt diesen Testalias;14 Tests unter Python3.12.14 bestehen erneut. Rust-UI/defaultfeature-TBS-Link und beide Imageengine-Jobs Ubuntu24.04/26.04 des älteren Laufs bestehen; neue CI-Abnahme wird separat beobachtet. Helfer und Diagnose-Rustcode ändern sich durch diese Testkorrektur nicht. Der Helfer akzeptiert optional einen dritten Parameter EXISTING_SOURCE und verändert diesen Checkout beim Wiederaufnehmen nicht; Compilerpin, alte BinarySHA, ELF-/Linkage-/Markerguards und deaktivierter Dienst bleiben erforderlich. Wiederholung kann den bereits geladenen Quellstand verwenden.

**Noch offen:** Tatsächlicher ARM64-Build / Installation auf dem Pi und begrenzter manueller Funklauf mit den neuen Feldern. Der Helfer beendet sich mit inactive/disabled. Erst danach gezielt Start / Write-Fortschritt / RX-/Slot-Wartezweig erfassen; konkrete Behebung und Autostart-/Dauerlauf-/On-Air-/Core-/Controller-/VPN-Abnahme bleiben gesondert. [Strukturierter Betreiberbeleg](evidence/pi-soapysx-permissions-2026-10-11.json).


**Dauerhafte Quellkorrektur:** `image/build-guest.sh` setzt ausschließlich im Gast-Softwarebuild Umask0022. Die Host-Worker-Umask0077 bleibt erhalten, ebenso die separate Personalisierung mit privaten Zugangsdaten. `image_build.guest_sdr()` prüft auf jeder Stationskopie nach der Personalisierung die erwarteten echten Modulpfade, setzt die beiden öffentlichen Verzeichnisse auf0755 / das Modul auf0644 und führt `SoapySDRUtil --info` als `netcore` aus. Nur eine exakt gemeldete Factory `sx` erlaubt die weitere Artefaktausgabe; es wird keine Hardware geöffnet. Dieser Schritt liegt außerhalb des Cache-Miss-Zweigs und erfasst deshalb auch zugelassene Cachekopien. Die unveränderte allgemeine Cachedatei wird nicht bearbeitet.

Der Recipekey enthält die geänderten Quelldateien einschließlich des GPIO-Helfers: Ein normaler neuer Auftrag erhält damit einen neuen Recipekey und übernimmt einen alten Softwarecache nicht automatisch. Die aktuelle Pi-Reparatur braucht keinen neuen Auftrag. Der Quellfix ist noch nicht auf VM119 installiert, und ein neuer vollständiger ARM64-Build dieses Fixes ist nicht belegt.

Lokale Regressionen verwenden reale Verzeichnisse / Dateien / chmod unter Umask0077, prüfen Cacheverzeichnisse0700, unveränderte private Nachbarpfade, fehlende oder irreführende Factorymeldungen und Fehlerabbruch. Der Produktions-Cacheweg wird bis zur Dateiveröffentlichung simuliert: Die Dienstbenutzerprüfung liegt nach der Personalisierung und vor der Kompression; ohne sx entsteht kein Artefakt. ARM64-Ausführung, Mounts und Kompression sind simuliert; dies ersetzt keine neue Pi-/SDR-Abnahme. Acht SDR-Fälle, neun Gast-DNS-/Resolverfälle und sechs native dpkg-/Initramfs-Rezeptfälle bestehen, zusammen23 lokale Tests; zusätzlich bestehen alle80 VM119-Update-/Rückwegtests und Bash-/Python-Syntax. Historische DNS-/Initramfs-Operatorpins bleiben unverändert; ihre Tests prüfen diese festen Pins und verwenden für den isolierten Dateiaustausch eigene Fixturefingerprints, wie der bestehende APT-Test. [SDR-Regression](../../../system-backend/deployment-core/tests/test_image_sdr.py).

## Downloadintegritaet bestanden: Flashen und Erststart

Der Betreiber meldet am 11.10.2026 das Ergebnis des vorgegebenen Windows-Prüfblocks:

- Build-ID: `6a0a67657cce49118c89f5b9479db29d`.
- Datei: `C:\Users\janho\Desktop\netcore-srv-m-tbs-03-1595259a2a76-6a0a6765.img.xz`.
- SHA-256: `d24d7ed3e6e8a878ac3a24fee52239c608eb000f4c87ab6aaecfe6af96286366`.
- Ergebnis: `PASS: Größe und SHA-256 stimmen`.

Der ausgeführte Block prüft außerdem Manifest-ID und Commit1595259. Die lokale komprimierte Datei ist damit dem erwarteten Recoveryartefakt zugeordnet und vollständig gegen das Manifest geprüft. Eine konkrete numerische Dateigröße wurde in dieser Ausgabe nicht gedruckt; die Größenübereinstimmung wurde geprüft. Der aktuelle Download ist abgenommen. Die Downloadprüfung allein prüft SD / Pi / SDR / VPN noch nicht; der anschließend gelieferte physische OS-/SD-/Agentenbefund steht im vorangehenden Abschnitt. SXceiver und VPN bleiben offen.

### Verifiziertes Image auf SD schreiben

1. Raspberry Pi Imager auf dem Windows-PC öffnen und das vorhandene Pi-Modell auswählen.
2. Unter Betriebssystem **Eigenes Image / Use Custom** wählen und die oben geprüfte `.img.xz` direkt auswählen.
3. Die vorgesehene SD-Karte als Ziel wählen; sie wird überschrieben.
4. Zusätzliche Imager-Anpassungen für Benutzer, SSH, WLAN und Hostname überspringen. Diese Werte stammen bereits aus dem Imageauftrag.
5. Schreiben starten, die anschließende Schreibprüfung vollständig abwarten und die Karte auswerfen.

Die [offizielle Imager-Anleitung](https://www.raspberrypi.com/documentation/computers/getting-started.html) beschreibt Custom-Image, Zielmedium und Schreibprüfung. Das Überspringen zusätzlicher Anpassungen folgt der vorhandenen NetCore-Personalisierung.

### Erster Pi-Start und lesende OS-Pruefung

Für den ersten OS-/Agententest den Pi ohne SXceiver über LAN im Managementnetz starten. Diese Reihenfolge ist durch den tatsächlich gebauten Quellstand1595259 begründet: `tetra.service` ist aktiviert und startet die Funkanwendung nach erfolgreichem SXceiver-Probe. Ohne HAT kann dieser Dienst fehlschlagen / wiederholen; SSH und Discovery werden separat geprüft. Nach bestandener OS-Prüfung den Pi sauber herunterfahren, bevor die Hardware ergänzt wird.

Die LAN-Adresse kommt per DHCP; im Router die Station identifizieren. SSH mit dem beim Imagebuild gesetzten Benutzer / Schlüssel oder Passwort verwenden. Ein Benutzername `pi` wird nicht angenommen; der Imageauftrag entscheidet den Zugang.

Auf dem Pi nach der SSH-Anmeldung:

```bash
hostnamectl --static
uname -m
ip -br -4 address
df -h /

sudo systemctl --no-pager show \
  netcore-firstboot.service netcore-discovery.service ssh.service \
  -p Id -p ActiveState -p SubState -p Result

sudo journalctl -b -u netcore-firstboot.service --no-pager

cat /usr/share/netcore-image/station.json

curl --noproxy '*' --fail --max-time 10 -sS \
  http://127.0.0.1:8321/health/ready |
  python3 -m json.tool
```

Erwartung: ARM64 / `aarch64`, Stationsprofil SRV-M-TBS-03 am Commit1595259, Firstboot normalerweise `active/exited`, Agent bereit. Die SD-Erweiterung anhand Journal und Dateisystemgröße prüfen; ein Firstboot-Marker allein belegt sie nicht. Dieser Block ist inzwischen mit der tatsächlichen Pi-Ausgabe vom 11.10.2026 bestanden; siehe [physischen Erststart](#physischer-pi-erststart-bestanden).

Geprüfte Quellen am Imagecommit: `image/build-guest.sh`, `image/personalize.py`, `image/firstboot.py`, `image/systemd/netcore-firstboot.service` und `image/systemd/tetra.service` unter `system-backend/deployment-core/`.

## Download auf den PC gemeldet: Datei pruefen

Am 11.10.2026 meldet der Betreiber: „so, ist auf meinen PC gedownloaded“. Dies dokumentiert den Download nach Nutzerangabe. Ohne Dateiname / Manifest / Byteprüfung ist damit noch kein vollständiger Dateiintegritäts- oder zugeordneter Recoveryabschlussbeleg gegeben.

Der folgende lesende Prüfblock ist auf dem Windows-PC in PowerShell auszuführen; keine Administratorrechte nötig. Er erwartet das zuvor angenommene Recoveryartefakt `6a0a67657cce49118c89f5b9479db29d` am Commit `1595259a2a76abfc9eff08842409156473b585a7`. Der Operator gibt den tatsächlichen Dateipfad ein; ein durch den Browser geänderter Dateiname ist zulässig. Ein anderes Artefakt darf nicht stillschweigend diesem Build zugeordnet werden. Der frühere Originalimage-Hash `4a6b…` ist für die neue Recoverydatei nicht maßgeblich.

```powershell
& {
    $ErrorActionPreference = 'Stop'
    $ncBuild = '6a0a67657cce49118c89f5b9479db29d'
    $ncCommit = '1595259a2a76abfc9eff08842409156473b585a7'
    $ncManifest = Invoke-RestMethod -TimeoutSec 20 -Uri "http://10.0.1.131:8320/api/v1/images/$ncBuild/manifest"

    if ($ncManifest.id -ne $ncBuild -or $ncManifest.commit -ne $ncCommit -or
        $ncManifest.sha256 -notmatch '^[0-9a-fA-F]{64}$') {
        throw 'Manifest passt nicht zum Recovery-Build.'
    }

    $ncPath = Read-Host 'Vollständiger Pfad zur heruntergeladenen .img.xz'
    $ncFile = Get-Item -LiteralPath ($ncPath.Trim().Trim('"'))
    $ncSize = [Int64]$ncManifest.size_bytes
    if ($ncFile.PSIsContainer -or $ncFile.Name -notlike '*.img.xz' -or
        $ncSize -le 0 -or $ncFile.Length -ne $ncSize) {
        throw 'Datei oder Dateigröße passt nicht zum Manifest.'
    }

    $ncHash = (Get-FileHash -LiteralPath $ncFile.FullName -Algorithm SHA256).Hash
    if ($ncHash -ine $ncManifest.sha256) { throw 'SHA-256 stimmt nicht überein.' }
    "PASS: Größe und SHA-256 stimmen. Build: $ncBuild"
    "Datei: $($ncFile.FullName)"
    "SHA-256: $ncHash"
}
```

Bei `PASS` sind die gewählte lokale Datei und die Manifestwerte anhand von ID / Commit / Größe / vollständiger SHA-256 zugeordnet. Dieser Block ist inzwischen am Nutzer-PC mit `PASS` nachgewiesen; der aktuelle Beleg steht im Abschnitt zur bestandenen Downloadintegrität. Anschließend die ausgegebene Prüfsumme dokumentieren und das verifizierte Image flashen. Pi-/SXceiver-Start und VPN-Wechsel benötigen ihre eigenen Gerätebefunde. Bei Manifest-, Größen- oder Hashfehler nicht flashen; Datei / Artefaktzuordnung zuerst klären.

## Chronik der Prüfungen und Betreiberbefunde

Die folgenden Status- und Fortsetzungssätze gelten jeweils zum beschriebenen Zeitpunkt. Hotfix- und Recoveryblöcke sind an ihre Host-, Quell- und Auftragsprüfungen gebunden; bereits ausgeführte Schritte nicht als heutige Wiederholungsanleitung lesen. Die historischen Originalangaben sind durch diesen Nachtrag nicht erneut vollständig geprüft.

### Z01.4 – VM119 Imagebuilder: Vorprüfung und Buildabnahme

Stand: 2026-10-09. CT150-Readiness/Recovery und CT136-TCP-/Vorschau-/NAS-Erfolgspfad einschließlich isoliertem Fehlermount sind als Betreiberbefunde dokumentiert. Die tatsächliche Vorprüfung auf `VM-H-DEPLOY-01`, VM119, `10.0.1.131`, besteht. Die neue Betreiber-Ausgabe bestätigt jetzt den vollständigen Imagebuilder-Erfolg an1595259 mit Dateiname, Größe, SHA-256 und Abschlussmarker. Das fertige Artefakt ist in der aktuellen Speicheransicht jedoch nicht gelistet und unter dem Standardpfad nicht gefunden. Der passende Softwarecache ist auf VM119 bestätigt. Nach einem unspezifizierten SQLite-Lesefehler ist der Recoveryauftrag inzwischen eindeutig zugeordnet: Cachekopie erfolgreich, losetup-Timeout30s vor Stationspersonalisierung. Die folgende Betreiber-Ausgabe belegt inzwischen eine Rootdisk-I/O-Blockade; lokales Live-QEMU-Backing und der SATA-Hostengpass sind inzwischen bestätigt. Die begrenzte Übernahme von VM119/scsi0 auf NVMe_OSData / rpool/data ist jetzt durch erfolgreichen Kopierabschluss, Konfiguration und Live-QEMU-Backing bestanden. Als Nächstes Gast-Schreibzugriff prüfen und das Stationsimage einmalig aus dem bestätigten Softwarecache wiederherstellen. Downloadprüfung und physische Abnahme bleiben offen. Die früheren DNS-/Conffile-/Initramfs-Abbrüche und der gesicherte Stand von17:03 bleiben unten ausdrücklich datierte Historie.

## NVMe-Cache-Recovery angenommen

Der Betreiber führt am 09.10.2026 den Helfer aus `c5bf54cb7a1a043064df0c899752c422e430ce9f` als `/var/tmp/netcore-image-nvme.OjRnJo.py` mit `--after-nvme-move` aus; die SHA-256-Prüfung `7861bec31a9c59cf3e7d9e64eb7e123d246631689cfe8bb480218951218848d2` besteht. Ausgabe: Status `queued`, `new_post=true`, Job `4ebe40edf4b443f592e7f7ad28ad0624`, Build `6a0a67657cce49118c89f5b9479db29d`, Commit1595259, separater Beleg `/var/lib/netcore-image-builder/recovery-2e2603beacd942dfaddc7119b85a4af0-nvme.json`. Anschließend meldet der Betreiber den laufenden Build im UI.

Die Annahme beweist das Passieren der Prüfungen im ausgeführten Helfer einschließlich begrenztem Datei-/Verzeichnis-fsync. Sie ist noch kein Nachweis der abgeschlossenen Cachekopie / Stationspersonalisierung / Kompression / Artefaktveröffentlichung oder des Downloads. Den laufenden Auftrag verfolgen; keine neue Einreihung oder Änderungen der Rezept-/Cachedaten. Derselbe Helferaufruf fragt ausschließlich den vorhandenen Auftrag ab. Download / vollständige Prüfsumme und physischer Pi-Start folgen nach erfolgreichem Ergebnis. [Strukturierter Betreiberbeleg](evidence/vm119-imagebuilder-success-2026-10-09.json).

## NVMe-Wechsel bestanden: Stationsimage fortsetzen

Der tatsächliche Betreiberbefund vom 09.10.2026 bestätigt 100,0 GiB übertragen in 3 min 3 s, `Completed successfully` und `mirror-job finished`. Die neue Konfiguration nennt `scsi0: NVMe_OSData:vm-119-disk-0,iothread=1,size=100G`; live QEMU öffnet raw `/dev/zvol/rpool/data/vm-119-disk-0` über die zeroinit-/host_device-Treiberkette, Cache `writeback, direct`. Die NAS-Datei bleibt `unused0: Media:119/vm-119-disk-0.qcow2`, die SATA-Quelle `unused1: VirtualMachines_OSData:vm-119-disk-0`. Damit sind Kopie und Umschaltung bestanden. Die erhaltenen Quellen sind nach dem Wechsel nicht weiter aktuell.

Die neue aktive Platte umgeht den belegten SATA-Pfad für VM119. Die allgemeine SATA-/Hoststörung, der aktuelle Gast-I/O-Zustand und ein erfolgreicher neuer Image-/Downloadlauf sind durch den Disk-Move noch nicht geprüft. Den früheren Anlage-/Moveblock unten nicht erneut ausführen. Die bisherige Recoveryquittung und der fehlgeschlagene Auftrag `4a99c6ec54e74264811d86817a568d47` bleiben erhalten; kein Belegreset.

Nächster Schritt: ein begrenzter kleiner Schreib-/Datei-fsync-/Verzeichnis-fsync-Probe im Builderstate und ein separater einmaliger Recoverypfad für den ursprünglichen Stationsauftrag. Er darf die bestätigte Softwarebasis für1595259 nur bei identischem installierten Recipekey verwenden; die Rezeptdateien bleiben unverändert. Damit entfällt der ARM64-Softwarebuild, Personalisierung / Dateisystemprüfung / Kompression und Veröffentlichung bleiben notwendig. Ein neu erzeugtes Image erhält eine neue ID und Prüfsumme; es ist keine bytegleiche Rekonstruktion der verschwundenen Datei. [Strukturierter Betreiberbeleg](evidence/vm119-imagebuilder-success-2026-10-09.json).

### Einmaliger Wiederholungsweg nach bestaetigtem NVMe-Wechsel

Der aktualisierte [Recoveryhelfer](vm119-cached-image-recovery.py) erlaubt ausschließlich den zusätzlichen Modus `--after-nvme-move`. Er liest den bisherigen Recoverybeleg ohne Änderungen und ordnet anhand dessen Baseline und vollständigem privatem Request-Fingerabdruck genau den bestätigten fehlgeschlagenen Auftrag `4a99c6ec54e74264811d86817a568d47` / Build `7ec061ca1ae24338bdc86a22030957cb` zu. Der ursprüngliche Erfolgsauftrag, Cachecommit und Stationseinstellungen müssen ebenfalls übereinstimmen. Laufende, ungeklärte oder mehrdeutige Vorgänger sperren den neuen Versuch.

Der neue Versuch verwendet den festen separaten Beleg `/var/lib/netcore-image-builder/recovery-2e2603beacd942dfaddc7119b85a4af0-nvme.json` und denselben exklusiven Lock. Der Beleg enthält die Verknüpfung und SHA-256 des ursprünglichen Belegs sowie nur den Fingerabdruck der privaten Stationswerte. Er wird vor dem einzigen POST dauerhaft gespeichert. Ein wiederholter Aufruf mit demselben Flag verfolgt ausschließlich diesen neuen Auftrag; auch nach verlorenem Antwortpaket oder erneutem Fehlschlag erfolgt kein weiterer POST. Der alte Beleg wird weder zurückgesetzt noch gelöscht oder umgeschrieben.

Vor einem neuen POST bestehen die bisherigen Cache-/Rezept-/Worker-/Auftrags-/Workpfadprüfungen. Zusätzlich schreibt ein separater Prozess eine kleine temporäre Datei im Builderstate, synchronisiert die Datei, entfernt sie und synchronisiert das Verzeichnis. Der Probe wartet höchstens15s und bei notwendigem Beenden weitere2s. Fehler / Zeitlimit verhindern den Build. Ein noch im Kernel blockierter Kindprozess kann nach SIGKILL weiter bestehen; der Helfer startet in diesem Fall keinen Auftrag. Rezeptdateien, Softwarecache und laufende Dienste werden nicht geändert.

Ausführung auf VM-H-DEPLOY-01 nach Download des geprüften Helfers:

```bash
sudo python3 -B /pfad/zum/vm119-cached-image-recovery.py --after-nvme-move
```

Lokale Validierung des erweiterten Helfers:45 Recoverytests bestanden (26 bisherige und19 neue NVMe-Fälle), einschließlich tatsächlichem kleinem Datei-/Verzeichnis-fsync-Probe, einmaligem POST, verlorener Antwort, fehlgeschlagenem Kindauftrag, geändertem / mehrdeutigem Vorgänger und unverändertem alten Beleg. Alle80 VM119-Operatortests bestanden. Unabhängiger Review ohne offenen Blocker. Die Tests verwenden SQLite-/Datei-Fixtures und isolieren den Workertransport; sie führen keinen VM119- oder ARM64-Build aus. Der neue Quellstand hat bis zur Veröffentlichung noch keinen eigenen main-CI- oder VM119-Recovery-Nachweis.

Danach denselben Aufruf erneut verwenden, um den vorhandenen Auftrag abzufragen, oder dessen Status in **Builds & Downloads** ansehen. Die tatsächliche NVMe-Recovery, Artefaktveröffentlichung, vollständige Download-Prüfsumme und der Pi-Start bleiben bis zu eigenen Betreiberbefunden offen.

## Ausfuehrbarer NVMe-Wechsel fuer VM119

Der Betreiber bestätigt `rpool/data` als vorhandenen Dataset-Parent: 96 KiB belegt, 378 GiB verfügbar, Mountpoint `/rpool/data`. Die gefilterte Storage-Konfiguration enthält nur den SATA-zfspool VirtualMachines_OSData und den lokalen Ordnerstorage. Der Ordner liegt ebenfalls auf rpool; für VM119 wird ein eigener Images-Storage auf dem vorhandenen Dataset-Parent gewählt, damit die Disk ein natives ZFS-Volume bleibt.

Historischer Operatorblock, inzwischen erfolgreich ausgeführt; nicht erneut starten. Vor der Durchführung galt: Auf SRV-H-PVE-01 den folgenden Block genau einmal ausführen. `set -euo pipefail` verhindert den Disk-Move, wenn die Storage-Anlage fehlschlägt, beispielsweise bei einer bestehenden gleichnamigen ID. Eine bereits bestehende ID wird nicht ungeprüft wiederverwendet. Der Kopierauftrag erhält kein äußeres Timeout; währenddessen keinen Imagebuild starten.

```bash
(
set -euo pipefail

pvesm add zfspool NVMe_OSData \
  --pool rpool/data \
  --content images \
  --nodes SRV-H-PVE-01 \
  --sparse 1

qm disk move 119 scsi0 NVMe_OSData --delete 0

qm config 119

timeout --kill-after=2s 8s \
  pvesh create /nodes/localhost/qemu/119/monitor \
  --command 'info block'
)
```

Erwarteter Nachweis: `scsi0` nennt `NVMe_OSData:`, und live QEMU verwendet `/dev/zvol/rpool/data/<neues-VM119-Volume>`. Der genaue Volumename kommt aus der tatsächlichen Ausgabe. Die SATA-Quelle wird als weiterer unused-Eintrag erhalten; die vorhandene alte Media-Datei bleibt ebenfalls erhalten. Nach der Umschaltung gehen neue Gast-Schreibzugriffe nur auf die NVMe. Die alte Disk ist deshalb kein dauerhaft aktueller Rückweg; vor einem späteren Rückwechsel wären neue Daten zu berücksichtigen.

Vorheriger Planungsstand vor der inzwischen oben bestätigten Ausführung: Die tatsächliche Storage-Anlage, Kopie und Umschaltung sind noch nicht beobachtet. Nach bestandenem Live-Nachweis den Gastzustand prüfen; ein kontrollierter Neustart ausschließlich von VM119 ist bei weiter blockiertem Gast zu bewerten. Der bestehende Recoverybeleg bleibt erhalten. Der alte Helfer startet bei erneutem Aufruf keinen zweiten Build; ein weiterer Cache-Recoveryauftrag benötigt nach behobener Plattformstörung einen gezielt geprüften Wiederholungsweg. [Zielstorage- und Operatorbeleg](evidence/vm119-imagebuilder-success-2026-10-09.json).

## Bestaetigter Hostengpass: VM119 auf NVMe vorbereiten

Historischer Befund vor dem oben bestätigten NVMe-Wechsel.

Die nächste Betreiber-Ausgabe vom 09.10.2026 bestätigt einen Engpass im Host-Speicherpfad: QEMU119-I/O-Worker PID3348986 / TID3693425 steht in D/`submit_bio_wait`. Der SATA-VM-Pool hat in drei aktuellen Samples 612/932/349 ms mittlere Gesamt-Schreibwartezeit; synchron warten 79/15/159 Zugriffe, asynchron 251/184/313. Die SSD mit Seriennummer-Endung K0Z5 zeigt 221/127/119 ms reine Schreib-I/O-Zeit und rund 1 s Gesamtwartezeit / 3 s asynchrone Queue-Wartezeit; die andere SSD liegt bei 49/24/26 ms reiner Schreib-I/O-Zeit. Ein bestimmter physischer SSD-Defekt ist damit noch nicht bewiesen. Der Host hat 78 GiB RAM, 14 GiB verfügbar und keinen Swap; Memory-PSI full 38,39 % über 10 s bleibt ein zusätzlicher Druckbefund, aber keine nachgewiesene Swap-Ursache.

Für den laufenden Nutzerauftrag wird eine konkrete Ausweichroute vorbereitet: ausschließlich VM119/scsi0 samt Betriebssystem und Builderstate auf den vorhandenen NVMe-Spiegel rpool verschieben. Die vorherige Messung nennt 393 GiB freien Poolplatz für die 100-GiB-Disk; Images-fähige Proxmox-Storage-ID, Pool/Dataset, Freigabe auf diesem Node und tatsächlicher Zielplatz sind noch zu prüfen. Die Übernahme ist noch nicht gestartet und eine NVMe-Latenzabnahme liegt noch nicht vor. Sie soll den belegten SATA-Pfad für VM119 umgehen. Die gesamte Host-/SATA-Störung ist damit noch nicht behoben.

Nächste lesende Betreiberabfragen auf SRV-H-PVE-01: zfspool-/dir-Blöcke aus `/etc/pve/storage.cfg` sowie `zfs list -r rpool -o name,type,used,avail,mountpoint`. Danach einen eng begrenzten Disk-Move auf den verifizierten Zielstorage vorbereiten, mit `--delete 0` und Erhalt der bereits ungenutzten alten NAS-Datei. Während des Kopierens keinen Imagebuild starten. Ein Gastneustart ist erst nach erfolgreichem Wechsel und bei weiterhin blockiertem Gast zu bewerten. SMART / Reparatur des SATA-Spiegels bleiben eine getrennte Betriebsaufgabe. [Strukturierte Latenz- und Korrekturbelege](evidence/vm119-imagebuilder-success-2026-10-09.json).

## Live-QEMU-Backing und Host-I/O-Warten

Der folgende Betreiberbefund vom09.10.2026 bestätigt die tatsächlich geöffnete VM-Platte: QEMU `drive-scsi0` verwendet raw `/dev/zvol/VirtualMachines_OSData/vm-119-disk-0`, Cache `writeback, direct`, Detect zeroes on. `info block-jobs` meldet keine aktiven Jobs. Die frühere Media-Datei ist damit in diesem Sample kein aktiver VM-Datenträger. Die Plattenrettung / Übernahme auf das lokale Live-Backing ist jetzt nachgewiesen; die alte ungenutzte Datei wurde durch diese Diagnose nicht gelöscht.

Beide ZFS-Pools sind ONLINE ohne gemeldete READ-/WRITE-/CKSUM- oder bekannte Datenfehler. VirtualMachines_OSData:888GiB,358GiB belegt,530GiB frei,40% Kapazität /46% Fragmentierung, Spiegel aus zwei KIOXIA-EXCERIA-SATA-SSDs. rpool:464GiB,71,4GiB belegt,393GiB frei,15% Kapazität, NVMe-Spiegel. Die zwei aktuellen VM-Pool-Samples zeigen174/200 Schreiboperationen/s und7,79/9,15MiB/s. Der erste iostat-Block ist dagegen der Durchschnitt seit Boot. Der Host selbst zeigt I/O-Pressure some73,55% / full9,89% über10s (60s:73,65% /10,35%;300s:65,88% /7,67%). Ein vollständig fehlerfreier / latenzarmer Host oder ein reiner Gast-Kernelbug folgt daraus nicht.

Nächster Schritt auf SRV-H-PVE-01: `zpool iostat -v -l -q -y VirtualMachines_OSData 1 3` für aktuelle Latenz / Warteschlangen, QEMU-Threads anhand `/run/qemu-server/119.pid` und Host-Kernelmeldungen lesen. Diese Messung ergänzt die bereits bestätigte Gast-Schreibblockade. Kein pauschales WBT-Tuning, vollständiger Hostneustart oder neuer Imageauftrag vor Zuordnung. [Strukturierter Live-Hostbeleg](evidence/vm119-imagebuilder-success-2026-10-09.json).

## Bestaetigte Rootdisk-I/O-Blockade und neue Plattenkonfiguration

Die angehängte Betreiber-Ausgabe vom09.10.2026, 11:33:43 Europe/Berlin, nennt Kernel `7.0.0-38-generic`, Uptime1:55. `jbd2/sda2-8` steht in D/`wait_on_buffer`, `systemd-journal` in D/`jbd2_log_wait_commit`. Dirty204KiB / Writeback0 widerlegen diese Blockade nicht; I/O-Pressure full beträgt91,54% über10s,79,21% über60s und81,80% über300s. Die Ausgabe listet aktuell keine NetCore-Loop-Backingdatei.

Der Kernel meldet um10:49:29 `losetup` PID9714 für mehr als122s blockiert. Der Stack führt über `rq_qos_wait`, `wbt_wait`, den ext4-Schreibpfad und `vfs_fsync` zu `loop_configure`. Auch curl hängt im Writeback-Throttling. journald hat Watchdog-/Starttimeouts und meldet eine beschädigte oder unsauber geschlossene Journaldatei; später erscheinen Kapazität / Partitionen von loop21. Diese Betreiberbefunde bestätigen eine Rootdisk-I/O-Störung statt lediglich eines langsamen Imagebuilders. Sie beweisen noch keinen bestimmten Host-Datenträgerdefekt oder Kernelbug.

Die aktuelle `qm config 119` nennt `scsi0: VirtualMachines_OSData:vm-119-disk-0,iothread=1,size=100G`, `scsihw: virtio-scsi-single`, RAM16496MiB. Die ehemalige Media-Datei steht als `unused0: Media:119/vm-119-disk-0.qcow2`; sie ist in dieser Konfiguration kein angeschlossenes Laufwerk. Die tatsächlich vom laufenden QEMU geöffnete Blockquelle ist damit noch nicht unabhängig beobachtet. Nächster Schritt auf SRV-H-PVE-01: live `info block` / `info block-jobs`, ZFS-Zustand / Kapazität und Host-Kernelmeldungen lesen. Erst danach eine konkrete Behebung wählen; Recoverybeleg und Softwarecache erhalten, keinen weiteren Build starten. [Strukturierter Betreiberbeleg](evidence/vm119-imagebuilder-success-2026-10-09.json).

## Zugeordneter Recoveryauftrag: Cachehit, losetup-Timeout

Aktueller Betreiberbefund vom09.10.2026: SQLite lässt sich mit `mode=ro` öffnen, das Schema und die Aufträge sind lesbar. Der Recoverybeleg steht auf `submit_started`; eine Auftrags-ID ist noch nicht eingetragen. Der identische private Request-Fingerabdruck, ohne `build_id`, findet außerhalb der gespeicherten Baseline genau einen Auftrag: `4a99c6ec54e74264811d86817a568d47`, Build-ID `7ec061ca1ae24338bdc86a22030957cb`, Commit `1595259a2a76abfc9eff08842409156473b585a7`, Status `failed`. Damit ist der POST-Ausgang eindeutig zugeordnet. Die konkrete Ursache des zuvor ausgegebenen `OperationalError` ist weiterhin nicht bekannt.

Der zugehörige Log bestätigt `Vorbereitete Softwarebasis aus Cache: 7699808215f7` und die erfolgreiche Kopie des Cacheimages in den Workordner. Danach scheitert `Disk.__enter__()` an `losetup --find --show --partscan .../work/7ec061ca1ae24338bdc86a22030957cb/image.img` mit `subprocess.TimeoutExpired` nach30 Sekunden. Die Stationspersonalisierung, Kompression und Veröffentlichung wurden in diesem Lauf noch nicht ausgeführt. `Command exited with 1: unshare` ist der äußere Folgestatus. Es wurde keine neue ARM64-Softwarekompilierung gestartet.

Nächster Schritt: auf VM119 blockierte Prozesse / Wartefunktionen, Kernelmeldungen, Dirty-/Writeback-Werte, I/O-Pressure und die Zuordnung der NetCore-Loopgeräte lesen; auf Proxmox die aktuelle `qm config 119` nach der gemeldeten Plattenrettung prüfen. Das Gast-Dateisystem ext4 allein belegt keinen lokalen Proxmox-Speicher. Eine Ursache im Kernel-/Block-I/O-/Lock-Zustand ist noch nicht nachgewiesen. Kein weiterer Recovery-POST oder Belegreset vor dieser Diagnose; die installierten Rezeptdateien und der geprüfte Softwarecache bleiben erhalten. [Betreiberbeleg](evidence/vm119-imagebuilder-success-2026-10-09.json).

Die SQLite-Diagnosekorrektur an `d9938723d5a320718e36db9b739b5ba6a9fcb11f` besteht beide main-Workflows: [Quellgate](https://github.com/JanHG98/netcore-tetra/actions/runs/37907594111) und [Deployment/Discovery](https://github.com/JanHG98/netcore-tetra/actions/runs/37907594176). Diese CI-Erfolge ersetzen weder die Plattformdiagnose noch den noch offenen Artefaktdownload und physischen Pi-Start. Die nachfolgenden Abschnitte halten die vorherigen Diagnose- und Recoveryphasen als Historie fest.

## Betreibermeldung: erfolgreiches Image, fehlende Downloadanzeige

Am09.10.2026 meldet der Nutzer: Der gestrige Build war erfolgreich und hat eine `.img.xz` erzeugt. Nach dem aktuellen Update fehlt die Downloadfunktion in der WebUI. Die erste Meldung enthält keine Job-ID, Manifest- oder Prüfsummendaten. Die folgenden älteren Buildabbruchabschnitte bleiben datierte Historie; sie sind nicht der aktuelle Gesamtstand.

Am geprüften TBS-Quellstand16d0d1f sind die Downloadlinks und GET-Routen weiterhin enthalten. Der neue Updatehelfer übernimmt keine Artefaktdateien und löscht keine Images. Ein tatsächlicher lokaler Browserfall mit fertigem Artefakt / Manifest / SQLite-Auftrag, leerer Profilliste und ohne Build-POST zeigt eine Downloadkarte, liefert die Testdatei bytegenau und behält die Karte nach Refresh. Ein genereller Verlust der Downloadfunktion durch die Profiländerung wurde damit nicht reproduziert.

**Anschließender tatsächlicher Betreiberbefund:** [Image-API / Worker / Artefaktpfad](evidence/vm119-download-diagnosis-2026-10-09.json). Die lokale GET-Abfrage liefert `available=true`, `error=null`, `artifacts=[]`. Der Imageworker ist aktiv und startet `image_worker.py` ohne `--state`; der geprüfte Quellcode verwendet dann `/var/lib/netcore-image-builder`. Die Prüfung `artifacts/*/*` als root findet keine Dateien. Damit ist ein Ausfall der gesamten Worker-Statusantwort für diese Abfrage ausgeschlossen. Die Downloadanzeige hat derzeit keine gelisteten Artefakte. Ob die Datei an einem anderen Ort liegt oder die gegenwärtige Datenträgeransicht unvollständig ist, bleibt offen; ein endgültiger Verlust ist nicht belegt. Auch der exakte installierte Quellstand ist durch diese Ausgabe noch nicht bestätigt.

Nächster Schritt auf VM119: Imagejobs einschließlich `request.build_id`, `result.id`, Ergebnisdateiname und Abschlusslog lesen; aufgelösten Statepfad / Mountquelle und die Dateien in cache, work und artifacts nur flach prüfen. Ein gültiges Artefakt benötigt `artifacts/<Build-ID>/image.img.xz` und ein `manifest.json` mit passender ID. `job.id` ist eine andere Kennung als die Artefakt-ID. Cachedateien sind OS-/Softwarebasisimages und belegen allein kein fertiges Stationsimage. Der Abschlussmarker `Fertig:` wird erst nach der Veröffentlichung des vollständigen Artefaktordners geschrieben. Die Managementadresse war aus der Assistenzumgebung nicht erreichbar.

**Zweiter tatsächlicher Betreiberbefund:** [Vollständiger Build / fehlendes Artefakt / neuer Timeout](evidence/vm119-imagebuilder-success-2026-10-09.json). Auftrag `f141c06d4b2f48699486e149c4887b16` ist `succeeded`, Artefakt-ID `2e2603beacd942dfaddc7119b85a4af0`, Commit `1595259a2a76abfc9eff08842409156473b585a7`. Ergebnis und Log nennen `netcore-srv-m-tbs-03-1595259a2a76-2e2603be.img.xz`, 2.043.954.184 Bytes und SHA-256 `4a6bc6549cace7559a3dfc9e1f8f6d6d0302e84df3128b63d31c4e6ac17d6c3b`; der Abschlussmarker `Fertig:` bestätigt die damalige Veröffentlichung. Der Statepfad liegt aktuell auf `/dev/sda2`, ext4. SQLite-Historie und beide Cacheimages sind vorhanden; fertige Images oder ausstehende Artefaktdateien unter work wurden nicht gefunden. Dies beweist den früheren vollständigen Build, noch keinen erfolgreichen aktuellen Download oder physischen Pi-Start.

Der neue Auftrag `8d83bec249dc4297a964db170c7b5f7d`, Artefakt-ID `66e4156625f5425e9cbd04e77bb40003`, an2815789 endet bei `sfdisk --no-reread --no-tell-kernel -N 2 .../image.img` nach30 Sekunden. `unshare` ist der äußere Fehlerstatus. Der kurze Logauszug zeigt nicht, welcher Aufrufer der gemeinsamen Partitionsroutine betroffen ist; eine genaue Ursache des Zeitlimits ist noch nicht belegt. Dieser Lauf verwendet einen anderen Quellcommit und damit einen anderen Recipekey.

Der vorhandene Softwarecache `7699808215f76322a6ed0cceddf5804d24f707e088ceabd7bdae5e7ef07b6eb6.img` hat5.108.662.272 Bytes. Die reine lokale Recipekey-Berechnung für1595259 ergibt genau diesen Schlüssel. Vor einer erneuten Personalisierung müssen der installierte Recipekey und die zugehörige versions-JSON mit Commit1595259 ebenfalls passen. Dann überspringt der vorhandene Code den kompletten ARM64-Softwarebuild einschließlich expand / shrink / sfdisk; Kopie, Stationspersonalisierung, Dateisystemprüfung und Kompression bleiben erforderlich. Das erzeugt ein neues Artefakt mit eigener ID / SHA-256, keine bytegleiche Rekonstruktion des ursprünglichen Images. Die normale WebUI verwendet weiterhin frisch abgerufenes main und bietet diesen alten Pin bewusst nicht an.

Der geprüfte Produktcode löscht fertige Artefakte ausschließlich über den ausdrücklich bestätigten Image-Löschpfad; die Erfolgsjobs bleiben dabei erhalten. Profilentfernung, Updatehelfer und Work-Bereinigung löschen keinen fertigen Artefaktordner. Welcher externe Lösch- oder Datenträgerverlauf hier vorliegt, ist nicht belegt. Als Nächstes nach bestehenden `.img.xz` auf der lokalen VM suchen und Cache-Metadaten lesen. Builder-Rezeptdateien vorher nicht ändern, damit der vorhandene Cachekey erhalten bleibt.

**Dritter tatsächlicher Betreiberbefund:** Die installierte Recipekey-Berechnung für1595259, `result.recipe` des Erfolgsjobs und der vorhandene Cachekey769980… stimmen exakt überein. Cacheimage und versions-JSON sind vorhanden, die JSON nennt Commit1595259. Die vollständige Suche nach `.img.xz` auf der lokalen VM-Dateisystemansicht findet ausschließlich den 443.120.736-Byte-Raspberry-Pi-OS-Download. Der längere Traceback des neuen Builds zeigt die anfängliche Partitionsvergrößerung in `build()` vor ARM64-Kompilierung; die Phase ist damit zugeordnet. Warum sfdisk nach30s nicht beendet ist, bleibt offen.

### Stationsimage aus dem bestätigten Softwarecache neu erzeugen

[vm119-cached-image-recovery.py](vm119-cached-image-recovery.py) ist ein eigenständiger root-Helfer für genau diesen bestätigten VM119-Auftrag. Er wird aus dem bereitgestellten geprüften Quellstand mit `sudo python3 -B` ausgeführt. Er liest den ursprünglichen vollständigen Stationsauftrag ausschließlich aus SQLite mit `mode=ro`, normalisiert ihn mit dem installierten Validator und verweigert veränderte Stationseinstellungen. Cachegröße / Recipekey / Commit, alle Imagejobs und verwaiste Workpfade werden vor dem Start geprüft. Der normale Controller-Endpunkt verwendet weiterhin frisch abgerufenes main; nur diese begrenzte Rekonstruktion verwendet den alten bereits erfolgreich gebauten Stand1595259.

Der Helfer reiht über den bestehenden lokalen Worker-Socket genau einen neuen Auftrag ein. Ein root-0600-Beleg unter `/var/lib/netcore-image-builder/recovery-2e2603beacd942dfaddc7119b85a4af0.json` wird vorher per Datei- und Verzeichnis-fsync gespeichert. Wiederholtes Ausführen fragt anhand des Belegs ausschließlich den vorhandenen Auftrag ab. Nach verlorener Antwort, unklarem Ausgang, fehlgeschlagenem Auftrag oder mehrdeutigen Treffern erfolgt kein zweiter POST. Zugangsdaten bleiben im lokalen gespeicherten Auftrag und werden nicht im Beleg oder der Ausgabe wiedergegeben. Es werden keine Laufzeit-/Rezeptdateien ausgetauscht, Dienste gestoppt oder Cache-/Artefaktdateien gelöscht.

Bei diesem bestätigten Cachehit verwendet der unveränderte Builder die Softwarebasis und führt nur Kopie, Stationspersonalisierung, Dateisystemprüfung, Kompression und Veröffentlichung aus. Beliebige parallele Root-Wartung kann der bestehende Worker nicht atomar ausschließen; während des Auftrags keine Rezept-/Cachedaten manuell ändern. Der neue Auftrag erhält eine eigene ID, einen neuen Dateinamen und SHA-256. Ein Erfolgsbericht prüft Manifest / Dateigröße / Prüfsummendatei und nennt die drei Download-URLs; die vollständige Dateiprüfsumme wird dabei noch nicht erneut berechnet und ist anschließend am Download zu prüfen. Die tatsächliche Ausführung auf VM119 und der erneute Download sind bis zur Betreiber-Ausgabe offen.

Lokale Validierung:21 neue Recoverytests mit echten SQLite-/Beleg-/Artefaktdateien und dem tatsächlichen Imagevalidator bestanden; alle56 VM119-Operatortests bestanden. Workertransport / Softwarebuild sind isoliert; kein VM119- oder ARM64-Auftrag wird durch diese Tests ausgeführt. Verlorene / verspätete Antworten, fsync-Fehler vor und nach Annahme, versteckte aktive / ungeklärte Jobs, fremde Workdateien, geänderte Zugangsdaten, defekte Belege und inkonsistente Artefakte sind geprüft. Unabhängiger Review ohne offene Blocker. [Regressionen](../../../tests/integration/test_vm119_cached_image_recovery.py); Reproduktion: `python3 -B -m unittest discover -s tests/integration -p 'test_vm119_*.py' -v`. CI-Trigger und Operator-Testauswahl umfassen jetzt auch die Recoverydateien; der neue CI-Lauf ist beim Erstellen dieses Nachtrags noch nicht beobachtet.

### Tatsächlicher Recoveryversuch: SQLite-Lesefehler

Am09.10.2026 führt der Betreiber den veröffentlichten Helfer an `f4e1215c7ffc2ee418b261bb7f98e14bdda44b32` aus. Die heruntergeladene Datei `/var/tmp/netcore-image-recovery.IIVfhV.py` besteht die SHA-256-Prüfung `1b719f76cd8fad68f0c3152f6bbf389e7c066cb134a627c3b8420bd6c402e943`. Danach endet der Helfer mit `STOP: OperationalError; kein automatischer Neuauftrag. Vorhandenen Recoverybeleg prüfen.` Die bisherige generische Ausgabe enthält weder SQLite-Detail noch Belegphase. Deshalb ist nicht belegt, ob der Fehler bei einer Vorprüfung oder beim Lesen nach Annahme eines Auftrags entstand. Aus dieser Ausgabe allein wird kein weiterer Build abgeleitet.

Die SQLite-Abfrage bleibt ausschließlich `mode=ro`. Der Helfer nennt nun `OPEN_READONLY` oder `SELECT_JOBS` sowie SQLite-Fehlername / Fehlercode / tatsächliche Meldung der konstanten Abfrage. Ein weiterer Operatorblock liest nur Belegexistenz / phase / job_id, Datenbank- und Journalmetadaten, Spaltennamen und Auftrags-ID / Status; private Requestfelder werden nicht ausgegeben. Keine Wiederholung des POST, kein read-write-Fallback und keine Datenbankreparatur zur Diagnose.

Fünf zusätzliche SQLite-Regressionsfälle bestehen: fehlende Tabelle / Spalte, echte exklusive Sperre, fehlgeschlagenes Öffnen und Lesefehler nach bereits persistierter Auftragsannahme. Insgesamt26 Recoverytests und alle61 VM119-Operatortests lokal bestanden. Der veröffentlichte Ausgangshelfer f4e1215 besteht inzwischen auch beide vollständigen main-CI-Läufe: [Quellgate](https://github.com/JanHG98/netcore-tetra/actions/runs/37905773577), [Deployment/Discovery](https://github.com/JanHG98/netcore-tetra/actions/runs/37905773628). Der neue Diagnosepatch hat damit noch keinen eigenen CI- oder VM119-Nachweis. Nächster Schritt: konkrete SQLite-Meldung und Belegphase auf VM119 lesend zuordnen, danach den vorhandenen beziehungsweise noch nicht angenommenen Recoveryauftrag gezielt fortsetzen.

Keinen weiteren vollständigen Build, Cachelöschung oder Dienstneustart zur Diagnose auslösen. Zuerst die vorhandene Image-Datei auffinden beziehungsweise den geprüften Softwarecache für eine begrenzte erneute Personalisierung zuordnen. Anschließend Download / Manifest / SHA-256 abnehmen. Der vollständige Build ist durch den neuen Betreiberbefund bestätigt; Download- und physische Pi-/SXceiver-/VPN-Abnahme bleiben offen.

## Aktuelle TBS-Bedienung vom 09.10.2026

[Workflow-Update und Prüfgrenzen](tbs-image-ablauf-2026-10-09.md): gespeicherte Profile lassen sich löschen, sofern kein aktiver oder ungeklärter Auftrag sie verwendet. Neue Images verwenden automatisch frisch abgerufenes `origin/main`; eine Git-Referenzeingabe entfällt. Die ältere Ref-Eingabe in den unten datierten Build-/Hotfixanweisungen beschreibt die damalige Oberfläche. Der neue gezielte Helfer übernimmt sieben Controller-/Worker-/UI-Dateien; das Gastrezept bleibt separat. Die tatsächliche Installation auf VM119 ist noch nicht bestätigt.

## Operatorblock

Als `jhoffmeister` auf **VM-H-DEPLOY-01** ausführen. Der Block liest Status, Profile, gespeichertes Template, Buildjobs, Artefaktmetadaten, Werkzeug-/binfmt-Voraussetzungen und freien Platz. `python3 -B` vermeidet Import-Cachedateien. Die Ausgabe wählt Profil- und Jobmetadaten aus; Zugangsdaten werden für den Build später lokal eingegeben.

```bash
sudo python3 -B - <<'PY'
import hashlib, json, sys, tomllib, urllib.request
from pathlib import Path

sys.path.insert(0, "/usr/local/lib/netcore-deployment")
from common import load_config
from image_build import preflight
from image_spec import BASE

http = urllib.request.build_opener(urllib.request.ProxyHandler({}))
def api(path):
    with http.open("http://10.0.1.131:8320" + path, timeout=20) as response:
        return json.load(response)

def job_summary(job):
    request = job.get("request", {})
    return {**{key: job.get(key) for key in ("id", "status")},
            **{key: request.get(key) for key in
               ("kind", "build_id", "node_id", "service", "commit", "hostname")}}

status = api("/api/v1/status")
profiles = api("/api/v1/profiles")
images = api("/api/v1/images")
jobs = api("/api/v1/jobs")
cfg = load_config("/etc/netcore/deployment.toml")
template = Path(cfg["state_dir"]) / "tbs-site-template.toml"
parsed = tomllib.loads(template.read_text()) if template.is_file() else {}

print(json.dumps({
    "controller": {key: status.get(key) for key in
                   ("node_id", "role", "version", "advertise_url", "has_template")},
    "runtime_jobs_sha256": hashlib.sha256(
        Path("/usr/local/lib/netcore-deployment/jobs.py").read_bytes()).hexdigest(),
    "ref": status.get("settings", {}).get("ref"),
    "checked_commit": status.get("desired", {}).get("commit"),
    "template_sections": {key: isinstance(parsed.get(key), dict)
                          for key in ("net_info", "cell_info", "phy_io")},
    "profiles": [{key: profile.get(key) for key in
                  ("name", "mcc", "mnc", "issi", "la", "cc")} for profile in profiles],
    "builder": {key: images.get(key) for key in
                ("available", "error", "base", "free_bytes")},
    "image_jobs": [job_summary(job) for job in images.get("jobs", [])],
    "artifacts": [{key: artifact.get(key) for key in
                   ("id", "filename", "commit", "sha256", "boot_tested")}
                  for artifact in images.get("artifacts", [])],
    "critical_deployments": [job_summary(job) for job in jobs
                            if job.get("status") in ("queued", "running")
                            or job.get("result", {}).get("remote_uncertain")],
}, indent=2, ensure_ascii=False))

state = Path("/var/lib/netcore-image-builder")
preflight(state)
print("PASS: Buildwerkzeuge, ARM64-Emulation und mindestens 32 GiB frei.")
print("OS-Basisdatei im Cache:",
      (state / "cache" / (BASE["sha256"] + ".img.xz")).is_file())
PY
```

## Auswertung und anschließender Build

- Builder erreichbar: `available=true`, kein Fehler. Offene Image-/Deploymentaufträge oder `remote_uncertain` vor dem Build anhand ihrer bestehenden ID zuordnen.
- Standort-TOML mit `net_info`, `cell_info`, `phy_io` vorhanden und parsebar; TBS-Profile mit Name/MCC/MNC/ISSI/LA/CC erfasst. Vorhandene Standortwerte erhalten.
- Installiertes `image_build.preflight()` prüft root, erforderliche Buildwerkzeuge, ARM64-binfmt mit F-Flag und mindestens 32 GiB frei. Die Prüfung benötigt keinen Rust-Compiler auf der VM; Gast-Buildwerkzeuge werden innerhalb des ARM64-Images eingerichtet.
- Cacheanzeige bestätigt ausschließlich Dateiexistenz. Der Build prüft die feste OS-Basis-SHA-256 und lädt bei Bedarf automatisch das in `image_spec.BASE` definierte Raspberry-Pi-OS-ARM64-Basisimage.
- Den Fingerprint von installiertem `jobs.py` mit dem gemeinsamen Quellstand vergleichen: Der zuletzt bekannte VM119-Rollout war c45a2ec; die später korrigierte SQLite-Auftragsverwaltung wurde in dieser Sitzung bislang auf CT150 installiert. Der Fingerprint ermöglicht die Zuordnung vor dem langen Build.
- Nach dem Workflow-Update vorhandenes Profil, Hostname und Betriebssystem-Zugang (SSH-Public-Key oder Passwort) wählen; passendes HAT-Overlay festlegen. Für die VPN-Abnahme gültiges Inline-OpenVPN-Profil und vertraute SSIDs lokal angeben. Zugangsdaten im Betrieb eingeben, nicht im Repo ablegen.
- Ein voller ARM64-NetCore-Build erzeugt ein Artefakt samt Manifest/Prüfsumme. Die physische Pi-/SXceiver- und VPN-Abnahme folgt anschließend; `boot_tested=false` bleibt bis zur tatsächlichen Hardwareprüfung maßgeblich.

Primärquellen: `system-backend/deployment-core/main.py`, `common.py`, `image_worker.py`, `image_spec.py`, `image_build.py`. Der Operatorblock ist gegen diese Quellschnittstellen geprüft und syntaktisch validiert. Die tatsächliche VM119-Vorprüfung besteht gemäß nachfolgendem Betreiberbefund.

## Tatsächliche Vorprüfung am 2026-10-08

[Betreiberbefund](evidence/vm119-imagebuilder-preflight-2026-10-08.json): Controller 0.3.0, Builder verfügbar ohne Fehler, ARM64-Emulation und Buildwerkzeuge geprüft, 84.586.332.160 Bytes / rund 78,78 GiB frei. Alle drei Template-Sektionen vorhanden; Profile SRV-M-TBS-01/02/03 mit MCC 901/MNC 1510 und ihren vorhandenen ISSI-/LA-/CC-Werten erfasst. Keine Imagejobs, Artefakte oder kritischen Deploymentaufträge. Die OS-Basisdatei ist noch nicht im Cache; der tatsächliche Build lädt und prüft sie automatisch.

Der Quellabgleich mit c45a2ec und main 0cf6df0 ordnet die installierte `jobs.py` eindeutig zu:

| Datei | SHA-256 | Stand |
| --- | --- | --- |
| Installierte `jobs.py` | `30ce8b0f936383f9c20ddb5ffda244c4065b1de1a567ddf3fbda360058d17e64` | Bytegleich mit c45a2ec, SQLite-Korrektur fehlt auf VM119 |
| Korrigierte `jobs.py` | `905e5fc4d257e5d1ad56542cbaef38d2f42a00af357240c0e5c92ea7facc2c21` | Bytegleich zwischen Fix 31829fc und main 0cf6df0 |

Controller und Imageworker importieren dieselbe installierte Datei. Zwischen c45a2ec und main 0cf6df0 ist sie die einzige geänderte Laufzeitdatei des Deployment-Core; weitere Änderungen dort betreffen Tests. Die API-Versionsnummer allein bestätigt diese Korrektur nicht.

## Eng begrenzte Übernahme vor dem Build

[vm119-jobs-update.py](vm119-jobs-update.py) ersetzt ausschließlich `/usr/local/lib/netcore-deployment/jobs.py` mit geprüftem Fingerprint und startet die beiden vorher untätigen Dienste neu. Kein Installer-, Paket-, Unit- oder Datenbankmigrationslauf. Hostname/Root, aktive Dienste, Controller- und Imageaufträge sowie unveränderte Deployment-Konfiguration, Settings, Profile und Standorttemplate werden geprüft; bei Fehler wird die vorherige Datei aus dem Arbeitsspeicher zurückgesetzt. Ein neuer automatischer Git-Prüfauftrag nach Controllerneustart ist zulässig. Der Helfer startet keinen Imagebuild.

Als jhoffmeister auf VM119 den Checkout auf den bereitgestellten vollständigen Quellcommit setzen, dann `sudo python3 -B "$NC_SRC/Docs/integration/Z01-2026-10-07/vm119-jobs-update.py"` ausführen. Der tatsächliche Austausch und die Nachprüfung auf VM119 bleiben bis zur Betreiber-Ausgabe offen.

Lokale Helperprüfung: Syntax und sechs Testmethoden mit elf isolierten Szenarien bestehen, einschließlich tatsächlichem Dateiaustausch / Rücktausch, Eigentümer-/Moduserhalt, unveränderten Konfigurationsbytes, echten SQLite-Dateien, aktiven / ungeklärten Aufträgen und automatischem Git-Check nach Start. Dienststeuerung und HTTP sind dabei simuliert; dies ersetzt keine VM119-Ausführung. Reproduktion: `python3 -B -m unittest discover -s tests/integration -p test_vm119_jobs_update.py -v`. [Tests](../../../tests/integration/test_vm119_jobs_update.py).

## Erster vollständiger Build

Vorhandenes Profil SRV-M-TBS-01 ist die vorgeschlagene erste Auswahl; die anderen Profile bleiben auswählbar. Einen eindeutigen Testhostname angeben. Nach dem Workflow-Update holt der Controller beim Absenden den neuesten Commit von `origin/main` und hält ihn für diesen Build fest. Dafür muss der globale Controllerref c45a2ec nicht geändert werden. SSH-Public-Key oder Betriebssystem-Passwort sowie gegebenenfalls WLAN-/OpenVPN-Daten lokal eingeben; keine Geheimnisse in Chat, Repo oder Diagnoseausgabe übernehmen.

Den Build genau einmal über `/api/v1/images/build` oder die Imagebuilder-WebUI absenden. Bei unklarer Antwort anhand der bestehenden Builds prüfen und den POST nicht wiederholen. Imagejobs werden über `/api/v1/images` unter `jobs[]` verfolgt; Controller-Job-URLs gehören zu einer anderen Auftragsdatenbank. `job.id` und `request.build_id` sind unterschiedliche IDs. Nach `succeeded` ist `request.build_id` beziehungsweise `result.id` die Artefakt-ID; Image, SHA-256 und Manifest sind unter `/api/v1/images/<Artefakt-ID>/image`, `/sha256`, `/manifest` abrufbar. Manifestcommit und Dateiprüfsumme vergleichen. Die anschließende physische Pi-/SXceiver- und VPN-Abnahme bleibt getrennt; `boot_tested=false` ist bis dahin korrekt.

### Bestehende WebUI bedienen

Nach bestandener Updateprüfung `http://10.0.1.131:8320` öffnen, Abschnitt **Vom Profil zur Basisstation.**:

| Feld | Erste Auswahl |
| --- | --- |
| TBS-Profil | SRV-M-TBS-01 |
| Quellstand | Automatisch neuester Commit von origin/main; keine Eingabe. Der Auftrag und das Manifest halten den vollständigen SHA fest. |
| Adresse dieser VM | http://10.0.1.131:8320 |
| Hostname des Pi | z014-pi-01 |
| Benutzer auf dem Pi / Zeitzone | jan / Europe/Berlin |
| SSH-Public-Key / Oder: Betriebssystem-Passwort | Lokal vollständigen Public-Key oder eigenes Passwort eingeben |
| SXceiver-HAT | EEPROM-Auswahl bei programmiertem HAT-EEPROM; andernfalls SX1255-Overlay aus dem Image |

Das EEPROM lässt sich aus der SXceiver-Platinenversion allein nicht ableiten. WLAN und VPN für einen ersten LAN/DHCP-Build bei Bedarf leer lassen; für die VPN-Abnahme das echte Inline-OpenVPN-Profil und die vertrauten lokalen SSIDs angeben. **Image erstellen →** genau einmal anklicken, danach unter **Builds & Downloads** / **Buildprotokoll** verfolgen. Nach Erfolg **Image ↓**, **SHA256** und **Manifest** herunterladen.

Der eigene OS-Hostname ändert nicht die Stationsidentität: Agent-node_id und Funkkennungen stammen aus dem ausgewählten Profil. Ein reiner Imagebuild beeinflusst keine laufende Station; der spätere physische Parallelboot benötigt eine bewusst gewählte Stationsidentität. Quellen: `static/index.html`, `static/app.js`, `image/personalize.py`, `image/check-hardware.py`.

## Tatsächlicher ARM64-Buildabbruch: Gast-DNS

[Betreiber-Logbefund vom 08.10.2026](evidence/vm119-imagebuilder-dns-2026-10-08.json): Build-ID `98a096a050284d6e952bdff78eff6caa` erreicht ARM64-chroot/Paketdownload; `deb.debian.org` und `archive.raspberrypi.com` melden wiederholt `Temporary failure resolving`. APT/chroot endet100, äußerer unshare-Aufruf1. Der Auszug dokumentiert sämtliche Unmounts und Detach von `/dev/loop20`. Ein Artefakterfolg oder eine Bestätigung des vorausgehenden jobs.py-Austauschs ist darin nicht enthalten.

**Konkreter Quellfehler:** Die Builder-Unit setzt `UMask=0077`. `Disk.guest()` schreibt die temporäre `/etc/resolv.conf` neu, ohne ihren Modus festzulegen; dadurch entsteht root-eigenes 0600. Ein tatsächlicher lokaler Datei-/UMask-Repro bestätigt diesen Modus. APT 2.6.1 verwendet für Netzwerkabrufe standardmäßig `_apt`, der diese Datei nicht lesen kann. Primärquellen: [APT-Benutzer](https://sources.debian.org/src/apt/2.6.1/apt-pkg/init.cc/), [Privilegienwechsel](https://sources.debian.org/src/apt/2.6.1/methods/aptmethod.h/). Dies erklärt den Logbefund; der damalige Gast wurde nicht direkt auf Rechte/effektive Unit/DNS untersucht.

**Gezielte Korrektur:** Der temporäre Gastresolver erhält explizit 0644; die originale Datei oder ihr Symlink wird nach dem Gastkontext wiederhergestellt. Worker-UMask 0077 und Hostresolver bleiben erhalten. Vor dem Paketbuild prüft `guest_dns()` als tatsächlicher Benutzer `_apt` die Lesbarkeit und IPv4-Auflösung beider Paketserver, mit begrenzten Zeitlimits. Die Prüfung läuft im realen ARM64-chroot beim nächsten Betreiberbuild. Der Builder teilt das Netz der VM; der Host-Stub 127.0.0.53 muss deshalb nicht pauschal ersetzt werden.

[vm119-image-dns-update.py](vm119-image-dns-update.py) verwendet die bereits geprüften Host-/Idle-/Dateierhalt-/Rückweg-Guards und ersetzt ausschließlich `/usr/local/lib/netcore-deployment/image_build.py`, mit festen vorher/nachher-SHA-256-Werten. Der bestehende Jobs-Helfer wurde dafür nur allgemein beschriftet. Kein Installer-, Paket-, Unit-, Host-DNS- oder Konfigurationswechsel. Ausführen als jhoffmeister auf VM119 nach Checkout des bereitgestellten vollständigen Korrekturcommits:

```bash
sudo python3 -B "$NC_SRC/Docs/integration/Z01-2026-10-07/vm119-image-dns-update.py"
```

Danach die WebUI neu laden und **einen neuen Build** mit demselben gewünschten Profil und dem vollständigen Korrekturcommit im Feld **Branch, Tag oder Commit** anlegen. Der bisherige fehlgeschlagene Auftrag wird nicht erneut abgesendet oder manuell weiterbearbeitet. Die geprüfte komprimierte OS-Basis kann aus dem Downloadcache verwendet werden; der neue Buildercode ändert automatisch den Recipekey. Der Logauszug liefert keinen Grund für manuelle Mount-, Loop- oder Cache-Bereinigung.

Lokale Regressionen verwenden echte Dateien, Umask, Gastkontext und SQLite-Dateien, simulieren aber Mounts/Geräteknoten, Dienste und externe DNS-Aufrufe. Temporäres 0644, Inhalt/Originalmodus, dangling Symlink, fehlender Resolver sowie Wiederherstellung bei Fehler sind geprüft. DNS-Tests prüfen `_apt`, beide Hosts, Zeitlimits und sofortigen Abbruch auf Lesbarkeits-/Auflösungsfehler. Zum damaligen DNS-Korrekturstand blieb der tatsächliche ARM64-DNS-/APT-Erfolg offen. Der nachfolgende zweite Betreiberbuild bestätigt inzwischen DNS-/Paketfortschritt; dessen dpkg-Abbruch steht im nächsten Abschnitt. Der vollständige Imagebuild bleibt offen.

Validierung des finalen Korrekturstands: Deployment-Core-Suite 62 Tests, 61 bestanden/ein erwarteter Unix-Socket-Skip; VM119-Update-Suite 11/11 bestanden. Syntax und unabhängiger Review bestehen. [Gastregression](../../../system-backend/deployment-core/tests/test_image_guest.py), [Update-/Rückwegregression](../../../tests/integration/test_vm119_image_dns_update.py).

## Zweiter ARM64-Build: dpkg-Konfigurationsrückfrage

[Betreiberbefund vom 08.10.2026](evidence/vm119-imagebuilder-conffile-2026-10-08.json), Build-ID `2dce020e97d745408fe5221453ad63ac`: ARM64-Pakete werden entpackt/konfiguriert. Der korrigierte Builder erreicht den Gast-Paketbuild nach seiner DNS-Vorprüfung; im neuen Auszug fehlen die vorherigen DNS-Abbrüche. Die beiden main-Workflows des DNS-Fixes 8359f75 bestehen inzwischen. Dies bestätigt Fortschritt über den früheren DNSfehler, keinen vollständigen Imageerfolg.

**Primärfehler:** `initramfs-tools-core` fordert eine Entscheidung für die im Raspberry-Pi-Basisimage bereits angepasste `/etc/initramfs-tools/initramfs.conf`. Der unbeaufsichtigte Build hat keinen Eingabestrom; dpkg endet mit `end of file on stdin at conffile prompt`. Die weiteren Initramfs-/Kernel-/Header-Fehler folgen aus diesem nicht konfigurierten Paket. Die Meldungen `policy-rc.d denied execution` entsprechen dagegen dem absichtlich unterdrückten Dienststart im Gast. Unmounts und Loopdetach sind laut Auszug durchgelaufen.

**Korrektur nur im Gastrezept:** Alle drei Paketoperationen nutzen `Dpkg::Options::=--force-confdef` und `Dpkg::Options::=--force-confold`, neben `DEBIAN_FRONTEND=noninteractive`. Damit entscheidet dpkg unbeaufsichtigt anhand des Defaults und behält bei fehlendem Default die vorhandene Konfiguration bei. [Primärquelle dpkg](https://manpages.debian.org/bookworm/dpkg/dpkg.1.en.html). `apt-get update` erhält zusätzlich `APT::Update::Error-Mode=any`, damit fehlgeschlagene Indexabrufe nicht still mit alten Listen weiterlaufen. Die bereits korrigierten DNS-Dateirechte bleiben erhalten. Die VM selbst erhält kein Paket-/Kernel- oder Standortkonfigurationsupdate.

[vm119-image-apt-update.py](vm119-image-apt-update.py) ersetzt ausschließlich `/usr/local/lib/netcore-deployment/image/build-guest.sh` mit festen SHA-256-Pins. Der wiederverwendete VM119-Guard prüft die Bash-Syntax, aktive Dienste, untätige Aufträge, Standort-Dateierhalt und Rücknahme. Seine Standard-Pythonprüfung bleibt für die bisherigen Jobs-/DNS-Wrapper erhalten; der ImageClient wird aus dem festen Deployment-Laufzeitverzeichnis geladen.

Als jhoffmeister auf VM119 nach Checkout des bereitgestellten vollständigen Korrekturcommits:

```bash
sudo python3 -B "$NC_SRC/Docs/integration/Z01-2026-10-07/vm119-image-apt-update.py"
```

Danach **einen neuen Build** mit demselben gewünschten Profil und dem vollständigen neuen Quellcommit im Imageformular anlegen. Der fehlgeschlagene, bereits aufgeräumte Gast wird nicht manuell repariert. Der Rezeptcode ändert den Recipekey automatisch; die feste komprimierte OS-Basis kann weiterhin aus dem Downloadcache genutzt werden. Tatsächlicher erneuter ARM64-Paket-/NetCore-Erfolg sowie Manifest/SHA-256 und physischer Pi-/SXceiver-/VPN-Nachweis bleiben offen.

Validierung des finalen APT-Korrekturstands: Deployment-Core 66 Tests, 65 bestanden/ein erwarteter Unix-Socket-Skip; VM119-Operator 18/18 bestanden; Bash-Syntax und unabhängiger Review bestehen. Die [Paketregression](../../../system-backend/deployment-core/tests/test_build_guest_packages.py) führt den tatsächlichen Bash-Rezeptanfang über einen isolierten APT-Harness aus und verarbeitet echte private Testpakete mit `dpkg --root`: Ohne Optionen EOF reproduziert, mit Rezeptoptionen Version 2 erfolgreich konfiguriert und eigene Datei bytegleich erhalten. [Skript-Austausch-/Rückwegtests](../../../tests/integration/test_vm119_image_apt_update.py). Dies ist ein nativer isolierter dpkg-Test, kein vollständiger ARM64-/Repository-Build.

## Gesicherter Fortsetzungsstand: 08.10.2026, 17:03 Europe/Berlin

Der vollständige Gast-APT-Korrekturcommit ist `001fb84ac566bb0f95e18d22439ee664fe0093e4`. Seine beiden main-Workflows sind erfolgreich abgeschlossen: [Deployment-/Quellgate](https://github.com/JanHG98/netcore-tetra/actions/runs/37795951602), [Deployment/Discovery](https://github.com/JanHG98/netcore-tetra/actions/runs/37795951576). [CI-Beleg](evidence/checkpoint-ci-2026-10-08.json).

Der bereitgestellte Operatorbefehl verwendet diesen Commit und `vm119-image-apt-update.py`; im Imageformular gehört derselbe vollständige SHA in **Branch, Tag oder Commit**. Ein globaler Controller-Ref-Wechsel ist dafür nicht erforderlich. Zum Zeitpunkt dieser Sicherung fehlt eine neue Betreiber-Ausgabe zur Gastrezept-Übernahme und zum erneuten vollständigen Build. Der jobs.py-Fix auf VM119 ist ebenfalls nicht durch einen neuen Runtime-Fingerprint bestätigt. Bereits gestartete Builds über ihren bestehenden Auftrag verfolgen; diese Dokumentationssicherung erfordert keinen weiteren Build oder Dienstneustart.

Bei einem Erfolg zuerst `/api/v1/images`, den bestehenden Imagejob, Artefakt und Manifest auswerten und den Download gegen die SHA-256 prüfen. Imagejob-ID und `request.build_id` sind verschiedene Kennungen; Controller-`/api/v1/jobs/<id>` ist kein Imagejob-Statuspfad. Das Manifest darf bis zur tatsächlichen physischen Abnahme weiterhin `boot_tested=false` melden. Erst danach folgen Pi-/SXceiver-Boot und VPN-Wechsel. [Gesamter Fortsetzungsstand](zwischenstand-2026-10-08.md).

## Dritter ARM64-Build: Rootgeräteerkennung beim Initramfs

[Betreiberbefund vom 08.10.2026](evidence/vm119-imagebuilder-initramfs-2026-10-08.json), Build-ID `fb5683fe496740deae2916551451e69c`: `initramfs-tools-core` übernimmt die vorhandene initramfs.conf unbeaufsichtigt (`Keeping old config file as default`). Der bisherige Conffile-Abbruch ist damit überwunden. Bei den Kernel-Postinst-Skripten für 6.12.109 Pi-v8 und Pi2712 sowie beim Trigger des bisherigen 6.12.25-Kernels endet mkinitramfs nun mit `failed to determine device for /`; der Log nennt selbst `MODULES=most` als Ausweg. Nachfolgende Kernel-/Header-Abhängigkeitsfehler sind Folgefehler. Unmounts und Detach von `/dev/loop20` sind dokumentiert.

**Quellanschluss:** Der Debian-Bookworm-Code ruft im Zweig `MODULES=dep` die laufzeitbezogene Rootgeräteerkennung auf. `MODULES=most` verwendet die breite Treiberauswahl für ein portables Image. Das ist für das ARM64-Image im Build-chroot passend; die tatsächliche Gastkonfiguration wurde nicht separat ausgelesen. [Initramfs-Konfiguration](https://manpages.debian.org/bookworm/initramfs-tools-core/initramfs.conf.5.en.html), [mkinitramfs](https://sources.debian.org/src/initramfs-tools/0.142%2Bdeb12u3/mkinitramfs/), [Geräteerkennung](https://sources.debian.org/src/initramfs-tools/0.142%2Bdeb12u3/hook-functions/).

**Gezielte Korrektur:** Vor der ersten APT-Operation erstellt das Gastrezept `/etc/initramfs-tools/conf.d/zz-netcore-image.conf` mit `MODULES=most`, explizit0644 auch unter Worker-Umask0077. conf.d wird nach der Hauptdatei geladen; die vorhandene initramfs.conf bleibt bytegleich erhalten. Die neue Datei bleibt Bestandteil des Pi-Images, damit spätere Kernelupdates dieselbe portable Auswahl verwenden. Normale Kernel-/Initramfs-Erzeugung und alle bisherigen Conffile-/DNS-Korrekturen bleiben aktiv. Die Initramfs kann durch die breitere Treiberauswahl größer werden.

[vm119-image-initramfs-update.py](vm119-image-initramfs-update.py) verwendet unverändert die bestehenden Host-/Idle-/Dateierhalt-/Rückweg-Guards und ersetzt ausschließlich `/usr/local/lib/netcore-deployment/image/build-guest.sh`. Die Ausführung erfolgt als jhoffmeister auf VM119 nach Checkout des bereitgestellten vollständigen Korrekturcommits:

```bash
sudo python3 -B "$NC_SRC/Docs/integration/Z01-2026-10-07/vm119-image-initramfs-update.py"
```

Danach einen neuen Build mit demselben gewünschten Profil und dem vollständigen Korrekturcommit im Imageformular anlegen. Die bisherige Build-ID benennt den gescheiterten Lauf; seine Imagejob-ID und der vollständige Request-/Quellcommit sind im Auszug nicht enthalten. Das neue Gastrezept ändert den Recipekey automatisch, die feste komprimierte OS-Basis kann aus dem Downloadcache wiederverwendet werden. Der aufgeräumte fehlgeschlagene Gast benötigt keine manuelle Paket-, Mount- oder Loopreparatur.

Die lokale Regression führt den echten Rezeptanfang im isolierten Testverzeichnis aus und prüft die wirksame Initramfs-Konfiguration vor den Paketaufrufen, Konfigurationserhalt, Rechte und Fehlerabbruch. Externe APT-Aufrufe sind isoliert. Eine tatsächliche ARM64-Initramfs wird in dieser Umgebung nicht erzeugt; der vollständige Betreiberbuild und der physische Pi-/SXceiver-/VPN-Test bleiben die nächsten Nachweise.

Validierung des finalen Initramfs-Korrekturstands: Deployment-Core68 Tests /67 bestanden /1 erwarteter Unix-Socket-Skip; VM119-Operator26/26 bestanden; sechs betroffene Gast-Pakettests bestanden; Bash-Syntax und unabhängiger Review ohne Blocker. [Gastregression](../../../system-backend/deployment-core/tests/test_build_guest_packages.py), [Update-/Rückwegtests](../../../tests/integration/test_vm119_image_initramfs_update.py). Die neue main-CI war beim Erstellen des Belegs noch nicht beobachtet.

## Bestätigte Initramfs-CI / weiterhin offener Betreiberbuild

Am Quellcommit `1595259a2a76abfc9eff08842409156473b585a7` sind beide main-Workflows erfolgreich abgeschlossen: [Deployment/Discovery](https://github.com/JanHG98/netcore-tetra/actions/runs/37804380374) und [Deployment-/Quellgate](https://github.com/JanHG98/netcore-tetra/actions/runs/37804380390). Der oben datierte Hinweis auf damals noch nicht beobachtete CI bleibt historisch. Diese Ergebnisse bestätigen keinen vollständigen ARM64-NetCore-Build oder physischen Pi-Boot. Die Anlagenübernahme der neuen TBS-Bedienung und des Gastrezepts wird jeweils durch die Betreiber-Ausgabe bestätigt.
