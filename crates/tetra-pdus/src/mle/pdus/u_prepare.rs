// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use core::fmt;

use tetra_core::typed_pdu_fields::{delimiters, typed};
use tetra_core::{BitBuffer, expect_pdu_type, pdu_parse_error::PduParseErr};

use crate::mle::enums::mle_pdu_type_ul::MlePduTypeUl;
use crate::mle::pdus::trailing_sdu::{read_trailing_sdu, write_trailing_sdu};

/// U-PREPARE PDU (ETSI EN 300 392-2, clause 18.4.1.4.6).
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für uprepare in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct UPrepare {
    pub cell_identifier_ca: Option<u8>,
    /// Optional embedded MM/OTAR PDU.
    pub sdu: Option<BitBuffer>,
}

// Was: Implementiert das zugehörige Verhalten für `UPrepare`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl UPrepare {
    // Was: Wandelt Eingangsdaten in bitbuf um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_bitbuf(buffer: &mut BitBuffer) -> Result<Self, PduParseErr> {
        let pdu_type = buffer.read_field(3, "pdu_type")?;
        expect_pdu_type!(pdu_type, MlePduTypeUl::UPrepare)?;

        let obit = delimiters::read_obit(buffer)?;
        let cell_identifier_ca = typed::parse_type2_generic(
            obit,
            buffer,
            5,
            "cell_identifier_ca",
        )?
        .map(|value| value as u8);

        Ok(Self {
            cell_identifier_ca,
            sdu: read_trailing_sdu(buffer),
        })
    }

    // Was: Wandelt den vorhandenen Wert in bitbuf um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_bitbuf(&self, buffer: &mut BitBuffer) -> Result<(), PduParseErr> {
        if let Some(value) = self.cell_identifier_ca
            && value > 0b1_1111
        {
            return Err(PduParseErr::InvalidValue {
                field: "cell_identifier_ca",
                value: value as u64,
            });
        }

        buffer.write_bits(MlePduTypeUl::UPrepare.into_raw(), 3);
        let obit = self.cell_identifier_ca.is_some();
        delimiters::write_obit(buffer, obit as u8);
        typed::write_type2_generic(
            obit,
            buffer,
            self.cell_identifier_ca.map(u64::from),
            5,
        );
        write_trailing_sdu(buffer, &self.sdu);
        Ok(())
    }
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Display for UPrepare`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for UPrepare {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "UPrepare {{ cell_identifier_ca: {:?}, sdu_bits: {} }}",
            self.cell_identifier_ca,
            self.sdu.as_ref().map_or(0, BitBuffer::get_len),
        )
    }
}
