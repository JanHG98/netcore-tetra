#[path = "../../shared/web-ui/service-design.rs"]
mod service_design;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use serde_json::{Value, json};

use crate::config::ProvisioningConfig;
use crate::upstream::{self, UpstreamResponse};

struct HttpRequest {
    method: String,
    path: String,
    body: Vec<u8>,
}

struct HttpResponse {
    status: u16,
    content_type: String,
    body: Vec<u8>,
}

pub fn serve(config: ProvisioningConfig) -> std::io::Result<()> {
    let listener = TcpListener::bind(config.server.bind)?;
    tracing::info!("Provisioning Core WebUI/API listening on http://{}", config.server.bind);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let config = config.clone();
                thread::spawn(move || {
                    if let Err(error) = handle_connection(stream, &config) {
                        tracing::warn!("HTTP connection failed: {error}");
                    }
                });
            }
            Err(error) => tracing::warn!("HTTP accept failed: {error}"),
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream, config: &ProvisioningConfig) -> Result<(), String> {
    let request = read_request(&mut stream, config.limits.max_body_bytes)?;
    let response = route(request, config);
    write_response(&mut stream, response).map_err(|error| error.to_string())
}

fn route(request: HttpRequest, config: &ProvisioningConfig) -> HttpResponse {
    match (request.method.as_str(), request.path.as_str()) {
        ("OPTIONS", _) => empty(204),
        ("GET", "/") => html(&service_design::render(INDEX_HTML, "Provisioning Core", "open-lab")),
        ("GET", "/health/live") => json_response(200, &json!({"status":"live","service":"provisioning-core"})),
        ("GET", "/health/ready") | ("GET", "/api/v1/status") => status_response(config),
        ("GET", "/api/v1/dashboard") => dashboard_response(config),
        ("POST", "/api/v1/sync") => sync_response(config),
        ("DELETE", path) if path.starts_with("/api/v1/subscribers/") => {
            delete_subscriber_with_memberships(path, config)
        }
        ("DELETE", path) if path.starts_with("/api/v1/groups/") => {
            delete_group_with_memberships(path, config)
        }
        (_, path) if path == "/api/v1/subscribers" || path.starts_with("/api/v1/subscribers/") => {
            proxy(&config.upstream.subscriber_core, &request, config)
        }
        (_, path) if path == "/api/v1/groups"
            || path.starts_with("/api/v1/groups/")
            || path == "/api/v1/memberships"
            || path.starts_with("/api/v1/memberships/") =>
        {
            proxy(&config.upstream.group_core, &request, config)
        }
        _ => json_response(404, &json!({"error":"not found"})),
    }
}

fn status_response(config: &ProvisioningConfig) -> HttpResponse {
    let subscriber = upstream_json(&config.upstream.subscriber_core, "/api/v1/status", config);
    let groups = upstream_json(&config.upstream.group_core, "/api/v1/status", config);
    let ready = subscriber.as_ref().is_ok_and(|(status, _)| *status < 500)
        && groups.as_ref().is_ok_and(|(status, _)| *status < 500);
    json_response(
        if ready { 200 } else { 503 },
        &json!({
            "service":"provisioning-core",
            "security_mode":"open_lab",
            "ready":ready,
            "subscriber_core": result_snapshot(subscriber),
            "group_core": result_snapshot(groups),
        }),
    )
}

fn dashboard_response(config: &ProvisioningConfig) -> HttpResponse {
    let subscriber_status = upstream_json(&config.upstream.subscriber_core, "/api/v1/status", config);
    let group_status = upstream_json(&config.upstream.group_core, "/api/v1/status", config);
    let subscribers = upstream_json(&config.upstream.subscriber_core, "/api/v1/subscribers", config);
    let observed = upstream_json(&config.upstream.subscriber_core, "/api/v1/observed", config);
    let groups = upstream_json(&config.upstream.group_core, "/api/v1/groups", config);
    let memberships = upstream_json(&config.upstream.group_core, "/api/v1/memberships", config);

    let failures = [
        ("subscriber_status", &subscriber_status),
        ("group_status", &group_status),
        ("subscribers", &subscribers),
        ("observed", &observed),
        ("groups", &groups),
        ("memberships", &memberships),
    ]
    .into_iter()
    .filter_map(|(name, result)| result.as_ref().err().map(|error| json!({"source":name,"error":error})))
    .collect::<Vec<_>>();

    json_response(
        if failures.is_empty() { 200 } else { 503 },
        &json!({
            "service":"provisioning-core",
            "security_mode":"open_lab",
            "subscriber_core": value_or_null(subscriber_status),
            "group_core": value_or_null(group_status),
            "subscribers": value_or_array(subscribers),
            "observed": value_or_array(observed),
            "groups": value_or_array(groups),
            "memberships": value_or_array(memberships),
            "failures": failures,
        }),
    )
}

fn sync_response(config: &ProvisioningConfig) -> HttpResponse {
    let subscriber = upstream::request(
        &config.upstream.subscriber_core,
        "POST",
        "/api/v1/sync",
        b"",
        config.timeout(),
    );
    let group = upstream::request(
        &config.upstream.group_core,
        "POST",
        "/api/v1/sync",
        b"",
        config.timeout(),
    );
    let ok = subscriber.as_ref().is_ok_and(|response| response.status < 400)
        && group.as_ref().is_ok_and(|response| response.status < 400);
    json_response(
        if ok { 202 } else { 503 },
        &json!({
            "subscriber_core": upstream_snapshot(subscriber),
            "group_core": upstream_snapshot(group),
        }),
    )
}

fn delete_subscriber_with_memberships(path: &str, config: &ProvisioningConfig) -> HttpResponse {
    let Some(issi) = path.trim_start_matches("/api/v1/subscribers/").parse::<u32>().ok() else {
        return json_response(400, &json!({"error":"invalid ISSI"}));
    };
    if let Ok((_, Value::Array(memberships))) = upstream_json(&config.upstream.group_core, "/api/v1/memberships", config) {
        for membership in memberships {
            if membership.get("issi").and_then(Value::as_u64) == Some(issi as u64) {
                if let Some(gssi) = membership.get("gssi").and_then(Value::as_u64) {
                    let membership_path = format!("/api/v1/memberships/{issi}/{gssi}");
                    let _ = upstream::request(&config.upstream.group_core, "DELETE", &membership_path, b"", config.timeout());
                }
            }
        }
    }
    match upstream::request(&config.upstream.subscriber_core, "DELETE", path, b"", config.timeout()) {
        Ok(response) => from_upstream(response),
        Err(error) => json_response(503, &json!({"error":error})),
    }
}

fn delete_group_with_memberships(path: &str, config: &ProvisioningConfig) -> HttpResponse {
    let Some(gssi) = path.trim_start_matches("/api/v1/groups/").parse::<u32>().ok() else {
        return json_response(400, &json!({"error":"invalid GSSI"}));
    };
    if let Ok((_, Value::Array(memberships))) = upstream_json(&config.upstream.group_core, "/api/v1/memberships", config) {
        for membership in memberships {
            if membership.get("gssi").and_then(Value::as_u64) == Some(gssi as u64) {
                if let Some(issi) = membership.get("issi").and_then(Value::as_u64) {
                    let membership_path = format!("/api/v1/memberships/{issi}/{gssi}");
                    let _ = upstream::request(&config.upstream.group_core, "DELETE", &membership_path, b"", config.timeout());
                }
            }
        }
    }
    match upstream::request(&config.upstream.group_core, "DELETE", path, b"", config.timeout()) {
        Ok(response) => from_upstream(response),
        Err(error) => json_response(503, &json!({"error":error})),
    }
}

fn proxy(base: &str, request: &HttpRequest, config: &ProvisioningConfig) -> HttpResponse {
    match upstream::request(base, &request.method, &request.path, &request.body, config.timeout()) {
        Ok(response) => from_upstream(response),
        Err(error) => json_response(503, &json!({"error":error})),
    }
}

fn upstream_json(base: &str, path: &str, config: &ProvisioningConfig) -> Result<(u16, Value), String> {
    let response = upstream::request(base, "GET", path, b"", config.timeout())?;
    let value = if response.body.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&response.body).map_err(|error| format!("invalid JSON from {base}{path}: {error}"))?
    };
    Ok((response.status, value))
}

fn result_snapshot(result: Result<(u16, Value), String>) -> Value {
    match result {
        Ok((status, value)) => json!({"reachable":true,"status":status,"data":value}),
        Err(error) => json!({"reachable":false,"error":error}),
    }
}

fn upstream_snapshot(result: Result<UpstreamResponse, String>) -> Value {
    match result {
        Ok(response) => {
            let data = serde_json::from_slice::<Value>(&response.body).unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&response.body).to_string()));
            json!({"reachable":true,"status":response.status,"data":data})
        }
        Err(error) => json!({"reachable":false,"error":error}),
    }
}

fn value_or_null(result: Result<(u16, Value), String>) -> Value {
    result.map(|(_, value)| value).unwrap_or(Value::Null)
}

fn value_or_array(result: Result<(u16, Value), String>) -> Value {
    result.map(|(_, value)| value).unwrap_or_else(|_| Value::Array(Vec::new()))
}

fn from_upstream(response: UpstreamResponse) -> HttpResponse {
    HttpResponse { status: response.status, content_type: response.content_type, body: response.body }
}

fn read_request(stream: &mut TcpStream, max_body_bytes: usize) -> Result<HttpRequest, String> {
    stream.set_read_timeout(Some(std::time::Duration::from_secs(10))).map_err(|error| error.to_string())?;
    let mut data = Vec::new();
    let mut buffer = [0u8; 8192];
    let header_end;
    loop {
        let count = stream.read(&mut buffer).map_err(|error| error.to_string())?;
        if count == 0 { return Err("connection closed before HTTP headers".into()); }
        data.extend_from_slice(&buffer[..count]);
        if data.len() > max_body_bytes + 65_536 { return Err("request exceeds configured limit".into()); }
        if let Some(position) = data.windows(4).position(|window| window == b"\r\n\r\n") {
            header_end = position + 4;
            break;
        }
    }
    let header = String::from_utf8_lossy(&data[..header_end]);
    let mut lines = header.lines();
    let request_line = lines.next().ok_or_else(|| "missing request line".to_string())?;
    let mut parts = request_line.split_whitespace();
    let method = parts.next().ok_or_else(|| "missing method".to_string())?.to_string();
    let raw_path = parts.next().ok_or_else(|| "missing path".to_string())?;
    let path = raw_path.split('?').next().unwrap_or(raw_path).to_string();
    let headers = lines.filter_map(|line| line.split_once(':')).map(|(name, value)| (name.trim().to_ascii_lowercase(), value.trim().to_string())).collect::<HashMap<_, _>>();
    let content_length = headers.get("content-length").and_then(|value| value.parse::<usize>().ok()).unwrap_or(0);
    if content_length > max_body_bytes { return Err("request body exceeds configured limit".into()); }
    while data.len() < header_end + content_length {
        let count = stream.read(&mut buffer).map_err(|error| error.to_string())?;
        if count == 0 { return Err("truncated request body".into()); }
        data.extend_from_slice(&buffer[..count]);
    }
    Ok(HttpRequest { method, path, body: data[header_end..header_end + content_length].to_vec() })
}

fn write_response(stream: &mut TcpStream, response: HttpResponse) -> std::io::Result<()> {
    let reason = match response.status {
        200 => "OK", 201 => "Created", 202 => "Accepted", 204 => "No Content",
        400 => "Bad Request", 404 => "Not Found", 405 => "Method Not Allowed",
        409 => "Conflict", 500 => "Internal Server Error", 503 => "Service Unavailable",
        _ => "OK",
    };
    write!(stream,
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Headers: Content-Type\r\nAccess-Control-Allow-Methods: GET,POST,PUT,DELETE,OPTIONS\r\nConnection: close\r\n\r\n",
        response.status, reason, response.content_type, response.body.len())?;
    stream.write_all(&response.body)
}

fn json_response(status: u16, value: &Value) -> HttpResponse {
    HttpResponse { status, content_type: "application/json; charset=utf-8".into(), body: serde_json::to_vec_pretty(value).unwrap_or_default() }
}

fn html(value: &str) -> HttpResponse {
    HttpResponse { status: 200, content_type: "text/html; charset=utf-8".into(), body: value.as_bytes().to_vec() }
}

fn empty(status: u16) -> HttpResponse {
    HttpResponse { status, content_type: "text/plain; charset=utf-8".into(), body: Vec::new() }
}

const INDEX_HTML: &str = include_str!("../web-ui/index.html");
