// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use super::*;

/// Local bridge entry points for ISI-style interworking.
///
/// EN 300 392-3-2 models individual call interworking as ANF-ISIIC, while
/// EN 300 392-3-3 models group call interworking as ANF-ISIGC. The current
/// Brew transport is not PSS1/ROSE ISI, but these handlers keep the network
/// side at the CC boundary instead of spreading it through CMCE PC routing.
// Was: Implementiert das zugehörige Verhalten für `CcBsSubentity`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl CcBsSubentity {
    // Was: Diese Funktion sucht Brew-Verbindung individual Ruf.
    // Warum: Die Suchlogik bleibt damit wiederverwendbar und muss nicht an mehreren Stellen kopiert werden.
    pub(in crate::cmce::subentities::cc_bs) fn find_brew_individual_call(&self, brew_uuid: uuid::Uuid) -> Option<(u16, IndividualCall)> {
        self.individual_calls
            .iter()
            .find(|(_, c)| (c.called_over_brew || c.calling_over_brew) && c.brew_uuid == Some(brew_uuid))
            .map(|(id, call)| (*id, call.clone()))
    }

    // Was: Führt den Arbeitsschritt `rx_network_circuit_setup_request` für rx network circuit setup request aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn rx_network_circuit_setup_request(
        &mut self,
        queue: &mut MessageQueue,
        network_entity: TetraEntity,
        brew_uuid: uuid::Uuid,
        call: NetworkCircuitCall,
    ) {
        self.fsm_on_network_circuit_setup_request(queue, network_entity, brew_uuid, call);
    }

    // Was: Führt den Arbeitsschritt `rx_network_circuit_setup_accept` für rx network circuit setup accept aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn rx_network_circuit_setup_accept(&mut self, brew_uuid: uuid::Uuid) {
        if let Some((_, call)) = self.find_brew_individual_call(brew_uuid) {
            tracing::info!("CMCE: {:?} setup accepted uuid={}", call.network_entity(), brew_uuid);
        } else {
            tracing::debug!("CMCE: network setup accept for unknown uuid={}", brew_uuid);
        }
    }

    // Was: Führt den Arbeitsschritt `rx_network_circuit_setup_reject` für rx network circuit setup reject aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn rx_network_circuit_setup_reject(&mut self, queue: &mut MessageQueue, brew_uuid: uuid::Uuid, cause: u8) {
        let Some((call_id, call)) = self.find_brew_individual_call(brew_uuid) else {
            tracing::debug!("CMCE: network setup reject for unknown uuid={} cause={}", brew_uuid, cause);
            return;
        };
        let mapped = DisconnectCause::try_from(cause as u64).unwrap_or(DisconnectCause::RequestedServiceNotAvailable);
        tracing::info!(
            "CMCE: {:?} setup rejected uuid={} call_id={} cause={} ({:?})",
            call.network_entity(),
            brew_uuid,
            call_id,
            cause,
            mapped
        );
        self.release_individual_call(queue, call_id, mapped);
    }

    // Was: Führt den Arbeitsschritt `rx_network_circuit_alert` für rx network circuit alert aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn rx_network_circuit_alert(&mut self, queue: &mut MessageQueue, brew_uuid: uuid::Uuid) {
        let Some((call_id, call)) = self.find_brew_individual_call(brew_uuid) else {
            tracing::debug!("CMCE: network alert for unknown uuid={}", brew_uuid);
            return;
        };
        let network_entity = call.network_entity();

        if let Err(err) = self.fsm_individual_on_alert(queue, call_id, None, CallTimeoutSetupPhase::T60s) {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match err {
                IndividualTransitionError::UnknownCall(_) => {
                    tracing::debug!(
                        "CMCE: {:?} alert for unknown call_id={} uuid={}",
                        network_entity,
                        call_id,
                        brew_uuid
                    );
                }
                IndividualTransitionError::InvalidTransition { state, .. } => {
                    tracing::trace!(
                        "CMCE: {:?} alert ignored call_id={} uuid={} invalid from state {:?}",
                        network_entity,
                        call_id,
                        brew_uuid,
                        state
                    );
                }
                IndividualTransitionError::MissingBrewUuid(_) => {
                    tracing::warn!("CMCE: {:?} alert missing brew_uuid on call_id={}", network_entity, call_id);
                }
                IndividualTransitionError::DuplicateCall(_)
                | IndividualTransitionError::NotBrewOriginated(_)
                | IndividualTransitionError::ConnectRequestAlreadySent(_) => {}
            }
        }
    }

    // Was: Führt den Arbeitsschritt `rx_network_circuit_connect_request` für rx network circuit connect request aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn rx_network_circuit_connect_request(
        &mut self,
        queue: &mut MessageQueue,
        brew_uuid: uuid::Uuid,
        call_info: NetworkCircuitCall,
    ) {
        self.fsm_on_network_circuit_connect_request(queue, brew_uuid, call_info);
    }

    // Was: Führt den Arbeitsschritt `rx_network_circuit_connect_confirm` für rx network circuit connect confirm aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn rx_network_circuit_connect_confirm(
        &mut self,
        queue: &mut MessageQueue,
        brew_uuid: uuid::Uuid,
        grant: u8,
        permission: u8,
    ) {
        self.fsm_on_network_circuit_connect_confirm(queue, brew_uuid, grant, permission);
    }

    // Was: Führt den Arbeitsschritt `rx_network_circuit_simplex_granted` für rx network circuit simplex granted aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn rx_network_circuit_simplex_granted(
        &mut self,
        queue: &mut MessageQueue,
        brew_uuid: uuid::Uuid,
        grant: u8,
        permission: u8,
    ) {
        self.fsm_on_network_circuit_simplex_granted(queue, brew_uuid, grant, permission);
    }

    // Was: Führt den Arbeitsschritt `rx_network_circuit_simplex_idle` für rx network circuit simplex idle aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn rx_network_circuit_simplex_idle(&mut self, queue: &mut MessageQueue, brew_uuid: uuid::Uuid, grant: u8, permission: u8) {
        self.fsm_on_network_circuit_simplex_idle(queue, brew_uuid, grant, permission);
    }

    // Was: Führt den Arbeitsschritt `rx_network_circuit_release` für rx network circuit release aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn rx_network_circuit_release(&mut self, queue: &mut MessageQueue, brew_uuid: uuid::Uuid, cause: u8) {
        let Some((call_id, call)) = self.find_brew_individual_call(brew_uuid) else {
            tracing::debug!("CMCE: network release for unknown uuid={} cause={}", brew_uuid, cause);
            return;
        };
        let mapped = if cause == 0 {
            DisconnectCause::UserRequestedDisconnection
        } else {
            DisconnectCause::try_from(cause as u64).unwrap_or(DisconnectCause::SwmiRequestedDisconnection)
        };
        tracing::info!(
            "CMCE: {:?} release uuid={} call_id={} cause={} ({:?})",
            call.network_entity(),
            brew_uuid,
            call_id,
            cause,
            mapped
        );
        self.release_individual_call(queue, call_id, mapped);
    }

    /// Handle network-initiated group call start
    // Was: Führt den Arbeitsschritt `rx_network_call_start` für rx network Ruf start aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn rx_network_call_start(
        &mut self,
        queue: &mut MessageQueue,
        network_entity: TetraEntity,
        brew_uuid: uuid::Uuid,
        source_issi: u32,
        dest_gssi: u32,
        priority: u8,
    ) {
        self.fsm_on_network_call_start(queue, network_entity, brew_uuid, source_issi, dest_gssi, priority);
    }

    /// Handle network call end request
    // Was: Führt den Arbeitsschritt `rx_network_call_end` für rx network Ruf end aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn rx_network_call_end(&mut self, queue: &mut MessageQueue, brew_uuid: uuid::Uuid) {
        // Find the call by brew_uuid field (works for both Local and Network origin calls)
        let Some((call_id, call)) = self
            .active_calls
            .iter()
            .find(|(_, c)| c.brew_uuid == Some(brew_uuid))
            .map(|(id, c)| (*id, c.clone()))
        else {
            tracing::debug!("CMCE: network call end for unknown brew_uuid={}", brew_uuid);
            return;
        };

        tracing::info!(
            "CMCE: network call ended brew_uuid={} call_id={} gssi={}",
            brew_uuid,
            call_id,
            call.dest_gssi
        );

        if let Err(err) = self.fsm_group_on_network_call_end(queue, call_id) {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match err {
                GroupTransitionError::UnknownCall(_) => {
                    tracing::debug!("CMCE: network call end for unknown call_id={} brew_uuid={}", call_id, brew_uuid);
                }
                GroupTransitionError::InvalidTransition { state, .. } => {
                    tracing::warn!("CMCE: network call end rejected call_id={} from state {:?}", call_id, state);
                }
                GroupTransitionError::NotCurrentSpeaker { .. } => {
                    tracing::debug!(
                        "CMCE: network call end produced unexpected NotCurrentSpeaker for call_id={}",
                        call_id
                    );
                }
                GroupTransitionError::MissingCachedSetup(_) => {
                    tracing::debug!("CMCE: network call end call_id={} missing cached setup", call_id);
                }
            }
        }
    }
}
