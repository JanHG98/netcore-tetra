//! Bidirectional base-station worker for the NetCore Control Room.
//!
//! One WebSocket carries node state, telemetry, control commands and media.
//! The worker also owns the edge-autonomy state machine: loss of the gateway
//! or one required backend service never stops local RF operation.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use tetra_config::bluestation::{
    EdgeFallbackMode, EdgeServiceLevel, EdgeServiceRuntime, SharedConfig,
};
use tetra_core::tetra_entities::TetraEntity;

use crate::{
    net_control::{CommandDispatcher, ControlCommand, ControlResponse},
    net_control_room::{
        edge_store::{EdgeEventSpool, update_spool_stats},
        CONTROL_ROOM_HEARTBEAT_INTERVAL, CONTROL_ROOM_PROTOCOL_VERSION,
        ControlCommandAck, ControlCommandEnvelope, ControlResponseEnvelope,
        ControlRoomCodecJson, ControlRoomNodeCapabilities, ControlRoomNodeHeartbeat,
        ControlRoomNodeHello, ControlRoomNodeIdentity, ControlRoomToNodeMessage,
        CoreServiceHealthLevel, CoreServicesSnapshot, NodeTelemetryEnvelope,
        NodeToControlRoomMessage,
    },
    net_media::{MediaDownlinkSink, MediaTryRecvError, MediaUplinkFrame, MediaUplinkSource},
    net_telemetry::{TelemetryEvent, TelemetrySource, channel::RecvEvent},
    network::transports::NetworkTransport,
};

const POLL_TIMEOUT: Duration = Duration::from_millis(10);
const RECONNECT_DELAY: Duration = Duration::from_secs(10);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CommandCorrelationKey {
    Handle(u32),
    KickMs(u32),
    Dgna { issi: u32, gssi: u32, attach: bool },
    Mobility(u32),
    SubscriberPolicy(u32),
    GroupPolicy(u32),
    GroupDgna(u32),
    CallControl(u32),
    RestartService,
    ShutdownService,
    LiveSdsAdd { source_issi: u32, protocol_id: u8, text: String },
    LiveSdsDelete(u32),
    LiveSdsClear,
    ClearEmergency(u32),
}

pub struct ControlRoomWorker<T: NetworkTransport> {
    identity: ControlRoomNodeIdentity,
    capabilities: ControlRoomNodeCapabilities,
    config: SharedConfig,
    telemetry_source: TelemetrySource,
    media_uplink_source: Option<MediaUplinkSource>,
    media_downlink_sink: Option<MediaDownlinkSink>,
    dispatchers: HashMap<TetraEntity, CommandDispatcher>,
    transport: T,
    connected: bool,
    last_connect_attempt: Option<Instant>,
    last_heartbeat_at: Instant,
    started_at: String,
    seq: u64,
    pending_commands: HashMap<CommandCorrelationKey, (String, TetraEntity)>,
    event_spool: EdgeEventSpool,
    unhealthy_since: Option<Instant>,
    healthy_since: Option<Instant>,
    last_service_snapshot_at: Option<Instant>,
}

impl<T: NetworkTransport> ControlRoomWorker<T> {
    pub fn new(
        identity: ControlRoomNodeIdentity,
        capabilities: ControlRoomNodeCapabilities,
        config: SharedConfig,
        telemetry_source: TelemetrySource,
        media_uplink_source: Option<MediaUplinkSource>,
        media_downlink_sink: Option<MediaDownlinkSink>,
        dispatchers: HashMap<TetraEntity, CommandDispatcher>,
        transport: T,
    ) -> Self {
        let now = Instant::now();
        let event_spool = EdgeEventSpool::from_config(config.config().as_ref());
        update_spool_stats(&config, &event_spool);
        Self {
            identity,
            capabilities,
            config,
            telemetry_source,
            media_uplink_source,
            media_downlink_sink,
            dispatchers,
            transport,
            connected: false,
            last_connect_attempt: None,
            last_heartbeat_at: now,
            started_at: now_iso(),
            seq: 0,
            pending_commands: HashMap::new(),
            event_spool,
            unhealthy_since: Some(now),
            healthy_since: None,
            last_service_snapshot_at: None,
        }
    }

    pub fn run(&mut self) {
        tracing::debug!("ControlRoom worker started for node_id={}", self.identity.node_id);
        self.try_connect();

        loop {
            match self.telemetry_source.recv_timeout(POLL_TIMEOUT) {
                RecvEvent::Event(event) => self.forward_telemetry(event),
                RecvEvent::Timeout => {}
                RecvEvent::Closed => {
                    tracing::debug!("ControlRoom worker: telemetry source closed, shutting down");
                    break;
                }
            }

            if self.connected {
                self.drain_media_uplink();
                self.poll_downlink();
                self.collect_responses();
                self.send_periodic_heartbeat();
                self.replay_spool();
            } else {
                std::thread::sleep(POLL_TIMEOUT);
            }

            if !self.transport.is_connected() && self.connected {
                tracing::warn!("ControlRoom transport disconnected");
                self.transport.disconnect();
                self.connected = false;
                self.mark_gateway_connected(false, "Node Gateway transport disconnected");
            }

            self.tick_edge_fallback();

            if !self.connected && self.reconnect_due() {
                self.try_connect();
            }
        }

        self.transport.disconnect();
        self.mark_gateway_connected(false, "Control Room worker stopped");
        tracing::info!("ControlRoom worker exiting");
    }

    fn reconnect_due(&self) -> bool {
        self.last_connect_attempt
            .map(|last| last.elapsed() >= RECONNECT_DELAY)
            .unwrap_or(true)
    }

    fn try_connect(&mut self) {
        self.last_connect_attempt = Some(Instant::now());
        self.transport.disconnect();
        match self.transport.connect() {
            Ok(()) => {
                tracing::info!("ControlRoom transport connected");
                self.connected = true;
                self.mark_gateway_connected(true, "Node Gateway connected; waiting for healthy service matrix");
                self.last_heartbeat_at = Instant::now() - CONTROL_ROOM_HEARTBEAT_INTERVAL;
                if self.send_hello() {
                    self.send_periodic_heartbeat();
                }
            }
            Err(error) => {
                tracing::warn!(
                    "ControlRoom transport connection failed: {}, will retry in {:?}",
                    error,
                    RECONNECT_DELAY
                );
                self.transport.disconnect();
                self.connected = false;
                self.mark_gateway_connected(false, &format!("Node Gateway connection failed: {error}"));
            }
        }
    }

    fn send_hello(&mut self) -> bool {
        let hello = ControlRoomNodeHello {
            protocol_version: CONTROL_ROOM_PROTOCOL_VERSION.to_string(),
            node: self.identity.clone(),
            capabilities: self.capabilities.clone(),
            started_at: self.started_at.clone(),
        };
        self.send_uplink(&NodeToControlRoomMessage::Hello { hello })
    }

    fn send_periodic_heartbeat(&mut self) {
        if !self.connected || self.last_heartbeat_at.elapsed() < CONTROL_ROOM_HEARTBEAT_INTERVAL {
            return;
        }
        self.seq = self.seq.wrapping_add(1);
        let heartbeat = ControlRoomNodeHeartbeat {
            node_id: self.identity.node_id.clone(),
            seq: self.seq,
            timestamp: now_iso(),
            connected: true,
        };
        if self.send_uplink(&NodeToControlRoomMessage::Heartbeat { heartbeat }) {
            self.last_heartbeat_at = Instant::now();
        }
    }

    fn forward_telemetry(&mut self, event: TelemetryEvent) {
        if !self.ensure_connected() {
            self.spool_event(event);
            return;
        }
        self.seq = self.seq.wrapping_add(1);
        let envelope = NodeTelemetryEnvelope {
            node_id: self.identity.node_id.clone(),
            seq: self.seq,
            timestamp: now_iso(),
            event: event.clone(),
        };
        if !self.send_uplink(&NodeToControlRoomMessage::Telemetry { envelope }) {
            self.spool_event(event);
        }
    }

    fn spool_event(&mut self, event: TelemetryEvent) {
        if let Err(error) = self.event_spool.append(event) {
            tracing::error!("failed to persist edge fallback event: {}", error);
        }
        update_spool_stats(&self.config, &self.event_spool);
    }

    fn replay_spool(&mut self) {
        if !self.connected || !self.config.edge_fallback_snapshot().gateway_connected {
            return;
        }
        let batch_size = self.config.config().edge_fallback.replay_batch_size;
        let records = match self.event_spool.peek_batch(batch_size) {
            Ok(records) => records,
            Err(error) => {
                tracing::warn!("failed to read edge fallback spool: {}", error);
                return;
            }
        };
        let mut acknowledged = None;
        for record in records {
            self.seq = self.seq.wrapping_add(1);
            let envelope = NodeTelemetryEnvelope {
                node_id: self.identity.node_id.clone(),
                seq: self.seq,
                timestamp: record.timestamp.clone(),
                event: record.event,
            };
            if !self.send_uplink(&NodeToControlRoomMessage::Telemetry { envelope }) {
                break;
            }
            acknowledged = Some(record.sequence);
        }
        if let Some(sequence) = acknowledged {
            if let Err(error) = self.event_spool.acknowledge_through(sequence) {
                tracing::warn!("failed to acknowledge edge fallback spool: {}", error);
            }
            update_spool_stats(&self.config, &self.event_spool);
        }
    }

    fn drain_media_uplink(&mut self) {
        let mut frames = Vec::new();
        if let Some(source) = self.media_uplink_source.as_ref() {
            for _ in 0..64 {
                match source.try_recv() {
                    Ok(frame) => frames.push(frame),
                    Err(MediaTryRecvError::Empty) => break,
                    Err(MediaTryRecvError::Disconnected) => {
                        tracing::warn!("ControlRoom media uplink queue disconnected");
                        break;
                    }
                }
            }
        }

        for frame in frames {
            let frame = MediaUplinkFrame {
                node_id: self.identity.node_id.clone(),
                sequence: frame.sequence,
                timestamp: now_iso(),
                carrier_num: frame.carrier_num,
                logical_ts: frame.logical_ts,
                codec: frame.codec,
                payload: frame.payload,
            };
            if !self.send_uplink(&NodeToControlRoomMessage::MediaFrame { frame }) {
                // Speech is real-time. Never fill storage with stale audio frames;
                // the local RF call continues and the local recorder remains active.
                break;
            }
        }
    }

    fn poll_downlink(&mut self) {
        for msg in self.transport.receive_reliable() {
            let codec = ControlRoomCodecJson;
            match codec.decode_downlink(&msg.payload) {
                Ok(ControlRoomToNodeMessage::HelloAck { accepted, message }) => {
                    if accepted {
                        tracing::info!(
                            "ControlRoom hello accepted: {}",
                            message.unwrap_or_else(|| "ok".to_string())
                        );
                    } else {
                        tracing::warn!(
                            "ControlRoom hello rejected: {}",
                            message.unwrap_or_else(|| "no reason".to_string())
                        );
                        self.transport.disconnect();
                        self.connected = false;
                        self.mark_gateway_connected(false, "Node Gateway rejected node hello");
                        break;
                    }
                }
                Ok(ControlRoomToNodeMessage::Ping { seq, .. }) => {
                    tracing::trace!("ControlRoom ping seq={}", seq);
                    self.send_periodic_heartbeat();
                }
                Ok(ControlRoomToNodeMessage::CoreServices { snapshot }) => {
                    self.apply_core_services(snapshot);
                }
                Ok(ControlRoomToNodeMessage::Command { envelope }) => self.handle_command(envelope),
                Ok(ControlRoomToNodeMessage::MediaFrame { frame }) => {
                    if !self.config.central_service_available("media-switch") {
                        tracing::warn!(
                            session_id = %frame.session_id,
                            "ignoring central media frame while media-switch is unavailable"
                        );
                        continue;
                    }
                    let Some(sink) = self.media_downlink_sink.as_ref() else {
                        tracing::warn!(
                            session_id = %frame.session_id,
                            logical_ts = frame.logical_ts,
                            "received Media Switch frame but no local media bridge is installed"
                        );
                        continue;
                    };
                    if let Err(error) = sink.try_send(frame) {
                        tracing::warn!("dropping Media Switch downlink frame: {:?}", error);
                    }
                }
                Err(error) => {
                    tracing::warn!(
                        "ControlRoom: failed to decode downlink message ({} bytes): {}",
                        msg.payload.len(),
                        error
                    );
                    self.send_error(format!("failed to decode downlink message: {error}"));
                }
            }
        }
    }

    fn apply_core_services(&mut self, snapshot: CoreServicesSnapshot) {
        let mut state = self.config.state_write();
        if snapshot.revision < state.edge_service_revision {
            tracing::warn!(
                received = snapshot.revision,
                current = state.edge_service_revision,
                "ignoring stale core-service matrix"
            );
            return;
        }
        state.edge_service_revision = snapshot.revision;
        state.edge_service_matrix_fresh = true;
        state.edge_service_matrix_received_at = Some(now_iso());
        state.edge_services.clear();
        for service in snapshot.services {
            let level = match service.level {
                CoreServiceHealthLevel::Unknown => EdgeServiceLevel::Unknown,
                CoreServiceHealthLevel::Available => EdgeServiceLevel::Available,
                CoreServiceHealthLevel::Degraded => EdgeServiceLevel::Degraded,
                CoreServiceHealthLevel::Unavailable => EdgeServiceLevel::Unavailable,
            };
            state.edge_services.insert(
                service.service.clone(),
                EdgeServiceRuntime {
                    service: service.service,
                    level,
                    critical_for_edge: service.critical_for_edge,
                    fallback_mode: service.fallback_mode,
                    checked_at: service.checked_at,
                    last_success_at: service.last_success_at,
                    message: service.message,
                },
            );
        }
        drop(state);
        // Equal revisions are deliberate lease renewals carrying a complete
        // current matrix. Older revisions above do not renew the lease.
        self.last_service_snapshot_at = Some(Instant::now());
        self.tick_edge_fallback();
    }

    fn handle_command(&mut self, envelope: ControlCommandEnvelope) {
        if envelope.target_node_id != self.identity.node_id && envelope.target_node_id != "*" {
            self.send_ack(
                envelope.command_id,
                false,
                None,
                format!(
                    "command target_node_id={} does not match this node_id={}",
                    envelope.target_node_id, self.identity.node_id
                ),
            );
            return;
        }

        let target = route_control_command(&envelope.command);
        let Some(dispatcher) = self.dispatchers.get(&target) else {
            self.send_ack(
                envelope.command_id,
                false,
                Some(target),
                format!("no dispatcher registered for {:?}", target),
            );
            return;
        };

        if let Some(key) = correlation_key_for_command(&envelope.command) {
            self.pending_commands.insert(key, (envelope.command_id.clone(), target));
        }

        dispatcher.send(envelope.command);
        self.send_ack(envelope.command_id, true, Some(target), format!("dispatched to {:?}", target));
    }

    fn collect_responses(&mut self) {
        let mut outgoing: Vec<(ControlResponse, Option<String>, Option<TetraEntity>)> = Vec::new();
        for (entity, dispatcher) in &self.dispatchers {
            for response in dispatcher.try_recv_responses() {
                let key = correlation_key_for_response(&response);
                let correlated = key.and_then(|key| self.pending_commands.remove(&key));
                let (command_id, target_entity) = match correlated {
                    Some((id, entity)) => (Some(id), Some(entity)),
                    None => (None, Some(*entity)),
                };
                outgoing.push((response, command_id, target_entity));
            }
        }

        for (response, command_id, target_entity) in outgoing {
            let envelope = ControlResponseEnvelope {
                command_id,
                node_id: self.identity.node_id.clone(),
                target_entity,
                timestamp: now_iso(),
                response,
            };
            self.send_uplink(&NodeToControlRoomMessage::ControlResponse { envelope });
        }
    }

    fn send_ack(
        &mut self,
        command_id: String,
        accepted: bool,
        target_entity: Option<TetraEntity>,
        message: String,
    ) {
        let ack = ControlCommandAck {
            command_id,
            node_id: self.identity.node_id.clone(),
            accepted,
            target_entity,
            message,
            timestamp: now_iso(),
        };
        self.send_uplink(&NodeToControlRoomMessage::ControlAck { ack });
    }

    fn send_error(&mut self, message: String) {
        let msg = NodeToControlRoomMessage::Error {
            node_id: self.identity.node_id.clone(),
            message,
            timestamp: now_iso(),
        };
        self.send_uplink(&msg);
    }

    fn ensure_connected(&mut self) -> bool {
        if self.connected && self.transport.is_connected() {
            return true;
        }
        if self.connected {
            tracing::warn!("ControlRoom transport no longer connected");
            self.transport.disconnect();
            self.connected = false;
            self.mark_gateway_connected(false, "Node Gateway transport no longer connected");
        }
        if !self.reconnect_due() {
            return false;
        }
        self.try_connect();
        self.connected
    }

    fn send_uplink(&mut self, message: &NodeToControlRoomMessage) -> bool {
        if !self.connected {
            return false;
        }
        let codec = ControlRoomCodecJson;
        let payload = codec.encode_uplink(message);
        match self.transport.send_reliable(&payload) {
            Ok(()) => true,
            Err(error) => {
                tracing::warn!(
                    "ControlRoom transport send failed: {}, will retry in {:?}",
                    error,
                    RECONNECT_DELAY
                );
                self.transport.disconnect();
                self.connected = false;
                self.mark_gateway_connected(false, &format!("Node Gateway send failed: {error}"));
                false
            }
        }
    }

    fn mark_gateway_connected(&mut self, connected: bool, reason: &str) {
        {
            let mut state = self.config.state_write();
            state.core_gateway_connected = connected;
            if !connected {
                state.edge_service_matrix_fresh = false;
                state.edge_fallback_reason = reason.to_string();
            } else {
                // A newly connected socket is not authoritative until a full
                // service matrix arrives and renews the lease.
                state.edge_service_matrix_fresh = false;
            }
        }
        if connected {
            self.unhealthy_since = None;
            self.healthy_since = Some(Instant::now());
        } else {
            self.healthy_since = None;
            self.unhealthy_since.get_or_insert_with(Instant::now);
        }
        self.tick_edge_fallback();
    }

    fn tick_edge_fallback(&mut self) {
        let cfg = self.config.config();
        if !cfg.edge_fallback.enabled {
            self.set_edge_mode(EdgeFallbackMode::Online, "edge fallback disabled by configuration");
            return;
        }

        let (gateway_connected, unavailable) = {
            let state = self.config.state_read();
            let unavailable = cfg
                .edge_fallback
                .required_services
                .iter()
                .filter(|service| match state.edge_services.get(service.as_str()) {
                    Some(status) => !matches!(status.level, EdgeServiceLevel::Available),
                    None => !cfg.edge_fallback.unknown_service_is_available,
                })
                .cloned()
                .collect::<Vec<_>>();
            (state.core_gateway_connected, unavailable)
        };

        let matrix_stale = self
            .last_service_snapshot_at
            .is_none_or(|received| received.elapsed() > Duration::from_secs(cfg.edge_fallback.service_matrix_lease_secs));
        {
            let mut state = self.config.state_write();
            state.edge_service_matrix_fresh = gateway_connected && !matrix_stale;
        }

        let unhealthy_reason = if !gateway_connected {
            Some((
                "Node Gateway unreachable; local edge authority active".to_string(),
                true,
            ))
        } else if matrix_stale {
            Some((
                "Node Gateway health matrix missing or stale; conservative local edge authority active".to_string(),
                true,
            ))
        } else if !unavailable.is_empty() {
            Some((
                format!(
                    "required core service(s) unavailable: {}; service-specific fallbacks active",
                    unavailable.join(", ")
                ),
                false,
            ))
        } else {
            None
        };

        if let Some((reason, full_isolation)) = unhealthy_reason {
            self.healthy_since = None;
            let since = self.unhealthy_since.get_or_insert_with(Instant::now);
            if full_isolation && since.elapsed() >= Duration::from_secs(cfg.edge_fallback.enter_after_secs) {
                self.set_edge_mode(EdgeFallbackMode::Isolated, &reason);
            } else {
                // A single backend failure activates only its documented
                // service-specific fallback. Other central services remain in
                // use; this is degraded operation rather than total isolation.
                self.set_edge_mode(EdgeFallbackMode::Degraded, &reason);
            }
        } else {
            self.unhealthy_since = None;
            let since = self.healthy_since.get_or_insert_with(Instant::now);
            if since.elapsed() >= Duration::from_secs(cfg.edge_fallback.recover_after_secs) {
                self.set_edge_mode(EdgeFallbackMode::Online, "central service plane healthy");
            } else {
                self.set_edge_mode(
                    EdgeFallbackMode::Recovering,
                    "central service plane healthy; hysteresis/replay in progress",
                );
            }
        }
    }

    fn set_edge_mode(&self, mode: EdgeFallbackMode, reason: &str) {
        let mut state = self.config.state_write();
        if state.edge_fallback_mode != mode || state.edge_fallback_reason != reason {
            tracing::warn!(old = ?state.edge_fallback_mode, new = ?mode, reason, "edge fallback transition");
            state.edge_fallback_mode = mode;
            state.edge_fallback_reason = reason.to_string();
            state.edge_fallback_last_transition_at = now_iso();
        }
    }
}

pub fn route_control_command(command: &ControlCommand) -> TetraEntity {
    match command {
        ControlCommand::SendSds { .. } => TetraEntity::Cmce,
        ControlCommand::SendRawSdsType4 { .. }
        | ControlCommand::DeliverSds { .. }
        | ControlCommand::SendStatus { .. } => TetraEntity::Cmce,
        ControlCommand::KickMs { .. } => TetraEntity::Cmce,
        ControlCommand::Dgna { .. }
        | ControlCommand::MobilityExportContext { .. }
        | ControlCommand::MobilityImportContext { .. }
        | ControlCommand::MobilityRemoveContext { .. }
        | ControlCommand::SubscriberAccessPolicyApply { .. }
        | ControlCommand::GroupAccessPolicyApply { .. }
        | ControlCommand::GroupDgnaApply { .. } => TetraEntity::Mm,
        ControlCommand::CallControlGroupStart { .. }
        | ControlCommand::CallControlIndividualStart { .. }
        | ControlCommand::CallControlRelease { .. }
        | ControlCommand::CallControlFloorRequest { .. }
        | ControlCommand::CallControlFloorRelease { .. }
        | ControlCommand::CallControlExportRestoreContext { .. }
        | ControlCommand::CallControlImportRestoreContext { .. }
        | ControlCommand::CallControlRemoveRestoreContext { .. } => TetraEntity::Cmce,
        ControlCommand::RestartService => TetraEntity::Cmce,
        ControlCommand::ShutdownService => TetraEntity::Cmce,
        ControlCommand::AddLiveSds { .. } => TetraEntity::Cmce,
        ControlCommand::DeleteLiveSds { .. } => TetraEntity::Cmce,
        ControlCommand::ClearLiveSds => TetraEntity::Cmce,
        ControlCommand::ClearEmergency { .. } => TetraEntity::Cmce,
        ControlCommand::CommandA { .. } => TetraEntity::Mm,
        ControlCommand::TestCmdB { .. } => TetraEntity::Cmce,
        ControlCommand::PacketDataContextDeactivate { .. }
        | ControlCommand::PacketDataContextModify { .. }
        | ControlCommand::PacketDataWake { .. }
        | ControlCommand::PacketDataEndOfData { .. } => TetraEntity::Sndcp,
    }
}

fn correlation_key_for_command(command: &ControlCommand) -> Option<CommandCorrelationKey> {
    match command {
        ControlCommand::SendSds { handle, .. }
        | ControlCommand::SendRawSdsType4 { handle, .. }
        | ControlCommand::DeliverSds { handle, .. }
        | ControlCommand::SendStatus { handle, .. }
        | ControlCommand::CommandA { handle, .. }
        | ControlCommand::TestCmdB { handle, .. }
        | ControlCommand::PacketDataContextDeactivate { handle, .. }
        | ControlCommand::PacketDataContextModify { handle, .. }
        | ControlCommand::PacketDataWake { handle, .. }
        | ControlCommand::PacketDataEndOfData { handle, .. } => Some(CommandCorrelationKey::Handle(*handle)),
        ControlCommand::KickMs { issi } => Some(CommandCorrelationKey::KickMs(*issi)),
        ControlCommand::MobilityExportContext { handle, .. }
        | ControlCommand::MobilityImportContext { handle, .. }
        | ControlCommand::MobilityRemoveContext { handle, .. } => {
            Some(CommandCorrelationKey::Mobility(*handle))
        }
        ControlCommand::SubscriberAccessPolicyApply { handle, .. } => {
            Some(CommandCorrelationKey::SubscriberPolicy(*handle))
        }
        ControlCommand::GroupAccessPolicyApply { handle, .. } => {
            Some(CommandCorrelationKey::GroupPolicy(*handle))
        }
        ControlCommand::GroupDgnaApply { handle, .. } => {
            Some(CommandCorrelationKey::GroupDgna(*handle))
        }
        ControlCommand::CallControlGroupStart { handle, .. }
        | ControlCommand::CallControlIndividualStart { handle, .. }
        | ControlCommand::CallControlRelease { handle, .. }
        | ControlCommand::CallControlFloorRequest { handle, .. }
        | ControlCommand::CallControlFloorRelease { handle, .. }
        | ControlCommand::CallControlExportRestoreContext { handle, .. }
        | ControlCommand::CallControlImportRestoreContext { handle, .. }
        | ControlCommand::CallControlRemoveRestoreContext { handle, .. } => {
            Some(CommandCorrelationKey::CallControl(*handle))
        }
        ControlCommand::Dgna { issi, gssi, attach } => Some(CommandCorrelationKey::Dgna {
            issi: *issi,
            gssi: *gssi,
            attach: *attach,
        }),
        ControlCommand::RestartService => Some(CommandCorrelationKey::RestartService),
        ControlCommand::ShutdownService => Some(CommandCorrelationKey::ShutdownService),
        ControlCommand::AddLiveSds {
            text,
            protocol_id,
            source_issi,
            ..
        } => Some(CommandCorrelationKey::LiveSdsAdd {
            source_issi: *source_issi,
            protocol_id: *protocol_id,
            text: text.clone(),
        }),
        ControlCommand::DeleteLiveSds { id } => Some(CommandCorrelationKey::LiveSdsDelete(*id)),
        ControlCommand::ClearLiveSds => Some(CommandCorrelationKey::LiveSdsClear),
        ControlCommand::ClearEmergency { issi } => Some(CommandCorrelationKey::ClearEmergency(*issi)),
    }
}

fn correlation_key_for_response(response: &ControlResponse) -> Option<CommandCorrelationKey> {
    match response {
        ControlResponse::CommandAResponse { handle, .. }
        | ControlResponse::SendSdsResponse { handle, .. }
        | ControlResponse::SdsDeliveryResponse { handle, .. }
        | ControlResponse::PacketDataActionResult { handle, .. } => {
            Some(CommandCorrelationKey::Handle(*handle))
        }
        ControlResponse::KickMsResponse { issi, .. } => Some(CommandCorrelationKey::KickMs(*issi)),
        ControlResponse::MobilityContextExported { handle, .. }
        | ControlResponse::MobilityContextImported { handle, .. }
        | ControlResponse::MobilityContextRemoved { handle, .. } => {
            Some(CommandCorrelationKey::Mobility(*handle))
        }
        ControlResponse::SubscriberAccessPolicyApplied { handle, .. } => {
            Some(CommandCorrelationKey::SubscriberPolicy(*handle))
        }
        ControlResponse::GroupAccessPolicyApplied { handle, .. } => {
            Some(CommandCorrelationKey::GroupPolicy(*handle))
        }
        ControlResponse::GroupDgnaApplied { handle, .. } => {
            Some(CommandCorrelationKey::GroupDgna(*handle))
        }
        ControlResponse::CallControlLegStarted { handle, .. }
        | ControlResponse::CallControlLegReleased { handle, .. }
        | ControlResponse::CallControlFloorChanged { handle, .. }
        | ControlResponse::CallControlRestoreContextExported { handle, .. }
        | ControlResponse::CallControlRestoreContextImported { handle, .. }
        | ControlResponse::CallControlRestoreContextRemoved { handle, .. } => {
            Some(CommandCorrelationKey::CallControl(*handle))
        }
    }
}

fn now_iso() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_dgna_to_mm() {
        assert_eq!(
            route_control_command(&ControlCommand::Dgna {
                issi: 1,
                gssi: 2,
                attach: true,
            }),
            TetraEntity::Mm
        );
    }

    #[test]
    fn routes_central_group_commands_to_mm() {
        assert_eq!(
            route_control_command(&ControlCommand::GroupAccessPolicyApply {
                handle: 7,
                revision: 1,
                allow_unlisted_groups: false,
                enforce_memberships: true,
                reconcile_registered: true,
                groups: Vec::new(),
                memberships: Vec::new(),
            }),
            TetraEntity::Mm
        );
        assert_eq!(
            route_control_command(&ControlCommand::GroupDgnaApply {
                handle: 8,
                issi: 1001,
                gssi: 15501,
                attach: true,
                force: false,
            }),
            TetraEntity::Mm
        );
    }

    #[test]
    fn correlates_group_policy_and_dgna_responses() {
        let policy_command = ControlCommand::GroupAccessPolicyApply {
            handle: 77,
            revision: 2,
            allow_unlisted_groups: false,
            enforce_memberships: true,
            reconcile_registered: true,
            groups: Vec::new(),
            memberships: Vec::new(),
        };
        let policy_response = ControlResponse::GroupAccessPolicyApplied {
            handle: 77,
            revision: 2,
            success: true,
            group_count: 0,
            membership_count: 0,
            attached_count: 0,
            detached_count: 0,
            message: String::new(),
        };
        assert_eq!(
            correlation_key_for_command(&policy_command),
            correlation_key_for_response(&policy_response)
        );

        let dgna_command = ControlCommand::GroupDgnaApply {
            handle: 78,
            issi: 1001,
            gssi: 15501,
            attach: true,
            force: false,
        };
        let dgna_response = ControlResponse::GroupDgnaApplied {
            handle: 78,
            issi: 1001,
            gssi: 15501,
            attach: true,
            success: true,
            message: String::new(),
        };
        assert_eq!(
            correlation_key_for_command(&dgna_command),
            correlation_key_for_response(&dgna_response)
        );
    }

    #[test]
    fn correlates_handle_based_sds() {
        let cmd = ControlCommand::SendSds {
            handle: 42,
            source_ssi: 9999,
            dest_ssi: 123,
            dest_is_group: false,
            len_bits: 8,
            payload: vec![1],
        };
        let resp = ControlResponse::SendSdsResponse { handle: 42, success: true };
        assert_eq!(correlation_key_for_command(&cmd), correlation_key_for_response(&resp));
    }
}
