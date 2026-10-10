# v1.7.0 registration air-interface restore

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: v1.7.0 registration air-interface restore. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die heutige lokale Registrierung und Rufeinleitung liegen in [MM](../../../crates/tetra-entities/src/mm/mm_bs.rs), [MLE](../../../crates/tetra-entities/src/mle/mle_bs.rs) und [CMCE](../../../crates/tetra-entities/src/cmce/cmce_bs.rs). [Registrierung und Gruppenbindung](../../wiki/registrierung-und-gruppenbindung.md) erläutert den aktuellen Einstieg; alte `mqtt`-/`swmi`-Branches und Vergleichshashes bleiben historische Referenzen.

Funkgeräte-ISSIs, Carrier, Softwareversionen, Diagnosefolgerungen und damalige Prüfresultate beziehen sich auf die genannten Läufe. Ein grüner Build, eine syntaktische Prüfung oder der Rückgriff auf v1.7.0 belegt keine heutige On-Air-Gesamtfreigabe.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### v1.7.0 registration air-interface restore

Live tests on the `mqtt` branch showed repeated self-initiated `RoamingLocationUpdating` from ISSI 5102 even after several MM compatibility experiments. The known-good release `v1.7.0` predates the central Call-Control/Node-Gateway integration and is confirmed by the operator to register immediately, allow PTT immediately, and remain stable.

This branch therefore restores the **radio-facing registration handshake** of `v1.7.0` while retaining the current central-core integration around it.

Restored behavior:

- no synthetic `StayAlive` Energy Saving Information on an initial registration when the MS did not request Energy Economy;
- AIv2/common-SCCH terminals receive a `D-LOCATION-UPDATE-ACCEPT` with the same Location Update type they requested, matching v1.7.0;
- ordinary registration again carries the same SSI field shape as v1.7.0 (`Some(ISSI)`), because that exact wire behavior is the known-good baseline for this lab;
- the experimental post-affiliation `D-LOCATION-UPDATE-COMMAND` fast-settle is removed, so the initial registration sequence is not followed by a second forced Demand Location Update.

Central Subscriber/Group/Call-Control/Node-Gateway hooks are not rolled back. The change is deliberately limited to the air-interface MM registration behavior.

Validation: `cargo fmt --all` and `cargo check -p tetra-entities --lib` pass in CI.
