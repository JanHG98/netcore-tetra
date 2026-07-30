// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! JSON codec for the NetCore Control-Room node protocol.
//!
//! JSON is deliberately used for v1: it is easy to inspect on the wire, easy for
//! a Rust/Python/TypeScript Leitstelle backend to consume, and stable enough for
//! early protocol evolution.  This codec intentionally has no dependency on the
//! runtime transport layer so `netcore-control-room` can build without SDR/audio
//! libraries.

use crate::net_control_room::protocol::{ControlRoomToNodeMessage, NodeToControlRoomMessage};

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für Steuerung room codec error in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ControlRoomCodecError(pub String);

// Was: Implementiert das zugehörige Verhalten für `std::fmt::Display for ControlRoomCodecError`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::fmt::Display for ControlRoomCodecError {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

// Was: Implementiert das zugehörige Verhalten für `std::error::Error for ControlRoomCodecError {}`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::error::Error for ControlRoomCodecError {}

#[derive(Default)]
// Was: Bündelt die zusammengehörigen Werte für Steuerung room codec JSON-Daten in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ControlRoomCodecJson;

// Was: Implementiert das zugehörige Verhalten für `ControlRoomCodecJson`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl ControlRoomCodecJson {
    // Was: Diese Funktion kodiert Uplink (Funkgerät zum Netz).
    // Warum: Alle Gegenstellen erhalten dadurch dasselbe erwartete Protokollformat.
    pub fn encode_uplink(&self, message: &NodeToControlRoomMessage) -> Vec<u8> {
        serde_json::to_vec(message).unwrap_or_default()
    }

    // Was: Diese Funktion dekodiert Uplink (Funkgerät zum Netz).
    // Warum: Empfangene Protokolldaten müssen vor der weiteren Nutzung eindeutig verstanden und geprüft werden.
    pub fn decode_uplink(&self, payload: &[u8]) -> Result<NodeToControlRoomMessage, ControlRoomCodecError> {
        serde_json::from_slice(payload).map_err(|e| ControlRoomCodecError(format!("control-room uplink decode: {}", e)))
    }

    // Was: Diese Funktion kodiert Downlink (Netz zum Funkgerät).
    // Warum: Alle Gegenstellen erhalten dadurch dasselbe erwartete Protokollformat.
    pub fn encode_downlink(&self, message: &ControlRoomToNodeMessage) -> Vec<u8> {
        serde_json::to_vec(message).unwrap_or_default()
    }

    // Was: Diese Funktion dekodiert Downlink (Netz zum Funkgerät).
    // Warum: Empfangene Protokolldaten müssen vor der weiteren Nutzung eindeutig verstanden und geprüft werden.
    pub fn decode_downlink(&self, payload: &[u8]) -> Result<ControlRoomToNodeMessage, ControlRoomCodecError> {
        serde_json::from_slice(payload).map_err(|e| ControlRoomCodecError(format!("control-room downlink decode: {}", e)))
    }
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;
    use crate::net_control_room::protocol::{ControlRoomNodeHeartbeat, NodeToControlRoomMessage};

    #[test]
    // Was: Führt den Arbeitsschritt `json_roundtrip_heartbeat` für JSON-Daten roundtrip heartbeat aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn json_roundtrip_heartbeat() {
        let codec = ControlRoomCodecJson;
        let msg = NodeToControlRoomMessage::Heartbeat {
            heartbeat: ControlRoomNodeHeartbeat {
                node_id: "tbs-test".to_string(),
                seq: 1,
                timestamp: "2026-06-30T19:00:00Z".to_string(),
                connected: true,
            },
        };
        let bytes = codec.encode_uplink(&msg);
        let decoded = codec.decode_uplink(&bytes).unwrap();
        assert!(matches!(decoded, NodeToControlRoomMessage::Heartbeat { .. }));
    }
}
