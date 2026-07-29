# v25 — exact v1.3.0 common-SCCH radio contract

## Evidence from the field log

The Sepura ISSI 5102 completed the initial ITSI attach and acknowledged the downlink, but the
D-LOCATION-UPDATE-ACCEPT contained
`scch_information_and_distribution_on_18th_frame: None` despite
`clch_needed=true` and `common_scch=true`. It then initiated repeated
`RoamingLocationUpdating` procedures.

The last known-good no-Core release is tag `v1.3.0`, commit
`7834f46748f3205ce6d3e6e1480345c6bdf27bca`. In that release MM always advertises
`Some(0x01)` when either `clch_needed` or `common_scch` is reported, including for
StayAlive terminals.

## Change

v25 restores that exact air-interface contract:

- `clch_needed || common_scch` -> frame-18 common-SCCH on TS1 (`0x01`)
- no suppression for StayAlive
- initial and roaming location-update handling otherwise remains the main-compatible path
- no changes to CMCE call setup, floor control, hangtime, release, SIP routing, or SWMI Core APIs

## Expected log

For ISSI 5102:

```text
MM: ISSI 5102 v1.3.0 radio compatibility assigns frame-18 common-SCCH on TS1
DLocationUpdateAccept ... scch_information_and_distribution_on_18th_frame: Some(1)
```

The prior message below must no longer occur:

```text
MM: ISSI 5102 is StayAlive; keeping it on the ordinary MCCH instead of assigning frame-18 common-SCCH
```
