# Central Media Library TTS

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: Central Media Library TTS. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Für heutige Medienfreigabe, Archivierung und Stations-Playout [Media Library](../../services/media-library/README.md), [Piper/TTS](../../services/tts/README.md) und [Quellcode](../../../system-backend/media-library) verwenden. Die Basisstation verarbeitet die vorbereitete Datei über ihren [lokalen Audiopfad](../../../crates/tetra-entities/src/net_audio_player); historische Migrationen/Rechtereparaturen sind vor erneutem Einsatz gegen die tatsächlichen Archivpfade zu prüfen.

Dieser Bericht hält den damaligen Audio-/TTS-/Pagingfehler oder Ausbau fest. Beobachtete Einzelrufe und Logursachen gelten für den dokumentierten Test; daraus folgt keine neue allgemeine Funk-/Dateisystemabnahme.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### Central Media Library TTS

## Zielbild

- Die Basisstation erzeugt keine TTS-Dateien mehr lokal.
- Piper läuft ausschließlich im Media-Library-LXC auf `127.0.0.1:5005`.
- Texteingabe, Stimmen, Vorlagen, Speichern, Vorschau und Freigabe befinden sich in der Media-Library-WebUI.
- Erzeugte Durchsagen werden als normale Media-Library-Assets mit `kind = "tts"` verarbeitet.
- Die Basisstation sieht fertige TTS-Dateien im bestehenden Media-Library-Dateibrowser und lädt sie vor der Aussendung vollständig in ihren lokalen Audiocache.
- TTS-Archive landen unter `/mnt/nfs-share/TTS-Dateien/YYYY/MM/DD`.

## Installation

Siehe `Docs/changes/media/medienbibliothek-zentrale-spracherzeugung.md`.

## Migration der Basisstation

`install/update-basisstation.sh` entfernt bei Erfolg den lokalen `[tts]`-Abschnitt sowie die alten TTS-NFS-Felder aus `/etc/netcore/config.toml`, legt vorher ein Backup an und deaktiviert `netcore-piper.service` auf der Basisstation.
