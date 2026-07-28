// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 14.8.48 Type 3 element identifier
///
/// Bits: 4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für CMCE-Rufsteuerung type3 elem Kennung auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum CmceType3ElemId {
    Dtmf = 1,
    ExtSubscriberNum = 2,
    Facility = 3,
    PollResponseAddr = 4,
    TempAddr = 5,
    DmMsAddr = 6,
    Proprietary = 15,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for CmceType3ElemId`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for CmceType3ElemId {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            1 => Ok(CmceType3ElemId::Dtmf),
            2 => Ok(CmceType3ElemId::ExtSubscriberNum),
            3 => Ok(CmceType3ElemId::Facility),
            4 => Ok(CmceType3ElemId::PollResponseAddr),
            5 => Ok(CmceType3ElemId::TempAddr),
            6 => Ok(CmceType3ElemId::DmMsAddr),
            15 => Ok(CmceType3ElemId::Proprietary),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `CmceType3ElemId`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl CmceType3ElemId {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            CmceType3ElemId::Dtmf => 1,
            CmceType3ElemId::ExtSubscriberNum => 2,
            CmceType3ElemId::Facility => 3,
            CmceType3ElemId::PollResponseAddr => 4,
            CmceType3ElemId::TempAddr => 5,
            CmceType3ElemId::DmMsAddr => 6,
            CmceType3ElemId::Proprietary => 15,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<CmceType3ElemId> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<CmceType3ElemId> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: CmceType3ElemId) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for CmceType3ElemId`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for CmceType3ElemId {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            CmceType3ElemId::Dtmf => write!(f, "Dtmf"),
            CmceType3ElemId::ExtSubscriberNum => write!(f, "ExtSubscriberNum"),
            CmceType3ElemId::Facility => write!(f, "Facility"),
            CmceType3ElemId::PollResponseAddr => write!(f, "PollResponseAddr"),
            CmceType3ElemId::TempAddr => write!(f, "TempAddr"),
            CmceType3ElemId::DmMsAddr => write!(f, "DmMsAddr"),
            CmceType3ElemId::Proprietary => write!(f, "Proprietary"),
        }
    }
}
