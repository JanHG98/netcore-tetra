use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use netcore_contracts::NetCoreCommand;
use reqwest::blocking::Client;
use reqwest::header::CONTENT_TYPE;
use serde_json::{json, Value};

use crate::config::{HomematicDatapointConfig, IotGatewayConfig};
use crate::state::SharedGateway;

#[derive(Clone)]
pub struct HomematicControl {
    poll_tx: mpsc::Sender<()>,
}

impl HomematicControl {
    pub fn poll_now(&self) -> Result<(), String> {
        self.poll_tx
            .send(())
            .map_err(|_| "Homematic worker is not available".to_string())
    }
}

pub fn spawn_homematic(
    config: IotGatewayConfig,
    state: SharedGateway,
) -> (Option<HomematicControl>, Option<thread::JoinHandle<()>>) {
    if !config.homematic.enabled || config.homematic.mode != "ccu_xml_rpc" {
        return (None, None);
    }
    let (poll_tx, poll_rx) = mpsc::channel();
    let control = HomematicControl { poll_tx };
    let handle = thread::spawn(move || {
        let client = match Client::builder()
            .timeout(Duration::from_millis(config.homematic.request_timeout_ms))
            .build()
        {
            Ok(client) => client,
            Err(error) => {
                state.record_bridge_event(
                    "homematic.worker_failed",
                    json!({"error":error.to_string()}),
                );
                return;
            }
        };
        loop {
            poll_all(&client, &config, &state);
            match poll_rx.recv_timeout(Duration::from_millis(
                config.homematic.poll_interval_ms.max(500),
            )) {
                Ok(()) | Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => return,
            }
        }
    });
    (Some(control), Some(handle))
}

pub fn poll_all(client: &Client, config: &IotGatewayConfig, state: &SharedGateway) {
    for datapoint in config
        .homematic_datapoints
        .iter()
        .filter(|datapoint| datapoint.enabled)
    {
        match get_value(client, config, datapoint) {
            Ok(value) => state.record_homematic_success(datapoint, value),
            Err(error) => state.record_homematic_failure(&datapoint.id, error),
        }
    }
}

pub fn execute_datapoint_command(
    config: &IotGatewayConfig,
    command: &NetCoreCommand,
) -> Result<Value, String> {
    if command.target.target_type != "homematic_datapoint" {
        return Err("target.type must be homematic_datapoint".to_string());
    }
    if !config.homematic.enabled || config.homematic.mode != "ccu_xml_rpc" {
        return Err("direct Homematic CCU XML-RPC mode is not enabled".to_string());
    }
    if !config.homematic.allow_writes {
        return Err("homematic.allow_writes is false".to_string());
    }
    let datapoint = config
        .homematic_datapoints
        .iter()
        .find(|datapoint| datapoint.enabled && datapoint.id == command.target.id)
        .ok_or_else(|| format!("unknown Homematic datapoint {}", command.target.id))?;
    if !datapoint.writable {
        return Err(format!("Homematic datapoint {} is read-only", datapoint.id));
    }
    let value = command
        .payload
        .get("value")
        .cloned()
        .ok_or_else(|| "payload.value is required".to_string())?;
    validate_value_type(&value, &datapoint.value_type)?;
    if command.dry_run {
        return Ok(json!({
            "datapoint_id":datapoint.id,
            "address":datapoint.address,
            "parameter":datapoint.parameter,
            "value":value,
            "dry_run":true
        }));
    }
    let client = Client::builder()
        .timeout(Duration::from_millis(config.homematic.request_timeout_ms))
        .build()
        .map_err(|error| error.to_string())?;
    set_value(&client, config, datapoint, &value)?;
    Ok(json!({
        "datapoint_id":datapoint.id,
        "address":datapoint.address,
        "parameter":datapoint.parameter,
        "value":value,
        "transport":"ccu_xml_rpc",
        "confirmed":"xmlrpc_method_response"
    }))
}

fn get_value(
    client: &Client,
    config: &IotGatewayConfig,
    datapoint: &HomematicDatapointConfig,
) -> Result<Value, String> {
    let body = method_call(
        "getValue",
        &[
            xml_string(&datapoint.address),
            xml_string(&datapoint.parameter),
        ],
    );
    let response = post_xml(client, config, body)?;
    let value = parse_xmlrpc_value(&response)?;
    validate_value_type(&value, &datapoint.value_type)?;
    Ok(value)
}

fn set_value(
    client: &Client,
    config: &IotGatewayConfig,
    datapoint: &HomematicDatapointConfig,
    value: &Value,
) -> Result<(), String> {
    let body = method_call(
        "setValue",
        &[
            xml_string(&datapoint.address),
            xml_string(&datapoint.parameter),
            xml_value(value, &datapoint.value_type)?,
        ],
    );
    let response = post_xml(client, config, body)?;
    if response.contains("<fault>") {
        return Err(format!("Homematic XML-RPC fault: {}", compact_xml(&response)));
    }
    Ok(())
}

fn post_xml(client: &Client, config: &IotGatewayConfig, body: String) -> Result<String, String> {
    let url = format!(
        "http://{}:{}/",
        config.homematic.ccu_host, config.homematic.ccu_port
    );
    let response = client
        .post(url)
        .header(CONTENT_TYPE, "text/xml; charset=utf-8")
        .body(body)
        .send()
        .map_err(|error| format!("Homematic XML-RPC request failed: {error}"))?;
    let status = response.status();
    let text = response
        .text()
        .map_err(|error| format!("Homematic XML-RPC response read failed: {error}"))?;
    if !status.is_success() {
        return Err(format!(
            "Homematic XML-RPC returned HTTP {}: {}",
            status,
            compact_xml(&text)
        ));
    }
    if text.contains("<fault>") {
        return Err(format!("Homematic XML-RPC fault: {}", compact_xml(&text)));
    }
    Ok(text)
}

fn method_call(method: &str, params: &[String]) -> String {
    let params = params
        .iter()
        .map(|value| format!("<param><value>{value}</value></param>"))
        .collect::<String>();
    format!(
        "<?xml version=\"1.0\"?><methodCall><methodName>{}</methodName><params>{}</params></methodCall>",
        xml_escape(method),
        params
    )
}

fn xml_string(value: &str) -> String {
    format!("<string>{}</string>", xml_escape(value))
}

fn xml_value(value: &Value, value_type: &str) -> Result<String, String> {
    match value_type {
        "bool" => value
            .as_bool()
            .map(|value| format!("<boolean>{}</boolean>", u8::from(value)))
            .ok_or_else(|| "Homematic boolean value required".to_string()),
        "integer" => value
            .as_i64()
            .and_then(|value| i32::try_from(value).ok())
            .map(|value| format!("<i4>{value}</i4>"))
            .ok_or_else(|| "Homematic integer value within the 32-bit XML-RPC range required".to_string()),
        "float" => value
            .as_f64()
            .map(|value| format!("<double>{value}</double>"))
            .ok_or_else(|| "Homematic float value required".to_string()),
        "string" => value
            .as_str()
            .map(xml_string)
            .ok_or_else(|| "Homematic string value required".to_string()),
        other => Err(format!("unsupported Homematic value_type {other}")),
    }
}

fn parse_xmlrpc_value(xml: &str) -> Result<Value, String> {
    if xml.contains("<fault>") {
        return Err(format!("Homematic XML-RPC fault: {}", compact_xml(xml)));
    }
    if let Some(value) = tag(xml, "boolean") {
        return match value.trim() {
            "1" | "true" => Ok(Value::Bool(true)),
            "0" | "false" => Ok(Value::Bool(false)),
            other => Err(format!("invalid XML-RPC boolean {other}")),
        };
    }
    for name in ["i4", "int", "i8"] {
        if let Some(value) = tag(xml, name) {
            return value
                .trim()
                .parse::<i64>()
                .map(Value::from)
                .map_err(|error| format!("invalid XML-RPC integer: {error}"));
        }
    }
    if let Some(value) = tag(xml, "double") {
        return value
            .trim()
            .parse::<f64>()
            .map(Value::from)
            .map_err(|error| format!("invalid XML-RPC double: {error}"));
    }
    if let Some(value) = tag(xml, "string") {
        return Ok(Value::String(xml_unescape(value.trim())));
    }
    if let Some(value) = tag(xml, "value") {
        return Ok(Value::String(xml_unescape(value.trim())));
    }
    Err(format!(
        "Homematic XML-RPC response contains no supported scalar value: {}",
        compact_xml(xml)
    ))
}

fn validate_value_type(value: &Value, value_type: &str) -> Result<(), String> {
    let valid = match value_type {
        "bool" => value.is_boolean(),
        "integer" => value.as_i64().is_some(),
        "float" => value.as_f64().is_some(),
        "string" => value.as_str().is_some(),
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err(format!("value does not match Homematic type {value_type}"))
    }
}

fn tag<'a>(xml: &'a str, name: &str) -> Option<&'a str> {
    let start_tag = format!("<{name}>");
    let end_tag = format!("</{name}>");
    let start = xml.find(&start_tag)? + start_tag.len();
    let end = xml[start..].find(&end_tag)? + start;
    Some(&xml[start..end])
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn xml_unescape(value: &str) -> String {
    value
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
}

fn compact_xml(value: &str) -> String {
    let compact = value.split_whitespace().collect::<Vec<_>>().join(" ");
    compact.chars().take(400).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_common_xmlrpc_scalars() {
        assert_eq!(
            parse_xmlrpc_value("<methodResponse><value><boolean>1</boolean></value></methodResponse>").unwrap(),
            Value::Bool(true)
        );
        assert_eq!(
            parse_xmlrpc_value("<value><double>21.5</double></value>").unwrap(),
            json!(21.5)
        );
        assert_eq!(
            parse_xmlrpc_value("<value><string>A&amp;B</string></value>").unwrap(),
            json!("A&B")
        );
    }
}
