// NETCORE-KOMMENTAR – Was: Konfiguriert die HTTP-Anbindung einer Basisstation an die zentrale Media Library.
// NETCORE-KOMMENTAR – Warum: Recordings werden zuverlässig zentral übernommen und freigegebene Medien lokal gecacht ausgesendet.

use std::collections::HashMap;

use serde::Deserialize;
use toml::Value;

/// Bidirectional Base Station ↔ Media Library integration.
#[derive(Debug, Clone)]
pub struct CfgMediaLibrary {
    /// Master switch for the integration.
    pub enabled: bool,
    /// Media Library HTTP base URL, for example `http://10.0.1.154:8230`.
    pub base_url: String,
    /// Stable station label written into tags and audit actor fields.
    pub station_id: String,
    /// Publish completed local recordings to the Media Library.
    pub publish_recordings: bool,
    /// Publicly reachable base URL of this base station dashboard/export endpoint.
    /// The Media Library appends `/api/media-library/recordings/{id}/audio`.
    pub recording_source_base_url: String,
    /// Let newly imported recordings immediately appear as approved assets.
    pub auto_approve_recordings: bool,
    /// Expose the Media Library as a source in the Audio Centre.
    pub audio_source_enabled: bool,
    /// Only list assets whose processing state is `ready`.
    pub only_ready: bool,
    /// Only list assets whose approval state is `approved`.
    pub only_approved: bool,
    /// Retry/poll interval for pending recording transfers.
    pub retry_seconds: u64,
    /// Timeout for metadata/API requests.
    pub request_timeout_seconds: u64,
    /// Timeout for media downloads into the local playout cache.
    pub download_timeout_seconds: u64,
    /// Maximum number of assets shown by the base-station media browser.
    pub max_list_entries: usize,
}

impl Default for CfgMediaLibrary {
    fn default() -> Self {
        apply_media_library_patch(CfgMediaLibraryDto::default())
            .expect("default media-library integration config must be valid")
    }
}

#[derive(Debug, Deserialize)]
pub struct CfgMediaLibraryDto {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub base_url: String,
    #[serde(default = "default_station_id")]
    pub station_id: String,
    #[serde(default)]
    pub publish_recordings: bool,
    #[serde(default)]
    pub recording_source_base_url: String,
    #[serde(default)]
    pub auto_approve_recordings: bool,
    #[serde(default)]
    pub audio_source_enabled: bool,
    #[serde(default = "default_true")]
    pub only_ready: bool,
    #[serde(default = "default_true")]
    pub only_approved: bool,
    #[serde(default = "default_retry_seconds")]
    pub retry_seconds: u64,
    #[serde(default = "default_request_timeout_seconds")]
    pub request_timeout_seconds: u64,
    #[serde(default = "default_download_timeout_seconds")]
    pub download_timeout_seconds: u64,
    #[serde(default = "default_max_list_entries")]
    pub max_list_entries: usize,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl Default for CfgMediaLibraryDto {
    fn default() -> Self {
        Self {
            enabled: false,
            base_url: String::new(),
            station_id: default_station_id(),
            publish_recordings: false,
            recording_source_base_url: String::new(),
            auto_approve_recordings: false,
            audio_source_enabled: false,
            only_ready: true,
            only_approved: true,
            retry_seconds: default_retry_seconds(),
            request_timeout_seconds: default_request_timeout_seconds(),
            download_timeout_seconds: default_download_timeout_seconds(),
            max_list_entries: default_max_list_entries(),
            extra: HashMap::new(),
        }
    }
}

fn default_true() -> bool {
    true
}

fn default_station_id() -> String {
    "basisstation".to_string()
}

fn default_retry_seconds() -> u64 {
    60
}

fn default_request_timeout_seconds() -> u64 {
    15
}

fn default_download_timeout_seconds() -> u64 {
    120
}

fn default_max_list_entries() -> usize {
    1_000
}

fn valid_http_url(value: &str) -> bool {
    value.starts_with("http://") || value.starts_with("https://")
}

fn valid_station_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 96
        && value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':' | b' ')
        })
}

pub fn apply_media_library_patch(mut src: CfgMediaLibraryDto) -> Result<CfgMediaLibrary, String> {
    src.base_url = src.base_url.trim().trim_end_matches('/').to_string();
    src.recording_source_base_url = src
        .recording_source_base_url
        .trim()
        .trim_end_matches('/')
        .to_string();
    src.station_id = src.station_id.trim().to_string();

    if src.enabled && !valid_http_url(&src.base_url) {
        return Err("media_library: base_url must use http:// or https:// when enabled".to_string());
    }
    if src.enabled && src.publish_recordings && !valid_http_url(&src.recording_source_base_url) {
        return Err(
            "media_library: recording_source_base_url must use http:// or https:// when publish_recordings=true"
                .to_string(),
        );
    }
    if !valid_station_id(&src.station_id) {
        return Err(
            "media_library: station_id must be 1-96 characters and contain only letters, numbers, space, '.', '-', '_', or ':'"
                .to_string(),
        );
    }
    if src.retry_seconds == 0 {
        return Err("media_library: retry_seconds must be greater than zero".to_string());
    }
    if src.request_timeout_seconds == 0 {
        return Err("media_library: request_timeout_seconds must be greater than zero".to_string());
    }
    if src.download_timeout_seconds == 0 {
        return Err("media_library: download_timeout_seconds must be greater than zero".to_string());
    }
    if src.max_list_entries == 0 {
        return Err("media_library: max_list_entries must be greater than zero".to_string());
    }

    Ok(CfgMediaLibrary {
        enabled: src.enabled,
        base_url: src.base_url,
        station_id: src.station_id,
        publish_recordings: src.publish_recordings,
        recording_source_base_url: src.recording_source_base_url,
        auto_approve_recordings: src.auto_approve_recordings,
        audio_source_enabled: src.audio_source_enabled,
        only_ready: src.only_ready,
        only_approved: src.only_approved,
        retry_seconds: src.retry_seconds,
        request_timeout_seconds: src.request_timeout_seconds,
        download_timeout_seconds: src.download_timeout_seconds,
        max_list_entries: src.max_list_entries.min(5_000),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_disabled() {
        let config = CfgMediaLibrary::default();
        assert!(!config.enabled);
        assert!(!config.publish_recordings);
        assert!(!config.audio_source_enabled);
    }

    #[test]
    fn validates_enabled_bidirectional_config() {
        let dto = CfgMediaLibraryDto {
            enabled: true,
            base_url: "http://10.0.1.154:8230/".to_string(),
            station_id: "SRV-M-TBS-01".to_string(),
            publish_recordings: true,
            recording_source_base_url: "http://10.0.1.163:8081/".to_string(),
            audio_source_enabled: true,
            ..CfgMediaLibraryDto::default()
        };
        let config = apply_media_library_patch(dto).unwrap();
        assert_eq!(config.base_url, "http://10.0.1.154:8230");
        assert_eq!(config.recording_source_base_url, "http://10.0.1.163:8081");
    }
}
