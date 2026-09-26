// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Nachrichten an den Schnittstellen zwischen TETRA-Protokollschichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use tetra_core::{BitBuffer, BurstType, PhyBlockNum, PhyBlockType, TrainingSequence};

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tp unitdata ind in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TpUnitdataInd {
    pub carrier_num: u16,
    pub train_type: TrainingSequence,
    pub burst_type: BurstType,
    pub block_type: PhyBlockType,
    /// Undefined for BBK. For all others: [ Block1 | Block2 | Both ]
    pub block_num: PhyBlockNum,
    pub block: BitBuffer,
    /// Received signal strength in dBFS. See RxBurstBits.rssi_dbfs.
    pub rssi_dbfs: f32,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tp unitdata req slot in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TpUnitdataReqSlot {
    pub carrier_num: u16,
    pub train_type: TrainingSequence,
    pub burst_type: BurstType,
    pub bbk: Option<BitBuffer>,
    pub blk1: Option<BitBuffer>,
    pub blk2: Option<BitBuffer>,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tp unitdata req slots in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TpUnitdataReqSlots {
    pub slots: Vec<TpUnitdataReqSlot>,
}
