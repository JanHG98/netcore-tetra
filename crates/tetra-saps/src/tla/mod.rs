#![allow(unused)]
// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Nachrichten an den Schnittstellen zwischen TETRA-Protokollschichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use tetra_core::{BitBuffer, EndpointId, LinkId, TetraAddress, Todo, TxReporter};

use crate::lcmc::fields::chan_alloc_req::CmceChanAllocReq;

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl cancel req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlCancelReq {
    pub handle: Todo,
}

/// advanced link
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl connect req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlConnectReq {
    // address_type: Todo,
    main_address: Todo,
    scrambling_code: Todo,
    link_id: LinkId,
    endpoint_id: EndpointId,
    pdu_prio: Todo,
    stealing_permission: bool,
    subscriber_class: Todo,
    qos: Todo,
    al_service: Todo,
    air_interface_encryption: Todo,
    req_handle: Todo,
    setup_report: Todo,
}
/// advanced link
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl connect ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlConnectInd {
    // address_type: Todo,
    main_address: Todo,
    scrambling_code: Todo,
    link_id: LinkId,
    endpoint_id: EndpointId,
    new_endpoint_id: Option<Todo>,
    css_endpoint_id: Option<Todo>,
    qos: Todo,
    al_service: Todo,
    air_interface_encryption: Todo,
    chan_change_resp_req: bool,
    chan_change_handle: Option<Todo>,
    chan_info: Option<Todo>,
    req_handle: Todo,
    setup_report: Todo,
}
/// advanced link
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl connect resp in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlConnectResp {
    // address_type: Todo,
    main_address: Todo,
    scrambling_code: Todo,
    link_id: LinkId,
    endpoint_id: EndpointId,
    pdu_prio: Todo,
    stealing_permission: bool,
    subscriber_class: Todo,
    qos: Todo,
    al_service: Todo,
    air_interface_encryption: Todo,
    req_handle: Todo,
    setup_report: Todo,
}
/// advanced link
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl connect conf in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlConnectConf {
    // address_type: Todo,
    main_address: Todo,
    scrambling_code: Todo,
    link_id: LinkId,
    endpoint_id: EndpointId,
    new_endpoint_id: Option<Todo>,
    css_endpoint_id: Option<Todo>,
    qos: Todo,
    al_service: Todo,
    air_interface_encryption: Todo,
    chan_change_resp_req: bool,
    chan_change_handle: Option<Todo>,
    chan_info: Option<Todo>,
    req_handle: Todo,
    setup_report: Todo,
}

/// advanced link only
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl data req al in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlDataReqAl;
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl data ind al in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlDataIndAl;
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl data conf al in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlDataConfAl;

/// Clause 20.3.5.1.4
/// TL-DATA request: this primitive shall be used by the layer 2 service user to request transmission of a TL-SDU. The
// TL-SDU will be acknowledged by the peer entity.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tla tl data req bl in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlaTlDataReqBl {
    // pub address_type: Todo,
    pub main_address: TetraAddress,
    pub link_id: LinkId,
    pub endpoint_id: EndpointId,
    pub tl_sdu: BitBuffer,
    // pub scrambling_code: u32, // TODO FIXME: according to the spec, should be there, but why do we need to provide this?
    // pub pdu_prio: Todo, // Optional feature
    pub stealing_permission: bool,
    pub subscriber_class: Todo,
    pub fcs_flag: bool,
    pub air_interface_encryption: Option<Todo>,
    pub stealing_repeats_flag: Option<bool>,
    pub data_class_info: Option<Todo>,
    pub req_handle: Todo,
    pub graceful_degradation: Option<Todo>,

    // Custom fields for BS stack:
    /// Optional Channel Allocation Request that may be included by CMCE
    pub chan_alloc: Option<CmceChanAllocReq>,

    /// Optional TxReporter that may be included to track transmission and optionally, acknowledgement
    pub tx_reporter: Option<TxReporter>,
}

/// Clause 20.3.5.1.4
/// TL-DATA indication: this primitive shall be used by the layer 2 to deliver the received TL-SDU to the layer 2 service
// user.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tla tl data ind bl in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlaTlDataIndBl {
    // pub address_type: Todo,
    pub main_address: TetraAddress,
    pub link_id: LinkId,
    pub endpoint_id: EndpointId,
    pub new_endpoint_id: Option<EndpointId>,
    pub css_endpoint_id: Option<EndpointId>,
    pub tl_sdu: Option<BitBuffer>,
    pub scrambling_code: u32,
    pub fcs_flag: bool,
    pub air_interface_encryption: Todo,
    pub chan_change_resp_req: bool,
    pub chan_change_handle: Option<Todo>,
    pub chan_info: Option<Todo>,
    pub req_handle: Todo,
}

/// Clause 20.3.5.1.4
/// TL-DATA response: this primitive shall be used by the layer 2 service user to respond to the previous TL-DATA
// indication primitive. The TL-DATA response primitive may contain a TL-SDU. That TL-SDU will be sent without an
// explicit acknowledgement from the peer entity.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl data resp bl in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlDataRespBl {
    // pub address_type: Todo,
    pub main_address: TetraAddress,
    pub link_id: LinkId,
    pub endpoint_id: EndpointId,
    pub tl_sdu: BitBuffer,
    pub scrambling_code: Todo,
    pub pdu_prio: Todo,
    pub stealing_permission: bool,
    pub subscriber_class: Todo,
    pub fcs_flag: bool,
    pub air_interface_encryption: Todo,
    pub stealing_repeats_flag: Option<bool>,
    pub data_class_info: Option<Todo>,
    pub req_handle: Todo,
}

/// Clause 20.3.5.1.4
// TL-DATA confirm: this primitive shall be used by the layer 2 to inform the layer 2 service user that it has completed
// successfully the transmission of the requested TL-SDU. Depending on the availability of the response primitive at the
// peer entity before transmission of the acknowledgement, the confirm primitive may or may not carry a TL-SDU.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl data conf bl in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlDataConfBl {
    // pub address_type: Todo,
    pub main_address: TetraAddress,
    pub link_id: LinkId,
    pub endpoint_id: EndpointId,
    pub new_endpoint_id: Option<Todo>,
    pub css_endpoint_id: Option<Todo>,
    pub tl_sdu: Option<BitBuffer>,
    pub scrambling_code: Todo,
    pub fcs_flag: bool,
    pub air_interface_encryption: Todo,
    pub chan_change_resp_req: bool,
    pub chan_change_handle: Option<Todo>,
    pub chan_info: Option<Todo>,
    pub req_handle: Todo,
    pub report: Todo,
}

/// Advanced link only
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl disconnect req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlDisconnectReq;
/// Advanced link only
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl disconnect ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlDisconnectInd;
/// Advanced link only
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl disconnect conf in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlDisconnectConf;

/// advanced link, BS only
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl receive ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlReceiveInd;

// advanced link
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl release req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlReleaseReq {
    // pub address_type: Todo,
    pub main_address: TetraAddress,
    pub link_id: LinkId,
}
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl release ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlReleaseInd {
    // pub address_type: Todo,
    pub main_address: TetraAddress,
    pub link_id: Option<Todo>,
    pub endpoint_id: EndpointId,
}

/// advanced link
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl reconnect req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlReconnectReq;
/// advanced link
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl reconnect resp in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlReconnectResp;

// pub enum TlaReport {
//     /// Confirm handle to the request
//     ConfirmHandle,

// }

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tla tl report ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlaTlReportInd {
    pub req_handle: Option<Todo>,
    pub report: Todo,
    pub chan_change_resp_req: Option<bool>,
    pub chan_change_handle: Option<Todo>,
    pub chan_info: Option<Todo>,
    pub endpoint_id: Option<Todo>,
}

/// Clause 20.3.5.1.9
/// TL-UNITDATA request: this primitive shall be used in the unacknowledged data transfer service by the layer 2
/// service user to request layer 2 to transmit a TL-SDU.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tla tl unitdata req bl in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlaTlUnitdataReqBl {
    // pub address_type: Todo,
    pub main_address: TetraAddress,
    pub link_id: LinkId,
    pub endpoint_id: EndpointId,
    pub tl_sdu: BitBuffer,
    // pub scrambling_code: Todo, // TODO FIXME reintroduce in MLE for sysinfo/sync
    // pub pdu_prio: Todo,
    pub stealing_permission: bool,
    pub subscriber_class: Todo,
    pub fcs_flag: bool,
    pub air_interface_encryption: Option<Todo>,
    // pub data_prio: Todo,
    pub packet_data_flag: bool,
    pub n_tlsdu_repeats: u8, // TODO check data type and purpose
    // pub scheduled_data_status: Todo,
    // pub max_schedule_interval: Option<Todo>,
    pub data_class_info: Option<Todo>,
    pub req_handle: Todo,

    // Custom fields for BS stack:
    /// Optional Channel Allocation Request that may be included by CMCE
    pub chan_alloc: Option<CmceChanAllocReq>,

    /// Optional TxReporter that may be included to track transmission and optionally, acknowledgement
    pub tx_reporter: Option<TxReporter>,
}

/// Clause 20.3.5.1.9
/// TL-UNITDATA indication: this primitive shall be used in the unacknowledged data transfer service to deliver
/// the received TL-SDU to the layer 2 service user.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tla tl unitdata ind bl in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlaTlUnitdataIndBl {
    // pub address_type: Todo,
    pub main_address: TetraAddress,
    pub link_id: LinkId,
    pub endpoint_id: EndpointId,
    pub new_endpoint_id: Option<EndpointId>,
    pub css_endpoint_id: Option<EndpointId>,
    pub tl_sdu: Option<BitBuffer>,
    pub scrambling_code: u32,
    pub fcs_flag: bool,
    pub air_interface_encryption: Todo,
    pub chan_change_resp_req: bool,
    pub chan_change_handle: Option<Todo>,
    pub chan_info: Option<Todo>,
    pub report: Option<Todo>,
}

/// Clause 20.3.5.1.9, optional
/// TL-UNITDATA confirm: this primitive may be used in the unacknowledged data transfer service to indicate
/// completion of sending of the requested TL-SDU.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl unitdata conf bl in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlUnitdataConfBl {
    // pub address_type: Todo,
    pub main_address: TetraAddress,
    pub link_id: LinkId,
    pub endpoint_id: EndpointId,
    pub req_handle: Todo,
    pub report: Option<Todo>,
}

/// Advanced link
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl unitdata req al in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlUnitdataReqAl;
/// Advanced link
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl unitdata ind al in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlUnitdataIndAl;
/// Advanced link, optional?
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tl unitdata conf al in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlUnitdataConfAl;
