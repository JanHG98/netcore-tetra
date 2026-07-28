// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 16.10.9 Energy saving mode
/// Bits: 3
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für energy saving mode auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum EnergySavingMode {
    StayAlive = 0,
    /// Economy Mode 1
    Eg1 = 1,
    /// Economy Mode 2
    Eg2 = 2,
    /// Economy Mode 3
    Eg3 = 3,
    /// Economy Mode 4
    Eg4 = 4,
    /// Economy Mode 5
    Eg5 = 5,
    /// Economy Mode 6
    Eg6 = 6,
    /// Economy Mode 7
    Eg7 = 7,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for EnergySavingMode`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for EnergySavingMode {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            0 => Ok(EnergySavingMode::StayAlive),
            1 => Ok(EnergySavingMode::Eg1),
            2 => Ok(EnergySavingMode::Eg2),
            3 => Ok(EnergySavingMode::Eg3),
            4 => Ok(EnergySavingMode::Eg4),
            5 => Ok(EnergySavingMode::Eg5),
            6 => Ok(EnergySavingMode::Eg6),
            7 => Ok(EnergySavingMode::Eg7),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `EnergySavingMode`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl EnergySavingMode {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            EnergySavingMode::StayAlive => 0,
            EnergySavingMode::Eg1 => 1,
            EnergySavingMode::Eg2 => 2,
            EnergySavingMode::Eg3 => 3,
            EnergySavingMode::Eg4 => 4,
            EnergySavingMode::Eg5 => 5,
            EnergySavingMode::Eg6 => 6,
            EnergySavingMode::Eg7 => 7,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<EnergySavingMode> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<EnergySavingMode> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: EnergySavingMode) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for EnergySavingMode`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for EnergySavingMode {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            EnergySavingMode::StayAlive => write!(f, "StayAlive"),
            EnergySavingMode::Eg1 => write!(f, "Eg1"),
            EnergySavingMode::Eg2 => write!(f, "Eg2"),
            EnergySavingMode::Eg3 => write!(f, "Eg3"),
            EnergySavingMode::Eg4 => write!(f, "Eg4"),
            EnergySavingMode::Eg5 => write!(f, "Eg5"),
            EnergySavingMode::Eg6 => write!(f, "Eg6"),
            EnergySavingMode::Eg7 => write!(f, "Eg7"),
        }
    }
}
