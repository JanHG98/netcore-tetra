# Sepura initial registration completion fix (v22)

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
