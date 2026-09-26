// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use core::fmt;

use tetra_core::BitBuffer;
use tetra_core::pdu_parse_error::PduParseErr;

/// Clause 21.4.5 MAC-U-SIGNAL
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für MAC-Funkzugriffssteuerung usignal in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MacUSignal {
    // 1
    pub second_half_stolen: bool,
}

// Was: Implementiert das zugehörige Verhalten für `MacUSignal`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MacUSignal {
    // Was: Wandelt Eingangsdaten in bitbuf um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_bitbuf(buf: &mut BitBuffer) -> Result<Self, PduParseErr> {
        // required constant mac_pdu_type
        let mac_pdu_type = buf.read_field(2, "mac_pdu_type")?;
        assert!(mac_pdu_type == 3);
        let second_half_stolen = buf.read_field(1, "second_half_stolen")? != 0;

        Ok(MacUSignal { second_half_stolen })
    }

    // Was: Wandelt den vorhandenen Wert in bitbuf um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_bitbuf(&self, buf: &mut BitBuffer) {
        // write required constant mac_pdu_type
        buf.write_bits(3, 2);
        buf.write_bits(self.second_half_stolen as u8 as u64, 1);
    }
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Display for MacUSignal`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for MacUSignal {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "mac_u_signal {{\n  second_half_stolen: {}\n}}\n", self.second_half_stolen,)
    }
}
