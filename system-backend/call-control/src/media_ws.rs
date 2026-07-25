use std::net::TcpStream;
use std::time::Duration;

use tungstenite::handshake::server::{ErrorResponse, Request, Response};
use tungstenite::http::StatusCode;
use tungstenite::{Message, accept_hdr};

use crate::state::SharedCalls;

pub const MEDIA_EVENT_PROTOCOL_VERSION: &str = "netcore-call-control-media-v1";
pub const MEDIA_EVENT_PATH: &str = "/ws/media";

fn reject_websocket(status: StatusCode, message: &str) -> ErrorResponse {
    tungstenite::http::Response::builder()
        .status(status)
        .header("Content-Type", "text/plain; charset=utf-8")
        .header("Content-Length", message.len().to_string())
        .header("Connection", "close")
        .body(Some(message.to_string()))
        .expect("valid websocket rejection response")
}

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
    loop {
        match receiver.recv_timeout(Duration::from_secs(10)) {
            Ok(event) => {
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
