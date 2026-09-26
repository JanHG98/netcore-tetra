// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Command codec — bitcode-based and JSON-based serialization of
//! [`Command`]s and [`CommandResponse`]s.

use crate::{
    net_control::commands::{ControlCommand, ControlResponse},
    network::transports::NetworkError,
};

// ---------------------------------------------------------------------------
// Codecs
// ---------------------------------------------------------------------------

/// Codec for commands using bitcode for serialization.
#[derive(Default)]
// Was: Bündelt die zusammengehörigen Werte für Steuerung codec bitcode in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ControlCodecBitcode;

// Was: Implementiert das zugehörige Verhalten für `ControlCodecBitcode`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl ControlCodecBitcode {
    /// Encode a [`Command`] to bitcode bytes.
    // Was: Diese Funktion kodiert command.
    // Warum: Alle Gegenstellen erhalten dadurch dasselbe erwartete Protokollformat.
    pub fn encode_command(&self, cmd: &ControlCommand) -> Vec<u8> {
        bitcode::encode(cmd)
    }

    /// Decode bitcode bytes into a [`Command`].
    // Was: Diese Funktion dekodiert command.
    // Warum: Empfangene Protokolldaten müssen vor der weiteren Nutzung eindeutig verstanden und geprüft werden.
    pub fn decode_command(&self, payload: &[u8]) -> Result<ControlCommand, NetworkError> {
        bitcode::decode(payload).map_err(|e| NetworkError::SerializationError(format!("command decode: {}", e)))
    }

    /// Encode a [`CommandResponse`] to bitcode bytes.
    // Was: Diese Funktion kodiert response.
    // Warum: Alle Gegenstellen erhalten dadurch dasselbe erwartete Protokollformat.
    pub fn encode_response(&self, resp: &ControlResponse) -> Vec<u8> {
        bitcode::encode(resp)
    }

    /// Decode bitcode bytes into a [`CommandResponse`].
    // Was: Diese Funktion dekodiert response.
    // Warum: Empfangene Protokolldaten müssen vor der weiteren Nutzung eindeutig verstanden und geprüft werden.
    pub fn decode_response(&self, payload: &[u8]) -> Result<ControlResponse, NetworkError> {
        bitcode::decode(payload).map_err(|e| NetworkError::SerializationError(format!("command response decode: {}", e)))
    }
}

/// Codec for commands using JSON for serialization.
#[derive(Default)]
// Was: Bündelt die zusammengehörigen Werte für Steuerung codec JSON-Daten in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ControlCodecJson;

// Was: Implementiert das zugehörige Verhalten für `ControlCodecJson`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl ControlCodecJson {
    /// Encode a [`Command`] to JSON bytes.
    // Was: Diese Funktion kodiert command.
    // Warum: Alle Gegenstellen erhalten dadurch dasselbe erwartete Protokollformat.
    pub fn encode_command(&self, cmd: &ControlCommand) -> Vec<u8> {
        serde_json::to_vec(cmd).unwrap_or_default()
    }

    /// Decode JSON bytes into a [`Command`].
    // Was: Diese Funktion dekodiert command.
    // Warum: Empfangene Protokolldaten müssen vor der weiteren Nutzung eindeutig verstanden und geprüft werden.
    pub fn decode_command(&self, payload: &[u8]) -> Result<ControlCommand, NetworkError> {
        serde_json::from_slice(payload).map_err(|e| NetworkError::SerializationError(format!("command decode: {}", e)))
    }

    /// Encode a [`CommandResponse`] to JSON bytes.
    // Was: Diese Funktion kodiert response.
    // Warum: Alle Gegenstellen erhalten dadurch dasselbe erwartete Protokollformat.
    pub fn encode_response(&self, resp: &ControlResponse) -> Vec<u8> {
        serde_json::to_vec(resp).unwrap_or_default()
    }

    /// Decode JSON bytes into a [`CommandResponse`].
    // Was: Diese Funktion dekodiert response.
    // Warum: Empfangene Protokolldaten müssen vor der weiteren Nutzung eindeutig verstanden und geprüft werden.
    pub fn decode_response(&self, payload: &[u8]) -> Result<ControlResponse, NetworkError> {
        serde_json::from_slice(payload).map_err(|e| NetworkError::SerializationError(format!("command response decode: {}", e)))
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Prüft automatisch den Fall roundtrip bitcode command a.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_roundtrip_bitcode_command_a() {
        let codec = ControlCodecBitcode;
        let cmd = ControlCommand::CommandA {
            handle: 1,
            parameter: 1234,
        };
        let bytes = codec.encode_command(&cmd);
        let decoded = codec.decode_command(&bytes).unwrap();
        let ControlCommand::CommandA { handle, parameter } = decoded else {
            panic!("expected CommandA");
        };
        assert_eq!(handle, 1);
        assert_eq!(parameter, 1234);
    }

    #[test]
    // Was: Prüft automatisch den Fall roundtrip JSON-Daten command a.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_roundtrip_json_command_a() {
        let codec = ControlCodecJson;
        let cmd = ControlCommand::CommandA {
            handle: 1,
            parameter: 1234,
        };
        let bytes = codec.encode_command(&cmd);
        let decoded = codec.decode_command(&bytes).unwrap();
        let ControlCommand::CommandA { handle, parameter } = decoded else {
            panic!("expected CommandA");
        };
        assert_eq!(handle, 1);
        assert_eq!(parameter, 1234);
    }

    #[test]
    // Was: Prüft automatisch den Fall roundtrip bitcode response.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_roundtrip_bitcode_response() {
        let codec = ControlCodecBitcode;
        let resp = ControlResponse::CommandAResponse { handle: 1, result: 42 };
        let bytes = codec.encode_response(&resp);
        let decoded = codec.decode_response(&bytes).unwrap();
        let ControlResponse::CommandAResponse { handle, result } = decoded else {
            panic!("expected CommandAResponse");
        };
        assert_eq!(handle, 1);
        assert_eq!(result, 42);
    }

    #[test]
    // Was: Prüft automatisch den Fall roundtrip JSON-Daten response.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_roundtrip_json_response() {
        let codec = ControlCodecJson;
        let resp = ControlResponse::SendSdsResponse { handle: 2, success: true };
        let bytes = codec.encode_response(&resp);
        let decoded = codec.decode_response(&bytes).unwrap();
        let ControlResponse::SendSdsResponse { handle, success } = decoded else {
            panic!("expected SendSdsResponse");
        };
        assert_eq!(handle, 2);
        assert!(success);
    }

    #[test]
    // Was: Prüft automatisch den Fall roundtrip central TETRA-Kurznachricht (SDS) commands.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_roundtrip_central_sds_commands() {
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for command in [
            ControlCommand::DeliverSds {
                handle: 77,
                source_ssi: 4_010_001,
                dest_ssi: 4_010_002,
                dest_is_group: false,
                sds_type: 4,
                len_bits: 40,
                payload: vec![0x82, 0x04, 0x01, 0x01, b'A'],
            },
            ControlCommand::SendStatus {
                handle: 78,
                source_ssi: 4_010_001,
                dest_ssi: 4_010_002,
                pre_coded_status: 32_780,
            },
        ] {
            let json = ControlCodecJson;
            let decoded = json.decode_command(&json.encode_command(&command)).unwrap();
            assert_eq!(
                serde_json::to_value(decoded).unwrap(),
                serde_json::to_value(command.clone()).unwrap()
            );

            let bitcode = ControlCodecBitcode;
            let decoded = bitcode
                .decode_command(&bitcode.encode_command(&command))
                .unwrap();
            assert_eq!(
                serde_json::to_value(decoded).unwrap(),
                serde_json::to_value(command).unwrap()
            );
        }
    }

    #[test]
    // Was: Prüft automatisch den Fall roundtrip central TETRA-Kurznachricht (SDS) response.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_roundtrip_central_sds_response() {
        let response = ControlResponse::SdsDeliveryResponse {
            handle: 77,
            success: true,
            message: "accepted by TBS".to_string(),
        };
        let codec = ControlCodecJson;
        let decoded = codec
            .decode_response(&codec.encode_response(&response))
            .unwrap();
        let ControlResponse::SdsDeliveryResponse {
            handle,
            success,
            message,
        } = decoded
        else {
            panic!("expected SdsDeliveryResponse");
        };
        assert_eq!(handle, 77);
        assert!(success);
        assert_eq!(message, "accepted by TBS");
    }

    #[test]
    // Was: Prüft automatisch den Fall Dekodierung invalid bytes.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_decode_invalid_bytes() {
        let codec = ControlCodecBitcode;
        // Use truncated bytes that cannot form a valid Command
        assert!(codec.decode_command(&[]).is_err());
    }
}
