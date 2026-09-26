// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use core::fmt;

use tetra_core::{BitBuffer, PduParseErr};
use tetra_saps::control::enums::{circuit_mode_type::CircuitModeType, communication_type::CommunicationType};

/// Clause 14.8.2 Basic service information
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für basic Dienst information in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct BasicServiceInformation {
    // 3
    pub circuit_mode_type: CircuitModeType,
    // 1, 0 = unencrypted, 1 = E2EE encrypted
    pub encryption_flag: bool,
    // 2, 0 = point-to-point, 1 = point-to-multipoint, 2 = point-to-multipoint acknowledged, 3 = broadcast
    pub communication_type: CommunicationType,
    /// 2 opt, 0 = 1 slot, 1 = 2 slots, 2 = 3 slots, 3 = 4 slots
    pub slots_per_frame: Option<u8>,
    /// 2 opt, 00 = TETRA encoded speech, 1|2 = reserved, 3 = proprietary
    pub speech_service: Option<u8>,
}

// Was: Implementiert das zugehörige Verhalten für `BasicServiceInformation`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl BasicServiceInformation {
    // Was: Wandelt Eingangsdaten in bitbuf um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_bitbuf(buffer: &mut BitBuffer) -> Result<Self, PduParseErr> {
        let val = buffer.read_field(3, "circuit_mode_type")?;
        let circuit_mode_type = CircuitModeType::try_from(val).unwrap(); // Never fails

        let encryption_flag = buffer.read_field(1, "encryption_flag")? != 0;
        let val = buffer.read_field(2, "communication_type")?;
        let communication_type = CommunicationType::try_from(val).unwrap(); // Never fails

        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        let (speech_service, slots_per_frame) = match circuit_mode_type {
            CircuitModeType::TchS => {
                let speech_service = buffer.read_field(2, "speech_service")? as u8;
                (Some(speech_service), None)
            }
            _ => {
                let slots_per_frame = buffer.read_field(2, "slots_per_frame")? as u8;
                (None, Some(slots_per_frame))
            }
        };

        let s = BasicServiceInformation {
            circuit_mode_type,
            encryption_flag,
            communication_type,
            slots_per_frame,
            speech_service,
        };

        Ok(s)
    }

    // Was: Wandelt den vorhandenen Wert in bitbuf um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_bitbuf(&self, buf: &mut BitBuffer) -> Result<(), PduParseErr> {
        buf.write_bits(self.circuit_mode_type as u64, 3);
        buf.write_bits(self.encryption_flag as u64, 1);
        buf.write_bits(self.communication_type as u64, 2);

        // Sanity check
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match (self.circuit_mode_type, self.slots_per_frame, self.speech_service) {
            (CircuitModeType::TchS, None, Some(_)) => Ok(()),
            (_, Some(_), None) => Ok(()),
            (_, _, _) => {
                return Err(PduParseErr::InvalidValue {
                    field: "circuit_mode_type",
                    value: self.circuit_mode_type as u64,
                });
            }
        }?;

        // TODO REVIEW: conditional write of slots_per_frame
        if let Some(v) = self.slots_per_frame {
            buf.write_bits(v as u64, 2);
        }
        // TODO REVIEW: conditional write of speech_service
        if let Some(v) = self.speech_service {
            buf.write_bits(v as u64, 2);
        }

        Ok(())
    }
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Display for BasicServiceInformation`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for BasicServiceInformation {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BasicServiceInformation {{ circuit_mode_type: {:?} encryption_flag: {:?} communication_type: {:?} slots_per_frame: {:?} speech_service: {:?} }}",
            self.circuit_mode_type, self.encryption_flag, self.communication_type, self.slots_per_frame, self.speech_service,
        )
    }
}
