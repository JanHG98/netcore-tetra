#[path = "../../shared/web-ui/service-design.rs"]
mod service_design;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use serde::Serialize;
use serde_json::json;
use uuid::Uuid;

use crate::config::IotGatewayConfig;
use crate::homematic::HomematicControl;
use crate::model::{ActionResult, TestPublishInput};
use crate::mqtt::MqttControl;
use crate::poller::PollControl;
use crate::state::SharedGateway;

pub fn spawn_http_server(
    config: IotGatewayConfig,
    state: SharedGateway,
    poll_control: PollControl,
    mqtt_control: MqttControl,
    homematic_control: Option<HomematicControl>,
) -> std::io::Result<thread::JoinHandle<()>> {
    let listener = TcpListener::bind(config.server.bind)?;
    tracing::info!("IoT Gateway WebUI/API listening on http://{}", config.server.bind);
    Ok(thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let config = config.clone();
                    let state = state.clone();
                    let poll_control = poll_control.clone();
                    let mqtt_control = mqtt_control.clone();
                    let homematic_control = homematic_control.clone();
                    thread::spawn(move || {
                        if let Err(error) = handle_connection(
                            stream,
                            config,
                            state,
                            poll_control,
                            mqtt_control,
                            homematic_control,
                        ) {
                            tracing::warn!("IoT Gateway HTTP connection failed: {}", error);
                        }
                    });
                }
                Err(error) => tracing::warn!("IoT Gateway HTTP accept failed: {}", error),
            }
        }
    }))
}

struct HttpRequest {
    method: String,
    path: String,
    query: HashMap<String, String>,
    body: Vec<u8>,
}

struct HttpResponse {
    status: u16,
    content_type: &'static str,
    body: Vec<u8>,
}

fn handle_connection(
    mut stream: TcpStream,
    config: IotGatewayConfig,
    state: SharedGateway,
    poll_control: PollControl,
    mqtt_control: MqttControl,
    homematic_control: Option<HomematicControl>,
) -> Result<(), String> {
    let request = read_request(&mut stream, config.server.max_body_bytes)?;
    let response = route(
        request,
        config,
        state,
        poll_control,
        mqtt_control,
        homematic_control,
    );
    write_response(&mut stream, response).map_err(|error| error.to_string())
}

fn route(
    request: HttpRequest,
    config: IotGatewayConfig,
    state: SharedGateway,
    poll_control: PollControl,
    mqtt_control: MqttControl,
    homematic_control: Option<HomematicControl>,
) -> HttpResponse {
    if request.method == "OPTIONS" {
        return empty(204);
    }
    if request.method == "GET" {
        if let Some(raw_id) = request.path.strip_prefix("/api/v1/commands/") {
            return match Uuid::parse_str(raw_id) {
                Ok(command_id) => match state.command(command_id) {
                    Some(record) => json_response(200, &record),
                    None => json_response(404, &json!({"error":"command not found"})),
                },
                Err(error) => json_response(400, &json!({"error":format!("invalid command UUID: {error}")})),
            };
        }
    }
    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => html(&service_design::render(INDEX_HTML, "IoT Gateway", "open-lab")),
        ("GET", "/health/live") => json_response(200, &json!({"status":"live"})),
        ("GET", "/health/ready") => {
            let status = state.status();
            let ready = status.mqtt_connected
                && status.sources_enabled > 0
                && status.sources_healthy == status.sources_enabled;
            let code = if ready { 200 } else { 503 };
            json_response(code, &status)
        }
        ("GET", "/api/v1/status") => json_response(200, &state.status()),
        ("GET", "/api/v1/sources") => json_response(200, &state.sources()),
        ("GET", "/api/v1/topics") => json_response(200, &state.topic_registry()),
        ("GET", "/api/v1/config") => json_response(200, &config),
        ("GET", "/api/v1/events") => {
            let limit = query_limit(&request.query, 100, 2_000);
            json_response(200, &state.recent_events(limit))
        }
        ("GET", "/api/v1/commands") => {
            let limit = query_limit(&request.query, 100, 2_000);
            json_response(200, &state.commands(limit))
        }
        ("GET", "/api/v1/policies") => json_response(200, &state.command_policies()),
        ("GET", "/api/v1/virtual-devices") => json_response(200, &state.virtual_devices()),
        ("GET", "/api/v1/home-assistant") => {
            json_response(200, &state.home_assistant_status())
        }
        ("GET", "/api/v1/home-assistant/entities") => {
            json_response(200, &state.external_entities())
        }
        ("GET", "/api/v1/homematic/datapoints") => {
            json_response(200, &state.homematic_datapoints())
        }
        ("GET", "/api/v1/outbox") => {
            let limit = query_limit(&request.query, 100, 2_000);
            json_response(200, &state.outbox_entries(limit))
        }
        ("POST", "/api/v1/actions/poll-now") => match poll_control.poll_now() {
            Ok(()) => json_response(
                202,
                &ActionResult {
                    accepted: true,
                    message: "event poll requested".to_string(),
                },
            ),
            Err(error) => json_response(503, &json!({"error":error})),
        },
        ("POST", "/api/v1/actions/reconnect") => {
            mqtt_control.reconnect();
            json_response(
                202,
                &ActionResult {
                    accepted: true,
                    message: "MQTT reconnect requested".to_string(),
                },
            )
        }
        ("POST", "/api/v1/actions/home-assistant-discovery") => {
            match state.enqueue_home_assistant_discovery("http_action") {
                Ok(messages) => json_response(
                    202,
                    &json!({"accepted":true,"messages_enqueued":messages}),
                ),
                Err(error) => json_response(409, &json!({"error":error})),
            }
        }
        ("POST", "/api/v1/actions/homematic-poll-now") => {
            match homematic_control {
                Some(control) => match control.poll_now() {
                    Ok(()) => json_response(
                        202,
                        &ActionResult {
                            accepted: true,
                            message: "Homematic poll requested".to_string(),
                        },
                    ),
                    Err(error) => json_response(503, &json!({"error":error})),
                },
                None => json_response(
                    409,
                    &json!({"error":"Homematic CCU XML-RPC worker is not enabled"}),
                ),
            }
        }
        ("POST", "/api/v1/test/homeassistant-state") => {
            if request.body.is_empty() {
                return json_response(
                    400,
                    &json!({"error":"Home Assistant state JSON body is required"}),
                );
            }
            match state.ingest_home_assistant_state(
                config.home_assistant_state_ingress_topic(),
                request.body,
            ) {
                Ok(update) => json_response(202, &update),
                Err(error) => json_response(400, &json!({"error":error})),
            }
        }
        ("POST", "/api/v1/test/command") => {
            if request.body.is_empty() {
                return json_response(400, &json!({"error":"netcore-command-v1 JSON body is required"}));
            }
            let topic = format!(
                "{}/commands/http-test",
                config.mqtt.topic_prefix.trim_matches('/')
            );
            match state.process_command(topic, request.body, 0, false) {
                Ok(record) => json_response(200, &record),
                Err(error) => json_response(409, &json!({"error":error})),
            }
        }
        ("POST", "/api/v1/test/publish") => {
            let input = if request.body.is_empty() {
                TestPublishInput {
                    topic: None,
                    payload: json!({"message":"NetCore IoT Gateway OPEN LAB test"}),
                    retain: false,
                    qos: None,
                }
            } else {
                match serde_json::from_slice::<TestPublishInput>(&request.body) {
                    Ok(value) => value,
                    Err(error) => {
                        return json_response(400, &json!({"error":format!("invalid JSON: {error}")}))
                    }
                }
            };
            let topic = input.topic.unwrap_or_else(|| {
                format!(
                    "{}/test/manual",
                    config.mqtt.topic_prefix.trim_matches('/')
                )
            });
            let payload = if input.payload.is_null() {
                json!({"message":"NetCore IoT Gateway OPEN LAB test"}).to_string()
            } else {
                input.payload.to_string()
            };
            match state.enqueue_manual_message(
                topic,
                payload,
                input.qos.unwrap_or(config.mqtt.qos),
                input.retain,
            ) {
                Ok(message) => json_response(202, &message),
                Err(error) => json_response(409, &json!({"error":error})),
            }
        }
        ("GET", "/metrics") => text(
            "text/plain; version=0.0.4; charset=utf-8",
            state.metrics(),
        ),
        ("GET", "/openapi.json") => json_response(200, &openapi()),
        _ => json_response(404, &json!({"error":"not found"})),
    }
}

fn openapi() -> serde_json::Value {
    json!({
        "openapi":"3.0.3",
        "info":{
            "title":"NetCore IoT Gateway",
            "version":env!("CARGO_PKG_VERSION"),
            "description":"Phase 5 MQTT bridge with Home Assistant discovery, Homematic IP adapters and policy-controlled commands in OPEN LAB mode."
        },
        "paths":{
            "/api/v1/status":{"get":{}},
            "/api/v1/sources":{"get":{}},
            "/api/v1/topics":{"get":{}},
            "/api/v1/events":{"get":{}},
            "/api/v1/commands":{"get":{}},
            "/api/v1/commands/{command_id}":{"get":{}},
            "/api/v1/policies":{"get":{}},
            "/api/v1/virtual-devices":{"get":{}},
            "/api/v1/home-assistant":{"get":{}},
            "/api/v1/home-assistant/entities":{"get":{}},
            "/api/v1/homematic/datapoints":{"get":{}},
            "/api/v1/outbox":{"get":{}},
            "/api/v1/actions/poll-now":{"post":{}},
            "/api/v1/actions/reconnect":{"post":{}},
            "/api/v1/actions/home-assistant-discovery":{"post":{}},
            "/api/v1/actions/homematic-poll-now":{"post":{}},
            "/api/v1/test/homeassistant-state":{"post":{}},
            "/api/v1/test/command":{"post":{}},
            "/api/v1/test/publish":{"post":{}},
            "/health/live":{"get":{}},
            "/health/ready":{"get":{}},
            "/metrics":{"get":{}}
        }
    })
}

fn query_limit(query: &HashMap<String, String>, default: usize, maximum: usize) -> usize {
    query
        .get("limit")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(default)
        .min(maximum)
}

fn json_response<T: Serialize>(status: u16, value: &T) -> HttpResponse {
    HttpResponse {
        status,
        content_type: "application/json; charset=utf-8",
        body: serde_json::to_vec_pretty(value).unwrap_or_else(|error| {
            format!("{{\"error\":\"JSON serialization failed: {error}\"}}").into_bytes()
        }),
    }
}

fn html(value: &str) -> HttpResponse {
    HttpResponse {
        status: 200,
        content_type: "text/html; charset=utf-8",
        body: value.as_bytes().to_vec(),
    }
}

fn text(content_type: &'static str, value: String) -> HttpResponse {
    HttpResponse {
        status: 200,
        content_type,
        body: value.into_bytes(),
    }
}

fn empty(status: u16) -> HttpResponse {
    HttpResponse {
        status,
        content_type: "text/plain; charset=utf-8",
        body: Vec::new(),
    }
}

fn read_request(stream: &mut TcpStream, max_body_bytes: usize) -> Result<HttpRequest, String> {
    stream
        .set_read_timeout(Some(std::time::Duration::from_secs(10)))
        .map_err(|error| error.to_string())?;
    let mut buffer = Vec::new();
    let mut chunk = [0_u8; 4096];
    let header_end = loop {
        let read = stream
            .read(&mut chunk)
            .map_err(|error| format!("request read failed: {error}"))?;
        if read == 0 {
            return Err("connection closed before HTTP headers were complete".to_string());
        }
        buffer.extend_from_slice(&chunk[..read]);
        if buffer.len() > 65_536 + max_body_bytes {
            return Err("HTTP request is too large".to_string());
        }
        if let Some(position) = find_subslice(&buffer, b"\r\n\r\n") {
            break position + 4;
        }
    };

    let headers = std::str::from_utf8(&buffer[..header_end])
        .map_err(|_| "HTTP headers are not valid UTF-8".to_string())?;
    let mut lines = headers.split("\r\n");
    let request_line = lines.next().ok_or_else(|| "missing HTTP request line".to_string())?;
    let mut request_parts = request_line.split_whitespace();
    let method = request_parts
        .next()
        .ok_or_else(|| "missing HTTP method".to_string())?
        .to_string();
    let target = request_parts
        .next()
        .ok_or_else(|| "missing HTTP target".to_string())?;
    let (path, query) = parse_path_and_query(target);
    let content_length = lines
        .filter_map(|line| line.split_once(':'))
        .find(|(name, _)| name.eq_ignore_ascii_case("content-length"))
        .and_then(|(_, value)| value.trim().parse::<usize>().ok())
        .unwrap_or(0);
    if content_length > max_body_bytes {
        return Err("HTTP body exceeds configured limit".to_string());
    }

    let mut body = buffer[header_end..].to_vec();
    while body.len() < content_length {
        let read = stream
            .read(&mut chunk)
            .map_err(|error| format!("body read failed: {error}"))?;
        if read == 0 {
            return Err("connection closed before HTTP body was complete".to_string());
        }
        body.extend_from_slice(&chunk[..read]);
        if body.len() > max_body_bytes {
            return Err("HTTP body exceeds configured limit".to_string());
        }
    }
    body.truncate(content_length);
    Ok(HttpRequest {
        method,
        path,
        query,
        body,
    })
}

fn write_response(stream: &mut TcpStream, response: HttpResponse) -> std::io::Result<()> {
    let header = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nX-NetCore-Security-Mode: open-lab\r\nConnection: close\r\n\r\n",
        response.status,
        reason_phrase(response.status),
        response.content_type,
        response.body.len(),
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(&response.body)?;
    stream.flush()
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

fn parse_path_and_query(raw: &str) -> (String, HashMap<String, String>) {
    let mut parts = raw.splitn(2, '?');
    let path = parts.next().unwrap_or(raw).to_string();
    let query = parts
        .next()
        .map(|query| {
            query
                .split('&')
                .filter(|pair| !pair.is_empty())
                .map(|pair| {
                    let mut fields = pair.splitn(2, '=');
                    (
                        fields
                            .next()
                            .unwrap_or_default()
                            .replace('+', " ")
                            .replace("%20", " "),
                        fields
                            .next()
                            .unwrap_or("true")
                            .replace('+', " ")
                            .replace("%20", " "),
                    )
                })
                .collect()
        })
        .unwrap_or_default();
    (path, query)
}

fn reason_phrase(status: u16) -> &'static str {
    match status {
        200 => "OK",
        202 => "Accepted",
        204 => "No Content",
        400 => "Bad Request",
        404 => "Not Found",
        409 => "Conflict",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        _ => "OK",
    }
}

const INDEX_HTML: &str = include_str!("../web-ui/index.html");
