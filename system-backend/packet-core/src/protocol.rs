// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für SNDCP-Kontexte und TETRA-Paketdaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use tetra_entities::net_control::ControlCommand;
use tetra_entities::net_control_room::{
    ControlRoomNodeCapabilities, ControlRoomNodeIdentity, NodeToControlRoomMessage,
};

// Was: Legt den festen Wert `BACKEND_PROTOCOL_VERSION` für Hintergrunddienst protocol version fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const BACKEND_PROTOCOL_VERSION: &str = "netcore-node-gateway-backend-v1";
// Was: Legt den festen Wert `EDGE_PROTOCOL_VERSION` für edge protocol version fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const EDGE_PROTOCOL_VERSION: &str = "netcore-packet-edge-v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Gateway Netzknoten snapshot in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct GatewayNodeSnapshot {
    pub node_id: String,
    pub session_id: String,
    pub peer: String,
    pub connected: bool,
    pub stale: bool,
    pub connected_at: String,
    pub last_seen: String,
    pub disconnected_at: Option<String>,
    pub disconnect_reason: Option<String>,
    pub heartbeat_seq: u64,
    pub message_count: u64,
    pub telemetry_count: u64,
    pub control_ack_count: u64,
    pub control_response_count: u64,
    #[serde(default)]
    pub media_frame_count: u64,
    pub error_count: u64,
    pub last_message_kind: String,
    pub last_telemetry: Option<Value>,
    pub identity: ControlRoomNodeIdentity,
    pub capabilities: ControlRoomNodeCapabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Gateway Status in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct GatewayStatus {
    pub service: String,
    pub started_at: String,
    pub security_mode: String,
    pub warning: String,
    pub remote_management_enabled: bool,
    pub node_path: String,
    pub backend_path: String,
    pub known_nodes: usize,
    pub connected_nodes: usize,
    pub stale_nodes: usize,
    pub backend_clients: usize,
    pub total_node_sessions: u64,
    pub total_node_messages: u64,
    pub total_commands: u64,
    #[serde(default)]
    pub total_media_frames: u64,
    pub total_disconnects: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Gateway snapshot in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct GatewaySnapshot {
    pub status: GatewayStatus,
    pub nodes: Vec<GatewayNodeSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Gateway Ereignis Datensatz in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct GatewayEventRecord {
    pub seq: u64,
    pub timestamp: String,
    pub kind: String,
    pub node_id: Option<String>,
    pub detail: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für Hintergrunddienst Ereignis auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum BackendEvent {
    Snapshot { snapshot: GatewaySnapshot },
    Event { event: GatewayEventRecord },
    NodeMessage { node_id: String, message: NodeToControlRoomMessage },
    ActionResult {
        request_id: Option<String>,
        command_id: Option<String>,
        ok: bool,
        message: String,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für Hintergrunddienst request auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum BackendRequest {
    Ping { request_id: Option<String> },
    Command {
        request_id: Option<String>,
        node_id: String,
        command: ControlCommand,
        operator_id: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für edge Ereignis input auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum EdgeEventInput {
    Hello {
        protocol_version: String,
        node_id: String,
        station_name: Option<String>,
        mcc: Option<u16>,
        mnc: Option<u16>,
        location_area: Option<u16>,
    },
    Heartbeat {
        node_id: String,
        sequence: u64,
    },
    SubscriberLocation {
        node_id: String,
        issi: u32,
    },
    ActivateDemand {
        node_id: String,
        issi: u32,
        nsapi: u8,
        requested_ipv4: Option<String>,
        primary_nsapi: Option<u8>,
        snei: Option<u16>,
        mtu: Option<u16>,
        priority: Option<u8>,
    },
    ContextActivated {
        node_id: String,
        issi: u32,
        nsapi: u8,
        ipv4: String,
        primary_nsapi: Option<u8>,
        snei: Option<u16>,
        mtu: u16,
        priority: u8,
    },
    DataTransmitRequest {
        node_id: String,
        issi: u32,
        nsapis: Vec<u8>,
    },
    EndOfData {
        node_id: String,
        issi: u32,
        nsapis: Vec<u8>,
    },
    Reconnect {
        node_id: String,
        issi: u32,
        nsapis: Vec<u8>,
        data_to_send: bool,
    },
    Modify {
        node_id: String,
        issi: u32,
        nsapi: u8,
        availability: Option<bool>,
        usage_active: Option<bool>,
        priority: Option<u8>,
        mtu: Option<u16>,
    },
    Deactivate {
        node_id: String,
        issi: u32,
        nsapi: Option<u8>,
        reason: Option<String>,
    },
    Bearer {
        node_id: String,
        issi: u32,
        carrier_num: u16,
        logical_ts: u8,
        air_ts: u8,
        nsapis: Vec<u8>,
        active: bool,
    },
    Fragment {
        node_id: String,
        issi: u32,
        nsapi: u8,
        datagram_id: String,
        direction: String,
        offset: usize,
        more_fragments: bool,
        total_len: Option<usize>,
        payload_hex: String,
    },
    PacketCounters {
        node_id: String,
        issi: u32,
        nsapi: u8,
        packets_up: u64,
        bytes_up: u64,
        packets_down: u64,
        bytes_down: u64,
        dropped: u64,
    },
    NodeLost {
        node_id: String,
        reason: Option<String>,
    },
}

// Was: Implementiert das zugehörige Verhalten für `EdgeEventInput`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl EdgeEventInput {
    // Was: Führt den Arbeitsschritt `node_id` für Netzknoten Kennung aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn node_id(&self) -> &str {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            Self::Hello { node_id, .. }
            | Self::Heartbeat { node_id, .. }
            | Self::SubscriberLocation { node_id, .. }
            | Self::ActivateDemand { node_id, .. }
            | Self::ContextActivated { node_id, .. }
            | Self::DataTransmitRequest { node_id, .. }
            | Self::EndOfData { node_id, .. }
            | Self::Reconnect { node_id, .. }
            | Self::Modify { node_id, .. }
            | Self::Deactivate { node_id, .. }
            | Self::Bearer { node_id, .. }
            | Self::Fragment { node_id, .. }
            | Self::PacketCounters { node_id, .. }
            | Self::NodeLost { node_id, .. } => node_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für edge action payload auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum EdgeActionPayload {
    ActivateAccept {
        issi: u32,
        nsapi: u8,
        ipv4: String,
        primary_nsapi: Option<u8>,
        snei: u16,
        mtu: u16,
        priority: u8,
        ready_timer_secs: u64,
        standby_timer_secs: u64,
        response_wait_secs: u64,
    },
    ActivateReject {
        issi: u32,
        nsapi: u8,
        cause: String,
    },
    DataTransmitResponse {
        issi: u32,
        nsapis: Vec<u8>,
        accepted: bool,
        cause: Option<String>,
    },
    EndOfData {
        issi: u32,
        nsapis: Vec<u8>,
    },
    Deactivate {
        issi: u32,
        nsapi: Option<u8>,
        reason: String,
    },
    Modify {
        issi: u32,
        nsapi: u8,
        availability: Option<bool>,
        usage_active: Option<bool>,
        priority: Option<u8>,
        mtu: Option<u16>,
    },
    Page {
        issi: u32,
        nsapi: u8,
    },
    NpduFragment {
        issi: u32,
        nsapi: u8,
        datagram_id: String,
        offset: usize,
        more_fragments: bool,
        total_len: usize,
        payload_hex: String,
        acknowledged: bool,
    },
    ReleaseBearer {
        issi: u32,
    },
}

#[derive(Debug, Clone, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für action ack input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ActionAckInput {
    #[serde(default = "default_ack_success")]
    pub success: bool,
    pub message: Option<String>,
}

// Was: Führt den Arbeitsschritt `default_ack_success` für default ack success aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_ack_success() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Downlink (Netz zum Funkgerät) npdu input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct DownlinkNpduInput {
    pub issi: u32,
    pub nsapi: u8,
    pub payload_hex: String,
    #[serde(default)]
    pub acknowledged: bool,
    #[serde(default)]
    pub priority: Option<u8>,
}

#[derive(Debug, Clone, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Kontext action input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ContextActionInput {
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub available: Option<bool>,
    #[serde(default)]
    pub usage_active: Option<bool>,
    #[serde(default)]
    pub priority: Option<u8>,
    #[serde(default)]
    pub mtu: Option<u16>,
    #[serde(default)]
    pub nsapis: Vec<u8>,
}
