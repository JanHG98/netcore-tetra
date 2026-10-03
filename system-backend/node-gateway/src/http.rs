// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für die Verbindung zwischen Basisstationen und Backend-Diensten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

#[path = "../../shared/web-ui/service-design.rs"]
mod service_design;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use serde::Deserialize;
use serde_json::{Value, json};
use tetra_entities::net_control::ControlCommand;

use crate::config::NodeGatewayConfig;
use crate::state::SharedGateway;

#[derive(Debug)]
// Was: Bündelt die zusammengehörigen Werte für HTTP request in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct HttpRequest {
    method: String,
    path: String,
    query: HashMap<String, String>,
    body: Vec<u8>,
}

// Was: Bündelt die zusammengehörigen Werte für HTTP response in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct HttpResponse {
    status: u16,
    content_type: &'static str,
    body: Vec<u8>,
}

// Was: Implementiert das zugehörige Verhalten für `HttpResponse`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl HttpResponse {
    // Was: Führt den Arbeitsschritt `json` für JSON-Daten aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn json(status: u16, value: &impl serde::Serialize) -> Self {
        Self {
            status,
            content_type: "application/json; charset=utf-8",
            body: serde_json::to_vec_pretty(value).unwrap_or_else(|_| b"{\"error\":\"serialization failed\"}".to_vec()),
        }
    }

    // Was: Führt den Arbeitsschritt `html` für html aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn html(status: u16, body: &str) -> Self {
        Self { status, content_type: "text/html; charset=utf-8", body: body.as_bytes().to_vec() }
    }

    // Was: Führt den Arbeitsschritt `text` für text aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn text(status: u16, content_type: &'static str, body: String) -> Self {
        Self { status, content_type, body: body.into_bytes() }
    }
}

#[derive(Debug, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für command request in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct CommandRequest {
    #[serde(default)]
    operator_id: Option<String>,
    command: ControlCommand,
}

// Was: Diese Funktion verarbeitet HTTP stream.
// Warum: Die Reaktion auf dieses Ereignis bleibt damit an einer Stelle nachvollziehbar.
pub fn handle_http_stream(mut stream: TcpStream, gateway: SharedGateway, config: NodeGatewayConfig) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    let response = match read_http_request(&mut stream, config.limits.max_http_body_bytes) {
        Ok(request) => route(request, &gateway, &config),
        Err(error) => HttpResponse::json(400, &json!({ "error": error })),
    };
    let _ = write_response(&mut stream, response);
}

// Was: Diese Funktion leitet den vorgesehenen Arbeitsschritt.
// Warum: Nachrichten und Daten gelangen dadurch nachvollziehbar an das richtige Ziel.
fn route(request: HttpRequest, gateway: &SharedGateway, config: &NodeGatewayConfig) -> HttpResponse {
    if request.method == "OPTIONS" {
        return HttpResponse::text(204, "text/plain", String::new());
    }

    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => HttpResponse::html(200, &service_design::render(INDEX_HTML, "Node Gateway", "open-lab")),
        ("GET", "/health/live") => HttpResponse::json(200, &json!({
            "ok": true,
            "service": "netcore-node-gateway",
            "security_mode": "open_lab",
        })),
        ("GET", "/health/ready") => HttpResponse::json(200, &json!({
            "ok": true,
            "ready": true,
            "listener": config.server.bind,
            "security_mode": "open_lab",
        })),
        ("GET", "/api/v1/status") => HttpResponse::json(200, &gateway.status()),
        ("GET", "/api/v1/core-services") => HttpResponse::json(200, &gateway.core_services()),
        ("GET", "/api/v1/nodes") => HttpResponse::json(200, &gateway.nodes()),
        ("GET", "/api/v1/events/netcore") => {
            let limit = request.query.get("limit").and_then(|value| value.parse::<usize>().ok()).unwrap_or(100).min(1_000);
            HttpResponse::json(200, &gateway.recent_netcore_events(limit))
        }
        ("GET", "/api/v1/events") => {
            let limit = request.query.get("limit").and_then(|value| value.parse::<usize>().ok()).unwrap_or(100).min(1_000);
            HttpResponse::json(200, &gateway.recent_events(limit))
        }
        ("GET", "/api/v1/config") => HttpResponse::json(200, &json!({
            "server": &config.server,
            "security": &config.security,
            "limits": &config.limits,
            "service_monitor": &config.service_monitor,
            "effective_warning": "NO AUTHENTICATION, NO TOKENS, NO TLS - TEST NETWORK ONLY"
        })),
        ("GET", "/metrics") => HttpResponse::text(200, "text/plain; version=0.0.4; charset=utf-8", gateway.metrics()),
        ("GET", "/openapi.json") => HttpResponse::json(200, &openapi(config)),
        _ => route_dynamic(request, gateway),
    }
}

// Was: Diese Funktion leitet dynamic.
// Warum: Nachrichten und Daten gelangen dadurch nachvollziehbar an das richtige Ziel.
fn route_dynamic(request: HttpRequest, gateway: &SharedGateway) -> HttpResponse {
    let Some((node_id, action)) = parse_node_route(&request.path) else {
        return HttpResponse::json(404, &json!({
            "error": "not found",
            "available": [
                "GET /", "GET /health/live", "GET /health/ready", "GET /api/v1/status",
                "GET /api/v1/core-services", "GET /api/v1/nodes", "GET /api/v1/nodes/{node_id}", "GET /api/v1/events",
                "GET /api/v1/config", "GET /metrics", "GET /openapi.json",
                "POST /api/v1/nodes/{node_id}/ping", "POST /api/v1/nodes/{node_id}/disconnect",
                "POST /api/v1/nodes/{node_id}/commands"
            ]
        }));
    };

    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match (request.method.as_str(), action.as_deref()) {
        ("GET", None) => match gateway.node(&node_id) {
            Some(node) => HttpResponse::json(200, &node),
            None => HttpResponse::json(404, &json!({ "error": "unknown node", "node_id": node_id })),
        },
        ("POST", Some("ping")) => action_response(gateway.ping_node(&node_id)),
        ("POST", Some("disconnect")) => action_response(gateway.disconnect_node(&node_id)),
        ("POST", Some("commands")) => {
            let parsed = serde_json::from_slice::<CommandRequest>(&request.body)
                .map_err(|error| format!("invalid command json: {error}"))
                .and_then(|body| gateway.send_command(&node_id, body.command, body.operator_id));
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parsed {
                Ok(command_id) => HttpResponse::json(202, &json!({ "ok": true, "command_id": command_id, "node_id": node_id })),
                Err(error) => HttpResponse::json(400, &json!({ "ok": false, "error": error })),
            }
        }
        _ => HttpResponse::json(404, &json!({ "error": "unknown node action", "node_id": node_id, "action": action })),
    }
}

// Was: Führt den Arbeitsschritt `action_response` für action response aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn action_response(result: Result<(), String>) -> HttpResponse {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match result {
        Ok(()) => HttpResponse::json(202, &json!({ "ok": true })),
        Err(error) => HttpResponse::json(409, &json!({ "ok": false, "error": error })),
    }
}

// Was: Diese Funktion liest und prüft Netzknoten Weiterleitung.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_node_route(path: &str) -> Option<(String, Option<String>)> {
    let tail = path.strip_prefix("/api/v1/nodes/")?;
    let mut parts = tail.split('/');
    let node_id = parts.next()?.trim();
    if node_id.is_empty() {
        return None;
    }
    let action = parts.next().map(str::to_string);
    if parts.next().is_some() {
        return None;
    }
    Some((percentish_decode(node_id), action))
}

// Was: Führt den Arbeitsschritt `openapi` für openapi aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn openapi(config: &NodeGatewayConfig) -> Value {
    json!({
        "openapi": "3.0.3",
        "info": { "title": "NetCore Node Gateway API", "version": "1.0.0-open-lab" },
        "servers": [{ "url": format!("http://{}", config.server.bind) }],
        "x-netcore-security-mode": "open_lab",
        "paths": {
            "/health/live": { "get": { "summary": "Liveness" } },
            "/health/ready": { "get": { "summary": "Readiness" } },
            "/api/v1/status": { "get": { "summary": "Gateway status" } },
            "/api/v1/core-services": { "get": { "summary": "Backend health matrix delivered to TBS nodes" } },
            "/api/v1/nodes": { "get": { "summary": "Known TBS nodes" } },
            "/api/v1/nodes/{node_id}": { "get": { "summary": "Node detail" } },
            "/api/v1/nodes/{node_id}/ping": { "post": { "summary": "Queue application ping" } },
            "/api/v1/nodes/{node_id}/disconnect": { "post": { "summary": "Disconnect node" } },
            "/api/v1/nodes/{node_id}/commands": { "post": { "summary": "Queue ControlCommand" } },
            "/api/v1/events": { "get": { "summary": "Legacy gateway events with embedded canonical event" } },
            "/api/v1/events/netcore": { "get": { "summary": "Canonical netcore-event-v1 records" } },
            "/metrics": { "get": { "summary": "Prometheus metrics" } }
        }
    })
}

// Was: Diese Funktion liest HTTP request.
// Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
fn read_http_request(stream: &mut TcpStream, max_body_bytes: usize) -> Result<HttpRequest, String> {
    let mut buffer = Vec::with_capacity(8_192);
    let mut chunk = [0u8; 4_096];
    let header_end = loop {
        let read = stream.read(&mut chunk).map_err(|error| format!("request read failed: {error}"))?;
        if read == 0 {
            return Err("connection closed before request was complete".to_string());
        }
        buffer.extend_from_slice(&chunk[..read]);
        if buffer.len() > max_body_bytes + 65_536 {
            return Err("request too large".to_string());
        }
        if let Some(position) = find_subslice(&buffer, b"\r\n\r\n") {
            break position + 4;
        }
    };

    let header_text = std::str::from_utf8(&buffer[..header_end]).map_err(|_| "request headers are not utf-8".to_string())?;
    let mut lines = header_text.split("\r\n");
    let request_line = lines.next().ok_or_else(|| "missing request line".to_string())?;
    let mut request_parts = request_line.split_whitespace();
    let method = request_parts.next().ok_or_else(|| "missing method".to_string())?.to_ascii_uppercase();
    let raw_path = request_parts.next().ok_or_else(|| "missing path".to_string())?;
    let (path, query) = parse_path_and_query(raw_path);

    let mut content_length = 0usize;
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for line in lines {
        if let Some((name, value)) = line.split_once(':') {
            if name.eq_ignore_ascii_case("content-length") {
                content_length = value.trim().parse::<usize>().map_err(|_| "invalid content-length".to_string())?;
            }
        }
    }
    if content_length > max_body_bytes {
        return Err("body too large".to_string());
    }

    let mut body = buffer[header_end..].to_vec();
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    while body.len() < content_length {
        let read = stream.read(&mut chunk).map_err(|error| format!("body read failed: {error}"))?;
        if read == 0 {
            return Err("connection closed before body was complete".to_string());
        }
        body.extend_from_slice(&chunk[..read]);
        if body.len() > max_body_bytes {
            return Err("body too large".to_string());
        }
    }
    body.truncate(content_length);
    Ok(HttpRequest { method, path, query, body })
}

// Was: Diese Funktion schreibt response.
// Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
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

// Was: Führt den Arbeitsschritt `looks_like_websocket_upgrade` für looks like websocket upgrade aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn looks_like_websocket_upgrade(peek: &[u8]) -> bool {
    let text = String::from_utf8_lossy(peek).to_ascii_lowercase();
    text.contains("upgrade: websocket") && text.contains("sec-websocket-key:")
}

// Was: Diese Funktion sucht subslice.
// Warum: Die Suchlogik bleibt damit wiederverwendbar und muss nicht an mehreren Stellen kopiert werden.
fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

// Was: Diese Funktion liest und prüft path and query.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
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
                        percentish_decode(fields.next().unwrap_or_default()),
                        percentish_decode(fields.next().unwrap_or("true")),
                    )
                })
                .collect()
        })
        .unwrap_or_default();
    (path, query)
}

// Was: Führt den Arbeitsschritt `percentish_decode` für percentish Dekodierung aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn percentish_decode(value: &str) -> String {
    value.replace('+', " ").replace("%20", " ")
}

// Was: Führt den Arbeitsschritt `reason_phrase` für reason phrase aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn reason_phrase(status: u16) -> &'static str {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match status {
        200 => "OK",
        202 => "Accepted",
        204 => "No Content",
        400 => "Bad Request",
        404 => "Not Found",
        409 => "Conflict",
        500 => "Internal Server Error",
        _ => "OK",
    }
}

// Was: Legt den festen Wert `INDEX_HTML` für index html fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const INDEX_HTML: &str = include_str!("../web-ui/index.html");

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Führt den Arbeitsschritt `parses_node_routes` für parses Netzknoten routes aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn parses_node_routes() {
        assert_eq!(parse_node_route("/api/v1/nodes/tbs-a"), Some(("tbs-a".to_string(), None)));
        assert_eq!(parse_node_route("/api/v1/nodes/tbs-a/ping"), Some(("tbs-a".to_string(), Some("ping".to_string()))));
    }
}
