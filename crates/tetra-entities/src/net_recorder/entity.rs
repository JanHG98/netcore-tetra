// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};
use std::thread::JoinHandle;
use crossbeam_channel::{Sender, Receiver, TrySendError, RecvTimeoutError};

use tetra_config::bluestation::SharedConfig;
use tetra_core::tetra_entities::TetraEntity;
use tetra_saps::{SapMsg, SapMsgInner, control::call_control::CallControl, tmd::TmdCircuitDataInd};
use uuid::Uuid;

use crate::net_audio::{TETRA_PCM_SAMPLE_RATE, TetraSpeechDecoder};
use crate::{MessageQueue, TetraEntityTrait};

use super::service::RecorderHandle;
use super::types::{RecordingMetadata, RecordingSegment};
use super::wav::PcmWavWriter;

/// Recorder work is ordered on one I/O thread. The RF thread only submits
/// bounded messages; disk writes, codec work and fsync never run on a TDMA tick.
pub struct RecorderEntity {
    mailbox: Option<RecorderMailbox>,
    worker: Option<JoinHandle<()>>,
}

struct RecorderEnvelope {
    epoch: u64,
    message: SapMsgInner,
}

struct RecorderMailbox {
    tx: Sender<RecorderEnvelope>,
    epoch: Arc<AtomicU64>,
}

impl RecorderMailbox {
    fn submit(&self, message: SapMsgInner) {
        let envelope = RecorderEnvelope {
            epoch: self.epoch.load(Ordering::SeqCst),
            message,
        };
        if let Err(TrySendError::Full(_)) = self.tx.try_send(envelope) {
            // A missing lifecycle event could attach audio to the wrong call.
            // Invalidate the entire old epoch; the worker closes those sessions
            // and discards queued stale events before accepting new ones.
            self.epoch.fetch_add(1, Ordering::SeqCst);
        }
    }
}

impl RecorderEntity {
    pub fn new(config: SharedConfig) -> Result<(Self, RecorderHandle), String> {
        let (state, handle) = RecorderWorker::new(config)?;
        let (tx, rx) = crossbeam_channel::bounded(2048);
        let epoch = Arc::new(AtomicU64::new(0));
        let worker_epoch = epoch.clone();
        let worker = std::thread::Builder::new()
            .name("tetra-recorder".into())
            .spawn(move || state.run(rx, worker_epoch))
            .map_err(|e| format!("cannot start recorder worker: {e}"))?;
        Ok((Self { mailbox: Some(RecorderMailbox { tx, epoch }), worker: Some(worker) }, handle))
    }
}

impl TetraEntityTrait for RecorderEntity {
    fn entity(&self) -> TetraEntity { TetraEntity::Recorder }

    fn rx_prim(&mut self, _queue: &mut MessageQueue, message: SapMsg) {
        if matches!(&message.msg,
            SapMsgInner::CmceCallControl(CallControl::FloorGranted { .. }
                | CallControl::FloorReleased { .. } | CallControl::CallEnded { .. })
                | SapMsgInner::TmdCircuitDataInd(_)) {
            if let Some(mailbox) = &self.mailbox { mailbox.submit(message.msg); }
        }
    }
}

impl Drop for RecorderEntity {
    fn drop(&mut self) {
        // Disconnect after the last message, then drain and finalize in order.
        self.mailbox.take();
        if let Some(worker) = self.worker.take() {
            if worker.join().is_err() { tracing::error!("Recorder worker panicked"); }
        }
    }
}

impl RecorderWorker {
    fn run(mut self, rx: Receiver<RecorderEnvelope>, epoch: Arc<AtomicU64>) {
        let mut current_epoch = 0;
        loop {
            let received = rx.recv_timeout(Duration::from_millis(100));
            let latest_epoch = epoch.load(Ordering::SeqCst);
            if latest_epoch != current_epoch {
                self.finish_all("recorder-queue-overflow");
                self.handle.note_error("Recorder queue overflow: incomplete recordings closed; RF processing continued");
                current_epoch = latest_epoch;
            }
            self.poll();
            match received {
                Ok(event) if event.epoch == current_epoch => self.on_message(event.message),
                Ok(_) | Err(RecvTimeoutError::Timeout) => {},
                Err(RecvTimeoutError::Disconnected) => break,
            }
        }
        // Drop finalizes any remaining sessions, including a clean BS shutdown.
    }
}

// Was: Bündelt die zusammengehörigen Werte für recording Sitzung in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct RecordingSession {
    id: String,
    call_id: u16,
    destination_id: u32,
    destination_is_group: bool,
    started_at: String,
    started_instant: Instant,
    last_activity: Instant,
    decoder: TetraSpeechDecoder,
    writer: PcmWavWriter,
    final_audio_path: PathBuf,
    metadata_part_path: PathBuf,
    relative_audio_path: String,
    samples_written: u64,
    segments: Vec<RecordingSegment>,
    current_segment: Option<usize>,
}

// Was: Implementiert das zugehörige Verhalten für `RecordingSession`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl RecordingSession {
    // Was: Führt den Arbeitsschritt `begin_segment` für begin segment aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn begin_segment(&mut self, source_issi: u32, carrier_num: u16, ts: u8) -> Result<(), String> {
        self.finish_segment();
        self.decoder = TetraSpeechDecoder::new().ok_or_else(|| "tetra decoder creation failed".to_string())?;
        let start_ms = samples_to_ms(self.samples_written);
        self.segments.push(RecordingSegment {
            source_issi,
            timeslot: ts,
            carrier_num,
            start_ms,
            end_ms: start_ms,
        });
        self.current_segment = Some(self.segments.len() - 1);
        self.last_activity = Instant::now();
        self.write_partial_metadata()
    }

    // Was: Führt den Arbeitsschritt `finish_segment` für finish segment aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn finish_segment(&mut self) {
        if let Some(index) = self.current_segment.take()
            && let Some(segment) = self.segments.get_mut(index)
        {
            segment.end_ms = samples_to_ms(self.samples_written);
        }
    }

    // Was: Führt den Arbeitsschritt `append_audio` für append audio aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn append_audio(&mut self, data: &[u8]) -> Result<(), String> {
        let pcm = self
            .decoder
            .decode_tmd_to_pcm(data)
            .ok_or_else(|| format!("unsupported TETRA audio block length {}", data.len()))?;
        self.writer.write_samples(&pcm).map_err(|e| e.to_string())?;
        self.samples_written = self.samples_written.saturating_add(pcm.len() as u64);
        if let Some(index) = self.current_segment
            && let Some(segment) = self.segments.get_mut(index)
        {
            segment.end_ms = samples_to_ms(self.samples_written);
        }
        self.last_activity = Instant::now();
        Ok(())
    }

    // Was: Führt den Arbeitsschritt `metadata` für metadata aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn metadata(&self, ended_at: String, recovered: bool, audio_bytes: u64) -> RecordingMetadata {
        RecordingMetadata {
            schema_version: 1,
            id: self.id.clone(),
            title: None,
            origin: None,
            call_id: self.call_id,
            source_issi: self.segments.first().map(|s| s.source_issi).unwrap_or(0),
            destination_id: self.destination_id,
            destination_type: if self.destination_is_group { "group" } else { "individual" }.to_string(),
            started_at: self.started_at.clone(),
            ended_at,
            duration_ms: samples_to_ms(self.samples_written),
            audio_bytes,
            relative_audio_path: self.relative_audio_path.clone(),
            recovered_after_unclean_shutdown: recovered,
            segments: self.segments.clone(),
        }
    }

    // Was: Diese Funktion schreibt partial metadata.
    // Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
    fn write_partial_metadata(&self) -> Result<(), String> {
        let metadata = self.metadata(String::new(), false, self.samples_written.saturating_mul(2));
        write_json_atomic(&self.metadata_part_path, &metadata)
    }
}

// Was: Bündelt die zusammengehörigen Werte für Aufzeichnung entity in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct RecorderWorker {
    config: SharedConfig,
    handle: RecorderHandle,
    sessions: HashMap<(u16, u8), RecordingSession>,
    calls_by_ts: HashMap<u8, u16>,
    active_floors: HashSet<(u16, u8)>,
    runtime_was_active: bool,
}

// Was: Implementiert das zugehörige Verhalten für `RecorderWorker`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl RecorderWorker {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    fn new(config: SharedConfig) -> Result<(Self, RecorderHandle), String> {
        let handle = RecorderHandle::new(
            config.config().recording.clone(),
            config.config().media_library.clone(),
        )
        .map_err(|e| format!("cannot initialize recording directory: {e}"))?;
        let entity = Self {
            runtime_was_active: handle.is_active(),
            config,
            handle: handle.clone(),
            sessions: HashMap::new(),
            calls_by_ts: HashMap::new(),
            active_floors: HashSet::new(),
        };
        Ok((entity, handle))
    }

    // Was: Führt den Arbeitsschritt `on_floor_granted` für on floor granted aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn on_floor_granted(
        &mut self,
        call_id: u16,
        source_issi: u32,
        destination_id: u32,
        destination_is_group: bool,
        carrier_num: u16,
        ts: u8,
    ) {
        if !self.handle.should_record(destination_id, destination_is_group) {
            return;
        }
        if !self.handle.has_minimum_free_space() {
            self.handle.note_error(format!(
                "minimum free space threshold reached ({} MiB); recording call {} refused",
                self.handle.config().minimum_free_space_mb,
                call_id
            ));
            return;
        }

        let session_key = (call_id, ts);
        if !self.sessions.contains_key(&session_key) {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match self.create_session(call_id, destination_id, destination_is_group, ts) {
                Ok(session) => {
                    tracing::info!(
                        "Recorder: started call_id={} destination={} type={}",
                        call_id,
                        destination_id,
                        if destination_is_group { "group" } else { "individual" }
                    );
                    self.sessions.insert(session_key, session);
                }
                Err(e) => {
                    self.handle.note_error(format!("failed to start recording call {call_id}: {e}"));
                    return;
                }
            }
        }

        if let Some(previous_call) = self.calls_by_ts.insert(ts, call_id)
            && previous_call != call_id
        {
            self.active_floors.remove(&(previous_call, ts));
        }
        self.active_floors.insert((call_id, ts));
        let result = self
            .sessions
            .get_mut(&session_key)
            .ok_or_else(|| "recording session disappeared".to_string())
            .and_then(|session| session.begin_segment(source_issi, carrier_num, ts));
        if let Err(e) = result {
            self.handle.note_error(format!("failed to begin recording segment for call {call_id}: {e}"));
            self.finish_session(session_key, "segment-error");
        }
        self.refresh_live_status();
    }

    // Was: Führt den Arbeitsschritt `on_floor_released` für on floor released aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn on_floor_released(&mut self, call_id: u16, ts: u8) {
        self.active_floors.remove(&(call_id, ts));
        if self.calls_by_ts.get(&ts).copied() == Some(call_id) {
            self.calls_by_ts.remove(&ts);
        }
        if let Some(session) = self.sessions.get_mut(&(call_id, ts)) {
            session.finish_segment();
            session.last_activity = Instant::now();
            let _ = session.write_partial_metadata();
        }
        self.refresh_live_status();
    }

    // Was: Führt den Arbeitsschritt `on_audio` für on audio aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn on_audio(&mut self, prim: TmdCircuitDataInd) {
        let Some(call_id) = self.calls_by_ts.get(&prim.ts).copied() else {
            return;
        };
        let session_key = (call_id, prim.ts);
        if !self.active_floors.contains(&session_key) {
            return;
        }
        let max_duration = Duration::from_secs(self.handle.config().max_recording_minutes as u64 * 60);
        let mut should_finish = false;
        if let Some(session) = self.sessions.get_mut(&session_key) {
            if session.started_instant.elapsed() >= max_duration {
                self.handle.note_error(format!("recording call {call_id} reached maximum duration"));
                should_finish = true;
            } else if let Err(e) = session.append_audio(&prim.data) {
                self.handle.note_error(format!("audio decode/write failed for call {call_id}: {e}"));
                should_finish = true;
            }
        }
        if should_finish {
            self.finish_session(session_key, "limit-or-error");
        }
    }

    // Was: Diese Funktion erstellt Sitzung.
    // Warum: Neue Objekte erhalten so immer einen vollständigen und gültigen Ausgangszustand.
    fn create_session(&self, call_id: u16, destination_id: u32, destination_is_group: bool, ts: u8) -> Result<RecordingSession, String> {
        let now = chrono::Local::now();
        let id = Uuid::new_v4().to_string();
        let day_dir = self
            .handle
            .root()
            .join(now.format("%Y").to_string())
            .join(now.format("%m").to_string())
            .join(now.format("%d").to_string());
        fs::create_dir_all(&day_dir).map_err(|e| e.to_string())?;
        let target_label = if destination_is_group { "GSSI" } else { "ISSI" };
        let stem = format!(
            "{}_CALL-{}_TS-{}_{}-{}_{}",
            now.format("%Y-%m-%d_%H-%M-%S"),
            call_id,
            ts,
            target_label,
            destination_id,
            id
        );
        let final_audio_path = day_dir.join(format!("{stem}.wav"));
        let part_audio_path = day_dir.join(format!("{stem}.wav.part"));
        let metadata_part_path = day_dir.join(format!("{stem}.json.part"));
        let relative_audio_path = final_audio_path
            .strip_prefix(self.handle.root())
            .map_err(|e| e.to_string())?
            .to_string_lossy()
            .replace('\\', "/");
        let writer = PcmWavWriter::create(part_audio_path).map_err(|e| e.to_string())?;
        let decoder = TetraSpeechDecoder::new().ok_or_else(|| "tetra decoder creation failed".to_string())?;
        let session = RecordingSession {
            id,
            call_id,
            destination_id,
            destination_is_group,
            started_at: now.to_rfc3339(),
            started_instant: Instant::now(),
            last_activity: Instant::now(),
            decoder,
            writer,
            final_audio_path,
            metadata_part_path,
            relative_audio_path,
            samples_written: 0,
            segments: Vec::new(),
            current_segment: None,
        };
        session.write_partial_metadata()?;
        Ok(session)
    }

    // Was: Führt den Arbeitsschritt `finish_session` für finish Sitzung aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn finish_session(&mut self, session_key: (u16, u8), reason: &str) {
        let (call_id, ts) = session_key;
        self.active_floors.remove(&session_key);
        if self.calls_by_ts.get(&ts).copied() == Some(call_id) {
            self.calls_by_ts.remove(&ts);
        }
        let Some(mut session) = self.sessions.remove(&session_key) else {
            self.refresh_live_status();
            return;
        };
        session.finish_segment();
        let id = session.id.clone();
        let final_audio_path = session.final_audio_path.clone();
        let metadata_part_path = session.metadata_part_path.clone();
        let final_metadata_path = final_audio_path.with_extension("json");
        let ended_at = chrono::Local::now().to_rfc3339();
        let mut metadata = session.metadata(ended_at, false, session.samples_written.saturating_mul(2));
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match session.writer.finalize(&final_audio_path) {
            Ok(audio_bytes) => {
                metadata.audio_bytes = audio_bytes;
                // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                match write_json_atomic(&final_metadata_path, &metadata) {
                    Ok(()) => {
                        let _ = fs::remove_file(metadata_part_path);
                        self.handle.note_completed(id);
                    }
                    Err(e) => {
                        self.handle.note_error(format!("recording metadata finalize failed for call {call_id}: {e}"));
                    }
                }
                tracing::info!(
                    "Recorder: finalized call_id={} duration_ms={} reason={} path={}",
                    call_id,
                    metadata.duration_ms,
                    reason,
                    final_audio_path.display()
                );
            }
            Err(e) => self.handle.note_error(format!("WAV finalize failed for call {call_id}: {e}")),
        }
        self.refresh_live_status();
    }

    // Was: Führt den Arbeitsschritt `finalize_expired_sessions` für finalize expired sessions aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn finalize_expired_sessions(&mut self) {
        let idle = Duration::from_secs(self.handle.config().idle_finalize_secs as u64);
        let max_duration = Duration::from_secs(self.handle.config().max_recording_minutes as u64 * 60);
        let keys: Vec<((u16, u8), &'static str)> = self
            .sessions
            .iter()
            .filter_map(|(key, session)| {
                if session.started_instant.elapsed() >= max_duration {
                    Some((*key, "maximum-duration"))
                } else if !self.active_floors.contains(key) && session.last_activity.elapsed() >= idle {
                    Some((*key, "idle-timeout"))
                } else {
                    None
                }
            })
            .collect();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for (key, reason) in keys {
            if reason == "maximum-duration" {
                self.handle.note_error(format!("recording call {} reached maximum duration", key.0));
            }
            self.finish_session(key, reason);
        }
    }

    // Was: Führt den Arbeitsschritt `finish_all` für finish all aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn finish_all(&mut self, reason: &str) {
        let keys: Vec<(u16, u8)> = self.sessions.keys().copied().collect();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for key in keys {
            self.finish_session(key, reason);
        }
    }

    // Was: Führt den Arbeitsschritt `refresh_live_status` für refresh live Status aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn refresh_live_status(&self) {
        self.handle.set_active_calls(self.sessions.keys().map(|(call_id, _)| *call_id).collect());
    }
}

impl RecorderWorker {
    // Was: Führt den Arbeitsschritt `rx_prim` für rx prim aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn on_message(&mut self, message: SapMsgInner) {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match message {
            SapMsgInner::CmceCallControl(CallControl::FloorGranted {
                call_id,
                source_issi,
                dest_gssi,
                dest_is_group,
                ts,
            }) => {
                let carrier_num = carrier_for_logical_ts(&self.config, ts);
                self.on_floor_granted(call_id, source_issi, dest_gssi, dest_is_group, carrier_num, ts);
            }
            SapMsgInner::CmceCallControl(CallControl::FloorReleased { call_id, ts }) => self.on_floor_released(call_id, ts),
            SapMsgInner::CmceCallControl(CallControl::CallEnded { call_id, .. }) => {
                let keys: Vec<(u16, u8)> = self.sessions.keys().copied().filter(|(id, _)| *id == call_id).collect();
                // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
                // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
                for key in keys {
                    self.finish_session(key, "call-ended");
                }
            }
            SapMsgInner::TmdCircuitDataInd(prim) => self.on_audio(prim),
            _ => {}
        }
    }

    // Was: Diese Funktion bearbeitet start.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn poll(&mut self) {
        let active = self.handle.is_active();
        if self.runtime_was_active && !active {
            self.finish_all("disabled-from-ui");
        }
        self.runtime_was_active = active;
        self.finalize_expired_sessions();
    }
}

// Was: Implementiert das zugehörige Verhalten für `Drop for RecorderWorker`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Drop for RecorderWorker {
    // Was: Führt den Arbeitsschritt `drop` für drop aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn drop(&mut self) {
        self.finish_all("shutdown");
    }
}

// Was: Führt den Arbeitsschritt `carrier_for_logical_ts` für carrier for logical ts aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn carrier_for_logical_ts(config: &SharedConfig, ts: u8) -> u16 {
    if ts >= 5 {
        config.config().cell.secondary_carrier.unwrap_or(config.config().cell.main_carrier)
    } else {
        config.config().cell.main_carrier
    }
}

// Was: Führt den Arbeitsschritt `samples_to_ms` für samples to ms aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn samples_to_ms(samples: u64) -> u64 {
    samples.saturating_mul(1000) / TETRA_PCM_SAMPLE_RATE as u64
}

// Was: Diese Funktion schreibt JSON-Daten atomic.
// Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
fn write_json_atomic(path: &Path, metadata: &RecordingMetadata) -> Result<(), String> {
    let body = serde_json::to_vec_pretty(metadata).map_err(|e| e.to_string())?;
    let tmp = PathBuf::from(format!("{}.tmp", path.display()));
    fs::write(&tmp, body).map_err(|e| e.to_string())?;
    fs::rename(&tmp, path).map_err(|e| e.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    fn floor(call_id: u16) -> SapMsgInner {
        SapMsgInner::CmceCallControl(CallControl::FloorGranted {
            call_id, source_issi: 5102, dest_gssi: 15201, dest_is_group: true, ts: 6,
        })
    }

    #[test]
    fn full_mailbox_never_waits_and_invalidates_stale_lifecycle_events() {
        let (tx, rx) = crossbeam_channel::bounded(1);
        let epoch = Arc::new(AtomicU64::new(0));
        let mailbox = RecorderMailbox { tx, epoch: epoch.clone() };
        mailbox.submit(floor(7));
        // No consumer is running: a blocking send would deadlock this test.
        mailbox.submit(SapMsgInner::CmceCallControl(CallControl::CallEnded { call_id: 7, ts: 6 }));
        assert_eq!(epoch.load(Ordering::SeqCst), 1);
        assert_eq!(rx.recv().unwrap().epoch, 0, "queued start is now stale");
        mailbox.submit(floor(8));
        assert_eq!(rx.recv().unwrap().epoch, 1, "new call uses a fresh epoch");
    }

    fn config(directory: &Path) -> SharedConfig {
        let mut cfg = tetra_config::bluestation::parsing::from_toml_str(r#"
config_version = "0.6"
stack_mode = "Bs"
[phy_io]
backend = "None"
[net_info]
mcc = 901
mnc = 1510
[cell_info]
main_carrier = 720
freq_band = 4
freq_offset = 0
duplex_spacing = 4
reverse_operation = false
location_area = 1
"#).unwrap();
        cfg.cell.secondary_carrier = Some(721);
        cfg.recording.enabled = true;
        cfg.recording.active = true;
        cfg.recording.directory = directory.to_str().unwrap().to_owned();
        cfg.recording.minimum_free_space_mb = 0;
        cfg.recording.retention_days = 0;
        cfg.recording.archive_enabled = false;
        cfg.recording.tts_archive_enabled = false;
        cfg.media_library.enabled = false;
        SharedConfig::from_parts(cfg, None)
    }

    #[test]
    fn shutdown_drains_audio_and_call_end_in_order() {
        let directory = std::env::temp_dir().join(format!("tetra-recorder-{}", Uuid::new_v4()));
        let (recorder, handle) = RecorderEntity::new(config(&directory)).unwrap();
        let mailbox = recorder.mailbox.as_ref().unwrap();
        for call_id in [7, 8] {
            mailbox.submit(floor(call_id));
            mailbox.submit(SapMsgInner::TmdCircuitDataInd(TmdCircuitDataInd {
                carrier_num: 721, ts: 6, data: vec![0; 274],
            }));
            if call_id == 7 {
                mailbox.submit(SapMsgInner::CmceCallControl(CallControl::CallEnded { call_id, ts: 6 }));
            }
        }
        drop(recorder); // Drains pending messages and finalizes call 8 on shutdown.
        let recordings = handle.list_recordings(None);
        assert_eq!(recordings.len(), 2, "{:?}", handle.status());
        for recording in recordings {
            assert_eq!(recording.duration_ms, 60);
            assert_eq!(recording.segments[0].carrier_num, 721);
            assert_eq!(recording.segments[0].timeslot, 6);
            assert!(directory.join(&recording.relative_audio_path).is_file());
        }
        fs::remove_dir_all(directory).unwrap();
    }
}
