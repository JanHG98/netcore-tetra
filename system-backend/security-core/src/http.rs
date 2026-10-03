#[path = "../../shared/web-ui/service-design.rs"]
mod service_design;

// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Sicherheitsrichtlinien und Authentifizierungsabläufe.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use serde::Serialize;
use serde_json::json;

use crate::config::{OPERATING_MODE_AUTHORITATIVE, SecurityCoreConfig};
use crate::protocol::{
    AlarmAckInput, AuthenticationResponseInput, AuthenticationStartInput, DisableInput,
    EdgeActionAckInput, EdgeClaimInput, PolicyInput, ProfileInput, RevokeInput,
};
use crate::state::SharedSecurityCore;

// Was: Diese Funktion startet HTTP server.
// Warum: Länger laufende Arbeit blockiert dadurch nicht den aufrufenden Ablauf.
pub fn spawn_http_server(
    config: SecurityCoreConfig,
    core: SharedSecurityCore,
) -> std::io::Result<thread::JoinHandle<()>> {
    let listener = TcpListener::bind(config.server.bind)?;
    tracing::info!(
        "Security Core WebUI/API listening on http://{}",
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
                    let core = core.clone();
                    let config = config.clone();
                    thread::spawn(move || {
                        if let Err(error) = handle_connection(stream, core, config) {
                            tracing::warn!("Security Core HTTP connection failed: {}", error);
                        }
                    });
                }
                Err(error) => tracing::warn!("Security Core HTTP accept failed: {}", error),
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
    core: SharedSecurityCore,
    config: SecurityCoreConfig,
) -> Result<(), String> {
    let request = read_request(&mut stream, config.limits.max_body_bytes)?;
    let response = route(request, core, config);
    write_response(&mut stream, response).map_err(|error| error.to_string())
}

// Was: Diese Funktion leitet den vorgesehenen Arbeitsschritt.
// Warum: Nachrichten und Daten gelangen dadurch nachvollziehbar an das richtige Ziel.
fn route(
    request: HttpRequest,
    core: SharedSecurityCore,
    config: SecurityCoreConfig,
) -> HttpResponse {
    if request.method == "OPTIONS" {
        return empty(204);
    }

    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => html(&service_design::render(INDEX_HTML, "Security Core", "open-lab")),
        ("GET", "/health/live") => json_response(200, &json!({"status":"live"})),
        ("GET", "/health/ready") => {
            let status = core.status();
            let ready = status.operating_mode != OPERATING_MODE_AUTHORITATIVE
                || !config.node_gateway.observe_nodes
                || status.node_gateway_connected;
            json_response(if ready { 200 } else { 503 }, &status)
        }
        ("GET", "/api/v1/status") => json_response(200, &core.status()),
        ("GET", "/api/v1/config") => json_response(200, &core.redacted_config()),
        ("GET", "/api/v1/policy") => json_response(200, &core.policy()),
        ("POST", "/api/v1/policy") => match parse_json::<PolicyInput>(&request.body)
            .and_then(|input| core.update_policy(input, "open-lab-api"))
        {
            Ok(policy) => json_response(200, &policy),
            Err(error) => json_response(409, &json!({"error":error})),
        },
        ("GET", "/api/v1/profiles") => json_response(200, &core.profiles()),
        ("POST", "/api/v1/profiles") => match parse_json::<ProfileInput>(&request.body)
            .and_then(|input| core.upsert_profile(input, "open-lab-api"))
        {
            Ok(profile) => json_response(201, &profile),
            Err(error) => json_response(409, &json!({"error":error})),
        },
        ("GET", "/api/v1/subscribers") => json_response(200, &core.subscribers()),
        ("GET", "/api/v1/auth-contexts") => json_response(200, &core.auth_contexts()),
        ("POST", "/api/v1/auth/start") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<AuthenticationStartInput>(&request.body)
                .and_then(|input| core.start_authentication(input))
            {
                Ok(context) => json_response(202, &context),
                Err(error) => json_response(409, &json!({"error":error})),
            }
        }
        ("GET", "/api/v1/dck-contexts") => json_response(200, &core.dck_contexts()),
        ("GET", "/api/v1/actions") => json_response(200, &core.actions()),
        ("POST", "/api/v1/edge/actions/claim") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<EdgeClaimInput>(&request.body)
                .and_then(|input| core.claim_edge_actions(input))
            {
                Ok(actions) => json_response(200, &json!({
                    "protocol_version":crate::protocol::EDGE_PROTOCOL_VERSION,
                    "actions":actions,
                    "warning":"This edge-only endpoint may return ephemeral challenge or DCK material. Do not expose it to operator browsers."
                })),
                Err(error) => json_response(409, &json!({"error":error})),
            }
        }
        ("GET", "/api/v1/alarms") => json_response(200, &core.alarms()),
        ("GET", "/api/v1/nodes") => json_response(200, &core.nodes()),
        ("GET", "/api/v1/audit") => {
            let limit = query_usize(&request, "limit", 500, 10_000);
            json_response(200, &core.audit(limit))
        }
        ("POST", "/api/v1/maintenance/expire") => match core.maintenance_tick() {
            Ok(status) => json_response(200, &status),
            Err(error) => json_response(500, &json!({"error":error})),
        },
        ("POST", "/api/v1/maintenance/backup") => match core.backup() {
            Ok(path) => json_response(200, &json!({"backup_path":path})),
            Err(error) => json_response(500, &json!({"error":error})),
        },
        ("GET", "/api/v1/export.json") => {
            download_json("netcore-security-core-export.json", &core.export())
        }
        ("GET", "/metrics") => text(
            "text/plain; version=0.0.4; charset=utf-8",
            core.metrics(),
        ),
        ("GET", "/openapi.json") => json_response(200, &openapi()),
        _ => dynamic_route(request, core),
    }
}

// Was: Führt den Arbeitsschritt `dynamic_route` für dynamic Weiterleitung aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn dynamic_route(request: HttpRequest, core: SharedSecurityCore) -> HttpResponse {
    let parts: Vec<&str> = request.path.trim_matches('/').split('/').collect();
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match (request.method.as_str(), parts.as_slice()) {
        ("GET", ["api", "v1", "profiles", issi]) => match parse_issi(issi)
            .ok()
            .and_then(|value| core.profile(value))
        {
            Some(profile) => json_response(200, &profile),
            None => json_response(404, &json!({"error":"profile not found"})),
        },
        ("DELETE", ["api", "v1", "profiles", issi]) => match parse_issi(issi)
            .and_then(|value| core.delete_profile(value, "open-lab-api"))
        {
            Ok(()) => empty(204),
            Err(error) => json_response(404, &json!({"error":error})),
        },
        ("POST", ["api", "v1", "profiles", issi, "disable"]) => {
            profile_disable_route(&request.body, &core, issi, true)
        }
        ("POST", ["api", "v1", "profiles", issi, "enable"]) => {
            profile_disable_route(&request.body, &core, issi, false)
        }
        ("GET", ["api", "v1", "auth-contexts", id]) => match core.auth_context(id) {
            Some(context) => json_response(200, &context),
            None => json_response(404, &json!({"error":"authentication context not found"})),
        },
        ("POST", ["api", "v1", "auth-contexts", id, "response"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<AuthenticationResponseInput>(&request.body)
                .and_then(|input| core.submit_authentication_response(id, input))
            {
                Ok(context) => json_response(200, &context),
                Err(error) => json_response(409, &json!({"error":error})),
            }
        }
        ("POST", ["api", "v1", "auth-contexts", id, "revoke"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<RevokeInput>(&request.body)
                .and_then(|input| core.revoke_authentication(id, input))
            {
                Ok(context) => json_response(200, &context),
                Err(error) => json_response(409, &json!({"error":error})),
            }
        }
        ("POST", ["api", "v1", "dck-contexts", id, "revoke"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<RevokeInput>(&request.body)
                .and_then(|input| core.revoke_dck(id, input))
            {
                Ok(context) => json_response(200, &context),
                Err(error) => json_response(409, &json!({"error":error})),
            }
        }
        ("POST", ["api", "v1", "edge", "actions", id, "ack"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<EdgeActionAckInput>(&request.body)
                .and_then(|input| core.acknowledge_edge_action(id, input))
            {
                Ok(action) => json_response(200, &action),
                Err(error) => json_response(409, &json!({"error":error})),
            }
        }
        ("POST", ["api", "v1", "alarms", id, "ack"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<AlarmAckInput>(&request.body)
                .and_then(|input| core.acknowledge_alarm(id, input))
            {
                Ok(alarm) => json_response(200, &alarm),
                Err(error) => json_response(409, &json!({"error":error})),
            }
        }
        _ => json_response(404, &json!({"error":"not found"})),
    }
}

// Was: Führt den Arbeitsschritt `profile_disable_route` für profile disable Weiterleitung aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn profile_disable_route(
    body: &[u8],
    core: &SharedSecurityCore,
    issi: &str,
    disabled: bool,
) -> HttpResponse {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match parse_issi(issi)
        .and_then(|value| parse_json_or_default::<DisableInput>(body).map(|input| (value, input)))
        .and_then(|(value, input)| core.set_disabled(value, disabled, input))
    {
        Ok(profile) => json_response(200, &profile),
        Err(error) => json_response(409, &json!({"error":error})),
    }
}

// Was: Diese Funktion liest und prüft Teilnehmerkennung (ISSI).
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_issi(value: &str) -> Result<u32, String> {
    let issi = value
        .parse::<u32>()
        .map_err(|error| format!("invalid ISSI: {error}"))?;
    if issi > 0x00ff_ffff {
        return Err("ISSI must fit into 24 bits".to_string());
    }
    Ok(issi)
}

// Was: Diese Funktion liest und prüft JSON-Daten.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_json<T: serde::de::DeserializeOwned>(body: &[u8]) -> Result<T, String> {
    serde_json::from_slice(body).map_err(|error| format!("invalid JSON: {error}"))
}

// Was: Diese Funktion liest und prüft JSON-Daten or default.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_json_or_default<T>(body: &[u8]) -> Result<T, String>
where
    T: serde::de::DeserializeOwned + Default,
{
    if body.is_empty() {
        Ok(T::default())
    } else {
        parse_json(body)
    }
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
            "title":"NetCore Security Core",
            "version":env!("CARGO_PKG_VERSION"),
            "description":"OPEN LAB authentication orchestration, security policy, disable/enable, DCK metadata and security audit API. The lab_hmac_sha256 provider is not a TETRA TA algorithm. Raw secrets are absent from normal management responses."
        },
        "paths":{
            "/api/v1/status":{"get":{}},
            "/api/v1/config":{"get":{}},
            "/api/v1/policy":{"get":{},"post":{}},
            "/api/v1/profiles":{"get":{},"post":{}},
            "/api/v1/profiles/{issi}":{"get":{},"delete":{}},
            "/api/v1/profiles/{issi}/disable":{"post":{}},
            "/api/v1/profiles/{issi}/enable":{"post":{}},
            "/api/v1/subscribers":{"get":{}},
            "/api/v1/auth/start":{"post":{}},
            "/api/v1/auth-contexts":{"get":{}},
            "/api/v1/auth-contexts/{id}":{"get":{}},
            "/api/v1/auth-contexts/{id}/response":{"post":{}},
            "/api/v1/auth-contexts/{id}/revoke":{"post":{}},
            "/api/v1/dck-contexts":{"get":{}},
            "/api/v1/dck-contexts/{id}/revoke":{"post":{}},
            "/api/v1/actions":{"get":{}},
            "/api/v1/edge/actions/claim":{"post":{}},
            "/api/v1/edge/actions/{id}/ack":{"post":{}},
            "/api/v1/alarms":{"get":{}},
            "/api/v1/alarms/{id}/ack":{"post":{}},
            "/api/v1/audit":{"get":{}},
            "/api/v1/nodes":{"get":{}},
            "/api/v1/maintenance/expire":{"post":{}},
            "/api/v1/maintenance/backup":{"post":{}},
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
fn download_json<T: Serialize>(filename: &str, value: &T) -> HttpResponse {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match serde_json::to_vec_pretty(value) {
        Ok(body) => HttpResponse {
            status: 200,
            content_type: "application/json; charset=utf-8",
            body,
            disposition: Some(format!("attachment; filename=\"{filename}\"")),
        },
        Err(error) => json_response(500, &json!({"error":error.to_string()})),
    }
}

// Was: Führt den Arbeitsschritt `html` für html aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn html(body: &str) -> HttpResponse {
    HttpResponse {
        status: 200,
        content_type: "text/html; charset=utf-8",
        body: body.as_bytes().to_vec(),
        disposition: None,
    }
}

// Was: Führt den Arbeitsschritt `text` für text aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn text(content_type: &'static str, body: String) -> HttpResponse {
    HttpResponse {
        status: 200,
        content_type,
        body: body.into_bytes(),
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
    let mut buffer = Vec::with_capacity(8_192);
    let mut temporary = [0_u8; 4_096];
    let header_end;
    // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
    // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
    loop {
        let count = stream
            .read(&mut temporary)
            .map_err(|error| format!("read request: {error}"))?;
        if count == 0 {
            return Err("connection closed before request was complete".to_string());
        }
        buffer.extend_from_slice(&temporary[..count]);
        if let Some(position) = find_bytes(&buffer, b"\r\n\r\n") {
            header_end = position + 4;
            break;
        }
        if buffer.len() > 65_536 {
            return Err("request header is too large".to_string());
        }
    }

    let header_text = std::str::from_utf8(&buffer[..header_end])
        .map_err(|error| format!("request header is not UTF-8: {error}"))?;
    let mut lines = header_text.split("\r\n");
    let request_line = lines.next().ok_or_else(|| "missing request line".to_string())?;
    let mut parts = request_line.split_whitespace();
    let method = parts
        .next()
        .ok_or_else(|| "missing request method".to_string())?
        .to_string();
    let target = parts
        .next()
        .ok_or_else(|| "missing request target".to_string())?
        .to_string();
    let content_length = lines
        .filter_map(|line| line.split_once(':'))
        .find(|(name, _)| name.eq_ignore_ascii_case("content-length"))
        .map(|(_, value)| value.trim().parse::<usize>())
        .transpose()
        .map_err(|error| format!("invalid Content-Length: {error}"))?
        .unwrap_or(0);
    if content_length > max_body_bytes {
        return Err(format!("request body exceeds {max_body_bytes} bytes"));
    }
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    while buffer.len() < header_end + content_length {
        let count = stream
            .read(&mut temporary)
            .map_err(|error| format!("read request body: {error}"))?;
        if count == 0 {
            return Err("connection closed before request body was complete".to_string());
        }
        buffer.extend_from_slice(&temporary[..count]);
    }
    let body = buffer[header_end..header_end + content_length].to_vec();
    let (path, query) = split_target(&target);
    Ok(HttpRequest {
        method,
        path,
        query,
        body,
    })
}

// Was: Führt den Arbeitsschritt `split_target` für split target aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn split_target(target: &str) -> (String, HashMap<String, String>) {
    let (path, raw_query) = target.split_once('?').unwrap_or((target, ""));
    let mut query = HashMap::new();
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for pair in raw_query.split('&').filter(|value| !value.is_empty()) {
        let (key, value) = pair.split_once('=').unwrap_or((pair, ""));
        query.insert(percent_decode(key), percent_decode(value));
    }
    (percent_decode(path), query)
}

// Was: Führt den Arbeitsschritt `percent_decode` für percent Dekodierung aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    while index < bytes.len() {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match bytes[index] {
            b'%' if index + 2 < bytes.len() => {
                let high = hex_value(bytes[index + 1]);
                let low = hex_value(bytes[index + 2]);
                if let (Some(high), Some(low)) = (high, low) {
                    output.push((high << 4) | low);
                    index += 3;
                    continue;
                }
                output.push(bytes[index]);
                index += 1;
            }
            b'+' => {
                output.push(b' ');
                index += 1;
            }
            other => {
                output.push(other);
                index += 1;
            }
        }
    }
    String::from_utf8_lossy(&output).into_owned()
}

// Was: Führt den Arbeitsschritt `hex_value` für hex value aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn hex_value(value: u8) -> Option<u8> {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

// Was: Diese Funktion sucht bytes.
// Warum: Die Suchlogik bleibt damit wiederverwendbar und muss nicht an mehreren Stellen kopiert werden.
fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
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
        413 => "Payload Too Large",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        _ => "Response",
    };
    let mut headers = format!(
        concat!(
            "HTTP/1.1 {} {}\r\n",
            "Content-Type: {}\r\n",
            "Content-Length: {}\r\n",
            "Access-Control-Allow-Origin: *\r\n",
            "Access-Control-Allow-Methods: GET, POST, DELETE, OPTIONS\r\n",
            "Access-Control-Allow-Headers: Content-Type\r\n",
            "X-NetCore-Security-Mode: open-lab\r\n",
            "X-Content-Type-Options: nosniff\r\n",
            "Content-Security-Policy: default-src 'self'; img-src 'self' data:; script-src 'unsafe-inline'; style-src 'unsafe-inline'; connect-src 'self'\r\n"
        ),
        response.status,
        reason,
        response.content_type,
        response.body.len()
    );
    if let Some(disposition) = response.disposition {
        headers.push_str(&format!("Content-Disposition: {disposition}\r\n"));
    }
    headers.push_str("Connection: close\r\n\r\n");
    stream.write_all(headers.as_bytes())?;
    stream.write_all(&response.body)?;
    stream.flush()
}

// Was: Legt den festen Wert `INDEX_HTML` für index html fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const INDEX_HTML: &str = include_str!("../web-ui/index.html");
