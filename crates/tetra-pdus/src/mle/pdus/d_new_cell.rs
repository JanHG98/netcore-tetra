// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use core::fmt;

use tetra_core::typed_pdu_fields::delimiters;
use tetra_core::{BitBuffer, expect_pdu_type, pdu_parse_error::PduParseErr};
use tetra_saps::common::MleChannelCommandValid;

use crate::mle::enums::mle_pdu_type_dl::MlePduTypeDl;
use crate::mle::pdus::trailing_sdu::{read_trailing_sdu, write_trailing_sdu};

/// D-NEW-CELL PDU (ETSI EN 300 392-2, clause 18.4.1.4.2).
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für dnew cell in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct DNewCell {
    pub channel_command_valid: MleChannelCommandValid,
    /// Optional embedded MM/OTAR PDU. It occupies all bits remaining in the
    /// enclosing LLC SDU and has no P-bit of its own.
    pub sdu: Option<BitBuffer>,
}

// Was: Implementiert das zugehörige Verhalten für `DNewCell`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl DNewCell {
    // Was: Wandelt Eingangsdaten in bitbuf um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_bitbuf(buffer: &mut BitBuffer) -> Result<Self, PduParseErr> {
        let pdu_type = buffer.read_field(3, "pdu_type")?;
        expect_pdu_type!(pdu_type, MlePduTypeDl::DNewCell)?;

        let channel_command_valid = MleChannelCommandValid::from_raw(
            buffer.read_field(2, "channel_command_valid")? as u8,
        );

        // Annex E requires O-bit=0. The embedded SDU follows directly.
        let obit = delimiters::read_obit(buffer)?;
        if obit {
            return Err(PduParseErr::InvalidValue {
                field: "d_new_cell_obit",
                value: 1,
            });
        }

        Ok(Self {
            channel_command_valid,
            sdu: read_trailing_sdu(buffer),
        })
    }

    // Was: Wandelt den vorhandenen Wert in bitbuf um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_bitbuf(&self, buffer: &mut BitBuffer) -> Result<(), PduParseErr> {
        buffer.write_bits(MlePduTypeDl::DNewCell.into_raw(), 3);
        buffer.write_bits(self.channel_command_valid.into_raw() as u64, 2);
        delimiters::write_obit(buffer, 0);
        write_trailing_sdu(buffer, &self.sdu);
        Ok(())
    }
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Display for DNewCell`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for DNewCell {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DNewCell {{ channel_command_valid: {:?}, sdu_bits: {} }}",
            self.channel_command_valid,
            self.sdu.as_ref().map_or(0, BitBuffer::get_len),
        )
    }
}
