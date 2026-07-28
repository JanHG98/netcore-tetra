// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use tetra_saps::tmv::enums::logical_chans::LogicalChannel;

/// Each LogicalChannel is associated with a set of error control parameters.
#[derive(Debug)]
// Was: Bündelt die zusammengehörigen Werte für error Steuerung params in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ErrorControlParams {
    // pub name:           &'static str,
    pub type345_bits: usize,
    pub type2_bits: usize,
    pub type1_bits: usize,
    pub interleave_a: usize,
    pub have_crc16: bool,
}

/// Parameters for the BSCH (Broadcast Synchronization Channel)
// Was: Legt den festen Wert `BSCH_PARAMS` für bsch params fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const BSCH_PARAMS: ErrorControlParams = ErrorControlParams {
    type345_bits: 120,
    type2_bits: 80,
    type1_bits: 60,
    interleave_a: 11,
    have_crc16: true,
};

/// Parameters for the SCH/HD (half slot) signalling channel, also for STCH and BNCH
// Was: Legt den festen Wert `SCH_HD_PARAMS` für sch hd params fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SCH_HD_PARAMS: ErrorControlParams = ErrorControlParams {
    type345_bits: 216,
    type2_bits: 144,
    type1_bits: 124,
    interleave_a: 101,
    have_crc16: true,
};

/// Parameters for the BBK (Broadcast Block) channel, used for AACH
// Was: Legt den festen Wert `AACH_PARAMS` für aach params fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const AACH_PARAMS: ErrorControlParams = ErrorControlParams {
    type345_bits: 30,
    type2_bits: 30,
    type1_bits: 14,
    interleave_a: 0, // No interleaving
    have_crc16: false,
};

/// Parameters for the SCH/F channel
// Was: Legt den festen Wert `SCH_F_PARAMS` für sch f params fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SCH_F_PARAMS: ErrorControlParams = ErrorControlParams {
    type345_bits: 432,
    type2_bits: 288,
    type1_bits: 268,
    interleave_a: 103,
    have_crc16: true,
};

/// Parameters for the SCH/HU (half slot uplink, Control Uplink Burst) channel
// Was: Legt den festen Wert `SCH_HU_PARAMS` für sch hu params fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SCH_HU_PARAMS: ErrorControlParams = ErrorControlParams {
    type345_bits: 168,
    type2_bits: 112,
    type1_bits: 92,
    interleave_a: 13,
    have_crc16: true,
};

/// Parameters for TCH/S (full-rate 7.2 kbit/s speech).
/// 274 bits + 4 tail + 10 padding = 288 type-2 bits. No CRC.
// Was: Legt den festen Wert `TCH_S_PARAMS` für tch s params fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const TCH_S_PARAMS: ErrorControlParams = ErrorControlParams {
    type345_bits: 432,
    type2_bits: 288,
    type1_bits: 274,
    interleave_a: 103,
    have_crc16: false,
};

/// Gets error control parameters for a given DL logical channel.
// Was: Diese Funktion liest params.
// Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
pub fn get_params(lchan: LogicalChannel) -> &'static ErrorControlParams {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match lchan {
        LogicalChannel::Bsch => &BSCH_PARAMS,
        LogicalChannel::SchHd | LogicalChannel::Stch | LogicalChannel::Bnch => &SCH_HD_PARAMS,
        LogicalChannel::Aach => &AACH_PARAMS,
        LogicalChannel::SchF => &SCH_F_PARAMS,
        LogicalChannel::SchHu => &SCH_HU_PARAMS,
        LogicalChannel::TchS => &TCH_S_PARAMS,

        LogicalChannel::Tch24 => unimplemented!(),
        LogicalChannel::Tch48 => unimplemented!(),
        LogicalChannel::Tch72 => unimplemented!(),

        LogicalChannel::Blch => unimplemented!(),
        LogicalChannel::Clch => unimplemented!(),
    }
}
