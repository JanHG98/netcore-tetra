// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::ffi::CString;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read};
use std::os::unix::ffi::OsStrExt;
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{SyncSender, sync_channel};
use std::sync::{Arc, Mutex};

use tetra_config::bluestation::{CfgMediaLibrary, CfgRecording, RecordingMode};
use uuid::Uuid;

use super::archive::{recording_is_archived, spawn_archive_worker};
use super::media_library::{marker_path as media_library_marker_path, recording_is_published, recording_requires_publish, spawn_media_library_worker};
use super::types::{RecorderStatus, RecordingMetadata};
use super::wav::recover_part;

#[derive(Default)]
// Was: Bündelt die zusammengehörigen Werte für live Status in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub(super) struct LiveStatus {
    active_sessions: usize,
    active_call_ids: Vec<u16>,
    last_recording_id: Option<String>,
    last_error: Option<String>,
    pub(super) archive_available: bool,
    pub(super) archive_active: bool,
    pub(super) archive_pending: usize,
    pub(super) archive_completed: usize,
    pub(super) archive_last_success_at: Option<String>,
    pub(super) archive_last_error: Option<String>,
    pub(super) media_library_available: bool,
    pub(super) media_library_active: bool,
    pub(super) media_library_pending: usize,
    pub(super) media_library_completed: usize,
    pub(super) media_library_last_success_at: Option<String>,
    pub(super) media_library_last_error: Option<String>,
}

// Was: Bündelt die zusammengehörigen Werte für Aufzeichnung shared in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub(super) struct RecorderShared {
    pub(super) config: CfgRecording,
    pub(super) media_library: CfgMediaLibrary,
    pub(super) root: PathBuf,
    active: AtomicBool,
    live: Mutex<LiveStatus>,
    archive_tx: Option<SyncSender<()>>,
    media_library_tx: Option<SyncSender<()>>,
}

#[derive(Clone)]
// Was: Bündelt die zusammengehörigen Werte für Aufzeichnung handle in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RecorderHandle {
    inner: Arc<RecorderShared>,
}

// Was: Implementiert das zugehörige Verhalten für `RecorderHandle`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl RecorderHandle {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub(crate) fn new(config: CfgRecording, media_library: CfgMediaLibrary) -> io::Result<Self> {
        let root = PathBuf::from(&config.directory);
        fs::create_dir_all(&root)?;
        let (archive_tx, archive_rx) = if config.archive_enabled || config.tts_archive_enabled {
            let (tx, rx) = sync_channel(1);
            (Some(tx), Some(rx))
        } else {
            (None, None)
        };
        let (media_library_tx, media_library_rx) = if media_library.enabled && media_library.publish_recordings {
            let (tx, rx) = sync_channel(1);
            (Some(tx), Some(rx))
        } else {
            (None, None)
        };
        let handle = Self {
            inner: Arc::new(RecorderShared {
                active: AtomicBool::new(config.active),
                config,
                media_library,
                root,
                live: Mutex::new(LiveStatus::default()),
                archive_tx,
                media_library_tx,
            }),
        };
        handle.recover_partials();
        handle.recover_metadata_partials();
        handle.cleanup_retention();
        if let Some(rx) = archive_rx {
            spawn_archive_worker(&handle.inner, rx);
        }
        if let Some(rx) = media_library_rx {
            spawn_media_library_worker(&handle.inner, rx);
        }
        Ok(handle)
    }

    // Was: Prüft, ob active zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_active(&self) -> bool {
        self.inner.active.load(Ordering::Relaxed)
    }

    // Was: Diese Funktion setzt active.
    // Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
    pub fn set_active(&self, active: bool) {
        self.inner.active.store(active, Ordering::Relaxed);
    }

    // Was: Führt den Arbeitsschritt `config` für Konfiguration aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn config(&self) -> &CfgRecording {
        &self.inner.config
    }

    // Was: Führt den Arbeitsschritt `root` für root aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn root(&self) -> &Path {
        &self.inner.root
    }

    /// Whether the deliberately narrow public recording-export route may serve
    /// completed WAV files to the configured Media Library.
    pub fn media_library_export_enabled(&self) -> bool {
        self.inner.media_library.enabled && self.inner.media_library.publish_recordings
    }

    // Was: Diese Funktion setzt active calls.
    // Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
    pub(crate) fn set_active_calls(&self, mut ids: Vec<u16>) {
        let active_sessions = ids.len();
        ids.sort_unstable();
        ids.dedup();
        if let Ok(mut live) = self.inner.live.lock() {
            live.active_sessions = active_sessions;
            live.active_call_ids = ids;
        }
    }

    // Was: Führt den Arbeitsschritt `note_completed` für note completed aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(crate) fn note_completed(&self, id: String) {
        if let Ok(mut live) = self.inner.live.lock() {
            live.last_recording_id = Some(id);
            live.last_error = None;
        }
        if let Some(tx) = &self.inner.archive_tx {
            let _ = tx.try_send(());
        }
        if let Some(tx) = &self.inner.media_library_tx {
            let _ = tx.try_send(());
        }
    }

    // Was: Führt den Arbeitsschritt `note_error` für note error aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(crate) fn note_error(&self, error: impl Into<String>) {
        let error = error.into();
        tracing::error!("Recorder: {}", error);
        if let Ok(mut live) = self.inner.live.lock() {
            live.last_error = Some(error);
        }
    }

    // Was: Prüft, ob Datensatz zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn should_record(&self, destination_id: u32, destination_is_group: bool) -> bool {
        if !self.is_active() {
            return false;
        }
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self.inner.config.mode {
            RecordingMode::All => true,
            RecordingMode::SelectedGroups => destination_is_group && self.inner.config.selected_groups.binary_search(&destination_id).is_ok(),
        }
    }

    // Was: Prüft, ob minimum free space zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn has_minimum_free_space(&self) -> bool {
        let required = self.inner.config.minimum_free_space_mb.saturating_mul(1024 * 1024);
        available_space(&self.inner.root).map(|free| free >= required).unwrap_or(false)
    }

    // Was: Führt den Arbeitsschritt `status` für Status aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn status(&self) -> RecorderStatus {
        let recordings = self.scan_recordings();
        let (
            active_sessions,
            active_call_ids,
            last_recording_id,
            last_error,
            archive_available,
            archive_active,
            archive_pending,
            archive_completed,
            archive_last_success_at,
            archive_last_error,
            media_library_available,
            media_library_active,
            media_library_pending,
            media_library_completed,
            media_library_last_success_at,
            media_library_last_error,
        ) = match self.inner.live.lock() {
            Ok(live) => (
                live.active_sessions,
                live.active_call_ids.clone(),
                live.last_recording_id.clone(),
                live.last_error.clone(),
                live.archive_available,
                live.archive_active,
                live.archive_pending,
                live.archive_completed,
                live.archive_last_success_at.clone(),
                live.archive_last_error.clone(),
                live.media_library_available,
                live.media_library_active,
                live.media_library_pending,
                live.media_library_completed,
                live.media_library_last_success_at.clone(),
                live.media_library_last_error.clone(),
            ),
            Err(_) => (
                0,
                Vec::new(),
                None,
                Some("recorder status lock poisoned".to_string()),
                false,
                false,
                0,
                0,
                None,
                None,
                false,
                false,
                0,
                0,
                None,
                None,
            ),
        };
        RecorderStatus {
            available: true,
            active: self.is_active(),
            directory: self.inner.root.display().to_string(),
            mode: match self.inner.config.mode {
                RecordingMode::All => "all",
                RecordingMode::SelectedGroups => "selected_groups",
            }
            .to_string(),
            selected_groups: self.inner.config.selected_groups.clone(),
            minimum_free_space_mb: self.inner.config.minimum_free_space_mb,
            free_space_bytes: available_space(&self.inner.root),
            used_bytes: directory_size(&self.inner.root),
            recording_count: recordings.len(),
            active_sessions,
            active_call_ids,
            last_recording_id,
            last_error,
            archive_enabled: self.inner.config.archive_enabled,
            archive_directory: self.inner.config.archive_directory.clone(),
            tts_archive_enabled: self.inner.config.tts_archive_enabled,
            tts_archive_directory: self.inner.config.tts_archive_directory.clone(),
            archive_available,
            archive_active,
            archive_pending,
            archive_completed,
            archive_last_success_at,
            archive_last_error,
            media_library_enabled: self.inner.media_library.enabled && self.inner.media_library.publish_recordings,
            media_library_url: self.inner.media_library.base_url.clone(),
            media_library_available,
            media_library_active,
            media_library_pending,
            media_library_completed,
            media_library_last_success_at,
            media_library_last_error,
        }
    }

    // Was: Diese Funktion liefert recordings.
    // Warum: Die Zusammenstellung der Einträge bleibt damit konsistent und wiederverwendbar.
    pub fn list_recordings(&self, limit: Option<usize>) -> Vec<RecordingMetadata> {
        let mut metadata = self.scan_recordings();
        metadata.truncate(limit.unwrap_or(self.inner.config.max_list_entries).min(self.inner.config.max_list_entries));
        metadata
    }

    /// Import a finished 8-kHz mono PCM WAV into the local recording library.
    /// The WAV and JSON sidecar are written exactly like normal call recordings,
    /// so playback, deletion and retention use the same code path. The archive
    /// worker routes `origin = "tts"` to the dedicated TTS server directory.
    // Was: Führt den Arbeitsschritt `import_named_wav` für import named wav aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn import_named_wav(&self, source: &Path, title: &str, origin: &str) -> Result<RecordingMetadata, String> {
        if !self.has_minimum_free_space() {
            return Err(format!(
                "minimum free space threshold reached ({} MiB)",
                self.inner.config.minimum_free_space_mb
            ));
        }
        let title = normalize_library_title(title)?;
        let origin = normalize_library_origin(origin)?;
        let (duration_ms, audio_bytes) = inspect_recording_wav(source)?;
        let now = chrono::Local::now();
        let id = Uuid::new_v4().to_string();
        let day_dir = self
            .root()
            .join(now.format("%Y").to_string())
            .join(now.format("%m").to_string())
            .join(now.format("%d").to_string());
        fs::create_dir_all(&day_dir).map_err(|error| format!("cannot create {}: {error}", day_dir.display()))?;
        let safe_title = library_filename_component(&title);
        let stem = format!(
            "{}-{}_{}_{}",
            origin.to_uppercase(),
            safe_title,
            now.format("%Y-%m-%d_%H-%M-%S"),
            id
        );
        let final_audio_path = day_dir.join(format!("{stem}.wav"));
        let part_audio_path = day_dir.join(format!("{stem}.wav.part"));
        let final_metadata_path = day_dir.join(format!("{stem}.json"));

        let result = (|| -> Result<RecordingMetadata, String> {
            fs::copy(source, &part_audio_path).map_err(|error| {
                format!(
                    "cannot copy generated WAV {} -> {}: {error}",
                    source.display(),
                    part_audio_path.display()
                )
            })?;
            OpenOptions::new()
                .write(true)
                .open(&part_audio_path)
                .and_then(|file| file.sync_all())
                .map_err(|error| format!("cannot sync {}: {error}", part_audio_path.display()))?;
            fs::rename(&part_audio_path, &final_audio_path).map_err(|error| {
                format!(
                    "cannot publish generated WAV {} -> {}: {error}",
                    part_audio_path.display(),
                    final_audio_path.display()
                )
            })?;

            let relative_audio_path = final_audio_path
                .strip_prefix(self.root())
                .map_err(|error| error.to_string())?
                .to_string_lossy()
                .replace('\\', "/");
            let timestamp = now.to_rfc3339();
            let metadata = RecordingMetadata {
                schema_version: 1,
                id: id.clone(),
                title: Some(title.clone()),
                origin: Some(origin.clone()),
                call_id: 0,
                source_issi: 0,
                destination_id: 0,
                destination_type: "library".to_string(),
                started_at: timestamp.clone(),
                ended_at: timestamp,
                duration_ms,
                audio_bytes,
                relative_audio_path,
                recovered_after_unclean_shutdown: false,
                segments: Vec::new(),
            };
            write_recording_metadata_atomic(&final_metadata_path, &metadata)?;
            Ok(metadata)
        })();

        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match result {
            Ok(metadata) => {
                self.note_completed(metadata.id.clone());
                tracing::info!(
                    "Recorder: imported {} WAV title={} id={} duration_ms={} path={}",
                    origin,
                    title,
                    metadata.id,
                    metadata.duration_ms,
                    final_audio_path.display()
                );
                Ok(metadata)
            }
            Err(error) => {
                let _ = fs::remove_file(&part_audio_path);
                let _ = fs::remove_file(&final_audio_path);
                let _ = fs::remove_file(&final_metadata_path);
                Err(error)
            }
        }
    }

    // Was: Führt den Arbeitsschritt `scan_recordings` für scan recordings aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn scan_recordings(&self) -> Vec<RecordingMetadata> {
        let mut metadata = Vec::new();
        let mut files = Vec::new();
        collect_files_with_suffix(&self.inner.root, ".json", &mut files);
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for path in files {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match fs::read_to_string(&path)
                .ok()
                .and_then(|body| serde_json::from_str::<RecordingMetadata>(&body).ok())
            {
                Some(item) => metadata.push(item),
                None => tracing::warn!("Recorder: ignoring invalid metadata {}", path.display()),
            }
        }
        metadata.sort_by(|a, b| b.started_at.cmp(&a.started_at));
        metadata
    }

    // Was: Diese Funktion sucht recording.
    // Warum: Die Suchlogik bleibt damit wiederverwendbar und muss nicht an mehreren Stellen kopiert werden.
    pub fn find_recording(&self, id: &str) -> Option<RecordingMetadata> {
        if !valid_id(id) {
            return None;
        }
        self.scan_recordings().into_iter().find(|item| item.id == id)
    }

    // Was: Führt den Arbeitsschritt `audio_path` für audio path aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn audio_path(&self, id: &str) -> Result<PathBuf, String> {
        let metadata = self.find_recording(id).ok_or_else(|| "recording not found".to_string())?;
        let relative = safe_relative_path(&metadata.relative_audio_path)?;
        let path = self.inner.root.join(relative);
        let canonical_root = self.inner.root.canonicalize().map_err(|e| e.to_string())?;
        let canonical_path = path.canonicalize().map_err(|e| e.to_string())?;
        if !canonical_path.starts_with(&canonical_root) {
            return Err("recording path escapes configured root".to_string());
        }
        Ok(canonical_path)
    }

    // Was: Diese Funktion löscht recording.
    // Warum: Das Entfernen wird dadurch kontrolliert durchgeführt und hinterlässt keine verwaisten Verweise.
    pub fn delete_recording(&self, id: &str) -> Result<(), String> {
        if !valid_id(id) {
            return Err("invalid recording id".to_string());
        }
        let metadata = self.find_recording(id).ok_or_else(|| "recording not found".to_string())?;
        let audio = self.audio_path(id)?;
        let json = audio.with_extension("json");
        let archived = audio.with_extension("archived");
        let media_library_marker = media_library_marker_path(&audio);
        if audio.exists() {
            fs::remove_file(&audio).map_err(|e| format!("failed to delete {}: {e}", audio.display()))?;
        }
        if json.exists() {
            fs::remove_file(&json).map_err(|e| format!("failed to delete {}: {e}", json.display()))?;
        } else {
            // Metadata may not share the WAV stem if manually imported; locate it by id.
            let mut files = Vec::new();
            collect_files_with_suffix(&self.inner.root, ".json", &mut files);
            // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
            // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
            for path in files {
                if fs::read_to_string(&path)
                    .ok()
                    .and_then(|body| serde_json::from_str::<RecordingMetadata>(&body).ok())
                    .is_some_and(|item| item.id == metadata.id)
                {
                    let _ = fs::remove_file(path);
                    break;
                }
            }
        }
        if archived.exists() {
            fs::remove_file(&archived).map_err(|e| format!("failed to delete {}: {e}", archived.display()))?;
        }
        if media_library_marker.exists() {
            fs::remove_file(&media_library_marker)
                .map_err(|e| format!("failed to delete {}: {e}", media_library_marker.display()))?;
        }
        Ok(())
    }

    // Was: Diese Funktion stellt partials.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn recover_partials(&self) {
        let mut parts = Vec::new();
        collect_files_with_suffix(&self.inner.root, ".wav.part", &mut parts);
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for part in parts {
            let final_path = PathBuf::from(part.to_string_lossy().trim_end_matches(".part"));
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match recover_part(&part, &final_path) {
                Ok(data_bytes) => {
                    let json_part = PathBuf::from(format!("{}.json.part", final_path.with_extension("").display()));
                    let json_final = final_path.with_extension("json");
                    let metadata_recovered = if let Ok(body) = fs::read_to_string(&json_part)
                        && let Ok(mut metadata) = serde_json::from_str::<RecordingMetadata>(&body)
                    {
                        metadata.recovered_after_unclean_shutdown = true;
                        metadata.ended_at = chrono::Local::now().to_rfc3339();
                        metadata.audio_bytes = data_bytes;
                        metadata.duration_ms = (data_bytes / 2).saturating_mul(1000) / 8_000;
                        if let Some(last) = metadata.segments.last_mut() {
                            last.end_ms = metadata.duration_ms;
                        }
                        serde_json::to_vec_pretty(&metadata)
                            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                            .and_then(|final_body| fs::write(&json_final, final_body))
                            .is_ok()
                    } else {
                        false
                    };
                    if metadata_recovered {
                        let _ = fs::remove_file(&json_part);
                    }
                    tracing::warn!("Recorder: recovered partial WAV {}", final_path.display());
                }
                Err(e) => self.note_error(format!("failed to recover {}: {e}", part.display())),
            }
        }
    }

    // Was: Diese Funktion stellt metadata partials.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn recover_metadata_partials(&self) {
        let mut parts = Vec::new();
        collect_files_with_suffix(&self.inner.root, ".json.part", &mut parts);
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for json_part in parts {
            let stem = json_part.to_string_lossy().trim_end_matches(".json.part").to_string();
            let wav_path = PathBuf::from(format!("{stem}.wav"));
            if !wav_path.is_file() {
                continue;
            }
            let data_bytes = wav_path.metadata().map(|m| m.len().saturating_sub(44)).unwrap_or(0);
            let Ok(body) = fs::read_to_string(&json_part) else { continue };
            let Ok(mut metadata) = serde_json::from_str::<RecordingMetadata>(&body) else { continue };
            metadata.recovered_after_unclean_shutdown = true;
            metadata.ended_at = chrono::Local::now().to_rfc3339();
            metadata.audio_bytes = data_bytes;
            metadata.duration_ms = (data_bytes / 2).saturating_mul(1000) / 8_000;
            if let Some(last) = metadata.segments.last_mut() {
                last.end_ms = metadata.duration_ms;
            }
            let json_final = PathBuf::from(format!("{stem}.json"));
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match serde_json::to_vec_pretty(&metadata)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
                .and_then(|body| fs::write(&json_final, body))
            {
                Ok(()) => {
                    let _ = fs::remove_file(&json_part);
                    tracing::warn!("Recorder: recovered metadata {}", json_final.display());
                }
                Err(e) => self.note_error(format!("failed to recover {}: {e}", json_part.display())),
            }
        }
    }

    // Was: Diese Funktion räumt retention.
    // Warum: Zurückgelassene Ressourcen würden sonst spätere Starts oder Verbindungen stören.
    fn cleanup_retention(&self) {
        let days = self.inner.config.retention_days;
        if days == 0 {
            return;
        }
        let cutoff = std::time::SystemTime::now()
            .checked_sub(std::time::Duration::from_secs(days as u64 * 86_400))
            .unwrap_or(std::time::UNIX_EPOCH);
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for item in self.scan_recordings() {
            let Ok(audio) = self.audio_path(&item.id) else { continue };
            let modified = audio.metadata().and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::now());
            if modified < cutoff {
                let archive_pending = super::archive::recording_requires_archive(&self.inner, &item)
                    && !recording_is_archived(&self.inner, &item);
                let media_library_pending = recording_requires_publish(&self.inner, &item)
                    && !recording_is_published(&self.inner, &item);
                if archive_pending || media_library_pending {
                    tracing::warn!(
                        "Recorder: retention kept recording id={} because archive/media-library transfer is not confirmed",
                        item.id
                    );
                    continue;
                }
                if let Err(e) = self.delete_recording(&item.id) {
                    self.note_error(format!("retention cleanup failed for {}: {e}", item.id));
                }
            }
        }
    }
}


// Was: Implementiert das zugehörige Verhalten für `RecorderShared`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl RecorderShared {
    // Was: Diese Funktion aktualisiert archive Status.
    // Warum: Bestehender Zustand wird dadurch kontrolliert und nach einheitlichen Regeln geändert.
    pub(super) fn update_archive_status(&self, update: impl FnOnce(&mut LiveStatus)) {
        if let Ok(mut live) = self.live.lock() {
            update(&mut live);
        }
    }

    pub(super) fn update_media_library_status(&self, update: impl FnOnce(&mut LiveStatus)) {
        if let Ok(mut live) = self.live.lock() {
            update(&mut live);
        }
    }

    // Was: Führt den Arbeitsschritt `scan_recordings` für scan recordings aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn scan_recordings(&self) -> Vec<RecordingMetadata> {
        let mut metadata = Vec::new();
        let mut files = Vec::new();
        collect_files_with_suffix(&self.root, ".json", &mut files);
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for path in files {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match fs::read_to_string(&path)
                .ok()
                .and_then(|body| serde_json::from_str::<RecordingMetadata>(&body).ok())
            {
                Some(item) => metadata.push(item),
                None => tracing::warn!("Recorder: ignoring invalid metadata {}", path.display()),
            }
        }
        metadata.sort_by(|a, b| b.started_at.cmp(&a.started_at));
        metadata
    }
}

// Was: Führt den Arbeitsschritt `normalize_library_title` für normalize library title aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn normalize_library_title(title: &str) -> Result<String, String> {
    let title = title.split_whitespace().collect::<Vec<_>>().join(" ");
    let count = title.chars().count();
    if count == 0 || count > 120 {
        return Err("recording name must contain 1-120 characters".to_string());
    }
    if title.chars().any(char::is_control) {
        return Err("recording name contains invalid control characters".to_string());
    }
    Ok(title)
}

// Was: Führt den Arbeitsschritt `normalize_library_origin` für normalize library origin aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn normalize_library_origin(origin: &str) -> Result<String, String> {
    let origin = origin.trim().to_ascii_lowercase();
    if origin.is_empty()
        || origin.len() > 24
        || !origin.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
    {
        return Err("invalid recording origin".to_string());
    }
    Ok(origin)
}

// Was: Führt den Arbeitsschritt `library_filename_component` für library filename component aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn library_filename_component(title: &str) -> String {
    let mut out = String::new();
    let mut separator = false;
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for ch in title.chars() {
        if ch.is_alphanumeric() || matches!(ch, '-' | '_') {
            out.push(ch);
            separator = false;
        } else if !separator {
            out.push('_');
            separator = true;
        }
        if out.chars().count() >= 80 {
            break;
        }
    }
    let out = out.trim_matches('_');
    if out.is_empty() {
        "Durchsage".to_string()
    } else {
        out.to_string()
    }
}

// Was: Führt den Arbeitsschritt `inspect_recording_wav` für inspect recording wav aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn inspect_recording_wav(path: &Path) -> Result<(u64, u64), String> {
    let mut file = File::open(path).map_err(|error| format!("cannot open generated WAV {}: {error}", path.display()))?;
    let mut header = [0u8; 44];
    file.read_exact(&mut header)
        .map_err(|error| format!("cannot read generated WAV header {}: {error}", path.display()))?;
    let pcm_format = u16::from_le_bytes([header[20], header[21]]);
    let channels = u16::from_le_bytes([header[22], header[23]]);
    let sample_rate = u32::from_le_bytes([header[24], header[25], header[26], header[27]]);
    let bits_per_sample = u16::from_le_bytes([header[34], header[35]]);
    let data_bytes = u32::from_le_bytes([header[40], header[41], header[42], header[43]]) as u64;
    if &header[0..4] != b"RIFF"
        || &header[8..12] != b"WAVE"
        || &header[12..16] != b"fmt "
        || &header[36..40] != b"data"
        || pcm_format != 1
        || channels != 1
        || sample_rate != 8_000
        || bits_per_sample != 16
    {
        return Err("generated WAV is not canonical PCM s16le/mono/8000Hz".to_string());
    }
    let file_len = file.metadata().map_err(|error| error.to_string())?.len();
    if data_bytes == 0 || file_len < 44u64.saturating_add(data_bytes) || data_bytes % 2 != 0 {
        return Err("generated WAV has an invalid or empty data chunk".to_string());
    }
    let samples = data_bytes / 2;
    let duration_ms = samples.saturating_mul(1000) / 8_000;
    Ok((duration_ms, data_bytes))
}

// Was: Diese Funktion schreibt recording metadata atomic.
// Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
fn write_recording_metadata_atomic(path: &Path, metadata: &RecordingMetadata) -> Result<(), String> {
    let body = serde_json::to_vec_pretty(metadata).map_err(|error| error.to_string())?;
    let tmp = PathBuf::from(format!("{}.tmp", path.display()));
    fs::write(&tmp, body).map_err(|error| format!("cannot write {}: {error}", tmp.display()))?;
    OpenOptions::new()
        .write(true)
        .open(&tmp)
        .and_then(|file| file.sync_all())
        .map_err(|error| format!("cannot sync {}: {error}", tmp.display()))?;
    fs::rename(&tmp, path).map_err(|error| format!("cannot rename {} -> {}: {error}", tmp.display(), path.display()))
}

// Was: Führt den Arbeitsschritt `valid_id` für valid Kennung aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn valid_id(id: &str) -> bool {
    id.len() == 36 && id.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
}

// Was: Führt den Arbeitsschritt `safe_relative_path` für safe relative path aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub(super) fn safe_relative_path(path: &str) -> Result<PathBuf, String> {
    let path = Path::new(path);
    if path.is_absolute() {
        return Err("absolute recording path rejected".to_string());
    }
    let mut clean = PathBuf::new();
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for component in path.components() {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match component {
            Component::Normal(part) => clean.push(part),
            _ => return Err("invalid recording path".to_string()),
        }
    }
    Ok(clean)
}

// Was: Diese Funktion sammelt files with suffix.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn collect_files_with_suffix(root: &Path, suffix: &str, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else { return };
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for entry in entries.flatten() {
        let Ok(file_type) = entry.file_type() else { continue };
        if file_type.is_symlink() {
            continue;
        }
        let path = entry.path();
        if file_type.is_dir() {
            collect_files_with_suffix(&path, suffix, out);
        } else if file_type.is_file() && path.to_string_lossy().ends_with(suffix) {
            out.push(path);
        }
    }
}

// Was: Führt den Arbeitsschritt `directory_size` für directory size aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn directory_size(root: &Path) -> u64 {
    let Ok(entries) = fs::read_dir(root) else { return 0 };
    entries
        .flatten()
        .map(|entry| {
            let Ok(file_type) = entry.file_type() else { return 0 };
            if file_type.is_symlink() {
                return 0;
            }
            let path = entry.path();
            if file_type.is_dir() {
                directory_size(&path)
            } else if file_type.is_file() {
                entry.metadata().map(|m| m.len()).unwrap_or(0)
            } else {
                0
            }
        })
        .sum()
}

// Was: Führt den Arbeitsschritt `available_space` für available space aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn available_space(path: &Path) -> Option<u64> {
    let c_path = CString::new(path.as_os_str().as_bytes()).ok()?;
    let mut stat = std::mem::MaybeUninit::<libc::statvfs>::uninit();
    let rc = unsafe { libc::statvfs(c_path.as_ptr(), stat.as_mut_ptr()) };
    if rc != 0 {
        return None;
    }
    let stat = unsafe { stat.assume_init() };
    Some((stat.f_bavail as u64).saturating_mul(stat.f_frsize as u64))
}
