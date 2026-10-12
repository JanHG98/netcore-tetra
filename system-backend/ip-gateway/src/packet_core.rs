// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für die Kopplung von TETRA-Paketdaten an IP-Netze.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

#[path = "../../shared/http-response.rs"]
mod http_response;

use std::io::Write;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::time::Duration;

use serde::de::DeserializeOwned;

use crate::config::PacketCoreClientConfig;
use crate::protocol::{DownlinkNpduInput, PacketCoreContext, PacketCoreNpdu, PacketCoreStatus};

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für Datenpaket core client in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct PacketCoreClient {
    base: HttpBase,
    timeout: Duration,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für HTTP base in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct HttpBase {
    host: String,
    port: u16,
    base_path: String,
}

// Was: Bündelt die zusammengehörigen Werte für HTTP response in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct HttpResponse {
    status: u16,
    body: Vec<u8>,
}

// Was: Implementiert das zugehörige Verhalten für `PacketCoreClient`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl PacketCoreClient {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(config: &PacketCoreClientConfig) -> Result<Self, String> {
        Ok(Self {
            base: HttpBase::parse(&config.url)?,
            timeout: Duration::from_millis(config.request_timeout_ms),
        })
    }

    // Was: Führt den Arbeitsschritt `status` für Status aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn status(&self) -> Result<PacketCoreStatus, String> {
        self.get_json("/api/v1/status")
    }

    // Was: Führt den Arbeitsschritt `contexts` für contexts aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn contexts(&self) -> Result<Vec<PacketCoreContext>, String> {
        self.get_json("/api/v1/contexts")
    }

    // Was: Führt den Arbeitsschritt `npdu_outbox` für npdu outbox aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn npdu_outbox(&self, limit: usize) -> Result<Vec<PacketCoreNpdu>, String> {
        self.get_json(&format!("/api/v1/npdu-outbox?limit={limit}"))
    }

    // Was: Diese Funktion löscht npdu.
    // Warum: Das Entfernen wird dadurch kontrolliert durchgeführt und hinterlässt keine verwaisten Verweise.
    pub fn delete_npdu(&self, id: &str) -> Result<(), String> {
        let response = self.request("DELETE", &format!("/api/v1/npdu-outbox/{id}"), None)?;
        if matches!(response.status, 200 | 204 | 404) {
            Ok(())
        } else {
            Err(format!(
                "Packet Core DELETE N-PDU returned HTTP {}: {}",
                response.status,
                String::from_utf8_lossy(&response.body)
            ))
        }
    }

    // Was: Führt den Arbeitsschritt `queue_downlink` für Warteschlange Downlink (Netz zum Funkgerät) aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn queue_downlink(&self, input: &DownlinkNpduInput) -> Result<(), String> {
        let body = serde_json::to_vec(input).map_err(|error| error.to_string())?;
        let response = self.request("POST", "/api/v1/downlink", Some(&body))?;
        if matches!(response.status, 200 | 201 | 202 | 204) {
            Ok(())
        } else {
            Err(format!(
                "Packet Core downlink returned HTTP {}: {}",
                response.status,
                String::from_utf8_lossy(&response.body)
            ))
        }
    }

    // Was: Diese Funktion liest JSON-Daten.
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<T, String> {
        let response = self.request("GET", path, None)?;
        if response.status != 200 {
            return Err(format!(
                "Packet Core GET {path} returned HTTP {}: {}",
                response.status,
                String::from_utf8_lossy(&response.body)
            ));
        }
        serde_json::from_slice(&response.body).map_err(|error| error.to_string())
    }

    // Was: Diese Funktion fordert den vorgesehenen Arbeitsschritt.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn request(&self, method: &str, path: &str, body: Option<&[u8]>) -> Result<HttpResponse, String> {
        let address = resolve_one(&self.base.host, self.base.port)?;
        let mut stream = TcpStream::connect_timeout(&address, self.timeout)
            .map_err(|error| format!("connect Packet Core {address}: {error}"))?;
        stream
            .set_read_timeout(Some(self.timeout))
            .map_err(|error| error.to_string())?;
        stream
            .set_write_timeout(Some(self.timeout))
            .map_err(|error| error.to_string())?;

        let full_path = self.base.join_path(path);
        let body = body.unwrap_or_default();
        let request = format!(
            "{method} {full_path} HTTP/1.1\r\nHost: {}:{}\r\nAccept: application/json\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            self.base.host,
            self.base.port,
            body.len()
        );
        stream
            .write_all(request.as_bytes())
            .and_then(|()| stream.write_all(body))
            .map_err(|error| format!("write Packet Core request: {error}"))?;

        // Read the declared message length, not the lifetime of the TCP connection.
        let response = http_response::read_response(stream, 16 * 1024 * 1024)
            .map_err(|error| format!("read Packet Core response: {error}"))?;
        Ok(HttpResponse { status: response.status, body: response.body })
    }
}

// Was: Implementiert das zugehörige Verhalten für `HttpBase`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl HttpBase {
    // Was: Diese Funktion liest und prüft den vorgesehenen Arbeitsschritt.
    // Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
    fn parse(value: &str) -> Result<Self, String> {
        let value = value
            .strip_prefix("http://")
            .ok_or_else(|| "only http:// Packet Core URLs are supported".to_string())?;
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        let (authority, path) = match value.split_once('/') {
            Some((authority, path)) => (authority, format!("/{path}")),
            None => (value, String::new()),
        };
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        let (host, port) = match authority.rsplit_once(':') {
            Some((host, port)) => {
                let port = port
                    .parse::<u16>()
                    .map_err(|_| "invalid Packet Core port".to_string())?;
                (host.to_string(), port)
            }
            None => (authority.to_string(), 80),
        };
        if host.is_empty() {
            return Err("Packet Core host may not be empty".to_string());
        }
        Ok(Self {
            host,
            port,
            base_path: path.trim_end_matches('/').to_string(),
        })
    }

    // Was: Diese Funktion verknüpft path.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn join_path(&self, path: &str) -> String {
        let path = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{path}")
        };
        format!("{}{}", self.base_path, path)
    }
}

// Was: Diese Funktion ermittelt one.
// Warum: Unklare oder indirekte Angaben werden so vor der weiteren Verarbeitung eindeutig gemacht.
fn resolve_one(host: &str, port: u16) -> Result<SocketAddr, String> {
    (host, port)
        .to_socket_addrs()
        .map_err(|error| format!("resolve Packet Core {host}:{port}: {error}"))?
        .next()
        .ok_or_else(|| format!("Packet Core {host}:{port} resolved to no address"))
}

#[cfg(test)]
mod response_tests {
    use super::*;
    use std::io::Read;
    use std::net::TcpListener;
    use std::sync::mpsc;

    #[test]
    fn packet_core_client_receives_json_before_persistent_peer_closes() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let (release, hold) = mpsc::channel();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
            let mut request = Vec::new();
            let mut buffer = [0; 256];
            while !request.windows(4).any(|part| part == b"\r\n\r\n") {
                let count = stream.read(&mut buffer).unwrap();
                assert!(count > 0);
                request.extend_from_slice(&buffer[..count]);
            }
            assert!(request.starts_with(b"GET /api/v1/status HTTP/1.1\r\n"));
            stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: keep-alive\r\n\r\n{}").unwrap();
            let _ = hold.recv_timeout(Duration::from_secs(2));
        });
        let client = PacketCoreClient {
            base: HttpBase { host: "127.0.0.1".to_string(), port: address.port(), base_path: String::new() },
            timeout: Duration::from_millis(150),
        };
        let response: Result<serde_json::Value, _> = client.get_json("/api/v1/status");
        let _ = release.send(());
        server.join().unwrap();
        assert_eq!(response.unwrap(), serde_json::json!({}));
    }
}


