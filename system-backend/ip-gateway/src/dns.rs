// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für die Kopplung von TETRA-Paketdaten an IP-Netze.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::net::{SocketAddr, UdpSocket};
use std::thread;
use std::time::Duration;

use crate::config::{IpGatewayConfig, MODE_AUTHORITATIVE};
use crate::state::SharedGateway;

// Was: Diese Funktion startet dns.
// Warum: Länger laufende Arbeit blockiert dadurch nicht den aufrufenden Ablauf.
pub fn spawn_dns(config: IpGatewayConfig, gateway: SharedGateway) -> Option<thread::JoinHandle<()>> {
    if !config.dns.enabled {
        return None;
    }
    if config.interface.mode != MODE_AUTHORITATIVE {
        tracing::info!(
            "IP Gateway DNS not started in shadow mode; switch interface.mode to authoritative to activate packet-data DNS"
        );
        return None;
    }
    Some(thread::spawn(move || run(config, gateway)))
}

// Was: Diese Funktion führt den vorgesehenen Arbeitsschritt.
// Warum: Der Lebenszyklus des Dienstes bleibt so an einer zentralen Stelle steuerbar.
fn run(config: IpGatewayConfig, gateway: SharedGateway) {
    let bind = config.effective_dns_bind();
    let mut attempts = 0u64;
    let socket = loop {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match UdpSocket::bind(bind) {
            Ok(socket) => break socket,
            Err(error) => {
                // The authoritative runtime creates and addresses the TUN in a
                // separate worker. A short startup race is therefore normal.
                attempts = attempts.saturating_add(1);
                if attempts == 1 || attempts % 15 == 0 {
                    tracing::warn!(
                        "DNS bind {bind} not ready yet: {error}; retrying in 2 seconds"
                    );
                }
                thread::sleep(Duration::from_secs(2));
            }
        }
    };
    tracing::info!("IP Gateway DNS listening on udp://{bind}");
    let _ = socket.set_read_timeout(Some(Duration::from_secs(1)));
    let mut buffer = [0u8; 4096];
    // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
    // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
    loop {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        let (size, peer) = match socket.recv_from(&mut buffer) {
            Ok(value) => value,
            Err(error) if matches!(error.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut) => continue,
            Err(error) => {
                tracing::warn!("DNS receive failed: {error}");
                continue;
            }
        };
        let query = &buffer[..size];
        let Some((name, qtype, question_end)) = parse_question(query) else {
            gateway.record_dns_query("<malformed>", &peer.to_string(), "malformed");
            continue;
        };
        if qtype == 1 {
            if let Some(address) = gateway.dns_lookup(&name) {
                let response = build_a_response(query, question_end, address.octets(), config.dns.ttl_secs);
                let _ = socket.send_to(&response, peer);
                gateway.record_dns_query(&name, &peer.to_string(), &address.to_string());
                continue;
            }
        }
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match forward_query(query, &config.dns.upstream, config.dns.query_timeout_ms) {
            Ok(response) => {
                let _ = socket.send_to(&response, peer);
                gateway.record_dns_query(&name, &peer.to_string(), "forwarded");
            }
            Err(error) => {
                let response = build_servfail(query, question_end);
                let _ = socket.send_to(&response, peer);
                gateway.record_dns_query(&name, &peer.to_string(), &format!("servfail: {error}"));
            }
        }
    }
}

// Was: Diese Funktion liest und prüft question.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_question(packet: &[u8]) -> Option<(String, u16, usize)> {
    if packet.len() < 12 || u16::from_be_bytes([packet[4], packet[5]]) == 0 {
        return None;
    }
    let mut offset = 12;
    let mut labels = Vec::new();
    // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
    // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
    loop {
        let length = *packet.get(offset)? as usize;
        offset += 1;
        if length == 0 {
            break;
        }
        if length > 63 || offset + length > packet.len() {
            return None;
        }
        labels.push(std::str::from_utf8(&packet[offset..offset + length]).ok()?.to_string());
        offset += length;
    }
    if offset + 4 > packet.len() {
        return None;
    }
    let qtype = u16::from_be_bytes([packet[offset], packet[offset + 1]]);
    let qclass = u16::from_be_bytes([packet[offset + 2], packet[offset + 3]]);
    if qclass != 1 {
        return None;
    }
    Some((labels.join(".").to_ascii_lowercase(), qtype, offset + 4))
}

// Was: Diese Funktion erstellt a response.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
fn build_a_response(query: &[u8], question_end: usize, address: [u8; 4], ttl: u32) -> Vec<u8> {
    let mut response = Vec::with_capacity(question_end + 16);
    response.extend_from_slice(&query[..2]);
    response.extend_from_slice(&0x8180u16.to_be_bytes());
    response.extend_from_slice(&1u16.to_be_bytes());
    response.extend_from_slice(&1u16.to_be_bytes());
    response.extend_from_slice(&0u16.to_be_bytes());
    response.extend_from_slice(&0u16.to_be_bytes());
    response.extend_from_slice(&query[12..question_end]);
    response.extend_from_slice(&0xc00cu16.to_be_bytes());
    response.extend_from_slice(&1u16.to_be_bytes());
    response.extend_from_slice(&1u16.to_be_bytes());
    response.extend_from_slice(&ttl.to_be_bytes());
    response.extend_from_slice(&4u16.to_be_bytes());
    response.extend_from_slice(&address);
    response
}

// Was: Diese Funktion erstellt servfail.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
fn build_servfail(query: &[u8], question_end: usize) -> Vec<u8> {
    let mut response = Vec::with_capacity(question_end);
    response.extend_from_slice(&query[..2]);
    response.extend_from_slice(&0x8182u16.to_be_bytes());
    response.extend_from_slice(&1u16.to_be_bytes());
    response.extend_from_slice(&0u16.to_be_bytes());
    response.extend_from_slice(&0u16.to_be_bytes());
    response.extend_from_slice(&0u16.to_be_bytes());
    response.extend_from_slice(&query[12..question_end]);
    response
}

// Was: Diese Funktion leitet query.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn forward_query(query: &[u8], upstream: &str, timeout_ms: u64) -> Result<Vec<u8>, String> {
    let upstream = upstream
        .parse::<SocketAddr>()
        .map_err(|_| "DNS upstream must currently be an IP socket address".to_string())?;
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|error| error.to_string())?;
    let timeout = Duration::from_millis(timeout_ms);
    socket
        .set_read_timeout(Some(timeout))
        .map_err(|error| error.to_string())?;
    socket
        .connect(upstream)
        .map_err(|error| error.to_string())?;
    socket.send(query).map_err(|error| error.to_string())?;
    let mut response = vec![0u8; 4096];
    let size = socket
        .recv(&mut response)
        .map_err(|error| error.to_string())?;
    response.truncate(size);
    Ok(response)
}
