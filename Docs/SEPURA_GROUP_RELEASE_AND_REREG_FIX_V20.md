# Sepura group-release and re-registration fix v20

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
