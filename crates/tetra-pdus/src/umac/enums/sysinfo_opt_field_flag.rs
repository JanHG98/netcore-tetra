// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 21.4.4.1 Table 21.65
/// Bits: 2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für sysinfo opt field flag auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum SysinfoOptFieldFlag {
    /// Even multiframe definition for TS mode
    EvenMfDefForTsMode = 0,
    /// Odd multiframe definition for TS mode
    OddMfDefForTsMode = 1,
    /// Default definition for access code A
    DefaultDefForAccCodeA = 2,
    /// Extended services broadcast
    ExtServicesBroadcast = 3,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for SysinfoOptFieldFlag`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for SysinfoOptFieldFlag {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            0 => Ok(SysinfoOptFieldFlag::EvenMfDefForTsMode),
            1 => Ok(SysinfoOptFieldFlag::OddMfDefForTsMode),
            2 => Ok(SysinfoOptFieldFlag::DefaultDefForAccCodeA),
            3 => Ok(SysinfoOptFieldFlag::ExtServicesBroadcast),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `SysinfoOptFieldFlag`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl SysinfoOptFieldFlag {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            SysinfoOptFieldFlag::EvenMfDefForTsMode => 0,
            SysinfoOptFieldFlag::OddMfDefForTsMode => 1,
            SysinfoOptFieldFlag::DefaultDefForAccCodeA => 2,
            SysinfoOptFieldFlag::ExtServicesBroadcast => 3,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<SysinfoOptFieldFlag> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<SysinfoOptFieldFlag> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: SysinfoOptFieldFlag) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for SysinfoOptFieldFlag`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for SysinfoOptFieldFlag {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            SysinfoOptFieldFlag::EvenMfDefForTsMode => write!(f, "EvenMfDefForTsMode"),
            SysinfoOptFieldFlag::OddMfDefForTsMode => write!(f, "OddMfDefForTsMode"),
            SysinfoOptFieldFlag::DefaultDefForAccCodeA => write!(f, "DefaultDefForAccCodeA"),
            SysinfoOptFieldFlag::ExtServicesBroadcast => write!(f, "ExtServicesBroadcast"),
        }
    }
}
