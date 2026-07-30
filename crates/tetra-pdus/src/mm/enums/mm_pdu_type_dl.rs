// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 16.10.39 MM PDU types
/// Bits: 4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für Mobilitätsverwaltung Protokollnachricht (PDU) type dl auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum MmPduTypeDl {
    DOtar = 0,
    DAuthentication = 1,
    DCkChangeDemand = 2,
    DDisable = 3,
    DEnable = 4,
    DLocationUpdateAccept = 5,
    DLocationUpdateCommand = 6,
    DLocationUpdateReject = 7,
    DLocationUpdateProceeding = 9,
    DAttachDetachGroupIdentity = 10,
    DAttachDetachGroupIdentityAcknowledgement = 11,
    DMmStatus = 12,
    MmPduFunctionNotSupported = 15,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for MmPduTypeDl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for MmPduTypeDl {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            0 => Ok(MmPduTypeDl::DOtar),
            1 => Ok(MmPduTypeDl::DAuthentication),
            2 => Ok(MmPduTypeDl::DCkChangeDemand),
            3 => Ok(MmPduTypeDl::DDisable),
            4 => Ok(MmPduTypeDl::DEnable),
            5 => Ok(MmPduTypeDl::DLocationUpdateAccept),
            6 => Ok(MmPduTypeDl::DLocationUpdateCommand),
            7 => Ok(MmPduTypeDl::DLocationUpdateReject),
            9 => Ok(MmPduTypeDl::DLocationUpdateProceeding),
            10 => Ok(MmPduTypeDl::DAttachDetachGroupIdentity),
            11 => Ok(MmPduTypeDl::DAttachDetachGroupIdentityAcknowledgement),
            12 => Ok(MmPduTypeDl::DMmStatus),
            15 => Ok(MmPduTypeDl::MmPduFunctionNotSupported),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `MmPduTypeDl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MmPduTypeDl {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            MmPduTypeDl::DOtar => 0,
            MmPduTypeDl::DAuthentication => 1,
            MmPduTypeDl::DCkChangeDemand => 2,
            MmPduTypeDl::DDisable => 3,
            MmPduTypeDl::DEnable => 4,
            MmPduTypeDl::DLocationUpdateAccept => 5,
            MmPduTypeDl::DLocationUpdateCommand => 6,
            MmPduTypeDl::DLocationUpdateReject => 7,
            MmPduTypeDl::DLocationUpdateProceeding => 9,
            MmPduTypeDl::DAttachDetachGroupIdentity => 10,
            MmPduTypeDl::DAttachDetachGroupIdentityAcknowledgement => 11,
            MmPduTypeDl::DMmStatus => 12,
            MmPduTypeDl::MmPduFunctionNotSupported => 15,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<MmPduTypeDl> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<MmPduTypeDl> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: MmPduTypeDl) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for MmPduTypeDl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for MmPduTypeDl {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            MmPduTypeDl::DOtar => write!(f, "DOtar"),
            MmPduTypeDl::DAuthentication => write!(f, "DAuthentication"),
            MmPduTypeDl::DCkChangeDemand => write!(f, "DCkChangeDemand"),
            MmPduTypeDl::DDisable => write!(f, "DDisable"),
            MmPduTypeDl::DEnable => write!(f, "DEnable"),
            MmPduTypeDl::DLocationUpdateAccept => write!(f, "DLocationUpdateAccept"),
            MmPduTypeDl::DLocationUpdateCommand => write!(f, "DLocationUpdateCommand"),
            MmPduTypeDl::DLocationUpdateReject => write!(f, "DLocationUpdateReject"),
            MmPduTypeDl::DLocationUpdateProceeding => write!(f, "DLocationUpdateProceeding"),
            MmPduTypeDl::DAttachDetachGroupIdentity => write!(f, "DAttachDetachGroupIdentity"),
            MmPduTypeDl::DAttachDetachGroupIdentityAcknowledgement => write!(f, "DAttachDetachGroupIdentityAck"),
            MmPduTypeDl::DMmStatus => write!(f, "DMmStatus"),
            MmPduTypeDl::MmPduFunctionNotSupported => write!(f, "MmPduFunctionNotSupported"),
        }
    }
}
