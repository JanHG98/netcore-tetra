// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 16.10.35 Location update type
/// Almost identical to MmLocationUpdateAcceptType
/// Bits: 3
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für location update type auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum LocationUpdateType {
    RoamingLocationUpdating = 0,
    MigratingLocationUpdating = 1,
    PeriodicLocationUpdating = 2,
    ItsiAttach = 3,
    ServiceRestorationRoamingLocationUpdating = 4,
    ServiceRestorationMigratingLocationUpdating = 5,
    DemandLocationUpdating = 6,
    DisabledMsUpdating = 7,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for LocationUpdateType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for LocationUpdateType {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            0 => Ok(LocationUpdateType::RoamingLocationUpdating),
            1 => Ok(LocationUpdateType::MigratingLocationUpdating),
            2 => Ok(LocationUpdateType::PeriodicLocationUpdating),
            3 => Ok(LocationUpdateType::ItsiAttach),
            4 => Ok(LocationUpdateType::ServiceRestorationRoamingLocationUpdating),
            5 => Ok(LocationUpdateType::ServiceRestorationMigratingLocationUpdating),
            6 => Ok(LocationUpdateType::DemandLocationUpdating),
            7 => Ok(LocationUpdateType::DisabledMsUpdating),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `LocationUpdateType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl LocationUpdateType {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            LocationUpdateType::RoamingLocationUpdating => 0,
            LocationUpdateType::MigratingLocationUpdating => 1,
            LocationUpdateType::PeriodicLocationUpdating => 2,
            LocationUpdateType::ItsiAttach => 3,
            LocationUpdateType::ServiceRestorationRoamingLocationUpdating => 4,
            LocationUpdateType::ServiceRestorationMigratingLocationUpdating => 5,
            LocationUpdateType::DemandLocationUpdating => 6,
            LocationUpdateType::DisabledMsUpdating => 7,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<LocationUpdateType> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<LocationUpdateType> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: LocationUpdateType) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for LocationUpdateType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for LocationUpdateType {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            LocationUpdateType::RoamingLocationUpdating => write!(f, "RoamingLocationUpdating"),
            LocationUpdateType::MigratingLocationUpdating => write!(f, "MigratingLocationUpdating"),
            LocationUpdateType::PeriodicLocationUpdating => write!(f, "PeriodicLocationUpdating"),
            LocationUpdateType::ItsiAttach => write!(f, "ItsiAttach"),
            LocationUpdateType::ServiceRestorationRoamingLocationUpdating => write!(f, "ServiceRestorationRoamingLocationUpdating"),
            LocationUpdateType::ServiceRestorationMigratingLocationUpdating => write!(f, "ServiceRestorationMigratingLocationUpdating"),
            LocationUpdateType::DemandLocationUpdating => write!(f, "DemandLocationUpdating"),
            LocationUpdateType::DisabledMsUpdating => write!(f, "DisabledMsUpdating"),
        }
    }
}
