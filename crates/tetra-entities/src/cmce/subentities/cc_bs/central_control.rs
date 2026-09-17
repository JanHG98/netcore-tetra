//! Central call commands adapted to the MAIN-COMPAT radio FSM.


use super::*;
use crate::net_control::ManagedCallKind;

#[derive(Debug, Clone)]
pub(in crate::cmce) struct ManagedLegResult {
    pub success: bool,
    pub kind: ManagedCallKind,
    pub call_id: Option<u16>,
    pub timeslot: Option<u8>,
    pub usage: Option<u8>,
    pub floor_holder: Option<u32>,
    pub queued_issi: Option<u32>,
    pub message: String,
}

impl ManagedLegResult {
    fn failure(kind: ManagedCallKind, message: impl Into<String>) -> Self {
        Self {
            success: false,
            kind,
            call_id: None,
            timeslot: None,
            usage: None,
            floor_holder: None,
            queued_issi: None,
            message: message.into(),
        }
    }
}

impl CcBsSubentity {
    pub(in crate::cmce) fn control_start_group_call(
        &mut self,
        queue: &mut MessageQueue,
        operation_id: &str,
        source_issi: u32,
        gssi: u32,
        priority: u8,
    ) -> ManagedLegResult {
        let operation_uuid = match uuid::Uuid::parse_str(operation_id) {
            Ok(value) => value,
            Err(error) => {
                return ManagedLegResult::failure(
                    ManagedCallKind::Group,
                    format!("invalid operation UUID: {error}"),
                );
            }
        };
        if let Some((call_id, call)) = self.active_calls.iter().find(|(_, c)| c.brew_uuid == Some(operation_uuid)) {
            if call.dest_gssi == gssi {
                return self.control_leg_result(*call_id).unwrap();
            }
            return ManagedLegResult::failure(ManagedCallKind::Group, "operation UUID already belongs to another call");
        }
        if self.find_brew_individual_call(operation_uuid).is_some() {
            return ManagedLegResult::failure(ManagedCallKind::Group, "operation UUID already belongs to an individual call");
        }
        if source_issi == 0 || source_issi > 0x00ff_ffff {
            return ManagedLegResult::failure(
                ManagedCallKind::Group,
                "source ISSI must be in 1..=16777215",
            );
        }
        if gssi == 0 || gssi > 0x00ff_ffff {
            return ManagedLegResult::failure(
                ManagedCallKind::Group,
                "GSSI must be in 1..=16777215",
            );
        }

        self.fsm_on_network_call_start(
            queue,
            TetraEntity::Cmce,
            operation_uuid,
            source_issi,
            gssi,
            priority.min(15),
        );

        let Some((call_id, call)) = self
            .active_calls
            .iter()
            .find(|(_, call)| call.brew_uuid == Some(operation_uuid))
            .map(|(call_id, call)| (*call_id, call.clone()))
        else {
            return ManagedLegResult::failure(
                ManagedCallKind::Group,
                "local group-call leg was not admitted (no listener, no resource or policy rejection)",
            );
        };

        self.bind_central_media(queue, operation_uuid, call.ts);
        ManagedLegResult {
            success: true,
            kind: ManagedCallKind::Group,
            call_id: Some(call_id),
            timeslot: Some(call.ts),
            usage: Some(call.usage),
            floor_holder: call.tx_active.then_some(call.source_issi),
            queued_issi: call.queued_tx_demand.map(|address| address.ssi),
            message: "group-call leg active".to_string(),
        }
    }

    pub(in crate::cmce) fn control_start_individual_call(
        &mut self,
        queue: &mut MessageQueue,
        operation_id: &str,
        calling_issi: u32,
        called_issi: u32,
        simplex: bool,
        priority: u8,
    ) -> ManagedLegResult {
        let operation_uuid = match uuid::Uuid::parse_str(operation_id) {
            Ok(value) => value,
            Err(error) => {
                return ManagedLegResult::failure(
                    ManagedCallKind::Individual,
                    format!("invalid operation UUID: {error}"),
                );
            }
        };
        if let Some((call_id, call)) = self.find_brew_individual_call(operation_uuid) {
            if call.calling_addr.ssi == calling_issi && call.called_addr.ssi == called_issi && call.is_simplex() == simplex {
                return self.control_leg_result(call_id).unwrap();
            }
            return ManagedLegResult::failure(ManagedCallKind::Individual, "operation UUID already belongs to another call");
        }
        if self.active_calls.values().any(|c| c.brew_uuid == Some(operation_uuid)) {
            return ManagedLegResult::failure(ManagedCallKind::Individual, "operation UUID already belongs to a group call");
        }
        if calling_issi == 0 || calling_issi > 0x00ff_ffff {
            return ManagedLegResult::failure(
                ManagedCallKind::Individual,
                "calling ISSI must be in 1..=16777215",
            );
        }
        if called_issi == 0 || called_issi > 0x00ff_ffff {
            return ManagedLegResult::failure(
                ManagedCallKind::Individual,
                "called ISSI must be in 1..=16777215",
            );
        }

        let call = NetworkCircuitCall {
            source_issi: calling_issi,
            destination: called_issi,
            number: calling_issi.to_string(),
            priority: priority.min(15),
            service: 0,
            mode: CircuitModeType::TchS.into_raw() as u8,
            duplex: u8::from(!simplex),
            method: 0,
            communication: CommunicationType::P2p.into_raw() as u8,
            grant: TransmissionGrant::GrantedToOtherUser.into_raw() as u8,
            permission: 1,
            timeout: CallTimeout::T5m.into_raw() as u8,
            ownership: 0,
            queued: 0,
        };

        self.fsm_on_network_circuit_setup_request(
            queue,
            TetraEntity::Cmce,
            operation_uuid,
            call,
        );

        let Some((call_id, leg)) = self.find_brew_individual_call(operation_uuid) else {
            return ManagedLegResult::failure(
                ManagedCallKind::Individual,
                "individual-call leg was not admitted (subscriber offline, busy, no resource or policy rejection)",
            );
        };

        let leg = leg.clone();
        self.bind_central_media(queue, operation_uuid, leg.called_ts);
        ManagedLegResult {
            success: true,
            kind: ManagedCallKind::Individual,
            call_id: Some(call_id),
            timeslot: Some(leg.called_ts),
            usage: Some(leg.called_usage),
            floor_holder: leg.floor_holder,
            queued_issi: leg.queued_tx_demand.map(|address| address.ssi),
            message: "individual-call setup leg created; awaiting called-party signalling".to_string(),
        }
    }

    pub(in crate::cmce) fn control_release_call(
        &mut self,
        queue: &mut MessageQueue,
        call_id: u16,
        cause: u8,
    ) -> ManagedLegResult {
        let cause = DisconnectCause::try_from(cause as u64)
            .unwrap_or(DisconnectCause::SwmiRequestedDisconnection);
        if self.active_calls.contains_key(&call_id) {
            self.release_group_call(queue, call_id, cause);
            return ManagedLegResult {
                success: true,
                kind: ManagedCallKind::Group,
                call_id: Some(call_id),
                timeslot: None,
                usage: None,
                floor_holder: None,
                queued_issi: None,
                message: "group-call leg released".to_string(),
            };
        }
        if self.individual_calls.contains_key(&call_id) {
            self.release_individual_call(queue, call_id, cause);
            return ManagedLegResult {
                success: true,
                kind: ManagedCallKind::Individual,
                call_id: Some(call_id),
                timeslot: None,
                usage: None,
                floor_holder: None,
                queued_issi: None,
                message: "individual-call leg released".to_string(),
            };
        }
        ManagedLegResult::failure(
            ManagedCallKind::Group,
            format!("unknown local call identifier {call_id}"),
        )
    }

    pub(in crate::cmce) fn control_request_floor(
        &mut self,
        queue: &mut MessageQueue,
        call_id: u16,
        source_issi: u32,
        force: bool,
    ) -> ManagedLegResult {
        let Some(before) = self.control_leg_result(call_id) else {
            return ManagedLegResult::failure(
                ManagedCallKind::Group,
                format!("unknown local call identifier {call_id}"),
            );
        };
        if source_issi == 0 || source_issi > 0x00ff_ffff {
            return ManagedLegResult::failure(before.kind, "source ISSI must be in 1..=16777215");
        }

        let requester = TetraAddress::new(source_issi, SsiType::Issi);
        let current_holder = before.floor_holder;
        if force && current_holder != Some(source_issi) {
            if let Some(call) = self.active_calls.get_mut(&call_id) {
                call.queued_tx_demand = None;
            }
            if let Some(call) = self.individual_calls.get_mut(&call_id) {
                call.queued_tx_demand = None;
            }
        }

        self.fsm_on_u_tx_demand(
            queue,
            requester,
            UTxDemand {
                call_identifier: call_id,
                tx_demand_priority: 3,
                encryption_control: false,
                reserved: false,
                facility: None,
                dm_ms_address: None,
                proprietary: None,
            },
        );

        if force
            && let Some(holder) = current_holder
            && holder != source_issi
        {
            self.fsm_on_u_tx_ceased(
                queue,
                TetraAddress::new(holder, SsiType::Issi),
                UTxCeased {
                    call_identifier: call_id,
                    facility: None,
                    dm_ms_address: None,
                    proprietary: None,
                },
            );
        }

        let Some(mut result) = self.control_leg_result(call_id) else {
            return ManagedLegResult::failure(before.kind, "call ended while changing floor");
        };
        result.success = result.floor_holder == Some(source_issi)
            || result.queued_issi == Some(source_issi);
        result.message = if result.floor_holder == Some(source_issi) {
            "floor granted".to_string()
        } else if result.queued_issi == Some(source_issi) {
            "floor request queued".to_string()
        } else {
            "floor request not granted".to_string()
        };
        result
    }

    pub(in crate::cmce) fn control_release_floor(
        &mut self,
        queue: &mut MessageQueue,
        call_id: u16,
    ) -> ManagedLegResult {
        let Some(before) = self.control_leg_result(call_id) else {
            return ManagedLegResult::failure(
                ManagedCallKind::Group,
                format!("unknown local call identifier {call_id}"),
            );
        };
        let Some(holder) = before.floor_holder else {
            let mut result = before;
            result.success = true;
            result.message = "floor already idle".to_string();
            return result;
        };

        self.fsm_on_u_tx_ceased(
            queue,
            TetraAddress::new(holder, SsiType::Issi),
            UTxCeased {
                call_identifier: call_id,
                facility: None,
                dm_ms_address: None,
                proprietary: None,
            },
        );
        let Some(mut result) = self.control_leg_result(call_id) else {
            return ManagedLegResult {
                success: true,
                kind: before.kind,
                call_id: Some(call_id),
                timeslot: None,
                usage: None,
                floor_holder: None,
                queued_issi: None,
                message: "call ended while releasing floor".to_string(),
            };
        };
        result.success = result.floor_holder != Some(holder);
        result.message = if result.floor_holder.is_some() {
            "floor handed to queued requester".to_string()
        } else {
            "floor released".to_string()
        };
        result
    }

    fn bind_central_media(&self, queue: &mut MessageQueue, operation_id: uuid::Uuid, ts: u8) {
        queue.push_back(SapMsg {
            sap: Sap::Control, src: TetraEntity::Cmce, dest: TetraEntity::Umac,
            msg: SapMsgInner::CmceCallControl(CallControl::BindCentralMedia { operation_id, ts }),
        });
    }

    /// Callbacks for a core-owned leg have no local SIP dialog. Complete the
    /// same incoming-call FSM used by SIP after the radio accepts the call.
    pub(super) fn central_callback(&mut self, queue: &mut MessageQueue, control: CallControl) {
        match control {
            CallControl::NetworkCircuitConnectRequest { brew_uuid, .. } => {
                if self.find_brew_individual_call(brew_uuid).is_some_and(|(_, call)|
                    call.network_entity() == TetraEntity::Cmce &&
                    matches!(call.state, IndividualCallState::IncomingSetupWaitNetworkAck)) {
                    self.fsm_on_network_circuit_connect_confirm(queue, brew_uuid, 0, 0);
                }
            }
            CallControl::NetworkCircuitMediaReady { brew_uuid, call_id, ts } => {
                if let Some((_, call)) = self.find_brew_individual_call(brew_uuid) {
                    self.emit(crate::net_telemetry::TelemetryEvent::IndividualCallStarted {
                        call_id, calling_issi: call.calling_addr.ssi, called_issi: call.called_addr.ssi,
                        simplex: call.is_simplex(), ts, carrier_num: self.carrier_num_for_logical_ts(ts),
                        priority: call.priority, source: "core".to_string(),
                    });
                }
            }
            _ => {}
        }
        // Ready/end/alert callbacks are already reflected in normal lifecycle
        // telemetry. Never feed these notifications back into the request FSM.
    }

    fn control_leg_result(&self, call_id: u16) -> Option<ManagedLegResult> {
        if let Some(call) = self.active_calls.get(&call_id) {
            return Some(ManagedLegResult {
                success: true,
                kind: ManagedCallKind::Group,
                call_id: Some(call_id),
                timeslot: Some(call.ts),
                usage: Some(call.usage),
                floor_holder: call.tx_active.then_some(call.source_issi),
                queued_issi: call.queued_tx_demand.map(|address| address.ssi),
                message: "group-call leg found".to_string(),
            });
        }
        self.individual_calls.get(&call_id).map(|call| ManagedLegResult {
            success: true,
            kind: ManagedCallKind::Individual,
            call_id: Some(call_id),
            timeslot: Some(call.called_ts),
            usage: Some(call.called_usage),
            floor_holder: call.floor_holder,
            queued_issi: call.queued_tx_demand.map(|address| address.ssi),
            message: "individual-call leg found".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cc() -> CcBsSubentity {
        let cfg = tetra_config::bluestation::parsing::from_toml_str(r#"
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
        let cfg = SharedConfig::from_parts(cfg, None);
        cfg.state_write().subscribers.register(5102);
        CcBsSubentity::new(cfg)
    }

    fn callbacks(cc: &mut CcBsSubentity, queue: &mut MessageQueue) {
        for _ in 0..128 {
            let Some(message) = queue.pop_front() else { return; };
            assert_ne!(message.dest, TetraEntity::Asterisk, "core legs must not require a local SIP dialog");
            if message.dest == TetraEntity::Cmce {
                cc.rx_call_control(queue, message);
            }
        }
        panic!("recursive callback loop");
    }

    #[test]
    fn central_control_accepts_radio_connect_without_asterisk_dialog() {
        let mut cc = cc();
        let mut queue = MessageQueue::new();
        let operation = uuid::Uuid::new_v4();
        let first = cc.control_start_individual_call(&mut queue, &operation.to_string(), 103, 5102, false, 0);
        assert!(first.success, "{}", first.message);
        let repeated = cc.control_start_individual_call(&mut queue, &operation.to_string(), 103, 5102, false, 0);
        assert_eq!(first.call_id, repeated.call_id);
        assert_eq!(cc.individual_calls.len(), 1);
        assert!(!cc.control_start_individual_call(&mut queue, &operation.to_string(), 103, 999, false, 0).success);
        callbacks(&mut cc, &mut queue);
        let id = first.call_id.unwrap();
        cc.fsm_on_u_connect(&mut queue, TetraAddress::new(5102, SsiType::Issi), 0, 1, 0,
            UConnect { call_identifier: id, hook_method_selection: true, simplex_duplex_selection: true,
                basic_service_information: None, facility: None, proprietary: None });
        callbacks(&mut cc, &mut queue);
        assert!(cc.individual_calls[&id].is_active());
        assert!(cc.control_release_call(&mut queue, id, 2).success);
        callbacks(&mut cc, &mut queue);
        assert!(!cc.individual_calls.contains_key(&id));
    }

    #[test]
    fn central_control_rejects_offline_subscriber_without_allocating() {
        let mut cc = cc();
        let mut queue = MessageQueue::new();
        let result = cc.control_start_individual_call(&mut queue, &uuid::Uuid::new_v4().to_string(), 103, 999, false, 0);
        assert!(!result.success);
        callbacks(&mut cc, &mut queue);
        assert!(cc.individual_calls.is_empty());
    }
}
