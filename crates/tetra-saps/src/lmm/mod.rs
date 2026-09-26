// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Nachrichten an den Schnittstellen zwischen TETRA-Protokollschichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

// Clause 17.3.2 Service primitives for the LMM-SAP
#![allow(unused)]
use tetra_core::{BitBuffer, Layer2Service, MleHandle, TetraAddress, Todo, TxReporter};

/// This shall be used as a request to initiate the selection of a cell for communications. The
/// request shall always be made after power on and may be made at any time thereafter.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung activate req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleActivateReq {
    pub mcc_list: Vec<u16>,
    pub mnc_list: Vec<u16>,
    pub la_list: Vec<u16>,
    pub cell_type_prefs: Option<Todo>,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung activate ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleActivateInd {
    pub cell_availability: Todo,
}

/// This shall be used as a confirmation to the MM entity that a cell has been selected with the
/// required characteristics.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung activate conf in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleActivateConf {
    pub registration_required: bool,
    pub la: u16,
    pub cell_type: Todo,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung activity req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleActivityReq {
    pub sleep_mode: Todo,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung busy req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleBusyReq {}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung cancel req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleCancelReq {
    pub handle: Todo,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung close req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleCloseReq {}
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung configure req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleConfigureReq {
    pub periodic_reporting_timer: Todo,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung configure ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleConfigureInd {
    pub periodic_reporting_timer: Todo,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung deactivate req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleDeactivateReq {}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung disable req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleDisableReq {
    pub permitted_services_in_temp_disabled_mode: Todo,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung enable req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleEnableReq {}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung identities req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleIdentitiesReq {
    pub issi: Todo,
    pub assi: Todo,
    pub attached_gssis: Vec<Todo>,
    pub detached_gssis: Vec<Todo>,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung idle req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleIdleReq {}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung info req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleInfoReq {
    pub subscriber_class: Todo,
    pub scch_config: Todo,
    pub energy_economy_config: Todo,
    pub minimal_mode_config: Todo,
    pub dual_watch_config: Todo,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung info ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleInfoInd {
    pub broadcast_params: Todo,
    pub subscriber_class_match: Todo,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung link req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleLinkReq {
    pub mcc: Todo,
    pub mnc: Todo,
    pub la_list: Vec<u16>,
    pub cell_type_prefs: Option<Todo>,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung link ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleLinkInd {
    pub mcc: Todo,
    pub mnc: Todo,
    pub la: u16,
    pub registration_type: Todo,
    pub security_params: Todo,
    pub cell_type: Todo,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung open in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleOpen {}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung prepare req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMlePrepareReq {
    pub sdu: Todo,
    pub handle: Todo,
    pub layer2service: Layer2Service,
    pub pdu_prio: Todo,
    pub stealing_permission: bool,
    pub stealing_repeats_flag: bool,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung prepare confirm in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMlePrepareConfirm {
    pub sdu: Todo,
    pub handle: Todo,
}

/// Infrastructure-side indication emitted by MLE when a U-PREPARE PDU carries
/// an embedded MM forward-registration request.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung prepare ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMlePrepareInd {
    pub sdu: BitBuffer,
    pub subscriber: TetraAddress,
    pub endpoint_id: u32,
    pub link_id: u32,
    pub cell_identifier_ca: Option<u8>,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung report ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleReportInd {
    pub handle: MleHandle,
    pub transfer_result: Todo,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung unitdata req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleUnitdataReq {
    pub sdu: BitBuffer,
    pub handle: MleHandle,
    // pub address_type: Todo,
    pub address: TetraAddress,
    pub layer2service: Layer2Service,
    // pub pdu_prio: Todo, // Optional feature
    pub stealing_permission: bool,
    pub stealing_repeats_flag: bool,
    pub encryption_flag: bool,
    pub is_null_pdu: bool, // Prio should be lowest and may not steal
    pub tx_reporter: Option<TxReporter>,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung unitdata ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleUnitdataInd {
    pub sdu: BitBuffer,
    pub handle: MleHandle,
    pub received_address: TetraAddress,
    // pub received_address_type: Todo,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für lmm MLE-Verbindungssteuerung update req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LmmMleUpdateReq {
    pub mcc: Todo,
    pub mnc: Todo,
    pub ra: Todo,
    pub cell_type_prefs: Option<Todo>,
    pub registration_result: Todo,
}
