# ISSI 5102 MM stability hybrid fix

## Live evidence

With `mqtt` at `3d6c219d`, an idle AIv2/common-SCCH terminal repeatedly emitted
`RoamingLocationUpdating` roughly every 16-49 seconds. The base station mirrored each roaming
request back as `RoamingLocationUpdating`, re-emitted `Affiliate`, and the loop continued.

Earlier live testing showed that the protocol-conservative registration-accept variant could
reach CMCE `U-SETUP`, but mirroring roaming updates did not stop the idle re-registration loop.

## Repair

This combines the two observed-good properties instead of rolling the entire radio stack backward:

- initial ITSI attach and DemandLocationUpdating keep their requested type;
- AIv2/common-SCCH roaming and service-restoration roaming updates are acknowledged as
  `PeriodicLocationUpdating` when periodic registration is configured;
- the optional SSI in D-LOCATION UPDATE ACCEPT stays absent because NetCore does not allocate ASSI;
- unsolicited `StayAlive` Energy Saving Information is omitted; real active Eg1..Eg3 grants survive
  re-registration;
- stored group affiliations are only replayed after an actual subscriber-registry drop, not on every
  harmless roaming refresh.

The group-call CMCE/FACCH release FSM is deliberately unchanged by this repair. Stabilize MM first;
explicit U-DISCONNECT can then be evaluated without a simultaneous registration loop.
