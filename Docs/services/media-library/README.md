# Medienbibliothek

**Quellen:** [system-backend/media-library](../../../system-backend/media-library) · [Repository-Root](../../..). Bei Befehlen das in der Anleitung angegebene Arbeitsverzeichnis beachten.

Stand: **9. Oktober 2026**. Die Anleitung beschreibt die im Hauptzweig vorhandene Umsetzung; Anlagen- und Funkabnahmen stehen in der [Gesamtroadmap](../../roadmaps/gesamtroadmap.md).

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
- vollständige WAV-/TTS-Aussendung über den nativen Audio Player einer ausgewählten Basisstation
- optional weiterhin kontrollierte TACELP-Einspeisung in **bereits bestehende** Media-Switch-Sessions
- Shadow- und Authoritative-Modus
- Jobfortschritt, Abbruch und bewusster manueller Retry ab Frame 0
- versionierte NFS-/Archivkopie mit Manifest und Dateihashes, ohne das Archiv als Live-Playout-Quelle zu missbrauchen
- WebUI, REST-API, OpenAPI, Prometheus-Metriken, Audit, Backup und Export
- systemd- und LXC-Installationsskripte

## Architekturgrenze

Der empfohlene Modus delegiert die Funkseite an die Basisstation, weil dort CMCE,
Timeslot, Floor und der native TETRA-Sprachcodec bereits zusammenlaufen:

```text
Media-Library-WebUI
        │ Asset-ID + GSSI/ISSI
        ▼
Media-Library-Worker
        │ POST /api/audio/play
        ▼
ausgewählte Basisstation
        │ vollständige WAV laden und lokal zwischenspeichern
        │ mit nativem TETRA-Codec kodieren
        │ Gruppen-/Einzelruf aufbauen und Audio senden
        │ Ruf nach Dateiende geordnet freigeben
        ▼
      Funknetz
```

Die Media Library erzeugt dabei selbst keinen CMCE-Zustand und übernimmt keinen
RF-Timeslot. Sie orchestriert einen vorhandenen, sendefähigen TBS-Audio-Player und
spiegelt dessen Jobfortschritt. Dadurch ist auf dem Media-Library-LXC kein zweiter
TETRA-Codec erforderlich.

Der frühere direkte Pfad bleibt als `playout.mode = "media_switch"` erhalten. Er
benötigt weiterhin einen validierten 35-Byte-TACELP-Cache und eine bereits vorhandene
Media-Switch-Session.

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

Im empfohlenen Modus `playout.mode = "basisstation"` genügt eine freigegebene
WAV-Vorschau. Die ausgewählte TBS lädt die Datei vollständig in ihren lokalen Cache
und nutzt anschließend ihren nativen TETRA-Sprachcodec. Deshalb darf
`encoder_command = []` bleiben und das Asset kann trotz `broadcast_ready = false`
über Funk ausgesendet werden.

Gepackte `.tacelp`-Dateien bleiben für die direkte Media-Switch-Einspeisung
verwendbar, wenn ihre Größe ein positives Vielfaches von 35 Byte ist. Für deren
hörbare Vorschau ist weiterhin ein Decoder-Helfer erforderlich.

## Playout

Im Betriebsmodus `shadow` wird jeder Auftrag nur protokolliert und als `shadowed`
abgeschlossen. Es findet unabhängig vom gewählten Playout-Pfad keine Funkübertragung
statt.

Für echte Aussendungen muss gelten:

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
# Nur eintragen, wenn das TBS-Dashboard geschützt ist:
# username = "admin"
# password = "..."
```

Im Basisstationsmodus erzeugt der Worker einen lokalen Job, meldet sich bei Bedarf
über `/api/login` am TBS-Dashboard an, prüft `/api/audio/status` und startet dann:

```text
POST /api/audio/play
```

Die TBS erhält Asset-ID, Zieltyp, 24-Bit-GSSI/ISSI und Priorität. Sie lädt die
Media-Library-Vorschau über ihre konfigurierte Quelle `media-library`, baut den Ruf
auf, sendet das Audio und beendet ihn wieder. Der Media-Library-Job verfolgt dabei
Remote-Job-ID, Audio-Blöcke, Fehler, Abbruch und Abschluss.

Für `playout.mode = "media_switch"` gilt weiterhin der alte Spezialpfad: Ein
validierter TACELP-Cache und eine vorhandene `session_id` sind Pflicht; Frames werden
im 60-ms-Takt an `/api/v1/sessions/{session_id}/inject` übergeben.

Ein Neustart während eines laufenden Jobs markiert ihn als fehlgeschlagen. Ein
manueller Retry beginnt bewusst wieder am Dateianfang.

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
- ein zentraler proprietärer TETRA-Sprachcodec im Media-Library-LXC,
- Musik- oder Lautheits-Mastering jenseits der technischen Normalisierung,
- eigenständige CMCE-/Floor-Implementierung in der Media Library; diese Aufgabe bleibt bei der TBS,
- framegenaue verteilte Playout-Synchronisierung über mehrere Regionen,
- S3-/Object-Storage oder Medien-CDN,
- rechtssichere WORM-Archivierung.

## Aufnahmen der Basisstation übernehmen

Die Basisstation kann abgeschlossene WAV-Aufnahmen per URL anmelden. Die Media Library lädt die Datei, verarbeitet sie und archiviert Aufnahmen standardmäßig nach `storage.recording_archive_root`. Das gemeinsame Archiv verwendet `YYYY/MM/DD` und aus Aufnahmemetadaten abgeleitete Dateinamen. Fertige Entwürfe sind vorhörbar; eine Funkübertragung benötigt weiterhin die Freigabe. Die Audio-Zentrale lädt die Vorschau vor der Aussendung vollständig in ihren lokalen Cache.

Details: [Basisstation anbinden](../../changes/media/medienbibliothek-basisstationsanbindung.md), [Aussendung über die TBS](../../changes/media/medienbibliothek-gruppen-und-einzelruf-aussendung.md) und [Archivansicht und Migration](../../changes/media/medienbibliothek-archiv-smb-und-oberflaechenkorrektur.md).

## Fehlende Dateirechte nach einer Archivmigration

Installer und Archivmigration laufen als root, der Dienst dagegen als
`netcore-media-library`. Die aktuellen Installer stellen nach der Migration die
Eigentumsrechte unter `/var/lib/netcore-media-library` wieder her. Betroffene
Bestandsinstallationen lassen sich vom Repository-Hauptverzeichnis ohne Neubau reparieren:

```bash
sudo ./system-backend/media-library/install/repair-permissions.sh
```

Lokaler Zustand und Cache bleiben geschützt (`UMask=0077`). Nur die Kopien im
gemeinsamen NFS-/SMB-Archiv erhalten ausdrücklich Verzeichnisse `0777` und Dateien
`0666` für das gemeinsam verwendete OPEN-LAB-Share.

## Basisstations-Katalog nach Jahr/Monat/Tag

Die Basisstations-Audio-Zentrale spiegelt die Archivstruktur der Media Library als virtuellen Baum `Recordings/Jahr/Monat/Tag`, `TTS-Dateien/Jahr/Monat/Tag` beziehungsweise `Media-Library/Jahr/Monat/Tag`. Dabei bleibt die Media Library die API-Quelle; NFS wird von der Basisstation nicht direkt als Playout-Quelle verwendet. Archivierte Assets übernehmen Ordner und verständlichen Dateinamen aus `archive_path`, noch nicht archivierte Assets werden anhand ihrer Aufnahme-/Erstellungszeit einsortiert.

## Zentrale Piper-TTS-Erzeugung

Piper läuft im Media-Library-LXC als `netcore-piper.service`. Texte, Stimmen und
Vorlagen werden in der Media-Library-WebUI verwaltet. Jede Synthese wird als normales
Asset mit `kind = "tts"` angelegt, verarbeitet und nach
`storage.tts_archive_root` archiviert. Die Basisstation enthält keine lokale
TTS-Oberfläche mehr; sie sieht fertige TTS-Assets über ihren bestehenden
Media-Library-Dateibrowser und lädt sie vor der Aussendung vollständig in den lokalen
Cache. Siehe [Zentrale Piper-TTS-Erzeugung](../../changes/media/medienbibliothek-zentrale-spracherzeugung.md).
