# Änderung: End-to-End-Aussendung aus der Media Library

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: Änderung: End-to-End-Aussendung aus der Media Library. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Für heutige Medienfreigabe, Archivierung und Stations-Playout [Media Library](../../services/media-library/README.md), [Piper/TTS](../../services/tts/README.md) und [Quellcode](../../../system-backend/media-library) verwenden. Die Basisstation verarbeitet die vorbereitete Datei über ihren [lokalen Audiopfad](../../../crates/tetra-entities/src/net_audio_player); historische Migrationen/Rechtereparaturen sind vor erneutem Einsatz gegen die tatsächlichen Archivpfade zu prüfen.

Dieser Bericht hält den damaligen Audio-/TTS-/Pagingfehler oder Ausbau fest. Beobachtete Einzelrufe und Logursachen gelten für den dokumentierten Test; daraus folgt keine neue allgemeine Funk-/Dateisystemabnahme.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### Änderung: End-to-End-Aussendung aus der Media Library

- neuer Playout-Modus `basisstation`
- mehrere TBS-Ziele mit Standardstation in `[playout]`
- Cookie-Login am geschützten TBS-Dashboard
- Übergabe von Asset-ID, GSSI/ISSI und Priorität an `/api/audio/play`
- TBS übernimmt vollständigen Download, lokalen Cache, nativen Codec und Rufaufbau
- Remote-Job-ID und Blockfortschritt werden in Media-Library-Jobs gespiegelt
- Abbruch über `/api/audio/stop`
- WAV-/TTS-Assets benötigen im Basisstationsmodus keinen zentralen TACELP-Cache
- Altpfad `media_switch` bleibt für vorhandene TACELP-/Session-Workflows erhalten
- bestehende Konfigurationen erhalten den `[playout]`-Block über eine idempotente Migration
