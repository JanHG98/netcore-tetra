// NETCORE-KOMMENTAR – Was: Enthält die Logik oder Einstellungen für address.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::{fmt, num::ParseIntError, str::FromStr};

use serde::{Deserialize, Serialize};

// Was: Legt den festen Wert `MAX_SSI` für max TETRA-Teilnehmerkennung (SSI) fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const MAX_SSI: u32 = 0x00ff_ffff;

#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für address error auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum AddressError {
    OutOfRange(u32),
    InvalidDecimal(String),
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Display for AddressError`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for AddressError {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            Self::OutOfRange(value) => write!(f, "SSI value {value} exceeds 24-bit range"),
            Self::InvalidDecimal(value) => write!(f, "invalid decimal SSI value: {value}"),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `std::error::Error for AddressError {}`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::error::Error for AddressError {}

// Was: Implementiert das zugehörige Verhalten für `From<ParseIntError> for AddressError`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<ParseIntError> for AddressError {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(error: ParseIntError) -> Self {
        Self::InvalidDecimal(error.to_string())
    }
}

// Was: Definiert das Makro `ssi_type`, das wiederkehrenden Rust-Code erzeugt.
// Warum: Gleichartige Strukturen werden dadurch nur einmal beschrieben und können nicht unbemerkt auseinanderlaufen.
macro_rules! ssi_type {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(u32);

        // Was: Implementiert das zugehörige Verhalten für `$name`.
        // Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
        impl $name {
            // Was: Legt den festen Wert `MIN` für min fest.
            // Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
            pub const MIN: u32 = 0;
            // Was: Legt den festen Wert `MAX` für max fest.
            // Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
            pub const MAX: u32 = MAX_SSI;

            // Was: Diese Funktion liest den vorgesehenen Arbeitsschritt.
            // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
            pub const fn get(self) -> u32 {
                self.0
            }

            // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
            // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
            pub fn new(value: u32) -> Result<Self, AddressError> {
                if value <= Self::MAX {
                    Ok(Self(value))
                } else {
                    Err(AddressError::OutOfRange(value))
                }
            }
        }

        // Was: Implementiert das zugehörige Verhalten für `TryFrom<u32> for $name`.
        // Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
        impl TryFrom<u32> for $name {
            // Was: Vergibt für error einen fachlich verständlichen Typnamen.
            // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
            type Error = AddressError;

            // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
            // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
            fn try_from(value: u32) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        // Was: Implementiert das zugehörige Verhalten für `From<$name> for u32`.
        // Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
        impl From<$name> for u32 {
            // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
            // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
            fn from(value: $name) -> Self {
                value.get()
            }
        }

        // Was: Implementiert das zugehörige Verhalten für `fmt::Display for $name`.
        // Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
        impl fmt::Display for $name {
            // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
            // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        // Was: Implementiert das zugehörige Verhalten für `FromStr for $name`.
        // Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
        impl FromStr for $name {
            // Was: Vergibt für err einen fachlich verständlichen Typnamen.
            // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
            type Err = AddressError;

            // Was: Wandelt Eingangsdaten in str um.
            // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let parsed = value
                    .parse::<u32>()
                    .map_err(|_| AddressError::InvalidDecimal(value.to_owned()))?;
                Self::new(parsed)
            }
        }
    };
}

ssi_type!(Ssi);
ssi_type!(Issi);
ssi_type!(Gssi);

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Führt den Arbeitsschritt `accepts_full_24_bit_range` für accepts full 24 bit range aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn accepts_full_24_bit_range() {
        assert_eq!(Issi::new(MAX_SSI).unwrap().get(), MAX_SSI);
        assert_eq!(Gssi::new(0).unwrap().get(), 0);
    }

    #[test]
    // Was: Führt den Arbeitsschritt `rejects_values_above_24_bits` für rejects values above 24 bits aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn rejects_values_above_24_bits() {
        assert!(matches!(Ssi::new(MAX_SSI + 1), Err(AddressError::OutOfRange(_))));
    }

    #[test]
    // Was: Führt den Arbeitsschritt `serde_is_numeric_and_roundtrips` für serde is numeric and roundtrips aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn serde_is_numeric_and_roundtrips() {
        let value = Issi::new(4_010_001).unwrap();
        let encoded = serde_json::to_string(&value).unwrap();
        assert_eq!(encoded, "4010001");
        assert_eq!(serde_json::from_str::<Issi>(&encoded).unwrap(), value);
    }
}
