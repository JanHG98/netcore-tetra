// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Nachrichten an den Schnittstellen zwischen TETRA-Protokollschichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! LTPD-SAP primitives between MLE and SNDCP.
//!
//! This module defines the typed local service interface used by the Package D
//! runtime in MLE and SNDCP. Timing-sensitive transport remains inside the TBS
//! process; only read-only diagnostics are intended to cross the management boundary.

use tetra_core::{BitBuffer, EndpointId, Layer2Service, LinkId, TetraAddress};

use crate::lcmc::fields::chan_alloc_req::CmceChanAllocReq;

use crate::common::{
    CallReleaseInstruction, ChannelAdvice, ChannelChangeDecision, ChannelChangeHandle, DataClass, DataPriority,
    DataPriorityRandomAccessDelayFactor, Layer2Qos, Layer2Report, LowerLayerResourceReason, MleBroadcastParameters,
    Nsapi, PduPriority, PermittedCellInformation, PermittedTemporaryServices, ReceivedAddressType,
    ReconnectionResult, RequestHandle, ReservationInfo, ScheduleRepetitionInformation, ScheduledDataStatus,
    SetupReport, SleepMode, SndcpStatus, StealingPermission, TransferResult,
};

/// SNDCP informs MLE whether the MS must remain awake.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung activity req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleActivityReq {
    pub sleep_mode: SleepMode,
}

/// Communication resources are temporarily unavailable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung break ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleBreakInd;

/// MM is using the signalling resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung busy ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleBusyInd;

/// Cancel a request that has not yet been transmitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung cancel req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleCancelReq {
    pub handle: RequestHandle,
}

/// Network access has been removed from SNDCP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung close ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleCloseInd;

/// Inter-layer packet-data configuration supplied by SNDCP.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung configure req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleConfigureReq {
    pub channel_change_accepted: Option<ChannelChangeDecision>,
    pub channel_change_handle: Option<ChannelChangeHandle>,
    pub call_release: CallReleaseInstruction,
    pub endpoint_id: EndpointId,
    pub encryption_flag: bool,
    pub ms_default_data_priority: DataPriority,
    pub layer_2_data_priority_lifetime: Option<std::time::Duration>,
    pub layer_2_data_priority_signalling_delay: Option<std::time::Duration>,
    pub data_priority_random_access_delay_factor: Option<DataPriorityRandomAccessDelayFactor>,
    pub data_class_information: DataClass,
    pub schedule_repetition_information: Option<ScheduleRepetitionInformation>,
    pub sndcp_status: SndcpStatus,
}

/// Packet-data/circuit-resource conflict reported by MLE.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung configure ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleConfigureInd {
    pub endpoint_id: EndpointId,
    pub channel_change_response_required: bool,
    pub channel_change_handle: Option<ChannelChangeHandle>,
    pub reason: LowerLayerResourceReason,
    pub conflicting_endpoint_id: Option<EndpointId>,
}

/// Request setup or reset of an advanced link.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung connect req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleConnectReq {
    pub address: TetraAddress,
    pub endpoint_id: EndpointId,
    pub link_id: LinkId,
    pub reservation_information: ReservationInfo,
    pub pdu_priority: PduPriority,
    pub layer_2_qos: Layer2Qos,
    pub encryption_flag: bool,
    pub setup_report: SetupReport,
}

/// Peer requested setup or reset of an advanced link.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung connect ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleConnectInd {
    pub address: TetraAddress,
    pub endpoint_id: EndpointId,
    pub new_endpoint_id: EndpointId,
    pub link_id: LinkId,
    pub layer_2_qos: Layer2Qos,
    pub encryption_flag: bool,
    pub channel_change_response_required: bool,
    pub channel_change_handle: Option<ChannelChangeHandle>,
    pub setup_report: SetupReport,
}

/// Accept or modify an incoming advanced-link setup.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung connect resp in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleConnectResp {
    pub address: TetraAddress,
    pub endpoint_id: EndpointId,
    pub link_id: LinkId,
    pub pdu_priority: PduPriority,
    pub stealing_permission: StealingPermission,
    pub layer_2_qos: Layer2Qos,
    pub encryption_flag: bool,
    pub setup_report: SetupReport,
}

/// Completion of an advanced-link setup or reset.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung connect confirm in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleConnectConfirm {
    pub address: TetraAddress,
    pub endpoint_id: EndpointId,
    pub link_id: LinkId,
    pub layer_2_qos: Layer2Qos,
    pub encryption_flag: bool,
    pub channel_change_response_required: bool,
    pub channel_change_handle: Option<ChannelChangeHandle>,
    pub setup_report: SetupReport,
}

/// Enter the temporarily disabled state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung disable ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleDisableInd {
    pub permitted_services: PermittedTemporaryServices,
}

/// Request disconnection of an advanced link.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung disconnect req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleDisconnectReq {
    pub endpoint_id: EndpointId,
    pub link_id: LinkId,
    pub pdu_priority: PduPriority,
    pub encryption_flag: bool,
    pub report: Layer2Report,
}

/// Advanced link was disconnected.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung disconnect ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleDisconnectInd {
    pub endpoint_id: EndpointId,
    pub new_endpoint_id: Option<EndpointId>,
    pub link_id: LinkId,
    pub encryption_flag: bool,
    pub channel_change_response_required: bool,
    pub channel_change_handle: Option<ChannelChangeHandle>,
    pub report: Layer2Report,
}

/// Recover from the temporarily disabled state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung enable ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleEnableInd;

/// Broadcast and serving-cell information relevant to SNDCP.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung info ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleInfoInd {
    pub broadcast_parameters: MleBroadcastParameters,
    pub subscriber_class_match: bool,
    pub schedule_timing_prompt: Option<Nsapi>,
    pub permitted_cell_information: PermittedCellInformation,
}

/// MM signalling exchange has completed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung idle ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleIdleInd;

/// SNDCP may use network communication resources.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung open ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleOpenInd {
    pub mcc: u16,
    pub mnc: u16,
}

/// SwMI-only indication that LLC reception is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung receive ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleReceiveInd {
    pub endpoint_id: EndpointId,
    pub received_tetra_address: TetraAddress,
    pub received_address_type: ReceivedAddressType,
}

/// Request advanced-link reconnection after cell reselection.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung reconnect req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleReconnectReq {
    pub endpoint_id: EndpointId,
    pub link_id: LinkId,
    pub reservation_information: ReservationInfo,
    pub pdu_priority: PduPriority,
    pub encryption_flag: bool,
    pub stealing_permission: StealingPermission,
}

/// Completion of an MS-originated reconnection attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung reconnect confirm in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleReconnectConfirm {
    pub endpoint_id: EndpointId,
    pub new_endpoint_id: Option<EndpointId>,
    pub link_id: LinkId,
    pub encryption_flag: bool,
    pub report: Layer2Report,
    pub reconnection_result: ReconnectionResult,
}

/// SwMI indication of an MS reconnection attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung reconnect ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleReconnectInd {
    pub endpoint_id: EndpointId,
    pub new_endpoint_id: Option<EndpointId>,
    pub link_id: LinkId,
    pub encryption_flag: bool,
    pub report: Layer2Report,
    pub reconnection_result: ReconnectionResult,
}

/// Locally release an advanced link.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung release req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleReleaseReq {
    pub link_id: LinkId,
}

/// Completion report for an MLE-UNITDATA request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung report ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleReportInd {
    pub handle: RequestHandle,
    pub transfer_result: TransferResult,
}

/// Communication resources and previous MLE associations are available again.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung resume ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleResumeInd {
    pub mcc: u16,
    pub mnc: u16,
}

/// SNDCP data transfer request to MLE.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung unitdata req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleUnitdataReq {
    pub sdu: BitBuffer,
    pub handle: RequestHandle,
    /// Optional route hint. MLE uses it to rebuild a local context after restart.
    pub address: Option<TetraAddress>,
    pub layer2service: Layer2Service,
    pub unacknowledged_basic_link_repetitions: u8,
    pub pdu_priority: PduPriority,
    pub endpoint_id: EndpointId,
    pub link_id: LinkId,
    pub stealing_permission: StealingPermission,
    pub stealing_repeats_flag: bool,
    pub channel_advice: ChannelAdvice,
    pub data_class_information: DataClass,
    pub data_priority: DataPriority,
    pub mle_data_priority_flag: bool,
    pub packet_data_flag: bool,
    pub scheduled_data_status: ScheduledDataStatus,
    pub maximum_schedule_interval_slots: Option<u32>,
    pub fcs_flag: bool,
    /// Optional dynamic PDCH allocation carried down to LLC/UMAC.
    pub chan_alloc: Option<CmceChanAllocReq>,
}

/// SNDCP data received from a peer entity.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Paketdatentransport MLE-Verbindungssteuerung unitdata ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LtpdMleUnitdataInd {
    pub sdu: BitBuffer,
    pub endpoint_id: EndpointId,
    pub link_id: LinkId,
    pub received_tetra_address: TetraAddress,
    pub received_address_type: ReceivedAddressType,
    pub chan_change_resp_req: bool,
    pub chan_change_handle: Option<ChannelChangeHandle>,
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;
    use crate::common::{AdvancedLinkFormat, ThroughputInformation};
    use tetra_core::SsiType;

    #[test]
    // Was: Führt den Arbeitsschritt `context_primitives_keep_endpoint_and_link_separate` für Kontext primitives keep endpoint and link separate aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn context_primitives_keep_endpoint_and_link_separate() {
        let qos = Layer2Qos {
            throughput: ThroughputInformation {
                bits_per_second: Some(4_800),
                timeslots: Some(1),
            },
            link_format: AdvancedLinkFormat::Original,
            acknowledged_window_size: 4,
            max_tl_sdu_retransmissions: 2,
            max_segment_retransmissions: 3,
        };
        let request = LtpdMleConnectReq {
            address: TetraAddress::new(1001, SsiType::Issi),
            endpoint_id: 5,
            link_id: 9,
            reservation_information: ReservationInfo { octets_available: 256 },
            pdu_priority: PduPriority::new(3).unwrap(),
            layer_2_qos: qos,
            encryption_flag: false,
            setup_report: SetupReport::Success,
        };

        assert_eq!(request.endpoint_id, 5);
        assert_eq!(request.link_id, 9);
        assert!(request.layer_2_qos.validate().is_ok());
    }

    #[test]
    // Was: Führt den Arbeitsschritt `unitdata_indication_contains_explicit_address_type` für unitdata indication contains explicit address type aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn unitdata_indication_contains_explicit_address_type() {
        let address = TetraAddress::new(4711, SsiType::Gssi);
        let indication = LtpdMleUnitdataInd {
            sdu: BitBuffer::new(0),
            endpoint_id: 1,
            link_id: 0,
            received_tetra_address: address,
            received_address_type: ReceivedAddressType::from_tetra_address(address),
            chan_change_resp_req: false,
            chan_change_handle: None,
        };

        assert_eq!(indication.received_address_type, ReceivedAddressType::Group);
    }
}
