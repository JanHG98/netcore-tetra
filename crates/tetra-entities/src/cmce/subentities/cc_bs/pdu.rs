// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use tetra_core::Todo;
use super::*;

// Was: Implementiert das zugehörige Verhalten für `CcBsSubentity`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl CcBsSubentity {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(config: SharedConfig) -> Self {
        CcBsSubentity {
            config,
            dltime: TdmaTime::default(),
            cached_setups: HashMap::new(),
            circuits: CircuitMgr::new(),
            active_calls: HashMap::new(),
            individual_calls: HashMap::new(),
            subscriber_groups: HashMap::new(),
            group_listeners: HashMap::new(),
            recent_deaffiliations: HashMap::new(),
            call_restore: CallRestoreRuntime::new(),
            telemetry: None,
        }
    }

    // Was: Diese Funktion setzt Konfiguration.
    // Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
    pub fn set_config(&mut self, config: SharedConfig) {
        self.config = config;
    }

    /// Wire the dashboard telemetry sink so call-lifecycle events (Group/Individual
    /// CallStarted/CallEnded) reach the dashboard. Mirrors `SdsBsSubentity::set_telemetry`.
    // Was: Diese Funktion setzt Telemetrie.
    // Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
    pub fn set_telemetry(&mut self, sink: crate::net_telemetry::TelemetrySink) {
        self.telemetry = Some(sink);
    }

    /// Fire-and-forget emit of a telemetry event. No-op when telemetry is disabled.
    // Was: Diese Funktion gibt den vorgesehenen Arbeitsschritt.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn emit(&self, event: crate::net_telemetry::TelemetryEvent) {
        if let Some(sink) = &self.telemetry {
            sink.send(event);
        }
    }

    // Was: Prüft, ob locally registered Teilnehmerkennung (ISSI) zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub(super) fn is_locally_registered_issi(&self, issi: u32) -> bool {
        let cmce_known = self.subscriber_groups.contains_key(&issi);
        let registry_known = self.config.state_read().subscribers.is_registered(issi);

        if cmce_known != registry_known {
            tracing::warn!(
                "CMCE: subscriber registry mismatch issi={} cmce_known={} registry_known={}",
                issi,
                cmce_known,
                registry_known
            );
        }

        registry_known
    }

    // Was: Führt den Arbeitsschritt `known_local_issis` für known local issis aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn known_local_issis(&self) -> Vec<u32> {
        self.config.state_read().subscribers.all_registered_issis().collect()
    }

    #[inline]
    // Was: Führt den Arbeitsschritt `p2p_call_timeout` für p2p Ruf timeout aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn p2p_call_timeout(simplex_duplex: bool) -> CallTimeout {
        if simplex_duplex { CallTimeout::Infinite } else { CallTimeout::T5m }
    }

    /// Internal carrier hint used inside CMCE->UMAC `CmceChanAllocReq`.
    /// `Todo` is the signed carrier-hint type used by `CmceChanAllocReq`, while a real TETRA carrier number is never negative.
    /// UMAC resolves this sentinel to `[cell_info].secondary_carrier` at runtime.
    // Was: Legt den festen Wert `SECONDARY_CARRIER_HINT` für secondary carrier hint fest.
    // Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
    pub(super) const SECONDARY_CARRIER_HINT: Todo = -2;

    /// Logical bearer id to physical TETRA air-interface timeslot.
    /// Logical TS5..TS7 represent secondary-carrier physical TS2..TS4.
    // Was: Führt den Arbeitsschritt `air_ts` für air ts aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn air_ts(logical_ts: u8) -> u8 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match logical_ts {
            5..=7 => logical_ts - 3,
            _ => logical_ts,
        }
    }

    // Was: Prüft, ob secondary logical ts zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub(super) fn is_secondary_logical_ts(logical_ts: u8) -> bool {
        (5..=7).contains(&logical_ts)
    }

    // Was: Führt den Arbeitsschritt `carrier_hint_for_logical_ts` für carrier hint for logical ts aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn carrier_hint_for_logical_ts(logical_ts: u8) -> Option<Todo> {
        if Self::is_secondary_logical_ts(logical_ts) {
            Some(Self::SECONDARY_CARRIER_HINT)
        } else {
            None
        }
    }

    // Was: Führt den Arbeitsschritt `chan_alloc_for_ts` für chan alloc for ts aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn chan_alloc_for_ts(usage: Option<u8>, logical_ts: u8, alloc_type: ChanAllocType, ul_dl: UlDlAssignment) -> CmceChanAllocReq {
        let air_ts = Self::air_ts(logical_ts);
        let mut timeslots = [false; 4];
        if (1..=4).contains(&air_ts) {
            timeslots[air_ts as usize - 1] = true;
        } else {
            tracing::warn!("CMCE: invalid logical traffic ts {} while building channel allocation", logical_ts);
        }
        CmceChanAllocReq {
            usage,
            alloc_type,
            carrier: Self::carrier_hint_for_logical_ts(logical_ts),
            timeslots,
            ul_dl_assigned: ul_dl,
        }
    }

    // Was: Führt den Arbeitsschritt `traffic_slot_capacity` für Nutzdatenverkehr slot capacity aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn traffic_slot_capacity(&self) -> usize {
        if self.config.config().cell.secondary_carrier.is_some() {
            tetra_core::TimeslotAllocator::DUAL_CARRIER_TRAFFIC_SLOTS
        } else {
            tetra_core::TimeslotAllocator::SINGLE_CARRIER_TRAFFIC_SLOTS
        }
    }

    // Was: Führt den Arbeitsschritt `carrier_num_for_logical_ts` für carrier num for logical ts aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn carrier_num_for_logical_ts(&self, logical_ts: u8) -> u16 {
        if Self::is_secondary_logical_ts(logical_ts) {
            self.config
                .config()
                .cell
                .secondary_carrier
                .unwrap_or(self.config.config().cell.main_carrier)
        } else {
            self.config.config().cell.main_carrier
        }
    }

    // Was: Diese Funktion erstellt d setup prim.
    // Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
    pub(super) fn build_d_setup_prim(pdu: &DSetup, usage: u8, ts: u8, ul_dl: UlDlAssignment) -> (BitBuffer, CmceChanAllocReq) {
        tracing::debug!("-> {:?}", pdu);

        let mut sdu = BitBuffer::new_autoexpand(80);
        pdu.to_bitbuf(&mut sdu).expect("Failed to serialize DSetup");
        sdu.seek(0);

        let chan_alloc = Self::chan_alloc_for_ts(Some(usage), ts, ChanAllocType::Replace, ul_dl);
        (sdu, chan_alloc)
    }

    // Was: Diese Funktion erstellt sapmsg.
    // Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
    pub(super) fn build_sapmsg(
        sdu: BitBuffer,
        chan_alloc: Option<CmceChanAllocReq>,
        _dltime: TdmaTime,
        address: TetraAddress,
        reporter: Option<TxReporter>,
    ) -> SapMsg {
        // Construct prim
        SapMsg {
            sap: Sap::LcmcSap,
            src: TetraEntity::Cmce,
            dest: TetraEntity::Mle,
            msg: SapMsgInner::LcmcMleUnitdataReq(LcmcMleUnitdataReq {
                sdu,
                handle: 0,
                endpoint_id: 0,
                link_id: 0,
                // Unacknowledged BL-UDATA. This builder carries the MCCH/group-addressed sends —
                // D-SETUP and D-RELEASE to a GSSI have no single peer to ACK, so acknowledged LLC
                // (the `Todo` default) is wrong and can stall/retry at LLC. The legacy `main` code
                // sent every CC PDU here unacknowledged (FH FIX 2).
                layer2service: Layer2Service::Unacknowledged,
                pdu_prio: 0,
                layer2_qos: 0,
                stealing_permission: false,
                stealing_repeats_flag: false,
                chan_alloc,
                main_address: address,
                tx_reporter: reporter,
            }),
        }
    }

    /// Build an unacknowledged group-signalling primitive that must be transmitted
    /// on the next usable frame-18 common-SCCH slot instead of the ordinary MCCH.
    ///
    /// The delivery hint is carried only as an internal request-handle marker through
    /// MLE and LLC; it is consumed by UMAC and is never encoded on the air interface.
    // Was: Diese Funktion erstellt sapmsg frame18 common scch.
    // Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
    pub(super) fn build_sapmsg_frame18_common_scch(
        call_id: u16,
        sdu: BitBuffer,
        chan_alloc: Option<CmceChanAllocReq>,
        address: TetraAddress,
        reporter: Option<TxReporter>,
    ) -> SapMsg {
        let mut msg = Self::build_sapmsg(sdu, chan_alloc, TdmaTime::default(), address, reporter);
        if let SapMsgInner::LcmcMleUnitdataReq(ref mut prim) = msg.msg {
            prim.handle = tetra_saps::tma::make_frame18_common_scch_handle(call_id) as u32;
        }
        msg
    }

    // Was: Diese Funktion erstellt sapmsg direct.
    // Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
    pub(super) fn build_sapmsg_direct(
        sdu: BitBuffer,
        dltime: TdmaTime,
        address: TetraAddress,
        handle: u32,
        link_id: u32,
        endpoint_id: u32,
    ) -> SapMsg {
        Self::build_sapmsg_direct_with_allocation(
            sdu, dltime, address, handle, link_id, endpoint_id, None,
        )
    }

    // Was: Diese Funktion erstellt sapmsg direct with allocation.
    // Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
    pub(super) fn build_sapmsg_direct_with_allocation(
        sdu: BitBuffer,
        _dltime: TdmaTime,
        address: TetraAddress,
        handle: u32,
        link_id: u32,
        endpoint_id: u32,
        chan_alloc: Option<CmceChanAllocReq>,
    ) -> SapMsg {
        SapMsg {
            sap: Sap::LcmcSap,
            src: TetraEntity::Cmce,
            dest: TetraEntity::Mle,
            msg: SapMsgInner::LcmcMleUnitdataReq(LcmcMleUnitdataReq {
                sdu,
                handle,
                endpoint_id,
                link_id,
                // Unacknowledged BL-UDATA. This builder serves the direct/reject broadcast paths
                // (e.g. congestion D-RELEASE in `reject_setup_request`); the legacy `main` code
                // hardcoded these unacknowledged (FH FIX 2).
                layer2service: Layer2Service::Unacknowledged,
                pdu_prio: 0,
                layer2_qos: 0,
                stealing_permission: false,
                stealing_repeats_flag: false,
                chan_alloc,
                main_address: address,
                tx_reporter: None,
            }),
        }
    }

    // Was: Diese Funktion erstellt sapmsg stealing.
    // Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
    pub(super) fn build_sapmsg_stealing(sdu: BitBuffer, dltime: TdmaTime, address: TetraAddress, ts: u8, usage: Option<u8>) -> SapMsg {
        Self::build_sapmsg_stealing_ul_dl(sdu, dltime, address, ts, usage, UlDlAssignment::Both)
    }

    // Was: Diese Funktion erstellt sapmsg stealing ul dl.
    // Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
    pub(super) fn build_sapmsg_stealing_ul_dl(
        sdu: BitBuffer,
        _dltime: TdmaTime,
        address: TetraAddress,
        ts: u8,
        usage: Option<u8>,
        ul_dl_assigned: UlDlAssignment,
    ) -> SapMsg {
        // For FACCH stealing on a traffic channel, specify the physical air
        // timeslot plus a carrier hint when the logical bearer lives on Carrier 2.
        let chan_alloc = Self::chan_alloc_for_ts(usage, ts, ChanAllocType::Replace, ul_dl_assigned);

        SapMsg {
            sap: Sap::LcmcSap,
            src: TetraEntity::Cmce,
            dest: TetraEntity::Mle,
            msg: SapMsgInner::LcmcMleUnitdataReq(LcmcMleUnitdataReq {
                sdu,
                handle: 0,
                endpoint_id: 0,
                link_id: 0,
                // Unacknowledged BL-UDATA over FACCH stealing. Group floor PDUs (D-TX-CEASED /
                // D-SETUP late-entry re-sends) carried here are GSSI-addressed, so acknowledged
                // LLC would have no single peer to ACK; the legacy `main` code sent these
                // unacknowledged (FH FIX 2).
                layer2service: Layer2Service::Unacknowledged,
                pdu_prio: 0,
                layer2_qos: 0,
                stealing_permission: true,
                stealing_repeats_flag: false,
                chan_alloc: Some(chan_alloc),
                main_address: address,
                tx_reporter: None,
            }),
        }
    }

    // Was: Diese Funktion erstellt d release.
    // Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
    pub(super) fn build_d_release(call_identifier: u16, disconnect_cause: DisconnectCause) -> BitBuffer {
        let pdu = DRelease {
            call_identifier,
            disconnect_cause,
            notification_indicator: None,
            facility: None,
            proprietary: None,
        };
        tracing::info!("-> {:?}", pdu);

        let mut sdu = BitBuffer::new_autoexpand(32);
        pdu.to_bitbuf(&mut sdu).expect("Failed to serialize DRelease");
        sdu.seek(0);
        sdu
    }

    // Was: Diese Funktion erstellt d release from d setup.
    // Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
    pub(super) fn build_d_release_from_d_setup(d_setup_pdu: &DSetup, disconnect_cause: DisconnectCause) -> BitBuffer {
        Self::build_d_release(d_setup_pdu.call_identifier, disconnect_cause)
    }

    // Was: Diese Funktion erstellt d disconnect.
    // Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
    pub(super) fn build_d_disconnect(call_identifier: u16, disconnect_cause: DisconnectCause) -> BitBuffer {
        let pdu = DDisconnect {
            call_identifier,
            disconnect_cause,
            notification_indicator: None,
            facility: None,
            proprietary: None,
        };
        tracing::info!("-> {:?}", pdu);

        let mut sdu = BitBuffer::new_autoexpand(32);
        pdu.to_bitbuf(&mut sdu).expect("Failed to serialize DDisconnect");
        sdu.seek(0);
        sdu
    }

    // Was: Diese Funktion erstellt d disconnect from d setup.
    // Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
    pub(super) fn build_d_disconnect_from_d_setup(d_setup_pdu: &DSetup, disconnect_cause: DisconnectCause) -> BitBuffer {
        Self::build_d_disconnect(d_setup_pdu.call_identifier, disconnect_cause)
    }

    // Was: Diese Funktion erstellt d Ruf Wiederherstellung.
    // Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
    pub(super) fn build_d_call_restore(
        call_identifier: u16,
        transmission_grant: TransmissionGrant,
        call_status: Option<CallStatus>,
    ) -> BitBuffer {
        Self::build_d_call_restore_extended(
            call_identifier,
            transmission_grant,
            None,
            None,
            call_status,
        )
    }

    // Was: Diese Funktion erstellt d Ruf Wiederherstellung extended.
    // Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
    pub(super) fn build_d_call_restore_extended(
        call_identifier: u16,
        transmission_grant: TransmissionGrant,
        new_call_identifier: Option<u16>,
        call_time_out: Option<CallTimeout>,
        call_status: Option<CallStatus>,
    ) -> BitBuffer {
        let pdu = DCallRestore {
            call_identifier,
            transmission_grant: transmission_grant.into_raw() as u8,
            transmission_request_permission: false,
            // T310 continues across call restoration unless the SwMI explicitly
            // supplies a replacement timeout value.
            reset_call_time_out_timer_t310_: call_time_out.is_some(),
            new_call_identifier: new_call_identifier.map(u64::from),
            call_time_out: call_time_out.map(CallTimeout::into_raw),
            call_status: call_status.map(CallStatus::into_raw),
            modify: None,
            notification_indicator: None,
            facility: None,
            temporary_address: None,
            dm_ms_address: None,
            proprietary: None,
        };
        tracing::info!("-> {:?}", pdu);

        let mut sdu = BitBuffer::new_autoexpand(64);
        pdu.to_bitbuf(&mut sdu).expect("Failed to serialize DCallRestore");
        sdu.seek(0);
        sdu
    }

    // Was: Diese Funktion erstellt d info.
    // Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
    pub(super) fn build_d_info(call_identifier: u16, modify: Option<u64>, call_status: Option<CallStatus>, reset_t310: bool) -> BitBuffer {
        let pdu = DInfo {
            call_identifier,
            reset_call_time_out_timer_t310_: reset_t310,
            poll_request: false,
            new_call_identifier: None,
            call_time_out: None,
            call_time_out_set_up_phase_t301_t302_: None,
            call_ownership: None,
            modify,
            call_status: call_status.map(CallStatus::into_raw),
            temporary_address: None,
            notification_indicator: None,
            poll_response_percentage: None,
            poll_response_number: None,
            dtmf: None,
            facility: None,
            poll_response_addresses: None,
            proprietary: None,
        };
        tracing::info!("-> {:?}", pdu);

        let mut sdu = BitBuffer::new_autoexpand(64);
        pdu.to_bitbuf(&mut sdu).expect("Failed to serialize DInfo");
        sdu.seek(0);
        sdu
    }

    // Was: Prüft, ob listener zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub(super) fn has_listener(&self, gssi: u32) -> bool {
        self.group_listener_count(gssi) > 0 || self.has_recent_deaffiliation_listener(gssi)
    }

    // Was: Führt den Arbeitsschritt `group_listener_count` für Gruppe listener count aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn group_listener_count(&self, gssi: u32) -> usize {
        self.group_listeners.get(&gssi).copied().unwrap_or(0)
    }

    // Was: Prüft, ob recent deaffiliation listener zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    fn has_recent_deaffiliation_listener(&self, gssi: u32) -> bool {
        self.recent_deaffiliations
            .iter()
            .any(|((_, grace_gssi), at)| *grace_gssi == gssi && at.age(self.dltime) <= BREW_AFFILIATION_GRACE_TS)
    }

    // Was: Führt den Arbeitsschritt `note_recent_deaffiliation` für note recent deaffiliation aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn note_recent_deaffiliation(&mut self, issi: u32, gssi: u32) {
        self.recent_deaffiliations.insert((issi, gssi), self.dltime);
    }

    // Was: Diese Funktion leert recent deaffiliation.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn clear_recent_deaffiliation(&mut self, issi: u32, gssi: u32) {
        self.recent_deaffiliations.remove(&(issi, gssi));
    }

    // Was: Führt den Arbeitsschritt `inc_group_listener` für inc Gruppe listener aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn inc_group_listener(&mut self, gssi: u32) {
        let entry = self.group_listeners.entry(gssi).or_insert(0);
        *entry += 1;
    }

    // Was: Führt den Arbeitsschritt `dec_group_listener` für dec Gruppe listener aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn dec_group_listener(&mut self, gssi: u32) {
        if let Some(entry) = self.group_listeners.get_mut(&gssi) {
            if *entry <= 1 {
                self.group_listeners.remove(&gssi);
            } else {
                *entry -= 1;
            }
        }
    }

    // ── Dashboard / API helpers ────────────────────────────────────────────────

    /// Returns all currently registered ISSI values.
    // Was: Führt den Arbeitsschritt `subscriber_issis` für Teilnehmer issis aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn subscriber_issis(&self) -> Vec<u32> {
        self.subscriber_groups.keys().copied().collect()
    }

    /// Returns the list of GSSIs the given ISSI is affiliated to.
    // Was: Führt den Arbeitsschritt `subscriber_groups_for` für Teilnehmer groups for aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn subscriber_groups_for(&self, issi: u32) -> Vec<u32> {
        self.subscriber_groups
            .get(&issi)
            .map(|s| s.iter().copied().collect())
            .unwrap_or_default()
    }

    /// Force-deregister an MS: release its active calls and clean up state.
    /// Returns true if the MS was known.
    // Was: Führt den Arbeitsschritt `kick_ms` für kick ms aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn kick_ms(&mut self, queue: &mut MessageQueue, issi: u32) -> bool {
        if !self.subscriber_groups.contains_key(&issi) {
            tracing::warn!("CMCE: kick_ms issi={} not found in subscriber_groups", issi);
            return false;
        }
        // Release all active individual calls involving this MS
        let individual_ids: Vec<u16> = self
            .individual_calls
            .iter()
            .filter(|(_, c)| c.calling_addr.ssi == issi || c.called_addr.ssi == issi)
            .map(|(&id, _)| id)
            .collect();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for id in individual_ids {
            self.release_individual_call(queue, id, DisconnectCause::UserRequestedDisconnection);
        }
        // Clean up CMCE state
        if let Some(groups) = self.subscriber_groups.remove(&issi) {
            // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
            // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
            for g in &groups {
                self.dec_group_listener(*g);
                self.clear_recent_deaffiliation(issi, *g);
            }
        }
        self.recent_deaffiliations.retain(|(grace_issi, _), _| *grace_issi != issi);
        // Tell MM to deregister the MS — this also notifies Brew
        queue.push_back(SapMsg {
            sap: Sap::Control,
            src: TetraEntity::Cmce,
            dest: TetraEntity::Mm,
            msg: SapMsgInner::MmSubscriberUpdate(MmSubscriberUpdate {
                issi,
                groups: Vec::new(),
                action: BrewSubscriberAction::Deregister,
            }),
        });
        tracing::info!("CMCE: kick_ms issi={} — deregistered", issi);
        true
    }

    // Was: Diese Funktion sucht individual Ruf by Teilnehmerkennung (ISSI).
    // Warum: Die Suchlogik bleibt damit wiederverwendbar und muss nicht an mehreren Stellen kopiert werden.
    pub(super) fn find_individual_call_by_issi(&self, issi: u32) -> Option<(u16, IndividualCallState)> {
        self.individual_calls
            .iter()
            .find(|(_, call)| call.calling_addr.ssi == issi || call.called_addr.ssi == issi)
            .map(|(call_id, call)| (*call_id, call.state))
    }

    // Was: Führt den Arbeitsschritt `drop_group_calls_if_unlistened` für drop Gruppe calls if unlistened aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn drop_group_calls_if_unlistened(&mut self, queue: &mut MessageQueue, gssi: u32) {
        if self.group_listener_count(gssi) > 0 {
            return;
        }
        if self.has_recent_deaffiliation_listener(gssi) {
            tracing::debug!(
                "CMCE: deferring unlistened-call drop for gssi={} during Brew affiliation grace",
                gssi
            );
            return;
        }

        let to_drop: Vec<(u16, CallOrigin)> = self
            .active_calls
            .iter()
            .filter(|(_, call)| call.dest_gssi == gssi)
            .map(|(call_id, call)| (*call_id, call.origin.clone()))
            .collect();

        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for (call_id, origin) in to_drop {
            tracing::info!("CMCE: dropping call_id={} gssi={} (no listeners)", call_id, gssi);
            if let CallOrigin::Network { network_entity, brew_uuid } = origin {
                self.notify_network_call_end(queue, network_entity, brew_uuid);
            };
            self.release_group_call(queue, call_id, DisconnectCause::SwmiRequestedDisconnection);
        }
    }

    // Was: Führt den Arbeitsschritt `expire_brew_affiliation_grace` für expire Brew-Verbindung affiliation grace aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn expire_brew_affiliation_grace(&mut self, queue: &mut MessageQueue) {
        if self.recent_deaffiliations.is_empty() {
            return;
        }

        let expired: Vec<(u32, u32)> = self
            .recent_deaffiliations
            .iter()
            .filter_map(|(key, at)| {
                if at.age(self.dltime) > BREW_AFFILIATION_GRACE_TS {
                    Some(*key)
                } else {
                    None
                }
            })
            .collect();
        if expired.is_empty() {
            return;
        }

        let mut affected_gssis = HashSet::new();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for key @ (_, gssi) in expired {
            self.recent_deaffiliations.remove(&key);
            affected_gssis.insert(gssi);
        }

        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for gssi in affected_gssis {
            self.drop_group_calls_if_unlistened(queue, gssi);
        }
    }

    // Was: Führt den Arbeitsschritt `reannounce_network_calls_after_affiliation` für reannounce network calls after affiliation aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn reannounce_network_calls_after_affiliation(
        &self,
        queue: &mut MessageQueue,
        issi: u32,
        groups: &[u32],
    ) {
        if groups.is_empty() {
            return;
        }

        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for (call_id, call) in &self.active_calls {
            if !call.is_tx_active()
                || !matches!(&call.origin, CallOrigin::Network { .. })
                || !groups.contains(&call.dest_gssi)
            {
                continue;
            }

            let Some(cached) = self.cached_setups.get(call_id) else {
                continue;
            };
            let (sdu, chan_alloc) =
                Self::build_d_setup_prim(&cached.pdu, call.usage, call.ts, UlDlAssignment::Both);
            let prim = Self::build_sapmsg(sdu, Some(chan_alloc), self.dltime, cached.dest_addr, None);
            queue.push_back(prim);

            tracing::info!(
                "CMCE: re-announcing active network call after affiliation refresh issi={} call_id={} gssi={} priority={}",
                issi,
                call_id,
                call.dest_gssi,
                cached.pdu.call_priority
            );
        }
    }

    // Was: Diese Funktion verarbeitet Teilnehmer update.
    // Warum: Die Reaktion auf dieses Ereignis bleibt damit an einer Stelle nachvollziehbar.
    pub fn handle_subscriber_update(&mut self, queue: &mut MessageQueue, update: MmSubscriberUpdate) {
        let issi = update.issi;
        let groups = update.groups;

        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match update.action {
            BrewSubscriberAction::Register => {
                let known = self.subscriber_groups.contains_key(&issi);
                self.subscriber_groups.entry(issi).or_insert_with(HashSet::new);
                tracing::debug!("CMCE: subscriber register issi={} known={}", issi, known);
            }
            BrewSubscriberAction::Deregister => {
                if let Some(existing) = self.subscriber_groups.remove(&issi) {
                    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
                    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
                    for gssi in existing {
                        self.dec_group_listener(gssi);
                        self.clear_recent_deaffiliation(issi, gssi);
                        self.drop_group_calls_if_unlistened(queue, gssi);
                    }
                }
                self.recent_deaffiliations.retain(|(grace_issi, _), _| *grace_issi != issi);
                tracing::debug!("CMCE: subscriber deregister issi={}", issi);
            }
            BrewSubscriberAction::Affiliate => {
                let reported_groups = groups;
                let mut new_groups = Vec::new();
                {
                    let entry = self.subscriber_groups.entry(issi).or_insert_with(HashSet::new);
                    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
                    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
                    for &gssi in &reported_groups {
                        if entry.insert(gssi) {
                            new_groups.push(gssi);
                        }
                    }
                }
                // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
                // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
                for gssi in &new_groups {
                    self.clear_recent_deaffiliation(issi, *gssi);
                    self.inc_group_listener(*gssi);
                }

                if new_groups.is_empty() {
                    tracing::debug!(
                        "CMCE: affiliate refresh (no new groups) issi={} groups={:?}",
                        issi,
                        reported_groups
                    );
                } else {
                    tracing::info!("CMCE: subscriber affiliate issi={} groups={:?}", issi, new_groups);
                }

                // A RoamingLocationUpdating/ITSI re-attach can temporarily pull the terminal back
                // to control-channel signalling while a network-originated group call is active.
                // Re-announce each matching active network call immediately after the group state
                // is restored, so the radio receives a fresh channel assignment instead of waiting
                // up to five seconds for normal late-entry paging.
                self.reannounce_network_calls_after_affiliation(queue, issi, &reported_groups);
            }
            BrewSubscriberAction::Deaffiliate => {
                let mut removed_groups = Vec::new();
                let mut known_issi = false;
                if let Some(entry) = self.subscriber_groups.get_mut(&issi) {
                    known_issi = true;
                    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
                    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
                    for gssi in groups {
                        if entry.remove(&gssi) {
                            removed_groups.push(gssi);
                        }
                    }
                } else {
                    removed_groups = groups;
                }
                if known_issi {
                    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
                    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
                    for gssi in &removed_groups {
                        self.dec_group_listener(*gssi);
                        self.note_recent_deaffiliation(issi, *gssi);
                    }
                }

                if removed_groups.is_empty() {
                    tracing::debug!("CMCE: deaffiliate ignored (no matching groups) issi={}", issi);
                } else {
                    tracing::info!("CMCE: subscriber deaffiliate issi={} groups={:?}", issi, removed_groups);
                    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
                    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
                    for gssi in &removed_groups {
                        self.drop_group_calls_if_unlistened(queue, *gssi);
                    }
                }
            }
        }
    }

    // Was: Diese Funktion sendet d Ruf proceeding.
    // Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
    pub(super) fn send_d_call_proceeding(
        &mut self,
        queue: &mut MessageQueue,
        message: &SapMsg,
        pdu_request: &USetup,
        call_id: u16,
        setup_timeout: CallTimeoutSetupPhase,
        hook_method_selection: bool,
    ) {
        tracing::trace!("send_d_call_proceeding");

        let SapMsgInner::LcmcMleUnitdataInd(prim) = &message.msg else {
            panic!()
        };

        let pdu_response = DCallProceeding {
            call_identifier: call_id,
            call_time_out_set_up_phase: setup_timeout,
            hook_method_selection,
            simplex_duplex_selection: pdu_request.simplex_duplex_selection,
            basic_service_information: None, // Only needed if different from requested
            call_status: None,
            notification_indicator: None,
            facility: None,
            proprietary: None,
        };

        let mut sdu = BitBuffer::new_autoexpand(25);
        pdu_response.to_bitbuf(&mut sdu).expect("Failed to serialize DCallProceeding");
        sdu.seek(0);
        tracing::debug!("send_d_call_proceeding: -> {:?} sdu {}", pdu_response, sdu.dump_bin());

        let msg = SapMsg {
            sap: Sap::LcmcSap,
            src: TetraEntity::Cmce,
            dest: TetraEntity::Mle,
            msg: SapMsgInner::LcmcMleUnitdataReq(LcmcMleUnitdataReq {
                sdu,
                handle: prim.handle,
                endpoint_id: prim.endpoint_id,
                link_id: prim.link_id,
                // D-CALL-PROCEEDING during setup: the legacy `main` code sent this unacknowledged
                // (FH FIX 2). It is a setup-phase MCCH response where the addressed MS is not yet
                // in a confirmed LLC link context, so acknowledged BL-DATA can stall.
                layer2service: Layer2Service::Unacknowledged,
                pdu_prio: 0,
                layer2_qos: 0,
                stealing_permission: false,
                stealing_repeats_flag: false,
                chan_alloc: None,
                main_address: prim.received_tetra_address,
                tx_reporter: None,
            }),
        };
        queue.push_back(msg);
    }

    // Was: Diese Funktion sendet d alert individual.
    // Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
    pub(super) fn send_d_alert_individual(
        &mut self,
        queue: &mut MessageQueue,
        _dltime: TdmaTime,
        call_id: u16,
        simplex_duplex: bool,
        calling_addr: TetraAddress,
        calling_handle: u32,
        calling_link_id: u32,
        calling_endpoint_id: u32,
        setup_timeout: CallTimeoutSetupPhase,
    ) {
        let d_alert = DAlert {
            call_identifier: call_id,
            call_time_out_set_up_phase: setup_timeout.into_raw() as u8,
            reserved: true, // per spec note: set to 1 for backwards compatibility
            simplex_duplex_selection: simplex_duplex,
            call_queued: false,
            basic_service_information: None,
            notification_indicator: None,
            facility: None,
            proprietary: None,
        };

        tracing::info!("-> {:?}", d_alert);
        let mut sdu = BitBuffer::new_autoexpand(32);
        d_alert.to_bitbuf(&mut sdu).expect("Failed to serialize DAlert");
        sdu.seek(0);

        let msg = SapMsg {
            sap: Sap::LcmcSap,
            src: TetraEntity::Cmce,
            dest: TetraEntity::Mle,
            msg: SapMsgInner::LcmcMleUnitdataReq(LcmcMleUnitdataReq {
                sdu,
                handle: calling_handle,
                endpoint_id: calling_endpoint_id,
                link_id: calling_link_id,
                // D-ALERT to the calling MS during individual setup: the legacy `main` code sent
                // this unacknowledged (FH FIX 2). Setup-phase MCCH signalling, same rationale as
                // D-CALL-PROCEEDING above.
                layer2service: Layer2Service::Unacknowledged,
                pdu_prio: 0,
                layer2_qos: 0,
                stealing_permission: false,
                stealing_repeats_flag: false,
                chan_alloc: None,
                main_address: calling_addr,
                tx_reporter: None,
            }),
        };
        queue.push_back(msg);
    }

    // Was: Diese Funktion dekodiert external Teilnehmer number.
    // Warum: Empfangene Protokolldaten müssen vor der weiteren Nutzung eindeutig verstanden und geprüft werden.
    pub(super) fn decode_external_subscriber_number(field: &Type3FieldGeneric) -> String {
        if field.len == 0 {
            return String::new();
        }

        // External number IE is commonly BCD-like packed digits.
        // Keep best-effort conversion and drop filler nibbles.
        let len_bits = field.len.min(128);
        let nibble_count = (len_bits / 4).min(24);
        let mut digits = String::with_capacity(nibble_count);
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for i in 0..nibble_count {
            let shift = len_bits - ((i + 1) * 4);
            let nibble = ((field.data >> shift) & 0x0f) as u8;
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match nibble {
                0..=9 => digits.push(char::from(b'0' + nibble)),
                0x0a => digits.push('*'),
                0x0b => digits.push('#'),
                0x0c..=0x0f => {}
                _ => {}
            }
        }
        digits
    }

    // Was: Diese Funktion kodiert external Teilnehmer number.
    // Warum: Alle Gegenstellen erhalten dadurch dasselbe erwartete Protokollformat.
    pub(super) fn encode_external_subscriber_number(number: &str) -> Option<Type3FieldGeneric> {
        let trimmed = number.trim();
        if trimmed.is_empty() {
            return None;
        }

        let mut nibbles = Vec::with_capacity(24);
        let mut encoded_preview = String::with_capacity(24);

        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for ch in trimmed.chars() {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            let nibble = match ch {
                '0'..='9' => ch as u8 - b'0',
                '*' => 0x0a,
                '#' => 0x0b,
                _ => {
                    tracing::debug!("CMCE: ignoring unsupported external number char '{}' in '{}'", ch, number);
                    continue;
                }
            };

            if nibbles.len() == 24 {
                tracing::debug!(
                    "CMCE: truncating external number '{}' to first 24 BCD digits ('{}')",
                    number,
                    encoded_preview
                );
                break;
            }

            nibbles.push(nibble);
            encoded_preview.push(ch);
        }

        if nibbles.is_empty() {
            tracing::debug!("CMCE: external number '{}' has no encodable digits", number);
            return None;
        }

        let len_bits = nibbles.len() * 4;
        let mut data = 0u128;
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for nibble in nibbles {
            data = (data << 4) | nibble as u128;
        }

        Some(Type3FieldGeneric {
            field_id: CmceType3ElemId::ExtSubscriberNum.into_raw(),
            len: len_bits,
            data,
        })
    }

    // Was: Führt den Arbeitsschritt `external_number_as_ssi` für external number as TETRA-Teilnehmerkennung (SSI) aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn external_number_as_ssi(number: &str) -> Option<u32> {
        let digits = number.trim();
        if digits.is_empty() || !digits.chars().all(|ch| ch.is_ascii_digit()) {
            return None;
        }
        let value = digits.parse::<u32>().ok()?;
        (value != 0 && value <= 0x00ff_ffff).then_some(value)
    }

    // Was: Diese Funktion erstellt network circuit Ruf from u setup.
    // Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
    pub(super) fn build_network_circuit_call_from_u_setup(pdu: &USetup, source_issi: u32) -> NetworkCircuitCall {
        let number = if let Some(ssi) = pdu.called_party_ssi {
            let ssi_u32 = ssi as u32;
            if ssi_u32 > 0 && ssi_u32 < 1_000_000 && pdu.external_subscriber_number.is_some() {
                ssi_u32.to_string()
            } else {
                pdu.external_subscriber_number
                    .as_ref()
                    .map(Self::decode_external_subscriber_number)
                    .unwrap_or_default()
            }
        } else {
            pdu.external_subscriber_number
                .as_ref()
                .map(Self::decode_external_subscriber_number)
                .unwrap_or_default()
        };

        NetworkCircuitCall {
            source_issi,
            destination: pdu.called_party_ssi.unwrap_or(0) as u32,
            number,
            priority: pdu.call_priority,
            service: pdu.basic_service_information.speech_service.unwrap_or(0),
            mode: pdu.basic_service_information.circuit_mode_type.into_raw() as u8,
            duplex: pdu.simplex_duplex_selection as u8,
            method: pdu.hook_method_selection as u8,
            communication: pdu.basic_service_information.communication_type.into_raw() as u8,
            grant: 0,
            permission: pdu.request_to_transmit_send_data as u8,
            timeout: Self::p2p_call_timeout(pdu.simplex_duplex_selection).into_raw() as u8,
            ownership: 1,
            queued: 0,
        }
    }

    #[inline]
    // Was: Prüft, ob external called party zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub(super) fn has_external_called_party(pdu: &USetup, network_call: &NetworkCircuitCall) -> bool {
        !network_call.number.is_empty() || pdu.external_subscriber_number.is_some() || pdu.called_party_short_number_address.is_some()
    }

    // Was: Führt den Arbeitsschritt `asterisk_route_number` für asterisk Weiterleitung number aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn asterisk_route_number(&self, network_call: &NetworkCircuitCall) -> Option<String> {
        let cfg = &self.config.config().asterisk;
        let raw = if !network_call.number.trim().is_empty() {
            network_call.number.trim().to_string()
        } else if network_call.destination != 0 {
            network_call.destination.to_string()
        } else {
            return None;
        };

        cfg.route_outbound_raw(&raw)
    }

    // Was: Führt den Arbeitsschritt `echolink_route_target` für echolink Weiterleitung target aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn echolink_route_target(&self, network_call: &NetworkCircuitCall) -> Option<String> {
        let cfg = self.config.effective_echolink();
        if !cfg.enabled || !cfg.outbound_enabled {
            return None;
        }

        let raw = if !network_call.number.trim().is_empty() {
            network_call.number.trim().to_string()
        } else if network_call.destination != 0 {
            network_call.destination.to_string()
        } else {
            return None;
        };

        let mut routed = raw.as_str();
        let prefix_matched = !cfg.outbound_prefix.is_empty() && raw.starts_with(&cfg.outbound_prefix);
        if prefix_matched && cfg.strip_outbound_prefix {
            routed = &raw[cfg.outbound_prefix.len()..];
        }

        let routed = routed.trim();
        if routed.is_empty() {
            return None;
        }

        if let Some(target) = cfg.routes.get(routed) {
            return Some(target.clone());
        }

        if cfg.service_numbers.iter().any(|n| n == routed) {
            if !cfg.auto_connect.trim().is_empty() {
                return Some(cfg.auto_connect.clone());
            }
            return Some(routed.to_string());
        }

        if cfg.service_numbers.is_empty() && prefix_matched {
            return Some(routed.to_string());
        }

        None
    }

    // Was: Führt den Arbeitsschritt `signal_umac_circuit_open` für signal UMAC-Funkzugriffssteuerung circuit open aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn signal_umac_circuit_open(
        queue: &mut MessageQueue,
        call: &CmceCircuit,
        _dltime: TdmaTime,
        peer_ts: Option<u8>,
        dl_media_source: CircuitDlMediaSource,
    ) {
        let circuit = Circuit {
            direction: call.direction,
            ts: call.ts,
            peer_ts,
            usage: call.usage,
            circuit_mode: call.circuit_mode,
            speech_service: call.speech_service,
            etee_encrypted: call.etee_encrypted,
            dl_media_source,
        };
        let cmd = SapMsg {
            sap: Sap::Control,
            src: TetraEntity::Cmce,
            dest: TetraEntity::Umac,
            msg: SapMsgInner::CmceCallControl(CallControl::Open(circuit)),
        };
        queue.push_back(cmd);
    }

    // Was: Führt den Arbeitsschritt `signal_umac_circuit_close` für signal UMAC-Funkzugriffssteuerung circuit close aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn signal_umac_circuit_close(queue: &mut MessageQueue, circuit: CmceCircuit, _dltime: TdmaTime) {
        let cmd = SapMsg {
            sap: Sap::Control,
            src: TetraEntity::Cmce,
            dest: TetraEntity::Umac,
            msg: SapMsgInner::CmceCallControl(CallControl::Close(circuit.direction, circuit.ts)),
        };
        queue.push_back(cmd);
    }

    // Was: Führt den Arbeitsschritt `feature_check_u_setup` für feature check u setup aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn feature_check_u_setup(pdu: &USetup) -> bool {
        let mut supported = true;

        if !(pdu.area_selection == 0 || pdu.area_selection == 1) {
            unimplemented_log!("Area selection not supported: {}", pdu.area_selection);
            supported = false;
        };
        // if pdu.hook_method_selection {
        //     // We do not implement explicit hook transitions yet; force hook_method_selection=false in responses.
        //     unimplemented_log!("Hook method selection requested, forcing hook_method_selection=false");
        // };
        // Duplex is supported only for P2P calls. P2P supports both simplex and duplex.
        if pdu.basic_service_information.communication_type != CommunicationType::P2p && pdu.simplex_duplex_selection {
            unimplemented_log!(
                "Duplex only supported for P2P calls (comm_type={})",
                pdu.basic_service_information.communication_type
            );
            supported = false;
        }
        // if pdu.basic_service_information != 0xFC {
        //     // TODO FIXME implement parsing
        //     tracing::error!("Basic service information not supported: {}", pdu.basic_service_information);
        //     return;
        // };
        // request_to_transmit_send_data can be false for speech group calls — the MS
        // implicitly requests to transmit by initiating the call. No action needed.
        if pdu.clir_control != 0 {
            unimplemented_log!("clir_control not supported: {}", pdu.clir_control);
        };
        if pdu.called_party_ssi.is_none() && pdu.called_party_short_number_address.is_none() && pdu.external_subscriber_number.is_none() {
            unimplemented_log!("U-SETUP called party not set (no SSI, short number or external number)");
        };
        if pdu.called_party_extension.is_some() && pdu.called_party_type_identifier != PartyTypeIdentifier::Tsi {
            unimplemented_log!(
                "U-SETUP called_party_extension present with unexpected called_party_type_identifier={}",
                pdu.called_party_type_identifier
            );
        };
        // Then, we warn about some other unhandled/unsupported fields
        if let Some(v) = &pdu.facility {
            unimplemented_log!("facility not supported: {:?}", v);
        };
        if let Some(v) = &pdu.dm_ms_address {
            unimplemented_log!("dm_ms_address not supported: {:?}", v);
        };
        if let Some(v) = &pdu.proprietary {
            unimplemented_log!("proprietary not supported: {:?}", v);
        };

        supported
    }

    /// Send D-TX GRANTED via FACCH stealing
    // Was: Diese Funktion sendet d tx granted facch.
    // Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
    pub(super) fn send_d_tx_granted_facch(&mut self, queue: &mut MessageQueue, call_id: u16, source_issi: u32, dest_gssi: u32, ts: u8) {
        let pdu = DTxGranted {
            call_identifier: call_id,
            transmission_grant: TransmissionGrant::GrantedToOtherUser.into_raw() as u8,
            transmission_request_permission: false,
            encryption_control: false,
            reserved: false,
            notification_indicator: None,
            transmitting_party_type_identifier: Some(1), // SSI
            transmitting_party_address_ssi: Some(source_issi as u64),
            transmitting_party_extension: None,
            external_subscriber_number: None,
            facility: None,
            dm_ms_address: None,
            proprietary: None,
        };

        tracing::debug!("-> D-TX GRANTED (FACCH) {:?}", pdu);
        let mut sdu = BitBuffer::new_autoexpand(30);
        pdu.to_bitbuf(&mut sdu).expect("Failed to serialize DTxGranted");
        sdu.seek(0);

        let dest_addr = TetraAddress::new(dest_gssi, SsiType::Gssi);
        let msg = Self::build_sapmsg_stealing(sdu, self.dltime, dest_addr, ts, None);
        queue.push_back(msg);
    }

    /// Send D-TX CEASED via FACCH stealing
    // Was: Diese Funktion sendet d tx ceased facch.
    // Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
    pub(super) fn send_d_tx_ceased_facch(&mut self, queue: &mut MessageQueue, call_id: u16, dest_gssi: u32, ts: u8) {
        let pdu = DTxCeased {
            call_identifier: call_id,
            transmission_request_permission: false, // ETSI 14.8.43: 0 = allowed to request transmission
            notification_indicator: None,
            facility: None,
            dm_ms_address: None,
            proprietary: None,
        };

        tracing::debug!("-> D-TX CEASED (FACCH) {:?}", pdu);
        let mut sdu = BitBuffer::new_autoexpand(30);
        pdu.to_bitbuf(&mut sdu).expect("Failed to serialize DTxCeased");
        sdu.seek(0);

        let dest_addr = TetraAddress::new(dest_gssi, SsiType::Gssi);
        let msg = Self::build_sapmsg_stealing(sdu, self.dltime, dest_addr, ts, None);
        queue.push_back(msg);
    }

    /// Operator / SwMI clear for active emergency calls involving `issi`.
    ///
    /// This is deliberately call-control level, not only dashboard/SDS state cleanup: emergency
    /// calls are represented as ordinary CC calls with ETSI priority 15. When the operator clears
    /// one from the Web UI, the BS releases every matching active emergency group or individual
    /// call with `SwmiRequestedDisconnection`, which sends the normal D-RELEASE path and tears down
    /// the traffic circuit.
    ///
    /// `issi == 0` is treated as "clear all active emergency calls" for future control clients,
    /// though the dashboard currently sends a concrete ISSI. Returns the number of calls released.
    // Was: Diese Funktion leert emergency calls for Teilnehmerkennung (ISSI).
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn clear_emergency_calls_for_issi(&mut self, queue: &mut MessageQueue, issi: u32) -> usize {
        let group_call_ids: Vec<u16> = self
            .active_calls
            .iter()
            .filter(|(_, call)| {
                is_emergency_priority(call.priority)
                    && (issi == 0
                        || call.source_issi == issi
                        || matches!(&call.origin, CallOrigin::Local { caller_addr } if caller_addr.ssi == issi))
            })
            .map(|(&call_id, _)| call_id)
            .collect();

        let individual_call_ids: Vec<u16> = self
            .individual_calls
            .iter()
            .filter(|(_, call)| {
                is_emergency_priority(call.priority)
                    && (issi == 0 || call.calling_addr.ssi == issi || call.called_addr.ssi == issi)
            })
            .map(|(&call_id, _)| call_id)
            .collect();

        let total = group_call_ids.len() + individual_call_ids.len();

        if total == 0 {
            tracing::info!(
                "CMCE: operator emergency clear for ISSI {} found no active emergency-priority call",
                issi
            );
            return 0;
        }

        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for call_id in group_call_ids {
            let snapshot = self.active_calls.get(&call_id).cloned();
            if let Some(call) = snapshot {
                tracing::warn!(
                    "CMCE: operator releasing EMERGENCY group call_id={} source_issi={} dest_gssi={} priority={} for ISSI {}",
                    call_id,
                    call.source_issi,
                    call.dest_gssi,
                    call.priority,
                    issi
                );

                // A priority-15 group emergency is special for the originating MS: some terminals
                // (notably with Hot Mic enabled) keep their *local emergency mode* active even if
                // they see only the group-addressed D-RELEASE. Clear the caller leg explicitly before
                // the normal group release so the HRT gets an addressed disconnect/release on the
                // traffic channel while it is still assigned, plus an MCCH fallback.
                self.send_emergency_originator_clear(queue, &call, call_id, DisconnectCause::SwmiRequestedDisconnection);

                if let CallOrigin::Network { network_entity, brew_uuid } = call.origin {
                    self.notify_network_call_end(queue, network_entity, brew_uuid);
                }
            }
            self.release_group_call(queue, call_id, DisconnectCause::SwmiRequestedDisconnection);
        }

        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for call_id in individual_call_ids {
            if let Some(call) = self.individual_calls.get(&call_id) {
                tracing::warn!(
                    "CMCE: operator releasing EMERGENCY individual call_id={} calling_issi={} called_issi={} priority={} for ISSI {}",
                    call_id,
                    call.calling_addr.ssi,
                    call.called_addr.ssi,
                    call.priority,
                    issi
                );
            }
            self.release_individual_call(queue, call_id, DisconnectCause::SwmiRequestedDisconnection);
        }

        total
    }

    /// Send an explicit clear sequence to the originating MS of an emergency group call.
    ///
    /// Normal group-call release is GSSI-addressed. That is enough to tear down the traffic call,
    /// but field terminals with Hot Mic / emergency personality may keep the local red emergency
    /// state latched until they receive something addressed to the originating ISSI or the user
    /// ends the emergency locally. Therefore operator clear sends both D-DISCONNECT and D-RELEASE
    /// directly to the caller before the ordinary group release.
    // Was: Diese Funktion sendet emergency originator clear.
    // Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
    fn send_emergency_originator_clear(
        &mut self,
        queue: &mut MessageQueue,
        call: &ActiveCall,
        call_id: u16,
        disconnect_cause: DisconnectCause,
    ) {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        let caller_addr = match &call.origin {
            CallOrigin::Local { caller_addr } => *caller_addr,
            _ if call.source_issi != 0 => TetraAddress::new(call.source_issi, SsiType::Issi),
            _ => {
                tracing::warn!(
                    "CMCE: emergency clear call_id={} has no local caller ISSI for caller-leg clear",
                    call_id
                );
                return;
            }
        };

        if caller_addr.ssi_type != SsiType::Issi {
            tracing::warn!(
                "CMCE: emergency clear call_id={} caller address {:?} is not ISSI, skipping caller-leg clear",
                call_id,
                caller_addr
            );
            return;
        }

        tracing::warn!(
            "CMCE: operator emergency clear call_id={} sending caller-leg D-DISCONNECT/D-RELEASE to ISSI {} on ts={} usage={}",
            call_id,
            caller_addr.ssi,
            call.ts,
            call.usage
        );

        // First try while the traffic circuit is still up. FACCH stealing reaches terminals that
        // are camped on the assigned traffic timeslot during Hot Mic / hangtime.
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for _ in 0..2 {
            let disconnect_sdu = Self::build_d_disconnect(call_id, disconnect_cause);
            queue.push_back(Self::build_sapmsg_stealing(
                disconnect_sdu,
                self.dltime,
                caller_addr,
                call.ts,
                Some(call.usage),
            ));

            let release_sdu = Self::build_d_release(call_id, disconnect_cause);
            queue.push_back(Self::build_sapmsg_stealing(
                release_sdu,
                self.dltime,
                caller_addr,
                call.ts,
                Some(call.usage),
            ));
        }

        // Also queue an MCCH fallback. If the MS has already dropped back from the traffic channel,
        // this is the path it should still be monitoring. Keep it unacknowledged like the existing
        // CC builders to match the rest of the BS signalling behaviour.
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for _ in 0..2 {
            let disconnect_sdu = Self::build_d_disconnect(call_id, disconnect_cause);
            queue.push_back(Self::build_sapmsg(disconnect_sdu, None, self.dltime, caller_addr, None));

            let release_sdu = Self::build_d_release(call_id, disconnect_cause);
            queue.push_back(Self::build_sapmsg(release_sdu, None, self.dltime, caller_addr, None));
        }
    }

    /// Release a group call: send D-RELEASE, close circuits, clean up state
    // Was: Diese Funktion gibt Gruppe Ruf.
    // Warum: Ressourcen werden dadurch rechtzeitig freigegeben und blockieren keine weiteren Vorgänge.
    pub(super) fn release_group_call(&mut self, queue: &mut MessageQueue, call_id: u16, disconnect_cause: DisconnectCause) {
        if let Some(call) = self.active_calls.get_mut(&call_id) {
            call.begin_release(disconnect_cause);
        }

        let Some(cached) = self.cached_setups.get(&call_id) else {
            tracing::error!("No cached D-SETUP for call_id={}", call_id);
            return;
        };
        let dest_addr = cached.dest_addr;

        // Send D-RELEASE to the group while the traffic circuit is still up.
        //
        // Dual-carrier gotcha: if the call lives on Carrier 2, members can be camped on the
        // secondary traffic bearer during hangtime and may not immediately hear an MCCH-only
        // release on Carrier 1. Deliver the release via FACCH/STCH on the assigned traffic
        // bearer first, then also queue an MCCH fallback for radios that have already returned
        // to the control channel. UMAC defers the actual circuit close until pending STCH has
        // drained, so these PDUs have a chance to leave before the bearer is torn down.
        if let Some(call) = self.active_calls.get(&call_id) {
            // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
            // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
            for _ in 0..2 {
                let sdu = Self::build_d_release_from_d_setup(&cached.pdu, disconnect_cause);
                let prim = Self::build_sapmsg_stealing(
                    sdu,
                    self.dltime,
                    dest_addr,
                    call.ts,
                    Some(call.usage),
                );
                queue.push_back(prim);
            }

            // The group originator may already have switched its address filter away from the
            // GSSI after U-TX-CEASED. Add an ISSI-addressed FACCH release so the source terminal
            // cannot remain in a stale call state when one of the group releases is lost.
            if matches!(&call.origin, CallOrigin::Local { .. }) {
                let source_addr = TetraAddress::issi(call.source_issi);
                let sdu = Self::build_d_release_from_d_setup(&cached.pdu, disconnect_cause);
                queue.push_back(Self::build_sapmsg_stealing(
                    sdu,
                    self.dltime,
                    source_addr,
                    call.ts,
                    Some(call.usage),
                ));
            }
        }

        // MCCH fallbacks. These remain useful for late monitors or terminals that already
        // dropped back to the main-carrier control channel before the FACCH/STCH release arrived.
        let sdu = Self::build_d_release_from_d_setup(&cached.pdu, disconnect_cause);
        let prim = Self::build_sapmsg(sdu, None, self.dltime, dest_addr, None);
        queue.push_back(prim);

        if let Some(call) = self.active_calls.get(&call_id)
            && matches!(&call.origin, CallOrigin::Local { .. })
        {
            let source_addr = TetraAddress::issi(call.source_issi);
            for _ in 0..2 {
                let sdu = Self::build_d_release_from_d_setup(&cached.pdu, disconnect_cause);
                queue.push_back(Self::build_sapmsg(sdu, None, self.dltime, source_addr, None));
            }
        }

        // Close the circuit in CircuitMgr and notify Brew
        if let Some(call) = self.active_calls.get(&call_id) {
            let ts = call.ts;
            let dest_ssi = call.dest_gssi;
            let brew_notification = if matches!(&call.origin, CallOrigin::Local { .. }) {
                BrewNotification::ForLocalSource {
                    source_issi: call.source_issi,
                    dest_gssi: dest_ssi,
                }
            } else {
                BrewNotification::Never
            };

            if let Ok(circuit) = self.circuits.close_circuit(Direction::Both, ts) {
                Self::signal_umac_circuit_close(queue, circuit, self.dltime);
            }

            // Ensure UMAC clears any hangtime override for this slot even if the circuit close is delayed.
            self.notify_call_ended(queue, CallTimeslot { call_id, ts }, true, brew_notification);

            self.release_timeslot(ts);
        }

        // Clean up
        self.cached_setups.remove(&call_id);
        self.call_restore.remove_context(call_id);
        let was_active = self.active_calls.remove(&call_id).is_some();

        // Dashboard telemetry: group call released (normal disconnect, timeout, hangtime or
        // pre-emption — all of which funnel through here). Only emit if a call was actually
        // removed, so a double-release can't produce a phantom Ended.
        if was_active {
            self.emit(crate::net_telemetry::TelemetryEvent::GroupCallEnded { call_id, gssi: 0 });
        }
    }

    /// Release an individual call: send D-RELEASE to both parties, close circuits, clean up state
    // Was: Diese Funktion gibt individual Ruf.
    // Warum: Ressourcen werden dadurch rechtzeitig freigegeben und blockieren keine weiteren Vorgänge.
    pub(super) fn release_individual_call(&mut self, queue: &mut MessageQueue, call_id: u16, disconnect_cause: DisconnectCause) {
        self.release_individual_call_inner(queue, call_id, disconnect_cause, None);
    }

    // Was: Diese Funktion gibt individual Ruf from u disconnect.
    // Warum: Ressourcen werden dadurch rechtzeitig freigegeben und blockieren keine weiteren Vorgänge.
    pub(super) fn release_individual_call_from_u_disconnect(
        &mut self,
        queue: &mut MessageQueue,
        call_id: u16,
        disconnect_cause: DisconnectCause,
        disconnecting_issi: u32,
    ) {
        self.release_individual_call_inner(queue, call_id, disconnect_cause, Some(disconnecting_issi));
    }

    // Was: Diese Funktion gibt individual Ruf inner.
    // Warum: Ressourcen werden dadurch rechtzeitig freigegeben und blockieren keine weiteren Vorgänge.
    fn release_individual_call_inner(
        &mut self,
        queue: &mut MessageQueue,
        call_id: u16,
        disconnect_cause: DisconnectCause,
        disconnecting_issi: Option<u32>,
    ) {
        if let Some(call) = self.individual_calls.get_mut(&call_id) {
            call.begin_release(disconnect_cause);
        }

        self.call_restore.remove_context(call_id);
        let Some(call) = self.individual_calls.remove(&call_id) else {
            tracing::warn!("No individual call for call_id={}", call_id);
            return;
        };

        let send_calling_leg = !call.calling_over_brew;
        let send_called_leg = !call.called_over_brew && !call.is_local_echo_call();

        // Was: Legt den festen Wert `SETUP_RELEASE_REPEATS` für setup release repeats fest.
        // Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
        const SETUP_RELEASE_REPEATS: usize = 3;

        if call.is_active() {
            // Deliver on traffic channel via FACCH stealing so the MS is still listening.
            // EN 300 392-2 14.5.1.3.1 allows the SwMI to inform the other MS
            // with either D-DISCONNECT or D-RELEASE. Use D-RELEASE for both legs
            // here so neither MS has to complete a U-RELEASE exchange while the
            // traffic circuits are being torn down.
            // Send twice to reduce "no response" due to occasional STCH loss.
            // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
            // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
            for _ in 0..2 {
                let sdu_calling = if let Some(cached) = self.cached_setups.get(&call_id) {
                    Self::build_d_release_from_d_setup(&cached.pdu, disconnect_cause)
                } else {
                    Self::build_d_release(call_id, disconnect_cause)
                };
                let sdu_called = if let Some(cached) = self.cached_setups.get(&call_id) {
                    Self::build_d_release_from_d_setup(&cached.pdu, disconnect_cause)
                } else {
                    Self::build_d_release(call_id, disconnect_cause)
                };
                if send_calling_leg {
                    let prim_calling = Self::build_sapmsg_stealing(
                        sdu_calling,
                        self.dltime,
                        call.calling_addr,
                        call.calling_ts,
                        Some(call.calling_usage),
                    );
                    queue.push_back(prim_calling);
                }
                if send_called_leg {
                    let prim_called =
                        Self::build_sapmsg_stealing(sdu_called, self.dltime, call.called_addr, call.called_ts, Some(call.called_usage));
                    queue.push_back(prim_called);
                }
            }
        } else {
            // Send D-RELEASE to calling and called MS via MCCH (no LLC link context).
            // During setup, both parties are monitoring MCCH, so force link_id=0.
            // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
            // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
            for _ in 0..SETUP_RELEASE_REPEATS {
                let sdu_calling = if let Some(cached) = self.cached_setups.get(&call_id) {
                    Self::build_d_release_from_d_setup(&cached.pdu, disconnect_cause)
                } else {
                    Self::build_d_release(call_id, disconnect_cause)
                };
                let sdu_called = if let Some(cached) = self.cached_setups.get(&call_id) {
                    Self::build_d_release_from_d_setup(&cached.pdu, disconnect_cause)
                } else {
                    Self::build_d_release(call_id, disconnect_cause)
                };
                if send_calling_leg {
                    let prim_calling = Self::build_sapmsg(sdu_calling, None, self.dltime, call.calling_addr, None);
                    queue.push_back(prim_calling);
                }

                if send_called_leg {
                    let prim_called = Self::build_sapmsg(sdu_called, None, self.dltime, call.called_addr, None);
                    queue.push_back(prim_called);
                }
            }
        }

        // Close the circuit(s)
        let mut ts_list = vec![call.calling_ts];
        if call.called_ts != call.calling_ts {
            ts_list.push(call.called_ts);
        }
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for ts in ts_list {
            if let Ok(circuit) = self.circuits.close_circuit(Direction::Both, ts) {
                Self::signal_umac_circuit_close(queue, circuit, self.dltime);
            }

            self.notify_call_ended(queue, CallTimeslot { call_id, ts }, true, BrewNotification::Never);

            self.release_timeslot(ts);
        }
        self.cached_setups.remove(&call_id);

        // If a local MS releases a network-routed individual call during setup/ringing,
        // the network side must be told immediately so Asterisk can CANCEL the still-ringing
        // SIP INVITE. Previously U-RELEASE lost the releasing ISSI and SwMIRequestedDisconnection
        // was filtered here, so the TETRA side cleaned up but the phone kept ringing.
        let local_party_requested_release = disconnecting_issi.is_some();

        if (call.called_over_brew || call.calling_over_brew)
            && (local_party_requested_release || disconnect_cause != DisconnectCause::SwmiRequestedDisconnection)
        {
            if let Some(brew_uuid) = call.brew_uuid {
                tracing::info!(
                    "CMCE: notifying {:?} about individual release uuid={} call_id={} cause={} local_party={:?}",
                    call.network_entity(),
                    brew_uuid,
                    call_id,
                    disconnect_cause,
                    disconnecting_issi
                );
                self.notify_network_circuit_release(queue, call.network_entity(), brew_uuid, disconnect_cause);
            }
        }

        // Dashboard telemetry: individual call released. Reaching here means the call was present
        // and removed at the top of this function (early-return otherwise), so this fires exactly
        // once per released individual call across every teardown path that funnels through here
        // (normal disconnect, setup/active timeout, pre-emption).
        self.emit(crate::net_telemetry::TelemetryEvent::IndividualCallEnded { call_id });
    }

    // Was: Diese Funktion gibt timeslot.
    // Warum: Ressourcen werden dadurch rechtzeitig freigegeben und blockieren keine weiteren Vorgänge.
    pub(super) fn release_timeslot(&mut self, ts: u8) {
        let mut state = self.config.state_write();
        if let Err(err) = state.timeslot_alloc.release(TimeslotOwner::Cmce, ts) {
            tracing::warn!("CcBsSubentity: failed to release timeslot ts={} err={:?}", ts, err);
        }
    }

    /// Map `cell.call_timeout_secs` from config to the nearest ETSI `CallTimeout` enum value.
    /// ETSI EN 300 392-2 Table 14.50: the BS sets D-SETUP/D-CONNECT call_time_out to indicate the
    /// maximum call duration. 0 means "no limit" (Infinite). Default config value is 120s (→ T2m).
    // Was: Führt den Arbeitsschritt `config_call_timeout` für Konfiguration Ruf timeout aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn config_call_timeout(&self) -> CallTimeout {
        let secs = self.config.config().cell.call_timeout_secs;
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match secs {
            0 => CallTimeout::Infinite, // 0 = no limit
            1..=37 => CallTimeout::T30s,
            38..=52 => CallTimeout::T45s,
            53..=90 => CallTimeout::T60s,
            91..=150 => CallTimeout::T2m,
            151..=210 => CallTimeout::T3m,
            211..=270 => CallTimeout::T4m,
            271..=390 => CallTimeout::T5m,
            391..=540 => CallTimeout::T6m,
            541..=720 => CallTimeout::T8m,
            721..=900 => CallTimeout::T10m,
            901..=1080 => CallTimeout::T12m,
            1081..=1350 => CallTimeout::T15m,
            1351..=1800 => CallTimeout::T20m,
            _ => CallTimeout::T30m,
        }
    }

    /// Number of currently free traffic timeslots (TS2..=TS4) on this cell.
    // Was: Führt den Arbeitsschritt `free_timeslot_count` für free timeslot count aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn free_timeslot_count(&self) -> usize {
        self.config.state_read().timeslot_alloc.free_count_with_capacity(self.traffic_slot_capacity())
    }

    /// Pick the best active call to pre-empt for a higher-priority call, or `None` if none is
    /// eligible. Only calls of *strictly lower* priority than `incoming_priority` may be
    /// pre-empted (equal priority keeps the channel — first come, first served). Among eligible
    /// calls the victim is chosen by: lowest priority first; then a call that is not actively
    /// transmitting (a group call in hangtime / a P2P call still in set-up — least disruptive to
    /// release); then the lowest call_id, purely for deterministic behaviour. `exclude` holds
    /// call_ids already released this round so the loop always makes progress.
    // Was: Diese Funktion wählt preemption victim.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn select_preemption_victim(&self, incoming_priority: u8, exclude: &[u16]) -> Option<PreemptVictim> {
        let mut candidates: Vec<(u8, u8, u16, PreemptVictim)> = Vec::new();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for (id, call) in self.active_calls.iter() {
            if call.priority < incoming_priority && !exclude.contains(id) {
                candidates.push((call.priority, call.tx_active as u8, *id, PreemptVictim::Group(*id)));
            }
        }
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for (id, call) in self.individual_calls.iter() {
            if call.priority < incoming_priority && !exclude.contains(id) {
                candidates.push((call.priority, call.is_active() as u8, *id, PreemptVictim::Individual(*id)));
            }
        }
        candidates
            .into_iter()
            .min_by_key(|(priority, active, call_id, _)| (*priority, *active, *call_id))
            .map(|(_, _, _, victim)| victim)
    }

    /// ETSI EN 300 392-2 clause 14.8 pre-emptive priority handling. When a call requested at a
    /// pre-emptive priority (>= 12, e.g. an emergency call) cannot be granted a traffic channel,
    /// the SwMI may release active calls of strictly lower priority to free up to `needed` slots.
    /// Each round releases the lowest-priority eligible call (see [`Self::select_preemption_victim`])
    /// with `DisconnectCause::PreEmptiveUseOfResource`. This is a no-op for non-pre-emptive
    /// priorities, and stops as soon as enough slots are free or no lower-priority call remains
    /// (in which case the caller's own allocation will fail and reject the call normally).
    // Was: Führt den Arbeitsschritt `preempt_for_priority` für preempt for Priorität aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub(super) fn preempt_for_priority(&mut self, queue: &mut MessageQueue, needed: usize, incoming_priority: u8) {
        if !is_preemptive_priority(incoming_priority) {
            return;
        }
        let mut attempted: Vec<u16> = Vec::new();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        while self.free_timeslot_count() < needed {
            let Some(victim) = self.select_preemption_victim(incoming_priority, &attempted) else {
                tracing::info!(
                    "CMCE: pre-emption for priority {} call cannot free enough channels ({} of {} slots free, no lower-priority call to release)",
                    incoming_priority,
                    self.free_timeslot_count(),
                    needed
                );
                break;
            };
            attempted.push(victim.call_id());
            tracing::info!(
                "CMCE: pre-empting {:?} to free a traffic channel for an incoming priority {} call",
                victim,
                incoming_priority
            );
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match victim {
                PreemptVictim::Group(call_id) => self.release_group_call(queue, call_id, DisconnectCause::PreEmptiveUseOfResource),
                PreemptVictim::Individual(call_id) => {
                    self.release_individual_call(queue, call_id, DisconnectCause::PreEmptiveUseOfResource)
                }
            }
        }
    }
}

// ── Call priority & pre-emption (ETSI EN 300 392-2 clause 14.8 "Call priority") ───────────────
//
// The call priority is a 4-bit field (0..=15) carried in U-SETUP / D-SETUP / D-CONNECT:
//   0        → priority not defined (treated as the lowest / normal priority)
//   1..=11   → ordinary priority levels (increasing)
//   12..=15  → the four *pre-emptive* priority levels
//   15       → highest priority; what a terminal's emergency button generates
//
// A call requested at a pre-emptive priority (>= 12) is entitled to pre-empt an active call of
// *strictly lower* priority when no traffic channel is free. An emergency call (priority 15) is
// the top pre-emptive level: it is surfaced distinctly on the dashboard and always granted the
// floor immediately on set-up.

/// Highest call priority (ETSI clause 14.8) — an emergency call.
// Was: Legt den festen Wert `CALL_PRIORITY_EMERGENCY` für Ruf Priorität emergency fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub(super) const CALL_PRIORITY_EMERGENCY: u8 = 15;
/// Lowest of the four pre-emptive priority levels (12..=15).
// Was: Legt den festen Wert `CALL_PRIORITY_PREEMPTIVE_MIN` für Ruf Priorität preemptive min fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub(super) const CALL_PRIORITY_PREEMPTIVE_MIN: u8 = 12;

/// True when a call at this priority may pre-empt a lower-priority call (pre-emptive priority).
#[inline]
// Was: Prüft, ob preemptive Priorität zutrifft.
// Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
pub(super) fn is_preemptive_priority(priority: u8) -> bool {
    priority >= CALL_PRIORITY_PREEMPTIVE_MIN
}

/// True when this priority denotes an emergency call (the highest priority level).
#[inline]
// Was: Prüft, ob emergency Priorität zutrifft.
// Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
pub(super) fn is_emergency_priority(priority: u8) -> bool {
    priority >= CALL_PRIORITY_EMERGENCY
}

/// A call selected for pre-emption: either an active group call or an individual (P2P) call.
#[derive(Clone, Copy, Debug)]
// Was: Listet die möglichen Varianten für preempt victim auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
enum PreemptVictim {
    Group(u16),
    Individual(u16),
}

// Was: Implementiert das zugehörige Verhalten für `PreemptVictim`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl PreemptVictim {
    #[inline]
    // Was: Führt den Arbeitsschritt `call_id` für Ruf Kennung aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn call_id(self) -> u16 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            PreemptVictim::Group(id) | PreemptVictim::Individual(id) => id,
        }
    }
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::super::BREW_AFFILIATION_GRACE_TS;
    use super::CcBsSubentity;
    use crate::MessageQueue;
    use tetra_config::bluestation::SharedConfig;
    use tetra_saps::control::brew::{BrewSubscriberAction, MmSubscriberUpdate};

    // Was: Prüft automatisch den Fall cfg.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_cfg() -> SharedConfig {
        let toml = r#"
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
"#;
        let cfg = tetra_config::bluestation::parsing::from_toml_str(toml).unwrap();
        SharedConfig::from_parts(cfg, None)
    }

    // Was: Führt den Arbeitsschritt `subscriber_update` für Teilnehmer update aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn subscriber_update(issi: u32, groups: Vec<u32>, action: BrewSubscriberAction) -> MmSubscriberUpdate {
        MmSubscriberUpdate { issi, groups, action }
    }

    #[test]
    // Was: Führt den Arbeitsschritt `external_subscriber_number_supports_24_digits` für external Teilnehmer number supports 24 digits aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn external_subscriber_number_supports_24_digits() {
        let number = "123456789012345678901234";
        let field = CcBsSubentity::encode_external_subscriber_number(number).expect("field should be generated");
        assert_eq!(field.len, 96);
        assert_ne!(field.data, 0);
        assert_eq!(CcBsSubentity::decode_external_subscriber_number(&field), number);
    }

    #[test]
    // Was: Führt den Arbeitsschritt `external_subscriber_number_truncates_to_24_digits` für external Teilnehmer number truncates to 24 digits aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn external_subscriber_number_truncates_to_24_digits() {
        let number = "1234567890123456789012345";
        let field = CcBsSubentity::encode_external_subscriber_number(number).expect("field should be generated");
        assert_eq!(field.len, 96);
        assert_eq!(CcBsSubentity::decode_external_subscriber_number(&field), "123456789012345678901234");
    }

    #[test]
    // Was: Führt den Arbeitsschritt `deaffiliate_keeps_listener_during_brew_resync_grace` für deaffiliate keeps listener during Brew-Verbindung resync grace aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn deaffiliate_keeps_listener_during_brew_resync_grace() {
        let mut cc = CcBsSubentity::new(test_cfg());
        let mut queue = MessageQueue::new();

        cc.handle_subscriber_update(&mut queue, subscriber_update(2635411, Vec::new(), BrewSubscriberAction::Register));
        cc.handle_subscriber_update(&mut queue, subscriber_update(2635411, vec![26225], BrewSubscriberAction::Affiliate));
        assert!(cc.has_listener(26225));

        cc.handle_subscriber_update(
            &mut queue,
            subscriber_update(2635411, vec![26225], BrewSubscriberAction::Deaffiliate),
        );
        assert_eq!(cc.group_listener_count(26225), 0);
        assert!(cc.has_listener(26225));

        cc.dltime = cc.dltime.add_timeslots(BREW_AFFILIATION_GRACE_TS + 1);
        cc.expire_brew_affiliation_grace(&mut queue);
        assert!(!cc.has_listener(26225));
    }

    #[test]
    // Was: Führt den Arbeitsschritt `affiliate_clears_brew_resync_grace` für affiliate clears Brew-Verbindung resync grace aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn affiliate_clears_brew_resync_grace() {
        let mut cc = CcBsSubentity::new(test_cfg());
        let mut queue = MessageQueue::new();

        cc.handle_subscriber_update(&mut queue, subscriber_update(2635411, Vec::new(), BrewSubscriberAction::Register));
        cc.handle_subscriber_update(&mut queue, subscriber_update(2635411, vec![26225], BrewSubscriberAction::Affiliate));
        cc.handle_subscriber_update(
            &mut queue,
            subscriber_update(2635411, vec![26225], BrewSubscriberAction::Deaffiliate),
        );
        cc.handle_subscriber_update(&mut queue, subscriber_update(2635411, vec![26225], BrewSubscriberAction::Affiliate));

        assert_eq!(cc.group_listener_count(26225), 1);
        assert!(cc.recent_deaffiliations.is_empty());
        assert!(cc.has_listener(26225));
    }
}
