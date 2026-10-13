# NetCore-Dienste aktualisieren

Stand: 13.10.2026 · Aufgabe **Z01.4** · geprüfter Ausgangsstand `main@5c83bfbdd48bd3de257125ae68a57e584c77ee86`.

Der folgende Block läuft **als root in der Proxmox-Host-Shell**. Er aktualisiert die installierten, verwalteten NetCore-Programme in den **laufenden LXCs dieses Hosts**. CT-IDs und IP-Adressen werden nicht geraten. Bei einem Cluster den Block auf jedem Host mit NetCore-LXCs ausführen. Der Pi und die Deployment-VM sind keine LXCs und haben eigene Schritte unten.

## Ein gemeinsamer Proxmox-Block: neuester main-Stand

```bash
(
set -euo pipefail
NC_UPDATE_SRC=$(mktemp -d /var/tmp/netcore-lxc-source.XXXXXX)
git -C "$NC_UPDATE_SRC" init -q
git -C "$NC_UPDATE_SRC" fetch --depth 1 \
  https://github.com/JanHG98/netcore-tetra.git main
git -C "$NC_UPDATE_SRC" checkout --detach FETCH_HEAD
NC_UPDATE_COMMIT=$(git -C "$NC_UPDATE_SRC" rev-parse HEAD)
printf 'Ausgerollter main-Commit: %s\n' "$NC_UPDATE_COMMIT"
python3 -B "$NC_UPDATE_SRC/tools/operator/update-netcore-lxcs.py" --apply
)
```

Voraussetzungen: `git` und Python3.11+ auf Proxmox; Python3.11+ und eine bestehende NetCore-Installation im Ziel-LXC. Der Lauf löst `main` beim Start genau einmal auf und verwendet diesen vollständigen Commit für alle Gäste. Für Rust-Rollen richtet die Vorbereitung fehlende Build-Pakete und eine explizite Root-stable-Toolchain ein; sie führt kein Betriebssystem-Upgrade aus. Der persistente Build-Cache liegt unter `/var/cache/netcore-service-update/target`. `--no-prepare-build-deps` lässt diese Einrichtung bei bewusst vorbereiteten Gästen aus.

Ohne `--apply` zeigt der Helfer nur das erkannte Inventar. Mit wiederholtem `--ctid 136 --ctid 150` lässt sich der Lauf auf nachweislich passende lokale Container beschränken; dies sind Beispiele für die Syntax, keine vollständige Zuordnung der Anlage.

### Ablauf und Ergebnisse

1. `pct list` erfasst die lokalen Container. Gestoppte LXCs werden protokolliert und nicht gestartet.
2. Installierte Dienste werden anhand von `LoadState=loaded` und `FragmentPath` erkannt. Eine vorhandene `managed_services`-Liste in `/etc/netcore/discovery.toml` begrenzt die zentralen Rollen. Fremde oder historisch zusätzlich kopierte Units außerhalb dieser Liste werden nicht ausgerollt.
3. Ein einziges Git-Quellpaket wird erstellt, mit `pct push` verteilt und im Gast gegen SHA-256 geprüft. Alle geplanten Runtime-/Konfigurationspfade werden vor dem ersten Programmaustausch geprüft.
4. **Vollständige Vorbereitung aller geplanten Dienste vor dem ersten Austausch:** alle Runtime- und neuen Paketdateien prüfen, Rust mit `--locked --release` bauen, Python-Syntax prüfen und unveränderliche Dateikopien mit SHA-256 bereitstellen. Scheitert eine Vorbereitung, wurde noch keine geplante NetCore-Runtime ausgetauscht.
5. **Anwenden in Abhängigkeitsreihenfolge:** vorhandene Programmdateien sichern, neue eigene Module/Static-Dateien ebenfalls installieren, passende eigene Python-Bytecode-Caches invalidieren und zuvor aktive Dienste neu starten. Das erfolgt auch bei identischen Dateien, damit kein alter Prozess als neuer Stand ausgegeben wird. Inaktive Dienste bleiben inaktiv; abgestürzte Dienste bekommen einen Recovery-Start.
6. **Tatsächlichen Stand prüfen:** installierte Dateien müssen den vorbereiteten SHA-256 entsprechen. Aktive Dienste benötigen eine neue systemd-Invocation und einen passenden MainPID; bei Rust wird das laufende `/proc/<PID>/exe` gegen das gebaute Binary geprüft, bei Python der neue Prozess und sein kanonischer Skriptpfad. HTTP-Liveness und Readiness bleiben separate Fachprüfungen. Bei Austausch-/Start-/Prozessfehlern werden alte Dateien zurückgesetzt und neu angelegte Dateien entfernt; weitere Ziele werden als nicht ausgeführt protokolliert.
7. **Versionsnachweis veröffentlichen:** erst nach erfolgreicher Datei-/Prozessprüfung werden die Runtime-Quittung und Discoverys `deployed-<service>.json` mit dem tatsächlichen Commit geschrieben. Der Discovery-Agent meldet seinen eigenen Stand als `source_commit`. Ein eigener Zustand `installing`/`failed` darf keinen alten Commit als erfolgreich neu installiert ausgeben. UI-Jobs und CLI-Austausch verwenden eine gemeinsame Sperre; laufende/ungeklärte Alt-Agentenjobs verhindern einen Austausch.

Readiness wird gesondert bewertet. `updated_degraded` bedeutet: neue Runtime läuft, Fachbereitschaft ist eingeschränkt. Die übrigen Updates laufen weiter, damit noch alte Abhängigkeiten ebenfalls aktualisiert werden können. Der Gesamtlauf endet mit Code2 statt einem uneingeschränkten Erfolg.

Der abschließende Pfad **Bericht: .../report.json** enthält Commit, erkannte Container und den tatsächlichen Zustand jedes Ziels: vorbereitet, installiert/verifiziert, eingeschränkt, fehlgeschlagen oder nicht ausgeführt. Die Quittungen enthalten den installierten Commit, Dateihashes und Prozessnachweise; der Host übernimmt die wirklichen Gast-Ergebnisse statt pauschal jeden Exit0 als „updated“ auszugeben. Daneben liegt pro CT/Dienst ein Build-/Updateprotokoll. Bei Code1 steht dort der Abbruchpunkt. Nicht inspizierbare Container blockieren `--apply`; erst Ursache prüfen oder die passenden CT-IDs gezielt wählen. Ein `completed` gilt ausschließlich für die tatsächlich erkannten und geplanten Rollen, nicht als Abnahme aller Systeme oder Funkfunktionen.

Konfigurationen, Zugangsdaten, Datenbanken, Aufnahmen, Archivpfade und Sprachmodelle bleiben erhalten. Bestehende effektive Units und Discovery-/Bediener-Drop-ins behalten ihre Startpfade; eigene neue Runtime-Dateien kommen vollständig hinzu. Es gibt keine Seed-Importe, Archivmigrationen, Piper-Neuinstallation, Betriebssystem-Upgrades oder Reboots. Observabilitys bereits installierter Python-Loghelfer gehört zum Update; bei geänderten Dateien werden die zuvor aktiven betroffenen Preview-/Archivunits koordiniert gestoppt und wieder gestartet. Das ist ein Runtime-Update bestehender Installationen, keine Neuinstallation oder allgemeine Schema-/Unitmigration.

## Vollständige Rollenliste

Diese 26 zentralen NetCore-Rollen sind im Updateweg enthalten. Die Dateien des [vorhandenen Agentenkatalogs](../../system-backend/deployment-core/catalog.json) liefern Unit, Konfigurationspfad, Binary und Abhängigkeiten.

| Dienst | Auswahl für Einzelupdate |
| --- | --- |
| Node Gateway | `node-gateway` |
| Mobility Core | `mobility-core` |
| Subscriber Core | `subscriber-core` |
| Group Core | `group-core` |
| Call Control | `call-control` |
| Media Switch | `media-switch` |
| Recorder | `recorder` |
| SDS Router | `sds-router` |
| Packet Core | `packet-core` |
| IP-Gateway | `ip-gateway` |
| Security Core | `security-core` |
| KMF | `kmf` |
| Transit | `transit` |
| Application Gateway | `application-gateway` |
| Media Library | `media-library` |
| IoT Gateway | `iot-gateway` |
| Hardware Gateway | `hardware-gateway` |
| RF Monitor | `rf-monitor` |
| Alarm Workflow | `alarm-workflow` |
| Task Workflow | `task-workflow` |
| Asset Management | `asset-management` |
| SIP Switch | `sip-switch` |
| Control Room | `control-room` |
| Observability | `observability` |
| Warnzentrale | `alert-service` |
| Provisioning Core, sofern installiert | `provisioning-core` |

Zusätzlich werden bereits installierte **Directory** (`directory`) und **Discovery-Agenten** (`discovery`) aktualisiert. Directory behält den tatsächlich in seiner Unit eingetragenen historischen oder neuen Skriptnamen; dadurch wird seine Datenbank nicht durch einen anderen Dateinamen ersetzt. Discovery bleibt beim bisherigen Modus, Inventar, Profil und Standort.

### Einzelner Dienst direkt im LXC

Dieser Block ist unabhängig vom Proxmox-Lauf. Auf dem jeweiligen Dienstserver als root oder als Benutzer mit sudo ausführen. Nur `NC_SERVICE` auf den Rollenwert aus der Tabelle setzen; hier ist Packet Core als konkretes Beispiel gewählt.

```bash
(
set -euo pipefail
NC_SERVICE=packet-core
NC_UPDATE_SRC=$(mktemp -d /var/tmp/netcore-service-source.XXXXXX)
git -C "$NC_UPDATE_SRC" init -q
git -C "$NC_UPDATE_SRC" fetch --depth 1 \
  https://github.com/JanHG98/netcore-tetra.git main
git -C "$NC_UPDATE_SRC" checkout --detach FETCH_HEAD
NC_UPDATE_COMMIT=$(git -C "$NC_UPDATE_SRC" rev-parse HEAD)
printf 'Ausgerollter main-Commit: %s\n' "$NC_UPDATE_COMMIT"
sudo python3 -B "$NC_UPDATE_SRC/tools/operator/update-netcore-service.py" \
  --service "$NC_SERVICE" --commit "$NC_UPDATE_COMMIT" --prepare-build-deps --apply
)
```

Die Runtime-Dateien und Installationspfade sind in [service-update-assets.json](../../tools/operator/service-update-assets.json) dokumentiert. Abweichende ExecStart-, Binary-, Script- oder Konfigurationspfade werden vor dem Austausch abgelehnt. Nicht blind einen Pfad umbenennen: zunächst die effektive Unit prüfen.

## Deployment-VM119 separat aktualisieren

In der SSH-Sitzung **jhoffmeister auf VM-H-DEPLOY-01 (`10.0.1.131`)**:

```bash
(
set -euo pipefail
NC_FIX_COMMIT=d3ea8283f3f837296742a2445070364ad7fa4813
NC_FIX_SRC=$(mktemp -d /tmp/netcore-image-features.XXXXXX)
git -C "$NC_FIX_SRC" init -q
git -C "$NC_FIX_SRC" fetch --depth 1 \
  https://github.com/JanHG98/netcore-tetra.git "$NC_FIX_COMMIT"
git -C "$NC_FIX_SRC" checkout --detach FETCH_HEAD
test "$(git -C "$NC_FIX_SRC" rev-parse HEAD)" = "$NC_FIX_COMMIT"
sudo python3 -B \
  "$NC_FIX_SRC/Docs/integration/Z01-2026-10-07/vm119-image-features-update.py"
)
```

Das ist der bereits geprüfte, konsolidierte Rezept-/Formular-Updater. Aktive/ungeklärte Imagejobs werden abgelehnt. Erwartet: `passed` oder `already_current`. Danach ein **neues KDE-Image** bauen; ein vorhandener Download wird dadurch nicht verändert. [Image-, Desktop- und SIP-Anleitung](../integration/Z01-2026-10-07/image-ui-desktop-fixes-2026-10-12.md).

## Pi und externe Anwendungen

- **Physischer Pi `10.0.1.20`:** Die aktuellen Carrier-/Telegram-/Nachbar-/WLAN-/Medienrechte-/Desktop-/SIP-Fallback-Korrekturen kommen über das neu gebaute Image. Der Proxmox-Lauf startet keinen Funkdienst und überschreibt den Pi nicht.
- **Piper/TTS:** externer Provider innerhalb der Media-Library-Installation, keine eigene neue NetCore-Programmquelle. Provider, virtuelle Umgebung, Modelle und Cache bleiben erhalten; das Media-Library-Backend und seine eigenen Codehelfer werden aktualisiert, ohne Archivmigrationen auszuführen.
- **FreePBX/Asterisk und Home Assistant:** externe Anwendungen. Asterisk-AGI des NetCore-SIP-Switches ist enthalten; PBX-, SIP-Zugangsdaten und Routen werden nicht automatisch eingerichtet.
- **Brew:** Der vorhandene Discovery-Agent kann aktualisiert werden. Die Brew-Anwendung selbst hat mehrere inkompatible historische Python-/Rustvarianten; ein ungeprüftes Überschreiben könnte den Backhaul-Port8080 entfernen. Ihre tatsächliche Installation ist noch nicht belegt. Dieser lesende Block auf dem Brew-Server liefert die nötige Grundlage für seinen konkreten Updateweg:

```bash
systemctl show brew-server.service \
  -p LoadState -p FragmentPath -p ExecStart -p WorkingDirectory
ss -ltnp
for NC_PATH in /var/opt/brew-server/app.py /opt/brew-server/brew-server; do
  if [ -f "$NC_PATH" ]; then sha256sum "$NC_PATH"; fi
done
```

## Prüfstand

Ausgangsvergleich: Die zuletzt gemeldeten zentralen Fehler liegen in den Rust-Runtimes von Packet Core, IP-Gateway, Media Library und Observability. Zwischen `0770e903` und `e1617b28` wurden ihre zentralen systemd-Units nicht geändert; das Erhalten der installierten Units berücksichtigt bestehende Discovery-Startwrapper. Der geänderte Medien-Datenpfad-Dienst gehört zum Pi-Image und wird dort durch das Rezept geliefert.

Der korrigierte Helfer besitzt 85 gezielte Regressionen für vollständige Vorbereitung, Quellidentität, neue Paketdateien, Prozess-/Dateinachweise, Versionsmarker, Busy-Jobs und Rückwege. Unter Windows bestanden 82 Fälle; drei echte POSIX-Checks laufen zusätzlich unter Linux. Der Build-Helfer hat zwölf Prüfungen einschließlich zehn tatsächlicher Bash-Fixtures. Der eigene Workflow `NetCore runtime update native proof` führt auf Ubuntu24.04 und26.04 zusätzlich echte systemd-/HTTP-Fälle aus: neues Python-Modul und Static-Datei, unveränderlicher vorbereiteter Stand, Neustart/Commitanzeige, fehlgeschlagener Start mit Rückweg sowie ein real kompilierter Rust-HTTP-Prozess mit `/proc/<PID>/exe`-Hash. Das sind isolierte Fixtures, kein tatsächlicher Packet-Core- oder Flottenrollout auf Jans Anlage.

Eine tatsächliche Aktualisierung auf Jans Proxmox/LXCs und deren Fach-/Funkabnahme ist noch nicht ausgeführt. Betriebsprotokolle des Operators sind der nächste Nachweis für Z01.4.

