// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Einlesen und Prüfen der TETRA-Konfiguration.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use serde::Deserialize;
use toml::Value;

/// One operator-facing voice backed by a voice name exposed by Piper HTTP.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für cfg tts voice in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgTtsVoice {
    /// Stable ID used by the dashboard and API.
    pub id: String,
    /// Human-readable label shown in the dashboard.
    pub name: String,
    /// Voice name sent to Piper, for example `de_DE-thorsten-medium`.
    pub provider_voice: String,
    /// Optional speaker ID for multi-speaker models.
    pub speaker_id: Option<u32>,
}

/// Local text-to-speech configuration.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für cfg tts in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgTts {
    /// Instantiate the TTS service and dashboard API.
    pub enabled: bool,
    /// Piper HTTP base URL. The service is expected to expose `/voices` and accept
    /// JSON synthesis requests on `/synthesize`.
    pub endpoint: String,
    /// Local directory for generated WAV files.
    pub cache_directory: String,
    /// Local directory for persistent operator templates.
    pub template_directory: String,
    /// Automatically store every successfully generated text as a local template.
    pub auto_save_generated_templates: bool,
    /// Stable ID of the voice selected by default in the dashboard.
    pub default_voice: String,
    /// Operator-facing speed multiplier. 1.0 is normal, values above 1.0 are faster.
    pub default_speed: f32,
    /// Default TETRA priority used for direct TTS dispatch.
    pub default_priority: u8,
    /// Maximum accepted Unicode character count.
    pub max_text_characters: usize,
    /// Complete HTTP synthesis timeout.
    pub synthesis_timeout_seconds: u64,
    /// Reject a generated WAV larger than this value.
    pub max_output_file_mb: u64,
    /// Remove generated cache files older than this many minutes. Zero disables cleanup.
    pub cache_retention_minutes: u64,
    /// Keep generated WAV files after a completed radio dispatch.
    pub keep_generated_audio: bool,
    /// Operator-visible voice definitions.
    pub voices: Vec<CfgTtsVoice>,
}

// Was: Implementiert das zugehörige Verhalten für `Default for CfgTts`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for CfgTts {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        apply_tts_patch(CfgTtsDto::default()).expect("default TTS config must be valid")
    }
}

#[derive(Debug, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für cfg tts voice dto in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgTtsVoiceDto {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub provider_voice: String,
    #[serde(default)]
    pub speaker_id: Option<u32>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

#[derive(Debug, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für cfg tts dto in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgTtsDto {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_endpoint")]
    pub endpoint: String,
    #[serde(default = "default_cache_directory")]
    pub cache_directory: String,
    #[serde(default = "default_template_directory")]
    pub template_directory: String,
    #[serde(default = "default_auto_save_generated_templates")]
    pub auto_save_generated_templates: bool,
    #[serde(default = "default_voice")]
    pub default_voice: String,
    #[serde(default = "default_speed")]
    pub default_speed: f32,
    #[serde(default = "default_priority")]
    pub default_priority: u8,
    #[serde(default = "default_max_text_characters")]
    pub max_text_characters: usize,
    #[serde(default = "default_synthesis_timeout_seconds")]
    pub synthesis_timeout_seconds: u64,
    #[serde(default = "default_max_output_file_mb")]
    pub max_output_file_mb: u64,
    #[serde(default = "default_cache_retention_minutes")]
    pub cache_retention_minutes: u64,
    #[serde(default)]
    pub keep_generated_audio: bool,
    #[serde(default)]
    pub voices: Vec<CfgTtsVoiceDto>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// Was: Implementiert das zugehörige Verhalten für `Default for CfgTtsDto`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for CfgTtsDto {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: default_endpoint(),
            cache_directory: default_cache_directory(),
            template_directory: default_template_directory(),
            auto_save_generated_templates: default_auto_save_generated_templates(),
            default_voice: default_voice(),
            default_speed: default_speed(),
            default_priority: default_priority(),
            max_text_characters: default_max_text_characters(),
            synthesis_timeout_seconds: default_synthesis_timeout_seconds(),
            max_output_file_mb: default_max_output_file_mb(),
            cache_retention_minutes: default_cache_retention_minutes(),
            keep_generated_audio: false,
            voices: Vec::new(),
            extra: HashMap::new(),
        }
    }
}

// Was: Führt den Arbeitsschritt `default_endpoint` für default endpoint aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_endpoint() -> String {
    "http://127.0.0.1:5005".to_string()
}

// Was: Führt den Arbeitsschritt `default_cache_directory` für default Zwischenspeicher directory aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_cache_directory() -> String {
    "/var/cache/netcore/tts".to_string()
}

// Was: Führt den Arbeitsschritt `default_template_directory` für default template directory aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_template_directory() -> String {
    "/var/lib/netcore/tts/templates".to_string()
}

// Was: Führt den Arbeitsschritt `default_auto_save_generated_templates` für default auto save generated templates aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_auto_save_generated_templates() -> bool {
    true
}

// Was: Führt den Arbeitsschritt `default_voice` für default voice aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_voice() -> String {
    "de-thorsten".to_string()
}

// Was: Führt den Arbeitsschritt `default_speed` für default speed aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_speed() -> f32 {
    0.95
}

// Was: Führt den Arbeitsschritt `default_priority` für default Priorität aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_priority() -> u8 {
    5
}

// Was: Führt den Arbeitsschritt `default_max_text_characters` für default max text characters aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_max_text_characters() -> usize {
    2_000
}

// Was: Führt den Arbeitsschritt `default_synthesis_timeout_seconds` für default synthesis timeout seconds aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_synthesis_timeout_seconds() -> u64 {
    90
}

// Was: Führt den Arbeitsschritt `default_max_output_file_mb` für default max output file mb aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_max_output_file_mb() -> u64 {
    25
}

// Was: Führt den Arbeitsschritt `default_cache_retention_minutes` für default Zwischenspeicher retention minutes aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_cache_retention_minutes() -> u64 {
    1_440
}

// Was: Führt den Arbeitsschritt `valid_id` für valid Kennung aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn valid_id(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_'))
}

// Was: Diese Funktion prüft endpoint.
// Warum: Unzulässige Werte werden dadurch erkannt, bevor sie im Betrieb Schaden anrichten.
fn validate_endpoint(endpoint: &str) -> Result<(), String> {
    let lower = endpoint.to_ascii_lowercase();
    if !(lower.starts_with("http://") || lower.starts_with("https://")) {
        return Err("tts: endpoint must start with http:// or https://".to_string());
    }
    if endpoint.chars().any(char::is_whitespace) {
        return Err("tts: endpoint must not contain whitespace".to_string());
    }
    Ok(())
}

// Was: Diese Funktion wendet tts patch.
// Warum: Die Änderung wird dadurch nur über einen definierten und prüfbaren Weg wirksam.
pub fn apply_tts_patch(mut src: CfgTtsDto) -> Result<CfgTts, String> {
    src.endpoint = src.endpoint.trim().trim_end_matches('/').to_string();
    src.cache_directory = src.cache_directory.trim().to_string();
    src.template_directory = src.template_directory.trim().to_string();
    src.default_voice = src.default_voice.trim().to_string();

    validate_endpoint(&src.endpoint)?;
    if src.cache_directory.is_empty() || !Path::new(&src.cache_directory).is_absolute() {
        return Err("tts: cache_directory must be a non-empty absolute path".to_string());
    }
    if src.template_directory.is_empty() || !Path::new(&src.template_directory).is_absolute() {
        return Err("tts: template_directory must be a non-empty absolute path".to_string());
    }
    if !(0.50..=1.50).contains(&src.default_speed) {
        return Err("tts: default_speed must be between 0.50 and 1.50".to_string());
    }
    if src.default_priority > 15 {
        return Err("tts: default_priority must be 0-15".to_string());
    }
    if src.max_text_characters == 0 || src.max_text_characters > 20_000 {
        return Err("tts: max_text_characters must be between 1 and 20000".to_string());
    }
    if src.synthesis_timeout_seconds == 0 || src.synthesis_timeout_seconds > 600 {
        return Err("tts: synthesis_timeout_seconds must be between 1 and 600".to_string());
    }
    if src.max_output_file_mb == 0 || src.max_output_file_mb > 512 {
        return Err("tts: max_output_file_mb must be between 1 and 512".to_string());
    }

    let mut seen = HashSet::new();
    let mut voices = Vec::with_capacity(src.voices.len());
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for (index, mut voice) in src.voices.into_iter().enumerate() {
        if !voice.extra.is_empty() {
            let mut keys = voice.extra.keys().map(String::as_str).collect::<Vec<_>>();
            keys.sort_unstable();
            return Err(format!("tts: unrecognized fields in voices[{index}]: {keys:?}"));
        }
        voice.id = voice.id.trim().to_string();
        voice.name = voice.name.trim().to_string();
        voice.provider_voice = voice.provider_voice.trim().to_string();
        if !valid_id(&voice.id) {
            return Err(format!(
                "tts: voices[{index}].id must contain only letters, digits, '.', '-' or '_'"
            ));
        }
        if voice.name.is_empty() {
            return Err(format!("tts: voices[{index}].name cannot be empty"));
        }
        if voice.provider_voice.is_empty() {
            return Err(format!("tts: voices[{index}].provider_voice cannot be empty"));
        }
        if !seen.insert(voice.id.clone()) {
            return Err(format!("tts: duplicate voice id '{}'", voice.id));
        }
        voices.push(CfgTtsVoice {
            id: voice.id,
            name: voice.name,
            provider_voice: voice.provider_voice,
            speaker_id: voice.speaker_id,
        });
    }

    if src.enabled {
        if voices.is_empty() {
            return Err("tts: at least one [[tts.voices]] entry is required when enabled = true".to_string());
        }
        if !voices.iter().any(|voice| voice.id == src.default_voice) {
            return Err(format!(
                "tts: default_voice '{}' does not match any configured voice id",
                src.default_voice
            ));
        }
    }

    Ok(CfgTts {
        enabled: src.enabled,
        endpoint: src.endpoint,
        cache_directory: src.cache_directory,
        template_directory: src.template_directory,
        auto_save_generated_templates: src.auto_save_generated_templates,
        default_voice: src.default_voice,
        default_speed: src.default_speed,
        default_priority: src.default_priority,
        max_text_characters: src.max_text_characters,
        synthesis_timeout_seconds: src.synthesis_timeout_seconds,
        max_output_file_mb: src.max_output_file_mb,
        cache_retention_minutes: src.cache_retention_minutes,
        keep_generated_audio: src.keep_generated_audio,
        voices,
    })
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    // Was: Führt den Arbeitsschritt `voice` für voice aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn voice() -> CfgTtsVoiceDto {
        CfgTtsVoiceDto {
            id: "de-thorsten".to_string(),
            name: "Deutsch – Thorsten".to_string(),
            provider_voice: "de_DE-thorsten-medium".to_string(),
            speaker_id: None,
            extra: HashMap::new(),
        }
    }

    #[test]
    // Was: Führt den Arbeitsschritt `defaults_are_safe_and_disabled` für defaults are safe and disabled aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn defaults_are_safe_and_disabled() {
        let cfg = CfgTts::default();
        assert!(!cfg.enabled);
        assert!(cfg.endpoint.starts_with("http://"));
        assert!(Path::new(&cfg.template_directory).is_absolute());
        assert!(cfg.auto_save_generated_templates);
        assert!(cfg.voices.is_empty());
    }

    #[test]
    // Was: Führt den Arbeitsschritt `accepts_enabled_http_provider` für accepts enabled HTTP provider aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn accepts_enabled_http_provider() {
        let dto = CfgTtsDto {
            enabled: true,
            voices: vec![voice()],
            ..CfgTtsDto::default()
        };
        let cfg = apply_tts_patch(dto).unwrap();
        assert_eq!(cfg.default_voice, "de-thorsten");
        assert_eq!(cfg.voices[0].provider_voice, "de_DE-thorsten-medium");
    }

    #[test]
    // Was: Führt den Arbeitsschritt `rejects_missing_default_voice` für rejects missing default voice aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn rejects_missing_default_voice() {
        let dto = CfgTtsDto {
            enabled: true,
            default_voice: "missing".to_string(),
            voices: vec![voice()],
            ..CfgTtsDto::default()
        };
        assert!(apply_tts_patch(dto).is_err());
    }
}
