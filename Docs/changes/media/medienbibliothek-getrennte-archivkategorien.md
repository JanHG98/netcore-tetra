# Media Library: getrennte Archivkategorien für Recordings und TTS

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: Media Library: getrennte Archivkategorien für Recordings und TTS. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Für heutige Medienfreigabe, Archivierung und Stations-Playout [Media Library](../../services/media-library/README.md), [Piper/TTS](../../services/tts/README.md) und [Quellcode](../../../system-backend/media-library) verwenden. Die Basisstation verarbeitet die vorbereitete Datei über ihren [lokalen Audiopfad](../../../crates/tetra-entities/src/net_audio_player); historische Migrationen/Rechtereparaturen sind vor erneutem Einsatz gegen die tatsächlichen Archivpfade zu prüfen.

Dieser Bericht hält den damaligen Audio-/TTS-/Pagingfehler oder Ausbau fest. Beobachtete Einzelrufe und Logursachen gelten für den dokumentierten Test; daraus folgt keine neue allgemeine Funk-/Dateisystemabnahme.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### Media Library: getrennte Archivkategorien für Recordings und TTS

Die Media Library verwendet drei getrennte Archivwurzeln:

- `recording` -> `/mnt/nfs-share/Recordings`
- `tts` -> `/mnt/nfs-share/TTS-Dateien`
- sonstige Medien -> `/mnt/nfs-share/Media-Library`

Der Basisstations-Dateibrowser spiegelt diese Kategorien als oberste Ordner und
zeigt darunter jeweils `Jahr/Monat/Tag`.

Beim Media-Library-Update verschiebt `migrate-archive-layout.py` bereits
archivierte TTS-Assets, die noch unter `Recordings` oder `Media-Library` liegen,
automatisch nach `TTS-Dateien`. `state.json` und das jeweilige Metadatenmanifest
werden dabei atomar aktualisiert. Die Migration ist wiederholbar.
