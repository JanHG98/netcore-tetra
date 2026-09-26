// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use core::fmt;

use tetra_core::BitBuffer;
use tetra_core::pdu_parse_error::PduParseErr;

/// Clause 21.4.2.5 MAC-U-BLCK
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für MAC-Funkzugriffssteuerung ublck in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MacUBlck {
    // 1
    pub fill_bits: bool,
    // 1
    pub encrypted: bool,
    // 10
    pub event_label: u16,
    // 4
    pub reservation_req: u8, // WARNING don't use the regular ReservationRequirement enum, as there is a caveat in the highest two values
}

// Was: Implementiert das zugehörige Verhalten für `MacUBlck`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MacUBlck {
    // Was: Wandelt Eingangsdaten in bitbuf um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_bitbuf(buf: &mut BitBuffer) -> Result<Self, PduParseErr> {
        // required constant mac_pdu_type
        let mac_pdu_type = buf.read_field(2, "mac_pdu_type")?;
        assert!(mac_pdu_type == 3);
        // required constant supp_pdu_subtype
        let supp_pdu_subtype = buf.read_field(1, "supp_pdu_subtype")?;
        assert!(supp_pdu_subtype == 0);
        let fill_bits = buf.read_field(1, "fill_bits")? != 0;
        let encrypted = buf.read_field(1, "encrypted")? != 0;
        let event_label = buf.read_field(10, "event_label")? as u16;
        let reservation_req = buf.read_field(4, "reservation_req")? as u8;

        Ok(MacUBlck {
            fill_bits,
            encrypted,
            event_label,
            reservation_req,
        })
    }

    // Was: Wandelt den vorhandenen Wert in bitbuf um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_bitbuf(&self, buf: &mut BitBuffer) {
        // write required constant mac_pdu_type
        buf.write_bits(3, 2);
        // write required constant supp_pdu_subtype
        buf.write_bits(0, 1);
        buf.write_bits(self.fill_bits as u8 as u64, 1);
        buf.write_bits(self.encrypted as u8 as u64, 1);
        buf.write_bits(self.event_label as u64, 10);
        buf.write_bits(self.reservation_req as u64, 4);
    }
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Display for MacUBlck`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for MacUBlck {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MacUBlck {{ fill_bits: {}", self.fill_bits)?;
        write!(f, "  encrypted: {}", self.encrypted)?;
        write!(f, "  addr: {}", self.event_label)?;
        write!(f, "  reservation_req: {}", self.reservation_req)?;
        write!(f, " }}")
    }
}
