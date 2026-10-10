# Piper HTTP route compatibility fix

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: Piper HTTP route compatibility fix. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Der gepflegte zentrale TTS-Weg steht in [Piper/TTS](../../services/tts/README.md) und [Media Library](../../services/media-library/README.md). Die alten lokalen Piper-/Vorlagen-/Recording-Workflows dokumentieren die Entwicklung; ihre Unterschiede zu zentraler Erzeugung und Freigabe bleiben erhalten. Aktueller lokaler Aussendepfad: [Quellcode](../../../crates/tetra-entities/src/net_audio_player); Funkrufsteuerung: [Quellcode](../../../crates/tetra-entities/src/cmce/cmce_bs.rs).

Dieser Bericht hält den damaligen Audio-/TTS-/Pagingfehler oder Ausbau fest. Beobachtete Einzelrufe und Logursachen gelten für den dokumentierten Test; daraus folgt keine neue allgemeine Funk-/Dateisystemabnahme.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### Piper HTTP route compatibility fix

Current Piper HTTP server versions expose the browser test page on `GET /`, the installed voice list on `GET /voices`, and synthesis on `POST /synthesize`.

FlowStation now treats `[tts].endpoint` as the provider base URL. Example:

```toml
[tts]
endpoint = "http://127.0.0.1:5005"
```

Internally FlowStation calls:

- `GET <endpoint>/voices` for availability checks
- `POST <endpoint>/synthesize` for WAV generation

For compatibility, an endpoint ending in `/synthesize` is also accepted and normalized back to the provider base URL before the two API routes are constructed.
