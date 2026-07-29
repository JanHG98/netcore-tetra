// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für Gruppe Ereignis auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub(in crate::cmce::subentities::cc_bs) enum GroupEvent {
    TxDemand,
    TxCeased,
    NetworkCallStart,
    NetworkCallEnd,
}

#[derive(Clone, Copy, Debug, PartialEq)]
// Was: Listet die möglichen Varianten für Gruppe transition error auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub(in crate::cmce::subentities::cc_bs) enum GroupTransitionError {
    UnknownCall(u16),
    InvalidTransition {
        call_id: u16,
        state: GroupCallState,
        formal_state: CcFormalState,
        event: GroupEvent,
    },
    NotCurrentSpeaker {
        call_id: u16,
        sender_issi: u32,
        current_speaker_issi: u32,
    },
    MissingCachedSetup(u16),
}

// Was: Implementiert das zugehörige Verhalten für `CcBsSubentity`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl CcBsSubentity {
    // Was: Diese Funktion prüft Gruppe transition.
    // Warum: Unzulässige Werte werden dadurch erkannt, bevor sie im Betrieb Schaden anrichten.
    fn validate_group_transition(
        call_id: u16,
        state: GroupCallState,
        formal_state: CcFormalState,
        event: GroupEvent,
    ) -> Result<(), GroupTransitionError> {
        let allowed = state.formal_state() == formal_state
            && matches!(
                (formal_state, state, event),
                (CcFormalState::Active, GroupCallState::Transmitting, GroupEvent::TxDemand)
                    | (CcFormalState::Active, GroupCallState::NoActiveSpeaker { .. }, GroupEvent::TxDemand)
                    | (CcFormalState::Active, GroupCallState::Transmitting, GroupEvent::TxCeased)
                    | (CcFormalState::Active, GroupCallState::Transmitting, GroupEvent::NetworkCallStart)
                    | (
                        CcFormalState::Active,
                        GroupCallState::NoActiveSpeaker { .. },
                        GroupEvent::NetworkCallStart
                    )
                    | (CcFormalState::Active, GroupCallState::Transmitting, GroupEvent::NetworkCallEnd)
                    | (
                        CcFormalState::Active,
                        GroupCallState::NoActiveSpeaker { .. },
                        GroupEvent::NetworkCallEnd
                    )
            );
        if allowed {
            Ok(())
        } else {
            Err(GroupTransitionError::InvalidTransition {
                call_id,
                state,
                formal_state,
                event,
            })
        }
    }

    // Was: Führt den Arbeitsschritt `fsm_send_d_tx_granted_individual` für fsm send d tx granted individual aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fsm_send_d_tx_granted_individual(
        &self,
        queue: &mut MessageQueue,
        call_id: u16,
        target_addr: TetraAddress,
        ts: u8,
        transmission_grant: TransmissionGrant,
        transmitting_party_issi: Option<u32>,
    ) {
        let d_tx_granted = DTxGranted {
            call_identifier: call_id,
            transmission_grant: transmission_grant.into_raw() as u8,
            transmission_request_permission: false,
            encryption_control: false,
            reserved: false,
            notification_indicator: None,
            transmitting_party_type_identifier: transmitting_party_issi.map(|_| 1), // SSI
            transmitting_party_address_ssi: transmitting_party_issi.map(|ssi| ssi as u64),
            transmitting_party_extension: None,
            external_subscriber_number: None,
            facility: None,
            dm_ms_address: None,
            proprietary: None,
        };

        tracing::info!(
            "FSM -> D-TX GRANTED (individual, {}) call_id={} to ISSI {}",
            transmission_grant,
            call_id,
            target_addr.ssi
        );
        let mut sdu = BitBuffer::new_autoexpand(50);
        d_tx_granted.to_bitbuf(&mut sdu).expect("Failed to serialize DTxGranted");
        sdu.seek(0);

        let msg = Self::build_sapmsg_stealing(sdu, self.dltime, target_addr, ts, None);
        queue.push_back(msg);
    }

    // Was: Führt den Arbeitsschritt `fsm_group_on_tx_demand` für fsm Gruppe on tx demand aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(in crate::cmce::subentities::cc_bs) fn fsm_group_on_tx_demand(
        &mut self,
        queue: &mut MessageQueue,
        call_id: u16,
        requesting_party: TetraAddress,
    ) -> Result<(), GroupTransitionError> {
        let (ts, current_speaker, queue_result, brew_notification) = {
            let Some(call) = self.active_calls.get_mut(&call_id) else {
                return Err(GroupTransitionError::UnknownCall(call_id));
            };

            let state = call.state();
            let formal_state = call.formal_state;
            Self::validate_group_transition(call_id, state, formal_state, GroupEvent::TxDemand)?;

            let ts = call.ts;
            let current_speaker = call.source_issi;
            let grant_now = matches!(state, GroupCallState::NoActiveSpeaker { .. });
            let queue_result = if grant_now {
                call.grant_floor(requesting_party.ssi, Some(requesting_party));
                None
            } else {
                Some(call.queue_tx_demand(requesting_party))
            };
            let brew_notification = if grant_now {
                Self::brew_notification_for_group_call(call, requesting_party.ssi)
            } else {
                BrewNotification::Never
            };

            (ts, current_speaker, queue_result, brew_notification)
        };

        let Some(cached) = self.cached_setups.get(&call_id) else {
            return Err(GroupTransitionError::MissingCachedSetup(call_id));
        };
        let dest_addr = cached.dest_addr;

        if let Some(queue_result) = queue_result {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match queue_result {
                TxDemandQueueResult::FromCurrentSpeaker => {
                    tracing::trace!(
                        "FSM: U-TX DEMAND call_id={} from current speaker ISSI {}, ignoring duplicate",
                        call_id,
                        requesting_party.ssi
                    );
                }
                TxDemandQueueResult::Queued | TxDemandQueueResult::AlreadyQueuedBySameUser => {
                    // Non-pre-emptive: keep current speaker active, queue requester.
                    self.fsm_send_d_tx_granted_individual(
                        queue,
                        call_id,
                        requesting_party,
                        ts,
                        TransmissionGrant::RequestQueued,
                        Some(current_speaker),
                    );
                }
                TxDemandQueueResult::QueueBusy => {
                    self.fsm_send_d_tx_granted_individual(
                        queue,
                        call_id,
                        requesting_party,
                        ts,
                        TransmissionGrant::NotGranted,
                        Some(current_speaker),
                    );
                }
            }
            return Ok(());
        }

        // NoActiveSpeaker -> Transmitting transition with granted floor.
        self.fsm_send_d_tx_granted_individual(
            queue,
            call_id,
            requesting_party,
            ts,
            TransmissionGrant::Granted,
            Some(requesting_party.ssi),
        );
        self.send_d_tx_granted_facch(queue, call_id, requesting_party.ssi, dest_addr.ssi, ts);

        self.notify_floor_granted(
            queue,
            GroupFloorGrant {
                call_id,
                source_issi: requesting_party.ssi,
                dest_gssi: dest_addr.ssi,
                dest_is_group: true,
                ts,
            },
            true,
            brew_notification,
        );

        Ok(())
    }

    // Was: Führt den Arbeitsschritt `fsm_group_on_tx_ceased` für fsm Gruppe on tx ceased aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(in crate::cmce::subentities::cc_bs) fn fsm_group_on_tx_ceased(
        &mut self,
        queue: &mut MessageQueue,
        call_id: u16,
        sender: TetraAddress,
    ) -> Result<(), GroupTransitionError> {
        let (ts, queued_request, brew_notification) = {
            let Some(call) = self.active_calls.get_mut(&call_id) else {
                return Err(GroupTransitionError::UnknownCall(call_id));
            };

            let state = call.state();
            let formal_state = call.formal_state;
            Self::validate_group_transition(call_id, state, formal_state, GroupEvent::TxCeased)?;

            if !call.is_current_speaker(sender.ssi) {
                return Err(GroupTransitionError::NotCurrentSpeaker {
                    call_id,
                    sender_issi: sender.ssi,
                    current_speaker_issi: call.source_issi,
                });
            }

            let ts = call.ts;
            let queued_request = call.take_queued_tx_demand();
            let notify_source = queued_request.map_or(sender.ssi, |requester| requester.ssi);
            if let Some(requester) = queued_request {
                // Transmitting -> Transmitting, hand over floor directly to queued requester.
                call.grant_floor(requester.ssi, Some(requester));
            } else {
                // Transmitting -> NoActiveSpeaker.
                call.enter_hangtime(self.dltime);
            }
            let brew_notification = Self::brew_notification_for_group_call(call, notify_source);

            (ts, queued_request, brew_notification)
        };

        let Some(cached) = self.cached_setups.get(&call_id) else {
            return Err(GroupTransitionError::MissingCachedSetup(call_id));
        };
        let dest_addr = cached.dest_addr;

        if let Some(requester) = queued_request {
            self.fsm_send_d_tx_granted_individual(queue, call_id, requester, ts, TransmissionGrant::Granted, Some(requester.ssi));
            self.send_d_tx_granted_facch(queue, call_id, requester.ssi, dest_addr.ssi, ts);

            self.notify_floor_granted(
                queue,
                GroupFloorGrant {
                    call_id,
                    source_issi: requester.ssi,
                    dest_gssi: dest_addr.ssi,
                    dest_is_group: true,
                    ts,
                },
                true,
                brew_notification,
            );
            return Ok(());
        }

        let d_tx_ceased = DTxCeased {
            call_identifier: call_id,
            transmission_request_permission: false,
            notification_indicator: None,
            facility: None,
            dm_ms_address: None,
            proprietary: None,
        };
        tracing::info!("FSM -> {:?}", d_tx_ceased);
        let mut sdu = BitBuffer::new_autoexpand(25);
        d_tx_ceased.to_bitbuf(&mut sdu).expect("Failed to serialize DTxCeased");
        sdu.seek(0);

        let msg = Self::build_sapmsg_stealing(sdu.clone(), self.dltime, dest_addr, ts, None);
        queue.push_back(msg);

        // Sepura terminals repeat U-TX-CEASED when the group-addressed FACCH confirmation is
        // lost or when they have already switched their receive filter back to the individual
        // identity. Send the same confirmation directly to the current speaker as well. The
        // duplicate is harmless for other terminals and makes floor release deterministic even
        // when the PHY has to skip a late TX block.
        let direct_msg = Self::build_sapmsg_stealing(sdu, self.dltime, sender, ts, None);
        queue.push_back(direct_msg);

        self.notify_floor_released(queue, CallTimeslot { call_id, ts }, true, brew_notification);

        // A configured zero hangtime means a classic one-PTT group call: release the complete
        // call immediately after the floor is returned instead of leaving the radio in
        // NoActiveSpeaker for another scheduler cycle. This is also the default because several
        // Sepura generations otherwise request service restoration after the delayed release.
        if self.config.config().cell.hangtime_secs == 0 {
            tracing::info!("CMCE: zero group hangtime, releasing call_id={} immediately after U-TX-CEASED", call_id);
            self.release_group_call(queue, call_id, DisconnectCause::SwmiRequestedDisconnection);
        }

        Ok(())
    }

    // Was: Führt den Arbeitsschritt `fsm_group_on_network_call_start` für fsm Gruppe on network Ruf start aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(in crate::cmce::subentities::cc_bs) fn fsm_group_on_network_call_start(
        &mut self,
        queue: &mut MessageQueue,
        call_id: u16,
        network_entity: TetraEntity,
        brew_uuid: uuid::Uuid,
        source_issi: u32,
    ) -> Result<(), GroupTransitionError> {
        let Some(call) = self.active_calls.get_mut(&call_id) else {
            return Err(GroupTransitionError::UnknownCall(call_id));
        };

        let state = call.state();
        let formal_state = call.formal_state;
        Self::validate_group_transition(call_id, state, formal_state, GroupEvent::NetworkCallStart)?;

        call.grant_floor(source_issi, None);
        call.brew_uuid = Some(brew_uuid);
        if let CallOrigin::Network {
            network_entity: old_entity,
            brew_uuid: old_uuid,
        } = &call.origin
            && (*old_uuid != brew_uuid || *old_entity != network_entity)
        {
            tracing::warn!("CMCE FSM: network call start changed brew_uuid call_id={}", call_id);
            call.origin = CallOrigin::Network { network_entity, brew_uuid };
        }

        let ts = call.ts;
        let usage = call.usage;
        let dest_gssi = call.dest_gssi;

        self.send_d_tx_granted_facch(queue, call_id, source_issi, dest_gssi, ts);

        self.notify_remote_floor_granted(queue, CallTimeslot { call_id, ts });

        queue.push_back(SapMsg {
            sap: Sap::Control,
            src: TetraEntity::Cmce,
            dest: network_entity,
            msg: SapMsgInner::CmceCallControl(CallControl::NetworkCallReady {
                brew_uuid,
                call_id,
                ts,
                usage,
            }),
        });

        Ok(())
    }

    // Was: Führt den Arbeitsschritt `fsm_group_on_network_call_end` für fsm Gruppe on network Ruf end aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(in crate::cmce::subentities::cc_bs) fn fsm_group_on_network_call_end(
        &mut self,
        queue: &mut MessageQueue,
        call_id: u16,
    ) -> Result<(), GroupTransitionError> {
        let Some(call) = self.active_calls.get(&call_id).cloned() else {
            return Err(GroupTransitionError::UnknownCall(call_id));
        };

        let state = call.state();
        Self::validate_group_transition(call_id, state, call.formal_state, GroupEvent::NetworkCallEnd)?;

        if matches!(state, GroupCallState::Transmitting) {
            if let Some(active_call) = self.active_calls.get_mut(&call_id) {
                active_call.enter_hangtime(self.dltime);
                active_call.brew_uuid = None;
            }

            self.send_d_tx_ceased_facch(queue, call_id, call.dest_gssi, call.ts);
            self.notify_floor_released(queue, CallTimeslot { call_id, ts: call.ts }, true, BrewNotification::Never);
            return Ok(());
        }

        self.release_group_call(queue, call_id, DisconnectCause::SwmiRequestedDisconnection);
        Ok(())
    }
}
