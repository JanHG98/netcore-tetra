// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// 14.8.18 Disconnect cause
/// Bits: 3
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für disconnect cause auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum DisconnectCause {
    CauseNotDefinedOrUnknown = 0,
    UserRequestedDisconnection = 1,
    CalledPartyBusy = 2,
    CalledPartyNotReachable = 3,
    CalledPartyDoesNotSupportEncryption = 4,
    CongestionInInfrastructure = 5,
    NotAllowedTrafficCase = 6,
    IncompatibleTrafficCase = 7,
    RequestedServiceNotAvailable = 8,
    PreEmptiveUseOfResource = 9,
    InvalidCallIdentifier = 10,
    CallRejectedByTheCalledParty = 11,
    NoIdleCcEntity = 12,
    ExpiryOfTimer = 13,
    SwmiRequestedDisconnection = 14,
    AcknowledgedServiceNotComplete = 15,
    UnknownTetraIdentity = 16,
    SsSpecificDisconnection = 17,
    UnknownExternalSubscriberIdentity = 18,
    CallRestorationOfTheOtherUserFailed = 19,
    CalledPartyRequiresEncryption = 20,
    ConcurrentSetUpNotSupported = 21,
    CalledPartyIsUnderTheSameDmGateOfTheCallingParty = 22,
    NonCallOwnerRequestedDisconnection = 23,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for DisconnectCause`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for DisconnectCause {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            0 => Ok(DisconnectCause::CauseNotDefinedOrUnknown),
            1 => Ok(DisconnectCause::UserRequestedDisconnection),
            2 => Ok(DisconnectCause::CalledPartyBusy),
            3 => Ok(DisconnectCause::CalledPartyNotReachable),
            4 => Ok(DisconnectCause::CalledPartyDoesNotSupportEncryption),
            5 => Ok(DisconnectCause::CongestionInInfrastructure),
            6 => Ok(DisconnectCause::NotAllowedTrafficCase),
            7 => Ok(DisconnectCause::IncompatibleTrafficCase),
            8 => Ok(DisconnectCause::RequestedServiceNotAvailable),
            9 => Ok(DisconnectCause::PreEmptiveUseOfResource),
            10 => Ok(DisconnectCause::InvalidCallIdentifier),
            11 => Ok(DisconnectCause::CallRejectedByTheCalledParty),
            12 => Ok(DisconnectCause::NoIdleCcEntity),
            13 => Ok(DisconnectCause::ExpiryOfTimer),
            14 => Ok(DisconnectCause::SwmiRequestedDisconnection),
            15 => Ok(DisconnectCause::AcknowledgedServiceNotComplete),
            16 => Ok(DisconnectCause::UnknownTetraIdentity),
            17 => Ok(DisconnectCause::SsSpecificDisconnection),
            18 => Ok(DisconnectCause::UnknownExternalSubscriberIdentity),
            19 => Ok(DisconnectCause::CallRestorationOfTheOtherUserFailed),
            20 => Ok(DisconnectCause::CalledPartyRequiresEncryption),
            21 => Ok(DisconnectCause::ConcurrentSetUpNotSupported),
            22 => Ok(DisconnectCause::CalledPartyIsUnderTheSameDmGateOfTheCallingParty),
            23 => Ok(DisconnectCause::NonCallOwnerRequestedDisconnection),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `DisconnectCause`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl DisconnectCause {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            DisconnectCause::CauseNotDefinedOrUnknown => 0,
            DisconnectCause::UserRequestedDisconnection => 1,
            DisconnectCause::CalledPartyBusy => 2,
            DisconnectCause::CalledPartyNotReachable => 3,
            DisconnectCause::CalledPartyDoesNotSupportEncryption => 4,
            DisconnectCause::CongestionInInfrastructure => 5,
            DisconnectCause::NotAllowedTrafficCase => 6,
            DisconnectCause::IncompatibleTrafficCase => 7,
            DisconnectCause::RequestedServiceNotAvailable => 8,
            DisconnectCause::PreEmptiveUseOfResource => 9,
            DisconnectCause::InvalidCallIdentifier => 10,
            DisconnectCause::CallRejectedByTheCalledParty => 11,
            DisconnectCause::NoIdleCcEntity => 12,
            DisconnectCause::ExpiryOfTimer => 13,
            DisconnectCause::SwmiRequestedDisconnection => 14,
            DisconnectCause::AcknowledgedServiceNotComplete => 15,
            DisconnectCause::UnknownTetraIdentity => 16,
            DisconnectCause::SsSpecificDisconnection => 17,
            DisconnectCause::UnknownExternalSubscriberIdentity => 18,
            DisconnectCause::CallRestorationOfTheOtherUserFailed => 19,
            DisconnectCause::CalledPartyRequiresEncryption => 20,
            DisconnectCause::ConcurrentSetUpNotSupported => 21,
            DisconnectCause::CalledPartyIsUnderTheSameDmGateOfTheCallingParty => 22,
            DisconnectCause::NonCallOwnerRequestedDisconnection => 23,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<DisconnectCause> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<DisconnectCause> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: DisconnectCause) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for DisconnectCause`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for DisconnectCause {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            DisconnectCause::CauseNotDefinedOrUnknown => write!(f, "CauseNotDefinedOrUnknown"),
            DisconnectCause::UserRequestedDisconnection => write!(f, "UserRequestedDisconnection"),
            DisconnectCause::CalledPartyBusy => write!(f, "CalledPartyBusy"),
            DisconnectCause::CalledPartyNotReachable => write!(f, "CalledPartyNotReachable"),
            DisconnectCause::CalledPartyDoesNotSupportEncryption => write!(f, "CalledPartyDoesNotSupportEncryption"),
            DisconnectCause::CongestionInInfrastructure => write!(f, "CongestionInInfrastructure"),
            DisconnectCause::NotAllowedTrafficCase => write!(f, "NotAllowedTrafficCase"),
            DisconnectCause::IncompatibleTrafficCase => write!(f, "IncompatibleTrafficCase"),
            DisconnectCause::RequestedServiceNotAvailable => write!(f, "RequestedServiceNotAvailable"),
            DisconnectCause::PreEmptiveUseOfResource => write!(f, "PreEmptiveUseOfResource"),
            DisconnectCause::InvalidCallIdentifier => write!(f, "InvalidCallIdentifier"),
            DisconnectCause::CallRejectedByTheCalledParty => write!(f, "CallRejectedByTheCalledParty"),
            DisconnectCause::NoIdleCcEntity => write!(f, "NoIdleCcEntity"),
            DisconnectCause::ExpiryOfTimer => write!(f, "ExpiryOfTimer"),
            DisconnectCause::SwmiRequestedDisconnection => write!(f, "SwmiRequestedDisconnection"),
            DisconnectCause::AcknowledgedServiceNotComplete => write!(f, "AcknowledgedServiceNotComplete"),
            DisconnectCause::UnknownTetraIdentity => write!(f, "UnknownTetraIdentity"),
            DisconnectCause::SsSpecificDisconnection => write!(f, "SsSpecificDisconnection"),
            DisconnectCause::UnknownExternalSubscriberIdentity => write!(f, "UnknownExternalSubscriberIdentity"),
            DisconnectCause::CallRestorationOfTheOtherUserFailed => write!(f, "CallRestorationOfTheOtherUserFailed"),
            DisconnectCause::CalledPartyRequiresEncryption => write!(f, "CalledPartyRequiresEncryption"),
            DisconnectCause::ConcurrentSetUpNotSupported => write!(f, "ConcurrentSetUpNotSupported"),
            DisconnectCause::CalledPartyIsUnderTheSameDmGateOfTheCallingParty => {
                write!(f, "CalledPartyIsUnderTheSameDmGateOfTheCallingParty")
            }
            DisconnectCause::NonCallOwnerRequestedDisconnection => write!(f, "NonCallOwnerRequestedDisconnection"),
        }
    }
}
