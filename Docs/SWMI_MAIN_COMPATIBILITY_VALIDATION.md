# Validation – SWMI/Main Compatibility Build

## Grundlage

Verglichen wurden die vom Projekt bereitgestellten vollständigen Quellstände:

- letzter funktionierender `main`-Stand ohne Core-Erweiterungen
- aktueller `swmi`-Stand mit Core-, Mobility-, Provisioning- und Fallback-Funktionen

## Geprüfter Fixumfang

- MM-Accept-Auswahl für AIv2/Common-SCCH gegen `main` abgeglichen
- Migration-Completion weiterhin als `DemandLocationUpdating`
- Asterisk-Wählplan vor lokaler ISSI-Routingentscheidung
- SIP-Ziele ohne Subscriber-/Provisioning-Eintrag
- Simplex-/Duplex-Anforderung unabhängig vom gespeicherten
  `ClassOfMs.freq_simplex_duplex`-Telemetriewert
- bestehender offener Edge-Fallback unverändert erhalten

## Statische Prüfungen

- 82 TOML-Dateien erfolgreich geparst
- 59 Shell-Skripte mit `bash -n` geprüft
- Python-Dateien in `system-backend`, `tools`, `tests` und `deploy` kompiliert
- Delimiter-/Strukturprüfung der geänderten Rust-Dateien
- Full-System-, Group-Core-, Call-Control-, MM-Mobility-, MLE-Cell-Change-,
  Subscriber-Core-, Node-Gateway-, Media-Switch-, Recorder-, SDS-Router- und
  weitere vorhandene statische Projektprüfungen erfolgreich

Ein vollständiger Cargo-Build war in der Erstellungsumgebung nicht möglich, weil
dort kein Rust-Toolchain vorhanden war und keine externe Installation möglich war.
Der verbindliche Compiler- und Laufzeittest erfolgt deshalb beim Build auf der
Basisstation.

Einige bereits im gelieferten unveränderten SWMI-Stand vorhandene globale
Prüfskripte melden unabhängig von diesem Fix veraltete Marker, nicht ausführbare
E2E-Dateirechte oder mitgelieferte PDF-Dateien. Diese Befunde waren im Ausgangs-ZIP
identisch und wurden nicht durch den Air-Interface-/Routing-Fix verursacht.
