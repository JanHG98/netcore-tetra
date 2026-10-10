# Recorder- und Player-Schnittstellen

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../system-backend/media-switch/src/state.rs) · [src/http.rs](../../../system-backend/media-switch/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Der Media Switch stellt zwei bewusst getrennte Tap-Arten bereit.

## Diagnose-Tap

```text
GET /api/v1/taps?limit=<n>
```

Dieser Endpunkt liefert nur kompakte Metadaten für WebUI und Fehlersuche. Er enthält keine Sprachpayloads und ist nicht für Aufzeichnung oder Wiedergabe geeignet.

## Replay-fähiger Recorder-Tap

```text
GET /api/v1/recorder/taps?after=<sequenz>&limit=<n>
```

Jeder Datensatz enthält:

- monotone Tap-Sequenz
- Zeitstempel
- logische Call-ID, Rufart und Rufphase
- Source-ISSI, GSSI beziehungsweise Calling-/Called-ISSI
- Priorität und Notrufkennzeichen
- aktuellen Floor Holder als Sprecher-ISSI
- Quell-TBS, logischen Timeslot und Quellsequenz
- Zielanzahl
- Codec-Bezeichnung
- vollständige gepackte Payload
- Kennzeichen für künstlich eingespeiste Frames

Die Antwort nennt außerdem älteste und neueste Sequenz im Ring sowie `dropped_before`, wenn der angefragte Cursor nicht mehr vollständig verfügbar ist.

Der Ring wird über `media.recorder_tap_history_frames` begrenzt. Er dient ausschließlich zur kurzen Entkopplung und ist kein persistentes Archiv.

## Audio Player / Media Library

Die bestehende Injection-API ist der Anschluss für frameweise Wiedergabe:

```text
POST /api/v1/sessions/{call-id}/inject
```

Sie akzeptiert exakt einen gepackten 35-Byte-TETRA-ACELP-Frame. Ein Audio Player kann daher `audio.tacelp` frameweise lesen und im ursprünglichen 60-ms-Takt einspeisen, ohne den Media Switch um einen Codec oder Dateisystemzugriff zu erweitern.

## Entkopplungsregel

Recorder und Player bleiben externe Dienste. Der Media Switch führt keine synchronen Aufrufe zu ihnen aus und wartet nicht auf deren Verarbeitung. Ein Recorder-Ausfall darf einen Ruf nicht beeinflussen; ein Player-Ausfall darf höchstens die jeweilige Injection beenden.

## Aktuelle Systemgrenze

Media Library und Recorder sind vorhandene separate Dienste. Eine Mediadatei in der Library erzeugt aber nicht automatisch einen Ruf oder einen laufenden Player: Rufaufbau und Injection müssen über die jeweils implementierte Bedien- beziehungsweise Anwendungsschnittstelle erfolgen. Diagnose-Tap und Vollframe-Tap bleiben In-Memory-Daten und werden bei Media-Switch-Neustart neu aufgebaut.
