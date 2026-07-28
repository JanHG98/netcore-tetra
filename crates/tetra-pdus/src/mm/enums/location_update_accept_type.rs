// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 16.10.35a Location update accept type
/// Bits: 3
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für location update accept type auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum LocationUpdateAcceptType {
    RoamingLocationUpdating = 0,
    TemporaryRegistration = 1,
    PeriodicLocationUpdating = 2,
    ItsiAttach = 3,
    ServiceRestorationRoamingLocationUpdating = 4,
    MigratingOrServiceRestorationMigratingLocationUpdating = 5,
    DemandLocationUpdating = 6,
    DisabledMsUpdating = 7,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for LocationUpdateAcceptType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for LocationUpdateAcceptType {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            0 => Ok(LocationUpdateAcceptType::RoamingLocationUpdating),
            1 => Ok(LocationUpdateAcceptType::TemporaryRegistration),
            2 => Ok(LocationUpdateAcceptType::PeriodicLocationUpdating),
            3 => Ok(LocationUpdateAcceptType::ItsiAttach),
            4 => Ok(LocationUpdateAcceptType::ServiceRestorationRoamingLocationUpdating),
            5 => Ok(LocationUpdateAcceptType::MigratingOrServiceRestorationMigratingLocationUpdating),
            6 => Ok(LocationUpdateAcceptType::DemandLocationUpdating),
            7 => Ok(LocationUpdateAcceptType::DisabledMsUpdating),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `LocationUpdateAcceptType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl LocationUpdateAcceptType {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            LocationUpdateAcceptType::RoamingLocationUpdating => 0,
            LocationUpdateAcceptType::TemporaryRegistration => 1,
            LocationUpdateAcceptType::PeriodicLocationUpdating => 2,
            LocationUpdateAcceptType::ItsiAttach => 3,
            LocationUpdateAcceptType::ServiceRestorationRoamingLocationUpdating => 4,
            LocationUpdateAcceptType::MigratingOrServiceRestorationMigratingLocationUpdating => 5,
            LocationUpdateAcceptType::DemandLocationUpdating => 6,
            LocationUpdateAcceptType::DisabledMsUpdating => 7,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<LocationUpdateAcceptType> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<LocationUpdateAcceptType> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: LocationUpdateAcceptType) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for LocationUpdateAcceptType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for LocationUpdateAcceptType {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            LocationUpdateAcceptType::RoamingLocationUpdating => write!(f, "RoamingLocationUpdating"),
            LocationUpdateAcceptType::TemporaryRegistration => write!(f, "TemporaryRegistration"),
            LocationUpdateAcceptType::PeriodicLocationUpdating => write!(f, "PeriodicLocationUpdating"),
            LocationUpdateAcceptType::ItsiAttach => write!(f, "ItsiAttach"),
            LocationUpdateAcceptType::ServiceRestorationRoamingLocationUpdating => write!(f, "ServiceRestorationRoamingLocationUpdating"),
            LocationUpdateAcceptType::MigratingOrServiceRestorationMigratingLocationUpdating => {
                write!(f, "MigratingOrServiceRestorationMigratingLocationUpdating")
            }
            LocationUpdateAcceptType::DemandLocationUpdating => write!(f, "DemandLocationUpdating"),
            LocationUpdateAcceptType::DisabledMsUpdating => write!(f, "DisabledMsUpdating"),
        }
    }
}
