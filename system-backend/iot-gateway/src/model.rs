use std::collections::BTreeMap;

use netcore_contracts::{CommandAckStatus, NetCoreCommand};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboundMessage {
    pub id: Uuid,
    pub created_at: String,
    pub kind: String,
    pub topic: String,
    pub qos: u8,
    pub retain: bool,
    pub payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_event_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OutboxEntrySummary {
    pub file_name: String,
    pub id: Option<Uuid>,
    pub created_at: Option<String>,
    pub kind: Option<String>,
    pub topic: Option<String>,
    pub qos: Option<u8>,
    pub retain: Option<bool>,
    pub bytes: u64,
    pub readable: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceStatus {
    pub id: String,
    pub url: String,
    pub enabled: bool,
    pub healthy: bool,
    pub last_poll_at: Option<String>,
    pub last_success_at: Option<String>,
    pub last_error: Option<String>,
    pub consecutive_failures: u64,
    pub events_seen: u64,
    pub events_enqueued: u64,
    pub duplicates_skipped: u64,
    pub invalid_events: u64,
}

impl SourceStatus {
    pub fn new(id: String, url: String, enabled: bool) -> Self {
        Self {
            id,
            url,
            enabled,
            healthy: false,
            last_poll_at: None,
            last_success_at: None,
            last_error: None,
            consecutive_failures: 0,
            events_seen: 0,
            events_enqueued: 0,
            duplicates_skipped: 0,
            invalid_events: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BridgeEventRecord {
    pub timestamp: String,
    pub kind: String,
    pub detail: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandRecord {
    pub command_id: Uuid,
    pub received_at: String,
    pub completed_at: Option<String>,
    pub topic: String,
    pub qos: u8,
    pub retained: bool,
    pub valid_json: bool,
    pub command: Option<NetCoreCommand>,
    pub status: CommandAckStatus,
    pub policy_id: Option<String>,
    pub reason_code: Option<String>,
    pub message: String,
    #[serde(default)]
    pub result: Value,
    pub duplicate_of: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TopicRegistry {
    pub prefix: String,
    pub event_pattern: String,
    pub state_pattern: String,
    pub service_state_topic: String,
    pub command_subscription: Option<String>,
    pub command_schema: &'static str,
    pub acknowledgement_pattern: String,
    pub acknowledgement_schema: &'static str,
    pub command_execution_enabled: bool,
    pub command_execution_mode: String,
    pub policy_count: usize,
    pub qos: u8,
    pub event_retain: bool,
    pub state_retain: bool,
    pub examples: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GatewayStatus {
    pub service: &'static str,
    pub version: &'static str,
    pub phase: u8,
    pub started_at: String,
    pub security_mode: &'static str,
    pub warning: &'static str,
    pub mqtt_host: String,
    pub mqtt_port: u16,
    pub mqtt_client_id: String,
    pub mqtt_connected: bool,
    pub mqtt_connected_at: Option<String>,
    pub mqtt_last_error: Option<String>,
    pub mqtt_reconnects: u64,
    pub mqtt_messages_received: u64,
    pub broker_publish_acks: u64,
    pub sources_total: usize,
    pub sources_enabled: usize,
    pub sources_healthy: usize,
    pub outbox_pending: usize,
    pub events_seen: u64,
    pub events_enqueued: u64,
    pub events_published: u64,
    pub duplicates_skipped: u64,
    pub invalid_events: u64,
    /// Compatibility alias for the Phase-3 status field.
    pub commands_observed: u64,
    pub commands_received: u64,
    pub commands_accepted: u64,
    pub commands_rejected: u64,
    pub commands_executed: u64,
    pub commands_failed: u64,
    pub command_duplicates: u64,
    pub commands_expired: u64,
    pub retained_commands_rejected: u64,
    pub command_dry_runs: u64,
    pub command_execution_enabled: bool,
    pub command_execution_mode: String,
    pub command_policy_count: usize,
    pub virtual_devices: usize,
    pub last_poll_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TestPublishInput {
    pub topic: Option<String>,
    #[serde(default)]
    pub payload: Value,
    #[serde(default)]
    pub retain: bool,
    pub qos: Option<u8>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ActionResult {
    pub accepted: bool,
    pub message: String,
}
