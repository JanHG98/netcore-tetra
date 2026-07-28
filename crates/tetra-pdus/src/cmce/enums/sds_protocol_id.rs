// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 29.4.3.9 SDS Protocol identifier. Values undefined here may be user definition or reserved
/// Bits: 8
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für TETRA-Kurznachricht (SDS) protocol Kennung auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum SdsProtocolId {
    Otak = 1,
    SimpleTextMessaging = 2,
    SimpleLocationSystem = 3,
    WirelessDatagramProtocol = 4,
    WirelessControlMessageProtocol = 5,
    MDmo = 6,
    PinAuth = 7,
    EteeMessage = 8,
    SimpleImmediateTextMessaging = 9,
    LocationInformationProtocol = 10,
    NetAssistProtocol2 = 11,
    ConcatenatedSdsMessage = 12,
    Dotam = 13,
    SimpleAgnssService = 14,
    TextMessagingSdsTl = 130,
    LocationSystemSdsTl = 131,
    WirelessDatagramProtocolSdsTl = 132,
    WirelessControlMessageProtocolSdsTl = 133,
    MDmoSdsTl = 134,
    EteeMessageSdsTl = 136,
    ImmediateTextMessagingSdsTl = 137,
    MessageWithUserDataHeader = 138,
    ConcatenatedSdsMessageSdsTl = 140,
    AgnssServiceSdsTl = 141,
}

// Was: Implementiert das zugehörige Verhalten für `std::convert::TryFrom<u64> for SdsProtocolId`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::convert::TryFrom<u64> for SdsProtocolId {
    // Was: Vergibt für error einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Error = ();
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            1 => Ok(SdsProtocolId::Otak),
            2 => Ok(SdsProtocolId::SimpleTextMessaging),
            3 => Ok(SdsProtocolId::SimpleLocationSystem),
            4 => Ok(SdsProtocolId::WirelessDatagramProtocol),
            5 => Ok(SdsProtocolId::WirelessControlMessageProtocol),
            6 => Ok(SdsProtocolId::MDmo),
            7 => Ok(SdsProtocolId::PinAuth),
            8 => Ok(SdsProtocolId::EteeMessage),
            9 => Ok(SdsProtocolId::SimpleImmediateTextMessaging),
            10 => Ok(SdsProtocolId::LocationInformationProtocol),
            11 => Ok(SdsProtocolId::NetAssistProtocol2),
            12 => Ok(SdsProtocolId::ConcatenatedSdsMessage),
            13 => Ok(SdsProtocolId::Dotam),
            14 => Ok(SdsProtocolId::SimpleAgnssService),
            130 => Ok(SdsProtocolId::TextMessagingSdsTl),
            131 => Ok(SdsProtocolId::LocationSystemSdsTl),
            132 => Ok(SdsProtocolId::WirelessDatagramProtocolSdsTl),
            133 => Ok(SdsProtocolId::WirelessControlMessageProtocolSdsTl),
            134 => Ok(SdsProtocolId::MDmoSdsTl),
            136 => Ok(SdsProtocolId::EteeMessageSdsTl),
            137 => Ok(SdsProtocolId::ImmediateTextMessagingSdsTl),
            138 => Ok(SdsProtocolId::MessageWithUserDataHeader),
            140 => Ok(SdsProtocolId::ConcatenatedSdsMessageSdsTl),
            141 => Ok(SdsProtocolId::AgnssServiceSdsTl),
            _ => Err(()),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `SdsProtocolId`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl SdsProtocolId {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u64 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            SdsProtocolId::Otak => 1,
            SdsProtocolId::SimpleTextMessaging => 2,
            SdsProtocolId::SimpleLocationSystem => 3,
            SdsProtocolId::WirelessDatagramProtocol => 4,
            SdsProtocolId::WirelessControlMessageProtocol => 5,
            SdsProtocolId::MDmo => 6,
            SdsProtocolId::PinAuth => 7,
            SdsProtocolId::EteeMessage => 8,
            SdsProtocolId::SimpleImmediateTextMessaging => 9,
            SdsProtocolId::LocationInformationProtocol => 10,
            SdsProtocolId::NetAssistProtocol2 => 11,
            SdsProtocolId::ConcatenatedSdsMessage => 12,
            SdsProtocolId::Dotam => 13,
            SdsProtocolId::SimpleAgnssService => 14,
            SdsProtocolId::TextMessagingSdsTl => 130,
            SdsProtocolId::LocationSystemSdsTl => 131,
            SdsProtocolId::WirelessDatagramProtocolSdsTl => 132,
            SdsProtocolId::WirelessControlMessageProtocolSdsTl => 133,
            SdsProtocolId::MDmoSdsTl => 134,
            SdsProtocolId::EteeMessageSdsTl => 136,
            SdsProtocolId::ImmediateTextMessagingSdsTl => 137,
            SdsProtocolId::MessageWithUserDataHeader => 138,
            SdsProtocolId::ConcatenatedSdsMessageSdsTl => 140,
            SdsProtocolId::AgnssServiceSdsTl => 141,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<SdsProtocolId> for u64`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<SdsProtocolId> for u64 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: SdsProtocolId) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for SdsProtocolId`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for SdsProtocolId {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            SdsProtocolId::Otak => write!(f, "Otak"),
            SdsProtocolId::SimpleTextMessaging => write!(f, "SimpleTextMessaging"),
            SdsProtocolId::SimpleLocationSystem => write!(f, "SimpleLocationSystem"),
            SdsProtocolId::WirelessDatagramProtocol => write!(f, "WirelessDatagramProtocol"),
            SdsProtocolId::WirelessControlMessageProtocol => write!(f, "WirelessControlMessageProtocol"),
            SdsProtocolId::MDmo => write!(f, "MDmo"),
            SdsProtocolId::PinAuth => write!(f, "PinAuth"),
            SdsProtocolId::EteeMessage => write!(f, "EteeMessage"),
            SdsProtocolId::SimpleImmediateTextMessaging => write!(f, "SimpleImmediateTextMessaging"),
            SdsProtocolId::LocationInformationProtocol => write!(f, "LocationInformationProtocol"),
            SdsProtocolId::NetAssistProtocol2 => write!(f, "NetAssistProtocol2"),
            SdsProtocolId::ConcatenatedSdsMessage => write!(f, "ConcatenatedSdsMessage"),
            SdsProtocolId::Dotam => write!(f, "Dotam"),
            SdsProtocolId::SimpleAgnssService => write!(f, "SimpleAgnssService"),
            SdsProtocolId::TextMessagingSdsTl => write!(f, "TextMessagingSdsTl"),
            SdsProtocolId::LocationSystemSdsTl => write!(f, "LocationSystemSdsTl"),
            SdsProtocolId::WirelessDatagramProtocolSdsTl => write!(f, "WirelessDatagramProtocolSdsTl"),
            SdsProtocolId::WirelessControlMessageProtocolSdsTl => write!(f, "WirelessControlMessageProtocolSdsTl"),
            SdsProtocolId::MDmoSdsTl => write!(f, "MDmoSdsTl"),
            SdsProtocolId::EteeMessageSdsTl => write!(f, "EteeMessageSdsTl"),
            SdsProtocolId::ImmediateTextMessagingSdsTl => write!(f, "ImmediateTextMessagingSdsTl"),
            SdsProtocolId::MessageWithUserDataHeader => write!(f, "MessageWithUserDataHeader"),
            SdsProtocolId::ConcatenatedSdsMessageSdsTl => write!(f, "ConcatenatedSdsMessageSdsTl"),
            SdsProtocolId::AgnssServiceSdsTl => write!(f, "AgnssServiceSdsTl"),
        }
    }
}
