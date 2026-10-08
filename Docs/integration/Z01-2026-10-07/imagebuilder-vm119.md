# Z01.4 – VM119 Imagebuilder-Vorprüfung

Stand: 2026-10-08. CT150-Readiness/Recovery und CT136-TCP-/Vorschau-/NAS-Erfolgspfad einschließlich isoliertem Fehlermount sind als Betreiberbefunde dokumentiert. Der nächste ausführbare Auftrag ist die Imagebuilder-Vorprüfung auf `VM-H-DEPLOY-01`, VM119, `10.0.1.131`.

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
- Installiertes `image_build.preflight()` prüft root, erforderliche Buildwerkzeuge, ARM64-binfmt mit F-Flag und mindestens32GiB frei. Die Prüfung benötigt keinen Rust-Compiler auf der VM; Gast-Buildwerkzeuge werden innerhalb des ARM64-Images eingerichtet.
- Cacheanzeige bestätigt ausschließlich Dateiexistenz. Der Build prüft die feste OS-Basis-SHA-256 und lädt bei Bedarf automatisch das in `image_spec.BASE` definierte Raspberry-Pi-OS-ARM64-Basisimage.
- Den Fingerprint von installiertem `jobs.py` mit dem gemeinsamen Quellstand vergleichen: Der zuletzt bekannte VM119-Rollout war c45a2ec; die später korrigierte SQLite-Auftragsverwaltung wurde in dieser Sitzung bislang auf CT150 installiert. Der Fingerprint ermöglicht die Zuordnung vor dem langen Build.
- Danach vorhandenes Profil, vollständigen gemeinsamen Quellcommit, Hostname und Betriebssystem-Zugang (SSH-Public-Key oder Passwort) wählen; passendes HAT-Overlay festlegen. Für die VPN-Abnahme gültiges Inline-OpenVPN-Profil und vertraute SSIDs lokal angeben. Zugangsdaten im Betrieb eingeben, nicht im Repo ablegen.
- Ein voller ARM64-NetCore-Build erzeugt ein Artefakt samt Manifest/Prüfsumme. Die physische Pi-/SXceiver- und VPN-Abnahme folgt anschließend; `boot_tested=false` bleibt bis zur tatsächlichen Hardwareprüfung maßgeblich.

Primärquellen: `system-backend/deployment-core/main.py`, `common.py`, `image_worker.py`, `image_spec.py`, `image_build.py`. Der Operatorblock ist gegen diese Quellschnittstellen geprüft und syntaktisch validiert. Die tatsächliche VM119-Vorprüfung bleibt bis zur Betreiber-Ausgabe offen.
