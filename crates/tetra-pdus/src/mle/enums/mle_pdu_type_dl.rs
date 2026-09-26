// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 18.5.20 MLE PDU types
/// Bits: 3
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für MLE-Verbindungssteuerung Protokollnachricht (PDU) type dl auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum MlePduTypeDl {
    DNewCell = 0,
    DPrepareFail = 1,
    DNwrkBroadcast = 2,
    DNwrkBroadcastExt = 3,
    DRestoreAck = 4,
    DRestoreFail = 5,
    DChannelResponse = 6,
    ExtPdu = 7,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for MlePduTypeDl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for MlePduTypeDl {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            0 => Ok(MlePduTypeDl::DNewCell),
            1 => Ok(MlePduTypeDl::DPrepareFail),
            2 => Ok(MlePduTypeDl::DNwrkBroadcast),
            3 => Ok(MlePduTypeDl::DNwrkBroadcastExt),
            4 => Ok(MlePduTypeDl::DRestoreAck),
            5 => Ok(MlePduTypeDl::DRestoreFail),
            6 => Ok(MlePduTypeDl::DChannelResponse),
            7 => Ok(MlePduTypeDl::ExtPdu),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `MlePduTypeDl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MlePduTypeDl {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            MlePduTypeDl::DNewCell => 0,
            MlePduTypeDl::DPrepareFail => 1,
            MlePduTypeDl::DNwrkBroadcast => 2,
            MlePduTypeDl::DNwrkBroadcastExt => 3,
            MlePduTypeDl::DRestoreAck => 4,
            MlePduTypeDl::DRestoreFail => 5,
            MlePduTypeDl::DChannelResponse => 6,
            MlePduTypeDl::ExtPdu => 7,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<MlePduTypeDl> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<MlePduTypeDl> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: MlePduTypeDl) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for MlePduTypeDl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for MlePduTypeDl {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            MlePduTypeDl::DNewCell => write!(f, "DNewCell"),
            MlePduTypeDl::DPrepareFail => write!(f, "DPrepareFail"),
            MlePduTypeDl::DNwrkBroadcast => write!(f, "DNwrkBroadcast"),
            MlePduTypeDl::DNwrkBroadcastExt => write!(f, "DNwrkBroadcastExt"),
            MlePduTypeDl::DRestoreAck => write!(f, "DRestoreAck"),
            MlePduTypeDl::DRestoreFail => write!(f, "DRestoreFail"),
            MlePduTypeDl::DChannelResponse => write!(f, "DChannelResponse"),
            MlePduTypeDl::ExtPdu => write!(f, "ExtPdu"),
        }
    }
}
