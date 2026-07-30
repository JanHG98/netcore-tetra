// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 18.5.21 Protocol discriminator
/// Bits: 3
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für MLE-Verbindungssteuerung protocol discriminator auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum MleProtocolDiscriminator {
    // RESERVED = 0,
    Mm = 1,
    Cmce = 2,
    // RESERVED = 3,
    Sndcp = 4,
    Mle = 5,
    TetraManagementEntity = 6,
    // ReservedForTesting = 7,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for MleProtocolDiscriminator`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for MleProtocolDiscriminator {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            // 0 => Ok(MleProtocolDiscriminator::RESERVED),
            1 => Ok(MleProtocolDiscriminator::Mm),
            2 => Ok(MleProtocolDiscriminator::Cmce),
            // 3 => Ok(MleProtocolDiscriminator::RESERVED),
            4 => Ok(MleProtocolDiscriminator::Sndcp),
            5 => Ok(MleProtocolDiscriminator::Mle),
            6 => Ok(MleProtocolDiscriminator::TetraManagementEntity),
            // 7 => Ok(MleProtocolDiscriminator::ReservedForTesting),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `MleProtocolDiscriminator`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MleProtocolDiscriminator {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            // MleProtocolDiscriminator::RESERVED => 0,
            MleProtocolDiscriminator::Mm => 1,
            MleProtocolDiscriminator::Cmce => 2,
            // MleProtocolDiscriminator::RESERVED => 3,
            MleProtocolDiscriminator::Sndcp => 4,
            MleProtocolDiscriminator::Mle => 5,
            MleProtocolDiscriminator::TetraManagementEntity => 6,
            // MleProtocolDiscriminator::ReservedForTesting => 7,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<MleProtocolDiscriminator> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<MleProtocolDiscriminator> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: MleProtocolDiscriminator) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for MleProtocolDiscriminator`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for MleProtocolDiscriminator {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            // MleProtocolDiscriminator::RESERVED => write!(f, "RESERVED"),
            MleProtocolDiscriminator::Mm => write!(f, "Mm"),
            MleProtocolDiscriminator::Cmce => write!(f, "Cmce"),
            // MleProtocolDiscriminator::RESERVED => write!(f, "RESERVED"),
            MleProtocolDiscriminator::Sndcp => write!(f, "Sndcp"),
            MleProtocolDiscriminator::Mle => write!(f, "Mle"),
            MleProtocolDiscriminator::TetraManagementEntity => write!(f, "TetraManagementEntity"),
            // MleProtocolDiscriminator::ReservedForTesting => write!(f, "ReservedForTesting"),
        }
    }
}
