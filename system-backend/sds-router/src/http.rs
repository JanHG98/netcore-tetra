// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Weiterleitung von SDS- und Statusnachrichten.
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

use crate::config::SdsRouterConfig;
use crate::protocol::BackendRequest;
use crate::state::{
    ApplicationAckInput, MessageInput, RouteInput, SharedSdsRouter, parse_message_state,
};

// Was: Diese Funktion startet HTTP server.
// Warum: Länger laufende Arbeit blockiert dadurch nicht den aufrufenden Ablauf.
pub fn spawn_http_server(
    config: SdsRouterConfig,
    router: SharedSdsRouter,
    gateway_tx: Sender<BackendRequest>,
) -> std::io::Result<thread::JoinHandle<()>> {
    let listener = TcpListener::bind(config.server.bind)?;
    tracing::info!(
        "SDS Router WebUI/API listening on http://{}",
        config.server.bind
    );
    Ok(thread::spawn(move || {
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for stream in listener.incoming() {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match stream {
                Ok(stream) => {
                    let router = router.clone();
                    let gateway_tx = gateway_tx.clone();
                    let config = config.clone();
                    thread::spawn(move || {
                        if let Err(error) =
                            handle_connection(stream, router, gateway_tx, config)
                        {
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
    disposition: Option<String>,
}

// Was: Diese Funktion verarbeitet connection.
// Warum: Die Reaktion auf dieses Ereignis bleibt damit an einer Stelle nachvollziehbar.
fn handle_connection(
    mut stream: TcpStream,
    router: SharedSdsRouter,
    gateway_tx: Sender<BackendRequest>,
    config: SdsRouterConfig,
) -> Result<(), String> {
    let request = read_request(&mut stream, config.limits.max_body_bytes)?;
    let response = route(request, router, gateway_tx, config);
    write_response(&mut stream, response).map_err(|error| error.to_string())
}

// Was: Diese Funktion leitet den vorgesehenen Arbeitsschritt.
// Warum: Nachrichten und Daten gelangen dadurch nachvollziehbar an das richtige Ziel.
fn route(
    request: HttpRequest,
    router: SharedSdsRouter,
    gateway_tx: Sender<BackendRequest>,
    config: SdsRouterConfig,
) -> HttpResponse {
    if request.method == "OPTIONS" {
        return empty(204);
    }

    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => html(&service_design::render(INDEX_HTML, "SDS Router", "open-lab")),
        ("GET", "/health/live") => json_response(200, &json!({"status":"live"})),
        ("GET", "/health/ready") => {
            let status = router.status();
            json_response(if status.node_gateway_connected { 200 } else { 503 }, &status)
        }
        ("GET", "/api/v1/status") => json_response(200, &router.status()),
        ("GET", "/api/v1/config") => json_response(200, &config),
        ("GET", "/api/v1/nodes") => json_response(200, &router.nodes()),
        ("GET", "/api/v1/subscribers") => json_response(200, &router.subscribers()),
        ("GET", "/api/v1/groups") => json_response(200, &router.groups()),
        ("GET", "/api/v1/routes") => json_response(200, &router.routes()),
        ("GET", "/api/v1/events/netcore") => {
            let limit = query_usize(&request, "limit", 100, 2_000);
            json_response(200, &router.recent_netcore_events(limit))
        }
        ("GET", "/api/v1/events") => {
            let limit = query_usize(&request, "limit", 100, 2_000);
            json_response(200, &router.recent_events(limit))
        }
        ("GET", "/api/v1/messages") => {
            let limit = query_usize(&request, "limit", 250, 5_000);
            let state = request
                .query
                .get("state")
                .and_then(|value| parse_message_state(value));
            json_response(200, &router.messages(limit, state))
        }
        ("GET", "/api/v1/application-outbox") => {
            let limit = query_usize(&request, "limit", 250, 2_000);
            let application = request.query.get("application").map(String::as_str);
            json_response(200, &router.application_outbox(application, limit))
        }
        ("GET", "/api/v1/export.json") => {
            let value = json!({
                "status": router.status(),
                "routes": router.routes(),
                "messages": router.messages(5000, None),
                "nodes": router.nodes(),
                "subscribers": router.subscribers(),
                "groups": router.groups(),
            });
            download_json("netcore-sds-router-export.json", &value)
        }
        ("GET", "/metrics") => text(
            "text/plain; version=0.0.4; charset=utf-8",
            router.metrics(),
        ),
        ("GET", "/openapi.json") => json_response(200, &openapi()),
        ("POST", "/api/v1/messages") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<MessageInput>(&request.body)
                .and_then(|input| router.create_message(input))
            {
                Ok((message, commands)) => {
                    dispatch_response(&gateway_tx, commands, 201, &message)
                }
                Err(error) => json_response(if error.starts_with("persistence_failed:") { 503 } else { 409 }, &json!({"error": error})),
            }
        }
        ("POST", "/api/v1/routes") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<RouteInput>(&request.body)
                .and_then(|input| router.create_route(input))
            {
                Ok(route) => json_response(201, &route),
                Err(error) => json_response(409, &json!({"error": error})),
            }
        }
        _ => dynamic_route(request, router, gateway_tx),
    }
}

// Was: Führt den Arbeitsschritt `dynamic_route` für dynamic Weiterleitung aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn dynamic_route(
    request: HttpRequest,
    router: SharedSdsRouter,
    gateway_tx: Sender<BackendRequest>,
) -> HttpResponse {
    let parts: Vec<&str> = request.path.trim_matches('/').split('/').collect();
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match (request.method.as_str(), parts.as_slice()) {
        ("GET", ["api", "v1", "idempotency", key]) => match router.idempotency_status(key) {
            Some(status) => json_response(200, &status),
            None => json_response(404, &json!({"error":"idempotency_key_not_found"})),
        },
        ("GET", ["api", "v1", "messages", id]) => match router.message(id) {
            Some(message) => json_response(200, &message),
            None => json_response(404, &json!({"error":"message not found"})),
        },
        ("POST", ["api", "v1", "messages", id, "retry"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match router.retry_message(id) {
                Ok(commands) => dispatch_response(
                    &gateway_tx,
                    commands,
                    202,
                    &json!({"message_id":id,"queued":true}),
                ),
                Err(error) => json_response(409, &json!({"error":error})),
            }
        }
        ("POST", ["api", "v1", "messages", id, "requeue"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match router.requeue_message(id) {
                Ok((message, commands)) => {
                    dispatch_response(&gateway_tx, commands, 202, &message)
                }
                Err(error) => json_response(409, &json!({"error":error})),
            }
        }
        ("POST", ["api", "v1", "messages", id, "cancel"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match router.cancel_message(id) {
                Ok(()) => json_response(200, &json!({"message_id":id,"cancelled":true})),
                Err(error) => json_response(409, &json!({"error":error})),
            }
        }
        ("DELETE", ["api", "v1", "messages", id]) => match router.delete_message(id) {
            Ok(()) => empty(204),
            Err(error) => json_response(409, &json!({"error":error})),
        },
        ("PUT", ["api", "v1", "routes", id]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<RouteInput>(&request.body)
                .and_then(|input| router.update_route(id, input))
            {
                Ok(route) => json_response(200, &route),
                Err(error) => json_response(409, &json!({"error":error})),
            }
        }
        ("DELETE", ["api", "v1", "routes", id]) => match router.delete_route(id) {
            Ok(()) => empty(204),
            Err(error) => json_response(409, &json!({"error":error})),
        },
        ("POST", ["api", "v1", "application-outbox", application, id, "ack"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<ApplicationAckInput>(&request.body)
                .and_then(|input| router.acknowledge_application(id, application, input))
            {
                Ok(message) => json_response(200, &message),
                Err(error) => json_response(409, &json!({"error":error})),
            }
        }
        _ => json_response(404, &json!({"error":"not found"})),
    }
}

// Was: Diese Funktion verteilt response.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn dispatch_response<T: Serialize>(
    tx: &Sender<BackendRequest>,
    commands: Vec<BackendRequest>,
    status: u16,
    value: &T,
) -> HttpResponse {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match dispatch_all(tx, commands) {
        Ok(()) => json_response(status, value),
        Err(error) => json_response(503, &json!({"error":error})),
    }
}

// Was: Diese Funktion verteilt all.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn dispatch_all(
    tx: &Sender<BackendRequest>,
    commands: Vec<BackendRequest>,
) -> Result<(), String> {
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for command in commands {
        tx.send(command)
            .map_err(|_| "node gateway worker is unavailable".to_string())?;
    }
    Ok(())
}

// Was: Diese Funktion liest und prüft JSON-Daten.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_json<T: serde::de::DeserializeOwned>(body: &[u8]) -> Result<T, String> {
    serde_json::from_slice(body).map_err(|error| format!("invalid JSON: {error}"))
}

// Was: Führt den Arbeitsschritt `query_usize` für query usize aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn query_usize(request: &HttpRequest, key: &str, default: usize, maximum: usize) -> usize {
    request
        .query
        .get(key)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(default)
        .min(maximum)
}

// Was: Führt den Arbeitsschritt `openapi` für openapi aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn openapi() -> serde_json::Value {
    json!({
        "openapi":"3.0.3",
        "info":{
            "title":"NetCore SDS Router",
            "version":env!("CARGO_PKG_VERSION"),
            "description":"OPEN LAB API. No authentication, no token and no TLS."
        },
        "paths":{
            "/api/v1/status":{"get":{}},
            "/api/v1/messages":{"get":{},"post":{"description":"Optional idempotency_key (1..160 URL-safe ASCII characters) durably deduplicates identical requests, including after message deletion. Conflicting payloads return 409. at_most_once=true requires a key and an individual recipient; it selects only the serving TBS (or one force_node), permits one dispatch and blocks manual retries/requeue. Optional expires_at is an absolute RFC3339 deadline, capped by ttl_secs; expired new submissions return 409. Delivery means TBS acceptance, not a handset receipt. Persist the original request before retrying after an HTTP timeout."}},
            "/api/v1/idempotency/{key}":{"get":{"description":"Read-only lookup of a durable submission key; returns message_id, retained and state. 404 means never submitted. retained=false means the message was deleted but the key remains consumed."}},
            "/api/v1/messages/{id}":{"get":{},"delete":{}},
            "/api/v1/messages/{id}/retry":{"post":{}},
            "/api/v1/messages/{id}/requeue":{"post":{}},
            "/api/v1/messages/{id}/cancel":{"post":{}},
            "/api/v1/routes":{"get":{},"post":{}},
            "/api/v1/routes/{id}":{"put":{},"delete":{}},
            "/api/v1/application-outbox":{"get":{}},
            "/api/v1/application-outbox/{application}/{id}/ack":{"post":{}},
            "/api/v1/nodes":{"get":{}},
            "/api/v1/subscribers":{"get":{}},
            "/api/v1/groups":{"get":{}},
            "/api/v1/events":{"get":{"description":"Legacy event records with embedded canonical event"}},
            "/api/v1/events/netcore":{"get":{"description":"Canonical netcore-event-v1 records"}},
            "/health/live":{"get":{}},
            "/health/ready":{"get":{}},
            "/metrics":{"get":{}}
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
            disposition: None,
        },
        Err(error) => HttpResponse {
            status: 500,
            content_type: "application/json; charset=utf-8",
            body: format!("{{\"error\":\"serialization failed: {error}\"}}")
                .into_bytes(),
            disposition: None,
        },
    }
}

// Was: Führt den Arbeitsschritt `download_json` für download JSON-Daten aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn download_json<T: Serialize>(name: &str, value: &T) -> HttpResponse {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match serde_json::to_vec_pretty(value) {
        Ok(body) => HttpResponse {
            status: 200,
            content_type: "application/json; charset=utf-8",
            body,
            disposition: Some(format!("attachment; filename=\"{name}\"")),
        },
        Err(error) => json_response(500, &json!({"error": error.to_string()})),
    }
}

// Was: Führt den Arbeitsschritt `text` für text aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn text(content_type: &'static str, value: String) -> HttpResponse {
    HttpResponse {
        status: 200,
        content_type,
        body: value.into_bytes(),
        disposition: None,
    }
}

// Was: Führt den Arbeitsschritt `html` für html aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn html(value: &str) -> HttpResponse {
    HttpResponse {
        status: 200,
        content_type: "text/html; charset=utf-8",
        body: value.as_bytes().to_vec(),
        disposition: None,
    }
}

// Was: Führt den Arbeitsschritt `empty` für empty aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn empty(status: u16) -> HttpResponse {
    HttpResponse {
        status,
        content_type: "text/plain; charset=utf-8",
        body: Vec::new(),
        disposition: None,
    }
}

// Was: Diese Funktion liest request.
// Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
fn read_request(stream: &mut TcpStream, max_body_bytes: usize) -> Result<HttpRequest, String> {
    stream
        .set_read_timeout(Some(std::time::Duration::from_secs(10)))
        .map_err(|error| error.to_string())?;
    let mut bytes = Vec::new();
    let mut buffer = [0_u8; 8192];
    let header_end;
    // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
    // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
    loop {
        let read = stream.read(&mut buffer).map_err(|error| error.to_string())?;
        if read == 0 {
            return Err("connection closed before request headers".to_string());
        }
        bytes.extend_from_slice(&buffer[..read]);
        if bytes.len() > max_body_bytes.saturating_add(64 * 1024) {
            return Err("request exceeds configured limit".to_string());
        }
        if let Some(index) = find_subslice(&bytes, b"\r\n\r\n") {
            header_end = index + 4;
            break;
        }
    }

    let (method, raw_path, content_length) = {
        let headers = String::from_utf8_lossy(&bytes[..header_end]);
        let mut lines = headers.lines();
        let first = lines.next().ok_or_else(|| "missing request line".to_string())?;
        let mut request_line = first.split_whitespace();
        let method = request_line
            .next()
            .ok_or_else(|| "missing method".to_string())?
            .to_string();
        let raw_path = request_line
            .next()
            .ok_or_else(|| "missing path".to_string())?
            .to_string();
        let content_length = lines
            .filter_map(|line| line.split_once(':'))
            .find(|(name, _)| name.eq_ignore_ascii_case("content-length"))
            .and_then(|(_, value)| value.trim().parse::<usize>().ok())
            .unwrap_or(0);
        (method, raw_path, content_length)
    };
    if content_length > max_body_bytes {
        return Err("request body exceeds configured limit".to_string());
    }
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    while bytes.len() < header_end + content_length {
        let read = stream.read(&mut buffer).map_err(|error| error.to_string())?;
        if read == 0 {
            return Err("connection closed before request body".to_string());
        }
        bytes.extend_from_slice(&buffer[..read]);
    }
    let (path, query) = parse_path_and_query(&raw_path);
    Ok(HttpRequest {
        method,
        path,
        query,
        body: bytes[header_end..header_end + content_length].to_vec(),
    })
}

// Was: Diese Funktion schreibt response.
// Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
fn write_response(stream: &mut TcpStream, response: HttpResponse) -> std::io::Result<()> {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    let reason = match response.status {
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        204 => "No Content",
        400 => "Bad Request",
        404 => "Not Found",
        405 => "Method Not Allowed",
        409 => "Conflict",
        413 => "Payload Too Large",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        _ => "OK",
    };
    let mut headers = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET,POST,PUT,DELETE,OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nX-NetCore-Security-Mode: open_lab\r\nConnection: close\r\n",
        response.status,
        reason,
        response.content_type,
        response.body.len()
    );
    if let Some(disposition) = response.disposition {
        headers.push_str(&format!("Content-Disposition: {disposition}\r\n"));
    }
    headers.push_str("\r\n");
    stream.write_all(headers.as_bytes())?;
    stream.write_all(&response.body)
}

// Was: Diese Funktion sucht subslice.
// Warum: Die Suchlogik bleibt damit wiederverwendbar und muss nicht an mehreren Stellen kopiert werden.
fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

// Was: Diese Funktion liest und prüft path and query.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_path_and_query(raw: &str) -> (String, HashMap<String, String>) {
    let (path, raw_query) = raw.split_once('?').unwrap_or((raw, ""));
    let query = raw_query
        .split('&')
        .filter(|value| !value.is_empty())
        .map(|item| item.split_once('=').unwrap_or((item, "")))
        .map(|(key, value)| (percent_decode(key), percent_decode(value)))
        .collect();
    (percent_decode(path), query)
}

// Was: Führt den Arbeitsschritt `percent_decode` für percent Dekodierung aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut output = Vec::new();
    let mut index = 0;
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            if let Ok(hex) = u8::from_str_radix(&value[index + 1..index + 3], 16) {
                output.push(hex);
                index += 3;
                continue;
            }
        }
        output.push(if bytes[index] == b'+' {
            b' '
        } else {
            bytes[index]
        });
        index += 1;
    }
    String::from_utf8_lossy(&output).into_owned()
}

// Was: Legt den festen Wert `INDEX_HTML` für index html fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const INDEX_HTML: &str = include_str!("../web-ui/index.html");
