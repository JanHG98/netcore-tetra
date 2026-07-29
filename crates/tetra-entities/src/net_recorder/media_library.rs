// NETCORE-KOMMENTAR – Was: Überträgt fertige lokale Aufzeichnungen per HTTP an die zentrale Media Library.
// NETCORE-KOMMENTAR – Warum: Die Basisstation bleibt lokal aufnahmefähig, während Übernahme, Verarbeitung und NFS-Archivierung zentral erfolgen.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::sync::{Arc, Weak};
use std::time::Duration;

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use super::service::RecorderShared;
use super::types::RecordingMetadata;

#[derive(Debug, Clone, Deserialize)]
struct RemoteAsset {
    asset_id: String,
    state: String,
    approval: String,
    #[serde(default)]
    last_error: Option<String>,
}

#[derive(Debug, Serialize)]
struct ImportRequest {
    schema: &'static str,
    source: &'static str,
    source_reference: String,
    source_url: String,
    name: String,
    filename: String,
    size_bytes: u64,
    media_type: &'static str,
    kind: String,
    tags: Vec<String>,
    approve: bool,
    actor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PublishMarker {
    schema_version: u8,
    recording_id: String,
    asset_id: String,
    media_library_url: String,
    source_url: String,
    state: String,
    audio_bytes: u64,
    accepted_at: String,
    last_checked_at: String,
    ready_at: Option<String>,
    last_error: Option<String>,
}

enum PublishOutcome {
    Ready,
    Pending,
}

pub(super) fn spawn_media_library_worker(shared: &Arc<RecorderShared>, rx: Receiver<()>) {
    let weak = Arc::downgrade(shared);
    if let Err(error) = std::thread::Builder::new()
        .name("netcore-recording-media-library".to_string())
        .spawn(move || media_library_worker(weak, rx))
    {
        tracing::error!("Recorder Media Library: failed to start worker: {error}");
    }
}

fn media_library_worker(shared: Weak<RecorderShared>, rx: Receiver<()>) {
    let mut run_immediately = true;
    loop {
        let Some(inner) = shared.upgrade() else {
            return;
        };
        let retry = Duration::from_secs(inner.media_library.retry_seconds.max(1));
        drop(inner);

        if !run_immediately {
            match rx.recv_timeout(retry) {
                Ok(()) | Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) => return,
            }
        }
        run_immediately = false;

        let Some(inner) = shared.upgrade() else {
            return;
        };
        run_publish_cycle(&inner);
    }
}

pub(super) fn recording_requires_publish(
    inner: &RecorderShared,
    _metadata: &RecordingMetadata,
) -> bool {
    inner.media_library.enabled && inner.media_library.publish_recordings
}

pub(super) fn recording_is_published(
    inner: &RecorderShared,
    metadata: &RecordingMetadata,
) -> bool {
    if !recording_requires_publish(inner, metadata) {
        return true;
    }
    let Ok(audio) = source_audio_path(inner, metadata) else {
        return false;
    };
    let Ok(body) = fs::read_to_string(marker_path(&audio)) else {
        return false;
    };
    let Ok(marker) = serde_json::from_str::<PublishMarker>(&body) else {
        return false;
    };
    let audio_bytes = audio.metadata().map(|value| value.len()).ok();
    marker.schema_version == 1
        && marker.recording_id == metadata.id
        && marker.media_library_url == inner.media_library.base_url
        && marker.state == "ready"
        && Some(marker.audio_bytes) == audio_bytes
        && marker.ready_at.is_some()
}

fn run_publish_cycle(inner: &RecorderShared) {
    if !inner.media_library.enabled || !inner.media_library.publish_recordings {
        return;
    }

    inner.update_media_library_status(|status| {
        status.media_library_active = true;
    });

    let client = match Client::builder()
        .connect_timeout(Duration::from_secs(
            inner.media_library.request_timeout_seconds.max(1),
        ))
        .timeout(Duration::from_secs(
            inner.media_library.request_timeout_seconds.max(1),
        ))
        .user_agent(concat!("NetCore-TETRA-Basisstation/", env!("CARGO_PKG_VERSION")))
        .build()
    {
        Ok(client) => client,
        Err(error) => {
            inner.update_media_library_status(|status| {
                status.media_library_active = false;
                status.media_library_available = false;
                status.media_library_last_error = Some(error.to_string());
            });
            return;
        }
    };

    let mut pending = 0usize;
    let mut completed = 0usize;
    let mut available = true;
    let mut last_success = None;
    let mut last_error = None;

    for metadata in inner.scan_recordings() {
        if recording_is_published(inner, &metadata) {
            completed = completed.saturating_add(1);
            continue;
        }
        match publish_or_poll(inner, &client, &metadata) {
            Ok(PublishOutcome::Ready) => {
                completed = completed.saturating_add(1);
                last_success = Some(chrono::Local::now().to_rfc3339());
            }
            Ok(PublishOutcome::Pending) => {
                pending = pending.saturating_add(1);
            }
            Err(error) => {
                pending = pending.saturating_add(1);
                if error.contains("request failed") || error.contains("HTTP request") {
                    available = false;
                }
                last_error = Some(format!("{}: {error}", metadata.id));
                tracing::warn!(
                    "Recorder Media Library: recording id={} pending: {error}",
                    metadata.id
                );
            }
        }
    }

    inner.update_media_library_status(|status| {
        status.media_library_active = false;
        status.media_library_available = available;
        status.media_library_pending = pending;
        status.media_library_completed = completed;
        if let Some(value) = last_success {
            status.media_library_last_success_at = Some(value);
        }
        status.media_library_last_error = last_error;
    });
}

fn publish_or_poll(
    inner: &RecorderShared,
    client: &Client,
    metadata: &RecordingMetadata,
) -> Result<PublishOutcome, String> {
    let audio = source_audio_path(inner, metadata)?;
    let marker_file = marker_path(&audio);

    if let Ok(body) = fs::read_to_string(&marker_file)
        && let Ok(mut marker) = serde_json::from_str::<PublishMarker>(&body)
        && marker.recording_id == metadata.id
        && marker.media_library_url == inner.media_library.base_url
        && marker.audio_bytes == audio.metadata().map(|value| value.len()).unwrap_or(0)
    {
        match fetch_asset(inner, client, &marker.asset_id) {
            Ok(asset) if asset.state == "ready" => {
                marker.state = "ready".to_string();
                marker.last_checked_at = chrono::Local::now().to_rfc3339();
                marker.ready_at.get_or_insert_with(|| marker.last_checked_at.clone());
                marker.last_error = None;
                write_marker(&marker_file, &marker)?;
                return Ok(PublishOutcome::Ready);
            }
            Ok(asset) if asset.state == "failed" => {
                tracing::warn!(
                    "Recorder Media Library: remote asset {} failed ({}); requeueing source recording {}",
                    asset.asset_id,
                    asset.last_error.unwrap_or_else(|| "unknown error".to_string()),
                    metadata.id
                );
            }
            Ok(asset) => {
                marker.state = asset.state;
                marker.last_checked_at = chrono::Local::now().to_rfc3339();
                marker.last_error = asset.last_error;
                write_marker(&marker_file, &marker)?;
                return Ok(PublishOutcome::Pending);
            }
            Err(error) if error.contains("HTTP 404") => {
                tracing::warn!(
                    "Recorder Media Library: remote asset {} disappeared; registering recording {} again",
                    marker.asset_id,
                    metadata.id
                );
            }
            Err(error) => return Err(error),
        }
    }

    register_recording(inner, client, metadata, &audio, &marker_file)
}

fn register_recording(
    inner: &RecorderShared,
    client: &Client,
    metadata: &RecordingMetadata,
    audio: &Path,
    marker_file: &Path,
) -> Result<PublishOutcome, String> {
    let source_url = format!(
        "{}/api/media-library/recordings/{}/audio",
        inner.media_library.recording_source_base_url,
        metadata.id
    );
    let kind = if metadata
        .origin
        .as_deref()
        .is_some_and(|origin| origin.eq_ignore_ascii_case("tts"))
    {
        "tts"
    } else {
        "recording"
    };
    let title = metadata.title.clone().unwrap_or_else(|| {
        format!(
            "{} {} → {} · {}",
            if metadata.destination_type == "group" {
                "Gruppenruf"
            } else {
                "Einzelruf"
            },
            metadata.source_issi,
            metadata.destination_id,
            metadata.started_at
        )
    });
    let filename = audio
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("recording.wav")
        .to_string();
    let audio_size = audio
        .metadata()
        .map_err(|error| format!("cannot read recording size {}: {error}", audio.display()))?
        .len();
    let payload = ImportRequest {
        schema: "netcore-media-import-v1",
        source: "basisstation-recording",
        source_reference: format!("{}:{}", inner.media_library.station_id, metadata.id),
        source_url: source_url.clone(),
        name: title,
        filename,
        size_bytes: audio_size,
        media_type: "audio/wav",
        kind: kind.to_string(),
        tags: vec![
            "basisstation".to_string(),
            inner.media_library.station_id.clone(),
            metadata.destination_type.clone(),
            format!("destination:{}", metadata.destination_id),
        ],
        approve: inner.media_library.auto_approve_recordings,
        actor: format!("basisstation:{}", inner.media_library.station_id),
    };
    let endpoint = format!("{}/api/v1/assets/import-url", inner.media_library.base_url);
    let response = client
        .post(&endpoint)
        .json(&payload)
        .send()
        .map_err(|error| format!("HTTP request failed: {error}"))?;
    if !response.status().is_success() {
        let status = response.status();
        let excerpt = response.text().unwrap_or_default();
        return Err(format!(
            "Media Library rejected import with HTTP {status}: {}",
            excerpt.chars().take(500).collect::<String>()
        ));
    }
    let asset = response
        .json::<RemoteAsset>()
        .map_err(|error| format!("invalid Media Library response: {error}"))?;
    let now = chrono::Local::now().to_rfc3339();
    let ready = asset.state == "ready";
    let marker = PublishMarker {
        schema_version: 1,
        recording_id: metadata.id.clone(),
        asset_id: asset.asset_id.clone(),
        media_library_url: inner.media_library.base_url.clone(),
        source_url,
        state: asset.state,
        audio_bytes: audio_size,
        accepted_at: now.clone(),
        last_checked_at: now.clone(),
        ready_at: ready.then_some(now),
        last_error: asset.last_error,
    };
    write_marker(marker_file, &marker)?;
    tracing::info!(
        "Recorder Media Library: registered recording id={} as asset={} approval={}",
        metadata.id,
        asset.asset_id,
        asset.approval
    );
    Ok(if ready {
        PublishOutcome::Ready
    } else {
        PublishOutcome::Pending
    })
}

fn fetch_asset(
    inner: &RecorderShared,
    client: &Client,
    asset_id: &str,
) -> Result<RemoteAsset, String> {
    let endpoint = format!("{}/api/v1/assets/{asset_id}", inner.media_library.base_url);
    let response = client
        .get(endpoint)
        .send()
        .map_err(|error| format!("HTTP request failed: {error}"))?;
    if !response.status().is_success() {
        return Err(format!("asset status returned HTTP {}", response.status()));
    }
    response
        .json::<RemoteAsset>()
        .map_err(|error| format!("invalid asset status response: {error}"))
}

fn source_audio_path(
    inner: &RecorderShared,
    metadata: &RecordingMetadata,
) -> Result<PathBuf, String> {
    let relative = super::service::safe_relative_path(&metadata.relative_audio_path)?;
    let audio = inner.root.join(relative);
    let canonical_root = inner
        .root
        .canonicalize()
        .map_err(|error| format!("cannot resolve recording root: {error}"))?;
    let canonical_audio = audio
        .canonicalize()
        .map_err(|error| format!("cannot resolve recording WAV: {error}"))?;
    if !canonical_audio.starts_with(&canonical_root) || !canonical_audio.is_file() {
        return Err("recording WAV escapes the configured recording root".to_string());
    }
    Ok(canonical_audio)
}

pub(super) fn marker_path(audio: &Path) -> PathBuf {
    audio.with_extension("media-library")
}

fn write_marker(path: &Path, marker: &PublishMarker) -> Result<(), String> {
    let bytes = serde_json::to_vec_pretty(marker).map_err(|error| error.to_string())?;
    let temporary = PathBuf::from(format!("{}.tmp", path.display()));
    fs::write(&temporary, bytes)
        .map_err(|error| format!("cannot write {}: {error}", temporary.display()))?;
    fs::rename(&temporary, path).map_err(|error| {
        format!(
            "cannot publish Media Library marker {} -> {}: {error}",
            temporary.display(),
            path.display()
        )
    })
}
