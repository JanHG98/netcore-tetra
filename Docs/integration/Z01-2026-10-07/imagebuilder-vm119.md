# Z01.4 – VM119 Imagebuilder: Vorprüfung und Buildabnahme

Stand: 2026-10-09. CT150-Readiness/Recovery und CT136-TCP-/Vorschau-/NAS-Erfolgspfad einschließlich isoliertem Fehlermount sind als Betreiberbefunde dokumentiert. Die tatsächliche Vorprüfung auf `VM-H-DEPLOY-01`, VM119, `10.0.1.131`, besteht. Die neue Betreiber-Ausgabe bestätigt jetzt den vollständigen Imagebuilder-Erfolg an1595259 mit Dateiname, Größe, SHA-256 und Abschlussmarker. Das fertige Artefakt ist in der aktuellen Speicheransicht jedoch nicht gelistet und unter dem Standardpfad nicht gefunden. Der passende Softwarecache ist auf VM119 bestätigt. Nach einem unspezifizierten SQLite-Lesefehler ist der Recoveryauftrag inzwischen eindeutig zugeordnet: Cachekopie erfolgreich, losetup-Timeout30s vor Stationspersonalisierung. Als Nächstes den Loop-/Kernel-/Block-I/O-Zustand auf VM119 lesen. Downloadprüfung und physische Abnahme bleiben offen. Die früheren DNS-/Conffile-/Initramfs-Abbrüche und der gesicherte Stand von17:03 bleiben unten ausdrücklich datierte Historie.

## Zugeordneter Recoveryauftrag: Cachehit, losetup-Timeout

Aktueller Betreiberbefund vom09.10.2026: SQLite lässt sich mit `mode=ro` öffnen, das Schema und die Aufträge sind lesbar. Der Recoverybeleg steht auf `submit_started`; eine Auftrags-ID ist noch nicht eingetragen. Der identische private Request-Fingerabdruck, ohne `build_id`, findet außerhalb der gespeicherten Baseline genau einen Auftrag: `4a99c6ec54e74264811d86817a568d47`, Build-ID `7ec061ca1ae24338bdc86a22030957cb`, Commit `1595259a2a76abfc9eff08842409156473b585a7`, Status `failed`. Damit ist der POST-Ausgang eindeutig zugeordnet. Die konkrete Ursache des zuvor ausgegebenen `OperationalError` ist weiterhin nicht bekannt.

Der zugehörige Log bestätigt `Vorbereitete Softwarebasis aus Cache: 7699808215f7` und die erfolgreiche Kopie des Cacheimages in den Workordner. Danach scheitert `Disk.__enter__()` an `losetup --find --show --partscan .../work/7ec061ca1ae24338bdc86a22030957cb/image.img` mit `subprocess.TimeoutExpired` nach30 Sekunden. Die Stationspersonalisierung, Kompression und Veröffentlichung wurden in diesem Lauf noch nicht ausgeführt. `Command exited with 1: unshare` ist der äußere Folgestatus. Es wurde keine neue ARM64-Softwarekompilierung gestartet.

Nächster Schritt: auf VM119 blockierte Prozesse / Wartefunktionen, Kernelmeldungen, Dirty-/Writeback-Werte, I/O-Pressure und die Zuordnung der NetCore-Loopgeräte lesen; auf Proxmox die aktuelle `qm config 119` nach der gemeldeten Plattenrettung prüfen. Das Gast-Dateisystem ext4 allein belegt keinen lokalen Proxmox-Speicher. Eine Ursache im Kernel-/Block-I/O-/Lock-Zustand ist noch nicht nachgewiesen. Kein weiterer Recovery-POST oder Belegreset vor dieser Diagnose; die installierten Rezeptdateien und der geprüfte Softwarecache bleiben erhalten. [Betreiberbeleg](evidence/vm119-imagebuilder-success-2026-10-09.json).

Die SQLite-Diagnosekorrektur an `d9938723d5a320718e36db9b739b5ba6a9fcb11f` besteht beide main-Workflows: [Quellgate](https://github.com/JanHG98/netcore-tetra/actions/runs/37907594111) und [Deployment/Discovery](https://github.com/JanHG98/netcore-tetra/actions/runs/37907594176). Diese CI-Erfolge ersetzen weder die Plattformdiagnose noch den noch offenen Artefaktdownload und physischen Pi-Start. Die nachfolgenden Abschnitte halten die vorherigen Diagnose- und Recoveryphasen als Historie fest.

## Betreibermeldung: erfolgreiches Image, fehlende Downloadanzeige

Am09.10.2026 meldet der Nutzer: Der gestrige Build war erfolgreich und hat eine `.img.xz` erzeugt. Nach dem aktuellen Update fehlt die Downloadfunktion in der WebUI. Die erste Meldung enthält keine Job-ID, Manifest- oder Prüfsummendaten. Die folgenden älteren Buildabbruchabschnitte bleiben datierte Historie; sie sind nicht der aktuelle Gesamtstand.

Am geprüften TBS-Quellstand16d0d1f sind die Downloadlinks und GET-Routen weiterhin enthalten. Der neue Updatehelfer übernimmt keine Artefaktdateien und löscht keine Images. Ein tatsächlicher lokaler Browserfall mit fertigem Artefakt / Manifest / SQLite-Auftrag, leerer Profilliste und ohne Build-POST zeigt eine Downloadkarte, liefert die Testdatei bytegenau und behält die Karte nach Refresh. Ein genereller Verlust der Downloadfunktion durch die Profiländerung wurde damit nicht reproduziert.

**Anschließender tatsächlicher Betreiberbefund:** [Image-API / Worker / Artefaktpfad](evidence/vm119-download-diagnosis-2026-10-09.json). Die lokale GET-Abfrage liefert `available=true`, `error=null`, `artifacts=[]`. Der Imageworker ist aktiv und startet `image_worker.py` ohne `--state`; der geprüfte Quellcode verwendet dann `/var/lib/netcore-image-builder`. Die Prüfung `artifacts/*/*` als root findet keine Dateien. Damit ist ein Ausfall der gesamten Worker-Statusantwort für diese Abfrage ausgeschlossen. Die Downloadanzeige hat derzeit keine gelisteten Artefakte. Ob die Datei an einem anderen Ort liegt oder die gegenwärtige Datenträgeransicht unvollständig ist, bleibt offen; ein endgültiger Verlust ist nicht belegt. Auch der exakte installierte Quellstand ist durch diese Ausgabe noch nicht bestätigt.

Nächster Schritt auf VM119: Imagejobs einschließlich `request.build_id`, `result.id`, Ergebnisdateiname und Abschlusslog lesen; aufgelösten Statepfad / Mountquelle und die Dateien in cache, work und artifacts nur flach prüfen. Ein gültiges Artefakt benötigt `artifacts/<Build-ID>/image.img.xz` und ein `manifest.json` mit passender ID. `job.id` ist eine andere Kennung als die Artefakt-ID. Cachedateien sind OS-/Softwarebasisimages und belegen allein kein fertiges Stationsimage. Der Abschlussmarker `Fertig:` wird erst nach der Veröffentlichung des vollständigen Artefaktordners geschrieben. Die Managementadresse war aus der Assistenzumgebung nicht erreichbar.

**Zweiter tatsächlicher Betreiberbefund:** [Vollständiger Build / fehlendes Artefakt / neuer Timeout](evidence/vm119-imagebuilder-success-2026-10-09.json). Auftrag `f141c06d4b2f48699486e149c4887b16` ist `succeeded`, Artefakt-ID `2e2603beacd942dfaddc7119b85a4af0`, Commit `1595259a2a76abfc9eff08842409156473b585a7`. Ergebnis und Log nennen `netcore-srv-m-tbs-03-1595259a2a76-2e2603be.img.xz`, 2.043.954.184 Bytes und SHA-256 `4a6bc6549cace7559a3dfc9e1f8f6d6d0302e84df3128b63d31c4e6ac17d6c3b`; der Abschlussmarker `Fertig:` bestätigt die damalige Veröffentlichung. Der Statepfad liegt aktuell auf `/dev/sda2`, ext4. SQLite-Historie und beide Cacheimages sind vorhanden; fertige Images oder ausstehende Artefaktdateien unter work wurden nicht gefunden. Dies beweist den früheren vollständigen Build, noch keinen erfolgreichen aktuellen Download oder physischen Pi-Start.

Der neue Auftrag `8d83bec249dc4297a964db170c7b5f7d`, Artefakt-ID `66e4156625f5425e9cbd04e77bb40003`, an2815789 endet bei `sfdisk --no-reread --no-tell-kernel -N 2 .../image.img` nach30 Sekunden. `unshare` ist der äußere Fehlerstatus. Der kurze Logauszug zeigt nicht, welcher Aufrufer der gemeinsamen Partitionsroutine betroffen ist; eine genaue Ursache des Zeitlimits ist noch nicht belegt. Dieser Lauf verwendet einen anderen Quellcommit und damit einen anderen Recipekey.

Der vorhandene Softwarecache `7699808215f76322a6ed0cceddf5804d24f707e088ceabd7bdae5e7ef07b6eb6.img` hat5.108.662.272 Bytes. Die reine lokale Recipekey-Berechnung für1595259 ergibt genau diesen Schlüssel. Vor einer erneuten Personalisierung müssen der installierte Recipekey und die zugehörige versions-JSON mit Commit1595259 ebenfalls passen. Dann überspringt der vorhandene Code den kompletten ARM64-Softwarebuild einschließlich expand / shrink / sfdisk; Kopie, Stationspersonalisierung, Dateisystemprüfung und Kompression bleiben erforderlich. Das erzeugt ein neues Artefakt mit eigener ID / SHA-256, keine bytegleiche Rekonstruktion des ursprünglichen Images. Die normale WebUI verwendet weiterhin frisch abgerufenes main und bietet diesen alten Pin bewusst nicht an.

Der geprüfte Produktcode löscht fertige Artefakte ausschließlich über den ausdrücklich bestätigten Image-Löschpfad; die Erfolgsjobs bleiben dabei erhalten. Profilentfernung, Updatehelfer und Work-Bereinigung löschen keinen fertigen Artefaktordner. Welcher externe Lösch- oder Datenträgerverlauf hier vorliegt, ist nicht belegt. Als Nächstes nach bestehenden `.img.xz` auf der lokalen VM suchen und Cache-Metadaten lesen. Builder-Rezeptdateien vorher nicht ändern, damit der vorhandene Cachekey erhalten bleibt.

**Dritter tatsächlicher Betreiberbefund:** Die installierte Recipekey-Berechnung für1595259, `result.recipe` des Erfolgsjobs und der vorhandene Cachekey769980… stimmen exakt überein. Cacheimage und versions-JSON sind vorhanden, die JSON nennt Commit1595259. Die vollständige Suche nach `.img.xz` auf der lokalen VM-Dateisystemansicht findet ausschließlich den 443.120.736-Byte-Raspberry-Pi-OS-Download. Der längere Traceback des neuen Builds zeigt die anfängliche Partitionsvergrößerung in `build()` vor ARM64-Kompilierung; die Phase ist damit zugeordnet. Warum sfdisk nach30s nicht beendet ist, bleibt offen.

### Stationsimage aus dem bestätigten Softwarecache neu erzeugen

[vm119-cached-image-recovery.py](vm119-cached-image-recovery.py) ist ein eigenständiger root-Helfer für genau diesen bestätigten VM119-Auftrag. Er wird aus dem bereitgestellten geprüften Quellstand mit `sudo python3 -B` ausgeführt. Er liest den ursprünglichen vollständigen Stationsauftrag ausschließlich aus SQLite mit `mode=ro`, normalisiert ihn mit dem installierten Validator und verweigert veränderte Stationseinstellungen. Cachegröße / Recipekey / Commit, alle Imagejobs und verwaiste Workpfade werden vor dem Start geprüft. Der normale Controller-Endpunkt verwendet weiterhin frisch abgerufenes main; nur diese begrenzte Rekonstruktion verwendet den alten bereits erfolgreich gebauten Stand1595259.

Der Helfer reiht über den bestehenden lokalen Worker-Socket genau einen neuen Auftrag ein. Ein root-0600-Beleg unter `/var/lib/netcore-image-builder/recovery-2e2603beacd942dfaddc7119b85a4af0.json` wird vorher per Datei- und Verzeichnis-fsync gespeichert. Wiederholtes Ausführen fragt anhand des Belegs ausschließlich den vorhandenen Auftrag ab. Nach verlorener Antwort, unklarem Ausgang, fehlgeschlagenem Auftrag oder mehrdeutigen Treffern erfolgt kein zweiter POST. Zugangsdaten bleiben im lokalen gespeicherten Auftrag und werden nicht im Beleg oder der Ausgabe wiedergegeben. Es werden keine Laufzeit-/Rezeptdateien ausgetauscht, Dienste gestoppt oder Cache-/Artefaktdateien gelöscht.

Bei diesem bestätigten Cachehit verwendet der unveränderte Builder die Softwarebasis und führt nur Kopie, Stationspersonalisierung, Dateisystemprüfung, Kompression und Veröffentlichung aus. Beliebige parallele Root-Wartung kann der bestehende Worker nicht atomar ausschließen; während des Auftrags keine Rezept-/Cachedaten manuell ändern. Der neue Auftrag erhält eine eigene ID, einen neuen Dateinamen und SHA-256. Ein Erfolgsbericht prüft Manifest / Dateigröße / Prüfsummendatei und nennt die drei Download-URLs; die vollständige Dateiprüfsumme wird dabei noch nicht erneut berechnet und ist anschließend am Download zu prüfen. Die tatsächliche Ausführung auf VM119 und der erneute Download sind bis zur Betreiber-Ausgabe offen.

Lokale Validierung:21 neue Recoverytests mit echten SQLite-/Beleg-/Artefaktdateien und dem tatsächlichen Imagevalidator bestanden; alle56 VM119-Operatortests bestanden. Workertransport / Softwarebuild sind isoliert; kein VM119- oder ARM64-Auftrag wird durch diese Tests ausgeführt. Verlorene / verspätete Antworten, fsync-Fehler vor und nach Annahme, versteckte aktive / ungeklärte Jobs, fremde Workdateien, geänderte Zugangsdaten, defekte Belege und inkonsistente Artefakte sind geprüft. Unabhängiger Review ohne offene Blocker. [Regressionen](../../../tests/integration/test_vm119_cached_image_recovery.py); Reproduktion: `python3 -B -m unittest discover -s tests/integration -p 'test_vm119_*.py' -v`. CI-Trigger und Operator-Testauswahl umfassen jetzt auch die Recoverydateien; der neue CI-Lauf ist beim Erstellen dieses Nachtrags noch nicht beobachtet.

### Tatsächlicher Recoveryversuch: SQLite-Lesefehler

Am09.10.2026 führt der Betreiber den veröffentlichten Helfer an `f4e1215c7ffc2ee418b261bb7f98e14bdda44b32` aus. Die heruntergeladene Datei `/var/tmp/netcore-image-recovery.IIVfhV.py` besteht die SHA-256-Prüfung `1b719f76cd8fad68f0c3152f6bbf389e7c066cb134a627c3b8420bd6c402e943`. Danach endet der Helfer mit `STOP: OperationalError; kein automatischer Neuauftrag. Vorhandenen Recoverybeleg prüfen.` Die bisherige generische Ausgabe enthält weder SQLite-Detail noch Belegphase. Deshalb ist nicht belegt, ob der Fehler bei einer Vorprüfung oder beim Lesen nach Annahme eines Auftrags entstand. Aus dieser Ausgabe allein wird kein weiterer Build abgeleitet.

Die SQLite-Abfrage bleibt ausschließlich `mode=ro`. Der Helfer nennt nun `OPEN_READONLY` oder `SELECT_JOBS` sowie SQLite-Fehlername / Fehlercode / tatsächliche Meldung der konstanten Abfrage. Ein weiterer Operatorblock liest nur Belegexistenz / phase / job_id, Datenbank- und Journalmetadaten, Spaltennamen und Auftrags-ID / Status; private Requestfelder werden nicht ausgegeben. Keine Wiederholung des POST, kein read-write-Fallback und keine Datenbankreparatur zur Diagnose.

Fünf zusätzliche SQLite-Regressionsfälle bestehen: fehlende Tabelle / Spalte, echte exklusive Sperre, fehlgeschlagenes Öffnen und Lesefehler nach bereits persistierter Auftragsannahme. Insgesamt26 Recoverytests und alle61 VM119-Operatortests lokal bestanden. Der veröffentlichte Ausgangshelfer f4e1215 besteht inzwischen auch beide vollständigen main-CI-Läufe: [Quellgate](https://github.com/JanHG98/netcore-tetra/actions/runs/37905773577), [Deployment/Discovery](https://github.com/JanHG98/netcore-tetra/actions/runs/37905773628). Der neue Diagnosepatch hat damit noch keinen eigenen CI- oder VM119-Nachweis. Nächster Schritt: konkrete SQLite-Meldung und Belegphase auf VM119 lesend zuordnen, danach den vorhandenen beziehungsweise noch nicht angenommenen Recoveryauftrag gezielt fortsetzen.

Keinen weiteren vollständigen Build, Cachelöschung oder Dienstneustart zur Diagnose auslösen. Zuerst die vorhandene Image-Datei auffinden beziehungsweise den geprüften Softwarecache für eine begrenzte erneute Personalisierung zuordnen. Anschließend Download / Manifest / SHA-256 abnehmen. Der vollständige Build ist durch den neuen Betreiberbefund bestätigt; Download- und physische Pi-/SXceiver-/VPN-Abnahme bleiben offen.

## Aktuelle TBS-Bedienung vom 09.10.2026

[Workflow-Update und Prüfgrenzen](tbs-workflow-2026-10-09.md): gespeicherte Profile lassen sich löschen, sofern kein aktiver oder ungeklärter Auftrag sie verwendet. Neue Images verwenden automatisch frisch abgerufenes `origin/main`; eine Git-Referenzeingabe entfällt. Die ältere Ref-Eingabe in den unten datierten Build-/Hotfixanweisungen beschreibt die damalige Oberfläche. Der neue gezielte Helfer übernimmt sieben Controller-/Worker-/UI-Dateien; das Gastrezept bleibt separat. Die tatsächliche Installation auf VM119 ist noch nicht bestätigt.

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
- Nach dem Workflow-Update vorhandenes Profil, Hostname und Betriebssystem-Zugang (SSH-Public-Key oder Passwort) wählen; passendes HAT-Overlay festlegen. Für die VPN-Abnahme gültiges Inline-OpenVPN-Profil und vertraute SSIDs lokal angeben. Zugangsdaten im Betrieb eingeben, nicht im Repo ablegen.
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

Vorhandenes Profil SRV-M-TBS-01 ist die vorgeschlagene erste Auswahl; die anderen Profile bleiben auswählbar. Einen eindeutigen Testhostname angeben. Nach dem Workflow-Update holt der Controller beim Absenden den neuesten Commit von `origin/main` und hält ihn für diesen Build fest. Dafür muss der globale Controllerref c45a2ec nicht geändert werden. SSH-Public-Key oder Betriebssystem-Passwort sowie gegebenenfalls WLAN-/OpenVPN-Daten lokal eingeben; keine Geheimnisse in Chat, Repo oder Diagnoseausgabe übernehmen.

Den Build genau einmal über `/api/v1/images/build` oder die Imagebuilder-WebUI absenden. Bei unklarer Antwort anhand der bestehenden Builds prüfen und den POST nicht wiederholen. Imagejobs werden über `/api/v1/images` unter `jobs[]` verfolgt; Controller-Job-URLs gehören zu einer anderen Auftragsdatenbank. `job.id` und `request.build_id` sind unterschiedliche IDs. Nach `succeeded` ist `request.build_id` beziehungsweise `result.id` die Artefakt-ID; Image, SHA-256 und Manifest sind unter `/api/v1/images/<Artefakt-ID>/image`, `/sha256`, `/manifest` abrufbar. Manifestcommit und Dateiprüfsumme vergleichen. Die anschließende physische Pi-/SXceiver- und VPN-Abnahme bleibt getrennt; `boot_tested=false` ist bis dahin korrekt.

### Bestehende WebUI bedienen

Nach bestandener Updateprüfung `http://10.0.1.131:8320` öffnen, Abschnitt **Vom Profil zur Basisstation.**:

| Feld | Erste Auswahl |
| --- | --- |
| TBS-Profil | SRV-M-TBS-01 |
| Quellstand | Automatisch neuester Commit von origin/main; keine Eingabe. Der Auftrag und das Manifest halten den vollständigen SHA fest. |
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

Lokale Regressionen verwenden echte Dateien, Umask, Gastkontext und SQLite-Dateien, simulieren aber Mounts/Geräteknoten, Dienste und externe DNS-Aufrufe. Temporäres 0644, Inhalt/Originalmodus, dangling Symlink, fehlender Resolver sowie Wiederherstellung bei Fehler sind geprüft. DNS-Tests prüfen `_apt`, beide Hosts, Zeitlimits und sofortigen Abbruch auf Lesbarkeits-/Auflösungsfehler. Zum damaligen DNS-Korrekturstand blieb der tatsächliche ARM64-DNS-/APT-Erfolg offen. Der nachfolgende zweite Betreiberbuild bestätigt inzwischen DNS-/Paketfortschritt; dessen dpkg-Abbruch steht im nächsten Abschnitt. Der vollständige Imagebuild bleibt offen.

Validierung des finalen Korrekturstands: Deployment-Core-Suite 62 Tests, 61 bestanden/ein erwarteter Unix-Socket-Skip; VM119-Update-Suite 11/11 bestanden. Syntax und unabhängiger Review bestehen. [Gastregression](../../../system-backend/deployment-core/tests/test_image_guest.py), [Update-/Rückwegregression](../../../tests/integration/test_vm119_image_dns_update.py).

## Zweiter ARM64-Build: dpkg-Konfigurationsrückfrage

[Betreiberbefund vom 08.10.2026](evidence/vm119-imagebuilder-conffile-2026-10-08.json), Build-ID `2dce020e97d745408fe5221453ad63ac`: ARM64-Pakete werden entpackt/konfiguriert. Der korrigierte Builder erreicht den Gast-Paketbuild nach seiner DNS-Vorprüfung; im neuen Auszug fehlen die vorherigen DNS-Abbrüche. Die beiden main-Workflows des DNS-Fixes 8359f75 bestehen inzwischen. Dies bestätigt Fortschritt über den früheren DNSfehler, keinen vollständigen Imageerfolg.

**Primärfehler:** `initramfs-tools-core` fordert eine Entscheidung für die im Raspberry-Pi-Basisimage bereits angepasste `/etc/initramfs-tools/initramfs.conf`. Der unbeaufsichtigte Build hat keinen Eingabestrom; dpkg endet mit `end of file on stdin at conffile prompt`. Die weiteren Initramfs-/Kernel-/Header-Fehler folgen aus diesem nicht konfigurierten Paket. Die Meldungen `policy-rc.d denied execution` entsprechen dagegen dem absichtlich unterdrückten Dienststart im Gast. Unmounts und Loopdetach sind laut Auszug durchgelaufen.

**Korrektur nur im Gastrezept:** Alle drei Paketoperationen nutzen `Dpkg::Options::=--force-confdef` und `Dpkg::Options::=--force-confold`, neben `DEBIAN_FRONTEND=noninteractive`. Damit entscheidet dpkg unbeaufsichtigt anhand des Defaults und behält bei fehlendem Default die vorhandene Konfiguration bei. [Primärquelle dpkg](https://manpages.debian.org/bookworm/dpkg/dpkg.1.en.html). `apt-get update` erhält zusätzlich `APT::Update::Error-Mode=any`, damit fehlgeschlagene Indexabrufe nicht still mit alten Listen weiterlaufen. Die bereits korrigierten DNS-Dateirechte bleiben erhalten. Die VM selbst erhält kein Paket-/Kernel- oder Standortkonfigurationsupdate.

[vm119-image-apt-update.py](vm119-image-apt-update.py) ersetzt ausschließlich `/usr/local/lib/netcore-deployment/image/build-guest.sh` mit festen SHA-256-Pins. Der wiederverwendete VM119-Guard prüft die Bash-Syntax, aktive Dienste, untätige Aufträge, Standort-Dateierhalt und Rücknahme. Seine Standard-Pythonprüfung bleibt für die bisherigen Jobs-/DNS-Wrapper erhalten; der ImageClient wird aus dem festen Deployment-Laufzeitverzeichnis geladen.

Als jhoffmeister auf VM119 nach Checkout des bereitgestellten vollständigen Korrekturcommits:

```bash
sudo python3 -B "$NC_SRC/Docs/integration/Z01-2026-10-07/vm119-image-apt-update.py"
```

Danach **einen neuen Build** mit demselben gewünschten Profil und dem vollständigen neuen Quellcommit im Imageformular anlegen. Der fehlgeschlagene, bereits aufgeräumte Gast wird nicht manuell repariert. Der Rezeptcode ändert den Recipekey automatisch; die feste komprimierte OS-Basis kann weiterhin aus dem Downloadcache genutzt werden. Tatsächlicher erneuter ARM64-Paket-/NetCore-Erfolg sowie Manifest/SHA-256 und physischer Pi-/SXceiver-/VPN-Nachweis bleiben offen.

Validierung des finalen APT-Korrekturstands: Deployment-Core 66 Tests, 65 bestanden/ein erwarteter Unix-Socket-Skip; VM119-Operator 18/18 bestanden; Bash-Syntax und unabhängiger Review bestehen. Die [Paketregression](../../../system-backend/deployment-core/tests/test_build_guest_packages.py) führt den tatsächlichen Bash-Rezeptanfang über einen isolierten APT-Harness aus und verarbeitet echte private Testpakete mit `dpkg --root`: Ohne Optionen EOF reproduziert, mit Rezeptoptionen Version 2 erfolgreich konfiguriert und eigene Datei bytegleich erhalten. [Skript-Austausch-/Rückwegtests](../../../tests/integration/test_vm119_image_apt_update.py). Dies ist ein nativer isolierter dpkg-Test, kein vollständiger ARM64-/Repository-Build.

## Gesicherter Fortsetzungsstand: 08.10.2026, 17:03 Europe/Berlin

Der vollständige Gast-APT-Korrekturcommit ist `001fb84ac566bb0f95e18d22439ee664fe0093e4`. Seine beiden main-Workflows sind erfolgreich abgeschlossen: [Deployment-/Quellgate](https://github.com/JanHG98/netcore-tetra/actions/runs/37795951602), [Deployment/Discovery](https://github.com/JanHG98/netcore-tetra/actions/runs/37795951576). [CI-Beleg](evidence/checkpoint-ci-2026-10-08.json).

Der bereitgestellte Operatorbefehl verwendet diesen Commit und `vm119-image-apt-update.py`; im Imageformular gehört derselbe vollständige SHA in **Branch, Tag oder Commit**. Ein globaler Controller-Ref-Wechsel ist dafür nicht erforderlich. Zum Zeitpunkt dieser Sicherung fehlt eine neue Betreiber-Ausgabe zur Gastrezept-Übernahme und zum erneuten vollständigen Build. Der jobs.py-Fix auf VM119 ist ebenfalls nicht durch einen neuen Runtime-Fingerprint bestätigt. Bereits gestartete Builds über ihren bestehenden Auftrag verfolgen; diese Dokumentationssicherung erfordert keinen weiteren Build oder Dienstneustart.

Bei einem Erfolg zuerst `/api/v1/images`, den bestehenden Imagejob, Artefakt und Manifest auswerten und den Download gegen die SHA-256 prüfen. Imagejob-ID und `request.build_id` sind verschiedene Kennungen; Controller-`/api/v1/jobs/<id>` ist kein Imagejob-Statuspfad. Das Manifest darf bis zur tatsächlichen physischen Abnahme weiterhin `boot_tested=false` melden. Erst danach folgen Pi-/SXceiver-Boot und VPN-Wechsel. [Gesamter Fortsetzungsstand](checkpoint-2026-10-08.md).

## Dritter ARM64-Build: Rootgeräteerkennung beim Initramfs

[Betreiberbefund vom 08.10.2026](evidence/vm119-imagebuilder-initramfs-2026-10-08.json), Build-ID `fb5683fe496740deae2916551451e69c`: `initramfs-tools-core` übernimmt die vorhandene initramfs.conf unbeaufsichtigt (`Keeping old config file as default`). Der bisherige Conffile-Abbruch ist damit überwunden. Bei den Kernel-Postinst-Skripten für 6.12.109 Pi-v8 und Pi2712 sowie beim Trigger des bisherigen 6.12.25-Kernels endet mkinitramfs nun mit `failed to determine device for /`; der Log nennt selbst `MODULES=most` als Ausweg. Nachfolgende Kernel-/Header-Abhängigkeitsfehler sind Folgefehler. Unmounts und Detach von `/dev/loop20` sind dokumentiert.

**Quellanschluss:** Der Debian-Bookworm-Code ruft im Zweig `MODULES=dep` die laufzeitbezogene Rootgeräteerkennung auf. `MODULES=most` verwendet die breite Treiberauswahl für ein portables Image. Das ist für das ARM64-Image im Build-chroot passend; die tatsächliche Gastkonfiguration wurde nicht separat ausgelesen. [Initramfs-Konfiguration](https://manpages.debian.org/bookworm/initramfs-tools-core/initramfs.conf.5.en.html), [mkinitramfs](https://sources.debian.org/src/initramfs-tools/0.142%2Bdeb12u3/mkinitramfs/), [Geräteerkennung](https://sources.debian.org/src/initramfs-tools/0.142%2Bdeb12u3/hook-functions/).

**Gezielte Korrektur:** Vor der ersten APT-Operation erstellt das Gastrezept `/etc/initramfs-tools/conf.d/zz-netcore-image.conf` mit `MODULES=most`, explizit0644 auch unter Worker-Umask0077. conf.d wird nach der Hauptdatei geladen; die vorhandene initramfs.conf bleibt bytegleich erhalten. Die neue Datei bleibt Bestandteil des Pi-Images, damit spätere Kernelupdates dieselbe portable Auswahl verwenden. Normale Kernel-/Initramfs-Erzeugung und alle bisherigen Conffile-/DNS-Korrekturen bleiben aktiv. Die Initramfs kann durch die breitere Treiberauswahl größer werden.

[vm119-image-initramfs-update.py](vm119-image-initramfs-update.py) verwendet unverändert die bestehenden Host-/Idle-/Dateierhalt-/Rückweg-Guards und ersetzt ausschließlich `/usr/local/lib/netcore-deployment/image/build-guest.sh`. Die Ausführung erfolgt als jhoffmeister auf VM119 nach Checkout des bereitgestellten vollständigen Korrekturcommits:

```bash
sudo python3 -B "$NC_SRC/Docs/integration/Z01-2026-10-07/vm119-image-initramfs-update.py"
```

Danach einen neuen Build mit demselben gewünschten Profil und dem vollständigen Korrekturcommit im Imageformular anlegen. Die bisherige Build-ID benennt den gescheiterten Lauf; seine Imagejob-ID und der vollständige Request-/Quellcommit sind im Auszug nicht enthalten. Das neue Gastrezept ändert den Recipekey automatisch, die feste komprimierte OS-Basis kann aus dem Downloadcache wiederverwendet werden. Der aufgeräumte fehlgeschlagene Gast benötigt keine manuelle Paket-, Mount- oder Loopreparatur.

Die lokale Regression führt den echten Rezeptanfang im isolierten Testverzeichnis aus und prüft die wirksame Initramfs-Konfiguration vor den Paketaufrufen, Konfigurationserhalt, Rechte und Fehlerabbruch. Externe APT-Aufrufe sind isoliert. Eine tatsächliche ARM64-Initramfs wird in dieser Umgebung nicht erzeugt; der vollständige Betreiberbuild und der physische Pi-/SXceiver-/VPN-Test bleiben die nächsten Nachweise.

Validierung des finalen Initramfs-Korrekturstands: Deployment-Core68 Tests /67 bestanden /1 erwarteter Unix-Socket-Skip; VM119-Operator26/26 bestanden; sechs betroffene Gast-Pakettests bestanden; Bash-Syntax und unabhängiger Review ohne Blocker. [Gastregression](../../../system-backend/deployment-core/tests/test_build_guest_packages.py), [Update-/Rückwegtests](../../../tests/integration/test_vm119_image_initramfs_update.py). Die neue main-CI war beim Erstellen des Belegs noch nicht beobachtet.

## Bestätigte Initramfs-CI / weiterhin offener Betreiberbuild

Am Quellcommit `1595259a2a76abfc9eff08842409156473b585a7` sind beide main-Workflows erfolgreich abgeschlossen: [Deployment/Discovery](https://github.com/JanHG98/netcore-tetra/actions/runs/37804380374) und [Deployment-/Quellgate](https://github.com/JanHG98/netcore-tetra/actions/runs/37804380390). Der oben datierte Hinweis auf damals noch nicht beobachtete CI bleibt historisch. Diese Ergebnisse bestätigen keinen vollständigen ARM64-NetCore-Build oder physischen Pi-Boot. Die Anlagenübernahme der neuen TBS-Bedienung und des Gastrezepts wird jeweils durch die Betreiber-Ausgabe bestätigt.
