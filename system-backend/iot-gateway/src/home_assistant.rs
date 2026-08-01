use std::collections::{BTreeMap, BTreeSet};

use serde_json::{json, Map, Value};
use uuid::Uuid;

use crate::command::VirtualDeviceState;
use crate::config::{HomematicDatapointConfig, IotGatewayConfig, PolicyEffect};
use crate::model::{OutboundMessage, SourceStatus};

pub fn discovery_messages(
    config: &IotGatewayConfig,
    sources: &[SourceStatus],
    virtual_devices: &[VirtualDeviceState],
) -> Vec<OutboundMessage> {
    if !config.home_assistant.enabled || !config.home_assistant.discovery_enabled {
        return Vec::new();
    }

    let mut messages = Vec::new();
    if config.home_assistant.expose_gateway {
        messages.push(gateway_connectivity_discovery(config));
    }
    if config.home_assistant.expose_sources {
        messages.extend(sources.iter().map(|source| source_discovery(config, source)));
    }
    if config.home_assistant.expose_virtual_devices {
        messages.extend(virtual_device_discovery(config, virtual_devices));
    }
    messages.extend(
        config
            .homematic_datapoints
            .iter()
            .filter(|datapoint| datapoint.enabled)
            .map(|datapoint| homematic_discovery(config, datapoint)),
    );
    messages
}

fn gateway_connectivity_discovery(config: &IotGatewayConfig) -> OutboundMessage {
    let prefix = config.mqtt.topic_prefix.trim_matches('/');
    let unique_id = "netcore_iot_gateway_connectivity";
    let payload = json!({
        "name":"IoT Gateway Verbindung",
        "unique_id":unique_id,
        "state_topic":format!("{prefix}/state/services/iot-gateway"),
        "value_template":"{{ value_json.status }}",
        "payload_on":"online",
        "payload_off":"offline",
        "device_class":"connectivity",
        "qos":config.mqtt.qos,
        "device":gateway_device(),
        "origin":origin()
    });
    discovery_message(config, "binary_sensor", unique_id, payload)
}

fn source_discovery(config: &IotGatewayConfig, source: &SourceStatus) -> OutboundMessage {
    let prefix = config.mqtt.topic_prefix.trim_matches('/');
    let object = format!("netcore_source_{}", safe_id(&source.id));
    let payload = json!({
        "name":format!("Quelle {}", source.id),
        "unique_id":object,
        "state_topic":format!("{prefix}/state/integrations/iot_gateway_sources/{}", topic_segment(&source.id)),
        "value_template":"{{ 'ON' if value_json.healthy else 'OFF' }}",
        "payload_on":"ON",
        "payload_off":"OFF",
        "device_class":"connectivity",
        "qos":config.mqtt.qos,
        "device":gateway_device(),
        "origin":origin()
    });
    discovery_message(config, "binary_sensor", &object, payload)
}

fn virtual_device_discovery(
    config: &IotGatewayConfig,
    virtual_devices: &[VirtualDeviceState],
) -> Vec<OutboundMessage> {
    let mut ids: BTreeMap<&str, BTreeSet<String>> = BTreeMap::new();
    ids.entry("virtual_relay")
        .or_default()
        .insert("lab-relay-01".to_string());
    ids.entry("virtual_light")
        .or_default()
        .insert("lab-light-01".to_string());
    ids.entry("virtual_button")
        .or_default()
        .insert("lab-button-01".to_string());
    for device in virtual_devices {
        if matches!(
            device.device_type.as_str(),
            "virtual_relay" | "virtual_light" | "virtual_button"
        ) {
            ids.entry(device.device_type.as_str())
                .or_default()
                .insert(device.id.clone());
        }
    }

    let mut messages = Vec::new();
    for (device_type, device_ids) in ids {
        for id in device_ids {
            messages.push(match device_type {
                "virtual_relay" => virtual_relay_discovery(config, &id),
                "virtual_light" => virtual_light_discovery(config, &id),
                "virtual_button" => virtual_button_discovery(config, &id),
                _ => continue,
            });
        }
    }
    messages
}

fn virtual_relay_discovery(config: &IotGatewayConfig, id: &str) -> OutboundMessage {
    let prefix = config.mqtt.topic_prefix.trim_matches('/');
    let object = format!("netcore_virtual_relay_{}", safe_id(id));
    let payload = json!({
        "name":format!("NetCore Relais {}", id),
        "unique_id":object,
        "state_topic":format!("{prefix}/state/virtual_relays/{}", topic_segment(id)),
        "value_template":"{{ 'ON' if value_json.state.state else 'OFF' }}",
        "command_topic":format!("{prefix}/integrations/homeassistant/commands/virtual_relay/{}/set", topic_segment(id)),
        "payload_on":"ON",
        "payload_off":"OFF",
        "qos":config.mqtt.qos,
        "device":virtual_device("virtual_relay", id, "OPEN-LAB Virtual Relay"),
        "origin":origin()
    });
    discovery_message(config, "switch", &object, payload)
}

fn virtual_light_discovery(config: &IotGatewayConfig, id: &str) -> OutboundMessage {
    let prefix = config.mqtt.topic_prefix.trim_matches('/');
    let object = format!("netcore_virtual_light_{}", safe_id(id));
    let payload = json!({
        "name":format!("NetCore Licht {}", id),
        "unique_id":object,
        "state_topic":format!("{prefix}/state/virtual_lights/{}", topic_segment(id)),
        "state_value_template":"{{ 'ON' if value_json.state.on else 'OFF' }}",
        "command_topic":format!("{prefix}/integrations/homeassistant/commands/virtual_light/{}/set", topic_segment(id)),
        "payload_on":"ON",
        "payload_off":"OFF",
        "brightness_state_topic":format!("{prefix}/state/virtual_lights/{}", topic_segment(id)),
        "brightness_value_template":"{{ value_json.state.brightness | int(0) }}",
        "brightness_command_topic":format!("{prefix}/integrations/homeassistant/commands/virtual_light/{}/brightness", topic_segment(id)),
        "brightness_scale":100,
        "qos":config.mqtt.qos,
        "device":virtual_device("virtual_light", id, "OPEN-LAB Virtual Light"),
        "origin":origin()
    });
    discovery_message(config, "light", &object, payload)
}

fn virtual_button_discovery(config: &IotGatewayConfig, id: &str) -> OutboundMessage {
    let prefix = config.mqtt.topic_prefix.trim_matches('/');
    let object = format!("netcore_virtual_button_{}", safe_id(id));
    let payload = json!({
        "name":format!("NetCore Taster {}", id),
        "unique_id":object,
        "command_topic":format!("{prefix}/integrations/homeassistant/commands/virtual_button/{}/press", topic_segment(id)),
        "payload_press":"PRESS",
        "qos":config.mqtt.qos,
        "device":virtual_device("virtual_button", id, "OPEN-LAB Virtual Button"),
        "origin":origin()
    });
    discovery_message(config, "button", &object, payload)
}

fn homematic_discovery(
    config: &IotGatewayConfig,
    datapoint: &HomematicDatapointConfig,
) -> OutboundMessage {
    let prefix = config.mqtt.topic_prefix.trim_matches('/');
    let object = format!("netcore_homematic_{}", safe_id(&datapoint.id));
    let mut payload = Map::new();
    payload.insert(
        "name".to_string(),
        Value::String(if datapoint.name.is_empty() {
            datapoint.id.clone()
        } else {
            datapoint.name.clone()
        }),
    );
    payload.insert("unique_id".to_string(), Value::String(object.clone()));
    payload.insert(
        "state_topic".to_string(),
        Value::String(format!(
            "{prefix}/state/homematic/{}",
            topic_segment(&datapoint.id)
        )),
    );
    payload.insert("qos".to_string(), Value::from(config.mqtt.qos));
    payload.insert(
        "availability_topic".to_string(),
        Value::String(format!(
            "{prefix}/state/homematic/{}",
            topic_segment(&datapoint.id)
        )),
    );
    payload.insert(
        "availability_template".to_string(),
        Value::String("{{ 'online' if value_json.healthy else 'offline' }}".to_string()),
    );
    payload.insert("payload_available".to_string(), Value::String("online".to_string()));
    payload.insert("payload_not_available".to_string(), Value::String("offline".to_string()));
    payload.insert("device".to_string(), homematic_device(datapoint));
    payload.insert("origin".to_string(), origin());
    match datapoint.platform.as_str() {
        "binary_sensor" => {
            payload.insert(
                "value_template".to_string(),
                Value::String("{{ 'ON' if value_json.value else 'OFF' }}".to_string()),
            );
            payload.insert("payload_on".to_string(), Value::String("ON".to_string()));
            payload.insert("payload_off".to_string(), Value::String("OFF".to_string()));
        }
        "switch" => {
            payload.insert(
                "value_template".to_string(),
                Value::String("{{ 'ON' if value_json.value else 'OFF' }}".to_string()),
            );
            payload.insert("payload_on".to_string(), Value::String("ON".to_string()));
            payload.insert("payload_off".to_string(), Value::String("OFF".to_string()));
            if homematic_write_discoverable(config, datapoint) {
                payload.insert(
                    "command_topic".to_string(),
                    Value::String(format!(
                        "{prefix}/integrations/homeassistant/commands/homematic_datapoint/{}/set",
                        topic_segment(&datapoint.id)
                    )),
                );
            }
        }
        _ => {
            payload.insert(
                "value_template".to_string(),
                Value::String("{{ value_json.value }}".to_string()),
            );
        }
    }
    if let Some(device_class) = &datapoint.device_class {
        payload.insert(
            "device_class".to_string(),
            Value::String(device_class.clone()),
        );
    }
    if let Some(unit) = &datapoint.unit {
        payload.insert(
            "unit_of_measurement".to_string(),
            Value::String(unit.clone()),
        );
    }
    discovery_message(config, &datapoint.platform, &object, Value::Object(payload))
}

fn homematic_write_discoverable(
    config: &IotGatewayConfig,
    datapoint: &HomematicDatapointConfig,
) -> bool {
    datapoint.writable
        && config.homematic.enabled
        && config.homematic.mode == "ccu_xml_rpc"
        && config.homematic.allow_writes
        && config.command_policies.iter().any(|policy| {
            policy.enabled
                && policy.effect == PolicyEffect::Allow
                && (policy.command_types.is_empty()
                    || policy
                        .command_types
                        .iter()
                        .any(|value| value == "*" || value == "homematic.datapoint.set"))
                && (policy.target_types.is_empty()
                    || policy
                        .target_types
                        .iter()
                        .any(|value| value == "*" || value == "homematic_datapoint"))
                && (policy.target_prefixes.is_empty()
                    || policy
                        .target_prefixes
                        .iter()
                        .any(|prefix| datapoint.id.starts_with(prefix)))
        })
}

fn discovery_message(
    config: &IotGatewayConfig,
    component: &str,
    object_id: &str,
    payload: Value,
) -> OutboundMessage {
    OutboundMessage {
        id: Uuid::new_v4(),
        created_at: chrono::Utc::now().to_rfc3339(),
        kind: "home_assistant_discovery".to_string(),
        topic: format!(
            "{}/{}/{}/{}/config",
            config.home_assistant.discovery_prefix.trim_matches('/'),
            component,
            config.home_assistant.node_id,
            safe_id(object_id)
        ),
        qos: config.home_assistant.discovery_qos,
        retain: config.home_assistant.discovery_retain,
        payload: payload.to_string(),
        source_event_id: None,
    }
}

fn gateway_device() -> Value {
    json!({
        "identifiers":["netcore-tetra-iot-gateway"],
        "name":"NetCore-TETRA IoT Gateway",
        "manufacturer":"NetCore-TETRA",
        "model":"IoT Gateway",
        "sw_version":env!("CARGO_PKG_VERSION")
    })
}

fn virtual_device(device_type: &str, id: &str, model: &str) -> Value {
    json!({
        "identifiers":[format!("netcore:{device_type}:{id}")],
        "name":format!("NetCore {}", id),
        "manufacturer":"NetCore-TETRA",
        "model":model,
        "sw_version":env!("CARGO_PKG_VERSION")
    })
}

fn homematic_device(datapoint: &HomematicDatapointConfig) -> Value {
    json!({
        "identifiers":[format!("netcore:homematic:{}", datapoint.address)],
        "name":if datapoint.name.is_empty() { datapoint.id.clone() } else { datapoint.name.clone() },
        "manufacturer":"eQ-3 / Homematic IP",
        "model":"XML-RPC datapoint",
        "via_device":"netcore-tetra-iot-gateway"
    })
}

fn origin() -> Value {
    json!({
        "name":"NetCore-TETRA IoT Gateway",
        "sw_version":env!("CARGO_PKG_VERSION")
    })
}

pub fn safe_id(value: &str) -> String {
    let mut output = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '_' | '-') {
                character
            } else {
                '_'
            }
        })
        .collect::<String>();
    if output.is_empty() {
        output.push_str("unnamed");
    }
    output
}

pub fn topic_segment(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.') {
                character
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovery_contains_virtual_open_lab_entities() {
        let config = IotGatewayConfig::default();
        let messages = discovery_messages(&config, &[], &[]);
        assert!(messages.iter().any(|message| message.topic.contains("/switch/")));
        assert!(messages.iter().any(|message| message.topic.contains("/light/")));
        assert!(messages.iter().any(|message| message.topic.contains("/button/")));
        assert!(messages.iter().all(|message| message.retain));
    }
}
