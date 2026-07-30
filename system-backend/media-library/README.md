# NetCore-Tetra Media Library

## Zweck

Die Media Library ist die zentrale, kontrollierte Ablage für Durchsagen, TTS-Dateien, Alarmtöne, importierte Recorder-Ausschnitte und vorbereitete TETRA-Sprachframes. Sie trennt Dateisystem, Vorschau, Freigabe und Playout vom zeitkritischen Media Switch.

Der Dienst läuft auf **Port 8230** und besitzt eine eigene WebUI.

## Enthalten

- Upload von WAV, MP3 und gepacktem `.tacelp`
- Import per URL mit optionaler Größe und SHA-256
- nativer Importvertrag `netcore-media-import-v1` für den Application Gateway
- gezielter Recorder-Import über dessen unveränderte `audio.tacelp`-Kopie
- persistente Asset-Metadaten, Tags, Quelle, TTS-Stimme/Text und Broadcast-Hinweise
- RIFF/WAVE-Parser und strikte TACELP-Frameausrichtung
- kanonische Vorschau als 8 kHz, mono, PCM16 WAV
- Waveform-Peaks für die WebUI
- Freigabezustände `draft`, `approved`, `rejected`
- optionale externe TETRA-Encoder-/Decoder-Helfer ohne Shell-Ausführung
- verlustfreier TETRA-Cache mit exakt 35 Byte pro 60-ms-Frame
- kontrollierte Einspeisung in **bereits bestehende** Media-Switch-Sessions
- Shadow- und Authoritative-Modus
- Jobfortschritt, Abbruch und bewusster manueller Retry ab Frame 0
- versionierte NFS-/Archivkopie mit Manifest und Dateihashes, ohne das Archiv als Live-Playout-Quelle zu missbrauchen
- WebUI, REST-API, OpenAPI, Prometheus-Metriken, Audit, Backup und Export
- systemd- und LXC-Installationsskripte

## Architekturgrenze

```text
Application Gateway / Upload / Recorder / Piper-TTS
                 │
                 ▼
          Media Library
  Original → Preview → TACELP-Cache
                 │
          Freigabe + Job
                 ▼
      bestehende Media Session
                 │
                 ▼
           Media Switch
```

Die Media Library:

- erstellt **keinen CMCE-Ruf**,
- besitzt **keinen Floor-Control-State**,
- ändert **keine Recorder-Retention und keinen Legal Hold**,
- transkodiert nicht im Media-Switch-Prozess,
- injiziert nur validierte 35-Byte-TETRA-Frames.

## Open Lab

Die aktuelle Teststufe ist absichtlich offen:

- keine Benutzeranmeldung,
- keine Tokens,
- kein TLS.

Jeder erreichbare Client kann Assets hochladen, freigeben, archivieren, löschen und aussenden. Der LXC gehört deshalb in ein isoliertes Managementnetz.

## WebUI

```text
http://<media-library-lxc>:8230/
```

Die Oberfläche enthält Übersicht, Bibliothek, zentrale **TTS-/Piper-Erzeugung mit Vorlagen**, Upload/URL-Import, Recorder-Import, Vorschau, Aussendung, Jobs, Storage/Audit und API.

## Audio- und Codec-Verhalten

WAV und MP3 werden zu einem kanonischen Vorschauformat normalisiert:

```text
8 kHz · mono · signed 16-bit PCM · RIFF/WAVE
```

Ohne konfigurierten TETRA-Encoder bleiben WAV/MP3 **previewfähig, aber nicht funkbereit**. Das ist Absicht. Eine Dateiendung wird nicht als Codec-Ersatz behandelt.

Gepackte `.tacelp`-Dateien sind sofort funkbereit, wenn ihre Größe ein positives Vielfaches von 35 Byte ist. Für deren hörbare Vorschau ist ein konfigurierter Decoder-Helfer erforderlich.

## Playout

Im Betriebsmodus `shadow` werden Aussendungen als Metadaten-Jobs protokolliert und anschließend als `shadowed` abgeschlossen. Dafür reichen `ready` und `approved`; ein TETRA-Cache ist nicht erforderlich, weil keine Audioframes gelesen oder gesendet werden.

Im Betriebsmodus `authoritative` wird weiterhin zwingend ein validierter gepackter TETRA-Cache verlangt. Für einen echten Job wird außerdem eine vorhandene `session_id` benötigt. Vor dem Start prüft der Worker den gespeicherten SHA-256 des TETRA-Caches. Anschließend liest er `audio.tacelp` frameweise und ruft im festen 60-ms-Takt auf:

```text
POST /api/v1/sessions/{session_id}/inject
```

Ein Neustart während eines laufenden Jobs markiert ihn als fehlgeschlagen. Er wird nicht automatisch neu gestartet, weil eine teilweise doppelte Durchsage schlimmer ist als ein sichtbarer Fehler.

## Wichtige Endpunkte

```text
GET  /health/live
GET  /health/ready
GET  /metrics
GET  /openapi.json
GET  /api/v1/assets
GET  /api/v1/tts/status
GET  /api/v1/tts/templates
POST /api/v1/tts/generate
POST /api/v1/assets/upload-json
POST /api/v1/assets/import-url
POST /api/v1/recorder/import
POST /api/v1/assets/{id}/approve
GET  /api/v1/assets/{id}/preview
GET  /api/v1/assets/{id}/audio.tacelp
POST /api/v1/dispatch
GET  /api/v1/jobs
```

## Bewusste Grenzen

Noch nicht enthalten sind:

- produktives RBAC, TLS oder mTLS,
- eingebettete proprietäre TETRA-Sprachcodec-Algorithmen,
- Musik- oder Lautheits-Mastering jenseits der technischen Normalisierung,
- CMCE-Call-Erzeugung und Floor-Control,
- framegenaue verteilte Playout-Synchronisierung über mehrere Regionen,
- S3-/Object-Storage oder Medien-CDN,
- rechtssichere WORM-Archivierung.

## Basisstation integration

The base station can register completed WAV recordings by URL. The Media Library pulls the file, processes it, and automatically archives recordings to `storage.recording_archive_root`. The shared archive uses `YYYY/MM/DD` and descriptive filenames derived from recording metadata. Ready draft assets remain visible for preview; radio playout is still blocked until approval. The Audio Centre downloads the preview into its local cache before starting radio playout.

See `Docs/MEDIA_LIBRARY_BASISSTATION_INTEGRATION.md` and `Docs/MEDIA_LIBRARY_ARCHIVE_UI_FIX.md` for configuration, migration and rollout steps.

## PermissionDenied directly after archive migration

The installer and archive-layout migration run as root, while the service runs as
`netcore-media-library`. Current installers preserve/repair ownership of
`/var/lib/netcore-media-library` after migration. Existing affected installations
can be repaired without rebuilding:

```bash
sudo ./system-backend/media-library/install/repair-permissions.sh
```

Local state and cache data remain private (`UMask=0077`). Only files copied into
the shared NFS/SMB archive are explicitly opened to directories `0777` and files
`0666` for the OPEN LAB multiprotocol share.

## Basisstations-Katalog nach Jahr/Monat/Tag

Die Basisstations-Audio-Zentrale spiegelt die Archivstruktur der Media Library als virtuellen Baum `Recordings/Jahr/Monat/Tag`, `TTS-Dateien/Jahr/Monat/Tag` beziehungsweise `Media-Library/Jahr/Monat/Tag`. Dabei bleibt die Media Library die API-Quelle; NFS wird von der Basisstation nicht direkt als Playout-Quelle verwendet. Archivierte Assets übernehmen Ordner und verständlichen Dateinamen aus `archive_path`, noch nicht archivierte Assets werden anhand ihrer Aufnahme-/Erstellungszeit einsortiert.

## Zentrale Piper-TTS-Erzeugung

Piper läuft im Media-Library-LXC als `netcore-piper.service`. Texte, Stimmen und
Vorlagen werden in der Media-Library-WebUI verwaltet. Jede Synthese wird als normales
Asset mit `kind = "tts"` angelegt, verarbeitet und nach
`storage.tts_archive_root` archiviert. Die Basisstation enthält keine lokale
TTS-Oberfläche mehr; sie sieht fertige TTS-Assets über ihren bestehenden
Media-Library-Dateibrowser und lädt sie vor der Aussendung vollständig in den lokalen
Cache. Siehe `Docs/MEDIA_LIBRARY_CENTRAL_TTS.md`.
