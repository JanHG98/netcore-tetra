// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use core::fmt;

use tetra_core::{BitBuffer, pdu_parse_error::PduParseErr};

use crate::umac::enums::{basic_slotgrant_cap_alloc::BasicSlotgrantCapAlloc, basic_slotgrant_granting_delay::BasicSlotgrantGrantingDelay};

/// 21.5.6 Basic slot granting
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für basic slotgrant in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct BasicSlotgrant {
    // 4
    pub capacity_allocation: BasicSlotgrantCapAlloc,
    // 4
    pub granting_delay: BasicSlotgrantGrantingDelay,
}

// Was: Implementiert das zugehörige Verhalten für `BasicSlotgrant`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl BasicSlotgrant {
    // Was: Wandelt Eingangsdaten in bitbuf um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_bitbuf(buf: &mut BitBuffer) -> Result<Self, PduParseErr> {
        let cap_alloc_val = buf.read_field(4, "capacity_allocation")?;
        let capacity_allocation = BasicSlotgrantCapAlloc::try_from(cap_alloc_val).map_err(|_| PduParseErr::InvalidValue {
            field: "capacity_allocation",
            value: cap_alloc_val,
        })?;

        let granting_delay_val = buf.read_field(4, "granting_delay")?;
        let granting_delay = BasicSlotgrantGrantingDelay::try_from(granting_delay_val).map_err(|_| PduParseErr::InvalidValue {
            field: "granting_delay",
            value: granting_delay_val,
        })?;

        Ok(BasicSlotgrant {
            capacity_allocation,
            granting_delay,
        })
    }

    // Was: Wandelt den vorhandenen Wert in bitbuf um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_bitbuf(&self, buf: &mut BitBuffer) {
        buf.write_bits(self.capacity_allocation as u64, 4);
        buf.write_bits(self.granting_delay.into_raw(), 4);
    }
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Display for BasicSlotgrant`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for BasicSlotgrant {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BasicSlotgrant {{cap {} delay {} }}",
            self.capacity_allocation, self.granting_delay
        )
    }
}
