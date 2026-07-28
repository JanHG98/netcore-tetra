// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Shared TBS media-bridge protocol and bounded in-process channels.
//!
//! The protocol types are available to backend-only crates without the full RF
//! runtime.  The channel implementation is runtime-gated and connects UMAC to
//! the Control-Room/Node-Gateway worker without putting network I/O into the
//! TDMA router thread.

use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// Packed TETRA speech service 0 frame size: 274 payload bits rounded to bytes.
// Was: Legt den festen Wert `TETRA_ACELP_FRAME_BYTES` für TETRA acelp Funkrahmen bytes fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const TETRA_ACELP_FRAME_BYTES: usize = 35;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für Audio- und Mediendaten codec auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum MediaCodec {
    /// TETRA encoded speech service 0, one 274-bit TCH/S frame packed into 35 bytes.
    TetraAcelp0,
}

#[derive(Debug, Clone, Encode, Decode, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Audio- und Mediendaten Uplink (Funkgerät zum Netz) Funkrahmen in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MediaUplinkFrame {
    pub node_id: String,
    pub sequence: u64,
    pub timestamp: String,
    pub carrier_num: u16,
    /// Logical TBS timeslot (1..=7; TS5..TS7 map to secondary-carrier air TS2..TS4).
    pub logical_ts: u8,
    pub codec: MediaCodec,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Encode, Decode, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Audio- und Mediendaten Downlink (Netz zum Funkgerät) Funkrahmen in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MediaDownlinkFrame {
    /// Logical Media-Switch session/call identifier used for diagnostics and taps.
    pub session_id: String,
    pub source_node_id: String,
    pub sequence: u64,
    /// Logical destination timeslot on the target TBS.
    pub logical_ts: u8,
    pub codec: MediaCodec,
    pub payload: Vec<u8>,
}

#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul Kanal in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod channel {
    use crossbeam_channel::{Receiver, Sender, TryRecvError, TrySendError, bounded};

    use super::{MediaCodec, MediaDownlinkFrame, TETRA_ACELP_FRAME_BYTES};

    #[derive(Debug, Clone)]
    // Was: Bündelt die zusammengehörigen Werte für local Audio- und Mediendaten Uplink (Funkgerät zum Netz) Funkrahmen in einem Datentyp.
    // Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
    pub struct LocalMediaUplinkFrame {
        pub sequence: u64,
        pub carrier_num: u16,
        pub logical_ts: u8,
        pub codec: MediaCodec,
        pub payload: Vec<u8>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    // Was: Listet die möglichen Varianten für Audio- und Mediendaten send error auf.
    // Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
    pub enum MediaSendError {
        Full,
        Disconnected,
        InvalidFrame,
    }

    #[derive(Clone)]
    // Was: Bündelt die zusammengehörigen Werte für Audio- und Mediendaten Uplink (Funkgerät zum Netz) sink in einem Datentyp.
    // Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
    pub struct MediaUplinkSink {
        tx: Sender<LocalMediaUplinkFrame>,
    }

    // Was: Bündelt die zusammengehörigen Werte für Audio- und Mediendaten Uplink (Funkgerät zum Netz) source in einem Datentyp.
    // Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
    pub struct MediaUplinkSource {
        rx: Receiver<LocalMediaUplinkFrame>,
    }

    #[derive(Clone)]
    // Was: Bündelt die zusammengehörigen Werte für Audio- und Mediendaten Downlink (Netz zum Funkgerät) sink in einem Datentyp.
    // Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
    pub struct MediaDownlinkSink {
        tx: Sender<MediaDownlinkFrame>,
    }

    // Was: Bündelt die zusammengehörigen Werte für Audio- und Mediendaten Downlink (Netz zum Funkgerät) source in einem Datentyp.
    // Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
    pub struct MediaDownlinkSource {
        rx: Receiver<MediaDownlinkFrame>,
    }

    // Was: Implementiert das zugehörige Verhalten für `MediaUplinkSink`.
    // Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
    impl MediaUplinkSink {
        // Was: Diese Funktion sendet den vorgesehenen Arbeitsschritt.
        // Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
        pub fn try_send(&self, frame: LocalMediaUplinkFrame) -> Result<(), MediaSendError> {
            if frame.payload.len() != TETRA_ACELP_FRAME_BYTES {
                return Err(MediaSendError::InvalidFrame);
            }
            self.tx.try_send(frame).map_err(|error| match error {
                TrySendError::Full(_) => MediaSendError::Full,
                TrySendError::Disconnected(_) => MediaSendError::Disconnected,
            })
        }
    }

    // Was: Implementiert das zugehörige Verhalten für `MediaUplinkSource`.
    // Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
    impl MediaUplinkSource {
        // Was: Führt den Arbeitsschritt `try_recv` für try recv aus.
        // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
        pub fn try_recv(&self) -> Result<LocalMediaUplinkFrame, TryRecvError> {
            self.rx.try_recv()
        }
    }

    // Was: Implementiert das zugehörige Verhalten für `MediaDownlinkSink`.
    // Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
    impl MediaDownlinkSink {
        // Was: Diese Funktion sendet den vorgesehenen Arbeitsschritt.
        // Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
        pub fn try_send(&self, frame: MediaDownlinkFrame) -> Result<(), MediaSendError> {
            if frame.payload.len() != TETRA_ACELP_FRAME_BYTES {
                return Err(MediaSendError::InvalidFrame);
            }
            self.tx.try_send(frame).map_err(|error| match error {
                TrySendError::Full(_) => MediaSendError::Full,
                TrySendError::Disconnected(_) => MediaSendError::Disconnected,
            })
        }
    }

    // Was: Implementiert das zugehörige Verhalten für `MediaDownlinkSource`.
    // Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
    impl MediaDownlinkSource {
        // Was: Führt den Arbeitsschritt `try_recv` für try recv aus.
        // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
        pub fn try_recv(&self) -> Result<MediaDownlinkFrame, TryRecvError> {
            self.rx.try_recv()
        }
    }

    /// Create independent bounded queues for UL (UMAC -> network worker) and DL
    /// (network worker -> UMAC). Bounded queues make overload visible and stop a
    /// slow management network from consuming unbounded RF-process memory.
    // Was: Führt den Arbeitsschritt `media_bridge_channel` für Audio- und Mediendaten bridge Kanal aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn media_bridge_channel(
        capacity: usize,
    ) -> (
        MediaUplinkSink,
        MediaUplinkSource,
        MediaDownlinkSink,
        MediaDownlinkSource,
    ) {
        let capacity = capacity.max(16);
        let (uplink_tx, uplink_rx) = bounded(capacity);
        let (downlink_tx, downlink_rx) = bounded(capacity);
        (
            MediaUplinkSink { tx: uplink_tx },
            MediaUplinkSource { rx: uplink_rx },
            MediaDownlinkSink { tx: downlink_tx },
            MediaDownlinkSource { rx: downlink_rx },
        )
    }

    pub use crossbeam_channel::TryRecvError as MediaTryRecvError;
}

#[cfg(feature = "runtime")]
pub use channel::{
    LocalMediaUplinkFrame, MediaDownlinkSink, MediaDownlinkSource, MediaSendError,
    MediaTryRecvError, MediaUplinkSink, MediaUplinkSource, media_bridge_channel,
};
