// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 16.10.42 Reject cause
/// Bits: 5
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für reject cause auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum RejectCause {
    ItsiAtsiUnknown = 1,
    IllegalMs = 2,
    LaNotAllowed = 3,
    LaUnknown = 4,
    NetworkFailure = 5,
    Congestion = 6,
    ForwardRegistrationFailure = 7,
    ServiceNotSubscribed = 8,
    MandatoryElementError = 9,
    MessageConsistencyError = 10,
    RoamingNotSupported = 11,
    MigrationNotSupported = 12,
    NoCipherKsg = 13,
    IdentifiedCipherKsgNotSupported = 14,
    RequestedCipherKeyTypeNotAvailable = 15,
    IdentifiedCipherKeyNotAvailable = 16,
    ExpiryOfTimer = 17,
    CipheringRequired = 18,
    AuthenticationFailure = 19,
    UseCaCellNotPermitted = 20,
    UseDaCellNotPermitted = 21,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for RejectCause`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for RejectCause {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            1 => Ok(RejectCause::ItsiAtsiUnknown),
            2 => Ok(RejectCause::IllegalMs),
            3 => Ok(RejectCause::LaNotAllowed),
            4 => Ok(RejectCause::LaUnknown),
            5 => Ok(RejectCause::NetworkFailure),
            6 => Ok(RejectCause::Congestion),
            7 => Ok(RejectCause::ForwardRegistrationFailure),
            8 => Ok(RejectCause::ServiceNotSubscribed),
            9 => Ok(RejectCause::MandatoryElementError),
            10 => Ok(RejectCause::MessageConsistencyError),
            11 => Ok(RejectCause::RoamingNotSupported),
            12 => Ok(RejectCause::MigrationNotSupported),
            13 => Ok(RejectCause::NoCipherKsg),
            14 => Ok(RejectCause::IdentifiedCipherKsgNotSupported),
            15 => Ok(RejectCause::RequestedCipherKeyTypeNotAvailable),
            16 => Ok(RejectCause::IdentifiedCipherKeyNotAvailable),
            17 => Ok(RejectCause::ExpiryOfTimer),
            18 => Ok(RejectCause::CipheringRequired),
            19 => Ok(RejectCause::AuthenticationFailure),
            20 => Ok(RejectCause::UseCaCellNotPermitted),
            21 => Ok(RejectCause::UseDaCellNotPermitted),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `RejectCause`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl RejectCause {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        self as u64
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<RejectCause> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<RejectCause> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: RejectCause) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for RejectCause`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for RejectCause {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            RejectCause::ItsiAtsiUnknown => write!(f, "ITSI/ATSI unknown"),
            RejectCause::IllegalMs => write!(f, "Illegal MS"),
            RejectCause::LaNotAllowed => write!(f, "LA not allowed"),
            RejectCause::LaUnknown => write!(f, "LA unknown"),
            RejectCause::NetworkFailure => write!(f, "Network failure"),
            RejectCause::Congestion => write!(f, "Congestion"),
            RejectCause::ForwardRegistrationFailure => write!(f, "Forward registration failure"),
            RejectCause::ServiceNotSubscribed => write!(f, "Service not subscribed"),
            RejectCause::MandatoryElementError => write!(f, "Mandatory element error"),
            RejectCause::MessageConsistencyError => write!(f, "Message consistency error"),
            RejectCause::RoamingNotSupported => write!(f, "Roaming not supported"),
            RejectCause::MigrationNotSupported => write!(f, "Migration not supported"),
            RejectCause::NoCipherKsg => write!(f, "No cipher KSG"),
            RejectCause::IdentifiedCipherKsgNotSupported => write!(f, "Identified cipher KSG not supported"),
            RejectCause::RequestedCipherKeyTypeNotAvailable => write!(f, "Requested cipher key type not available"),
            RejectCause::IdentifiedCipherKeyNotAvailable => write!(f, "Identified cipher key not available"),
            RejectCause::ExpiryOfTimer => write!(f, "Expiry of timer"),
            RejectCause::CipheringRequired => write!(f, "Ciphering required"),
            RejectCause::AuthenticationFailure => write!(f, "Authentication failure"),
            RejectCause::UseCaCellNotPermitted => write!(f, "Use of CA cell not permitted"),
            RejectCause::UseDaCellNotPermitted => write!(f, "Use of DA cell not permitted"),
        }
    }
}
