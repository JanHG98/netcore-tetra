// NetCore Media Library: central Piper TTS generation and template management.

use std::collections::BTreeSet;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use base64::Engine;
use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::config::{TtsConfig, TtsVoiceConfig};
use crate::media;
use crate::model::{AssetRecord, UploadInput};
use crate::state::SharedLibrary;

const TEMPLATE_SUFFIX: &str = ".tts.toml";
const TEMPLATE_SCHEMA_VERSION: u32 = 1;
const MAX_TEMPLATE_BYTES: u64 = 256 * 1024;

#[derive(Debug, Clone, Serialize)]
pub struct TtsVoiceStatus {
    pub id: String,
    pub name: String,
    pub provider_voice: String,
    pub speaker_id: Option<u32>,
    pub available: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TtsStatus {
    pub enabled: bool,
    pub provider_available: bool,
    pub provider_endpoint: String,
    pub provider_error: Option<String>,
    pub default_voice: String,
    pub default_speed: f32,
    pub auto_approve_tts: bool,
    pub max_text_characters: usize,
    pub template_directory: String,
    pub template_available: bool,
    pub template_error: Option<String>,
    pub voices: Vec<TtsVoiceStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsTemplate {
    pub schema_version: u32,
    pub id: String,
    pub name: String,
    pub text: String,
    pub voice_id: String,
    pub speed: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TtsTemplateInput {
    pub id: Option<String>,
    pub name: String,
    pub text: String,
    pub voice_id: String,
    pub speed: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TtsTemplateDeleteInput {
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TtsGenerateInput {
    pub name: String,
    pub text: String,
    pub voice_id: String,
    pub speed: f32,
    pub approve: Option<bool>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub actor: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TtsGenerateResult {
    pub asset: AssetRecord,
    pub provider_voice: String,
    pub bytes: usize,
}

struct TtsInner {
    config: TtsConfig,
    auto_approve_tts: bool,
    client: Client,
    template_root: Option<PathBuf>,
    template_error: Option<String>,
    template_lock: Mutex<()>,
}

#[derive(Clone)]
pub struct TtsService {
    inner: Arc<TtsInner>,
}

impl TtsService {
    pub fn new(config: TtsConfig, auto_approve_tts: bool) -> Result<Self, String> {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(config.synthesis_timeout_secs.max(5)))
            .build()
            .map_err(|error| format!("cannot create Piper HTTP client: {error}"))?;
        let (template_root, template_error) = if config.enabled {
            match prepare_template_directory(&config.template_directory) {
                Ok(path) => (Some(path), None),
                Err(error) => (None, Some(error)),
            }
        } else {
            (None, None)
        };
        Ok(Self {
            inner: Arc::new(TtsInner {
                config,
                auto_approve_tts,
                client,
                template_root,
                template_error,
                template_lock: Mutex::new(()),
            }),
        })
    }

    pub fn status(&self) -> TtsStatus {
        let (provider_available, provider_error, installed) = match self.installed_provider_voices() {
            Ok(voices) => (true, None, voices),
            Err(error) => (false, Some(error), BTreeSet::new()),
        };
        let voices = self
            .inner
            .config
            .voices
            .iter()
            .map(|voice| voice_status(voice, &installed, provider_available))
            .collect();
        TtsStatus {
            enabled: self.inner.config.enabled,
            provider_available: self.inner.config.enabled && provider_available,
            provider_endpoint: self.inner.config.endpoint.clone(),
            provider_error,
            default_voice: self.inner.config.default_voice.clone(),
            default_speed: self.inner.config.default_speed,
            auto_approve_tts: self.inner.auto_approve_tts,
            max_text_characters: self.inner.config.max_text_characters,
            template_directory: self.inner.config.template_directory.display().to_string(),
            template_available: self.inner.template_root.is_some(),
            template_error: self.inner.template_error.clone(),
            voices,
        }
    }

    pub fn voices(&self) -> Vec<TtsVoiceStatus> {
        self.status().voices
    }

    pub fn templates(&self) -> Result<Vec<TtsTemplate>, String> {
        self.require_enabled()?;
        let root = self.template_root()?;
        let _guard = self.inner.template_lock.lock().unwrap_or_else(|p| p.into_inner());
        list_templates(root)
    }

    pub fn save_template(&self, input: TtsTemplateInput) -> Result<TtsTemplate, String> {
        self.require_enabled()?;
        let text = self.normalise_text(&input.text)?;
        let _ = self.voice(&input.voice_id)?;
        validate_speed(input.speed)?;
        let name = validate_name(&input.name)?;
        let root = self.template_root()?;
        let _guard = self.inner.template_lock.lock().unwrap_or_else(|p| p.into_inner());
        let id = match input.id.as_deref().map(str::trim).filter(|value| !value.is_empty()) {
            Some(id) => validate_template_id(id)?.to_string(),
            None => Uuid::new_v4().to_string(),
        };
        let previous = read_template(root, &id).ok();
        let now = Utc::now();
        let template = TtsTemplate {
            schema_version: TEMPLATE_SCHEMA_VERSION,
            id,
            name,
            text,
            voice_id: input.voice_id.trim().to_string(),
            speed: input.speed,
            created_at: previous
                .as_ref()
                .map(|value| value.created_at.clone())
                .unwrap_or(now),
            updated_at: now,
        };
        write_template(root, &template)?;
        Ok(template)
    }

    pub fn delete_template(&self, id: &str) -> Result<(), String> {
        self.require_enabled()?;
        let id = validate_template_id(id.trim())?;
        let root = self.template_root()?;
        let _guard = self.inner.template_lock.lock().unwrap_or_else(|p| p.into_inner());
        let path = template_path(root, id);
        match fs::remove_file(&path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                Err(format!("TTS template '{id}' does not exist"))
            }
            Err(error) => Err(format!("cannot delete TTS template {}: {error}", path.display())),
        }
    }

    pub fn generate(
        &self,
        library: &SharedLibrary,
        input: TtsGenerateInput,
    ) -> Result<TtsGenerateResult, String> {
        self.require_enabled()?;
        let name = validate_name(&input.name)?;
        let text = self.normalise_text(&input.text)?;
        validate_speed(input.speed)?;
        let voice = self.voice(&input.voice_id)?;
        let installed = self.installed_provider_voices()?;
        if !installed.contains(&voice.provider_voice) {
            return Err(format!(
                "Piper voice model '{}' is not installed",
                voice.provider_voice
            ));
        }
        let bytes = self.synthesize(&text, voice, input.speed)?;
        let approve = input.approve.unwrap_or(self.inner.auto_approve_tts);
        let mut tags = input.tags;
        tags.extend([
            "tts".to_string(),
            "piper".to_string(),
            voice.id.clone(),
        ]);
        tags.sort();
        tags.dedup();
        let filename = format!("{}.wav", media::safe_filename(&name, "tts"));
        let asset = library.create_upload(UploadInput {
            name,
            filename,
            media_type: Some("audio/wav".to_string()),
            kind: Some("tts".to_string()),
            description: Some("Mit Piper in der NetCore Media Library erzeugte Textdurchsage".to_string()),
            tags,
            data_base64: base64::engine::general_purpose::STANDARD.encode(&bytes),
            approve: Some(approve),
            actor: input.actor.or_else(|| Some("media-library-tts".to_string())),
            broadcast: None,
            source: Some("piper_tts".to_string()),
            source_reference: Some(Uuid::new_v4().to_string()),
            voice: Some(voice.id.clone()),
            text: Some(text),
        })?;
        Ok(TtsGenerateResult {
            asset,
            provider_voice: voice.provider_voice.clone(),
            bytes: bytes.len(),
        })
    }

    fn synthesize(&self, text: &str, voice: &TtsVoiceConfig, speed: f32) -> Result<Vec<u8>, String> {
        let max_bytes = self.inner.config.max_output_file_mb.saturating_mul(1024 * 1024);
        let mut payload = json!({
            "text": text,
            "voice": voice.provider_voice,
            "length_scale": 1.0_f32 / speed,
        });
        if let Some(speaker_id) = voice.speaker_id {
            payload["speaker_id"] = json!(speaker_id);
        }
        let url = format!("{}/synthesize", piper_base_url(&self.inner.config.endpoint));
        let mut response = self
            .inner
            .client
            .post(url)
            .json(&payload)
            .send()
            .map_err(|error| format!("Piper HTTP request failed: {error}"))?;
        if !response.status().is_success() {
            let status = response.status();
            let mut body = String::new();
            let _ = response.by_ref().take(4096).read_to_string(&mut body);
            return Err(if body.trim().is_empty() {
                format!("Piper returned HTTP {status}")
            } else {
                format!("Piper returned HTTP {status}: {}", body.trim())
            });
        }
        if response.content_length().is_some_and(|length| length > max_bytes) {
            return Err(format!(
                "Piper output exceeds {} MiB",
                self.inner.config.max_output_file_mb
            ));
        }
        let mut bytes = Vec::new();
        response
            .take(max_bytes.saturating_add(1))
            .read_to_end(&mut bytes)
            .map_err(|error| format!("cannot read Piper WAV: {error}"))?;
        if bytes.len() as u64 > max_bytes {
            return Err(format!(
                "Piper output exceeds {} MiB",
                self.inner.config.max_output_file_mb
            ));
        }
        validate_wav_bytes(&bytes)?;
        Ok(bytes)
    }

    fn installed_provider_voices(&self) -> Result<BTreeSet<String>, String> {
        self.require_enabled()?;
        let url = format!("{}/voices", piper_base_url(&self.inner.config.endpoint));
        let value = self
            .inner
            .client
            .get(url)
            .send()
            .map_err(|error| format!("Piper provider unavailable: {error}"))?
            .error_for_status()
            .map_err(|error| format!("Piper /voices failed: {error}"))?
            .json::<Value>()
            .map_err(|error| format!("Piper /voices returned invalid JSON: {error}"))?;
        let object = value
            .as_object()
            .ok_or_else(|| "Piper /voices did not return a JSON object".to_string())?;
        Ok(object.keys().cloned().collect())
    }

    fn voice(&self, id: &str) -> Result<&TtsVoiceConfig, String> {
        let id = id.trim();
        self.inner
            .config
            .voices
            .iter()
            .find(|voice| voice.id == id)
            .ok_or_else(|| format!("unknown TTS voice '{id}'"))
    }

    fn normalise_text(&self, text: &str) -> Result<String, String> {
        let text = text
            .replace("\r\n", "\n")
            .replace('\r', "\n")
            .chars()
            .filter(|character| !character.is_control() || matches!(character, '\n' | '\t'))
            .collect::<String>();
        let text = text.trim().to_string();
        let count = text.chars().count();
        if count == 0 || count > self.inner.config.max_text_characters {
            return Err(format!(
                "TTS text must contain 1-{} characters",
                self.inner.config.max_text_characters
            ));
        }
        Ok(text)
    }

    fn template_root(&self) -> Result<&Path, String> {
        self.inner.template_root.as_deref().ok_or_else(|| {
            self.inner
                .template_error
                .clone()
                .unwrap_or_else(|| "TTS template storage is unavailable".to_string())
        })
    }

    fn require_enabled(&self) -> Result<(), String> {
        if self.inner.config.enabled {
            Ok(())
        } else {
            Err("Media Library TTS is disabled".to_string())
        }
    }
}

fn voice_status(
    voice: &TtsVoiceConfig,
    installed: &BTreeSet<String>,
    provider_available: bool,
) -> TtsVoiceStatus {
    let available = provider_available && installed.contains(&voice.provider_voice);
    TtsVoiceStatus {
        id: voice.id.clone(),
        name: voice.name.clone(),
        provider_voice: voice.provider_voice.clone(),
        speaker_id: voice.speaker_id,
        available,
        error: (!available).then(|| {
            if provider_available {
                format!("Piper voice model '{}' is not installed", voice.provider_voice)
            } else {
                "Piper provider is unavailable".to_string()
            }
        }),
    }
}

fn piper_base_url(endpoint: &str) -> String {
    endpoint
        .trim()
        .trim_end_matches('/')
        .strip_suffix("/synthesize")
        .unwrap_or(endpoint.trim().trim_end_matches('/'))
        .to_string()
}

fn validate_name(value: &str) -> Result<String, String> {
    let value = value.trim();
    let count = value.chars().count();
    if count == 0 || count > 120 {
        return Err("name must contain 1-120 characters".to_string());
    }
    Ok(value.to_string())
}

fn validate_speed(value: f32) -> Result<(), String> {
    if (0.5..=1.5).contains(&value) {
        Ok(())
    } else {
        Err("TTS speed must be between 0.50 and 1.50".to_string())
    }
}

fn validate_template_id(value: &str) -> Result<&str, String> {
    if value.len() <= 80
        && !value.is_empty()
        && value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '-')
    {
        Ok(value)
    } else {
        Err("invalid TTS template id".to_string())
    }
}

fn prepare_template_directory(path: &Path) -> Result<PathBuf, String> {
    fs::create_dir_all(path)
        .map_err(|error| format!("cannot create TTS template directory {}: {error}", path.display()))?;
    let canonical = path
        .canonicalize()
        .map_err(|error| format!("cannot canonicalize TTS template directory {}: {error}", path.display()))?;
    if !canonical.is_dir() {
        return Err(format!("{} is not a directory", canonical.display()));
    }
    let probe = canonical.join(format!(".write-probe-{}", Uuid::new_v4()));
    fs::write(&probe, b"netcore-media-library-tts")
        .map_err(|error| format!("TTS template directory {} is not writable: {error}", canonical.display()))?;
    fs::remove_file(&probe)
        .map_err(|error| format!("cannot remove TTS template probe {}: {error}", probe.display()))?;
    Ok(canonical)
}

fn template_path(root: &Path, id: &str) -> PathBuf {
    root.join(format!("{id}{TEMPLATE_SUFFIX}"))
}

fn list_templates(root: &Path) -> Result<Vec<TtsTemplate>, String> {
    let mut templates = Vec::new();
    for entry in fs::read_dir(root)
        .map_err(|error| format!("cannot read TTS template directory {}: {error}", root.display()))?
    {
        let entry = entry.map_err(|error| format!("cannot read TTS template entry: {error}"))?;
        if !entry.file_type().map(|value| value.is_file()).unwrap_or(false)
            || !entry.file_name().to_string_lossy().ends_with(TEMPLATE_SUFFIX)
        {
            continue;
        }
        match read_template_file(&entry.path()) {
            Ok(template) => templates.push(template),
            Err(error) => tracing::warn!("Ignoring invalid TTS template {}: {}", entry.path().display(), error),
        }
    }
    templates.sort_by(|left, right| left.name.to_lowercase().cmp(&right.name.to_lowercase()));
    Ok(templates)
}

fn read_template(root: &Path, id: &str) -> Result<TtsTemplate, String> {
    read_template_file(&template_path(root, id))
}

fn read_template_file(path: &Path) -> Result<TtsTemplate, String> {
    let metadata = fs::metadata(path)
        .map_err(|error| format!("cannot inspect TTS template {}: {error}", path.display()))?;
    if !metadata.is_file() || metadata.len() > MAX_TEMPLATE_BYTES {
        return Err("invalid TTS template file".to_string());
    }
    let source = fs::read_to_string(path)
        .map_err(|error| format!("cannot read TTS template {}: {error}", path.display()))?;
    let template: TtsTemplate = toml::from_str(&source)
        .map_err(|error| format!("cannot parse TTS template {}: {error}", path.display()))?;
    validate_template_id(&template.id)?;
    if template.schema_version != TEMPLATE_SCHEMA_VERSION {
        return Err(format!("unsupported TTS template schema {}", template.schema_version));
    }
    validate_name(&template.name)?;
    validate_speed(template.speed)?;
    if template.text.trim().is_empty() {
        return Err("TTS template text is empty".to_string());
    }
    Ok(template)
}

fn write_template(root: &Path, template: &TtsTemplate) -> Result<(), String> {
    let final_path = template_path(root, &template.id);
    let temp_path = root.join(format!(".{}.{}.tmp", template.id, Uuid::new_v4()));
    let body = toml::to_string_pretty(template)
        .map_err(|error| format!("cannot serialize TTS template: {error}"))?;
    fs::write(&temp_path, body)
        .map_err(|error| format!("cannot write TTS template {}: {error}", temp_path.display()))?;
    fs::rename(&temp_path, &final_path)
        .map_err(|error| format!("cannot install TTS template {}: {error}", final_path.display()))
}

fn validate_wav_bytes(bytes: &[u8]) -> Result<(), String> {
    if bytes.len() < 44 || &bytes[..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return Err("Piper response is not a valid RIFF/WAVE file".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn piper_endpoint_is_normalised() {
        assert_eq!(piper_base_url("http://127.0.0.1:5005/synthesize"), "http://127.0.0.1:5005");
        assert_eq!(piper_base_url("http://127.0.0.1:5005/"), "http://127.0.0.1:5005");
    }

    #[test]
    fn wav_header_is_checked() {
        let mut bytes = vec![0u8; 44];
        bytes[..4].copy_from_slice(b"RIFF");
        bytes[8..12].copy_from_slice(b"WAVE");
        assert!(validate_wav_bytes(&bytes).is_ok());
        assert!(validate_wav_bytes(b"not wav").is_err());
    }
}
