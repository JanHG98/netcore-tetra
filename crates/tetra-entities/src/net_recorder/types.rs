// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für recording segment in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RecordingSegment {
    pub source_issi: u32,
    pub timeslot: u8,
    pub carrier_num: u16,
    pub start_ms: u64,
    pub end_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für recording metadata in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RecordingMetadata {
    pub schema_version: u8,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    pub call_id: u16,
    pub source_issi: u32,
    pub destination_id: u32,
    pub destination_type: String,
    pub started_at: String,
    pub ended_at: String,
    pub duration_ms: u64,
    pub audio_bytes: u64,
    pub relative_audio_path: String,
    pub recovered_after_unclean_shutdown: bool,
    pub segments: Vec<RecordingSegment>,
}

#[derive(Debug, Clone, Serialize)]
// Was: Bündelt die zusammengehörigen Werte für Aufzeichnung Status in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RecorderStatus {
    pub available: bool,
    pub active: bool,
    pub directory: String,
    pub mode: String,
    pub selected_groups: Vec<u32>,
    pub minimum_free_space_mb: u64,
    pub free_space_bytes: Option<u64>,
    pub used_bytes: u64,
    pub recording_count: usize,
    pub active_sessions: usize,
    pub active_call_ids: Vec<u16>,
    pub last_recording_id: Option<String>,
    pub last_error: Option<String>,
    pub archive_enabled: bool,
    pub archive_directory: String,
    pub tts_archive_enabled: bool,
    pub tts_archive_directory: String,
    pub archive_available: bool,
    pub archive_active: bool,
    pub archive_pending: usize,
    pub archive_completed: usize,
    pub archive_last_success_at: Option<String>,
    pub archive_last_error: Option<String>,
    pub media_library_enabled: bool,
    pub media_library_url: String,
    pub media_library_available: bool,
    pub media_library_active: bool,
    pub media_library_pending: usize,
    pub media_library_completed: usize,
    pub media_library_last_success_at: Option<String>,
    pub media_library_last_error: Option<String>,
}
