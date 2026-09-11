from pathlib import Path

path = Path("crates/tetra-entities/src/mm/mm_bs.rs")
text = path.read_text()


def replace_once(old: str, new: str, label: str) -> None:
    global text
    count = text.count(old)
    if count != 1:
        raise SystemExit(f"{label}: expected exactly one match, found {count}")
    text = text.replace(old, new, 1)


replace_once(
    """        // v1.7.0 radio compatibility: preserve the previously granted energy-saving
        // state on re-registration, including StayAlive. On the very first attach, when neither
        // the MS nor client state supplies an ESM, the optional IE remains absent.
        let effective_esm_request = pdu.energy_saving_mode.or(prior_esm);
        let esi = effective_esm_request
            .map(|esm| Self::grant_energy_saving(prim.received_address.ssi, esm));
""",
    """        // D-LOCATION UPDATE ACCEPT carries Energy Saving Information only when the MS
        // requested an economy mode, or when a real Eg1..Eg3 grant must survive a
        // re-registration. StayAlive is our local default, not an unsolicited optional IE.
        // This keeps the wire behaviour that allowed ISSI 5102 to reach CMCE U-SETUP.
        let effective_esm_request = pdu
            .energy_saving_mode
            .or_else(|| prior_esm.filter(|mode| *mode != EnergySavingMode::StayAlive));
        let esi = effective_esm_request
            .map(|esm| Self::grant_energy_saving(prim.received_address.ssi, esm));
""",
    "energy-saving response",
)

replace_once(
    """        // Fix: when a *known* MS re-registers without supplying a group report, but we
        // still hold groups for it in client_mgr, re-emit Affiliate for those groups so
        // CMCE's group_listeners (and Brew) are resynced with what the MS believes.
        if !is_new && !_has_groups {
""",
    """        // Only restore stored affiliations after a confirmed registry drop. A normal
        // roaming refresh does not remove the subscriber from CMCE/Brew and must not generate
        // a fresh Affiliate on every location update; that churn can race call setup/release.
        if was_dropped && !_has_groups {
""",
    "coverage-return affiliation",
)

replace_once(
    """        // v1.7.0 compatibility path. AIv2 terminals advertising common SCCH were
        // deliberately exempted from forcing PeriodicLocationUpdating in the ACCEPT. For this
        // capability set the MS-requested type is mirrored (ITSI attach, roaming update, ...).
        let hytera_periodic_accept_compat = pdu
            .class_of_ms
            .as_ref()
            .map(|class| class.common_scch && class.air_interface_version >= 2)
            .unwrap_or(false);

        let _ = self.client_mgr.set_client_class_of_ms(issi, pdu.class_of_ms);
        self.config.state_write().subscribers.set_duplex_capable(issi, duplex_capable);

        // Reset periodic registration timer on every successful registration.
        self.client_mgr.reset_registration_timer(issi);

        // Registration / affiliation / EE state changed — persist for restart recovery (debounced).
        self.recovery_mark_dirty();

        // v1.7.0 periodic-registration compatibility.
        let periodic_secs = self.config.config().cell.periodic_registration_secs;
        let accept_type = if periodic_secs > 0 && !hytera_periodic_accept_compat {
            LocationUpdateType::PeriodicLocationUpdating
        } else {
            if periodic_secs > 0 && hytera_periodic_accept_compat {
                tracing::debug!(
                    "MM: ISSI {} uses AIv2/common-SCCH; mirroring {:?} in D-LOCATION-UPDATE-ACCEPT instead of forcing PeriodicLocationUpdating (v1.7 compatibility)",
                    issi,
                    pdu.location_update_type
                );
            }
            pdu.location_update_type
        };
""",
    """        // Capability discriminator for the strict AIv2/common-SCCH radios seen in the
        // live traces. Initial ITSI attach and DemandLocationUpdating keep their requested type;
        // only roaming/service-restoration roaming updates are settled into the configured
        // periodic-registration state.
        let ai_v2_common_scch = pdu
            .class_of_ms
            .as_ref()
            .map(|class| class.common_scch && class.air_interface_version >= 2)
            .unwrap_or(false);

        let _ = self.client_mgr.set_client_class_of_ms(issi, pdu.class_of_ms);
        self.config.state_write().subscribers.set_duplex_capable(issi, duplex_capable);

        // Reset periodic registration timer on every successful registration.
        self.client_mgr.reset_registration_timer(issi);

        // Registration / affiliation / EE state changed — persist for restart recovery (debounced).
        self.recovery_mark_dirty();

        let periodic_secs = self.config.config().cell.periodic_registration_secs;
        let roaming_refresh = matches!(
            pdu.location_update_type,
            LocationUpdateType::RoamingLocationUpdating
                | LocationUpdateType::ServiceRestorationRoamingLocationUpdating
        );
        let accept_type = if periodic_secs > 0 && ai_v2_common_scch && roaming_refresh {
            tracing::info!(
                "MM: ISSI {} AIv2/common-SCCH roaming update settled as PeriodicLocationUpdating to stop the REREG loop",
                issi
            );
            LocationUpdateType::PeriodicLocationUpdating
        } else {
            pdu.location_update_type
        };
""",
    "location-update accept selector",
)

replace_once(
    """            // Restore v1.7.0 wire behaviour first; revisit ASSI semantics behind multi-vendor tests.
            ssi: Some(issi as u64),
""",
    """            // The optional SSI here is an allocated ASSI/(V)ASSI, not the subscriber ISSI.
            // NetCore has no ASSI allocator/mapping yet, so do not make the MS adopt its ISSI as ASSI.
            ssi: None,
""",
    "D-LOCATION UPDATE ACCEPT SSI",
)

path.write_text(text)

Path("Docs/MM_5102_STABILITY_HYBRID_FIX.md").write_text(
    """# ISSI 5102 MM stability hybrid fix

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
"""
)
