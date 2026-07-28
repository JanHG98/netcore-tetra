// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Rufaufbau, Rufzustände und Rufwiederherstellung.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::net::TcpStream;
use std::time::Duration;

use tungstenite::handshake::server::{ErrorResponse, Request, Response};
use tungstenite::http::StatusCode;
use tungstenite::{Message, accept_hdr};

use crate::state::SharedCalls;

// Was: Legt den festen Wert `MEDIA_EVENT_PROTOCOL_VERSION` für Audio- und Mediendaten Ereignis protocol version fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const MEDIA_EVENT_PROTOCOL_VERSION: &str = "netcore-call-control-media-v1";
// Was: Legt den festen Wert `MEDIA_EVENT_PATH` für Audio- und Mediendaten Ereignis path fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const MEDIA_EVENT_PATH: &str = "/ws/media";

// Was: Führt den Arbeitsschritt `reject_websocket` für reject websocket aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn reject_websocket(status: StatusCode, message: &str) -> ErrorResponse {
    tungstenite::http::Response::builder()
        .status(status)
        .header("Content-Type", "text/plain; charset=utf-8")
        .header("Content-Length", message.len().to_string())
        .header("Connection", "close")
        .body(Some(message.to_string()))
        .expect("valid websocket rejection response")
}

// Was: Diese Funktion verarbeitet Audio- und Mediendaten websocket.
// Warum: Die Reaktion auf dieses Ereignis bleibt damit an einer Stelle nachvollziehbar.
pub fn handle_media_websocket(stream: TcpStream, calls: SharedCalls) {
    let peer = stream.peer_addr().ok();
    let callback = move |request: &Request, mut response: Response| {
        if request.uri().path() != MEDIA_EVENT_PATH {
            return Err(reject_websocket(
                StatusCode::NOT_FOUND,
                "unknown Call Control websocket endpoint",
            ));
        }

        let protocol_supported = request
            .headers()
            .get("sec-websocket-protocol")
            .and_then(|value| value.to_str().ok())
            .is_some_and(|requested| {
                requested
                    .split(',')
                    .map(str::trim)
                    .any(|protocol| protocol == MEDIA_EVENT_PROTOCOL_VERSION)
            });
        if !protocol_supported {
            return Err(reject_websocket(
                StatusCode::BAD_REQUEST,
                "missing or unsupported Call Control media websocket subprotocol",
            ));
        }
        response.headers_mut().insert(
            "Sec-WebSocket-Protocol",
            MEDIA_EVENT_PROTOCOL_VERSION
                .parse()
                .expect("valid media websocket protocol header"),
        );

        response.headers_mut().insert(
            "x-netcore-call-control-media",
            "1".parse().expect("valid media marker"),
        );
        response.headers_mut().insert(
            "x-netcore-security-mode",
            "open-lab".parse().expect("valid security mode marker"),
        );
        Ok(response)
    };

    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    let mut socket = match accept_hdr(stream, callback) {
        Ok(socket) => socket,
        Err(error) => {
            tracing::warn!(?peer, "Call Control media websocket handshake failed: {}", error);
            return;
        }
    };

    let _ = socket.get_mut().set_write_timeout(Some(Duration::from_secs(2)));
    let _ = socket.get_mut().set_nodelay(true);
    let receiver = calls.subscribe_media();

    tracing::info!(?peer, "Media Switch subscribed to Call Control events");
    // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
    // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
    loop {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match receiver.recv_timeout(Duration::from_secs(10)) {
            Ok(event) => {
                // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                let payload = match serde_json::to_string(&event) {
                    Ok(payload) => payload,
                    Err(error) => {
                        tracing::error!("Call Control media event serialization failed: {}", error);
                        continue;
                    }
                };
                if let Err(error) = socket.send(Message::Text(payload.into())) {
                    tracing::warn!(?peer, "Call Control media websocket send failed: {}", error);
                    break;
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                if let Err(error) = socket.send(Message::Ping(Vec::new().into())) {
                    tracing::warn!(?peer, "Call Control media websocket ping failed: {}", error);
                    break;
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
}
