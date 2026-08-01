use std::collections::BTreeMap;

use chrono::{SecondsFormat, Utc};
use netcore_contracts::{CommandPolicyDecision, NetCoreCommand};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::config::{CommandPolicyConfig, IotGatewayConfig, PolicyEffect};

#[derive(Debug, Clone, Serialize)]
pub struct PolicyEvaluation {
    pub decision: CommandPolicyDecision,
    pub policy_id: Option<String>,
    pub reason_code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualDeviceState {
    pub device_type: String,
    pub id: String,
    pub state: Value,
    pub updated_at: String,
    pub command_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct IntegrationOutbound {
    pub kind: String,
    pub topic: String,
    pub qos: u8,
    pub retain: bool,
    pub payload: String,
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub result: Value,
    pub state_update: Option<VirtualDeviceState>,
    pub outbound: Vec<IntegrationOutbound>,
}

pub fn evaluate_policy(config: &IotGatewayConfig, command: &NetCoreCommand) -> PolicyEvaluation {
    let ttl = command
        .expires_at
        .signed_duration_since(command.requested_at)
        .num_seconds();
    if ttl <= 0 || ttl > config.commands.max_ttl_secs as i64 {
        return PolicyEvaluation {
            decision: CommandPolicyDecision::Deny,
            policy_id: None,
            reason_code: "ttl_exceeds_global_limit".to_string(),
            message: format!(
                "command TTL {ttl}s exceeds global limit {}s",
                config.commands.max_ttl_secs
            ),
        };
    }

    let matches = config
        .command_policies
        .iter()
        .filter(|policy| policy.enabled && policy_matches(policy, command))
        .collect::<Vec<_>>();

    if let Some(policy) = matches
        .iter()
        .copied()
        .find(|policy| policy.effect == PolicyEffect::Deny)
    {
        return decision_from_policy(policy, command, ttl, CommandPolicyDecision::Deny);
    }

    if let Some(policy) = matches
        .iter()
        .copied()
        .find(|policy| policy.effect == PolicyEffect::Allow)
    {
        return decision_from_policy(policy, command, ttl, CommandPolicyDecision::Allow);
    }

    PolicyEvaluation {
        decision: if config.commands.default_deny {
            CommandPolicyDecision::Deny
        } else {
            CommandPolicyDecision::Allow
        },
        policy_id: None,
        reason_code: if config.commands.default_deny {
            "default_deny"
        } else {
            "default_allow"
        }
        .to_string(),
        message: if config.commands.default_deny {
            "no allow policy matched the command"
        } else {
            "no policy matched; global default allows the command"
        }
        .to_string(),
    }
}

fn decision_from_policy(
    policy: &CommandPolicyConfig,
    command: &NetCoreCommand,
    ttl: i64,
    decision: CommandPolicyDecision,
) -> PolicyEvaluation {
    if let Some(max_ttl_secs) = policy.max_ttl_secs {
        if ttl > max_ttl_secs as i64 {
            return PolicyEvaluation {
                decision: CommandPolicyDecision::Deny,
                policy_id: Some(policy.id.clone()),
                reason_code: "ttl_exceeds_policy_limit".to_string(),
                message: format!(
                    "command TTL {ttl}s exceeds policy limit {max_ttl_secs}s"
                ),
            };
        }
    }
    if command.dry_run && !policy.allow_dry_run {
        return PolicyEvaluation {
            decision: CommandPolicyDecision::Deny,
            policy_id: Some(policy.id.clone()),
            reason_code: "dry_run_not_allowed".to_string(),
            message: "the matching policy does not allow dry-run execution".to_string(),
        };
    }
    PolicyEvaluation {
        decision,
        policy_id: Some(policy.id.clone()),
        reason_code: match decision {
            CommandPolicyDecision::Allow => "policy_allow",
            CommandPolicyDecision::Deny => "policy_deny",
        }
        .to_string(),
        message: format!(
            "policy {} {} the command",
            policy.id,
            match decision {
                CommandPolicyDecision::Allow => "allows",
                CommandPolicyDecision::Deny => "denies",
            }
        ),
    }
}

fn policy_matches(policy: &CommandPolicyConfig, command: &NetCoreCommand) -> bool {
    list_matches(&policy.command_types, &command.command_type)
        && list_matches(&policy.target_types, &command.target.target_type)
        && (policy.target_prefixes.is_empty()
            || policy
                .target_prefixes
                .iter()
                .any(|prefix| command.target.id.starts_with(prefix)))
}

fn list_matches(values: &[String], candidate: &str) -> bool {
    values.is_empty() || values.iter().any(|value| value == "*" || value == candidate)
}

pub fn execute_command(
    config: &IotGatewayConfig,
    command: &NetCoreCommand,
    virtual_devices: &mut BTreeMap<String, VirtualDeviceState>,
) -> Result<ExecutionResult, String> {
    match command.command_type.as_str() {
        "virtual.relay.set" => execute_virtual_relay(command, virtual_devices),
        "virtual.light.set" => execute_virtual_light(command, virtual_devices),
        "virtual.button.press" => execute_virtual_button(command, virtual_devices),
        "homeassistant.entity.command" => execute_home_assistant_command(config, command),
        "homematic.datapoint.set" => {
            let result = crate::homematic::execute_datapoint_command(config, command)?;
            Ok(ExecutionResult {
                result,
                state_update: None,
                outbound: Vec::new(),
            })
        }
        other => Err(format!("no Phase-5 OPEN-LAB executor exists for {other}")),
    }
}

pub fn execute_sandbox(
    command: &NetCoreCommand,
    virtual_devices: &mut BTreeMap<String, VirtualDeviceState>,
) -> Result<ExecutionResult, String> {
    execute_command(&IotGatewayConfig::default(), command, virtual_devices)
}

fn execute_home_assistant_command(
    config: &IotGatewayConfig,
    command: &NetCoreCommand,
) -> Result<ExecutionResult, String> {
    require_target_type(command, "home_assistant_entity")?;
    if !config.home_assistant.enabled {
        return Err("Home Assistant integration is disabled".to_string());
    }
    if !config.home_assistant.allow_command_egress {
        return Err("home_assistant.allow_command_egress is false".to_string());
    }
    let action = command
        .payload
        .get("action")
        .and_then(Value::as_str)
        .ok_or_else(|| "payload.action is required".to_string())?;
    if action.is_empty() || action.len() > 128 {
        return Err("payload.action must contain 1..128 characters".to_string());
    }
    let data = command
        .payload
        .get("data")
        .cloned()
        .unwrap_or_else(|| json!({}));
    if !data.is_object() {
        return Err("payload.data must be an object".to_string());
    }
    let result = json!({
        "entity_id":command.target.id,
        "action":action,
        "data":data,
        "transport":"mqtt_to_home_assistant",
        "dry_run":command.dry_run
    });
    if command.dry_run {
        return Ok(ExecutionResult {
            result,
            state_update: None,
            outbound: Vec::new(),
        });
    }
    let payload = json!({
        "schema":"netcore-home-assistant-command-v1",
        "command_id":command.command_id,
        "entity_id":command.target.id,
        "action":action,
        "data":data,
        "requested_at":command.requested_at,
        "expires_at":command.expires_at
    })
    .to_string();
    Ok(ExecutionResult {
        result,
        state_update: None,
        outbound: vec![IntegrationOutbound {
            kind: "home_assistant_command".to_string(),
            topic: config.home_assistant_command_egress_topic(),
            qos: config.mqtt.qos,
            retain: false,
            payload,
        }],
    })
}

fn execute_virtual_relay(
    command: &NetCoreCommand,
    virtual_devices: &mut BTreeMap<String, VirtualDeviceState>,
) -> Result<ExecutionResult, String> {
    require_target_type(command, "virtual_relay")?;
    let state = command
        .payload
        .get("state")
        .and_then(Value::as_bool)
        .ok_or_else(|| "payload.state must be a boolean".to_string())?;
    let result = json!({"state":state,"dry_run":command.dry_run});
    if command.dry_run {
        return Ok(ExecutionResult {
            result,
            state_update: None,
            outbound: Vec::new(),
        });
    }
    let update = VirtualDeviceState {
        device_type: command.target.target_type.clone(),
        id: command.target.id.clone(),
        state: json!({"state":state}),
        updated_at: now_iso(),
        command_id: command.command_id,
    };
    virtual_devices.insert(device_key(&update.device_type, &update.id), update.clone());
    Ok(ExecutionResult {
        result,
        state_update: Some(update),
        outbound: Vec::new(),
    })
}

fn execute_virtual_light(
    command: &NetCoreCommand,
    virtual_devices: &mut BTreeMap<String, VirtualDeviceState>,
) -> Result<ExecutionResult, String> {
    require_target_type(command, "virtual_light")?;
    let on = command
        .payload
        .get("on")
        .and_then(Value::as_bool)
        .ok_or_else(|| "payload.on must be a boolean".to_string())?;
    let brightness = command
        .payload
        .get("brightness")
        .map(|value| {
            value
                .as_u64()
                .filter(|value| *value <= 100)
                .ok_or_else(|| "payload.brightness must be an integer within 0..100".to_string())
        })
        .transpose()?;
    let result = json!({"on":on,"brightness":brightness,"dry_run":command.dry_run});
    if command.dry_run {
        return Ok(ExecutionResult {
            result,
            state_update: None,
            outbound: Vec::new(),
        });
    }
    let update = VirtualDeviceState {
        device_type: command.target.target_type.clone(),
        id: command.target.id.clone(),
        state: json!({"on":on,"brightness":brightness}),
        updated_at: now_iso(),
        command_id: command.command_id,
    };
    virtual_devices.insert(device_key(&update.device_type, &update.id), update.clone());
    Ok(ExecutionResult {
        result,
        state_update: Some(update),
        outbound: Vec::new(),
    })
}

fn execute_virtual_button(
    command: &NetCoreCommand,
    virtual_devices: &mut BTreeMap<String, VirtualDeviceState>,
) -> Result<ExecutionResult, String> {
    require_target_type(command, "virtual_button")?;
    let key = device_key(&command.target.target_type, &command.target.id);
    let previous = virtual_devices
        .get(&key)
        .and_then(|state| state.state.get("press_count"))
        .and_then(Value::as_u64)
        .unwrap_or(0);
    let press_count = previous.saturating_add(1);
    let result = json!({"pressed":true,"press_count":press_count,"dry_run":command.dry_run});
    if command.dry_run {
        return Ok(ExecutionResult {
            result,
            state_update: None,
            outbound: Vec::new(),
        });
    }
    let update = VirtualDeviceState {
        device_type: command.target.target_type.clone(),
        id: command.target.id.clone(),
        state: json!({"last_pressed_at":now_iso(),"press_count":press_count}),
        updated_at: now_iso(),
        command_id: command.command_id,
    };
    virtual_devices.insert(key, update.clone());
    Ok(ExecutionResult {
        result,
        state_update: Some(update),
        outbound: Vec::new(),
    })
}

fn require_target_type(command: &NetCoreCommand, expected: &str) -> Result<(), String> {
    if command.target.target_type == expected {
        Ok(())
    } else {
        Err(format!(
            "command {} requires target.type={expected}, got {}",
            command.command_type, command.target.target_type
        ))
    }
}

pub fn device_key(device_type: &str, id: &str) -> String {
    format!("{device_type}/{id}")
}

fn now_iso() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use netcore_contracts::{CommandSource, CommandTarget};

    #[test]
    fn default_policy_allows_only_lab_virtual_relay() {
        let config = IotGatewayConfig::default();
        let allowed = NetCoreCommand::new(
            "virtual.relay.set",
            CommandSource::new("test", "test-1"),
            CommandTarget::new("virtual_relay", "lab-relay-01"),
            json!({"state":true}),
            30,
        );
        assert_eq!(
            evaluate_policy(&config, &allowed).decision,
            CommandPolicyDecision::Allow
        );

        let denied = NetCoreCommand::new(
            "virtual.relay.set",
            CommandSource::new("test", "test-1"),
            CommandTarget::new("virtual_relay", "real-relay-01"),
            json!({"state":true}),
            30,
        );
        assert_eq!(
            evaluate_policy(&config, &denied).decision,
            CommandPolicyDecision::Deny
        );
    }

    #[test]
    fn dry_run_does_not_change_virtual_state() {
        let mut command = NetCoreCommand::new(
            "virtual.relay.set",
            CommandSource::new("test", "test-1"),
            CommandTarget::new("virtual_relay", "lab-relay-01"),
            json!({"state":true}),
            30,
        );
        command.dry_run = true;
        let mut devices = BTreeMap::new();
        let result = execute_sandbox(&command, &mut devices).unwrap();
        assert!(result.state_update.is_none());
        assert!(devices.is_empty());
    }
}
