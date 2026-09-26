// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use core::fmt;

use tetra_core::{PduParseErr, expect_value};

use crate::cmce::enums::short_report_type::ShortReportType;

/// Clause 29.4.2.3 SDS-SHORT REPORT
/// This PDU shall be used to report on the progress of previously received SDS data
#[derive(Debug, Clone, Copy, PartialEq)]
// Was: Bündelt die zusammengehörigen Werte für TETRA-Kurznachricht (SDS) short report in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct SdsShortReport {
    /// 2 bits
    short_report_type: ShortReportType,
    /// 8 bits. The same value as in the corresponding request PDU
    message_reference: u8,
}

// Was: Implementiert das zugehörige Verhalten für `SdsShortReport`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl SdsShortReport {
    // Was: Führt den Arbeitsschritt `short_report_type` für short report type aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn short_report_type(&self) -> ShortReportType {
        self.short_report_type
    }

    // Was: Führt den Arbeitsschritt `message_reference` für Nachricht reference aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn message_reference(&self) -> u8 {
        self.message_reference
    }

    // No from_bitbuf, to_bitbuf functions, as we'll parse this in a bit of a different way originating from an enum field in the U-STATUS PDU pre-coded status field
    // Was: Wandelt Eingangsdaten in u16 um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_u16(val: u16) -> Result<Self, PduParseErr> {
        // TODO FIXME implement parsing of the pre-coded status field into this struct, as defined in table 14.72
        let pdu_type = val >> 10;
        expect_value!(pdu_type, 0b011111)?;
        let raw = ((val >> 8) & 0x3) as u64;
        // raw is masked to 2 bits, ShortReportType covers all 4 values today,
        // but propagate as InvalidValue to stay panic-free if the enum changes.
        let short_report_type = ShortReportType::try_from(raw).map_err(|_| PduParseErr::InvalidValue {
            field: "short_report_type",
            value: raw,
        })?;
        let message_reference = (val & 0xFF) as u8;

        Ok(SdsShortReport {
            short_report_type,
            message_reference,
        })
    }

    // Was: Wandelt den vorhandenen Wert in u16 um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_u16(&self) -> u16 {
        // TODO FIXME implement conversion of this struct into the pre-coded status field, as defined in table 14.72
        assert!(self.short_report_type.into_raw() <= 0b11, "short_report_type must be 2 bits");
        (0b011111 << 10) | ((self.short_report_type.into_raw() as u16) << 8) | (self.message_reference as u16 & 0xFF)
    }
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Display for SdsShortReport`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for SdsShortReport {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SdsShortReport {{ short_report_type: {:?}, message_reference: {:?} }}",
            self.short_report_type, self.message_reference,
        )
    }
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Prüft automatisch den Fall to u16 roundtrip.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_to_u16_roundtrip() {
        // Synthetic test, not real world data
        let sds = SdsShortReport {
            short_report_type: ShortReportType::MessageReceived,
            message_reference: 0b00000001,
        };

        let converted = sds.to_u16();
        assert_eq!(converted, 0b0111111000000001);

        let parsed = SdsShortReport::from_u16(converted).unwrap();
        assert_eq!(parsed.short_report_type, sds.short_report_type);
        assert_eq!(parsed.message_reference, sds.message_reference);
    }
}
