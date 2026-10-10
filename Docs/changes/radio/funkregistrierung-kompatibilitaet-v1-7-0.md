# Restore v1.7.0 radio-registration compatibility

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: Restore v1.7.0 radio-registration compatibility. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die heutige lokale Registrierung und Rufeinleitung liegen in [MM](../../../crates/tetra-entities/src/mm/mm_bs.rs), [MLE](../../../crates/tetra-entities/src/mle/mle_bs.rs) und [CMCE](../../../crates/tetra-entities/src/cmce/cmce_bs.rs). [Registrierung und Gruppenbindung](../../wiki/registrierung-und-gruppenbindung.md) erläutert den aktuellen Einstieg; alte `mqtt`-/`swmi`-Branches und Vergleichshashes bleiben historische Referenzen.

Funkgeräte-ISSIs, Carrier, Softwareversionen, Diagnosefolgerungen und damalige Prüfresultate beziehen sich auf die genannten Läufe. Ein grüner Build, eine syntaktische Prüfung oder der Rückgriff auf v1.7.0 belegt keine heutige On-Air-Gesamtfreigabe.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### Restore v1.7.0 radio-registration compatibility

Live testing on ISSI 5102 showed repeated idle `RoamingLocationUpdating` after later registration changes. PTT reaches CMCE, and the U-DISCONNECT handler is unchanged from v1.7.0, so this patch restores only the radio-facing MM registration behaviour from the last known-good v1.7.0 path.

Restored behaviour:
- AIv2 + `common_scch` radios mirror the requested Location Update type.
- Other radios retain periodic-registration ACCEPT behaviour when T351 is configured.
- `D-LOCATION-UPDATE-ACCEPT.ssi` is populated as in v1.7.0.
- First attach without ESM has no ESI; re-registration preserves prior ESM including StayAlive.

Acceptance criterion: no idle REREG loop, normal group PTT, and user U-DISCONNECT returns the handset to idle after D-RELEASE.
