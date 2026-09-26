// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use core::fmt;

use tetra_core::BitBuffer;
use tetra_core::pdu_parse_error::*;
use tetra_core::{expect_value, let_field};

/// Clause 21.2.2.2 BL-ADATA
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für bl adata in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct BlAdata {
    // 1
    pub has_fcs: bool,
    // 1
    pub nr: u8,
    // 1
    pub ns: u8,
}

// Was: Implementiert das zugehörige Verhalten für `BlAdata`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl BlAdata {
    // Was: Wandelt Eingangsdaten in bitbuf um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_bitbuf(buf: &mut BitBuffer) -> Result<Self, PduParseErr> {
        // Parse 4-bit type, perform sanity checks
        let_field!(buf, llc_link_type, 1);
        expect_value!(llc_link_type, 0)?;
        let_field!(buf, has_fcs, 1);
        let_field!(buf, bl_pdu_type, 2);
        expect_value!(bl_pdu_type, 0)?;

        // Parse rx/tx 1-bit sequence numbers
        let_field!(buf, nr, 1);
        let_field!(buf, ns, 1);

        Ok(BlAdata {
            has_fcs: has_fcs != 0,
            nr: nr as u8,
            ns: ns as u8,
        })
    }

    // Was: Wandelt den vorhandenen Wert in bitbuf um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_bitbuf(&self, buf: &mut BitBuffer) {
        // write required constant llc_link_type
        buf.write_bits(0, 1);
        buf.write_bits(self.has_fcs as u8 as u64, 1);
        // write required constant bl_pdu_type
        buf.write_bits(0, 2);
        buf.write_bits(self.nr as u64, 1);
        buf.write_bits(self.ns as u64, 1);
    }
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Display for BlAdata`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for BlAdata {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bl_adata {{")?;
        write!(f, "  has_fcs: {}", self.has_fcs)?;
        write!(f, "  nr: {}", self.nr)?;
        write!(f, "  ns: {}", self.ns)?;
        write!(f, "}}")
    }
}
