# Registration loop and group PTT fix (v18)

## Symptom

An AIv2/common-SCCH terminal registers and affiliates successfully, but repeatedly sends
`RoamingLocationUpdating`. The terminal reports the TMO service as unavailable when PTT is
pressed and the base-station log contains no CMCE `U-SETUP`.

## Cause

The MM compatibility path preserved the request type for every registration from this terminal
class. A known terminal therefore received `RoamingLocationUpdating` again in
`D-LOCATION-UPDATE-ACCEPT`, remained in a terminal-side re-registration cycle and did not
progress to CMCE call setup.

## Behaviour after the fix

- Initial AIv2/common-SCCH `ITSI attach` is still acknowledged as `ITSI attach`.
- A later roaming refresh from an already-known terminal is acknowledged as
  `PeriodicLocationUpdating` when periodic registration is enabled.
- Migration completion still uses `DemandLocationUpdating`.
- If periodic registration is disabled, the requested type is preserved.

Expected log after a refresh:

```text
MM: ISSI 5102 known roaming refresh settled as PeriodicLocationUpdating to prevent re-registration loop
```

After the terminal has settled, pressing PTT should produce:

```text
<- U-SETUP
```

The group policy and membership paths are unchanged by this fix.
