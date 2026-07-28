// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für Gruppe floor grant in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub(super) struct GroupFloorGrant {
    pub(super) call_id: u16,
    pub(super) source_issi: u32,
    pub(super) dest_gssi: u32,
    pub(super) dest_is_group: bool,
    pub(super) ts: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für Ruf timeslot in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub(super) struct CallTimeslot {
    pub(super) call_id: u16,
    pub(super) ts: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für Brew-Verbindung notification auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub(super) enum BrewNotification {
    Never,
    ToEntityForLocalSource { entity: TetraEntity, source_issi: u32 },
    ForLocalSource { source_issi: u32, dest_gssi: u32 },
}

// Was: Implementiert das zugehörige Verhalten für `BrewNotification`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl BrewNotification {
    // Was: Führt den Arbeitsschritt `destination` für destination aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn destination(self, config: &SharedConfig) -> Option<TetraEntity> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            BrewNotification::Never => None,
            BrewNotification::ToEntityForLocalSource { entity, source_issi } => {
                if brew::is_active_for_entity(config, entity) && brew::is_brew_local_issi_allowed_for_entity(config, entity, source_issi) {
                    Some(entity)
                } else {
                    None
                }
            }
            BrewNotification::ForLocalSource { source_issi, dest_gssi } => {
                let entity = brew::route_entity_for_local_issi(config, source_issi)?;
                if brew::is_brew_gssi_routable_for_entity(config, entity, dest_gssi) {
                    Some(entity)
                } else {
                    None
                }
            }
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `CcBsSubentity`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl CcBsSubentity {
    // Was: Führt den Arbeitsschritt `brew_notification_for_group_call` für Brew-Verbindung notification for Gruppe Ruf aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn brew_notification_for_group_call(call: &ActiveCall, local_source_issi: u32) -> BrewNotification {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match &call.origin {
            CallOrigin::Network { network_entity, .. } => BrewNotification::ToEntityForLocalSource {
                entity: *network_entity,
                source_issi: local_source_issi,
            },
            CallOrigin::Local { .. } => BrewNotification::ForLocalSource {
                source_issi: local_source_issi,
                dest_gssi: call.dest_gssi,
            },
        }
    }

    // Was: Diese Funktion legt Steuerung.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn push_control(queue: &mut MessageQueue, dest: TetraEntity, control: CallControl) {
        queue.push_back(SapMsg {
            sap: Sap::Control,
            src: TetraEntity::Cmce,
            dest,
            msg: SapMsgInner::CmceCallControl(control),
        });
    }

    // Was: Diese Funktion meldet floor granted.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn notify_floor_granted(
        &self,
        queue: &mut MessageQueue,
        grant: GroupFloorGrant,
        notify_umac: bool,
        notify_brew: BrewNotification,
    ) {
        if notify_umac {
            Self::push_control(
                queue,
                TetraEntity::Umac,
                CallControl::FloorGranted {
                    call_id: grant.call_id,
                    source_issi: grant.source_issi,
                    dest_gssi: grant.dest_gssi,
                    dest_is_group: grant.dest_is_group,
                    ts: grant.ts,
                },
            );
        }

        #[cfg(feature = "recording")]
        if self.config.config().recording.enabled {
            Self::push_control(
                queue,
                TetraEntity::Recorder,
                CallControl::FloorGranted {
                    call_id: grant.call_id,
                    source_issi: grant.source_issi,
                    dest_gssi: grant.dest_gssi,
                    dest_is_group: grant.dest_is_group,
                    ts: grant.ts,
                },
            );
        }

        if let Some(brew_entity) = notify_brew.destination(&self.config) {
            Self::push_control(
                queue,
                brew_entity,
                CallControl::FloorGranted {
                    call_id: grant.call_id,
                    source_issi: grant.source_issi,
                    dest_gssi: grant.dest_gssi,
                    dest_is_group: grant.dest_is_group,
                    ts: grant.ts,
                },
            );
        }
    }

    // Was: Diese Funktion meldet remote floor granted.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn notify_remote_floor_granted(&self, queue: &mut MessageQueue, slot: CallTimeslot) {
        Self::push_control(
            queue,
            TetraEntity::Umac,
            CallControl::RemoteFloorGranted {
                call_id: slot.call_id,
                ts: slot.ts,
            },
        );
    }

    // Was: Diese Funktion meldet floor released.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn notify_floor_released(
        &self,
        queue: &mut MessageQueue,
        slot: CallTimeslot,
        notify_umac: bool,
        notify_brew: BrewNotification,
    ) {
        if notify_umac {
            Self::push_control(
                queue,
                TetraEntity::Umac,
                CallControl::FloorReleased {
                    call_id: slot.call_id,
                    ts: slot.ts,
                },
            );
        }

        #[cfg(feature = "recording")]
        if self.config.config().recording.enabled {
            Self::push_control(
                queue,
                TetraEntity::Recorder,
                CallControl::FloorReleased {
                    call_id: slot.call_id,
                    ts: slot.ts,
                },
            );
        }

        if let Some(brew_entity) = notify_brew.destination(&self.config) {
            Self::push_control(
                queue,
                brew_entity,
                CallControl::FloorReleased {
                    call_id: slot.call_id,
                    ts: slot.ts,
                },
            );
        }
    }

    // Was: Diese Funktion meldet Ruf ended.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn notify_call_ended(&self, queue: &mut MessageQueue, slot: CallTimeslot, notify_umac: bool, notify_brew: BrewNotification) {
        if notify_umac {
            Self::push_control(
                queue,
                TetraEntity::Umac,
                CallControl::CallEnded {
                    call_id: slot.call_id,
                    ts: slot.ts,
                },
            );
        }

        #[cfg(feature = "recording")]
        if self.config.config().recording.enabled {
            Self::push_control(
                queue,
                TetraEntity::Recorder,
                CallControl::CallEnded {
                    call_id: slot.call_id,
                    ts: slot.ts,
                },
            );
        }

        if let Some(brew_entity) = notify_brew.destination(&self.config) {
            Self::push_control(
                queue,
                brew_entity,
                CallControl::CallEnded {
                    call_id: slot.call_id,
                    ts: slot.ts,
                },
            );
        }
    }

    // Was: Diese Funktion meldet network Ruf end.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn notify_network_call_end(&self, queue: &mut MessageQueue, network_entity: TetraEntity, brew_uuid: uuid::Uuid) {
        Self::push_control(queue, network_entity, CallControl::NetworkCallEnd { brew_uuid });
    }

    // Was: Diese Funktion meldet network circuit release.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn notify_network_circuit_release(
        &self,
        queue: &mut MessageQueue,
        network_entity: TetraEntity,
        brew_uuid: uuid::Uuid,
        cause: DisconnectCause,
    ) {
        Self::push_control(
            queue,
            network_entity,
            CallControl::NetworkCircuitRelease {
                brew_uuid,
                cause: cause.into_raw() as u8,
            },
        );
    }
}
