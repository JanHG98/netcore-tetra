// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};

use tetra_config::bluestation::CfgAudioPlayer;
use uuid::Uuid;

use super::types::{
    AudioPlayerCommand, AudioPlayerState, AudioPlayerStatus, AudioSourceType, AudioTargetType, MediaEntry, MediaSourceInfo,
    ResolvedAudioSource,
};

#[derive(Debug)]
// Was: Bündelt die zusammengehörigen Werte für live Status in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct LiveStatus {
    state: AudioPlayerState,
    job_id: Option<String>,
    file_name: Option<String>,
    source_type: Option<AudioSourceType>,
    source_id: Option<String>,
    target_type: Option<AudioTargetType>,
    target_id: Option<u32>,
    priority: Option<u8>,
    duration_ms: u64,
    position_ms: u64,
    total_blocks: usize,
    sent_blocks: usize,
    call_id: Option<u16>,
    timeslot: Option<u8>,
    last_error: Option<String>,
}

// Was: Implementiert das zugehörige Verhalten für `Default for LiveStatus`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for LiveStatus {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            state: AudioPlayerState::Idle,
            job_id: None,
            file_name: None,
            source_type: None,
            source_id: None,
            target_type: None,
            target_id: None,
            priority: None,
            duration_ms: 0,
            position_ms: 0,
            total_blocks: 0,
            sent_blocks: 0,
            call_id: None,
            timeslot: None,
            last_error: None,
        }
    }
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für Audio- und Mediendaten root in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct MediaRoot {
    id: String,
    name: String,
    configured_path: PathBuf,
    source_type: &'static str,
    cache_before_decode: bool,
}

// Was: Bündelt die zusammengehörigen Werte für audio player shared in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct AudioPlayerShared {
    config: CfgAudioPlayer,
    local_root: PathBuf,
    cache_root: PathBuf,
    startup_warning: Option<String>,
    media_roots: Vec<MediaRoot>,
    command_tx: crossbeam_channel::Sender<AudioPlayerCommand>,
    live: Mutex<LiveStatus>,
    ffmpeg_available: bool,
}

#[derive(Clone)]
// Was: Bündelt die zusammengehörigen Werte für audio player handle in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct AudioPlayerHandle {
    inner: Arc<AudioPlayerShared>,
}

// Was: Implementiert das zugehörige Verhalten für `AudioPlayerHandle`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl AudioPlayerHandle {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub(crate) fn new(
        mut config: CfgAudioPlayer,
        command_tx: crossbeam_channel::Sender<AudioPlayerCommand>,
        ffmpeg_available: bool,
    ) -> Result<Self, String> {
        let local_root = PathBuf::from(&config.directory);
        fs::create_dir_all(&local_root).map_err(|e| format!("cannot create {}: {e}", local_root.display()))?;
        let local_root = local_root
            .canonicalize()
            .map_err(|e| format!("cannot canonicalize {}: {e}", local_root.display()))?;

        let configured_cache = PathBuf::from(&config.cache_directory);
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        let (cache_root, startup_warning) = match prepare_writable_cache(&configured_cache) {
            Ok(path) => (path, None),
            Err(primary_error) => {
                let candidates = [
                    std::env::temp_dir().join("netcore-audio"),
                    local_root.join(".netcore-audio-cache"),
                ];
                let mut failures = Vec::new();
                let mut selected = None;
                // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
                // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
                for candidate in candidates {
                    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                    match prepare_writable_cache(&candidate) {
                        Ok(path) => {
                            selected = Some(path);
                            break;
                        }
                        Err(error) => failures.push(format!("{}: {error}", candidate.display())),
                    }
                }
                let Some(path) = selected else {
                    return Err(format!(
                        "audio cache unavailable at {} ({primary_error}); fallback cache attempts failed: {}",
                        configured_cache.display(),
                        failures.join("; ")
                    ));
                };
                let warning = format!(
                    "configured audio cache {} is unavailable ({primary_error}); using fallback {}",
                    configured_cache.display(),
                    path.display()
                );
                tracing::warn!("AudioPlayer: {warning}");
                config.cache_directory = path.display().to_string();
                (path, Some(warning))
            }
        };
        cleanup_stale_cache(&cache_root);

        let mut media_roots = Vec::with_capacity(config.shares.len() + 1);
        media_roots.push(MediaRoot {
            id: "local".to_string(),
            name: "Lokale Dateien".to_string(),
            configured_path: local_root.clone(),
            source_type: "local",
            cache_before_decode: false,
        });
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for share in &config.shares {
            media_roots.push(MediaRoot {
                id: share.id.clone(),
                name: share.name.clone(),
                configured_path: PathBuf::from(&share.path),
                source_type: "server",
                cache_before_decode: true,
            });
        }

        Ok(Self {
            inner: Arc::new(AudioPlayerShared {
                config,
                local_root,
                cache_root,
                startup_warning,
                media_roots,
                command_tx,
                live: Mutex::new(LiveStatus::default()),
                ffmpeg_available,
            }),
        })
    }

    // Was: Führt den Arbeitsschritt `config` für Konfiguration aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn config(&self) -> &CfgAudioPlayer {
        &self.inner.config
    }

    // Was: Führt den Arbeitsschritt `root` für root aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn root(&self) -> &Path {
        &self.inner.local_root
    }

    // Was: Führt den Arbeitsschritt `cache_root` für Zwischenspeicher root aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn cache_root(&self) -> &Path {
        &self.inner.cache_root
    }

    // Was: Führt den Arbeitsschritt `startup_warning` für startup warning aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn startup_warning(&self) -> Option<&str> {
        self.inner.startup_warning.as_deref()
    }

    // Was: Führt den Arbeitsschritt `status` für Status aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn status(&self) -> AudioPlayerStatus {
        let live = self.inner.live.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        AudioPlayerStatus {
            available: true,
            state: live.state,
            directory: self.inner.local_root.display().to_string(),
            cache_directory: self.inner.cache_root.display().to_string(),
            startup_warning: self.inner.startup_warning.clone(),
            job_id: live.job_id.clone(),
            file_name: live.file_name.clone(),
            source_type: live.source_type,
            source_id: live.source_id.clone(),
            target_type: live.target_type,
            target_id: live.target_id,
            priority: live.priority,
            duration_ms: live.duration_ms,
            position_ms: live.position_ms,
            total_blocks: live.total_blocks,
            sent_blocks: live.sent_blocks,
            call_id: live.call_id,
            timeslot: live.timeslot,
            ffmpeg_available: self.inner.ffmpeg_available,
            last_error: live.last_error.clone(),
        }
    }

    // Was: Führt den Arbeitsschritt `media_sources` für Audio- und Mediendaten sources aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn media_sources(&self) -> Vec<MediaSourceInfo> {
        self.inner
            .media_roots
            .iter()
            .map(|root| match canonical_media_root(root) {
                Ok(_) => MediaSourceInfo {
                    id: root.id.clone(),
                    name: root.name.clone(),
                    path: root.configured_path.display().to_string(),
                    source_type: root.source_type.to_string(),
                    available: true,
                    error: None,
                },
                Err(error) => MediaSourceInfo {
                    id: root.id.clone(),
                    name: root.name.clone(),
                    path: root.configured_path.display().to_string(),
                    source_type: root.source_type.to_string(),
                    available: false,
                    error: Some(error),
                },
            })
            .collect()
    }

    // Was: Diese Funktion liefert Audio- und Mediendaten.
    // Warum: Die Zusammenstellung der Einträge bleibt damit konsistent und wiederverwendbar.
    pub fn list_media(&self, source_id: &str, relative: &str) -> Result<Vec<MediaEntry>, String> {
        let root = self.find_media_root(source_id)?;
        let canonical_root = canonical_media_root(root)?;
        let directory = resolve_directory(&canonical_root, relative)?;
        let relative_base = directory.strip_prefix(&canonical_root).map_err(|e| e.to_string())?;
        let mut entries = Vec::new();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for entry in fs::read_dir(&directory).map_err(|e| format!("cannot read {}: {e}", directory.display()))? {
            let entry = entry.map_err(|e| e.to_string())?;
            let file_type = entry.file_type().map_err(|e| e.to_string())?;
            if file_type.is_symlink() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                continue;
            }
            let child_relative = relative_base.join(&name).to_string_lossy().replace('\\', "/");
            if file_type.is_dir() {
                entries.push(MediaEntry {
                    name,
                    path: child_relative,
                    entry_type: "directory".to_string(),
                    size_bytes: None,
                    extension: None,
                });
            } else if file_type.is_file() {
                let extension = entry
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(str::to_ascii_lowercase);
                if !matches!(extension.as_deref(), Some("wav" | "mp3")) {
                    continue;
                }
                entries.push(MediaEntry {
                    name,
                    path: child_relative,
                    entry_type: "file".to_string(),
                    size_bytes: entry.metadata().ok().map(|metadata| metadata.len()),
                    extension,
                });
            }
        }
        entries.sort_by(|a, b| {
            let a_dir = a.entry_type == "directory";
            let b_dir = b.entry_type == "directory";
            b_dir.cmp(&a_dir).then_with(|| a.name.to_ascii_lowercase().cmp(&b.name.to_ascii_lowercase()))
        });
        Ok(entries)
    }

    /// Resolve a WAV/MP3 file for authenticated browser preview.
    ///
    /// The returned path is guaranteed to stay below the configured media root,
    /// to reference a regular WAV/MP3 file, and to respect the same maximum
    /// source-file size used by radio dispatch.
    // Was: Führt den Arbeitsschritt `preview_media_path` für preview Audio- und Mediendaten path aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn preview_media_path(&self, source_id: &str, relative_path: &str) -> Result<PathBuf, String> {
        let root = self.find_media_root(source_id)?;
        let canonical_root = canonical_media_root(root)?;
        let path = resolve_media_file(&canonical_root, relative_path)?;
        let size = path
            .metadata()
            .map_err(|e| format!("cannot read media metadata: {e}"))?
            .len();
        let max_bytes = self.inner.config.max_file_size_mb.saturating_mul(1024 * 1024);
        if size > max_bytes {
            return Err(format!(
                "file is too large for preview: {:.1} MiB (limit {} MiB)",
                size as f64 / 1_048_576.0,
                self.inner.config.max_file_size_mb
            ));
        }
        Ok(path)
    }

    // Was: Führt den Arbeitsschritt `play_media` für play Audio- und Mediendaten aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn play_media(
        &self,
        source_id: &str,
        relative_path: &str,
        target_type: AudioTargetType,
        target_id: u32,
        priority: Option<u8>,
    ) -> Result<String, String> {
        let root = self.find_media_root(source_id)?;
        let canonical_root = canonical_media_root(root)?;
        let path = resolve_media_file(&canonical_root, relative_path)?;
        let display_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("audio")
            .to_string();
        self.play_resolved(
            ResolvedAudioSource {
                path,
                display_name,
                source_type: AudioSourceType::Media,
                source_id: Some(root.id.clone()),
                cache_before_decode: root.cache_before_decode,
            },
            target_type,
            target_id,
            priority,
        )
    }

    // Was: Führt den Arbeitsschritt `play_recording` für play recording aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn play_recording(
        &self,
        path: PathBuf,
        display_name: String,
        target_type: AudioTargetType,
        target_id: u32,
        priority: Option<u8>,
    ) -> Result<String, String> {
        let canonical = path.canonicalize().map_err(|e| format!("cannot open recording: {e}"))?;
        if !canonical.is_file() {
            return Err("recording is not a regular file".to_string());
        }
        self.play_resolved(
            ResolvedAudioSource {
                path: canonical,
                display_name,
                source_type: AudioSourceType::Recording,
                source_id: None,
                cache_before_decode: false,
            },
            target_type,
            target_id,
            priority,
        )
    }

    // Was: Führt den Arbeitsschritt `play_resolved` für play resolved aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn play_resolved(
        &self,
        source: ResolvedAudioSource,
        target_type: AudioTargetType,
        target_id: u32,
        priority: Option<u8>,
    ) -> Result<String, String> {
        if target_id == 0 || target_id > 0x00ff_ffff {
            return Err("target must be a valid 24-bit ISSI/GSSI".to_string());
        }
        let priority = priority.unwrap_or(self.inner.config.default_priority);
        if priority > 15 {
            return Err("priority must be 0-15".to_string());
        }
        let job_id = Uuid::new_v4().to_string();
        let mut live = self.inner.live.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if !matches!(live.state, AudioPlayerState::Idle | AudioPlayerState::Failed) {
            return Err("an audio transmission is already active".to_string());
        }
        *live = LiveStatus {
            state: AudioPlayerState::Preparing,
            job_id: Some(job_id.clone()),
            file_name: Some(source.display_name.clone()),
            source_type: Some(source.source_type),
            source_id: source.source_id.clone(),
            target_type: Some(target_type),
            target_id: Some(target_id),
            priority: Some(priority),
            ..LiveStatus::default()
        };
        if self
            .inner
            .command_tx
            .send(AudioPlayerCommand::Play {
                job_id: job_id.clone(),
                source,
                target_type,
                target_id,
                priority,
            })
            .is_err()
        {
            live.state = AudioPlayerState::Failed;
            live.last_error = Some("audio-player entity is not running".to_string());
            return Err("audio-player entity is not running".to_string());
        }
        Ok(job_id)
    }

    // Was: Diese Funktion stoppt den vorgesehenen Arbeitsschritt.
    // Warum: Der Betrieb wird dadurch geordnet beendet und Ressourcen werden freigegeben.
    pub fn stop(&self) -> Result<(), String> {
        self.inner
            .command_tx
            .send(AudioPlayerCommand::Stop)
            .map_err(|_| "audio-player entity is not running".to_string())
    }

    // Was: Diese Funktion kennzeichnet preparing.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(crate) fn mark_preparing(
        &self,
        job_id: String,
        file_name: String,
        source_type: AudioSourceType,
        source_id: Option<String>,
        target_type: AudioTargetType,
        target_id: u32,
        priority: u8,
    ) {
        let mut live = self.inner.live.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        *live = LiveStatus {
            state: AudioPlayerState::Preparing,
            job_id: Some(job_id),
            file_name: Some(file_name),
            source_type: Some(source_type),
            source_id,
            target_type: Some(target_type),
            target_id: Some(target_id),
            priority: Some(priority),
            ..LiveStatus::default()
        };
    }

    // Was: Diese Funktion setzt Zustand.
    // Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
    pub(crate) fn set_state(&self, state: AudioPlayerState) {
        let mut live = self.inner.live.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        live.state = state;
    }

    // Was: Diese Funktion kennzeichnet prepared.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(crate) fn mark_prepared(&self, duration_ms: u64, total_blocks: usize) {
        let mut live = self.inner.live.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        live.duration_ms = duration_ms;
        live.total_blocks = total_blocks;
        live.position_ms = 0;
        live.sent_blocks = 0;
    }

    // Was: Diese Funktion kennzeichnet Audio- und Mediendaten ready.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(crate) fn mark_media_ready(&self, call_id: u16, ts: u8) {
        let mut live = self.inner.live.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        live.call_id = Some(call_id);
        live.timeslot = Some(ts);
        live.state = AudioPlayerState::Playing;
    }

    // Was: Diese Funktion kennzeichnet progress.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(crate) fn mark_progress(&self, sent_blocks: usize) {
        let mut live = self.inner.live.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        live.sent_blocks = sent_blocks;
        live.position_ms = (sent_blocks as u64 * 60).min(live.duration_ms);
    }

    // Was: Diese Funktion kennzeichnet idle.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(crate) fn mark_idle(&self) {
        let mut live = self.inner.live.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        *live = LiveStatus::default();
    }

    // Was: Diese Funktion kennzeichnet failed.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(crate) fn mark_failed(&self, error: impl Into<String>) {
        let error = error.into();
        tracing::error!("AudioPlayer: {}", error);
        let mut live = self.inner.live.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        live.state = AudioPlayerState::Failed;
        live.last_error = Some(error);
        live.call_id = None;
        live.timeslot = None;
    }

    // Was: Diese Funktion sucht Audio- und Mediendaten root.
    // Warum: Die Suchlogik bleibt damit wiederverwendbar und muss nicht an mehreren Stellen kopiert werden.
    fn find_media_root(&self, source_id: &str) -> Result<&MediaRoot, String> {
        let source_id = source_id.trim();
        self.inner
            .media_roots
            .iter()
            .find(|root| root.id == source_id)
            .ok_or_else(|| format!("unknown media source '{source_id}'"))
    }
}


// Was: Diese Funktion bereitet writable Zwischenspeicher.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn prepare_writable_cache(path: &Path) -> Result<PathBuf, String> {
    fs::create_dir_all(path).map_err(|e| format!("cannot create {}: {e}", path.display()))?;
    let canonical = path
        .canonicalize()
        .map_err(|e| format!("cannot canonicalize {}: {e}", path.display()))?;
    if !canonical.is_dir() {
        return Err(format!("{} is not a directory", canonical.display()));
    }
    let probe = canonical.join(format!(".write-probe-{}", Uuid::new_v4()));
    fs::write(&probe, b"netcore-audio-cache-probe")
        .map_err(|e| format!("{} is not writable: {e}", canonical.display()))?;
    fs::remove_file(&probe).map_err(|e| format!("cannot remove cache write probe {}: {e}", probe.display()))?;
    Ok(canonical)
}

// Was: Diese Funktion räumt stale Zwischenspeicher.
// Warum: Zurückgelassene Ressourcen würden sonst spätere Starts oder Verbindungen stören.
fn cleanup_stale_cache(root: &Path) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for entry in entries.flatten() {
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if !file_type.is_file() {
            continue;
        }
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let Some((uuid, suffix)) = name.split_once('.') else {
            continue;
        };
        if Uuid::parse_str(uuid).is_err() || !matches!(suffix, "wav" | "mp3" | "wav.part" | "mp3.part") {
            continue;
        }
        if let Err(error) = fs::remove_file(entry.path()) {
            tracing::warn!("AudioPlayer: cannot remove stale cache entry {}: {}", entry.path().display(), error);
        }
    }
}

// Was: Führt den Arbeitsschritt `canonical_media_root` für canonical Audio- und Mediendaten root aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn canonical_media_root(root: &MediaRoot) -> Result<PathBuf, String> {
    let canonical = root
        .configured_path
        .canonicalize()
        .map_err(|e| format!("media source '{}' unavailable at {}: {e}", root.name, root.configured_path.display()))?;
    if !canonical.is_dir() {
        return Err(format!(
            "media source '{}' is not a directory: {}",
            root.name,
            canonical.display()
        ));
    }
    Ok(canonical)
}

// Was: Diese Funktion ermittelt directory.
// Warum: Unklare oder indirekte Angaben werden so vor der weiteren Verarbeitung eindeutig gemacht.
fn resolve_directory(root: &Path, relative: &str) -> Result<PathBuf, String> {
    let relative = safe_relative_path(relative)?;
    let path = root.join(relative);
    let canonical = path.canonicalize().map_err(|e| format!("directory not found: {e}"))?;
    if !canonical.starts_with(root) || !canonical.is_dir() {
        return Err("directory escapes the configured media root".to_string());
    }
    Ok(canonical)
}

// Was: Diese Funktion ermittelt Audio- und Mediendaten file.
// Warum: Unklare oder indirekte Angaben werden so vor der weiteren Verarbeitung eindeutig gemacht.
fn resolve_media_file(root: &Path, relative: &str) -> Result<PathBuf, String> {
    let relative = safe_relative_path(relative)?;
    let path = root.join(relative);
    let canonical = path.canonicalize().map_err(|e| format!("file not found: {e}"))?;
    if !canonical.starts_with(root) || !canonical.is_file() {
        return Err("file escapes the configured media root".to_string());
    }
    let extension = canonical
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if !matches!(extension.as_str(), "wav" | "mp3") {
        return Err("only .wav and .mp3 files are supported".to_string());
    }
    Ok(canonical)
}

// Was: Führt den Arbeitsschritt `safe_relative_path` für safe relative path aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn safe_relative_path(path: &str) -> Result<PathBuf, String> {
    let path = Path::new(path.trim().trim_start_matches('/'));
    let mut clean = PathBuf::new();
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for component in path.components() {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match component {
            Component::Normal(part) => clean.push(part),
            Component::CurDir => {}
            _ => return Err("invalid media path".to_string()),
        }
    }
    Ok(clean)
}

// Was: Führt den Arbeitsschritt `detect_ffmpeg` für detect ffmpeg aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub(crate) fn detect_ffmpeg(path: &str) -> bool {
    Command::new(path)
        .arg("-version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
