use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use chrono::{SecondsFormat, Utc};
use netcore_contracts::{
    CommandAck, CommandAckStatus, CommandPolicyDecision, CommandSource, CommandTarget,
    NetCoreCommand, NetCoreEvent, NETCORE_COMMAND_ACK_SCHEMA_V1, NETCORE_COMMAND_SCHEMA_V1,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::command::{VirtualDeviceState, evaluate_policy, execute_sandbox};
use crate::config::{CommandPolicyConfig, IotGatewayConfig};
use crate::model::{
    BridgeEventRecord, CommandRecord, GatewayStatus, OutboundMessage, OutboxEntrySummary,
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
    commands: VecDeque<CommandRecord>,
    command_ledger: HashMap<Uuid, CommandRecord>,
    command_order: VecDeque<Uuid>,
    virtual_devices: BTreeMap<String, VirtualDeviceState>,
    dedup: HashSet<Uuid>,
    dedup_order: VecDeque<Uuid>,
    events_seen: u64,
    events_enqueued: u64,
    events_published: u64,
    duplicates_skipped: u64,
    invalid_events: u64,
    commands_received: u64,
    commands_accepted: u64,
    commands_rejected: u64,
    commands_executed: u64,
    commands_failed: u64,
    command_duplicates: u64,
    commands_expired: u64,
    retained_commands_rejected: u64,
    command_dry_runs: u64,
    last_poll_at: Option<String>,
}

#[derive(Clone)]
pub struct SharedGateway(Arc<Mutex<GatewayState>>);

impl SharedGateway {
    pub fn new(config: IotGatewayConfig) -> Result<Self, Box<dyn std::error::Error>> {
        fs::create_dir_all(config.state_dir())?;
        fs::create_dir_all(config.outbox_dir())?;
        let (dedup, dedup_order) = load_dedup(&config.dedup_path(), config.storage.dedup_limit)?;
        let (command_ledger, command_order) = load_command_ledger(
            &config.command_ledger_path(),
            config.storage.command_ledger_limit,
        )?;
        let virtual_devices = load_virtual_devices(&config.virtual_state_path())?;
        let commands = command_order
            .iter()
            .filter_map(|command_id| command_ledger.get(command_id).cloned())
            .collect();
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
            commands,
            command_ledger,
            command_order,
            virtual_devices,
            dedup,
            dedup_order,
            events_seen: 0,
            events_enqueued: 0,
            events_published: 0,
            duplicates_skipped: 0,
            invalid_events: 0,
            commands_received: 0,
            commands_accepted: 0,
            commands_rejected: 0,
            commands_executed: 0,
            commands_failed: 0,
            command_duplicates: 0,
            commands_expired: 0,
            retained_commands_rejected: 0,
            command_dry_runs: 0,
            last_poll_at: None,
        };
        let gateway = Self(Arc::new(Mutex::new(state)));
        gateway.record_bridge_event(
            "iot_gateway.started",
            json!({
                "phase":4,
                "command_execution":gateway.config().commands.enabled,
                "execution_mode":gateway.config().commands.mode,
                "security_mode":"open_lab",
                "default_deny":gateway.config().commands.default_deny
            }),
        );
        Ok(gateway)
    }

    pub fn config(&self) -> IotGatewayConfig {
        self.0
            .lock()
            .expect("IoT Gateway state poisoned")
            .config
            .clone()
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
            phase: 4,
            started_at: state.started_at.clone(),
            security_mode: "open_lab",
            warning: "No login, no tokens and no TLS; commands are restricted to the OPEN-LAB sandbox",
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
            commands_observed: state.commands_received,
            commands_received: state.commands_received,
            commands_accepted: state.commands_accepted,
            commands_rejected: state.commands_rejected,
            commands_executed: state.commands_executed,
            commands_failed: state.commands_failed,
            command_duplicates: state.command_duplicates,
            commands_expired: state.commands_expired,
            retained_commands_rejected: state.retained_commands_rejected,
            command_dry_runs: state.command_dry_runs,
            command_execution_enabled: state.config.commands.enabled,
            command_execution_mode: state.config.commands.mode.clone(),
            command_policy_count: state
                .config
                .command_policies
                .iter()
                .filter(|policy| policy.enabled)
                .count(),
            virtual_devices: state.virtual_devices.len(),
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

    pub fn commands(&self, limit: usize) -> Vec<CommandRecord> {
        let state = self.0.lock().expect("IoT Gateway state poisoned");
        state
            .commands
            .iter()
            .rev()
            .take(limit.min(state.commands.len()))
            .cloned()
            .collect()
    }

    pub fn command(&self, command_id: Uuid) -> Option<CommandRecord> {
        self.0
            .lock()
            .expect("IoT Gateway state poisoned")
            .command_ledger
            .get(&command_id)
            .cloned()
    }

    pub fn command_policies(&self) -> Vec<CommandPolicyConfig> {
        self.0
            .lock()
            .expect("IoT Gateway state poisoned")
            .config
            .command_policies
            .clone()
    }

    pub fn virtual_devices(&self) -> Vec<VirtualDeviceState> {
        self.0
            .lock()
            .expect("IoT Gateway state poisoned")
            .virtual_devices
            .values()
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
            "virtual_relay_command".to_string(),
            format!("{prefix}/commands/virtual_relay/lab-relay-01/set"),
        );
        examples.insert(
            "command_ack".to_string(),
            format!("{prefix}/acks/<command-id>"),
        );
        examples.insert(
            "virtual_relay_state".to_string(),
            format!("{prefix}/state/virtual_relays/lab-relay-01"),
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
            command_schema: NETCORE_COMMAND_SCHEMA_V1,
            acknowledgement_pattern: format!("{prefix}/acks/<command-id>"),
            acknowledgement_schema: NETCORE_COMMAND_ACK_SCHEMA_V1,
            command_execution_enabled: state.config.commands.enabled,
            command_execution_mode: state.config.commands.mode.clone(),
            policy_count: state
                .config
                .command_policies
                .iter()
                .filter(|policy| policy.enabled)
                .count(),
            qos: state.config.mqtt.qos,
            event_retain: state.config.mqtt.event_retain,
            state_retain: state.config.mqtt.state_retain,
            examples,
        }
    }

    pub fn mqtt_connected(&self) -> bool {
        self.0
            .lock()
            .expect("IoT Gateway state poisoned")
            .mqtt_connected
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
            push_event_locked(&mut state, "source.recovered", json!({"source":source_id}));
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
        ensure_outbox_capacity(&state, if event.subject.is_some() { 2 } else { 1 })?;

        let prefix = state.config.mqtt.topic_prefix.trim_matches('/');
        let event_topic = format!("{prefix}/events/{}", event.event_type.replace('.', "/"));
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
        if topic
            .chars()
            .any(|character| matches!(character, '#' | '+' | '\0'))
        {
            return Err("test publish topic must not contain MQTT wildcards or NUL".to_string());
        }
        if qos > 1 {
            return Err("Phase 4 supports QoS 0 or 1 only".to_string());
        }
        ensure_outbox_capacity(&state, 1)?;
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

    pub fn process_command(
        &self,
        topic: String,
        payload: Vec<u8>,
        qos: u8,
        retained: bool,
    ) -> Result<CommandRecord, String> {
        let config = self.config();
        let payload_text = String::from_utf8_lossy(&payload).to_string();
        let parsed_value = serde_json::from_slice::<Value>(&payload).ok();
        let candidate_id = parsed_value
            .as_ref()
            .and_then(|value| value.get("command_id"))
            .and_then(Value::as_str)
            .and_then(|value| Uuid::parse_str(value).ok())
            .unwrap_or_else(Uuid::new_v4);
        let parsed_command = parsed_value
            .clone()
            .and_then(|value| serde_json::from_value::<NetCoreCommand>(value).ok());

        append_json_line(
            &config.command_inbox_path(),
            &json!({
                "received_at":now_iso(),
                "topic":topic,
                "qos":qos,
                "retained":retained,
                "payload":payload_text
            }),
        )?;

        let mut state = self.0.lock().expect("IoT Gateway state poisoned");
        state.commands_received = state.commands_received.saturating_add(1);

        if let Some(original) = state.command_ledger.get(&candidate_id).cloned() {
            state.command_duplicates = state.command_duplicates.saturating_add(1);
            let command = parsed_command
                .clone()
                .or_else(|| original.command.clone())
                .unwrap_or_else(|| synthetic_command(candidate_id, &topic));
            let record = CommandRecord {
                command_id: candidate_id,
                received_at: now_iso(),
                completed_at: Some(now_iso()),
                topic: topic.clone(),
                qos,
                retained,
                valid_json: parsed_value.is_some(),
                command: Some(command.clone()),
                status: CommandAckStatus::Duplicate,
                policy_id: original.policy_id.clone(),
                reason_code: Some("duplicate_command_id".to_string()),
                message: "command_id was already processed; command was not executed again"
                    .to_string(),
                result: json!({"original_status":original.status,"original_completed_at":original.completed_at}),
                duplicate_of: Some(candidate_id),
            };
            ensure_outbox_capacity(&state, 1)?;
            let mut ack = CommandAck::new(
                &command,
                CommandAckStatus::Duplicate,
                gateway_source(),
                record.message.clone(),
            )
            .with_reason("duplicate_command_id")
            .with_result(record.result.clone());
            if let Some(policy_id) = &record.policy_id {
                ack = ack.with_policy(policy_id.clone());
            }
            enqueue_ack_locked(&state, &ack)?;
            push_recent_command_locked(&mut state, record.clone());
            append_json_line(&state.config.command_audit_path(), &record)?;
            push_event_locked(
                &mut state,
                "command.duplicate",
                json!({"command_id":candidate_id,"topic":topic}),
            );
            return Ok(record);
        }

        let command = match parsed_command {
            Some(command) => command,
            None => {
                let reason = if parsed_value.is_some() {
                    "JSON does not match netcore-command-v1"
                } else {
                    "payload is not valid JSON"
                };
                return reject_command_locked(
                    &mut state,
                    synthetic_command(candidate_id, &topic),
                    topic,
                    qos,
                    retained,
                    parsed_value.is_some(),
                    "invalid_command",
                    reason,
                    None,
                    false,
                );
            }
        };

        if let Err(error) = command.validate() {
            return reject_command_locked(
                &mut state,
                command,
                topic,
                qos,
                retained,
                true,
                "schema_validation_failed",
                &error.to_string(),
                None,
                false,
            );
        }

        if !state.config.commands.enabled {
            return reject_command_locked(
                &mut state,
                command,
                topic,
                qos,
                retained,
                true,
                "command_execution_disabled",
                "commands.enabled is false",
                None,
                false,
            );
        }

        if retained && !state.config.commands.allow_retained {
            state.retained_commands_rejected =
                state.retained_commands_rejected.saturating_add(1);
            return reject_command_locked(
                &mut state,
                command,
                topic,
                qos,
                retained,
                true,
                "retained_command_rejected",
                "retained MQTT commands are disabled to prevent replay after reconnect",
                None,
                false,
            );
        }

        let now = Utc::now();
        if command.expires_at <= now {
            state.commands_expired = state.commands_expired.saturating_add(1);
            return reject_command_locked(
                &mut state,
                command,
                topic,
                qos,
                retained,
                true,
                "command_expired",
                "expires_at is in the past",
                None,
                false,
            );
        }
        let future_skew = command.requested_at.signed_duration_since(now).num_seconds();
        if future_skew > state.config.commands.max_future_skew_secs as i64 {
            return reject_command_locked(
                &mut state,
                command,
                topic,
                qos,
                retained,
                true,
                "requested_at_in_future",
                "requested_at exceeds the configured future clock-skew tolerance",
                None,
                false,
            );
        }

        let evaluation = evaluate_policy(&state.config, &command);
        if evaluation.decision == CommandPolicyDecision::Deny {
            return reject_command_locked(
                &mut state,
                command,
                topic,
                qos,
                retained,
                true,
                &evaluation.reason_code,
                &evaluation.message,
                evaluation.policy_id.clone(),
                false,
            );
        }

        let lifecycle_acks = if state.config.commands.publish_lifecycle_acks { 2 } else { 0 };
        // accepted + executing + terminal + optional retained virtual state
        ensure_outbox_capacity(&state, lifecycle_acks + 2)?;
        state.commands_accepted = state.commands_accepted.saturating_add(1);
        if command.dry_run {
            state.command_dry_runs = state.command_dry_runs.saturating_add(1);
        }
        if state.config.commands.publish_lifecycle_acks {
            let mut accepted = CommandAck::new(
                &command,
                CommandAckStatus::Accepted,
                gateway_source(),
                "command passed schema, replay and policy checks",
            )
            .with_reason("accepted");
            if let Some(policy_id) = &evaluation.policy_id {
                accepted = accepted.with_policy(policy_id.clone());
            }
            enqueue_ack_locked(&state, &accepted)?;

            let mut executing = CommandAck::new(
                &command,
                CommandAckStatus::Executing,
                gateway_source(),
                if command.dry_run {
                    "validating sandbox execution in dry-run mode"
                } else {
                    "executing command in OPEN-LAB sandbox"
                },
            )
            .with_reason("executing");
            if let Some(policy_id) = &evaluation.policy_id {
                executing = executing.with_policy(policy_id.clone());
            }
            enqueue_ack_locked(&state, &executing)?;
        }

        let execution = execute_sandbox(&command, &mut state.virtual_devices);
        let completed_at = now_iso();
        match execution {
            Ok(execution) => {
                if let Some(update) = &execution.state_update {
                    persist_virtual_devices(
                        &state.config.virtual_state_path(),
                        &state.virtual_devices,
                    )?;
                    enqueue_virtual_state_locked(&state, update)?;
                }
                if !command.dry_run {
                    state.commands_executed = state.commands_executed.saturating_add(1);
                }
                let message = if command.dry_run {
                    "command passed sandbox dry-run validation"
                } else {
                    "command completed in OPEN-LAB sandbox"
                };
                let record = CommandRecord {
                    command_id: command.command_id,
                    received_at: now_iso(),
                    completed_at: Some(completed_at),
                    topic,
                    qos,
                    retained,
                    valid_json: true,
                    command: Some(command.clone()),
                    status: CommandAckStatus::Succeeded,
                    policy_id: evaluation.policy_id.clone(),
                    reason_code: Some(if command.dry_run {
                        "dry_run_succeeded"
                    } else {
                        "executed"
                    }
                    .to_string()),
                    message: message.to_string(),
                    result: execution.result.clone(),
                    duplicate_of: None,
                };
                let mut ack = CommandAck::new(
                    &command,
                    CommandAckStatus::Succeeded,
                    gateway_source(),
                    message,
                )
                .with_reason(record.reason_code.clone().unwrap_or_default())
                .with_result(execution.result);
                if let Some(policy_id) = &evaluation.policy_id {
                    ack = ack.with_policy(policy_id.clone());
                }
                enqueue_ack_locked(&state, &ack)?;
                store_terminal_command_locked(&mut state, record.clone())?;
                push_event_locked(
                    &mut state,
                    "command.succeeded",
                    json!({
                        "command_id":record.command_id,
                        "command_type":command.command_type,
                        "target":command.target,
                        "dry_run":command.dry_run,
                        "policy_id":record.policy_id
                    }),
                );
                Ok(record)
            }
            Err(error) => {
                state.commands_failed = state.commands_failed.saturating_add(1);
                let record = CommandRecord {
                    command_id: command.command_id,
                    received_at: now_iso(),
                    completed_at: Some(completed_at),
                    topic,
                    qos,
                    retained,
                    valid_json: true,
                    command: Some(command.clone()),
                    status: CommandAckStatus::Failed,
                    policy_id: evaluation.policy_id.clone(),
                    reason_code: Some("executor_failed".to_string()),
                    message: error.clone(),
                    result: Value::Null,
                    duplicate_of: None,
                };
                let mut ack = CommandAck::new(
                    &command,
                    CommandAckStatus::Failed,
                    gateway_source(),
                    error,
                )
                .with_reason("executor_failed");
                if let Some(policy_id) = &evaluation.policy_id {
                    ack = ack.with_policy(policy_id.clone());
                }
                enqueue_ack_locked(&state, &ack)?;
                store_terminal_command_locked(&mut state, record.clone())?;
                push_event_locked(
                    &mut state,
                    "command.failed",
                    json!({"command_id":record.command_id,"reason":"executor_failed"}),
                );
                Ok(record)
            }
        }
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
                "# HELP netcore_iot_gateway_commands_received MQTT commands received.\n",
                "# TYPE netcore_iot_gateway_commands_received counter\n",
                "netcore_iot_gateway_commands_received {}\n",
                "# HELP netcore_iot_gateway_commands_rejected Commands rejected by validation, replay or policy checks.\n",
                "# TYPE netcore_iot_gateway_commands_rejected counter\n",
                "netcore_iot_gateway_commands_rejected {}\n",
                "# HELP netcore_iot_gateway_commands_executed Commands completed by the OPEN-LAB sandbox executor.\n",
                "# TYPE netcore_iot_gateway_commands_executed counter\n",
                "netcore_iot_gateway_commands_executed {}\n",
                "# HELP netcore_iot_gateway_command_duplicates Duplicate command identifiers suppressed.\n",
                "# TYPE netcore_iot_gateway_command_duplicates counter\n",
                "netcore_iot_gateway_command_duplicates {}\n",
                "# HELP netcore_iot_gateway_virtual_devices Persistent virtual sandbox devices.\n",
                "# TYPE netcore_iot_gateway_virtual_devices gauge\n",
                "netcore_iot_gateway_virtual_devices {}\n"
            ),
            u8::from(status.mqtt_connected),
            status.outbox_pending,
            status.sources_healthy,
            status.events_enqueued,
            status.events_published,
            status.commands_received,
            status.commands_rejected,
            status.commands_executed,
            status.command_duplicates,
            status.virtual_devices,
        )
    }

    pub fn record_bridge_event(&self, kind: &str, detail: Value) {
        let mut state = self.0.lock().expect("IoT Gateway state poisoned");
        push_event_locked(&mut state, kind, detail);
    }
}

fn reject_command_locked(
    state: &mut GatewayState,
    command: NetCoreCommand,
    topic: String,
    qos: u8,
    retained: bool,
    valid_json: bool,
    reason_code: &str,
    message: &str,
    policy_id: Option<String>,
    duplicate: bool,
) -> Result<CommandRecord, String> {
    ensure_outbox_capacity(state, 1)?;
    state.commands_rejected = state.commands_rejected.saturating_add(1);
    let record = CommandRecord {
        command_id: command.command_id,
        received_at: now_iso(),
        completed_at: Some(now_iso()),
        topic,
        qos,
        retained,
        valid_json,
        command: Some(command.clone()),
        status: if duplicate {
            CommandAckStatus::Duplicate
        } else {
            CommandAckStatus::Rejected
        },
        policy_id: policy_id.clone(),
        reason_code: Some(reason_code.to_string()),
        message: message.to_string(),
        result: Value::Null,
        duplicate_of: duplicate.then_some(command.command_id),
    };
    let mut ack = CommandAck::new(
        &command,
        record.status,
        gateway_source(),
        message.to_string(),
    )
    .with_reason(reason_code);
    if let Some(policy_id) = &policy_id {
        ack = ack.with_policy(policy_id.clone());
    }
    enqueue_ack_locked(state, &ack)?;
    store_terminal_command_locked(state, record.clone())?;
    push_event_locked(
        state,
        "command.rejected",
        json!({
            "command_id":command.command_id,
            "command_type":command.command_type,
            "reason_code":reason_code,
            "policy_id":policy_id
        }),
    );
    Ok(record)
}

fn store_terminal_command_locked(
    state: &mut GatewayState,
    record: CommandRecord,
) -> Result<(), String> {
    state.command_ledger.insert(record.command_id, record.clone());
    state.command_order.push_back(record.command_id);
    while state.command_order.len() > state.config.storage.command_ledger_limit {
        if let Some(oldest) = state.command_order.pop_front() {
            state.command_ledger.remove(&oldest);
        }
    }
    push_recent_command_locked(state, record.clone());
    persist_command_ledger(
        &state.config.command_ledger_path(),
        &state.command_ledger,
        &state.command_order,
    )?;
    append_json_line(&state.config.command_audit_path(), &record)
}

fn push_recent_command_locked(state: &mut GatewayState, record: CommandRecord) {
    state.commands.push_back(record);
    while state.commands.len() > state.config.server.history_limit {
        state.commands.pop_front();
    }
}

fn enqueue_ack_locked(state: &GatewayState, ack: &CommandAck) -> Result<PathBuf, String> {
    let prefix = state.config.mqtt.topic_prefix.trim_matches('/');
    let message = OutboundMessage {
        id: Uuid::new_v4(),
        created_at: now_iso(),
        kind: "command_ack".to_string(),
        topic: format!("{prefix}/acks/{}", ack.command_id),
        qos: state.config.commands.ack_qos,
        retain: state.config.commands.ack_retain,
        payload: serde_json::to_string(ack).map_err(|error| error.to_string())?,
        source_event_id: None,
    };
    write_outbox_message(&state.config.outbox_dir(), &message)
}

fn enqueue_virtual_state_locked(
    state: &GatewayState,
    update: &VirtualDeviceState,
) -> Result<PathBuf, String> {
    let prefix = state.config.mqtt.topic_prefix.trim_matches('/');
    let message = OutboundMessage {
        id: Uuid::new_v4(),
        created_at: now_iso(),
        kind: "virtual_device_state".to_string(),
        topic: format!(
            "{prefix}/state/{}/{}",
            plural_subject(&update.device_type),
            topic_segment(&update.id)
        ),
        qos: state.config.mqtt.qos,
        retain: true,
        payload: serde_json::to_string(update).map_err(|error| error.to_string())?,
        source_event_id: None,
    };
    write_outbox_message(&state.config.outbox_dir(), &message)
}

fn ensure_outbox_capacity(state: &GatewayState, additional: usize) -> Result<(), String> {
    let pending = count_outbox(&state.config.outbox_dir());
    if pending.saturating_add(additional) > state.config.storage.outbox_limit {
        Err(format!(
            "outbox limit {} would be exceeded",
            state.config.storage.outbox_limit
        ))
    } else {
        Ok(())
    }
}

fn synthetic_command(command_id: Uuid, topic: &str) -> NetCoreCommand {
    let mut command = NetCoreCommand::new(
        "command.invalid",
        CommandSource::new("mqtt-openlab", "unparsed-publisher"),
        CommandTarget::new("command", topic_segment(topic)),
        Value::Null,
        30,
    );
    command.command_id = command_id;
    command
}

fn gateway_source() -> CommandSource {
    let instance = fs::read_to_string("/etc/hostname")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "iot-gateway".to_string());
    CommandSource::new("netcore-iot-gateway", instance)
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
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
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
        "virtual_relay" => "virtual_relays".to_string(),
        "virtual_light" => "virtual_lights".to_string(),
        "virtual_button" => "virtual_buttons".to_string(),
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
                .and_then(|raw| {
                    serde_json::from_str::<OutboundMessage>(&raw)
                        .map_err(|error| error.to_string())
                }) {
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

fn load_dedup(
    path: &Path,
    limit: usize,
) -> Result<(HashSet<Uuid>, VecDeque<Uuid>), Box<dyn std::error::Error>> {
    if !path.exists() {
        return Ok((HashSet::new(), VecDeque::new()));
    }
    let raw = fs::read_to_string(path)?;
    let values = serde_json::from_str::<Vec<Uuid>>(&raw)?;
    let order: VecDeque<_> = values
        .into_iter()
        .rev()
        .take(limit)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
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

fn load_command_ledger(
    path: &Path,
    limit: usize,
) -> Result<(HashMap<Uuid, CommandRecord>, VecDeque<Uuid>), Box<dyn std::error::Error>> {
    if !path.exists() {
        return Ok((HashMap::new(), VecDeque::new()));
    }
    let raw = fs::read_to_string(path)?;
    let records = serde_json::from_str::<Vec<CommandRecord>>(&raw)?;
    let records = records
        .into_iter()
        .rev()
        .take(limit)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>();
    let order = records.iter().map(|record| record.command_id).collect();
    let ledger = records
        .into_iter()
        .map(|record| (record.command_id, record))
        .collect();
    Ok((ledger, order))
}

fn persist_command_ledger(
    path: &Path,
    ledger: &HashMap<Uuid, CommandRecord>,
    order: &VecDeque<Uuid>,
) -> Result<(), String> {
    let temporary = path.with_extension("json.tmp");
    let records = order
        .iter()
        .filter_map(|command_id| ledger.get(command_id))
        .collect::<Vec<_>>();
    let raw = serde_json::to_vec_pretty(&records).map_err(|error| error.to_string())?;
    fs::write(&temporary, raw).map_err(|error| error.to_string())?;
    fs::rename(&temporary, path).map_err(|error| error.to_string())
}

fn load_virtual_devices(
    path: &Path,
) -> Result<BTreeMap<String, VirtualDeviceState>, Box<dyn std::error::Error>> {
    if !path.exists() {
        return Ok(BTreeMap::new());
    }
    let raw = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&raw)?)
}

fn persist_virtual_devices(
    path: &Path,
    devices: &BTreeMap<String, VirtualDeviceState>,
) -> Result<(), String> {
    let temporary = path.with_extension("json.tmp");
    let raw = serde_json::to_vec_pretty(devices).map_err(|error| error.to_string())?;
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
        assert_eq!(plural_subject("virtual_relay"), "virtual_relays");
    }

    #[test]
    fn synthetic_command_uses_the_candidate_id() {
        let command_id = Uuid::new_v4();
        let command = synthetic_command(command_id, "netcore/v1/commands/broken");
        assert_eq!(command.command_id, command_id);
        command.validate().unwrap();
    }
}
