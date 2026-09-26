// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use core::fmt;

use tetra_core::typed_pdu_fields::{delimiters, typed};
use tetra_core::{BitBuffer, expect_pdu_type, pdu_parse_error::PduParseErr};
use tetra_saps::common::{
    MleChannelRequestReason, MleChannelRequestRetryDelay, MleChannelResponseType,
};

use crate::mle::enums::mle_pdu_type_dl::MlePduTypeDl;

/// D-CHANNEL-RESPONSE PDU (ETSI EN 300 392-2, clause 18.4.1.4.5a).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für dchannel response in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct DChannelResponse {
    pub channel_response_type: MleChannelResponseType,
    pub reason_for_the_channel_request: MleChannelRequestReason,
    pub channel_request_retry_delay: MleChannelRequestRetryDelay,
    /// Reserved Type-2 fields. They are retained for tolerant decoding but
    /// shall normally be `None` in this implementation.
    pub reserved1: Option<u8>,
    pub reserved2: Option<u8>,
}

// Was: Implementiert das zugehörige Verhalten für `DChannelResponse`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl DChannelResponse {
    // Was: Wandelt Eingangsdaten in bitbuf um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_bitbuf(buffer: &mut BitBuffer) -> Result<Self, PduParseErr> {
        let pdu_type = buffer.read_field(3, "pdu_type")?;
        expect_pdu_type!(pdu_type, MlePduTypeDl::DChannelResponse)?;

        let channel_response_type = MleChannelResponseType::from_raw(
            buffer.read_field(1, "channel_response_type")? as u8,
        );
        let reason_for_the_channel_request = MleChannelRequestReason::from_raw(
            buffer.read_field(3, "reason_for_the_channel_request")? as u8,
        );
        let channel_request_retry_delay = MleChannelRequestRetryDelay::from_raw(
            buffer.read_field(4, "channel_request_retry_delay")? as u8,
        );

        let obit = delimiters::read_obit(buffer)?;
        let reserved1 = typed::parse_type2_generic(obit, buffer, 8, "reserved1")?.map(|v| v as u8);
        let reserved2 = typed::parse_type2_generic(obit, buffer, 8, "reserved2")?.map(|v| v as u8);
        if obit && delimiters::read_mbit(buffer)? {
            return Err(PduParseErr::InvalidTrailingMbitValue);
        }

        Ok(Self {
            channel_response_type,
            reason_for_the_channel_request,
            channel_request_retry_delay,
            reserved1,
            reserved2,
        })
    }

    // Was: Wandelt den vorhandenen Wert in bitbuf um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_bitbuf(&self, buffer: &mut BitBuffer) -> Result<(), PduParseErr> {
        buffer.write_bits(MlePduTypeDl::DChannelResponse.into_raw(), 3);
        buffer.write_bits(self.channel_response_type.into_raw() as u64, 1);
        buffer.write_bits(self.reason_for_the_channel_request.into_raw() as u64, 3);
        buffer.write_bits(self.channel_request_retry_delay.into_raw() as u64, 4);

        let obit = self.reserved1.is_some() || self.reserved2.is_some();
        delimiters::write_obit(buffer, obit as u8);
        typed::write_type2_generic(obit, buffer, self.reserved1.map(u64::from), 8);
        typed::write_type2_generic(obit, buffer, self.reserved2.map(u64::from), 8);
        if obit {
            delimiters::write_mbit(buffer, 0);
        }
        Ok(())
    }
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Display for DChannelResponse`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for DChannelResponse {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DChannelResponse {{ response: {:?}, reason: {:?}, retry: {:?} }}",
            self.channel_response_type,
            self.reason_for_the_channel_request,
            self.channel_request_retry_delay,
        )
    }
}
