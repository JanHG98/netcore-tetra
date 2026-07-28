// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 21.5.6 Basic slot granting, granting delay element
/// Bits: 4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für basic slotgrant granting delay auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum BasicSlotgrantGrantingDelay {
    CapAllocAtNextOpportunity = 0,
    /// Delay N opportunities, where N is in the range 1..=13
    DelayNOpportunities(u8),
    AllocStartsAtOpportunityInFr18 = 14,
    WaitForAnotherSlotgrantMessage = 15,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for BasicSlotgrantGrantingDelay`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for BasicSlotgrantGrantingDelay {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            0 => Ok(BasicSlotgrantGrantingDelay::CapAllocAtNextOpportunity),
            1..=13 => Ok(BasicSlotgrantGrantingDelay::DelayNOpportunities(x as u8)),
            14 => Ok(BasicSlotgrantGrantingDelay::AllocStartsAtOpportunityInFr18),
            15 => Ok(BasicSlotgrantGrantingDelay::WaitForAnotherSlotgrantMessage),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `BasicSlotgrantGrantingDelay`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl BasicSlotgrantGrantingDelay {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            BasicSlotgrantGrantingDelay::CapAllocAtNextOpportunity => 0,
            BasicSlotgrantGrantingDelay::DelayNOpportunities(n) => n as u64,
            BasicSlotgrantGrantingDelay::AllocStartsAtOpportunityInFr18 => 14,
            BasicSlotgrantGrantingDelay::WaitForAnotherSlotgrantMessage => 15,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<BasicSlotgrantGrantingDelay> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<BasicSlotgrantGrantingDelay> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: BasicSlotgrantGrantingDelay) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for BasicSlotgrantGrantingDelay`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for BasicSlotgrantGrantingDelay {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            BasicSlotgrantGrantingDelay::CapAllocAtNextOpportunity => write!(f, "CapAllocAtNextOpportunity"),
            BasicSlotgrantGrantingDelay::DelayNOpportunities(n) => write!(f, "Delay{}Opportunities", n),
            BasicSlotgrantGrantingDelay::AllocStartsAtOpportunityInFr18 => write!(f, "AllocStartsAtOpportunityInFr18"),
            BasicSlotgrantGrantingDelay::WaitForAnotherSlotgrantMessage => write!(f, "WaitForAnotherSlotgrantMessage"),
        }
    }
}
