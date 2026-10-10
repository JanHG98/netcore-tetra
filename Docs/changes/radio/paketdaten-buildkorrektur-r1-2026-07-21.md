# Paketdaten-Gateway Compile-Fix R1

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: Paketdaten-Gateway Compile-Fix R1. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Der heutige Datenpfad wird in [Packet Core](../../services/packet-core/README.md), [IP Gateway](../../services/ip-gateway/README.md) und [lokaler SNDCP-/IPv4-Code](../../../crates/tetra-entities/src/sndcp) beschrieben. Der frühere WAP-MVP, Multi-PDCH-Schritt und allgemeine IP-Ausbau sind unterschiedliche Entwicklungsstände; deren einzelne Grenzen nicht zu einem heutigen Gesamtprofil vermischen.

Funkgeräte-ISSIs, Carrier, Softwareversionen, Diagnosefolgerungen und damalige Prüfresultate beziehen sich auf die genannten Läufe. Ein grüner Build, eine syntaktische Prüfung oder der Rückgriff auf v1.7.0 belegt keine heutige On-Air-Gesamtfreigabe.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### Paketdaten-Gateway Compile-Fix R1

Behobene Compilerfehler:

1. `IPV4_UDP_HEADER_BYTES` wird nun zentral in `sndcp/ip.rs` definiert.
2. `snei_optional_section()` erzeugt das optionale SNEI-Type-2-IE für SN-PAGE.
3. `IpError::UnsupportedProtocol(u8)` bildet nicht unterstützte IPv4-Protokolle ab.
4. Die nftables-Setup-Closure besitzt einen expliziten `Result<(), GatewayError>`-Typ.
5. WAP-Rohantworten berechnen ihr 548-Byte-Budget aus `576 - IPV4_UDP_HEADER_BYTES`.

Der Patch setzt auf `netcore-tetra-packet-data-complete-2026-07-21.zip` auf.
