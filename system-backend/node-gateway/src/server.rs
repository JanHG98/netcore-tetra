// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für die Verbindung zwischen Basisstationen und Backend-Diensten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::io::ErrorKind;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use crate::config::NodeGatewayConfig;
use crate::http::{handle_http_stream, looks_like_websocket_upgrade};
use crate::state::SharedGateway;
use crate::ws::handle_websocket_stream;

// Was: Bündelt die zusammengehörigen Werte für Netzknoten Gateway server in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct NodeGatewayServer {
    config: NodeGatewayConfig,
    gateway: SharedGateway,
}

// Was: Implementiert das zugehörige Verhalten für `NodeGatewayServer`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl NodeGatewayServer {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(config: NodeGatewayConfig, gateway: SharedGateway) -> Self {
        Self { config, gateway }
    }

    // Was: Diese Funktion führt den vorgesehenen Arbeitsschritt.
    // Warum: Der Lebenszyklus des Dienstes bleibt so an einer zentralen Stelle steuerbar.
    pub fn run(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.config.server.bind)?;
        tracing::warn!(
            bind = %self.config.server.bind,
            node_path = %self.config.server.node_path,
            backend_path = %self.config.server.backend_path,
            "Node Gateway listening in OPEN LAB mode: no authentication, no tokens, no TLS"
        );

        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for stream in listener.incoming() {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match stream {
                Ok(stream) => self.spawn_connection(stream),
                Err(error) if error.kind() == ErrorKind::Interrupted => continue,
                Err(error) => tracing::warn!("accept failed: {}", error),
            }
        }
        Ok(())
    }

    // Was: Diese Funktion startet connection.
    // Warum: Länger laufende Arbeit blockiert dadurch nicht den aufrufenden Ablauf.
    fn spawn_connection(&self, stream: TcpStream) {
        let gateway = self.gateway.clone();
        let config = self.config.clone();
        let peer = stream.peer_addr().ok();
        let _ = thread::Builder::new()
            .name("node-gateway-client".to_string())
            .spawn(move || {
                let mut peek = [0u8; 2_048];
                let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
                // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                match stream.peek(&mut peek) {
                    Ok(read) if read > 0 && looks_like_websocket_upgrade(&peek[..read]) => {
                        handle_websocket_stream(stream, gateway, config);
                    }
                    Ok(_) => handle_http_stream(stream, gateway, config),
                    Err(error) => tracing::warn!(?peer, "initial stream peek failed: {}", error),
                }
            });
    }
}
