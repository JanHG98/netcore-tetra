// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Metriken, Protokolle und Betriebsüberwachung.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::thread;
use std::time::Duration as StdDuration;

use chrono::Utc;

use crate::config::{ObservabilityConfig, StackConfig};
use crate::state::{MetricPointInput, SharedObservability, StackProbe, TargetRecord};

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für scrape result in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ScrapeResult {
    pub target_id: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub live: bool,
    pub ready: bool,
    pub metrics_ok: bool,
    pub response_ms: f64,
    pub metrics: Vec<MetricPointInput>,
    pub error: Option<String>,
}

// Was: Diese Funktion startet collector.
// Warum: Länger laufende Arbeit blockiert dadurch nicht den aufrufenden Ablauf.
pub fn spawn_collector(
    config: ObservabilityConfig,
    observability: SharedObservability,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        if !config.collection.scrape_on_start {
            thread::sleep(StdDuration::from_secs(config.collection.scrape_interval_secs));
        }
        // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
        // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
        loop {
            let started = std::time::Instant::now();
            run_cycle(&config, &observability);
            let elapsed = started.elapsed();
            let interval = StdDuration::from_secs(config.collection.scrape_interval_secs);
            if elapsed < interval {
                thread::sleep(interval - elapsed);
            }
        }
    })
}

// Was: Diese Funktion führt cycle.
// Warum: Der Lebenszyklus des Dienstes bleibt so an einer zentralen Stelle steuerbar.
pub fn run_cycle(config: &ObservabilityConfig, observability: &SharedObservability) {
    let targets = observability.targets_for_scrape();
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for target in targets {
        let result = scrape_target(config, &target);
        if let Err(error) = observability.record_scrape(result) {
            tracing::warn!(target=%target.target_id, "failed to persist scrape: {}", error);
        }
    }
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for probe in probe_stack(&config.stack, config.collection.request_timeout_ms, config.collection.max_response_bytes) {
        observability.record_stack_probe(probe);
    }
    if let Err(error) = observability.maintenance(None) {
        tracing::warn!("observability maintenance failed: {}", error);
    }
}

// Was: Führt den Arbeitsschritt `scrape_target` für scrape target aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn scrape_target(config: &ObservabilityConfig, target: &TargetRecord) -> ScrapeResult {
    let started = std::time::Instant::now();
    let timeout_ms = config.collection.request_timeout_ms;
    let limit = config.collection.max_response_bytes;
    let live = http_get(&join_url(&target.base_url, &target.live_path), timeout_ms, limit)
        .map(|response| (200..300).contains(&response.status))
        .unwrap_or(false);
    let ready = http_get(&join_url(&target.base_url, &target.ready_path), timeout_ms, limit)
        .map(|response| (200..300).contains(&response.status))
        .unwrap_or(false);
    let metrics_response = http_get(&join_url(&target.base_url, &target.metrics_path), timeout_ms, limit);
    let mut error = None;
    let mut metrics = Vec::new();
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    let metrics_ok = match metrics_response {
        Ok(response) if (200..300).contains(&response.status) => {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match String::from_utf8(response.body) {
                Ok(text) => {
                    metrics = parse_prometheus(&text, target);
                    true
                }
                Err(value) => {
                    error = Some(format!("metrics are not UTF-8: {value}"));
                    false
                }
            }
        }
        Ok(response) => {
            error = Some(format!("metrics endpoint returned HTTP {}", response.status));
            false
        }
        Err(value) => {
            error = Some(value);
            false
        }
    };
    ScrapeResult {
        target_id: target.target_id.clone(),
        timestamp: Utc::now(),
        live,
        ready,
        metrics_ok,
        response_ms: started.elapsed().as_secs_f64() * 1000.0,
        metrics,
        error,
    }
}

// Was: Führt den Arbeitsschritt `probe_stack` für probe stack aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn probe_stack(config: &StackConfig, timeout_ms: u64, limit: usize) -> Vec<StackProbe> {
    [
        ("prometheus", &config.prometheus_url, &config.prometheus_ready_path),
        ("grafana", &config.grafana_url, &config.grafana_ready_path),
        ("loki", &config.loki_url, &config.loki_ready_path),
        ("alertmanager", &config.alertmanager_url, &config.alertmanager_ready_path),
    ]
    .into_iter()
    .map(|(component, base, path)| {
        let started = std::time::Instant::now();
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match http_get(&join_url(base, path), timeout_ms, limit) {
            Ok(response) => StackProbe {
                component: component.to_string(),
                endpoint: base.to_string(),
                ready: (200..300).contains(&response.status),
                response_ms: started.elapsed().as_secs_f64() * 1000.0,
                checked_at: Utc::now(),
                last_error: if (200..300).contains(&response.status) { None } else { Some(format!("HTTP {}", response.status)) },
            },
            Err(error) => StackProbe {
                component: component.to_string(),
                endpoint: base.to_string(),
                ready: false,
                response_ms: started.elapsed().as_secs_f64() * 1000.0,
                checked_at: Utc::now(),
                last_error: Some(error),
            },
        }
    })
    .collect()
}

// Was: Diese Funktion verknüpft url.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn join_url(base: &str, path: &str) -> String {
    format!("{}{}", base.trim_end_matches('/'), if path.starts_with('/') { path.to_string() } else { format!("/{path}") })
}

// Was: Bündelt die zusammengehörigen Werte für HTTP result in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct HttpResult {
    status: u16,
    body: Vec<u8>,
}

// Was: Führt den Arbeitsschritt `http_get` für HTTP get aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn http_get(url: &str, timeout_ms: u64, max_bytes: usize) -> Result<HttpResult, String> {
    let parsed = ParsedUrl::parse(url)?;
    let mut addresses = (parsed.host.as_str(), parsed.port)
        .to_socket_addrs()
        .map_err(|error| format!("DNS failed for {}: {error}", parsed.host))?;
    let address = addresses.next().ok_or_else(|| "no socket address resolved".to_string())?;
    let timeout = StdDuration::from_millis(timeout_ms);
    let mut stream = TcpStream::connect_timeout(&address, timeout)
        .map_err(|error| format!("connect {} failed: {error}", address))?;
    stream.set_read_timeout(Some(timeout)).map_err(|error| error.to_string())?;
    stream.set_write_timeout(Some(timeout)).map_err(|error| error.to_string())?;
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}:{}\r\nUser-Agent: netcore-observability/1\r\nAccept: */*\r\nConnection: close\r\n\r\n",
        parsed.path, parsed.host, parsed.port
    );
    stream.write_all(request.as_bytes()).map_err(|error| error.to_string())?;
    let mut raw = Vec::new();
    let mut buffer = [0u8; 8192];
    // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
    // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
    loop {
        let count = stream.read(&mut buffer).map_err(|error| error.to_string())?;
        if count == 0 { break; }
        if raw.len() + count > max_bytes + 16_384 {
            return Err(format!("response exceeds {} bytes", max_bytes));
        }
        raw.extend_from_slice(&buffer[..count]);
    }
    let header_end = raw.windows(4).position(|window| window == b"\r\n\r\n")
        .ok_or_else(|| "invalid HTTP response".to_string())?;
    let headers = String::from_utf8_lossy(&raw[..header_end]);
    let status = headers.lines().next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|value| value.parse::<u16>().ok())
        .ok_or_else(|| "invalid HTTP status line".to_string())?;
    let mut body = raw[header_end + 4..].to_vec();
    if headers.to_ascii_lowercase().contains("transfer-encoding: chunked") {
        body = decode_chunked(&body)?;
    }
    if body.len() > max_bytes { return Err(format!("response body exceeds {} bytes", max_bytes)); }
    Ok(HttpResult { status, body })
}

// Was: Diese Funktion dekodiert chunked.
// Warum: Empfangene Protokolldaten müssen vor der weiteren Nutzung eindeutig verstanden und geprüft werden.
fn decode_chunked(raw: &[u8]) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    let mut cursor = 0usize;
    // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
    // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
    loop {
        let line_end = raw[cursor..].windows(2).position(|window| window == b"\r\n")
            .map(|value| cursor + value)
            .ok_or_else(|| "invalid chunked response".to_string())?;
        let size_text = String::from_utf8_lossy(&raw[cursor..line_end]);
        let size = usize::from_str_radix(size_text.split(';').next().unwrap_or("0").trim(), 16)
            .map_err(|_| "invalid chunk size".to_string())?;
        cursor = line_end + 2;
        if size == 0 { break; }
        if cursor + size + 2 > raw.len() { return Err("truncated chunked response".to_string()); }
        output.extend_from_slice(&raw[cursor..cursor + size]);
        cursor += size + 2;
    }
    Ok(output)
}

// Was: Bündelt die zusammengehörigen Werte für parsed url in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct ParsedUrl {
    host: String,
    port: u16,
    path: String,
}
// Was: Implementiert das zugehörige Verhalten für `ParsedUrl`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl ParsedUrl {
    // Was: Diese Funktion liest und prüft den vorgesehenen Arbeitsschritt.
    // Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
    fn parse(url: &str) -> Result<Self, String> {
        let rest = url.strip_prefix("http://").ok_or_else(|| "only http:// URLs are supported in open_lab".to_string())?;
        let (authority, path) = rest.split_once('/').map(|(a, p)| (a, format!("/{p}"))).unwrap_or((rest, "/".to_string()));
        let (host, port) = if authority.starts_with('[') {
            let end = authority.find(']').ok_or_else(|| "invalid IPv6 URL".to_string())?;
            let host = authority[1..end].to_string();
            let port = authority[end + 1..].strip_prefix(':').unwrap_or("80").parse::<u16>().map_err(|_| "invalid URL port".to_string())?;
            (host, port)
        } else if let Some((host, port)) = authority.rsplit_once(':') {
            (host.to_string(), port.parse::<u16>().map_err(|_| "invalid URL port".to_string())?)
        } else {
            (authority.to_string(), 80)
        };
        if host.is_empty() { return Err("URL host must not be empty".to_string()); }
        Ok(Self { host, port, path })
    }
}

// Was: Diese Funktion liest und prüft prometheus.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
pub fn parse_prometheus(text: &str, target: &TargetRecord) -> Vec<MetricPointInput> {
    let timestamp = Utc::now();
    let mut points = Vec::new();
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        let Some((lhs, rhs)) = split_sample(line) else { continue; };
        let Some(value_text) = rhs.split_whitespace().next() else { continue; };
        let Ok(value) = parse_value(value_text) else { continue; };
        let (name, mut labels) = parse_metric_lhs(lhs);
        if name.is_empty() { continue; }
        labels.insert("target_id".to_string(), target.target_id.clone());
        labels.insert("service".to_string(), target.service.clone());
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for (key, value) in &target.labels { labels.entry(key.clone()).or_insert_with(|| value.clone()); }
        points.push(MetricPointInput { name, labels, value, timestamp });
    }
    points
}

// Was: Führt den Arbeitsschritt `split_sample` für split sample aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn split_sample(line: &str) -> Option<(&str, &str)> {
    let mut braces = 0i32;
    let mut quoted = false;
    let mut escaped = false;
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for (index, ch) in line.char_indices() {
        if escaped { escaped = false; continue; }
        if ch == '\\' && quoted { escaped = true; continue; }
        if ch == '"' { quoted = !quoted; continue; }
        if !quoted {
            if ch == '{' { braces += 1; }
            if ch == '}' { braces -= 1; }
            if ch.is_whitespace() && braces == 0 {
                return Some((&line[..index], line[index..].trim()));
            }
        }
    }
    None
}

// Was: Diese Funktion liest und prüft metric lhs.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_metric_lhs(lhs: &str) -> (String, BTreeMap<String, String>) {
    let Some(open) = lhs.find('{') else { return (lhs.to_string(), BTreeMap::new()); };
    let close = lhs.rfind('}').unwrap_or(lhs.len());
    let mut labels = BTreeMap::new();
    if close > open {
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for part in split_labels(&lhs[open + 1..close]) {
            if let Some((key, raw)) = part.split_once('=') {
                labels.insert(key.trim().to_string(), unquote(raw.trim()));
            }
        }
    }
    (lhs[..open].to_string(), labels)
}

// Was: Führt den Arbeitsschritt `split_labels` für split labels aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn split_labels(input: &str) -> Vec<&str> {
    let mut output = Vec::new();
    let mut start = 0usize;
    let mut quoted = false;
    let mut escaped = false;
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for (index, ch) in input.char_indices() {
        if escaped { escaped = false; continue; }
        if ch == '\\' && quoted { escaped = true; continue; }
        if ch == '"' { quoted = !quoted; continue; }
        if ch == ',' && !quoted { output.push(input[start..index].trim()); start = index + 1; }
    }
    if start < input.len() { output.push(input[start..].trim()); }
    output
}

// Was: Führt den Arbeitsschritt `unquote` für unquote aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn unquote(input: &str) -> String {
    let value = input.strip_prefix('"').and_then(|v| v.strip_suffix('"')).unwrap_or(input);
    value.replace("\\n", "\n").replace("\\\"", "\"").replace("\\\\", "\\")
}

// Was: Diese Funktion liest und prüft value.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_value(value: &str) -> Result<f64, ()> {
    let parsed = value.parse::<f64>().map_err(|_| ())?;
    if parsed.is_finite() { Ok(parsed) } else { Err(()) }
}
