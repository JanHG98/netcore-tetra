// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Nachrichten an den Schnittstellen zwischen TETRA-Protokollschichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Pass TMD circuit data to UMAC for TX scheduling
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tmd circuit data req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TmdCircuitDataReq {
    // call_id: CallId,
    pub carrier_num: u16,
    pub ts: u8,
    pub data: Vec<u8>,
}

/// Rx'ed traffic
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tmd circuit data ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TmdCircuitDataInd {
    // call_id: CallId,
    pub carrier_num: u16,
    pub ts: u8,
    pub data: Vec<u8>,
}
