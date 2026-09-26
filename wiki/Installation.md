# Installation der lokalen Basisstation

Diese Anleitung beschreibt eine **quellbasierte TBS-Installation** auf Debian/Raspberry Pi OS. Sie installiert keine der 24 Backend-LXCs. Die lokale Umgebung muss zu Hardware, Treiber, Codec und Konfiguration passen. Für einen großen Aufbau anschließend [[Open-Lab-Deployment]] lesen.

## 1. System und SDR vorbereiten

- 64-Bit-Linux, ausreichend RAM/Storage, stabile Versorgung und Zeitquelle;
- Rust/Cargo mit Unterstützung für Edition 2024, C/C++-Toolchain, `pkg-config`;
- SoapySDR und passender **konkreter SDR-Treiber**; native Codec-Abhängigkeiten für Default-Build mit Asterisk/Audio;
- `ffmpeg` für Medien-/TTS-Pfade, Netzwerkzugriff zu bewusst aktivierten Diensten.

```bash
sudo apt update
sudo apt install -y git curl build-essential pkg-config cmake clang libsoapysdr-dev soapysdr-tools ffmpeg
SoapySDRUtil --find
SoapySDRUtil --probe="driver=<DEIN-TREIBER>"
```

Die Cargofeatures sind in [`bins/bluestation-bs/Cargo.toml`](https://github.com/JanHG98/netcore-tetra/blob/main/bins/bluestation-bs/Cargo.toml) definiert; der Default-Build enthält `asterisk`, `recording` und `audio-player`. Falls eine native Codec-Bibliothek fehlt, ist ein erfolgreich gebautes Minimal-Binary nicht gleichbedeutend mit aktivierter SIP-/Audiofunktion. [[Common-Build-Errors]]

## 2. Quellcode und lokalen Stand sichern

```bash
git clone https://github.com/JanHG98/netcore-tetra.git /opt/netcore-tetra
cd /opt/netcore-tetra
git status --short --branch
git rev-parse HEAD
```

Für eine bereits bestehende Installation **keinen neuen Clone über Daten und lokale Änderungen schreiben**. Vor Updates Branch/Commit, Konfiguration, Units und Persistenz sichern. [[Backup-and-Fallback]]

## 3. TBS-Konfiguration erstellen

```bash
cp Docs/basisstation.config.sanitized.example.toml config.local.toml
chmod 600 config.local.toml
python3 -c 'import pathlib,tomllib; tomllib.loads(pathlib.Path("config.local.toml").read_text()); print("TOML OK")'
```

Die Datei ist **nur ein syntaktisch lesbares Beispiel**. Die markierten Werte für SDR, Funk, Netz, Directory, SIP, Brew, Control Room und Zugangsdaten gegen den echten Standort prüfen; ungenutzte Integrationen deaktivieren. Die TOML-Syntaxprüfung ersetzt nicht die Laufzeitvalidierung. Falls `[control_room]` für das verteilte Backend aktiviert wird: `host` auf den Node Gateway setzen und `endpoint_path = "/ws/node"`. [[Configuration]] · [[Netzwerk-und-Ports]]

## 4. Bauen und zuerst manuell starten

```bash
cd /opt/netcore-tetra
cargo build --release -p bluestation-bs
RUST_LOG=info ./target/release/bluestation-bs ./config.local.toml
```

Bei fehlenden nativen Default-Abhängigkeiten lässt sich für **einen bewusst eingeschränkten Test** `cargo build --release -p bluestation-bs --no-default-features` verwenden. Für den gewünschten Audio-/SIP-Betrieb müssen die Features später korrekt gebaut werden. Vor dem Senden RF-Kette messen und Betriebsvoraussetzungen prüfen. Beim Start auf Konfigurations-Fallback, SDR-Identität, RX/TX-Center, Sample-Rate, Downlink und Time-Skips achten. Danach Registrierung, SDS, Ruf, Release und Dashboard testen. [[Hardware-und-RF]] · [[Abnahme]]

## 5. Systemd erst nach dem manuellen Test

Die getestete Binärdatei an einen festen Ort installieren, Konfiguration nach `/etc/netcore/` übernehmen und den Dienstbenutzer passend für SDR, Audio- und Cache-Pfade berechtigen. Die Beispiel-Unit unter [[Systemd-Service]] an diese echten Pfade anpassen. **Nicht gleichzeitig** einen manuellen und einen systemd-TBS-Prozess mit denselben RF- und Dashboard-Ressourcen starten.

## 6. Weitere Komponenten

| Bedarf | Weiterführende Seite |
|---|---|
| 24 Backend-LXCs | [[Open-Lab-Deployment]] |
| Directory-Namen/Status | [[NetCore-Directory]] |
| Teilnehmer/Gruppen | [[Provisioning]] |
| TTS/Medien | [[Audio-Zentrale]] |
| Telefonie | [[SIP-und-Brew]] |
| Leitstelle | [[Control-Room]] |
