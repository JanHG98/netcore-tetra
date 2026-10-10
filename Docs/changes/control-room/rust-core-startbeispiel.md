# Control Room Core - Startsnippet

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: Control Room Core - Startsnippet. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die aktuelle Server-/Transportbeschreibung steht in [Control Room](../../services/control-room/README.md); die gepflegten Quellen liegen unter [Quellcode](../../../system-backend/control-room) und [Rust-Core](../../../bins/netcore-control-room) . Frühere Startsnippets und Handshake-Korrekturen sind keine vollständige heutige Installationsanleitung.

Dieser Änderungsnachweis bewahrt den beschriebenen UI-/API-/Buildstand. Damalige Tokenregeln, Feldnamen, Überschreib- und Buildbefehle gelten für diese Revision und dürfen nicht als aktuelle Komplettanleitung übernommen werden.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### Control Room Core - Startsnippet

```bash
cargo build --release --features asterisk && ./target/release/netcore-control-room --bind 127.0.0.1:9010
```

Oder direkt:

```bash
cargo run --release -p netcore-control-room -- --bind 127.0.0.1:9010
```

Base-Station-Config:

```toml
[control_room]
enabled = true
host = "127.0.0.1"
port = 9010
use_tls = false
endpoint_path = "/node"

node_id = "tbs-04010001"
station_name = "NetCore TBS 04010001"
site = "Lab / Rack"
```

Schnelle Checks:

```bash
curl http://127.0.0.1:9010/health | jq
curl http://127.0.0.1:9010/api/overview | jq
curl 'http://127.0.0.1:9010/api/events?limit=20&quiet=true' | jq
curl http://127.0.0.1:9010/api/rf | jq
curl http://127.0.0.1:9010/api/health/full | jq
```

Command-Shortcuts:

```bash
curl -X POST http://127.0.0.1:9010/api/nodes/tbs-04010001/commands/kick \
  -H 'Content-Type: application/json' \
  -d '{"operator_id":"jan","issi":2010001}'

curl -X POST http://127.0.0.1:9010/api/nodes/tbs-04010001/commands/dgna \
  -H 'Content-Type: application/json' \
  -d '{"operator_id":"jan","issi":2010001,"gssi":1001,"attach":true}'

curl -X POST http://127.0.0.1:9010/api/nodes/tbs-04010001/commands/clear-emergency \
  -H 'Content-Type: application/json' \
  -d '{"operator_id":"jan","issi":0}'
```
