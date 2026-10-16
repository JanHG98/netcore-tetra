# Deployment-VM auf leeren Anwendungszustand zuruecksetzen

Z01.4, 16.10.2026. Der Betreiber wuenscht einen Neustart mit leerer
Deployment-Anwendung. Das ist keine Neuinstallation des Betriebssystems und
kein Update des installierten Programmcodes.

Der Helfer `tools/operator/reset-deployment-vm.py` entfernt alle fertigen Images,
unvollstaendigen Builddateien, allgemeinen Image-Caches und den Git-Quellcache.
Auftrags- und Sammelupdateverlauf, Profile, Standort-Template, Inventar samt
Entfernungsliste, Endpunkte und im UI gespeicherte Einstellungen werden in der
aktiven Anwendung geleert. Lokale Builds und noch nicht dispatchte Warteschlangen
werden beendet. Aktive oder ungeklaerte LXC-Auftraege verhindern den Reset;
ein Controller-Stopp beendet keinen Installer auf einem anderen Host.

Die VM-IP, SSH, installierte Software, systemd-Units und die installierten Dienste
auf den LXC bleiben erhalten. Die technische Controller-Identitaet,
Erreichbarkeit, Discovery-Netzkonfiguration und erlaubten Netze werden erhalten.
Manuelle Seeds/Bindings und der ausgewaehlte Update-Ref werden zurueckgesetzt;
es gilt danach wieder `main`. Der erste Imagebuild muss seine Basis neu aufbauen.

## Auf VM-H-DEPLOY-01 ausfuehren

Der folgende Block pinnt den ausgecheckten Stand einmal auf einen vollstaendigen
Commit. Ohne `--apply` zeigt der Helfer nur den Umfang und prueft die Voraussetzungen.
Der dargestellte Block fuehrt den vom Betreiber angeforderten Reset aus.

```bash
(
  set -Eeuo pipefail
  NC_RESET_SRC=$(mktemp -d /tmp/netcore-deploy-reset.XXXXXX)
  git -C "$NC_RESET_SRC" init -q
  git -C "$NC_RESET_SRC" remote add origin https://github.com/JanHG98/netcore-tetra.git
  git -C "$NC_RESET_SRC" config http.version HTTP/1.1
  git -C "$NC_RESET_SRC" config remote.origin.promisor true
  git -C "$NC_RESET_SRC" config remote.origin.partialclonefilter blob:none
  git -C "$NC_RESET_SRC" fetch --depth 1 --filter=blob:none origin refs/heads/main
  NC_RESET_REF=$(git -C "$NC_RESET_SRC" rev-parse FETCH_HEAD)
  git -C "$NC_RESET_SRC" sparse-checkout init --cone
  git -C "$NC_RESET_SRC" sparse-checkout set tools/operator
  git -C "$NC_RESET_SRC" checkout --detach "$NC_RESET_REF"
  sudo python3 -B "$NC_RESET_SRC/tools/operator/reset-deployment-vm.py" --apply
)
```

Der Helfer stoppt Controller und Builder mit ihren kompletten Prozessgruppen.
Vor Dateiaenderungen prueft er erneut die Remote-Quittungen, die Cgroups und
auch in anderen privaten Namespaces verbliebene Mounts. Er loest ausschliesslich
Loopgeraete mit dem exakten eigenen `work/<32-stellige-ID>/image.img`-Backing;
fremde Geraete bleiben erhalten. Nach erfolgreichem `losetup --detach` wartet er
hoechstens zehn Sekunden auf das tatsaechliche Verschwinden der ausgewaehlten
Loopgeraete. Der Kernel darf sie verzoegert freigeben; bleiben sie sichtbar,
endet der Reset vor dem Umbenennen oder Loeschen von Anwendungsdateien. Symlinks und abweichende
Unit-/Zustandspfade werden vor dem Reset abgewiesen.

Konfiguration, private Auftragsquittungen, alte Profil-/Inventardaten und ein
begrenzter Dienstjournalausschnitt bleiben unter dem ausgegebenen
`/var/lib/netcore-deployment-resets/<Zeitstempel>-<Zufall>/` gesichert. Der Ordner
hat Modus0700 und ist fuer root bestimmt. Images und reproduzierbare Caches
werden aus dieser Sicherung entfernt; es wird keine grosse Imagekopie angelegt.
Die Zustandsverzeichnisse werden auf demselben Dateisystem umbenannt. Ein Fehler
vor dem vollstaendigen Anlegen des neuen Zustands stellt die umbenannten
Verzeichnisse zurueck. Ein Fehler nach diesem Schritt nennt die Sicherung;
er wird nicht als erfolgreicher Abschluss ausgegeben.

Erfolg ist `phase: completed`, nachdem der echte lokale HTTP-Zugriff auf Readiness,
Builder, leere Image-/Buildlisten, leere Profile und leeren Sammelverlauf besteht.
Danach die WebUI mit Strg+F5 laden. Die automatische Versionspruefung darf sofort
einen neuen Checkauftrag erzeugen. AutoDiscovery darf reale Hosts erneut anzeigen;
das sind keine wiederhergestellten Altauftraege. Standort-TOML, TBS-Profile,
manuelle Bindings und bei Bedarf Seeds werden neu eingerichtet.

## Geprüfter Quellstand

PR #100 ist in `main@e589287457be441f65f480df2386d00f808ccbfc` übernommen. Der produktive
Stand `1dab9ec2` besteht auf Ubuntu24.04 und26.04 jeweils20 native Fälle ohne
Skip, einschließlich eigener/fremder Loopgeräte, privatem Mountnamespace,
SQLite und Rücknahme. Der vollständige Bash-Block ist syntaktisch geprüft.
[Prüfnachweis](../archive/assets/2026-10-16_deploy-abnahme/reset-fortsetzung-nachweise.json).

## Nachweisgrenze

`tools/test_deployment_reset.py` prueft echte temporaere Dateien und SQLite sowie
die Stoppreihenfolge, Abbruch ohne Datenverlust, geschuetzte Sicherungen,
Symlink-/Mount-/Cgroup-Sperren, gezielte Loopauswahl und Ruecknahme bei einem
Umbenennungsfehler. Die Linux-Pruefung laeuft auf Ubuntu24.04 und26.04 als root;
zusaetzlich werden ein echtes eigenes und ein fremdes Loopgeraet sowie ein echter
Mount in einem getrennten Namespace geprueft. systemd-Steuerung und die
abschliessende HTTP-Antwort werden in diesen Tests simuliert. Der tatsaechliche
Reset auf VM119 bleibt ein eigener Betreiberbeleg. Ein Reset beweist keine
Ursache eines vorherigen Buildstillstands.

## Betreiberabbruch vor Änderungen am 16.10.2026

Auf VM-H-DEPLOY-01 blockiert die Vorprüfung am Quellstand `e5892874` folgende
Controlleraufträge, jeweils lokal `failed` und ohne gespeicherte Remote-ID:

| Host | Dienst | Controller-Auftrag |
| --- | --- | --- |
| Observability | observability | `9d4f6ec344c34cafa32529e9145aa801` |
| warn-control | node-gateway | `a96edf31da0c40d1a6f03797a32b738f` |

Der Reset wurde vor Unit-Stopp und Dateiänderungen abgebrochen. Der tatsächliche
Agenten-Endzustand ist dadurch noch nicht bekannt. Zuerst Agentenursprung,
Zielcommit, lokales Versuchzeitfenster und etwaige Originalquittungen im Log lesen.
Danach den [gezielten Endbelegabgleich](netcore-dienste-aktualisieren.md#verlorene-post-antwort-mit-ausdrücklich-gewähltem-agentenauftrag)
für genau diese IDs durchführen. Frühere Zuordnungen anderer Controlleraufträge
sind kein Endbeleg dieser beiden Versuche. Tatsächlicher Abgleich und erneuter
Reset bleiben bis zu eigenen Betreiber-Ausgaben offen.

## Die beiden bestätigten Endbelege übernehmen und Reset fortsetzen

Direktprüfung am16.10.2026 um23:03 Europe/Berlin: Agentenidentität,
Dienst/Aktion, fest angenommener Commit `03422f36920e0a94e0238fc1ceaa13a901177c77`
und Annahmezeit innerhalb des jeweiligen lokalen Versuchzeitfensters stimmen
überein. Die unveränderte `proof()`-Funktion des vorhandenen Abgleichhelfers
bestätigt beide Endbelege mit frischen GETs jeweils zweimal.
[Maschinenlesbarer Befund](../integration/Z01-2026-10-07/evidence/reset-remote-abschluesse-2026-10-16.json).

| Controller-Auftrag | Ausdrücklich zugeordneter Agentenauftrag | Tatsächlicher Endzustand |
| --- | --- | --- |
| `9d4f6ec344c34cafa32529e9145aa801` | `4df693018f55412b901cd51bd2e0e9e8` auf Observability | `failed`, `Command exited with 1: bash` |
| `a96edf31da0c40d1a6f03797a32b738f` | `3ecc5df49ff64ac0a3c3b0cb88a79a0a` auf warn-control / node-gateway | `succeeded`, `live=true`, `ready=true` |

Auf **VM-H-DEPLOY-01** ausführen. Der Block prüft zunächst beide Zuordnungen
lesend und verlangt freie Controlleraufträge. Bei gestopptem Controller prüft
der Helfer erneut, übernimmt nur diese beiden Endbelege und protokolliert die
ursprünglichen Zeilen samt Nachweis in der privaten Datenbank. Ein Fehler nach
dem Controller-Stopp startet ihn über den EXIT-Trap wieder; der Reset folgt erst
nach erfolgreichem Abgleich. Der Observability-Updatefehler wird als Fehler
erhalten, während Node-Gateway seinen tatsächlich erfolgreichen Endstatus erhält.
Der folgende Anwendungsreset sichert diesen Verlauf und die Konfiguration.

```bash
(
  set -Eeuo pipefail
  NC_REF=e589287457be441f65f480df2386d00f808ccbfc
  NC_DIR=$(mktemp -d /tmp/netcore-reset-abgleich.XXXXXX)

  for NC_FILE in reconcile-remote-job.py reset-deployment-vm.py; do
    curl --http1.1 --fail --show-error --retry 3 \
      "https://raw.githubusercontent.com/JanHG98/netcore-tetra/$NC_REF/tools/operator/$NC_FILE" \
      -o "$NC_DIR/$NC_FILE"
  done

  nc_abgleichen() {
    sudo python3 -B "$NC_DIR/reconcile-remote-job.py" \
      9d4f6ec344c34cafa32529e9145aa801 \
      --remote-job 4df693018f55412b901cd51bd2e0e9e8 "$@"
    sudo python3 -B "$NC_DIR/reconcile-remote-job.py" \
      a96edf31da0c40d1a6f03797a32b738f \
      --remote-job 3ecc5df49ff64ac0a3c3b0cb88a79a0a "$@"
  }

  nc_abgleichen

  sudo python3 - <<'PY'
import sqlite3
from contextlib import closing
with closing(sqlite3.connect(
    "file:/var/lib/netcore-deployment/jobs.sqlite3?mode=ro",
    uri=True, timeout=5
)) as db:
    active = db.execute(
        "SELECT id,status FROM jobs WHERE status IN ('queued','running')"
    ).fetchall()
if active:
    raise SystemExit("Noch aktive Controller-Aufträge: " + str(active))
PY

  trap 'sudo systemctl start netcore-deployment.service' EXIT
  sudo systemctl stop netcore-deployment.service
  nc_abgleichen --apply
  sudo systemctl start netcore-deployment.service
  trap - EXIT

  sudo python3 -B "$NC_DIR/reset-deployment-vm.py" --apply
)
```

Der Block ist auf Bash-Syntax geprüft; Abgleichhelfer und Reset sind auf den
vorhandenen geprüften Quellstand gepinnt. Die HTTP-Prüfung war lesend. Die
Datenbankübernahme auf der VM und der tatsächliche Reset sind bis zur passenden
Betreiber-Ausgabe nicht als ausgeführt dokumentiert. Erfolg des Resets ist
`phase=completed`; danach die WebUI mit Strg+F5 neu laden.
