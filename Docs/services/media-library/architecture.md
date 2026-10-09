# Architektur

## Eigentümerschaft

| Zustand | Eigentümer |
|---|---|
| Originaldatei, Preview, Freigabe und Archivmetadaten | Media Library |
| delegierter Aussendeauftrag und Remote-Job-Zuordnung | Media Library |
| WAV-Downloadcache, nativer TETRA-Codec, Ruf, Floor und RF-Timeslot | ausgewählte Basisstation |
| optionaler zentraler TACELP-Cache | Media Library |
| direkte TACELP-Sessioneinspeisung | Media Switch |
| unverändertes Beweis-/Recorderarchiv | Recorder |
| TTS-Synthese | Media Library / zentraler Piper-Dienst |

## Empfohlene Pipeline: `basisstation`

1. Import schreibt immer zuerst eine partielle Datei und veröffentlicht sie atomar.
2. SHA-256 und Größenlimit werden vor der Verarbeitung geprüft.
3. WAV/MP3 wird zu 8-kHz-Mono-PCM16 normalisiert.
4. Eine Aussendung erfordert `ready`, `approved` und `preview_ready`.
5. Die Media Library übergibt Asset-ID, GSSI/ISSI und Priorität an `/api/audio/play` der ausgewählten TBS.
6. Die TBS lädt die vollständige WAV in ihren lokalen Cache.
7. Die TBS kodiert mit ihrem nativen TETRA-Sprachcodec und erzeugt den Gruppen- oder Einzelruf über ihre CMCE-Instanz.
8. Die Media Library verfolgt Remote-Job-ID, Blockfortschritt, Fehler, Abbruch und Abschluss.

## Spezialpipeline: `media_switch`

Der bestehende direkte Pfad bleibt für bereits gepackte Medien erhalten:

1. Das Asset benötigt `broadcast_ready` und eine gültige `audio.tacelp`.
2. Eine bestehende Media-Switch-`session_id` muss vorgegeben werden.
3. Der Media Switch erhält ausschließlich einzelne 35-Byte-Frames im 60-ms-Takt.

Dieser Pfad erzeugt selbst keinen Ruf und ist nicht der Standard für TTS-/WAV-Durchsagen.

## Ausfallverhalten

- Import-/Codec-Fehler stoppen keinen anderen Dienst.
- Ein Media-Library-Ausfall erzeugt keine Backpressure im Media Switch.
- Ein delegierter Job prüft vor dem Start, ob der TBS-Audio-Player frei und verfügbar ist.
- Ein Playout-Neustart wird nicht automatisch wiederholt; ein manueller Retry beginnt am Dateianfang.
- NFS ist nur Archivziel; ein NFS-Ausfall blockiert keine lokale Vorschau oder TBS-Aussendung.
