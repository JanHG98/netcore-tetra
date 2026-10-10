# API-Beispiele im offenen Labor

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../../system-backend/call-control/src/state.rs) · [src/http.rs](../../../../system-backend/call-control/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Diese Aufrufe sind eine Bedien- und Prüfanleitung für den Call Control, kein Protokoll bereits ausgeführter Tests. Der Dienst muss laufen; `127.0.0.1:8120` funktioniert nur bei einem Listener auf Loopback oder allen Interfaces. Nach LXC-Installation die tatsächliche Adresse aus `/etc/netcore/lxc-network.env` verwenden. Platzhalter für Node-, Ruf- und Aufnahme-IDs durch vorhandene IDs aus den lesenden API-Antworten ersetzen. Schreibende Beispiele verändern den Laborzustand.

```bash
curl http://127.0.0.1:8120/api/v1/status
```

Gruppenruf:

```bash
curl -X POST http://127.0.0.1:8120/api/v1/calls/group \
  -H 'Content-Type: application/json' \
  -d '{"gssi":15502,"source_issi":9999,"priority":3,"target_nodes":[]}'
```

Individualruf:

```bash
curl -X POST http://127.0.0.1:8120/api/v1/calls/individual \
  -H 'Content-Type: application/json' \
  -d '{"calling_issi":9999,"called_issi":1234,"simplex":true,"priority":3,"target_node":null}'
```

Alle Beispiele sind absichtlich ohne Authorization-Header. Das gilt ausschließlich für die isolierte Testumgebung.

Die Beispiel-ISSIs und GSSI müssen im Labornetz vorhanden und zugelassen sein. Ein Individualruf ohne `target_node` benötigt eine gültige Mobility-Core-Route. Ein Operator-Floor verlangt aktive Legs und eine vom Media Switch bestätigte RouteReady-Revision; die API-Annahme eines Rufauftrags allein belegt noch keinen Sprachpfad.
