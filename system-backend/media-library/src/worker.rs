// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für gespeicherte Aufzeichnungen, TTS- und Mediendateien.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::thread;
use std::time::{Duration, Instant};

use reqwest::blocking::{Client, Response};
use reqwest::header::{COOKIE, SET_COOKIE};
use serde_json::{json, Value};

use crate::config::{
    BASISSTATION_PLAYOUT_MODE, MEDIA_SWITCH_PLAYOUT_MODE, MediaLibraryConfig,
    PlayoutStationConfig,
};
use crate::media;
use crate::model::{ActionInput, DispatchClaim, ImportClaim};
use crate::state::SharedLibrary;

// Was: Diese Funktion startet Hintergrundverarbeitung.
// Warum: Länger laufende Arbeit blockiert dadurch nicht den aufrufenden Ablauf.
pub fn spawn_worker(
    config: MediaLibraryConfig,
    library: SharedLibrary,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        let client = match build_client(&config) {
            Ok(client) => client,
            Err(error) => {
                tracing::error!("Media Library cannot build HTTP client: {error}");
                return;
            }
        };
        let mut last_probe = Instant::now()
            .checked_sub(Duration::from_secs(config.runtime.probe_interval_secs))
            .unwrap_or_else(Instant::now);
        let mut last_maintenance = Instant::now();
        // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
        // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
        loop {
            run_cycle(&config, &library, &client, &mut last_probe, &mut last_maintenance);
            thread::sleep(Duration::from_millis(config.runtime.worker_interval_ms));
        }
    })
}

// Was: Diese Funktion erstellt client.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_client(config: &MediaLibraryConfig) -> Result<Client, String> {
    Client::builder()
        .timeout(Duration::from_secs(config.runtime.import_timeout_secs))
        .user_agent(format!("netcore-media-library/{}", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(|error| error.to_string())
}

// Was: Diese Funktion führt cycle.
// Warum: Der Lebenszyklus des Dienstes bleibt so an einer zentralen Stelle steuerbar.
pub fn run_cycle(
    config: &MediaLibraryConfig,
    library: &SharedLibrary,
    client: &Client,
    last_probe: &mut Instant,
    last_maintenance: &mut Instant,
) {
    if last_probe.elapsed() >= Duration::from_secs(config.runtime.probe_interval_secs) {
        probe_dependencies(config, library, client);
        *last_probe = Instant::now();
    }

    if let Some(claim) = library.claim_import() {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match download_import(config, client, &claim)
            .and_then(|bytes| library.complete_import(&claim, &bytes).map(|_| ()))
        {
            Ok(()) => tracing::info!(asset_id = %claim.asset_id, "Media import completed"),
            Err(error) => {
                tracing::warn!(asset_id = %claim.asset_id, "Media import failed: {error}");
                library.fail_import(&claim.asset_id, error);
            }
        }
    }

    if let Some(claim) = library.claim_processing() {
        let asset_id = claim.asset.asset_id.clone();
        let result = claim
            .asset
            .original_path
            .as_ref()
            .ok_or_else(|| "asset has no original path".to_string())
            .and_then(|original| {
                let directory = original
                    .parent()
                    .ok_or_else(|| "asset original path has no parent".to_string())?;
                media::process_asset(config, original, directory, &claim.asset.media_type)
            });
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match result.and_then(|result| library.complete_processing(&asset_id, result)) {
            Ok(asset) => {
                tracing::info!(asset_id = %asset_id, "Media processing completed");
                let auto_archive = (asset.kind == "recording"
                    && config.runtime.auto_archive_recordings
                    && config.storage.recording_archive_root.is_some())
                    || (asset.kind == "tts"
                        && config.runtime.auto_archive_tts
                        && config.storage.tts_archive_root.is_some());
                if auto_archive && !asset.archived {
                    if let Err(error) = library.archive_asset(
                        &asset_id,
                        ActionInput {
                            actor: Some("worker:auto-archive".to_string()),
                        },
                    ) {
                        // The live asset is already ready. A missing NAS therefore
                        // never breaks playout; maintenance or a later manual action
                        // can archive it after storage returns.
                        tracing::warn!(asset_id = %asset_id, "Automatic Media Library archive failed: {error}");
                    }
                }
            }
            Err(error) => {
                tracing::warn!(asset_id = %asset_id, "Media processing failed: {error}");
                library.fail_processing(&asset_id, error);
            }
        }
    }

    // Retry automatic archiving independently from import/processing. A temporary
    // NFS outage therefore leaves the live asset ready and is healed once the
    // archive mount returns. Limit this to one asset per worker cycle.
    if config.storage.archive_root.is_some()
        || config.storage.recording_archive_root.is_some()
        || config.storage.tts_archive_root.is_some()
    {
        let pending_archive = library
            .assets(None, None, Some("ready"), None, config.runtime.max_assets)
            .into_iter()
            .find(|asset| {
                !asset.archived
                    && ((asset.kind == "recording"
                        && config.runtime.auto_archive_recordings)
                        || (asset.kind == "tts" && config.runtime.auto_archive_tts))
            });
        if let Some(asset) = pending_archive {
            if let Err(error) = library.archive_asset(
                &asset.asset_id,
                ActionInput {
                    actor: Some("worker:auto-archive-retry".to_string()),
                },
            ) {
                tracing::debug!(
                    asset_id = %asset.asset_id,
                    "Automatic Media Library archive still pending: {error}"
                );
            }
        }
    }

    if let Some(claim) = library.claim_dispatch() {
        if let Err(error) = play_dispatch(config, library, client, &claim) {
            tracing::warn!(job_id = %claim.job.job_id, "Media playout failed: {error}");
            library.fail_dispatch(&claim.job.job_id, error);
        }
    }

    if last_maintenance.elapsed() >= Duration::from_secs(60) {
        let _ = library.maintenance(Some("worker".to_string()));
        *last_maintenance = Instant::now();
    }
}

// Was: Führt den Arbeitsschritt `download_import` für download import aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn download_import(
    config: &MediaLibraryConfig,
    client: &Client,
    claim: &ImportClaim,
) -> Result<Vec<u8>, String> {
    let response = client
        .get(&claim.source_url)
        .send()
        .map_err(|error| format!("source request failed: {error}"))?;
    let response = require_success(response)?;
    if let Some(length) = response.content_length()
        && length > config.storage.max_asset_bytes
    {
        return Err(format!(
            "source Content-Length {length} exceeds {} byte limit",
            config.storage.max_asset_bytes
        ));
    }
    let mut limited = response.take(config.storage.max_asset_bytes.saturating_add(1));
    let mut bytes = Vec::new();
    limited
        .read_to_end(&mut bytes)
        .map_err(|error| format!("cannot read source response: {error}"))?;
    if bytes.len() as u64 > config.storage.max_asset_bytes {
        return Err("source exceeded configured asset size while downloading".to_string());
    }
    if bytes.is_empty() {
        return Err("source returned an empty body".to_string());
    }
    Ok(bytes)
}

// Was: Führt den Arbeitsschritt `require_success` für require success aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn require_success(response: Response) -> Result<Response, String> {
    if response.status().is_success() {
        Ok(response)
    } else {
        Err(format!("source returned HTTP {}", response.status()))
    }
}

// Was: Führt den Arbeitsschritt `play_dispatch` für play dispatch aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn play_dispatch(
    config: &MediaLibraryConfig,
    library: &SharedLibrary,
    client: &Client,
    claim: &DispatchClaim,
) -> Result<(), String> {
    match claim.job.playout_mode.as_str() {
        BASISSTATION_PLAYOUT_MODE => play_via_basisstation(config, library, client, claim),
        MEDIA_SWITCH_PLAYOUT_MODE => play_via_media_switch(config, library, client, claim),
        value => Err(format!("unsupported playout mode '{value}'")),
    }
}

fn play_via_basisstation(
    config: &MediaLibraryConfig,
    library: &SharedLibrary,
    client: &Client,
    claim: &DispatchClaim,
) -> Result<(), String> {
    let station_id = claim
        .job
        .target_node
        .as_deref()
        .ok_or_else(|| "basis-station playout job has no station_id".to_string())?;
    let station = config
        .playout
        .stations
        .iter()
        .find(|station| station.id == station_id && station.enabled)
        .ok_or_else(|| format!("basis-station playout target '{station_id}' is missing or disabled"))?;
    let destination_kind = claim
        .job
        .destination_kind
        .as_deref()
        .filter(|value| matches!(*value, "group" | "individual"))
        .ok_or_else(|| "basis-station playout requires destination_kind group or individual".to_string())?;
    let destination_id = claim
        .job
        .destination_id
        .filter(|value| *value > 0 && *value <= 0x00ff_ffff)
        .ok_or_else(|| "basis-station playout requires a valid 24-bit destination_id".to_string())?;

    let status_endpoint = format!("{}/api/audio/status", station.base_url);
    let play_endpoint = format!("{}/api/audio/play", station.base_url);
    let stop_endpoint = format!("{}/api/audio/stop", station.base_url);
    let session_cookie = station_session_cookie(
        client,
        station,
        config.playout.request_timeout_secs,
    )?;
    let initial_status = station_get_json(
        client,
        station,
        &status_endpoint,
        config.playout.request_timeout_secs,
        session_cookie.as_deref(),
    )?;
    if !initial_status
        .get("available")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        return Err(format!(
            "basis station '{}' reports that the audio player is unavailable: {}",
            station.name,
            initial_status
                .get("last_error")
                .and_then(Value::as_str)
                .unwrap_or("no detail")
        ));
    }
    let initial_state = initial_status
        .get("state")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    if !matches!(initial_state, "idle" | "failed") {
        return Err(format!(
            "basis station '{}' audio player is busy ({initial_state})",
            station.name
        ));
    }

    let response = station_request(
        client.post(&play_endpoint),
        config.playout.request_timeout_secs,
        session_cookie.as_deref(),
    )
    .json(&json!({
        "source_type":"media",
        "source_id":"media-library",
        "path":claim.job.asset_id,
        "target_type":destination_kind,
        "target_id":destination_id,
        "priority":claim.job.priority,
    }))
    .send()
    .map_err(|error| format!("basis station '{}' playout request failed: {error}", station.name))?;
    if !response.status().is_success() {
        let status = response.status();
        let detail = response.text().unwrap_or_default();
        return Err(format!(
            "basis station '{}' rejected playout with HTTP {status}: {}",
            station.name,
            detail.chars().take(500).collect::<String>()
        ));
    }
    let accepted = response
        .json::<Value>()
        .map_err(|error| format!("basis station '{}' returned invalid playout JSON: {error}", station.name))?;
    let remote_job_id = accepted
        .get("job_id")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("basis station '{}' response contains no job_id", station.name))?
        .to_string();
    library.dispatch_remote_started(&claim.job.job_id, remote_job_id.clone(), None);

    let started = Instant::now();
    let timeout = Duration::from_secs(config.playout.completion_timeout_secs);
    let poll_interval = Duration::from_millis(config.playout.poll_interval_ms);
    let mut observed_remote_job = false;
    let mut consecutive_idle_polls = 0u8;

    loop {
        if library.dispatch_cancel_requested(&claim.job.job_id) {
            let _ = station_request(
                client.post(&stop_endpoint),
                config.playout.request_timeout_secs,
                session_cookie.as_deref(),
            )
            .send();
            library.complete_dispatch(&claim.job.job_id);
            return Ok(());
        }
        if started.elapsed() >= timeout {
            let _ = station_request(
                client.post(&stop_endpoint),
                config.playout.request_timeout_secs,
                session_cookie.as_deref(),
            )
            .send();
            return Err(format!(
                "basis station '{}' playout timed out after {} seconds",
                station.name, config.playout.completion_timeout_secs
            ));
        }

        thread::sleep(poll_interval);
        let status = station_get_json(
            client,
            station,
            &status_endpoint,
            config.playout.request_timeout_secs,
            session_cookie.as_deref(),
        )?;
        let state = status
            .get("state")
            .and_then(Value::as_str)
            .unwrap_or("unknown");
        let current_remote_job = status.get("job_id").and_then(Value::as_str);
        let sent_blocks = status
            .get("sent_blocks")
            .and_then(Value::as_u64)
            .unwrap_or(0);
        let total_blocks = status
            .get("total_blocks")
            .and_then(Value::as_u64)
            .unwrap_or(0);

        if current_remote_job == Some(remote_job_id.as_str()) {
            observed_remote_job = true;
            consecutive_idle_polls = 0;
            library.dispatch_remote_progress(&claim.job.job_id, sent_blocks, total_blocks);
            if state == "failed" {
                return Err(format!(
                    "basis station '{}' playout failed: {}",
                    station.name,
                    status
                        .get("last_error")
                        .and_then(Value::as_str)
                        .unwrap_or("no detail")
                ));
            }
            continue;
        }

        if state == "idle" && current_remote_job.is_none() {
            consecutive_idle_polls = consecutive_idle_polls.saturating_add(1);
            // A very short clip may complete between the POST response and the first poll.
            // Two idle observations avoid treating a transient status reset as success.
            if observed_remote_job || consecutive_idle_polls >= 2 {
                library.complete_dispatch(&claim.job.job_id);
                return Ok(());
            }
            continue;
        }

        if let Some(other_job) = current_remote_job {
            return Err(format!(
                "basis station '{}' switched from remote job {} to unrelated job {}",
                station.name, remote_job_id, other_job
            ));
        }
        if state == "failed" {
            return Err(format!(
                "basis station '{}' audio player failed: {}",
                station.name,
                status
                    .get("last_error")
                    .and_then(Value::as_str)
                    .unwrap_or("no detail")
            ));
        }
    }
}

fn station_session_cookie(
    client: &Client,
    station: &PlayoutStationConfig,
    timeout_secs: u64,
) -> Result<Option<String>, String> {
    let (Some(username), Some(password)) = (&station.username, &station.password) else {
        return Ok(None);
    };
    let endpoint = format!("{}/api/login", station.base_url);
    let response = client
        .post(endpoint)
        .timeout(Duration::from_secs(timeout_secs.max(2)))
        .json(&json!({"user": username, "password": password}))
        .send()
        .map_err(|error| format!("basis station '{}' login failed: {error}", station.name))?;
    if !response.status().is_success() {
        let status = response.status();
        let detail = response.text().unwrap_or_default();
        return Err(format!(
            "basis station '{}' login returned HTTP {status}: {}",
            station.name,
            detail.chars().take(500).collect::<String>()
        ));
    }
    let cookie = response
        .headers()
        .get_all(SET_COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .filter_map(|value| value.split(';').next())
        .find(|value| value.starts_with("fs_session="))
        .map(str::to_string)
        .ok_or_else(|| {
            format!(
                "basis station '{}' login succeeded but returned no fs_session cookie",
                station.name
            )
        })?;
    Ok(Some(cookie))
}

fn station_request(
    request: reqwest::blocking::RequestBuilder,
    timeout_secs: u64,
    session_cookie: Option<&str>,
) -> reqwest::blocking::RequestBuilder {
    let request = request.timeout(Duration::from_secs(timeout_secs.max(2)));
    match session_cookie {
        Some(cookie) => request.header(COOKIE, cookie),
        None => request,
    }
}

fn station_get_json(
    client: &Client,
    station: &PlayoutStationConfig,
    endpoint: &str,
    timeout_secs: u64,
    session_cookie: Option<&str>,
) -> Result<Value, String> {
    let response = station_request(client.get(endpoint), timeout_secs, session_cookie)
        .send()
        .map_err(|error| format!("basis station '{}' request failed: {error}", station.name))?;
    if !response.status().is_success() {
        let status = response.status();
        let detail = response.text().unwrap_or_default();
        return Err(format!(
            "basis station '{}' returned HTTP {status}: {}",
            station.name,
            detail.chars().take(500).collect::<String>()
        ));
    }
    response
        .json::<Value>()
        .map_err(|error| format!("basis station '{}' returned invalid JSON: {error}", station.name))
}

fn play_via_media_switch(
    config: &MediaLibraryConfig,
    library: &SharedLibrary,
    client: &Client,
    claim: &DispatchClaim,
) -> Result<(), String> {
    let tetra_path = claim
        .tetra_path
        .as_ref()
        .ok_or_else(|| "direct Media Switch playout has no TETRA cache".to_string())?;
    let mut file = File::open(tetra_path)
        .map_err(|error| format!("cannot open TETRA cache {}: {error}", tetra_path.display()))?;
    let size = file
        .metadata()
        .map_err(|error| format!("cannot stat TETRA cache: {error}"))?
        .len();
    if size == 0 || size % config.codec.frame_bytes as u64 != 0 {
        return Err("TETRA cache is empty or not aligned to 35-byte frames".to_string());
    }
    if let Some(expected) = &claim.expected_tetra_sha256 {
        let actual = media::sha256_file(tetra_path)?;
        if !expected.eq_ignore_ascii_case(&actual) {
            return Err(format!(
                "TETRA cache integrity mismatch: expected {expected}, received {actual}"
            ));
        }
    }
    let frame_count = size / config.codec.frame_bytes as u64;
    if claim.job.frame_count != 0 && claim.job.frame_count != frame_count {
        tracing::warn!(
            job_id = %claim.job.job_id,
            expected = claim.job.frame_count,
            actual = frame_count,
            "Dispatch frame count changed since queueing"
        );
    }
    file.seek(SeekFrom::Start(claim.job.frame_index.saturating_mul(config.codec.frame_bytes as u64)))
        .map_err(|error| format!("cannot seek TETRA cache: {error}"))?;
    let endpoint = format!(
        "{}/api/v1/sessions/{}/inject",
        config.dependencies.media_switch_base_url,
        url_component(&claim.job.session_id)
    );
    let mut frame_index = claim.job.frame_index;
    let mut queued_targets = claim.job.queued_targets;
    let mut frame = vec![0u8; config.codec.frame_bytes];
    let frame_interval = Duration::from_millis(config.runtime.frame_interval_ms);
    let mut next_deadline = Instant::now();

    while frame_index < frame_count {
        if library.dispatch_cancel_requested(&claim.job.job_id) {
            library.complete_dispatch(&claim.job.job_id);
            return Ok(());
        }
        file.read_exact(&mut frame)
            .map_err(|error| format!("cannot read TETRA frame {frame_index}: {error}"))?;
        let response = client
            .post(&endpoint)
            .timeout(Duration::from_secs(5))
            .json(&json!({
                "payload": frame,
                "target_node": claim.job.target_node.clone(),
                "target_logical_ts": claim.job.target_logical_ts,
            }))
            .send()
            .map_err(|error| format!("Media Switch injection failed at frame {frame_index}: {error}"))?;
        if !response.status().is_success() {
            let status = response.status();
            let excerpt = response.text().unwrap_or_default();
            return Err(format!(
                "Media Switch rejected frame {frame_index} with HTTP {status}: {}",
                excerpt.chars().take(500).collect::<String>()
            ));
        }
        let body = response.json::<Value>().unwrap_or(Value::Null);
        queued_targets = queued_targets.saturating_add(
            body.get("queued_targets")
                .and_then(Value::as_u64)
                .unwrap_or(0),
        );
        frame_index = frame_index.saturating_add(1);
        library.dispatch_progress(&claim.job.job_id, frame_index, queued_targets);
        next_deadline += frame_interval;
        if let Some(delay) = next_deadline.checked_duration_since(Instant::now()) {
            thread::sleep(delay);
        } else {
            next_deadline = Instant::now();
        }
    }
    library.complete_dispatch(&claim.job.job_id);
    Ok(())
}

// Was: Führt den Arbeitsschritt `probe_dependencies` für probe dependencies aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn probe_dependencies(config: &MediaLibraryConfig, library: &SharedLibrary, client: &Client) {
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for (service, base_url) in [
        ("media-switch", &config.dependencies.media_switch_base_url),
        ("recorder", &config.dependencies.recorder_base_url),
        (
            "application-gateway",
            &config.dependencies.application_gateway_base_url,
        ),
    ] {
        let outcome = client
            .get(format!("{base_url}/health/live"))
            .timeout(Duration::from_secs(3))
            .send();
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match outcome {
            Ok(response) if response.status().is_success() => {
                library.update_dependency_probe(service, true, None)
            }
            Ok(response) => library.update_dependency_probe(
                service,
                false,
                Some(format!("HTTP {}", response.status())),
            ),
            Err(error) => library.update_dependency_probe(service, false, Some(error.to_string())),
        }
    }
}

// Was: Führt den Arbeitsschritt `url_component` für url component aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn url_component(value: &str) -> String {
    let mut output = String::new();
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            output.push(byte as char);
        } else {
            output.push_str(&format!("%{byte:02X}"));
        }
    }
    output
}
