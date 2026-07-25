---
title: "NetCore-Tetra Komplettguide"
subtitle: "Installation der Basisstation und aller 17 LXC-Dienste - Konfiguration, Bedienung, Offline-Fallback und Abnahme"
author: "NetCore-Tetra Projekt"
date: "Stand: 24. Juli 2026"
lang: de-DE
---

> **WICHTIG - OPEN LAB:** Die in diesem Stand enthaltenen LXC-WebUIs arbeiten absichtlich ohne Login, Management-Token und TLS. Jeder Client, der das Managementnetz erreicht, kann administrative Änderungen durchführen. Das komplette System gehört deshalb in ein isoliertes Labor-/Management-VLAN ohne Portweiterleitung aus dem Internet. Security Core und KMF sollten zusätzlich in einem besonders restriktiven Segment liegen.

> **Funkrecht:** Frequenzen, Sendeleistung, Rufzeichen/Identitäten und Betriebsart müssen zur jeweiligen Genehmigung passen. Die Anleitung ersetzt weder Frequenzzuteilung noch EMV-/HF-Abnahme.

# Inhalt
- 1. Schnellstart und Zielbild
- 2. Architektur, Dienste und Ports
- 3. Planung von Netzwerk, LXC und Storage
- 4. Installation der TETRA-Basisstation
- 5. Konfiguration der Basisstation
- 6. Proxmox-LXC vorbereiten
- 7. Automatisches Deployment aller LXC
- 8. Manuelle Installation und Bedienung je Dienst
- 9. Konfigurationsreferenz der LXC-Dienste
- 10. Betriebs- und Bedienabläufe
- 11. Offline-Fallback der Basisstation
- 12. Backup, Update und Wiederherstellung
- 13. Systemtest und Abnahme
- 14. Fehlersuche
- 15. Inbetriebnahme-Checkliste
- Anhang A: Port-, Pfad- und Befehlsreferenz

# 1. Schnellstart und Zielbild
Dieser Guide führt vom leeren Debian-/Raspberry-Pi-System und frisch angelegten Proxmox-LXC bis zur ersten vollständigen Laborabnahme. Die empfohlene Topologie besteht aus einer funkseitigen Basisstation und 17 voneinander getrennten Backend-LXC. Die Basisstation bleibt bei Ausfall von Internet, VPN oder Core-Diensten lokal betriebsfähig.

## 1.1 Empfohlene Reihenfolge
1. Management-VLAN, IP-Adressen, DNS/NTP und Storage planen.
2. Basisstation bauen und zunächst **standalone** mit lokaler Konfiguration und Fallback-Datei testen.
3. 17 Debian-LXC anlegen; IP Gateway erhält zusätzlich `/dev/net/tun`.
4. Korrigiertes Inventory aus diesem Guide anpassen und `validate`, `plan`, `render`, `apply --dry-run` ausführen.
5. LXC in Abhängigkeitsreihenfolge deployen.
6. Basisstation an den Node Gateway anbinden; zentrale SDS-Routen zunächst noch deaktiviert lassen.
7. Smoke-Test, dann funktionalen Full-Test, zuletzt Fault-/Fallback-Test durchführen.
8. Shadow-Dienste einzeln und kontrolliert auf `authoritative` schalten.

![Deployment-Ablauf](assets/deployment.png){ width=95% }

## 1.2 Minimale Startbefehle
```bash
# Auf dem Deployment-Host
cp inventory.open-lab.corrected.example.toml deploy/open-lab/inventory.toml
${EDITOR:-nano} deploy/open-lab/inventory.toml
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml validate
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml plan
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml render
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml apply --dry-run
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml apply
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml status
```

Danach auf der Basisstation `[control_room]` auf den Node Gateway zeigen lassen und den Dienst neu starten. Der erste Systemtest ist:
```bash
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile smoke
```
# 2. Architektur, Dienste und Ports
![Vereinfachte Systemarchitektur](assets/architecture.png){ width=100% }

## 2.1 Dienstmatrix
| Reihenfolge | Dienst | Beispiel-IP | Port | Aufgabe | Abhängigkeiten |
|---:|---|---|---:|---|---|
| 1 | `node-gateway` | `10.0.20.10` | 8080 | Zentraler Einstiegspunkt für TBS-WebSockets und normalisierter Transport zu den Backend-Diensten. | - |
| 2 | `mobility-core` | `10.0.20.11` | 8090 | Teilnehmerlage, Serving-Node-Zuordnung und MM-Context-Transfers zwischen TBS. | node-gateway |
| 3 | `subscriber-core` | `10.0.20.12` | 8100 | Zentrale Teilnehmerprofile und Admission-Policy. | node-gateway |
| 4 | `group-core` | `10.0.20.13` | 8110 | GSSI-Stammdaten, Mitgliedschaften, Affiliationen und DGNA. | node-gateway, subscriber-core |
| 5 | `call-control` | `10.0.20.14` | 8120 | Netzweite logische Gruppen-/Individualrufe, Call Legs, Floor und Restore. | node-gateway, subscriber-core, group-core, mobility-core |
| 6 | `media-switch` | `10.0.20.15` | 8130 | Routing bereits codierter 35-Byte-TETRA-Sprachframes zwischen TBS-Call-Legs. | node-gateway, call-control |
| 7 | `recorder` | `10.0.20.16` | 8140 | Passive, verlustfreie Aufzeichnung von TACELP-Frames außerhalb des Rufpfads. | media-switch |
| 8 | `sds-router` | `10.0.20.17` | 8150 | Zentrale SDS-/Statusvermittlung, Store-and-forward und Anwendungsrouten. | node-gateway, subscriber-core, group-core, mobility-core |
| 9 | `packet-core` | `10.0.20.18` | 8160 | PDP-/NSAPI-State-Machine, IPv4-Leases, Mobility Anchoring, Fragmente und Flow Control. | node-gateway, subscriber-core, mobility-core |
| 10 | `ip-gateway` | `10.0.20.19` | 8170 | Layer-3-Übergang zwischen Packet Core und normalen IPv4-Netzen. | packet-core |
| 11 | `security-core` | `10.0.20.20` | 8180 | Security-Class-Policy, Lab-Authentisierung, DCK-Kontexte, Sperren und Audit. | node-gateway, subscriber-core |
| 12 | `kmf` | `10.0.20.21` | 8190 | Lifecycle für CCK/GCK/SCK, Rotation, Crypto Periods und OTAR-Orchestrierung. | security-core |
| 13 | `transit` | `10.0.20.22` | 8200 | NetCore-native Regionalvermittlung mit Peers, Routen, Sessions und Failover. | mobility-core, call-control, media-switch, sds-router |
| 14 | `application-gateway` | `10.0.20.23` | 8220 | Adapter- und Workflow-Gateway für externe Anwendungen, Webhooks, Vorlagen und TTS. | sds-router |
| 15 | `media-library` | `10.0.20.24` | 8230 | Zentrale Medienablage, Vorschau, Freigabe, TACELP-Cache und Playout. | media-switch, recorder, application-gateway |
| 16 | `control-room` | `10.0.20.25` | 9010 | Zentrale Lage-, Operator-, Incident- und Schichtbuchebene. | node-gateway, subscriber-core, group-core, mobility-core, call-control, media-switch, recorder, sds-router, packet-core, ip-gateway, security-core, kmf, transit, application-gateway, media-library |
| 17 | `observability` | `10.0.20.26` | 8210 | Zentrale Metriken, Logs, Traces, Alerts, Silences und Diagnosepakete. | node-gateway, subscriber-core, group-core, mobility-core, call-control, media-switch, recorder, sds-router, packet-core, ip-gateway, security-core, kmf, transit, application-gateway, media-library, control-room |

Alle Dienste stellen ihre WebUI direkt auf dem jeweiligen Managementport bereit. Standard-Endpunkte sind `GET /health/live`, `GET /health/ready`, `GET /metrics` und `GET /openapi.json`; einzelne ältere APIs verwenden zusätzlich abweichende OpenAPI-Pfade.

## 2.2 Autorität und Zuständigkeit
- Die **TBS** bleibt Eigentümerin von PHY, MAC, LLC, MLE, lokaler MM/CMCE-Zeitkritik, lokaler Sprachführung und Air-PDU-Encoding.
- Die **Fachkerne** halten langlebige netzweite Zustände und Policies.
- Der **Control Room** zeigt Lage und Bedienwege, erzeugt aber keine zweite Wahrheit neben den Fachkernen.
- **Observability** überwacht, darf aber keinen Funk- oder Call-Pfad blockieren.
- `shared/` ist eine Library und **kein** 18. Container.

## 2.3 Aktivierungsstrategie
Dienste mit `shadow`/`authoritative` werden zuerst im Shadow-Modus installiert. So lassen sich URLs, Abhängigkeiten und Zustandsmodelle prüfen, ohne sofort externe Nebenwirkungen, Kerneländerungen, OTAR oder Funkinjektionen auszulösen.

| Dienst | Startmodus | Erst nach erfolgreichem Test auf `authoritative` |
|---|---|---|
| packet-core | `shadow` | packet.mode |
| ip-gateway | `shadow` | interface.mode |
| security-core | `shadow` | policy.operating_mode |
| kmf | `shadow` | policy.operating_mode |
| transit | `shadow` | region.operating_mode |
| application-gateway | `shadow` | runtime.operating_mode |
| media-library | `shadow` | runtime.operating_mode |
# 3. Planung von Netzwerk, LXC und Storage
## 3.1 Managementnetz
Die Beispieltopologie verwendet `10.0.20.0/24`. Sie kann ersetzt werden, muss aber in Inventory, gerenderten Konfigurationen, Firewallregeln und Basisstationskonfiguration konsistent bleiben. Ein Default-Gateway ist für den lokalen Corebetrieb nicht erforderlich; es wird nur für Paketupdates, externe Connectoren, Transit-WAN oder Internetzugang des IP Gateways benötigt.

Empfehlungen:
- eigenes VLAN und eigene Firewallzone;
- nur Deployment-/Adminhost, Basisstation und notwendige LXC dürfen zugreifen;
- keine Portweiterleitung;
- NTP intern bereitstellen;
- statische DHCP-Leases oder feste Adressen;
- Hostnamen aus `deploy/open-lab/generated/hosts.example` übernehmen oder internes DNS pflegen.

## 3.2 LXC-Ressourcen
Die folgenden Werte sind Labor-Empfehlungen, keine harten Mindestwerte. Da die Installer Rust lokal kompilieren, benötigen sie während des Builds mehr RAM und CPU als später im Runtimebetrieb. Bei 2 GB RAM sollte temporär zusätzlicher Swap vorhanden sein; komfortabler sind 4 GB während des Builds.

| Dienst | vCPU | RAM | Disk | Hinweis |
|---|---:|---:|---:|---|
| `node-gateway` | 2 | 2 GB | 8 GB | Standard-LXC |
| `mobility-core` | 2 | 2 GB | 8 GB | Standard-LXC |
| `subscriber-core` | 2 | 2 GB | 8 GB | Standard-LXC |
| `group-core` | 2 | 2 GB | 8 GB | Standard-LXC |
| `call-control` | 2 | 2 GB | 8 GB | Standard-LXC |
| `media-switch` | 2-4 | 2-4 GB | 12 GB | Standard-LXC |
| `recorder` | 2 | 2 GB | 16 GB + Archiv | Aufzeichnungen separat dimensionieren |
| `sds-router` | 2 | 2 GB | 8 GB | Standard-LXC |
| `packet-core` | 2 | 2 GB | 8 GB | Standard-LXC |
| `ip-gateway` | 2 | 2 GB | 12 GB + PCAP | /dev/net/tun; CAP_NET_ADMIN/RAW |
| `security-core` | 2 | 2 GB | 8 GB | restriktives Managementsegment |
| `kmf` | 2 | 2 GB | 12 GB | restriktives Managementsegment |
| `transit` | 2 | 2 GB | 8 GB | Standard-LXC |
| `application-gateway` | 2 | 2 GB | 12 GB | Standard-LXC |
| `media-library` | 2-4 | 4 GB | 32 GB + Archiv | Medien/Preview/Archive; ffmpeg |
| `control-room` | 2 | 2 GB | 12 GB | Standard-LXC |
| `observability` | 4 | 4-8 GB | 32-100 GB | Retention bestimmt Diskbedarf |

## 3.3 Storage und NFS
- Live-State bleibt lokal auf dem LXC-Dateisystem.
- Recorder und Media Library dürfen Archive auf NFS ablegen, sollen aber nicht von einem langsamen NFS im zeitkritischen Pfad abhängen.
- KMF-Master-Key und KMF-Backup **nicht** am selben Ort sichern.
- Observability-Retention vorab an die Diskgröße anpassen.
- PCAP, Diagnosepakete und TAR-Exporte können sehr schnell wachsen.

Beispiel `/etc/fstab` für ein optionales Archiv:
```fstab
10.0.20.5:/netcore-archive /mnt/nfs-share nfs4 rw,_netdev,nofail,x-systemd.automount,x-systemd.idle-timeout=60 0 0
```
Mit `nofail` und Automount blockiert ein fehlendes NAS den Start nicht unbegrenzt.
# 4. Installation der TETRA-Basisstation
## 4.1 Voraussetzungen
- Raspberry Pi 4/5 oder vergleichbarer 64-Bit-Linux-Rechner;
- Debian/Raspberry Pi OS 64 Bit mit systemd;
- funktionierendes SDR samt SoapySDR-Treiber;
- stabile Zeitbasis und ausreichend Kühlung;
- Netzwerkzugriff zum Node Gateway im Management-VLAN;
- optional NFS, ffmpeg, Piper, Asterisk/Brew.

## 4.2 Betriebssystempakete
```bash
sudo apt update
sudo apt install -y git curl ca-certificates build-essential pkg-config cmake clang \
  libsoapysdr-dev soapysdr-tools ffmpeg jq sqlite3 nfs-common
```
Gerätespezifische SoapySDR-Treiber zusätzlich installieren. Für eine komplett offline betriebene Station werden diese Pakete und der Rust-Toolchain vorab über ein internes Repository, Paketcache oder ein vorbereitetes Image bereitgestellt; zur Laufzeit benötigt die lokale TBS kein Internet.

Rust installieren:
```bash
curl --proto =https --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup update stable
```

## 4.3 SDR prüfen
```bash
SoapySDRUtil --info
SoapySDRUtil --find
SoapySDRUtil --probe="driver=<TREIBER>"
```
Vor dem ersten Sendebetrieb Sample-Rate, Kanalnummer, Center-Frequenz, Gains und Passband prüfen. Dual-Carrier ist nur sinnvoll, wenn beide 25-kHz-Träger sauber im SDR-Passband liegen.

## 4.4 Benutzer und Verzeichnisse
```bash
sudo useradd --system --home /var/lib/netcore --shell /usr/sbin/nologin netcore 2>/dev/null || true
sudo install -d -o netcore -g netcore /var/lib/netcore /var/lib/netcore/recordings \
  /var/lib/netcore/audio /var/lib/netcore/tts/templates /var/cache/netcore/audio \
  /var/cache/netcore/tts /var/lib/flowstation
sudo install -d -m 0750 /etc/netcore /opt/netcore
```
Den Benutzer der Gerätegruppe des SDR hinzufügen, beispielsweise `plugdev`, `dialout` oder eine gerätespezifische Gruppe:
```bash
sudo usermod -aG plugdev,dialout netcore
```

## 4.5 Repository bereitstellen
Online per Git:
```bash
cd /opt
sudo git clone <REPOSITORY-URL> netcore-tetra
sudo chown -R "$USER":"$USER" /opt/netcore-tetra
cd /opt/netcore-tetra
```
Offline per ZIP:
```bash
sudo mkdir -p /opt/netcore-tetra
sudo unzip netcore-tetra-swmi-full-system-edge-fallback-open-lab-no-pdf.zip -d /opt
cd /opt/netcore-tetra-swmi
```

## 4.6 Build und Installation
```bash
cd /opt/netcore-tetra-swmi
cargo clean
rm -rf target
cargo build --release -p bluestation-bs
sudo install -m 0755 target/release/bluestation-bs /usr/local/bin/bluestation-bs
sudo install -m 0640 -o root -g netcore basisstation.config.sanitized.example.toml /etc/netcore/config.toml
sudo cp /etc/netcore/config.toml /etc/netcore/config.toml.fallback
sudo chmod 0640 /etc/netcore/config.toml /etc/netcore/config.toml.fallback
```
Die bereitgestellte bereinigte Konfiguration ist eine Startvorlage. Vor Start müssen Frequenzen, Netzwerkidentitäten, Dashboard-Zugang und Integrationen angepasst werden.

## 4.7 Erster manueller Start
```bash
sudo -u netcore RUST_LOG=info /usr/local/bin/bluestation-bs /etc/netcore/config.toml
```
Prüfen: TOML ohne Parserfehler, SDR erkannt, Downlink stabil, WebUI erreichbar, keine dauerhaften Underflow-/Passbandfehler. Mit `Ctrl+C` beenden.

## 4.8 systemd-Unit
```ini
[Unit]
Description=NetCore TETRA Basisstation
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=netcore
Group=netcore
WorkingDirectory=/opt/netcore-tetra-swmi
ExecStart=/usr/local/bin/bluestation-bs /etc/netcore/config.toml
Restart=on-failure
RestartSec=5
TimeoutStopSec=20
LimitNOFILE=65536
Environment=RUST_LOG=info

[Install]
WantedBy=multi-user.target
```
Als `/etc/systemd/system/tetra.service` speichern:
```bash
sudo systemctl daemon-reload
sudo systemctl enable --now tetra.service
sudo systemctl status tetra.service --no-pager
sudo journalctl -u tetra.service -n 200 --no-pager
```

## 4.9 Optional: Piper TTS
```bash
sudo system-backend/tts/install-piper.sh
sudo systemctl status netcore-piper.service --no-pager
curl -fsS http://127.0.0.1:5005/voices | jq .
```
TTS erzeugt Dateien. Die eigentliche Funk-Aussendung erfolgt kontrolliert über Media Library/Recording-Workflow, nicht direkt aus der Synthese.
# 5. Konfiguration der Basisstation
## 5.1 Grundregeln
- Vor jeder Änderung `config.toml`, `config.toml.fallback` und letzte `.bak` sichern.
- Konfigurationsdateien nur für root und Dienstgruppe lesbar machen.
- Primärdatei nach Änderung manuell testen; erst danach systemd neu starten.
- Die Fallback-Datei konservativ halten und nicht automatisch überschreiben.
- Die mitgelieferte Originalkonfiguration enthielt standortspezifische Werte; der Guide liefert deshalb eine bereinigte Vorlage ohne ursprüngliche Zugangsdaten.

```bash
sudo cp /etc/netcore/config.toml /etc/netcore/config.toml.$(date +%F-%H%M).bak
sudo -u netcore /usr/local/bin/bluestation-bs /etc/netcore/config.toml
```

## 5.2 Pflichtsektionen
| Sektion | Bedeutung |
|---|---|
| `[phy_io] / [phy_io.soapysdr]` | SDR-Backend, Frequenzen, Sample-Rate, Center-Frequenzen, Kanal und Gains. |
| `[net_info]` | MCC und MNC. |
| `[cell_info]` | Band, Carrier, Duplex, LAC, Colour Code, Dienste und Rufverhalten. |
| `[dashboard]` | Bind-Adresse, Port und lokaler Dashboard-Zugang. |
| `[control_room]` | Verbindung der TBS zum Node Gateway. |
| `[edge_fallback]` | Lokale Autonomie, Matrix-Lease, Policycache und Replay-Spool. |

## 5.3 RF- und Zellparameter
Beispiel:
```toml
[phy_io]
backend = "SoapySdr"

[phy_io.soapysdr]
tx_freq = 418000000             # ANPASSEN
rx_freq = 408000000             # ANPASSEN
device = "driver=<TREIBER>"
sample_rate = 600000
tx_center_freq = 418012500       # bei Dual Carrier
rx_center_freq = 408012500

[net_info]
mcc = 1                          # ANPASSEN
mnc = 333                        # ANPASSEN

[cell_info]
freq_band = 4
main_carrier = 720               # ANPASSEN
secondary_carrier = 721          # optional
duplex_spacing = 0
freq_offset = 0
reverse_operation = false
location_area = 1
colour_code = 1
timezone = "Europe/Berlin"
registration = true
deregistration = true
voice_service = true
sndcp_service = true
advanced_link = true
```
Carrier-Nummer und Center-Frequenz sind getrennte Ebenen: formal korrekte Carrier können trotzdem außerhalb des eingestellten SDR-Passbands liegen.

## 5.4 Packet Data auf der TBS
Die lokale `[cell_info.wap_ip]`- und `[cell_info.packet_data_gateway]`-Funktion bildet den Air-Interface-nahen Fallback. Beim zentralen Betrieb müssen TBS-Pool, Packet-Core-Pool und IP-Gateway-Netz zusammenpassen. Nicht gleichzeitig zwei Gateways mit derselben TETRA-IP aktiv routen lassen.

Empfohlener Übergang:
1. TBS Packet Data lokal testen.
2. Packet Core im `shadow`-Modus anbinden.
3. IP Gateway im `shadow`-Modus und Kernel-Plan prüfen.
4. In einem Wartungsfenster zentrale Autorität aktivieren.
5. Fallback der lokalen TBS getrennt testen.

## 5.5 SDS-Kommandos
`[cell_info.sds_command_control]` darf nur explizit autorisierte ISSI enthalten. Restart/Shutdown sind RF-wirksame Aktionen. Im offenen Labornetz darf der Managementzugang nicht als Ersatz für diese Allowlist missverstanden werden.

## 5.6 Dashboard, Recording, Audio und TTS
- Dashboard-Passwort sofort ändern; Port nur im Managementnetz öffnen.
- Recording- und Audioverzeichnisse auf Eigentümer/Rechte prüfen.
- NFS-Archive mit `nofail`/Automount anbinden.
- TTS-Endpunkt standardmäßig lokal `http://127.0.0.1:5005` oder über Application Gateway/Media Library verwenden.
- Ohne echten TETRA-Encoder bleiben WAV/MP3 nur previewfähig.

## 5.7 Node-Gateway-Anbindung
```toml
[control_room]
enabled = true
host = "10.0.20.10"
port = 8080
use_tls = false
endpoint_path = "/ws/node"
node_id = "SRV-M-TBS-01"
station_name = "SRV-M-TBS-01"
site = "Main"
central_sds_routing = false
```
Die TBS verbindet sich in der verteilten LXC-Topologie **zum Node Gateway**, nicht direkt zum Control Room. `central_sds_routing` erst aktivieren, wenn SDS Router, Node Gateway und Fallback getestet sind.

## 5.8 Edge-Fallback
```toml
[edge_fallback]
enabled = true
enter_after_secs = 15
recover_after_secs = 20
unknown_service_is_available = false
service_matrix_lease_secs = 60
policy_cache_path = "/var/lib/flowstation/edge-policy-cache.json"
policy_cache_max_age_secs = 604800
keep_last_known_policy = true
event_spool_path = "/var/lib/flowstation/edge-event-spool.jsonl"
event_spool_max_entries = 10000
event_spool_max_bytes = 16777216
replay_batch_size = 128
required_services = ["subscriber-core", "group-core", "mobility-core", "call-control", "media-switch", "sds-router"]
```
`unknown_service_is_available=false` ist fail-closed. Eine veraltete oder unbekannte zentrale Dienstlage wird nicht als gesund angenommen. `keep_last_known_policy=true` verhindert, dass eine isolierte TBS unbemerkt von einer restriktiven Policy in ein offenes Netz fällt.

## 5.9 Primär- und Fallback-Konfiguration
Kann die Primärdatei nicht geladen werden, versucht die TBS `<datei>.fallback`. Das Dashboard zeigt den Fallbackbetrieb dauerhaft an. Nach einem solchen Start:
```bash
sudo journalctl -u tetra.service -b --no-pager | grep -iE 'config|fallback|parse|error'
diff -u /etc/netcore/config.toml.fallback /etc/netcore/config.toml
```
# 6. Proxmox-LXC vorbereiten
## 6.1 Gemeinsame LXC-Basis
Empfohlen: Debian 13, unprivilegierter LXC, statische IP, `nesting=1`, 2 Kerne, 2 GB RAM, 512 MB Swap, Autostart. Beispiel aus dem Repository:
```text
unprivileged: 1
features: nesting=1
memory: 2048
swap: 512
cores: 2
onboot: 1
startup: order=20,up=20,down=30
```

## 6.2 Pakete pro LXC
```bash
apt update
apt install -y ca-certificates curl git openssh-server build-essential pkg-config cmake clang jq
curl --proto =https --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source /root/.cargo/env
rustup update stable
systemctl enable --now ssh
```
Media Library benötigt zusätzlich `ffmpeg`; Recorder/Media Library optional `nfs-common`; IP Gateway zusätzlich `iproute2 nftables tcpdump dnsutils`.

## 6.3 SSH vom Deployment-Host
```bash
ssh-keygen -t ed25519 -f ~/.ssh/netcore-deploy
ssh-copy-id -i ~/.ssh/netcore-deploy.pub root@10.0.20.10
# für alle weiteren LXC wiederholen
```
Inventory-`ssh_options` gegebenenfalls um `-i ~/.ssh/netcore-deploy` ergänzen.

## 6.4 IP Gateway - TUN-Passthrough
Auf dem Proxmox-Host in `/etc/pve/lxc/<CTID>.conf`:
```text
lxc.cgroup2.devices.allow: c 10:200 rwm
lxc.mount.entry: /dev/net/tun dev/net/tun none bind,create=file
```
Danach Container neu starten und prüfen:
```bash
ls -l /dev/net/tun
ip tuntap add dev ntc-test mode tun
ip link del ntc-test
```

## 6.5 Recorder/Media Library NFS
NFS kann im LXC selbst gemountet oder als Proxmox-Mountpoint durchgereicht werden. Schreibrechte müssen zum jeweiligen Dienstbenutzer passen. Live-State bleibt lokal; nur Archivdaten gehen auf NFS.
# 7. Automatisches Deployment aller LXC
## 7.1 Korrigiertes Inventory verwenden
Im Repository-Inventory zeigt `control-room.config_target` auf `/etc/netcore/control-room.toml`, während Installer und systemd-Unit `/etc/netcore-control-room/control-room.toml` verwenden. Der mit diesem Guide gelieferte Inventory-Entwurf korrigiert diesen Pfad. Ohne Korrektur würde der Deployer eine Datei schreiben, die der Dienst nicht liest.

```bash
cp inventory.open-lab.corrected.example.toml deploy/open-lab/inventory.toml
${EDITOR:-nano} deploy/open-lab/inventory.toml
```
Anpassen: Hosts, SSH-Benutzer/Optionen, ggf. Ports, Remote-Quellpfad und Serviceauswahl.

## 7.2 Validieren und rendern
```bash
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml validate
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml plan
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml render
```
`render` ersetzt Dienst-URLs passend zum Inventory und erzeugt Servicekatalog, Hosts-Datei, Portliste und Abhängigkeitsgraph. Vor dem Deploy die gerenderten Configs auf falsche Loopback-Adressen, Netzbereiche und Betriebsmodi prüfen.

## 7.3 Dry Run und Deployment
```bash
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml apply --dry-run
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml apply
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml status
```
Der Deployer erstellt ein deterministisches Quellbundle ohne PDFs, `.git`, `target`, Caches oder Node-Module, kopiert es auf jedes Ziel, ruft den dienstspezifischen Installer auf, kopiert die gerenderte Konfiguration und startet in Abhängigkeitsreihenfolge.

## 7.4 Selektive Updates
Ein einzelner Dienst kann zusammen mit seinen transitiven Abhängigkeiten ausgewählt werden. Vor einem Update immer State-Backup des Dienstes und eine Kopie der aktiven Konfiguration anlegen. Das Deployment überschreibt keine Fach-State-Dateien, kann aber Binärdatei, Unit und Konfiguration aktualisieren.

## 7.5 Erstprüfung nach Deployment
```bash
for hp in 10.0.20.10:8080 10.0.20.11:8090 10.0.20.12:8100 10.0.20.13:8110 \
          10.0.20.14:8120 10.0.20.15:8130 10.0.20.16:8140 10.0.20.17:8150 \
          10.0.20.18:8160 10.0.20.19:8170 10.0.20.20:8180 10.0.20.21:8190 \
          10.0.20.22:8200 10.0.20.23:8220 10.0.20.24:8230 10.0.20.25:9010 \
          10.0.20.26:8210; do
  curl -fsS "http://$hp/health/live" >/dev/null && echo "OK $hp" || echo "FAIL $hp"
done
```
# 8. Manuelle Installation und Bedienung je Dienst
## 8.1 Gemeinsames Muster
Auf dem jeweiligen LXC:
```bash
cd /opt/netcore-tetra-swmi
sudo system-backend/<dienst>/install/install.sh
sudo editor <config-pfad>
sudo systemctl restart <unit>
sudo systemctl status <unit> --no-pager
curl -fsS http://127.0.0.1:<port>/health/live
curl -i http://127.0.0.1:<port>/health/ready
```
Readiness kann beim Start oder bei fehlender Abhängigkeit nachvollziehbar `503` liefern; Liveness muss bei laufendem Prozess `200` liefern.

## 8.2 node-gateway
**Zweck:** Zentraler Einstiegspunkt für TBS-WebSockets und normalisierter Transport zu den Backend-Diensten.

| Eigenschaft | Wert |
|---|---|
| WebUI/API | `http://10.0.20.10:8080/` |
| systemd | `netcore-node-gateway.service` |
| Konfiguration | `/etc/netcore/node-gateway.toml` |
| Installer | `system-backend/node-gateway/install/install.sh` |
| Abhängigkeiten | `keine` |
| State | `In-Memory; keine Fachdatenbank` |

**Vor dem ersten Start:**
- service_monitor.targets müssen auf alle 16 anderen LXCs zeigen
- Bind-Adresse und WebSocket-Pfade unverändert lassen, sofern kein Reverse Proxy eingesetzt wird

**Bedienung in der WebUI:**
- TBS-Nodes und Heartbeats prüfen
- Node-Ping auslösen
- stale oder doppelte Sessions trennen
- Core-Service-Matrix und Ereignisse kontrollieren

**Prüfbefehle:**
```bash
systemctl status netcore-node-gateway.service --no-pager
journalctl -u netcore-node-gateway.service -n 200 --no-pager
curl -fsS http://127.0.0.1:8080/health/live
curl -i http://127.0.0.1:8080/health/ready
curl -fsS http://127.0.0.1:8080/api/v1/status | jq .
```

## 8.3 mobility-core
**Zweck:** Teilnehmerlage, Serving-Node-Zuordnung und MM-Context-Transfers zwischen TBS.

| Eigenschaft | Wert |
|---|---|
| WebUI/API | `http://10.0.20.11:8090/` |
| systemd | `netcore-mobility-core.service` |
| Konfiguration | `/etc/netcore/mobility-core.toml` |
| Installer | `system-backend/mobility-core/install/install.sh` |
| Abhängigkeiten | `node-gateway` |
| State | `aktuell primär Laufzeitlage` |

**Vor dem ersten Start:**
- node_gateway.url auf ws://<NODE-GATEWAY>:8080/ws/backend setzen

**Bedienung in der WebUI:**
- Serving Node je ISSI prüfen
- Context Transfer starten und Phasen beobachten
- Transfers kontrolliert abbrechen
- RSSI/Energy-Saving-Lage prüfen

**Prüfbefehle:**
```bash
systemctl status netcore-mobility-core.service --no-pager
journalctl -u netcore-mobility-core.service -n 200 --no-pager
curl -fsS http://127.0.0.1:8090/health/live
curl -i http://127.0.0.1:8090/health/ready
curl -fsS http://127.0.0.1:8090/api/v1/status | jq .
```

## 8.4 subscriber-core
**Zweck:** Zentrale Teilnehmerprofile und Admission-Policy.

| Eigenschaft | Wert |
|---|---|
| WebUI/API | `http://10.0.20.12:8100/` |
| systemd | `netcore-subscriber-core.service` |
| Konfiguration | `/etc/netcore/subscriber-core.toml` |
| Installer | `system-backend/subscriber-core/install/install.sh` |
| Abhängigkeiten | `node-gateway` |
| State | `/var/lib/netcore-subscriber-core/subscribers.json` |

**Vor dem ersten Start:**
- access_policy.mode bewusst wählen
- bei allow_list zuerst mindestens einen Admin-/Testteilnehmer anlegen
- node_gateway.url setzen

**Bedienung in der WebUI:**
- Teilnehmer anlegen/ändern/sperren
- Allowlist oder Open Network wählen
- Import/Export durchführen
- TBS-Synchronisation prüfen

**Prüfbefehle:**
```bash
systemctl status netcore-subscriber-core.service --no-pager
journalctl -u netcore-subscriber-core.service -n 200 --no-pager
curl -fsS http://127.0.0.1:8100/health/live
curl -i http://127.0.0.1:8100/health/ready
curl -fsS http://127.0.0.1:8100/api/v1/status | jq .
```

## 8.5 group-core
**Zweck:** GSSI-Stammdaten, Mitgliedschaften, Affiliationen und DGNA.

| Eigenschaft | Wert |
|---|---|
| WebUI/API | `http://10.0.20.13:8110/` |
| systemd | `netcore-group-core.service` |
| Konfiguration | `/etc/netcore/group-core.toml` |
| Installer | `system-backend/group-core/install/install.sh` |
| Abhängigkeiten | `node-gateway, subscriber-core` |
| State | `/var/lib/netcore-group-core/groups.json` |

**Vor dem ersten Start:**
- allow_unlisted_groups=false ist der sichere Ausgangspunkt
- node_gateway.url setzen

**Bedienung in der WebUI:**
- Gruppenprofile anlegen
- Mitgliedschaften pflegen
- Affiliationen beobachten
- DGNA Attach/Detach auslösen

**Prüfbefehle:**
```bash
systemctl status netcore-group-core.service --no-pager
journalctl -u netcore-group-core.service -n 200 --no-pager
curl -fsS http://127.0.0.1:8110/health/live
curl -i http://127.0.0.1:8110/health/ready
curl -fsS http://127.0.0.1:8110/api/v1/status | jq .
```

## 8.6 call-control
**Zweck:** Netzweite logische Gruppen-/Individualrufe, Call Legs, Floor und Restore.

| Eigenschaft | Wert |
|---|---|
| WebUI/API | `http://10.0.20.14:8120/` |
| systemd | `netcore-call-control.service` |
| Konfiguration | `/etc/netcore/call-control.toml` |
| Installer | `system-backend/call-control/install/install.sh` |
| Abhängigkeiten | `node-gateway, subscriber-core, group-core, mobility-core` |
| State | `/var/lib/netcore-call-control/calls.json` |

**Vor dem ersten Start:**
- node_gateway.url setzen
- Ruf- und Restore-Timeouts an Labornetz anpassen

**Bedienung in der WebUI:**
- Rufe starten/beenden
- Floor Holder und Queue beobachten
- Operator-Floor nur gezielt erzwingen
- Restore-Vorgänge prüfen

**Prüfbefehle:**
```bash
systemctl status netcore-call-control.service --no-pager
journalctl -u netcore-call-control.service -n 200 --no-pager
curl -fsS http://127.0.0.1:8120/health/live
curl -i http://127.0.0.1:8120/health/ready
curl -fsS http://127.0.0.1:8120/api/v1/status | jq .
```

## 8.7 media-switch
**Zweck:** Routing bereits codierter 35-Byte-TETRA-Sprachframes zwischen TBS-Call-Legs.

| Eigenschaft | Wert |
|---|---|
| WebUI/API | `http://10.0.20.15:8130/` |
| systemd | `netcore-media-switch.service` |
| Konfiguration | `/etc/netcore/media-switch.toml` |
| Installer | `system-backend/media-switch/install/install.sh` |
| Abhängigkeiten | `node-gateway, call-control` |
| State | `zeitkritische Laufzeitdaten im Speicher` |

**Vor dem ersten Start:**
- node_gateway.url und call_control.url setzen
- Recorder-Tap-Historie passend dimensionieren

**Bedienung in der WebUI:**
- Sessions und Jitter-Puffer beobachten
- Streams stummschalten
- Puffer nur zur Fehlersuche leeren
- Testframe-Injection ausschließlich im Labor nutzen

**Prüfbefehle:**
```bash
systemctl status netcore-media-switch.service --no-pager
journalctl -u netcore-media-switch.service -n 200 --no-pager
curl -fsS http://127.0.0.1:8130/health/live
curl -i http://127.0.0.1:8130/health/ready
curl -fsS http://127.0.0.1:8130/api/v1/status | jq .
```

## 8.8 recorder
**Zweck:** Passive, verlustfreie Aufzeichnung von TACELP-Frames außerhalb des Rufpfads.

| Eigenschaft | Wert |
|---|---|
| WebUI/API | `http://10.0.20.16:8140/` |
| systemd | `netcore-recorder.service` |
| Konfiguration | `/etc/netcore/recorder.toml` |
| Installer | `system-backend/recorder/install/install.sh` |
| Abhängigkeiten | `media-switch` |
| State | `/var/lib/netcore-recorder/recordings` |

**Vor dem ersten Start:**
- media_switch.tap_url/sessions_url setzen
- Storage und freien Speicher prüfen
- NFS nur als separates Archiv verwenden

**Bedienung in der WebUI:**
- aktive Aufnahmen beobachten
- Integrität prüfen
- Retention/Legal Hold setzen
- TAR exportieren oder Aufnahme löschen

**Prüfbefehle:**
```bash
systemctl status netcore-recorder.service --no-pager
journalctl -u netcore-recorder.service -n 200 --no-pager
curl -fsS http://127.0.0.1:8140/health/live
curl -i http://127.0.0.1:8140/health/ready
curl -fsS http://127.0.0.1:8140/api/v1/status | jq .
```

## 8.9 sds-router
**Zweck:** Zentrale SDS-/Statusvermittlung, Store-and-forward und Anwendungsrouten.

| Eigenschaft | Wert |
|---|---|
| WebUI/API | `http://10.0.20.17:8150/` |
| systemd | `netcore-sds-router.service` |
| Konfiguration | `/etc/netcore/sds-router.toml` |
| Installer | `system-backend/sds-router/install/install.sh` |
| Abhängigkeiten | `node-gateway, subscriber-core, group-core, mobility-core` |
| State | `/var/lib/netcore-sds-router/messages.json` |

**Vor dem ersten Start:**
- node_gateway.url setzen
- TBS central_sds_routing erst nach Test aktivieren
- TTL und Payloadgrenzen prüfen

**Bedienung in der WebUI:**
- Nachrichten senden/suchen
- Retry/Requeue/Cancel durchführen
- Routen verwalten
- Application-Outbox quittieren

**Prüfbefehle:**
```bash
systemctl status netcore-sds-router.service --no-pager
journalctl -u netcore-sds-router.service -n 200 --no-pager
curl -fsS http://127.0.0.1:8150/health/live
curl -i http://127.0.0.1:8150/health/ready
curl -fsS http://127.0.0.1:8150/api/v1/status | jq .
```

## 8.10 packet-core
**Zweck:** PDP-/NSAPI-State-Machine, IPv4-Leases, Mobility Anchoring, Fragmente und Flow Control.

| Eigenschaft | Wert |
|---|---|
| WebUI/API | `http://10.0.20.18:8160/` |
| systemd | `netcore-packet-core.service` |
| Konfiguration | `/etc/netcore/packet-core.toml` |
| Installer | `system-backend/packet-core/install/install.sh` |
| Abhängigkeiten | `node-gateway, subscriber-core, mobility-core` |
| State | `/var/lib/netcore-packet-core/state.json` |

**Vor dem ersten Start:**
- zunächst packet.mode=shadow
- node_gateway.url setzen
- Adresspool muss zum IP Gateway passen

**Bedienung in der WebUI:**
- Kontexte/NSAPI prüfen
- Wake/End-of-Data/Modify/Deactivate auslösen
- Bearers und Reassemblies beobachten
- Downlink-N-PDUs kontrollieren

**Prüfbefehle:**
```bash
systemctl status netcore-packet-core.service --no-pager
journalctl -u netcore-packet-core.service -n 200 --no-pager
curl -fsS http://127.0.0.1:8160/health/live
curl -i http://127.0.0.1:8160/health/ready
curl -fsS http://127.0.0.1:8160/api/v1/status | jq .
```

## 8.11 ip-gateway
**Zweck:** Layer-3-Übergang zwischen Packet Core und normalen IPv4-Netzen.

| Eigenschaft | Wert |
|---|---|
| WebUI/API | `http://10.0.20.19:8170/` |
| systemd | `netcore-ip-gateway.service` |
| Konfiguration | `/etc/netcore/ip-gateway.toml` |
| Installer | `system-backend/ip-gateway/install/install.sh` |
| Abhängigkeiten | `packet-core` |
| State | `/var/lib/netcore-ip-gateway/state.json und captures/` |

**Vor dem ersten Start:**
- /dev/net/tun im LXC durchreichen
- zunächst interface.mode=shadow
- packet_core.url und TETRA-IP-Netz konsistent setzen
- vor authoritative nftables-Regeln prüfen

**Bedienung in der WebUI:**
- Kernel-Plan prüfen
- Routen/NAT/Firewall verwalten
- DNS/Testserver nutzen
- PCAP-Captures starten und stoppen

**Prüfbefehle:**
```bash
systemctl status netcore-ip-gateway.service --no-pager
journalctl -u netcore-ip-gateway.service -n 200 --no-pager
curl -fsS http://127.0.0.1:8170/health/live
curl -i http://127.0.0.1:8170/health/ready
curl -fsS http://127.0.0.1:8170/api/v1/status | jq .
```
**Authoritative-Check:** Vor Umschaltung `GET /api/v1/kernel/plan` prüfen. Danach TUN, Routen und nftables separat mit `ip addr`, `ip route` und `nft list ruleset` kontrollieren.

## 8.12 security-core
**Zweck:** Security-Class-Policy, Lab-Authentisierung, DCK-Kontexte, Sperren und Audit.

| Eigenschaft | Wert |
|---|---|
| WebUI/API | `http://10.0.20.20:8180/` |
| systemd | `netcore-security-core.service` |
| Konfiguration | `/etc/netcore/security-core.toml` |
| Installer | `system-backend/security-core/install/install.sh` |
| Abhängigkeiten | `node-gateway, subscriber-core` |
| State | `/var/lib/netcore-security-core/state.json; Seed separat` |

**Vor dem ersten Start:**
- zunächst policy.operating_mode=shadow
- Lab-Provider nicht als produktive TETRA-Kryptografie betrachten
- kein stilles Downgrade konfigurieren

**Bedienung in der WebUI:**
- Profile und Security Class verwalten
- Challenges/Alarme beobachten
- Teilnehmer/Geräte sperren oder freigeben
- Edge-Aktionen quittieren

**Prüfbefehle:**
```bash
systemctl status netcore-security-core.service --no-pager
journalctl -u netcore-security-core.service -n 200 --no-pager
curl -fsS http://127.0.0.1:8180/health/live
curl -i http://127.0.0.1:8180/health/ready
curl -fsS http://127.0.0.1:8180/api/v1/status | jq .
```

## 8.13 kmf
**Zweck:** Lifecycle für CCK/GCK/SCK, Rotation, Crypto Periods und OTAR-Orchestrierung.

| Eigenschaft | Wert |
|---|---|
| WebUI/API | `http://10.0.20.21:8190/` |
| systemd | `netcore-kmf.service` |
| Konfiguration | `/etc/netcore/kmf.toml` |
| Installer | `system-backend/kmf/install/install.sh` |
| Abhängigkeiten | `security-core` |
| State | `/var/lib/netcore-kmf/state.json + vault.json + master.key` |

**Vor dem ersten Start:**
- zunächst policy.operating_mode=shadow
- Master-Key separat sichern
- Managementnetz besonders restriktiv halten

**Bedienung in der WebUI:**
- Keys erzeugen/rotieren/aktivieren/widerrufen
- Vier-Augen-Jobs freigeben
- OTAR-Zustellungen und ACKs prüfen
- Backups erzeugen

**Prüfbefehle:**
```bash
systemctl status netcore-kmf.service --no-pager
journalctl -u netcore-kmf.service -n 200 --no-pager
curl -fsS http://127.0.0.1:8190/health/live
curl -i http://127.0.0.1:8190/health/ready
curl -fsS http://127.0.0.1:8190/api/v1/status | jq .
```
**Backup-Hinweis:** `master.key` getrennt vom normalen KMF-Backup sichern. Ohne Master-Key ist ein Vault-Backup nicht nutzbar; zusammen am selben Ort wäre die Trennung wirkungslos.

## 8.14 transit
**Zweck:** NetCore-native Regionalvermittlung mit Peers, Routen, Sessions und Failover.

| Eigenschaft | Wert |
|---|---|
| WebUI/API | `http://10.0.20.22:8200/` |
| systemd | `netcore-transit.service` |
| Konfiguration | `/etc/netcore/transit.toml` |
| Installer | `system-backend/transit/install/install.sh` |
| Abhängigkeiten | `mobility-core, call-control, media-switch, sds-router` |
| State | `/var/lib/netcore-transit/state.json` |

**Vor dem ersten Start:**
- region_id/swmi_id/advertised_endpoint eindeutig setzen
- zunächst operating_mode=shadow
- kein ETSI-ISI behaupten

**Bedienung in der WebUI:**
- Regionen/Peers verwalten
- Routen und Erreichbarkeit prüfen
- Sessions/Queues beobachten
- Failover kontrolliert auslösen

**Prüfbefehle:**
```bash
systemctl status netcore-transit.service --no-pager
journalctl -u netcore-transit.service -n 200 --no-pager
curl -fsS http://127.0.0.1:8200/health/live
curl -i http://127.0.0.1:8200/health/ready
curl -fsS http://127.0.0.1:8200/api/v1/status | jq .
```

## 8.15 application-gateway
**Zweck:** Adapter- und Workflow-Gateway für externe Anwendungen, Webhooks, Vorlagen und TTS.

| Eigenschaft | Wert |
|---|---|
| WebUI/API | `http://10.0.20.23:8220/` |
| systemd | `netcore-application-gateway.service` |
| Konfiguration | `/etc/netcore/application-gateway.toml` |
| Installer | `system-backend/application-gateway/install/install.sh` |
| Abhängigkeiten | `sds-router` |
| State | `state.json, secrets.json, spool, backups` |

**Vor dem ersten Start:**
- interne URLs rendern
- zunächst runtime.operating_mode=shadow
- Secrets ausschließlich über secrets.json/WebUI pflegen

**Bedienung in der WebUI:**
- Connectoren testen
- Routen/Vorlagen pflegen
- manuelle Dispatches auslösen
- TTS-Jobs publizieren
- Dead Letters erneut zustellen

**Prüfbefehle:**
```bash
systemctl status netcore-application-gateway.service --no-pager
journalctl -u netcore-application-gateway.service -n 200 --no-pager
curl -fsS http://127.0.0.1:8220/health/live
curl -i http://127.0.0.1:8220/health/ready
curl -fsS http://127.0.0.1:8220/api/v1/status | jq .
```

## 8.16 media-library
**Zweck:** Zentrale Medienablage, Vorschau, Freigabe, TACELP-Cache und Playout.

| Eigenschaft | Wert |
|---|---|
| WebUI/API | `http://10.0.20.24:8230/` |
| systemd | `netcore-media-library.service` |
| Konfiguration | `/etc/netcore/media-library.toml` |
| Installer | `system-backend/media-library/install/install.sh` |
| Abhängigkeiten | `media-switch, recorder, application-gateway` |
| State | `/var/lib/netcore-media-library/assets + state.json` |

**Vor dem ersten Start:**
- Abhängigkeits-URLs setzen
- ffmpeg installieren
- zunächst runtime.operating_mode=shadow
- TACELP-Encoder/Decoder bei Bedarf konfigurieren

**Bedienung in der WebUI:**
- Assets hochladen/importieren
- Vorschau prüfen
- Assets freigeben/ablehnen
- Playout-Jobs in bestehende Sessions starten
- Archivkopien prüfen

**Prüfbefehle:**
```bash
systemctl status netcore-media-library.service --no-pager
journalctl -u netcore-media-library.service -n 200 --no-pager
curl -fsS http://127.0.0.1:8230/health/live
curl -i http://127.0.0.1:8230/health/ready
curl -fsS http://127.0.0.1:8230/api/v1/status | jq .
```

## 8.17 control-room
**Zweck:** Zentrale Lage-, Operator-, Incident- und Schichtbuchebene.

| Eigenschaft | Wert |
|---|---|
| WebUI/API | `http://10.0.20.25:9010/` |
| systemd | `netcore-control-room.service` |
| Konfiguration | `/etc/netcore-control-room/control-room.toml` |
| Installer | `system-backend/control-room/install/install.sh` |
| Abhängigkeiten | `node-gateway, subscriber-core, group-core, mobility-core, call-control, media-switch, recorder, sds-router, packet-core, ip-gateway, security-core, kmf, transit, application-gateway, media-library` |
| State | `control-room.sqlite3 + operations.json` |

**Vor dem ersten Start:**
- alle Service-base_urls setzen
- Config-Pfad /etc/netcore-control-room/control-room.toml verwenden
- Open-Lab-Rechte bedenken

**Bedienung in der WebUI:**
- Gesamtlage und kritische Dienste prüfen
- Incidents quittieren/lösen
- Schichtbuch führen
- Schnellaktionen Kick/Clear Emergency/DGNA verwenden
- Fach-WebUIs öffnen

**Prüfbefehle:**
```bash
systemctl status netcore-control-room.service --no-pager
journalctl -u netcore-control-room.service -n 200 --no-pager
curl -fsS http://127.0.0.1:9010/health/live
curl -i http://127.0.0.1:9010/health/ready
curl -fsS http://127.0.0.1:9010/api/v1/status | jq .
```
**Pfadhinweis:** Dieser Dienst ist die Ausnahme: aktive Konfiguration liegt unter `/etc/netcore-control-room/control-room.toml`.

## 8.18 observability
**Zweck:** Zentrale Metriken, Logs, Traces, Alerts, Silences und Diagnosepakete.

| Eigenschaft | Wert |
|---|---|
| WebUI/API | `http://10.0.20.26:8210/` |
| systemd | `netcore-observability.service` |
| Konfiguration | `/etc/netcore/observability.toml` |
| Installer | `system-backend/observability/install/install.sh` |
| Abhängigkeiten | `node-gateway, subscriber-core, group-core, mobility-core, call-control, media-switch, recorder, sds-router, packet-core, ip-gateway, security-core, kmf, transit, application-gateway, media-library, control-room` |
| State | `/var/lib/netcore-observability/state.json + diagnostics/` |

**Vor dem ersten Start:**
- alle Targets korrekt rendern
- Retention an Disk anpassen
- klassischen Stack nur installieren, wenn Binaries vorhanden sind

**Bedienung in der WebUI:**
- Scrape Targets testen
- Metrikserien und Logs durchsuchen
- Alarme quittieren
- Silences setzen
- Diagnosepakete erstellen

**Prüfbefehle:**
```bash
systemctl status netcore-observability.service --no-pager
journalctl -u netcore-observability.service -n 200 --no-pager
curl -fsS http://127.0.0.1:8210/health/live
curl -i http://127.0.0.1:8210/health/ready
curl -fsS http://127.0.0.1:8210/api/v1/status | jq .
```

# 9. Konfigurationsreferenz der LXC-Dienste
Die folgenden Tabellen listen die eindeutigen Schlüssel der mitgelieferten Beispielkonfigurationen. Bei wiederholten TOML-Arrays wie `[[connectors]]`, `[[services]]` oder `[[targets]]` wird das Schema nur einmal dargestellt. IP-Adressen aus den Templates sind Beispiele; der Deployer rendert sie anhand des Inventory.

## 9.1 node-gateway - `system-backend/node-gateway/config/node-gateway.example.toml`
### `server`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `bind` | `"0.0.0.0:8080"` | Open lab listener. Put this LXC only into an isolated management/test VLAN. |
| `node_path` | `"/ws/node"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `backend_path` | `"/ws/backend"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `history_limit` | `1000` | Maximale Zahl im Arbeitsspeicher sichtbarer Historieneinträge. |
| `stale_after_secs` | `20` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `hello_timeout_secs` | `10` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `application_ping_secs` | `15` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `security`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `mode` | `"open_lab"` | This package deliberately supports only open_lab. No token fields exist and no user/password login is performed. |
| `allow_remote_management` | `true` | Erlaubt Managementzugriffe über die Bind-Adresse. |

### `limits`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `max_message_bytes` | `1048576` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_http_body_bytes` | `1048576` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `service_monitor`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `enabled` | `true` | Health is evaluated by service, not by Internet reachability. The TBS receives this matrix over its existing Node Gateway WebSocket and chooses central or local authority per feature. |
| `interval_secs` | `5` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `timeout_ms` | `1500` | Timeout in Millisekunden. |
| `failure_threshold` | `2` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `recovery_threshold` | `2` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `service_monitor.targets[]`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `name` | `"mobility-core"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `url` | `"http://10.0.20.11:8090/health/ready"` | URL der Abhängigkeit. |
| `critical_for_edge` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `fallback_mode` | `"local_registration_and_location_area"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

## 9.2 mobility-core - `system-backend/mobility-core/config/mobility-core.example.toml`
### `server`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `bind` | `"0.0.0.0:8090"` | WebUI and REST API. Use only inside the isolated test/management network. |
| `history_limit` | `2000` | Maximale Zahl im Arbeitsspeicher sichtbarer Historieneinträge. |
| `transfer_timeout_secs` | `45` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `node_gateway`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `url` | `"ws://10.0.1.30:8080/ws/backend"` | Backend WebSocket of the Node Gateway LXC. |
| `reconnect_secs` | `5` | Zeit zwischen Wiederverbindungsversuchen. |

### `security`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `mode` | `"open_lab"` | This package deliberately supports only open_lab. There are no token, password or TLS fields. |
| `allow_remote_management` | `true` | Erlaubt Managementzugriffe über die Bind-Adresse. |

### `limits`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `max_body_bytes` | `1048576` | Maximal akzeptierte HTTP-Request-Größe. |
| `max_transfers` | `10000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_subscribers` | `100000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

## 9.3 subscriber-core - `system-backend/subscriber-core/config/subscriber-core.example.toml`
### `server`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `bind` | `"0.0.0.0:8100"` | Bind-Adresse und Port des Dienstes. |
| `history_limit` | `2000` | Maximale Zahl im Arbeitsspeicher sichtbarer Historieneinträge. |

### `node_gateway`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `url` | `"ws://127.0.0.1:8080/ws/backend"` | URL der Abhängigkeit. |
| `reconnect_secs` | `5` | Zeit zwischen Wiederverbindungsversuchen. |

### `storage`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `database_path` | `"/var/lib/netcore-subscriber-core/subscribers.json"` | Primäre persistente Zustandsdatei. |
| `backup_path` | `"/var/lib/netcore-subscriber-core/subscribers.json.bak"` | Lokale Backup-/Fallback-Datei. |

### `access_policy`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `mode` | `"allow_list"` | allow_list: only enabled + registration_allowed profiles may register. open_network: all ISSIs may register; profiles remain metadata only. |
| `auto_sync` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `disconnect_unauthorized` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `sync_timeout_secs` | `30` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `security`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `mode` | `"open_lab"` | Betriebs- oder Sicherheitsmodus. |
| `allow_remote_management` | `true` | Erlaubt Managementzugriffe über die Bind-Adresse. |

### `limits`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `max_body_bytes` | `2097152` | Maximal akzeptierte HTTP-Request-Größe. |
| `max_subscribers` | `100000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_groups_per_subscriber` | `1024` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

## 9.4 group-core - `system-backend/group-core/config/group-core.example.toml`
### `server`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `bind` | `"0.0.0.0:8110"` | Bind-Adresse und Port des Dienstes. |
| `history_limit` | `2000` | Maximale Zahl im Arbeitsspeicher sichtbarer Historieneinträge. |

### `node_gateway`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `url` | `"ws://10.0.1.XX:8080/ws/backend"` | URL der Abhängigkeit. |
| `reconnect_secs` | `5` | Zeit zwischen Wiederverbindungsversuchen. |

### `storage`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `database_path` | `"/var/lib/netcore-group-core/groups.json"` | Primäre persistente Zustandsdatei. |
| `backup_path` | `"/var/lib/netcore-group-core/groups.json.bak"` | Lokale Backup-/Fallback-Datei. |

### `policy`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `allow_unlisted_groups` | `false` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `enforce_memberships` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `reconcile_registered` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `auto_sync` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `sync_timeout_secs` | `30` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `dgna_timeout_secs` | `30` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `security`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `mode` | `"open_lab"` | Betriebs- oder Sicherheitsmodus. |
| `allow_remote_management` | `true` | Erlaubt Managementzugriffe über die Bind-Adresse. |

### `limits`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `max_body_bytes` | `2097152` | Maximal akzeptierte HTTP-Request-Größe. |
| `max_groups` | `65536` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_memberships` | `1000000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

## 9.5 call-control - `system-backend/call-control/config/call-control.example.toml`
### `server`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `bind` | `"0.0.0.0:8120"` | Bind-Adresse und Port des Dienstes. |
| `history_limit` | `2000` | Maximale Zahl im Arbeitsspeicher sichtbarer Historieneinträge. |

### `node_gateway`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `url` | `"ws://10.0.1.XX:8080/ws/backend"` | URL der Abhängigkeit. |
| `reconnect_secs` | `5` | Zeit zwischen Wiederverbindungsversuchen. |

### `storage`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `database_path` | `"/var/lib/netcore-call-control/calls.json"` | Primäre persistente Zustandsdatei. |
| `backup_path` | `"/var/lib/netcore-call-control/calls.json.bak"` | Lokale Backup-/Fallback-Datei. |

### `calls`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `command_timeout_secs` | `30` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `restore_timeout_secs` | `45` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `reconcile_interval_secs` | `2` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `auto_target_affiliated_nodes` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `release_partial_start_on_failure` | `false` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `allow_operator_force_floor` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `security`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `mode` | `"open_lab"` | Betriebs- oder Sicherheitsmodus. |
| `allow_remote_management` | `true` | Erlaubt Managementzugriffe über die Bind-Adresse. |

### `limits`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `max_body_bytes` | `2097152` | Maximal akzeptierte HTTP-Request-Größe. |
| `max_calls` | `100000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_legs_per_call` | `1024` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_pending_commands` | `20000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

## 9.6 media-switch - `system-backend/media-switch/config/media-switch.example.toml`
### `server`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `bind` | `"0.0.0.0:8130"` | Bind-Adresse und Port des Dienstes. |
| `history_limit` | `2000` | Maximale Zahl im Arbeitsspeicher sichtbarer Historieneinträge. |

### `node_gateway`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `url` | `"ws://10.0.1.20:8080/ws/backend"` | URL der Abhängigkeit. |
| `reconnect_secs` | `2` | Zeit zwischen Wiederverbindungsversuchen. |

### `call_control`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `url` | `"http://10.0.1.24:8120/api/v1/calls"` | Startup- und Fallback-Snapshot der Rufstruktur. |
| `events_url` | `"ws://10.0.1.24:8120/ws/media"` | Ereignisgesteuerter Call-/Leg-/Floor-Pfad zum Media Switch. |
| `route_ready_url` | `"http://10.0.1.24:8120/api/v1/media/route-ready"` | Bestätigung des Media Switch, dass alle Ziel-Legs routbar sind. |
| `reconcile_secs` | `15` | Nur Sicherheitsabgleich; nicht mehr Bestandteil des Sprachpfads. |
| `reconnect_secs` | `1` | Zeit zwischen WebSocket-Wiederverbindungsversuchen. |
| `request_timeout_secs` | `2` | Timeout für Snapshot- und RouteReady-HTTP-Aufrufe. |

### `media`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `frame_duration_ms` | `60` | Ein gepackter TETRA-TCH/S-Sprachframe pro 60-ms-Rahmen. |
| `jitter_buffer_frames` | `2` | Startwert von 120 ms für den adaptiven Jitterpuffer. |
| `min_jitter_buffer_frames` | `1` | Untergrenze von 60 ms bei stabilem Transport. |
| `max_jitter_buffer_frames` | `12` | Harte Obergrenze pro Zielstream. |
| `adaptive_jitter` | `true` | Regelt den Puffer anhand der gemessenen Ankunftsschwankung. |
| `adaptive_jitter_up_threshold_ms` | `18` | Abweichung, ab der der Zielpuffer erhöht wird. |
| `adaptive_jitter_down_stable_frames` | `120` | Stabile Frames bis zur vorsichtigen Verringerung. |
| `cold_start_buffer_frames` | `5` | Schützt die ersten 300 ms eines kalten Rufes bis RouteReady. |
| `cold_start_buffer_max_age_ms` | `600` | Maximales Alter noch replay-fähiger Startframes. |
| `session_idle_secs` | `30` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_frames_per_tick` | `256` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `allow_same_leg_loopback` | `false` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `tap_history_frames` | `256` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `recorder_tap_history_frames` | `20000` | Replay-fähiger Vollframe-Tap für den Recorder. 20.000 Frames entsprechen bei einem einzelnen aktiven Sprecher ungefähr 20 Minuten Roh-Audio. |

### `security`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `mode` | `"open_lab"` | Betriebs- oder Sicherheitsmodus. |
| `allow_remote_management` | `true` | Erlaubt Managementzugriffe über die Bind-Adresse. |

### `limits`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `max_body_bytes` | `1048576` | Maximal akzeptierte HTTP-Request-Größe. |
| `max_sessions` | `10000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_streams` | `50000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_pending_frames` | `100000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

## 9.7 recorder - `system-backend/recorder/config/recorder.example.toml`
### `server`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `bind` | `"0.0.0.0:8140"` | Bind-Adresse und Port des Dienstes. |
| `history_limit` | `2000` | Maximale Zahl im Arbeitsspeicher sichtbarer Historieneinträge. |

### `media_switch`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `tap_url` | `"http://10.0.1.25:8130/api/v1/recorder/taps"` | Replay-fähiger Vollframe-Tap des Media Switch. |
| `sessions_url` | `"http://10.0.1.25:8130/api/v1/sessions"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `poll_interval_ms` | `100` | Polling-Intervall in Millisekunden. |
| `session_reconcile_ms` | `1000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `request_timeout_secs` | `3` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `batch_limit` | `500` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `storage`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `root` | `"/var/lib/netcore-recorder/recordings"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `export_root` | `"/var/lib/netcore-recorder/exports"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `frame_duration_ms` | `60` | Ein gepackter TETRA Speech Service 0 Frame umfasst 35 Bytes und 60 ms. |
| `session_absent_grace_secs` | `3` | Erst nach dieser Zeit ohne Media-Switch-Session wird eine Aufnahme finalisiert. |
| `maximum_idle_secs` | `600` | Sicherheitsnetz für Calls, die nie sauber beendet gemeldet werden. |
| `default_retention_days` | `30` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `retention_scan_secs` | `60` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `fsync_every_frames` | `50` | Daten, Index und aktives Manifest werden spätestens alle n Frames synchronisiert. |
| `minimum_free_space_mb` | `512` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `security`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `mode` | `"open_lab"` | Betriebs- oder Sicherheitsmodus. |
| `allow_remote_management` | `true` | Erlaubt Managementzugriffe über die Bind-Adresse. |
| `allow_delete` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `limits`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `max_body_bytes` | `1048576` | Maximal akzeptierte HTTP-Request-Größe. |
| `max_active_recordings` | `1000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_recordings` | `100000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

## 9.8 sds-router - `system-backend/sds-router/config/sds-router.example.toml`
### `server`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `bind` | `"0.0.0.0:8150"` | Bind-Adresse und Port des Dienstes. |
| `history_limit` | `4000` | Maximale Zahl im Arbeitsspeicher sichtbarer Historieneinträge. |

### `node_gateway`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `url` | `"ws://10.0.1.20:8080/ws/backend"` | URL der Abhängigkeit. |
| `reconnect_secs` | `5` | Zeit zwischen Wiederverbindungsversuchen. |

### `storage`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `database_path` | `"/var/lib/netcore-sds-router/messages.json"` | Primäre persistente Zustandsdatei. |
| `backup_path` | `"/var/lib/netcore-sds-router/messages.json.bak"` | Lokale Backup-/Fallback-Datei. |

### `routing`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `default_ttl_secs` | `300` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_ttl_secs` | `86400` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_attempts` | `5` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `initial_retry_secs` | `2` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_retry_secs` | `60` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `dedupe_window_secs` | `30` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `presence_timeout_secs` | `90` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `authoritative_ingress` | `true` | true: ordinary SDS/STATUS ingress is decided centrally. Safety-critical local handlers in the TBS stay local. |

### `security`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `mode` | `"open_lab"` | Betriebs- oder Sicherheitsmodus. |
| `allow_remote_management` | `true` | Erlaubt Managementzugriffe über die Bind-Adresse. |
| `mask_payload_in_list` | `false` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `limits`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `max_body_bytes` | `2097152` | Maximal akzeptierte HTTP-Request-Größe. |
| `max_payload_bytes` | `2048` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_messages` | `100000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_routes` | `4096` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

## 9.9 packet-core - `system-backend/packet-core/config/packet-core.example.toml`
### `server`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `bind` | `"0.0.0.0:8160"` | Bind-Adresse und Port des Dienstes. |
| `history_limit` | `5000` | Maximale Zahl im Arbeitsspeicher sichtbarer Historieneinträge. |

### `node_gateway`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `url` | `"ws://127.0.0.1:8080/ws/backend"` | URL der Abhängigkeit. |
| `reconnect_secs` | `5` | Zeit zwischen Wiederverbindungsversuchen. |

### `storage`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `database_path` | `"/var/lib/netcore-packet-core/state.json"` | Primäre persistente Zustandsdatei. |
| `backup_path` | `"/var/lib/netcore-packet-core/state.json.bak"` | Lokale Backup-/Fallback-Datei. |

### `packet`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `mode` | `"shadow"` | shadow: lokale TBS entscheidet, Core rechnet parallel authoritative: Edge API liefert verbindliche Core-Aktionen |
| `ready_timer_secs` | `5` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `standby_timer_secs` | `300` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `response_wait_secs` | `10` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `context_ready_secs` | `30` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `default_mtu` | `1500` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_n_pdu_bytes` | `65535` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_contexts_per_subscriber` | `14` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_total_contexts` | `4096` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `strict_source_address` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `preserve_context_on_node_loss` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `address_pool`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `network_prefix` | `[10, 44, 0]` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `first_host` | `2` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `last_host` | `254` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `gateway` | `"10.44.0.1"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `allow_static` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `fragmentation`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `timeout_secs` | `30` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_datagrams` | `256` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_total_bytes` | `8388608` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_fragments_per_datagram` | `512` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `reject_overlaps` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `flow_control`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `max_queue_packets_per_context` | `64` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_queue_bytes_per_context` | `262144` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `queue_ttl_secs` | `30` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `action_retry_secs` | `5` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `action_max_attempts` | `5` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `security`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `mode` | `"open_lab"` | Betriebs- oder Sicherheitsmodus. |
| `allow_remote_management` | `true` | Erlaubt Managementzugriffe über die Bind-Adresse. |
| `expose_payloads` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `limits`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `max_body_bytes` | `1048576` | Maximal akzeptierte HTTP-Request-Größe. |
| `max_events` | `10000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_actions` | `10000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_payload_bytes` | `65535` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

## 9.10 ip-gateway - `system-backend/ip-gateway/config/ip-gateway.example.toml`
### `server`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `bind` | `"0.0.0.0:8170"` | Bind-Adresse und Port des Dienstes. |

### `packet_core`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `url` | `"http://127.0.0.1:8160"` | URL der Abhängigkeit. |
| `poll_interval_ms` | `250` | Polling-Intervall in Millisekunden. |
| `context_refresh_ms` | `1000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `request_timeout_ms` | `2000` | HTTP-Timeout in Millisekunden. |
| `outbox_batch` | `250` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `storage`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `database_path` | `"/var/lib/netcore-ip-gateway/state.json"` | Primäre persistente Zustandsdatei. |
| `backup_path` | `"/var/lib/netcore-ip-gateway/state.json.bak"` | Lokale Backup-/Fallback-Datei. |

### `interface`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `mode` | `"shadow"` | shadow: Packet Core, Regeln und Kernel-Plan beobachten, aber kein TUN öffnen. authoritative: /dev/net/tun öffnen, IP-Pakete transportieren und Kernelregeln anwenden. |
| `name` | `"ntc-tun0"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `address` | `"10.0.0.1/24"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `network` | `"10.0.0.0/24"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `mtu` | `480` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `owner_user` | `"netcore"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `delete_on_exit` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `routing`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `enable_ipv4_forwarding` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `reconcile_interval_secs` | `5` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `install_connected_route` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `nat`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `enabled` | `true` | Funktion aktivieren oder deaktivieren. |
| `masquerade` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `egress_interface` | `"eth0"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `firewall`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `enabled` | `true` | Funktion aktivieren oder deaktivieren. |
| `default_forward_policy` | `"drop"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `allow_established` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `allow_general_internet` | `true` | Im isolierten Labor standardmäßig freie Fahrt nach außen; vor Produktivbetrieb einschränken. |
| `allow_icmp` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `log_drops` | `false` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `dns`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `enabled` | `true` | Funktion aktivieren oder deaktivieren. |
| `bind` | `"0.0.0.0:53"` | Bind-Adresse und Port des Dienstes. |
| `upstream` | `"1.1.1.1:53"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `local_domain` | `"netcore.test"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `ttl_secs` | `30` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `query_timeout_ms` | `2000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `test_server`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `enabled` | `true` | Funktion aktivieren oder deaktivieren. |
| `bind` | `"0.0.0.0:8088"` | Bind-Adresse und Port des Dienstes. |
| `udp_echo_bind` | `"0.0.0.0:7007"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `capture`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `directory` | `"/var/lib/netcore-ip-gateway/captures"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_captures` | `64` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_file_bytes` | `268435456` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `snaplen` | `65535` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `security`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `mode` | `"open_lab"` | Betriebs- oder Sicherheitsmodus. |
| `allow_remote_management` | `true` | Erlaubt Managementzugriffe über die Bind-Adresse. |

### `limits`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `max_body_bytes` | `2097152` | Maximal akzeptierte HTTP-Request-Größe. |
| `max_events` | `100000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_flows` | `100000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_packet_bytes` | `65535` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

## 9.11 security-core - `system-backend/security-core/config/security-core.example.toml`
### `server`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `bind` | `"0.0.0.0:8180"` | Bind-Adresse und Port des Dienstes. |
| `history_limit` | `5000` | Maximale Zahl im Arbeitsspeicher sichtbarer Historieneinträge. |

### `node_gateway`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `url` | `"ws://127.0.0.1:8080/ws/backend"` | URL der Abhängigkeit. |
| `reconnect_secs` | `5` | Zeit zwischen Wiederverbindungsversuchen. |
| `observe_nodes` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `storage`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `database_path` | `"/var/lib/netcore-security-core/state.json"` | Primäre persistente Zustandsdatei. |
| `backup_path` | `"/var/lib/netcore-security-core/state.json.bak"` | Lokale Backup-/Fallback-Datei. |
| `lab_seed_path` | `"/var/lib/netcore-security-core/lab-auth.seed"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `policy`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `operating_mode` | `"shadow"` | shadow: policy decisions and edge actions are visible but not authoritative. authoritative: the Security Core may issue challenge/install/revoke/disable actions. |
| `default_security_class` | `1` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `minimum_security_class` | `1` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `authentication_required` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `allow_class1_fallback` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `reject_unknown_subscribers` | `false` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `disable_after_failures` | `false` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `authentication`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `provider` | `"lab_hmac_sha256"` | Integration-test provider only. The following KMF package replaces this with real TETRA authentication/key-provider hooks. |
| `challenge_bytes` | `16` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `response_bytes` | `16` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `challenge_ttl_secs` | `30` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_attempts` | `3` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `lockout_secs` | `300` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `issue_dck_on_success` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `dck`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `key_bytes` | `16` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `ttl_secs` | `3600` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `rotate_before_secs` | `300` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_active_per_subscriber` | `2` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `security`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `mode` | `"open_lab"` | OPEN LAB: management API/WebUI intentionally have no users, tokens or TLS. |
| `allow_remote_management` | `true` | Erlaubt Managementzugriffe über die Bind-Adresse. |
| `expose_ephemeral_edge_material` | `true` | This endpoint is only for the TBS edge adapter. It is deliberately separate from normal management views and must stay inside the isolated lab network. |

### `limits`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `max_body_bytes` | `1048576` | Maximal akzeptierte HTTP-Request-Größe. |
| `max_profiles` | `100000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_contexts` | `20000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_actions` | `20000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_alarms` | `20000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_audit` | `100000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

## 9.12 kmf - `system-backend/kmf/config/kmf.example.toml`
### `server`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `bind` | `"0.0.0.0:8190"` | Bind-Adresse und Port des Dienstes. |
| `history_limit` | `5000` | Maximale Zahl im Arbeitsspeicher sichtbarer Historieneinträge. |

### `storage`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `database_path` | `"/var/lib/netcore-kmf/state.json"` | Primäre persistente Zustandsdatei. |
| `vault_path` | `"/var/lib/netcore-kmf/vault.json"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `master_key_path` | `"/var/lib/netcore-kmf/master.key"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `backup_dir` | `"/var/lib/netcore-kmf/backups"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `bootstrap_dir` | `"/var/lib/netcore-kmf/bootstrap"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `policy`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `operating_mode` | `"shadow"` | shadow: Workflows and OTAR actions are prepared but never released to an Edge. authoritative: approved and queued OTAR actions can be claimed by matching TBS nodes. |
| `default_key_bytes` | `16` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `default_crypto_period_secs` | `86400` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `rotation_lead_secs` | `3600` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `require_dual_approval` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `allow_overlapping_crypto_periods` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `auto_retire_predecessor` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `vault`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `provider` | `"lab_file_vault"` | Integration provider only. It keeps encrypted blobs separate from metadata. It is not a production HSM and the lab envelope is not a certified cipher. |
| `master_key_bytes` | `32` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `fsync` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `otar`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `action_ttl_secs` | `600` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_attempts` | `5` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `retry_backoff_secs` | `15` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_claim_batch` | `100` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `security`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `mode` | `"open_lab"` | Current isolated test-lab stage: deliberately no login, token or TLS. |
| `allow_remote_management` | `true` | Erlaubt Managementzugriffe über die Bind-Adresse. |
| `expose_raw_keys` | `false` | Hard invariant in this package. Setting true is rejected by design later if introduced. |

### `limits`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `max_body_bytes` | `1048576` | Maximal akzeptierte HTTP-Request-Größe. |
| `max_keys` | `100000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_nodes` | `10000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_jobs` | `100000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_actions` | `500000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_audit` | `100000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

## 9.13 transit - `system-backend/transit/config/transit.example.toml`
### `server`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `bind` | `"0.0.0.0:8200"` | Bind-Adresse und Port des Dienstes. |
| `history_limit` | `5000` | Maximale Zahl im Arbeitsspeicher sichtbarer Historieneinträge. |

### `storage`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `database_path` | `"/var/lib/netcore-transit/state.json"` | Primäre persistente Zustandsdatei. |
| `backup_path` | `"/var/lib/netcore-transit/state.json.bak"` | Lokale Backup-/Fallback-Datei. |

### `region`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `region_id` | `"region-a"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `swmi_id` | `"netcore-swmi-a"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `display_name` | `"NetCore Region A"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `advertised_endpoint` | `"http://10.0.10.12:8200"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `protocol_version` | `"netcore-transit-v1"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `operating_mode` | `"shadow"` | shadow = beobachten; authoritative = verbindlich ausführen. |
| `capabilities` | `[` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `routing`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `max_hops` | `8` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `dedupe_ttl_secs` | `900` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `session_idle_ttl_secs` | `3600` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `route_stale_secs` | `120` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `prefer_direct_region_peer` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `allow_transitive_routing` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `allow_dynamic_peers` | `false` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `fail_closed_on_loop` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `transport`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `connect_timeout_ms` | `2000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `io_timeout_ms` | `5000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `heartbeat_interval_secs` | `5` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `peer_timeout_secs` | `20` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `retry_backoff_secs` | `3` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_attempts` | `5` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_batch` | `100` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `security`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `mode` | `"open_lab"` | Betriebs- oder Sicherheitsmodus. |
| `allow_remote_management` | `true` | Erlaubt Managementzugriffe über die Bind-Adresse. |
| `tls` | `false` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `token_auth` | `false` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `limits`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `max_body_bytes` | `4194304` | Maximal akzeptierte HTTP-Request-Größe. |
| `max_peers` | `1000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_routes` | `100000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_sessions` | `100000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_envelopes` | `500000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_local_deliveries` | `500000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_events` | `100000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

## 9.14 application-gateway - `system-backend/application-gateway/config/application-gateway.example.toml`
### `server`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `bind` | `"0.0.0.0:8220"` | Bind-Adresse und Port des Dienstes. |
| `public_base_url` | `"http://127.0.0.1:8220"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_body_bytes` | `4194304` | Maximal akzeptierte HTTP-Request-Größe. |
| `history_limit` | `5000` | Maximale Zahl im Arbeitsspeicher sichtbarer Historieneinträge. |

### `storage`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `state_path` | `"/var/lib/netcore-application-gateway/state.json"` | Persistente Zustandsdatei. |
| `state_backup_path` | `"/var/lib/netcore-application-gateway/state.json.bak"` | Backup der Zustandsdatei. |
| `secrets_path` | `"/var/lib/netcore-application-gateway/secrets.json"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `spool_dir` | `"/var/lib/netcore-application-gateway/spool"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `backup_dir` | `"/var/lib/netcore-application-gateway/backups"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `security`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `mode` | `"open_lab"` | Betriebs- oder Sicherheitsmodus. |
| `management_token_auth` | `false` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `management_tls` | `false` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `allow_remote_management` | `true` | Erlaubt Managementzugriffe über die Bind-Adresse. |
| `connector_secrets_allowed` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `warning_banner` | `"OPEN LAB: no login, no management tokens and no TLS. Isolated management network only."` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `runtime`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `operating_mode` | `"shadow"` | shadow calculates and records routes without calling external systems. Change to authoritative only after connector endpoints were verified. |
| `worker_interval_ms` | `1000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `probe_interval_secs` | `30` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `default_ttl_secs` | `300` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_attempts` | `6` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `base_backoff_secs` | `2` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_backoff_secs` | `120` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `dedupe_window_secs` | `600` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_response_bytes` | `65536` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_artifact_bytes` | `33554432` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_events` | `20000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_deliveries` | `50000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_tts_jobs` | `5000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_audit_records` | `50000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `event_retention_secs` | `604800` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `delivery_retention_secs` | `1209600` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `audit_retention_secs` | `2592000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `connectors[]`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `connector_id` | `"sds-router"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `display_name` | `"SDS Router"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `kind` | `"sds_router"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `direction` | `"outbound"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `endpoint` | `"http://127.0.0.1:8150/api/v1/messages"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `health_endpoint` | `"http://127.0.0.1:8150/health/ready"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `enabled` | `true` | Funktion aktivieren oder deaktivieren. |
| `timeout_ms` | `5000` | Timeout in Millisekunden. |
| `rate_limit_per_minute` | `600` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `circuit_failure_threshold` | `5` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `circuit_open_secs` | `60` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `required_secrets` | `[]` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `settings` | `{ source_issi = "9999", sds_type = "4", protocol_id = "0", priority = "3" }` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `rules[]`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `rule_id` | `"manual-to-sds"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `name` | `"Manual messages to SDS Router"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `enabled` | `true` | Funktion aktivieren oder deaktivieren. |
| `priority` | `100` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `source_connector` | `"manual"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `event_type` | `"sds.message"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `target_connector` | `"sds-router"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `template_id` | `"sds-standard"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `stop_processing` | `false` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `templates[]`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `template_id` | `"sds-standard"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `name` | `"SDS Standard"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `kind` | `"text"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `body` | `"{{text}}"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `content_type` | `"text/plain; charset=utf-8"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `enabled` | `true` | Funktion aktivieren oder deaktivieren. |
| `target_connector` | `"sds-router"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `description` | `"Plain SDS text with destination supplied by the event"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

## 9.15 media-library - `system-backend/media-library/config/media-library.example.toml`
### `server`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `bind` | `"0.0.0.0:8230"` | Bind-Adresse und Port des Dienstes. |
| `public_base_url` | `"http://127.0.0.1:8230"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_body_bytes` | `100663296` | Maximal akzeptierte HTTP-Request-Größe. |

### `security`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `mode` | `"open_lab"` | Betriebs- oder Sicherheitsmodus. |
| `token_auth` | `false` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `tls` | `false` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `allow_remote_management` | `true` | Erlaubt Managementzugriffe über die Bind-Adresse. |
| `allow_delete` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `allow_url_import` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `allow_private_import_urls` | `true` | Required for imports from the local Application Gateway and Recorder in the lab. |

### `storage`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `root` | `"/var/lib/netcore-media-library/assets"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `state_file` | `"/var/lib/netcore-media-library/state.json"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `temp_root` | `"/var/lib/netcore-media-library/tmp"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `backup_root` | `"/var/lib/netcore-media-library/backups"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `archive_root` | `"/mnt/nfs-share/Media-Library"` | Mount the NFS share before using Archive in the WebUI. The service never treats the archive as the live playout source. |
| `max_asset_bytes` | `67108864` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_total_bytes` | `21474836480` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `fsync_imports` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `runtime`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `operating_mode` | `"shadow"` | shadow prepares jobs but suppresses Media Switch injection. authoritative injects validated 35-byte frames into an existing media session. |
| `worker_interval_ms` | `500` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `probe_interval_secs` | `15` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `import_timeout_secs` | `20` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_assets` | `10000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_jobs` | `2000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_events` | `5000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_audit_records` | `10000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_attempts` | `3` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `frame_interval_ms` | `60` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `auto_approve_tts` | `false` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `codec`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `frame_bytes` | `35` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `ffmpeg_command` | `[` | The command is executed directly without a shell. {input} and {output} are replaced with paths owned by the service. |
| `encoder_command` | `[]` | Optional native helper commands. They must produce/consume packed 35-byte TETRA speech service 0 frames. Empty means WAV/MP3 assets remain preview-only, while imported .tacelp assets remain directly playable. |
| `decoder_command` | `[]` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `dependencies`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `media_switch_base_url` | `"http://127.0.0.1:8130"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `recorder_base_url` | `"http://127.0.0.1:8140"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `application_gateway_base_url` | `"http://127.0.0.1:8220"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

## 9.16 control-room - `system-backend/control-room/config/control-room.example.toml`
### `server`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `bind` | `"0.0.0.0:9010"` | Bind-Adresse und Port des Dienstes. |
| `node_path` | `"/node"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `ui_path` | `"/ui"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `history_limit` | `2000` | Maximale Zahl im Arbeitsspeicher sichtbarer Historieneinträge. |

### `persistence`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `enabled` | `true` | Funktion aktivieren oder deaktivieren. |
| `database_path` | `"/var/lib/netcore-control-room/control-room.sqlite3"` | Primäre persistente Zustandsdatei. |
| `persist_events` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `persist_noisy_events` | `false` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `load_recent_limit` | `2000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `auth`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `enabled` | `false` | Current laboratory phase: deliberately no login, no node token and no TLS. |
| `allow_health_unauthenticated` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `node_token_env` | `""` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `bootstrap_username_env` | `""` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `bootstrap_password_env` | `""` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `bootstrap_role` | `"admin"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `federation`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `enabled` | `true` | Funktion aktivieren oder deaktivieren. |
| `poll_interval_secs` | `5` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `request_timeout_ms` | `1200` | HTTP-Timeout in Millisekunden. |
| `failure_threshold` | `3` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `fetch_summaries` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `operations`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `state_path` | `"/var/lib/netcore-control-room/operations.json"` | Persistente Zustandsdatei. |
| `backup_path` | `"/var/lib/netcore-control-room/operations.json.bak"` | Lokale Backup-/Fallback-Datei. |
| `auto_service_incidents` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `incident_limit` | `5000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `shift_log_limit` | `10000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `services[]`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `name` | `"node-gateway"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `display_name` | `"Node Gateway"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `kind` | `"edge"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `base_url` | `"http://10.0.20.10:8080"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `health_live` | `"/health/live"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `health_ready` | `"/health/ready"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `summary_path` | `"/api/v1/status"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `webui_path` | `"/"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `critical` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `enabled` | `true` | Funktion aktivieren oder deaktivieren. |

### `directory`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `hide_infrastructure` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

## 9.17 observability - `system-backend/observability/config/observability.example.toml`
### `server`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `bind` | `"0.0.0.0:8210"` | Bind-Adresse und Port des Dienstes. |
| `history_limit` | `5000` | Maximale Zahl im Arbeitsspeicher sichtbarer Historieneinträge. |
| `max_body_bytes` | `4194304` | Maximal akzeptierte HTTP-Request-Größe. |

### `storage`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `state_path` | `"/var/lib/netcore-observability/state.json"` | Persistente Zustandsdatei. |
| `backup_path` | `"/var/lib/netcore-observability/state.json.bak"` | Lokale Backup-/Fallback-Datei. |
| `diagnostic_dir` | `"/var/lib/netcore-observability/diagnostics"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `security`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `mode` | `"open_lab"` | Betriebs- oder Sicherheitsmodus. |
| `token_auth` | `false` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `tls` | `false` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `allow_remote_management` | `true` | Erlaubt Managementzugriffe über die Bind-Adresse. |
| `warning_banner` | `"OPEN LAB: no login, no tokens and no TLS. Isolated management network only."` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `collection`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `scrape_interval_secs` | `15` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `request_timeout_ms` | `2000` | HTTP-Timeout in Millisekunden. |
| `max_response_bytes` | `2097152` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `scrape_on_start` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `ingest_logs` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `ingest_traces` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `retention`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `metric_retention_secs` | `86400` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `log_retention_secs` | `604800` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `trace_retention_secs` | `86400` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `audit_retention_secs` | `2592000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_series` | `10000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_samples_per_series` | `5760` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_logs` | `100000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_spans` | `50000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_alerts` | `10000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_audit_records` | `50000` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `stack`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `prometheus_url` | `"http://127.0.0.1:9090"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `grafana_url` | `"http://127.0.0.1:3000"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `loki_url` | `"http://127.0.0.1:3100"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `alertmanager_url` | `"http://127.0.0.1:9093"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `prometheus_ready_path` | `"/-/ready"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `grafana_ready_path` | `"/api/health"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `loki_ready_path` | `"/ready"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `alertmanager_ready_path` | `"/-/ready"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `targets[]`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `target_id` | `"node-gateway"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `display_name` | `"Node Gateway"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `service` | `"node-gateway"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `base_url` | `"http://127.0.0.1:8080"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `metrics_path` | `"/metrics"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `live_path` | `"/health/live"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `ready_path` | `"/health/ready"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `enabled` | `true` | Funktion aktivieren oder deaktivieren. |
| `labels` | `{ environment = "open-lab", component = "node-gateway" }` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `alert_rules[]`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `rule_id` | `"target-down"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `name` | `"Target down"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `description` | `"A monitored target does not answer its liveness endpoint"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `metric` | `"netcore_observability_target_up"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `comparator` | `"<"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `threshold` | `1.0` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `for_secs` | `30` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `severity` | `"critical"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `enabled` | `true` | Funktion aktivieren oder deaktivieren. |
| `labels` | `{ source = "netcore-observability" }` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `annotations` | `{ runbook = "Check service process, network path and /health/live" }` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

## 9.18 Optionaler Operator-Client - `system-backend/control-room/config/operator.example.toml`
### `profiles.default`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `api` | `"http://10.0.1.25:9010"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `default_node` | `"SRV-M_TBS-01"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `operator_id` | `"jan"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `username` | `"jan"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `profiles.event`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `api` | `"http://10.0.1.25:9010"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `default_node` | `"SRV-M_TBS-01"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `operator_id` | `"event-lst"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `username` | `"event"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

## 9.18 Optionaler Operator-Client - `system-backend/control-room/config/operator-ui.example.toml`
### `profiles.default`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `api` | `"http://10.0.1.25:9010"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `default_node` | `"SRV-M_TBS-01"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `operator_id` | `"jan"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `username` | `"jan"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `profiles.event`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `api` | `"http://10.0.1.25:9010"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `default_node` | `"SRV-M_TBS-01"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `operator_id` | `"event-lst"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `username` | `"event"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `ui.map`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `online_tiles` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `tile_url` | `"https://tile.openstreetmap.org/{z}/{x}/{y}.png"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `tile_attribution` | `"© OpenStreetMap contributors"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `default_lat` | `52.3759` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `default_lon` | `9.7320` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `default_zoom` | `13` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `min_zoom` | `3` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `max_zoom` | `18` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `directory`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `hide_infrastructure` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `directory.subscribers."2010002"`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `name` | `"Jan HRT"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `device_class` | `"HRT"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `status` | `"Einsatzbereit"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `status_group` | `"crew"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `groups` | `[15201, 15205]` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `directory.subscribers."2020004"`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `name` | `"Event Operator"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `device_class` | `"HRT"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `status_group` | `"crew"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `groups` | `[15201]` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `directory.subscribers."4010001"`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `name` | `"SRV-M_TBS-01"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `device_class` | `"Basisstation"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `hide_in_subscribers` | `true` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `directory.groups."15201"`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `name` | `"NetCore Crew"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `kind` | `"Sprechgruppe"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `directory.groups."15205"`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `name` | `"Tactical"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `kind` | `"Sprechgruppe"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `directory.status_groups."crew"`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `name` | `"Crew-Status"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `directory.statuses."1"`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `label` | `"Frei / bereit"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `group` | `"crew"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

### `directory.statuses."2"`
| Schlüssel | Beispielwert | Bedeutung / Hinweis |
|---|---|---|
| `label` | `"Beschäftigt"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |
| `group` | `"crew"` | Dienstspezifischer Parameter; vor Änderung mit WebUI/README und Logs abgleichen. |

# 10. Betriebs- und Bedienabläufe
## 10.1 Start- und Stopp-Reihenfolge
Start: Node Gateway → Mobility/Subscriber → Group → Call/Packet/Security → Media/SDS/IP/KMF → Transit/Application/Recorder/Media Library → Control Room → Observability. Das Inventory berechnet diese Reihenfolge automatisch.

Beim geplanten Komplettstopp umgekehrt vorgehen. Die Basisstation kann lokal weiterlaufen; für Wartung am Core ist ein bewusstes `degraded`/`isolated`-Fenster daher zulässig.

## 10.2 Teilnehmer anlegen
1. Subscriber-Core-WebUI öffnen.
2. ISSI, Name, Organisation, Home-MCC/MNC, Status `enabled` und `registration_allowed` setzen.
3. Sprach-, SDS- und Packet-Data-Berechtigungen passend aktivieren.
4. Speichern und TBS-Sync prüfen.
5. Funkgerät registrieren; Live-Lage und Node Gateway kontrollieren.
6. Bei `allow_list` ist ein fehlendes Profil eine Ablehnung - das ist beabsichtigt.

## 10.3 Gruppe und Mitgliedschaft
1. Group Core: GSSI, Name, Priorität/Class of Usage und Dienstfreigaben anlegen.
2. Teilnehmer als feste oder dynamische Mitglieder hinzufügen.
3. Optional Auto-Affiliation definieren.
4. DGNA erst nach erfolgreichem Policy-Sync auslösen.
5. Affiliation in Group Core und TBS prüfen.

## 10.4 Gruppenruf
1. Teilnehmer registriert und affiliiert?
2. Call Control: logischen Gruppenruf starten oder Ruf von Funkgerät initiieren.
3. Call Legs und Floor Holder prüfen.
4. Media Switch: Sessions, Streams und Jitter-Puffer kontrollieren.
5. Recorder: aktive Aufnahme und später Integrität prüfen.
6. Ruf sauber beenden; keine Testframe-Injection in belegtem Netz verwenden.

## 10.5 SDS senden
1. SDS Router: Zielart Individual/Group, Ziel-SSI, Typ, Protocol-ID, Priorität und TTL setzen.
2. Nachricht absenden und Zustelllegs beobachten.
3. Offline-Ziel wird persistent gequeued; Retry/Dead Letter nur nachvollziehbar bedienen.
4. Application Gateway kann Vorlagen, Webhooks oder TTS-Workflows auslösen; im Shadow-Modus entstehen keine externen Nebenwirkungen.

## 10.6 Packet Data
1. Packet Core und IP Gateway zunächst Shadow.
2. Adresspool/Netz/MTU konsistent prüfen.
3. IP Gateway Kernel-Plan prüfen; TUN-Passthrough und nftables bereitstellen.
4. Packet Core auf authoritative, danach IP Gateway auf authoritative umstellen.
5. PDP-Kontext aktivieren, Lease prüfen, WAP-Testseite `http://10.0.0.1:8088/` beziehungsweise das konfigurierte Gateway testen.
6. PCAP nur zeitlich und größenmäßig begrenzt aktivieren.

## 10.7 Aufnahme, TTS und Aussendung
1. Recorder nimmt automatisch aus dem Media-Switch-Tap auf.
2. Application Gateway erzeugt TTS-WAV über Piper.
3. Media Library importiert WAV/MP3/TACELP; Vorschau und Metadaten prüfen.
4. Asset freigeben. WAV/MP3 brauchen für Funkbereitschaft einen echten TETRA-Encoder; `.tacelp` muss positives Vielfaches von 35 Byte sein.
5. Für Playout muss bereits eine Media-Switch-Session existieren.
6. Job starten, Fortschritt beobachten, bei Fehler bewusst ab Frame 0 wiederholen.

## 10.8 Security und KMF
1. Security Core zunächst Shadow; Profile/Policy beobachten.
2. Lab-HMAC ist nur Integrationsprovider, keine produktive TETRA-Authentisierung.
3. KMF-Keys und Crypto Periods in Shadow erzeugen/prüfen.
4. Vier-Augen-Freigaben im Open Lab sind nur deklarativ.
5. Authoritative/OTAR erst aktivieren, wenn der Air-Interface-Adapter und sichere Schlüsselwege nachweislich funktionieren.
6. Bei Core-Ausfall keine neuen Schlüssel erfinden; bereits installierte Schlüssel bleiben lokal nutzbar.

## 10.9 Control Room und Observability
Der Control Room ist die erste Lageansicht. Für fachliche Änderungen in die jeweilige Service-WebUI wechseln. Incidents quittieren, Notizen ergänzen, Ursache beheben und erst dann lösen. In Observability nach Trace-/Correlation-ID suchen, Scrape Targets testen und bei Wartungen Silences setzen.
# 11. Offline-Fallback der Basisstation
![Fallback-State-Machine](assets/fallback.png){ width=95% }

## 11.1 Auslöser
Die TBS pingt keinen öffentlichen Internetdienst. Entscheidend sind ihre WebSocket-Verbindung zum Node Gateway und die vollständige, revisionierte Core-Service-Matrix. Damit führen Internet-, VPN-, Routing- oder Core-Ausfälle zum selben klaren Ergebnis.

| Zustand | Bedeutung |
|---|---|
| `online` | Gateway und alle benötigten Dienste gesund. |
| `degraded` | Einzelne Dienste ausgefallen; nur deren lokaler Fallback aktiv. |
| `isolated` | Gateway nicht erreichbar oder Matrix-Lease abgelaufen; TBS lokal autoritativ. |
| `recovering` | Dienste wieder gesund; Hysterese und Replay laufen. |

## 11.2 Lokale Funktionen
| Ausgefallener Dienst | Lokales Verhalten |
|---|---|
| Node Gateway | lokale Edge-Autonomie; Reconnect läuft weiter |
| Subscriber Core | letzte Policy, dann statische Konfiguration |
| Group Core | letzte Gruppenpolicy und lokale Affiliationen |
| Mobility Core | lokale Registrierung und Location Area |
| Call Control | Rufe innerhalb der lokalen Zelle |
| Media Switch | lokale Air-Interface-Medien; keine zentralen Frames |
| Recorder | lokaler Recorder läuft weiter |
| SDS Router | lokale Zustellung; nicht-lokale Nachrichten persistent gequeued |
| Packet Core | lokale SNDCP/PDP-Kontexte |
| IP Gateway | lokales TUN/Routing, sofern auf TBS konfiguriert |
| Security Core | letzte Policy; kein stilles Downgrade |
| KMF | installierte Keys; kein erfundenes OTAR |
| Transit | kein Inter-Region-Routing |
| Control Room | lokales TBS-Dashboard und Audit |
| Observability | lokale Logs/Health |
| Application Gateway | nur lokal erreichbare Integrationen |
| Media Library | lokal gecachte/freigegebene Medien |

## 11.3 Diagnose
```bash
# Basisstation
curl -fsS http://127.0.0.1:8080/api/edge-fallback | jq .

# Node Gateway
curl -fsS http://10.0.20.10:8080/api/v1/core-services | jq .
```
Auf der TBS sind insbesondere `gateway_connected`, `service_matrix_fresh`, `mode`, `reason`, `services`, `policy_cache` und `event_spool` zu prüfen.

## 11.4 Recovery
1. Ursache im Netz/Core beheben.
2. Node Gateway muss wieder vollständige gesunde Matrix liefern.
3. `recover_after_secs` abwarten; nicht durch wiederholte Neustarts flappen lassen.
4. Replay-Spool beobachten. Sprachframes werden absichtlich nicht nachträglich ausgesendet.
5. Subscriber-/Group-Policy-Sync und SDS-Remotelegs prüfen.
6. Erst nach `online` wieder zentrale Operatoraktionen auslösen.
# 12. Backup, Update und Wiederherstellung
## 12.1 Basisstation sichern
```bash
sudo systemctl stop tetra.service
sudo tar -C / -czf /var/backups/netcore-tbs-$(date +%Y%m%d-%H%M%S).tar.gz \
  etc/netcore var/lib/netcore var/lib/flowstation var/cache/netcore
sudo systemctl start tetra.service
```
Große Aufzeichnungen/Archive separat behandeln.

## 12.2 LXC-State sichern
Mindestens Konfiguration, `/var/lib/<dienst>`, systemd-Unit und ggf. `/opt/<dienst>` sichern. KMF-Master-Key separat und offline sichern. Für Recorder/Media Library Archivdaten, für IP Gateway PCAPs, für Observability Diagnose/Retention berücksichtigen.

## 12.3 Update der Basisstation
```bash
sudo systemctl stop tetra.service
cd /opt/netcore-tetra-swmi
git status
git pull --ff-only              # bei Git-Installation
cargo clean && rm -rf target
cargo build --release -p bluestation-bs
sudo install -m 0755 target/release/bluestation-bs /usr/local/bin/bluestation-bs
sudo systemctl start tetra.service
sudo journalctl -u tetra.service -n 200 --no-pager
```

## 12.4 Update eines LXC-Dienstes
```bash
sudo cp <config> <config>.pre-update
sudo system-backend/<dienst>/install/update.sh
sudo systemctl status <unit> --no-pager
curl -i http://127.0.0.1:<port>/health/ready
```
Oder das vorher geprüfte Deployment-Bundle erneut über den Deployer anwenden.

## 12.5 Rollback
1. Dienst stoppen.
2. Vorherige Binär-/Quellversion wiederherstellen.
3. Passende alte Konfiguration zurückkopieren.
4. State nur dann zurückrollen, wenn Schema/Upgrade dies erfordert; vorher aktuellen State sichern.
5. Dienst manuell starten, Logs prüfen, dann systemd aktivieren.
# 13. Systemtest und Abnahme
## 13.1 Statische Vorprüfung
```bash
python3 tools/check_full_system_integration.py
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml validate
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile full --validate-only
```

## 13.2 Smoke-Test
```bash
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile smoke
```
Erwartet: Liveness, Status, OpenAPI, Metrics und WebUI aller 17 Dienste; Mock-TBS im Node Gateway; Control-Room-Polling.

## 13.3 Vollständiger Funktionstest
```bash
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml \
  test --profile full --allow-mutations --timeout 35
```
Prüft Teilnehmer/Gruppe, Gruppenruf, Media/Recorder, SDS Store-and-forward, Packet Data, Federation und Observability. Nur in einem leeren Labornetz ausführen.

## 13.4 Fault- und Fallback-Test
```bash
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml \
  test --profile fault --allow-mutations --allow-restarts --timeout 45
```
Dieser Test stoppt Dienste absichtlich. Abnahme: `failed=0`, alle Opferdienste wieder active/ready, TBS sieht `unavailable` und anschließende Recovery, keine Testfixtures bleiben liegen.

## 13.5 On-Air-Abnahme
Mock-Tests belegen keine HF-, MAC-, LLC-, MLE-, MM- oder CMCE-Konformität. Mit mindestens zwei Geräteherstellern dokumentieren: MCC/MNC/LAC/Colour Code/Carrier, Registrierung, Gruppenruf, SDS, Packet Data, Audio, Release/Restore, Zeitstempel, Softwarestände und Evidenzdateien.
# 14. Fehlersuche
## 14.1 Standardreihenfolge
1. Fehlerzeitpunkt, ISSI/GSSI, TBS und Correlation-ID notieren.
2. `systemctl status` und **erste** konkrete Journal-Fehlermeldung lesen.
3. Liveness, Readiness und Abhängigkeiten getrennt prüfen.
4. Nur eine Variable ändern.
5. Nach Fix den vollständigen Ablauf inklusive Release wiederholen.

## 14.2 Schnelldiagnose
```bash
systemctl status <unit> --no-pager
journalctl -u <unit> -b -n 300 --no-pager
ss -ltnup
curl -v http://127.0.0.1:<port>/health/live
curl -v http://127.0.0.1:<port>/health/ready
curl -fsS http://127.0.0.1:<port>/api/v1/status | jq .
```

| Symptom | Wahrscheinliche Ursache | Prüfung |
|---|---|---|
| TBS startet mit Fallback | TOML-Fehler/Primärdatei unlesbar | Journal nach config/fallback/parse durchsuchen; diff zur Fallback-Datei |
| Node im Gateway nicht sichtbar | falscher host/port/path, Firewall, Node-ID-Duplikat | TBS `[control_room]`, WebSocket `/ws/node`, Gateway-Events |
| Core-Dienst live aber nicht ready | Abhängigkeit nicht erreichbar oder Shadow-/Kernel-Voraussetzung | Status JSON und gerenderte URL prüfen |
| Gruppenruf ohne Sprache | keine Media-Session, falsche Legs, Jitter/Frameformat | Call Control Legs, Media Switch RX/TX/Drops |
| Recorder leer | Tap-URL falsch oder Cursor-Lücke | Media-Switch Recorder-Tap und Recorder-Events |
| SDS bleibt queued | Ziel nicht präsent, keine Route, zentrale SDS-Funktion nicht aktiv | Presence, TBS-Sync, TTL/Retry |
| Packet Data ohne Internet | IP Gateway shadow, TUN fehlt, NAT/Firewall/DNS falsch | kernel/plan, ip addr/route, nft ruleset |
| Control Room zeigt alte Config | falscher Config-Pfad | `/etc/netcore-control-room/control-room.toml` und ExecStart prüfen |
| Media-Vorschau fehlt | ffmpeg/Decoder fehlt oder Dateiformat ungültig | Media-Library-Logs und Codec-Konfig |
| Fallback bleibt recovering | Matrix nicht vollständig gesund oder Replay offen | TBS `/api/edge-fallback`, Gateway `/api/v1/core-services` |
| NFS blockiert/readonly | Mountoptionen, UID/GID, NAS nicht erreichbar | mount, findmnt, touch als Dienstuser |
# 15. Inbetriebnahme-Checkliste
## 15.1 Infrastruktur
- [ ] Management-VLAN isoliert; keine Portweiterleitung
- [ ] 17 statische LXC-Adressen eingetragen
- [ ] NTP/DNS/Hosts funktionieren
- [ ] SSH-Key-Deployment getestet
- [ ] IP-Gateway-TUN vorhanden
- [ ] NFS/Archiv mit nofail und korrekten Rechten
- [ ] Backupspeicher getrennt vorhanden

## 15.2 Basisstation
- [ ] SDR erkannt und Passband geprüft
- [ ] Frequenzen/MCC/MNC/LAC/Colour Code genehmigt und korrekt
- [ ] config.toml und config.toml.fallback getrennt geprüft
- [ ] Dashboard-Passwort geändert
- [ ] Node-ID eindeutig
- [ ] Node Gateway erreichbar
- [ ] Edge-Fallback online/degraded/isolated/recovering getestet
- [ ] lokaler Ruf/SDS bei abgetrenntem Core erfolgreich

## 15.3 Core
- [ ] Inventory mit korrigiertem Control-Room-Pfad
- [ ] validate/plan/render ohne Fehler
- [ ] alle Dienste live
- [ ] Readiness-Ausfälle erklärbar
- [ ] Shadow-Dienste noch nicht unbeabsichtigt authoritative
- [ ] Subscriber/Group Policy synchron
- [ ] Recorder/Media Storage geprüft
- [ ] KMF-Master-Key separat gesichert
- [ ] Control Room federiert alle Dienste
- [ ] Observability Targets grün

## 15.4 Abnahme
- [ ] Smoke failed=0
- [ ] Full failed=0
- [ ] Fault failed=0
- [ ] keine Testfixtures übrig
- [ ] On-Air-Test dokumentiert
- [ ] Rollback einmal praktisch getestet
- [ ] Betriebs- und Notfallkontakte dokumentiert
# Anhang A: Port-, Pfad- und Befehlsreferenz
## A.1 Dienste
| Dienst | Port | Unit | Konfiguration | State/Arbeitsverzeichnis |
|---|---:|---|---|---|
| `node-gateway` | 8080 | `netcore-node-gateway.service` | `/etc/netcore/node-gateway.toml` | `In-Memory; keine Fachdatenbank` |
| `mobility-core` | 8090 | `netcore-mobility-core.service` | `/etc/netcore/mobility-core.toml` | `aktuell primär Laufzeitlage` |
| `subscriber-core` | 8100 | `netcore-subscriber-core.service` | `/etc/netcore/subscriber-core.toml` | `/var/lib/netcore-subscriber-core/subscribers.json` |
| `group-core` | 8110 | `netcore-group-core.service` | `/etc/netcore/group-core.toml` | `/var/lib/netcore-group-core/groups.json` |
| `call-control` | 8120 | `netcore-call-control.service` | `/etc/netcore/call-control.toml` | `/var/lib/netcore-call-control/calls.json` |
| `media-switch` | 8130 | `netcore-media-switch.service` | `/etc/netcore/media-switch.toml` | `zeitkritische Laufzeitdaten im Speicher` |
| `recorder` | 8140 | `netcore-recorder.service` | `/etc/netcore/recorder.toml` | `/var/lib/netcore-recorder/recordings` |
| `sds-router` | 8150 | `netcore-sds-router.service` | `/etc/netcore/sds-router.toml` | `/var/lib/netcore-sds-router/messages.json` |
| `packet-core` | 8160 | `netcore-packet-core.service` | `/etc/netcore/packet-core.toml` | `/var/lib/netcore-packet-core/state.json` |
| `ip-gateway` | 8170 | `netcore-ip-gateway.service` | `/etc/netcore/ip-gateway.toml` | `/var/lib/netcore-ip-gateway/state.json und captures/` |
| `security-core` | 8180 | `netcore-security-core.service` | `/etc/netcore/security-core.toml` | `/var/lib/netcore-security-core/state.json; Seed separat` |
| `kmf` | 8190 | `netcore-kmf.service` | `/etc/netcore/kmf.toml` | `/var/lib/netcore-kmf/state.json + vault.json + master.key` |
| `transit` | 8200 | `netcore-transit.service` | `/etc/netcore/transit.toml` | `/var/lib/netcore-transit/state.json` |
| `application-gateway` | 8220 | `netcore-application-gateway.service` | `/etc/netcore/application-gateway.toml` | `state.json, secrets.json, spool, backups` |
| `media-library` | 8230 | `netcore-media-library.service` | `/etc/netcore/media-library.toml` | `/var/lib/netcore-media-library/assets + state.json` |
| `control-room` | 9010 | `netcore-control-room.service` | `/etc/netcore-control-room/control-room.toml` | `control-room.sqlite3 + operations.json` |
| `observability` | 8210 | `netcore-observability.service` | `/etc/netcore/observability.toml` | `/var/lib/netcore-observability/state.json + diagnostics/` |

## A.2 Zusätzliche Ports
| Komponente | Port | Zweck |
|---|---:|---|
| Piper TTS | `5005/tcp` | lokale Sprachsynthese |
| IP Gateway DNS | `53/udp` | DNS Forwarder |
| IP Gateway Testserver | `8088/tcp` | HTTP/WAP/WML |
| IP Gateway UDP Echo | `7007/udp` | UDP-Test |
| Grafana | `3000/tcp` | Dashboards |
| Prometheus | `9090/tcp` | Metriken |
| Alertmanager | `9093/tcp` | Alarmrouting |
| Loki | `3100/tcp` | Logs |

## A.3 Standardbefehle
```bash
# Logs
journalctl -u <unit> -f
journalctl -u <unit> -b -n 300 --no-pager

# Health
curl -fsS http://<ip>:<port>/health/live
curl -i    http://<ip>:<port>/health/ready
curl -fsS http://<ip>:<port>/metrics | head

# Basisstation-Fallback
curl -fsS http://<tbs>:8080/api/edge-fallback | jq .
curl -fsS http://<node-gateway>:8080/api/v1/core-services | jq .

# Deployment/E2E
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml status
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml test --profile smoke
```

## A.4 Mitgelieferte Begleitdateien
- `inventory.open-lab.corrected.example.toml` - Inventory mit korrigiertem Control-Room-Konfigurationspfad.
- `basisstation.config.sanitized.example.toml` - vollständige, bereinigte Basisstationsvorlage ohne ursprüngliche Zugangsdaten.
- `NetCore-Tetra-Komplettguide.md` - editierbare Markdown-Quelle dieses Guides.
