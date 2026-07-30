# Phase 1 – Mobility Core als kanonische Teilnehmerroute

Diese Phase macht den Mobility Core zur zentralen Quelle für die Frage:

> Auf welcher TBS ist eine ISSI aktuell registriert?

## Neue API

```http
GET /api/v1/subscribers/{issi}/route
```

Beispiel:

```json
{
  "issi": 4010001,
  "state": "confirmed",
  "registered": true,
  "serving_node": "tbs-04010001",
  "location_area": 1,
  "node_connected": true,
  "node_stale": false,
  "first_seen": "2026-07-30T18:10:00Z",
  "last_seen": "2026-07-30T18:12:00Z",
  "age_ms": 241,
  "route_generation": 4,
  "confidence": "confirmed"
}
```

Mögliche Zustände:

- `confirmed`: Teilnehmer registriert, Serving-TBS online und nicht stale.
- `stale`: Registrierung bekannt, aber TBS-Verbindung nicht aktuell bestätigt.
- `detached`: Teilnehmer wurde abgemeldet oder durch Timeout entfernt.
- `unknown`: Mobility Core kennt die ISSI nicht; HTTP 404.

`route_generation` wird bei einem Wechsel der Serving-TBS sowie bei Attach/Detach erhöht. Damit können spätere Komponenten veraltete Routen erkennen.

## Call Control

Bei einem Individualruf ohne manuell angegebene `target_node` fragt Call Control nun zuerst den Mobility Core ab. Nur eine bestätigte Route wird standardmäßig akzeptiert. Die lokale Teilnehmerbeobachtung im Call Control bleibt vorerst für UI, Diagnose und optionalen Fallback erhalten, ist aber nicht mehr die bevorzugte Routingquelle.

Konfiguration:

```toml
[mobility_core]
enabled = true
base_url = "http://10.0.1.123:8090"
timeout_ms = 1500
allow_local_fallback = false
accept_stale_route = false
```

Für den regulären Testbetrieb sollten `allow_local_fallback` und `accept_stale_route` auf `false` bleiben. Dann fällt ein Ruf sauber fehl, statt möglicherweise auf einer alten TBS gesucht zu werden.
