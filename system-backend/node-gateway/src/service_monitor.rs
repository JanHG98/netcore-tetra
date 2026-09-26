// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für die Verbindung zwischen Basisstationen und Backend-Diensten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::thread;
use std::time::Duration;

use crate::config::{NodeGatewayConfig, ServiceTargetConfig};
use crate::state::SharedGateway;

// Was: Diese Funktion startet Dienst monitor.
// Warum: Länger laufende Arbeit blockiert dadurch nicht den aufrufenden Ablauf.
pub fn spawn_service_monitor(gateway: SharedGateway, config: NodeGatewayConfig) {
    if !config.service_monitor.enabled || config.service_monitor.targets.is_empty() {
        tracing::warn!("backend service monitor disabled or has no targets; connected TBS nodes will remain in conservative fallback mode");
        return;
    }
    let _ = thread::Builder::new()
        .name("node-gateway-service-monitor".to_string())
        .spawn(move || loop {
            // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
            // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
            for target in &config.service_monitor.targets {
                let result = probe(target, config.service_monitor.timeout_ms);
                // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                match result {
                    Ok(message) => gateway.record_service_probe(&target.name, true, Some(message)),
                    Err(error) => gateway.record_service_probe(&target.name, false, Some(error)),
                }
            }
            gateway.publish_core_services();
            thread::sleep(Duration::from_secs(config.service_monitor.interval_secs));
        });
}

// Was: Führt den Arbeitsschritt `probe` für probe aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn probe(target: &ServiceTargetConfig, timeout_ms: u64) -> Result<String, String> {
    let (host, port, path) = parse_http_url(&target.url)?;
    let timeout = Duration::from_millis(timeout_ms);
    let addresses: Vec<_> = (host.as_str(), port)
        .to_socket_addrs()
        .map_err(|error| format!("DNS/resolve failed: {error}"))?
        .collect();
    if addresses.is_empty() {
        return Err("DNS/resolve returned no address".to_string());
    }
    let mut last_error = None;
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for address in addresses {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match TcpStream::connect_timeout(&address, timeout) {
            Ok(mut stream) => {
                let _ = stream.set_read_timeout(Some(timeout));
                let _ = stream.set_write_timeout(Some(timeout));
                let request = format!(
                    "GET {path} HTTP/1.1\r\nHost: {host}:{port}\r\nUser-Agent: NetCore-Node-Gateway/edge-health\r\nConnection: close\r\n\r\n"
                );
                stream.write_all(request.as_bytes()).map_err(|error| format!("write failed: {error}"))?;
                let mut status = String::new();
                BufReader::new(stream)
                    .read_line(&mut status)
                    .map_err(|error| format!("read failed: {error}"))?;
                let code = status.split_whitespace().nth(1).and_then(|value| value.parse::<u16>().ok())
                    .ok_or_else(|| format!("invalid HTTP status line: {}", status.trim()))?;
                return if (200..300).contains(&code) {
                    Ok(format!("HTTP {code}"))
                } else {
                    Err(format!("HTTP {code}"))
                };
            }
            Err(error) => last_error = Some(error.to_string()),
        }
    }
    Err(format!("connect failed: {}", last_error.unwrap_or_else(|| "unknown error".to_string())))
}

// Was: Diese Funktion liest und prüft HTTP url.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_http_url(url: &str) -> Result<(String, u16, String), String> {
    let rest = url.strip_prefix("http://").ok_or_else(|| "only http:// URLs are supported in open_lab".to_string())?;
    let (authority, path) = rest.split_once('/').map(|(a, p)| (a, format!("/{p}"))).unwrap_or((rest, "/".to_string()));
    let (host, port) = authority.rsplit_once(':').ok_or_else(|| "URL must include an explicit port".to_string())?;
    let port = port.parse::<u16>().map_err(|_| "invalid URL port".to_string())?;
    if host.trim().is_empty() {
        return Err("URL host must not be empty".to_string());
    }
    Ok((host.to_string(), port, path))
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Führt den Arbeitsschritt `parses_explicit_open_lab_url` für parses explicit open lab url aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn parses_explicit_open_lab_url() {
        assert_eq!(
            parse_http_url("http://10.0.20.17:8150/health/ready").unwrap(),
            ("10.0.20.17".to_string(), 8150, "/health/ready".to_string())
        );
    }
}
