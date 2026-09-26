//! Per-dialog codec worker. No codec calls, waits or joins on the RF thread.
use std::time::{Duration, Instant};
use crossbeam_channel::{Receiver, Sender, TryRecvError};
use super::audio::AsteriskAudioTranscoder;

const QUEUE_CAPACITY: usize = 8;
const MAX_AUDIO_AGE: Duration = Duration::from_millis(240);

enum MediaInput { Uplink(Vec<u8>), Downlink(Vec<u8>) }
pub(super) enum MediaOutput { Uplink(Vec<u8>), Downlink(Vec<u8>), CodecUnavailable }
struct Timed<T> { created: Instant, value: T }

pub(super) struct MediaWorker {
    tx: Sender<Timed<MediaInput>>,
    rx: Receiver<Timed<MediaOutput>>,
}

impl MediaWorker {
    pub(super) fn new() -> Option<Self> {
        let (tx, commands) = crossbeam_channel::bounded::<Timed<MediaInput>>(QUEUE_CAPACITY);
        let (results, rx) = crossbeam_channel::bounded(QUEUE_CAPACITY);
        // Dropping a dialog disconnects both channels. The worker terminates
        // independently; queued results cannot reach a later call on the same TS.
        std::thread::Builder::new().name("asterisk-codec".into()).spawn(move || {
            let Some(mut codec) = AsteriskAudioTranscoder::new() else {
                let _ = results.try_send(Timed { created: Instant::now(), value: MediaOutput::CodecUnavailable });
                return;
            };
            while let Ok(command) = commands.recv() {
                if command.created.elapsed() > MAX_AUDIO_AGE { continue; }
                let frames = match command.value {
                    MediaInput::Uplink(data) => codec.decode_tmd_to_pcmu(&data)
                        .map(MediaOutput::Uplink).into_iter().collect::<Vec<_>>(),
                    MediaInput::Downlink(data) => codec.encode_pcmu_to_tmd(&data)
                        .into_iter().map(MediaOutput::Downlink).collect(),
                };
                for frame in frames {
                    // A slow consumer drops audio, never blocks signalling or RF.
                    if let Err(crossbeam_channel::TrySendError::Disconnected(_)) =
                        results.try_send(Timed { created: command.created, value: frame }) { return; }
                }
            }
        }).ok()?;
        Some(Self { tx, rx })
    }

    pub(super) fn uplink(&self, data: Vec<u8>) -> bool {
        self.tx.try_send(Timed { created: Instant::now(), value: MediaInput::Uplink(data) }).is_ok()
    }

    pub(super) fn downlink(&self, data: &[u8]) -> bool {
        self.tx.try_send(Timed { created: Instant::now(), value: MediaInput::Downlink(data.to_vec()) }).is_ok()
    }

    pub(super) fn drain(&self) -> Vec<MediaOutput> {
        let mut output = Vec::new();
        for _ in 0..QUEUE_CAPACITY {
            match self.rx.try_recv() {
                Ok(frame) if matches!(frame.value, MediaOutput::CodecUnavailable)
                    || frame.created.elapsed() <= MAX_AUDIO_AGE => output.push(frame.value),
                Ok(_) => {},
                Err(TryRecvError::Empty | TryRecvError::Disconnected) => break,
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stalled_codec_and_consumer_never_block_rf_or_replay_old_audio() {
        let (tx, pending) = crossbeam_channel::bounded(1);
        let (results, rx) = crossbeam_channel::bounded(2);
        let worker = MediaWorker { tx, rx };
        assert!(worker.uplink(vec![0; 274]));
        assert!(!worker.downlink(&[0xff; 160]), "full queue must return immediately");
        results.send(Timed { created: Instant::now() - Duration::from_secs(1), value: MediaOutput::Downlink(vec![1]) }).unwrap();
        results.send(Timed { created: Instant::now(), value: MediaOutput::Downlink(vec![2]) }).unwrap();
        let frames = worker.drain();
        assert_eq!(frames.len(), 1);
        assert!(matches!(&frames[0], MediaOutput::Downlink(data) if data == &[2]));
        drop(worker);
        assert!(pending.recv().is_ok());
        assert!(pending.recv().is_err(), "dialog removal disconnects its codec");
        assert!(results.try_send(Timed { created: Instant::now(), value: MediaOutput::Downlink(vec![]) }).is_err());
    }

    #[test]
    fn real_codec_preserves_twenty_ms_rtp_assembly_and_sixty_ms_tetra_blocks() {
        let worker = MediaWorker::new().expect("worker spawn");
        for _ in 0..3 { assert!(worker.downlink(&[0xff; 160])); }
        let frame = worker.rx.recv_timeout(Duration::from_secs(5)).expect("encoded 60 ms block");
        let MediaOutput::Downlink(tetra) = frame.value else { panic!("codec unavailable") };
        assert_eq!(tetra.len(), 35, "274 speech bits packed into 35 bytes");
        assert!(worker.uplink(tetra));
        let frame = worker.rx.recv_timeout(Duration::from_secs(5)).expect("decoded block");
        let MediaOutput::Uplink(pcmu) = frame.value else { panic!("wrong direction") };
        assert_eq!(pcmu.len(), 480);
    }
}
