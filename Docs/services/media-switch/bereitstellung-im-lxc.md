# Media Switch im LXC bereitstellen

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [install/install.sh](../../../system-backend/media-switch/install/install.sh) · [config/media-switch.example.toml](../../../system-backend/media-switch/config/media-switch.example.toml) · [systemd/netcore-media-switch.service](../../../system-backend/media-switch/systemd/netcore-media-switch.service). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

## Voraussetzungen

Labor-Richtwert: Debian-LXC; mindestens 2 vCPU und 512 MiB RAM, direkte latenzarme Verbindung zum Node Gateway. Dies ist keine verifizierte Mindest- oder Produktivkapazität.

Eigener Debian-LXC mit einer im Managementnetz erreichbaren Adresse; DHCP mit fester Zuordnung ist ebenso möglich wie statische Konfiguration. Die Ressourcenangaben sind Labor-Richtwerte, keine gemessene Kapazitätszusage. Für den Recorder ausreichend persistenten Speicher einplanen; für Media Switch geringe Netzlatenz beachten.

Der Installer baut aus dem vollständigen Checkout mit `cargo build --release -p netcore-media-switch`. Build-Werkzeuge, `pkg-config`, `iproute2` und eine Rust-Toolchain müssen vorhanden sein. `cargo` muss auch für den ausführenden root-/sudo-Prozess erreichbar sein; eine ausschließlich im Benutzerprofil installierte Toolchain ist dafür nicht automatisch sichtbar.

## Konfiguration vorbereiten

Alle relativen Befehle ab dem Repository-Root ausführen. Bei abweichendem Checkout den `cd`-Pfad anpassen. Eine bestehende Konfiguration erhalten und direkt bearbeiten; die Beispielkopie ist nur für die erste Einrichtung:

```bash
cd /opt/netcore-tetra
sudo install -d /etc/netcore
# Nur bei der ersten Einrichtung:
sudo test -f /etc/netcore/media-switch.toml || sudo cp system-backend/media-switch/config/media-switch.example.toml /etc/netcore/media-switch.toml
sudo editor /etc/netcore/media-switch.toml
```

`[node_gateway].url` sowie `[call_control]`-URLs auf dieselbe tatsächliche Call-Control-Instanz setzen: HTTP-Calls, `/ws/media` und `/api/v1/media/route-ready`. Der reguläre Pfad ist ereignisgesteuert; `reconcile_secs = 15` ist der langsame HTTP-Abgleich.

## Installieren und Dienst prüfen

```bash
cd /opt/netcore-tetra
sudo system-backend/media-switch/install/install.sh
systemctl status netcore-media-switch --no-pager
journalctl -u netcore-media-switch -n 100 --no-pager
source /etc/netcore/lxc-network.env
curl "${NETCORE_WEBUI_URL}health/live"
curl "${NETCORE_WEBUI_URL}health/ready"
curl "${NETCORE_WEBUI_URL}api/v1/status"
```

Der Installer bindet die WebUI/API an die erkannte LXC-Adresse und speichert sie in `/etc/netcore/lxc-network.env`. Deshalb ist `127.0.0.1:8130` nach dieser Installation nicht zwingend erreichbar. Bei mehreren Interfaces kann die Adresse über `NETCORE_LXC_IP` bewusst vorgegeben werden. Die URL im Installerausgabefeld und die tatsächlich gespeicherte Konfiguration prüfen.

Konfiguration: `/etc/netcore/media-switch.toml`; Binary: `/usr/local/bin/netcore-media-switch`; Unit: `netcore-media-switch.service`. Die vorhandenen Skripte `install/update.sh` und `install/uninstall.sh` stehen bei der Quelle; vor Entfernung die dortigen Datenoptionen prüfen.

## Netzwerk und Abnahme

TCP `8130` nur aus dem isolierten Management-/Testnetz freigeben. Die fachlichen Abhängigkeiten oben müssen erreichbar sein. Die Dienste implementieren hier keine Managementanmeldung oder TLS. `health/live` belegt nur den Prozess; `health/ready`, Statusfelder, fachliche Antworten und bei Funkfunktionen tatsächliche Gerätebeobachtung getrennt bewerten. Diese Anleitung enthält keine bereits ausgeführte LXC- oder On-Air-Abnahme.
