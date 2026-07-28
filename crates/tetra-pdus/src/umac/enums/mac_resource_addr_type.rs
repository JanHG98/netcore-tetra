// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 21.4.3.1 Table 21.55 MAC-RESOURCE address types
/// Bits: 3
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für MAC-Funkzugriffssteuerung resource addr type auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum MacResourceAddrType {
    NullPdu = 0,
    Ssi = 1,
    EventLabel = 2,
    Ussi = 3,
    Smi = 4,
    SsiAndEventLabel = 5,
    SsiAndUsageMarker = 6,
    SmiAndEventLabel = 7,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for MacResourceAddrType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for MacResourceAddrType {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            0 => Ok(MacResourceAddrType::NullPdu),
            1 => Ok(MacResourceAddrType::Ssi),
            2 => Ok(MacResourceAddrType::EventLabel),
            3 => Ok(MacResourceAddrType::Ussi),
            4 => Ok(MacResourceAddrType::Smi),
            5 => Ok(MacResourceAddrType::SsiAndEventLabel),
            6 => Ok(MacResourceAddrType::SsiAndUsageMarker),
            7 => Ok(MacResourceAddrType::SmiAndEventLabel),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `MacResourceAddrType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MacResourceAddrType {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            MacResourceAddrType::NullPdu => 0,
            MacResourceAddrType::Ssi => 1,
            MacResourceAddrType::EventLabel => 2,
            MacResourceAddrType::Ussi => 3,
            MacResourceAddrType::Smi => 4,
            MacResourceAddrType::SsiAndEventLabel => 5,
            MacResourceAddrType::SsiAndUsageMarker => 6,
            MacResourceAddrType::SmiAndEventLabel => 7,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<MacResourceAddrType> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<MacResourceAddrType> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: MacResourceAddrType) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for MacResourceAddrType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for MacResourceAddrType {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            MacResourceAddrType::NullPdu => write!(f, "NullPdu"),
            MacResourceAddrType::Ssi => write!(f, "Ssi"),
            MacResourceAddrType::EventLabel => write!(f, "EventLabel"),
            MacResourceAddrType::Ussi => write!(f, "Ussi"),
            MacResourceAddrType::Smi => write!(f, "Smi"),
            MacResourceAddrType::SsiAndEventLabel => write!(f, "SsiAndEventLabel"),
            MacResourceAddrType::SsiAndUsageMarker => write!(f, "SsiAndUsageMarker"),
            MacResourceAddrType::SmiAndEventLabel => write!(f, "SmiAndEventLabel"),
        }
    }
}
