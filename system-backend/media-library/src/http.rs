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
        ("GET", "/") => html(INDEX_HTML),
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
fn html(value: &'static str) -> HttpResponse {
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
const INDEX_HTML: &str = r##"<!doctype html>
<html lang="de"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>NetCore Media Library</title>
<style>
:root{color-scheme:dark;--bg:#071119;--panel:#101f29;--panel2:#152a35;--line:#2b4553;--text:#eef6fa;--muted:#9db1bd;--ok:#50d890;--warn:#ffc857;--bad:#ff6b6b;--accent:#4aa9ff;--purple:#b78cff}*{box-sizing:border-box}body{margin:0;background:var(--bg);color:var(--text);font:14px system-ui,-apple-system,sans-serif}.lab{padding:10px 18px;background:#8d2020;color:#fff;text-align:center;font-weight:800}header{display:flex;justify-content:space-between;gap:20px;padding:20px 26px;background:#0d1921;border-bottom:1px solid var(--line)}h1,h2,h3{margin:0 0 10px}.muted{color:var(--muted)}.ok{color:var(--ok)}.warn{color:var(--warn)}.bad{color:var(--bad)}.layout{display:grid;grid-template-columns:220px minmax(0,1fr);min-height:calc(100vh - 118px)}nav{padding:16px;background:#0b171e;border-right:1px solid var(--line);display:flex;flex-direction:column;gap:5px}nav button{width:100%;text-align:left;background:transparent;border:0;color:var(--muted);padding:10px 12px;border-radius:7px;cursor:pointer}nav button.active,nav button:hover{background:#17303c;color:#fff}main{padding:20px;min-width:0}.page{display:none}.page.active{display:block}.cards{display:grid;grid-template-columns:repeat(auto-fit,minmax(145px,1fr));gap:10px;margin-bottom:16px}.card,.panel{background:var(--panel);border:1px solid var(--line);border-radius:10px;padding:14px}.value{font-size:25px;font-weight:800}.grid2{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:14px}.toolbar{display:flex;gap:8px;flex-wrap:wrap;margin:10px 0}input,select,textarea,button{background:#19303b;color:var(--text);border:1px solid var(--line);border-radius:6px;padding:8px}button{cursor:pointer}.primary{background:#126eae}.danger{background:#8b3138}.secondary{background:#304554}.purple{background:#62488b}table{width:100%;border-collapse:collapse}th,td{text-align:left;padding:8px;border-bottom:1px solid var(--line);vertical-align:top}.scroll{overflow:auto;max-height:650px}.pill{display:inline-block;padding:2px 7px;border-radius:999px;background:#28404d;font-size:12px}.pill.ready,.pill.approved,.pill.completed{background:#1d5f41}.pill.processing,.pill.importing,.pill.queued,.pill.playing{background:#725b1d}.pill.failed,.pill.rejected{background:#7d2d35}.pill.shadowed,.pill.draft{background:#503e75}.mono{font-family:ui-monospace,SFMono-Regular,monospace}.notice{border-left:4px solid var(--warn);padding:9px 12px;background:#2b2517;margin:10px 0}label{display:grid;gap:4px;color:var(--muted)}.formgrid{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:10px}.wide{grid-column:1/-1}pre{white-space:pre-wrap;word-break:break-word;max-height:420px;overflow:auto}audio{width:100%}.small{font-size:12px}.wave{height:70px;display:flex;align-items:center;gap:1px;background:#0b171e;padding:5px}.wave i{display:block;flex:1;background:#4aa9ff;min-height:1px}@media(max-width:850px){.layout{grid-template-columns:1fr}nav{position:sticky;top:0;z-index:3;flex-direction:row;overflow:auto;border-right:0;border-bottom:1px solid var(--line)}nav button{width:auto;white-space:nowrap}.grid2,.formgrid{grid-template-columns:1fr}header{display:block}}
</style></head><body>
<div class="lab">⚠ OPEN LAB – keine Anmeldung, keine Tokens, kein TLS. Jeder erreichbare Client darf Medien hochladen, freigeben, archivieren, löschen und in bestehende Ruf-Sessions einspeisen.</div>
<header><div><h1>Media Library</h1><div class="muted">Audio-Assets, TTS, Recorder-Import, Vorschau, TETRA-Cache und kontrollierte Aussendung</div></div><div><span id="mode" class="pill">…</span> <span id="deps"></span></div></header>
<div class="layout"><nav>
<button data-page="overview" class="active">Übersicht</button><button data-page="library">Bibliothek</button><button data-page="tts">TTS / Piper</button><button data-page="import">Import / Upload</button><button data-page="recorder">Recorder</button><button data-page="dispatch">Aussendung</button><button data-page="jobs">Jobs</button><button data-page="maintenance">Storage / Audit</button><button data-page="api">API</button>
</nav><main>
<section id="overview" class="page active"><div class="cards" id="cards"></div><div class="grid2"><div class="panel"><h2>Pipeline</h2><pre>Upload / URL / Recorder
  → Formatprüfung + SHA-256
  → 8 kHz Mono PCM16 Preview
  → optional TETRA-ACELP Cache
  → Freigabe
  → vorhandene Media-Switch-Session</pre><div class="notice">Die Media Library erzeugt absichtlich keinen CMCE-Ruf. Für eine Aussendung muss bereits eine passende Media-Switch-Session existieren.</div></div><div class="panel"><h2>Letzte Ereignisse</h2><pre id="recentEvents">Lade …</pre></div></div></section>
<section id="library" class="page"><div class="toolbar"><input id="search" placeholder="Titel, Tag, Datei, Text …"><select id="kindFilter"><option value="">alle Typen</option><option>tts</option><option>recording</option><option>announcement</option><option>alarm</option><option>music</option><option>prompt</option><option>other</option></select><select id="approvalFilter"><option value="">alle Freigaben</option><option>draft</option><option>approved</option><option>rejected</option></select><button class="primary" onclick="refresh()">Suchen</button></div><div class="panel scroll"><table><thead><tr><th>Asset</th><th>Quelle / Datei</th><th>Audio</th><th>Status</th><th>Freigabe</th><th>Aktionen</th></tr></thead><tbody id="assetRows"></tbody></table></div></section>
<section id="tts" class="page">
<div class="grid2">
  <div class="panel">
    <div style="display:flex;justify-content:space-between;gap:10px;align-items:center;flex-wrap:wrap"><div><h2>Piper Textdurchsage</h2><div class="muted" id="ttsProviderDetail">Lade Status …</div></div><div><span id="ttsProviderState" class="pill">PRÜFE …</span> <button onclick="loadCentralTts()">Aktualisieren</button></div></div>
    <div class="formgrid" style="margin-top:14px">
      <label class="wide">Sprechtext<textarea id="ttsText" rows="8" maxlength="2000" placeholder="Text der Durchsage eingeben …" oninput="updateTtsCount()"></textarea><span class="small muted" id="ttsCount">0 Zeichen</span></label>
      <label>Stimme<select id="ttsVoice"></select></label>
      <label>Sprechtempo<div style="display:flex;gap:10px;align-items:center"><input id="ttsSpeed" type="range" min="50" max="150" step="5" value="95" oninput="updateTtsSpeed()" style="width:100%"><b id="ttsSpeedLabel">95 %</b></div></label>
      <label class="wide">Name der Durchsage<input id="ttsName" maxlength="120" placeholder="z. B. Evakuierung Haupthalle"></label>
      <label><input id="ttsApprove" type="checkbox"> sofort für Aussendungen freigeben</label>
      <label>Tags, Komma-getrennt<input id="ttsTags" value="durchsage"></label>
    </div>
    <div class="toolbar"><button class="primary" id="ttsGenerate" onclick="generateCentralTts()">Als TTS-Medium speichern</button></div>
    <pre id="ttsOutput">Bereit.</pre>
    <audio id="ttsPreview" controls preload="metadata" style="display:none"></audio>
  </div>
  <div class="panel">
    <h2>TTS-Vorlagen</h2>
    <div class="formgrid">
      <label class="wide">Vorlage<select id="ttsTemplateSelect" onchange="selectCentralTtsTemplate()"><option value="">Keine Vorlage ausgewählt</option></select></label>
      <label class="wide">Vorlagenname<input id="ttsTemplateName" maxlength="120" placeholder="z. B. Evakuierung Haupthalle"></label>
    </div>
    <div class="toolbar"><button onclick="newCentralTtsTemplate()">Neue Vorlage</button><button class="primary" onclick="saveCentralTtsTemplate()">Speichern</button><button class="danger" id="ttsTemplateDelete" onclick="deleteCentralTtsTemplate()" disabled>Löschen</button></div>
    <div class="muted small" id="ttsTemplateState">Vorlagen werden zentral in der Media Library gespeichert.</div>
  </div>
</div>
<div class="notice">Die Basisstation erzeugt selbst keine TTS-Dateien mehr. Fertige TTS-Assets erscheinen nach Verarbeitung und Freigabe automatisch im Media-Library-Dateibrowser der Basisstation.</div>
</section>
<section id="import" class="page"><div class="grid2"><form class="panel" onsubmit="uploadAsset(event)"><h2>Datei hochladen</h2><div class="formgrid"><label>Name<input id="uName" required></label><label>Typ<select id="uKind"><option>announcement</option><option>tts</option><option>alarm</option><option>music</option><option>prompt</option><option>other</option></select></label><label class="wide">Datei<input id="uFile" type="file" accept="audio/wav,audio/mpeg,.tacelp" required></label><label class="wide">Tags, Komma-getrennt<input id="uTags"></label><label class="wide">Beschreibung<textarea id="uDescription" rows="3"></textarea></label><label><input id="uApprove" type="checkbox"> sofort freigeben</label></div><button class="primary">Hochladen</button><pre id="uploadOutput"></pre></form><form class="panel" onsubmit="importUrl(event)"><h2>URL importieren</h2><div class="formgrid"><label>Name<input id="iName" required></label><label>Typ<select id="iKind"><option>tts</option><option>announcement</option><option>recording</option><option>other</option></select></label><label class="wide">Source URL<input id="iUrl" type="url" required></label><label>Media-Type<input id="iMediaType" value="audio/wav"></label><label>SHA-256 optional<input id="iSha"></label><label><input id="iApprove" type="checkbox"> sofort freigeben</label></div><button class="primary">Import einreihen</button><pre id="importOutput"></pre></form></div></section>
<section id="recorder" class="page"><div class="grid2"><form class="panel" onsubmit="importRecorder(event)"><h2>Recorder-Aufnahme übernehmen</h2><div class="formgrid"><label>Recording-ID<input id="rId" required></label><label>Name optional<input id="rName"></label><label><input id="rApprove" type="checkbox"> sofort freigeben</label></div><button class="primary">TACELP importieren</button><pre id="recorderOutput"></pre></form><div class="panel"><h2>Saubere Trennung</h2><p>Der Recorder bleibt das unveränderte Beweis-/Archivsystem. Die Media Library importiert bei Bedarf eine Kopie des gepackten <span class="mono">audio.tacelp</span> und verwaltet daraus Vorschau, Freigabe und Wiedergabe.</p><p>Legal Hold und Recorder-Retention werden dadurch nicht verändert.</p></div></div></section>
<section id="dispatch" class="page"><div class="grid2"><form class="panel" onsubmit="createDispatch(event)"><h2>Asset aussenden</h2><div id="dispatchAssetHint" class="notice small"></div><div class="formgrid"><label class="wide">Asset<select id="dAsset" required></select></label><label id="dStationLabel">Basisstation<select id="dStation"></select></label><label>Zieltyp<select id="dKind"><option value="group">Gruppe (GSSI)</option><option value="individual">Einzelruf (ISSI)</option></select></label><label>Ziel-ID<input id="dDestination" type="number" min="1" max="16777215" required placeholder="z. B. 15201"></label><label>Priorität<input id="dPriority" type="number" min="0" max="15" value="3"></label><details id="dDirectFields" class="wide"><summary>Direkte Media-Switch-Einspeisung (nur Modus media_switch)</summary><div class="formgrid" style="margin-top:10px"><label>Media Session ID<input id="dSession"></label><label>Ziel-Node optional<input id="dNode"></label><label>Ziel-Timeslot optional<input id="dTs" type="number" min="1" max="7"></label></div></details></div><button class="primary" id="dSubmit">Aussendung starten</button></form><div class="panel"><h2>Vorschau</h2><select id="previewAsset" onchange="showPreview()"></select><audio id="player" controls></audio><div id="wave" class="wave"></div><pre id="dispatchOutput">Noch kein Job.</pre></div></div></section>
<section id="jobs" class="page"><div class="toolbar"><select id="jobFilter"><option value="">alle Zustände</option><option>queued</option><option>playing</option><option>completed</option><option>failed</option><option>cancelled</option><option>shadowed</option></select><button class="primary" onclick="processNow()">Jetzt verarbeiten</button></div><div class="panel scroll"><table><thead><tr><th>Job</th><th>Asset / Ziel</th><th>Status</th><th>Fortschritt</th><th>Fehler</th><th>Aktionen</th></tr></thead><tbody id="jobRows"></tbody></table></div></section>
<section id="maintenance" class="page"><div class="toolbar"><button onclick="maintenance()">Reconcile / Retention</button><button onclick="backup()">State-Backup</button><button onclick="location.href='/api/v1/export.json'">JSON-Export</button></div><div class="grid2"><div class="panel"><h2>Konfiguration</h2><pre id="configView"></pre></div><div class="panel"><h2>Audit</h2><pre id="auditView"></pre></div></div></section>
<section id="api" class="page"><div class="panel"><h2>API</h2><p><a href="/openapi.json">OpenAPI</a> · <a href="/metrics">Prometheus Metrics</a> · <a href="/health/ready">Readiness</a></p><pre>GET  /api/v1/tts/status
GET  /api/v1/tts/voices
GET  /api/v1/tts/templates
POST /api/v1/tts/generate
POST /api/v1/assets/upload-json
POST /api/v1/assets/import-url
POST /api/v1/recorder/import
POST /api/v1/assets/{id}/approve
GET  /api/v1/assets/{id}/preview
GET  /api/v1/assets/{id}/audio.tacelp
POST /api/v1/dispatch
POST /api/v1/jobs/{id}/cancel</pre></div></section>
</main></div>
<script>
const el=id=>document.getElementById(id);let assets=[],jobs=[],libraryStatus=null,libraryConfig=null,centralTtsStatus=null,centralTtsVoices=[],centralTtsTemplates=[],centralTtsTemplateId='';const pages=[...document.querySelectorAll('.page')];document.querySelectorAll('nav button').forEach(b=>b.onclick=()=>{document.querySelectorAll('nav button').forEach(x=>x.classList.remove('active'));b.classList.add('active');pages.forEach(p=>p.classList.toggle('active',p.id===b.dataset.page));if(b.dataset.page==='dispatch')showPreview();if(b.dataset.page==='tts')loadCentralTts()});
async function api(path,opt){const r=await fetch(path,opt);if(!r.ok&&r.status!==204){let e={};try{e=await r.json()}catch{}throw new Error(e.error||r.statusText)}if(r.status===204)return null;return r.json()}
function post(path,body){return api(path,{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(body||{})})}function esc(v){return String(v??'').replace(/[&<>"']/g,c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]))}function pill(v){return `<span class="pill ${esc(v)}">${esc(v)}</span>`}function bytes(n){if(n==null)return '–';const u=['B','KiB','MiB','GiB'];let i=0;while(n>=1024&&i<u.length-1){n/=1024;i++}return n.toFixed(i?1:0)+' '+u[i]}function duration(ms){if(ms==null)return '–';return (ms/1000).toFixed(1)+' s'}
function updateTtsCount(){const text=el('ttsText')?.value||'',max=centralTtsStatus?.max_text_characters||2000;if(el('ttsText'))el('ttsText').maxLength=max;if(el('ttsCount'))el('ttsCount').textContent=[...text].length+' / '+max+' Zeichen'}
function updateTtsSpeed(){if(el('ttsSpeedLabel'))el('ttsSpeedLabel').textContent=(el('ttsSpeed')?.value||95)+' %'}
function renderCentralTts(){const status=centralTtsStatus||{},online=!!status.provider_available;if(el('ttsProviderState')){el('ttsProviderState').textContent=online?'PIPER ONLINE':'PIPER OFFLINE';el('ttsProviderState').className='pill '+(online?'ready':'failed')}if(el('ttsProviderDetail'))el('ttsProviderDetail').textContent=status.provider_endpoint||'Piper nicht konfiguriert';if(el('ttsGenerate'))el('ttsGenerate').disabled=!online||!centralTtsVoices.some(v=>v.id===el('ttsVoice')?.value&&v.available);updateTtsCount();updateTtsSpeed()}
async function loadCentralTts(){try{const[s,v,t]=await Promise.all([api('/api/v1/tts/status'),api('/api/v1/tts/voices'),api('/api/v1/tts/templates')]);centralTtsStatus=s;centralTtsVoices=Array.isArray(v.voices)?v.voices:[];centralTtsTemplates=Array.isArray(t.templates)?t.templates:[];const voice=el('ttsVoice'),wanted=voice?.value||s.default_voice||'';if(voice){voice.innerHTML=centralTtsVoices.map(row=>`<option value="${esc(row.id)}" ${row.id===wanted?'selected':''} ${row.available?'':'disabled'}>${esc(row.name+(row.available?'':' · NICHT INSTALLIERT'))}</option>`).join('')||'<option value="">Keine Stimme</option>';if(!voice.value){const first=centralTtsVoices.find(row=>row.available);if(first)voice.value=first.id}}const select=el('ttsTemplateSelect');if(select){select.innerHTML='<option value="">Keine Vorlage ausgewählt</option>'+centralTtsTemplates.map(row=>`<option value="${esc(row.id)}">${esc(row.name)}</option>`).join('');if(centralTtsTemplateId&&centralTtsTemplates.some(row=>row.id===centralTtsTemplateId))select.value=centralTtsTemplateId}if(el('ttsSpeed')&&!el('ttsSpeed').dataset.initialized){el('ttsSpeed').value=Math.round(Number(s.default_speed||0.95)*100);el('ttsSpeed').dataset.initialized='1'}if(el('ttsApprove')&&!el('ttsApprove').dataset.initialized){el('ttsApprove').checked=!!s.auto_approve_tts;el('ttsApprove').dataset.initialized='1'}renderCentralTts()}catch(error){centralTtsStatus={provider_available:false,provider_error:error.message};if(el('ttsOutput'))el('ttsOutput').textContent='TTS-Statusfehler: '+error.message;renderCentralTts()}}
function selectCentralTtsTemplate(){centralTtsTemplateId=el('ttsTemplateSelect')?.value||'';const row=centralTtsTemplates.find(item=>item.id===centralTtsTemplateId);if(row){el('ttsTemplateName').value=row.name;el('ttsText').value=row.text;el('ttsVoice').value=row.voice_id;el('ttsSpeed').value=Math.round(Number(row.speed||0.95)*100)}el('ttsTemplateDelete').disabled=!row;updateTtsCount();updateTtsSpeed();renderCentralTts()}
function newCentralTtsTemplate(){centralTtsTemplateId='';if(el('ttsTemplateSelect'))el('ttsTemplateSelect').value='';if(el('ttsTemplateName'))el('ttsTemplateName').value='';if(el('ttsTemplateDelete'))el('ttsTemplateDelete').disabled=true}
function centralTtsTemplatePayload(){const name=el('ttsTemplateName').value.trim(),text=el('ttsText').value.trim();if(!name)throw new Error('Bitte einen Vorlagennamen eingeben.');if(!text)throw new Error('Bitte einen Sprechtext eingeben.');return{id:centralTtsTemplateId||null,name,text,voice_id:el('ttsVoice').value,speed:Number(el('ttsSpeed').value)/100}}
async function saveCentralTtsTemplate(){try{const out=await post('/api/v1/tts/templates/save',centralTtsTemplatePayload());centralTtsTemplateId=out.template.id;el('ttsTemplateState').textContent='Vorlage gespeichert.';await loadCentralTts();el('ttsTemplateSelect').value=centralTtsTemplateId;el('ttsTemplateDelete').disabled=false}catch(error){alert(error.message)}}
async function deleteCentralTtsTemplate(){if(!centralTtsTemplateId)return;if(!confirm('Vorlage wirklich löschen?'))return;try{await post('/api/v1/tts/templates/delete',{id:centralTtsTemplateId});newCentralTtsTemplate();el('ttsTemplateState').textContent='Vorlage gelöscht.';await loadCentralTts()}catch(error){alert(error.message)}}
async function waitForTtsAsset(id){for(let i=0;i<90;i++){const asset=await api('/api/v1/assets/'+encodeURIComponent(id));if(asset.state==='ready'){return asset}if(asset.state==='failed')throw new Error(asset.last_error||'TTS-Verarbeitung fehlgeschlagen');await new Promise(resolve=>setTimeout(resolve,1000))}throw new Error('TTS wird weiterhin verarbeitet. Die Datei erscheint später in der Bibliothek.')}
async function generateCentralTts(){const name=el('ttsName').value.trim()||(centralTtsTemplates.find(row=>row.id===centralTtsTemplateId)?.name||'').trim(),text=el('ttsText').value.trim();if(!name){alert('Bitte einen Namen der Durchsage eingeben.');return}if(!text){alert('Bitte einen Sprechtext eingeben.');return}const button=el('ttsGenerate');button.disabled=true;el('ttsOutput').textContent='Piper erzeugt die WAV …';try{const result=await post('/api/v1/tts/generate',{name,text,voice_id:el('ttsVoice').value,speed:Number(el('ttsSpeed').value)/100,approve:el('ttsApprove').checked,tags:(el('ttsTags').value||'').split(',').map(x=>x.trim()).filter(Boolean),actor:'media-library-webui'});el('ttsOutput').textContent='Asset '+result.asset.asset_id+' wurde angelegt und wird verarbeitet …';const asset=await waitForTtsAsset(result.asset.asset_id);el('ttsOutput').textContent='TTS gespeichert: '+asset.title+' · '+asset.state+' · '+asset.approval;const player=el('ttsPreview');player.src='/api/v1/assets/'+encodeURIComponent(asset.asset_id)+'/preview?_='+Date.now();player.style.display='block';player.load();await refresh()}catch(error){el('ttsOutput').textContent=error.message}finally{renderCentralTts()}}
async function refresh(){try{const q=new URLSearchParams({limit:'1000'});if(el('search')?.value)q.set('q',el('search').value);if(el('kindFilter')?.value)q.set('kind',el('kindFilter').value);if(el('approvalFilter')?.value)q.set('approval',el('approvalFilter').value);const jq=new URLSearchParams({limit:'1000'});if(el('jobFilter')?.value)jq.set('state',el('jobFilter').value);const[s,a,j,e,c,au]=await Promise.all([api('/api/v1/status'),api('/api/v1/assets?'+q),api('/api/v1/jobs?'+jq),api('/api/v1/events?limit=40'),api('/api/v1/config'),api('/api/v1/audit?limit=80')]);assets=a;jobs=j;libraryStatus=s;libraryConfig=c;el('mode').textContent=s.operating_mode;el('mode').className='pill '+(s.operating_mode==='authoritative'?'ready':'draft');el('deps').innerHTML=(s.media_switch_connected?'<span class="ok">● Media</span>':'<span class="bad">● Media</span>')+' '+(s.recorder_connected?'<span class="ok">● Recorder</span>':'<span class="bad">● Recorder</span>')+' '+(s.application_gateway_connected?'<span class="ok">● AppGW</span>':'<span class="bad">● AppGW</span>');el('cards').innerHTML=[['Assets',s.assets_total],['bereit',s.assets_ready],['freigegeben',s.assets_approved],['Preview',s.preview_ready],['TETRA-ready',s.broadcast_ready],['Playout',c.playout_mode],['Import',s.assets_importing],['Jobs aktiv',s.jobs_queued+s.jobs_playing],['Speicher',bytes(s.storage_used_bytes)]].map(x=>`<div class="card"><div class="muted">${x[0]}</div><div class="value">${x[1]}</div></div>`).join('');el('recentEvents').textContent=e.map(x=>`${x.timestamp} #${x.seq} ${x.kind} ${x.asset_id||''} ${x.job_id||''}`).join('\n');renderAssets();renderJobs();renderSelects();el('configView').textContent=JSON.stringify(c,null,2);el('auditView').textContent=au.map(x=>`${x.timestamp} ${x.actor} ${x.action} ${x.object_type}/${x.object_id} ${x.result}`).join('\n')}catch(e){el('deps').innerHTML='<span class="bad">UI: '+esc(e.message)+'</span>'}}
function renderAssets(){el('assetRows').innerHTML=assets.map(a=>`<tr><td><b>${esc(a.title)}</b><br><span class="mono small">${esc(a.asset_id)}</span><br>${pill(a.kind)} ${a.tags.map(t=>pill(t)).join(' ')}</td><td>${esc(a.source)}<br>${esc(a.original_filename)}<br><span class="muted">${bytes(a.size_bytes)} · ${esc(a.sha256?.slice(0,12)||'–')}</span></td><td>${esc(a.metadata.format||'–')} / ${esc(a.metadata.codec||'–')}<br>${duration(a.metadata.duration_ms)} · ${a.metadata.tetra_frame_count??'–'} Frames<br>${a.preview_ready?'<span class="ok">Preview</span>':'<span class="muted">kein Preview</span>'} · ${a.broadcast_ready?'<span class="ok">TETRA</span>':'<span class="warn">kein TETRA-Cache</span>'}</td><td>${pill(a.state)}${a.last_error?'<br><span class="bad">'+esc(a.last_error)+'</span>':''}${a.archived?'<br><span class="ok">archiviert</span>':''}</td><td>${pill(a.approval)}<br>${esc(a.approved_by||'')}</td><td>${a.preview_ready?`<button onclick="preview('${a.asset_id}')">Anhören</button>`:''} ${a.approval!=='approved'?`<button class="primary" onclick="approve('${a.asset_id}')">Freigeben</button>`:`<button onclick="reject('${a.asset_id}')">Sperren</button>`} <button onclick="reprocess('${a.asset_id}')">Neu verarbeiten</button> <button onclick="archiveAsset('${a.asset_id}')">Archiv</button> <button class="danger" onclick="deleteAsset('${a.asset_id}')">Löschen</button></td></tr>`).join('')}
function renderJobs(){el('jobRows').innerHTML=jobs.map(j=>{const pct=j.frame_count?Math.round(j.frame_index*100/j.frame_count):0;const route=j.playout_mode==='basisstation'?`TBS ${esc(j.target_node||'–')}<br>Remote ${esc(j.remote_job_id||'wartet')}`:`Session ${esc(j.session_id)}`;return `<tr><td><span class="mono">${esc(j.job_id)}</span><br>${esc(j.created_at)}</td><td>${esc(j.asset_id)}<br>${route}<br>${esc(j.destination_kind||'')} ${j.destination_id??''}</td><td>${pill(j.state)}<br>Versuch ${j.attempts}/${j.max_attempts}</td><td>${j.frame_index}/${j.frame_count} (${pct}%)<br>Ziel-Queues ${j.queued_targets}</td><td>${esc(j.last_error||'')}</td><td>${['queued','playing'].includes(j.state)?`<button class="danger" onclick="cancelJob('${j.job_id}')">Abbrechen</button>`:''} ${['failed','cancelled','shadowed'].includes(j.state)?`<button onclick="retryJob('${j.job_id}')">Retry ab Anfang</button>`:''}</td></tr>`}).join('')}
function renderSelects(){const shadow=libraryStatus?.operating_mode==='shadow';const mode=libraryConfig?.playout_mode||'media_switch';const approved=assets.filter(a=>a.state==='ready'&&a.approval==='approved');const selectable=shadow?approved:(mode==='basisstation'?approved.filter(a=>a.preview_ready):approved.filter(a=>a.broadcast_ready));const currentDispatch=el('dAsset').value;const options=selectable.map(a=>`<option value="${a.asset_id}">${esc(a.title)} (${a.kind}${mode==='basisstation'&&!a.broadcast_ready?' · TBS kodiert':''}${shadow?' · Shadow':''})</option>`).join('');el('dAsset').innerHTML=options||`<option value="">${shadow?'Keine freigegebenen, bereiten Assets':(mode==='basisstation'?'Keine freigegebenen Assets mit WAV-Vorschau':'Keine TETRA-sendefähigen Assets')}</option>`;if(selectable.some(a=>a.asset_id===currentDispatch))el('dAsset').value=currentDispatch;el('dAsset').disabled=selectable.length===0;const stations=(libraryConfig?.playout_stations||[]).filter(x=>x.enabled);const currentStation=el('dStation')?.value;el('dStation').innerHTML=stations.map(x=>`<option value="${esc(x.id)}">${esc(x.name)} · ${esc(x.base_url)}</option>`).join('')||'<option value="">Keine Basisstation konfiguriert</option>';const preferred=libraryConfig?.playout_default_station;if(stations.some(x=>x.id===currentStation))el('dStation').value=currentStation;else if(stations.some(x=>x.id===preferred))el('dStation').value=preferred;const delegated=mode==='basisstation';el('dStationLabel').style.display=delegated?'grid':'none';el('dDirectFields').style.display=delegated?'none':'block';el('dStation').required=delegated;el('dSession').required=!delegated&&!shadow;const hint=el('dispatchAssetHint');if(hint){if(shadow){hint.innerHTML='<b>Shadow-Modus:</b> Der Auftrag wird nur protokolliert; es findet keine Funkübertragung statt.'}else if(delegated){hint.innerHTML='<b>Basisstations-Playout:</b> Die ausgewählte TBS lädt die vollständige WAV-Vorschau, kodiert sie lokal in TETRA-ACELP, baut den Gruppen- oder Einzelruf auf und beendet ihn nach der Datei automatisch. Ein zentraler TACELP-Cache ist nicht erforderlich.'+(stations.length?'':'<br><span class="bad">Es ist noch keine aktive Basisstation unter [playout] konfiguriert.</span>')}else if(selectable.length===0&&approved.length>0){hint.innerHTML='<b>Keine TETRA-sendefähigen Assets:</b> Für direkte Media-Switch-Einspeisung ist ein gepackter TACELP-Cache erforderlich.'}else{hint.innerHTML='<b>Direkte Media-Switch-Einspeisung:</b> Die Session muss bereits bestehen und das Asset einen validierten TACELP-Cache besitzen.'}}el('dSubmit').disabled=selectable.length===0||(delegated&&stations.length===0);const previews=assets.filter(a=>a.preview_ready);const popts=previews.map(a=>`<option value="${a.asset_id}">${esc(a.title)}</option>`).join('');const current=el('previewAsset').value;el('previewAsset').innerHTML='<option value="">Vorschau wählen</option>'+popts;if(previews.some(a=>a.asset_id===current))el('previewAsset').value=current}
function preview(id){document.querySelector('button[data-page="dispatch"]').click();el('previewAsset').value=id;showPreview()}async function showPreview(){const id=el('previewAsset').value;if(!id){el('player').removeAttribute('src');el('wave').innerHTML='';return}el('player').src=`/api/v1/assets/${id}/preview`;try{const w=await api(`/api/v1/assets/${id}/waveform?points=180`);el('wave').innerHTML=w.points.map(v=>`<i style="height:${Math.max(2,Math.round(v*60))}px"></i>`).join('')}catch{el('wave').innerHTML=''}}
async function uploadAsset(ev){ev.preventDefault();const file=el('uFile').files[0];if(!file)return;el('uploadOutput').textContent='Lese Datei …';const data=await new Promise((resolve,reject)=>{const r=new FileReader();r.onload=()=>resolve(r.result);r.onerror=()=>reject(r.error);r.readAsDataURL(file)});try{const out=await post('/api/v1/assets/upload-json',{name:el('uName').value,filename:file.name,media_type:file.type||null,kind:el('uKind').value,description:el('uDescription').value,tags:el('uTags').value.split(',').map(x=>x.trim()).filter(Boolean),data_base64:data,approve:el('uApprove').checked});el('uploadOutput').textContent=JSON.stringify(out,null,2);refresh()}catch(e){el('uploadOutput').textContent=e.message}}
async function importUrl(ev){ev.preventDefault();try{const out=await post('/api/v1/assets/import-url',{schema:'netcore-media-import-v1',source:'webui',source_url:el('iUrl').value,name:el('iName').value,media_type:el('iMediaType').value||null,kind:el('iKind').value,sha256:el('iSha').value||null,approve:el('iApprove').checked});el('importOutput').textContent=JSON.stringify(out,null,2);refresh()}catch(e){el('importOutput').textContent=e.message}}
async function importRecorder(ev){ev.preventDefault();try{const out=await post('/api/v1/recorder/import',{recording_id:el('rId').value,name:el('rName').value||null,approve:el('rApprove').checked});el('recorderOutput').textContent=JSON.stringify(out,null,2);refresh()}catch(e){el('recorderOutput').textContent=e.message}}
async function approve(id){try{await post(`/api/v1/assets/${id}/approve`,{});refresh()}catch(e){alert(e.message)}}async function reject(id){if(!confirm('Asset für Aussendungen sperren?'))return;try{await post(`/api/v1/assets/${id}/reject`,{});refresh()}catch(e){alert(e.message)}}async function reprocess(id){try{await post(`/api/v1/assets/${id}/process`,{});refresh()}catch(e){alert(e.message)}}async function archiveAsset(id){try{await post(`/api/v1/assets/${id}/archive`,{});refresh()}catch(e){alert(e.message)}}async function deleteAsset(id){if(!confirm('Asset und lokale Dateien endgültig löschen?'))return;try{await api(`/api/v1/assets/${id}`,{method:'DELETE',headers:{'Content-Type':'application/json'},body:'{}'});refresh()}catch(e){alert(e.message)}}
async function createDispatch(ev){ev.preventDefault();if(!el('dAsset').value){el('dispatchOutput').textContent='Kein geeignetes Asset ausgewählt.';return}const mode=libraryConfig?.playout_mode||'media_switch';if(mode==='basisstation'&&!el('dStation').value){el('dispatchOutput').textContent='Keine Basisstation ausgewählt oder konfiguriert.';return}try{const out=await post('/api/v1/dispatch',{asset_id:el('dAsset').value,station_id:mode==='basisstation'?el('dStation').value:null,session_id:mode==='media_switch'?(el('dSession').value||null):null,target_node:mode==='media_switch'?(el('dNode').value||null):null,target_logical_ts:mode==='media_switch'&&el('dTs').value?Number(el('dTs').value):null,destination_kind:el('dKind').value,destination_id:Number(el('dDestination').value),priority:Number(el('dPriority').value)});el('dispatchOutput').textContent=JSON.stringify(out,null,2);refresh()}catch(e){el('dispatchOutput').textContent=e.message}}async function cancelJob(id){try{await post(`/api/v1/jobs/${id}/cancel`,{});refresh()}catch(e){alert(e.message)}}async function retryJob(id){if(!confirm('Retry beginnt absichtlich wieder bei Frame 0. Fortfahren?'))return;try{await post(`/api/v1/jobs/${id}/retry`,{});refresh()}catch(e){alert(e.message)}}async function processNow(){try{await post('/api/v1/maintenance/process-now',{});refresh()}catch(e){alert(e.message)}}async function maintenance(){try{alert(JSON.stringify(await post('/api/v1/maintenance/tick',{}),null,2));refresh()}catch(e){alert(e.message)}}async function backup(){try{alert(JSON.stringify(await post('/api/v1/backups',{}),null,2));refresh()}catch(e){alert(e.message)}}
refresh();setInterval(refresh,3000);
</script></body></html>"##;
