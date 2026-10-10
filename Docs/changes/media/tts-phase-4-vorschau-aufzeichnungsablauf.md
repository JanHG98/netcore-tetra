# Phase 4: TTS-Aufzeichnungsworkflow

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: Phase 4: TTS-Aufzeichnungsworkflow. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Der gepflegte zentrale TTS-Weg steht in [Piper/TTS](../../services/tts/README.md) und [Media Library](../../services/media-library/README.md). Die alten lokalen Piper-/Vorlagen-/Recording-Workflows dokumentieren die Entwicklung; ihre Unterschiede zu zentraler Erzeugung und Freigabe bleiben erhalten. Aktueller lokaler Aussendepfad: [Quellcode](../../../crates/tetra-entities/src/net_audio_player); Funkrufsteuerung: [Quellcode](../../../crates/tetra-entities/src/cmce/cmce_bs.rs).

Dieser Bericht hält den damaligen Audio-/TTS-/Pagingfehler oder Ausbau fest. Beobachtete Einzelrufe und Logursachen gelten für den dokumentierten Test; daraus folgt keine neue allgemeine Funk-/Dateisystemabnahme.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### Phase 4: TTS-Aufzeichnungsworkflow

Dieses Dokument ersetzt den früheren Vorschau-und-direkt-senden-Ablauf.

TTS kann keinen Funkruf mehr unmittelbar starten. Jede erzeugte TTS wird als benannte WAV-Datei in der lokalen Aufzeichnungsbibliothek gespeichert. Die spätere Aussendung erfolgt ausschließlich, indem der Eintrag unter `Aufzeichnungen & TTS-WAVs` ausgewählt und wie eine normale Gesprächsaufzeichnung gesendet wird.

Die vollständige Beschreibung steht in:

```text
Docs/changes/media/PHASE4_TTS_RECORDING_LIBRARY_WORKFLOW.md
```
