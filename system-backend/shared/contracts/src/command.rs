// NETCORE-KOMMENTAR – Was: Definiert das gemeinsame Command-, Ack- und Policy-Grundmodell.
// NETCORE-KOMMENTAR – Warum: MQTT, HTTP und spätere Hardware-Adapter benötigen dieselben validierten Befehle und Quittungen.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

pub const NETCORE_COMMAND_SCHEMA_V1: &str = "netcore-command-v1";
pub const NETCORE_COMMAND_ACK_SCHEMA_V1: &str = "netcore-command-ack-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandSource {
    pub service: String,
    pub instance: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
}

impl CommandSource {
    pub fn new(service: impl Into<String>, instance: impl Into<String>) -> Self {
        Self {
            service: service.into(),
            instance: instance.into(),
            actor: None,
        }
    }

    pub fn with_actor(mut self, actor: impl Into<String>) -> Self {
        self.actor = Some(actor.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandTarget {
    #[serde(rename = "type")]
    pub target_type: String,
    pub id: String,
}

impl CommandTarget {
    pub fn new(target_type: impl Into<String>, id: impl Into<String>) -> Self {
        Self {
            target_type: target_type.into(),
            id: id.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandAckStatus {
    Received,
    Accepted,
    Rejected,
    Executing,
    Succeeded,
    Failed,
    Duplicate,
}

impl CommandAckStatus {
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Rejected | Self::Succeeded | Self::Failed | Self::Duplicate
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandPolicyDecision {
    Allow,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetCoreCommand {
    pub schema: String,
    pub command_id: Uuid,
    pub command_type: String,
    pub source: CommandSource,
    pub requested_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub target: CommandTarget,
    #[serde(default)]
    pub payload: Value,
    #[serde(default)]
    pub dry_run: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causation_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub labels: BTreeMap<String, String>,
}

impl NetCoreCommand {
    pub fn new(
        command_type: impl Into<String>,
        source: CommandSource,
        target: CommandTarget,
        payload: Value,
        ttl_seconds: i64,
    ) -> Self {
        let requested_at = Utc::now();
        Self {
            schema: NETCORE_COMMAND_SCHEMA_V1.to_owned(),
            command_id: Uuid::new_v4(),
            command_type: command_type.into(),
            source,
            requested_at,
            expires_at: requested_at + chrono::Duration::seconds(ttl_seconds.max(1)),
            target,
            payload,
            dry_run: false,
            correlation_id: None,
            causation_id: None,
            idempotency_key: None,
            labels: BTreeMap::new(),
        }
    }

    pub fn validate(&self) -> Result<(), CommandValidationError> {
        if self.schema != NETCORE_COMMAND_SCHEMA_V1 {
            return Err(CommandValidationError::InvalidSchema(self.schema.clone()));
        }
        if self.command_id.is_nil() {
            return Err(CommandValidationError::NilCommandId);
        }
        if !is_command_type(&self.command_type) {
            return Err(CommandValidationError::InvalidCommandType(
                self.command_type.clone(),
            ));
        }
        if !is_component_name(&self.source.service, 128) {
            return Err(CommandValidationError::InvalidSourceService(
                self.source.service.clone(),
            ));
        }
        if !is_component_name(&self.source.instance, 128) {
            return Err(CommandValidationError::InvalidSourceInstance(
                self.source.instance.clone(),
            ));
        }
        if let Some(actor) = &self.source.actor {
            if !is_free_identifier(actor, 160) {
                return Err(CommandValidationError::InvalidSourceActor(actor.clone()));
            }
        }
        if !is_lower_identifier(&self.target.target_type)
            || self.target.target_type.len() > 64
        {
            return Err(CommandValidationError::InvalidTargetType(
                self.target.target_type.clone(),
            ));
        }
        if !is_free_identifier(&self.target.id, 256) {
            return Err(CommandValidationError::InvalidTargetId(
                self.target.id.clone(),
            ));
        }
        if self.expires_at <= self.requested_at {
            return Err(CommandValidationError::InvalidTimeWindow);
        }
        if let Some(value) = &self.idempotency_key {
            if !is_free_identifier(value, 320) {
                return Err(CommandValidationError::InvalidIdempotencyKey(
                    value.clone(),
                ));
            }
        }
        for (key, value) in &self.labels {
            if !is_label_key(key) || value.len() > 256 {
                return Err(CommandValidationError::InvalidLabel(key.clone()));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandAck {
    pub schema: String,
    pub ack_id: Uuid,
    pub command_id: Uuid,
    pub command_type: String,
    pub status: CommandAckStatus,
    pub source: CommandSource,
    pub timestamp: DateTime<Utc>,
    pub target: CommandTarget,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    pub message: String,
    #[serde(default)]
    pub result: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<Uuid>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub labels: BTreeMap<String, String>,
}

impl CommandAck {
    pub fn new(
        command: &NetCoreCommand,
        status: CommandAckStatus,
        source: CommandSource,
        message: impl Into<String>,
    ) -> Self {
        Self {
            schema: NETCORE_COMMAND_ACK_SCHEMA_V1.to_owned(),
            ack_id: Uuid::new_v4(),
            command_id: command.command_id,
            command_type: command.command_type.clone(),
            status,
            source,
            timestamp: Utc::now(),
            target: command.target.clone(),
            policy_id: None,
            reason_code: None,
            message: message.into(),
            result: Value::Null,
            correlation_id: command.correlation_id,
            labels: BTreeMap::new(),
        }
    }

    pub fn with_policy(mut self, policy_id: impl Into<String>) -> Self {
        self.policy_id = Some(policy_id.into());
        self
    }

    pub fn with_reason(mut self, reason_code: impl Into<String>) -> Self {
        self.reason_code = Some(reason_code.into());
        self
    }

    pub fn with_result(mut self, result: Value) -> Self {
        self.result = result;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandValidationError {
    InvalidSchema(String),
    NilCommandId,
    InvalidCommandType(String),
    InvalidSourceService(String),
    InvalidSourceInstance(String),
    InvalidSourceActor(String),
    InvalidTargetType(String),
    InvalidTargetId(String),
    InvalidTimeWindow,
    InvalidIdempotencyKey(String),
    InvalidLabel(String),
}

impl fmt::Display for CommandValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSchema(value) => write!(formatter, "unsupported command schema: {value}"),
            Self::NilCommandId => formatter.write_str("command_id must not be nil"),
            Self::InvalidCommandType(value) => write!(formatter, "invalid command_type: {value}"),
            Self::InvalidSourceService(value) => write!(formatter, "invalid source.service: {value}"),
            Self::InvalidSourceInstance(value) => write!(formatter, "invalid source.instance: {value}"),
            Self::InvalidSourceActor(value) => write!(formatter, "invalid source.actor: {value}"),
            Self::InvalidTargetType(value) => write!(formatter, "invalid target.type: {value}"),
            Self::InvalidTargetId(value) => write!(formatter, "invalid target.id: {value}"),
            Self::InvalidTimeWindow => formatter.write_str("expires_at must be after requested_at"),
            Self::InvalidIdempotencyKey(value) => write!(formatter, "invalid idempotency_key: {value}"),
            Self::InvalidLabel(value) => write!(formatter, "invalid command label: {value}"),
        }
    }
}

impl Error for CommandValidationError {}

pub fn is_command_type(value: &str) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn command_roundtrips_and_validates() {
        let command = NetCoreCommand::new(
            "virtual.relay.set",
            CommandSource::new("openlab-cli", "shell-01").with_actor("jan"),
            CommandTarget::new("virtual_relay", "lab-relay-01"),
            json!({"state":true}),
            30,
        );
        command.validate().unwrap();
        let encoded = serde_json::to_vec(&command).unwrap();
        let decoded: NetCoreCommand = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(decoded, command);
    }

    #[test]
    fn invalid_time_window_is_rejected() {
        let mut command = NetCoreCommand::new(
            "virtual.relay.set",
            CommandSource::new("test", "test-1"),
            CommandTarget::new("virtual_relay", "lab-relay-01"),
            json!({"state":true}),
            30,
        );
        command.expires_at = command.requested_at;
        assert!(command.validate().is_err());
    }
}
