// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Einlesen und Prüfen der TETRA-Konfiguration.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use toml::Value;

/// Selects which locally-originated speech calls are written to disk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für recording mode auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum RecordingMode {
    /// Record every call for which CMCE exposes a local speech floor.
    All,
    /// Record only group calls whose GSSI appears in `selected_groups`.
    SelectedGroups,
}

// Was: Implementiert das zugehörige Verhalten für `Default for RecordingMode`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for RecordingMode {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self::All
    }
}

/// Local TETRA speech recording configuration.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für cfg recording in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgRecording {
    /// Instantiate the recorder entity and expose its dashboard API.
    pub enabled: bool,
    /// Initial runtime state after process start. The dashboard may toggle this live.
    pub active: bool,
    /// Root directory for WAV files and JSON sidecars.
    pub directory: String,
    /// Call selection policy.
    pub mode: RecordingMode,
    /// GSSI allow-list used when `mode = "selected_groups"`.
    pub selected_groups: Vec<u32>,
    /// Do not begin a new recording when free space falls below this threshold.
    pub minimum_free_space_mb: u64,
    /// Delete completed recordings older than this many days. Zero disables retention cleanup.
    pub retention_days: u32,
    /// Hard limit for one recording. Prevents an orphaned call from filling the disk.
    pub max_recording_minutes: u32,
    /// Finalize a call after this many seconds without an active floor or call-end event.
    pub idle_finalize_secs: u32,
    /// Maximum number of entries returned by the recordings API.
    pub max_list_entries: usize,
    /// Copy completed WAV/JSON pairs to the configured archive directory.
    pub archive_enabled: bool,
    /// Existing writable directory on an OS-mounted server share for normal call recordings.
    pub archive_directory: String,
    /// Copy imported TTS library WAV/JSON pairs to a separate server directory.
    pub tts_archive_enabled: bool,
    /// Existing writable directory on an OS-mounted server share for TTS library WAVs.
    pub tts_archive_directory: String,
    /// Retry interval for pending copies while either share is unavailable.
    pub archive_retry_seconds: u64,
}

// Was: Implementiert das zugehörige Verhalten für `Default for CfgRecording`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for CfgRecording {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        apply_recording_patch(CfgRecordingDto::default()).expect("default recording config must be valid")
    }
}

#[derive(Debug, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für cfg recording dto in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgRecordingDto {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub active: bool,
    #[serde(default = "default_directory")]
    pub directory: String,
    #[serde(default)]
    pub mode: RecordingMode,
    #[serde(default)]
    pub selected_groups: Vec<u32>,
    #[serde(default = "default_minimum_free_space_mb")]
    pub minimum_free_space_mb: u64,
    #[serde(default = "default_retention_days")]
    pub retention_days: u32,
    #[serde(default = "default_max_recording_minutes")]
    pub max_recording_minutes: u32,
    #[serde(default = "default_idle_finalize_secs")]
    pub idle_finalize_secs: u32,
    #[serde(default = "default_max_list_entries")]
    pub max_list_entries: usize,
    #[serde(default)]
    pub archive_enabled: bool,
    #[serde(default = "default_archive_directory")]
    pub archive_directory: String,
    #[serde(default)]
    pub tts_archive_enabled: bool,
    #[serde(default = "default_tts_archive_directory")]
    pub tts_archive_directory: String,
    #[serde(default = "default_archive_retry_seconds")]
    pub archive_retry_seconds: u64,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// Was: Implementiert das zugehörige Verhalten für `Default for CfgRecordingDto`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for CfgRecordingDto {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            enabled: false,
            active: false,
            directory: default_directory(),
            mode: RecordingMode::All,
            selected_groups: Vec::new(),
            minimum_free_space_mb: default_minimum_free_space_mb(),
            retention_days: default_retention_days(),
            max_recording_minutes: default_max_recording_minutes(),
            idle_finalize_secs: default_idle_finalize_secs(),
            max_list_entries: default_max_list_entries(),
            archive_enabled: false,
            archive_directory: default_archive_directory(),
            tts_archive_enabled: false,
            tts_archive_directory: default_tts_archive_directory(),
            archive_retry_seconds: default_archive_retry_seconds(),
            extra: HashMap::new(),
        }
    }
}

// Was: Führt den Arbeitsschritt `default_directory` für default directory aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_directory() -> String {
    "/var/lib/netcore/recordings".to_string()
}

// Was: Führt den Arbeitsschritt `default_minimum_free_space_mb` für default minimum free space mb aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_minimum_free_space_mb() -> u64 {
    2_048
}

// Was: Führt den Arbeitsschritt `default_retention_days` für default retention days aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_retention_days() -> u32 {
    30
}

// Was: Führt den Arbeitsschritt `default_max_recording_minutes` für default max recording minutes aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_max_recording_minutes() -> u32 {
    120
}

// Was: Führt den Arbeitsschritt `default_idle_finalize_secs` für default idle finalize secs aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_idle_finalize_secs() -> u32 {
    15
}

// Was: Führt den Arbeitsschritt `default_max_list_entries` für default max list entries aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_max_list_entries() -> usize {
    2_000
}

// Was: Führt den Arbeitsschritt `default_archive_directory` für default archive directory aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_archive_directory() -> String {
    "/mnt/nfs-share/Recordings".to_string()
}

// Was: Führt den Arbeitsschritt `default_tts_archive_directory` für default tts archive directory aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_tts_archive_directory() -> String {
    "/mnt/nfs-share/TTS-Dateien".to_string()
}

// Was: Führt den Arbeitsschritt `default_archive_retry_seconds` für default archive retry seconds aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_archive_retry_seconds() -> u64 {
    60
}

// Was: Diese Funktion wendet recording patch.
// Warum: Die Änderung wird dadurch nur über einen definierten und prüfbaren Weg wirksam.
pub fn apply_recording_patch(mut src: CfgRecordingDto) -> Result<CfgRecording, String> {
    src.directory = src.directory.trim().to_string();
    if src.directory.is_empty() {
        return Err("recording: directory cannot be empty".to_string());
    }
    if src.max_recording_minutes == 0 {
        return Err("recording: max_recording_minutes must be greater than zero".to_string());
    }
    if src.idle_finalize_secs == 0 {
        return Err("recording: idle_finalize_secs must be greater than zero".to_string());
    }
    if src.max_list_entries == 0 {
        return Err("recording: max_list_entries must be greater than zero".to_string());
    }
    src.archive_directory = src.archive_directory.trim().to_string();
    src.tts_archive_directory = src.tts_archive_directory.trim().to_string();
    validate_archive_directory(
        "archive_directory",
        src.archive_enabled,
        &src.archive_directory,
        &src.directory,
    )?;
    validate_archive_directory(
        "tts_archive_directory",
        src.tts_archive_enabled,
        &src.tts_archive_directory,
        &src.directory,
    )?;
    if src.archive_enabled
        && src.tts_archive_enabled
        && src.archive_directory == src.tts_archive_directory
    {
        return Err(
            "recording: archive_directory and tts_archive_directory must differ".to_string(),
        );
    }
    if (src.archive_enabled || src.tts_archive_enabled) && src.archive_retry_seconds == 0 {
        return Err("recording: archive_retry_seconds must be greater than zero".to_string());
    }
    if src.selected_groups.iter().any(|gssi| *gssi == 0 || *gssi > 0x00ff_ffff) {
        return Err("recording: selected_groups entries must be valid 24-bit GSSIs".to_string());
    }
    src.selected_groups.sort_unstable();
    src.selected_groups.dedup();

    Ok(CfgRecording {
        enabled: src.enabled,
        active: src.active,
        directory: src.directory,
        mode: src.mode,
        selected_groups: src.selected_groups,
        minimum_free_space_mb: src.minimum_free_space_mb,
        retention_days: src.retention_days,
        max_recording_minutes: src.max_recording_minutes,
        idle_finalize_secs: src.idle_finalize_secs,
        max_list_entries: src.max_list_entries,
        archive_enabled: src.archive_enabled,
        archive_directory: src.archive_directory,
        tts_archive_enabled: src.tts_archive_enabled,
        tts_archive_directory: src.tts_archive_directory,
        archive_retry_seconds: src.archive_retry_seconds,
    })
}

// Was: Diese Funktion prüft archive directory.
// Warum: Unzulässige Werte werden dadurch erkannt, bevor sie im Betrieb Schaden anrichten.
fn validate_archive_directory(
    field: &str,
    enabled: bool,
    value: &str,
    local_directory: &str,
) -> Result<(), String> {
    if !enabled {
        return Ok(());
    }
    if value.is_empty() {
        return Err(format!(
            "recording: {field} cannot be empty when its archive is enabled"
        ));
    }
    if !Path::new(value).is_absolute() {
        return Err(format!("recording: {field} must be an absolute path"));
    }
    if value == local_directory {
        return Err(format!(
            "recording: {field} must differ from directory"
        ));
    }
    Ok(())
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Führt den Arbeitsschritt `defaults_are_safe_and_disabled` für defaults are safe and disabled aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn defaults_are_safe_and_disabled() {
        let cfg = CfgRecording::default();
        assert!(!cfg.enabled);
        assert!(!cfg.active);
        assert_eq!(cfg.mode, RecordingMode::All);
        assert!(cfg.minimum_free_space_mb > 0);
    }

    #[test]
    // Was: Führt den Arbeitsschritt `rejects_invalid_group_ids` für rejects invalid Gruppe ids aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn rejects_invalid_group_ids() {
        let dto = CfgRecordingDto {
            selected_groups: vec![0, 0x0100_0000],
            ..CfgRecordingDto::default()
        };
        assert!(apply_recording_patch(dto).is_err());
    }

    #[test]
    // Was: Führt den Arbeitsschritt `rejects_relative_archive_directory` für rejects relative archive directory aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn rejects_relative_archive_directory() {
        let dto = CfgRecordingDto {
            archive_enabled: true,
            archive_directory: "relative/archive".to_string(),
            ..CfgRecordingDto::default()
        };
        assert!(apply_recording_patch(dto).is_err());
    }

    #[test]
    // Was: Führt den Arbeitsschritt `rejects_relative_tts_archive_directory` für rejects relative tts archive directory aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn rejects_relative_tts_archive_directory() {
        let dto = CfgRecordingDto {
            tts_archive_enabled: true,
            tts_archive_directory: "relative/tts".to_string(),
            ..CfgRecordingDto::default()
        };
        assert!(apply_recording_patch(dto).is_err());
    }

    #[test]
    // Was: Führt den Arbeitsschritt `keeps_recording_and_tts_archive_separate` für keeps recording and tts archive separate aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn keeps_recording_and_tts_archive_separate() {
        let dto = CfgRecordingDto {
            archive_enabled: true,
            tts_archive_enabled: true,
            archive_directory: "/mnt/nfs-share/Recordings".to_string(),
            tts_archive_directory: "/mnt/nfs-share/TTS-Dateien".to_string(),
            ..CfgRecordingDto::default()
        };
        let cfg = apply_recording_patch(dto).expect("split archive config should be valid");
        assert_eq!(cfg.archive_directory, "/mnt/nfs-share/Recordings");
        assert_eq!(cfg.tts_archive_directory, "/mnt/nfs-share/TTS-Dateien");
    }
}
