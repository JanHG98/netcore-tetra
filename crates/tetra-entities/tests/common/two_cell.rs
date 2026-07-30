// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use tetra_config::bluestation::StackMode;
use tetra_core::tetra_entities::TetraEntity;
use tetra_core::{BitBuffer, Sap, TdmaTime, TetraAddress};
use tetra_entities::mle::cell_change_runtime::MleCellChangeRuntimeSnapshot;
use tetra_entities::mle::ltpd_runtime::LtpdRuntimeSnapshot;
use tetra_entities::mle::mle_bs::MleBs;
use tetra_pdus::mle::enums::mle_protocol_discriminator::MleProtocolDiscriminator;
use tetra_pdus::mle::pdus::u_channel_request::UChannelRequest;
use tetra_pdus::mle::pdus::u_prepare::UPrepare;
use tetra_pdus::mle::pdus::u_restore::URestore;
use tetra_saps::common::{
    LowerLayerResourceAvailability, LowerLayerResourceReason, PduPriority,
};
use tetra_saps::control::mle_cell_change::MleCellChangeControl;
use tetra_saps::ltpd::LtpdMleDisconnectReq;
use tetra_saps::tla::TlaTlDataIndBl;
use tetra_saps::tlmc::TlmcConfigureInd;
use tetra_saps::{SapMsg, SapMsgInner};

use super::ComponentTest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für test cell auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum TestCell {
    A,
    B,
}

/// First reusable two-cell harness for the SWMI roadmap.
///
/// It deliberately stays below real RF and D-NEW-CELL signalling. Each cell has
/// an independent MLE/TLPD runtime and independent message router. Later phases
/// can extend this harness with mobility-core coordination and real restore PDUs
/// without replacing the basic test topology.
// Was: Bündelt die zusammengehörigen Werte für two cell harness in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TwoCellHarness {
    pub cell_a: ComponentTest,
    pub cell_b: ComponentTest,
}

// Was: Implementiert das zugehörige Verhalten für `TwoCellHarness`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl TwoCellHarness {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new() -> Self {
        let mut config_a = ComponentTest::get_default_test_config(StackMode::Bs);
        config_a.cell.main_carrier = 1521;
        config_a.cell.location_area = 10;
        config_a.cell.colour_code = 1;
        config_a.cell.sndcp_service = true;

        let mut config_b = config_a.clone();
        config_b.cell.main_carrier = 1522;
        config_b.cell.location_area = 11;
        config_b.cell.colour_code = 2;

        let mut cell_a = ComponentTest::from_config(config_a, Some(TdmaTime::default()));
        let mut cell_b = ComponentTest::from_config(config_b, Some(TdmaTime::default()));
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for cell in [&mut cell_a, &mut cell_b] {
            cell.populate_entities(
                vec![TetraEntity::Mle],
                vec![TetraEntity::Sndcp, TetraEntity::Llc],
            );
        }

        Self { cell_a, cell_b }
    }

    // Was: Führt den Arbeitsschritt `cell` für cell aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn cell(&self, cell: TestCell) -> &ComponentTest {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match cell {
            TestCell::A => &self.cell_a,
            TestCell::B => &self.cell_b,
        }
    }

    // Was: Führt den Arbeitsschritt `cell_mut` für cell mut aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn cell_mut(&mut self, cell: TestCell) -> &mut ComponentTest {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match cell {
            TestCell::A => &mut self.cell_a,
            TestCell::B => &mut self.cell_b,
        }
    }

    // Was: Diese Funktion startet den vorgesehenen Arbeitsschritt.
    // Warum: Der Dienst oder Teilprozess wird so in einer festen und überprüfbaren Reihenfolge gestartet.
    pub fn start(&mut self) {
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for cell in [&mut self.cell_a, &mut self.cell_b] {
            cell.router.tick_start();
            cell.deliver_all_messages();
        }
    }

    // Was: Führt den Arbeitsschritt `learn_route` für learn Weiterleitung aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn learn_route(
        &mut self,
        cell: TestCell,
        address: TetraAddress,
        endpoint_id: u32,
        link_id: u32,
    ) {
        let mut sdu = BitBuffer::new(11);
        sdu.write_bits(0b100, 3);
        sdu.write_bits(0x21, 8);
        sdu.seek(0);
        self.cell_mut(cell).submit_message(SapMsg {
            sap: Sap::TlaSap,
            src: TetraEntity::Llc,
            dest: TetraEntity::Mle,
            msg: SapMsgInner::TlaTlDataIndBl(TlaTlDataIndBl {
                main_address: address,
                link_id,
                endpoint_id,
                new_endpoint_id: None,
                css_endpoint_id: None,
                tl_sdu: Some(sdu),
                scrambling_code: 0,
                fcs_flag: false,
                air_interface_encryption: 0,
                chan_change_resp_req: false,
                chan_change_handle: None,
                chan_info: None,
                req_handle: 0,
            }),
        });
        self.cell_mut(cell).deliver_all_messages();
    }

    // Was: Diese Funktion trennt Weiterleitung.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn disconnect_route(&mut self, cell: TestCell, endpoint_id: u32, link_id: u32) {
        self.cell_mut(cell).submit_message(SapMsg {
            sap: Sap::TlpdSap,
            src: TetraEntity::Sndcp,
            dest: TetraEntity::Mle,
            msg: SapMsgInner::LtpdMleDisconnectReq(LtpdMleDisconnectReq {
                endpoint_id,
                link_id,
                pdu_priority: PduPriority::default(),
                encryption_flag: false,
                report: tetra_saps::common::Layer2Report::LocalDisconnection,
            }),
        });
        self.cell_mut(cell).deliver_all_messages();
    }

    // Was: Führt den Arbeitsschritt `transfer_route` für transfer Weiterleitung aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn transfer_route(
        &mut self,
        source: TestCell,
        target: TestCell,
        address: TetraAddress,
        old_endpoint_id: u32,
        old_link_id: u32,
        new_endpoint_id: u32,
        new_link_id: u32,
    ) {
        self.disconnect_route(source, old_endpoint_id, old_link_id);
        self.learn_route(target, address, new_endpoint_id, new_link_id);
    }

    // Was: Diese Funktion setzt resources available.
    // Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
    pub fn set_resources_available(&mut self, cell: TestCell, available: bool) {
        self.cell_mut(cell).submit_message(SapMsg {
            sap: Sap::TlmcSap,
            src: TetraEntity::Umac,
            dest: TetraEntity::Mle,
            msg: SapMsgInner::TlmcConfigureInd(TlmcConfigureInd {
                endpoint_id: 0,
                lower_layer_resource_availability: if available {
                    LowerLayerResourceAvailability::Available
                } else {
                    LowerLayerResourceAvailability::Unavailable
                },
                reason: if available {
                    LowerLayerResourceReason::RecoveryOfRadioResources
                } else {
                    LowerLayerResourceReason::LossOfRadioResources
                },
            }),
        });
        self.cell_mut(cell).deliver_all_messages();
    }

    // Was: Führt den Arbeitsschritt `submit_mle_body` für submit MLE-Verbindungssteuerung body aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn submit_mle_body(
        &mut self,
        cell: TestCell,
        address: TetraAddress,
        endpoint_id: u32,
        link_id: u32,
        mut body: BitBuffer,
    ) {
        body.seek(0);
        let body_len = body.get_len();
        let mut sdu = BitBuffer::new(3 + body_len);
        sdu.write_bits(MleProtocolDiscriminator::Mle.into_raw(), 3);
        sdu.copy_bits(&mut body, body_len);
        sdu.seek(0);
        self.cell_mut(cell).submit_message(SapMsg {
            sap: Sap::TlaSap,
            src: TetraEntity::Llc,
            dest: TetraEntity::Mle,
            msg: SapMsgInner::TlaTlDataIndBl(TlaTlDataIndBl {
                main_address: address,
                link_id,
                endpoint_id,
                new_endpoint_id: None,
                css_endpoint_id: None,
                tl_sdu: Some(sdu),
                scrambling_code: 0,
                fcs_flag: false,
                air_interface_encryption: 0,
                chan_change_resp_req: false,
                chan_change_handle: None,
                chan_info: None,
                req_handle: 0,
            }),
        });
        self.cell_mut(cell).deliver_all_messages();
    }

    // Was: Führt den Arbeitsschritt `submit_u_prepare` für submit u prepare aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn submit_u_prepare(
        &mut self,
        cell: TestCell,
        address: TetraAddress,
        endpoint_id: u32,
        link_id: u32,
        pdu: UPrepare,
    ) {
        let mut body = BitBuffer::new_autoexpand(128);
        pdu.to_bitbuf(&mut body).expect("encode U-PREPARE");
        self.submit_mle_body(cell, address, endpoint_id, link_id, body);
    }

    // Was: Führt den Arbeitsschritt `submit_u_restore` für submit u Wiederherstellung aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn submit_u_restore(
        &mut self,
        cell: TestCell,
        address: TetraAddress,
        endpoint_id: u32,
        link_id: u32,
        pdu: URestore,
    ) {
        let mut body = BitBuffer::new_autoexpand(128);
        pdu.to_bitbuf(&mut body).expect("encode U-RESTORE");
        self.submit_mle_body(cell, address, endpoint_id, link_id, body);
    }

    // Was: Führt den Arbeitsschritt `submit_u_channel_request` für submit u Kanal request aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn submit_u_channel_request(
        &mut self,
        cell: TestCell,
        address: TetraAddress,
        endpoint_id: u32,
        link_id: u32,
        pdu: UChannelRequest,
    ) {
        let mut body = BitBuffer::new_autoexpand(96);
        pdu.to_bitbuf(&mut body).expect("encode U-CHANNEL-REQUEST");
        self.submit_mle_body(cell, address, endpoint_id, link_id, body);
    }

    // Was: Führt den Arbeitsschritt `control_cell_change` für Steuerung cell change aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn control_cell_change(&mut self, cell: TestCell, control: MleCellChangeControl) {
        self.cell_mut(cell).submit_message(SapMsg {
            sap: Sap::Control,
            src: TetraEntity::Mm,
            dest: TetraEntity::Mle,
            msg: SapMsgInner::MleCellChangeControl(control),
        });
        self.cell_mut(cell).deliver_all_messages();
    }

    // Was: Führt den Arbeitsschritt `cell_change_snapshot` für cell change snapshot aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn cell_change_snapshot(&mut self, cell: TestCell) -> MleCellChangeRuntimeSnapshot {
        let component = self
            .cell_mut(cell)
            .router
            .get_entity(TetraEntity::Mle)
            .expect("MLE missing from two-cell harness");
        component
            .as_any_mut()
            .downcast_mut::<MleBs>()
            .expect("MLE-BS downcast failed")
            .cell_change_snapshot()
    }

    // Was: Führt den Arbeitsschritt `ltpd_snapshot` für TETRA-Paketdatentransport snapshot aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn ltpd_snapshot(&mut self, cell: TestCell) -> LtpdRuntimeSnapshot {
        let component = self
            .cell_mut(cell)
            .router
            .get_entity(TetraEntity::Mle)
            .expect("MLE missing from two-cell harness");
        component
            .as_any_mut()
            .downcast_mut::<MleBs>()
            .expect("MLE-BS downcast failed")
            .ltpd_snapshot()
    }

    // Was: Diese Funktion arbeitet den vorgesehenen Arbeitsschritt.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn drain(&mut self, cell: TestCell) -> Vec<SapMsg> {
        self.cell_mut(cell).dump_sinks()
    }
}

// Was: Implementiert das zugehörige Verhalten für `Default for TwoCellHarness`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for TwoCellHarness {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self::new()
    }
}
