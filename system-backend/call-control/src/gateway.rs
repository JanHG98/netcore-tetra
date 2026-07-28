// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Rufaufbau, Rufzustände und Rufwiederherstellung.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use tungstenite::client::IntoClientRequest;
use tungstenite::http::HeaderValue;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket, connect};

use crate::config::CallControlConfig;
use crate::protocol::{BACKEND_PROTOCOL_VERSION, BackendEvent, BackendRequest};
use crate::state::SharedCalls;

// Was: Diese Funktion startet Gateway Hintergrundverarbeitung.
// Warum: Länger laufende Arbeit blockiert dadurch nicht den aufrufenden Ablauf.
pub fn spawn_gateway_worker(
    config: CallControlConfig,
    calls: SharedCalls,
    rx: Receiver<BackendRequest>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match connect_gateway(&config) {
            Ok(mut socket) => {
                tracing::info!("Call Control connected to Node Gateway {}", config.node_gateway.url);
                calls.gateway_connected();
                if let Err(error) = connected_loop(&mut socket, &calls, &rx) {
                    tracing::warn!("Call Control gateway connection ended: {}", error);
                    calls.gateway_disconnected(error);
                }
            }
            Err(error) => {
                tracing::warn!("Call Control cannot connect to Node Gateway: {}", error);
                calls.gateway_disconnected(error);
            }
        }
        thread::sleep(Duration::from_secs(config.node_gateway.reconnect_secs));
    })
}

// Was: Diese Funktion verbindet Gateway.
// Warum: Der Verbindungsaufbau wird dadurch zentral überwacht und kann sauber fehlschlagen.
fn connect_gateway(
    config: &CallControlConfig,
) -> Result<WebSocket<MaybeTlsStream<std::net::TcpStream>>, String> {
    let mut request = config
        .node_gateway
        .url
        .clone()
        .into_client_request()
        .map_err(|error| format!("invalid Node Gateway URL: {error}"))?;
    request.headers_mut().insert(
        "Sec-WebSocket-Protocol",
        HeaderValue::from_static(BACKEND_PROTOCOL_VERSION),
    );
    let (mut socket, response) =
        connect(request).map_err(|error| format!("Node Gateway connection failed: {error}"))?;
    if response.status() != tungstenite::http::StatusCode::SWITCHING_PROTOCOLS {
        return Err(format!("Node Gateway returned {}", response.status()));
    }
    if let MaybeTlsStream::Plain(stream) = socket.get_mut() {
        let _ = stream.set_read_timeout(Some(Duration::from_millis(200)));
        let _ = stream.set_nodelay(true);
    }
    Ok(socket)
}

// Was: Führt den Arbeitsschritt `connected_loop` für connected loop aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn connected_loop(
    socket: &mut WebSocket<MaybeTlsStream<std::net::TcpStream>>,
    calls: &SharedCalls,
    rx: &Receiver<BackendRequest>,
) -> Result<(), String> {
    // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
    // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
    loop {
        // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
        // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
        loop {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match rx.try_recv() {
                Ok(request) => send_request(socket, &request)?,
                Err(std::sync::mpsc::TryRecvError::Empty) => break,
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    return Err("Call Control command queue closed".to_string());
                }
            }
        }

        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match socket.read() {
            Ok(Message::Text(text)) => {
                handle_event(socket, calls, serde_json::from_str(text.as_str()))?;
            }
            Ok(Message::Binary(data)) => {
                handle_event(socket, calls, serde_json::from_slice(data.as_ref()))?;
            }
            Ok(Message::Ping(payload)) => socket
                .send(Message::Pong(payload))
                .map_err(|error| format!("pong failed: {error}"))?,
            Ok(Message::Pong(_)) => {}
            Ok(Message::Close(_)) => return Err("Node Gateway closed connection".to_string()),
            Ok(_) => {}
            Err(tungstenite::Error::Io(ref error))
                if error.kind() == std::io::ErrorKind::WouldBlock
                    || error.kind() == std::io::ErrorKind::TimedOut => {}
            Err(error) => return Err(format!("Node Gateway read failed: {error}")),
        }

        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for request in calls.expire_operations() {
            send_request(socket, &request)?;
        }
    }
}

// Was: Diese Funktion verarbeitet Ereignis.
// Warum: Die Reaktion auf dieses Ereignis bleibt damit an einer Stelle nachvollziehbar.
fn handle_event(
    socket: &mut WebSocket<MaybeTlsStream<std::net::TcpStream>>,
    calls: &SharedCalls,
    event: Result<BackendEvent, serde_json::Error>,
) -> Result<(), String> {
    let event = event.map_err(|error| format!("invalid Node Gateway event: {error}"))?;
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for request in calls.handle_backend_event(event) {
        send_request(socket, &request)?;
    }
    Ok(())
}

// Was: Diese Funktion sendet request.
// Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
fn send_request(
    socket: &mut WebSocket<MaybeTlsStream<std::net::TcpStream>>,
    request: &BackendRequest,
) -> Result<(), String> {
    let payload = serde_json::to_string(request)
        .map_err(|error| format!("request serialization failed: {error}"))?;
    socket
        .send(Message::Text(payload.into()))
        .map_err(|error| format!("Node Gateway send failed: {error}"))
}
