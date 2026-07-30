// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

// Was: Bindet das Untermodul common in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod common;

use common::{TestCell, TwoCellHarness};
use tetra_core::{BitBuffer, SsiType, TetraAddress};
use tetra_entities::mle::cell_change_runtime::MleCellChangePhase;
use tetra_pdus::mle::enums::mle_protocol_discriminator::MleProtocolDiscriminator;
use tetra_pdus::mle::pdus::d_new_cell::DNewCell;
use tetra_pdus::mle::pdus::d_restore_ack::DRestoreAck;
use tetra_pdus::mle::pdus::u_prepare::UPrepare;
use tetra_pdus::mle::pdus::u_restore::URestore;
use tetra_saps::common::{CellIdentity, CellType, MleChannelCommandValid};
use tetra_saps::control::mle_cell_change::MleCellChangeControl;
use tetra_saps::{SapMsg, SapMsgInner};

// Was: Führt den Arbeitsschritt `address` für address aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn address(issi: u32) -> TetraAddress {
    TetraAddress::new(issi, SsiType::Issi)
}

// Was: Führt den Arbeitsschritt `target_cell_b` für target cell b aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn target_cell_b() -> CellIdentity {
    CellIdentity {
        mcc: 262,
        mnc: 1,
        location_area: Some(11),
        colour_code: Some(2),
        main_carrier: 1522,
        cell_type: CellType::ConventionalAccess,
    }
}

// Was: Führt den Arbeitsschritt `take_mle_downlink` für take MLE-Verbindungssteuerung Downlink (Netz zum Funkgerät) aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn take_mle_downlink(messages: Vec<SapMsg>) -> BitBuffer {
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for message in messages {
        if let SapMsgInner::TlaTlDataReqBl(primitive) = message.msg {
            let mut sdu = primitive.tl_sdu;
            sdu.seek(0);
            assert_eq!(
                sdu.read_bits(3),
                Some(MleProtocolDiscriminator::Mle.into_raw())
            );
            return BitBuffer::from_bitbuffer_pos(&sdu);
        }
    }
    panic!("no MLE downlink found in LLC sink");
}

#[test]
// Was: Diese Funktion bereitet on old cell and Wiederherstellung on target und weitere Angaben.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn prepare_on_old_cell_and_restore_on_target_cell_are_isolated() {
    let mut harness = TwoCellHarness::new();
    let subscriber = address(4101);

    harness.submit_u_prepare(
        TestCell::A,
        subscriber,
        2,
        3,
        UPrepare {
            cell_identifier_ca: Some(17),
            sdu: Some(BitBuffer::from_bitstr("101101")),
        },
    );
    let prepare_snapshot = harness.cell_change_snapshot(TestCell::A);
    assert_eq!(prepare_snapshot.transactions.len(), 1);
    assert_eq!(
        prepare_snapshot.transactions[0].phase,
        MleCellChangePhase::PrepareReceived
    );
    assert!(harness.cell_change_snapshot(TestCell::B).transactions.is_empty());
    let prepare_indications = harness.drain(TestCell::A);
    assert!(prepare_indications.iter().any(|message| matches!(
        &message.msg,
        SapMsgInner::LmmMleUnitdataInd(indication)
            if indication.received_address == subscriber
                && indication.sdu.dump_bin_unformatted() == "101101"
    )));

    harness.control_cell_change(
        TestCell::A,
        MleCellChangeControl::GrantPrepare {
            subscriber,
            command: MleChannelCommandValid::ChangeChannelImmediately,
            target_cell: Some(target_cell_b()),
            mm_sdu: None,
        },
    );
    let mut downlink = take_mle_downlink(harness.drain(TestCell::A));
    let new_cell = DNewCell::from_bitbuf(&mut downlink).unwrap();
    assert_eq!(
        new_cell.channel_command_valid,
        MleChannelCommandValid::ChangeChannelImmediately
    );
    assert_eq!(
        harness.cell_change_snapshot(TestCell::A).transactions[0].phase,
        MleCellChangePhase::NewCellGranted
    );

    harness.submit_u_restore(
        TestCell::B,
        subscriber,
        6,
        7,
        URestore {
            mcc: Some(262),
            mnc: Some(1),
            la: Some(10),
            sdu: Some(BitBuffer::from_bitstr("01101100")),
        },
    );
    let restore_snapshot = harness.cell_change_snapshot(TestCell::B);
    assert_eq!(restore_snapshot.transactions.len(), 1);
    assert_eq!(
        restore_snapshot.transactions[0].phase,
        MleCellChangePhase::RestoreReceived
    );
    assert_eq!(restore_snapshot.transactions[0].old_location_area, Some(10));
    let restore_indications = harness.drain(TestCell::B);
    assert!(restore_indications.iter().any(|message| matches!(
        &message.msg,
        SapMsgInner::LcmcMleRestoreInd(indication)
            if indication.subscriber == subscriber
                && indication.endpoint_id == 6
                && indication.link_id == 7
                && indication.sdu.dump_bin_unformatted() == "01101100"
    )));

    harness.control_cell_change(
        TestCell::B,
        MleCellChangeControl::AcknowledgeRestore {
            subscriber,
            cmce_sdu: BitBuffer::from_bitstr("11100011"),
            chan_alloc: None,
        },
    );
    let mut downlink = take_mle_downlink(harness.drain(TestCell::B));
    let restore_ack = DRestoreAck::from_bitbuf(&mut downlink).unwrap();
    assert_eq!(
        restore_ack.sdu.unwrap().dump_bin_unformatted(),
        "11100011"
    );
    assert_eq!(
        harness.cell_change_snapshot(TestCell::B).transactions[0].phase,
        MleCellChangePhase::Restored
    );

    assert_eq!(
        harness.cell_change_snapshot(TestCell::A).transactions[0].phase,
        MleCellChangePhase::NewCellGranted
    );
}
