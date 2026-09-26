// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use core::fmt;

use tetra_core::BitBuffer;
use tetra_core::pdu_parse_error::*;
use tetra_core::{expect_value, let_field};

/// Clause 21.2.2.4 BL-UDATA
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für bl udata in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct BlUdata {
    // 1
    pub has_fcs: bool,
}

// Was: Implementiert das zugehörige Verhalten für `BlUdata`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl BlUdata {
    // Was: Wandelt Eingangsdaten in bitbuf um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_bitbuf(buf: &mut BitBuffer) -> Result<Self, PduParseErr> {
        // Parse 4-bit type, perform sanity checks
        let_field!(buf, llc_link_type, 1);
        expect_value!(llc_link_type, 0)?;
        let_field!(buf, has_fcs, 1);
        let_field!(buf, bl_pdu_type, 2);
        expect_value!(bl_pdu_type, 2)?;

        Ok(BlUdata { has_fcs: has_fcs != 0 })
    }

    // Was: Wandelt den vorhandenen Wert in bitbuf um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_bitbuf(&self, buf: &mut BitBuffer) {
        // write required constant llc_link_type
        buf.write_bits(0, 1);
        buf.write_bits(self.has_fcs as u8 as u64, 1);
        // write required constant bl_pdu_type
        buf.write_bits(2, 2);
    }
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Display for BlUdata`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for BlUdata {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bl_udata {{ has_fcs: {} }}", self.has_fcs)
    }
}
