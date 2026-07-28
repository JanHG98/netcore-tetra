// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use tetra_core::TdmaTime;

use tetra_pdus::phy::traits::rxtx_dev::TxSlotBits;

use crate::phy::components::dsp_types::*;
use crate::phy::components::fir;
use crate::phy::components::modem_common::*;

/// Samples per symbol
// Was: Legt den festen Wert `SPS` für sps fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const SPS: SampleCount = 4;

/// Samples per slot
// Was: Legt den festen Wert `SAMPLES_SLOT` für samples slot fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const SAMPLES_SLOT: SampleCount = SPS * 255;

/// Output sample rate
// Was: Legt den festen Wert `SAMPLE_RATE` für sample rate fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SAMPLE_RATE: f64 = 18000.0 * SPS as f64;

#[derive(PartialEq)]
// Was: Listet die möglichen Varianten für mode auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum Mode {
    /// Downlink modulation.
    Dl,
}

// Was: Bündelt die zusammengehörigen Werte für modulator in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct Modulator {
    mode: Mode,
    /// Sample counter value at the beginning of hyperframe number 0
    reference_time: SampleCount,
    /// Pulse shaping filter
    filter: fir::FirComplexSym,
    dqpsk: DqpskMapper,
}

// Was: Listet die möglichen Varianten für error auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum Error {
    /// Modulator needs data for another slot
    /// before it can continue producing TX signal.
    NeedMoreData,
}

// Was: Implementiert das zugehörige Verhalten für `Modulator`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Modulator {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            reference_time: 0,
            filter: fir::FirComplexSym::new(CHANNEL_FILTER_TAPS.len()),
            dqpsk: DqpskMapper::new(),
        }
    }

    /// Produce one output sample.
    // Was: Führt den Arbeitsschritt `sample` für sample aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn sample(&mut self, sample_counter: SampleCount, tx_slot: &TxSlotBits) -> Result<ComplexSample, Error> {
        // Compensate for delay of pulse shaping filter in sample count
        let sample_counter = sample_counter + CHANNEL_FILTER_TAPS.len() as SampleCount;

        // Sample counter at beginning of current slot.
        // TODO: adjust self.reference_time when hyperframe number wraps to 0.
        // Now it breaks after 46 days.
        // This could also be further optimized by computing and storing it
        // only when a new slot becomes available.
        let slot_begin = self.reference_time + TdmaTime::to_int(tx_slot.time) as SampleCount * SAMPLES_SLOT;

        let mut sample = ComplexSample::ZERO;
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self.mode {
            Mode::Dl => {
                let sample_in_slot = sample_counter - slot_begin;
                if sample_in_slot < 0 {
                    // Slot is in the future.
                    // Transmit silence until we reach the slot.
                } else if sample_in_slot >= SAMPLES_SLOT {
                    // Slot is in the past, so it has already been transmitted.
                    // Return and wait for data for the next slot to be available.
                    return Err(Error::NeedMoreData);
                } else if let Some(bits) = tx_slot.slot {
                    if sample_in_slot % SPS == 0 {
                        let symbol_i = (sample_in_slot / SPS) as usize;
                        sample = self.dqpsk.symbol(bits[symbol_i * 2] != 0, bits[symbol_i * 2 + 1] != 0);
                    }
                }
            }
        }
        Ok(self.filter.sample(&CHANNEL_FILTER_TAPS, sample))
    }
}

// Was: Bündelt die zusammengehörigen Werte für dqpsk mapper in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct DqpskMapper {
    pub phase: i8,
}

// Was: Implementiert das zugehörige Verhalten für `DqpskMapper`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl DqpskMapper {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new() -> Self {
        Self { phase: 0 }
    }

    #[allow(dead_code)]
    // Was: Diese Funktion setzt phase.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn reset_phase(&mut self) {
        self.phase = 0;
    }

    // Was: Führt den Arbeitsschritt `symbol` für symbol aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn symbol(&mut self, bit0: bool, bit1: bool) -> ComplexSample {
        self.phase = (self.phase
            + match (bit0, bit1) {
                (true, true) => -3,
                (true, false) => -1,
                (false, false) => 1,
                (false, true) => 3,
            })
            & 7;
        // Look-up table to map phase (in multiples of pi/4)
        // to constellation points. Generated in Python with:
        // import numpy as np
        // print(",\n".join("ComplexSample{ re: %9.6f, im: %9.6f }" % (v.real, v.imag) for v in np.exp(1j*np.linspace(0, np.pi*2, 8, endpoint=False))))
        // The 0.707107 entries are the π/4-DQPSK diagonal constellation points (= 1/√2).
        // Kept as the exact literals emitted by the Python generator above rather than
        // f32::consts::FRAC_1_SQRT_2, so the table matches the documented source verbatim.
        #[allow(clippy::approx_constant)]
        // Was: Legt den festen Wert `CONSTELLATION` für constellation fest.
        // Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
        const CONSTELLATION: [ComplexSample; 8] = [
            ComplexSample {
                re: 1.000000,
                im: 0.000000,
            },
            ComplexSample {
                re: 0.707107,
                im: 0.707107,
            },
            ComplexSample {
                re: 0.000000,
                im: 1.000000,
            },
            ComplexSample {
                re: -0.707107,
                im: 0.707107,
            },
            ComplexSample {
                re: -1.000000,
                im: 0.000000,
            },
            ComplexSample {
                re: -0.707107,
                im: -0.707107,
            },
            ComplexSample {
                re: -0.000000,
                im: -1.000000,
            },
            ComplexSample {
                re: 0.707107,
                im: -0.707107,
            },
        ];
        CONSTELLATION[self.phase as usize]
    }
}
