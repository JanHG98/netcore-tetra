// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 14.8.28 PDU type
/// Bits: 5
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für CMCE-Rufsteuerung Protokollnachricht (PDU) type ul auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum CmcePduTypeUl {
    UAlert = 0,
    UConnect = 2,
    UDisconnect = 4,
    UInfo = 5,
    URelease = 6,
    USetup = 7,
    UStatus = 8,
    UTxCeased = 9,
    UTxDemand = 10,
    UCallRestore = 14,
    USdsData = 15,
    UFacility = 16,
    CmceFunctionNotSupported = 31,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for CmcePduTypeUl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for CmcePduTypeUl {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(raw: u64) -> Result<Self, Self::Error> {
        let x = raw as u8;
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            0 => Ok(CmcePduTypeUl::UAlert),
            2 => Ok(CmcePduTypeUl::UConnect),
            4 => Ok(CmcePduTypeUl::UDisconnect),
            5 => Ok(CmcePduTypeUl::UInfo),
            6 => Ok(CmcePduTypeUl::URelease),
            7 => Ok(CmcePduTypeUl::USetup),
            8 => Ok(CmcePduTypeUl::UStatus),
            9 => Ok(CmcePduTypeUl::UTxCeased),
            10 => Ok(CmcePduTypeUl::UTxDemand),
            14 => Ok(CmcePduTypeUl::UCallRestore),
            15 => Ok(CmcePduTypeUl::USdsData),
            16 => Ok(CmcePduTypeUl::UFacility),
            31 => Ok(CmcePduTypeUl::CmceFunctionNotSupported),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `CmcePduTypeUl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl CmcePduTypeUl {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            CmcePduTypeUl::UAlert => 0,
            CmcePduTypeUl::UConnect => 2,
            CmcePduTypeUl::UDisconnect => 4,
            CmcePduTypeUl::UInfo => 5,
            CmcePduTypeUl::URelease => 6,
            CmcePduTypeUl::USetup => 7,
            CmcePduTypeUl::UStatus => 8,
            CmcePduTypeUl::UTxCeased => 9,
            CmcePduTypeUl::UTxDemand => 10,
            CmcePduTypeUl::UCallRestore => 14,
            CmcePduTypeUl::USdsData => 15,
            CmcePduTypeUl::UFacility => 16,
            CmcePduTypeUl::CmceFunctionNotSupported => 31,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<CmcePduTypeUl> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<CmcePduTypeUl> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: CmcePduTypeUl) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for CmcePduTypeUl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for CmcePduTypeUl {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            CmcePduTypeUl::UAlert => write!(f, "UAlert"),
            CmcePduTypeUl::UConnect => write!(f, "UConnect"),
            CmcePduTypeUl::UDisconnect => write!(f, "UDisconnect"),
            CmcePduTypeUl::UInfo => write!(f, "UInfo"),
            CmcePduTypeUl::URelease => write!(f, "URelease"),
            CmcePduTypeUl::USetup => write!(f, "USetup"),
            CmcePduTypeUl::UStatus => write!(f, "UStatus"),
            CmcePduTypeUl::UTxCeased => write!(f, "UTxCeased"),
            CmcePduTypeUl::UTxDemand => write!(f, "UTxDemand"),
            CmcePduTypeUl::UCallRestore => write!(f, "UCallRestore"),
            CmcePduTypeUl::USdsData => write!(f, "USdsData"),
            CmcePduTypeUl::UFacility => write!(f, "UFacility"),
            CmcePduTypeUl::CmceFunctionNotSupported => write!(f, "CmceFunctionNotSupported"),
        }
    }
}
