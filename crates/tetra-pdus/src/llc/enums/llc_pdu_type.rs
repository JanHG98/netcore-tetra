// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 21.2.1 LLC PDU types
/// Bits: 4
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für LLC-Verbindungsschicht Protokollnachricht (PDU) type auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum LlcPduType {
    BlAdata = 0,
    BlData = 1,
    BlUdata = 2,
    BlAck = 3,
    BlAdataFcs = 4,
    BlDataFcs = 5,
    BlUdataFcs = 6,
    BlAckFcs = 7,
    AlSetup = 8,
    AlDataAlFinal = 9,
    AlAlUdataAlUfinal = 10,
    AlAckAlRnr = 11,
    AlReconnect = 12,
    SuppLlcPdu = 13,
    L2SigPdu = 14,
    AlDisc = 15,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for LlcPduType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for LlcPduType {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            0 => Ok(LlcPduType::BlAdata),
            1 => Ok(LlcPduType::BlData),
            2 => Ok(LlcPduType::BlUdata),
            3 => Ok(LlcPduType::BlAck),
            4 => Ok(LlcPduType::BlAdataFcs),
            5 => Ok(LlcPduType::BlDataFcs),
            6 => Ok(LlcPduType::BlUdataFcs),
            7 => Ok(LlcPduType::BlAckFcs),
            8 => Ok(LlcPduType::AlSetup),
            9 => Ok(LlcPduType::AlDataAlFinal),
            10 => Ok(LlcPduType::AlAlUdataAlUfinal),
            11 => Ok(LlcPduType::AlAckAlRnr),
            12 => Ok(LlcPduType::AlReconnect),
            13 => Ok(LlcPduType::SuppLlcPdu),
            14 => Ok(LlcPduType::L2SigPdu),
            15 => Ok(LlcPduType::AlDisc),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `LlcPduType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl LlcPduType {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            LlcPduType::BlAdata => 0,
            LlcPduType::BlData => 1,
            LlcPduType::BlUdata => 2,
            LlcPduType::BlAck => 3,
            LlcPduType::BlAdataFcs => 4,
            LlcPduType::BlDataFcs => 5,
            LlcPduType::BlUdataFcs => 6,
            LlcPduType::BlAckFcs => 7,
            LlcPduType::AlSetup => 8,
            LlcPduType::AlDataAlFinal => 9,
            LlcPduType::AlAlUdataAlUfinal => 10,
            LlcPduType::AlAckAlRnr => 11,
            LlcPduType::AlReconnect => 12,
            LlcPduType::SuppLlcPdu => 13,
            LlcPduType::L2SigPdu => 14,
            LlcPduType::AlDisc => 15,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<LlcPduType> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<LlcPduType> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: LlcPduType) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for LlcPduType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for LlcPduType {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            LlcPduType::BlAdata => write!(f, "BlAdata"),
            LlcPduType::BlData => write!(f, "BlData"),
            LlcPduType::BlUdata => write!(f, "BlUdata"),
            LlcPduType::BlAck => write!(f, "BlAck"),
            LlcPduType::BlAdataFcs => write!(f, "BlAdataFcs"),
            LlcPduType::BlDataFcs => write!(f, "BlDataFcs"),
            LlcPduType::BlUdataFcs => write!(f, "BlUdataFcs"),
            LlcPduType::BlAckFcs => write!(f, "BlAckFcs"),
            LlcPduType::AlSetup => write!(f, "AlSetup"),
            LlcPduType::AlDataAlFinal => write!(f, "AlDataAlFinal"),
            LlcPduType::AlAlUdataAlUfinal => write!(f, "AlAlUdataAlUfinal"),
            LlcPduType::AlAckAlRnr => write!(f, "AlAckAlRnr"),
            LlcPduType::AlReconnect => write!(f, "AlReconnect"),
            LlcPduType::SuppLlcPdu => write!(f, "SuppLlcPdu"),
            LlcPduType::L2SigPdu => write!(f, "L2SigPdu"),
            LlcPduType::AlDisc => write!(f, "AlDisc"),
        }
    }
}
