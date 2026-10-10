# API-Beispiele im offenen Labor

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../../system-backend/media-switch/src/state.rs) · [src/http.rs](../../../../system-backend/media-switch/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Diese Aufrufe sind eine Bedien- und Prüfanleitung für den Media Switch, kein Protokoll bereits ausgeführter Tests. Der Dienst muss laufen; `127.0.0.1:8130` funktioniert nur bei einem Listener auf Loopback oder allen Interfaces. Nach LXC-Installation die tatsächliche Adresse aus `/etc/netcore/lxc-network.env` verwenden. Platzhalter für Node-, Ruf- und Aufnahme-IDs durch vorhandene IDs aus den lesenden API-Antworten ersetzen. Schreibende Beispiele verändern den Laborzustand.

```bash
curl http://127.0.0.1:8130/api/v1/status
curl http://127.0.0.1:8130/api/v1/sessions
curl http://127.0.0.1:8130/api/v1/streams
curl http://127.0.0.1:8130/api/v1/buffers
curl 'http://127.0.0.1:8130/api/v1/recorder/taps?after=0&limit=10'
```

Stream stummschalten:

```bash
curl -X POST http://127.0.0.1:8130/api/v1/sessions/CALL-ID/mute \
  -H 'Content-Type: application/json' \
  -d '{"node_id":"tbs-b","logical_ts":3,"muted":true}'
```

Testframe einspeisen:

```bash
curl -X POST http://127.0.0.1:8130/api/v1/sessions/CALL-ID/inject \
  -H 'Content-Type: application/json' \
  -d '{"payload":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]}'
```

Die 35 Nullbytes zeigen nur die akzeptierte Framegröße. Sie sind kein erzeugtes TETRA-Sprachsignal und kein Audioqualitätstest. Für eine Injection muss `CALL-ID` zu einer vorhandenen, nutzbaren Session gehören; eine API-Annahme belegt noch keine hörbare Wiedergabe am Funkgerät.
