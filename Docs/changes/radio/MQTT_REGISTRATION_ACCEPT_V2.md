# MQTT registration-accept v2

## Evidence from the live TBS

The v1 fix is active, but ISSI 5102 still completes acknowledged `ItsiAttach`, then repeats `ItsiAttach` and `RoamingLocationUpdating` without ever producing CMCE `U-SETUP`. The v1 coercion of roaming accepts to `PeriodicLocationUpdating` therefore does not settle the MS state.

## Protocol corrections

1. **No fake ASSI allocation.** ETSI EN 300 392-2 clause 16.9.2.7 defines the optional SSI in `D-LOCATION UPDATE ACCEPT` as ASSI/(V)ASSI. NetCore currently has no ASSI allocator/mapping, so the field is omitted. The subscriber ISSI remains the layer-2 address.
2. **Do not rewrite the registration type.** The BS acknowledges the `LocationUpdateType` requested by the MS instead of converting roaming updates into periodic updates based on the local T351 timer.
3. **Do not invent an energy-economy response.** `Energy saving information` is omitted when the MS did not request an EE mode and no non-StayAlive mode is active. Internally, NetCore still treats the subscriber as StayAlive.

## Expected live log

Initial attach without an EE request should now contain:

```text
DLocationUpdateAccept { location_update_accept_type: ItsiAttach ssi: None ... energy_saving_information: None ... }
```

A subsequent roaming update should now contain:

```text
MM: ISSI 5102 completing RoamingLocationUpdating with matching D-LOCATION-UPDATE-ACCEPT
DLocationUpdateAccept { location_update_accept_type: RoamingLocationUpdating ssi: None ... }
```

The next discriminator is CMCE: after the registration settles, a PTT press should create a `USetup`/`U-SETUP` trace. If it still does not, the remaining fault is below CMCE and should be investigated in MLE/MAC access state rather than call-control policy.
