# Sepura group-release and re-registration fix v20

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: Sepura group-release and re-registration fix v20. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Der gepflegte lokale Rufablauf liegt in [CMCE](../../../crates/tetra-entities/src/cmce/cmce_bs.rs) und [UMAC](../../../crates/tetra-entities/src/umac/umac_bs.rs); [Rufabläufe](../../wiki/gruppen-und-einzelrufe.md) ordnet die Funktionen ein. Die nachfolgenden v20/v21-/FACCH-Varianten sind historische Experimente und kein Auftrag, eine ältere Binary zu installieren.

Funkgeräte-ISSIs, Carrier, Softwareversionen, Diagnosefolgerungen und damalige Prüfresultate beziehen sich auf die genannten Läufe. Ein grüner Build, eine syntaktische Prüfung oder der Rückgriff auf v1.7.0 belegt keine heutige On-Air-Gesamtfreigabe.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### Sepura group-release and re-registration fix v20

## Observed sequence

The captured run showed a valid group U-SETUP and U-TX-CEASED, followed by exactly five seconds in
`NoActiveSpeaker`. At release time the PHY skipped four late TX blocks. Fifteen seconds later the
terminal requested `ServiceRestorationRoamingLocationUpdating`, followed by repeated roaming
updates. The network call had already ended, but the source terminal had not reliably consumed the
release signalling.

## Changes

- Group hangtime defaults to `0` seconds; non-zero hangtime remains configurable.
- With zero hangtime a group call is released immediately after the first valid U-TX-CEASED.
- D-TX-CEASED is sent to the GSSI and directly to the transmitting ISSI.
- D-RELEASE is sent via group FACCH, source-ISSI FACCH, group MCCH and source-ISSI MCCH.
- Normal roaming/service-restoration refreshes no longer re-emit central Affiliate updates.
  Stored affiliations are restored only after a confirmed registry drop.
- Per-burst PHY logging is TRACE instead of INFO.
- An unavailable recorder archive emits one root warning instead of one warning per recording.

Use `RUST_LOG=info` for normal RF operation. DEBUG/TRACE logging is intended for short captures;
continuous verbose logging can create scheduler pressure on a Raspberry Pi.
