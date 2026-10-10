# API-Beispiele im offenen Testmodus

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../../system-backend/node-gateway/src/state.rs) · [src/http.rs](../../../../system-backend/node-gateway/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

Diese Aufrufe sind eine Bedien- und Prüfanleitung für den Node Gateway, kein Protokoll bereits ausgeführter Tests. Der Dienst muss laufen; `127.0.0.1:8080` funktioniert nur bei einem Listener auf Loopback oder allen Interfaces. Nach LXC-Installation die tatsächliche Adresse aus `/etc/netcore/lxc-network.env` verwenden. Platzhalter für Node-, Ruf- und Aufnahme-IDs durch vorhandene IDs aus den lesenden API-Antworten ersetzen. Schreibende Beispiele verändern den Laborzustand.

Node-Liste:

```bash
curl http://127.0.0.1:8080/api/v1/nodes
```

Node anpingen:

```bash
curl -X POST http://127.0.0.1:8080/api/v1/nodes/tbs-test/ping
```

Node trennen:

```bash
curl -X POST http://127.0.0.1:8080/api/v1/nodes/tbs-test/disconnect
```

Beispielkommando:

```bash
curl -X POST http://127.0.0.1:8080/api/v1/nodes/tbs-test/commands \
  -H 'Content-Type: application/json' \
  -d '{"operator_id":"jan-test","command":{"KickMs":{"issi":1234567}}}'
```

Die genaue JSON-Darstellung der Rust-Enums entspricht der bestehenden Serde-Darstellung des `ControlCommand`-Typs.

`ping`, Disconnect und Kommandoannahme beziehen sich auf eine vorhandene verbundene Node-ID. Eine Command-ID vom Gateway bestätigt zunächst das Einreihen; die fachliche `ControlResponse` der TBS getrennt in Ereignissen und Node-Zustand prüfen.
