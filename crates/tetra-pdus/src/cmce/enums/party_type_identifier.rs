// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 14.8.5 / 14.8.9 — Called/Calling Party Type Identifier (CPTI).
/// Indicates the type of address which follows in the PDU (Table 14.39).
/// Bits: 2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für party type identifier auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum PartyTypeIdentifier {
    /// Short Number Address (SNA)
    Sna = 0,
    /// Short Subscriber Identity (SSI)
    Ssi = 1,
    /// TETRA Subscriber Identity (TSI = SSI + Extension)
    Tsi = 2,
    /// Reserved
    Reserved = 3,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for PartyTypeIdentifier`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for PartyTypeIdentifier {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            0 => Ok(PartyTypeIdentifier::Sna),
            1 => Ok(PartyTypeIdentifier::Ssi),
            2 => Ok(PartyTypeIdentifier::Tsi),
            3 => Ok(PartyTypeIdentifier::Reserved),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `PartyTypeIdentifier`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl PartyTypeIdentifier {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        self as u64
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<PartyTypeIdentifier> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<PartyTypeIdentifier> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: PartyTypeIdentifier) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for PartyTypeIdentifier`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for PartyTypeIdentifier {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            PartyTypeIdentifier::Sna => write!(f, "SNA"),
            PartyTypeIdentifier::Ssi => write!(f, "SSI"),
            PartyTypeIdentifier::Tsi => write!(f, "TSI"),
            PartyTypeIdentifier::Reserved => write!(f, "Reserved"),
        }
    }
}
