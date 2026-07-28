// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für audio target type auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum AudioTargetType {
    Group,
    Individual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für audio source type auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum AudioSourceType {
    Media,
    Recording,
    Tts,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für audio player Zustand auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum AudioPlayerState {
    Idle,
    Preparing,
    Calling,
    WaitingForAnswer,
    Playing,
    Finishing,
    Failed,
}

#[derive(Debug, Clone, Serialize)]
// Was: Bündelt die zusammengehörigen Werte für audio player Status in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct AudioPlayerStatus {
    pub available: bool,
    pub state: AudioPlayerState,
    pub directory: String,
    pub cache_directory: String,
    pub startup_warning: Option<String>,
    pub job_id: Option<String>,
    pub file_name: Option<String>,
    pub source_type: Option<AudioSourceType>,
    pub source_id: Option<String>,
    pub target_type: Option<AudioTargetType>,
    pub target_id: Option<u32>,
    pub priority: Option<u8>,
    pub duration_ms: u64,
    pub position_ms: u64,
    pub total_blocks: usize,
    pub sent_blocks: usize,
    pub call_id: Option<u16>,
    pub timeslot: Option<u8>,
    pub ffmpeg_available: bool,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
// Was: Bündelt die zusammengehörigen Werte für Audio- und Mediendaten source info in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MediaSourceInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    pub source_type: String,
    pub available: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
// Was: Bündelt die zusammengehörigen Werte für Audio- und Mediendaten entry in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MediaEntry {
    pub name: String,
    pub path: String,
    pub entry_type: String,
    pub size_bytes: Option<u64>,
    pub extension: Option<String>,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für resolved audio source in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub(crate) struct ResolvedAudioSource {
    pub path: std::path::PathBuf,
    pub display_name: String,
    pub source_type: AudioSourceType,
    pub source_id: Option<String>,
    pub cache_before_decode: bool,
}

#[derive(Debug, Clone)]
// Was: Listet die möglichen Varianten für audio player command auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub(crate) enum AudioPlayerCommand {
    Play {
        job_id: String,
        source: ResolvedAudioSource,
        target_type: AudioTargetType,
        target_id: u32,
        priority: u8,
    },
    Stop,
}

#[derive(Debug)]
// Was: Bündelt die zusammengehörigen Werte für prepared audio in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub(crate) struct PreparedAudio {
    pub job_id: String,
    pub target_type: AudioTargetType,
    pub target_id: u32,
    pub priority: u8,
    pub duration_ms: u64,
    pub blocks: Vec<Vec<u8>>,
}

#[derive(Debug)]
// Was: Listet die möglichen Varianten für prepare Ereignis auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub(crate) enum PrepareEvent {
    Ready(PreparedAudio),
    Failed { job_id: String, error: String },
}
