use std::collections::HashSet;
use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub const OPEN_LAB_MODE: &str = "open_lab";
pub const OPEN_LAB_SANDBOX_MODE: &str = "open_lab_sandbox";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IotGatewayConfig {
    pub server: ServerConfig,
    pub security: SecurityConfig,
    pub mqtt: MqttConfig,
    pub commands: CommandConfig,
    pub storage: StorageConfig,
    pub polling: PollingConfig,
    pub sources: Vec<EventSourceConfig>,
    pub command_policies: Vec<CommandPolicyConfig>,
}

impl Default for IotGatewayConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            security: SecurityConfig::default(),
            mqtt: MqttConfig::default(),
            commands: CommandConfig::default(),
            storage: StorageConfig::default(),
            polling: PollingConfig::default(),
            sources: default_sources(),
            command_policies: default_command_policies(),
        }
    }
}

impl IotGatewayConfig {
    pub fn load(path: Option<&Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let config = match path {
            Some(path) => {
                let raw = fs::read_to_string(path)?;
                toml::from_str::<Self>(&raw)?
            }
            None => Self::default(),
        };
        config.validate().map_err(std::io::Error::other)?;
        Ok(config)
    }

    pub fn apply_bind_override(&mut self, bind: Option<SocketAddr>) -> Result<(), String> {
        if let Some(bind) = bind {
            self.server.bind = bind;
        }
        self.validate()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.security.mode != OPEN_LAB_MODE {
            return Err("Phase 4 supports only security.mode = open_lab".to_string());
        }
        if self.mqtt.host.trim().is_empty() {
            return Err("mqtt.host must not be empty".to_string());
        }
        if self.mqtt.port == 0 {
            return Err("mqtt.port must be greater than zero".to_string());
        }
        if self.mqtt.client_id.trim().is_empty()
            || self.mqtt.client_id.len() > 128
            || self.mqtt.client_id.contains('\0')
        {
            return Err("mqtt.client_id must contain 1..128 valid UTF-8 characters".to_string());
        }
        if self.mqtt.keep_alive_secs == 0 {
            return Err("mqtt.keep_alive_secs must be greater than zero".to_string());
        }
        if self.mqtt.publish_timeout_secs == 0 {
            return Err("mqtt.publish_timeout_secs must be greater than zero".to_string());
        }
        if self.mqtt.qos > 1 {
            return Err("Phase 4 supports MQTT QoS 0 or 1 only".to_string());
        }
        if self.mqtt.execute_commands {
            return Err(
                "mqtt.execute_commands is deprecated and must remain false; use commands.enabled"
                    .to_string(),
            );
        }
        validate_topic_prefix(&self.mqtt.topic_prefix)?;

        if self.commands.mode != OPEN_LAB_SANDBOX_MODE {
            return Err(format!(
                "Phase 4 supports only commands.mode = {OPEN_LAB_SANDBOX_MODE}"
            ));
        }
        if self.commands.default_ttl_secs == 0 {
            return Err("commands.default_ttl_secs must be greater than zero".to_string());
        }
        if self.commands.max_ttl_secs < self.commands.default_ttl_secs {
            return Err(
                "commands.max_ttl_secs must be greater than or equal to default_ttl_secs"
                    .to_string(),
            );
        }
        if self.commands.max_ttl_secs > 86_400 {
            return Err("commands.max_ttl_secs must not exceed 86400".to_string());
        }
        if self.commands.max_future_skew_secs > 3_600 {
            return Err("commands.max_future_skew_secs must not exceed 3600".to_string());
        }
        if self.commands.ack_qos > 1 {
            return Err("commands.ack_qos must be 0 or 1".to_string());
        }

        if self.polling.interval_ms < 250 {
            return Err("polling.interval_ms must be at least 250".to_string());
        }
        if self.polling.batch_limit == 0 || self.polling.batch_limit > 10_000 {
            return Err("polling.batch_limit must be within 1..10000".to_string());
        }
        if self.storage.dedup_limit < 100 {
            return Err("storage.dedup_limit must be at least 100".to_string());
        }
        if self.storage.outbox_limit < 100 {
            return Err("storage.outbox_limit must be at least 100".to_string());
        }
        if self.storage.command_ledger_limit < 100 {
            return Err("storage.command_ledger_limit must be at least 100".to_string());
        }

        let mut source_ids = HashSet::new();
        for source in &self.sources {
            if source.id.trim().is_empty() {
                return Err("source.id must not be empty".to_string());
            }
            if !source_ids.insert(source.id.as_str()) {
                return Err(format!("duplicate source.id: {}", source.id));
            }
            if source.enabled
                && !source.url.starts_with("http://")
                && !source.url.starts_with("https://")
            {
                return Err(format!(
                    "source {} must use an http:// or https:// URL",
                    source.id
                ));
            }
        }

        let mut policy_ids = HashSet::new();
        for policy in &self.command_policies {
            if policy.id.trim().is_empty() || policy.id.len() > 128 {
                return Err("command policy id must contain 1..128 characters".to_string());
            }
            if !policy_ids.insert(policy.id.as_str()) {
                return Err(format!("duplicate command policy id: {}", policy.id));
            }
            if policy.command_types.is_empty() {
                return Err(format!(
                    "command policy {} must contain at least one command_type",
                    policy.id
                ));
            }
            if policy
                .command_types
                .iter()
                .any(|value| value != "*" && !netcore_contracts::is_command_type(value))
            {
                return Err(format!(
                    "command policy {} contains an invalid command_type",
                    policy.id
                ));
            }
            if policy.target_types.iter().any(|value| {
                value != "*"
                    && (value.is_empty()
                        || value.len() > 64
                        || !value.bytes().all(|byte| {
                            byte.is_ascii_lowercase()
                                || byte.is_ascii_digit()
                                || byte == b'_'
                        }))
            }) {
                return Err(format!(
                    "command policy {} contains an invalid target_type",
                    policy.id
                ));
            }
            if policy
                .target_prefixes
                .iter()
                .any(|value| value.is_empty() || value.len() > 256)
            {
                return Err(format!(
                    "command policy {} contains an invalid target_prefix",
                    policy.id
                ));
            }
            if let Some(max_ttl_secs) = policy.max_ttl_secs {
                if max_ttl_secs == 0 || max_ttl_secs > self.commands.max_ttl_secs {
                    return Err(format!(
                        "command policy {} max_ttl_secs exceeds the global limit",
                        policy.id
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn state_dir(&self) -> PathBuf {
        PathBuf::from(&self.storage.state_dir)
    }

    pub fn outbox_dir(&self) -> PathBuf {
        self.state_dir().join(&self.storage.outbox_dir)
    }

    pub fn dedup_path(&self) -> PathBuf {
        self.state_dir().join(&self.storage.dedup_file)
    }

    pub fn command_inbox_path(&self) -> PathBuf {
        self.state_dir().join(&self.storage.command_inbox_file)
    }

    pub fn command_ledger_path(&self) -> PathBuf {
        self.state_dir().join(&self.storage.command_ledger_file)
    }

    pub fn command_audit_path(&self) -> PathBuf {
        self.state_dir().join(&self.storage.command_audit_file)
    }

    pub fn virtual_state_path(&self) -> PathBuf {
        self.state_dir().join(&self.storage.virtual_state_file)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    pub bind: SocketAddr,
    pub history_limit: usize,
    pub max_body_bytes: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind: "0.0.0.0:8240".parse().expect("valid IoT Gateway bind"),
            history_limit: 1_000,
            max_body_bytes: 1_048_576,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SecurityConfig {
    pub mode: String,
    pub allow_remote_management: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            mode: OPEN_LAB_MODE.to_string(),
            allow_remote_management: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MqttConfig {
    pub host: String,
    pub port: u16,
    pub client_id: String,
    pub topic_prefix: String,
    pub keep_alive_secs: u16,
    pub clean_session: bool,
    pub reconnect_secs: u64,
    pub publish_timeout_secs: u64,
    pub qos: u8,
    pub event_retain: bool,
    pub state_retain: bool,
    pub observe_commands: bool,
    /// Deprecated Phase-3 compatibility switch. Phase 4 uses `commands.enabled`.
    pub execute_commands: bool,
}

impl Default for MqttConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 1883,
            client_id: "netcore-iot-gateway".to_string(),
            topic_prefix: "netcore/v1".to_string(),
            keep_alive_secs: 30,
            clean_session: false,
            reconnect_secs: 3,
            publish_timeout_secs: 8,
            qos: 1,
            event_retain: false,
            state_retain: true,
            observe_commands: true,
            execute_commands: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CommandConfig {
    pub enabled: bool,
    pub mode: String,
    pub default_deny: bool,
    pub allow_retained: bool,
    pub default_ttl_secs: u64,
    pub max_ttl_secs: u64,
    pub max_future_skew_secs: u64,
    pub publish_lifecycle_acks: bool,
    pub ack_qos: u8,
    pub ack_retain: bool,
}

impl Default for CommandConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            mode: OPEN_LAB_SANDBOX_MODE.to_string(),
            default_deny: true,
            allow_retained: false,
            default_ttl_secs: 30,
            max_ttl_secs: 300,
            max_future_skew_secs: 30,
            publish_lifecycle_acks: true,
            ack_qos: 1,
            ack_retain: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyEffect {
    Allow,
    Deny,
}

impl Default for PolicyEffect {
    fn default() -> Self {
        Self::Deny
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CommandPolicyConfig {
    pub id: String,
    pub enabled: bool,
    pub effect: PolicyEffect,
    pub command_types: Vec<String>,
    pub target_types: Vec<String>,
    pub target_prefixes: Vec<String>,
    pub max_ttl_secs: Option<u64>,
    pub allow_dry_run: bool,
}

impl Default for CommandPolicyConfig {
    fn default() -> Self {
        Self {
            id: String::new(),
            enabled: true,
            effect: PolicyEffect::Deny,
            command_types: vec!["*".to_string()],
            target_types: vec!["*".to_string()],
            target_prefixes: Vec::new(),
            max_ttl_secs: None,
            allow_dry_run: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct StorageConfig {
    pub state_dir: String,
    pub outbox_dir: String,
    pub dedup_file: String,
    pub command_inbox_file: String,
    pub command_ledger_file: String,
    pub command_audit_file: String,
    pub virtual_state_file: String,
    pub dedup_limit: usize,
    pub outbox_limit: usize,
    pub command_ledger_limit: usize,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            state_dir: "/var/lib/netcore-iot-gateway".to_string(),
            outbox_dir: "outbox".to_string(),
            dedup_file: "dedup.json".to_string(),
            command_inbox_file: "command-inbox.ndjson".to_string(),
            command_ledger_file: "command-ledger.json".to_string(),
            command_audit_file: "command-audit.ndjson".to_string(),
            virtual_state_file: "virtual-device-state.json".to_string(),
            dedup_limit: 50_000,
            outbox_limit: 20_000,
            command_ledger_limit: 50_000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PollingConfig {
    pub interval_ms: u64,
    pub batch_limit: usize,
    pub request_timeout_ms: u64,
}

impl Default for PollingConfig {
    fn default() -> Self {
        Self {
            interval_ms: 2_000,
            batch_limit: 500,
            request_timeout_ms: 2_500,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct EventSourceConfig {
    pub id: String,
    pub url: String,
    pub enabled: bool,
}

impl Default for EventSourceConfig {
    fn default() -> Self {
        Self {
            id: String::new(),
            url: String::new(),
            enabled: true,
        }
    }
}

fn default_sources() -> Vec<EventSourceConfig> {
    [
        (
            "node-gateway",
            "http://node-gateway:8080/api/v1/events/netcore",
        ),
        (
            "mobility-core",
            "http://mobility-core:8090/api/v1/events/netcore",
        ),
        (
            "call-control",
            "http://call-control:8120/api/v1/events/netcore",
        ),
        (
            "sds-router",
            "http://sds-router:8150/api/v1/events/netcore",
        ),
    ]
    .into_iter()
    .map(|(id, url)| EventSourceConfig {
        id: id.to_string(),
        url: url.to_string(),
        enabled: true,
    })
    .collect()
}

fn default_command_policies() -> Vec<CommandPolicyConfig> {
    vec![
        CommandPolicyConfig {
            id: "allow-openlab-virtual-relays".to_string(),
            enabled: true,
            effect: PolicyEffect::Allow,
            command_types: vec!["virtual.relay.set".to_string()],
            target_types: vec!["virtual_relay".to_string()],
            target_prefixes: vec!["lab-".to_string()],
            max_ttl_secs: Some(120),
            allow_dry_run: true,
        },
        CommandPolicyConfig {
            id: "allow-openlab-virtual-lights".to_string(),
            enabled: true,
            effect: PolicyEffect::Allow,
            command_types: vec!["virtual.light.set".to_string()],
            target_types: vec!["virtual_light".to_string()],
            target_prefixes: vec!["lab-".to_string()],
            max_ttl_secs: Some(120),
            allow_dry_run: true,
        },
        CommandPolicyConfig {
            id: "allow-openlab-virtual-buttons".to_string(),
            enabled: true,
            effect: PolicyEffect::Allow,
            command_types: vec!["virtual.button.press".to_string()],
            target_types: vec!["virtual_button".to_string()],
            target_prefixes: vec!["lab-".to_string()],
            max_ttl_secs: Some(60),
            allow_dry_run: true,
        },
    ]
}

fn validate_topic_prefix(value: &str) -> Result<(), String> {
    let trimmed = value.trim_matches('/');
    if trimmed.is_empty() {
        return Err("mqtt.topic_prefix must not be empty".to_string());
    }
    if trimmed.len() > 200
        || trimmed.contains('#')
        || trimmed.contains('+')
        || trimmed.contains('\0')
    {
        return Err("mqtt.topic_prefix contains invalid MQTT topic characters".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_four_enables_only_the_open_lab_sandbox() {
        let config = IotGatewayConfig::default();
        assert!(config.commands.enabled);
        assert_eq!(config.commands.mode, "open_lab_sandbox");
        assert!(config.commands.default_deny);
        assert!(!config.commands.allow_retained);
        assert!(!config.mqtt.execute_commands);
        config.validate().unwrap();
    }

    #[test]
    fn deprecated_mqtt_execution_switch_is_rejected() {
        let mut config = IotGatewayConfig::default();
        config.mqtt.execute_commands = true;
        assert!(config.validate().is_err());
    }

    #[test]
    fn default_is_open_lab_on_port_8240() {
        let config = IotGatewayConfig::default();
        assert_eq!(config.security.mode, "open_lab");
        assert_eq!(config.server.bind.port(), 8240);
        assert_eq!(config.command_policies.len(), 3);
    }
}
