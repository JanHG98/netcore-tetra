# Gemeinsame Plattform und LXC-Integration – historische Einspielung

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: SWMI Core 1 / Paket P. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Für gemeinsame Verträge und Deployment-Schicht sind [shared](../../services/shared/README.md) und [Quellcode](../../../system-backend/shared) die aktuellen Einstiege. Die damaligen Paketlisten beschreiben keine heute vollständig installierte Anlage.

Die damaligen ZIP-Namen, vollständigen Quellbaum-Ersetzungen und Rückbaubefehle gehören zum beschriebenen Lieferstand. Für ein heutiges Update den aktuellen Komponenten-Installer und die bestehende Standortkonfiguration verwenden; diese Einspielsequenz nicht ungeprüft erneut ausführen.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### Package P anwenden

Das Paket ist bereits vollständig in den Repository-Stand integriert.

## Prüfen

```bash
python3 tools/check_shared_platform.py
python3 deploy/open-lab/netcore-deploy.py validate
python3 deploy/open-lab/netcore-deploy.py render
python3 tests/integration/open_lab_contract_test.py
```

Mit installiertem Rust-Toolchain zusätzlich:

```bash
cargo test --locked --package netcore-contracts \
  --package netcore-service-common \
  --package netcore-database-common \
  --package netcore-telemetry-common
cargo fmt --all --check
cargo clippy --locked --package netcore-contracts \
  --package netcore-service-common \
  --package netcore-database-common \
  --package netcore-telemetry-common --all-targets -- -D warnings
```

## LXC-Konfiguration rendern

```bash
cp deploy/open-lab/inventory.example.toml deploy/open-lab/inventory.toml
$EDITOR deploy/open-lab/inventory.toml
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml plan
python3 deploy/open-lab/netcore-deploy.py --inventory deploy/open-lab/inventory.toml render
```

Vor `apply` müssen die gerenderten Dateien unter `deploy/open-lab/generated/configs/` geprüft werden.
