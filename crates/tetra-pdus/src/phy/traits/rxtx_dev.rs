// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use tetra_core::TdmaTime;
use tetra_core::TrainingSequence;

#[derive(Debug, PartialEq, Clone, Copy)]
// Was: Listet die möglichen Varianten für rx tx dev error auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum RxTxDevError {
    RxEndOfData,
    RxReadError,
}

#[derive(Debug, Default)]
// Was: Bündelt die zusammengehörigen Werte für rx burst bits in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RxBurstBits<'a> {
    pub train_type: TrainingSequence,
    pub bits: &'a [u8],
    /// Received signal strength in dBFS (dB relative to ADC full-scale).
    /// 0.0 = full scale, negative = weaker signal. Not calibrated to dBm.
    pub rssi_dbfs: f32,
}

#[derive(Debug, Default)]
// Was: Bündelt die zusammengehörigen Werte für rx slot bits in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RxSlotBits<'a> {
    /// Carrier number from which this slot was received.
    pub carrier_num: u16,
    /// Number of slot received
    pub time: TdmaTime,
    /// Burst received in full slot
    pub slot: RxBurstBits<'a>,
    /// Burst received in subslot 1
    pub subslot1: RxBurstBits<'a>,
    /// Burst received in subslot 2
    pub subslot2: RxBurstBits<'a>,
}

#[derive(Debug, Default)]
// Was: Bündelt die zusammengehörigen Werte für tx slot bits in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TxSlotBits<'a> {
    /// Carrier number to transmit on.
    pub carrier_num: u16,
    /// Number of slot to transmit
    pub time: TdmaTime,
    /// Burst to transmit in full slot
    pub slot: Option<&'a [u8]>,
    // /// Burst to transmit in subslot 1
    // pub subslot1: Option<&'a [u8]>,
    // /// Burst to transmit in subslot 2
    // pub subslot2: Option<&'a [u8]>,
}

/// Trait for RX/TX devices that work with full slots.
// Was: Beschreibt das gemeinsame Verhalten für rx tx dev.
// Warum: Unterschiedliche Implementierungen können dadurch über dieselbe verständliche Schnittstelle benutzt werden.
pub trait RxTxDev {
    // Was: Führt den Arbeitsschritt `rxtx_timeslot` für rxtx timeslot aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn rxtx_timeslot(&mut self, tx_slot: &[TxSlotBits]) -> Result<Vec<Option<RxSlotBits<'_>>>, RxTxDevError>;
}
