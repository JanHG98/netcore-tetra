// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use core::fmt;

use crate::mm::{enums::type34_elem_id_dl::MmType34ElemIdDl, fields::group_identity_downlink::GroupIdentityDownlink};
use tetra_core::typed_pdu_fields::{delimiters, typed};
use tetra_core::{BitBuffer, pdu_parse_error::PduParseErr};

/// Representation of the Group identity location accept PDU (Clause 16.10.23).
/// The group identity location accept information element shall be a collection of sub elements.
#[derive(Debug)]
// Was: Bündelt die zusammengehörigen Werte für Gruppe identity location accept in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct GroupIdentityLocationAccept {
    /// Type1, 1 bit. 0 = accept, 1 = reject
    pub group_identity_accept_reject: u8,
    /// Type1, 1 bits, reserved
    // pub reserved: bool,
    /// Type4, Group identity downlink
    pub group_identity_downlink: Option<Vec<GroupIdentityDownlink>>,
}

#[allow(unreachable_code)] // TODO FIXME review, finalize and remove this
// Was: Implementiert das zugehörige Verhalten für `GroupIdentityLocationAccept`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl GroupIdentityLocationAccept {
    /// Parse from BitBuffer
    // Was: Wandelt Eingangsdaten in bitbuf um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_bitbuf(buffer: &mut BitBuffer) -> Result<Self, PduParseErr> {
        // Type1
        let group_identity_accept_reject = buffer.read_field(1, "group_identity_accept_reject")? as u8;

        // Type1
        let _reserved = buffer.read_field(1, "reserved")? != 0;

        // obit designates presence of any further type2, type3 or type4 fields
        let mut obit = delimiters::read_obit(buffer)?;

        // Type4
        let group_identity_downlink = typed::parse_type4_struct(
            obit,
            buffer,
            MmType34ElemIdDl::GroupIdentityDownlink,
            GroupIdentityDownlink::from_bitbuf,
        )?;

        // Read trailing mbit (if not previously encountered)
        obit = if obit { buffer.read_field(1, "trailing_obit")? == 1 } else { obit };
        if obit {
            return Err(PduParseErr::InvalidTrailingMbitValue);
        }
        Ok(GroupIdentityLocationAccept {
            group_identity_accept_reject,
            // reserved: reserved,
            group_identity_downlink,
        })
    }

    /// Serialize this PDU into the given BitBuffer.
    // Was: Wandelt den vorhandenen Wert in bitbuf um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_bitbuf(&self, buffer: &mut BitBuffer) -> Result<(), PduParseErr> {
        // Type1
        buffer.write_bits(self.group_identity_accept_reject as u64, 1);
        // Type1, reserved
        buffer.write_bits(0, 1);

        // Check if any optional field present and place o-bit
        let obit = self.group_identity_downlink.is_some();
        delimiters::write_obit(buffer, obit as u8);
        if !obit {
            return Ok(());
        }

        // Type4
        typed::write_type4_struct(
            obit,
            buffer,
            &self.group_identity_downlink,
            MmType34ElemIdDl::GroupIdentityDownlink,
            GroupIdentityDownlink::to_bitbuf,
        )?;

        // Write terminating m-bit
        delimiters::write_mbit(buffer, 0);
        Ok(())
    }
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Display for GroupIdentityLocationAccept`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for GroupIdentityLocationAccept {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "GroupIdentityLocationAccept {{ group_identity_accept_reject: {:?} group_identity_downlink: {:?} }}",
            self.group_identity_accept_reject,
            // self.reserved,
            self.group_identity_downlink,
        )
    }
}
