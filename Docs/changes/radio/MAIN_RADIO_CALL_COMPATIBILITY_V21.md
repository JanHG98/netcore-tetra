# NetCore-TETRA v21 — Main radio-call compatibility

This release deliberately removes the experimental Sepura-specific group-release changes from v20.
The radio-facing group-call lifecycle is restored to the same sequence used by the last working
`main` baseline:

1. `U-SETUP` starts the group call.
2. `U-TX-CEASED` produces exactly one group-addressed `D-TX-CEASED` on FACCH.
3. The call remains in the normal `NoActiveSpeaker` hangtime state.
4. Hangtime expiry emits the established release sequence from `main`:
   two group-addressed FACCH `D-RELEASE` PDUs plus one MCCH fallback.
5. The traffic circuit closes only after queued FACCH/STCH signalling has drained.

Removed from v20:

- immediate full call destruction inside the first `U-TX-CEASED` handler;
- duplicate ISSI-addressed `D-TX-CEASED`;
- extra ISSI-addressed FACCH and MCCH `D-RELEASE` copies;
- default zero-second group hangtime.

Those changes overloaded the FACCH/STCH release window. A late TX block could then drop the only
signalling the terminal needed, leaving the Sepura in the call until its local timeout and causing a
subsequent service-restoration location update.

The Core/LXC services remain present. Central policy is evaluated before call admission, but the
accepted local radio call uses the established `main` CMCE/UMAC lifecycle without Core-specific
changes to floor release, call release, timers or air-interface PDU repetition.

Simplex and duplex remain unrestricted by Provisioning Core policy. Explicit SIP routing remains
outside subscriber provisioning and may target arbitrary numbers accepted by the configured dial
plan.
