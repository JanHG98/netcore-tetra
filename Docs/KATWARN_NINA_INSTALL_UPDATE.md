# NINA/KATWARN: neuer Warn-LXC und Update aller LXCs/TBS

Diese Anleitung gehört zur Warnfunktion für den Zielbranch `katwarn/nina`. Der vollständige Funktionsstand liegt zunächst im Review-Branch `feat/katwarn-nina-alerts`; die Befehle verwenden diesen Branch. Nach dem Merge kann `NETCORE_REF=katwarn/nina` verwendet werden. Sie ergänzt die vorhandene Installation um **einen** LXC `alert-service`. Die Warnzentrale verwendet die im Control Room vorhandenen Gerätepositionen und den vorhandenen SDS Router. Eigene Warnungen werden in ihrer WebUI auf einer Karte mit Mittelpunkt, Radius und Gültigkeit erstellt und wieder gelöscht.

Die Schritte erst mit dem veröffentlichten Commit dieses Funktionsstands ausführen. Alle IP-Adressen, CT-IDs, Benutzernamen und ISSIs unten sind Beispiele und müssen zum eigenen Netz passen. Vorhandene lokale Änderungen bleiben erhalten; es wird kein `git reset --hard` verwendet.

## 1. Verhalten und Voraussetzungen

- Online-Geräte mit ausreichend aktueller GPS-Position werden gegen aktive Warngebiete geprüft. Dadurch erhält ein neu angemeldetes Gerät auch eine bereits bestehende Warnung. Die Erkennung erfolgt beim nächsten Geräteabgleich, standardmäßig nach spätestens etwa 5 Sekunden; NINA wird standardmäßig alle 60 Sekunden abgefragt, jeweils zuzüglich Laufzeit der Abfragen.
- Eine Warnung wird nur für Geräte innerhalb ihres Warngebiets versendet. Ohne verwertbare Position erfolgt keine Zuordnung. `gps_max_age_seconds` begrenzt das Alter der verwendeten GPS-Position, im Beispiel auf eine Stunde.
- Zusätzlich muss die zugehörige TBS im Control Room verbunden sein und einen aktuellen Status besitzen. `node_max_age_seconds` begrenzt dessen Alter, standardmäßig auf 120 Sekunden. Damit werden alte Online-Einträge einer abgetrennten TBS nicht als aktuelle Empfänger verwendet.
- Die Empfängerhistorie liegt **dauerhaft in SQLite**. Neustart, erneute Anmeldung und Verlassen/Wiedereintritt in denselben Warnbereich setzen diese Historie nicht zurück.
- Der Dienst liest die öffentliche NINA-Schnittstelle des BBK (`mowas`, `katwarn`, `biwapp`, `dwd`, `lhp`, jeweils `mapData.json` und zugehörige Warnungsdetails/Geometrien). Quellen sind in `[nina].sources` konfigurierbar. KATWARN-Meldungen sind enthalten, soweit sie dort bereitgestellt werden. Ein separater authentifizierter KATWARN-Partnerzugang ist nicht Bestandteil dieser Installation.
- Die eigene WebUI benötigt ein Zugangstoken. Der Installer erzeugt es lokal. Der Dienst läuft als `netcore-alert`, nicht als root. Der HTTP-Port gehört in das interne Managementnetz; vorhandenen HTTPS-Reverse-Proxy bei Bedarf davorschalten.
- Die Erstinstallation startet mit `delivery.enabled = false`. Kartenanzeige, Datenabruf und Konfiguration können damit geprüft werden, bevor Funkgeräte Nachrichten erhalten.

Ein angenommener SDS-Versand ist kein Nachweis, dass ein Funkgerät die Meldung angezeigt oder ein Benutzer sie gelesen hat. Die Warnzentrale ist eine zusätzliche Meldungsanzeige für das bestehende Netz. Details zu Zustellstatus, Wiederholungen und Mehrdeutigkeiten stehen auch in der [Dienstbeschreibung](../system-backend/alert-service/README.md).

Der aktualisierte SDS Router speichert den Auftragsschlüssel dauerhaft und behandelt Warnungen im Modus `at_most_once`. Wiederholte HTTP-Anfragen nach einem Timeout erzeugen dadurch keinen zweiten Funkauftrag. Bei unklarem Ausgang nach Übergabe an die Funkstrecke erfolgt keine automatische erneute Funkzustellung; dies vermeidet Duplikate, kann aber eine Warnung unbestätigt lassen. Im WebUI bedeutet `pending` vorbereitet, `submitted` an den Router übergeben, `accepted` von der TBS angenommen, `uncertain` Ergebnis unklar, `failed` fehlgeschlagen, `cancelled` zurückgezogen und `expired` abgelaufen. Ein alter SDS Router ohne die neuen Fähigkeiten wird erkannt; die Warnzentrale sendet dann nicht. Auch die persistente SDS-Router-Datenbank muss beim Update erhalten bleiben: maßgeblich ist `storage.database_path`, standardmäßig `/var/lib/netcore-sds-router/messages.json`. Sie enthält zusätzlich zur Queue die dauerhafte Duplikatsperre, auch nach Ablauf einzelner Queue-Einträge.

## 2. Adressen und Reihenfolge

| Komponente | Beispieladresse | Zweck |
|---|---|---|
| Warnzentrale (neu) | `10.0.20.34:8310` | Karten-WebUI, Warnungen, Zustellhistorie |
| Control Room | `10.0.20.25:9010` | Online-Geräte und letzte GPS-Position |
| SDS Router | `10.0.20.17:8150` | Unicast-SDS an die Ziel-ISSI |
| Node Gateway | tatsächliche vorhandene Adresse, Port `8080` | Weiterleitung an die verbundene TBS |
| NINA | `https://warnung.bund.de/api31` | Warnmeldungen und Warngebiete |

Port **8230 bleibt Media Library**; der neue Dienst verwendet **8310**. Die tatsächlichen bestehenden Adressen aus `/etc/netcore/*.toml`, dem eigenen Inventar und der DHCP-Reservierung übernehmen.

Reihenfolge im Wartungsfenster: Sicherungen → Quellen auf allen betroffenen Hosts aktualisieren → Node Gateway/SDS Router/Control Room aktualisieren → weitere bereits installierte LXCs nach Tabelle aktualisieren → TBS nacheinander aktualisieren → neuen Warn-LXC installieren → GPS und SDS prüfen → Versand aktivieren. Laufende Gespräche vor einem TBS-Neustart beenden.

Für diese Erweiterung müssen keine zusätzlichen MQTT-, SIP-, Medien- oder Funkhardware-Dienste installiert werden. Bestehende dieser Dienste können anhand der Tabelle auf denselben Quellstand gebracht werden.

Der erforderliche Funktions-Rollout besteht aus dem **neuen Warn-LXC und dem aktualisierten SDS Router**. Bereits funktionierende Control-Room-/Node-Gateway-/TBS-Versionen mit Online-/GPS-Telemetrie und `DeliverSds` können weiterverwendet werden; diese Erweiterung verändert ihre Schnittstelle nicht. Die folgenden vollständigen Updatepfade dienen dazu, auf Wunsch alle vorhandenen LXCs und TBS auf denselben Branchstand zu bringen.

## 3. Quellen auf vorhandenen Hosts aktualisieren

Auf LXCs liegt das Repository normalerweise unter `/opt/netcore-tetra`, auf einer TBS beispielsweise unter `/home/jan/netcore-tetra`. Die folgenden Befehle als Eigentümer des jeweiligen Checkouts ausführen:

```bash
cd /opt/netcore-tetra
git status --short
git rev-parse HEAD
NETCORE_REF=feat/katwarn-nina-alerts
git fetch origin "$NETCORE_REF:refs/remotes/origin/$NETCORE_REF"
git switch "$NETCORE_REF"
git pull --ff-only origin "$NETCORE_REF"
git log -1 --oneline
```

Bei noch nicht lokal vorhandenem Branch alternativ einmalig `git switch --track "origin/$NETCORE_REF"` verwenden. Meldet Git lokale Änderungen oder divergierende Historie, vor dem nächsten Schritt diese Änderungen sichern und bewusst zusammenführen. Keine Dateien oder Konfigurationen verwerfen. Bei einem Checkout mit nur einem zuvor geklonten Branch diesen Remote-Branch ausdrücklich holen:

```bash
git fetch origin "$NETCORE_REF:refs/remotes/origin/$NETCORE_REF"
```

Die vor dem Update ausgegebene Commit-ID im Wartungsprotokoll sichern. Auf allen Hosts dieselbe neue Commit-ID verwenden. Bestehende `/etc`-Konfigurationen mit den Beispielen vergleichen; Beispiele niemals pauschal über funktionierende Konfigurationen kopieren.

## 4. Vorhandene LXCs sichern und aktualisieren

Die folgende Tabelle deckt alle aktuell deploybaren Backend-Dienste ab. **Pro LXC nur die dort tatsächlich installierten Dienste aktualisieren.** Alle Updatepfade beziehen sich auf das Repository-Hauptverzeichnis. Normalerweise lautet die Konfiguration `/etc/netcore/<Dienst>.toml` und die Unit `netcore-<Dienst>.service`; Ausnahmen stehen in der Tabelle.

| Dienst/LXC | Port | Updatebefehl als root im Repository | Besonderheit |
|---|---:|---|---|
| node-gateway | 8080 | `bash system-backend/node-gateway/install/update.sh` | Bestehende TBS-Verbindungen danach prüfen |
| mobility-core | 8090 | `bash system-backend/mobility-core/install/update.sh` | Online-/Serving-TBS-Zustand prüfen |
| subscriber-core | 8100 | `bash system-backend/subscriber-core/install/update.sh` | Teilnehmerprofile beibehalten |
| group-core | 8110 | `bash system-backend/group-core/install/update.sh` | Gruppen/Mitgliedschaften prüfen |
| call-control | 8120 | `bash system-backend/call-control/install/update.sh` | Gespräche vor Neustart beenden |
| provisioning-core (falls vorhanden) | 8125 | `bash system-backend/provisioning-core/install/update.sh` | Geräte-/Gruppenmatrix sichern |
| media-switch | 8130 | `bash system-backend/media-switch/install/update.sh` | Medien-/Sprechtest nach Neustart |
| recorder | 8140 | `bash system-backend/recorder/install/update.sh` | Aufnahmeverzeichnis/NFS nicht ersetzen |
| sds-router | 8150 | `bash system-backend/sds-router/install/update.sh` | Für Warnzustellung erforderlich; Queue sichern |
| packet-core | 8160 | `bash system-backend/packet-core/install/update.sh` | Bestehende PDP-/NSAPI-Konfiguration erhalten |
| ip-gateway | 8170 | `bash system-backend/ip-gateway/install/update.sh` | TUN/NAT/Firewall unverändert überprüfen |
| security-core | 8180 | `bash system-backend/security-core/install/update.sh` | Persistente Sicherheitsdaten sichern |
| kmf | 8190 | `bash system-backend/kmf/install/update.sh` | Schlüssel/Vault separat nach KMF-Anleitung sichern |
| transit | 8200 | `bash system-backend/transit/install/update.sh` | Peer-/Routingkonfiguration prüfen |
| observability | 8210 | `bash system-backend/observability/install/update.sh` | Bestehende Monitoring-Targets beibehalten |
| application-gateway | 8220 | `bash system-backend/application-gateway/install/update.sh` | Connector-/Webhook-Konfiguration beibehalten |
| media-library | 8230 | `bash system-backend/media-library/install/update.sh` | Vorher reale `NETCORE_TBS_*`-Werte setzen, siehe unten |
| iot-gateway | 8240 | `INSTALL_LOCAL_MQTT_BROKER=0 bash system-backend/iot-gateway/install/update.sh` | Vorhandenen Broker weiterverwenden |
| hardware-gateway | 8250 | `bash system-backend/hardware-gateway/install/update.sh` | Python-Dienst, vorhandene I/O-Zuordnung prüfen |
| rf-monitor | 8260 | `bash system-backend/rf-monitor/install/update.sh` | Python-Dienst, vorhandene Agenten prüfen |
| alarm-workflow | 8270 | `bash system-backend/alarm-workflow/install/update.sh` | Vorhandene Alarmregeln beibehalten |
| task-workflow | 8280 | `bash system-backend/task-workflow/install/update.sh` | Aufträge/Workflows sichern |
| asset-management | 8290 | `bash system-backend/asset-management/install/update.sh` | Bestands-/Ausgabedaten sichern |
| sip-switch | 8300 | `bash system-backend/sip-switch/install/update.sh` | Nur zentraler SIP-LXC; Asterisk-Konfiguration sichern |
| control-room | 9010 | `bash system-backend/control-room/install/update.sh` | Konfiguration: `/etc/netcore-control-room/control-room.toml` |
| alert-service (künftige Updates) | 8310 | `bash system-backend/alert-service/install/update.sh` | Automatische Sicherung; Empfängerhistorie bleibt erhalten |

Die vorhandenen Rust-Installer benötigen Cargo und die bereits bei der Erstinstallation verwendeten Build-Abhängigkeiten. Im Root-Terminal `cargo --version` prüfen; bei Rustup unter `/root/.cargo` gegebenenfalls `source /root/.cargo/env` laden. Bestehende Build-Features beibehalten. Es ist für die Warnzentrale selbst keine Rust-Installation nötig.

### Sicherung pro bestehendem LXC

Vor dem ersten Update einen Proxmox-Backup/Snapshot nach dem vorhandenen Betriebsverfahren erstellen. Auch externe NFS-Daten, KMF-Schlüssel und eigene Datenbankpfade berücksichtigen; sie liegen möglicherweise außerhalb des Root-Dateisystems.

Zusätzlich für den jeweiligen Dienst lokal sichern. Beispiel für `sds-router`, im Root-Terminal; `SERVICE`, Pfade und Port an den Host anpassen:

```bash
SERVICE=sds-router
UNIT="netcore-${SERVICE}.service"
BACKUP="/var/backups/netcore/${SERVICE}-$(date -u +%Y%m%dT%H%M%SZ)"
install -d -m 0700 "$BACKUP"
git -C /opt/netcore-tetra rev-parse HEAD > "$BACKUP/source-commit.txt"
systemctl cat "$UNIT" > "$BACKUP/unit-effective.txt"
systemctl show "$UNIT" -p ExecStart -p User -p WorkingDirectory
cp -a /etc/netcore "$BACKUP/"
if [ -d /etc/netcore-control-room ]; then cp -a /etc/netcore-control-room "$BACKUP/"; fi
if [ -d /etc/asterisk ]; then cp -a /etc/asterisk "$BACKUP/"; fi
systemctl stop "$UNIT"
if [ -d "/var/lib/netcore-${SERVICE}" ]; then
  cp -a "/var/lib/netcore-${SERVICE}" "$BACKUP/state"
fi
if [ -e "/usr/local/bin/netcore-${SERVICE}" ]; then
  cp -a "/usr/local/bin/netcore-${SERVICE}" "$BACKUP/"
fi
if [ -d "/opt/netcore-${SERVICE}" ]; then
  cp -a "/opt/netcore-${SERVICE}" "$BACKUP/application"
fi
if [ -d "/usr/local/lib/netcore-${SERVICE}" ]; then
  cp -a "/usr/local/lib/netcore-${SERVICE}" "$BACKUP/library"
fi
if [ -d "/usr/local/share/netcore-${SERVICE}" ]; then
  cp -a "/usr/local/share/netcore-${SERVICE}" "$BACKUP/webui"
fi
systemctl start "$UNIT"
```

Den in `ExecStart` tatsächlich verwendeten Programmstand ebenfalls sichern, falls er von diesen Standardpfaden abweicht. Bei gemeinsam verwendeten Datenbanken alle Schreiber stoppen oder das Datenbank-eigene Backup nutzen. Einige vorhandene Update-Skripte stoppen den Dienst bereits vor dem Build; daher Wartungszeit einplanen. Sie können außerdem Bind-Adressen an die aktuelle LXC-IP anpassen: feste DHCP-Lease vorab überprüfen.

Anschließend Updatebefehl aus der Tabelle ausführen und **vor dem nächsten LXC** prüfen:

```bash
systemctl is-active "$UNIT"
journalctl -u "$UNIT" -n 80 --no-pager
curl --fail --show-error --max-time 5 http://<TATSAECHLICHE-LXC-IP>:<PORT>/health/live
curl --fail --show-error --max-time 5 http://<TATSAECHLICHE-LXC-IP>:<PORT>/health/ready
```

Die tatsächliche Bind-IP verwenden; vorhandene Dienste lauschen teilweise nur auf der LXC-IP und nicht auf `127.0.0.1`. Bei Fehlern zuerst korrigieren oder zurücknehmen, bevor abhängige Dienste aktualisiert werden.

### Besonderheiten vorhandener Zusatzdienste

**Media Library:** Der Updatepfad kann bestehende TBS-Playout-Konfiguration migrieren. Die reale TBS setzen, damit keine Beispieladresse eingetragen wird:

```bash
NETCORE_TBS_ID='SRV-M-TBS-01' \
NETCORE_TBS_NAME='SRV-M-TBS-01' \
NETCORE_TBS_URL='http://<TBS-IP>:8080' \
bash system-backend/media-library/install/update.sh
```

Bei geschütztem TBS-Dashboard die bereits eingerichteten Zugangsdaten gemäß Media-Library-Anleitung erhalten. NFS-Mounts und lokale TTS-Verzeichnisse nach dem Update prüfen.

**Piper/TTS:** Läuft üblicherweise auf dem Media-Library-LXC. Für diese Erweiterung genügt die Prüfung `systemctl status netcore-piper` und `curl --fail http://127.0.0.1:5005/voices`. Nur bei beabsichtigtem Piper-Update `bash system-backend/tts/install-piper.sh` ausführen; dies aktualisiert zusätzlich Python-Pakete und lädt/prüft Sprachmodelle. Bestehende Modellpfade und gewählte Stimme vorher sichern. Auf TBS keine zusätzliche Piper-Instanz für Warn-SDS installieren.

**Directory (falls als eigener LXC betrieben):** Das Repository enthält `system-backend/directory/netcore-directory.py`, aber keinen konsistenten universellen Updater; alte Units verwenden unterschiedliche Dateinamen. Mit `systemctl cat netcore-directory` den echten Skript- und Datenbankpfad ermitteln. Dienst stoppen, Skript und Datenbank sichern, dann **nur das Skript** am bestehenden `ExecStart`-Pfad ersetzen:

```bash
# DIRECTORY_SCRIPT auf den gerade in ExecStart gelesenen absoluten .py-Pfad setzen.
DIRECTORY_SCRIPT='/opt/netcore-directory/netcore-directory.py'
test -f "$DIRECTORY_SCRIPT"
systemctl stop netcore-directory
cp -a "$DIRECTORY_SCRIPT" "${DIRECTORY_SCRIPT}.before-alert-update"
install -m 0644 system-backend/directory/netcore-directory.py "$DIRECTORY_SCRIPT"
systemctl start netcore-directory
curl --fail http://<DIRECTORY-IP>:8095/api/devices
```

Keinen Seed importieren, keine Datenbank aus dem Repository kopieren und die historische Unit nicht blind ersetzen. Die Warnzentrale benötigt kein Directory-Update.

**TBS Connect/BREW (falls noch separat betrieben):** `system-backend/tbs-connect/server.py` hat keinen standardisierten Installer und ist für den Warnpfad über Node Gateway nicht erforderlich. Den tatsächlichen Dienstnamen und Python-/Virtualenv-/Skriptpfad über die vorhandene Unit ermitteln, Skript und `nodes.json` sichern, Dienst stoppen, `server.py` an dessen bestehenden Pfad kopieren und wieder starten. Python-Abhängigkeiten, `BREW_*`-Umgebung und Session-Key erhalten. WebUI auf bestehendem Port (Standard `8081`) sowie Verbindungen prüfen. Keine neue parallele BREW-Instanz starten.

**Mosquitto, Monitoring-Stack und Log-Forwarder:** Bestehende Installation beibehalten; für diese Funktion ist kein Paketupgrade nötig. Bereits installierte `netcore-observability-journal-forwarder` können nach gesicherter Unit mit `system-backend/observability/agents/journal_forwarder.py` am vorhandenen `ExecStart`-Pfad aktualisiert und neu gestartet werden; Ziel-URL und Service-Name in der Umgebung erhalten. `shared/` ist eine Bibliothek und kein LXC.

## 5. Jede TBS aktualisieren

TBS nacheinander bearbeiten. Zuerst Quellen aus Abschnitt 3 im vorhandenen TBS-Checkout aktualisieren und `/etc/netcore/config.toml`, aktive Unit und eventuell `/etc/asterisk` sichern. Den Service-Namen und tatsächlichen Programmpfad vorher ermitteln:

```bash
systemctl list-units --type=service --all | grep -E 'tetra|bluestation'
systemctl cat tetra.service
systemctl show tetra.service -p ExecStart -p User
```

`tetra.service` in allen folgenden Befehlen durch die tatsächlich verwendete Unit ersetzen, z. B. `bluestation.service` oder `bluestation-bs.service`.

```bash
cd /home/jan/netcore-tetra
sudo cp -a /etc/netcore/config.toml /etc/netcore/config.toml.before-alert-update
# Beispiel: vorhandene Features und Build-Benutzer an die Installation anpassen.
sudo env BUILD_USER=jan UNIT=tetra.service \
  CONFIG_PATH=/etc/netcore/config.toml \
  CARGO_FEATURES='asterisk,recording,audio-player' \
  DISABLE_LOCAL_PIPER=0 MIGRATE_LOCAL_TTS_CONFIG=0 \
  bash install/update-basisstation.sh
```

Die beiden letzten Optionen verhindern bei diesem Warnungsupdate eine zusätzliche Migration der bestehenden TTS-Konfiguration. Der Updater baut vor dem Austausch, findet das tatsächlich von systemd gestartete Programm, sichert die alte Binary unter `/var/backups/netcore-tetra/` und nimmt den Austausch bei erkanntem Startfehler automatisch zurück.

In `/etc/netcore/config.toml` die vorhandene Anbindung kontrollieren; **den bestehenden Abschnitt bearbeiten, nicht doppelt anhängen**:

```toml
[control_room]
enabled = true
host = "<NODE-GATEWAY-IP>"
port = 8080
use_tls = false
endpoint_path = "/ws/node"
node_id = "SRV-M-TBS-01"
station_name = "SRV-M-TBS-01"
central_sds_routing = true
```

Die Station muss ihre Geräte-/GPS-Daten an die bestehende Zentrale liefern. Eine Warnservice-URL ist auf der TBS nicht erforderlich. Anschließend:

```bash
sudo systemctl restart tetra.service
systemctl is-active tetra.service
journalctl -u tetra.service -n 100 --no-pager
```

Mit einem Testfunkgerät anmelden, aktuelle GPS-Position übertragen und im Control Room den passenden Online-Eintrag prüfen. Danach eine gewöhnliche Unicast-SDS über den vorhandenen SDS Router testen, bevor die Warnzentrale aktiviert wird.

Bereits installierte TBS-Zusatzdienste bei Bedarf aus demselben Checkout aktualisieren:

```bash
# Nur falls der RF-Agent auf dieser TBS bereits verwendet wird:
sudo bash system-backend/rf-monitor/install/install-tbs-agent.sh \
  'http://127.0.0.1:8080' 'http://<RF-MONITOR-IP>:8260' 'SRV-M-TBS-01'
sudo systemctl restart netcore-rf-agent

# Nur falls lokaler SIP-Fallback bereits eingerichtet ist:
sudo bash system-backend/sip-switch/install/update-tbs-local-fallback.sh
systemctl status netcore-tbs-sip-failover --no-pager
```

Das RF-Skript erhält eine vorhandene `/etc/netcore/rf-agent.toml`; bei bereits laufendem Agenten ist der ausdrückliche Neustart oben wichtig. Der SIP-Updater benötigt `/etc/netcore/tbs-sip-fallback.toml` und erhält den konfigurierten Fallback. Den **zentralen** SIP-Switch-Installer niemals auf der TBS ausführen. Nachher Gruppengespräch, Einzelruf, SDS und gegebenenfalls zentralen SIP-Weg/Fallback prüfen.

## 6. Neuen Warn-LXC erstellen

In Proxmox einen freien CT mit Hostname `alert-service` anlegen: unprivilegierter Debian-13-Container, als Startgröße 1 vCPU, 1 GiB RAM, 256 MiB Swap und 8 GiB Disk. Ein gewöhnlicher Management-Netzanschluss reicht; kein TUN, USB-/SDR-Passthrough, Nesting oder NFS erforderlich. Autostart aktivieren. Bei DHCP wie bei den vorhandenen LXCs eine feste Lease und einen DNS-Eintrag anlegen.

Die Startressourcen sind Richtwerte; die dauerhaft aufbewahrte Zustellhistorie benötigt mit zunehmender Geräte-/Warnungszahl mehr Speicher. Für eine bereinigte Neuinstallation ist Debian 12 ebenfalls geeignet, sofern `python3 --version` mindestens 3.11 ausgibt.

Im neuen Container als root:

```bash
apt-get update
apt-get install -y --no-install-recommends git python3 ca-certificates curl nano
timedatectl set-timezone Europe/Berlin
python3 --version
NETCORE_REF=feat/katwarn-nina-alerts
git clone --branch "$NETCORE_REF" --single-branch \
  https://github.com/JanHG98/netcore-tetra.git /opt/netcore-tetra
cd /opt/netcore-tetra
bash system-backend/alert-service/install/install.sh
```

Der Installer legt Folgendes an:

| Pfad | Inhalt |
|---|---|
| `/usr/local/lib/netcore-alert-service/` | Programm und Karten-WebUI |
| `/etc/netcore/alert-service.toml` | Eigene Einstellungen |
| `/etc/netcore/alert-service.env` | Lokal erzeugtes API-Token; optional Control-Room-Passwort |
| `/var/lib/netcore-alert-service/alerts.sqlite3` | Meldungen und dauerhafte Empfängerhistorie |
| `/etc/systemd/system/netcore-alert-service.service` | systemd-Dienst als `netcore-alert` |
| `/var/backups/netcore-alert-service/<Zeitstempel>/` | Update-Sicherungen |

Netzwerkfreigaben:

| Richtung | Freigabe |
|---|---|
| Administrator/Browser → Warn-LXC | TCP 8310 |
| Warn-LXC → Control Room | TCP 9010 an dessen tatsächlicher Adresse |
| Warn-LXC → SDS Router | TCP 8150 |
| Warn-LXC → Internet | DNS und HTTPS 443 zu `warnung.bund.de`; Paket-/Git-Zugriff während Installation |
| Browser → Kartenanbieter | HTTPS für die im WebUI verwendeten Hintergrundkarten |

## 7. Warnzentrale konfigurieren

```bash
nano /etc/netcore/alert-service.toml
```

Mindestens diese **bestehenden** Werte setzen:

```toml
[netcore]
control_room_url = "http://10.0.20.25:9010"
sds_router_url = "http://<SDS-ROUTER-IP>:8150"
control_room_username = ""
control_room_password = ""
source_issi = 9999
poll_seconds = 5
gps_max_age_seconds = 3600
node_max_age_seconds = 120
http_timeout_seconds = 10

[delivery]
enabled = false
ttl_seconds = 300
max_text_length = 120
```

Für `source_issi` eine im eigenen Netz zulässige, freie Dienst-ISSI wählen. Die `9999` ist nur der Beispielwert. Die NINA-URL und den Standard-Datenbankpfad normalerweise beibehalten. Bei verändertem Datenbankpfad zusätzlich Besitz/Rechte und `ReadWritePaths` der Unit anpassen; andernfalls verhindert die systemd-Abschirmung Schreibzugriffe.

Bei einem Control Room mit Anmeldung `control_room_username` setzen und dessen Passwort in `/etc/netcore/alert-service.env` als `NETCORE_CONTROL_ROOM_PASSWORD=...` hinterlegen. Keine Zugangsdaten in Git oder Inventardateien speichern. Die Datei ist eine systemd-Environment-Datei; Werte mit Leerzeichen korrekt in Anführungszeichen setzen. Das automatisch erzeugte `NETCORE_ALERT_TOKEN` erhalten.

```bash
chown root:netcore-alert /etc/netcore/alert-service.toml /etc/netcore/alert-service.env
chmod 0640 /etc/netcore/alert-service.toml /etc/netcore/alert-service.env
systemctl restart netcore-alert-service
systemctl is-active netcore-alert-service
curl --fail http://127.0.0.1:8310/health/live
journalctl -u netcore-alert-service -n 80 --no-pager
```

Die WebUI unter `http://<WARN-LXC-IP>:8310/` öffnen. Den Wert aus `NETCORE_ALERT_TOKEN` lokal aus `/etc/netcore/alert-service.env` in das Tokenfeld übernehmen. Der Installer schreibt dieses Geheimnis nicht in seine Ausgabe. Token nicht als URL-Parameter weitergeben. `allow_unauthenticated` auf `false` belassen.

`GET /health/live` prüft den Prozess; `GET /health/ready` meldet bei noch fehlendem Erstabgleich oder abhängigen Fehlern HTTP 503. Vor der Freigabe muss auch die Bereitschaft stimmen. Die authentifizierten APIs sind `GET /api/v1/status`, `/api/v1/alerts`, `/api/v1/devices` und `/api/v1/deliveries`; alle erwarten `Authorization: Bearer <Token>`. `POST /api/v1/alerts` erstellt eigene Warnungen; `DELETE /api/v1/alerts/<URL-kodierte-ID>` zieht sie zurück. Die WebUI benutzt dieselben Schnittstellen.

Bei NINA-Abruffehlern bleibt der letzte Datenstand sichtbar. Wenn der letzte erfolgreiche vollständige Abruf älter als `[nina].max_stale_seconds` ist (Standard 300 Sekunden), werden daraus keine neuen Funkmeldungen erzeugt. Der Fehler bleibt im Status sichtbar. Eigene Warnungen hängen nicht von diesem Feed ab.

## 8. Abnahme und Freigabe des SDS-Versands

Zuerst mit deaktiviertem Versand prüfen:

1. WebUI und Anmeldung funktionieren; ein falsches Token erhält keinen API-Zugriff.
2. NINA-Abgleich zeigt entweder aktive Warnungen oder einen erfolgreichen Abruf ohne Warnungen. Fehler/Timeouts dürfen nicht als erfolgreicher leerer Abruf bewertet werden.
3. Das Testgerät erscheint online mit plausibler, aktueller GPS-Position; ein zweites Testgerät außerhalb des geplanten Bereichs vorbereiten.
4. Auf der Karte eine eigene kurze Testwarnung mit eindeutigem Text, passendem Mittelpunkt, z. B. 1 km Radius und kurzer Gültigkeit anlegen. Bei weiterhin deaktiviertem Versand darf keine SDS erscheinen.

Danach in `[delivery]` `enabled = true` setzen und `systemctl restart netcore-alert-service` ausführen. Innerhalb von Geräteabgleich/Netzlaufzeit muss das Gerät im Gebiet die SDS erhalten; das Gerät außerhalb darf sie nicht erhalten.

Die folgenden Fälle gehören zur Abnahme:

| Test | Erwartung |
|---|---|
| Neue Warnung bei bereits angemeldetem Gerät im Gebiet | Einmalige SDS |
| Gerät erst nach Erstellung der Warnung anmelden | Einmalige SDS nach Online-/GPS-Abgleich |
| Mehrere Abfragen derselben Warnung | Kein erneuter Versand |
| Gerät ab-/anmelden oder Bereich verlassen/wieder betreten | Bereits versendete Warnung kommt nicht erneut |
| Warnservice neu starten | Historie bleibt erhalten, keine Wiederholung |
| Zweites Gerät mit anderer ISSI im Gebiet anmelden | Eigene einmalige SDS |
| Fehlende/veraltete GPS-Position | Kein Versand anhand unzuverlässiger Position |
| Eigene Warnung löschen oder ablaufen lassen | Kein Versand an später hinzukommende Geräte |
| Control Room, NINA oder SDS Router vorübergehend nicht erreichbar | Fehler sichtbar; Runtime/WebUI bleibt bedienbar |

Eine bereits auf einem Funkgerät eingegangene SDS kann durch Löschen der Warnung nicht vom Funkgerät entfernt werden. Eine vollständig neue selbst erstellte Warnung besitzt eine neue Identität und darf dieselben Geräte erneut informieren.

Nach dem Test die Testwarnung löschen und echte aktive Warnungen in WebUI und Zustellhistorie kontrollieren. Vor Freigabe prüfen, dass keine Testmeldung mit zu großem Radius aktiv geblieben ist.

## 9. Spätere Updates und Sicherung der Warnzentrale

```bash
cd /opt/netcore-tetra
git status --short
NETCORE_REF=feat/katwarn-nina-alerts
git fetch origin "$NETCORE_REF:refs/remotes/origin/$NETCORE_REF"
git switch "$NETCORE_REF"
git pull --ff-only origin "$NETCORE_REF"
bash system-backend/alert-service/install/update.sh
systemctl is-active netcore-alert-service
curl --fail http://127.0.0.1:8310/health/live
journalctl -u netcore-alert-service -n 80 --no-pager
```

Der Updater prüft Python/Syntax vor der Unterbrechung, sichert Programm, Unit, Konfiguration, Geheimnisse sowie eine konsistente SQLite-Kopie und startet den Dienst neu. **Die vorhandene Konfiguration, das Token und die aktive Datenbank werden nicht durch Beispiele ersetzt.** Bei erkanntem Start-/Health-Fehler wird der vorherige Programmstand samt Unit wiederhergestellt. Die Datenbank wird dabei bewusst nicht auf einen älteren Zustellstand zurückgedreht.

Regelmäßig zusätzlich konsistente Sicherungen auf das vorhandene Backupziel übernehmen. Eine laufende SQLite-Datenbank nicht allein mit `cp alerts.sqlite3` sichern: Änderungen können noch in WAL-Dateien liegen. Beispielsweise als root:

```bash
BACKUP="/var/backups/netcore-alert-service/manual-$(date -u +%Y%m%dT%H%M%SZ)"
install -d -m 0700 "$BACKUP"
cp -a /etc/netcore/alert-service.toml /etc/netcore/alert-service.env "$BACKUP/"
python3 - "$BACKUP/alerts.sqlite3" <<'PY'
import sqlite3, sys, tomllib
with open('/etc/netcore/alert-service.toml', 'rb') as stream:
    database = tomllib.load(stream)['storage']['database']
with sqlite3.connect(database) as source, sqlite3.connect(sys.argv[1]) as target:
    source.backup(target)
PY
chmod 0600 "$BACKUP/alerts.sqlite3"
```

Diese Datenbank ist Teil des dauerhaften Betriebszustands. Eine leere oder ältere Datenbank verliert Zustellinformationen und kann erneute Warnungen auslösen. Keine zweite aktive Warnzentrale mit einer unabhängigen Kopie derselben Datenbank parallel auf dieselben Funkgeräte senden lassen.

## 10. Rücknahme

**Nur Warnversand stoppen:** In `/etc/netcore/alert-service.toml` `delivery.enabled = false` setzen und Dienst neu starten. Bereits vom SDS Router angenommene Nachrichten können noch zugestellt werden. Soll die gesamte Warnzentrale stoppen: `systemctl disable --now netcore-alert-service`. Die Datenbank und Konfiguration bleiben erhalten.

**Vorherigen Warnservice wiederherstellen:** Den gewünschten Sicherungsordner aus `/var/backups/netcore-alert-service/` einsetzen. Die derzeitige Datenbank behalten, sofern die vorherige Version damit kompatibel ist. Bei einer späteren inkompatiblen Datenmigration zuerst nach der betreffenden Versionsanleitung vorgehen.

```bash
BACKUP='/var/backups/netcore-alert-service/<ZEITSTEMPEL>'
test -f "$BACKUP/app/main.py"
test -f "$BACKUP/netcore-alert-service.service"
systemctl stop netcore-alert-service
mv /usr/local/lib/netcore-alert-service \
  "/usr/local/lib/netcore-alert-service.failed-$(date -u +%Y%m%dT%H%M%SZ)"
cp -a "$BACKUP/app" /usr/local/lib/netcore-alert-service
cp -a "$BACKUP/netcore-alert-service.service" /etc/systemd/system/
systemctl daemon-reload
systemctl start netcore-alert-service
curl --fail http://127.0.0.1:8310/health/live
```

**Bestehender LXC:** Den gesicherten tatsächlichen Programmstand und bei Bedarf dessen Unit zurückspielen, aktuelle Konfiguration mit der gesicherten vergleichen, `systemctl daemon-reload` und Dienststart ausführen. Für Rust-Dienste kann alternativ der notierte alte Commit in einem separaten Checkout gebaut werden. Keine alte Datenbank ungeprüft über neue Nutzdaten kopieren. Bei Proxmox-Restore zuerst die vorhandenen Zustell-/Queue-Daten prüfen; insbesondere die Warnzentrale während einer Rücknahme ihrer Abhängigkeiten deaktivieren.

**TBS:** Falls der automatische Rollback nicht bereits gegriffen hat, Unit stoppen, die passende Datei `bluestation-bs.<Zeitstempel>.bak` aus `/var/backups/netcore-tetra/` an den zuvor ermittelten tatsächlichen `ExecStart`-Binärpfad installieren (`install -m 0755 ... ...`), gegebenenfalls die gesicherte Konfiguration wiederherstellen und Unit starten. Anschließend Anmeldung, GPS, SDS und Sprechverbindung erneut testen.

## 11. Inventar und Fehlersuche

Der Beispielkatalog `system-backend/services.toml` und `deploy/open-lab/inventory.example.toml` enthalten `alert-service` auf Port 8310. Im eigenen Inventar die reale Adresse ergänzen. Offline validieren:

```bash
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml validate
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml plan
```

Der allgemeine ältere `apply`-Befehl kopiert gerenderte Beispielkonfigurationen auf die Hosts. Für die hier beschriebenen **bestandserhaltenden Updates die manuellen Updatepfade verwenden**; insbesondere `delivery.enabled`, Zugangsdaten und eigene Dienstadressen nicht durch einen pauschalen Apply überschreiben.

| Symptom | Prüfung |
|---|---|
| Dienst startet nicht | `journalctl -u netcore-alert-service -n 100 --no-pager`; Python ≥3.11, gültiges TOML, Tokendatei, Datenbankrechte |
| WebUI erreichbar, API meldet 401 | Richtiges `NETCORE_ALERT_TOKEN`; Token nach Rotation erneut im WebUI eingeben |
| Keine NINA-Meldungen | Abrufstatus, DNS, HTTPS, Uhrzeit, Internetzugang; ein leerer erfolgreicher Abruf ist möglich |
| Keine Geräte/Positionen | Reale Control-Room-Adresse, Anmeldedaten, Online-Status und GPS-Übertragung der TBS |
| Gerät liegt im Gebiet, keine SDS | Versandfreigabe, Warnungsgültigkeit, GPS-Alter, bestehende Zustellhistorie, Source-ISSI, SDS-Router-/TBS-Route |
| Doppelte Meldungen | Parallele Warninstanzen, gelöschte/zurückgespielte Datenbank oder neu angelegte Warn-ID prüfen |
| Karte ohne Hintergrund | Browserzugriff auf Kartenanbieter und Internet prüfen; Meldungsverwaltung/Koordinaten weiterhin prüfen |

Ein erfolgreicher lokaler Softwaretest ersetzt die Abnahme auf den eigenen LXCs und echten TBS/Funkgeräten nicht. Dieses Dokument liefert dafür die Schritte; es behauptet keine bereits auf den Zielhosts durchgeführte Installation.
