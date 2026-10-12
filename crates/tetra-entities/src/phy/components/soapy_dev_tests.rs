//! End-to-end TX waveform checks without opening an SDR device.
use super::*;
use tetra_core::TdmaTime;

struct Waveform {
    bank: fcfb::SynthesisOutputProcessor,
    channels: Vec<ModulatorChannel>,
    block: fcfb::BlockCount,
    samples: Vec<ComplexSample>,
}

impl Waveform {
    fn new(carriers: &[u16]) -> Self {
        let mut planner = FftPlanner::new();
        // Production SXceiver rate: 900 output / 108 modem samples per block.
        // A 1020-sample TDMA slot therefore spans a partial block at its end.
        let params = fcfb::SynthesisOutputParameters {
            ifft_size: 1200,
            sample_rate: 600_000.0,
            center_frequency: 418_000_000.0,
            overlap: fcfb::Overlap::O1_4,
        };
        let bank = fcfb::SynthesisOutputProcessor::new(&mut planner, params);
        let channels = carriers
            .iter()
            .map(|&carrier| {
                let frequency = 418_000_000.0 + f64::from(carrier - 720) * 25_000.0;
                ModulatorChannel::new(&mut planner, params, frequency, carrier)
            })
            .collect();
        Self { bank, channels, block: 0, samples: Vec::new() }
    }

    fn push(&mut self, slots: &[TxSlotBits], retries: usize) {
        let limit = self.block + 20;
        while modulate_tx_block(&mut self.bank, &mut self.channels, self.block, slots) {
            self.samples.extend_from_slice(self.bank.process());
            self.block += 1;
            assert!(self.block < limit, "TX did not stop at the end of the supplied timeslot");
        }
        // Model repeated polling while RX catches up and no new TX slot is ready.
        for _ in 0..retries {
            assert!(!modulate_tx_block(&mut self.bank, &mut self.channels, self.block, slots));
        }
    }
}

fn bursts() -> [[u8; 510]; 2] {
    std::array::from_fn(|carrier| std::array::from_fn(|bit| ((bit / (carrier + 1) + bit / 17) % 2) as u8))
}

fn batch(bits: &[[u8; 510]; 2], time: i32, active: [bool; 2]) -> [TxSlotBits<'_>; 2] {
    std::array::from_fn(|i| TxSlotBits {
        carrier_num: 720 + i as u16,
        time: TdmaTime::from_int(time),
        slot: active[i].then_some(bits[i].as_slice()),
    })
}

fn assert_waveforms_match(actual: &[ComplexSample], expected: &[ComplexSample]) {
    assert!(!actual.is_empty());
    assert_eq!(actual.len(), expected.len(), "TX timeline changed");
    assert!(actual.iter().chain(expected).all(|v| v.re.is_finite() && v.im.is_finite()));
    let error = actual.iter().zip(expected).map(|(a, b)| (*a - *b).norm()).fold(0.0_f32, f32::max);
    assert!(error < 0.00001, "waveforms differ: max IQ error = {error}");
}

#[test]
fn tx_carriers_match_sum_of_independent_waveforms_across_slots() {
    let bits = bursts();
    let mut dual = Waveform::new(&[720, 721]);
    let mut primary = Waveform::new(&[720]);
    let mut secondary = Waveform::new(&[721]);
    for time in 0..18 {
        let slots = batch(&bits, time, [true, true]);
        dual.push(&slots, 0);
        primary.push(&slots[..1], 0);
        secondary.push(&slots[1..], 0);
    }
    assert_eq!(primary.samples.len(), secondary.samples.len());
    let sum = primary.samples.iter().zip(&secondary.samples).map(|(a, b)| *a + *b).collect::<Vec<_>>();
    assert!(primary.samples.iter().any(|v| v.norm() > 0.01));
    assert!(secondary.samples.iter().any(|v| v.norm() > 0.01));
    assert_waveforms_match(&dual.samples, &sum);
}

#[test]
fn tx_carrier_assignment_is_independent_of_batch_order() {
    let bits = bursts();
    let mut ordered = Waveform::new(&[720, 721]);
    let mut reordered = Waveform::new(&[720, 721]);
    for time in 0..18 {
        let mut slots = batch(&bits, time, [true, true]);
        ordered.push(&slots, 0);
        if time % 2 == 0 {
            slots.reverse();
        }
        reordered.push(&slots, 0);
    }
    assert_waveforms_match(&reordered.samples, &ordered.samples);
}

#[test]
fn tx_missing_carriers_match_explicit_silence_and_resume_cleanly() {
    let bits = bursts();
    let mut explicit = Waveform::new(&[720, 721]);
    let mut sparse = Waveform::new(&[720, 721]);
    for time in 0..18 {
        // Exercise both carriers individually, together and with both idle.
        let active = match time % 4 {
            0 => [true, true],
            1 => [false, true],
            2 => [false, false],
            _ => [true, false],
        };
        let slots = batch(&bits, time, active);
        explicit.push(&slots, 0);
        let sparse_slots = if active == [false, false] {
            // Even an all-idle batch needs one timestamp to delimit its silence.
            vec![TxSlotBits { carrier_num: 720, time: slots[0].time, slot: None }]
        } else {
            slots.into_iter().filter(|slot| slot.slot.is_some()).collect()
        };
        sparse.push(&sparse_slots, 0);
    }
    assert_waveforms_match(&sparse.samples, &explicit.samples);
}

#[test]
fn tx_repeated_polls_and_empty_batches_do_not_change_waveform() {
    let bits = bursts();
    let mut once = Waveform::new(&[720, 721]);
    let mut polled = Waveform::new(&[720, 721]);
    for time in 0..18 {
        let slots = batch(&bits, time, [true, true]);
        once.push(&slots, 0);
        polled.push(&slots, 5);
        polled.push(&[], 2);
    }
    assert_waveforms_match(&polled.samples, &once.samples);
}

#[test]
fn tx_late_skip_discards_partial_block_and_filter_history() {
    let bits = bursts();
    let mut skipped = Waveform::new(&[720, 721]);
    skipped.push(&batch(&bits, 0, [true, true]), 0);
    assert_eq!(skipped.block, 9); // Block 9 straddles the first slot boundary.
    assert!(skipped.channels.iter().all(|channel| channel.buffer_i > 0));

    // Hardware has advanced beyond the pending block. Compare to a clean start
    // at that exact absolute sample time, not to replayed samples from block 9.
    skipped.block = 20;
    skipped.samples.clear();
    let mut fresh = Waveform::new(&[720, 721]);
    fresh.block = 20;
    for time in 2..5 {
        let slots = batch(&bits, time, [true, true]);
        skipped.push(&slots, 0);
        fresh.push(&slots, 0);
    }
    assert_waveforms_match(&skipped.samples, &fresh.samples);
}

#[test]
fn tx_skip_burst_is_rate_limited_without_losing_cumulative_evidence() {
    let mut diagnostics = TxDiagnostics::default();
    let mut warning_blocks = Vec::new();
    let interval = 3333; // Production SXceiver: approximately five SDR seconds.
    for next_block in 900..10_900 {
        let skipped = if next_block == 900 { 900 } else { 1 };
        if diagnostics.record_late_skip(next_block, skipped, interval) {
            warning_blocks.push(next_block);
        }
    }
    assert_eq!(diagnostics.late_skip_events, 10_000);
    assert_eq!(diagnostics.late_skipped_blocks, 10_899);
    assert_eq!(warning_blocks, [900, 4233, 7566, 10_899]);
    assert_eq!(diagnostics.written_blocks, 0);
}

#[test]
fn tx_failed_write_is_not_counted_as_successful_production() {
    let mut diagnostics = TxDiagnostics::default();
    assert!(diagnostics.record_write_result(Ok(())).is_ok());
    assert!(matches!(
        diagnostics.record_write_result(Err(RxTxDevError::RxReadError)),
        Err(RxTxDevError::RxReadError)
    ));
    assert_eq!(diagnostics.write_attempts, 2);
    assert_eq!(diagnostics.written_blocks, 1);
}

#[test]
fn tx_stale_slots_show_waits_without_claiming_driver_writes() {
    let bits = bursts();
    let mut waveform = Waveform::new(&[720, 721]);
    let mut diagnostics = TxDiagnostics::default();
    // Model one skipped RX timeslot: the router remains one slot behind the
    // completed RX slot. Exercise the real modulator gate, not a clock mock.
    for router_slot in 100..1100 {
        let actual_rx_slot = router_slot + 1;
        let modem_sample = (actual_rx_slot + 1) * 1020 + 16;
        waveform.block = (modem_sample / 108 + 2) as fcfb::BlockCount;
        let slots = batch(&bits, router_slot + 1, [true, true]);
        assert!(!modulate_tx_block(&mut waveform.bank, &mut waveform.channels, waveform.block, &slots));
        diagnostics.record_wait(TxWaitReason::SlotNotReady);
    }
    assert_eq!(diagnostics.slot_not_ready_returns, 1000);
    assert_eq!(diagnostics.hw_ahead_returns, 0);
    assert_eq!(diagnostics.rx_ahead_returns, 0);
    assert_eq!(diagnostics.write_attempts, 0);
    assert_eq!(diagnostics.written_blocks, 0);
}

#[test]
fn tx_slot_time_report_keeps_each_supplied_carrier_time() {
    let slots = [
        TxSlotBits { carrier_num: 721, time: TdmaTime::from_int(7), slot: None },
        TxSlotBits { carrier_num: 720, time: TdmaTime::from_int(8), slot: None },
    ];
    let expected = format!("{:?}", [(721_u16, TdmaTime::from_int(7)), (720_u16, TdmaTime::from_int(8))]);
    assert_eq!(format!("{:?}", TxSlotTimes(&slots)), expected);
}
