// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::time::Instant;

use serde::{Deserialize, Serialize};

#[cfg(test)]
// Was: Bindet das Untermodul mock in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mock;

/// QUIC transport implementation
// Was: Bindet das Untermodul quic in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod quic;

/// WebSocket transport implementation
// Was: Bindet das Untermodul websocket in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod websocket;

/// Basic TCP transport implementation
// Was: Bindet das Untermodul tcp in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod tcp;

/// Network transport abstraction for Entity-to-network external communications
///
/// This trait defines a unified interface for both reliable (TCP, QUIC streams)
/// and unreliable (UDP, QUIC datagrams) transports. Transports should either
/// implement those methods or raise an unimplemented!() panic.
// Was: Beschreibt das gemeinsame Verhalten für network transport.
// Warum: Unterschiedliche Implementierungen können dadurch über dieselbe verständliche Schnittstelle benutzt werden.
pub trait NetworkTransport: Send {
    /// Connect or reconnect the transport. Destroys any existing connection.
    // Was: Diese Funktion verbindet den vorgesehenen Arbeitsschritt.
    // Warum: Der Verbindungsaufbau wird dadurch zentral überwacht und kann sauber fehlschlagen.
    fn connect(&mut self) -> Result<(), NetworkError>;

    /// Send a message reliably (guaranteed delivery, ordered arrival)
    // Was: Diese Funktion sendet reliable.
    // Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
    fn send_reliable(&mut self, payload: &[u8]) -> Result<(), NetworkError>;

    /// Send a message unreliably (no delivery guarantee, unordered, lower latency)
    // Was: Diese Funktion sendet unreliable.
    // Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
    fn send_unreliable(&mut self, payload: &[u8]) -> Result<(), NetworkError>;

    /// Receive pending messages from the reliable channel (non-blocking)
    // Was: Diese Funktion empfängt reliable.
    // Warum: Eingehende Daten werden so geordnet geprüft, bevor sie weiterverteilt werden.
    fn receive_reliable(&mut self) -> Vec<NetworkMessage>;

    /// Receive pending messages from the unreliable channel (non-blocking)
    // Was: Diese Funktion empfängt unreliable.
    // Warum: Eingehende Daten werden so geordnet geprüft, bevor sie weiterverteilt werden.
    fn receive_unreliable(&mut self) -> Vec<NetworkMessage>;

    /// Wait for a single response on the reliable channel (blocking with timeout)
    // Was: Diese Funktion wartet for response reliable.
    // Warum: Nachfolgende Schritte laufen dadurch erst weiter, wenn ihre Voraussetzung wirklich erfüllt ist.
    fn wait_for_response_reliable(&mut self) -> Result<NetworkMessage, NetworkError>;

    /// Disconnect the transport gracefully
    // Was: Diese Funktion trennt den vorgesehenen Arbeitsschritt.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn disconnect(&mut self) {}

    /// Check if the transport is currently connected
    // Was: Prüft, ob connected zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    fn is_connected(&self) -> bool {
        true
    }

    /// Return the Brew protocol version advertised by the server in the last connect response.
    /// Default is 0 (v0 / unknown). WebSocketTransport overrides this.
    // Was: Führt den Arbeitsschritt `server_brew_version` für server Brew-Verbindung version aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn server_brew_version(&self) -> u8 {
        0
    }
}

/// Factory trait for creating transport instances
///
/// Each transport type implements this to define how it gets constructed
/// from a configuration type. This allows generic workers to create transports
/// without knowing the specific construction details.
// Was: Beschreibt das gemeinsame Verhalten für transport factory.
// Warum: Unterschiedliche Implementierungen können dadurch über dieselbe verständliche Schnittstelle benutzt werden.
pub trait TransportFactory: NetworkTransport + Sized {
    /// Configuration type needed to construct this transport
    // Was: Vergibt für Konfiguration einen fachlich verständlichen Typnamen.
    // Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
    type Config: Send + 'static;

    /// Create a new transport instance from configuration
    // Was: Diese Funktion erstellt den vorgesehenen Arbeitsschritt.
    // Warum: Neue Objekte erhalten so immer einen vollständigen und gültigen Ausgangszustand.
    fn create(config: Self::Config) -> Result<Self, NetworkError>;
}

/// Network address abstraction
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// Was: Listet die möglichen Varianten für network address auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum NetworkAddress {
    /// TCP endpoint
    Tcp { host: String, port: u16 },
    /// UDP endpoint  
    Udp { host: String, port: u16 },
    /// Custom addressing scheme
    Custom { scheme: String, address: String },
}

/// Network message received from external source
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für network Nachricht in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct NetworkMessage {
    pub source: NetworkAddress,
    pub payload: Vec<u8>,
    pub timestamp: Instant,
}

/// Network-related errors
#[derive(Debug, Clone)]
// Was: Listet die möglichen Varianten für network error auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum NetworkError {
    ConnectionFailed(String),
    SendFailed(String),
    ReceiveFailed(String),
    SerializationError(String),
    InvalidService(String),
    InvalidServiceVersion(String),
    Timeout,
}

// Was: Implementiert das zugehörige Verhalten für `std::fmt::Display for NetworkError`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::fmt::Display for NetworkError {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            NetworkError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            NetworkError::SendFailed(msg) => write!(f, "Send failed: {}", msg),
            NetworkError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            NetworkError::InvalidService(msg) => write!(f, "Invalid service: {}", msg),
            NetworkError::InvalidServiceVersion(msg) => write!(f, "Invalid service version: {}", msg),
            NetworkError::ReceiveFailed(_) => write!(f, "Receive failed"),
            NetworkError::Timeout => write!(f, "Operation timed out"),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `std::error::Error for NetworkError {}`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::error::Error for NetworkError {}
