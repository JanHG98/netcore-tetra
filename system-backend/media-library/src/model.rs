// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für gespeicherte Aufzeichnungen, TTS- und Mediendateien.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
// Was: Bündelt die zusammengehörigen Werte für broadcast hint in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct BroadcastHint {
    pub destination_kind: Option<String>,
    pub destination_id: Option<u32>,
    pub priority: Option<u8>,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
// Was: Bündelt die zusammengehörigen Werte für audio metadata in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct AudioMetadata {
    pub format: String,
    pub codec: Option<String>,
    pub channels: Option<u16>,
    pub sample_rate_hz: Option<u32>,
    pub bits_per_sample: Option<u16>,
    pub duration_ms: Option<u64>,
    pub data_bytes: Option<u64>,
    pub tetra_frame_count: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
// Was: Bewahrt die für verständliche Archivnamen relevanten Metadaten einer Basisstationsaufzeichnung.
// Warum: Das NFS-/SMB-Archiv kann dadurch nach Datum sortieren und Dateien ohne UUID-Rätsel benennen.
pub struct SourceRecordingMetadata {
    pub station_id: Option<String>,
    pub recording_id: Option<String>,
    pub recorded_at: Option<String>,
    pub ended_at: Option<String>,
    pub source_issi: Option<u32>,
    pub destination_id: Option<u32>,
    pub destination_type: Option<String>,
    pub duration_ms: Option<u64>,
    pub call_id: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für asset Datensatz in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct AssetRecord {
    pub asset_id: String,
    pub title: String,
    pub description: Option<String>,
    pub kind: String,
    pub state: String,
    pub approval: String,
    pub tags: BTreeSet<String>,
    pub source: String,
    pub source_url: Option<String>,
    pub source_reference: Option<String>,
    #[serde(default)]
    pub source_metadata: Option<SourceRecordingMetadata>,
    pub original_filename: String,
    pub media_type: String,
    pub original_path: Option<PathBuf>,
    pub preview_path: Option<PathBuf>,
    pub tetra_path: Option<PathBuf>,
    pub archive_path: Option<PathBuf>,
    pub sha256: Option<String>,
    #[serde(default)]
    pub preview_sha256: Option<String>,
    #[serde(default)]
    pub tetra_sha256: Option<String>,
    pub size_bytes: Option<u64>,
    pub metadata: AudioMetadata,
    pub preview_ready: bool,
    pub broadcast_ready: bool,
    pub archived: bool,
    pub voice: Option<String>,
    pub text: Option<String>,
    pub broadcast_hint: Option<BroadcastHint>,
    pub duplicate_of: Option<String>,
    pub processing_attempts: u32,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für dispatch job in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct DispatchJob {
    pub job_id: String,
    pub asset_id: String,
    pub session_id: String,
    pub target_node: Option<String>,
    pub target_logical_ts: Option<u8>,
    pub destination_kind: Option<String>,
    pub destination_id: Option<u32>,
    pub priority: u8,
    pub state: String,
    pub frame_index: u64,
    pub frame_count: u64,
    pub attempts: u32,
    pub max_attempts: u32,
    pub queued_targets: u64,
    pub cancel_requested: bool,
    pub last_error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Ereignis Datensatz in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct EventRecord {
    pub seq: u64,
    pub timestamp: DateTime<Utc>,
    pub kind: String,
    pub asset_id: Option<String>,
    pub job_id: Option<String>,
    pub detail: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für audit Datensatz in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct AuditRecord {
    pub seq: u64,
    pub timestamp: DateTime<Utc>,
    pub actor: String,
    pub action: String,
    pub object_type: String,
    pub object_id: String,
    pub result: String,
    pub detail: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für backup Datensatz in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct BackupRecord {
    pub backup_id: String,
    pub path: PathBuf,
    pub sha256: String,
    pub size_bytes: u64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
// Was: Bündelt die zusammengehörigen Werte für Status view in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct StatusView {
    pub service: String,
    pub security_mode: String,
    pub operating_mode: String,
    pub ready: bool,
    pub storage_available: bool,
    pub archive_available: bool,
    pub media_switch_connected: bool,
    pub recorder_connected: bool,
    pub application_gateway_connected: bool,
    pub assets_total: usize,
    pub assets_importing: usize,
    pub assets_processing: usize,
    pub assets_ready: usize,
    pub assets_failed: usize,
    pub assets_approved: usize,
    pub preview_ready: usize,
    pub broadcast_ready: usize,
    pub jobs_queued: usize,
    pub jobs_playing: usize,
    pub jobs_completed: usize,
    pub jobs_failed: usize,
    pub storage_used_bytes: u64,
    pub started_at: DateTime<Utc>,
    pub last_dependency_probe_at: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für upload input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct UploadInput {
    pub name: String,
    pub filename: String,
    pub media_type: Option<String>,
    pub kind: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub data_base64: String,
    pub approve: Option<bool>,
    pub actor: Option<String>,
    pub broadcast: Option<BroadcastHint>,
}

#[derive(Debug, Clone, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für import url input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ImportUrlInput {
    pub schema: Option<String>,
    pub source: Option<String>,
    /// Stable idempotency key supplied by the source system.
    pub source_reference: Option<String>,
    #[serde(default)]
    pub source_metadata: Option<SourceRecordingMetadata>,
    pub source_url: String,
    pub name: String,
    pub filename: Option<String>,
    pub sha256: Option<String>,
    pub size_bytes: Option<u64>,
    pub media_type: Option<String>,
    pub kind: Option<String>,
    pub voice: Option<String>,
    pub text: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub approve: Option<bool>,
    pub actor: Option<String>,
    pub broadcast: Option<BroadcastHint>,
}

#[derive(Debug, Clone, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Aufzeichnung import input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RecorderImportInput {
    pub recording_id: String,
    pub name: Option<String>,
    pub approve: Option<bool>,
    pub actor: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für asset update input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct AssetUpdateInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub kind: Option<String>,
    pub tags: Option<Vec<String>>,
    pub broadcast_hint: Option<BroadcastHint>,
    pub actor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
// Was: Bündelt die zusammengehörigen Werte für action input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ActionInput {
    pub actor: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
// Was: Bündelt die zusammengehörigen Werte für approval input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ApprovalInput {
    pub actor: Option<String>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für dispatch input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct DispatchInput {
    pub asset_id: String,
    pub session_id: Option<String>,
    pub target_node: Option<String>,
    pub target_logical_ts: Option<u8>,
    pub destination_kind: Option<String>,
    pub destination_id: Option<u32>,
    pub priority: Option<u8>,
    pub actor: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
// Was: Bündelt die zusammengehörigen Werte für Konfiguration view in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ConfigView {
    pub server_bind: String,
    pub public_base_url: String,
    pub security_mode: String,
    pub token_auth: bool,
    pub tls: bool,
    pub allow_delete: bool,
    pub operating_mode: String,
    pub storage_root: PathBuf,
    pub archive_root: Option<PathBuf>,
    pub max_asset_bytes: u64,
    pub max_total_bytes: u64,
    pub ffmpeg_available: bool,
    pub tetra_encoder_configured: bool,
    pub tetra_decoder_configured: bool,
    pub dependencies: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für import claim in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ImportClaim {
    pub asset_id: String,
    pub source_url: String,
    pub expected_sha256: Option<String>,
    pub expected_size_bytes: Option<u64>,
    pub filename: String,
    pub media_type: String,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für processing claim in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ProcessingClaim {
    pub asset: AssetRecord,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für dispatch claim in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct DispatchClaim {
    pub job: DispatchJob,
    pub tetra_path: PathBuf,
    pub expected_tetra_sha256: Option<String>,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für process result in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ProcessResult {
    pub metadata: AudioMetadata,
    pub preview_path: Option<PathBuf>,
    pub tetra_path: Option<PathBuf>,
    pub preview_sha256: Option<String>,
    pub tetra_sha256: Option<String>,
    pub preview_ready: bool,
    pub broadcast_ready: bool,
}
