#[path = "../../shared/web-ui/service-design.rs"]
mod service_design;

// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Anwendungsdienste wie TTS und externe Integrationen.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, Value};

use crate::config::ApplicationGatewayConfig;
use crate::model::{
    ActionInput, BackupInput, ConnectorInput, DispatchInput, RouteRuleInput, SecretSetInput,
    TemplateInput, TemplateRenderInput, TtsJobInput, TtsPublishInput,
};
use crate::state::SharedGateway;
use crate::worker;

// Was: Diese Funktion startet HTTP server.
// Warum: Länger laufende Arbeit blockiert dadurch nicht den aufrufenden Ablauf.
pub fn spawn_http_server(
    config: ApplicationGatewayConfig,
    gateway: SharedGateway,
) -> std::io::Result<thread::JoinHandle<()>> {
    let listener = TcpListener::bind(config.server.bind)?;
    tracing::info!(
        "Application Gateway WebUI/API listening on http://{}",
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
                    thread::spawn(move || {
                        if let Err(error) = handle_connection(stream, config, gateway) {
                            tracing::warn!("Application Gateway HTTP connection failed: {}", error);
                        }
                    });
                }
                Err(error) => tracing::warn!("Application Gateway HTTP accept failed: {}", error),
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
    extra_headers: Vec<(String, String)>,
}

// Was: Diese Funktion verarbeitet connection.
// Warum: Die Reaktion auf dieses Ereignis bleibt damit an einer Stelle nachvollziehbar.
fn handle_connection(
    mut stream: TcpStream,
    config: ApplicationGatewayConfig,
    gateway: SharedGateway,
) -> Result<(), String> {
    let request = read_request(&mut stream, config.server.max_body_bytes)?;
    let response = route(request, config, gateway);
    write_response(&mut stream, response).map_err(|error| error.to_string())
}

// Was: Diese Funktion leitet den vorgesehenen Arbeitsschritt.
// Warum: Nachrichten und Daten gelangen dadurch nachvollziehbar an das richtige Ziel.
fn route(
    request: HttpRequest,
    config: ApplicationGatewayConfig,
    gateway: SharedGateway,
) -> HttpResponse {
    if request.method == "OPTIONS" {
        return empty(204);
    }
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => html(&service_design::render(INDEX_HTML, "Application Gateway", "open-lab")),
        ("GET", "/health/live") => json_response(200, &json!({"status":"live"})),
        ("GET", "/health/ready") => {
            let status = gateway.status();
            json_response(if status.ready { 200 } else { 503 }, &status)
        }
        ("GET", "/api/v1/status") => json_response(200, &gateway.status()),
        ("GET", "/api/v1/config") => json_response(200, &gateway.redacted_config()),
        ("GET", "/api/v1/connectors") => json_response(200, &gateway.connectors()),
        ("POST", "/api/v1/connectors") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<ConnectorInput>(&request.body)
                .and_then(|input| gateway.create_connector(input))
            {
                Ok(value) => json_response(201, &value),
                Err(error) => conflict(error),
            }
        }
        ("GET", "/api/v1/secrets") => json_response(
            200,
            &gateway.secret_statuses(request.query.get("connector_id").map(String::as_str)),
        ),
        ("GET", "/api/v1/rules") => json_response(200, &gateway.rules()),
        ("POST", "/api/v1/rules") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<RouteRuleInput>(&request.body)
                .and_then(|input| gateway.create_rule(input))
            {
                Ok(value) => json_response(201, &value),
                Err(error) => conflict(error),
            }
        }
        ("GET", "/api/v1/templates") => json_response(200, &gateway.templates()),
        ("POST", "/api/v1/templates") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<TemplateInput>(&request.body)
                .and_then(|input| gateway.create_template(input))
            {
                Ok(value) => json_response(201, &value),
                Err(error) => conflict(error),
            }
        }
        ("GET", "/api/v1/events") => json_response(
            200,
            &gateway.events(
                request.query.get("state").map(String::as_str),
                request.query.get("source").map(String::as_str),
                query_usize(&request, "limit", 500, 5_000),
            ),
        ),
        ("POST", "/api/v1/events") | ("POST", "/api/v1/dispatch") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<DispatchInput>(&request.body).and_then(|input| gateway.dispatch(input))
            {
                Ok(value) => json_response(202, &value),
                Err(error) => conflict(error),
            }
        }
        ("GET", "/api/v1/deliveries") => json_response(
            200,
            &gateway.deliveries(
                request.query.get("state").map(String::as_str),
                request.query.get("connector_id").map(String::as_str),
                query_usize(&request, "limit", 500, 5_000),
            ),
        ),
        ("GET", "/api/v1/tts/jobs") => json_response(
            200,
            &gateway.tts_jobs(
                request.query.get("state").map(String::as_str),
                query_usize(&request, "limit", 500, 5_000),
            ),
        ),
        ("POST", "/api/v1/tts/jobs") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<TtsJobInput>(&request.body)
                .and_then(|input| gateway.create_tts_job(input))
            {
                Ok(value) => json_response(202, &value),
                Err(error) => conflict(error),
            }
        }
        ("GET", "/api/v1/audit") => {
            json_response(200, &gateway.audit(query_usize(&request, "limit", 500, 5_000)))
        }
        ("GET", "/api/v1/backups") => json_response(200, &gateway.backups()),
        ("POST", "/api/v1/backups") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<BackupInput>(&request.body)
                .and_then(|input| gateway.backup(input))
            {
                Ok(value) => json_response(201, &value),
                Err(error) => json_response(500, &json!({"error":error})),
            }
        }
        ("POST", "/api/v1/maintenance/tick") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<ActionInput>(&request.body)
                .and_then(|input| gateway.maintenance(input.actor))
            {
                Ok(value) => json_response(200, &value),
                Err(error) => json_response(500, &json!({"error":error})),
            }
        }
        ("POST", "/api/v1/maintenance/process-now") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match worker::build_client() {
                Ok(client) => {
                    worker::run_cycle(&config, &gateway, &client, 100);
                    json_response(200, &gateway.status())
                }
                Err(error) => json_response(500, &json!({"error":error})),
            }
        }
        ("GET", "/api/v1/export.json") => download(
            "netcore-application-gateway-export.json",
            "application/json",
            serde_json::to_vec_pretty(&gateway.export()).unwrap_or_default(),
        ),
        ("GET", "/metrics") => text(
            "text/plain; version=0.0.4; charset=utf-8",
            gateway.metrics(),
        ),
        ("GET", "/openapi.json") => json_response(200, &openapi()),
        _ => dynamic_route(request, config, gateway),
    }
}

// Was: Führt den Arbeitsschritt `dynamic_route` für dynamic Weiterleitung aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn dynamic_route(
    request: HttpRequest,
    _config: ApplicationGatewayConfig,
    gateway: SharedGateway,
) -> HttpResponse {
    let parts: Vec<&str> = request.path.trim_matches('/').split('/').collect();
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match (request.method.as_str(), parts.as_slice()) {
        ("GET", ["api", "v1", "connectors", connector_id]) => gateway
            .connector(connector_id)
            .map_or_else(|| not_found(format!("connector {connector_id} not found")), |value| json_response(200, &value)),
        ("PUT", ["api", "v1", "connectors", connector_id]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<ConnectorInput>(&request.body)
                .and_then(|input| gateway.update_connector(connector_id, input))
            {
                Ok(value) => json_response(200, &value),
                Err(error) => conflict(error),
            }
        }
        ("DELETE", ["api", "v1", "connectors", connector_id]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<ActionInput>(&request.body)
                .and_then(|input| gateway.delete_connector(connector_id, input))
            {
                Ok(()) => empty(204),
                Err(error) => conflict(error),
            }
        }
        ("POST", ["api", "v1", "connectors", connector_id, "test"]) => {
            let Some(connector) = gateway.connector_for_probe(connector_id) else {
                return not_found(format!("connector {connector_id} not found"));
            };
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match worker::build_client() {
                Ok(client) => {
                    let outcome = worker::test_connector(&client, &connector);
                    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                    match gateway.record_probe(outcome) {
                        Ok(value) => json_response(200, &value),
                        Err(error) => conflict(error),
                    }
                }
                Err(error) => json_response(500, &json!({"error":error})),
            }
        }
        ("GET", ["api", "v1", "connectors", connector_id, "secrets"]) => {
            json_response(200, &gateway.secret_statuses(Some(connector_id)))
        }
        ("POST", ["api", "v1", "connectors", connector_id, "secrets"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<SecretSetInput>(&request.body)
                .and_then(|input| gateway.set_secret(connector_id, input))
            {
                Ok(value) => json_response(200, &value),
                Err(error) => conflict(error),
            }
        }
        ("POST", ["api", "v1", "connectors", connector_id, action]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<ActionInput>(&request.body)
                .and_then(|input| gateway.connector_action(connector_id, action, input))
            {
                Ok(value) => json_response(200, &value),
                Err(error) => conflict(error),
            }
        }
        ("DELETE", ["api", "v1", "connectors", connector_id, "secrets", name]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<ActionInput>(&request.body)
                .and_then(|input| gateway.delete_secret(connector_id, name, input))
            {
                Ok(()) => empty(204),
                Err(error) => not_found(error),
            }
        }
        ("POST", ["api", "v1", "webhooks", connector_id]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<DispatchInput>(&request.body)
                .and_then(|input| gateway.ingest_webhook(connector_id, input))
            {
                Ok(value) => json_response(202, &value),
                Err(error) => conflict(error),
            }
        }
        ("PUT", ["api", "v1", "rules", rule_id]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<RouteRuleInput>(&request.body)
                .and_then(|input| gateway.update_rule(rule_id, input))
            {
                Ok(value) => json_response(200, &value),
                Err(error) => conflict(error),
            }
        }
        ("POST", ["api", "v1", "rules", rule_id, action]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<ActionInput>(&request.body)
                .and_then(|input| gateway.rule_action(rule_id, action, input))
            {
                Ok(value) => json_response(200, &value),
                Err(error) => conflict(error),
            }
        }
        ("DELETE", ["api", "v1", "rules", rule_id]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<ActionInput>(&request.body)
                .and_then(|input| gateway.delete_rule(rule_id, input))
            {
                Ok(()) => empty(204),
                Err(error) => conflict(error),
            }
        }
        ("PUT", ["api", "v1", "templates", template_id]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<TemplateInput>(&request.body)
                .and_then(|input| gateway.update_template(template_id, input))
            {
                Ok(value) => json_response(200, &value),
                Err(error) => conflict(error),
            }
        }
        ("POST", ["api", "v1", "templates", template_id, "render"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<TemplateRenderInput>(&request.body)
                .and_then(|input| gateway.render_template(template_id, input))
            {
                Ok(value) => json_response(200, &value),
                Err(error) => conflict(error),
            }
        }
        ("DELETE", ["api", "v1", "templates", template_id]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<ActionInput>(&request.body)
                .and_then(|input| gateway.delete_template(template_id, input))
            {
                Ok(()) => empty(204),
                Err(error) => conflict(error),
            }
        }
        ("GET", ["api", "v1", "deliveries", delivery_id]) => gateway
            .delivery(delivery_id)
            .map_or_else(|| not_found(format!("delivery {delivery_id} not found")), |value| json_response(200, &value)),
        ("POST", ["api", "v1", "deliveries", delivery_id, action]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<ActionInput>(&request.body)
                .and_then(|input| gateway.delivery_action(delivery_id, action, input))
            {
                Ok(value) => json_response(200, &value),
                Err(error) => conflict(error),
            }
        }
        ("POST", ["api", "v1", "tts", "jobs", job_id, "publish"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<TtsPublishInput>(&request.body)
                .and_then(|input| gateway.publish_tts_job(job_id, input))
            {
                Ok(value) => json_response(202, &value),
                Err(error) => conflict(error),
            }
        }
        ("GET", ["api", "v1", "tts", "jobs", job_id, "artifact"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match gateway.tts_artifact(job_id) {
                Ok((name, bytes)) => download(&name, "audio/wav", bytes),
                Err(error) => not_found(error),
            }
        }
        _ => not_found("route not found"),
    }
}

// Was: Führt den Arbeitsschritt `openapi` für openapi aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn openapi() -> Value {
    json!({
        "openapi":"3.1.0",
        "info":{
            "title":"NetCore-Tetra Application Gateway API",
            "version":"1.0.0",
            "description":"OPEN LAB management API without login, management tokens or TLS"
        },
        "paths":{
            "/api/v1/status":{"get":{}},
            "/api/v1/connectors":{"get":{},"post":{}},
            "/api/v1/connectors/{connector_id}":{"get":{},"put":{},"delete":{}},
            "/api/v1/connectors/{connector_id}/enable":{"post":{}},
            "/api/v1/connectors/{connector_id}/disable":{"post":{}},
            "/api/v1/connectors/{connector_id}/reset-circuit":{"post":{}},
            "/api/v1/connectors/{connector_id}/test":{"post":{}},
            "/api/v1/connectors/{connector_id}/secrets":{"get":{},"post":{}},
            "/api/v1/connectors/{connector_id}/secrets/{name}":{"delete":{}},
            "/api/v1/webhooks/{connector_id}":{"post":{}},
            "/api/v1/rules":{"get":{},"post":{}},
            "/api/v1/rules/{rule_id}":{"put":{},"delete":{}},
            "/api/v1/templates":{"get":{},"post":{}},
            "/api/v1/templates/{template_id}":{"put":{},"delete":{}},
            "/api/v1/templates/{template_id}/render":{"post":{}},
            "/api/v1/events":{"get":{},"post":{}},
            "/api/v1/deliveries":{"get":{}},
            "/api/v1/deliveries/{delivery_id}/{action}":{"post":{}},
            "/api/v1/tts/jobs":{"get":{},"post":{}},
            "/api/v1/tts/jobs/{job_id}/publish":{"post":{}},
            "/api/v1/tts/jobs/{job_id}/artifact":{"get":{}},
            "/api/v1/audit":{"get":{}},
            "/api/v1/backups":{"get":{},"post":{}},
            "/api/v1/maintenance/tick":{"post":{}},
            "/api/v1/maintenance/process-now":{"post":{}},
            "/metrics":{"get":{}},
            "/health/live":{"get":{}},
            "/health/ready":{"get":{}}
        }
    })
}

// Was: Diese Funktion liest request.
// Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
fn read_request(stream: &mut TcpStream, max_body_bytes: usize) -> Result<HttpRequest, String> {
    stream
        .set_read_timeout(Some(std::time::Duration::from_secs(10)))
        .map_err(|error| error.to_string())?;
    let mut bytes = Vec::new();
    let mut buffer = [0_u8; 8192];
    let header_end = loop {
        let count = stream.read(&mut buffer).map_err(|error| error.to_string())?;
        if count == 0 {
            return Err("connection closed before request headers".into());
        }
        bytes.extend_from_slice(&buffer[..count]);
        if bytes.len() > 64 * 1024 {
            return Err("request headers too large".into());
        }
        if let Some(position) = find_bytes(&bytes, b"\r\n\r\n") {
            break position + 4;
        }
    };
    let (method, target, content_length) = {
        let header = std::str::from_utf8(&bytes[..header_end]).map_err(|_| "request header is not UTF-8")?;
        let mut lines = header.split("\r\n");
        let first = lines.next().ok_or("missing request line")?;
        let mut request_line = first.split_whitespace();
        let method = request_line.next().ok_or("missing request method")?.to_string();
        let target = request_line.next().ok_or("missing request target")?.to_string();
        let content_length = lines
            .filter_map(|line| line.split_once(':'))
            .find(|(name, _)| name.eq_ignore_ascii_case("content-length"))
            .and_then(|(_, value)| value.trim().parse::<usize>().ok())
            .unwrap_or(0);
        (method, target, content_length)
    };
    if content_length > max_body_bytes {
        return Err(format!("request body exceeds {max_body_bytes} bytes"));
    }
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    while bytes.len() < header_end + content_length {
        let count = stream.read(&mut buffer).map_err(|error| error.to_string())?;
        if count == 0 {
            return Err("connection closed during request body".into());
        }
        bytes.extend_from_slice(&buffer[..count]);
    }
    let body = bytes[header_end..header_end + content_length].to_vec();
    let (raw_path, raw_query) = target.split_once('?').unwrap_or((target.as_str(), ""));
    Ok(HttpRequest {
        method,
        path: percent_decode(raw_path),
        query: parse_query(raw_query),
        body,
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
        413 => "Payload Too Large",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        _ => "OK",
    };
    write!(
        stream,
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Headers: Content-Type\r\nAccess-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS\r\n",
        response.status,
        reason,
        response.content_type,
        response.body.len()
    )?;
    if let Some(disposition) = response.disposition {
        write!(stream, "Content-Disposition: {disposition}\r\n")?;
    }
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for (name, value) in response.extra_headers {
        write!(stream, "{name}: {value}\r\n")?;
    }
    write!(stream, "\r\n")?;
    stream.write_all(&response.body)?;
    stream.flush()
}

// Was: Diese Funktion liest und prüft JSON-Daten.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_json<T: DeserializeOwned>(body: &[u8]) -> Result<T, String> {
    serde_json::from_slice(body).map_err(|error| format!("invalid JSON: {error}"))
}

// Was: Diese Funktion liest und prüft JSON-Daten or default.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_json_or_default<T: DeserializeOwned + Default>(body: &[u8]) -> Result<T, String> {
    if body.is_empty() {
        Ok(T::default())
    } else {
        parse_json(body)
    }
}

// Was: Führt den Arbeitsschritt `query_usize` für query usize aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn query_usize(request: &HttpRequest, name: &str, default: usize, max: usize) -> usize {
    request
        .query
        .get(name)
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(default)
        .min(max)
}

// Was: Diese Funktion liest und prüft query.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_query(raw: &str) -> HashMap<String, String> {
    raw.split('&')
        .filter(|item| !item.is_empty())
        .map(|item| {
            let (name, value) = item.split_once('=').unwrap_or((item, ""));
            (percent_decode(name), percent_decode(value))
        })
        .collect()
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
                let high = hex(bytes[index + 1]);
                let low = hex(bytes[index + 2]);
                if let (Some(high), Some(low)) = (high, low) {
                    output.push(high * 16 + low);
                    index += 3;
                    continue;
                }
                output.push(bytes[index]);
            }
            b'+' => output.push(b' '),
            byte => output.push(byte),
        }
        index += 1;
    }
    String::from_utf8_lossy(&output).into_owned()
}

// Was: Führt den Arbeitsschritt `hex` für hex aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn hex(value: u8) -> Option<u8> {
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

// Was: Führt den Arbeitsschritt `json_response` für JSON-Daten response aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn json_response<T: Serialize>(status: u16, value: &T) -> HttpResponse {
    HttpResponse {
        status,
        content_type: "application/json; charset=utf-8",
        body: serde_json::to_vec_pretty(value).unwrap_or_else(|_| b"{}".to_vec()),
        disposition: None,
        extra_headers: Vec::new(),
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
        extra_headers: Vec::new(),
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
        extra_headers: Vec::new(),
    }
}

// Was: Führt den Arbeitsschritt `download` für download aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn download(name: &str, content_type: &'static str, bytes: Vec<u8>) -> HttpResponse {
    HttpResponse {
        status: 200,
        content_type,
        body: bytes,
        disposition: Some(format!("attachment; filename=\"{}\"", name.replace('"', "_"))),
        extra_headers: Vec::new(),
    }
}

// Was: Führt den Arbeitsschritt `empty` für empty aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn empty(status: u16) -> HttpResponse {
    HttpResponse {
        status,
        content_type: "application/json; charset=utf-8",
        body: Vec::new(),
        disposition: None,
        extra_headers: Vec::new(),
    }
}

// Was: Führt den Arbeitsschritt `conflict` für conflict aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn conflict(error: impl ToString) -> HttpResponse {
    json_response(409, &json!({"error":error.to_string()}))
}

// Was: Führt den Arbeitsschritt `not_found` für not found aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn not_found(error: impl ToString) -> HttpResponse {
    json_response(404, &json!({"error":error.to_string()}))
}

// Was: Legt den festen Wert `INDEX_HTML` für index html fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const INDEX_HTML: &str = include_str!("../web-ui/index.html");
