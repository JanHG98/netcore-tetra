# Leitstelle dependency split

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: NetCore Control Room dependency split. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die aktuelle Server-/Transportbeschreibung steht in [Control Room](../../services/control-room/README.md); die gepflegten Quellen liegen unter [Quellcode](../../../system-backend/control-room) und [Rust-Core](../../../bins/netcore-control-room) . Frühere Startsnippets und Handshake-Korrekturen sind keine vollständige heutige Installationsanleitung.

Dieser Änderungsnachweis bewahrt den beschriebenen UI-/API-/Buildstand. Damalige Tokenregeln, Feldnamen, Überschreib- und Buildbefehle gelten für diese Revision und dürfen nicht als aktuelle Komplettanleitung übernommen werden.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### NetCore Control Room dependency split

This patch makes `netcore-control-room` depend on `tetra-entities` in protocol-only mode:

```toml
tetra-entities = { workspace = true, default-features = false }
```

`tetra-entities` now has:

- `runtime` feature: full base-station entity stack, SDR, Brew, dashboard, EchoLink, network transports.
- default = `["runtime"]`: existing base-station builds keep current behavior.
- protocol-only mode: `default-features = false`, used by Control Room Core.
- no-op `asterisk` feature on `netcore-control-room`, so `cargo build -p netcore-control-room --features asterisk` does not pull native voice-codec dependencies.

Build Control Room in the LXC with:

```bash
cargo clean -p tetra-entities -p netcore-control-room
cargo build --release -p netcore-control-room
```

Optional accepted no-op:

```bash
cargo build --release -p netcore-control-room --features asterisk
```

Do not use plain workspace-wide `cargo build --release --features asterisk` on the LXC unless you also want to build the base-station binary and therefore install SDR/voice-codec libraries.
