# Restore v1.7.0 radio-registration compatibility

Live testing on ISSI 5102 showed repeated idle `RoamingLocationUpdating` after later registration changes. PTT reaches CMCE, and the U-DISCONNECT handler is unchanged from v1.7.0, so this patch restores only the radio-facing MM registration behaviour from the last known-good v1.7.0 path.

Restored behaviour:
- AIv2 + `common_scch` radios mirror the requested Location Update type.
- Other radios retain periodic-registration ACCEPT behaviour when T351 is configured.
- `D-LOCATION-UPDATE-ACCEPT.ssi` is populated as in v1.7.0.
- First attach without ESM has no ESI; re-registration preserves prior ESM including StayAlive.

Acceptance criterion: no idle REREG loop, normal group PTT, and user U-DISCONNECT returns the handset to idle after D-RELEASE.
