// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use core::fmt;

use tetra_core::expect_pdu_type;
use tetra_core::typed_pdu_fields::*;
use tetra_core::{BitBuffer, pdu_parse_error::PduParseErr};

use crate::mm::enums::mm_pdu_type_ul::MmPduTypeUl;
use crate::mm::enums::type34_elem_id_ul::MmType34ElemIdUl;

/// Representation of the U-TEI PROVIDE PDU (Clause 16.9.3.12).
/// The MS sends this message to the infrastructure to provide its Terminal Equipment Identity (TEI).
/// This is sent in response to a TEI request from the infrastructure, or spontaneously during registration.
/// TEI is a 60-bit hardware identifier unique to each physical terminal (analogous to IMEI in GSM).
/// Response to: D-LOCATION UPDATE COMMAND (implicit TEI request)
#[derive(Debug)]
// Was: Bündelt die zusammengehörigen Werte für utei provide in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct UTeiProvide {
    /// Mandatory, 60 bits: Terminal Equipment Identity (hardware identifier)
    pub tei: u64,
    /// Type3, Proprietary
    pub proprietary: Option<Type3FieldGeneric>,
}

// Was: Implementiert das zugehörige Verhalten für `UTeiProvide`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl UTeiProvide {
    /// Parse from BitBuffer
    // Was: Wandelt Eingangsdaten in bitbuf um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_bitbuf(buffer: &mut BitBuffer) -> Result<Self, PduParseErr> {
        let pdu_type = buffer.read_field(4, "pdu_type")?;
        expect_pdu_type!(pdu_type, MmPduTypeUl::UTeiProvide)?;

        // Mandatory: TEI is 60 bits (ETSI Table 16.83)
        let tei = buffer.read_field(60, "tei")?;

        // o-bit designates presence of any further type3 fields
        let mut obit = delimiters::read_obit(buffer)?;

        // Type3
        let proprietary = typed::parse_type3_generic(obit, buffer, MmType34ElemIdUl::Proprietary)?;

        // Read trailing mbit (if not previously encountered)
        obit = if obit { buffer.read_field(1, "trailing_obit")? == 1 } else { obit };
        if obit {
            return Err(PduParseErr::InvalidTrailingMbitValue);
        }

        Ok(UTeiProvide { tei, proprietary })
    }

    /// Serialize this PDU into the given BitBuffer.
    // Was: Wandelt den vorhandenen Wert in bitbuf um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_bitbuf(&self, buffer: &mut BitBuffer) -> Result<(), PduParseErr> {
        // PDU Type
        buffer.write_bits(MmPduTypeUl::UTeiProvide.into_raw(), 4);

        // Mandatory: TEI 60 bits
        buffer.write_bits(self.tei, 60);

        // Check if any optional field present and place o-bit
        let obit = self.proprietary.is_some();
        delimiters::write_obit(buffer, obit as u8);
        if !obit {
            return Ok(());
        }

        // Type3
        typed::write_type3_generic(obit, buffer, &self.proprietary, MmType34ElemIdUl::Proprietary)?;

        // Write terminating m-bit
        delimiters::write_mbit(buffer, 0);
        Ok(())
    }

    /// Format TEI as a hex string for display (e.g. "0x1A2B3C4D5E6F")
    // Was: Führt den Arbeitsschritt `tei_hex` für tei hex aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn tei_hex(&self) -> String {
        format!("0x{:015X}", self.tei)
    }
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Display for UTeiProvide`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for UTeiProvide {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "UTeiProvide {{ tei: {} ({:060b}) proprietary: {:?} }}",
            self.tei_hex(),
            self.tei,
            self.proprietary,
        )
    }
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use tetra_core::debug;

    use super::*;

    /// Build a minimal U-TEI-PROVIDE bitstring manually and verify round-trip parsing.
    /// PDU type (4 bits) = 1001 (9), TEI (60 bits), o-bit = 0
    #[test]
    // Was: Prüft automatisch den Fall u tei provide minimal.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_u_tei_provide_minimal() {
        debug::setup_logging_verbose();

        // Construct a known PDU in a BitBuffer and parse it
        let mut buf_out = BitBuffer::new_autoexpand(8);
        let pdu = UTeiProvide {
            tei: 0x123456789ABCDE, // 60-bit value
            proprietary: None,
        };
        pdu.to_bitbuf(&mut buf_out).unwrap();
        buf_out.seek(0);

        tracing::info!("Serialized: {}", buf_out.dump_bin());

        let pdu2 = UTeiProvide::from_bitbuf(&mut buf_out).expect("Failed parsing");
        assert_eq!(pdu2.tei, pdu.tei);
        assert!(pdu2.proprietary.is_none());
        tracing::info!("Parsed: {}", pdu2);
    }
}
