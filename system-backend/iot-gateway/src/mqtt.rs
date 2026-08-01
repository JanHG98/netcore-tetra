use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::json;

use crate::config::{IotGatewayConfig, MqttConfig};
use crate::model::OutboundMessage;
use crate::state::SharedGateway;

struct PublishRequest {
    message: OutboundMessage,
    response: mpsc::Sender<Result<(), String>>,
}

#[derive(Clone)]
pub struct MqttPublisher {
    sender: Arc<Mutex<Option<mpsc::Sender<PublishRequest>>>>,
    timeout: Duration,
}

impl MqttPublisher {
    fn new(timeout: Duration) -> Self {
        Self {
            sender: Arc::new(Mutex::new(None)),
            timeout,
        }
    }

    pub fn publish(&self, message: OutboundMessage) -> Result<(), String> {
        let sender = self
            .sender
            .lock()
            .map_err(|_| "MQTT publisher state poisoned".to_string())?
            .clone()
            .ok_or_else(|| "MQTT broker is not connected".to_string())?;
        let (response_tx, response_rx) = mpsc::channel();
        sender
            .send(PublishRequest {
                message,
                response: response_tx,
            })
            .map_err(|_| "MQTT connection worker is unavailable".to_string())?;
        response_rx
            .recv_timeout(self.timeout)
            .map_err(|_| "MQTT publish acknowledgement timed out".to_string())?
    }

    fn install_sender(&self, sender: mpsc::Sender<PublishRequest>) {
        if let Ok(mut slot) = self.sender.lock() {
            *slot = Some(sender);
        }
    }

    fn clear_sender(&self) {
        if let Ok(mut slot) = self.sender.lock() {
            *slot = None;
        }
    }
}

#[derive(Clone)]
pub struct MqttControl {
    force_reconnect: Arc<AtomicBool>,
}

impl MqttControl {
    pub fn reconnect(&self) {
        self.force_reconnect.store(true, Ordering::SeqCst);
    }
}

pub fn spawn_mqtt(
    config: IotGatewayConfig,
    state: SharedGateway,
) -> (MqttPublisher, MqttControl, Vec<thread::JoinHandle<()>>) {
    let publisher = MqttPublisher::new(Duration::from_secs(config.mqtt.publish_timeout_secs));
    let control = MqttControl {
        force_reconnect: Arc::new(AtomicBool::new(false)),
    };

    let connection_publisher = publisher.clone();
    let connection_control = control.clone();
    let connection_config = config.clone();
    let connection_state = state.clone();
    let connection = thread::spawn(move || {
        run_connection_loop(
            connection_config,
            connection_state,
            connection_publisher,
            connection_control,
        );
    });

    let outbox_publisher = publisher.clone();
    let outbox_state = state.clone();
    let outbox = thread::spawn(move || run_outbox_loop(outbox_state, outbox_publisher));

    (publisher, control, vec![connection, outbox])
}

fn run_connection_loop(
    config: IotGatewayConfig,
    state: SharedGateway,
    publisher: MqttPublisher,
    control: MqttControl,
) {
    loop {
        control.force_reconnect.store(false, Ordering::SeqCst);
        let result = run_one_connection(
            &config.mqtt,
            config.commands.enabled,
            &config.commands.mode,
            &state,
            &publisher,
            &control,
        );
        publisher.clear_sender();
        state.mark_mqtt_disconnected(result.err().unwrap_or_else(|| "connection closed".to_string()));
        thread::sleep(Duration::from_secs(config.mqtt.reconnect_secs.max(1)));
    }
}

fn run_one_connection(
    config: &MqttConfig,
    command_execution_enabled: bool,
    command_execution_mode: &str,
    state: &SharedGateway,
    publisher: &MqttPublisher,
    control: &MqttControl,
) -> Result<(), String> {
    let address = (config.host.as_str(), config.port)
        .to_socket_addrs()
        .map_err(|error| format!("MQTT DNS lookup failed: {error}"))?
        .next()
        .ok_or_else(|| "MQTT broker address did not resolve".to_string())?;
    let mut stream = TcpStream::connect_timeout(&address, Duration::from_secs(5))
        .map_err(|error| format!("MQTT TCP connect failed: {error}"))?;
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .map_err(|error| error.to_string())?;
    stream
        .set_write_timeout(Some(Duration::from_secs(5)))
        .map_err(|error| error.to_string())?;
    stream.set_nodelay(true).map_err(|error| error.to_string())?;

    write_connect(&mut stream, config)?;
    let connack = read_packet(&mut stream).map_err(packet_error)?;
    validate_connack(&connack)?;
    stream
        .set_read_timeout(Some(Duration::from_millis(200)))
        .map_err(|error| error.to_string())?;

    let (request_tx, request_rx) = mpsc::channel();
    publisher.install_sender(request_tx);
    state.mark_mqtt_connected();
    tracing::info!(
        "MQTT connected to {}:{} as {}",
        config.host,
        config.port,
        config.client_id
    );

    let mut next_packet_id = 1_u16;
    publish_gateway_online(
        &mut stream,
        config,
        command_execution_enabled,
        command_execution_mode,
        &mut next_packet_id,
    )?;
    if config.observe_commands {
        let subscription = format!("{}/commands/#", config.topic_prefix.trim_matches('/'));
        write_subscribe(&mut stream, next_id(&mut next_packet_id), &subscription, config.qos)?;
        tracing::warn!(
            "OPEN LAB command processing subscribed to {}; policy-controlled sandbox execution={}",
            subscription, command_execution_enabled
        );
    }

    let mut pending: HashMap<u16, mpsc::Sender<Result<(), String>>> = HashMap::new();
    let mut last_io = Instant::now();
    loop {
        if control.force_reconnect.swap(false, Ordering::SeqCst) {
            return Err("manual reconnect requested".to_string());
        }

        while let Ok(request) = request_rx.try_recv() {
            if request.message.qos == 0 {
                write_publish(&mut stream, &request.message, None)?;
                let _ = request.response.send(Ok(()));
            } else {
                let packet_id = next_id(&mut next_packet_id);
                write_publish(&mut stream, &request.message, Some(packet_id))?;
                pending.insert(packet_id, request.response);
            }
            last_io = Instant::now();
        }

        match read_packet(&mut stream) {
            Ok(packet) => {
                last_io = Instant::now();
                handle_packet(&mut stream, packet, config, state, &mut pending)?;
            }
            Err(PacketReadError::Timeout) => {
                let ping_after = Duration::from_secs((u64::from(config.keep_alive_secs) / 2).max(5));
                if last_io.elapsed() >= ping_after {
                    write_packet(&mut stream, 0xC0, &[])?;
                    last_io = Instant::now();
                }
            }
            Err(PacketReadError::Io(error)) => {
                for (_, response) in pending.drain() {
                    let _ = response.send(Err(format!("MQTT connection lost: {error}")));
                }
                return Err(format!("MQTT read failed: {error}"));
            }
            Err(PacketReadError::Protocol(error)) => {
                for (_, response) in pending.drain() {
                    let _ = response.send(Err(format!("MQTT protocol error: {error}")));
                }
                return Err(error);
            }
        }
    }
}

fn handle_packet(
    stream: &mut TcpStream,
    packet: Packet,
    config: &MqttConfig,
    state: &SharedGateway,
    pending: &mut HashMap<u16, mpsc::Sender<Result<(), String>>>,
) -> Result<(), String> {
    match packet.packet_type() {
        3 => {
            let incoming = parse_publish(&packet)?;
            state.record_mqtt_message();
            if incoming.qos == 1 {
                if let Some(packet_id) = incoming.packet_id {
                    write_packet(stream, 0x40, &packet_id.to_be_bytes())?;
                }
            }
            let command_prefix = format!("{}/commands/", config.topic_prefix.trim_matches('/'));
            if config.observe_commands && incoming.topic.starts_with(&command_prefix) {
                match state.process_command(
                    incoming.topic,
                    incoming.payload,
                    incoming.qos,
                    incoming.retain,
                ) {
                    Ok(record) => tracing::info!(
                        "MQTT command {} completed with status {:?}",
                        record.command_id,
                        record.status
                    ),
                    Err(error) => tracing::warn!("failed to process MQTT command: {}", error),
                }
            }
        }
        4 => {
            if packet.body.len() != 2 {
                return Err("invalid MQTT PUBACK length".to_string());
            }
            let packet_id = u16::from_be_bytes([packet.body[0], packet.body[1]]);
            if let Some(response) = pending.remove(&packet_id) {
                let _ = response.send(Ok(()));
            }
        }
        9 | 13 => {}
        _ => {}
    }
    Ok(())
}

fn run_outbox_loop(state: SharedGateway, publisher: MqttPublisher) {
    loop {
        if !state.mqtt_connected() {
            thread::sleep(Duration::from_millis(500));
            continue;
        }
        let Some((path, message)) = state.next_outbox_message() else {
            thread::sleep(Duration::from_millis(250));
            continue;
        };
        match publisher.publish(message.clone()) {
            Ok(()) => state.complete_outbox_message(&path, &message),
            Err(error) => {
                state.record_bridge_event(
                    "mqtt.publish_deferred",
                    json!({"message_id":message.id,"topic":message.topic,"error":error}),
                );
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}

fn publish_gateway_online(
    stream: &mut TcpStream,
    config: &MqttConfig,
    command_execution_enabled: bool,
    command_execution_mode: &str,
    next_packet_id: &mut u16,
) -> Result<(), String> {
    let prefix = config.topic_prefix.trim_matches('/');
    let message = OutboundMessage {
        id: uuid::Uuid::new_v4(),
        created_at: chrono::Utc::now().to_rfc3339(),
        kind: "service_state".to_string(),
        topic: format!("{prefix}/state/services/iot-gateway"),
        qos: config.qos,
        retain: true,
        payload: json!({
            "status":"online",
            "service":"netcore-iot-gateway",
            "security_mode":"open_lab",
            "command_execution":command_execution_enabled,
            "command_execution_mode":command_execution_mode,
            "phase":4,
            "timestamp":chrono::Utc::now().to_rfc3339()
        })
        .to_string(),
        source_event_id: None,
    };
    let packet_id = (message.qos == 1).then(|| next_id(next_packet_id));
    write_publish(stream, &message, packet_id)
}

fn write_connect(stream: &mut TcpStream, config: &MqttConfig) -> Result<(), String> {
    let prefix = config.topic_prefix.trim_matches('/');
    let will_topic = format!("{prefix}/state/services/iot-gateway");
    let will_payload = json!({
        "status":"offline",
        "service":"netcore-iot-gateway",
        "reason":"mqtt_last_will",
        "security_mode":"open_lab"
    })
    .to_string();

    let mut body = Vec::new();
    push_utf8(&mut body, "MQTT")?;
    body.push(4);
    let mut flags = 0_u8;
    if config.clean_session {
        flags |= 0b0000_0010;
    }
    flags |= 0b0000_0100;
    flags |= (config.qos.min(1)) << 3;
    flags |= 0b0010_0000;
    body.push(flags);
    body.extend_from_slice(&config.keep_alive_secs.to_be_bytes());
    push_utf8(&mut body, &config.client_id)?;
    push_utf8(&mut body, &will_topic)?;
    push_binary(&mut body, will_payload.as_bytes())?;
    write_packet(stream, 0x10, &body)
}

fn validate_connack(packet: &Packet) -> Result<(), String> {
    if packet.packet_type() != 2 || packet.body.len() != 2 {
        return Err("broker did not return a valid MQTT CONNACK".to_string());
    }
    if packet.body[1] != 0 {
        return Err(format!("MQTT broker rejected connection with code {}", packet.body[1]));
    }
    Ok(())
}

fn write_subscribe(
    stream: &mut TcpStream,
    packet_id: u16,
    topic: &str,
    qos: u8,
) -> Result<(), String> {
    let mut body = Vec::new();
    body.extend_from_slice(&packet_id.to_be_bytes());
    push_utf8(&mut body, topic)?;
    body.push(qos.min(1));
    write_packet(stream, 0x82, &body)
}

fn write_publish(
    stream: &mut TcpStream,
    message: &OutboundMessage,
    packet_id: Option<u16>,
) -> Result<(), String> {
    let qos = message.qos.min(1);
    let mut header = 0x30_u8;
    header |= qos << 1;
    if message.retain {
        header |= 0x01;
    }
    let mut body = Vec::new();
    push_utf8(&mut body, &message.topic)?;
    if qos == 1 {
        body.extend_from_slice(
            &packet_id
                .ok_or_else(|| "QoS 1 publish requires a packet identifier".to_string())?
                .to_be_bytes(),
        );
    }
    body.extend_from_slice(message.payload.as_bytes());
    write_packet(stream, header, &body)
}

fn write_packet(stream: &mut TcpStream, header: u8, body: &[u8]) -> Result<(), String> {
    let mut packet = Vec::with_capacity(body.len() + 5);
    packet.push(header);
    encode_remaining_length(body.len(), &mut packet)?;
    packet.extend_from_slice(body);
    stream
        .write_all(&packet)
        .and_then(|_| stream.flush())
        .map_err(|error| format!("MQTT write failed: {error}"))
}

fn encode_remaining_length(mut length: usize, output: &mut Vec<u8>) -> Result<(), String> {
    if length > 268_435_455 {
        return Err("MQTT packet is too large".to_string());
    }
    loop {
        let mut encoded = (length % 128) as u8;
        length /= 128;
        if length > 0 {
            encoded |= 0x80;
        }
        output.push(encoded);
        if length == 0 {
            return Ok(());
        }
    }
}

fn push_utf8(output: &mut Vec<u8>, value: &str) -> Result<(), String> {
    push_binary(output, value.as_bytes())
}

fn push_binary(output: &mut Vec<u8>, value: &[u8]) -> Result<(), String> {
    let length = u16::try_from(value.len()).map_err(|_| "MQTT string exceeds 65535 bytes".to_string())?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(value);
    Ok(())
}

fn next_id(value: &mut u16) -> u16 {
    let current = if *value == 0 { 1 } else { *value };
    *value = current.wrapping_add(1);
    if *value == 0 {
        *value = 1;
    }
    current
}

struct Packet {
    header: u8,
    body: Vec<u8>,
}

impl Packet {
    fn packet_type(&self) -> u8 {
        self.header >> 4
    }
}

enum PacketReadError {
    Timeout,
    Io(io::Error),
    Protocol(String),
}

fn packet_error(error: PacketReadError) -> String {
    match error {
        PacketReadError::Timeout => "MQTT packet timed out".to_string(),
        PacketReadError::Io(error) => error.to_string(),
        PacketReadError::Protocol(error) => error,
    }
}

fn read_packet(stream: &mut TcpStream) -> Result<Packet, PacketReadError> {
    let mut first = [0_u8; 1];
    match stream.read_exact(&mut first) {
        Ok(()) => {}
        Err(error) if matches!(error.kind(), io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut) => {
            return Err(PacketReadError::Timeout)
        }
        Err(error) => return Err(PacketReadError::Io(error)),
    }

    let mut multiplier = 1_usize;
    let mut remaining = 0_usize;
    for index in 0..4 {
        let mut encoded = [0_u8; 1];
        stream.read_exact(&mut encoded).map_err(PacketReadError::Io)?;
        remaining = remaining
            .checked_add(usize::from(encoded[0] & 0x7f) * multiplier)
            .ok_or_else(|| PacketReadError::Protocol("MQTT remaining length overflow".to_string()))?;
        if encoded[0] & 0x80 == 0 {
            let mut body = vec![0_u8; remaining];
            stream.read_exact(&mut body).map_err(PacketReadError::Io)?;
            return Ok(Packet {
                header: first[0],
                body,
            });
        }
        if index == 3 {
            return Err(PacketReadError::Protocol(
                "MQTT remaining length uses more than four bytes".to_string(),
            ));
        }
        multiplier *= 128;
    }
    Err(PacketReadError::Protocol(
        "invalid MQTT remaining length".to_string(),
    ))
}

struct IncomingPublish {
    topic: String,
    payload: Vec<u8>,
    qos: u8,
    retain: bool,
    packet_id: Option<u16>,
}

fn parse_publish(packet: &Packet) -> Result<IncomingPublish, String> {
    if packet.body.len() < 2 {
        return Err("MQTT PUBLISH has no topic length".to_string());
    }
    let topic_length = usize::from(u16::from_be_bytes([packet.body[0], packet.body[1]]));
    if packet.body.len() < 2 + topic_length {
        return Err("MQTT PUBLISH topic is truncated".to_string());
    }
    let topic = std::str::from_utf8(&packet.body[2..2 + topic_length])
        .map_err(|_| "MQTT PUBLISH topic is not UTF-8".to_string())?
        .to_string();
    let qos = (packet.header >> 1) & 0x03;
    if qos > 1 {
        return Err("Phase 4 does not support inbound MQTT QoS 2".to_string());
    }
    let mut offset = 2 + topic_length;
    let packet_id = if qos == 1 {
        if packet.body.len() < offset + 2 {
            return Err("MQTT QoS 1 PUBLISH lacks packet identifier".to_string());
        }
        let value = u16::from_be_bytes([packet.body[offset], packet.body[offset + 1]]);
        offset += 2;
        Some(value)
    } else {
        None
    };
    Ok(IncomingPublish {
        topic,
        payload: packet.body[offset..].to_vec(),
        qos,
        retain: packet.header & 0x01 != 0,
        packet_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remaining_length_encoding_matches_mqtt() {
        let mut output = Vec::new();
        encode_remaining_length(321, &mut output).unwrap();
        assert_eq!(output, vec![0xC1, 0x02]);
    }

    #[test]
    fn packet_ids_never_use_zero() {
        let mut value = u16::MAX;
        assert_eq!(next_id(&mut value), u16::MAX);
        assert_eq!(value, 1);
        assert_eq!(next_id(&mut value), 1);
    }

    #[test]
    fn inbound_publish_preserves_retain_and_qos() {
        let topic = "netcore/v1/commands/test";
        let mut body = Vec::new();
        push_utf8(&mut body, topic).unwrap();
        body.extend_from_slice(&7_u16.to_be_bytes());
        body.extend_from_slice(br#"{"command_id":"demo"}"#);
        let packet = Packet {
            header: 0x33, // PUBLISH, QoS 1, RETAIN
            body,
        };
        let incoming = parse_publish(&packet).unwrap();
        assert_eq!(incoming.topic, topic);
        assert_eq!(incoming.qos, 1);
        assert!(incoming.retain);
        assert_eq!(incoming.packet_id, Some(7));
    }
}
