#[path = "../../shared/web-ui/service-design.rs"]
mod service_design;

// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für gespeicherte Aufzeichnungen, TTS- und Mediendateien.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::thread;
use std::time::Instant;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, Value};

use crate::config::MediaLibraryConfig;
use crate::model::{
    ActionInput, ApprovalInput, AssetUpdateInput, DispatchInput, ImportUrlInput,
    RecorderImportInput, UploadInput,
};
use crate::state::SharedLibrary;
use crate::tts::{TtsGenerateInput, TtsService, TtsTemplateDeleteInput, TtsTemplateInput};
use crate::worker;

// Was: Diese Funktion startet HTTP server.
// Warum: Länger laufende Arbeit blockiert dadurch nicht den aufrufenden Ablauf.
pub fn spawn_http_server(
    config: MediaLibraryConfig,
    library: SharedLibrary,
    tts: TtsService,
) -> std::io::Result<thread::JoinHandle<()>> {
    let listener = TcpListener::bind(config.server.bind)?;
    tracing::info!("Media Library WebUI/API listening on http://{}", config.server.bind);
    Ok(thread::spawn(move || {
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for stream in listener.incoming() {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match stream {
                Ok(stream) => {
                    let config = config.clone();
                    let library = library.clone();
                    let tts = tts.clone();
                    thread::spawn(move || {
                        if let Err(error) = handle_connection(stream, config, library, tts) {
                            tracing::warn!("Media Library HTTP connection failed: {error}");
                        }
                    });
                }
                Err(error) => tracing::warn!("Media Library HTTP accept failed: {error}"),
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

// Was: Listet die möglichen Varianten für response body auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
enum ResponseBody {
    Bytes(Vec<u8>),
    File(PathBuf),
}

// Was: Bündelt die zusammengehörigen Werte für HTTP response in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct HttpResponse {
    status: u16,
    content_type: &'static str,
    headers: Vec<(String, String)>,
    body: ResponseBody,
}

// Was: Diese Funktion verarbeitet connection.
// Warum: Die Reaktion auf dieses Ereignis bleibt damit an einer Stelle nachvollziehbar.
fn handle_connection(
    mut stream: TcpStream,
    config: MediaLibraryConfig,
    library: SharedLibrary,
    tts: TtsService,
) -> Result<(), String> {
    let request = read_request(&mut stream, config.server.max_body_bytes)?;
    let response = route(request, config, library, tts);
    write_response(&mut stream, response).map_err(|error| error.to_string())
}

// Was: Diese Funktion leitet den vorgesehenen Arbeitsschritt.
// Warum: Nachrichten und Daten gelangen dadurch nachvollziehbar an das richtige Ziel.
fn route(
    request: HttpRequest,
    config: MediaLibraryConfig,
    library: SharedLibrary,
    tts: TtsService,
) -> HttpResponse {
    if request.method == "OPTIONS" {
        return empty(204);
    }
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => html(&service_design::render(INDEX_HTML, "Media Library", "open-lab")),
        ("GET", "/health/live") => json_response(200, &json!({"status":"live"})),
        ("GET", "/health/ready") => {
            let status = library.status();
            json_response(if status.ready { 200 } else { 503 }, &status)
        }
        ("GET", "/api/v1/status") => json_response(200, &library.status()),
        ("GET", "/api/v1/config") => json_response(200, &library.config_view()),
        ("GET", "/api/v1/tts/status") => json_response(200, &tts.status()),
        ("GET", "/api/v1/tts/voices") => json_response(200, &json!({"voices":tts.voices()})),
        ("GET", "/api/v1/tts/templates") => match tts.templates() {
            Ok(templates) => json_response(200, &json!({"templates":templates})),
            Err(error) => conflict(error),
        },
        ("POST", "/api/v1/tts/templates/save") => {
            match parse_json::<TtsTemplateInput>(&request.body).and_then(|input| tts.save_template(input)) {
                Ok(template) => json_response(200, &json!({"template":template})),
                Err(error) => conflict(error),
            }
        }
        ("POST", "/api/v1/tts/templates/delete") => {
            match parse_json::<TtsTemplateDeleteInput>(&request.body)
                .and_then(|input| tts.delete_template(&input.id))
            {
                Ok(()) => empty(204),
                Err(error) => conflict(error),
            }
        }
        ("POST", "/api/v1/tts/generate") => {
            match parse_json::<TtsGenerateInput>(&request.body)
                .and_then(|input| tts.generate(&library, input))
            {
                Ok(result) => json_response(201, &result),
                Err(error) => conflict(error),
            }
        }
        ("GET", "/api/v1/assets") => json_response(
            200,
            &library.assets(
                request.query.get("q").map(String::as_str),
                request.query.get("kind").map(String::as_str),
                request.query.get("state").map(String::as_str),
                request.query.get("approval").map(String::as_str),
                query_usize(&request, "limit", 500, 5_000),
            ),
        ),
        ("POST", "/api/v1/assets/upload-json") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<UploadInput>(&request.body).and_then(|input| library.create_upload(input)) {
                Ok(asset) => json_response(201, &asset),
                Err(error) => conflict(error),
            }
        }
        ("POST", "/api/v1/assets/import-url") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<ImportUrlInput>(&request.body)
                .and_then(|input| library.create_import_url(input))
            {
                Ok(asset) => json_response(202, &asset),
                Err(error) => conflict(error),
            }
        }
        ("POST", "/api/v1/recorder/import") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<RecorderImportInput>(&request.body)
                .and_then(|input| library.create_recorder_import(input))
            {
                Ok(asset) => json_response(202, &asset),
                Err(error) => conflict(error),
            }
        }
        ("POST", "/api/v1/dispatch") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<DispatchInput>(&request.body)
                .and_then(|input| library.create_dispatch(input))
            {
                Ok(job) => json_response(202, &job),
                Err(error) => conflict(error),
            }
        }
        ("GET", "/api/v1/jobs") => json_response(
            200,
            &library.jobs(
                request.query.get("state").map(String::as_str),
                query_usize(&request, "limit", 500, 5_000),
            ),
        ),
        ("GET", "/api/v1/events") => {
            json_response(200, &library.events(query_usize(&request, "limit", 250, 5_000)))
        }
        ("GET", "/api/v1/audit") => {
            json_response(200, &library.audit(query_usize(&request, "limit", 250, 5_000)))
        }
        ("GET", "/api/v1/backups") => json_response(200, &library.backups()),
        ("POST", "/api/v1/backups") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<ActionInput>(&request.body).and_then(|input| library.backup(input)) {
                Ok(record) => json_response(201, &record),
                Err(error) => json_response(500, &json!({"error":error})),
            }
        }
        ("POST", "/api/v1/maintenance/tick") => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<ActionInput>(&request.body)
                .and_then(|input| library.maintenance(input.actor))
            {
                Ok(value) => json_response(200, &value),
                Err(error) => conflict(error),
            }
        }
        ("POST", "/api/v1/maintenance/process-now") => match worker::build_client(&config) {
            Ok(client) => {
                let mut probe = Instant::now();
                let mut maintenance = Instant::now();
                worker::run_cycle(
                    &config,
                    &library,
                    &client,
                    &mut probe,
                    &mut maintenance,
                );
                json_response(200, &library.status())
            }
            Err(error) => json_response(500, &json!({"error":error})),
        },
        ("GET", "/api/v1/export.json") => download_bytes(
            "netcore-media-library-export.json",
            "application/json",
            serde_json::to_vec_pretty(&library.export()).unwrap_or_default(),
        ),
        ("GET", "/metrics") => text(
            "text/plain; version=0.0.4; charset=utf-8",
            library.metrics(),
        ),
        ("GET", "/openapi.json") => json_response(200, &openapi()),
        _ => dynamic_route(request, library),
    }
}

// Was: Führt den Arbeitsschritt `dynamic_route` für dynamic Weiterleitung aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn dynamic_route(request: HttpRequest, library: SharedLibrary) -> HttpResponse {
    let parts = request.path.trim_matches('/').split('/').collect::<Vec<_>>();
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match (request.method.as_str(), parts.as_slice()) {
        ("GET", ["api", "v1", "assets", asset_id]) => library
            .asset(asset_id)
            .map_or_else(|| not_found("asset not found"), |asset| json_response(200, &asset)),
        ("PUT", ["api", "v1", "assets", asset_id]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json::<AssetUpdateInput>(&request.body)
                .and_then(|input| library.update_asset(asset_id, input))
            {
                Ok(asset) => json_response(200, &asset),
                Err(error) => conflict(error),
            }
        }
        ("DELETE", ["api", "v1", "assets", asset_id]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<ActionInput>(&request.body)
                .and_then(|input| library.delete_asset(asset_id, input))
            {
                Ok(()) => empty(204),
                Err(error) => conflict(error),
            }
        }
        ("POST", ["api", "v1", "assets", asset_id, "approve"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<ApprovalInput>(&request.body)
                .and_then(|input| library.approve_asset(asset_id, input))
            {
                Ok(asset) => json_response(200, &asset),
                Err(error) => conflict(error),
            }
        }
        ("POST", ["api", "v1", "assets", asset_id, "reject"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<ApprovalInput>(&request.body)
                .and_then(|input| library.reject_asset(asset_id, input))
            {
                Ok(asset) => json_response(200, &asset),
                Err(error) => conflict(error),
            }
        }
        ("POST", ["api", "v1", "assets", asset_id, "process"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<ActionInput>(&request.body)
                .and_then(|input| library.reprocess_asset(asset_id, input))
            {
                Ok(asset) => json_response(202, &asset),
                Err(error) => conflict(error),
            }
        }
        ("POST", ["api", "v1", "assets", asset_id, "archive"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<ActionInput>(&request.body)
                .and_then(|input| library.archive_asset(asset_id, input))
            {
                Ok(asset) => json_response(200, &asset),
                Err(error) => conflict(error),
            }
        }
        ("GET", ["api", "v1", "assets", asset_id, "original"]) => {
            asset_file_response(&library, asset_id, "original", true)
        }
        ("GET", ["api", "v1", "assets", asset_id, "preview"]) => {
            asset_file_response(&library, asset_id, "preview", false)
        }
        ("GET", ["api", "v1", "assets", asset_id, "audio.tacelp"]) => {
            asset_file_response(&library, asset_id, "tacelp", true)
        }
        ("GET", ["api", "v1", "assets", asset_id, "waveform"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match library.waveform(asset_id, query_usize(&request, "points", 256, 2_048)) {
                Ok(points) => json_response(200, &json!({"asset_id":asset_id,"points":points})),
                Err(error) => not_found(error),
            }
        }
        ("GET", ["api", "v1", "jobs", job_id]) => library
            .job(job_id)
            .map_or_else(|| not_found("job not found"), |job| json_response(200, &job)),
        ("POST", ["api", "v1", "jobs", job_id, "cancel"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<ActionInput>(&request.body)
                .and_then(|input| library.cancel_job(job_id, input))
            {
                Ok(job) => json_response(200, &job),
                Err(error) => conflict(error),
            }
        }
        ("POST", ["api", "v1", "jobs", job_id, "retry"]) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match parse_json_or_default::<ActionInput>(&request.body)
                .and_then(|input| library.retry_job(job_id, input))
            {
                Ok(job) => json_response(202, &job),
                Err(error) => conflict(error),
            }
        }
        _ => not_found("not found"),
    }
}

// Was: Führt den Arbeitsschritt `asset_file_response` für asset file response aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn asset_file_response(
    library: &SharedLibrary,
    asset_id: &str,
    kind: &str,
    attachment: bool,
) -> HttpResponse {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match library.file_for(asset_id, kind) {
        Ok((path, content_type, filename)) => file_response(
            path,
            content_type,
            vec![(
                "Content-Disposition".to_string(),
                format!(
                    "{}; filename=\"{}\"",
                    if attachment { "attachment" } else { "inline" },
                    filename.replace('"', "_")
                ),
            )],
        ),
        Err(error) => not_found(error),
    }
}

// Was: Führt den Arbeitsschritt `openapi` für openapi aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn openapi() -> Value {
    json!({
        "openapi":"3.0.3",
        "info":{
            "title":"NetCore Media Library OPEN LAB API",
            "version":env!("CARGO_PKG_VERSION"),
            "description":"No authentication, no token and no TLS. Manages media assets, previews, approvals, TETRA preparation and controlled injection into existing Media Switch sessions."
        },
        "paths":{
            "/health/live":{"get":{}},
            "/health/ready":{"get":{}},
            "/api/v1/status":{"get":{}},
            "/api/v1/assets":{"get":{}},
            "/api/v1/tts/status":{"get":{}},
            "/api/v1/tts/voices":{"get":{}},
            "/api/v1/tts/templates":{"get":{}},
            "/api/v1/tts/templates/save":{"post":{}},
            "/api/v1/tts/templates/delete":{"post":{}},
            "/api/v1/tts/generate":{"post":{}},
            "/api/v1/assets/upload-json":{"post":{}},
            "/api/v1/assets/import-url":{"post":{}},
            "/api/v1/recorder/import":{"post":{}},
            "/api/v1/assets/{asset_id}":{"get":{},"put":{},"delete":{}},
            "/api/v1/assets/{asset_id}/approve":{"post":{}},
            "/api/v1/assets/{asset_id}/reject":{"post":{}},
            "/api/v1/assets/{asset_id}/process":{"post":{}},
            "/api/v1/assets/{asset_id}/archive":{"post":{}},
            "/api/v1/assets/{asset_id}/preview":{"get":{}},
            "/api/v1/assets/{asset_id}/audio.tacelp":{"get":{}},
            "/api/v1/assets/{asset_id}/waveform":{"get":{}},
            "/api/v1/dispatch":{"post":{}},
            "/api/v1/jobs":{"get":{}},
            "/api/v1/jobs/{job_id}/cancel":{"post":{}},
            "/api/v1/jobs/{job_id}/retry":{"post":{}},
            "/metrics":{"get":{}},
            "/openapi.json":{"get":{}}
        }
    })
}

// Was: Diese Funktion liest request.
// Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
fn read_request(stream: &mut TcpStream, max_body_bytes: usize) -> Result<HttpRequest, String> {
    stream
        .set_read_timeout(Some(std::time::Duration::from_secs(30)))
        .map_err(|error| error.to_string())?;
    let mut bytes = Vec::new();
    let mut buffer = [0u8; 16 * 1024];
    let header_end = loop {
        let read = stream.read(&mut buffer).map_err(|error| error.to_string())?;
        if read == 0 {
            return Err("connection closed before HTTP header completed".to_string());
        }
        bytes.extend_from_slice(&buffer[..read]);
        if bytes.len() > 128 * 1024 {
            return Err("HTTP headers too large".to_string());
        }
        if let Some(position) = find_subslice(&bytes, b"\r\n\r\n") {
            break position + 4;
        }
    };
    let (method, target, content_length) = {
        let header = String::from_utf8_lossy(&bytes[..header_end]);
        let mut lines = header.lines();
        let request_line = lines.next().ok_or_else(|| "missing request line".to_string())?;
        let mut request_parts = request_line.split_whitespace();
        let method = request_parts.next().ok_or_else(|| "missing method".to_string())?.to_string();
        let target = request_parts.next().ok_or_else(|| "missing target".to_string())?.to_string();
        let content_length = lines
            .filter_map(|line| line.split_once(':'))
            .find(|(name, _)| name.eq_ignore_ascii_case("content-length"))
            .map(|(_, value)| value.trim().parse::<usize>())
            .transpose()
            .map_err(|error| format!("invalid Content-Length: {error}"))?
            .unwrap_or(0);
        (method, target, content_length)
    };
    if content_length > max_body_bytes {
        return Err(format!("request body exceeds {max_body_bytes} byte limit"));
    }
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    while bytes.len().saturating_sub(header_end) < content_length {
        let read = stream.read(&mut buffer).map_err(|error| error.to_string())?;
        if read == 0 {
            return Err("connection closed before HTTP body completed".to_string());
        }
        bytes.extend_from_slice(&buffer[..read]);
    }
    let body = bytes[header_end..header_end + content_length].to_vec();
    let (path, query) = parse_path_and_query(&target);
    Ok(HttpRequest { method, path, query, body })
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
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    let length = match &response.body {
        ResponseBody::Bytes(bytes) => bytes.len() as u64,
        ResponseBody::File(path) => std::fs::metadata(path)?.len(),
    };
    let mut headers = format!(
        concat!(
            "HTTP/1.1 {} {}\r\n",
            "Content-Type: {}\r\n",
            "Content-Length: {}\r\n",
            "Cache-Control: no-store\r\n",
            "Access-Control-Allow-Origin: *\r\n",
            "Access-Control-Allow-Methods: GET,POST,PUT,DELETE,OPTIONS\r\n",
            "Access-Control-Allow-Headers: Content-Type\r\n",
            "X-NetCore-Security-Mode: open_lab\r\n"
        ),
        response.status, reason, response.content_type, length
    );
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for (name, value) in response.headers {
        headers.push_str(&format!("{name}: {value}\r\n"));
    }
    headers.push_str("Connection: close\r\n\r\n");
    stream.write_all(headers.as_bytes())?;
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match response.body {
        ResponseBody::Bytes(bytes) => stream.write_all(&bytes),
        ResponseBody::File(path) => {
            let mut file = File::open(path)?;
            std::io::copy(&mut file, stream)?;
            Ok(())
        }
    }
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

// Was: Führt den Arbeitsschritt `json_response` für JSON-Daten response aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn json_response<T: Serialize>(status: u16, value: &T) -> HttpResponse {
    HttpResponse {
        status,
        content_type: "application/json; charset=utf-8",
        headers: Vec::new(),
        body: ResponseBody::Bytes(serde_json::to_vec(value).unwrap_or_else(|_| b"{}".to_vec())),
    }
}

// Was: Führt den Arbeitsschritt `text` für text aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn text(content_type: &'static str, value: String) -> HttpResponse {
    HttpResponse {
        status: 200,
        content_type,
        headers: Vec::new(),
        body: ResponseBody::Bytes(value.into_bytes()),
    }
}

// Was: Führt den Arbeitsschritt `html` für html aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn html(value: &str) -> HttpResponse {
    HttpResponse {
        status: 200,
        content_type: "text/html; charset=utf-8",
        headers: Vec::new(),
        body: ResponseBody::Bytes(value.as_bytes().to_vec()),
    }
}

// Was: Führt den Arbeitsschritt `file_response` für file response aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn file_response(path: PathBuf, content_type: &'static str, headers: Vec<(String, String)>) -> HttpResponse {
    HttpResponse {
        status: 200,
        content_type,
        headers,
        body: ResponseBody::File(path),
    }
}

// Was: Führt den Arbeitsschritt `download_bytes` für download bytes aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn download_bytes(name: &str, content_type: &'static str, bytes: Vec<u8>) -> HttpResponse {
    HttpResponse {
        status: 200,
        content_type,
        headers: vec![(
            "Content-Disposition".to_string(),
            format!("attachment; filename=\"{}\"", name.replace('"', "_")),
        )],
        body: ResponseBody::Bytes(bytes),
    }
}

// Was: Führt den Arbeitsschritt `empty` für empty aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn empty(status: u16) -> HttpResponse {
    HttpResponse {
        status,
        content_type: "application/json; charset=utf-8",
        headers: Vec::new(),
        body: ResponseBody::Bytes(Vec::new()),
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

// Was: Diese Funktion sucht subslice.
// Warum: Die Suchlogik bleibt damit wiederverwendbar und muss nicht an mehreren Stellen kopiert werden.
fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

// Was: Diese Funktion liest und prüft path and query.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_path_and_query(raw: &str) -> (String, HashMap<String, String>) {
    let (path, query) = raw.split_once('?').unwrap_or((raw, ""));
    let query = query
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
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    while index < bytes.len() {
        if bytes[index] == b'%'
            && index + 2 < bytes.len()
            && let (Some(high), Some(low)) = (hex(bytes[index + 1]), hex(bytes[index + 2]))
        {
            output.push((high << 4) | low);
            index += 3;
        } else {
            output.push(if bytes[index] == b'+' { b' ' } else { bytes[index] });
            index += 1;
        }
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

// Was: Legt den festen Wert `INDEX_HTML` für index html fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const INDEX_HTML: &str = include_str!("../web-ui/index.html");
