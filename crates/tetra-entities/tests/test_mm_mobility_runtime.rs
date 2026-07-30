// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use tetra_core::{TdmaTime, TetraAddress};
use tetra_entities::mm::components::client_state::{MmClientMobilityContext, MmClientState};
use tetra_entities::mm::mobility_runtime::{
    MmMobilityError, MmMobilityPhase, MmMobilityRuntime, MmMobilityTimeout,
    MM_MOBILITY_TIMEOUT_SLOTS,
};
use tetra_pdus::mm::enums::energy_saving_mode::EnergySavingMode;
use tetra_pdus::mm::enums::location_update_type::LocationUpdateType;
use tetra_pdus::mm::pdus::u_location_update_demand::ULocationUpdateDemand;

// Was: Führt den Arbeitsschritt `demand` für demand aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn demand(kind: LocationUpdateType) -> ULocationUpdateDemand {
    ULocationUpdateDemand {
        location_update_type: kind,
        request_to_append_la: false,
        cipher_control: false,
        ciphering_parameters: None,
        class_of_ms: None,
        energy_saving_mode: None,
        la_information: None,
        ssi: None,
        address_extension: None,
        group_identity_location_demand: None,
        group_report_response: None,
        authentication_uplink: None,
        extended_capabilities: None,
        proprietary: None,
    }
}

// Was: Führt den Arbeitsschritt `context` für Kontext aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn context(issi: u32) -> MmClientMobilityContext {
    MmClientMobilityContext {
        issi,
        state: MmClientState::Attached,
        groups: vec![100, 101],
        energy_saving_mode: EnergySavingMode::Eg2,
        monitoring_frame: Some(2),
        monitoring_multiframe: Some(1),
        class_of_ms: None,
        last_handle: 0,
        tei: Some(0x1234),
    }
}

#[test]
// Was: Führt den Arbeitsschritt `two_stage_migration_allocates_vassi_and_transfers_context` für two stage migration allocates vassi and transfers und weitere Angaben aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn two_stage_migration_allocates_vassi_and_transfers_context() {
    let now = TdmaTime::default();
    let mut runtime = MmMobilityRuntime::new();
    let mut first = demand(LocationUpdateType::MigratingLocationUpdating);
    first.address_extension = Some(0x0400_01);
    let (vassi, home_mni) = runtime
        .begin_migration(TetraAddress::issi(0x123456), 7, &first, now, |_| false)
        .unwrap();
    assert_eq!(home_mni, 0x0400_01);
    runtime
        .provide_migration_context(vassi, context(2_260_575), now.add_timeslots(1))
        .unwrap();

    let mut second = demand(LocationUpdateType::DemandLocationUpdating);
    second.ssi = Some(2_260_575);
    second.address_extension = Some(home_mni as u64);
    let completion = runtime
        .complete_migration(vassi, &second, now.add_timeslots(2))
        .unwrap();
    assert_eq!(completion.local_issi, vassi);
    assert_eq!(completion.home_issi, 2_260_575);
    assert_eq!(completion.imported_context.unwrap().groups, vec![100, 101]);
    assert_eq!(runtime.home_issi_for_local(vassi), Some(2_260_575));
    assert_eq!(runtime.snapshot(now.add_timeslots(2)).migrations[0].phase, MmMobilityPhase::MigrationAccepted);
}

#[test]
// Was: Führt den Arbeitsschritt `migration_rejects_a_changed_home_identity` für migration rejects a changed home identity aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn migration_rejects_a_changed_home_identity() {
    let now = TdmaTime::default();
    let mut runtime = MmMobilityRuntime::new();
    let mut first = demand(LocationUpdateType::MigratingLocationUpdating);
    first.ssi = Some(2_260_575);
    first.address_extension = Some(0x0400_01);
    let (vassi, _) = runtime
        .begin_migration(TetraAddress::issi(0x123456), 0, &first, now, |_| false)
        .unwrap();
    let mut second = demand(LocationUpdateType::DemandLocationUpdating);
    second.ssi = Some(9_999_999);
    second.address_extension = Some(0x0400_01);
    assert!(matches!(
        runtime.complete_migration(vassi, &second, now.add_timeslots(1)),
        Err(MmMobilityError::IdentityMismatch)
    ));
}

#[test]
// Was: Diese Funktion leitet registration exports the existing Kontext.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn forward_registration_exports_the_existing_context() {
    let now = TdmaTime::default();
    let mut runtime = MmMobilityRuntime::new();
    let mut request = demand(LocationUpdateType::ServiceRestorationRoamingLocationUpdating);
    request.la_information = Some(11);
    let subscriber = TetraAddress::issi(2_260_575);
    let result = runtime
        .begin_forward_registration(subscriber, Some(3), &request, context(subscriber.ssi), now)
        .unwrap();
    assert_eq!(result.target_location_area, 11);
    runtime.accept_forward_registration(subscriber.ssi, now.add_timeslots(1)).unwrap();
    assert_eq!(runtime.take_forward_context(subscriber.ssi).unwrap().groups, vec![100, 101]);
}

#[test]
// Was: Führt den Arbeitsschritt `pending_migration_has_a_bounded_timeout` für pending migration has a bounded timeout aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn pending_migration_has_a_bounded_timeout() {
    let now = TdmaTime::default();
    let mut runtime = MmMobilityRuntime::new();
    let mut first = demand(LocationUpdateType::MigratingLocationUpdating);
    first.address_extension = Some(0x0400_01);
    runtime
        .begin_migration(TetraAddress::issi(0x123456), 9, &first, now, |_| false)
        .unwrap();
    let timeouts = runtime.tick(now.add_timeslots(MM_MOBILITY_TIMEOUT_SLOTS));
    assert!(matches!(timeouts.as_slice(), [MmMobilityTimeout::Migration { handle: 9, .. }]));
}

#[test]
// Was: Führt den Arbeitsschritt `terminal_migration_history_is_bounded_and_vassi_can_be_reused` für terminal migration history is bounded and vassi und weitere Angaben aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn terminal_migration_history_is_bounded_and_vassi_can_be_reused() {
    use tetra_entities::mm::mobility_runtime::MM_MOBILITY_HISTORY_SLOTS;

    let now = TdmaTime::default();
    let mut runtime = MmMobilityRuntime::new();
    let subscriber = TetraAddress::issi(0x123456);
    let mut first = demand(LocationUpdateType::MigratingLocationUpdating);
    first.ssi = Some(2_260_575);
    first.address_extension = Some(0x0400_01);
    let (first_vassi, _) = runtime
        .begin_migration(subscriber, 1, &first, now, |_| false)
        .unwrap();
    let mut second = demand(LocationUpdateType::DemandLocationUpdating);
    second.ssi = Some(2_260_575);
    second.address_extension = Some(0x0400_01);
    runtime
        .complete_migration(first_vassi, &second, now.add_timeslots(1))
        .unwrap();

    runtime.tick(now.add_timeslots(1 + MM_MOBILITY_HISTORY_SLOTS));
    assert!(runtime.snapshot(now.add_timeslots(1 + MM_MOBILITY_HISTORY_SLOTS)).migrations.is_empty());
    assert_eq!(
        runtime.home_issi_for_local(first_vassi),
        Some(2_260_575),
        "the admission-policy identity mapping must outlive bounded transaction history"
    );
    runtime.forget_local_identity(first_vassi);
    assert_eq!(runtime.home_issi_for_local(first_vassi), None);

    let (second_vassi, _) = runtime
        .begin_migration(
            subscriber,
            2,
            &first,
            now.add_timeslots(2 + MM_MOBILITY_HISTORY_SLOTS),
            |_| false,
        )
        .unwrap();
    assert_ne!(second_vassi, 0);
}
