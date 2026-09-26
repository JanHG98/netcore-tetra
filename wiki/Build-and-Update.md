# Build, Update und Rollback

Ein Update beginnt mit **Ist-Stand, Sicherung und Testplan**. Die Basisstation und der LXC-Verbund haben unterschiedliche Installer; diesen Ablauf nur für die lokale TBS verwenden. [[Open-Lab-Deployment]] beschreibt die Backends.

## Version und Arbeitsbaum

```bash
cd /opt/netcore-tetra
git status --short --branch
git rev-parse HEAD
git remote -v
git fetch --all --prune
```

Lokale Änderungen und installierte Konfiguration erhalten. Bei Branchwechseln oder divergierenden Branches kein `reset --hard` oder erzwungenes `pull` verwenden. Nach Sicherung kann für einen sauberen Tracking-Branch `git pull --ff-only` folgen.

## TBS bauen und installieren

```bash
cargo build --release -p bluestation-bs
sudo systemctl stop tetra.service
sudo install -m 0755 target/release/bluestation-bs /usr/local/bin/bluestation-bs
sudo systemctl start tetra.service
sudo journalctl -u tetra.service -b -n 200 --no-pager
```

Die `systemd`-Unit muss **diese** Binärdatei und die tatsächlich geprüfte TOML starten. Nach Feature-/Toolchain-/nativen ABI-Wechseln oder rätselhaften Cache-Fehlern gezielt `cargo clean` und erneut bauen; das Löschen des gesamten Build-Verzeichnisses bei jeder kleinen Änderung ist nicht erforderlich. Default-Features umfassen Asterisk, Recording und Audio-Player. [[Common-Build-Errors]]

## Abnahme und Rückweg

Version im Dashboard/Log, geladene Konfiguration, RF-/SDR-Erkennung, Registrierung, Gruppen-/Einzelruf, SDS, Audio und Release nach dem Neustart kontrollieren. Bei Fehlern die letzte **kompatible** Binärdatei und Konfiguration gemeinsam zurückstellen. Ein altes Binary mit migrierter Datenbank ist kein verlässlicher Rollback. [[Abnahme]] · [[Backup-and-Fallback]]

Dashboard-Updates können einen Quellcode-Updatevorgang anstoßen; die gleichen Sicherungs- und Prüfschritte gelten. Der Rust-Workspace-Wert `1.3.0` aus `Cargo.toml` ist nicht automatisch der Git-Release-Tag oder die Version eines entfernten Hosts. [[Projektstand]]
