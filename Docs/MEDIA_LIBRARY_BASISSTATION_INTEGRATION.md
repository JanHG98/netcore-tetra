# Basisstation ↔ Media Library

Diese Erweiterung verbindet die lokale Basisstations-Aufzeichnung bidirektional mit der zentralen Media Library.

## Datenfluss

### Aufzeichnungen

1. Die Basisstation zeichnet einen Ruf zunächst lokal als WAV plus JSON auf.
2. Nach Finalisierung meldet der Recorder die Datei per `POST /api/v1/assets/import-url` an.
3. Die Media Library lädt die WAV selbst über die schmale öffentliche Route
   `GET /api/media-library/recordings/<recording-id>/audio` von der Basisstation.
4. Die Media Library verarbeitet die Datei zu einer kanonischen Vorschau.
5. Fertige Recordings werden automatisch nach `/mnt/nfs-share/Recordings` archiviert.
6. Ein Marker `<recording>.media-library` verhindert doppelte Imports und hält den Asset-Zustand fest.
7. Ist die Media Library vorübergehend nicht erreichbar, bleibt die lokale Aufnahme bestehen und wird erneut versucht.

### Medien zur Basisstation

1. Die Audio-Zentrale listet `ready`/`approved` Assets über die Media-Library-API.
2. Beim Vorhören oder Aussenden wird `preview.wav` in den lokalen Audiocache der Basisstation geladen.
3. Erst nach vollständigem Download und vollständiger ACELP-Aufbereitung wird der TETRA-Ruf aufgebaut.
4. Während der Aussendung wird nicht live über HTTP oder NFS gestreamt.

## Basisstationskonfiguration

```toml
[media_library]
enabled = true
base_url = "http://10.0.1.154:8230"
station_id = "SRV-M-TBS-01"
publish_recordings = true
recording_source_base_url = "http://10.0.1.163:8080"
auto_approve_recordings = false
audio_source_enabled = true
only_ready = true
only_approved = true
retry_seconds = 60
request_timeout_seconds = 15
download_timeout_seconds = 120
max_list_entries = 1000
```

`recording_source_base_url` muss auf den Dashboard-Port der Basisstation zeigen und aus dem Media-Library-LXC erreichbar sein.

Direkte NFS-Archivierung der Basisstation wird bei dieser Architektur abgeschaltet:

```toml
[recording]
archive_enabled = false
tts_archive_enabled = false
```

Die lokalen Aufzeichnungen unter `/var/lib/netcore/recordings` bleiben die ausfallsichere Quelle, bis die Media Library den Asset-Zustand `ready` bestätigt hat.

## Media-Library-Konfiguration

```toml
[security]
allow_url_import = true
allow_private_import_urls = true

[storage]
archive_root = "/mnt/nfs-share/Media-Library"
recording_archive_root = "/mnt/nfs-share/Recordings"
tts_archive_root = "/mnt/nfs-share/TTS-Dateien"

[runtime]
# HTTP pull timeout for WAV imports from the basis station.
import_timeout_secs = 120
auto_archive_recordings = true
auto_archive_tts = true
```

Die Installation und Aktualisierung der Media Library legen die drei Verzeichnisse nur an, wenn `/mnt/nfs-share` wirklich gemountet ist. Im OPEN-LAB-Betrieb werden sie mit `0777` erstellt bzw. repariert, damit paralleler SMB-Zugriff funktioniert. Die systemd-Unit verwendet `UMask=0000` und erlaubt Schreibzugriff auf alle drei Pfade.

## API-Endpunkte

### Basisstation

- `GET /api/media-library/recordings/<id>/audio`
- `GET /api/media-library/recordings/<id>/metadata`

Diese Route wird nur aktiv, wenn `[media_library].enabled = true` und `publish_recordings = true` gesetzt sind. Sie bietet keine Liste, keine Löschfunktion und keine Steuerbefehle.

### Media Library

- `POST /api/v1/assets/import-url`
- `GET /api/v1/assets`
- `GET /api/v1/assets/<asset-id>`
- `GET /api/v1/assets/<asset-id>/preview`

`source + source_reference` bilden einen Idempotenzschlüssel. Wiederholte Meldungen derselben Basisstationsaufnahme erzeugen keinen zweiten Asset-Datensatz. Fehlgeschlagene Imports werden über denselben Datensatz erneut eingereiht.
