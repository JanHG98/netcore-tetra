// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use core::fmt;

use tetra_core::typed_pdu_fields::{delimiters, typed};
use tetra_core::{BitBuffer, expect_pdu_type, pdu_parse_error::PduParseErr};

use crate::mle::enums::mle_pdu_type_ul::MlePduTypeUl;
use crate::mle::pdus::trailing_sdu::{read_trailing_sdu, write_trailing_sdu};

/// U-RESTORE PDU (ETSI EN 300 392-2, clause 18.4.1.4.7).
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für urestore in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct URestore {
    pub mcc: Option<u16>,
    pub mnc: Option<u16>,
    pub la: Option<u16>,
    /// Embedded CMCE U-CALL RESTORE PDU.
    pub sdu: Option<BitBuffer>,
}

// Was: Implementiert das zugehörige Verhalten für `URestore`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl URestore {
    // Was: Wandelt Eingangsdaten in bitbuf um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_bitbuf(buffer: &mut BitBuffer) -> Result<Self, PduParseErr> {
        let pdu_type = buffer.read_field(3, "pdu_type")?;
        expect_pdu_type!(pdu_type, MlePduTypeUl::URestore)?;

        let obit = delimiters::read_obit(buffer)?;
        let mcc = typed::parse_type2_generic(obit, buffer, 10, "mcc")?.map(|v| v as u16);
        let mnc = typed::parse_type2_generic(obit, buffer, 14, "mnc")?.map(|v| v as u16);
        let la = typed::parse_type2_generic(obit, buffer, 14, "la")?.map(|v| v as u16);

        Ok(Self {
            mcc,
            mnc,
            la,
            sdu: read_trailing_sdu(buffer),
        })
    }

    // Was: Wandelt den vorhandenen Wert in bitbuf um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_bitbuf(&self, buffer: &mut BitBuffer) -> Result<(), PduParseErr> {
        validate_range(self.mcc, 10, "mcc")?;
        validate_range(self.mnc, 14, "mnc")?;
        validate_range(self.la, 14, "la")?;

        buffer.write_bits(MlePduTypeUl::URestore.into_raw(), 3);
        let obit = self.mcc.is_some() || self.mnc.is_some() || self.la.is_some();
        delimiters::write_obit(buffer, obit as u8);
        typed::write_type2_generic(obit, buffer, self.mcc.map(u64::from), 10);
        typed::write_type2_generic(obit, buffer, self.mnc.map(u64::from), 14);
        typed::write_type2_generic(obit, buffer, self.la.map(u64::from), 14);
        write_trailing_sdu(buffer, &self.sdu);
        Ok(())
    }
}

// Was: Diese Funktion prüft range.
// Warum: Unzulässige Werte werden dadurch erkannt, bevor sie im Betrieb Schaden anrichten.
fn validate_range(
    value: Option<u16>,
    bits: usize,
    field: &'static str,
) -> Result<(), PduParseErr> {
    if let Some(value) = value {
        let maximum = (1u64 << bits) - 1;
        if u64::from(value) > maximum {
            return Err(PduParseErr::InvalidValue {
                field,
                value: u64::from(value),
            });
        }
    }
    Ok(())
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Display for URestore`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for URestore {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "URestore {{ mcc: {:?}, mnc: {:?}, la: {:?}, sdu_bits: {} }}",
            self.mcc,
            self.mnc,
            self.la,
            self.sdu.as_ref().map_or(0, BitBuffer::get_len),
        )
    }
}
