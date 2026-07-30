// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use crate::cmce::fields::sds_short_report::SdsShortReport;

/// Clause 14.8.34 Pre-coded status
/// The pre-coded status information element shall define general purpose status messages known to all TETRA systems as
/// defined in table 14.72 and shall provide support for the SDS-TL "short reporting" protocol.
/// Bits: 2
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für pre coded Status auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum PreCodedStatus {
    Emergency,
    Reserved(u16),
    SdsTl(SdsShortReport),
    NetworkUserSpecific(u16),
}

// Was: Implementiert das zugehörige Verhalten für `From<u16> for PreCodedStatus`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<u16> for PreCodedStatus {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(x: u16) -> Self {
        // ETSI EN 300 392-2 Table 14.72:
        //   0           = Emergency
        //   1..=31743   = Reserved
        //   31744..=32767 = SDS-TL short report (pdu_type bits 15..10 == 0b011111)
        //   32768..=65535 = Network/User Specific
        //
        // SDS-TL parsing can fail (expect_value on pdu_type bits, plus future
        // additions to ShortReportType), so fall back to Reserved(x) on Err
        // rather than panic on an unwrap. Wire traffic is never trusted input.
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match x {
            0 => PreCodedStatus::Emergency,
            1..=31743 => PreCodedStatus::Reserved(x),
            31744..=32767 => match SdsShortReport::from_u16(x) {
                Ok(report) => PreCodedStatus::SdsTl(report),
                Err(_) => PreCodedStatus::Reserved(x),
            },
            32768..=65535 => PreCodedStatus::NetworkUserSpecific(x),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `PreCodedStatus`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl PreCodedStatus {
    /// Convert this enum back into the raw integer value
    // Was: Wandelt den vorhandenen Wert in raw um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn into_raw(self) -> u16 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            PreCodedStatus::Emergency => 0,
            PreCodedStatus::Reserved(x) => x,
            PreCodedStatus::SdsTl(x) => x.to_u16(),
            PreCodedStatus::NetworkUserSpecific(x) => x,
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `From<PreCodedStatus> for u16`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<PreCodedStatus> for u16 {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(e: PreCodedStatus) -> Self {
        e.into_raw()
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for PreCodedStatus`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for PreCodedStatus {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            PreCodedStatus::Emergency => write!(f, "Emergency"),
            PreCodedStatus::Reserved(x) => write!(f, "Reserved({})", x),
            PreCodedStatus::SdsTl(x) => write!(f, "SdsTl({})", x),
            PreCodedStatus::NetworkUserSpecific(x) => write!(f, "NetworkUserSpecific({})", x),
        }
    }
}
