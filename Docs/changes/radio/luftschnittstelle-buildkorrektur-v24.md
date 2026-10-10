# Main Air-Interface Compile Fix v24

**Dokumenttyp: historischer Entwicklungs- und Prüfnachweis.** Gegenstand: Main Air-Interface Compile Fix v24. Die folgende Originalfassung behält ihre damaligen Versionen, Befunde und Grenzen; sie ist keine aktuelle Installationsfreigabe.

## Einordnung im heutigen Repository

Die heutige lokale Registrierung und Rufeinleitung liegen in [MM](../../../crates/tetra-entities/src/mm/mm_bs.rs), [MLE](../../../crates/tetra-entities/src/mle/mle_bs.rs) und [CMCE](../../../crates/tetra-entities/src/cmce/cmce_bs.rs). [Registrierung und Gruppenbindung](../../wiki/registrierung-und-gruppenbindung.md) erläutert den aktuellen Einstieg; alte `mqtt`-/`swmi`-Branches und Vergleichshashes bleiben historische Referenzen.

Funkgeräte-ISSIs, Carrier, Softwareversionen, Diagnosefolgerungen und damalige Prüfresultate beziehen sich auf die genannten Läufe. Ein grüner Build, eine syntaktische Prüfung oder der Rückgriff auf v1.7.0 belegt keine heutige On-Air-Gesamtfreigabe.

[Gesamtroadmap](../../roadmaps/gesamtroadmap.md) · [Dokumentationsindex](../../README.md)

## Dokumentierter damaliger Stand

### Main Air-Interface Compile Fix v24

This revision keeps the v23 main-compatible MM/MLE/CMCE radio path and adapts
`mle_bs.rs` to the newer SWMI LTPD SAP interface.

Changes:

- populate `LtpdMleUnitdataInd.received_address_type` from the received TETRA address;
- convert the legacy TLA `Option<i32>` channel-change handle into the strongly typed
  `Option<ChannelChangeHandle>` without accepting negative values;
- leave MM, CMCE, call release, Simplex/Duplex and SIP routing unchanged from v23.
