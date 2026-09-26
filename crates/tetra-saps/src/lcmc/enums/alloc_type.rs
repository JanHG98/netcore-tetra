// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Nachrichten an den Schnittstellen zwischen TETRA-Protokollschichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// 14.8.17a Circuit mode type
/// Bits: 2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für chan alloc type auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum ChanAllocType {
    Replace = 0,
    Additional = 1,
    QuitAndGo = 2,
    /// Replace current channel with specified channel plus carrier specific signalling channel in slot 1
    ReplaceWithCarrierSignalling = 3,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for ChanAllocType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for ChanAllocType {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            0 => Ok(ChanAllocType::Replace),
            1 => Ok(ChanAllocType::Additional),
            2 => Ok(ChanAllocType::QuitAndGo),
            3 => Ok(ChanAllocType::ReplaceWithCarrierSignalling),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `ChanAllocType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl ChanAllocType {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            ChanAllocType::Replace => 0,
            ChanAllocType::Additional => 1,
            ChanAllocType::QuitAndGo => 2,
            ChanAllocType::ReplaceWithCarrierSignalling => 3,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<ChanAllocType> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<ChanAllocType> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: ChanAllocType) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for ChanAllocType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for ChanAllocType {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            ChanAllocType::Replace => write!(f, "Replace"),
            ChanAllocType::Additional => write!(f, "Additional"),
            ChanAllocType::QuitAndGo => write!(f, "QuitAndGo"),
            ChanAllocType::ReplaceWithCarrierSignalling => write!(f, "ReplaceWithCarrierSignalling"),
        }
    }
}
