// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für die Kopplung von TETRA-Paketdaten an IP-Netze.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

#[path = "../../shared/web-ui/service-design.rs"]
mod service_design;

use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use serde::Serialize;
use serde_json::json;

use crate::config::IpGatewayConfig;
use crate::kernel;
use crate::protocol::{
    BlockAddressInput, CaptureStartInput, FirewallRuleInput, NatRuleInput, ReconcileInput,
    RouteRuleInput, StaticDnsInput,
};
use crate::runtime::RuntimeHandle;
use crate::state::SharedGateway;

// Was: Diese Funktion startet HTTP server.
// Warum: Länger laufende Arbeit blockiert dadurch nicht den aufrufenden Ablauf.
pub fn spawn_http_server(
    config: IpGatewayConfig,
    gateway: SharedGateway,
    runtime: RuntimeHandle,
) -> std::io::Result<thread::JoinHandle<()>> {
    let listener = TcpListener::bind(config.server.bind)?;
    tracing::info!(
        "IP Gateway WebUI/API listening on http://{}",
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
                    let config = config.clone();
                    let gateway = gateway.clone();
                    let runtime = runtime.clone();
                    thread::spawn(move || {
                        if let Err(error) = handle_connection(stream, config, gateway, runtime) {
                            tracing::warn!("HTTP connection failed: {error}");
                        }
                    });
                }
                Err(error) => tracing::warn!("HTTP accept failed: {error}"),
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
    config: IpGatewayConfig,
    gateway: SharedGateway,
    runtime: RuntimeHandle,
) -> Result<(), String> {
    let request = read_request(&mut stream, config.limits.max_body_bytes)?;
    let response = route(request, config, gateway, runtime);
    write_response(&mut stream, response).map_err(|error| error.to_string())
}

// Was: Diese Funktion leitet den vorgesehenen Arbeitsschritt.
// Warum: Nachrichten und Daten gelangen dadurch nachvollziehbar an das richtige Ziel.
fn route(
    request: HttpRequest,
    config: IpGatewayConfig,
    gateway: SharedGateway,
    runtime: RuntimeHandle,
) -> HttpResponse {
    if request.method == "OPTIONS" {
        return empty(204);
    }
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => html(&service_design::render(INDEX_HTML, "IP Gateway", "open-lab")),
        ("GET", "/health/live") => json_response(200, &json!({"status":"live"})),
        ("GET", "/health/ready") => {
            let status = gateway.status();
            let ready = if status.authoritative {
                status.packet_core_connected && status.tun_open && status.kernel_last_error.is_none()
            } else {
                status.packet_core_connected
            };
            json_response(if ready { 200 } else { 503 }, &status)
        }
        ("GET", "/api/v1/status") => json_response(200, &gateway.status()),
        ("GET", "/api/v1/config") => json_response(200, &config),
        ("GET", "/api/v1/contexts") => json_response(200, &gateway.contexts()),
        ("GET", "/api/v1/routes") => json_response(200, &gateway.routes()),
        ("GET", "/api/v1/nat") => json_response(200, &gateway.nat_rules()),
        ("GET", "/api/v1/firewall") => json_response(200, &gateway.firewall_rules()),
        ("GET", "/api/v1/dns") => json_response(200, &gateway.dns_records()),
        ("GET", "/api/v1/blocked") => json_response(200, &gateway.blocked_addresses()),
        ("GET", "/api/v1/flows") => {
            let limit = query_usize(&request, "limit", 500, 10_000);
            json_response(200, &gateway.flows(limit))
        }
        ("GET", "/api/v1/captures") => json_response(200, &gateway.captures()),
        ("GET", "/api/v1/events") => {
            let limit = query_usize(&request, "limit", 250, 5_000);
            json_response(200, &gateway.recent_events(limit))
        }
        ("GET", "/api/v1/kernel/plan") => {
            json_response(200, &kernel::build_plan(&config, &gateway.kernel_snapshot()))
        }
        ("POST", "/api/v1/kernel/reconcile") => {
            let input = if request.body.is_empty() {
                Ok(ReconcileInput { force: false })
            } else {
                parse_json::<ReconcileInput>(&request.body)
            };
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match input {
                Ok(input) => match runtime.reconcile() {
                    Ok(plan) => json_response(202, &json!({"force":input.force,"plan":plan})),
                    Err(error) => json_response(503, &json!({"error":error})),
                },
                Err(error) => json_response(400, &json!({"error":error})),
            }
        }
        ("POST", "/api/v1/routes") => match parse_json::<RouteRuleInput>(&request.body)
            .and_then(|input| gateway.upsert_route(None, input))
        {
            Ok(record) => json_response(201, &record),
            Err(error) => json_response(409, &json!({"error":error})),
        },
        ("POST", "/api/v1/nat") => match parse_json::<NatRuleInput>(&request.body)
            .and_then(|input| gateway.upsert_nat(None, input))
        {
            Ok(record) => json_response(201, &record),
            Err(error) => json_response(409, &json!({"error":error})),
        },
        ("POST", "/api/v1/firewall") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<FirewallRuleInput>(&request.body)
                .and_then(|input| gateway.upsert_firewall(None, input))
            {
                Ok(record) => json_response(201, &record),
                Err(error) => json_response(409, &json!({"error":error})),
            }
        }
        ("POST", "/api/v1/dns") => match parse_json::<StaticDnsInput>(&request.body)
            .and_then(|input| gateway.upsert_dns(None, input))
        {
            Ok(record) => json_response(201, &record),
            Err(error) => json_response(409, &json!({"error":error})),
        },
        ("POST", "/api/v1/blocked") => match parse_json::<BlockAddressInput>(&request.body)
            .and_then(|input| gateway.block_address(input))
        {
            Ok(record) => json_response(201, &record),
            Err(error) => json_response(409, &json!({"error":error})),
        },
        ("POST", "/api/v1/captures") => match parse_json::<CaptureStartInput>(&request.body)
            .and_then(|input| gateway.start_capture(input))
        {
            Ok(record) => json_response(201, &record),
            Err(error) => json_response(409, &json!({"error":error})),
        },
        ("GET", "/api/v1/export.json") => {
            download_json("netcore-ip-gateway-export.json", &gateway.export())
        }
        ("GET", "/metrics") => text(
            "text/plain; version=0.0.4; charset=utf-8",
            gateway.metrics(),
        ),
        ("GET", "/openapi.json") => json_response(200, &openapi()),
        _ => dynamic_route(request, gateway),
    }
}

// Was: Führt den Arbeitsschritt `dynamic_route` für dynamic Weiterleitung aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn dynamic_route(request: HttpRequest, gateway: SharedGateway) -> HttpResponse {
    let parts: Vec<&str> = request.path.trim_matches('/').split('/').collect();
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match (request.method.as_str(), parts.as_slice()) {
        ("PUT", ["api", "v1", "routes", id]) => match parse_json::<RouteRuleInput>(&request.body)
            .and_then(|input| gateway.upsert_route(Some(id), input))
        {
            Ok(record) => json_response(200, &record),
            Err(error) => json_response(409, &json!({"error":error})),
        },
        ("DELETE", ["api", "v1", "routes", id]) => result_empty(gateway.delete_route(id)),
        ("PUT", ["api", "v1", "nat", id]) => match parse_json::<NatRuleInput>(&request.body)
            .and_then(|input| gateway.upsert_nat(Some(id), input))
        {
            Ok(record) => json_response(200, &record),
            Err(error) => json_response(409, &json!({"error":error})),
        },
        ("DELETE", ["api", "v1", "nat", id]) => result_empty(gateway.delete_nat(id)),
        ("PUT", ["api", "v1", "firewall", id]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<FirewallRuleInput>(&request.body)
                .and_then(|input| gateway.upsert_firewall(Some(id), input))
            {
                Ok(record) => json_response(200, &record),
                Err(error) => json_response(409, &json!({"error":error})),
            }
        }
        ("DELETE", ["api", "v1", "firewall", id]) => {
            result_empty(gateway.delete_firewall(id))
        }
        ("PUT", ["api", "v1", "dns", id]) => match parse_json::<StaticDnsInput>(&request.body)
            .and_then(|input| gateway.upsert_dns(Some(id), input))
        {
            Ok(record) => json_response(200, &record),
            Err(error) => json_response(409, &json!({"error":error})),
        },
        ("DELETE", ["api", "v1", "dns", id]) => result_empty(gateway.delete_dns(id)),
        ("DELETE", ["api", "v1", "blocked", address]) => {
            result_empty(gateway.unblock_address(address))
        }
        ("POST", ["api", "v1", "captures", id, "stop"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match gateway.stop_capture(id) {
                Ok(record) => json_response(200, &record),
                Err(error) => json_response(404, &json!({"error":error})),
            }
        }
        ("DELETE", ["api", "v1", "captures", id]) => {
            result_empty(gateway.delete_capture(id))
        }
        ("GET", ["api", "v1", "captures", id, "download"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match gateway.capture(id) {
                Some(capture) => match fs::read(&capture.path) {
                    Ok(body) => HttpResponse {
                        status: 200,
                        content_type: "application/vnd.tcpdump.pcap",
                        body,
                        disposition: Some(format!(
                            "attachment; filename=\"{}.pcap\"",
                            safe_filename(&capture.name)
                        )),
                    },
                    Err(error) => json_response(404, &json!({"error":error.to_string()})),
                },
                None => json_response(404, &json!({"error":"capture not found"})),
            }
        }
        _ => json_response(404, &json!({"error":"not found"})),
    }
}

// Was: Führt den Arbeitsschritt `result_empty` für result empty aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn result_empty(result: Result<(), String>) -> HttpResponse {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match result {
        Ok(()) => empty(204),
        Err(error) => json_response(404, &json!({"error":error})),
    }
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
            "title":"NetCore IP Gateway",
            "version":env!("CARGO_PKG_VERSION"),
            "description":"OPEN LAB Layer-3 TUN, routing, NAT, firewall, DNS, WAP/test and packet-capture API. No authentication, token or TLS."
        },
        "paths":{
            "/api/v1/status":{"get":{}},
            "/api/v1/contexts":{"get":{}},
            "/api/v1/routes":{"get":{},"post":{}},
            "/api/v1/routes/{id}":{"put":{},"delete":{}},
            "/api/v1/nat":{"get":{},"post":{}},
            "/api/v1/nat/{id}":{"put":{},"delete":{}},
            "/api/v1/firewall":{"get":{},"post":{}},
            "/api/v1/firewall/{id}":{"put":{},"delete":{}},
            "/api/v1/dns":{"get":{},"post":{}},
            "/api/v1/dns/{id}":{"put":{},"delete":{}},
            "/api/v1/blocked":{"get":{},"post":{}},
            "/api/v1/blocked/{address}":{"delete":{}},
            "/api/v1/flows":{"get":{}},
            "/api/v1/captures":{"get":{},"post":{}},
            "/api/v1/captures/{id}/stop":{"post":{}},
            "/api/v1/captures/{id}/download":{"get":{}},
            "/api/v1/kernel/plan":{"get":{}},
            "/api/v1/kernel/reconcile":{"post":{}},
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
        Err(error) => json_response(500, &json!({"error":error.to_string()})),
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
        409 => "Conflict",
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
    haystack.windows(needle.len()).position(|window| window == needle)
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
        output.push(if bytes[index] == b'+' { b' ' } else { bytes[index] });
        index += 1;
    }
    String::from_utf8_lossy(&output).into_owned()
}

// Was: Führt den Arbeitsschritt `safe_filename` für safe filename aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn safe_filename(value: &str) -> String {
    let value: String = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.') {
                character
            } else {
                '_'
            }
        })
        .collect();
    if value.is_empty() {
        "capture".to_string()
    } else {
        value
    }
}

// Was: Legt den festen Wert `INDEX_HTML` für index html fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const INDEX_HTML: &str = include_str!("../web-ui/index.html");
