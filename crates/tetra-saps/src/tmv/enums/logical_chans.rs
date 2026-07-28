// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Nachrichten an den Schnittstellen zwischen TETRA-Protokollschichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Logical channels as defined in the standard
#[derive(Debug, Clone, Copy, PartialEq)]
// Was: Listet die möglichen Varianten für logical Kanal auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum LogicalChannel {
    /// Access Assignment CHannel
    Aach,

    /// Signalling Channel (half slot, downlink)
    SchHd,
    /// Signalling Channel (full slot)
    SchF,
    /// STealing Channel (half slot)
    Stch,
    /// Signalling Channel (half slot, uplink)
    SchHu,

    /// Traffic Channel (Voice)
    TchS,
    /// Traffic Channel (24 kbps)
    Tch24,
    /// Traffic Channel (48 kbps)
    Tch48,
    /// Traffic Channel (72 kbps)
    Tch72,

    /// Broadcast Synchronization Channel
    Bsch,
    /// Broadcast Network Channel
    Bnch,

    /// BS Linearization CHannel (downlink)
    Blch,
    /// Common Linearization Channel (uplink)
    Clch,
}

// Was: Implementiert das zugehörige Verhalten für `LogicalChannel`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl LogicalChannel {
    /// Returns the number of bits required to represent the logical channel
    // Was: Prüft, ob Nutzdatenverkehr zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_traffic(self) -> bool {
        matches!(
            self,
            LogicalChannel::TchS | LogicalChannel::Tch24 | LogicalChannel::Tch48 | LogicalChannel::Tch72
        )
    }

    /// TODO FIXME actually, BNCH, BSCH, AACH are also part of CP
    // Was: Prüft, ob Steuerung Kanal zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_control_channel(self) -> bool {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            LogicalChannel::Aach | // Odd one since very different decoding, but actually part of CP
            LogicalChannel::Bsch | // Also not containing regular mac blocks but still CP
            LogicalChannel::Bnch |
            LogicalChannel::SchHd |
            LogicalChannel::SchF |
            LogicalChannel::Stch |
            LogicalChannel::SchHu => true,
            _ => false,
        }
    }

    /// Returns true if channel is a linearization channel
    // Was: Prüft, ob linearization Kanal zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_linearization_channel(self) -> bool {
        self == LogicalChannel::Clch || self == LogicalChannel::Blch
    }

    /// Returns true if channel may be encountered on the downlink
    // Was: Prüft, ob dl Kanal zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_dl_channel(self) -> bool {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            LogicalChannel::Aach
            | LogicalChannel::SchHd
            | LogicalChannel::SchF
            | LogicalChannel::Stch
            | LogicalChannel::Bsch
            | LogicalChannel::Bnch
            | LogicalChannel::Blch
            | LogicalChannel::TchS
            | LogicalChannel::Tch24
            | LogicalChannel::Tch48
            | LogicalChannel::Tch72 => true,
            LogicalChannel::SchHu | LogicalChannel::Clch => false,
        }
    }

    /// Returns true if channel may be encountered on the uplink
    // Was: Prüft, ob ul Kanal zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_ul_channel(self) -> bool {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            LogicalChannel::SchHu
            | LogicalChannel::SchF
            | LogicalChannel::Stch
            | LogicalChannel::Clch
            | LogicalChannel::TchS
            | LogicalChannel::Tch24
            | LogicalChannel::Tch48
            | LogicalChannel::Tch72 => true,
            LogicalChannel::Aach | LogicalChannel::SchHd | LogicalChannel::Bsch | LogicalChannel::Bnch | LogicalChannel::Blch => false,
        }
    }
}
