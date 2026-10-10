# WAP-Integration – Validierungsprotokoll

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: WAP-Integration – Validierungsprotokoll. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Der heutige Datenpfad wird in [Packet Core](../../services/packet-core/README.md), [IP Gateway](../../services/ip-gateway/README.md) und [lokaler SNDCP-/IPv4-Code](../../../crates/tetra-entities/src/sndcp) beschrieben. Der frühere WAP-MVP, Multi-PDCH-Schritt und allgemeine IP-Ausbau sind unterschiedliche Entwicklungsstände; deren einzelne Grenzen nicht zu einem heutigen Gesamtprofil vermischen.

Funkgeräte-ISSIs, Carrier, Softwareversionen, Diagnosefolgerungen und damalige Prüfresultate beziehen sich auf die genannten Läufe. Ein grüner Build, eine syntaktische Prüfung oder der Rückgriff auf v1.7.0 belegt keine heutige On-Air-Gesamtfreigabe.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### WAP-Integration – Validierungsprotokoll

Stand: 21.07.2026

## Durchgeführt

- alle 447 Rust-Quelldateien mit einer Rust-Tree-sitter-Grammatik geparst: keine Syntaxfehler
- `Cargo.toml`, `config.toml` und `config.toml.fallback` als TOML geparst
- alle vollständigen Struct-Literale in den geänderten Rust-Dateien heuristisch gegen die im Workspace definierten Felder geprüft
- MLE-Routing für unacknowledged `TL-UNITDATA` bis zum SNDCP-Entity ergänzt
- TS2-Reservierung gegen den bestehenden CMCE-/Brew-Timeslot-Allocator geprüft
- folgende Protokollvektoren unabhängig nachgerechnet:

```text
IPv4/UDP:
4500001f00070000201116c4c0000201c0000202c00023f0000b0000776170
IPv4-Header-Prüfsumme über den fertigen Header: 0x0000

SN-ACTIVATE PDP CONTEXT ACCEPT, 70 Bit, gepaddet:
02 90 8e 82 80 00 38 80 10

WTP Result + WSP ConnectReply:
12 93 cc 02 01 08 00 03 80 84 21 03 81 84 21

SN-UNITDATA, NSAPI 2, ohne Kompression:
42 00 45 00 00 14
```

## Nicht in dieser Arbeitsumgebung möglich

Im bereitgestellten Container waren weder `cargo` noch `rustc`/`rustfmt` vorhanden. Der vollständige Rust-Typcheck, die Cargo-Tests, der Release-Build und ein echter On-Air-Test mit einem Funkgerät konnten deshalb hier nicht ausgeführt werden.

Die dafür vorgesehenen Befehle stehen in `Docs/guides/wap/wap-statusdienst-ueber-sndcp.md`. Vor einem produktiven Start müssen mindestens `cargo test` und der vollständige Release-Build auf dem Raspberry Pi beziehungsweise dem normalen Build-Rechner durchlaufen.
