use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::thread;
use std::time::{Duration, Instant};

use tungstenite::client::IntoClientRequest;
use tungstenite::http::HeaderValue;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket, connect};

use crate::config::MediaSwitchConfig;
use crate::protocol::{
    CALL_CONTROL_MEDIA_PROTOCOL_VERSION, CallControlCall, CallControlMediaEvent,
};
use crate::state::SharedMedia;

pub fn spawn_call_control_worker(
    config: MediaSwitchConfig,
    media: SharedMedia,
) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        // Always seed the route graph before opening the event stream. This also
        // makes a restarted Media Switch useful when Call Control is temporarily
        // unable to accept WebSocket clients.
        match fetch_calls(&config) {
            Ok(calls) => {
                let _ = media.reconcile_calls(calls);
            }
            Err(error) => media.call_control_failed(error),
        }

        match connect_events(&config) {
            Ok(mut socket) => {
                tracing::info!(
                    "Media Switch subscribed to Call Control events {}",
                    config.call_control.events_url
                );
                if let Err(error) = connected_loop(&mut socket, &config, &media) {
                    tracing::warn!("Call Control event connection ended: {}", error);
                    media.call_control_failed(error);
                }
            }
            Err(error) => {
                tracing::warn!("Cannot connect to Call Control event stream: {}", error);
                media.call_control_failed(error);
            }
        }

        thread::sleep(Duration::from_secs(config.call_control.reconnect_secs));
    })
}

fn connect_events(
    config: &MediaSwitchConfig,
) -> Result<WebSocket<MaybeTlsStream<TcpStream>>, String> {
    let mut request = config
        .call_control
        .events_url
        .clone()
        .into_client_request()
        .map_err(|error| format!("invalid Call Control event URL: {error}"))?;
    request.headers_mut().insert(
        "Sec-WebSocket-Protocol",
        HeaderValue::from_static(CALL_CONTROL_MEDIA_PROTOCOL_VERSION),
    );
    let (mut socket, response) =
        connect(request).map_err(|error| format!("Call Control event connection failed: {error}"))?;
    if response.status() != tungstenite::http::StatusCode::SWITCHING_PROTOCOLS {
        return Err(format!("Call Control event endpoint returned {}", response.status()));
    }
    let negotiated_protocol = response
        .headers()
        .get("sec-websocket-protocol")
        .and_then(|value| value.to_str().ok());
    if negotiated_protocol != Some(CALL_CONTROL_MEDIA_PROTOCOL_VERSION) {
        return Err("Call Control did not negotiate the media websocket subprotocol".to_string());
    }
    if let MaybeTlsStream::Plain(stream) = socket.get_mut() {
        let _ = stream.set_read_timeout(Some(Duration::from_millis(200)));
        let _ = stream.set_write_timeout(Some(Duration::from_secs(2)));
        let _ = stream.set_nodelay(true);
    }
    Ok(socket)
}

fn connected_loop(
    socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
    config: &MediaSwitchConfig,
    media: &SharedMedia,
) -> Result<(), String> {
    let mut last_fallback = Instant::now();
    let mut last_ack_retry = Instant::now();
    let mut last_revision = None;
    let mut acknowledged = HashMap::<String, u64>::new();
    loop {
        match socket.read() {
            Ok(Message::Text(text)) => {
                last_revision = Some(apply_event(
                    media,
                    config,
                    serde_json::from_str(text.as_str()),
                    &mut acknowledged,
                )?);
            }
            Ok(Message::Binary(data)) => {
                last_revision = Some(apply_event(
                    media,
                    config,
                    serde_json::from_slice(data.as_ref()),
                    &mut acknowledged,
                )?);
            }
            Ok(Message::Ping(payload)) => socket
                .send(Message::Pong(payload))
                .map_err(|error| format!("Call Control pong failed: {error}"))?,
            Ok(Message::Pong(_)) => {}
            Ok(Message::Close(_)) => {
                return Err("Call Control closed event connection".to_string());
            }
            Ok(_) => {}
            Err(tungstenite::Error::Io(ref error))
                if error.kind() == std::io::ErrorKind::WouldBlock
                    || error.kind() == std::io::ErrorKind::TimedOut => {}
            Err(error) => return Err(format!("Call Control event read failed: {error}")),
        }

        if last_ack_retry.elapsed() >= Duration::from_secs(1) {
            if let Some(revision) = last_revision {
                acknowledge_ready_calls(
                    config,
                    media.route_ready_sessions(),
                    revision,
                    &mut acknowledged,
                );
                acknowledged.retain(|session_id, _| media.session(session_id).is_some());
            }
            last_ack_retry = Instant::now();
        }

        // This is a safety net, not the speech-path synchronisation mechanism.
        // Normal call/leg/floor changes arrive immediately over the WebSocket.
        if last_fallback.elapsed() >= Duration::from_secs(config.call_control.reconcile_secs) {
            match fetch_calls(config) {
                Ok(calls) => {
                    let ready_calls = media.reconcile_calls(calls);
                    if let Some(revision) = last_revision {
                        acknowledge_ready_calls(
                            config,
                            ready_calls,
                            revision,
                            &mut acknowledged,
                        );
                    }
                }
                Err(error) => tracing::warn!("Call Control fallback reconcile failed: {}", error),
            }
            last_fallback = Instant::now();
        }
    }
}

fn apply_event(
    media: &SharedMedia,
    config: &MediaSwitchConfig,
    event: Result<CallControlMediaEvent, serde_json::Error>,
    acknowledged: &mut HashMap<String, u64>,
) -> Result<u64, String> {
    let event = event.map_err(|error| format!("invalid Call Control media event: {error}"))?;
    tracing::debug!(
        kind = %event.kind,
        reason = %event.reason,
        revision = event.revision,
        emitted_at = %event.emitted_at,
        logical_call_id = ?event.logical_call_id,
        "Call Control media topology update"
    );
    let revision = event.revision;
    acknowledge_ready_calls(
        config,
        media.reconcile_calls(event.calls),
        revision,
        acknowledged,
    );
    Ok(revision)
}

fn acknowledge_ready_calls(
    config: &MediaSwitchConfig,
    ready_calls: Vec<String>,
    revision: u64,
    acknowledged: &mut HashMap<String, u64>,
) {
    for logical_call_id in ready_calls {
        if acknowledged
            .get(&logical_call_id)
            .is_some_and(|known_revision| *known_revision >= revision)
        {
            continue;
        }
        if let Err(error) = post_route_ready(config, &logical_call_id, revision) {
            tracing::warn!(
                logical_call_id = %logical_call_id,
                revision,
                "RouteReady acknowledgement failed: {}",
                error
            );
        } else {
            acknowledged.insert(logical_call_id, revision);
        }
    }
}

fn post_route_ready(
    config: &MediaSwitchConfig,
    logical_call_id: &str,
    revision: u64,
) -> Result<(), String> {
    let parsed = ParsedHttpUrl::parse(&config.call_control.route_ready_url)?;
    let timeout = Duration::from_secs(config.call_control.request_timeout_secs);
    let address = (parsed.host.as_str(), parsed.port)
        .to_socket_addrs()
        .map_err(|error| format!("Call Control RouteReady DNS failed: {error}"))?
        .next()
        .ok_or_else(|| "Call Control RouteReady address did not resolve".to_string())?;
    let mut stream = TcpStream::connect_timeout(&address, timeout)
        .map_err(|error| format!("Call Control RouteReady connection failed: {error}"))?;
    stream
        .set_read_timeout(Some(timeout))
        .map_err(|error| format!("Call Control RouteReady read timeout failed: {error}"))?;
    stream
        .set_write_timeout(Some(timeout))
        .map_err(|error| format!("Call Control RouteReady write timeout failed: {error}"))?;
    let _ = stream.set_nodelay(true);

    let body = serde_json::to_vec(&serde_json::json!({
        "logical_call_id": logical_call_id,
        "revision": revision
    }))
    .map_err(|error| format!("Call Control RouteReady JSON failed: {error}"))?;
    let request = format!(
        concat!(
            "POST {} HTTP/1.1\r\n",
            "Host: {}:{}\r\n",
            "Content-Type: application/json\r\n",
            "Accept: application/json\r\n",
            "Content-Length: {}\r\n",
            "Connection: close\r\n\r\n"
        ),
        parsed.path,
        parsed.host,
        parsed.port,
        body.len()
    );
    stream
        .write_all(request.as_bytes())
        .and_then(|_| stream.write_all(&body))
        .map_err(|error| format!("Call Control RouteReady request failed: {error}"))?;

    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .map_err(|error| format!("Call Control RouteReady response failed: {error}"))?;
    let header_end = find_subslice(&response, b"\r\n\r\n")
        .ok_or_else(|| "Call Control RouteReady returned an invalid HTTP response".to_string())?;
    let header = std::str::from_utf8(&response[..header_end])
        .map_err(|_| "Call Control RouteReady headers are not UTF-8".to_string())?;
    let status = header
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|value| value.parse::<u16>().ok())
        .ok_or_else(|| "Call Control RouteReady response has no valid status".to_string())?;
    if !(200..300).contains(&status) {
        return Err(format!("Call Control RouteReady returned HTTP {status}"));
    }
    Ok(())
}

fn fetch_calls(config: &MediaSwitchConfig) -> Result<Vec<CallControlCall>, String> {
    let parsed = ParsedHttpUrl::parse(&config.call_control.url)?;
    let timeout = Duration::from_secs(config.call_control.request_timeout_secs);
    let address = (parsed.host.as_str(), parsed.port)
        .to_socket_addrs()
        .map_err(|error| format!("Call Control DNS failed: {error}"))?
        .next()
        .ok_or_else(|| "Call Control address did not resolve".to_string())?;
    let mut stream = TcpStream::connect_timeout(&address, timeout)
        .map_err(|error| format!("Call Control connection failed: {error}"))?;
    stream
        .set_read_timeout(Some(timeout))
        .map_err(|error| format!("Call Control read timeout failed: {error}"))?;
    stream
        .set_write_timeout(Some(timeout))
        .map_err(|error| format!("Call Control write timeout failed: {error}"))?;
    let _ = stream.set_nodelay(true);

    let request = format!(
        concat!(
            "GET {} HTTP/1.1\r\n",
            "Host: {}:{}\r\n",
            "Accept: application/json\r\n",
            "Connection: close\r\n\r\n"
        ),
        parsed.path, parsed.host, parsed.port
    );
    stream
        .write_all(request.as_bytes())
        .map_err(|error| format!("Call Control request failed: {error}"))?;

    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .map_err(|error| format!("Call Control response failed: {error}"))?;
    let header_end = find_subslice(&response, b"\r\n\r\n")
        .ok_or_else(|| "Call Control returned an invalid HTTP response".to_string())?;
    let header = std::str::from_utf8(&response[..header_end])
        .map_err(|_| "Call Control response headers are not UTF-8".to_string())?;
    let status = header
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|value| value.parse::<u16>().ok())
        .ok_or_else(|| "Call Control response has no valid status".to_string())?;
    if status != 200 {
        return Err(format!("Call Control returned HTTP {status}"));
    }
    serde_json::from_slice(&response[header_end + 4..])
        .map_err(|error| format!("Call Control JSON failed: {error}"))
}

struct ParsedHttpUrl {
    host: String,
    port: u16,
    path: String,
}

impl ParsedHttpUrl {
    fn parse(url: &str) -> Result<Self, String> {
        let rest = url
            .strip_prefix("http://")
            .ok_or_else(|| "Call Control URL must start with http://".to_string())?;
        let (authority, path) = rest.split_once('/').unwrap_or((rest, ""));
        let (host, port) = authority
            .rsplit_once(':')
            .map(|(host, port)| {
                port.parse::<u16>()
                    .map(|port| (host.to_string(), port))
                    .map_err(|_| "invalid Call Control port".to_string())
            })
            .transpose()?
            .unwrap_or_else(|| (authority.to_string(), 80));
        if host.trim().is_empty() {
            return Err("Call Control host must not be empty".to_string());
        }
        Ok(Self {
            host,
            port,
            path: format!("/{path}"),
        })
    }
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

#[cfg(test)]
mod tests {
    use super::ParsedHttpUrl;

    #[test]
    fn parses_open_lab_call_control_url() {
        let url = ParsedHttpUrl::parse("http://127.0.0.1:8120/api/v1/calls")
            .expect("URL parses");
        assert_eq!(url.host, "127.0.0.1");
        assert_eq!(url.port, 8120);
        assert_eq!(url.path, "/api/v1/calls");
    }
}
