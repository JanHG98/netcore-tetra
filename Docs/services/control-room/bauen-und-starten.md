# Leitstelle: Bauen und Starten

Stand: **9. Oktober 2026**. Befehle aus dem Repository-Hauptverzeichnis ausführen. Server und CLI gehören zum Hauptworkspace; die Desktop-UI ist ein eigener Workspace. Quellcode, vorhandene Konfiguration und einen laufenden Dienst beim Aktualisieren getrennt behandeln.

## Hauptzweig vorbereiten

In einer sauberen Arbeitskopie:

```bash
git fetch origin
git switch main
git pull --ff-only
```

Lokale Änderungen erhalten. Für einen festgelegten Rollout den geprüften Commit verwenden; kein historischer `control-room`-Featurezweig ist erforderlich.

## Server im Control-Room-LXC

Der Server braucht keine SDR-/TETRA-Codec-Buildabhängigkeiten:

```bash
cargo build --locked --release -p netcore-control-room
./target/release/netcore-control-room --config system-backend/control-room/config/control-room.example.toml --no-auth
```

Dies ist ein manueller Teststart. Läuft bereits `netcore-control-room.service`, statt einer zweiten Instanz den [LXC-Installationsweg](installation-im-lxc.md) verwenden. Die WebUI öffnet auf `http://<CONTROL-ROOM-IP>:9010/`.

```bash
curl -fsS http://127.0.0.1:9010/health/live
curl -fsS http://127.0.0.1:9010/health/ready
```

Readiness kann bei fehlenden Fachdiensten eingeschränkt sein, obwohl Liveness bereits erfolgreich ist. Beispieladressen unter `[[services]]` vor dem Betrieb anpassen.

## TBS und zentraler Telemetriepfad

Auf der Basisstation mit installierten SDR-/Codec-Abhängigkeiten:

```bash
cargo build --locked --release -p bluestation-bs
./target/release/bluestation-bs /PFAD/ZUR/BESTEHENDEN/config.toml
```

Die Standardfeatures enthalten Asterisk, Recording und Audio Player. Im zentralen Netzbetrieb verbindet sich die TBS mit **Node Gateway**, und der Control Room liest dessen `/ws/backend`-Telemetrie. Dazu in der Control-Room-Konfiguration die realen Werte setzen:

```toml
[node_gateway]
enabled = true
url = "ws://NODE-GATEWAY-IP:8080/ws/backend"
reconnect_secs = 3
timeout_secs = 10
stale_after_secs = 30
```

Der ältere direkte TBS-Control-Room-Pfad bleibt als Kompatibilität möglich:

```toml
[control_room]
enabled = true
host = "CONTROL-ROOM-IP"
port = 9010
use_tls = false
endpoint_path = "/node"
```

Dieses Beispiel ist kein Auftrag, eine bereits am Node Gateway angebundene TBS umzustellen. Die bestehende Topologie nach dem [zentralen Rolloutplan](../../roadmaps/zentraler-netzbetrieb.md) weiterführen.

## Kommandozeilen-Bedienplatz

```bash
cargo build --locked --release -p netcore-control-room-operator
./target/release/netcore-control-room-operator --api http://CONTROL-ROOM-IP:9010 dashboard
```

Alternativ `NETCORE_CONTROL_ROOM_API` setzen. Im ausgelieferten OPEN LAB sind keine Zugangsdaten erforderlich. Eine native Desktop-UI wird separat nach der [Desktop-Anleitung](ui/README.md) gebaut; sie ist nicht dieselbe Anwendung wie die CLI.

## Rollenverteilung

| System | Aufgabe |
|---|---|
| TBS | Funk, SDR, Codec und lokale Ruf-/Medienverarbeitung |
| Node Gateway | Zentrale TBS-Verbindung und Backend-Telemetrie |
| Control-Room-LXC | Lageaggregation, typisierte Bedienaktionen und Operator-Audit |
| Bedienrechner | Browser, CLI oder separat gebaute Desktop-UI |

**Quellabgleich: 9. Oktober 2026.** [Server](../../../bins/netcore-control-room/src), [CLI](../../../system-backend/control-room/operator/src), [Konfiguration](../../../system-backend/control-room/config).
