// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 16.10.39 MM PDU types
/// Bits: 4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für Mobilitätsverwaltung Protokollnachricht (PDU) type ul auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum MmPduTypeUl {
    UAuthentication = 0,
    UItsiDetach = 1,
    ULocationUpdateDemand = 2,
    UMmStatus = 3,
    UCkChangeResult = 4,
    UOtar = 5,
    UInformationProvide = 6,
    UAttachDetachGroupIdentity = 7,
    UAttachDetachGroupIdentityAcknowledgement = 8,
    UTeiProvide = 9,
    UDisableStatus = 11,
    MmPduFunctionNotSupported = 15,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for MmPduTypeUl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for MmPduTypeUl {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            0 => Ok(MmPduTypeUl::UAuthentication),
            1 => Ok(MmPduTypeUl::UItsiDetach),
            2 => Ok(MmPduTypeUl::ULocationUpdateDemand),
            3 => Ok(MmPduTypeUl::UMmStatus),
            4 => Ok(MmPduTypeUl::UCkChangeResult),
            5 => Ok(MmPduTypeUl::UOtar),
            6 => Ok(MmPduTypeUl::UInformationProvide),
            7 => Ok(MmPduTypeUl::UAttachDetachGroupIdentity),
            8 => Ok(MmPduTypeUl::UAttachDetachGroupIdentityAcknowledgement),
            9 => Ok(MmPduTypeUl::UTeiProvide),
            11 => Ok(MmPduTypeUl::UDisableStatus),
            15 => Ok(MmPduTypeUl::MmPduFunctionNotSupported),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `MmPduTypeUl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MmPduTypeUl {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            MmPduTypeUl::UAuthentication => 0,
            MmPduTypeUl::UItsiDetach => 1,
            MmPduTypeUl::ULocationUpdateDemand => 2,
            MmPduTypeUl::UMmStatus => 3,
            MmPduTypeUl::UCkChangeResult => 4,
            MmPduTypeUl::UOtar => 5,
            MmPduTypeUl::UInformationProvide => 6,
            MmPduTypeUl::UAttachDetachGroupIdentity => 7,
            MmPduTypeUl::UAttachDetachGroupIdentityAcknowledgement => 8,
            MmPduTypeUl::UTeiProvide => 9,
            MmPduTypeUl::UDisableStatus => 11,
            MmPduTypeUl::MmPduFunctionNotSupported => 15,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<MmPduTypeUl> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<MmPduTypeUl> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: MmPduTypeUl) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for MmPduTypeUl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for MmPduTypeUl {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            MmPduTypeUl::UAuthentication => write!(f, "UAuthentication"),
            MmPduTypeUl::UItsiDetach => write!(f, "UItsiDetach"),
            MmPduTypeUl::ULocationUpdateDemand => write!(f, "ULocationUpdateDemand"),
            MmPduTypeUl::UMmStatus => write!(f, "UMmStatus"),
            MmPduTypeUl::UCkChangeResult => write!(f, "UCkChangeResult"),
            MmPduTypeUl::UOtar => write!(f, "UOtar"),
            MmPduTypeUl::UInformationProvide => write!(f, "UInformationProvide"),
            MmPduTypeUl::UAttachDetachGroupIdentity => write!(f, "UAttachDetachGroupIdentity"),
            MmPduTypeUl::UAttachDetachGroupIdentityAcknowledgement => write!(f, "UAttachDetachGroupIdentityAck"),
            MmPduTypeUl::UTeiProvide => write!(f, "UTeiProvide"),
            MmPduTypeUl::UDisableStatus => write!(f, "UDisableStatus"),
            MmPduTypeUl::MmPduFunctionNotSupported => write!(f, "MmPduFunctionNotSupported"),
        }
    }
}
