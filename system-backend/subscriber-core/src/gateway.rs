// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Teilnehmerdaten, Berechtigungen und Registrierung.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use tungstenite::client::IntoClientRequest;
use tungstenite::http::HeaderValue;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket, connect};

use crate::config::SubscriberCoreConfig;
use crate::protocol::{BACKEND_PROTOCOL_VERSION, BackendEvent, BackendRequest};
use crate::state::SharedSubscribers;

// Was: Diese Funktion startet Gateway Hintergrundverarbeitung.
// Warum: Länger laufende Arbeit blockiert dadurch nicht den aufrufenden Ablauf.
pub fn spawn_gateway_worker(
    config: SubscriberCoreConfig,
    subscribers: SharedSubscribers,
    rx: Receiver<BackendRequest>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || run_gateway_worker(config, subscribers, rx))
}

// Was: Diese Funktion führt Gateway Hintergrundverarbeitung.
// Warum: Der Lebenszyklus des Dienstes bleibt so an einer zentralen Stelle steuerbar.
fn run_gateway_worker(
    config: SubscriberCoreConfig,
    subscribers: SharedSubscribers,
    rx: Receiver<BackendRequest>,
) {
    // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
    // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
    loop {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match connect_gateway(&config) {
            Ok(mut socket) => {
                subscribers.gateway_connected();
                if let Err(error) = connected_loop(&mut socket, &subscribers, &rx) {
                    subscribers.gateway_disconnected(error);
                }
            }
            Err(error) => subscribers.gateway_disconnected(error),
        }
        thread::sleep(Duration::from_secs(config.node_gateway.reconnect_secs));
    }
}

// Was: Diese Funktion verbindet Gateway.
// Warum: Der Verbindungsaufbau wird dadurch zentral überwacht und kann sauber fehlschlagen.
fn connect_gateway(
    config: &SubscriberCoreConfig,
) -> Result<WebSocket<MaybeTlsStream<std::net::TcpStream>>, String> {
    let mut request = config.node_gateway.url.clone()
        .into_client_request()
        .map_err(|error| format!("invalid node gateway URL: {error}"))?;
    request.headers_mut().insert(
        "Sec-WebSocket-Protocol",
        HeaderValue::from_static(BACKEND_PROTOCOL_VERSION),
    );
    let (mut socket, response) = connect(request)
        .map_err(|error| format!("node gateway connection failed: {error}"))?;
    if response.status() != tungstenite::http::StatusCode::SWITCHING_PROTOCOLS {
        return Err(format!("node gateway returned {}", response.status()));
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
    subscribers: &SharedSubscribers,
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
                    return Err("subscriber command queue closed".to_string());
                }
            }
        }

        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match socket.read() {
            Ok(Message::Text(text)) => {
                handle_event(socket, subscribers, serde_json::from_str(text.as_str()))?;
            }
            Ok(Message::Binary(data)) => {
                handle_event(socket, subscribers, serde_json::from_slice(data.as_ref()))?;
            }
            Ok(Message::Ping(payload)) => {
                socket.send(Message::Pong(payload))
                    .map_err(|error| format!("pong failed: {error}"))?;
            }
            Ok(Message::Pong(_)) => {}
            Ok(Message::Close(_)) => return Err("node gateway closed connection".to_string()),
            Ok(_) => {}
            Err(tungstenite::Error::Io(ref error))
                if error.kind() == std::io::ErrorKind::WouldBlock
                    || error.kind() == std::io::ErrorKind::TimedOut => {}
            Err(error) => return Err(format!("node gateway read failed: {error}")),
        }

        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for request in subscribers.expire_syncs() {
            send_request(socket, &request)?;
        }
    }
}

// Was: Diese Funktion verarbeitet Ereignis.
// Warum: Die Reaktion auf dieses Ereignis bleibt damit an einer Stelle nachvollziehbar.
fn handle_event(
    socket: &mut WebSocket<MaybeTlsStream<std::net::TcpStream>>,
    subscribers: &SharedSubscribers,
    event: Result<BackendEvent, serde_json::Error>,
) -> Result<(), String> {
    let event = event.map_err(|error| format!("invalid node gateway event: {error}"))?;
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for request in subscribers.handle_backend_event(event) {
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
    socket.send(Message::Text(payload.into()))
        .map_err(|error| format!("node gateway send failed: {error}"))
}
