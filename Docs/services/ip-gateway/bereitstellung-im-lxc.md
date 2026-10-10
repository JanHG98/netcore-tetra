# IP Gateway im LXC bereitstellen

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [install/install.sh](../../../system-backend/ip-gateway/install/install.sh) · [config/ip-gateway.example.toml](../../../system-backend/ip-gateway/config/ip-gateway.example.toml) · [systemd/netcore-ip-gateway.service](../../../system-backend/ip-gateway/systemd/netcore-ip-gateway.service). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

## Voraussetzungen

Eigener Debian-LXC mit einer im Managementnetz erreichbaren Adresse; DHCP mit fester Zuordnung ist ebenso möglich wie statische Konfiguration. Die Ressourcenangaben sind Labor-Richtwerte, keine gemessene Kapazitätszusage. CPU und Speicherbedarf unter tatsächlicher Teilnehmer- und Datenlast messen.

Der Installer baut aus dem vollständigen Checkout mit `cargo build --release -p netcore-ip-gateway`. Build-Werkzeuge, `pkg-config`, `iproute2` und eine Rust-Toolchain müssen vorhanden sein. `cargo` muss auch für den ausführenden root-/sudo-Prozess erreichbar sein; eine ausschließlich im Benutzerprofil installierte Toolchain ist dafür nicht automatisch sichtbar.

## Konfiguration vorbereiten

Alle relativen Befehle ab dem Repository-Root ausführen. Bei abweichendem Checkout den `cd`-Pfad anpassen. Eine bestehende Konfiguration erhalten und direkt bearbeiten; die Beispielkopie ist nur für die erste Einrichtung:

```bash
cd /opt/netcore-tetra
sudo install -d /etc/netcore
# Nur bei der ersten Einrichtung:
sudo test -f /etc/netcore/ip-gateway.toml || sudo cp system-backend/ip-gateway/config/ip-gateway.example.toml /etc/netcore/ip-gateway.toml
sudo editor /etc/netcore/ip-gateway.toml
```

`[packet_core].url`, `nat.egress_interface`, TUN-Netz/Gateway, DNS-Bind und Firewall-CIDRs abgleichen. Packet Core und IP Gateway verwenden unterschiedliche Beispielpools (`10.44.0.0/24` bzw. `10.0.0.0/24`). Der Installer verlangt `ip`, `nft` und `/dev/net/tun` bereits vor dem Build, auch wenn der spätere Dienst zunächst Shadow nutzt.

## Installieren und Dienst prüfen

```bash
cd /opt/netcore-tetra
sudo system-backend/ip-gateway/install/install.sh
systemctl status netcore-ip-gateway --no-pager
journalctl -u netcore-ip-gateway -n 100 --no-pager
source /etc/netcore/lxc-network.env
curl "${NETCORE_WEBUI_URL}health/live"
curl "${NETCORE_WEBUI_URL}health/ready"
curl "${NETCORE_WEBUI_URL}api/v1/status"
```

Der Installer bindet die WebUI/API an die erkannte LXC-Adresse und speichert sie in `/etc/netcore/lxc-network.env`. Deshalb ist `127.0.0.1:8170` nach dieser Installation nicht zwingend erreichbar. Bei mehreren Interfaces kann die Adresse über `NETCORE_LXC_IP` bewusst vorgegeben werden. Die URL im Installerausgabefeld und die tatsächlich gespeicherte Konfiguration prüfen.

Konfiguration: `/etc/netcore/ip-gateway.toml`; Binary: `/usr/local/bin/netcore-ip-gateway`; Unit: `netcore-ip-gateway.service`. Die vorhandenen Skripte `install/update.sh` und `install/uninstall.sh` stehen bei der Quelle; vor Entfernung die dortigen Datenoptionen prüfen.

## Netzwerk und Abnahme

TCP `8170` nur aus dem isolierten Management-/Testnetz freigeben. Die fachlichen Abhängigkeiten oben müssen erreichbar sein. Die Dienste implementieren hier keine Managementanmeldung oder TLS. `health/live` belegt nur den Prozess; `health/ready`, Statusfelder, fachliche Antworten und bei Funkfunktionen tatsächliche Gerätebeobachtung getrennt bewerten. Diese Anleitung enthält keine bereits ausgeführte LXC- oder On-Air-Abnahme.

## TUN-Gerät und Rechte

Auf dem Proxmox-Host für den betroffenen Container die Durchreichung lokal prüfen. Übliche Konfiguration für `/dev/net/tun`:

```text
lxc.cgroup2.devices.allow: c 10:200 rwm
lxc.mount.entry: /dev/net/tun dev/net/tun none bind,create=file
```

Im Container `ls -l /dev/net/tun`, `ip -V` und `nft --version` prüfen; `iproute2` und `nftables` installieren. Der Container muss die von der systemd-Unit erteilten `CAP_NET_ADMIN`, `CAP_NET_RAW` und `CAP_NET_BIND_SERVICE` zulassen.

Zunächst `[interface].mode = "shadow"` verwenden und `/api/v1/kernel/plan` prüfen. Erst im bewusst gewählten `authoritative`-Modus werden TUN, Adressen, IPv4-Forwarding, Routing und eigene nftables-Tabellen tatsächlich angewendet. Nach Modusänderung `sudo systemctl restart netcore-ip-gateway`.

Der DNS-Proxy bindet nur in Authoritative. `0.0.0.0:53` wird auf die konfigurierte TUN-Gateway-Adresse eingegrenzt; im Beispiel ist dies `10.0.0.1:53`. HTTP-/WAP-Testserver `8088/tcp` und UDP-Echo `7007/udp` sind eigene Listener. Die TBS-Datenpfadanbindung muss zusätzlich Ende zu Ende geprüft werden.
