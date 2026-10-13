# NetCore-Dienste aktualisieren

Stand: 13.10.2026 · Aufgabe **Z01.4** · geprüfter Ausgangsstand `e1617b28e6f2e8f79958f88e84bc1dc1606c59ed`.

Der folgende Block läuft **als root in der Proxmox-Host-Shell**. Er aktualisiert die installierten, verwalteten NetCore-Programme in den **laufenden LXCs dieses Hosts**. CT-IDs und IP-Adressen werden nicht geraten. Bei einem Cluster den Block auf jedem Host mit NetCore-LXCs ausführen. Der Pi und die Deployment-VM sind keine LXCs und haben eigene Schritte unten.

## Ein gemeinsamer Proxmox-Block

```bash
(
set -euo pipefail
NC_UPDATE_COMMIT=35dc385ca97fd02c8554db3c5b9afd81ec251d40
NC_UPDATE_SRC=$(mktemp -d /var/tmp/netcore-lxc-source.XXXXXX)
git -C "$NC_UPDATE_SRC" init -q
git -C "$NC_UPDATE_SRC" fetch --depth 1 \
  https://github.com/JanHG98/netcore-tetra.git "$NC_UPDATE_COMMIT"
git -C "$NC_UPDATE_SRC" checkout --detach FETCH_HEAD
test "$(git -C "$NC_UPDATE_SRC" rev-parse HEAD)" = "$NC_UPDATE_COMMIT"
python3 -B "$NC_UPDATE_SRC/tools/operator/update-netcore-lxcs.py" --apply
)
```

Voraussetzungen: `git` und Python3.11+ auf Proxmox; Python3.11+ und funktionierende bestehende Installation im Ziel-LXC. Rust-Rollen benötigen ihre bereits vorhandene Root-Cargo-Toolchain und Build-Abhängigkeiten. Der Helfer setzt `/root/.cargo/bin` in seinen Prozess-PATH und baut mit einem persistenten Cache unter `/var/cache/netcore-service-update/target`; fehlende Voraussetzungen führen zum Abbruch. Neue Pakete und Toolchains werden durch diesen Block nicht installiert.

Ohne `--apply` zeigt der Helfer nur das erkannte Inventar. Mit wiederholtem `--ctid 136 --ctid 150` lässt sich der Lauf auf nachweislich passende lokale Container beschränken; dies sind Beispiele für die Syntax, keine vollständige Zuordnung der Anlage.

### Ablauf und Ergebnisse

1. `pct list` erfasst die lokalen Container. Gestoppte LXCs werden protokolliert und nicht gestartet.
2. Installierte Dienste werden anhand von `LoadState=loaded` und `FragmentPath` erkannt. Eine vorhandene `managed_services`-Liste in `/etc/netcore/discovery.toml` begrenzt die zentralen Rollen. Fremde oder historisch zusätzlich kopierte Units außerhalb dieser Liste werden nicht ausgerollt.
3. Ein einziges Git-Quellpaket wird erstellt, mit `pct push` verteilt und im Gast gegen SHA-256 geprüft. Alle geplanten Runtime-/Konfigurationspfade werden vor dem ersten Programmaustausch geprüft.
4. Aktualisierung in der Abhängigkeitsreihenfolge des vorhandenen Dienstkatalogs. Rust-Build mit `--locked --release` und Python-Syntaxprüfung passieren vor dem Dienststopp.
5. Geänderte Programmdateien werden gesichert und auf ihrem jeweiligen Dateisystem atomar ersetzt. Ein zuvor aktiver Dienst wird anschließend wieder gestartet und auf HTTP-Liveness geprüft. Bereits inaktive Dienste bleiben inaktiv. Ein mit `failed` abgestürzter Dienst bekommt dagegen einen Recovery-Start mit zurückgesetztem Fehlerlimit. Enabled-/Disabled-Zustand und Units werden nicht geändert.
6. Bei Austausch-/Start-/Livenessfehlern werden die gesicherten Programmdateien zurückgesetzt und die zuvor aktiven Dienste erneut gestartet. Der Flottenlauf stoppt an diesem Dienst. Rückwegfehler werden ausdrücklich gemeldet; ein unterbrochener Host/Prozess ist keine Garantie für eine vollständige Rücksetzung.
7. Readiness wird gesondert bewertet. `updated_degraded` bedeutet: neue Runtime läuft, Fachbereitschaft ist eingeschränkt. Die übrigen Updates laufen weiter, damit noch alte Abhängigkeiten ebenfalls aktualisiert werden können. Der Gesamtlauf endet mit Code2 statt einem uneingeschränkten Erfolg.

Der abschließende Pfad **Bericht: .../report.json** enthält Commit, erkannte Container, Ausnahmen, geplante Rollen und Ergebnisse. Daneben liegt pro CT/Dienst ein Build-/Updateprotokoll. Bei Code1 steht dort der Abbruchpunkt. Nicht inspizierbare Container blockieren `--apply`; erst Ursache prüfen oder die passenden CT-IDs gezielt wählen. Ein `completed` gilt ausschließlich für die tatsächlich erkannten und geplanten Rollen, nicht als Abnahme aller Systeme oder Funkfunktionen.

Konfigurationen, Zugangsdaten, Datenbanken, Aufnahmen, Archivpfade, Sprachmodelle und vorhandene systemd-Units/Drop-ins werden von diesem Runtime-Update nicht geschrieben. Es gibt keine Seed-Importe, Archivmigrationen, Piper-Neuinstallation, Betriebssystem-Upgrades oder Reboots. Observabilitys bereits installierter Python-Loghelfer gehört zum Update; bei geänderten Dateien werden die zuvor aktiven betroffenen Preview-/Archivunits koordiniert gestoppt und wieder gestartet. Das ist ein Runtime-Update bestehender Installationen, keine Neuinstallation oder allgemeine Schema-/Unitmigration.

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
NC_UPDATE_COMMIT=35dc385ca97fd02c8554db3c5b9afd81ec251d40
NC_UPDATE_SRC=$(mktemp -d /var/tmp/netcore-service-source.XXXXXX)
git -C "$NC_UPDATE_SRC" init -q
git -C "$NC_UPDATE_SRC" fetch --depth 1 \
  https://github.com/JanHG98/netcore-tetra.git "$NC_UPDATE_COMMIT"
git -C "$NC_UPDATE_SRC" checkout --detach FETCH_HEAD
test "$(git -C "$NC_UPDATE_SRC" rev-parse HEAD)" = "$NC_UPDATE_COMMIT"
sudo python3 -B "$NC_UPDATE_SRC/tools/operator/update-netcore-service.py" \
  --service "$NC_SERVICE" --apply
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
- **Piper/TTS:** externer Provider innerhalb der Media-Library-Installation, keine eigene neue NetCore-Programmquelle. Provider, virtuelle Umgebung, Modelle und Cache bleiben erhalten; das Media-Library-Backend wird aktualisiert.
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

Die gezielten Regressionen umfassen 60 Fälle: 29 für Proxmox-Inventar/Planung und 31 für Runtime-Austausch, Buildfehler, Architektur, Rückweg, Konfigurationserhalt und Dienstzustände. Unter Windows bestanden 59 Fälle; nur der tatsächliche POSIX-Eigentümer-/Modustest ist dort nicht ausführbar. Der ergänzte Linux-CI-Schritt führt denselben Satz einschließlich der nativen Metadatenprüfung aus. Die Tests verwenden echte Dateiaustausche und gemockte systemd-/pct-Aufrufe; sie sind kein ausgeführter Anlagenrollout.

Eine tatsächliche Aktualisierung auf Jans Proxmox/LXCs und deren Fach-/Funkabnahme ist noch nicht ausgeführt. Betriebsprotokolle des Operators sind der nächste Nachweis für Z01.4.

