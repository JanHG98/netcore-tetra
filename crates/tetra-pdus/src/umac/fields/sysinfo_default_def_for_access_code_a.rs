// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use core::fmt::Display;

use tetra_core::{BitBuffer, pdu_parse_error::PduParseErr};

/// Clause 21.4.4.1 SYSINFO -> Default definition for access code A information element contents
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für sysinfo default def for access code a in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct SysinfoDefaultDefForAccessCodeA {
    // 4 0: always randomize, 0b1111: imm access allowed, other: randomize after n tdma frames
    pub imm: u8,
    // 4 Waiting time
    pub wt: u8,
    // 4 Number of random access transmissions on uplink
    pub nu: u8,
    // 1 if true, multiply base frame len by 4
    pub fl_factor: bool,
    // 4
    pub ts_ptr: u8,
    // 3
    pub min_pdu_prio: u8,
}

// Was: Implementiert das zugehörige Verhalten für `SysinfoDefaultDefForAccessCodeA`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl SysinfoDefaultDefForAccessCodeA {
    // Was: Wandelt Eingangsdaten in bitbuf um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_bitbuf(buf: &mut BitBuffer) -> Result<Self, PduParseErr> {
        let imm = buf.read_field(4, "imm")? as u8;
        let wt = buf.read_field(4, "wt")? as u8;
        let nu = buf.read_field(4, "nu")? as u8;
        let fl_factor = buf.read_field(1, "fl_factor")? != 0;
        let ts_ptr = buf.read_field(4, "ts_ptr")? as u8;
        let min_pdu_prio = buf.read_field(3, "min_pdu_prio")? as u8;

        Ok(SysinfoDefaultDefForAccessCodeA {
            imm,
            wt,
            nu,
            fl_factor,
            ts_ptr,
            min_pdu_prio,
        })
    }

    // Was: Wandelt den vorhandenen Wert in bitbuf um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_bitbuf(&self, buf: &mut BitBuffer) {
        buf.write_bits(self.imm as u64, 4);
        buf.write_bits(self.wt as u64, 4);
        buf.write_bits(self.nu as u64, 4);
        buf.write_bits(self.fl_factor as u8 as u64, 1);
        buf.write_bits(self.ts_ptr as u64, 4);
        buf.write_bits(self.min_pdu_prio as u64, 3);
    }
}

// Was: Implementiert das zugehörige Verhalten für `Display for SysinfoDefaultDefForAccessCodeA`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Display for SysinfoDefaultDefForAccessCodeA {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "SysinfoDefaultDefForAccessCodeA {{ imm: {}, wt: {}, nu: {}, fl_factor: {}, ts_ptr: {}, min_pdu_prio: {} }}",
            self.imm, self.wt, self.nu, self.fl_factor, self.ts_ptr, self.min_pdu_prio
        )
    }
}
