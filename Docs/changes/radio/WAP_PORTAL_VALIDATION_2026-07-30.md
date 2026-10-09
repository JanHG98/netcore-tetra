# WAP-Portal-Validierung 2026-07-30

## Umfang

Validiert wurden:

- 19 statische XHTML-Basic-Seiten
- 19 statische WML-1.1-Seiten
- formatreine Navigation innerhalb XHTML und WML
- Erreichbarkeit aller Seiten ab der jeweiligen Startseite
- kompakte Openwave-Routen und lesbare Aliase
- Bytebudgets der dynamischen Kurzseiten
- Kompatibilitaet von `/status.xhtml?s=1` mit der Health-Seite

## Ergebnis

```text
Static portal validation OK:
19 XHTML + 19 WML pages,
valid XML,
same-format links,
all pages reachable.
```

Die dynamischen Seiten wurden mit dem Referenz-Snapshot aus den Rust-Tests modelliert:

| Format | Seiten | groesste Seite | Grenze |
|---|---:|---:|---:|
| XHTML | 19 | 126 Byte | 144 Byte |
| XHTML Status | 1 | 102 Byte | 104 Byte |
| WML | 19 | 131 Byte | 144 Byte |

## Lokaler Validator

```bash
python3 contrib/wap-portal/validate.py
```

## Rust-Tests auf dem Zielsystem

```bash
cargo test -p tetra-entities sndcp::wap_portal
cargo test -p tetra-entities sndcp::wap_ip
```

Die aktuelle Arbeitsumgebung enthielt keinen Rust-Toolchain. Deshalb konnten die Rust-Tests hier nicht kompiliert ausgefuehrt werden. Die XML-, Link- und Groessenpruefungen wurden vollstaendig ausgefuehrt.
