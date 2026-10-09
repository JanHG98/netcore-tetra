# Installation: Media Library → echte Funk-Aussendung über die TBS

Dieses Paket ersetzt den bisherigen unvollständigen Aussendepfad. Im Modus
`basisstation` muss die Media Library weder einen zentralen TACELP-Cache erzeugen
noch eine bestehende Media-Switch-Session kennen. Sie übergibt Asset, Ziel und
Priorität an den vorhandenen Audio Player einer Basisstation. Die TBS lädt die
vollständige WAV-Vorschau, kodiert lokal mit dem nativen TETRA-Codec, baut den
Gruppen- oder Einzelruf auf und beendet ihn nach der Datei wieder.

## 1. Basisstation prüfen

Auf der TBS:

```bash
grep -n -A25 '^\[media_library\]' /etc/netcore/config.toml
grep -n -A25 '^\[audio_player\]' /etc/netcore/config.toml
grep -n -A20 '^\[dashboard\]' /etc/netcore/config.toml
```

Mindestens erforderlich:

```toml
[media_library]
enabled = true
base_url = "http://10.0.1.154:8230"
audio_source_enabled = true
only_ready = true
only_approved = true

[audio_player]
enabled = true
```

Die IP und der Dashboard-Port der TBS notieren. In diesem Beispiel wird
`http://10.0.1.22:8080` verwendet.

## 2. ZIP im Media-Library-LXC einspielen

```bash
cd /tmp
rm -rf /tmp/netcore-ml-playout
mkdir -p /tmp/netcore-ml-playout

unzip \
  netcore-tetra-swmi-media-library-basisstation-playout.zip \
  -d /tmp/netcore-ml-playout

rsync -a \
  /tmp/netcore-ml-playout/netcore-tetra-swmi/ \
  /opt/netcore-tetra/
```

## 3. Media Library aktualisieren

### TBS-Dashboard ohne Anmeldung

```bash
cd /opt/netcore-tetra

NETCORE_TBS_ID="srv-m-tbs-01" \
NETCORE_TBS_NAME="SRV-M-TBS-01" \
NETCORE_TBS_URL="http://10.0.1.22:8080" \
bash system-backend/media-library/install/update.sh
```

### TBS-Dashboard mit Anmeldung

Die tatsächlichen Dashboard-Zugangsdaten einsetzen:

```bash
cd /opt/netcore-tetra

NETCORE_TBS_ID="srv-m-tbs-01" \
NETCORE_TBS_NAME="SRV-M-TBS-01" \
NETCORE_TBS_URL="http://10.0.1.22:8080" \
NETCORE_TBS_USERNAME="admin" \
NETCORE_TBS_PASSWORD="TATSAECHLICHES_PASSWORT" \
bash system-backend/media-library/install/update.sh
```

Die Migration ergänzt einen fehlenden `[playout]`-Block atomar und behält
Eigentümer, Gruppe und Modus der vorhandenen Konfigurationsdatei bei. Existiert
bereits ein `[playout]`-Block, wird er absichtlich nicht überschrieben.

## 4. Konfiguration prüfen

```bash
grep -n -A10 '^\[runtime\]' /etc/netcore/media-library.toml
grep -n -A30 '^\[playout\]' /etc/netcore/media-library.toml
```

Erwartet:

```toml
[runtime]
operating_mode = "authoritative"

[playout]
mode = "basisstation"
default_station = "srv-m-tbs-01"
request_timeout_secs = 15
completion_timeout_secs = 900
poll_interval_ms = 500

[[playout.stations]]
id = "srv-m-tbs-01"
name = "SRV-M-TBS-01"
base_url = "http://10.0.1.22:8080"
enabled = true
```

Bei geschütztem Dashboard zusätzlich:

```toml
username = "admin"
password = "TATSAECHLICHES_PASSWORT"
```

## 5. Verbindung und Audioquelle diagnostizieren

```bash
/opt/netcore-media-library/bin/diagnose-basisstation-playout.py
```

Alternativ direkt aus dem Quellbaum:

```bash
python3 \
  /opt/netcore-tetra/system-backend/media-library/install/diagnose-basisstation-playout.py
```

Erwartete Kernaussagen:

```text
OK: Audio Player verfügbar (state=idle, ...).
OK: TBS-Audioquelle 'media-library' ist verfügbar (...).
FERTIG: SRV-M-TBS-01 ist für delegiertes Playout bereit.
```

## 6. Aussendung testen

Browser hart neu laden (`Strg` + `F5`) und öffnen:

```text
Media Library → Aussendung
```

Dann:

1. TTS- oder WAV-Asset auswählen.
2. Basisstation auswählen.
3. `Gruppe (GSSI)` auswählen.
4. Ziel-ID `15201` eintragen.
5. Priorität wählen.
6. `Aussendung starten` drücken.

Im Basisstationsmodus ist Folgendes normal:

```text
broadcast_ready = false
tetra_path = null
```

Der zentrale TACELP-Cache wird nicht benötigt; die TBS kodiert die vollständige
WAV lokal.

## 7. Live beobachten

Media Library:

```bash
journalctl -u netcore-media-library.service -f
```

Basisstation:

```bash
journalctl -u tetra.service -f
```

Jobs:

```bash
curl -fsS \
  'http://10.0.1.154:8230/api/v1/jobs?limit=10' \
  | python3 -m json.tool
```

Ein laufender Auftrag zeigt unter anderem:

```json
{
  "playout_mode": "basisstation",
  "target_node": "srv-m-tbs-01",
  "remote_job_id": "<TBS-AudioPlayer-Job-ID>",
  "state": "playing"
}
```

Danach wechselt der Auftrag auf `completed`. Bei einem Fehler steht die konkrete
TBS- oder HTTP-Ursache in `last_error`.

## 8. Manuelle TBS-API-Prüfung bei Login

```bash
curl -sS -c /tmp/tbs.cookie \
  -H 'Content-Type: application/json' \
  -d '{"user":"admin","password":"TATSAECHLICHES_PASSWORT"}' \
  http://10.0.1.22:8080/api/login

curl -fsS -b /tmp/tbs.cookie \
  http://10.0.1.22:8080/api/audio/status \
  | python3 -m json.tool

curl -fsS -b /tmp/tbs.cookie \
  http://10.0.1.22:8080/api/audio/sources \
  | python3 -m json.tool
```

Die Quellenliste muss einen verfügbaren Eintrag mit der ID `media-library`
enthalten.

## Rückfall auf direkte Media-Switch-Einspeisung

Der alte Pfad bleibt verfügbar:

```toml
[playout]
mode = "media_switch"
```

Dieser Modus benötigt weiterhin einen gültigen `audio.tacelp`-Cache und eine
bereits vorhandene Media-Switch-Session. Für normale TTS-/WAV-Durchsagen ist
`basisstation` vorgesehen.
