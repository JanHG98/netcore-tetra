# Z01.4 – VM119 Imagebuilder-Vorprüfung

Stand: 2026-10-08. CT150-Readiness/Recovery und CT136-TCP-/Vorschau-/NAS-Erfolgspfad einschließlich isoliertem Fehlermount sind als Betreiberbefunde dokumentiert. Die tatsächliche Vorprüfung auf `VM-H-DEPLOY-01`, VM119, `10.0.1.131`, besteht. Ein vollständiger Buildversuch scheitert anschließend beim Gast-Paketdownload an DNS. Der reproduzierte Dateirechtefehler im Builder ist korrigiert; die Übernahme auf VM119 und der erneute vollständige Build bleiben offen.

## Operatorblock

Als `jhoffmeister` auf **VM-H-DEPLOY-01** ausführen. Der Block liest Status, Profile, gespeichertes Template, Buildjobs, Artefaktmetadaten, Werkzeug-/binfmt-Voraussetzungen und freien Platz. `python3 -B` vermeidet Import-Cachedateien. Die Ausgabe wählt Profil- und Jobmetadaten aus; Zugangsdaten werden für den Build später lokal eingegeben.

```bash
sudo python3 -B - <<'PY'
import hashlib, json, sys, tomllib, urllib.request
from pathlib import Path

sys.path.insert(0, "/usr/local/lib/netcore-deployment")
from common import load_config
from image_build import preflight
from image_spec import BASE

http = urllib.request.build_opener(urllib.request.ProxyHandler({}))
def api(path):
    with http.open("http://10.0.1.131:8320" + path, timeout=20) as response:
        return json.load(response)

def job_summary(job):
    request = job.get("request", {})
    return {**{key: job.get(key) for key in ("id", "status")},
            **{key: request.get(key) for key in
               ("kind", "build_id", "node_id", "service", "commit", "hostname")}}

status = api("/api/v1/status")
profiles = api("/api/v1/profiles")
images = api("/api/v1/images")
jobs = api("/api/v1/jobs")
cfg = load_config("/etc/netcore/deployment.toml")
template = Path(cfg["state_dir"]) / "tbs-site-template.toml"
parsed = tomllib.loads(template.read_text()) if template.is_file() else {}

print(json.dumps({
    "controller": {key: status.get(key) for key in
                   ("node_id", "role", "version", "advertise_url", "has_template")},
    "runtime_jobs_sha256": hashlib.sha256(
        Path("/usr/local/lib/netcore-deployment/jobs.py").read_bytes()).hexdigest(),
    "ref": status.get("settings", {}).get("ref"),
    "checked_commit": status.get("desired", {}).get("commit"),
    "template_sections": {key: isinstance(parsed.get(key), dict)
                          for key in ("net_info", "cell_info", "phy_io")},
    "profiles": [{key: profile.get(key) for key in
                  ("name", "mcc", "mnc", "issi", "la", "cc")} for profile in profiles],
    "builder": {key: images.get(key) for key in
                ("available", "error", "base", "free_bytes")},
    "image_jobs": [job_summary(job) for job in images.get("jobs", [])],
    "artifacts": [{key: artifact.get(key) for key in
                   ("id", "filename", "commit", "sha256", "boot_tested")}
                  for artifact in images.get("artifacts", [])],
    "critical_deployments": [job_summary(job) for job in jobs
                            if job.get("status") in ("queued", "running")
                            or job.get("result", {}).get("remote_uncertain")],
}, indent=2, ensure_ascii=False))

state = Path("/var/lib/netcore-image-builder")
preflight(state)
print("PASS: Buildwerkzeuge, ARM64-Emulation und mindestens 32 GiB frei.")
print("OS-Basisdatei im Cache:",
      (state / "cache" / (BASE["sha256"] + ".img.xz")).is_file())
PY
```

## Auswertung und anschließender Build

- Builder erreichbar: `available=true`, kein Fehler. Offene Image-/Deploymentaufträge oder `remote_uncertain` vor dem Build anhand ihrer bestehenden ID zuordnen.
- Standort-TOML mit `net_info`, `cell_info`, `phy_io` vorhanden und parsebar; TBS-Profile mit Name/MCC/MNC/ISSI/LA/CC erfasst. Vorhandene Standortwerte erhalten.
- Installiertes `image_build.preflight()` prüft root, erforderliche Buildwerkzeuge, ARM64-binfmt mit F-Flag und mindestens 32 GiB frei. Die Prüfung benötigt keinen Rust-Compiler auf der VM; Gast-Buildwerkzeuge werden innerhalb des ARM64-Images eingerichtet.
- Cacheanzeige bestätigt ausschließlich Dateiexistenz. Der Build prüft die feste OS-Basis-SHA-256 und lädt bei Bedarf automatisch das in `image_spec.BASE` definierte Raspberry-Pi-OS-ARM64-Basisimage.
- Den Fingerprint von installiertem `jobs.py` mit dem gemeinsamen Quellstand vergleichen: Der zuletzt bekannte VM119-Rollout war c45a2ec; die später korrigierte SQLite-Auftragsverwaltung wurde in dieser Sitzung bislang auf CT150 installiert. Der Fingerprint ermöglicht die Zuordnung vor dem langen Build.
- Danach vorhandenes Profil, vollständigen gemeinsamen Quellcommit, Hostname und Betriebssystem-Zugang (SSH-Public-Key oder Passwort) wählen; passendes HAT-Overlay festlegen. Für die VPN-Abnahme gültiges Inline-OpenVPN-Profil und vertraute SSIDs lokal angeben. Zugangsdaten im Betrieb eingeben, nicht im Repo ablegen.
- Ein voller ARM64-NetCore-Build erzeugt ein Artefakt samt Manifest/Prüfsumme. Die physische Pi-/SXceiver- und VPN-Abnahme folgt anschließend; `boot_tested=false` bleibt bis zur tatsächlichen Hardwareprüfung maßgeblich.

Primärquellen: `system-backend/deployment-core/main.py`, `common.py`, `image_worker.py`, `image_spec.py`, `image_build.py`. Der Operatorblock ist gegen diese Quellschnittstellen geprüft und syntaktisch validiert. Die tatsächliche VM119-Vorprüfung besteht gemäß nachfolgendem Betreiberbefund.

## Tatsächliche Vorprüfung am 2026-10-08

[Betreiberbefund](evidence/vm119-imagebuilder-preflight-2026-10-08.json): Controller 0.3.0, Builder verfügbar ohne Fehler, ARM64-Emulation und Buildwerkzeuge geprüft, 84.586.332.160 Bytes / rund 78,78 GiB frei. Alle drei Template-Sektionen vorhanden; Profile SRV-M-TBS-01/02/03 mit MCC 901/MNC 1510 und ihren vorhandenen ISSI-/LA-/CC-Werten erfasst. Keine Imagejobs, Artefakte oder kritischen Deploymentaufträge. Die OS-Basisdatei ist noch nicht im Cache; der tatsächliche Build lädt und prüft sie automatisch.

Der Quellabgleich mit c45a2ec und main 0cf6df0 ordnet die installierte `jobs.py` eindeutig zu:

| Datei | SHA-256 | Stand |
| --- | --- | --- |
| Installierte `jobs.py` | `30ce8b0f936383f9c20ddb5ffda244c4065b1de1a567ddf3fbda360058d17e64` | Bytegleich mit c45a2ec, SQLite-Korrektur fehlt auf VM119 |
| Korrigierte `jobs.py` | `905e5fc4d257e5d1ad56542cbaef38d2f42a00af357240c0e5c92ea7facc2c21` | Bytegleich zwischen Fix 31829fc und main 0cf6df0 |

Controller und Imageworker importieren dieselbe installierte Datei. Zwischen c45a2ec und main 0cf6df0 ist sie die einzige geänderte Laufzeitdatei des Deployment-Core; weitere Änderungen dort betreffen Tests. Die API-Versionsnummer allein bestätigt diese Korrektur nicht.

## Eng begrenzte Übernahme vor dem Build

[vm119-jobs-update.py](vm119-jobs-update.py) ersetzt ausschließlich `/usr/local/lib/netcore-deployment/jobs.py` mit geprüftem Fingerprint und startet die beiden vorher untätigen Dienste neu. Kein Installer-, Paket-, Unit- oder Datenbankmigrationslauf. Hostname/Root, aktive Dienste, Controller- und Imageaufträge sowie unveränderte Deployment-Konfiguration, Settings, Profile und Standorttemplate werden geprüft; bei Fehler wird die vorherige Datei aus dem Arbeitsspeicher zurückgesetzt. Ein neuer automatischer Git-Prüfauftrag nach Controllerneustart ist zulässig. Der Helfer startet keinen Imagebuild.

Als jhoffmeister auf VM119 den Checkout auf den bereitgestellten vollständigen Quellcommit setzen, dann `sudo python3 -B "$NC_SRC/Docs/integration/Z01-2026-10-07/vm119-jobs-update.py"` ausführen. Der tatsächliche Austausch und die Nachprüfung auf VM119 bleiben bis zur Betreiber-Ausgabe offen.

Lokale Helperprüfung: Syntax und sechs Testmethoden mit elf isolierten Szenarien bestehen, einschließlich tatsächlichem Dateiaustausch / Rücktausch, Eigentümer-/Moduserhalt, unveränderten Konfigurationsbytes, echten SQLite-Dateien, aktiven / ungeklärten Aufträgen und automatischem Git-Check nach Start. Dienststeuerung und HTTP sind dabei simuliert; dies ersetzt keine VM119-Ausführung. Reproduktion: `python3 -B -m unittest discover -s tests/integration -p test_vm119_jobs_update.py -v`. [Tests](../../../tests/integration/test_vm119_jobs_update.py).

## Erster vollständiger Build

Vorhandenes Profil SRV-M-TBS-01 ist die vorgeschlagene erste Auswahl; die anderen Profile bleiben auswählbar. Einen eindeutigen Testhostname und einen aktuellen vollständigen Quell-SHA pro Build angeben, der die SQLite-Korrektur enthält. Dafür muss der globale Controllerref c45a2ec nicht geändert werden. SSH-Public-Key oder Betriebssystem-Passwort sowie gegebenenfalls WLAN-/OpenVPN-Daten lokal eingeben; keine Geheimnisse in Chat, Repo oder Diagnoseausgabe übernehmen.

Den Build genau einmal über `/api/v1/images/build` oder die Imagebuilder-WebUI absenden. Bei unklarer Antwort anhand der bestehenden Builds prüfen und den POST nicht wiederholen. Imagejobs werden über `/api/v1/images` unter `jobs[]` verfolgt; Controller-Job-URLs gehören zu einer anderen Auftragsdatenbank. `job.id` und `request.build_id` sind unterschiedliche IDs. Nach `succeeded` ist `request.build_id` beziehungsweise `result.id` die Artefakt-ID; Image, SHA-256 und Manifest sind unter `/api/v1/images/<Artefakt-ID>/image`, `/sha256`, `/manifest` abrufbar. Manifestcommit und Dateiprüfsumme vergleichen. Die anschließende physische Pi-/SXceiver- und VPN-Abnahme bleibt getrennt; `boot_tested=false` ist bis dahin korrekt.

### Bestehende WebUI bedienen

Nach bestandener Updateprüfung `http://10.0.1.131:8320` öffnen, Abschnitt **Vom Profil zur Basisstation.**:

| Feld | Erste Auswahl |
| --- | --- |
| TBS-Profil | SRV-M-TBS-01 |
| Branch, Tag oder Commit | Vollständiger aktueller Quell-SHA aus dem Operatorauftrag; gilt nur für diesen Build |
| Adresse dieser VM | http://10.0.1.131:8320 |
| Hostname des Pi | z014-pi-01 |
| Benutzer auf dem Pi / Zeitzone | jan / Europe/Berlin |
| SSH-Public-Key / Oder: Betriebssystem-Passwort | Lokal vollständigen Public-Key oder eigenes Passwort eingeben |
| SXceiver-HAT | EEPROM-Auswahl bei programmiertem HAT-EEPROM; andernfalls SX1255-Overlay aus dem Image |

Das EEPROM lässt sich aus der SXceiver-Platinenversion allein nicht ableiten. WLAN und VPN für einen ersten LAN/DHCP-Build bei Bedarf leer lassen; für die VPN-Abnahme das echte Inline-OpenVPN-Profil und die vertrauten lokalen SSIDs angeben. **Image erstellen →** genau einmal anklicken, danach unter **Builds & Downloads** / **Buildprotokoll** verfolgen. Nach Erfolg **Image ↓**, **SHA256** und **Manifest** herunterladen.

Der eigene OS-Hostname ändert nicht die Stationsidentität: Agent-node_id und Funkkennungen stammen aus dem ausgewählten Profil. Ein reiner Imagebuild beeinflusst keine laufende Station; der spätere physische Parallelboot benötigt eine bewusst gewählte Stationsidentität. Quellen: `static/index.html`, `static/app.js`, `image/personalize.py`, `image/check-hardware.py`.

## Tatsächlicher ARM64-Buildabbruch: Gast-DNS

[Betreiber-Logbefund vom 08.10.2026](evidence/vm119-imagebuilder-dns-2026-10-08.json): Build-ID `98a096a050284d6e952bdff78eff6caa` erreicht ARM64-chroot/Paketdownload; `deb.debian.org` und `archive.raspberrypi.com` melden wiederholt `Temporary failure resolving`. APT/chroot endet100, äußerer unshare-Aufruf1. Der Auszug dokumentiert sämtliche Unmounts und Detach von `/dev/loop20`. Ein Artefakterfolg oder eine Bestätigung des vorausgehenden jobs.py-Austauschs ist darin nicht enthalten.

**Konkreter Quellfehler:** Die Builder-Unit setzt `UMask=0077`. `Disk.guest()` schreibt die temporäre `/etc/resolv.conf` neu, ohne ihren Modus festzulegen; dadurch entsteht root-eigenes 0600. Ein tatsächlicher lokaler Datei-/UMask-Repro bestätigt diesen Modus. APT 2.6.1 verwendet für Netzwerkabrufe standardmäßig `_apt`, der diese Datei nicht lesen kann. Primärquellen: [APT-Benutzer](https://sources.debian.org/src/apt/2.6.1/apt-pkg/init.cc/), [Privilegienwechsel](https://sources.debian.org/src/apt/2.6.1/methods/aptmethod.h/). Dies erklärt den Logbefund; der damalige Gast wurde nicht direkt auf Rechte/effektive Unit/DNS untersucht.

**Gezielte Korrektur:** Der temporäre Gastresolver erhält explizit 0644; die originale Datei oder ihr Symlink wird nach dem Gastkontext wiederhergestellt. Worker-UMask 0077 und Hostresolver bleiben erhalten. Vor dem Paketbuild prüft `guest_dns()` als tatsächlicher Benutzer `_apt` die Lesbarkeit und IPv4-Auflösung beider Paketserver, mit begrenzten Zeitlimits. Die Prüfung läuft im realen ARM64-chroot beim nächsten Betreiberbuild. Der Builder teilt das Netz der VM; der Host-Stub 127.0.0.53 muss deshalb nicht pauschal ersetzt werden.

[vm119-image-dns-update.py](vm119-image-dns-update.py) verwendet die bereits geprüften Host-/Idle-/Dateierhalt-/Rückweg-Guards und ersetzt ausschließlich `/usr/local/lib/netcore-deployment/image_build.py`, mit festen vorher/nachher-SHA-256-Werten. Der bestehende Jobs-Helfer wurde dafür nur allgemein beschriftet. Kein Installer-, Paket-, Unit-, Host-DNS- oder Konfigurationswechsel. Ausführen als jhoffmeister auf VM119 nach Checkout des bereitgestellten vollständigen Korrekturcommits:

```bash
sudo python3 -B "$NC_SRC/Docs/integration/Z01-2026-10-07/vm119-image-dns-update.py"
```

Danach die WebUI neu laden und **einen neuen Build** mit demselben gewünschten Profil und dem vollständigen Korrekturcommit im Feld **Branch, Tag oder Commit** anlegen. Der bisherige fehlgeschlagene Auftrag wird nicht erneut abgesendet oder manuell weiterbearbeitet. Die geprüfte komprimierte OS-Basis kann aus dem Downloadcache verwendet werden; der neue Buildercode ändert automatisch den Recipekey. Der Logauszug liefert keinen Grund für manuelle Mount-, Loop- oder Cache-Bereinigung.

Lokale Regressionen verwenden echte Dateien, Umask, Gastkontext und SQLite-Dateien, simulieren aber Mounts/Geräteknoten, Dienste und externe DNS-Aufrufe. Temporäres 0644, Inhalt/Originalmodus, dangling Symlink, fehlender Resolver sowie Wiederherstellung bei Fehler sind geprüft. DNS-Tests prüfen `_apt`, beide Hosts, Zeitlimits und sofortigen Abbruch auf Lesbarkeits-/Auflösungsfehler. Der tatsächliche ARM64-DNS-/APT-Erfolg und der vollständige Imagebuild bleiben bis zur Betreiber-Ausgabe offen.

Validierung des finalen Korrekturstands: Deployment-Core-Suite 62 Tests, 61 bestanden/ein erwarteter Unix-Socket-Skip; VM119-Update-Suite 11/11 bestanden. Syntax und unabhängiger Review bestehen. [Gastregression](../../../system-backend/deployment-core/tests/test_image_guest.py), [Update-/Rückwegregression](../../../tests/integration/test_vm119_image_dns_update.py).
