// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 21.4.1 Table 21.38: MAC PDU types for SCH/F, SCH/HD, STCH, SCH-P8/F, SCH-P8/HD, SCH-Q/D, SCH-Q/B and SCH-Q/U
/// Bits: 2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für MAC-Funkzugriffssteuerung Protokollnachricht (PDU) type auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum MacPduType {
    /// TMA-SAP: MAC-RESOURCE (DL) or MAC-DATA (UL)
    MacResourceMacData = 0,
    /// TMA-SAP: MAC-END or MAC-FRAG
    MacFragMacEnd = 1,
    /// TMB-SAP: Broadcast
    Broadcast = 2,
    /// TMA-SAP: Supplementary, or TMD-SAP: MAC-U-SIGNAL
    SuppMacUSignal = 3,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for MacPduType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for MacPduType {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            0 => Ok(MacPduType::MacResourceMacData),
            1 => Ok(MacPduType::MacFragMacEnd),
            2 => Ok(MacPduType::Broadcast),
            3 => Ok(MacPduType::SuppMacUSignal),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `MacPduType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MacPduType {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            MacPduType::MacResourceMacData => 0,
            MacPduType::MacFragMacEnd => 1,
            MacPduType::Broadcast => 2,
            MacPduType::SuppMacUSignal => 3,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<MacPduType> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<MacPduType> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: MacPduType) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for MacPduType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for MacPduType {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            MacPduType::MacResourceMacData => write!(f, "MacResourceMacData"),
            MacPduType::MacFragMacEnd => write!(f, "MacFragMacEnd"),
            MacPduType::Broadcast => write!(f, "Broadcast"),
            MacPduType::SuppMacUSignal => write!(f, "SuppMacUSignal"),
        }
    }
}
