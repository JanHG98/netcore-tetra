// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

use super::{NetworkAddress, NetworkError, NetworkMessage, NetworkTransport};

/// Configuration for creating a TCP transport
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tcp transport Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TcpTransportConfig {
    /// Server address to connect to
    pub server_addr: NetworkAddress,
    /// Connection timeout
    pub connect_timeout: Duration,
    /// Read timeout
    pub read_timeout: Duration,
}

// Was: Implementiert das zugehörige Verhalten für `TcpTransportConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl TcpTransportConfig {
    /// Create a new TCP transport configuration with default timeouts
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(server_addr: NetworkAddress) -> Self {
        Self {
            server_addr,
            connect_timeout: Duration::from_secs(5),
            read_timeout: Duration::from_secs(30),
        }
    }

    /// Create a new TCP transport configuration with custom timeouts
    // Was: Führt den Arbeitsschritt `with_timeouts` für with timeouts aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn with_timeouts(server_addr: NetworkAddress, connect_timeout: Duration, read_timeout: Duration) -> Self {
        Self {
            server_addr,
            connect_timeout,
            read_timeout,
        }
    }
}

/// TCP-based network transport
// Was: Bündelt die zusammengehörigen Werte für tcp transport in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TcpTransport {
    stream: Option<TcpStream>,
    server_addr: NetworkAddress,
    connect_timeout: Duration,
    read_timeout: Duration,
}

// Was: Implementiert das zugehörige Verhalten für `TcpTransport`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl TcpTransport {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(server_addr: NetworkAddress, connect_timeout: Duration, read_timeout: Duration) -> Self {
        Self {
            stream: None,
            server_addr,
            connect_timeout,
            read_timeout,
        }
    }

    // Was: Diese Funktion stellt stream exists.
    // Warum: So wird die notwendige Voraussetzung hergestellt, bevor abhängiger Code weiterläuft.
    fn ensure_stream_exists(&mut self) -> Result<(), NetworkError> {
        if self.stream.is_none() {
            self.connect()?;
        }
        Ok(())
    }

    // Was: Diese Funktion liest tcp addr.
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    fn get_tcp_addr(&self) -> Result<String, NetworkError> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match &self.server_addr {
            NetworkAddress::Tcp { host, port } => Ok(format!("{}:{}", host, port)),
            _ => Err(NetworkError::ConnectionFailed("Invalid address type for TcpTransport".to_string())),
        }
    }

    /// Closes the current TCP connection, if any.
    // Was: Diese Funktion schließt connection.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn close_connection(&mut self) {
        if let Some(stream) = self.stream.take() {
            let _ = stream.shutdown(std::net::Shutdown::Both);
        }
    }

    /// Internal send implementation - does the actual I/O
    // Was: Diese Funktion sendet den vorgesehenen Arbeitsschritt.
    // Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
    fn try_send(&mut self, payload: &[u8]) -> Result<(), NetworkError> {
        if let Some(ref mut stream) = self.stream {
            // Send message length first, then payload
            let len = payload.len() as u32;
            let len_bytes = len.to_be_bytes();

            stream
                .write_all(&len_bytes)
                .map_err(|e| NetworkError::SendFailed(format!("Failed to send length: {}", e)))?;

            stream
                .write_all(payload)
                .map_err(|e| NetworkError::SendFailed(format!("Failed to send payload: {}", e)))?;

            stream
                .flush()
                .map_err(|e| NetworkError::SendFailed(format!("Failed to flush: {}", e)))?;

            Ok(())
        } else {
            Err(NetworkError::SendFailed("No active connection".to_string()))
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `NetworkTransport for TcpTransport`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl NetworkTransport for TcpTransport {
    // Was: Diese Funktion verbindet den vorgesehenen Arbeitsschritt.
    // Warum: Der Verbindungsaufbau wird dadurch zentral überwacht und kann sauber fehlschlagen.
    fn connect(&mut self) -> Result<(), NetworkError> {
        tracing::debug!("TcpTransport connecting to {:?}", self.server_addr);
        // Close any existing connection gracefully
        self.close_connection();
        let addr = self.get_tcp_addr()?;

        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match TcpStream::connect_timeout(
            &addr
                .parse()
                .map_err(|e| NetworkError::ConnectionFailed(format!("Invalid address: {}", e)))?,
            self.connect_timeout,
        ) {
            Ok(stream) => {
                // Set read timeout
                stream
                    .set_read_timeout(Some(self.read_timeout))
                    .map_err(|e| NetworkError::ConnectionFailed(format!("Failed to set timeout: {}", e)))?;

                self.stream = Some(stream);
                Ok(())
            }
            Err(e) => Err(NetworkError::ConnectionFailed(format!("TCP connect failed: {}", e))),
        }
    }

    // Was: Diese Funktion sendet reliable.
    // Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
    fn send_reliable(&mut self, payload: &[u8]) -> Result<(), NetworkError> {
        self.ensure_stream_exists()?;

        // Try to send, and retry once after reconnect if the connection was stale
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self.try_send(payload) {
            Ok(()) => Ok(()),
            Err(e) => {
                // Connection may have been closed by server (idle timeout, etc.)
                tracing::trace!("Send failed, attempting reconnect: {}", e);
                self.connect()?;
                self.try_send(payload)
            }
        }
    }

    // Was: Diese Funktion sendet unreliable.
    // Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
    fn send_unreliable(&mut self, _payload: &[u8]) -> Result<(), NetworkError> {
        unimplemented!("TCP transport does not support unreliable messaging")
    }

    // Was: Diese Funktion empfängt reliable.
    // Warum: Eingehende Daten werden so geordnet geprüft, bevor sie weiterverteilt werden.
    fn receive_reliable(&mut self) -> Vec<NetworkMessage> {
        let mut messages = Vec::new();

        if let Some(ref mut stream) = self.stream {
            tracing::debug!("TCP receive: checking for messages on connection");
            // Set non-blocking mode for receiving
            if let Err(e) = stream.set_nonblocking(true) {
                tracing::error!("Failed to set non-blocking mode: {}", e);
                return messages;
            }

            // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
            // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
            loop {
                // Try to read message length
                let mut len_bytes = [0u8; 4];
                // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                match stream.read_exact(&mut len_bytes) {
                    Ok(()) => {
                        let payload_len = u32::from_be_bytes(len_bytes) as usize;
                        tracing::info!("Received message length: {} bytes", payload_len);

                        // Reasonable message size limit
                        if payload_len > 1024 * 1024 {
                            tracing::warn!("Message too large: {} bytes", payload_len);
                            break;
                        }

                        // Read payload
                        let mut payload = vec![0u8; payload_len];
                        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                        match stream.read_exact(&mut payload) {
                            Ok(()) => {
                                messages.push(NetworkMessage {
                                    source: self.server_addr.clone(),
                                    payload,
                                    timestamp: Instant::now(),
                                });
                            }
                            Err(_) => break, // Connection closed or error
                        }
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        // No more data available
                        tracing::debug!("TCP receive: no data available (would block)");
                        break;
                    }
                    Err(e) => {
                        tracing::debug!("TCP receive: error reading length header: {}", e);
                        break; // Connection error
                    }
                }
            }

            // Restore blocking mode
            let _ = stream.set_nonblocking(false);
        }

        messages
    }

    // Was: Diese Funktion empfängt unreliable.
    // Warum: Eingehende Daten werden so geordnet geprüft, bevor sie weiterverteilt werden.
    fn receive_unreliable(&mut self) -> Vec<NetworkMessage> {
        unimplemented!("TCP transport does not support unreliable messaging")
    }

    /// Wait for a single response message with blocking read and timeout
    /// Used for request-response patterns where we expect a reply
    // Was: Diese Funktion wartet for response reliable.
    // Warum: Nachfolgende Schritte laufen dadurch erst weiter, wenn ihre Voraussetzung wirklich erfüllt ist.
    fn wait_for_response_reliable(&mut self) -> Result<NetworkMessage, NetworkError> {
        if let Some(ref mut stream) = self.stream {
            // Ensure blocking mode (should already be set from reconnect)
            stream
                .set_nonblocking(false)
                .map_err(|e| NetworkError::ReceiveFailed(format!("Failed to set blocking mode: {}", e)))?;

            // Read message length (blocking with timeout)
            let mut len_bytes = [0u8; 4];
            stream
                .read_exact(&mut len_bytes)
                .map_err(|e| NetworkError::ReceiveFailed(format!("Failed to read length header: {}", e)))?;

            let payload_len = u32::from_be_bytes(len_bytes) as usize;
            tracing::info!("Received message length: {} bytes", payload_len);

            // Reasonable message size limit
            if payload_len > 1024 * 1024 {
                return Err(NetworkError::ReceiveFailed(format!("Message too large: {} bytes", payload_len)));
            }

            // Read payload (blocking with timeout)
            let mut payload = vec![0u8; payload_len];
            stream
                .read_exact(&mut payload)
                .map_err(|e| NetworkError::ReceiveFailed(format!("Failed to read payload: {}", e)))?;

            Ok(NetworkMessage {
                source: self.server_addr.clone(),
                payload,
                timestamp: Instant::now(),
            })
        } else {
            Err(NetworkError::ReceiveFailed("No active connection".to_string()))
        }
    }
}
