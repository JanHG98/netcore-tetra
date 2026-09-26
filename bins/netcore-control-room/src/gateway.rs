//! Read-only Node Gateway telemetry. No command sender is registered for these
//! nodes; central SDS and other control remain owned by their existing services.

use std::net::{TcpStream, ToSocketAddrs};
use std::thread;
use std::time::{Duration, Instant};

use serde::Deserialize;
use tetra_entities::net_control_room::{
    ControlRoomNodeCapabilities, ControlRoomNodeIdentity, NodeToControlRoomMessage,
};
use tungstenite::client::IntoClientRequest;
use tungstenite::http::HeaderValue;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Message, WebSocket};

use crate::config::NodeGatewayConfig;
use crate::state::SharedControlRoom;

const PROTOCOL: &str = "netcore-node-gateway-backend-v1";

#[derive(Debug, Deserialize)]
pub struct GatewayNodeSnapshot {
    pub node_id: String,
    pub session_id: String,
    pub connected: bool,
    pub stale: bool,
    pub last_seen: String,
    pub connected_at: String,
    pub identity: ControlRoomNodeIdentity,
    pub capabilities: ControlRoomNodeCapabilities,
}

#[derive(Debug, Deserialize)]
struct GatewaySnapshot {
    nodes: Vec<GatewayNodeSnapshot>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum GatewayEvent {
    Snapshot { snapshot: GatewaySnapshot },
    NodeMessage { node_id: String, message: NodeToControlRoomMessage },
    // Event/action_result frames are transport metadata, never TBS commands.
    #[serde(other)]
    Other,
}

pub fn spawn(config: NodeGatewayConfig, state: SharedControlRoom) -> Result<(), Box<dyn std::error::Error>> {
    if !config.enabled {
        return Ok(());
    }
    let request = config.url.as_str().into_client_request()?;
    if !matches!(request.uri().scheme_str(), Some("ws" | "wss")) || request.uri().host().is_none() {
        return Err("node_gateway.url must be a ws:// or wss:// endpoint".into());
    }
    thread::Builder::new().name("control-room-gateway".to_string()).spawn(move || {
        loop {
            let result = connect(&config).and_then(|mut socket| {
                tracing::info!("Node Gateway telemetry observer connected");
                connected_loop(&mut socket, &config, &state)
            });
            state.gateway_disconnected();
            if let Err(error) = result {
                tracing::warn!(%error, "Node Gateway telemetry observer disconnected");
            }
            thread::sleep(Duration::from_secs(config.reconnect_secs));
        }
    })?;
    Ok(())
}

fn connect(config: &NodeGatewayConfig) -> Result<WebSocket<MaybeTlsStream<TcpStream>>, String> {
    let mut request = config.url.as_str().into_client_request().map_err(|error| error.to_string())?;
    request.headers_mut().insert("Sec-WebSocket-Protocol", HeaderValue::from_static(PROTOCOL));
    let host = request.uri().host().ok_or("Gateway host missing")?.trim_matches(['[', ']']);
    let port = request.uri().port_u16().unwrap_or(if request.uri().scheme_str() == Some("wss") { 443 } else { 80 });
    let addresses = (host, port).to_socket_addrs().map_err(|error| error.to_string())?;
    let mut failure = "Gateway host has no addresses".to_string();
    let mut connected = None;
    for address in addresses {
        match TcpStream::connect_timeout(&address, Duration::from_secs(config.timeout_secs)) {
            Ok(stream) => { connected = Some(stream); break; }
            Err(error) => failure = error.to_string(),
        }
    }
    let stream = connected.ok_or(failure)?;
    stream.set_read_timeout(Some(Duration::from_secs(config.timeout_secs))).map_err(|error| error.to_string())?;
    stream.set_write_timeout(Some(Duration::from_secs(config.timeout_secs))).map_err(|error| error.to_string())?;
    let (mut socket, _) = tungstenite::client_tls_with_config(request, stream, None, None)
        .map_err(|error| error.to_string())?;
    let stream = match socket.get_mut() {
        MaybeTlsStream::Plain(stream) => stream,
        MaybeTlsStream::Rustls(stream) => &mut stream.sock,
        _ => return Err("unsupported Gateway TLS transport".to_string()),
    };
    stream.set_read_timeout(Some(Duration::from_millis(200))).map_err(|error| error.to_string())?;
    Ok(socket)
}

fn connected_loop(
    socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
    config: &NodeGatewayConfig,
    state: &SharedControlRoom,
) -> Result<(), String> {
    let mut last_received = Instant::now();
    let mut last_ping = Instant::now();
    let ping_interval = Duration::from_secs((config.stale_after_secs / 3).max(1));
    loop {
        state.expire_gateway_nodes(config.stale_after_secs);
        if last_received.elapsed() >= Duration::from_secs(config.stale_after_secs) {
            return Err("Gateway telemetry timed out".to_string());
        }
        if last_ping.elapsed() >= ping_interval {
            // Application ping reaches the Gateway worker; there is deliberately
            // no BackendRequest::Command (or TBS ping/reconnect) in this module.
            socket.send(Message::Text(r#"{"kind":"ping","request_id":"control-room-telemetry"}"#.into()))
                .map_err(|error| error.to_string())?;
            last_ping = Instant::now();
        }
        match socket.read() {
            Ok(Message::Text(text)) => {
                apply_event(state, serde_json::from_str(&text).map_err(|error| error.to_string())?, config.stale_after_secs);
                last_received = Instant::now();
            }
            Ok(Message::Binary(bytes)) => {
                apply_event(state, serde_json::from_slice(&bytes).map_err(|error| error.to_string())?, config.stale_after_secs);
                last_received = Instant::now();
            }
            Ok(Message::Ping(payload)) => {
                socket.send(Message::Pong(payload)).map_err(|error| error.to_string())?;
                last_received = Instant::now();
            }
            Ok(Message::Pong(_)) => last_received = Instant::now(),
            Ok(Message::Close(_)) => return Err("Gateway closed connection".to_string()),
            Ok(_) => {},
            Err(tungstenite::Error::Io(ref error))
                if matches!(error.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut) => {},
            Err(error) => return Err(error.to_string()),
        }
    }
}

fn apply_event(state: &SharedControlRoom, event: GatewayEvent, max_age_secs: u64) {
    match event {
        GatewayEvent::Snapshot { snapshot } => state.apply_gateway_snapshot(&snapshot.nodes, max_age_secs),
        GatewayEvent::NodeMessage { node_id, message } => state.handle_gateway_message(&node_id, message, max_age_secs),
        GatewayEvent::Other => {},
    }
}

pub(crate) fn message_node_id(message: &NodeToControlRoomMessage) -> &str {
    match message {
        NodeToControlRoomMessage::Hello { hello } => &hello.node.node_id,
        NodeToControlRoomMessage::Heartbeat { heartbeat } => &heartbeat.node_id,
        NodeToControlRoomMessage::Telemetry { envelope } => &envelope.node_id,
        NodeToControlRoomMessage::ControlAck { ack } => &ack.node_id,
        NodeToControlRoomMessage::ControlResponse { envelope } => &envelope.node_id,
        NodeToControlRoomMessage::MediaFrame { frame } => &frame.node_id,
        NodeToControlRoomMessage::Error { node_id, .. } => node_id,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};
    use crate::state::now_iso;

    fn node(at: &str) -> Value {
        json!({
            "node_id":"tbs-one", "session_id":"session-one", "connected":true, "stale":false,
            "last_seen":at, "connected_at":at,
            "identity":{"node_id":"tbs-one","station_name":"Test TBS","site":"lab","stack_version":"test",
                "mcc":262,"mnc":1,"location_area":1,"main_carrier":1,"secondary_carrier":null,"colour_code":1,"system_code":1},
            "capabilities":{"telemetry":true,"command":true,"sds":true,"raw_sds":true,"dgna":true,"kick_ms":true,
                "emergency_clear":true,"live_sds":true,"service_control":true,"brew_bridge":false,"dual_carrier":false}
        })
    }

    fn snapshot(state: &SharedControlRoom, nodes: Vec<Value>) {
        apply_event(state, serde_json::from_value(json!({"kind":"snapshot","snapshot":{"nodes":nodes}})).unwrap(), 30);
    }

    fn telemetry(state: &SharedControlRoom, timestamp: &str, event: Value) {
        apply_event(state, serde_json::from_value(json!({"kind":"node_message","node_id":"tbs-one",
            "message":{"kind":"telemetry","envelope":{"node_id":"tbs-one","seq":1,"timestamp":timestamp,"event":event}}})).unwrap(), 30);
    }

    fn gps(state: &SharedControlRoom, at: &str) {
        telemetry(state, at, json!({"SdsLog":{"direction":"rx","source_issi":1234,"dest_issi":9999,
            "is_group":false,"protocol_id":10,"text":"LIP position: 52.500000, 13.400000"}}));
    }

    #[test]
    fn gateway_snapshot_registration_and_gps_keep_original_timestamps() {
        let state = SharedControlRoom::new_with_persistence(20, None);
        let at = now_iso();
        snapshot(&state, vec![node(&at)]);
        telemetry(&state, &at, json!({"MsRegistration":{"issi":1234}}));
        gps(&state, &at);
        let nodes = state.snapshot().nodes;
        assert!(nodes[0].connected && nodes[0].transport_connected);
        assert_eq!(nodes[0].last_seen.as_deref(), Some(at.as_str()));
        let subscribers = state.subscribers_snapshot(None, true).unwrap().subscribers;
        assert_eq!(subscribers.len(), 1);
        let location = subscribers[0].last_location.as_ref().unwrap();
        assert_eq!(location.latitude, 52.5);
        assert_eq!(location.longitude, 13.4);
        assert_eq!(location.updated_at, at);
    }

    #[test]
    fn disconnect_reconnect_retains_gps_but_requires_new_presence() {
        let state = SharedControlRoom::new_with_persistence(20, None);
        let at = now_iso();
        snapshot(&state, vec![node(&at)]);
        gps(&state, &at);
        state.gateway_disconnected();
        assert!(!state.snapshot().nodes[0].connected);
        assert_eq!(state.subscribers_snapshot(None, true).unwrap().count, 0);
        snapshot(&state, vec![node(&now_iso())]);
        assert_eq!(state.subscribers_snapshot(None, true).unwrap().count, 0);
        telemetry(&state, &now_iso(), json!({"MsRegistration":{"issi":1234}}));
        let subscribers = state.subscribers_snapshot(None, true).unwrap().subscribers;
        assert_eq!(subscribers.len(), 1);
        assert_eq!(subscribers[0].last_location.as_ref().unwrap().updated_at, at);
    }

    #[test]
    fn disconnected_stale_missing_or_changed_session_clears_presence() {
        for mode in ["disconnected", "stale", "missing", "session"] {
            let state = SharedControlRoom::new_with_persistence(20, None);
            let at = now_iso();
            snapshot(&state, vec![node(&at)]);
            gps(&state, &at);
            let mut next = node(&at);
            match mode {
                "disconnected" => next["connected"] = json!(false),
                "stale" => next["stale"] = json!(true),
                "session" => next["session_id"] = json!("new-session"),
                _ => {},
            }
            snapshot(&state, if mode == "missing" { vec![] } else { vec![next] });
            assert_eq!(state.subscribers_snapshot(None, true).unwrap().count, 0, "{mode}");
        }
    }

    #[test]
    fn old_snapshot_cannot_make_node_fresh_and_gateway_cannot_override_direct_tbs() {
        let state = SharedControlRoom::new_with_persistence(20, None);
        snapshot(&state, vec![node("2000-01-01T00:00:00Z")]);
        assert!(!state.snapshot().nodes[0].connected);
        let (sender, _receiver) = std::sync::mpsc::channel();
        state.register_node_sender("tbs-one".to_string(), sender);
        let before = state.snapshot().nodes[0].last_seen.clone();
        snapshot(&state, vec![]);
        state.gateway_disconnected();
        assert!(state.snapshot().nodes[0].connected);
        assert_eq!(state.snapshot().nodes[0].last_seen, before);
    }

    #[test]
    fn gateway_config_is_opt_in_and_wrapper_id_must_match() {
        let config: crate::config::ControlRoomConfig = toml::from_str("").unwrap();
        assert!(!config.node_gateway.enabled);
        let state = SharedControlRoom::new_with_persistence(20, None);
        snapshot(&state, vec![node(&now_iso())]);
        let message = serde_json::from_value(json!({"kind":"telemetry","envelope":{
            "node_id":"other-tbs","seq":1,"timestamp":now_iso(),"event":{"MsRegistration":{"issi":1234}}}})).unwrap();
        state.handle_gateway_message("tbs-one", message, 30);
        assert_eq!(state.snapshot().nodes.len(), 1);
        assert_eq!(state.subscribers_snapshot(None, true).unwrap().count, 0);
    }

    #[test]
    fn replayed_old_or_future_events_do_not_resurrect_presence() {
        let state = SharedControlRoom::new_with_persistence(20, None);
        snapshot(&state, vec![node(&now_iso())]);
        for old in ["2000-01-01T00:00:00Z", "2099-01-01T00:00:00Z", "invalid"] {
            telemetry(&state, old, json!({"MsRegistration":{"issi":1234}}));
            gps(&state, old);
        }
        assert_eq!(state.subscribers_snapshot(None, true).unwrap().count, 0);
        assert_eq!(state.locations_snapshot(None).unwrap().count, 0);
    }

    #[test]
    fn recent_replay_cannot_undo_newer_deregistration_or_gps() {
        let state = SharedControlRoom::new_with_persistence(20, None);
        let recent = now_iso();
        let older = (chrono::Utc::now() - chrono::Duration::seconds(10)).to_rfc3339();
        snapshot(&state, vec![node(&recent)]);
        gps(&state, &recent);
        telemetry(&state, &now_iso(), json!({"MsDeregistration":{"issi":1234}}));
        telemetry(&state, &older, json!({"MsRegistration":{"issi":1234}}));
        gps(&state, &older);
        assert_eq!(state.subscribers_snapshot(None, true).unwrap().count, 0);
        assert_eq!(state.locations_snapshot(None).unwrap().locations[0].updated_at, recent);
    }

    #[test]
    fn individually_silent_node_expires_even_when_upstream_remains_live() {
        let state = SharedControlRoom::new_with_persistence(20, None);
        let old = (chrono::Utc::now() - chrono::Duration::seconds(20)).to_rfc3339();
        snapshot(&state, vec![node(&old)]);
        assert!(state.snapshot().nodes[0].connected);
        state.expire_gateway_nodes(5);
        assert!(!state.snapshot().nodes[0].connected);
    }
}
