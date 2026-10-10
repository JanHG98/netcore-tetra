# Call Control im LXC bereitstellen

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [install/install.sh](../../../system-backend/call-control/install/install.sh) · [config/call-control.example.toml](../../../system-backend/call-control/config/call-control.example.toml) · [systemd/netcore-call-control.service](../../../system-backend/call-control/systemd/netcore-call-control.service). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

## Voraussetzungen

Labor-Richtwert: Debian 12 oder Ubuntu 24.04; 2 vCPU, 1–2 GiB RAM, 8 GiB Datenträger. Dies ist keine verifizierte Mindest- oder Produktivkapazität.

Eigener Debian-LXC mit einer im Managementnetz erreichbaren Adresse; DHCP mit fester Zuordnung ist ebenso möglich wie statische Konfiguration. Die Ressourcenangaben sind Labor-Richtwerte, keine gemessene Kapazitätszusage. CPU und Speicherbedarf unter tatsächlicher Teilnehmer- und Datenlast messen.

Der Installer baut aus dem vollständigen Checkout mit `cargo build --release -p netcore-call-control`. Build-Werkzeuge, `pkg-config`, `iproute2` und eine Rust-Toolchain müssen vorhanden sein. `cargo` muss auch für den ausführenden root-/sudo-Prozess erreichbar sein; eine ausschließlich im Benutzerprofil installierte Toolchain ist dafür nicht automatisch sichtbar.

## Konfiguration vorbereiten

Alle relativen Befehle ab dem Repository-Root ausführen. Bei abweichendem Checkout den `cd`-Pfad anpassen. Eine bestehende Konfiguration erhalten und direkt bearbeiten; die Beispielkopie ist nur für die erste Einrichtung:

```bash
cd /opt/netcore-tetra
sudo install -d /etc/netcore
# Nur bei der ersten Einrichtung:
sudo test -f /etc/netcore/call-control.toml || sudo cp system-backend/call-control/config/call-control.example.toml /etc/netcore/call-control.toml
sudo editor /etc/netcore/call-control.toml
```

`[node_gateway].url` und `[mobility_core].base_url` enthalten im Beispiel `10.0.1.XX` und müssen ersetzt werden. Die Standardwerte `allow_local_fallback = false` und `accept_stale_route = false` beibehalten, sofern kein gezielter Fallback-Test geplant ist. Für Sprachrufe den Media Switch und seine RouteReady-Rückmeldung berücksichtigen.

## Installieren und Dienst prüfen

```bash
cd /opt/netcore-tetra
sudo system-backend/call-control/install/install.sh
systemctl status netcore-call-control --no-pager
journalctl -u netcore-call-control -n 100 --no-pager
source /etc/netcore/lxc-network.env
curl "${NETCORE_WEBUI_URL}health/live"
curl "${NETCORE_WEBUI_URL}health/ready"
curl "${NETCORE_WEBUI_URL}api/v1/status"
```

Der Installer bindet die WebUI/API an die erkannte LXC-Adresse und speichert sie in `/etc/netcore/lxc-network.env`. Deshalb ist `127.0.0.1:8120` nach dieser Installation nicht zwingend erreichbar. Bei mehreren Interfaces kann die Adresse über `NETCORE_LXC_IP` bewusst vorgegeben werden. Die URL im Installerausgabefeld und die tatsächlich gespeicherte Konfiguration prüfen.

Konfiguration: `/etc/netcore/call-control.toml`; Binary: `/usr/local/bin/netcore-call-control`; Unit: `netcore-call-control.service`. Die vorhandenen Skripte `install/update.sh` und `install/uninstall.sh` stehen bei der Quelle; vor Entfernung die dortigen Datenoptionen prüfen.

## Netzwerk und Abnahme

TCP `8120` nur aus dem isolierten Management-/Testnetz freigeben. Die fachlichen Abhängigkeiten oben müssen erreichbar sein. Die Dienste implementieren hier keine Managementanmeldung oder TLS. `health/live` belegt nur den Prozess; `health/ready`, Statusfelder, fachliche Antworten und bei Funkfunktionen tatsächliche Gerätebeobachtung getrennt bewerten. Diese Anleitung enthält keine bereits ausgeführte LXC- oder On-Air-Abnahme.
