#[path = "../../shared/web-ui/service-design.rs"]
mod service_design;

// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Aufzeichnung und Ereignisprotokollierung.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::thread;

use serde::Serialize;
use serde_json::json;

use crate::config::RecorderConfig;
use crate::state::{HoldInput, RecordingMetadata, RetentionInput, SharedRecorder};

// Was: Diese Funktion startet HTTP server.
// Warum: Länger laufende Arbeit blockiert dadurch nicht den aufrufenden Ablauf.
pub fn spawn_http_server(
    config: RecorderConfig,
    recorder: SharedRecorder,
) -> std::io::Result<thread::JoinHandle<()>> {
    let listener = TcpListener::bind(config.server.bind)?;
    tracing::info!(
        "Recorder WebUI/API listening on http://{}",
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
                    let recorder = recorder.clone();
                    thread::spawn(move || {
                        if let Err(error) = handle_connection(stream, config, recorder) {
                            tracing::warn!("Recorder HTTP connection failed: {}", error);
                        }
                    });
                }
                Err(error) => tracing::warn!("Recorder HTTP accept failed: {}", error),
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
    config: RecorderConfig,
    recorder: SharedRecorder,
) -> Result<(), String> {
    let request = read_request(&mut stream, config.limits.max_body_bytes)?;
    let response = route(request, recorder);
    write_response(&mut stream, response).map_err(|error| error.to_string())
}

// Was: Diese Funktion leitet den vorgesehenen Arbeitsschritt.
// Warum: Nachrichten und Daten gelangen dadurch nachvollziehbar an das richtige Ziel.
fn route(request: HttpRequest, recorder: SharedRecorder) -> HttpResponse {
    if request.method == "OPTIONS" {
        return empty(204);
    }
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match (request.method.as_str(), request.path.as_str()) {
        ("GET", "/") => html(&service_design::render(INDEX_HTML, "Recorder", "open-lab")),
        ("GET", "/health/live") => json_response(200, &json!({"status":"live"})),
        ("GET", "/health/ready") => {
            let status = recorder.status();
            json_response(if status.ready { 200 } else { 503 }, &status)
        }
        ("GET", "/api/v1/status") => json_response(200, &recorder.status()),
        ("GET", "/api/v1/active") => json_response(200, &recorder.active_recordings()),
        ("GET", "/api/v1/recordings") => {
            let recordings = filter_recordings(recorder.recordings(), &request.query);
            json_response(200, &recordings)
        }
        ("GET", "/api/v1/events") => {
            let limit = query_limit(&request, 100);
            json_response(200, &recorder.events(limit))
        }
        ("GET", "/api/v1/config") => json_response(200, &recorder.config_view()),
        ("GET", "/metrics") => text(
            "text/plain; version=0.0.4; charset=utf-8",
            recorder.metrics(),
        ),
        ("GET", "/openapi.json") => json_response(200, &openapi()),
        _ if request.path.starts_with("/api/v1/recordings/") => {
            recording_route(request, recorder)
        }
        _ => json_response(404, &json!({"error":"not found"})),
    }
}

// Was: Führt den Arbeitsschritt `recording_route` für recording Weiterleitung aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn recording_route(request: HttpRequest, recorder: SharedRecorder) -> HttpResponse {
    let tail = request.path.trim_start_matches("/api/v1/recordings/");
    if request.method == "GET" && !tail.contains('/') {
        return recorder.recording(tail).map_or_else(
            || json_response(404, &json!({"error":"recording not found"})),
            |recording| json_response(200, &recording),
        );
    }
    let Some((id, action)) = tail.rsplit_once('/') else {
        return json_response(404, &json!({"error":"not found"}));
    };
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match (request.method.as_str(), action) {
        ("POST", "verify") => result_response(recorder.verify_recording(id)),
        ("POST", "retention") => parse_json::<RetentionInput>(&request)
            .and_then(|input| recorder.set_retention(id, input))
            .map_or_else(error_response, |value| json_response(200, &value)),
        ("POST", "hold") => parse_json::<HoldInput>(&request)
            .and_then(|input| recorder.set_hold(id, input))
            .map_or_else(error_response, |value| json_response(200, &value)),
        ("POST", "delete") => recorder.delete_recording(id).map_or_else(
            error_response,
            |()| json_response(200, &json!({"deleted":true,"id":id})),
        ),
        ("POST", "finalize") => result_response(recorder.finalize_active(id)),
        ("GET", "audio.tacelp") => match recorder.recording_audio_path(id) {
            Ok(path) => file_response(
                path,
                "application/x-tetra-acelp",
                vec![(
                    "Content-Disposition".to_string(),
                    format!("attachment; filename=\"netcore-recording-{id}.tacelp\""),
                )],
            ),
            Err(error) => error_response(error),
        },
        ("GET", "export") => match recorder.export_recording(id) {
            Ok(path) => file_response(
                path,
                "application/x-tar",
                vec![(
                    "Content-Disposition".to_string(),
                    format!("attachment; filename=\"netcore-recording-{id}.tar\""),
                )],
            ),
            Err(error) => error_response(error),
        },
        _ => json_response(404, &json!({"error":"not found"})),
    }
}

// Was: Führt den Arbeitsschritt `filter_recordings` für filter recordings aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn filter_recordings(
    mut recordings: Vec<RecordingMetadata>,
    query: &HashMap<String, String>,
) -> Vec<RecordingMetadata> {
    if let Some(q) = query.get("q").map(|value| value.trim().to_ascii_lowercase())
        && !q.is_empty()
    {
        recordings.retain(|recording| {
            let mut haystack = format!(
                "{} {} {} {:?} {:?} {:?} {:?}",
                recording.id,
                recording.session_id,
                recording.call_kind,
                recording.source_issi,
                recording.gssi,
                recording.calling_issi,
                recording.called_issi
            )
            .to_ascii_lowercase();
            // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
            // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
            for node in &recording.source_nodes {
                haystack.push(' ');
                haystack.push_str(&node.to_ascii_lowercase());
            }
            // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
            // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
            for speaker in &recording.speakers {
                haystack.push(' ');
                haystack.push_str(&speaker.to_string());
            }
            haystack.contains(&q)
        });
    }
    if let Some(gssi) = query.get("gssi").and_then(|value| value.parse::<u32>().ok()) {
        recordings.retain(|recording| recording.gssi == Some(gssi));
    }
    if let Some(issi) = query.get("issi").and_then(|value| value.parse::<u32>().ok()) {
        recordings.retain(|recording| {
            recording.source_issi == Some(issi)
                || recording.calling_issi == Some(issi)
                || recording.called_issi == Some(issi)
                || recording.speakers.contains(&issi)
        });
    }
    if let Some(emergency) = query.get("emergency").and_then(|value| value.parse::<bool>().ok()) {
        recordings.retain(|recording| recording.emergency == emergency);
    }
    let limit = query
        .get("limit")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(200)
        .clamp(1, 2_000);
    recordings.truncate(limit);
    recordings
}

// Was: Führt den Arbeitsschritt `result_response` für result response aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn result_response(result: Result<RecordingMetadata, String>) -> HttpResponse {
    result.map_or_else(error_response, |value| json_response(200, &value))
}

// Was: Führt den Arbeitsschritt `error_response` für error response aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn error_response(error: String) -> HttpResponse {
    let status = if error.contains("not found") {
        404
    } else if error.contains("disabled")
        || error.contains("legal hold")
        || error.contains("still active")
    {
        409
    } else {
        400
    };
    json_response(status, &json!({"error":error}))
}

// Was: Diese Funktion liest und prüft JSON-Daten.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_json<T: serde::de::DeserializeOwned>(request: &HttpRequest) -> Result<T, String> {
    serde_json::from_slice(&request.body).map_err(|error| format!("invalid JSON: {error}"))
}

// Was: Führt den Arbeitsschritt `query_limit` für query limit aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn query_limit(request: &HttpRequest, default: usize) -> usize {
    request
        .query
        .get("limit")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(default)
        .clamp(1, 2_000)
}

// Was: Führt den Arbeitsschritt `openapi` für openapi aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn openapi() -> serde_json::Value {
    json!({
        "openapi":"3.0.3",
        "info":{
            "title":"NetCore Recorder OPEN LAB API",
            "version":env!("CARGO_PKG_VERSION"),
            "description":"OPEN LAB API. No authentication, no token and no TLS. Stores packed TETRA ACELP frames, metadata, hashes and retention state."
        },
        "paths":{
            "/health/live":{"get":{}},
            "/health/ready":{"get":{}},
            "/api/v1/status":{"get":{}},
            "/api/v1/active":{"get":{}},
            "/api/v1/recordings":{"get":{}},
            "/api/v1/recordings/{id}":{"get":{}},
            "/api/v1/recordings/{id}/verify":{"post":{}},
            "/api/v1/recordings/{id}/retention":{"post":{}},
            "/api/v1/recordings/{id}/hold":{"post":{}},
            "/api/v1/recordings/{id}/delete":{"post":{}},
            "/api/v1/recordings/{id}/finalize":{"post":{}},
            "/api/v1/recordings/{id}/export":{"get":{}},
            "/api/v1/recordings/{id}/audio.tacelp":{"get":{}},
            "/api/v1/events":{"get":{}},
            "/api/v1/config":{"get":{}},
            "/metrics":{"get":{}}
        }
    })
}

// Was: Diese Funktion liest request.
// Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
fn read_request(stream: &mut TcpStream, max_body_bytes: usize) -> Result<HttpRequest, String> {
    let mut buffer = Vec::new();
    let mut chunk = [0u8; 8192];
    let header_end;
    // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
    // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
    loop {
        let read = stream
            .read(&mut chunk)
            .map_err(|error| format!("request read failed: {error}"))?;
        if read == 0 {
            return Err("connection closed before request headers".to_string());
        }
        buffer.extend_from_slice(&chunk[..read]);
        if let Some(position) = find_subslice(&buffer, b"\r\n\r\n") {
            header_end = position;
            break;
        }
        if buffer.len() > 64 * 1024 {
            return Err("request headers are too large".to_string());
        }
    }

    let (method, raw_path, content_length) = {
        let header = std::str::from_utf8(&buffer[..header_end])
            .map_err(|_| "request headers are not UTF-8".to_string())?;
        let mut lines = header.lines();
        let request_line = lines
            .next()
            .ok_or_else(|| "missing request line".to_string())?;
        let mut request_parts = request_line.split_whitespace();
        let method = request_parts
            .next()
            .ok_or_else(|| "missing method".to_string())?
            .to_string();
        let raw_path = request_parts
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
        return Err("request body is too large".to_string());
    }

    let body_start = header_end + 4;
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    while buffer.len() < body_start + content_length {
        let read = stream
            .read(&mut chunk)
            .map_err(|error| format!("request body read failed: {error}"))?;
        if read == 0 {
            return Err("connection closed before request body".to_string());
        }
        buffer.extend_from_slice(&chunk[..read]);
    }
    let (path, query) = parse_path_and_query(&raw_path);
    Ok(HttpRequest {
        method,
        path,
        query,
        body: buffer[body_start..body_start + content_length].to_vec(),
    })
}

// Was: Führt den Arbeitsschritt `json_response` für JSON-Daten response aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn json_response<T: Serialize>(status: u16, value: &T) -> HttpResponse {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match serde_json::to_vec(value) {
        Ok(body) => HttpResponse {
            status,
            content_type: "application/json; charset=utf-8",
            headers: Vec::new(),
            body: ResponseBody::Bytes(body),
        },
        Err(error) => HttpResponse {
            status: 500,
            content_type: "application/json; charset=utf-8",
            headers: Vec::new(),
            body: ResponseBody::Bytes(
                serde_json::to_vec(&json!({"error":error.to_string()})).unwrap_or_default(),
            ),
        },
    }
}

// Was: Führt den Arbeitsschritt `html` für html aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn html(value: &str) -> HttpResponse {
    text("text/html; charset=utf-8", value.to_string())
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

// Was: Führt den Arbeitsschritt `file_response` für file response aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn file_response(
    path: PathBuf,
    content_type: &'static str,
    headers: Vec<(String, String)>,
) -> HttpResponse {
    HttpResponse {
        status: 200,
        content_type,
        headers,
        body: ResponseBody::File(path),
    }
}

// Was: Führt den Arbeitsschritt `empty` für empty aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn empty(status: u16) -> HttpResponse {
    HttpResponse {
        status,
        content_type: "text/plain; charset=utf-8",
        headers: Vec::new(),
        body: ResponseBody::Bytes(Vec::new()),
    }
}

// Was: Diese Funktion schreibt response.
// Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
fn write_response(stream: &mut TcpStream, response: HttpResponse) -> std::io::Result<()> {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    let reason = match response.status {
        200 => "OK",
        204 => "No Content",
        400 => "Bad Request",
        404 => "Not Found",
        409 => "Conflict",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        _ => "OK",
    };
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    let content_length = match &response.body {
        ResponseBody::Bytes(body) => body.len() as u64,
        ResponseBody::File(path) => std::fs::metadata(path)?.len(),
    };
    let mut headers = format!(
        concat!(
            "HTTP/1.1 {} {}\r\n",
            "Content-Type: {}\r\n",
            "Content-Length: {}\r\n",
            "Cache-Control: no-store\r\n",
            "Access-Control-Allow-Origin: *\r\n",
            "Access-Control-Allow-Methods: GET,POST,OPTIONS\r\n",
            "Access-Control-Allow-Headers: Content-Type\r\n",
            "X-NetCore-Security-Mode: open_lab\r\n"
        ),
        response.status, reason, response.content_type, content_length
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
        ResponseBody::Bytes(body) => stream.write_all(&body),
        ResponseBody::File(path) => {
            let mut file = File::open(path)?;
            std::io::copy(&mut file, stream)?;
            Ok(())
        }
    }
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
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    while index < bytes.len() {
        if bytes[index] == b'%'
            && index + 2 < bytes.len()
            && let (Some(high), Some(low)) = (
                hex_nibble(bytes[index + 1]),
                hex_nibble(bytes[index + 2]),
            )
        {
            output.push((high << 4) | low);
            index += 3;
            continue;
        }
        output.push(if bytes[index] == b'+' { b' ' } else { bytes[index] });
        index += 1;
    }
    String::from_utf8_lossy(&output).into_owned()
}

// Was: Führt den Arbeitsschritt `hex_nibble` für hex nibble aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn hex_nibble(value: u8) -> Option<u8> {
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
