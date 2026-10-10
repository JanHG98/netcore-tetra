# SDS Router im LXC bereitstellen

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [install/install.sh](../../../system-backend/sds-router/install/install.sh) · [config/sds-router.example.toml](../../../system-backend/sds-router/config/sds-router.example.toml) · [systemd/netcore-sds-router.service](../../../system-backend/sds-router/systemd/netcore-sds-router.service). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

## Voraussetzungen

Labor-Richtwert: Debian-LXC; persistenten Speicher für Nachrichten, Idempotenzdatenbank und Backup vorsehen. Dies ist keine verifizierte Mindest- oder Produktivkapazität.

Eigener Debian-LXC mit einer im Managementnetz erreichbaren Adresse; DHCP mit fester Zuordnung ist ebenso möglich wie statische Konfiguration. Die Ressourcenangaben sind Labor-Richtwerte, keine gemessene Kapazitätszusage. CPU und Speicherbedarf unter tatsächlicher Teilnehmer- und Datenlast messen.

Der Installer baut aus dem vollständigen Checkout mit `cargo build --release -p netcore-sds-router`. Build-Werkzeuge, `pkg-config`, `iproute2` und eine Rust-Toolchain müssen vorhanden sein. `cargo` muss auch für den ausführenden root-/sudo-Prozess erreichbar sein; eine ausschließlich im Benutzerprofil installierte Toolchain ist dafür nicht automatisch sichtbar.

## Konfiguration vorbereiten

Alle relativen Befehle ab dem Repository-Root ausführen. Bei abweichendem Checkout den `cd`-Pfad anpassen. Eine bestehende Konfiguration erhalten und direkt bearbeiten; die Beispielkopie ist nur für die erste Einrichtung:

```bash
cd /opt/netcore-tetra
sudo install -d /etc/netcore
# Nur bei der ersten Einrichtung:
sudo test -f /etc/netcore/sds-router.toml || sudo cp system-backend/sds-router/config/sds-router.example.toml /etc/netcore/sds-router.toml
sudo editor /etc/netcore/sds-router.toml
```

`[node_gateway].url` anpassen. Nachrichten und dauerhafte Idempotenzschlüssel liegen in `/var/lib/netcore-sds-router/messages.json` plus Backup. Für zentrale Übergabe auf jeder betroffenen TBS `[control_room].central_sds_routing = true` setzen; nach Konfigurationsänderung TBS neu starten. `delivered` ist keine garantierte Funkgerätequittung.

## Installieren und Dienst prüfen

```bash
cd /opt/netcore-tetra
sudo system-backend/sds-router/install/install.sh
systemctl status netcore-sds-router --no-pager
journalctl -u netcore-sds-router -n 100 --no-pager
source /etc/netcore/lxc-network.env
curl "${NETCORE_WEBUI_URL}health/live"
curl "${NETCORE_WEBUI_URL}health/ready"
curl "${NETCORE_WEBUI_URL}api/v1/status"
```

Der Installer bindet die WebUI/API an die erkannte LXC-Adresse und speichert sie in `/etc/netcore/lxc-network.env`. Deshalb ist `127.0.0.1:8150` nach dieser Installation nicht zwingend erreichbar. Bei mehreren Interfaces kann die Adresse über `NETCORE_LXC_IP` bewusst vorgegeben werden. Die URL im Installerausgabefeld und die tatsächlich gespeicherte Konfiguration prüfen.

Konfiguration: `/etc/netcore/sds-router.toml`; Binary: `/usr/local/bin/netcore-sds-router`; Unit: `netcore-sds-router.service`. Die vorhandenen Skripte `install/update.sh` und `install/uninstall.sh` stehen bei der Quelle; vor Entfernung die dortigen Datenoptionen prüfen.

## Netzwerk und Abnahme

TCP `8150` nur aus dem isolierten Management-/Testnetz freigeben. Die fachlichen Abhängigkeiten oben müssen erreichbar sein. Die Dienste implementieren hier keine Managementanmeldung oder TLS. `health/live` belegt nur den Prozess; `health/ready`, Statusfelder, fachliche Antworten und bei Funkfunktionen tatsächliche Gerätebeobachtung getrennt bewerten. Diese Anleitung enthält keine bereits ausgeführte LXC- oder On-Air-Abnahme.
