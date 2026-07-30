// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 16.10.39 MM PDU types
/// Bits: 3
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für Mobilitätsverwaltung type34 elem Kennung ul auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum MmType34ElemIdUl {
    GroupIdentityLocationDemand = 3,
    GroupReportResponse = 4,
    DmMsAddress = 6,
    GroupIdentityUplink = 8,
    AuthenticationUplink = 9,
    ExtendedCapabilities = 11,
    Proprietary = 15,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for MmType34ElemIdUl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for MmType34ElemIdUl {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            3 => Ok(MmType34ElemIdUl::GroupIdentityLocationDemand),
            4 => Ok(MmType34ElemIdUl::GroupReportResponse),
            6 => Ok(MmType34ElemIdUl::DmMsAddress),
            8 => Ok(MmType34ElemIdUl::GroupIdentityUplink),
            9 => Ok(MmType34ElemIdUl::AuthenticationUplink),
            11 => Ok(MmType34ElemIdUl::ExtendedCapabilities),
            15 => Ok(MmType34ElemIdUl::Proprietary),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `MmType34ElemIdUl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MmType34ElemIdUl {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            MmType34ElemIdUl::GroupIdentityLocationDemand => 3,
            MmType34ElemIdUl::GroupReportResponse => 4,
            MmType34ElemIdUl::DmMsAddress => 6,
            MmType34ElemIdUl::GroupIdentityUplink => 8,
            MmType34ElemIdUl::AuthenticationUplink => 9,
            MmType34ElemIdUl::ExtendedCapabilities => 11,
            MmType34ElemIdUl::Proprietary => 15,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<MmType34ElemIdUl> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<MmType34ElemIdUl> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: MmType34ElemIdUl) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for MmType34ElemIdUl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for MmType34ElemIdUl {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            MmType34ElemIdUl::GroupIdentityLocationDemand => write!(f, "GroupIdentityLocationDemand"),
            MmType34ElemIdUl::GroupReportResponse => write!(f, "GroupReportResponse"),
            MmType34ElemIdUl::DmMsAddress => write!(f, "DmMsAddress"),
            MmType34ElemIdUl::GroupIdentityUplink => write!(f, "GroupIdentityUplink"),
            MmType34ElemIdUl::AuthenticationUplink => write!(f, "AuthenticationUplink"),
            MmType34ElemIdUl::ExtendedCapabilities => write!(f, "ExtendedCapabilities"),
            MmType34ElemIdUl::Proprietary => write!(f, "Proprietary"),
        }
    }
}
