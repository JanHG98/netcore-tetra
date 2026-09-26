// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use core::fmt;

use tetra_core::{BitBuffer, pdu_parse_error::PduParseErr};

/// 16.10.19 Group Identity Attachment
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für Gruppe identity attachment in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct GroupIdentityAttachment {
    /// 2 bits.
    /// 0: Attachment not needed
    /// 1: Attachment for next ITSI attach required
    /// 2: Attachment not allowed for next ITSI attach
    /// 3: Attachment for next location update required (good default)
    pub group_identity_attachment_lifetime: u8,
    /// 3 bits
    pub class_of_usage: u8,
}

// Was: Implementiert das zugehörige Verhalten für `GroupIdentityAttachment`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl GroupIdentityAttachment {
    // Was: Wandelt Eingangsdaten in bitbuf um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_bitbuf(buf: &mut BitBuffer) -> Result<Self, PduParseErr> {
        let mut s = GroupIdentityAttachment {
            group_identity_attachment_lifetime: 0,
            class_of_usage: 0,
        };

        s.group_identity_attachment_lifetime = buf.read_field(2, "group_identity_attachment_lifetime")? as u8;
        s.class_of_usage = buf.read_field(3, "class_of_usage")? as u8;

        Ok(s)
    }

    // Was: Wandelt den vorhandenen Wert in bitbuf um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_bitbuf(&self, buf: &mut BitBuffer) {
        buf.write_bits(self.group_identity_attachment_lifetime as u64, 2);
        buf.write_bits(self.class_of_usage as u64, 3);
    }
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Display for GroupIdentityAttachment`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for GroupIdentityAttachment {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "group_identity_attachment {{ group_identity_attachment_lifetime: {} class_of_usage: {} }}",
            self.group_identity_attachment_lifetime, self.class_of_usage
        )
    }
}
