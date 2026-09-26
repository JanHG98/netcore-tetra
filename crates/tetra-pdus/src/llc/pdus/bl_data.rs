// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use tetra_core::BitBuffer;
use tetra_core::pdu_parse_error::*;
use tetra_core::{expect_value, let_field};

/// Clause 21.2.2.3 BL-DATA
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für bl data in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct BlData {
    // 1
    pub has_fcs: bool,
    // 1
    pub ns: u8,
}

// Was: Implementiert das zugehörige Verhalten für `BlData`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl BlData {
    // Was: Wandelt Eingangsdaten in bitbuf um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_bitbuf(buf: &mut BitBuffer) -> Result<Self, PduParseErr> {
        // Parse 4-bit type, perform sanity checks
        let_field!(buf, llc_link_type, 1);
        expect_value!(llc_link_type, 0)?;
        let_field!(buf, has_fcs, 1);
        let_field!(buf, bl_pdu_type, 2);
        expect_value!(bl_pdu_type, 1)?;

        // Parse sequence number
        let_field!(buf, ns, 1);

        Ok(BlData {
            has_fcs: has_fcs != 0,
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
        buf.write_bits(1, 2);
        buf.write_bits(self.ns as u64, 1);
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for BlData`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for BlData {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "bl_data {{")?;
        write!(f, "  has_fcs: {}", self.has_fcs)?;
        write!(f, "  ns: {}", self.ns)?;
        write!(f, "}}")
    }
}
