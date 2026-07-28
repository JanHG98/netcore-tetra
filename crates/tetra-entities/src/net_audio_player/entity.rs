// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::VecDeque;
use std::thread;
use std::time::{Duration, Instant};

use tetra_config::bluestation::SharedConfig;
use tetra_core::{Sap, TdmaTime, tetra_entities::TetraEntity};
use tetra_pdus::cmce::enums::{call_timeout::CallTimeout, disconnect_cause::DisconnectCause};
use tetra_saps::{
    SapMsg, SapMsgInner,
    control::call_control::{CallControl, NetworkCircuitCall},
    tmd::TmdCircuitDataReq,
};
use uuid::Uuid;

use crate::{MessageQueue, TetraEntityTrait};

use super::media::prepare_audio;
use super::service::{AudioPlayerHandle, detect_ffmpeg};
use super::types::{
    AudioPlayerCommand, AudioPlayerState, AudioTargetType, PrepareEvent, PreparedAudio,
};

// Was: Legt den festen Wert `GROUP_CALL_PREPARE_SETTLE` für Gruppe Ruf prepare settle fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const GROUP_CALL_PREPARE_SETTLE: Duration = Duration::from_millis(1000);

// Was: Bündelt die zusammengehörigen Werte für pending playback in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct PendingPlayback {
    prepared: PreparedAudio,
    not_before: Instant,
}

// Was: Bündelt die zusammengehörigen Werte für active playback in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct ActivePlayback {
    job_id: String,
    call_uuid: Uuid,
    target_type: AudioTargetType,
    blocks: VecDeque<Vec<u8>>,
    total_blocks: usize,
    sent_blocks: usize,
    ts: Option<u8>,
    phase_started: Instant,
    finishing: bool,
    finish_error: Option<String>,
}

// Was: Bündelt die zusammengehörigen Werte für audio player entity in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct AudioPlayerEntity {
    config: SharedConfig,
    handle: AudioPlayerHandle,
    command_rx: crossbeam_channel::Receiver<AudioPlayerCommand>,
    prepare_tx: crossbeam_channel::Sender<PrepareEvent>,
    prepare_rx: crossbeam_channel::Receiver<PrepareEvent>,
    current_job_id: Option<String>,
    pending_playback: Option<PendingPlayback>,
    playback: Option<ActivePlayback>,
    dltime: TdmaTime,
}

// Was: Implementiert das zugehörige Verhalten für `AudioPlayerEntity`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl AudioPlayerEntity {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(config: SharedConfig) -> Result<(Self, AudioPlayerHandle), String> {
        let player_config = config.config().audio_player.clone();
        let (command_tx, command_rx) = crossbeam_channel::unbounded();
        let (prepare_tx, prepare_rx) = crossbeam_channel::unbounded();
        let ffmpeg_available = detect_ffmpeg(&player_config.ffmpeg_path);
        let handle = AudioPlayerHandle::new(player_config, command_tx, ffmpeg_available)?;
        Ok((
            Self {
                config,
                handle: handle.clone(),
                command_rx,
                prepare_tx,
                prepare_rx,
                current_job_id: None,
                pending_playback: None,
                playback: None,
                dltime: TdmaTime::default(),
            },
            handle,
        ))
    }

    // Was: Diese Funktion verarbeitet commands.
    // Warum: Die einzelnen Verarbeitungsschritte bleiben damit gebündelt und leichter testbar.
    fn process_commands(&mut self, queue: &mut MessageQueue) {
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        while let Ok(command) = self.command_rx.try_recv() {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match command {
                AudioPlayerCommand::Play {
                    job_id,
                    source,
                    target_type,
                    target_id,
                    priority,
                } => {
                    if self.current_job_id.is_some() || self.playback.is_some() {
                        self.handle.mark_failed("an audio transmission is already active");
                        continue;
                    }
                    self.current_job_id = Some(job_id.clone());
                    self.handle.mark_preparing(
                        job_id.clone(),
                        source.display_name.clone(),
                        source.source_type,
                        source.source_id.clone(),
                        target_type,
                        target_id,
                        priority,
                    );
                    let config = self.handle.config().clone();
                    let tx = self.prepare_tx.clone();
                    if let Err(error) = thread::Builder::new()
                        .name("audio-player-prepare".into())
                        .spawn(move || {
                            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                            let event = match prepare_audio(&config, job_id.clone(), source, target_type, target_id, priority) {
                                Ok(prepared) => PrepareEvent::Ready(prepared),
                                Err(error) => PrepareEvent::Failed { job_id, error },
                            };
                            let _ = tx.send(event);
                        })
                    {
                        self.current_job_id = None;
                        self.handle.mark_failed(format!("failed to start audio preparation worker: {error}"));
                    }
                }
                AudioPlayerCommand::Stop => self.stop_current(queue, "stopped from dashboard"),
            }
        }
    }

    // Was: Diese Funktion verarbeitet prepare events.
    // Warum: Die einzelnen Verarbeitungsschritte bleiben damit gebündelt und leichter testbar.
    fn process_prepare_events(&mut self, _queue: &mut MessageQueue) {
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        while let Ok(event) = self.prepare_rx.try_recv() {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match event {
                PrepareEvent::Ready(prepared) => {
                    if self.current_job_id.as_deref() != Some(prepared.job_id.as_str()) {
                        tracing::debug!("AudioPlayer: ignoring stale prepared job {}", prepared.job_id);
                        continue;
                    }
                    let delayed = prepared.target_type == AudioTargetType::Group;
                    let not_before = if delayed {
                        Instant::now() + GROUP_CALL_PREPARE_SETTLE
                    } else {
                        Instant::now()
                    };
                    tracing::info!(
                        "AudioPlayer: prepared media queued for common launch gate job={} target={:?}:{} settle_ms={}",
                        prepared.job_id,
                        prepared.target_type,
                        prepared.target_id,
                        if delayed { GROUP_CALL_PREPARE_SETTLE.as_millis() } else { 0 }
                    );
                    self.pending_playback = Some(PendingPlayback { prepared, not_before });
                }
                PrepareEvent::Failed { job_id, error } => {
                    if self.current_job_id.as_deref() != Some(job_id.as_str()) {
                        continue;
                    }
                    self.current_job_id = None;
                    self.handle.mark_failed(error);
                }
            }
        }
    }


    // Was: Diese Funktion startet pending.
    // Warum: Der Dienst oder Teilprozess wird so in einer festen und überprüfbaren Reihenfolge gestartet.
    fn try_start_pending(&mut self, queue: &mut MessageQueue) {
        let should_start = self.pending_playback.as_ref().is_some_and(|pending| {
            if Instant::now() < pending.not_before {
                return false;
            }
            pending.prepared.target_type != AudioTargetType::Group || group_call_launch_slot(self.dltime)
        });
        if !should_start {
            return;
        }
        let Some(pending) = self.pending_playback.take() else {
            return;
        };
        tracing::info!(
            "AudioPlayer: launching prepared media through common recording/TTS gate job={} target={:?}:{} dltime={:?}",
            pending.prepared.job_id,
            pending.prepared.target_type,
            pending.prepared.target_id,
            self.dltime
        );
        self.start_prepared(queue, pending.prepared);
    }

    // Was: Diese Funktion startet prepared.
    // Warum: Der Dienst oder Teilprozess wird so in einer festen und überprüfbaren Reihenfolge gestartet.
    fn start_prepared(&mut self, queue: &mut MessageQueue, prepared: PreparedAudio) {
        let call_uuid = Uuid::new_v4();
        let total_blocks = prepared.blocks.len();
        self.handle.mark_prepared(prepared.duration_ms, total_blocks);
        self.handle.set_state(AudioPlayerState::Calling);
        self.playback = Some(ActivePlayback {
            job_id: prepared.job_id.clone(),
            call_uuid,
            target_type: prepared.target_type,
            blocks: VecDeque::from(prepared.blocks),
            total_blocks,
            sent_blocks: 0,
            ts: None,
            phase_started: Instant::now(),
            finishing: false,
            finish_error: None,
        });

        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match prepared.target_type {
            AudioTargetType::Group => queue.push_back(SapMsg {
                sap: Sap::Control,
                src: TetraEntity::AudioPlayer,
                dest: TetraEntity::Cmce,
                msg: SapMsgInner::CmceCallControl(CallControl::NetworkCallStart {
                    brew_uuid: call_uuid,
                    source_issi: self.handle.config().source_issi,
                    dest_gssi: prepared.target_id,
                    priority: prepared.priority,
                }),
            }),
            AudioTargetType::Individual => {
                let call = NetworkCircuitCall {
                    source_issi: self.handle.config().source_issi,
                    destination: prepared.target_id,
                    number: "NetCore Audio".to_string(),
                    priority: prepared.priority,
                    service: 0,
                    mode: 0,
                    duplex: 0,
                    method: 0,
                    communication: 0,
                    grant: 0,
                    permission: 0,
                    timeout: CallTimeout::Infinite.into_raw() as u8,
                    ownership: 0,
                    queued: 0,
                };
                queue.push_back(SapMsg {
                    sap: Sap::Control,
                    src: TetraEntity::AudioPlayer,
                    dest: TetraEntity::Cmce,
                    msg: SapMsgInner::CmceCallControl(CallControl::NetworkCircuitSetupRequest {
                        brew_uuid: call_uuid,
                        call,
                    }),
                });
            }
        }
        tracing::info!(
            "AudioPlayer: prepared job={} target={:?}:{} blocks={}",
            prepared.job_id,
            prepared.target_type,
            prepared.target_id,
            total_blocks
        );
    }

    // Was: Führt den Arbeitsschritt `playout` für playout aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn playout(&mut self, queue: &mut MessageQueue) {
        if self.dltime.f == 18 {
            return;
        }
        let mut should_finish = false;
        let mut frame_to_send: Option<(u8, Vec<u8>, usize)> = None;
        if let Some(playback) = self.playback.as_mut() {
            if playback.finishing {
                return;
            }
            let Some(ts) = playback.ts else {
                return;
            };
            if ts != self.dltime.t {
                return;
            }
            if let Some(block) = playback.blocks.pop_front() {
                playback.sent_blocks = playback.sent_blocks.saturating_add(1);
                frame_to_send = Some((ts, block, playback.sent_blocks));
                should_finish = playback.sent_blocks >= playback.total_blocks;
            } else {
                should_finish = true;
            }
        }
        if let Some((ts, block, sent_blocks)) = frame_to_send {
            queue.push_back(SapMsg {
                sap: Sap::TmdSap,
                src: TetraEntity::AudioPlayer,
                dest: TetraEntity::Umac,
                msg: SapMsgInner::TmdCircuitDataReq(TmdCircuitDataReq {
                    carrier_num: carrier_for_logical_ts(&self.config, ts),
                    ts,
                    data: block,
                }),
            });
            self.handle.mark_progress(sent_blocks);
        }
        if should_finish {
            self.begin_finish(queue);
        }
    }

    // Was: Führt den Arbeitsschritt `begin_finish` für begin finish aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn begin_finish(&mut self, queue: &mut MessageQueue) {
        self.request_finish(queue, "end of file", None);
    }

    // Was: Diese Funktion fordert finish.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn request_finish(&mut self, queue: &mut MessageQueue, reason: &str, finish_error: Option<String>) {
        let Some(playback) = self.playback.as_mut() else {
            // A stop during asynchronous preparation or the common launch gate cancels the
            // pending job. Any later worker result is ignored because current_job_id no longer
            // matches it.
            self.pending_playback = None;
            self.current_job_id = None;
            self.handle.mark_idle();
            return;
        };
        if playback.finishing {
            return;
        }
        playback.finishing = true;
        playback.finish_error = finish_error;
        playback.phase_started = Instant::now();
        let target_type = playback.target_type;
        let call_uuid = playback.call_uuid;
        let job_id = playback.job_id.clone();
        self.handle.set_state(AudioPlayerState::Finishing);
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match target_type {
            AudioTargetType::Group => queue.push_back(SapMsg {
                sap: Sap::Control,
                src: TetraEntity::AudioPlayer,
                dest: TetraEntity::Cmce,
                msg: SapMsgInner::CmceCallControl(CallControl::NetworkCallEnd { brew_uuid: call_uuid }),
            }),
            AudioTargetType::Individual => queue.push_back(SapMsg {
                sap: Sap::Control,
                src: TetraEntity::AudioPlayer,
                dest: TetraEntity::Cmce,
                msg: SapMsgInner::CmceCallControl(CallControl::NetworkCircuitRelease {
                    brew_uuid: call_uuid,
                    cause: DisconnectCause::SwmiRequestedDisconnection.into_raw() as u8,
                }),
            }),
        }
        tracing::info!("AudioPlayer: finishing job={} ({})", job_id, reason);
    }

    // Was: Diese Funktion stoppt current.
    // Warum: Der Betrieb wird dadurch geordnet beendet und Ressourcen werden freigegeben.
    fn stop_current(&mut self, queue: &mut MessageQueue, reason: &str) {
        self.request_finish(queue, reason, None);
    }

    // Was: Führt den Arbeitsschritt `fail_current` für fail current aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fail_current(&mut self, error: impl Into<String>) {
        self.pending_playback = None;
        self.playback = None;
        self.current_job_id = None;
        self.handle.mark_failed(error);
    }

    // Was: Führt den Arbeitsschritt `complete_current` für complete current aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn complete_current(&mut self) {
        let finish_error = self.playback.take().and_then(|playback| {
            tracing::info!(
                "AudioPlayer: completed job={} blocks={}/{}",
                playback.job_id,
                playback.sent_blocks,
                playback.total_blocks
            );
            playback.finish_error
        });
        self.current_job_id = None;
        if let Some(error) = finish_error {
            self.handle.mark_failed(error);
        } else {
            self.handle.mark_idle();
        }
    }

    // Was: Führt den Arbeitsschritt `lifecycle_timeout` für lifecycle timeout aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn lifecycle_timeout(&mut self, queue: &mut MessageQueue) {
        let (finish_timeout, setup_timeout) = self
            .playback
            .as_ref()
            .map(|playback| {
                let elapsed = playback.phase_started.elapsed();
                // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                let finish_guard_seconds = match playback.target_type {
                    AudioTargetType::Group => self.handle.config().group_release_guard_seconds as u64,
                    AudioTargetType::Individual => 3,
                };
                (
                    playback.finishing && elapsed >= Duration::from_secs(finish_guard_seconds),
                    !playback.finishing
                        && playback.ts.is_none()
                        && elapsed >= Duration::from_secs(self.handle.config().individual_answer_timeout_seconds as u64),
                )
            })
            .unwrap_or((false, false));
        if finish_timeout {
            self.complete_current();
        } else if setup_timeout {
            self.request_finish(
                queue,
                "call setup/answer timeout",
                Some("call setup or individual answer timed out".to_string()),
            );
        }
    }

    // Was: Führt den Arbeitsschritt `current_matches` für current matches aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn current_matches(&self, uuid: Uuid) -> bool {
        self.playback.as_ref().is_some_and(|playback| playback.call_uuid == uuid)
    }

    // Was: Führt den Arbeitsschritt `media_ready` für Audio- und Mediendaten ready aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn media_ready(&mut self, uuid: Uuid, call_id: u16, ts: u8) {
        let Some(playback) = self.playback.as_mut() else {
            return;
        };
        if playback.call_uuid != uuid {
            return;
        }
        playback.ts = Some(ts);
        playback.phase_started = Instant::now();
        self.handle.mark_media_ready(call_id, ts);
        tracing::info!("AudioPlayer: media ready uuid={} call_id={} ts={}", uuid, call_id, ts);
    }
}

// Was: Implementiert das zugehörige Verhalten für `TetraEntityTrait for AudioPlayerEntity`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl TetraEntityTrait for AudioPlayerEntity {
    // Was: Führt den Arbeitsschritt `entity` für entity aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn entity(&self) -> TetraEntity {
        TetraEntity::AudioPlayer
    }

    // Was: Führt den Arbeitsschritt `rx_prim` für rx prim aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn rx_prim(&mut self, queue: &mut MessageQueue, message: SapMsg) {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match message.msg {
            SapMsgInner::CmceCallControl(CallControl::NetworkCallReady {
                brew_uuid,
                call_id,
                ts,
                ..
            }) => self.media_ready(brew_uuid, call_id, ts),
            SapMsgInner::CmceCallControl(CallControl::NetworkCallEnd { brew_uuid }) => {
                if self.current_matches(brew_uuid) {
                    let finishing = self.playback.as_ref().is_some_and(|playback| playback.finishing);
                    if finishing {
                        self.complete_current();
                    } else {
                        self.fail_current("group call ended before audio playout completed");
                    }
                }
            }
            SapMsgInner::CmceCallControl(CallControl::NetworkCircuitSetupAccept { brew_uuid }) => {
                if self.current_matches(brew_uuid) {
                    self.handle.set_state(AudioPlayerState::WaitingForAnswer);
                    if let Some(playback) = self.playback.as_mut() {
                        playback.phase_started = Instant::now();
                    }
                }
            }
            SapMsgInner::CmceCallControl(CallControl::NetworkCircuitSetupReject { brew_uuid, cause }) => {
                if self.current_matches(brew_uuid) {
                    self.fail_current(format!("individual call rejected (cause {cause})"));
                }
            }
            SapMsgInner::CmceCallControl(CallControl::NetworkCircuitAlert { brew_uuid }) => {
                if self.current_matches(brew_uuid) {
                    self.handle.set_state(AudioPlayerState::WaitingForAnswer);
                }
            }
            SapMsgInner::CmceCallControl(CallControl::NetworkCircuitConnectRequest { brew_uuid, .. }) => {
                if self.current_matches(brew_uuid) {
                    queue.push_back(SapMsg {
                        sap: Sap::Control,
                        src: TetraEntity::AudioPlayer,
                        dest: TetraEntity::Cmce,
                        msg: SapMsgInner::CmceCallControl(CallControl::NetworkCircuitConnectConfirm {
                            brew_uuid,
                            grant: 0,
                            permission: 0,
                        }),
                    });
                }
            }
            SapMsgInner::CmceCallControl(CallControl::NetworkCircuitMediaReady {
                brew_uuid,
                call_id,
                ts,
            }) => self.media_ready(brew_uuid, call_id, ts),
            SapMsgInner::CmceCallControl(CallControl::NetworkCircuitRelease { brew_uuid, cause }) => {
                if self.current_matches(brew_uuid) {
                    let finishing = self.playback.as_ref().is_some_and(|playback| playback.finishing);
                    if finishing {
                        self.complete_current();
                    } else {
                        self.fail_current(format!("individual call released (cause {cause})"));
                    }
                }
            }
            _ => {}
        }
    }

    // Was: Diese Funktion bearbeitet start.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn tick_start(&mut self, queue: &mut MessageQueue, ts: TdmaTime) {
        self.dltime = ts;
        self.process_commands(queue);
        self.process_prepare_events(queue);
        self.try_start_pending(queue);
        self.lifecycle_timeout(queue);
        self.playout(queue);
    }
}

// Was: Implementiert das zugehörige Verhalten für `Drop for AudioPlayerEntity`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Drop for AudioPlayerEntity {
    // Was: Führt den Arbeitsschritt `drop` für drop aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn drop(&mut self) {
        self.current_job_id = None;
        self.pending_playback = None;
        self.playback = None;
        self.handle.mark_idle();
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


// Was: Führt den Arbeitsschritt `group_call_launch_slot` für Gruppe Ruf launch slot aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn group_call_launch_slot(ts: TdmaTime) -> bool {
    ts.t == 4 && !matches!(ts.f, 1 | 17 | 18)
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::group_call_launch_slot;
    use tetra_core::TdmaTime;

    #[test]
    // Was: Führt den Arbeitsschritt `group_launch_gate_accepts_clean_timeslot_four` für Gruppe launch gate accepts clean timeslot four aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn group_launch_gate_accepts_clean_timeslot_four() {
        assert!(group_call_launch_slot(TdmaTime { h: 0, m: 1, f: 2, t: 4 }));
        assert!(group_call_launch_slot(TdmaTime { h: 0, m: 1, f: 16, t: 4 }));
    }

    #[test]
    // Was: Führt den Arbeitsschritt `group_launch_gate_rejects_special_frames_and_other_slots` für Gruppe launch gate rejects special frames and und weitere Angaben aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn group_launch_gate_rejects_special_frames_and_other_slots() {
        assert!(!group_call_launch_slot(TdmaTime { h: 0, m: 1, f: 1, t: 4 }));
        assert!(!group_call_launch_slot(TdmaTime { h: 0, m: 1, f: 17, t: 4 }));
        assert!(!group_call_launch_slot(TdmaTime { h: 0, m: 1, f: 18, t: 4 }));
        assert!(!group_call_launch_slot(TdmaTime { h: 0, m: 1, f: 2, t: 1 }));
    }
}
