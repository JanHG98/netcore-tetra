// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use core::fmt;

use tetra_core::BitBuffer;
use tetra_core::pdu_parse_error::PduParseErr;

/// Clause 21.4.2.4 MAC-FRAG (uplink)
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für MAC-Funkzugriffssteuerung frag ul in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MacFragUl {
    // 1
    pub fill_bits: bool,
}

// Was: Implementiert das zugehörige Verhalten für `MacFragUl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MacFragUl {
    // Was: Wandelt Eingangsdaten in bitbuf um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_bitbuf(buf: &mut BitBuffer) -> Result<Self, PduParseErr> {
        // required constant mac_pdu_type
        let mac_pdu_type = buf.read_field(2, "mac_pdu_type")?;
        assert!(mac_pdu_type == 1);
        // required constant pdu_subtype
        let pdu_subtype = buf.read_field(1, "pdu_subtype")?;
        assert!(pdu_subtype == 0);
        let fill_bits = buf.read_field(1, "fill_bits")? != 0;

        Ok(MacFragUl { fill_bits })
    }

    // Was: Wandelt den vorhandenen Wert in bitbuf um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_bitbuf(&self, buf: &mut BitBuffer) {
        // write required constant mac_pdu_type
        buf.write_bits(1, 2);
        // write required constant pdu_subtype
        buf.write_bits(0, 1);
        buf.write_bits(self.fill_bits as u8 as u64, 1);
    }
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Display for MacFragUl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for MacFragUl {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MacFragUl {{ fill_bits: {} }}", self.fill_bits)
    }
}
