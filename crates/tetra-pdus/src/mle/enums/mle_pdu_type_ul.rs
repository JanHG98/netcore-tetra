// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 18.5.20 MLE PDU types
/// Bits: 3
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für MLE-Verbindungssteuerung Protokollnachricht (PDU) type ul auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum MlePduTypeUl {
    UPrepare = 0,
    UPrepareDa = 1,
    UIrregularChannelAdvice = 2,
    UChannelClassAdvice = 3,
    URestore = 4,
    UChannelRequest = 6,
    ExtPdu = 7,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for MlePduTypeUl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for MlePduTypeUl {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            0 => Ok(MlePduTypeUl::UPrepare),
            1 => Ok(MlePduTypeUl::UPrepareDa),
            2 => Ok(MlePduTypeUl::UIrregularChannelAdvice),
            3 => Ok(MlePduTypeUl::UChannelClassAdvice),
            4 => Ok(MlePduTypeUl::URestore),
            6 => Ok(MlePduTypeUl::UChannelRequest),
            7 => Ok(MlePduTypeUl::ExtPdu),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `MlePduTypeUl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MlePduTypeUl {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            MlePduTypeUl::UPrepare => 0,
            MlePduTypeUl::UPrepareDa => 1,
            MlePduTypeUl::UIrregularChannelAdvice => 2,
            MlePduTypeUl::UChannelClassAdvice => 3,
            MlePduTypeUl::URestore => 4,
            MlePduTypeUl::UChannelRequest => 6,
            MlePduTypeUl::ExtPdu => 7,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<MlePduTypeUl> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<MlePduTypeUl> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: MlePduTypeUl) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for MlePduTypeUl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for MlePduTypeUl {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            MlePduTypeUl::UPrepare => write!(f, "UPrepare"),
            MlePduTypeUl::UPrepareDa => write!(f, "UPrepareDa"),
            MlePduTypeUl::UIrregularChannelAdvice => write!(f, "UIrregularChannelAdvice"),
            MlePduTypeUl::UChannelClassAdvice => write!(f, "UChannelClassAdvice"),
            MlePduTypeUl::URestore => write!(f, "URestore"),
            MlePduTypeUl::UChannelRequest => write!(f, "UChannelRequest"),
            MlePduTypeUl::ExtPdu => write!(f, "ExtPdu"),
        }
    }
}
