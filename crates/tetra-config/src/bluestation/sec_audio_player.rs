// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Einlesen und Prüfen der TETRA-Konfiguration.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use serde::Deserialize;
use toml::Value;

/// Read-only external media source mounted by the operating system.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für cfg audio share in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgAudioShare {
    /// Stable identifier used by the dashboard/API.
    pub id: String,
    /// Human-readable label shown in the media browser.
    pub name: String,
    /// Absolute local mount path, for example `/mnt/nfs-share`.
    pub path: String,
}

/// Local audio dispatch configuration.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für cfg audio player in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgAudioPlayer {
    /// Instantiate the audio-player entity and dashboard API.
    pub enabled: bool,
    /// Root directory for locally managed WAV/MP3 files.
    pub directory: String,
    /// Local staging directory used before decoding files from network shares.
    pub cache_directory: String,
    /// Additional read-only media roots mounted by the operating system.
    pub shares: Vec<CfgAudioShare>,
    /// TETRA identity displayed as the network-side source of generated calls.
    pub source_issi: u32,
    /// Default TETRA call priority used when the UI does not override it.
    pub default_priority: u8,
    /// Reject source files larger than this value.
    pub max_file_size_mb: u64,
    /// Reject or truncate decoded audio beyond this duration.
    pub max_duration_seconds: u32,
    /// Number of encoded silence blocks sent before the source audio starts.
    ///
    /// This gives group-call subscribers time to receive D-SETUP and switch to
    /// the assigned traffic channel before the first spoken syllable.
    pub lead_in_silence_blocks: u8,
    /// Number of encoded silence blocks appended before call release.
    pub tail_silence_blocks: u8,
    /// Keep a completed group dispatch busy until CMCE hangtime has fully expired.
    ///
    /// This prevents a subsequent dispatch from replacing the Brew UUID of a
    /// group call that is still in NoActiveSpeaker/hangtime.
    pub group_release_guard_seconds: u32,
    /// Maximum time to wait for an individual subscriber to answer.
    pub individual_answer_timeout_seconds: u32,
    /// ffmpeg executable used for MP3 and non-native WAV conversion.
    pub ffmpeg_path: String,
}

// Was: Implementiert das zugehörige Verhalten für `Default for CfgAudioPlayer`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for CfgAudioPlayer {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        apply_audio_player_patch(CfgAudioPlayerDto::default()).expect("default audio-player config must be valid")
    }
}

#[derive(Debug, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für cfg audio share dto in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgAudioShareDto {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub path: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für cfg audio player dto in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgAudioPlayerDto {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_directory")]
    pub directory: String,
    #[serde(default = "default_cache_directory")]
    pub cache_directory: String,
    #[serde(default)]
    pub shares: Vec<CfgAudioShareDto>,
    #[serde(default = "default_source_issi")]
    pub source_issi: u32,
    #[serde(default = "default_priority")]
    pub default_priority: u8,
    #[serde(default = "default_max_file_size_mb")]
    pub max_file_size_mb: u64,
    #[serde(default = "default_max_duration_seconds")]
    pub max_duration_seconds: u32,
    #[serde(default = "default_lead_in_silence_blocks")]
    pub lead_in_silence_blocks: u8,
    #[serde(default = "default_tail_silence_blocks")]
    pub tail_silence_blocks: u8,
    #[serde(default = "default_group_release_guard_seconds")]
    pub group_release_guard_seconds: u32,
    #[serde(default = "default_individual_answer_timeout_seconds")]
    pub individual_answer_timeout_seconds: u32,
    #[serde(default = "default_ffmpeg_path")]
    pub ffmpeg_path: String,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// Was: Implementiert das zugehörige Verhalten für `Default for CfgAudioPlayerDto`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for CfgAudioPlayerDto {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            enabled: false,
            directory: default_directory(),
            cache_directory: default_cache_directory(),
            shares: Vec::new(),
            source_issi: default_source_issi(),
            default_priority: default_priority(),
            max_file_size_mb: default_max_file_size_mb(),
            max_duration_seconds: default_max_duration_seconds(),
            lead_in_silence_blocks: default_lead_in_silence_blocks(),
            tail_silence_blocks: default_tail_silence_blocks(),
            group_release_guard_seconds: default_group_release_guard_seconds(),
            individual_answer_timeout_seconds: default_individual_answer_timeout_seconds(),
            ffmpeg_path: default_ffmpeg_path(),
            extra: HashMap::new(),
        }
    }
}

// Was: Führt den Arbeitsschritt `default_directory` für default directory aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_directory() -> String {
    "/var/lib/netcore/audio".to_string()
}
// Was: Führt den Arbeitsschritt `default_cache_directory` für default Zwischenspeicher directory aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_cache_directory() -> String {
    "/var/cache/netcore/audio".to_string()
}
// Was: Führt den Arbeitsschritt `default_source_issi` für default source Teilnehmerkennung (ISSI) aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_source_issi() -> u32 {
    4_010_099
}
// Was: Führt den Arbeitsschritt `default_priority` für default Priorität aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_priority() -> u8 {
    5
}
// Was: Führt den Arbeitsschritt `default_max_file_size_mb` für default max file size mb aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_max_file_size_mb() -> u64 {
    100
}
// Was: Führt den Arbeitsschritt `default_max_duration_seconds` für default max duration seconds aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_max_duration_seconds() -> u32 {
    1_800
}
// Was: Führt den Arbeitsschritt `default_lead_in_silence_blocks` für default lead in silence blocks aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_lead_in_silence_blocks() -> u8 {
    12
}
// Was: Führt den Arbeitsschritt `default_tail_silence_blocks` für default tail silence blocks aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_tail_silence_blocks() -> u8 {
    3
}
// Was: Führt den Arbeitsschritt `default_group_release_guard_seconds` für default Gruppe release guard seconds aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_group_release_guard_seconds() -> u32 {
    6
}
// Was: Führt den Arbeitsschritt `default_individual_answer_timeout_seconds` für default individual answer timeout seconds aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_individual_answer_timeout_seconds() -> u32 {
    30
}
// Was: Führt den Arbeitsschritt `default_ffmpeg_path` für default ffmpeg path aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_ffmpeg_path() -> String {
    "ffmpeg".to_string()
}

// Was: Diese Funktion prüft absolute directory.
// Warum: Unzulässige Werte werden dadurch erkannt, bevor sie im Betrieb Schaden anrichten.
fn validate_absolute_directory(value: &str, field: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err(format!("audio_player: {field} cannot be empty"));
    }
    if !Path::new(value).is_absolute() {
        return Err(format!("audio_player: {field} must be an absolute path"));
    }
    Ok(())
}

// Was: Führt den Arbeitsschritt `valid_source_id` für valid source Kennung aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn valid_source_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 48
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

// Was: Diese Funktion wendet audio player patch.
// Warum: Die Änderung wird dadurch nur über einen definierten und prüfbaren Weg wirksam.
pub fn apply_audio_player_patch(mut src: CfgAudioPlayerDto) -> Result<CfgAudioPlayer, String> {
    src.directory = src.directory.trim().to_string();
    src.cache_directory = src.cache_directory.trim().to_string();
    src.ffmpeg_path = src.ffmpeg_path.trim().to_string();
    validate_absolute_directory(&src.directory, "directory")?;
    validate_absolute_directory(&src.cache_directory, "cache_directory")?;
    if src.source_issi == 0 || src.source_issi > 0x00ff_ffff {
        return Err("audio_player: source_issi must be a valid 24-bit ISSI".to_string());
    }
    if src.default_priority > 15 {
        return Err("audio_player: default_priority must be 0-15".to_string());
    }
    if src.max_file_size_mb == 0 {
        return Err("audio_player: max_file_size_mb must be greater than zero".to_string());
    }
    if src.max_duration_seconds == 0 {
        return Err("audio_player: max_duration_seconds must be greater than zero".to_string());
    }
    if src.lead_in_silence_blocks > 40 {
        return Err("audio_player: lead_in_silence_blocks must be 0-40".to_string());
    }
    if src.tail_silence_blocks > 20 {
        return Err("audio_player: tail_silence_blocks must be 0-20".to_string());
    }
    if !(5..=30).contains(&src.group_release_guard_seconds) {
        return Err("audio_player: group_release_guard_seconds must be 5-30".to_string());
    }
    if src.individual_answer_timeout_seconds == 0 {
        return Err("audio_player: individual_answer_timeout_seconds must be greater than zero".to_string());
    }
    if src.ffmpeg_path.is_empty() {
        return Err("audio_player: ffmpeg_path cannot be empty".to_string());
    }

    let mut ids = HashSet::new();
    ids.insert("local".to_string());
    let mut shares = Vec::with_capacity(src.shares.len());
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for (index, mut share) in src.shares.into_iter().enumerate() {
        share.id = share.id.trim().to_ascii_lowercase();
        share.name = share.name.trim().to_string();
        share.path = share.path.trim().to_string();
        if !share.extra.is_empty() {
            let mut keys: Vec<_> = share.extra.keys().cloned().collect();
            keys.sort();
            return Err(format!(
                "audio_player: unrecognized fields in shares[{index}]: {keys:?}"
            ));
        }
        if !valid_source_id(&share.id) {
            return Err(format!(
                "audio_player: shares[{index}].id must contain only letters, numbers, '.', '-' or '_'"
            ));
        }
        if !ids.insert(share.id.clone()) {
            return Err(format!("audio_player: duplicate media-source id '{}'", share.id));
        }
        if share.name.is_empty() {
            return Err(format!("audio_player: shares[{index}].name cannot be empty"));
        }
        validate_absolute_directory(&share.path, &format!("shares[{index}].path"))?;
        shares.push(CfgAudioShare {
            id: share.id,
            name: share.name,
            path: share.path,
        });
    }

    Ok(CfgAudioPlayer {
        enabled: src.enabled,
        directory: src.directory,
        cache_directory: src.cache_directory,
        shares,
        source_issi: src.source_issi,
        default_priority: src.default_priority,
        max_file_size_mb: src.max_file_size_mb,
        max_duration_seconds: src.max_duration_seconds,
        lead_in_silence_blocks: src.lead_in_silence_blocks,
        tail_silence_blocks: src.tail_silence_blocks,
        group_release_guard_seconds: src.group_release_guard_seconds,
        individual_answer_timeout_seconds: src.individual_answer_timeout_seconds,
        ffmpeg_path: src.ffmpeg_path,
    })
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
        let cfg = CfgAudioPlayer::default();
        assert!(!cfg.enabled);
        assert_eq!(cfg.source_issi, 4_010_099);
        assert!(cfg.max_duration_seconds > 0);
        assert_eq!(cfg.lead_in_silence_blocks, 12);
        assert_eq!(cfg.group_release_guard_seconds, 6);
        assert!(cfg.shares.is_empty());
    }

    #[test]
    // Was: Führt den Arbeitsschritt `rejects_invalid_identity_and_priority` für rejects invalid identity and Priorität aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn rejects_invalid_identity_and_priority() {
        let dto = CfgAudioPlayerDto {
            source_issi: 0,
            default_priority: 16,
            ..CfgAudioPlayerDto::default()
        };
        assert!(apply_audio_player_patch(dto).is_err());
    }

    #[test]
    // Was: Führt den Arbeitsschritt `rejects_unsafe_rf_guard_values` für rejects unsafe Funkstrecke guard values aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn rejects_unsafe_rf_guard_values() {
        let dto = CfgAudioPlayerDto {
            lead_in_silence_blocks: 41,
            ..CfgAudioPlayerDto::default()
        };
        assert!(apply_audio_player_patch(dto).is_err());

        let dto = CfgAudioPlayerDto {
            group_release_guard_seconds: 4,
            ..CfgAudioPlayerDto::default()
        };
        assert!(apply_audio_player_patch(dto).is_err());
    }

    #[test]
    // Was: Führt den Arbeitsschritt `accepts_read_only_mounted_share` für accepts read only mounted share aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn accepts_read_only_mounted_share() {
        let dto = CfgAudioPlayerDto {
            shares: vec![CfgAudioShareDto {
                id: "server".to_string(),
                name: "NFS Server".to_string(),
                path: "/mnt/nfs-share".to_string(),
                extra: HashMap::new(),
            }],
            ..CfgAudioPlayerDto::default()
        };
        let cfg = apply_audio_player_patch(dto).unwrap();
        assert_eq!(cfg.shares[0].id, "server");
    }

    #[test]
    // Was: Führt den Arbeitsschritt `rejects_duplicate_or_relative_share` für rejects duplicate or relative share aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn rejects_duplicate_or_relative_share() {
        let dto = CfgAudioPlayerDto {
            shares: vec![CfgAudioShareDto {
                id: "local".to_string(),
                name: "Bad".to_string(),
                path: "relative".to_string(),
                extra: HashMap::new(),
            }],
            ..CfgAudioPlayerDto::default()
        };
        assert!(apply_audio_player_patch(dto).is_err());
    }
}
