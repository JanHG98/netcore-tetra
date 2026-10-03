# Neues NetCore-Design für alle Dienst-WebUIs

Basisstation und Dienstoberflächen liegen gemeinsam auf **`feat/netcore-dashboard-design`**. Die Dienst-UIs verwenden das originale NetCore-Logo, helle Flächen, blaue Akzente und eine horizontale Navigation. Die bestehenden Verwaltungs-APIs und Anmeldungen bleiben die Grundlage der Bedienung. Die geplante zentrale IAM/RBAC-Anbindung wird durch dieses Design-Update nicht aktiviert.

Die Assets werden in die Rust-Binaries, Python-Dateien oder HTML-Dateien eingebettet. In Produktion sind weder Node.js noch ein neuer Frontend-Container erforderlich. Jede Dienstoberfläche bleibt unabhängig erreichbar. TTS/Piper gehört zur Media Library; die Dienstoberfläche für Observability gestaltet die fremden Grafana-/Prometheus-Oberflächen nicht um.

## Umfang und übliche Adressen

Die Ports sind die Vorgaben im Repository. Die tatsächlich installierte Konfiguration hat Vorrang; jede LXC-/VM-IP bleibt ihre eigene Adresse.

| Dienst | Standardport | Paket bzw. Oberfläche |
|---|---:|---|
| Node Gateway | 8080 | Rust `netcore-node-gateway` |
| Subscriber Core | 8100 | Rust `netcore-subscriber-core` |
| Group Core | 8110 | Rust `netcore-group-core` |
| Mobility Core | 8090 | Rust `netcore-mobility-core` |
| Call Control | 8120 | Rust `netcore-call-control` |
| Media Switch | 8130 | Rust `netcore-media-switch` |
| SDS Router | 8150 | Rust `netcore-sds-router` |
| Packet Core | 8160 | Rust `netcore-packet-core` |
| IP Gateway | 8170 | Rust `netcore-ip-gateway` |
| Security Core | 8180 | Rust `netcore-security-core` |
| KMF | 8190 | Rust `netcore-kmf` |
| Transit | 8200 | Rust `netcore-transit` |
| Application Gateway | 8220 | Rust `netcore-application-gateway` |
| Media Library, einschließlich TTS/Piper | 8230 | Rust `netcore-media-library` |
| Recorder | 8140 | Rust `netcore-recorder` |
| Observability | 8210 | Rust `netcore-observability` |
| Provisioning Core | 8125 | Rust `netcore-provisioning-core` |
| IoT Gateway | 8240 | Rust `netcore-iot-gateway` |
| Control Room, Browseroberfläche | 9010 | Rust `netcore-control-room` |
| Warnzentrale, Zugang und eigene Meldungen | 8310 | `system-backend/alert-service/` |
| Alarm Workflow | 8270 | Python und `web-ui/index.html` |
| Task Workflow | 8280 | Python und `web-ui/index.html` |
| Asset Management | 8290 | Python, eingebettete Oberfläche |
| SIP Switch | 8300 | Python, eingebettete Oberfläche |
| Directory | 8095 | Python, eingebettete Oberfläche |
| TBS Connect / Python Brew, einschließlich Anmeldung | 8081 | Python, eingebettete Oberfläche |
| Rust Brew, Legacy-Komponente | 9003 | eigenständiges Rust-Paket `misc/brew-server` |
| Hardware Gateway | 8250 | Python, eingebettete Oberfläche |
| RF Monitor | 8260 | Python, eingebettete Oberfläche |

Die Basisstation wird separat nach [BASISSTATION-DESIGN-UPDATE.md](BASISSTATION-DESIGN-UPDATE.md) aktualisiert. Das Auschecken des Branches allein ersetzt keine bereits installierten Dienst-Binaries oder Python-Dateien.

## 1. Den bestehenden Checkout aktualisieren

Diese Befehle im vorhandenen NetCore-Checkout auf dem jeweiligen Diensthost ausführen. Lokale Änderungen zuerst sichern und bewusst committen oder stashen.

```bash
git status --short --branch
git rev-parse HEAD
git fetch origin --prune
if git show-ref --verify --quiet refs/heads/feat/netcore-dashboard-design; then
    git switch feat/netcore-dashboard-design
    git merge --ff-only origin/feat/netcore-dashboard-design
else
    git switch --track -c feat/netcore-dashboard-design origin/feat/netcore-dashboard-design
fi
git status --short --branch
git rev-parse HEAD
```

Den gesamten Branch beziehen. Einzelne HTML-Dateien reichen bei Rust nicht aus: gemeinsame Design-Assets, Renderer und Diensttemplates werden zusammen kompiliert. Vor einem Rollout auf alle LXCs zunächst einen weniger kritischen Dienst aktualisieren und dessen Oberfläche prüfen. Ein Neustart eines Funk-Kerndienstes kann laufende Verbindungen oder Rufe kurz unterbrechen.

## 2. Einen vorhandenen Rust-Dienst aktualisieren

Beispiel Subscriber Core. Für einen anderen Rust-Dienst den Paketnamen aus der Tabelle verwenden. Auf dem tatsächlich zuständigen Host ausführen. Die bestehende Unit und ihren Binärpfad prüfen; mehrere Dienste verwenden `/opt/netcore-…/bin/` statt `/usr/local/bin/`.

```bash
set -e
UI_PACKAGE='netcore-subscriber-core'
UI_UNIT="${UI_PACKAGE}.service"
systemctl show "$UI_UNIT" -p ExecStart -p User -p MainPID
systemctl cat "$UI_UNIT"
```

`UI_BINARY` ausdrücklich auf die vom Dienst verwendete Binary setzen. Bei einem laufenden Prozess hilft `sudo readlink -f /proc/<MainPID>/exe`; ein eventuell angehängtes ` (deleted)` gehört nicht zum Pfad.

```bash
set -e
UI_BINARY='/usr/local/bin/netcore-subscriber-core'
test -f "$UI_BINARY"
cargo build --release --locked -p "$UI_PACKAGE"
UI_BACKUP="/var/backups/netcore-ui/${UI_PACKAGE}-$(date +%Y%m%d-%H%M%S)"
sudo install -d -m 0700 "$UI_BACKUP"
git rev-parse HEAD | sudo tee "$UI_BACKUP/new-commit.txt" >/dev/null
sudo cp -a "$UI_BINARY" "$UI_BACKUP/binary"
sudo systemctl stop "$UI_UNIT"
sudo install -m 0755 "target/release/$UI_PACKAGE" "$UI_BINARY"
sudo systemctl start "$UI_UNIT"
sudo systemctl is-active "$UI_UNIT"
sudo journalctl -u "$UI_UNIT" --since '5 minutes ago' --no-pager -n 80
printf 'Sicherung: %s\n' "$UI_BACKUP"
```

Nur nach erfolgreichem Build mit dem Stoppen fortfahren. Dieses gezielte Verfahren ersetzt die Binary und belässt Konfiguration, Datenbanken, Benutzer, Units und Netzwerkeinstellungen an ihrem bisherigen Ort. Die allgemeinen Dienst-Updater können zusätzlich die LXC-Endpunktkonfiguration ändern; für einen reinen Designwechsel ist das hier nicht notwendig. Die vorhandenen Build-Abhängigkeiten bleiben erforderlich, beim Control Room insbesondere die nativen Abhängigkeiten des Cargo-Workspaces.

Rückweg mit der zuvor gesicherten Binary:

```bash
sudo systemctl stop "$UI_UNIT"
sudo cp -a "$UI_BACKUP/binary" "$UI_BINARY"
sudo systemctl start "$UI_UNIT"
sudo systemctl is-active "$UI_UNIT"
```

## 3. Python-Dienste mit eingebetteter Oberfläche

Für Hardware Gateway, RF Monitor, Asset Management und SIP Switch wird jeweils die vorhandene ausführbare Python-Datei ersetzt. Erst `ExecStart` prüfen; dann die passende Quelle und das tatsächliche Ziel aus dieser Tabelle wählen.

| Dienst | Quellpfad im Checkout | Übliches installiertes Ziel |
|---|---|---|
| Hardware Gateway | `system-backend/hardware-gateway/src/netcore_hardware_gateway.py` | `/usr/local/bin/netcore-hardware-gateway` |
| RF Monitor | `system-backend/rf-monitor/src/netcore_rf_monitor.py` | `/usr/local/bin/netcore-rf-monitor` |
| Asset Management | `system-backend/asset-management/src/netcore_asset_management.py` | `/usr/local/bin/netcore-asset-management` |
| SIP Switch | `system-backend/sip-switch/src/netcore_sip_switch.py` | `/usr/local/bin/netcore-sip-switch` |

Beispiel RF Monitor:

```bash
set -e
UI_UNIT='netcore-rf-monitor.service'
UI_SOURCE='system-backend/rf-monitor/src/netcore_rf_monitor.py'
UI_BINARY='/usr/local/bin/netcore-rf-monitor'
systemctl show "$UI_UNIT" -p ExecStart -p User
python3 -m py_compile "$UI_SOURCE"
UI_BACKUP="/var/backups/netcore-ui/rf-monitor-$(date +%Y%m%d-%H%M%S)"
sudo install -d -m 0700 "$UI_BACKUP"
sudo cp -a "$UI_BINARY" "$UI_BACKUP/binary"
sudo systemctl stop "$UI_UNIT"
sudo install -m 0755 "$UI_SOURCE" "$UI_BINARY"
sudo systemctl start "$UI_UNIT"
sudo systemctl is-active "$UI_UNIT"
sudo journalctl -u "$UI_UNIT" --since '5 minutes ago' --no-pager -n 80
```

Der Rückweg entspricht Abschnitt 2. Python-Datei und Sicherung reichen; das eingebettete Design lädt keine neuen Dateien aus dem Checkout zur Laufzeit.

## 4. Alarm Workflow und Task Workflow

Die vorhandenen Launcher verwenden die installierte HTML-Datei. Beide Quellen enthalten zusätzlich einen passenden HTML-Fallback für den direkten Start des Python-Moduls. Für diese Dienste müssen Python-Modul und HTML gemeinsam aktualisiert werden; der Launcher und die Unit brauchen für das Design keine Änderung.

Beispiel Task Workflow, analog mit `alarm-workflow` und `netcore_alarm_workflow.py`:

```bash
set -e
UI_UNIT='netcore-task-workflow.service'
UI_MODULE='/usr/local/lib/netcore-task-workflow/netcore_task_workflow.py'
UI_HTML='/usr/local/share/netcore-task-workflow/index.html'
systemctl show "$UI_UNIT" -p ExecStart -p User
python3 -m py_compile system-backend/task-workflow/src/netcore_task_workflow.py
UI_BACKUP="/var/backups/netcore-ui/task-workflow-$(date +%Y%m%d-%H%M%S)"
sudo install -d -m 0700 "$UI_BACKUP"
sudo cp -a "$UI_MODULE" "$UI_BACKUP/module.py"
sudo cp -a "$UI_HTML" "$UI_BACKUP/index.html"
sudo systemctl stop "$UI_UNIT"
sudo install -m 0644 system-backend/task-workflow/src/netcore_task_workflow.py "$UI_MODULE"
sudo install -m 0644 system-backend/task-workflow/web-ui/index.html "$UI_HTML"
sudo systemctl start "$UI_UNIT"
sudo systemctl is-active "$UI_UNIT"
```

Für den Rückweg beide gesicherten Dateien an ihre geprüften Pfade zurückkopieren und den Dienst starten. Die Zustandsdateien und Audit-Logs bleiben erhalten.

## 5. Warnzentrale

Die Warnzentrale hat einen eigenen Updater mit Staging, Sicherung und Rückweg. Bei vorhandener Installation aus dem aktualisierten Checkout ausführen:

```bash
sudo bash system-backend/alert-service/install/update.sh
sudo systemctl is-active netcore-alert-service.service
sudo journalctl -u netcore-alert-service.service --since '5 minutes ago' --no-pager -n 80
```

Die vorhandene Konfiguration, der Zugangsschlüssel und die Zustell-Datenbank bleiben bestehen. Den vom Updater ausgegebenen Sicherungspfad aufbewahren. Zugang und Anzeigemodus prüfen, bevor eigene Meldungen erstellt oder versandt werden. Der Designwechsel schaltet den Versand nicht zusätzlich frei.

## 6. Directory, TBS Connect und Rust Brew

Diese älteren Komponenten besitzen unterschiedliche lokale Installationen. Ihre bestehende Unit bzw. ihren Container-Startbefehl prüfen und den tatsächlich verwendeten Quellpfad ersetzen; für sie keinen angenommenen Dienstnamen verwenden.

- **Directory:** `system-backend/directory/netcore-directory.py` oder die funktional entsprechende Kopie `misc/ID-Server/netcore_directory_server.py`. Die mitgelieferte Unit nennt einen historisch abweichenden Zieldateinamen. Maßgeblich ist die installierte `ExecStart`-Zeile. Datenbank und Seed-Datei nicht ersetzen. Syntax prüfen, laufende Skriptdatei sichern, neue Datei an denselben Pfad kopieren und genau diese Unit neu starten.
- **TBS Connect / Python Brew:** `system-backend/tbs-connect/server.py` bzw. `misc/brew-server.py`. Die bisher verwendete Kopie aktualisieren, vorhandene Einstellungen und Anmeldedaten beibehalten. Vor einem Ersatz lokale Änderungen am alten Skript mit der neuen Fassung vergleichen, weil diese Komponente Einstellungen direkt im Python-Code enthält.
- **Rust Brew:** eigenständiges Paket unter `misc/brew-server/`. Mit `cargo build --release --locked --manifest-path misc/brew-server/Cargo.toml` bauen und die tatsächlich verwendete Binary aus `misc/brew-server/target/release/brew-server` nach Sicherung ersetzen. Die lokale Konfiguration und die vorhandene optionale HTTP-Basic-/TLS-Anmeldung beibehalten. Bei Docker die vorhandene Compose-Konfiguration weiterverwenden und das Image aus dem aktualisierten `misc/brew-server`-Verzeichnis neu bauen; persistente Volumes nicht löschen.

## 7. Oberfläche und Funktionen prüfen

Die bisherigen WebUI-Adressen öffnen und einmal mit `Strg`+`F5` vollständig neu laden. Das Original-Logo und die helle horizontale Navigation müssen sichtbar sein. Bei Bedarf einen privaten Tab verwenden. Die Daten müssen aus dem jeweiligen Dienst stammen; die Vorschauwerte aus den Designbildern werden nicht installiert.

- Jede vorhandene Registerkarte, Suche, Detailansicht und ein kleineres Browserfenster prüfen. Formulare öffnen und ihre bestehenden Werte kontrollieren.
- Bei geschützten Diensten Anmeldung, Abmeldung und Zugriff ohne Sitzung prüfen. Control Room kann abhängig vom bisherigen Startmodus seine vorhandene HTTP-Basic-Anmeldung und lokalen Rollen verwenden; die IAM-Roadmap ist ein separates Vorhaben.
- Beim RF Monitor DSP-Pegel vor der Endstufe und externe HF-Probe auseinanderhalten. Fehlende Proben ergeben fehlende Watt-/VSWR-/PA-Werte. Ein SDR-Pegel in dBFS ist keine kalibrierte Antennenleistung.
- Bei Medienbibliothek und Recorder Vorschau, Freigabe, Aussendung und Legal Hold nach den bisherigen Betriebsregeln prüfen. Für den Funktionstest eine Testdatei verwenden.
- Beim Alarm-/Task-Workflow die Zustände und SDS-Aufträge kontrollieren. Ein eingereihter Auftrag allein bestätigt keinen Empfang am Funkgerät.

Bei Funktionsverlust die gesicherten Programm-/HTML-Dateien zurückspielen. Keine neue Beispielkonfiguration über die vorhandene Installation kopieren. Für einen späteren Build auch den vorherigen Git-Commit aufbewahren und bei Bedarf gezielt wieder auschecken.
