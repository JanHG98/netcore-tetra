// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use core::fmt;

use crate::cmce::enums::cmce_pdu_type_ul::CmcePduTypeUl;
use tetra_core::typed_pdu_fields::*;
use tetra_core::{BitBuffer, expect_pdu_type, pdu_parse_error::PduParseErr};

/// Representation of the U-FACILITY PDU (Clause 14.7.2.5).
/// This PDU shall be used to send call unrelated SS information.
/// Response expected: -
/// Response to: -

// note 1: Contents of this PDU shall be defined by SS protocols.
#[derive(Debug)]
// Was: Bündelt die zusammengehörigen Werte für ufacility in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct UFacility {}

#[allow(unreachable_code)] // TODO FIXME review, finalize and remove this
// Was: Implementiert das zugehörige Verhalten für `UFacility`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl UFacility {
    /// Parse from BitBuffer
    // Was: Wandelt Eingangsdaten in bitbuf um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_bitbuf(buffer: &mut BitBuffer) -> Result<Self, PduParseErr> {
        let pdu_type = buffer.read_field(5, "pdu_type")?;
        expect_pdu_type!(pdu_type, CmcePduTypeUl::UFacility)?;

        // obit designates presence of any further type2, type3 or type4 fields
        let mut obit = delimiters::read_obit(buffer)?;

        // Read trailing obit (if not previously encountered)
        obit = if obit { buffer.read_field(1, "trailing_obit")? == 1 } else { obit };
        if obit {
            return Err(PduParseErr::InvalidTrailingMbitValue);
        }

        Ok(UFacility {})
    }

    /// Serialize this PDU into the given BitBuffer.
    // Was: Wandelt den vorhandenen Wert in bitbuf um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_bitbuf(&self, buffer: &mut BitBuffer) -> Result<(), PduParseErr> {
        // PDU Type
        buffer.write_bits(CmcePduTypeUl::UFacility.into_raw(), 5);
        // Write terminating m-bit
        delimiters::write_mbit(buffer, 0);
        Ok(())
    }
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Display for UFacility`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for UFacility {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UFacility {{ }}",)
    }
}
