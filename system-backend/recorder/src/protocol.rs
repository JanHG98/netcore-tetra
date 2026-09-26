// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Aufzeichnung und Ereignisprotokollierung.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Aufzeichnung tap batch in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RecorderTapBatch {
    pub requested_after: u64,
    #[serde(default)]
    pub oldest_available_seq: Option<u64>,
    #[serde(default)]
    pub newest_available_seq: Option<u64>,
    #[serde(default)]
    pub dropped_before: u64,
    #[serde(default)]
    pub records: Vec<RecorderTapRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Aufzeichnung tap Datensatz in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RecorderTapRecord {
    pub seq: u64,
    pub timestamp: String,
    pub session_id: String,
    pub call_kind: String,
    pub call_phase: String,
    #[serde(default)]
    pub source_issi: Option<u32>,
    #[serde(default)]
    pub gssi: Option<u32>,
    #[serde(default)]
    pub calling_issi: Option<u32>,
    #[serde(default)]
    pub called_issi: Option<u32>,
    #[serde(default)]
    pub priority: u8,
    #[serde(default)]
    pub emergency: bool,
    #[serde(default)]
    pub speaker_issi: Option<u32>,
    pub source_node_id: String,
    pub source_logical_ts: u8,
    pub source_sequence: u64,
    #[serde(default)]
    pub target_count: usize,
    pub codec: String,
    pub payload: Vec<u8>,
    #[serde(default)]
    pub injected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Audio- und Mediendaten switch Sitzung in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MediaSwitchSession {
    pub logical_call_id: String,
    pub kind: String,
    pub phase: String,
    #[serde(default)]
    pub source_issi: Option<u32>,
    #[serde(default)]
    pub gssi: Option<u32>,
    #[serde(default)]
    pub calling_issi: Option<u32>,
    #[serde(default)]
    pub called_issi: Option<u32>,
    #[serde(default)]
    pub priority: u8,
    #[serde(default)]
    pub emergency: bool,
    #[serde(default)]
    pub floor_holder: Option<u32>,
}
