# Node Gateway im LXC bereitstellen

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [install/install.sh](../../../system-backend/node-gateway/install/install.sh) · [config/node-gateway.example.toml](../../../system-backend/node-gateway/config/node-gateway.example.toml) · [systemd/netcore-node-gateway.service](../../../system-backend/node-gateway/systemd/netcore-node-gateway.service). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

## Voraussetzungen

Labor-Richtwert: Debian 12; 2 vCPU, 1–2 GiB RAM, 8 GiB Datenträger. Dies ist keine verifizierte Mindest- oder Produktivkapazität.

Eigener Debian-LXC mit einer im Managementnetz erreichbaren Adresse; DHCP mit fester Zuordnung ist ebenso möglich wie statische Konfiguration. Die Ressourcenangaben sind Labor-Richtwerte, keine gemessene Kapazitätszusage. CPU und Speicherbedarf unter tatsächlicher Teilnehmer- und Datenlast messen.

Der Installer baut aus dem vollständigen Checkout mit `cargo build --release -p netcore-node-gateway`. Build-Werkzeuge, `pkg-config`, `iproute2` und eine Rust-Toolchain müssen vorhanden sein. `cargo` muss auch für den ausführenden root-/sudo-Prozess erreichbar sein; eine ausschließlich im Benutzerprofil installierte Toolchain ist dafür nicht automatisch sichtbar.

## Konfiguration vorbereiten

Alle relativen Befehle ab dem Repository-Root ausführen. Bei abweichendem Checkout den `cd`-Pfad anpassen. Eine bestehende Konfiguration erhalten und direkt bearbeiten; die Beispielkopie ist nur für die erste Einrichtung:

```bash
cd /opt/netcore-tetra
sudo install -d /etc/netcore
# Nur bei der ersten Einrichtung:
sudo test -f /etc/netcore/node-gateway.toml || sudo cp system-backend/node-gateway/config/node-gateway.example.toml /etc/netcore/node-gateway.toml
sudo editor /etc/netcore/node-gateway.toml
```

Die `[service_monitor.targets]` enthalten Beispieladressen. Vor dem ersten Start an die tatsächlichen LXC-Adressen anpassen. TBS `[control_room]` verbindet sich mit `host`, `port = 8080`, `use_tls = false` und `endpoint_path = "/ws/node"`; der Kompatibilitätsname bleibt `netcore-control-room-node-v1`.

## Installieren und Dienst prüfen

```bash
cd /opt/netcore-tetra
sudo system-backend/node-gateway/install/install.sh
systemctl status netcore-node-gateway --no-pager
journalctl -u netcore-node-gateway -n 100 --no-pager
source /etc/netcore/lxc-network.env
curl "${NETCORE_WEBUI_URL}health/live"
curl "${NETCORE_WEBUI_URL}health/ready"
curl "${NETCORE_WEBUI_URL}api/v1/status"
```

Der Installer bindet die WebUI/API an die erkannte LXC-Adresse und speichert sie in `/etc/netcore/lxc-network.env`. Deshalb ist `127.0.0.1:8080` nach dieser Installation nicht zwingend erreichbar. Bei mehreren Interfaces kann die Adresse über `NETCORE_LXC_IP` bewusst vorgegeben werden. Die URL im Installerausgabefeld und die tatsächlich gespeicherte Konfiguration prüfen.

Konfiguration: `/etc/netcore/node-gateway.toml`; Binary: `/usr/local/bin/netcore-node-gateway`; Unit: `netcore-node-gateway.service`. Die vorhandenen Skripte `install/update.sh` und `install/uninstall.sh` stehen bei der Quelle; vor Entfernung die dortigen Datenoptionen prüfen.

## Netzwerk und Abnahme

TCP `8080` nur aus dem isolierten Management-/Testnetz freigeben. Die fachlichen Abhängigkeiten oben müssen erreichbar sein. Die Dienste implementieren hier keine Managementanmeldung oder TLS. `health/live` belegt nur den Prozess; `health/ready`, Statusfelder, fachliche Antworten und bei Funkfunktionen tatsächliche Gerätebeobachtung getrennt bewerten. Diese Anleitung enthält keine bereits ausgeführte LXC- oder On-Air-Abnahme.
