# Häufige Buildfehler

## `pkg-config` findet SoapySDR nicht

```bash
pkg-config --modversion SoapySDR
sudo apt install libsoapysdr-dev pkg-config
```

Bei selbst installierten Bibliotheken `PKG_CONFIG_PATH` und Architekturpfad prüfen.

## Native Sprachcodec-Bibliothek fehlt

Asterisk, Recorder und Audio-Player können native Codec-Abhängigkeiten benötigen. Prüfen:

```bash
pkg-config --list-all | grep -i tetra
ldconfig -p | grep -i tetra
```

Nach Installation oder ABI-Wechsel der Bibliothek neu bauen; bei veralteten Linkartefakten das betroffene Paket bereinigen.

## Linkerfehler nach Featurewechsel

```bash
cargo clean -p bluestation-bs
cargo build --release -p bluestation-bs
```

Zuerst die erste Linkermeldung, Featureauswahl und Bibliothekspfade prüfen. Ein vollständiges `cargo clean` ist sinnvoll, wenn der gezielte Neubau den Cachefehler nicht behebt; ein zusätzliches `rm -rf target` nach `cargo clean` bringt keinen Nutzen.

## Rust-Version zu alt

Das Workspace nutzt Edition 2024.

```bash
rustup update stable
rustc --version
cargo --version
```

## Build des ganzen Workspace zieht unnötige Abhängigkeiten

Auf einem Leitstellenserver gezielt bauen:

```bash
cargo build --release -p netcore-control-room
cargo build --release -p netcore-control-room-operator
```

Auf der Basisstation:

```bash
cargo build --release -p bluestation-bs
```

## Speicherplatz reicht nicht

```bash
df -h
sudo du -sh target ~/.cargo/registry ~/.cargo/git 2>/dev/null
```

Das Buildverzeichnis `target` enthält reproduzierbare Artefakte. Vor dem Bereinigen sicherstellen, dass kein laufender Dienst seine einzige Binary daraus verwendet, und das installierte Binary separat erhalten. Cargo-Caches nur bewusst bereinigen, da sie später erneut heruntergeladen bzw. gebaut werden.

## Fehler erst zur Laufzeit

Ein erfolgreicher Build beweist nicht, dass der dynamische Loader alle Bibliotheken findet:

```bash
ldd target/release/bluestation-bs | grep 'not found'
```

## Weiterführend

[Installation der lokalen Basisstation](basisstation-installieren.md) · [Build, Update und Rollback](software-bauen-und-aktualisieren.md) · [Fehlersuche](fehlersuche.md)

## Quellen zur Pflege dieser Seite

[Workspace und Edition](../../Cargo.toml) · [TBS-Features](../../bins/bluestation-bs/Cargo.toml) · [Operatorpaket](../../system-backend/control-room/operator/Cargo.toml).
