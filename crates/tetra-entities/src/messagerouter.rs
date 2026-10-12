// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use tetra_config::bluestation::SharedConfig;
use tetra_core::{TdmaTime, tetra_entities::TetraEntity};
use tetra_saps::SapMsg;

use crate::TetraEntityTrait;

#[derive(Default)]
// Was: Listet die möglichen Varianten für Nachricht prio auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum MessagePrio {
    Immediate,
    #[default]
    Normal,
}

// Was: Bündelt die zusammengehörigen Werte für Nachricht Warteschlange in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MessageQueue {
    messages: VecDeque<SapMsg>,
    completed_bs_dl_time: Option<TdmaTime>,
}

// Was: Implementiert das zugehörige Verhalten für `MessageQueue`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MessageQueue {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new() -> Self {
        Self {
            messages: VecDeque::new(),
            completed_bs_dl_time: None,
        }
    }

    /// Report the completed physical BS slot, independently of decoded bursts.
    /// This is a clock observation, not permission to reset any DSP phase.
    pub fn observe_bs_dl_time(&mut self, time: TdmaTime) {
        if time.is_valid() && self.completed_bs_dl_time.is_none_or(|previous| time.diff(previous) > 0) {
            self.completed_bs_dl_time = Some(time);
        }
    }

    // Was: Diese Funktion legt back.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn push_back(&mut self, message: SapMsg) {
        self.messages.push_back(message);
    }

    // Was: Diese Funktion legt prio.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn push_prio(&mut self, message: SapMsg, prio: MessagePrio) {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match prio {
            MessagePrio::Immediate => {
                // Insert at the front for immediate processing
                self.messages.push_front(message);
            }
            MessagePrio::Normal => {
                // Insert at the back for normal processing
                self.messages.push_back(message);
            }
        }
    }

    // Was: Führt den Arbeitsschritt `pop_front` für pop front aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn pop_front(&mut self) -> Option<SapMsg> {
        self.messages.pop_front()
    }

    // Was: Führt den Arbeitsschritt `len` für len aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    // Was: Prüft, ob empty zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    // Was: Führt den Arbeitsschritt `iter` für iter aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn iter(&self) -> impl Iterator<Item = &SapMsg> {
        self.messages.iter()
    }
}

// Was: Bündelt die zusammengehörigen Werte für Nachricht Router in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MessageRouter {
    /// While currently unused by the MessageRouter, this may change in the future
    /// As such, we provide the MessageRouter with a copy of the SharedConfig
    _config: SharedConfig,
    entities: HashMap<TetraEntity, Box<dyn TetraEntityTrait>>,
    msg_queue: MessageQueue,

    /// The current TDMA time, if applicable.
    /// For Bs mode, this is always available
    /// For Ms/Mon mode, it is recovered from a received SYNC frame and communicated in a different way
    ts: TdmaTime,
    replaying_missed_slots: bool,
}

// Was: Implementiert das zugehörige Verhalten für `MessageRouter`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MessageRouter {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(config: SharedConfig) -> Self {
        Self {
            entities: HashMap::new(),
            msg_queue: MessageQueue::new(),
            _config: config,
            ts: TdmaTime::default(),
            replaying_missed_slots: false,
        }
    }

    /// For BS mode, sets global TDMA time
    /// Incremented each tick and passed to entities in tick() function
    // Was: Diese Funktion setzt dl time.
    // Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
    pub fn set_dl_time(&mut self, ts: TdmaTime) {
        self.ts = ts;
    }

    // Was: Diese Funktion registriert entity.
    // Warum: Die Zuordnung bleibt dadurch eindeutig und kann später sauber wieder entfernt werden.
    pub fn register_entity(&mut self, entity: Box<dyn TetraEntityTrait>) {
        let comp_type = entity.entity();
        tracing::debug!("register_entity {:?}", comp_type);
        self.entities.insert(comp_type, entity);
    }

    /// Returns a mut ref to a component of the requested type
    // Was: Diese Funktion liest entity.
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    pub fn get_entity(&mut self, comp: TetraEntity) -> Option<&mut dyn TetraEntityTrait> {
        self.entities.get_mut(&comp).map(|entity| entity.as_mut())
    }

    // Was: Führt den Arbeitsschritt `submit_message` für submit Nachricht aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn submit_message(&mut self, message: SapMsg) {
        tracing::debug!(
            "submit_message {:?}: {:?} -> {:?}",
            message.get_sap(),
            message.get_source(),
            message.get_dest()
        );
        self.msg_queue.push_back(message);
    }

    // Was: Führt den Arbeitsschritt `deliver_message` für deliver Nachricht aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn deliver_message(&mut self) {
        let message = self.msg_queue.pop_front();
        if let Some(message) = message {
            tracing::debug!(
                "deliver_message: got {:?}: {:?} -> {:?}",
                message.get_sap(),
                message.get_source(),
                message.get_dest()
            );

            // Determine the destination entity
            let dest = message.get_dest();

            // Virtual catch-up advances protocol timers and schedulers, but the
            // skipped air slots must not perform more blocking SDR exchanges.
            if self.replaying_missed_slots && *dest == TetraEntity::Phy {
                return;
            }

            // Check if the destination entity registered and deliver if found
            if let Some(entity) = self.entities.get_mut(dest) {
                entity.rx_prim(&mut self.msg_queue, message);
            } else {
                tracing::warn!(
                    "deliver_message: entity {:?} not found for {:?}: {:?} -> {:?}",
                    dest,
                    message.get_sap(),
                    message.get_source(),
                    message.get_dest()
                );
            }
        }
    }

    // Was: Führt den Arbeitsschritt `deliver_all_messages` für deliver all messages aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn deliver_all_messages(&mut self) {
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        while !self.msg_queue.messages.is_empty() {
            // PHY has returned a newer completed slot. Hold its RX primitives
            // until every protocol entity has reached that slot's clock.
            if self.msg_queue.completed_bs_dl_time.is_some_and(|time| time.diff(self.ts) > 0) {
                break;
            }
            self.deliver_message();
        }
    }

    // Was: Diese Funktion liest msgqueue len.
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    pub fn get_msgqueue_len(&self) -> usize {
        self.msg_queue.messages.len()
    }

    // Was: Diese Funktion bearbeitet start.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn tick_start(&mut self) {
        // tracing::info!("--- tick dl {} ul {} txdl {} ----------------------------",
        //     self.ts, self.ts.add_timeslots(-2), self.ts.add_timeslots(MACSCHED_TX_AHEAD as i32));
        tracing::info!("--- tick dl {} ----------------------------", self.ts);

        // Call tick on all entities
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for entity in self.entities.values_mut() {
            entity.tick_start(&mut self.msg_queue, self.ts);
        }
    }

    /// Executes all end-of-tick functions:
    /// - LLC sends down all outstanding BL-ACKs
    /// - UMAC finalizes any resources for ts and sends down to LMAC
    ///
    // Was: Diese Funktion bearbeitet end.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn tick_end(&mut self) {
        // A PHY exchange may happen while end-of-tick messages are drained.
        // Finish this tick once before examining that physical observation.
        self.finish_tick();
        if let Some(completed) = self.msg_queue.completed_bs_dl_time.take()
            && completed.diff(self.ts.add_timeslots(-1)) > 0
        {
            self.catch_up_missed_slots(completed);
        }
    }

    fn catch_up_missed_slots(&mut self, completed: TdmaTime) {
        let pending_rx = std::mem::take(&mut self.msg_queue.messages);
        tracing::warn!(
            previous_dl = %self.ts.add_timeslots(-1),
            completed_dl = %completed,
            missed_slots = completed.diff(self.ts.add_timeslots(-1)),
            "PHY RX clock advanced; replaying missed BS protocol slots without RF exchanges"
        );
        self.replaying_missed_slots = true;
        while completed.diff(self.ts) > 0 {
            self.tick_start();
            self.deliver_all_messages();
            self.finish_tick();
        }
        // The first received batch after the gap belongs to this actual slot.
        // Deliver it only after LMAC/UMAC and their schedulers share that time.
        self.tick_start();
        self.msg_queue.messages.extend(pending_rx);
        self.deliver_all_messages();
        self.finish_tick();
        self.replaying_missed_slots = false;
    }

    fn finish_tick(&mut self) {
        tracing::debug!("############################ end-of-tick ############################");

        // Llc should send down outstanding BL-ACKs
        let target = TetraEntity::Llc;
        if let Some(entity) = self.entities.get_mut(&target) {
            tracing::trace!("tick_end for entity {:?}", target);
            entity.tick_end(&mut self.msg_queue, self.ts);
        }
        self.deliver_all_messages();

        // Umac should finalize any resources and send down to Lmac
        let target = TetraEntity::Umac;
        if let Some(entity) = self.entities.get_mut(&target) {
            tracing::trace!("tick_end for entity {:?}", target);
            entity.tick_end(&mut self.msg_queue, self.ts);
        }
        self.deliver_all_messages();

        // Then call tick_end on all other entities
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for entity in self.entities.values_mut() {
            let entity_id = entity.entity();
            if entity_id == TetraEntity::Llc || entity_id == TetraEntity::Umac {
                continue;
            }
            entity.tick_end(&mut self.msg_queue, self.ts);
        }
        self.deliver_all_messages();

        // Increment the TDMA time if set
        self.ts = self.ts.add_timeslots(1);
    }

    /// Runs the full stack either forever or for a specified number of ticks.
    /// If `running` is provided, the loop will exit when the flag is set to false
    /// (e.g. by a Ctrl+C signal handler), allowing entities to be dropped cleanly.
    // Was: Diese Funktion führt stack.
    // Warum: Der Lebenszyklus des Dienstes bleibt so an einer zentralen Stelle steuerbar.
    pub fn run_stack(&mut self, num_ticks: Option<usize>, running: Option<Arc<AtomicBool>>) {
        let mut ticks: usize = 0;

        // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
        // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
        loop {
            // Check if we've been asked to stop (e.g. Ctrl+C)
            if let Some(ref flag) = running {
                if !flag.load(Ordering::Relaxed) {
                    eprintln!("\n[INFO] Shutting down gracefully...");
                    break;
                }
            }

            // Health watchdog: stamp that the core loop is alive this tick (lock-free atomic).
            crate::health::registry().note_tick();

            // Send tick_start event
            self.tick_start();

            // Drain once: a newer RX clock intentionally leaves its batch held
            // for tick_end's sequential catch-up instead of busy-spinning here.
            self.deliver_all_messages();

            // Send tick_end event and process final messages
            self.tick_end();

            // Check if we should stop
            ticks += 1;
            if let Some(num_ticks) = num_ticks {
                if ticks >= num_ticks {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lmac::lmac_bs::LmacBs;
    use crate::phy::components::burst_consts::NUB_BITS;
    use crate::phy::phy_bs::PhyBs;
    use crate::umac::umac_bs::UmacBs;
    use std::sync::Mutex;
    use tetra_core::TrainingSequence;
    use tetra_pdus::phy::traits::rxtx_dev::{RxBurstBits, RxSlotBits, RxTxDev, RxTxDevError, TxSlotBits};

    #[derive(Default)]
    struct Trace {
        ticks: Vec<TdmaTime>,
        ends: Vec<TdmaTime>,
        physical_tx: Vec<Vec<(u16, TdmaTime)>>,
        uplink_delivery: Vec<(u16, TdmaTime)>,
    }

    type RxFrame = (u16, TdmaTime, bool);
    struct FakeRadio {
        received: VecDeque<Vec<RxFrame>>,
        trace: Arc<Mutex<Trace>>,
    }

    impl RxTxDev for FakeRadio {
        fn rxtx_timeslot(&mut self, tx: &[TxSlotBits]) -> Result<Vec<Option<RxSlotBits<'_>>>, RxTxDevError> {
            self.trace
                .lock()
                .unwrap()
                .physical_tx
                .push(tx.iter().map(|slot| (slot.carrier_num, slot.time)).collect());
            let frames = self
                .received
                .pop_front()
                .expect("virtual catch-up must not exchange another physical slot");
            static BURST: [u8; NUB_BITS] = [0; NUB_BITS];
            Ok(frames
                .into_iter()
                .map(|(carrier_num, time, burst)| {
                    Some(RxSlotBits {
                        carrier_num,
                        time,
                        slot: if burst {
                            RxBurstBits {
                                train_type: TrainingSequence::NormalTrainSeq1,
                                bits: &BURST,
                                rssi_dbfs: -10.0,
                            }
                        } else {
                            RxBurstBits::default()
                        },
                        ..Default::default()
                    })
                })
                .collect())
        }
    }

    struct ClockObserver(Arc<Mutex<Trace>>);
    impl TetraEntityTrait for ClockObserver {
        fn entity(&self) -> TetraEntity {
            TetraEntity::Llc
        }
        fn rx_prim(&mut self, _: &mut MessageQueue, _: SapMsg) {}
        fn tick_start(&mut self, _: &mut MessageQueue, time: TdmaTime) {
            let mut trace = self.0.lock().unwrap();
            if let Some(previous) = trace.ticks.last() {
                assert_eq!(
                    time.diff(*previous),
                    1,
                    "all protocol timers must see each skipped tick exactly once"
                );
            }
            trace.ticks.push(time);
        }
        fn tick_end(&mut self, _: &mut MessageQueue, time: TdmaTime) -> bool {
            let mut trace = self.0.lock().unwrap();
            assert_eq!(
                trace.ticks.get(trace.ends.len()),
                Some(&time),
                "each started protocol tick must end exactly once"
            );
            trace.ends.push(time);
            false
        }
    }

    struct ObservedLmac {
        inner: LmacBs,
        time: TdmaTime,
        trace: Arc<Mutex<Trace>>,
    }
    impl TetraEntityTrait for ObservedLmac {
        fn entity(&self) -> TetraEntity {
            TetraEntity::Lmac
        }
        fn rx_prim(&mut self, queue: &mut MessageQueue, message: SapMsg) {
            if let tetra_saps::SapMsgInner::TpUnitdataInd(primitive) = &message.msg {
                self.trace.lock().unwrap().uplink_delivery.push((primitive.carrier_num, self.time));
            }
            self.inner.rx_prim(queue, message);
        }
        fn tick_start(&mut self, queue: &mut MessageQueue, time: TdmaTime) {
            self.time = time;
            self.inner.tick_start(queue, time);
        }
    }

    struct DeferredUmac {
        inner: UmacBs,
        pending: VecDeque<SapMsg>,
    }
    impl TetraEntityTrait for DeferredUmac {
        fn entity(&self) -> TetraEntity {
            TetraEntity::Umac
        }
        fn rx_prim(&mut self, queue: &mut MessageQueue, message: SapMsg) {
            self.inner.rx_prim(queue, message);
        }
        fn tick_start(&mut self, _: &mut MessageQueue, time: TdmaTime) {
            let mut pending = MessageQueue::new();
            self.inner.tick_start(&mut pending, time);
            self.pending.extend(pending.messages);
        }
        fn tick_end(&mut self, queue: &mut MessageQueue, _: TdmaTime) -> bool {
            queue.messages.extend(self.pending.drain(..));
            false
        }
    }

    fn exercise(start: TdmaTime, received: Vec<Vec<RxFrame>>) -> (MessageRouter, Arc<Mutex<Trace>>) {
        exercise_delivery(start, received, false)
    }

    fn exercise_delivery(start: TdmaTime, received: Vec<Vec<RxFrame>>, defer_until_end: bool) -> (MessageRouter, Arc<Mutex<Trace>>) {
        let config = tetra_config::bluestation::parsing::from_toml_str(
            r#"
config_version = "0.6"
stack_mode = "Bs"
[phy_io]
backend = "None"
[net_info]
mcc = 901
mnc = 1510
[cell_info]
main_carrier = 720
secondary_carrier = 721
freq_band = 4
freq_offset = 0
duplex_spacing = 0
reverse_operation = false
location_area = 3
colour_code = 1
"#,
        )
        .unwrap();
        let config = SharedConfig::from_parts(config, None);
        let trace = Arc::new(Mutex::new(Trace::default()));
        let physical_calls = received.len();
        let radio = FakeRadio {
            received: received.into(),
            trace: trace.clone(),
        };
        let mut router = MessageRouter::new(config.clone());
        router.set_dl_time(start);
        // Use actual UMAC schedulers (including their strict +1 assertion),
        // actual LMAC encoding, and the actual formerly broken PHY extraction.
        if defer_until_end {
            router.register_entity(Box::new(DeferredUmac {
                inner: UmacBs::new(config.clone()),
                pending: VecDeque::new(),
            }));
        } else {
            router.register_entity(Box::new(UmacBs::new(config.clone())));
        }
        router.register_entity(Box::new(ObservedLmac {
            inner: LmacBs::new(config.clone()),
            time: start,
            trace: trace.clone(),
        }));
        router.register_entity(Box::new(PhyBs::new(config, radio)));
        router.register_entity(Box::new(ClockObserver(trace.clone())));
        router.run_stack(Some(physical_calls), None);
        (router, trace)
    }

    fn idle_pair(ul: i32) -> Vec<RxFrame> {
        vec![(720, TdmaTime::from_int(ul), false), (721, TdmaTime::from_int(ul), false)]
    }

    #[test]
    fn dropped_rx_slots_replay_protocol_clock_and_resume_future_dual_carrier_tx() {
        let (router, trace) = exercise(
            TdmaTime::from_int(2),
            vec![
                idle_pair(0),
                vec![(720, TdmaTime::from_int(12), true), (721, TdmaTime::from_int(12), true)],
                idle_pair(13),
            ],
        );
        let trace = trace.lock().unwrap();
        assert_eq!(trace.ticks, (2..=15).map(TdmaTime::from_int).collect::<Vec<_>>());
        assert_eq!(trace.ends, trace.ticks);
        assert_eq!(trace.physical_tx.len(), 3);
        // The idle secondary carrier has no burst in air TS4 (absolute slot 3).
        // Its TS1 control/guard burst is present in slots 4 and 16, including
        // the first physical exchange after the RX gap. Keep exact batches so
        // recovering the clock cannot hide a lost carrier or an extra RF call.
        assert_eq!(
            trace.physical_tx,
            [
                vec![(720, TdmaTime::from_int(3))],
                vec![(720, TdmaTime::from_int(4)), (721, TdmaTime::from_int(4))],
                vec![(720, TdmaTime::from_int(16)), (721, TdmaTime::from_int(16))],
            ]
        );
        assert_eq!(trace.uplink_delivery, [(720, TdmaTime::from_int(14))]);
        assert_eq!(router.ts, TdmaTime::from_int(16));
    }

    #[test]
    fn rx_clock_discovered_during_tick_end_catches_up_without_duplicate_callbacks() {
        let (router, trace) = exercise_delivery(TdmaTime::from_int(2), vec![idle_pair(0), idle_pair(12), idle_pair(13)], true);
        let trace = trace.lock().unwrap();
        assert_eq!(trace.ticks, (2..=15).map(TdmaTime::from_int).collect::<Vec<_>>());
        assert_eq!(trace.ends, trace.ticks);
        assert_eq!(trace.physical_tx.len(), 3);
        assert_eq!(trace.physical_tx[2], [(720, TdmaTime::from_int(16)), (721, TdmaTime::from_int(16))]);
        assert_eq!(router.ts, TdmaTime::from_int(16));
    }

    #[test]
    fn stale_all_carrier_rx_batch_is_not_delivered_under_a_newer_mac_clock() {
        let (router, trace) = exercise(
            TdmaTime::from_int(2),
            vec![
                idle_pair(0),
                idle_pair(12),
                vec![(720, TdmaTime::from_int(0), true), (721, TdmaTime::from_int(0), true)],
                idle_pair(14),
            ],
        );
        let trace = trace.lock().unwrap();
        assert!(trace.uplink_delivery.is_empty());
        assert_eq!(trace.physical_tx.len(), 4);
        assert_eq!(trace.ticks, (2..=16).map(TdmaTime::from_int).collect::<Vec<_>>());
        assert_eq!(trace.ends, trace.ticks);
        assert_eq!(router.ts, TdmaTime::from_int(17));
    }

    #[test]
    fn idle_rx_slot_clock_keeps_healthy_ticks_and_initial_ul_wrap_unchanged() {
        let (router, trace) = exercise(TdmaTime::default(), vec![idle_pair(-2), idle_pair(-1), idle_pair(0)]);
        let trace = trace.lock().unwrap();
        assert_eq!(trace.ticks, (0..3).map(TdmaTime::from_int).collect::<Vec<_>>());
        assert_eq!(trace.ends, trace.ticks);
        assert_eq!(trace.physical_tx.len(), 3);
        assert!(trace.uplink_delivery.is_empty());
        assert_eq!(router.ts, TdmaTime::from_int(3));
    }

    #[test]
    fn newest_rx_carrier_wraps_clock_and_does_not_relabel_stale_carrier_bursts() {
        let start = TdmaTime::from_int(-2);
        let (router, trace) = exercise(
            start,
            vec![vec![
                (721, TdmaTime::from_int(-3), true),
                // Invalid frame zero normalizes to the same completed DL zero.
                // It must not displace the valid primary burst during deduplication.
                (721, TdmaTime { h: 0, m: 1, f: 0, t: 3 }, true),
                (720, TdmaTime::from_int(-2), true),
            ]],
        );
        let trace = trace.lock().unwrap();
        assert_eq!(trace.ticks, (-2..=0).map(TdmaTime::from_int).collect::<Vec<_>>());
        assert_eq!(trace.ends, trace.ticks);
        assert_eq!(trace.physical_tx.len(), 1);
        assert_eq!(trace.uplink_delivery, [(720, TdmaTime::default())]);
        assert_eq!(router.ts, TdmaTime::from_int(1));
    }
}
