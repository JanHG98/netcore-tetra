# Recorder im LXC bereitstellen

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [install/install.sh](../../../system-backend/recorder/install/install.sh) · [config/recorder.example.toml](../../../system-backend/recorder/config/recorder.example.toml) · [systemd/netcore-recorder.service](../../../system-backend/recorder/systemd/netcore-recorder.service). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

## Voraussetzungen

Labor-Richtwert: Debian 12/13; 2 vCPU und 2 GiB RAM, Datenträger entsprechend Anzahl Streams und Aufbewahrung. Dies ist keine verifizierte Mindest- oder Produktivkapazität.

Eigener Debian-LXC mit einer im Managementnetz erreichbaren Adresse; DHCP mit fester Zuordnung ist ebenso möglich wie statische Konfiguration. Die Ressourcenangaben sind Labor-Richtwerte, keine gemessene Kapazitätszusage. Für den Recorder ausreichend persistenten Speicher einplanen; für Media Switch geringe Netzlatenz beachten.

Der Installer baut aus dem vollständigen Checkout mit `cargo build --release -p netcore-recorder`. Build-Werkzeuge, `pkg-config`, `iproute2` und eine Rust-Toolchain müssen vorhanden sein. `cargo` muss auch für den ausführenden root-/sudo-Prozess erreichbar sein; eine ausschließlich im Benutzerprofil installierte Toolchain ist dafür nicht automatisch sichtbar.

## Konfiguration vorbereiten

Alle relativen Befehle ab dem Repository-Root ausführen. Bei abweichendem Checkout den `cd`-Pfad anpassen. Eine bestehende Konfiguration erhalten und direkt bearbeiten; die Beispielkopie ist nur für die erste Einrichtung:

```bash
cd /opt/netcore-tetra
sudo install -d /etc/netcore
# Nur bei der ersten Einrichtung:
sudo test -f /etc/netcore/recorder.toml || sudo cp system-backend/recorder/config/recorder.example.toml /etc/netcore/recorder.toml
sudo editor /etc/netcore/recorder.toml
```

`[media_switch].tap_url` und `sessions_url` auf dieselbe tatsächliche Instanz setzen. Storage-Mount, Eigentümer `netcore`, freien Mindestplatz, Retention und Fsync-Intervall vor Betrieb prüfen. `allow_delete = false` sperrt nur manuelle Löschung; automatische Retention läuft weiter.

## Installieren und Dienst prüfen

```bash
cd /opt/netcore-tetra
sudo system-backend/recorder/install/install.sh
systemctl status netcore-recorder --no-pager
journalctl -u netcore-recorder -n 100 --no-pager
source /etc/netcore/lxc-network.env
curl "${NETCORE_WEBUI_URL}health/live"
curl "${NETCORE_WEBUI_URL}health/ready"
curl "${NETCORE_WEBUI_URL}api/v1/status"
```

Der Installer bindet die WebUI/API an die erkannte LXC-Adresse und speichert sie in `/etc/netcore/lxc-network.env`. Deshalb ist `127.0.0.1:8140` nach dieser Installation nicht zwingend erreichbar. Bei mehreren Interfaces kann die Adresse über `NETCORE_LXC_IP` bewusst vorgegeben werden. Die URL im Installerausgabefeld und die tatsächlich gespeicherte Konfiguration prüfen.

Konfiguration: `/etc/netcore/recorder.toml`; Binary: `/usr/local/bin/netcore-recorder`; Unit: `netcore-recorder.service`. Die vorhandenen Skripte `install/update.sh` und `install/uninstall.sh` stehen bei der Quelle; vor Entfernung die dortigen Datenoptionen prüfen.

## Netzwerk und Abnahme

TCP `8140` nur aus dem isolierten Management-/Testnetz freigeben. Die fachlichen Abhängigkeiten oben müssen erreichbar sein. Die Dienste implementieren hier keine Managementanmeldung oder TLS. `health/live` belegt nur den Prozess; `health/ready`, Statusfelder, fachliche Antworten und bei Funkfunktionen tatsächliche Gerätebeobachtung getrennt bewerten. Diese Anleitung enthält keine bereits ausgeführte LXC- oder On-Air-Abnahme.

## Persistenten Speicher einbinden

`/var/lib/netcore-recorder` kann als LXC-Mountpoint dienen und muss für Benutzer und Gruppe `netcore` schreibbar sein. Node Gateway, Call Control und Media Switch bilden den Rufpfad; der Recorder darf später starten. Sein Ausfall darf den Ruf nicht blockieren, aber der endliche Replayring begrenzt nachholbare Sprachframes.
