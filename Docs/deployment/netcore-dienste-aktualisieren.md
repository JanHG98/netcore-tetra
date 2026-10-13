# NetCore-Dienste aktualisieren

Stand: 14.10.2026 · Aufgabe **Z01.4** · geprüfter Ausgangsstand `main@5c83bfbdd48bd3de257125ae68a57e584c77ee86`.

Der Proxmox-Block weiter unten läuft **als root in der Proxmox-Host-Shell**. Er aktualisiert die installierten, verwalteten NetCore-Programme in den **laufenden LXCs dieses Hosts**. CT-IDs und IP-Adressen werden nicht geraten. Bei einem Cluster den Block auf jedem Host mit NetCore-LXCs ausführen. Der Pi und die Deployment-VM sind keine LXCs und haben eigene Schritte unten.

## Alle verwalteten Dienste in der WebUI aktualisieren

Nach dem [VM119-Update](#deployment-vm119-separat-aktualisieren) die Deployment-WebUI mit **Strg+F5** neu laden. Unter **Gezielt ausrollen → Alle Updates anzeigen** zunächst die gemeinsame Vorschau laden. Der Controller holt dafür den aktuellen `main` genau einmal und zeigt den vollständigen Zielcommit, jeden installierten verwalteten Dienst und die ausgelassenen Ziele mit Grund. Auch erreichbare Dienste mit eingeschränkter Readiness werden aufgenommen, damit alte Abhängigkeiten aktualisiert werden können. Die Filter in der Netzübersicht beschränken diesen Sammellauf nicht.

Die Vorschau mit **… Dienste jetzt aktualisieren** ausführen. Mehrere Zielhosts werden parallel angestoßen; der Agent arbeitet die Dienste eines Hosts nacheinander ab. Jeder Dienst erhält einen eigenen dauerhaften Auftrag. Die kompakte Tabelle zeigt Host, Dienst, wartend/laufend/erfolgreich/fehlgeschlagen oder ungeklärt; das jeweilige Protokoll bleibt über den Auftragsverlauf erreichbar. Ein Fehler hält unabhängige Ziele nicht an. Ein verlorener Remote-Auftrag wird nicht automatisch erneut abgesendet. Wiederholtes Absenden derselben Vorschau erzeugt keinen zweiten Lauf.

Der Umfang sind die installierten Katalogrollen einschließlich TBS auf erreichbaren Discovery-Agenten. Offlinehosts, Controller und nicht verwaltete Fremdanwendungen werden mit Grund ausgelassen. Discovery-Agenten und die Deployment-VM selbst behalten ihre gesonderten Updatewege. Laufende Verbindungen und Funkbetrieb können durch die Dienstneustarts unterbrochen werden.

### Veraltete Einträge löschen und die Netzübersicht filtern

Die Netzübersicht bietet neben der Textsuche Filter für **Dienste/TBS**, die konkrete **Dienstrolle** und **online/offline**. Filter wirken auf die einzelnen Dienstzeilen, auch wenn mehrere Rollen auf demselben Host liegen.

Über **Löschen** einen veralteten Host aus dem Discovery-Inventar nehmen. Das entfernt seine gecachten Zuordnungen und blendet seine erneuten Meldungen dauerhaft aus; es deinstalliert oder stoppt nichts auf dem Zielhost. Die eigene Instanz und Ziele laufender oder ungeklärter Updateaufträge bleiben erhalten. Unter den entfernten Einträgen kann ein Host ausdrücklich **wiederhergestellt** werden. Manuell festgelegte Dienstbindungen bleiben gespeichert und werden bei einem entfernten Ziel als ungelöst behandelt; für einen neuen Zielhost die Zuordnung in den Einstellungen bewusst ändern.

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

In der SSH-Sitzung **jhoffmeister auf VM-H-DEPLOY-01 (`10.0.1.131`)**, bevor ein neuer Imagebuild oder Sammellauf gestartet wird:

```bash
(
set -euo pipefail
NC_VM_SRC=$(mktemp -d /tmp/netcore-vm-current.XXXXXX)
git -C "$NC_VM_SRC" init -q
git -C "$NC_VM_SRC" fetch --depth 1 \
  https://github.com/JanHG98/netcore-tetra.git refs/heads/main
NC_VM_COMMIT=$(git -C "$NC_VM_SRC" rev-parse FETCH_HEAD)
git -C "$NC_VM_SRC" checkout --detach "$NC_VM_COMMIT"
sudo python3 -B "$NC_VM_SRC/tools/operator/update-vm119-image-features.py" \
  --commit "$NC_VM_COMMIT" --apply
)
```

Dieser Updater übernimmt Controller, Builder, vollständiges Image-Rezept und Formular gemeinsam aus genau diesem Commit. Aktive oder ungeklärte Deployment-/Imagejobs werden abgelehnt. Erwartet: `passed` oder `already_current`. Danach die WebUI mit **Strg+F5** neu laden. Die UI-Funktionen sind sofort auf der VM verfügbar; ein neues Pi-Image wird gesondert gebaut. [Image-, Desktop- und SIP-Anleitung](../integration/Z01-2026-10-07/image-ui-desktop-fixes-2026-10-12.md).


### Ungeklärten Remote-Auftrag vor dem VM-Update abgleichen

Ein lokal `failed` gespeicherter Auftrag mit `remote_uncertain=true` kann auf dem Agenten weiterlaufen oder dort bereits beendet sein. Der VM-Updater und weitere Updates desselben Ziels bleiben deshalb gesperrt. `tools/operator/reconcile-remote-job.py` prüft die gespeicherte exakte Remote-Job-ID über GET, die Agentenidentität, Umgebung, Dienst/Aktion und den damals angenommenen Commit. Ein vollständiger SHA in `ref` gilt auch bei einem alten Auftrag ohne `commit` als feste Vorgabe. Der Helfer löst für diesen alten Auftrag keinen neuen Branchstand auf.

Der folgende Block läuft **auf VM-H-DEPLOY-01**. Er lädt nur die benötigten Quellbereiche, prüft den Auftrag zunächst ohne Änderung und speichert einen übereinstimmenden Endzustand erst bei gestopptem Controller. Den beispielhaften `NC_JOB` nur bei einem anderen ausdrücklich untersuchten Auftrag ersetzen.

```bash
(
  set -Eeuo pipefail
  NC_JOB=193b08c7b05b4b54951baab1eea95df3
  NC_VM_SRC=$(mktemp -d /tmp/netcore-vm-reconcile.XXXXXX)
  git -C "$NC_VM_SRC" init -q
  git -C "$NC_VM_SRC" remote add origin https://github.com/JanHG98/netcore-tetra.git
  git -C "$NC_VM_SRC" config http.version HTTP/1.1
  git -C "$NC_VM_SRC" config remote.origin.promisor true
  git -C "$NC_VM_SRC" config remote.origin.partialclonefilter blob:none
  git -C "$NC_VM_SRC" fetch --depth 1 --filter=blob:none origin refs/heads/main
  NC_VM_COMMIT=$(git -C "$NC_VM_SRC" rev-parse FETCH_HEAD)
  git -C "$NC_VM_SRC" sparse-checkout init --cone
  git -C "$NC_VM_SRC" sparse-checkout set tools/operator system-backend/deployment-core
  git -C "$NC_VM_SRC" checkout --detach "$NC_VM_COMMIT"

  sudo python3 -B "$NC_VM_SRC/tools/operator/reconcile-remote-job.py" "$NC_JOB"
  trap 'sudo systemctl start netcore-deployment.service' EXIT
  sudo systemctl stop netcore-deployment.service
  sudo python3 -B "$NC_VM_SRC/tools/operator/reconcile-remote-job.py" "$NC_JOB" --apply
  sudo systemctl start netcore-deployment.service
  trap - EXIT
  sudo python3 -B "$NC_VM_SRC/tools/operator/update-vm119-image-features.py" \
    --commit "$NC_VM_COMMIT" --apply
)
```

Die Diagnose liefert `phase=verified`, wenn der genaue Remote-Auftrag eindeutig beendet ist. `--apply` prüft ihn erneut, verlangt vollständig inaktive lokale Aufträge einschließlich Checks und vergleicht die komplette ursprüngliche Datenbankzeile innerhalb einer SQLite-Transaktion. Die private Datenbank muss root oder dem tatsächlichen systemd-Dienstbenutzer gehören; der StateDirectory-Pfad wird gegen die effektive Unit geprüft. Vorheriger Zustand und Beleg werden gemeinsam mit der gezielten Änderung in `remote_job_reconciliations` protokolliert. Ein Remote-Fehlschlag oder Abbruch bleibt `failed` beziehungsweise `interrupted`. Wiederholung ist idempotent.

`phase=pending` mit Exitcode2 beendet den Block ohne Abgleich; etwa bei laufendem, nicht erreichbarem oder widersprüchlichem Remote-Auftrag. Nach einem begonnenen Controller-Stop sorgt der EXIT-Trap auch dann für seinen Neustart. Die Endzustandsprüfung sendet kein neues Update. Der abschließende VM-Updater behält seine eigenen Builder-/Jobsperren; weitere ungeklärte Aufträge werden einzeln untersucht.

Betreiberbefund vom 14.10.2026: lokaler Auftrag `193b08c7b05b4b54951baab1eea95df3` für Node-Gateway/`node-gateway`, Update auf `c45a2ec1f5b7cdcfb766a2d81e8901b65f3dbacf`, verweist auf Agent `10.0.1.179:8321` und Remote-Auftrag `721b00fef0b143b28253123f7a168c7c`. Dessen tatsächlicher Endzustand und die anschließende VM-Übernahme sind noch vom Betreiber zu belegen.

## Pi und externe Anwendungen

- **Physischer Pi `10.0.1.20`:** Die aktuellen Carrier-/Telegram-/Nachbar-/WLAN-/Medienrechte-/Desktop-/SIP-Fallback-Korrekturen kommen über das neu gebaute Image. Der Proxmox-Lauf startet keinen Funkdienst und überschreibt den Pi nicht.
- **Piper/TTS:** externer Provider innerhalb der Media-Library-Installation, keine eigene neue NetCore-Programmquelle. Provider, virtuelle Umgebung, Modelle und Cache bleiben erhalten; das Media-Library-Backend und seine eigenen Codehelfer werden aktualisiert, ohne Archivmigrationen auszuführen.
- **FreePBX/Asterisk und Home Assistant:** externe Anwendungen. Asterisk-AGI des NetCore-SIP-Switches ist enthalten; PBX-, SIP-Zugangsdaten und Routen werden nicht automatisch eingerichtet.
- **Brew:** Der vorhandene Discovery-Agent kann aktualisiert werden. Die Brew-Anwendung selbst hat mehrere inkompatible historische Python-/Rustvarianten; ein ungeprüftes Überschreiben könnte den Backhaul-Port8080 entfernen. Für die im historischen Inventar dokumentierte Python-Variante `netcore-brew.service` / `/var/opt/netcore-tetra/server.py` gibt es einen [eigenen Updateblock mit Prüfung der Variante und Erhalt lokaler Einstellungen](../integration/Z01-2026-10-07/image-ui-desktop-fixes-2026-10-12.md#python-brew). Seine Prüfungen müssen zum tatsächlichen Bestand passen. Für andere Brew-Varianten liefert dieser lesende Block auf dem Brew-Server die nötige Grundlage für seinen konkreten Updateweg:

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

Der korrigierte Helfer besitzt 86 gezielte Regressionen für vollständige Vorbereitung, Quellidentität, neue Paketdateien, Prozess-/Dateinachweise, Versionsmarker, Busy-Jobs und Rückwege. Unter Windows bestanden 83 Fälle; drei echte POSIX-Checks laufen zusätzlich unter Linux. Der Build-Helfer hat zwölf Prüfungen einschließlich zehn tatsächlicher Bash-Fixtures. Der eigene Workflow `NetCore runtime update native proof` führt auf Ubuntu24.04 und26.04 zusätzlich echte systemd-/HTTP-Fälle aus: neues Python-Modul und Static-Datei, unveränderlicher vorbereiteter Stand, Neustart/Commitanzeige, fehlgeschlagener Start mit Rückweg sowie ein real kompilierter Rust-HTTP-Prozess mit `/proc/<PID>/exe`-Hash. Das sind isolierte Fixtures, kein tatsächlicher Packet-Core- oder Flottenrollout auf Jans Anlage.

Eine tatsächliche Aktualisierung auf Jans Proxmox/LXCs und deren Fach-/Funkabnahme ist noch nicht ausgeführt. Betriebsprotokolle des Operators sind der nächste Nachweis für Z01.4.



