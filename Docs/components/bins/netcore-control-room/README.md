# NetCore Control Room Core

**Quellstand:** `main`, `c3ccdb4`, 09.10.2026. Quellen: [Rust-Binary](../../../../bins/netcore-control-room), [Dienstkonfiguration und Installer](../../../../system-backend/control-room). Die Angaben beschreiben den vorhandenen Code, keine bestätigte laufende Installation.

Der Rust-Core sammelt TBS-Telemetrie, föderiert Backend-Status und stellt eine eigene Browseroberfläche sowie HTTP- und WebSocket-Schnittstellen bereit. Fachzustände wie Teilnehmerfreigabe, Gruppen, Serving-TBS und Rufsteuerung bleiben bei ihren zuständigen Diensten. Die [zentrale Dienstanleitung](../../../services/control-room/README.md) beschreibt Deployment, Authentisierung und Operatorbetrieb.

## Vorhandene Funktionen

- Node-Sessions, Heartbeats, Teilnehmer-, Gruppen-, Ruf-, SDS-, Notfall-, Standort- und RF-Ansichten;
- Föderations-Poller, Dienstübersicht, Incident-Akten und Schichtbuch;
- eingebettete Browser-WebUI, Directory-Import und ein separates Operator-Paket;
- optionale lokale Benutzer mit Passwortprüfung und Rollen für HTTP-Aktionen, optionales Node-Token;
- optionale SQLite-Persistenz für Benutzer und Verlauf, begrenzter In-Memory-Verlauf ohne Persistenz;
- korrelierte Control Commands und Antworten über den Node-Kanal.

Die Standardeinstellungen des Rust-Cores aktivieren weder Authentisierung noch Persistenz automatisch. Für den Betrieb gelten die tatsächlich gestartete TOML-Datei und CLI-Overrides. Das offene Lab-Profil ist ein eigenes Deployment-Profil; die Existenz von Auth-Code schaltet es nicht auf jedem Dienst ein.

## Bauen und starten

Im Repository-Root:

```bash
cargo build --release -p netcore-control-room
./target/release/netcore-control-room --help
./target/release/netcore-control-room --config system-backend/control-room/config/control-room.example.toml
```

Die CLI unterstützt unter anderem `--bind`, `--node-path`, `--ui-path`, `--history-limit`, `--database`, `--auth-enabled`, `--no-auth`, `--node-token`, `--bootstrap-user`, `--bootstrap-password` und `--no-persistence`. Geheimnisse möglichst über eine geschützte Konfiguration übergeben; ein CLI-Passwort kann in der Prozessliste sichtbar sein.

Ohne TOML oder Overrides gelten `127.0.0.1:9010`, `/node`, `/ui` und das in `config.rs` definierte Verlaufslimit. `--database` aktiviert die SQLite-Option. Konfigurationspfad und Dienst-Unit vor einem Update überprüfen.

## Schnittstellen

| Pfad | Zweck |
| --- | --- |
| `/` | Browser-WebUI |
| `/health/live`, `/health/ready` | Liveness und Readiness |
| `/api/v1/status`, `/api/v1/dependencies`, `/api/v1/config` | gemeinsamer Dienstvertrag und redigierte Konfiguration |
| `/api/v1/services`, `/api/v1/control-room/overview` | Föderation und gemeinsame Übersicht |
| `/api/v1/incidents`, `/api/v1/shift-log` | Incident-Akten und Schichtbuch |
| `/api/overview`, `/api/state`, `/api/nodes` | lokale Telemetrie und Node-Sessions |
| `/api/directory`, `/api/subscribers`, `/api/groups`, `/api/calls`, `/api/sds` | Fachansichten |
| `/api/events`, `/api/commands` | Verlauf und Control Commands |
| `/api/login`, `/api/me`, `/api/admin/users` | optionale lokale Anmeldung und Benutzerverwaltung |
| `/openapi.json`, `/api/v1/openapi.json` | implementierter HTTP-Vertrag |
| `/node`, `/ui` | konfigurierbare Node-/UI-WebSockets |

Die zentrale Backend-Topologie verbindet die TBS über den **Node Gateway** auf Port 8080. Eine direkte Verbindung zum Core auf 9010 ist eine alternative Diagnose-/Standalone-Konfiguration und ersetzt den Gateway nicht im Inventory. Die genaue Command-Payload aus dem aktiven OpenAPI-/Control-Vertrag übernehmen; freie Beispiel-JSONs sind kein universeller Funkbefehl.
