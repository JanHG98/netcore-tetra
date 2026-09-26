// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use core::fmt;

use crate::cmce::enums::disconnect_cause::DisconnectCause;
use crate::cmce::enums::{cmce_pdu_type_dl::CmcePduTypeDl, type3_elem_id::CmceType3ElemId};
use tetra_core::typed_pdu_fields::*;
use tetra_core::{BitBuffer, expect_pdu_type, pdu_parse_error::PduParseErr};

/// Representation of the D-DISCONNECT PDU (Clause 14.7.1.6).
/// This PDU shall be the disconnect request message sent from the infrastructure to the MS.
/// Response expected: U-RELEASE
/// Response to: -

#[derive(Debug)]
// Was: Bündelt die zusammengehörigen Werte für ddisconnect in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct DDisconnect {
    /// Type1, 14 bits, Call identifier
    pub call_identifier: u16,
    /// Type1, 5 bits, Disconnect cause
    pub disconnect_cause: DisconnectCause,
    /// Type2, 6 bits, Notification indicator
    pub notification_indicator: Option<u64>,
    /// Type3, Facility
    pub facility: Option<Type3FieldGeneric>,
    /// Type3, Proprietary
    pub proprietary: Option<Type3FieldGeneric>,
}

#[allow(unreachable_code)] // TODO FIXME review, finalize and remove this
// Was: Implementiert das zugehörige Verhalten für `DDisconnect`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl DDisconnect {
    /// Parse from BitBuffer
    // Was: Wandelt Eingangsdaten in bitbuf um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_bitbuf(buffer: &mut BitBuffer) -> Result<Self, PduParseErr> {
        let pdu_type = buffer.read_field(5, "pdu_type")?;
        expect_pdu_type!(pdu_type, CmcePduTypeDl::DDisconnect)?;
        // Type1
        let call_identifier = buffer.read_field(14, "call_identifier")? as u16;

        let val = buffer.read_field(5, "disconnect_cause")?;
        let disconnect_cause = DisconnectCause::try_from(val).map_err(|_| PduParseErr::InvalidValue {
            field: "disconnect_cause",
            value: val,
        })?;

        // obit designates presence of any further type2, type3 or type4 fields
        let mut obit = delimiters::read_obit(buffer)?;

        // Type2
        let notification_indicator = typed::parse_type2_generic(obit, buffer, 6, "notification_indicator")?;

        // Type3
        let facility = typed::parse_type3_generic(obit, buffer, CmceType3ElemId::Facility)?;

        // Type3
        let proprietary = typed::parse_type3_generic(obit, buffer, CmceType3ElemId::Proprietary)?;

        // Read trailing mbit (if not previously encountered)
        obit = if obit { buffer.read_field(1, "trailing_obit")? == 1 } else { obit };
        if obit {
            return Err(PduParseErr::InvalidTrailingMbitValue);
        }

        Ok(DDisconnect {
            call_identifier,
            disconnect_cause,
            notification_indicator,
            facility,
            proprietary,
        })
    }

    /// Serialize this PDU into the given BitBuffer.
    // Was: Wandelt den vorhandenen Wert in bitbuf um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_bitbuf(&self, buffer: &mut BitBuffer) -> Result<(), PduParseErr> {
        // PDU Type
        buffer.write_bits(CmcePduTypeDl::DDisconnect.into_raw(), 5);
        // Type1
        buffer.write_bits(self.call_identifier as u64, 14);
        // Type1
        buffer.write_bits(self.disconnect_cause as u64, 5);

        // Check if any optional field present and place o-bit
        let obit = self.notification_indicator.is_some() || self.facility.is_some() || self.proprietary.is_some();
        delimiters::write_obit(buffer, obit as u8);
        if !obit {
            return Ok(());
        }

        // Type2
        typed::write_type2_generic(obit, buffer, self.notification_indicator, 6);

        // Type3
        typed::write_type3_generic(obit, buffer, &self.facility, CmceType3ElemId::Facility)?;

        // Type3
        typed::write_type3_generic(obit, buffer, &self.proprietary, CmceType3ElemId::Proprietary)?;

        // Write terminating m-bit
        delimiters::write_mbit(buffer, 0);
        Ok(())
    }
}
// Was: Implementiert das zugehörige Verhalten für `fmt::Display for DDisconnect`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for DDisconnect {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DDisconnect {{ call_identifier: {:?} disconnect_cause: {:?} notification_indicator: {:?} facility: {:?} proprietary: {:?} }}",
            self.call_identifier, self.disconnect_cause, self.notification_indicator, self.facility, self.proprietary,
        )
    }
}
