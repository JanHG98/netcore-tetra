# Fast-Realtime Call/Media Path

Diese Ausbaustufe entfernt das bisherige Zwei-Sekunden-Reconcile-Polling aus dem aktiven
Sprachpfad. Der Callgraph wird weiterhin per HTTP initialisiert, Änderungen werden danach
aber sofort und revisionsbehaftet über einen dedizierten WebSocket übertragen.

## Ablauf eines neuen Rufes

```text
TBS A / Call Control
        |
        | call_created + vollständiger Call-Snapshot
        v
Media Switch baut Quell-Leg auf und puffert frühe Frames
        |
        | leg_ready für weitere Ziel-Legs
        v
Media Switch hat alle erwarteten aktiven Legs und verbundene Medienwege
        |
        | POST /api/v1/media/route-ready { logical_call_id, revision }
        v
Call Control darf Floor-Anforderung freigeben
        |
        v
Vorgepufferte Frames werden geordnet an alle Ziel-Legs verteilt
```

## Schnittstellen

- Call-Control-Ereignisse: `ws://<call-control>:8120/ws/media`
- WebSocket-Subprotokoll: `netcore-call-control-media-v1`
- RouteReady-ACK: `POST http://<call-control>:8120/api/v1/media/route-ready`
- HTTP-Fallback-Snapshot: `GET http://<call-control>:8120/api/v1/calls`

Das HTTP-Reconcile ist nur noch ein Sicherheitsnetz (`reconcile_secs = 15`) und kein
Bestandteil der normalen Rufaufbauzeit. Fehlgeschlagene RouteReady-ACKs werden beim
Fallback-Abgleich erneut versucht.

## Medienparameter

```toml
[media]
frame_duration_ms = 60
jitter_buffer_frames = 2
min_jitter_buffer_frames = 1
max_jitter_buffer_frames = 12
adaptive_jitter = true
cold_start_buffer_frames = 5
cold_start_buffer_max_age_ms = 600
```

Der adaptive Puffer startet bei 120 ms und kann bei stabilem Transport auf 60 ms sinken.
Die ersten fünf 60-ms-Pakete eines kalten Rufes werden bis zum vollständigen Routinggraphen
zurückgehalten. Ein unvollständiger Graph wird nicht vorzeitig bespielt.

## Bauen und prüfen

```bash
cargo build --release \
  -p bluestation-bs \
  -p netcore-control-room \
  -p netcore-control-room-operator \
  --features "bluestation-bs/asterisk,bluestation-bs/recording,bluestation-bs/audio-player"

cargo build --release \
  -p netcore-call-control \
  -p netcore-media-switch

cargo test -p netcore-media-switch
```

Für den Testbetrieb bleiben die Dienste bewusst im offenen Labormodus ohne Tokens und TLS.
