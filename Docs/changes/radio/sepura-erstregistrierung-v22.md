# Sepura initial registration completion fix (v22)

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: Sepura initial registration completion fix (v22). Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die heutige lokale Registrierung und Rufeinleitung liegen in [MM](../../../crates/tetra-entities/src/mm/mm_bs.rs), [MLE](../../../crates/tetra-entities/src/mle/mle_bs.rs) und [CMCE](../../../crates/tetra-entities/src/cmce/cmce_bs.rs). [Registrierung und Gruppenbindung](../../wiki/registrierung-und-gruppenbindung.md) erläutert den aktuellen Einstieg; alte `mqtt`-/`swmi`-Branches und Vergleichshashes bleiben historische Referenzen.

Funkgeräte-ISSIs, Carrier, Softwareversionen, Diagnosefolgerungen und damalige Prüfresultate beziehen sich auf die genannten Läufe. Ein grüner Build, eine syntaktische Prüfung oder der Rückgriff auf v1.7.0 belegt keine heutige On-Air-Gesamtfreigabe.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### Sepura initial registration completion fix (v22)

Observed sequence before this fix:

1. U-LOCATION-UPDATE-DEMAND (`ItsiAttach`) without an energy-saving request.
2. D-LOCATION-UPDATE-ACCEPT without `energy_saving_information`.
3. The BS nevertheless stored `StayAlive` internally.
4. A few seconds later the terminal sent `RoamingLocationUpdating`.
5. Only the second D-LOCATION-UPDATE-ACCEPT contained explicit `StayAlive`.

The initial response and the BS state were therefore inconsistent. v22 always puts an
explicit `StayAlive` allocation into the first D-LOCATION-UPDATE-ACCEPT when neither
the terminal nor an existing client state supplies another energy-saving mode.

No call-control, group-call, release, Core-policy, SIP, Simplex or Duplex behaviour is
changed by this fix. The radio call FSM remains the main-compatible v21 implementation.
