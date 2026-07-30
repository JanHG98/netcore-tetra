// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für grundlegende TETRA-Datentypen und Hilfsfunktionen.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

// Entities as used in the standard
#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy, Encode, Decode, Serialize, Deserialize)]
// Was: Listet die möglichen Varianten für TETRA entity auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum TetraEntity {
    /// Physical layer
    Phy,
    /// Lower MAC layer
    Lmac,
    /// Upper MAC layer
    Umac,
    /// Logical link control
    Llc,
    /// Mobile Link Entity
    Mle,
    /// Mobility Management
    Mm,
    /// Circuit Mode Control Entity
    Cmce,
    /// SubNetwork Dependent Convergence Protocol
    Sndcp,

    /// Any U-plane entity. SAP determines routing
    User,

    /// Brew protocol bridge (TetraPack/BrandMeister integration)
    Brew,

    /// Asterisk SIP/RTP bridge
    Asterisk,

    /// EchoLink UDP/GSM bridge
    Echolink,

    /// Secondary Brew protocol bridge.
    Brew2,

    /// Local TETRA speech recorder
    Recorder,

    /// Local WAV/MP3 TETRA audio dispatcher
    AudioPlayer,
}
