use crate::net_control::{ControlCommand, ControlEndpoint, ControlResponse};
use crate::net_telemetry::TelemetrySink;
use crate::{MessageQueue, TetraEntityTrait};
use tetra_config::bluestation::SharedConfig;
use tetra_core::tetra_entities::TetraEntity;
use tetra_core::{Sap, TdmaTime, unimplemented_log};
use tetra_saps::{SapMsg, SapMsgInner};

use super::components::pc_bs::{ControlRoute, LcmcRoute, PcBs};
use super::subentities::cc_bs::CcBsSubentity;
use super::subentities::sds_bs::{SdsBsSubentity, SdsPendingAction};
use super::subentities::ss_bs::SsBsSubentity;

pub struct CmceBs {
    config: SharedConfig,
    telemetry: Option<TelemetrySink>,
    control: Option<ControlEndpoint>,
    dashboard_control: Option<ControlEndpoint>,

    pc: PcBs,
    cc: CcBsSubentity,
    sds: SdsBsSubentity,
    ss: SsBsSubentity,
}

impl CmceBs {
    pub fn new(config: SharedConfig, telemetry: Option<TelemetrySink>, control: Option<ControlEndpoint>) -> Self {
        let mut sds = SdsBsSubentity::new(config.clone());
        if let Some(ref sink) = telemetry {
            sds.set_telemetry(sink.clone());
        }

        let mut cc = CcBsSubentity::new(config.clone());
        if let Some(ref sink) = telemetry {
            cc.set_telemetry(sink.clone());
        }

        Self {
            config: config.clone(),
            telemetry,
            control,
            dashboard_control: None,
            pc: PcBs::new(),
            sds,
            cc,
            ss: SsBsSubentity::new(),
        }
    }

    pub fn set_dashboard_control(&mut self, endpoint: ControlEndpoint) {
        self.dashboard_control = Some(endpoint);
    }

    pub fn set_wx_cmd_sender(&mut self, tx: crossbeam_channel::Sender<ControlCommand>) {
        self.sds.set_wx_cmd_sender(tx);
    }

    /// Execute a single control command. Shared by both the main `control` link (where a
    /// `responder` is supplied so request/response commands can reply) and the dashboard
    /// control link (where `responder` is `None`). Unknown commands are logged, never panic —
    /// a control-plane peer must not be able to crash the base station.
    fn do_control_command(
        sds: &mut SdsBsSubentity,
        cc: &mut CcBsSubentity,
        queue: &mut MessageQueue,
        cmd: ControlCommand,
        responder: Option<&ControlEndpoint>,
    ) {
        match cmd {
            ControlCommand::CallControlGroupStart { handle, operation_id, source_issi, gssi, priority } => {
                let result = cc.control_start_group_call(queue, &operation_id, source_issi, gssi, priority);
                if let Some(cep) = responder {
                    cep.respond(ControlResponse::CallControlLegStarted {
                        handle, operation_id, kind: result.kind, success: result.success,
                        call_id: result.call_id, timeslot: result.timeslot, usage: result.usage,
                        floor_holder: result.floor_holder, message: result.message,
                    });
                }
            }
            ControlCommand::CallControlIndividualStart { handle, operation_id, calling_issi, called_issi, simplex, priority } => {
                let result = cc.control_start_individual_call(queue, &operation_id, calling_issi, called_issi, simplex, priority);
                if let Some(cep) = responder {
                    cep.respond(ControlResponse::CallControlLegStarted {
                        handle, operation_id, kind: result.kind, success: result.success,
                        call_id: result.call_id, timeslot: result.timeslot, usage: result.usage,
                        floor_holder: result.floor_holder, message: result.message,
                    });
                }
            }
            ControlCommand::CallControlRelease { handle, call_id, cause } => {
                let result = cc.control_release_call(queue, call_id, cause);
                if let Some(cep) = responder {
                    cep.respond(ControlResponse::CallControlLegReleased { handle, call_id,
                        success: result.success, message: result.message });
                }
            }
            ControlCommand::CallControlFloorRequest { handle, call_id, source_issi, force } => {
                let result = cc.control_request_floor(queue, call_id, source_issi, force);
                if let Some(cep) = responder {
                    cep.respond(ControlResponse::CallControlFloorChanged { handle, call_id,
                        success: result.success, floor_holder: result.floor_holder,
                        queued_issi: result.queued_issi, message: result.message });
                }
            }
            ControlCommand::CallControlFloorRelease { handle, call_id } => {
                let result = cc.control_release_floor(queue, call_id);
                if let Some(cep) = responder {
                    cep.respond(ControlResponse::CallControlFloorChanged { handle, call_id,
                        success: result.success, floor_holder: result.floor_holder,
                        queued_issi: result.queued_issi, message: result.message });
                }
            }
            ControlCommand::SendSds { handle, .. } => {
                let success = sds.rx_sds_from_control(queue, cmd);
                if let Some(cep) = responder {
                    cep.respond(ControlResponse::SendSdsResponse { handle, success });
                }
            }
            ControlCommand::SendRawSdsType4 { handle, .. } => {
                let success = sds.rx_sds_from_control(queue, cmd);
                if let Some(cep) = responder {
                    cep.respond(ControlResponse::SendSdsResponse { handle, success });
                }
            }
            ControlCommand::DeliverSds { handle, .. } => {
                let success = sds.rx_sds_from_control(queue, cmd);
                if let Some(cep) = responder {
                    cep.respond(ControlResponse::SdsDeliveryResponse {
                        handle,
                        success,
                        // Acceptance into the local SDS path is not an MS receipt confirmation.
                        message: if success { "SDS accepted for local radio delivery" } else { "Invalid SDS delivery payload" }.to_string(),
                    });
                }
            }
            ControlCommand::SendStatus { handle, source_ssi, dest_ssi, pre_coded_status } => {
                let success = sds.send_status_from_control(queue, source_ssi, dest_ssi, pre_coded_status);
                if let Some(cep) = responder {
                    cep.respond(ControlResponse::SdsDeliveryResponse {
                        handle,
                        success,
                        message: if success { "Status accepted for local radio delivery" } else { "Invalid status source or destination" }.to_string(),
                    });
                }
            }
            ControlCommand::KickMs { issi } => {
                tracing::info!("CMCE: KickMs issi={} requested", issi);
                let success = cc.kick_ms(queue, issi);
                if let Some(cep) = responder {
                    cep.respond(ControlResponse::KickMsResponse { issi, success });
                }
            }
            ControlCommand::Dgna { issi, gssi, attach } => {
                // The dashboard control channel terminates at CMCE, but DGNA is a Mobility
                // Management procedure — group attach/detach state and the D-ATTACH/DETACH GROUP
                // IDENTITY send path both live in MM. Forward the request there.
                tracing::info!(
                    "CMCE: forwarding DGNA {} of GSSI {} on ISSI {} to MM",
                    if attach { "assign" } else { "deassign" },
                    gssi,
                    issi
                );
                queue.push_back(SapMsg {
                    sap: Sap::Control,
                    src: TetraEntity::Cmce,
                    dest: TetraEntity::Mm,
                    msg: SapMsgInner::MmDgnaRequest { issi, gssi, attach },
                });
            }
            ControlCommand::RestartService => {
                tracing::info!("CMCE: RestartService requested");
                crate::service_control::schedule_service_action(
                    crate::service_control::ServiceAction::Restart,
                    std::time::Duration::from_millis(500),
                );
            }
            ControlCommand::ShutdownService => {
                tracing::info!("CMCE: ShutdownService requested");
                crate::service_control::schedule_service_action(
                    crate::service_control::ServiceAction::Stop,
                    std::time::Duration::from_millis(500),
                );
            }
            ControlCommand::AddLiveSds {
                text,
                protocol_id,
                source_issi,
                repeat_count,
            } => {
                let mut state = sds.shared_config().state_write();
                let id = state.next_live_sds_id;
                state.next_live_sds_id = state.next_live_sds_id.wrapping_add(1).max(1);
                state.live_sds_queue.push_back(tetra_config::bluestation::LiveSdsMessage {
                    id,
                    text: text.clone(),
                    protocol_id,
                    source_issi,
                    repeat_count,
                    sent_count: 0,
                });
                tracing::info!("CMCE: AddLiveSds id={} repeat={} text={:?}", id, repeat_count, text);
            }
            ControlCommand::DeleteLiveSds { id } => {
                let mut state = sds.shared_config().state_write();
                let before = state.live_sds_queue.len();
                state.live_sds_queue.retain(|m| m.id != id);
                let removed = before - state.live_sds_queue.len();
                tracing::info!("CMCE: DeleteLiveSds id={} removed={}", id, removed);
            }
            ControlCommand::ClearLiveSds => {
                let mut state = sds.shared_config().state_write();
                let n = state.live_sds_queue.len();
                state.live_sds_queue.clear();
                tracing::info!("CMCE: ClearLiveSds removed={}", n);
            }
            ControlCommand::ClearEmergency { issi } => {
                tracing::info!("CMCE: ClearEmergency issi={} (operator)", issi);

                // First perform the SwMI/Call-Control part: emergency button calls arrive as
                // priority-15 CC calls, so clearing them from the dashboard must release the
                // traffic call, not only remove a local banner.
                let released = cc.clear_emergency_calls_for_issi(queue, issi);
                if released > 0 {
                    tracing::warn!(
                        "CMCE: ClearEmergency issi={} released {} active emergency-priority call(s)",
                        issi,
                        released
                    );
                }

                // Then clear any status-based emergency session/banner for the same ISSI. If the
                // emergency came purely from a priority-15 call this is harmless/no-op, while a
                // U-STATUS emergency still gets the existing EmergencyCancel telemetry.
                sds.clear_emergency_command(queue, issi);
            }
            _ => {
                tracing::warn!("CMCE: ignoring unsupported control command {:?}", cmd);
            }
        }
    }

    pub fn rx_lcmc_mle_unitdata_ind(&mut self, _queue: &mut MessageQueue, mut message: SapMsg) {
        tracing::trace!("rx_lcmc_mle_unitdata_ind");

        let Some(route) = self.pc.route_lcmc_unitdata_ind(&mut message) else {
            return;
        };

        match route {
            LcmcRoute::CcRd => {
                self.cc.route_rd_deliver(_queue, message);
            }
            LcmcRoute::SdsStatus => {
                self.sds.route_status_deliver(_queue, message);
            }
            LcmcRoute::SdsRf => {
                self.sds.route_rf_deliver(_queue, message);
            }
            LcmcRoute::SsRe => {
                self.ss.route_re_deliver(_queue, message);
            }
            LcmcRoute::Unsupported(pdu_type) => {
                unimplemented_log!("{:?}", pdu_type);
            }
        };
    }
}

impl TetraEntityTrait for CmceBs {
    fn entity(&self) -> TetraEntity {
        TetraEntity::Cmce
    }

    fn set_config(&mut self, config: SharedConfig) {
        self.config = config;
    }

    fn tick_start(&mut self, queue: &mut MessageQueue, ts: TdmaTime) {
        // Propagate tick to subentities
        self.cc.tick_start(queue, ts);
        // Republish the in-call ISSI→timeslot map so SDS can FACCH-steal to in-call radios
        // (FH-BUG-034). Rebuilt from the live call tables every tick.
        self.cc.publish_active_call_ts();
        self.sds.tick_start(queue, ts);
        self.sds.tick_periodic_wx();

        // Process incoming control commands, if the main control link is enabled (request/response).
        if let Some(cep) = &self.control {
            for _ in 0..8 {
                let Some(cmd) = cep.try_recv() else { break; };
                CmceBs::do_control_command(&mut self.sds, &mut self.cc, queue, cmd, Some(cep));
            }
        }
        // Process commands from the dashboard control link (fire-and-forget, no responder).
        if let Some(cep) = &self.dashboard_control {
            for _ in 0..8 {
                let Some(cmd) = cep.try_recv() else { break; };
                CmceBs::do_control_command(&mut self.sds, &mut self.cc, queue, cmd, None);
            }
        }

        // Drain SDS-triggered actions that require access to CcBsSubentity.
        let pending = std::mem::take(&mut self.sds.pending_actions);
        for action in pending {
            match action {
                SdsPendingAction::KickAll => {
                    let issis: Vec<u32> = self.cc.subscriber_issis();
                    tracing::info!("SDS-CMD: kick_all — deregistering {} subscribers", issis.len());
                    for issi in issis {
                        self.cc.kick_ms(queue, issi);
                    }
                }
            }
        }
    }

    fn rx_prim(&mut self, queue: &mut MessageQueue, message: SapMsg) {
        tracing::debug!("rx_prim: {:?}", message);
        // tracing::debug!(ts=%message.dltime, "rx_prim: {:?}", message);

        match message.sap {
            Sap::LcmcSap => match message.msg {
                SapMsgInner::LcmcMleUnitdataInd(_) => {
                    self.rx_lcmc_mle_unitdata_ind(queue, message);
                }
                _ => {
                    panic!("Unexpected message on LcmcSap: {:?}", message.msg);
                }
            },
            Sap::Control => match self.pc.route_control(&message) {
                ControlRoute::CcRa => {
                    self.cc.rx_call_control(queue, message);
                }
                ControlRoute::CcSubscriberUpdate => {
                    let SapMsgInner::MmSubscriberUpdate(update) = message.msg else {
                        unreachable!();
                    };
                    self.sds.handle_subscriber_update(queue, &update);
                    self.cc.handle_subscriber_update(queue, update);
                }
                ControlRoute::SdsRc => {
                    self.sds.rx_sds_from_brew(queue, message);
                }
                ControlRoute::Unsupported => {
                    panic!("Unexpected control message: {:?}", message.msg);
                }
            },
            _ => {
                panic!("Unexpected SAP: {:?}", message.sap);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::net_control::make_control_link;
    use tetra_core::{SsiType, TetraAddress};
    use tetra_pdus::cmce::enums::pre_coded_status::PreCodedStatus;
    use tetra_pdus::cmce::pdus::{d_sds_data::DSdsData, d_status::DStatus};
    use tetra_saps::control::enums::sds_user_data::SdsUserData;
    use tetra_saps::lcmc::LcmcMleUnitdataReq;

    fn dispatch(command: ControlCommand) -> (MessageQueue, ControlResponse) {
        let config = tetra_config::bluestation::parsing::from_toml_str(r#"
config_version = "0.6"
stack_mode = "Bs"
[phy_io]
backend = "None"
[net_info]
mcc = 901
mnc = 9999
[cell_info]
main_carrier = 1584
freq_band = 4
freq_offset = 0
duplex_spacing = 4
reverse_operation = false
location_area = 1
"#).unwrap();
        let config = SharedConfig::from_parts(config, None);
        config.state_write().subscribers.register(5102);
        let mut cmce = CmceBs::new(config, None, None);
        let (dispatcher, endpoint) = make_control_link();
        let mut queue = MessageQueue::new();
        CmceBs::do_control_command(&mut cmce.sds, &mut cmce.cc, &mut queue, command, Some(&endpoint));
        let response = dispatcher.try_recv_response().expect("central command must receive a correlated result");
        assert!(dispatcher.try_recv_response().is_none(), "exactly one response per command");
        (queue, response)
    }

    fn assert_delivery_response(response: ControlResponse, expected_handle: u32, expected_success: bool) {
        let ControlResponse::SdsDeliveryResponse { handle, success, message } = response else {
            panic!("central SDS/status commands require SdsDeliveryResponse, got {response:?}");
        };
        assert_eq!(handle, expected_handle);
        assert_eq!(success, expected_success);
        assert!(!message.is_empty());
    }

    fn radio_request(mut queue: MessageQueue, is_group: bool) -> LcmcMleUnitdataReq {
        let message = queue.pop_front().expect("command must enqueue a radio PDU");
        assert!(queue.is_empty(), "command must enqueue exactly one radio PDU");
        assert_eq!(message.sap, Sap::LcmcSap);
        assert_eq!(message.src, TetraEntity::Cmce);
        assert_eq!(message.dest, TetraEntity::Mle);
        let SapMsgInner::LcmcMleUnitdataReq(request) = message.msg else {
            panic!("expected LCMC radio delivery request");
        };
        assert_eq!(request.main_address, TetraAddress::new(5102, if is_group { SsiType::Gssi } else { SsiType::Issi }));
        request
    }

    #[test]
    fn central_sds_dispatch_preserves_payload_and_destination_on_radio_path() {
        let cases = [
            SdsUserData::Type1(0x1234),
            SdsUserData::Type2(0x12345678),
            SdsUserData::Type3(0x123456789abcdef0),
            SdsUserData::Type4(64, vec![0x82, 0, 1, 1, b'T', b'e', b's', b't']),
        ];
        for data in cases {
            for is_group in [false, true] {
                let (queue, response) = dispatch(ControlCommand::DeliverSds {
                    handle: 67,
                    source_ssi: 4010112,
                    dest_ssi: 5102,
                    dest_is_group: is_group,
                    sds_type: data.type_identifier() + 1,
                    len_bits: data.length_bits(),
                    payload: data.to_arr(),
                });
                assert_delivery_response(response, 67, true);
                let mut request = radio_request(queue, is_group);
                let pdu = DSdsData::from_bitbuf(&mut request.sdu).unwrap();
                assert_eq!(pdu.calling_party_address_ssi, Some(4010112));
                assert_eq!(pdu.user_defined_data, data, "central payload must not be wrapped again");
            }
        }
    }

    #[test]
    fn invalid_central_sds_payload_reports_failure_without_radio_delivery() {
        for (sds_type, len_bits, payload) in [(1, 16, vec![0]), (4, 16, vec![0]), (4, 0, vec![]), (5, 8, vec![0])] {
            let (queue, response) = dispatch(ControlCommand::DeliverSds {
                handle: 68,
                source_ssi: 4010112,
                dest_ssi: 5102,
                dest_is_group: false,
                sds_type,
                len_bits,
                payload,
            });
            assert_delivery_response(response, 68, false);
            assert!(queue.is_empty());
        }
    }

    #[test]
    fn central_status_dispatch_enqueues_status_pdu_and_correlated_response() {
        let (queue, response) = dispatch(ControlCommand::SendStatus {
            handle: 69,
            source_ssi: 4010112,
            dest_ssi: 5102,
            pre_coded_status: 32769,
        });
        assert_delivery_response(response, 69, true);
        let mut request = radio_request(queue, false);
        let pdu = DStatus::from_bitbuf(&mut request.sdu).unwrap();
        assert_eq!(pdu.calling_party_address_ssi, Some(4010112));
        assert_eq!(pdu.pre_coded_status, PreCodedStatus::NetworkUserSpecific(32769));
    }

    #[test]
    fn invalid_central_status_address_reports_failure_without_radio_delivery() {
        for (source_ssi, dest_ssi) in [(0, 5102), (4010112, 0), (0x1000000, 5102), (4010112, 0x1000000)] {
            let (queue, response) = dispatch(ControlCommand::SendStatus {
                handle: 70,
                source_ssi,
                dest_ssi,
                pre_coded_status: 32769,
            });
            assert_delivery_response(response, 70, false);
            assert!(queue.is_empty());
        }
    }
}
