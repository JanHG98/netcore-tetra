# Zentrale TTS-Erzeugung in der Media Library

## Zielbild

Piper und die TTS-Bedienoberfläche laufen ausschließlich im Media-Library-LXC.
Die Basisstation erzeugt keine Sprachdateien mehr. Sie verwendet fertige und
freigegebene TTS-Assets über denselben Media-Library-Dateibrowser wie andere
WAV-/MP3-Medien.

```text
Operator -> Media-Library-WebUI -> Piper -> WAV-Asset -> Verarbeitung
                                              |
                                              +-> /mnt/nfs-share/TTS-Dateien/YYYY/MM/DD
                                              |
Basisstation <- Media-Library-API <- Freigabe + Preview
      |
      +-> lokaler Cache -> vollständige Funkaufbereitung -> TETRA-Aussendung
```

Während einer Funkdurchsage wird nicht live von Piper, NFS oder der Media Library
gestreamt. Die Basisstation lädt die Vorschau vollständig in ihren lokalen Cache,
bevor der Ruf aufgebaut wird.

## Media-Library-Konfiguration

```toml
[tts]
enabled = true
endpoint = "http://127.0.0.1:5005"
template_directory = "/var/lib/netcore-media-library/tts/templates"
default_voice = "de-thorsten"
default_speed = 0.95
max_text_characters = 2000
synthesis_timeout_secs = 90
max_output_file_mb = 25

[[tts.voices]]
id = "de-thorsten"
name = "Deutsch – Thorsten (mittel)"
provider_voice = "de_DE-thorsten-medium"
```

Weitere Stimmen sind in `system-backend/media-library/config/media-library.example.toml`
aufgeführt. `runtime.auto_approve_tts = false` lässt neue Durchsagen zunächst als
Entwurf stehen. Mit Freigabe in der Media Library werden sie für die Basisstation
sendbar.

## TTS-API

```text
GET  /api/v1/tts/status
GET  /api/v1/tts/voices
GET  /api/v1/tts/templates
POST /api/v1/tts/templates/save
POST /api/v1/tts/templates/delete
POST /api/v1/tts/generate
```

`POST /api/v1/tts/generate` erzeugt ein normales Media-Library-Asset mit
`kind = "tts"`. Der bestehende Worker normalisiert die WAV, erzeugt die Vorschau,
archiviert sie nach `storage.tts_archive_root` und stellt sie über die normale
Asset-API bereit.

## Basisstation

Der bisherige `[tts]`-Abschnitt wird aus `/etc/netcore/config.toml` entfernt. Die
Basisstation benötigt nur ihre bestehende `[media_library]`-Anbindung. Im
Media-Library-Dateibrowser erscheinen fertige TTS-Dateien automatisch im virtuellen
Baum `Jahr/Monat/Tag`. Entwürfe können vorgehört werden; eine Funk-Aussendung wird
bei `only_approved = true` erst nach Freigabe zugelassen.
