use std::collections::{BTreeMap, HashSet, VecDeque};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use chrono::Utc;
use netcore_contracts::NetCoreEvent;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::config::IotGatewayConfig;
use crate::model::{
    BridgeEventRecord, GatewayStatus, ObservedCommand, OutboundMessage, OutboxEntrySummary,
    SourceStatus, TopicRegistry,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnqueueOutcome {
    Enqueued,
    Duplicate,
}

struct GatewayState {
    config: IotGatewayConfig,
    started_at: String,
    mqtt_connected: bool,
    mqtt_connected_at: Option<String>,
    mqtt_last_error: Option<String>,
    mqtt_reconnects: u64,
    mqtt_messages_received: u64,
    broker_publish_acks: u64,
    sources: BTreeMap<String, SourceStatus>,
    recent_events: VecDeque<BridgeEventRecord>,
    commands: VecDeque<ObservedCommand>,
    dedup: HashSet<Uuid>,
    dedup_order: VecDeque<Uuid>,
    events_seen: u64,
    events_enqueued: u64,
    events_published: u64,
    duplicates_skipped: u64,
    invalid_events: u64,
    commands_observed: u64,
    commands_executed: u64,
    last_poll_at: Option<String>,
}

#[derive(Clone)]
pub struct SharedGateway(Arc<Mutex<GatewayState>>);

impl SharedGateway {
    pub fn new(config: IotGatewayConfig) -> Result<Self, Box<dyn std::error::Error>> {
        fs::create_dir_all(config.state_dir())?;
        fs::create_dir_all(config.outbox_dir())?;
        let (dedup, dedup_order) = load_dedup(&config.dedup_path(), config.storage.dedup_limit)?;
        let sources = config
            .sources
            .iter()
            .map(|source| {
                (
                    source.id.clone(),
                    SourceStatus::new(source.id.clone(), source.url.clone(), source.enabled),
                )
            })
            .collect();
        let state = GatewayState {
            config,
            started_at: now_iso(),
            mqtt_connected: false,
            mqtt_connected_at: None,
            mqtt_last_error: None,
            mqtt_reconnects: 0,
            mqtt_messages_received: 0,
            broker_publish_acks: 0,
            sources,
            recent_events: VecDeque::new(),
            commands: VecDeque::new(),
            dedup,
            dedup_order,
            events_seen: 0,
            events_enqueued: 0,
            events_published: 0,
            duplicates_skipped: 0,
            invalid_events: 0,
            commands_observed: 0,
            commands_executed: 0,
            last_poll_at: None,
        };
        let gateway = Self(Arc::new(Mutex::new(state)));
        gateway.record_bridge_event(
            "iot_gateway.started",
            json!({"phase":3,"command_execution":false,"security_mode":"open_lab"}),
        );
        Ok(gateway)
    }

    pub fn config(&self) -> IotGatewayConfig {
        self.0.lock().expect("IoT Gateway state poisoned").config.clone()
    }

    pub fn status(&self) -> GatewayStatus {
        let state = self.0.lock().expect("IoT Gateway state poisoned");
        let enabled = state.sources.values().filter(|source| source.enabled).count();
        let healthy = state
            .sources
            .values()
            .filter(|source| source.enabled && source.healthy)
            .count();
        GatewayStatus {
            service: "netcore-iot-gateway",
            version: env!("CARGO_PKG_VERSION"),
            started_at: state.started_at.clone(),
            security_mode: "open_lab",
            warning: "No login, no tokens and no TLS; isolated test network only",
            mqtt_host: state.config.mqtt.host.clone(),
            mqtt_port: state.config.mqtt.port,
            mqtt_client_id: state.config.mqtt.client_id.clone(),
            mqtt_connected: state.mqtt_connected,
            mqtt_connected_at: state.mqtt_connected_at.clone(),
            mqtt_last_error: state.mqtt_last_error.clone(),
            mqtt_reconnects: state.mqtt_reconnects,
            mqtt_messages_received: state.mqtt_messages_received,
            broker_publish_acks: state.broker_publish_acks,
            sources_total: state.sources.len(),
            sources_enabled: enabled,
            sources_healthy: healthy,
            outbox_pending: count_outbox(&state.config.outbox_dir()),
            events_seen: state.events_seen,
            events_enqueued: state.events_enqueued,
            events_published: state.events_published,
            duplicates_skipped: state.duplicates_skipped,
            invalid_events: state.invalid_events,
            commands_observed: state.commands_observed,
            commands_executed: state.commands_executed,
            command_execution_enabled: state.config.mqtt.execute_commands,
            last_poll_at: state.last_poll_at.clone(),
        }
    }

    pub fn sources(&self) -> Vec<SourceStatus> {
        self.0
            .lock()
            .expect("IoT Gateway state poisoned")
            .sources
            .values()
            .cloned()
            .collect()
    }

    pub fn recent_events(&self, limit: usize) -> Vec<BridgeEventRecord> {
        let state = self.0.lock().expect("IoT Gateway state poisoned");
        state
            .recent_events
            .iter()
            .rev()
            .take(limit.min(state.recent_events.len()))
            .cloned()
            .collect()
    }

    pub fn commands(&self, limit: usize) -> Vec<ObservedCommand> {
        let state = self.0.lock().expect("IoT Gateway state poisoned");
        state
            .commands
            .iter()
            .rev()
            .take(limit.min(state.commands.len()))
            .cloned()
            .collect()
    }

    pub fn topic_registry(&self) -> TopicRegistry {
        let state = self.0.lock().expect("IoT Gateway state poisoned");
        let prefix = state.config.mqtt.topic_prefix.trim_matches('/').to_string();
        let mut examples = BTreeMap::new();
        examples.insert(
            "subscriber_route_changed".to_string(),
            format!("{prefix}/events/subscriber/route_changed"),
        );
        examples.insert(
            "subscriber_state".to_string(),
            format!("{prefix}/state/subscribers/4010001"),
        );
        examples.insert(
            "node_state".to_string(),
            format!("{prefix}/state/nodes/TBS-01"),
        );
        examples.insert(
            "gateway_state".to_string(),
            format!("{prefix}/state/services/iot-gateway"),
        );
        TopicRegistry {
            prefix: prefix.clone(),
            event_pattern: format!("{prefix}/events/<domain>/<action>"),
            state_pattern: format!("{prefix}/state/<subject-type>/<subject-id>"),
            service_state_topic: format!("{prefix}/state/services/iot-gateway"),
            command_subscription: state
                .config
                .mqtt
                .observe_commands
                .then(|| format!("{prefix}/commands/#")),
            command_execution_enabled: state.config.mqtt.execute_commands,
            qos: state.config.mqtt.qos,
            event_retain: state.config.mqtt.event_retain,
            state_retain: state.config.mqtt.state_retain,
            examples,
        }
    }

    pub fn mqtt_connected(&self) -> bool {
        self.0.lock().expect("IoT Gateway state poisoned").mqtt_connected
    }

    pub fn mark_mqtt_connected(&self) {
        let mut state = self.0.lock().expect("IoT Gateway state poisoned");
        state.mqtt_connected = true;
        state.mqtt_connected_at = Some(now_iso());
        state.mqtt_last_error = None;
        state.mqtt_reconnects = state.mqtt_reconnects.saturating_add(1);
        let host = state.config.mqtt.host.clone();
        let port = state.config.mqtt.port;
        push_event_locked(
            &mut state,
            "mqtt.connected",
            json!({"host":host,"port":port}),
        );
    }

    pub fn mark_mqtt_disconnected(&self, error: impl Into<String>) {
        let error = error.into();
        let mut state = self.0.lock().expect("IoT Gateway state poisoned");
        state.mqtt_connected = false;
        state.mqtt_last_error = Some(error.clone());
        push_event_locked(&mut state, "mqtt.disconnected", json!({"error":error}));
    }

    pub fn record_mqtt_message(&self) {
        let mut state = self.0.lock().expect("IoT Gateway state poisoned");
        state.mqtt_messages_received = state.mqtt_messages_received.saturating_add(1);
    }

    pub fn record_publish_ack(&self, message: &OutboundMessage) {
        let mut state = self.0.lock().expect("IoT Gateway state poisoned");
        state.broker_publish_acks = state.broker_publish_acks.saturating_add(1);
        state.events_published = state.events_published.saturating_add(1);
        if state.events_published <= 10 || state.events_published % 100 == 0 {
            push_event_locked(
                &mut state,
                "mqtt.message_published",
                json!({"topic":message.topic,"kind":message.kind,"message_id":message.id}),
            );
        }
    }

    pub fn mark_poll_started(&self) {
        let mut state = self.0.lock().expect("IoT Gateway state poisoned");
        state.last_poll_at = Some(now_iso());
    }

    pub fn record_source_success(
        &self,
        source_id: &str,
        seen: u64,
        enqueued: u64,
        duplicates: u64,
        invalid: u64,
    ) {
        let mut state = self.0.lock().expect("IoT Gateway state poisoned");
        let now = now_iso();
        let recovered = if let Some(source) = state.sources.get_mut(source_id) {
            let recovered = !source.healthy && source.consecutive_failures > 0;
            source.healthy = true;
            source.last_poll_at = Some(now.clone());
            source.last_success_at = Some(now);
            source.last_error = None;
            source.consecutive_failures = 0;
            source.events_seen = source.events_seen.saturating_add(seen);
            source.events_enqueued = source.events_enqueued.saturating_add(enqueued);
            source.duplicates_skipped = source.duplicates_skipped.saturating_add(duplicates);
            source.invalid_events = source.invalid_events.saturating_add(invalid);
            recovered
        } else {
            false
        };
        if recovered {
            push_event_locked(
                &mut state,
                "source.recovered",
                json!({"source":source_id}),
            );
        }
    }

    pub fn record_source_failure(&self, source_id: &str, error: impl Into<String>) {
        let error = error.into();
        let mut state = self.0.lock().expect("IoT Gateway state poisoned");
        let failures = if let Some(source) = state.sources.get_mut(source_id) {
            source.healthy = false;
            source.last_poll_at = Some(now_iso());
            source.last_error = Some(error.clone());
            source.consecutive_failures = source.consecutive_failures.saturating_add(1);
            Some(source.consecutive_failures)
        } else {
            None
        };
        if let Some(failures) = failures {
            if failures == 1 || failures % 10 == 0 {
                push_event_locked(
                    &mut state,
                    "source.failed",
                    json!({"source":source_id,"error":error,"consecutive_failures":failures}),
                );
            }
        }
    }

    pub fn record_invalid_event(&self, source_id: &str, reason: impl Into<String>) {
        let reason = reason.into();
        let mut state = self.0.lock().expect("IoT Gateway state poisoned");
        state.invalid_events = state.invalid_events.saturating_add(1);
        push_event_locked(
            &mut state,
            "event.rejected",
            json!({"source":source_id,"reason":reason}),
        );
    }

    pub fn enqueue_event(&self, event: &NetCoreEvent) -> Result<EnqueueOutcome, String> {
        event.validate().map_err(|error| error.to_string())?;
        let mut state = self.0.lock().expect("IoT Gateway state poisoned");
        state.events_seen = state.events_seen.saturating_add(1);
        if state.dedup.contains(&event.event_id) {
            state.duplicates_skipped = state.duplicates_skipped.saturating_add(1);
            return Ok(EnqueueOutcome::Duplicate);
        }
        if count_outbox(&state.config.outbox_dir()) >= state.config.storage.outbox_limit {
            return Err(format!(
                "outbox limit {} reached",
                state.config.storage.outbox_limit
            ));
        }

        let prefix = state.config.mqtt.topic_prefix.trim_matches('/');
        let event_topic = format!(
            "{prefix}/events/{}",
            event.event_type.replace('.', "/")
        );
        let payload = serde_json::to_string(event).map_err(|error| error.to_string())?;
        let event_message = OutboundMessage {
            id: Uuid::new_v4(),
            created_at: now_iso(),
            kind: "event".to_string(),
            topic: event_topic,
            qos: state.config.mqtt.qos,
            retain: state.config.mqtt.event_retain,
            payload: payload.clone(),
            source_event_id: Some(event.event_id),
        };
        let event_path = write_outbox_message(&state.config.outbox_dir(), &event_message)?;

        let mut additional_path = None;
        if let Some(subject) = &event.subject {
            let state_topic = format!(
                "{prefix}/state/{}/{}",
                plural_subject(&subject.subject_type),
                topic_segment(&subject.id)
            );
            let state_message = OutboundMessage {
                id: Uuid::new_v4(),
                created_at: now_iso(),
                kind: "state".to_string(),
                topic: state_topic,
                qos: state.config.mqtt.qos,
                retain: state.config.mqtt.state_retain,
                payload,
                source_event_id: Some(event.event_id),
            };
            match write_outbox_message(&state.config.outbox_dir(), &state_message) {
                Ok(path) => additional_path = Some(path),
                Err(error) => {
                    let _ = fs::remove_file(event_path);
                    return Err(error);
                }
            }
        }

        state.dedup.insert(event.event_id);
        state.dedup_order.push_back(event.event_id);
        while state.dedup_order.len() > state.config.storage.dedup_limit {
            if let Some(oldest) = state.dedup_order.pop_front() {
                state.dedup.remove(&oldest);
            }
        }
        persist_dedup(&state.config.dedup_path(), &state.dedup_order)?;
        state.events_enqueued = state.events_enqueued.saturating_add(1);
        push_event_locked(
            &mut state,
            "event.enqueued",
            json!({
                "event_id":event.event_id,
                "event_type":event.event_type,
                "state_message":additional_path.is_some()
            }),
        );
        Ok(EnqueueOutcome::Enqueued)
    }

    pub fn enqueue_manual_message(
        &self,
        topic: String,
        payload: String,
        qos: u8,
        retain: bool,
    ) -> Result<OutboundMessage, String> {
        let mut state = self.0.lock().expect("IoT Gateway state poisoned");
        let prefix = state.config.mqtt.topic_prefix.trim_matches('/');
        let below_prefix = topic
            .strip_prefix(prefix)
            .is_some_and(|suffix| suffix.starts_with('/') && suffix.len() > 1);
        if !below_prefix {
            return Err(format!("test topic must stay below {prefix}/"));
        }
        if topic.chars().any(|character| matches!(character, '#' | '+' | '\0')) {
            return Err("test publish topic must not contain MQTT wildcards or NUL".to_string());
        }
        if qos > 1 {
            return Err("Phase 3 supports QoS 0 or 1 only".to_string());
        }
        if count_outbox(&state.config.outbox_dir()) >= state.config.storage.outbox_limit {
            return Err("outbox limit reached".to_string());
        }
        let message = OutboundMessage {
            id: Uuid::new_v4(),
            created_at: now_iso(),
            kind: "manual_test".to_string(),
            topic,
            qos,
            retain,
            payload,
            source_event_id: None,
        };
        write_outbox_message(&state.config.outbox_dir(), &message)?;
        push_event_locked(
            &mut state,
            "mqtt.test_enqueued",
            json!({"message_id":message.id,"topic":message.topic}),
        );
        Ok(message)
    }

    pub fn observe_command(&self, topic: String, payload: Vec<u8>) -> Result<ObservedCommand, String> {
        let payload_text = String::from_utf8_lossy(&payload).to_string();
        let parsed = serde_json::from_slice::<Value>(&payload).ok();
        let command = ObservedCommand {
            command_id: Uuid::new_v4(),
            received_at: now_iso(),
            topic,
            payload: payload_text,
            valid_json: parsed.is_some(),
            parsed,
            status: "observed_only".to_string(),
            warning: "Phase 3 stores commands but never executes them; policy and acknowledgements follow in Phase 4"
                .to_string(),
        };
        let mut state = self.0.lock().expect("IoT Gateway state poisoned");
        append_json_line(&state.config.command_inbox_path(), &command)?;
        state.commands_observed = state.commands_observed.saturating_add(1);
        state.commands.push_back(command.clone());
        while state.commands.len() > state.config.server.history_limit {
            state.commands.pop_front();
        }
        push_event_locked(
            &mut state,
            "command.observed",
            json!({"command_id":command.command_id,"topic":command.topic,"executed":false}),
        );
        Ok(command)
    }

    pub fn outbox_entries(&self, limit: usize) -> Vec<OutboxEntrySummary> {
        let config = self.config();
        list_outbox(&config.outbox_dir(), limit)
    }

    pub fn next_outbox_message(&self) -> Option<(PathBuf, OutboundMessage)> {
        let config = self.config();
        let mut files = outbox_files(&config.outbox_dir());
        files.sort();
        files.into_iter().find_map(|path| {
            let raw = fs::read_to_string(&path).ok()?;
            match serde_json::from_str::<OutboundMessage>(&raw) {
                Ok(message) => Some((path, message)),
                Err(error) => {
                    self.record_bridge_event(
                        "outbox.invalid_file",
                        json!({"path":path,"error":error.to_string()}),
                    );
                    let _ = fs::rename(&path, path.with_extension("invalid"));
                    None
                }
            }
        })
    }

    pub fn complete_outbox_message(&self, path: &Path, message: &OutboundMessage) {
        match fs::remove_file(path) {
            Ok(()) => self.record_publish_ack(message),
            Err(error) => self.record_bridge_event(
                "outbox.remove_failed",
                json!({"path":path,"error":error.to_string()}),
            ),
        }
    }

    pub fn metrics(&self) -> String {
        let status = self.status();
        format!(
            concat!(
                "# HELP netcore_iot_gateway_mqtt_connected MQTT broker connection state.\n",
                "# TYPE netcore_iot_gateway_mqtt_connected gauge\n",
                "netcore_iot_gateway_mqtt_connected {}\n",
                "# HELP netcore_iot_gateway_outbox_pending Durable MQTT messages waiting for publication.\n",
                "# TYPE netcore_iot_gateway_outbox_pending gauge\n",
                "netcore_iot_gateway_outbox_pending {}\n",
                "# HELP netcore_iot_gateway_sources_healthy Healthy enabled event sources.\n",
                "# TYPE netcore_iot_gateway_sources_healthy gauge\n",
                "netcore_iot_gateway_sources_healthy {}\n",
                "# HELP netcore_iot_gateway_events_enqueued Canonical NetCore events added to the MQTT outbox.\n",
                "# TYPE netcore_iot_gateway_events_enqueued counter\n",
                "netcore_iot_gateway_events_enqueued {}\n",
                "# HELP netcore_iot_gateway_events_published MQTT messages acknowledged by the broker.\n",
                "# TYPE netcore_iot_gateway_events_published counter\n",
                "netcore_iot_gateway_events_published {}\n",
                "# HELP netcore_iot_gateway_commands_observed MQTT commands stored but not executed.\n",
                "# TYPE netcore_iot_gateway_commands_observed counter\n",
                "netcore_iot_gateway_commands_observed {}\n"
            ),
            u8::from(status.mqtt_connected),
            status.outbox_pending,
            status.sources_healthy,
            status.events_enqueued,
            status.events_published,
            status.commands_observed,
        )
    }

    pub fn record_bridge_event(&self, kind: &str, detail: Value) {
        let mut state = self.0.lock().expect("IoT Gateway state poisoned");
        push_event_locked(&mut state, kind, detail);
    }
}

fn push_event_locked(state: &mut GatewayState, kind: &str, detail: Value) {
    state.recent_events.push_back(BridgeEventRecord {
        timestamp: now_iso(),
        kind: kind.to_string(),
        detail,
    });
    while state.recent_events.len() > state.config.server.history_limit {
        state.recent_events.pop_front();
    }
}

fn now_iso() -> String {
    Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

fn plural_subject(value: &str) -> String {
    match value {
        "subscriber" => "subscribers".to_string(),
        "node" => "nodes".to_string(),
        "call" => "calls".to_string(),
        "group" => "groups".to_string(),
        "service" => "services".to_string(),
        "transfer" => "transfers".to_string(),
        "command" => "commands".to_string(),
        "sds" => "sds".to_string(),
        other if other.ends_with('s') => other.to_string(),
        other => format!("{other}s"),
    }
}

fn topic_segment(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            '/' | '+' | '#' | '\0' => '_',
            other => other,
        })
        .collect()
}

fn write_outbox_message(directory: &Path, message: &OutboundMessage) -> Result<PathBuf, String> {
    fs::create_dir_all(directory).map_err(|error| error.to_string())?;
    let stamp = Utc::now().timestamp_micros();
    let file_name = format!("{stamp:020}-{}.json", message.id);
    let path = directory.join(file_name);
    let temporary = path.with_extension("json.tmp");
    let raw = serde_json::to_vec_pretty(message).map_err(|error| error.to_string())?;
    fs::write(&temporary, raw).map_err(|error| error.to_string())?;
    fs::rename(&temporary, &path).map_err(|error| error.to_string())?;
    Ok(path)
}

fn outbox_files(directory: &Path) -> Vec<PathBuf> {
    fs::read_dir(directory)
        .ok()
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|value| value.to_str()) == Some("json"))
        .collect()
}

fn count_outbox(directory: &Path) -> usize {
    outbox_files(directory).len()
}

fn list_outbox(directory: &Path, limit: usize) -> Vec<OutboxEntrySummary> {
    let mut files = outbox_files(directory);
    files.sort();
    files
        .into_iter()
        .take(limit)
        .map(|path| {
            let metadata = fs::metadata(&path).ok();
            let file_name = path
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or_default()
                .to_string();
            match fs::read_to_string(&path)
                .map_err(|error| error.to_string())
                .and_then(|raw| serde_json::from_str::<OutboundMessage>(&raw).map_err(|error| error.to_string()))
            {
                Ok(message) => OutboxEntrySummary {
                    file_name,
                    id: Some(message.id),
                    created_at: Some(message.created_at),
                    kind: Some(message.kind),
                    topic: Some(message.topic),
                    qos: Some(message.qos),
                    retain: Some(message.retain),
                    bytes: metadata.map(|value| value.len()).unwrap_or_default(),
                    readable: true,
                    error: None,
                },
                Err(error) => OutboxEntrySummary {
                    file_name,
                    id: None,
                    created_at: None,
                    kind: None,
                    topic: None,
                    qos: None,
                    retain: None,
                    bytes: metadata.map(|value| value.len()).unwrap_or_default(),
                    readable: false,
                    error: Some(error),
                },
            }
        })
        .collect()
}

fn load_dedup(path: &Path, limit: usize) -> Result<(HashSet<Uuid>, VecDeque<Uuid>), Box<dyn std::error::Error>> {
    if !path.exists() {
        return Ok((HashSet::new(), VecDeque::new()));
    }
    let raw = fs::read_to_string(path)?;
    let values = serde_json::from_str::<Vec<Uuid>>(&raw)?;
    let order: VecDeque<_> = values.into_iter().rev().take(limit).collect::<Vec<_>>().into_iter().rev().collect();
    let set = order.iter().copied().collect();
    Ok((set, order))
}

fn persist_dedup(path: &Path, order: &VecDeque<Uuid>) -> Result<(), String> {
    let temporary = path.with_extension("json.tmp");
    let values: Vec<_> = order.iter().copied().collect();
    let raw = serde_json::to_vec(&values).map_err(|error| error.to_string())?;
    fs::write(&temporary, raw).map_err(|error| error.to_string())?;
    fs::rename(&temporary, path).map_err(|error| error.to_string())
}

fn append_json_line<T: serde::Serialize>(path: &Path, value: &T) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|error| error.to_string())?;
    let line = serde_json::to_string(value).map_err(|error| error.to_string())?;
    writeln!(file, "{line}").map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mqtt_topic_segments_are_safe() {
        assert_eq!(topic_segment("rack/01+#"), "rack_01__");
        assert_eq!(plural_subject("subscriber"), "subscribers");
        assert_eq!(plural_subject("sds"), "sds");
    }
}
