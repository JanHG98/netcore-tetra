# brew-server

**Quellstand:** `main` (`c3ccdb4`, 09.10.2026). Dieses importierte Brew-Paket ist ein eigener Cargo-Workspace (`1.9.0`) und wird aus `misc/brew-server/` gebaut. Es ist eine separate Legacy-/Experimentalkomponente neben TBS Connect, kein zusätzlicher Eintrag der 26 Open-Lab-Inventardienste. Broker-Port und optionaler Dashboard-Port stammen aus `brew-server.toml`; die Beispiele sind keine Aussage über laufende Hosts.

**Quellen:** [misc/brew-server](../../../../misc/brew-server) · [Repository-Root](../../../..). Bei Befehlen das in der Anleitung angegebene Arbeitsverzeichnis beachten.

Experimental Rust Brew core for linking two or more MidnightBlue Basestation or Flowstation TETRA base stations.

Reference spec from https://wiki.tetrapack.online/tetra/specifications/brew/

- **What's new:** see [CHANGELOG.md](../../../../misc/brew-server/CHANGELOG.md)
- **Configuration and feature docs:** see the [wiki](https://github.com/ysamouhos/brew-server/wiki)

## Run directly

Requires a Rust toolchain and a C compiler (the vendored ACELP codec in
`third_party/tetra-codec/` is compiled by `build.rs`).

```bash
cargo run --release -- brew-server.toml
```

## Run via Docker

```bash
docker compose up --build
```

## Health check

```bash
curl http://127.0.0.1:9000/healthz
```
