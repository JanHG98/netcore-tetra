# Installation der lokalen Basisstation

Diese Anleitung beschreibt eine **quellbasierte TBS-Installation** auf Debian/Raspberry Pi OS. Die 26 Backend-Rollen werden separat bereitgestellt. Die lokale Umgebung muss zu Hardware, Treiber, Codec und Konfiguration passen. Für einen großen Aufbau anschließend [Dienste und Pi-Images im Open Lab bereitstellen](dienste-und-pi-images-bereitstellen.md) lesen.

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

Die Cargofeatures sind in [Cargo.toml](../../bins/bluestation-bs/Cargo.toml) definiert; der Default-Build enthält `asterisk`, `recording` und `audio-player`. Falls eine native Codec-Bibliothek fehlt, ist ein erfolgreich gebautes Minimal-Binary nicht gleichbedeutend mit aktivierter SIP-/Audiofunktion. [Häufige Buildfehler](haeufige-buildfehler.md)

## 2. Quellcode und lokalen Stand sichern

```bash
git clone https://github.com/JanHG98/netcore-tetra.git /opt/netcore-tetra
cd /opt/netcore-tetra
git status --short --branch
git rev-parse HEAD
```

Der gezeigte Clone benötigt Schreibrechte unter `/opt`; für einen neuen Quellbaum das Verzeichnis vorher mit dem vorgesehenen Buildbenutzer anlegen. Für eine bereits bestehende Installation **keinen neuen Clone über Daten und lokale Änderungen schreiben**. Vor Updates Branch/Commit, Konfiguration, Units und Persistenz sichern. [Backup und Fallback](datensicherung-und-fallback.md)

## 3. TBS-Konfiguration erstellen

```bash
cp Docs/basisstation.config.sanitized.example.toml config.local.toml
chmod 600 config.local.toml
python3 -c 'import pathlib,tomllib; tomllib.loads(pathlib.Path("config.local.toml").read_text()); print("TOML OK")'
```

Die Datei ist **nur ein syntaktisch lesbares Beispiel**. Die markierten Werte für SDR, Funk, Netz, Directory, SIP, Brew, Control Room und Zugangsdaten gegen den echten Standort prüfen; ungenutzte Integrationen deaktivieren. Die TOML-Syntaxprüfung ersetzt nicht die Laufzeitvalidierung. Falls `[control_room]` für das verteilte Backend aktiviert wird: `host` auf den Node Gateway setzen und `endpoint_path = "/ws/node"`. [Konfiguration der TBS](basisstation-konfigurieren.md) · [Netzwerk, Ports und Protokolle](netzwerk-und-ports.md)

## 4. Bauen und zuerst manuell starten

```bash
cd /opt/netcore-tetra
cargo build --release -p bluestation-bs
RUST_LOG=info ./target/release/bluestation-bs ./config.local.toml
```

Bei fehlenden nativen Default-Abhängigkeiten lässt sich für **einen bewusst eingeschränkten Test** `cargo build --release -p bluestation-bs --no-default-features` verwenden. Für den gewünschten Audio-/SIP-Betrieb müssen die Features später korrekt gebaut werden. Vor dem Senden RF-Kette messen und Betriebsvoraussetzungen prüfen. Beim Start auf Konfigurations-Fallback, SDR-Identität, RX/TX-Center, Sample-Rate, Downlink und Time-Skips achten. Danach Registrierung, SDS, Ruf, Release und Dashboard testen. [Hardware, SDR und HF-Aufbau](hardware-sdr-und-hf-aufbau.md) · [Inbetriebnahme und Abnahme](inbetriebnahme-und-abnahme.md)

## 5. Systemd erst nach dem manuellen Test

Die getestete Binärdatei an einen festen Ort installieren, Konfiguration nach `/etc/netcore/` übernehmen und den Dienstbenutzer passend für SDR, Audio- und Cache-Pfade berechtigen. Die Beispiel-Unit unter [Basisstation als systemd-Dienst betreiben](basisstation-als-dienst.md) an diese echten Pfade anpassen. **Nicht gleichzeitig** einen manuellen und einen systemd-TBS-Prozess mit denselben RF- und Dashboard-Ressourcen starten.

## 6. Weitere Komponenten

| Bedarf | Weiterführende Seite |
|---|---|
| Backend-Dienste und Deployment-VM | [Dienste und Pi-Images im Open Lab bereitstellen](dienste-und-pi-images-bereitstellen.md) |
| Directory-Namen/Status | [Namen und Metadaten im NetCore Directory](namen-und-metadaten-im-directory.md) |
| Teilnehmer/Gruppen | [Provisioning und Datenhoheit](teilnehmer-und-gruppen-anlegen.md) |
| TTS/Medien | [Audio-Zentrale](audio-aufnahmen-und-tts.md) |
| Telefonie | [SIP, lokaler Asterisk und Brew](telefonie-sip-und-brew.md) |
| Leitstelle | [Control Room und Node Gateway](leitstelle-und-node-gateway.md) |

## Alternative: personalisiertes Pi-Image

Für ein neues Pi-System kann der vorhandene [Imagebuilder](../services/deployment-core/README.md) auf der Deployment-VM verwendet werden. Vor dem Flashen Artefakt und SHA-256 vergleichen; danach physischen Boot, SDR, Standortkonfiguration und VPN-Regel separat prüfen. Diese Alternative ersetzt keinen Test der realen Funkkette.

## Quellen zur Pflege dieser Seite

[Startargument und Stackaufbau](../../bins/bluestation-bs/src/main.rs) · [Buildfeatures](../../bins/bluestation-bs/Cargo.toml).
