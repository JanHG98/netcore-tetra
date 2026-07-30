// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Nachrichten an den Schnittstellen zwischen TETRA-Protokollschichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// 14.8.17a Circuit mode type
/// Bits: 3
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für circuit mode type auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum CircuitModeType {
    /// Tch/S
    TchS = 0,
    /// Tch/7.2
    Tch72 = 1,
    /// Tch/4.8 N=1
    Tch48n1 = 2,
    /// Tch/4.8 N=4
    Tch48n4 = 3,
    /// Tch/4.8 N=8
    Tch48n8 = 4,
    /// Tch/2.4 N=1
    Tch24n1 = 5,
    /// Tch/2.4 N=4
    Tch24n4 = 6,
    /// Tch/2.4 N=8
    Tch24n8 = 7,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for CircuitModeType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for CircuitModeType {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            0 => Ok(CircuitModeType::TchS),
            1 => Ok(CircuitModeType::Tch72),
            2 => Ok(CircuitModeType::Tch48n1),
            3 => Ok(CircuitModeType::Tch48n4),
            4 => Ok(CircuitModeType::Tch48n8),
            5 => Ok(CircuitModeType::Tch24n1),
            6 => Ok(CircuitModeType::Tch24n4),
            7 => Ok(CircuitModeType::Tch24n8),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `CircuitModeType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl CircuitModeType {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            CircuitModeType::TchS => 0,
            CircuitModeType::Tch72 => 1,
            CircuitModeType::Tch48n1 => 2,
            CircuitModeType::Tch48n4 => 3,
            CircuitModeType::Tch48n8 => 4,
            CircuitModeType::Tch24n1 => 5,
            CircuitModeType::Tch24n4 => 6,
            CircuitModeType::Tch24n8 => 7,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<CircuitModeType> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<CircuitModeType> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: CircuitModeType) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for CircuitModeType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for CircuitModeType {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            CircuitModeType::TchS => write!(f, "Tch/S"),
            CircuitModeType::Tch72 => write!(f, "Tch/7.2"),
            CircuitModeType::Tch48n1 => write!(f, "Tch/4.8 N=1"),
            CircuitModeType::Tch48n4 => write!(f, "Tch/4.8 N=4"),
            CircuitModeType::Tch48n8 => write!(f, "Tch/4.8 N=8"),
            CircuitModeType::Tch24n1 => write!(f, "Tch/2.4 N=1"),
            CircuitModeType::Tch24n4 => write!(f, "Tch/2.4 N=4"),
            CircuitModeType::Tch24n8 => write!(f, "Tch/2.4 N=8"),
        }
    }
}
