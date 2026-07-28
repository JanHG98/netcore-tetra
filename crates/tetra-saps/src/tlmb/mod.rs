// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Nachrichten an den Schnittstellen zwischen TETRA-Protokollschichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use tetra_core::{BitBuffer, EndpointId, Todo};

/// BS only
/// TL-SAP and TMB-SAP merged into TLMB-SAP
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tlmb sync req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlmbSyncReq {
    pub endpoint_id: EndpointId,
    pub tl_sdu: BitBuffer,
    pub priority: Todo,
}

/// MS only
/// TL-SAP and TMB-SAP merged into TLMB-SAP
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tlmb sync ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlmbSyncInd {
    pub endpoint_id: EndpointId,
    pub tl_sdu: BitBuffer,
}

/// BS only
/// TL-SAP and TMB-SAP merged into TLMB-SAP
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tlmb sysinfo req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlmbSysinfoReq {
    pub endpoint_id: EndpointId,
    pub tl_sdu: BitBuffer,
    pub mac_broadcast_info: Option<Todo>,
    pub priority: Todo,
}

/// MS only
/// TL-SAP and TMB-SAP merged into TLMB-SAP
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tlmb sysinfo ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlmbSysinfoInd {
    pub endpoint_id: EndpointId,
    pub tl_sdu: BitBuffer,
    pub mac_broadcast_info: Option<Todo>,
}
