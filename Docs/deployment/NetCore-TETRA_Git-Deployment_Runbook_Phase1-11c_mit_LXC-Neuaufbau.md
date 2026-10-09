# NetCore-TETRA – vollständiges Git-Deployment-Runbook inklusive neuer Proxmox-LXCs und TBS

Stand: Phase 1–11c, Branch `mqtt`, OPEN LAB

> Dieses Runbook arbeitet ausschließlich mit `git clone` und `git pull`. Es kopiert kein ZIP auf die Systeme.

## 0. Was neu angelegt werden muss

Die elf Entwicklungsphasen bedeuten **nicht elf neue Container**. Mehrere Phasen erweitern bereits vorhandene Dienste.

### Bereits vorhandene LXCs aktualisieren

Diese Dienste sollten in deiner bestehenden Backend-Landschaft bereits als eigene LXCs vorhanden sein und werden später per `git pull` plus `install/update.sh` aktualisiert:

```text
node-gateway
mobility-core
subscriber-core
group-core
call-control
media-switch
recorder
sds-router
packet-core
ip-gateway
security-core
kmf
transit
application-gateway
media-library
control-room
observability
```

### Genau sieben neue LXCs anlegen

| Neuer LXC | Enthaltene Phasen/Funktion | Management-Port | Weitere Ports |
|---|---|---:|---|
| `iot-gateway` | Phasen 3–5: MQTT, Eventbridge, Command/Ack/Policy, Home Assistant/Homematic | `8240/TCP` | `1883/TCP` für Mosquitto |
| `hardware-gateway` | Phase 6: Hardware-I/O- und Rack-Monitoring | `8250/TCP` | keine zusätzlichen Listener |
| `rf-monitor` | Phase 7: RF- und Senderüberwachung | `8260/TCP` | keine zusätzlichen Listener |
| `alarm-workflow` | Phase 8: Alarm-, SDS- und Statusworkflows | `8270/TCP` | keine zusätzlichen Listener |
| `task-workflow` | Phase 9: WAP-Formulare und strukturierte Aufträge | `8280/TCP` | keine zusätzlichen Listener |
| `asset-management` | Phase 10: Assets, Funkgeräte, Benutzer und Wartung | `8290/TCP` | keine zusätzlichen Listener |
| `sip-switch` | Phase 11c: zentrale SIP-Vermittlung | `8300/TCP` | `5060/UDP`, `10000–20000/UDP` |

Keinen eigenen LXC benötigen:

- Phase 1 und 2: Änderungen in vorhandenen Core-Diensten und gemeinsamer Ereignisvertrag;
- Phase 4 und 5: laufen innerhalb des `iot-gateway`;
- Mosquitto: läuft zunächst auf dem `iot-gateway`;
- lokaler SIP-Fallback: läuft auf jeder TBS selbst, nicht in einem weiteren LXC.

## 0.1 Empfohlene Startressourcen

Das sind pragmatische Startwerte für dein OPEN LAB. Sie lassen sich später ohne Neuinstallation erhöhen.

| Hostname | vCPU | RAM | Swap | Root-Disk | Autostart |
|---|---:|---:|---:|---:|---|
| `iot-gateway` | 2 | 2048 MiB | 512 MiB | 12 GiB | ja |
| `hardware-gateway` | 1 | 1024 MiB | 256 MiB | 8 GiB | ja |
| `rf-monitor` | 1 | 1024 MiB | 256 MiB | 8 GiB | ja |
| `alarm-workflow` | 1 | 1024 MiB | 256 MiB | 8 GiB | ja |
| `task-workflow` | 1 | 1024 MiB | 256 MiB | 8 GiB | ja |
| `asset-management` | 1 | 1536 MiB | 512 MiB | 12 GiB | ja |
| `sip-switch` | 2 | 2048 MiB | 512 MiB | 16 GiB | ja |

Alle sieben können als **unprivilegierte Debian-13-LXCs** laufen. Für diese sieben Dienste werden weder `/dev/net/tun` noch ein NFS-Mount noch USB-/SDR-Passthrough benötigt.

## 0.2 Proxmox: Debian-13-Template bereitstellen

Auf dem Proxmox-Host:

```bash
pveam update
pveam available | grep 'debian-13-standard'
```

Das aktuell angebotene Template laden. Den tatsächlich angezeigten Dateinamen einsetzen:

```bash
pveam download local debian-13-standard_<VERSION>_amd64.tar.zst
```

Kontrolle:

```bash
pveam list local | grep debian-13
```

## 0.3 Jeden neuen LXC in der Proxmox-WebUI anlegen

Für jeden der sieben Einträge nacheinander **Create CT** ausführen:

1. **General**
   - freie CT-ID wählen;
   - Hostname exakt wie in der Tabelle, nur Kleinbuchstaben und Bindestriche;
   - `Unprivileged container` aktivieren;
   - starkes temporäres Root-Kennwort oder SSH-Key setzen.
2. **Template**
   - Debian 13 Standard auswählen.
3. **Disks**
   - Größe aus der Ressourcentabelle;
   - dein normales LXC-Storage verwenden.
4. **CPU**
   - vCPU aus der Tabelle.
5. **Memory**
   - RAM und Swap aus der Tabelle.
6. **Network**
   - Bridge deines Server-/Management-Netzes auswählen;
   - bei VLAN-Nutzung den korrekten VLAN-Tag setzen;
   - IPv4 zunächst auf `DHCP`;
   - Firewall nur aktivieren, wenn du die weiter unten genannten Regeln direkt pflegst.
7. **DNS**
   - deinen internen DNS eintragen oder `Use host settings` verwenden.
8. **Confirm**
   - `Start after created` aktivieren.

Danach in **Options** jedes Containers setzen:

```text
Start at boot: Yes
Start/Shutdown order: nach deiner Infrastruktur, SIP-Switch und IoT-Gateway vor den davon abhängigen Workflows
```

Nesting, FUSE, Keyctl oder privilegierter Betrieb sind für diese sieben LXCs nicht erforderlich.

## 0.4 Optional: Erstellung per Proxmox-CLI

Zuerst deine Werte eintragen:

```bash
TEMPLATE='local:vztmpl/debian-13-standard_<VERSION>_amd64.tar.zst'
STORAGE='local-lvm'
BRIDGE='vmbr0'
SSH_KEY_FILE='/root/.ssh/authorized_keys'
```

Freie CT-IDs festlegen; die Zahlen sind nur ein Vorschlag:

```bash
pct create 230 "$TEMPLATE" --hostname iot-gateway      --unprivileged 1 --cores 2 --memory 2048 --swap 512 --rootfs "$STORAGE":12 --net0 name=eth0,bridge="$BRIDGE",ip=dhcp,type=veth --onboot 1 --ssh-public-keys "$SSH_KEY_FILE"
pct create 231 "$TEMPLATE" --hostname hardware-gateway --unprivileged 1 --cores 1 --memory 1024 --swap 256 --rootfs "$STORAGE":8  --net0 name=eth0,bridge="$BRIDGE",ip=dhcp,type=veth --onboot 1 --ssh-public-keys "$SSH_KEY_FILE"
pct create 232 "$TEMPLATE" --hostname rf-monitor        --unprivileged 1 --cores 1 --memory 1024 --swap 256 --rootfs "$STORAGE":8  --net0 name=eth0,bridge="$BRIDGE",ip=dhcp,type=veth --onboot 1 --ssh-public-keys "$SSH_KEY_FILE"
pct create 233 "$TEMPLATE" --hostname alarm-workflow    --unprivileged 1 --cores 1 --memory 1024 --swap 256 --rootfs "$STORAGE":8  --net0 name=eth0,bridge="$BRIDGE",ip=dhcp,type=veth --onboot 1 --ssh-public-keys "$SSH_KEY_FILE"
pct create 234 "$TEMPLATE" --hostname task-workflow     --unprivileged 1 --cores 1 --memory 1024 --swap 256 --rootfs "$STORAGE":8  --net0 name=eth0,bridge="$BRIDGE",ip=dhcp,type=veth --onboot 1 --ssh-public-keys "$SSH_KEY_FILE"
pct create 235 "$TEMPLATE" --hostname asset-management  --unprivileged 1 --cores 1 --memory 1536 --swap 512 --rootfs "$STORAGE":12 --net0 name=eth0,bridge="$BRIDGE",ip=dhcp,type=veth --onboot 1 --ssh-public-keys "$SSH_KEY_FILE"
pct create 236 "$TEMPLATE" --hostname sip-switch        --unprivileged 1 --cores 2 --memory 2048 --swap 512 --rootfs "$STORAGE":16 --net0 name=eth0,bridge="$BRIDGE",ip=dhcp,type=veth --onboot 1 --ssh-public-keys "$SSH_KEY_FILE"
```

Container starten:

```bash
for CT in 230 231 232 233 234 235 236; do
  pct start "$CT"
done
```

## 0.5 Feste DHCP-Leases in OPNsense anlegen

Da du die LXCs per DHCP mit statischen Leases betreibst:

1. In Proxmox beim jeweiligen Container unter **Hardware → Network Device** die MAC-Adresse notieren.
2. In OPNsense zu **Services → DHCPv4 → dein Server-LAN** gehen.
3. Für jede MAC eine feste Lease anlegen.
4. Hostnamen und gewünschte IP eintragen.
5. Änderungen anwenden.
6. Container neu starten oder Lease erneuern.

Auf dem Proxmox-Host:

```bash
pct reboot 230
pct reboot 231
pct reboot 232
pct reboot 233
pct reboot 234
pct reboot 235
pct reboot 236
```

Auf jedem LXC kontrollieren:

```bash
hostname
hostname -I
ip -4 addr show dev eth0
ip route
getent hosts github.com
```

Erst wenn alle sieben ihre endgültige Adresse erhalten haben, die IP-Tabelle in Abschnitt 1 ausfüllen.

## 0.6 Basisinstallation auf jedem neuen LXC

Auf jedem der sieben neuen Container als `root`:

```bash
apt-get update
DEBIAN_FRONTEND=noninteractive apt-get full-upgrade -y
apt-get install -y --no-install-recommends \
  git ca-certificates curl wget nano vim-tiny \
  python3 python3-venv python3-pip \
  jq unzip procps iproute2 iputils-ping dnsutils \
  build-essential pkg-config cmake clang libssl-dev

timedatectl set-timezone Europe/Berlin
systemctl enable systemd-timesyncd --now
```

Repository direkt von GitHub klonen:

```bash
git clone \
  --branch mqtt \
  --single-branch \
  https://github.com/JanHG98/netcore-tetra.git \
  /opt/netcore-tetra

cd /opt/netcore-tetra
git branch --show-current
git log -1 --oneline
```

Erwartet:

```text
mqtt
```

Installationsskripte ausführbar setzen und vorab auf Shell-Syntax prüfen:

```bash
cd /opt/netcore-tetra
find system-backend -path '*/install/*.sh' -type f -exec chmod 755 {} +

FAILED=0
while IFS= read -r FILE; do
  bash -n "$FILE" || FAILED=1
done < <(find system-backend -path '*/install/*.sh' -type f -print)

exit "$FAILED"
```

Falls diese Prüfung einen Fehler meldet, den betreffenden Installer **vor** der Installation im Git-Branch korrigieren und danach erneut `git pull` ausführen. Keine dauerhaften Hotfixes direkt im LXC-Checkout hinterlassen.

## 0.7 Firewall-Regeln für die neuen LXCs

Innerhalb des isolierten OPEN-LAB-Netzes mindestens erlauben:

| Ziel-LXC | Eingehend erlauben |
|---|---|
| `iot-gateway` | `8240/TCP`, `1883/TCP` |
| `hardware-gateway` | `8250/TCP` |
| `rf-monitor` | `8260/TCP` |
| `alarm-workflow` | `8270/TCP` |
| `task-workflow` | `8280/TCP` |
| `asset-management` | `8290/TCP` |
| `sip-switch` | `8300/TCP`, `5060/UDP`, `10000–20000/UDP` |

Zusätzlich müssen die Container ausgehend ihre in der Konfiguration genannten Abhängigkeiten erreichen. Zwischen PBX, SIP-Switch und TBS möglichst kein NAT einsetzen.

## 0.8 Tatsächliche Installationsreihenfolge

Nach dem Erstellen der Container nicht einfach alphabetisch installieren. Diese Reihenfolge verwenden:

```text
A. Vorhandene Core-LXCs aktualisieren
   node-gateway → mobility-core/subscriber-core → group-core → call-control
   → media-switch/recorder → sds-router → packet-core/ip-gateway
   → security-core/kmf/transit → application-gateway/media-library

B. Neuen MQTT-Unterbau installieren
   iot-gateway inklusive Mosquitto

C. Neue Mess- und Workflow-LXCs installieren
   hardware-gateway → rf-monitor → alarm-workflow → task-workflow
   → asset-management

D. Zentrale Telefonie installieren
   sip-switch

E. Bestehende Übersichten aktualisieren
   control-room → observability → node-gateway nochmals prüfen

F. Ganz zuletzt jede TBS aktualisieren
   Basisstations-Binary → RF-Agent → lokaler Asterisk/Fallback → End-to-End-Test
```

## 1. Vor dem Start: IP-Plan ausfüllen

Die Adressen aus `deploy/open-lab/inventory.example.toml` sind Beispiele. Trage deine realen LXC-Adressen in diese Liste ein und verwende sie in den späteren Befehlen:

```bash
NODE_GATEWAY_IP=...
MOBILITY_CORE_IP=...
SUBSCRIBER_CORE_IP=...
GROUP_CORE_IP=...
CALL_CONTROL_IP=...
MEDIA_SWITCH_IP=...
RECORDER_IP=...
SDS_ROUTER_IP=...
PACKET_CORE_IP=...
IP_GATEWAY_IP=...
SECURITY_CORE_IP=...
KMF_IP=...
TRANSIT_IP=...
APPLICATION_GATEWAY_IP=...
MEDIA_LIBRARY_IP=...
CONTROL_ROOM_IP=...
OBSERVABILITY_IP=...
IOT_GATEWAY_IP=...
HARDWARE_GATEWAY_IP=...
RF_MONITOR_IP=...
ALARM_WORKFLOW_IP=...
TASK_WORKFLOW_IP=...
ASSET_MANAGEMENT_IP=...
SIP_SWITCH_IP=...
PBX_IP=...
TBS_01_IP=...
DEFAULT_GSSI=15201
```

## 2. Soll-Struktur und Ports

| Dienst | Port | systemd-Unit | Inventar-Beispiel |
|---|---:|---|---|
| `node-gateway` | `8080` | `netcore-node-gateway.service` | `10.0.20.10` (Beispiel) |
| `mobility-core` | `8090` | `netcore-mobility-core.service` | `10.0.20.11` (Beispiel) |
| `subscriber-core` | `8100` | `netcore-subscriber-core.service` | `10.0.20.12` (Beispiel) |
| `group-core` | `8110` | `netcore-group-core.service` | `10.0.20.13` (Beispiel) |
| `call-control` | `8120` | `netcore-call-control.service` | `10.0.20.14` (Beispiel) |
| `media-switch` | `8130` | `netcore-media-switch.service` | `10.0.20.15` (Beispiel) |
| `recorder` | `8140` | `netcore-recorder.service` | `10.0.20.16` (Beispiel) |
| `sds-router` | `8150` | `netcore-sds-router.service` | `10.0.20.17` (Beispiel) |
| `packet-core` | `8160` | `netcore-packet-core.service` | `10.0.20.18` (Beispiel) |
| `ip-gateway` | `8170` | `netcore-ip-gateway.service` | `10.0.20.19` (Beispiel) |
| `security-core` | `8180` | `netcore-security-core.service` | `10.0.20.20` (Beispiel) |
| `kmf` | `8190` | `netcore-kmf.service` | `10.0.20.21` (Beispiel) |
| `transit` | `8200` | `netcore-transit.service` | `10.0.20.22` (Beispiel) |
| `application-gateway` | `8220` | `netcore-application-gateway.service` | `10.0.20.23` (Beispiel) |
| `media-library` | `8230` | `netcore-media-library.service` | `10.0.20.24` (Beispiel) |
| `control-room` | `9010` | `netcore-control-room.service` | `10.0.20.25` (Beispiel) |
| `observability` | `8210` | `netcore-observability.service` | `10.0.20.26` (Beispiel) |
| `iot-gateway` | `8240` | `netcore-iot-gateway.service` | `10.0.20.27` (Beispiel) |
| `hardware-gateway` | `8250` | `netcore-hardware-gateway.service` | `10.0.20.28` (Beispiel) |
| `rf-monitor` | `8260` | `netcore-rf-monitor.service` | `10.0.20.29` (Beispiel) |
| `alarm-workflow` | `8270` | `netcore-alarm-workflow.service` | `10.0.20.30` (Beispiel) |
| `task-workflow` | `8280` | `netcore-task-workflow.service` | `10.0.20.31` (Beispiel) |
| `asset-management` | `8290` | `netcore-asset-management.service` | `10.0.20.32` (Beispiel) |
| `sip-switch` | `8300` | `netcore-sip-switch.service` | `10.0.20.33` (Beispiel) |

Nicht als eigene Runtime-LXCs installieren: `shared`, `directory`, `tbs-connect`, `tts` und `provisioning-core`, sofern du sie nicht bewusst separat testen möchtest. Das offizielle Open-Lab-Inventar enthält 24 Runtime-Dienste.

## 3. Einheitliche Git-Vorbereitung auf jedem LXC

### Neuer LXC

```bash
apt-get update
apt-get install -y git ca-certificates curl python3 build-essential pkg-config cmake clang libssl-dev

git clone --branch mqtt --single-branch   https://github.com/JanHG98/netcore-tetra.git   /opt/netcore-tetra

cd /opt/netcore-tetra
git branch --show-current
git log -1 --oneline
```

Erwartet wird als Branch:

```text
mqtt
```

### Bereits vorhandener LXC

```bash
cd /opt/netcore-tetra

git status --short
```

Ist die Ausgabe **nicht leer**, zuerst prüfen und die lokalen Änderungen sichern. Nicht blind `reset --hard` ausführen.

Danach:

```bash
git fetch origin
git switch mqtt
git pull --ff-only origin mqtt
git log -1 --oneline
```

Bei `detected dubious ownership`:

```bash
git config --global --add safe.directory /opt/netcore-tetra
```

### Konfiguration sichern

Vor dem ersten großen Durchlauf auf jedem LXC:

```bash
STAMP=$(date +%Y%m%d-%H%M%S)
mkdir -p /root/netcore-backups/$STAMP
cp -a /etc/netcore /root/netcore-backups/$STAMP/ 2>/dev/null || true
cp -a /etc/netcore-control-room /root/netcore-backups/$STAMP/ 2>/dev/null || true
```

Auf SIP-Systemen zusätzlich:

```bash
cp -a /etc/asterisk /root/netcore-backups/$STAMP/ 2>/dev/null || true
```

## 4. Rust auf allen Rust-Build-LXCs prüfen

Rust wird benötigt für:

```text
node-gateway, mobility-core, subscriber-core, group-core, call-control,
media-switch, recorder, sds-router, packet-core, ip-gateway,
security-core, kmf, transit, application-gateway, media-library,
control-room, observability und iot-gateway
```

Prüfung:

```bash
cargo --version
rustc --version
```

Fehlt Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source /root/.cargo/env
cargo --version
```

## 5. Installationsregel pro LXC

Nach jedem `git pull`:

```bash
cd /opt/netcore-tetra/system-backend/DIENST
find install -maxdepth 1 -type f -name '*.sh' -exec chmod 755 {} +
```

- Neuer Dienst/LXC: `./install/install.sh`
- Bereits installierter Dienst: `./install/update.sh`

Danach immer:

```bash
systemctl status UNIT --no-pager --full
journalctl -u UNIT -n 100 --no-pager
```

# 6. Installation in korrekter Reihenfolge

> Für die sieben in Abschnitt 0 neu angelegten LXCs wird beim ersten Durchlauf immer `./install/install.sh` verwendet. `./install/update.sh` ist erst für spätere Git-Updates vorgesehen.

## 6.1 Node Gateway

```bash
cd /opt/netcore-tetra
git pull --ff-only origin mqtt
cd system-backend/node-gateway
chmod 755 install/*.sh
./install/update.sh   # bei Neuinstallation: ./install/install.sh
systemctl status netcore-node-gateway --no-pager --full
curl -fsS http://$NODE_GATEWAY_IP:8080/health/live
```

Anschließend `/etc/netcore/node-gateway.toml` prüfen. Die Health-Ziele müssen auf die realen IPs aller übrigen Dienste zeigen. Die Beispielwerte `10.0.20.x` nicht unverändert übernehmen.

## 6.2 Mobility Core

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/mobility-core
chmod 755 install/*.sh
./install/update.sh   # neu: ./install/install.sh
systemctl status netcore-mobility-core --no-pager --full
curl -fsS http://$MOBILITY_CORE_IP:8090/health/live
```

## 6.3 Subscriber Core

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/subscriber-core
chmod 755 install/*.sh
./install/update.sh   # neu: ./install/install.sh
systemctl status netcore-subscriber-core --no-pager --full
curl -fsS http://$SUBSCRIBER_CORE_IP:8100/health/live
```

## 6.4 Group Core

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/group-core
chmod 755 install/*.sh
./install/update.sh   # neu: ./install/install.sh
systemctl status netcore-group-core --no-pager --full
curl -fsS http://$GROUP_CORE_IP:8110/health/live
```

## 6.5 Call Control

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/call-control
chmod 755 install/*.sh
./install/update.sh   # neu: ./install/install.sh
```

Danach `/etc/netcore/call-control.toml` prüfen:

```toml
[node_gateway]
url = "ws://<NODE_GATEWAY_IP>:8080/ws/backend"
reconnect_secs = 5

[mobility_core]
enabled = true
base_url = "http://<MOBILITY_CORE_IP>:8090"
timeout_ms = 1500
allow_local_fallback = false
accept_stale_route = false
```

Dann:

```bash
systemctl restart netcore-call-control
systemctl status netcore-call-control --no-pager --full
curl -fsS http://$CALL_CONTROL_IP:8120/health/live
```

## 6.6 Media Switch

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/media-switch
chmod 755 install/*.sh
./install/update.sh   # neu: ./install/install.sh
```

In `/etc/netcore/media-switch.toml`:

```toml
[node_gateway]
url = "ws://<NODE_GATEWAY_IP>:8080/ws/backend"
reconnect_secs = 2

[call_control]
url = "http://<CALL_CONTROL_IP>:8120/api/v1/calls"
events_url = "ws://<CALL_CONTROL_IP>:8120/ws/media"
route_ready_url = "http://<CALL_CONTROL_IP>:8120/api/v1/media/route-ready"
```

Dann:

```bash
systemctl restart netcore-media-switch
curl -fsS http://$MEDIA_SWITCH_IP:8130/health/live
```

## 6.7 Recorder

Vorher den NFS-Speicher bereitstellen. Bei unprivilegierten LXCs ist ein Mount auf dem Proxmox-Host mit Bind-Mount in den Container meist zuverlässiger als ein NFS-Mount direkt im LXC.

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/recorder
chmod 755 install/*.sh
./install/update.sh   # neu: ./install/install.sh
```

In `/etc/netcore/recorder.toml`:

```toml
tap_url = "http://<MEDIA_SWITCH_IP>:8130/api/v1/recorder/taps"
sessions_url = "http://<MEDIA_SWITCH_IP>:8130/api/v1/sessions"
```

Dann:

```bash
systemctl restart netcore-recorder
curl -fsS http://$RECORDER_IP:8140/health/live
```

## 6.8 SDS Router

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/sds-router
chmod 755 install/*.sh
./install/update.sh   # neu: ./install/install.sh
systemctl status netcore-sds-router --no-pager --full
curl -fsS http://$SDS_ROUTER_IP:8150/health/live
```

## 6.9 Packet Core

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/packet-core
chmod 755 install/*.sh
./install/update.sh   # neu: ./install/install.sh
systemctl status netcore-packet-core --no-pager --full
curl -fsS http://$PACKET_CORE_IP:8160/health/live
```

## 6.10 IP Gateway

Der LXC braucht `/dev/net/tun` und die für Routing/NAT erforderlichen Rechte.

```bash
ls -l /dev/net/tun
```

Dann:

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/ip-gateway
chmod 755 install/*.sh
./install/update.sh   # neu: ./install/install.sh
```

In `/etc/netcore/ip-gateway.toml`:

```toml
[packet_core]
url = "http://<PACKET_CORE_IP>:8160"
```

Danach:

```bash
systemctl restart netcore-ip-gateway
curl -fsS http://$IP_GATEWAY_IP:8170/health/live
```

## 6.11 Security Core

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/security-core
chmod 755 install/*.sh
./install/update.sh   # neu: ./install/install.sh
systemctl status netcore-security-core --no-pager --full
curl -fsS http://$SECURITY_CORE_IP:8180/health/live
```

## 6.12 KMF

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/kmf
chmod 755 install/*.sh
./install/update.sh   # neu: ./install/install.sh
systemctl status netcore-kmf --no-pager --full
curl -fsS http://$KMF_IP:8190/health/live
```

OPEN LAB ersetzt keine echte TETRA-Schlüsselverwaltung. Keine echten Netzschlüssel in Git ablegen.

## 6.13 Transit

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/transit
chmod 755 install/*.sh
./install/update.sh   # neu: ./install/install.sh
systemctl status netcore-transit --no-pager --full
curl -fsS http://$TRANSIT_IP:8200/health/live
```

`/etc/netcore/transit.toml` auf Mobility Core, Call Control, Media Switch und SDS Router ausrichten.

## 6.14 Application Gateway

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/application-gateway
chmod 755 install/*.sh
./install/update.sh   # neu: ./install/install.sh
```

In `/etc/netcore/application-gateway.toml` nur die tatsächlich verwendeten Connectoren aktivieren. Für die Kernanbindung mindestens die URLs von SDS Router und Media Library prüfen. Externe Connectoren wie Telegram bleiben deaktiviert, bis sie bewusst eingerichtet werden.

```bash
systemctl restart netcore-application-gateway
curl -fsS http://$APPLICATION_GATEWAY_IP:8220/health/live
```

## 6.15 Media Library

NFS/Shared Storage vorher mounten. Dann:

```bash
apt-get update
apt-get install -y ffmpeg python3-venv

cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/media-library
chmod 755 install/*.sh

NETCORE_TBS_ID=SRV-M-TBS-01 NETCORE_TBS_NAME=SRV-M-TBS-01 NETCORE_TBS_URL=http://$TBS_01_IP:8080 ./install/update.sh
```

Bei Neuinstallation statt `update.sh`:

```bash
NETCORE_TBS_ID=SRV-M-TBS-01 NETCORE_TBS_NAME=SRV-M-TBS-01 NETCORE_TBS_URL=http://$TBS_01_IP:8080 ./install/install.sh
```

In `/etc/netcore/media-library.toml` prüfen:

```toml
media_switch_base_url = "http://<MEDIA_SWITCH_IP>:8130"
recorder_base_url = "http://<RECORDER_IP>:8140"
application_gateway_base_url = "http://<APPLICATION_GATEWAY_IP>:8220"
```

Danach:

```bash
systemctl status netcore-media-library --no-pager --full
curl -fsS http://$MEDIA_LIBRARY_IP:8230/health/live
```

## 6.16 IoT Gateway und MQTT-Broker – erster der sieben neuen LXCs

Vor diesem Abschnitt müssen die vorhandenen Core-Dienste bis einschließlich Media Library erreichbar sein.

Dieser LXC ist der zentrale MQTT-Broker. Beim ersten Installieren:

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/iot-gateway
chmod 755 install/*.sh
INSTALL_LOCAL_MQTT_BROKER=1 ./install/install.sh
```

Bei Updates:

```bash
INSTALL_LOCAL_MQTT_BROKER=0 ./install/update.sh
```

Quellen konfigurieren:

```bash
./install/configure-openlab.sh   $NODE_GATEWAY_IP   $MOBILITY_CORE_IP   $CALL_CONTROL_IP   $SDS_ROUTER_IP   $IOT_GATEWAY_IP
```

Prüfen:

```bash
systemctl status mosquitto --no-pager --full
systemctl status netcore-iot-gateway --no-pager --full
ss -ltnp | grep 8240
ss -ltnp | grep 1883
curl -fsS http://$IOT_GATEWAY_IP:8240/api/v1/status | python3 -m json.tool
```

## 6.17 Hardware Gateway

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/hardware-gateway
chmod 755 install/*.sh
./install/install.sh
```

In `/etc/netcore/hardware-gateway.toml`:

```toml
[mqtt]
host = "<IOT_GATEWAY_IP>"
port = 1883
```

Dann:

```bash
systemctl restart netcore-hardware-gateway
curl -fsS http://$HARDWARE_GATEWAY_IP:8250/health/live
```

## 6.18 RF Monitor

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/rf-monitor
chmod 755 install/*.sh
./install/install.sh
./install/configure-openlab.sh $IOT_GATEWAY_IP
curl -fsS http://$RF_MONITOR_IP:8260/health/live
```

## 6.19 Alarm Workflow

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/alarm-workflow
chmod 755 install/*.sh
./install/install.sh
./install/configure-openlab.sh $IOT_GATEWAY_IP $SDS_ROUTER_IP
curl -fsS http://$ALARM_WORKFLOW_IP:8270/health/live
```

Empfänger und GSSI in `/etc/netcore/alarm-workflow.toml` prüfen. Für deine Technikgruppe ist `15201` der vorgesehene Startwert.

## 6.20 Task Workflow

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/task-workflow
chmod 755 install/*.sh
./install/install.sh
./install/configure-openlab.sh $IOT_GATEWAY_IP $SDS_ROUTER_IP 15201
curl -fsS http://$TASK_WORKFLOW_IP:8280/health/live
```

WAP-Tests:

```bash
curl -fsS "http://$TASK_WORKFLOW_IP:8280/x?issi=4010001" | head
curl -fsS "http://$TASK_WORKFLOW_IP:8280/w?issi=4010001" | head
```

## 6.21 Asset Management

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/asset-management
chmod 755 install/*.sh
./install/install.sh
./install/configure-openlab.sh   $IOT_GATEWAY_IP   $SUBSCRIBER_CORE_IP   $MOBILITY_CORE_IP   $TASK_WORKFLOW_IP
curl -fsS http://$ASSET_MANAGEMENT_IP:8290/health/live
```

## 6.22 Zentraler SIP-Switch

Firewall/NAT für diese Bereiche berücksichtigen:

```text
5060/UDP
10000-20000/UDP
8300/TCP
```

Installation/Update:

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/sip-switch
chmod 755 install/*.sh
./install/install.sh
```

Zentralen PBX-Weg als **eine Registrierung** konfigurieren:

```bash
./install/configure-openlab.sh   $PBX_IP   $MOBILITY_CORE_IP   $IOT_GATEWAY_IP   registration
```

In `/etc/netcore/sip-switch.toml` den PBX-Abschnitt prüfen und gegebenenfalls Benutzer/Kennwort ergänzen. Der normale PBX-Kontakt gehört ausschließlich dem zentralen SIP-Switch.

Jede TBS einmal anlegen:

```bash
./install/add-tbs-openlab.sh   SRV-M-TBS-01   tbs-srv-m-tbs-01   EIN_EIGENES_OPENLAB_PASSWORT
```

Für weitere TBS entsprechend wiederholen. Das Skript nicht erneut für bereits vorhandene `node_id` ausführen.

Prüfen:

```bash
systemctl status asterisk --no-pager --full
systemctl status netcore-sip-switch --no-pager --full
asterisk -rx 'pjsip show registrations'
asterisk -rx 'pjsip show endpoints'
curl -fsS http://$SIP_SWITCH_IP:8300/api/v1/status | python3 -m json.tool
```

## 6.23 Control Room

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/control-room
chmod 755 install/*.sh
./install/update.sh   # neu: ./install/install.sh
```

In `/etc/netcore-control-room/control-room.toml` sämtliche `base_url`- und `url`-Einträge von den Beispieladressen auf deine realen Dienst-IP-Adressen umstellen. Danach:

```bash
systemctl restart netcore-control-room
curl -fsS http://$CONTROL_ROOM_IP:9010/health/live
```

## 6.24 Observability

```bash
cd /opt/netcore-tetra && git pull --ff-only origin mqtt
cd system-backend/observability
chmod 755 install/*.sh
./install/update.sh   # neu: ./install/install.sh
```

In `/etc/netcore/observability.toml` alle Targets auf die realen LXC-Adressen umstellen. Die Beispielkonfiguration enthält teils `127.0.0.1`; das ist bei getrennten LXCs falsch.

```bash
systemctl restart netcore-observability
curl -fsS http://$OBSERVABILITY_IP:8210/health/live
```

# 7. Basisstationen aktualisieren

## 7.1 Git auf jeder TBS

Die TBS arbeitet typischerweise unter `/home/jan/netcore-tetra`:

```bash
cd /home/jan/netcore-tetra
git status --short
git fetch origin
git switch mqtt
git pull --ff-only origin mqtt
git log -1 --oneline
```

## 7.2 SoapySX prüfen

```bash
SoapySDRUtil --find
```

Fehlt SoapySX:

```bash
sudo apt-get update
sudo apt-get install -y --no-install-recommends   git make g++ cmake libsoapysdr-dev libasound2-dev   soapysdr-tools python3-soapysdr

cd /home/jan
rm -rf sxxcvr
git clone https://github.com/tejeez/sxxcvr.git
cd sxxcvr/SoapySX
mkdir -p build
cd build
cmake ..
make -j"$(nproc)"
sudo make install
sudo ldconfig
```

## 7.3 Basisstations-Binary bauen und austauschen

```bash
cd /home/jan/netcore-tetra
sudo -E env   BUILD_USER=jan   CARGO_FEATURES="asterisk,recording,audio-player"   ./install/update-basisstation.sh
```

Danach:

```bash
systemctl status bluestation --no-pager --full || systemctl status tetra --no-pager --full
```

## 7.4 TBS-Verbindung zum Node Gateway prüfen

In `/etc/netcore/config.toml`:

```toml
[control_room]
enabled = true
host = "<NODE_GATEWAY_IP>"
port = 8080
use_tls = false
endpoint_path = "/ws/node"
node_id = "SRV-M-TBS-01"
station_name = "SRV-M-TBS-01"
central_sds_routing = true
```

Keine Login-/Token-Felder im OPEN LAB eintragen.

## 7.5 Media Library auf der TBS

In `/etc/netcore/config.toml`:

```toml
[media_library]
enabled = true
base_url = "http://<MEDIA_LIBRARY_IP>:8230"
station_id = "SRV-M-TBS-01"
publish_recordings = true
recording_source_base_url = "http://<TBS_01_IP>:8080"
auto_approve_recordings = false
audio_source_enabled = true
only_ready = true
only_approved = true
retry_seconds = 60
request_timeout_seconds = 15
download_timeout_seconds = 120
max_list_entries = 1000
```

## 7.6 RF-Agent auf jeder TBS

Erstinstallation:

```bash
cd /home/jan/netcore-tetra/system-backend/rf-monitor
sudo ./install/install-tbs-agent.sh   http://127.0.0.1:8080   http://$RF_MONITOR_IP:8260   SRV-M-TBS-01
```

Danach `/etc/netcore/rf-agent.toml` prüfen, insbesondere Dashboard-Zugang, Station-ID und RF-Monitor-URL.

```bash
systemctl restart netcore-rf-agent
systemctl status netcore-rf-agent --no-pager --full
```

## 7.7 Lokalen Asterisk-Fallback Phase 11c installieren

Auf dem zentralen SIP-Switch muss die TBS vorher mit denselben Zugangsdaten angelegt sein.

Auf TBS-01:

```bash
cd /home/jan/netcore-tetra/system-backend/sip-switch
sudo ./install/install-tbs-local-fallback.sh   SRV-M-TBS-01   $TBS_01_IP   $SIP_SWITCH_IP   tbs-srv-m-tbs-01   DAS_GLEICHE_PASSWORT_WIE_AM_SWITCH   $PBX_IP   netcore-tbs-01
```

Danach die native TBS-SIP-Bridge auf den lokalen Asterisk umstellen:

```bash
sudo ./install/apply-tbs-local-asterisk-config.sh   --config /etc/netcore/config.toml
```

Basisstation neu starten:

```bash
systemctl restart bluestation || systemctl restart tetra
```

Prüfen:

```bash
systemctl status asterisk --no-pager --full
systemctl status netcore-tbs-sip-failover --no-pager --full
./install/tbs-fallback-status.sh | python3 -m json.tool
asterisk -rx 'pjsip show registrations'
```

Normalzustand:

```text
central / CENTRAL_ACTIVE
zentrale TBS→SIP-Switch-Registrierung aktiv
TBS→PBX-Direktregistrierung nicht geladen
```

Bei einem bestätigten Ausfall des zentralen SIP-Switches:

```text
pbx_direct / PBX_DIRECT_ACTIVE
zentrale Registrierung entfernt
direkte PBX-Registrierung der betroffenen TBS aktiv
```

## 7.8 Spätere Updates des lokalen TBS-Fallbacks

Nach einem späteren `git pull`:

```bash
cd /home/jan/netcore-tetra/system-backend/sip-switch
sudo ./install/update-tbs-local-fallback.sh
```

# 8. Gesamtprüfung

## 8.1 Health-Checks

Von einem Rechner, der alle Management-LXCs erreicht:

```bash
for URL in   http://$NODE_GATEWAY_IP:8080/health/live   http://$MOBILITY_CORE_IP:8090/health/live   http://$SUBSCRIBER_CORE_IP:8100/health/live   http://$GROUP_CORE_IP:8110/health/live   http://$CALL_CONTROL_IP:8120/health/live   http://$MEDIA_SWITCH_IP:8130/health/live   http://$RECORDER_IP:8140/health/live   http://$SDS_ROUTER_IP:8150/health/live   http://$PACKET_CORE_IP:8160/health/live   http://$IP_GATEWAY_IP:8170/health/live   http://$SECURITY_CORE_IP:8180/health/live   http://$KMF_IP:8190/health/live   http://$TRANSIT_IP:8200/health/live   http://$OBSERVABILITY_IP:8210/health/live   http://$APPLICATION_GATEWAY_IP:8220/health/live   http://$MEDIA_LIBRARY_IP:8230/health/live   http://$IOT_GATEWAY_IP:8240/health/live   http://$HARDWARE_GATEWAY_IP:8250/health/live   http://$RF_MONITOR_IP:8260/health/live   http://$ALARM_WORKFLOW_IP:8270/health/live   http://$TASK_WORKFLOW_IP:8280/health/live   http://$ASSET_MANAGEMENT_IP:8290/health/live   http://$SIP_SWITCH_IP:8300/health/live   http://$CONTROL_ROOM_IP:9010/health/live
do
  printf '%-65s ' "$URL"
  curl -fsS "$URL" >/dev/null && echo OK || echo FEHLER
done
```

## 8.2 MQTT prüfen

```bash
mosquitto_sub   -h $IOT_GATEWAY_IP   -p 1883   -v   -t 'netcore/v1/#'
```

## 8.3 Mobility und SIP prüfen

```bash
curl -fsS   http://$MOBILITY_CORE_IP:8090/api/v1/subscribers/4010001/route   | python3 -m json.tool

curl -fsS   "http://$SIP_SWITCH_IP:8300/api/v1/resolve?number=4010001"   | python3 -m json.tool
```

## 8.4 Git-Stand prüfen

Auf jedem LXC und jeder TBS:

```bash
git -C /opt/netcore-tetra branch --show-current 2>/dev/null || git -C /home/jan/netcore-tetra branch --show-current

git -C /opt/netcore-tetra log -1 --oneline 2>/dev/null || git -C /home/jan/netcore-tetra log -1 --oneline
```

# 9. Empfohlene Reihenfolge für deinen echten Durchlauf

1. IP-Liste festlegen.
2. Node Gateway aktualisieren.
3. Mobility, Subscriber und Group Core aktualisieren.
4. Call Control und Media Switch aktualisieren.
5. Recorder, SDS Router, Packet Core und IP Gateway aktualisieren.
6. Security Core, KMF, Transit und Application Gateway aktualisieren.
7. Media Library aktualisieren und NFS prüfen.
8. IoT Gateway samt Mosquitto installieren/aktualisieren.
9. Hardware Gateway und RF Monitor installieren.
10. Alarm Workflow, Task Workflow und Asset Management installieren.
11. Zentralen SIP-Switch installieren und PBX-Registrierung konfigurieren.
12. Control Room und Observability zuletzt aktualisieren.
13. Erst danach jede TBS einzeln aktualisieren.
14. Pro TBS RF-Agent und Phase-11c-Fallback installieren.
15. Gesamt-Healthcheck, MQTT, SDS, WAP und SIP-Failover testen.

# 10. Wichtige OPEN-LAB-Grenze

WebUIs, MQTT, APIs und teilweise SIP laufen ohne Login, Token oder TLS. Deshalb ausschließlich in deinem isolierten Managementnetz/VLAN betreiben und nicht ins Internet weiterleiten.

# 11. Spätere Updates der sieben neuen LXCs

Nach der Erstinstallation läuft jedes weitere Update ausschließlich über Git:

```bash
cd /opt/netcore-tetra
git status --short
git fetch origin
git switch mqtt
git pull --ff-only origin mqtt
```

Dann nur den lokalen Dienst aktualisieren:

```bash
cd /opt/netcore-tetra/system-backend/DIENST
find install -maxdepth 1 -type f -name '*.sh' -exec chmod 755 {} +
./install/update.sh
```

Die sieben Werte für `DIENST` sind:

```text
iot-gateway
hardware-gateway
rf-monitor
alarm-workflow
task-workflow
asset-management
sip-switch
```

Danach immer Konfiguration und Laufzeitdaten außerhalb des Repositories kontrollieren:

```text
/etc/netcore/
/var/lib/netcore-*/
/etc/asterisk/              nur SIP-Switch und lokale TBS
```

