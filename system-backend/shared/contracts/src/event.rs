// NETCORE-KOMMENTAR – Was: Definiert das gemeinsame, transportneutrale NetCore-Ereignismodell.
// NETCORE-KOMMENTAR – Warum: MQTT, Webhooks, Audit, Recorder und Backend-Dienste dürfen nicht jeweils eigene inkompatible Ereignisformate erfinden.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Wire-Schema der ersten gemeinsamen NetCore-Ereignisgeneration.
pub const NETCORE_EVENT_SCHEMA_V1: &str = "netcore-event-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Debug,
    Info,
    Notice,
    Warning,
    Error,
    Critical,
    Emergency,
}

impl Default for Severity {
    fn default() -> Self {
        Self::Info
    }
}

/// Eindeutige Herkunft eines Ereignisses. `service` bezeichnet den Diensttyp,
/// `instance` die konkrete laufende Instanz. In der aktuellen Open-Lab-Stufe
/// darf beides identisch sein; spätere Deployments können Instanznamen vergeben.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventSource {
    pub service: String,
    pub instance: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

impl EventSource {
    pub fn new(service: impl Into<String>, instance: impl Into<String>) -> Self {
        Self {
            service: service.into(),
            instance: instance.into(),
            node_id: None,
        }
    }

    pub fn service(service: impl Into<String>) -> Self {
        let service = service.into();
        Self::new(service.clone(), service)
    }

    pub fn with_node_id(mut self, node_id: impl Into<String>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
}

/// Fachobjekt, auf das sich das Ereignis primär bezieht.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventSubject {
    #[serde(rename = "type")]
    pub subject_type: String,
    pub id: String,
}

impl EventSubject {
    pub fn new(subject_type: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            subject_type: subject_type.into(),
            id: id.into(),
        }
    }
}

/// Gemeinsames Ereignisformat für interne APIs und die spätere MQTT-Brücke.
///
/// Das Format ist absichtlich transportneutral. Es enthält keine MQTT-Topics,
/// HTTP-Ziele oder Broker-spezifischen Zustände. Diese Zuordnung übernimmt erst
/// der spätere IoT-Gateway-Dienst.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetCoreEvent {
    pub schema: String,
    pub event_id: Uuid,
    pub event_type: String,
    pub source: EventSource,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causation_id: Option<Uuid>,
    pub severity: Severity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<EventSubject>,
    #[serde(default)]
    pub payload: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduplication_key: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub labels: BTreeMap<String, String>,
}

impl NetCoreEvent {
    pub fn new(
        event_type: impl Into<String>,
        source: EventSource,
        severity: Severity,
        subject: Option<EventSubject>,
        payload: Value,
    ) -> Self {
        Self {
            schema: NETCORE_EVENT_SCHEMA_V1.to_owned(),
            event_id: Uuid::new_v4(),
            event_type: event_type.into(),
            source,
            timestamp: Utc::now(),
            sequence: None,
            correlation_id: None,
            causation_id: None,
            severity,
            subject,
            payload,
            deduplication_key: None,
            labels: BTreeMap::new(),
        }
    }

    pub fn with_sequence(mut self, sequence: u64) -> Self {
        self.sequence = Some(sequence);
        if self.deduplication_key.is_none() {
            self.deduplication_key = Some(format!(
                "{}:{}:{}",
                self.source.service, self.source.instance, sequence
            ));
        }
        self
    }

    pub fn with_correlation_id(mut self, correlation_id: Uuid) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    pub fn with_causation_id(mut self, causation_id: Uuid) -> Self {
        self.causation_id = Some(causation_id);
        self
    }

    pub fn with_deduplication_key(mut self, value: impl Into<String>) -> Self {
        self.deduplication_key = Some(value.into());
        self
    }

    pub fn with_label(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.labels.insert(key.into(), value.into());
        self
    }

    pub fn validate(&self) -> Result<(), EventValidationError> {
        if self.schema != NETCORE_EVENT_SCHEMA_V1 {
            return Err(EventValidationError::InvalidSchema(self.schema.clone()));
        }
        if self.event_id.is_nil() {
            return Err(EventValidationError::NilEventId);
        }
        if !is_event_type(&self.event_type) {
            return Err(EventValidationError::InvalidEventType(self.event_type.clone()));
        }
        if !is_component_name(&self.source.service, 128) {
            return Err(EventValidationError::InvalidSourceService(
                self.source.service.clone(),
            ));
        }
        if !is_component_name(&self.source.instance, 128) {
            return Err(EventValidationError::InvalidSourceInstance(
                self.source.instance.clone(),
            ));
        }
        if let Some(node_id) = &self.source.node_id {
            if !is_free_identifier(node_id, 160) {
                return Err(EventValidationError::InvalidSourceNode(node_id.clone()));
            }
        }
        if let Some(subject) = &self.subject {
            if !is_subject_type(&subject.subject_type) {
                return Err(EventValidationError::InvalidSubjectType(
                    subject.subject_type.clone(),
                ));
            }
            if !is_free_identifier(&subject.id, 256) {
                return Err(EventValidationError::InvalidSubjectId(subject.id.clone()));
            }
        }
        if let Some(value) = &self.deduplication_key {
            if !is_free_identifier(value, 320) {
                return Err(EventValidationError::InvalidDeduplicationKey(value.clone()));
            }
        }
        for (key, value) in &self.labels {
            if !is_label_key(key) || value.len() > 256 {
                return Err(EventValidationError::InvalidLabel(key.clone()));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventValidationError {
    InvalidSchema(String),
    NilEventId,
    InvalidEventType(String),
    InvalidSourceService(String),
    InvalidSourceInstance(String),
    InvalidSourceNode(String),
    InvalidSubjectType(String),
    InvalidSubjectId(String),
    InvalidDeduplicationKey(String),
    InvalidLabel(String),
}

impl fmt::Display for EventValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSchema(value) => write!(formatter, "unsupported event schema: {value}"),
            Self::NilEventId => formatter.write_str("event_id must not be nil"),
            Self::InvalidEventType(value) => write!(formatter, "invalid event_type: {value}"),
            Self::InvalidSourceService(value) => write!(formatter, "invalid source.service: {value}"),
            Self::InvalidSourceInstance(value) => write!(formatter, "invalid source.instance: {value}"),
            Self::InvalidSourceNode(value) => write!(formatter, "invalid source.node_id: {value}"),
            Self::InvalidSubjectType(value) => write!(formatter, "invalid subject.type: {value}"),
            Self::InvalidSubjectId(value) => write!(formatter, "invalid subject.id: {value}"),
            Self::InvalidDeduplicationKey(value) => {
                write!(formatter, "invalid deduplication_key: {value}")
            }
            Self::InvalidLabel(value) => write!(formatter, "invalid event label: {value}"),
        }
    }
}

impl Error for EventValidationError {}

/// Prüft die verbindliche Event-Namenskonvention `domain.action_name`.
pub fn is_event_type(value: &str) -> bool {
    if value.is_empty() || value.len() > 160 {
        return false;
    }
    let mut segments = value.split('.');
    let Some(first) = segments.next() else {
        return false;
    };
    if !is_lower_identifier(first) {
        return false;
    }
    let mut count = 1;
    for segment in segments {
        count += 1;
        if !is_lower_identifier(segment) {
            return false;
        }
    }
    count >= 2
}

fn is_subject_type(value: &str) -> bool {
    !value.is_empty() && value.len() <= 64 && is_lower_identifier(value)
}

fn is_lower_identifier(value: &str) -> bool {
    let mut bytes = value.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    first.is_ascii_lowercase()
        && bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn is_component_name(value: &str, max: usize) -> bool {
    if value.is_empty() || value.len() > max {
        return false;
    }
    value.bytes().all(|byte| {
        byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':' | b'/')
    })
}

fn is_free_identifier(value: &str, max: usize) -> bool {
    !value.is_empty()
        && value.len() <= max
        && !value.bytes().any(|byte| byte.is_ascii_control())
}

fn is_label_key(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 96
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'_' | b'.' | b'-')
        })
}

/// Verbindlicher Ereigniskatalog für Phase 2. Neue Ereignisse werden hier
/// ergänzt, bevor ein Dienst sie veröffentlicht.
pub mod event_types {
    pub const SERVICE_DEPENDENCY_CONNECTED: &str = "service.dependency_connected";
    pub const SERVICE_DEPENDENCY_DISCONNECTED: &str = "service.dependency_disconnected";
    pub const SERVICE_STATE_CHANGED: &str = "service.state_changed";

    pub const NODE_CONNECTED: &str = "node.connected";
    pub const NODE_DISCONNECTED: &str = "node.disconnected";
    pub const NODE_DEGRADED: &str = "node.degraded";
    pub const NODE_MESSAGE_RECEIVED: &str = "node.message_received";
    pub const NODE_COMMAND_QUEUED: &str = "node.command_queued";

    pub const SUBSCRIBER_REGISTERED: &str = "subscriber.registered";
    pub const SUBSCRIBER_ROUTE_CHANGED: &str = "subscriber.route_changed";
    pub const SUBSCRIBER_DETACHED: &str = "subscriber.detached";
    pub const SUBSCRIBER_ROUTE_EXPIRED: &str = "subscriber.route_expired";

    pub const MOBILITY_TRANSFER_CREATED: &str = "mobility.transfer_created";
    pub const MOBILITY_TRANSFER_CANCELLED: &str = "mobility.transfer_cancelled";
    pub const MOBILITY_TRANSFER_COMPLETED: &str = "mobility.transfer_completed";
    pub const MOBILITY_TRANSFER_FAILED: &str = "mobility.transfer_failed";
    pub const MOBILITY_TRANSFER_TIMED_OUT: &str = "mobility.transfer_timed_out";

    pub const CALL_REQUESTED: &str = "call.requested";
    pub const CALL_STARTED: &str = "call.started";
    pub const CALL_CONNECTED: &str = "call.connected";
    pub const CALL_RELEASE_REQUESTED: &str = "call.release_requested";
    pub const CALL_RELEASED: &str = "call.released";
    pub const CALL_FAILED: &str = "call.failed";
    pub const CALL_RESTORE_REQUESTED: &str = "call.restore_requested";
    pub const CALL_RESTORED: &str = "call.restored";
    pub const CALL_RESTORE_FAILED: &str = "call.restore_failed";
    pub const CALL_MEDIA_ROUTE_READY: &str = "call.media_route_ready";

    pub const FLOOR_REQUESTED: &str = "floor.requested";
    pub const FLOOR_RELEASE_REQUESTED: &str = "floor.release_requested";
    pub const FLOOR_CHANGED: &str = "floor.changed";

    pub const SDS_CREATED: &str = "sds.created";
    pub const SDS_RECEIVED: &str = "sds.received";
    pub const SDS_RETRY_SCHEDULED: &str = "sds.retry_scheduled";
    pub const SDS_DELIVERY_ACCEPTED: &str = "sds.delivery_accepted";
    pub const SDS_DELIVERY_RETRY: &str = "sds.delivery_retry";
    pub const SDS_DELIVERED: &str = "sds.delivered";
    pub const SDS_FAILED: &str = "sds.failed";
    pub const SDS_CANCELLED: &str = "sds.cancelled";
    pub const SDS_EXPIRED: &str = "sds.expired";
    pub const SDS_DUPLICATE: &str = "sds.duplicate";
    pub const SDS_REQUEUED: &str = "sds.requeued";
    pub const SDS_DELETED: &str = "sds.deleted";
    pub const SDS_ACKNOWLEDGED: &str = "sds.acknowledged";
    pub const SDS_ROUTE_CREATED: &str = "sds.route_created";
    pub const SDS_ROUTE_UPDATED: &str = "sds.route_updated";
    pub const SDS_ROUTE_DELETED: &str = "sds.route_deleted";

    pub const HARDWARE_DEVICE_REGISTERED: &str = "hardware.device_registered";
    pub const HARDWARE_DEVICE_ONLINE: &str = "hardware.device_online";
    pub const HARDWARE_DEVICE_OFFLINE: &str = "hardware.device_offline";
    pub const HARDWARE_THRESHOLD_EXCEEDED: &str = "hardware.threshold_exceeded";
    pub const HARDWARE_THRESHOLD_CLEARED: &str = "hardware.threshold_cleared";
    pub const HARDWARE_INPUT_ACTIVATED: &str = "hardware.input_activated";
    pub const HARDWARE_INPUT_CLEARED: &str = "hardware.input_cleared";

    pub const RF_STATION_REGISTERED: &str = "rf.station_registered";
    pub const RF_STATION_ONLINE: &str = "rf.station_online";
    pub const RF_STATION_OFFLINE: &str = "rf.station_offline";
    pub const RF_TX_STATE_CHANGED: &str = "rf.tx_state_changed";
    pub const RF_ALARM_RAISED: &str = "rf.alarm_raised";
    pub const RF_ALARM_CLEARED: &str = "rf.alarm_cleared";
    pub const RF_TELEMETRY_INVALID: &str = "rf.telemetry_invalid";

    pub const ALARM_MANUAL_REQUEST: &str = "alarm.manual_request";
    pub const ALARM_CREATED: &str = "alarm.created";
    pub const ALARM_OCCURRENCE_ADDED: &str = "alarm.occurrence_added";
    pub const ALARM_NOTIFICATION_STARTED: &str = "alarm.notification_started";
    pub const ALARM_NOTIFICATION_QUEUED: &str = "alarm.notification_queued";
    pub const ALARM_NOTIFICATION_FAILED: &str = "alarm.notification_failed";
    pub const ALARM_ESCALATED: &str = "alarm.escalated";
    pub const ALARM_ACKNOWLEDGED: &str = "alarm.acknowledged";
    pub const ALARM_ASSIGNED: &str = "alarm.assigned";
    pub const ALARM_IN_PROGRESS: &str = "alarm.in_progress";
    pub const ALARM_RESOLVED: &str = "alarm.resolved";
    pub const ALARM_CLOSED: &str = "alarm.closed";
    pub const ALARM_CANCELLED: &str = "alarm.cancelled";
    pub const ALARM_REOPENED: &str = "alarm.reopened";
    pub const ALARM_STATUS_ACTION_APPLIED: &str = "alarm.status_action_applied";
    pub const ALARM_TEXT_ACTION_APPLIED: &str = "alarm.text_action_applied";

    pub const ALL: &[&str] = &[
        SERVICE_DEPENDENCY_CONNECTED,
        SERVICE_DEPENDENCY_DISCONNECTED,
        SERVICE_STATE_CHANGED,
        NODE_CONNECTED,
        NODE_DISCONNECTED,
        NODE_DEGRADED,
        NODE_MESSAGE_RECEIVED,
        NODE_COMMAND_QUEUED,
        SUBSCRIBER_REGISTERED,
        SUBSCRIBER_ROUTE_CHANGED,
        SUBSCRIBER_DETACHED,
        SUBSCRIBER_ROUTE_EXPIRED,
        MOBILITY_TRANSFER_CREATED,
        MOBILITY_TRANSFER_CANCELLED,
        MOBILITY_TRANSFER_COMPLETED,
        MOBILITY_TRANSFER_FAILED,
        MOBILITY_TRANSFER_TIMED_OUT,
        CALL_REQUESTED,
        CALL_STARTED,
        CALL_CONNECTED,
        CALL_RELEASE_REQUESTED,
        CALL_RELEASED,
        CALL_FAILED,
        CALL_RESTORE_REQUESTED,
        CALL_RESTORED,
        CALL_RESTORE_FAILED,
        CALL_MEDIA_ROUTE_READY,
        FLOOR_REQUESTED,
        FLOOR_RELEASE_REQUESTED,
        FLOOR_CHANGED,
        SDS_CREATED,
        SDS_RECEIVED,
        SDS_RETRY_SCHEDULED,
        SDS_DELIVERY_ACCEPTED,
        SDS_DELIVERY_RETRY,
        SDS_DELIVERED,
        SDS_FAILED,
        SDS_CANCELLED,
        SDS_EXPIRED,
        SDS_DUPLICATE,
        SDS_REQUEUED,
        SDS_DELETED,
        SDS_ACKNOWLEDGED,
        SDS_ROUTE_CREATED,
        SDS_ROUTE_UPDATED,
        SDS_ROUTE_DELETED,
        HARDWARE_DEVICE_REGISTERED,
        HARDWARE_DEVICE_ONLINE,
        HARDWARE_DEVICE_OFFLINE,
        HARDWARE_THRESHOLD_EXCEEDED,
        HARDWARE_THRESHOLD_CLEARED,
        HARDWARE_INPUT_ACTIVATED,
        HARDWARE_INPUT_CLEARED,
        RF_STATION_REGISTERED,
        RF_STATION_ONLINE,
        RF_STATION_OFFLINE,
        RF_TX_STATE_CHANGED,
        RF_ALARM_RAISED,
        RF_ALARM_CLEARED,
        RF_TELEMETRY_INVALID,
        ALARM_MANUAL_REQUEST,
        ALARM_CREATED,
        ALARM_OCCURRENCE_ADDED,
        ALARM_NOTIFICATION_STARTED,
        ALARM_NOTIFICATION_QUEUED,
        ALARM_NOTIFICATION_FAILED,
        ALARM_ESCALATED,
        ALARM_ACKNOWLEDGED,
        ALARM_ASSIGNED,
        ALARM_IN_PROGRESS,
        ALARM_RESOLVED,
        ALARM_CLOSED,
        ALARM_CANCELLED,
        ALARM_REOPENED,
        ALARM_STATUS_ACTION_APPLIED,
        ALARM_TEXT_ACTION_APPLIED,
    ];
}

pub mod subject_types {
    pub const SERVICE: &str = "service";
    pub const NODE: &str = "node";
    pub const SUBSCRIBER: &str = "subscriber";
    pub const GROUP: &str = "group";
    pub const CALL: &str = "call";
    pub const FLOOR: &str = "floor";
    pub const RESTORE: &str = "restore";
    pub const SDS_MESSAGE: &str = "sds_message";
    pub const SDS_ROUTE: &str = "sds_route";
    pub const MOBILITY_TRANSFER: &str = "mobility_transfer";
    pub const HARDWARE_DEVICE: &str = "hardware_device";
    pub const RF_STATION: &str = "rf_station";
    pub const ALARM: &str = "alarm";
}

/// Bestehender einfacher Datensatz bleibt für Quellkompatibilität erhalten.
/// Neue Runtime-Ereignisse sollen `NetCoreEvent` verwenden.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventRecord {
    pub event_id: Uuid,
    pub event_type: String,
    pub source: String,
    pub severity: Severity,
    pub occurred_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<Uuid>,
    #[serde(default)]
    pub attributes: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuditRecord {
    pub audit_id: Uuid,
    pub service: String,
    pub actor: String,
    pub action: String,
    pub outcome: String,
    pub occurred_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<Uuid>,
    #[serde(default)]
    pub changes: BTreeMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn canonical_event_roundtrips_and_validates() {
        let event = NetCoreEvent::new(
            event_types::SUBSCRIBER_ROUTE_CHANGED,
            EventSource::new("netcore-mobility-core", "mobility-core-01"),
            Severity::Info,
            Some(EventSubject::new(subject_types::SUBSCRIBER, "4010001")),
            json!({"previous_node":"TBS-01","serving_node":"TBS-02"}),
        )
        .with_sequence(14)
        .with_label("legacy_kind", "subscriber_route_changed");

        event.validate().unwrap();
        let encoded = serde_json::to_vec(&event).unwrap();
        let decoded: NetCoreEvent = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(decoded, event);
        assert_eq!(decoded.schema, NETCORE_EVENT_SCHEMA_V1);
        assert_eq!(decoded.deduplication_key.as_deref(), Some("netcore-mobility-core:mobility-core-01:14"));
    }

    #[test]
    fn event_type_requires_dotted_lowercase_name() {
        assert!(is_event_type("subscriber.route_changed"));
        assert!(is_event_type("call.restore_failed"));
        assert!(!is_event_type("subscriber_route_changed"));
        assert!(!is_event_type("Subscriber.route_changed"));
        assert!(!is_event_type("subscriber."));
    }

    #[test]
    fn all_catalog_entries_are_valid_and_unique() {
        let mut seen = std::collections::BTreeSet::new();
        for value in event_types::ALL {
            assert!(is_event_type(value), "invalid catalog entry: {value}");
            assert!(seen.insert(*value), "duplicate catalog entry: {value}");
        }
    }
}
