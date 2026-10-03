// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Registrierung, Aufenthaltsbereiche und Teilnehmermobilität.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

#[path = "../../shared/web-ui/service-design.rs"]
mod service_design;

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::thread;

use serde::Serialize;
use serde_json::json;

use crate::config::MobilityCoreConfig;
use crate::protocol::BackendRequest;
use crate::state::{CreateTransferRequest, SharedMobility};

// Was: Diese Funktion startet HTTP server.
// Warum: Länger laufende Arbeit blockiert dadurch nicht den aufrufenden Ablauf.
pub fn spawn_http_server(
    config: MobilityCoreConfig,
    mobility: SharedMobility,
    gateway_tx: Sender<BackendRequest>,
) -> std::io::Result<thread::JoinHandle<()>> {
    let listener = TcpListener::bind(config.server.bind)?;
    tracing::info!("Mobility Core WebUI/API listening on http://{}", config.server.bind);
    Ok(thread::spawn(move || {
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for stream in listener.incoming() {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match stream {
                Ok(stream) => {
                    let mobility = mobility.clone();
                    let gateway_tx = gateway_tx.clone();
                    let config = config.clone();
                    thread::spawn(move || {
                        if let Err(error) = handle_connection(stream, mobility, gateway_tx, config) {
                            tracing::warn!("HTTP connection failed: {}", error);
                        }
                    });
                }
                Err(error) => tracing::warn!("HTTP accept failed: {}", error),
            }
        }
    }))
}

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

// Was: Diese Funktion verarbeitet connection.
// Warum: Die Reaktion auf dieses Ereignis bleibt damit an einer Stelle nachvollziehbar.
fn handle_connection(
    mut stream: TcpStream,
    mobility: SharedMobility,
    gateway_tx: Sender<BackendRequest>,
    config: MobilityCoreConfig,
) -> Result<(), String> {
    let request = read_request(&mut stream, config.limits.max_body_bytes)?;
    let response = route(request, mobility, gateway_tx, config);
    write_response(&mut stream, response).map_err(|error| error.to_string())
}

// Was: Diese Funktion leitet den vorgesehenen Arbeitsschritt.
// Warum: Nachrichten und Daten gelangen dadurch nachvollziehbar an das richtige Ziel.
fn route(
    request: HttpRequest,
    mobility: SharedMobility,
    gateway_tx: Sender<BackendRequest>,
    config: MobilityCoreConfig,
) -> HttpResponse {
    if request.method == "OPTIONS" {
        return empty(204);
    }
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => html(&service_design::render(INDEX_HTML, "Mobility Core", "open-lab")),
        ("GET", "/health/live") => json_response(200, &json!({ "status": "live" })),
        ("GET", "/health/ready") => {
            let status = mobility.status();
            let code = if status.node_gateway_connected { 200 } else { 503 };
            json_response(code, &status)
        }
        ("GET", "/api/v1/status") => json_response(200, &mobility.status()),
        ("GET", "/api/v1/nodes") => json_response(200, &mobility.nodes()),
        ("GET", "/api/v1/subscribers") => json_response(200, &mobility.subscribers()),
        _ if request.method == "GET" && request.path.starts_with("/api/v1/subscribers/") && request.path.ends_with("/route") => {
            let value = request.path.trim_start_matches("/api/v1/subscribers/").trim_end_matches("/route").trim_end_matches('/');
            match value.parse::<u32>() {
                Ok(issi) if issi <= 0x00ff_ffff => {
                    let route = mobility.subscriber_route(issi);
                    let status = if matches!(route.state, crate::state::RouteState::Unknown) { 404 } else { 200 };
                    json_response(status, &route)
                }
                _ => json_response(400, &json!({"error":"invalid ISSI"})),
            }
        }
        ("GET", "/api/v1/transfers") => json_response(200, &mobility.transfers()),
        ("GET", "/api/v1/events/netcore") => {
            let limit = request.query.get("limit")
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(100)
                .min(1_000);
            json_response(200, &mobility.recent_netcore_events(limit))
        }
        ("GET", "/api/v1/events") => {
            let limit = request.query.get("limit")
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(100)
                .min(1_000);
            json_response(200, &mobility.recent_events(limit))
        }
        ("GET", "/api/v1/config") => json_response(200, &config),
        ("GET", "/metrics") => HttpResponse {
            status: 200,
            content_type: "text/plain; version=0.0.4; charset=utf-8",
            body: mobility.metrics().into_bytes(),
        },
        ("GET", "/openapi.json") => json_response(200, &openapi()),
        ("POST", "/api/v1/transfers") => {
            let parsed = serde_json::from_slice::<CreateTransferRequest>(&request.body)
                .map_err(|error| format!("invalid JSON: {error}"));
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parsed.and_then(|request| mobility.create_transfer(request)) {
                Ok((transfer, command)) => {
                    if gateway_tx.send(command).is_err() {
                        json_response(503, &json!({ "error": "node gateway worker is unavailable" }))
                    } else {
                        json_response(202, &transfer)
                    }
                }
                Err(error) => json_response(409, &json!({ "error": error })),
            }
        }
        _ if request.method == "POST" && request.path.starts_with("/api/v1/transfers/") => {
            let tail = request.path.trim_start_matches("/api/v1/transfers/");
            if let Some((id, action)) = tail.rsplit_once('/') {
                // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                match action {
                    "cancel" => match mobility.cancel_transfer(id) {
                        Ok(transfer) => json_response(200, &transfer),
                        Err(error) => json_response(409, &json!({ "error": error })),
                    },
                    _ => json_response(404, &json!({ "error": "unknown transfer action" })),
                }
            } else {
                json_response(404, &json!({ "error": "missing transfer action" }))
            }
        }
        _ => json_response(404, &json!({ "error": "not found" })),
    }
}

// Was: Führt den Arbeitsschritt `openapi` für openapi aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn openapi() -> serde_json::Value {
    json!({
        "openapi": "3.0.3",
        "info": {
            "title": "NetCore Mobility Core",
            "version": env!("CARGO_PKG_VERSION"),
            "description": "OPEN LAB API. No authentication, no token and no TLS."
        },
        "paths": {
            "/api/v1/status": { "get": {} },
            "/api/v1/nodes": { "get": {} },
            "/api/v1/subscribers": { "get": {} },
            "/api/v1/subscribers/{issi}/route": { "get": { "description": "Canonical serving-TBS route for one ISSI" } },
            "/api/v1/transfers": { "get": {}, "post": {} },
            "/api/v1/transfers/{id}/cancel": { "post": {} },
            "/api/v1/events": { "get": { "description": "Legacy event records with embedded canonical event" } },
            "/api/v1/events/netcore": { "get": { "description": "Canonical netcore-event-v1 records" } },
            "/health/live": { "get": {} },
            "/health/ready": { "get": {} },
            "/metrics": { "get": {} }
        }
    })
}

// Was: Führt den Arbeitsschritt `json_response` für JSON-Daten response aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn json_response<T: Serialize>(status: u16, value: &T) -> HttpResponse {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match serde_json::to_vec_pretty(value) {
        Ok(body) => HttpResponse {
            status,
            content_type: "application/json; charset=utf-8",
            body,
        },
        Err(error) => HttpResponse {
            status: 500,
            content_type: "application/json; charset=utf-8",
            body: format!("{{\"error\":\"serialization failed: {error}\"}}").into_bytes(),
        },
    }
}

// Was: Führt den Arbeitsschritt `html` für html aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn html(value: &str) -> HttpResponse {
    HttpResponse {
        status: 200,
        content_type: "text/html; charset=utf-8",
        body: value.as_bytes().to_vec(),
    }
}

// Was: Führt den Arbeitsschritt `empty` für empty aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn empty(status: u16) -> HttpResponse {
    HttpResponse {
        status,
        content_type: "text/plain; charset=utf-8",
        body: Vec::new(),
    }
}

// Was: Diese Funktion liest request.
// Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
fn read_request(stream: &mut TcpStream, max_body_bytes: usize) -> Result<HttpRequest, String> {
    let mut buffer = Vec::new();
    let mut chunk = [0u8; 4096];
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

    let header_text = std::str::from_utf8(&buffer[..header_end])
        .map_err(|_| "request headers are not utf-8".to_string())?;
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
                content_length = value.trim().parse::<usize>()
                    .map_err(|_| "invalid content-length".to_string())?;
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
    let query = parts.next().map(|query| {
        query.split('&')
            .filter(|pair| !pair.is_empty())
            .map(|pair| {
                let mut fields = pair.splitn(2, '=');
                (
                    fields.next().unwrap_or_default().replace('+', " ").replace("%20", " "),
                    fields.next().unwrap_or("true").replace('+', " ").replace("%20", " "),
                )
            })
            .collect()
    }).unwrap_or_default();
    (path, query)
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
        503 => "Service Unavailable",
        _ => "OK",
    }
}

// Was: Legt den festen Wert `INDEX_HTML` für index html fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const INDEX_HTML: &str = include_str!("../web-ui/index.html");
