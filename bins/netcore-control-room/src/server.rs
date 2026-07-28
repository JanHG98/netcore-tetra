// NETCORE-KOMMENTAR – Was: Enthält die Logik oder Einstellungen für server.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::io::ErrorKind;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use serde_json::Value;

use crate::auth::AuthState;
use crate::http::{handle_http_stream, looks_like_websocket_upgrade, SharedDirectory};
use crate::operations::SharedOperations;
use crate::state::SharedControlRoom;
use crate::ws::handle_websocket_stream;

// Was: Bündelt die zusammengehörigen Werte für Steuerung room server in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ControlRoomServer {
    bind: SocketAddr,
    node_path: String,
    ui_path: String,
    state: SharedControlRoom,
    auth: AuthState,
    directory: SharedDirectory,
    operations: SharedOperations,
}

// Was: Implementiert das zugehörige Verhalten für `ControlRoomServer`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl ControlRoomServer {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(
        bind: SocketAddr,
        node_path: String,
        ui_path: String,
        state: SharedControlRoom,
        auth: AuthState,
        directory: Value,
        operations: SharedOperations,
    ) -> Self {
        Self {
            bind,
            node_path: normalize_path(node_path),
            ui_path: normalize_path(ui_path),
            state,
            auth,
            directory: std::sync::Arc::new(std::sync::Mutex::new(directory)),
            operations,
        }
    }

    // Was: Diese Funktion führt den vorgesehenen Arbeitsschritt.
    // Warum: Der Lebenszyklus des Dienstes bleibt so an einer zentralen Stelle steuerbar.
    pub fn run(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.bind)?;
        tracing::info!(bind = %self.bind, node_path = %self.node_path, ui_path = %self.ui_path, "NetCore Control Room listening");

        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for stream in listener.incoming() {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match stream {
                Ok(stream) => self.spawn_connection(stream),
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(e) => tracing::warn!("accept failed: {}", e),
            }
        }
        Ok(())
    }

    // Was: Diese Funktion startet connection.
    // Warum: Länger laufende Arbeit blockiert dadurch nicht den aufrufenden Ablauf.
    fn spawn_connection(&self, stream: TcpStream) {
        let state = self.state.clone();
        let node_path = self.node_path.clone();
        let ui_path = self.ui_path.clone();
        let auth = self.auth.clone();
        let directory = self.directory.clone();
        let operations = self.operations.clone();
        let peer = stream.peer_addr().ok();

        let _ = thread::Builder::new()
            .name("control-room-client".to_string())
            .spawn(move || {
                let mut peek = [0u8; 2048];
                let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
                // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                match stream.peek(&mut peek) {
                    Ok(n) if n > 0 && looks_like_websocket_upgrade(&peek[..n]) => {
                        handle_websocket_stream(stream, state, node_path, ui_path, auth);
                    }
                    Ok(_) => {
                        handle_http_stream(stream, state, &node_path, &ui_path, auth, directory, operations);
                    }
                    Err(e) => {
                        tracing::warn!(?peer, "initial stream peek failed: {}", e);
                    }
                }
            });
    }
}

// Was: Führt den Arbeitsschritt `normalize_path` für normalize path aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn normalize_path(path: String) -> String {
    if path.starts_with('/') {
        path
    } else {
        format!("/{}", path)
    }
}
