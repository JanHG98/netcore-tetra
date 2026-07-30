// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::{HashMap, HashSet};

use crate::net_telemetry::{TelemetryEvent, channel::TelemetrySink};
use tetra_core::TdmaTime;
use tetra_pdus::mm::enums::energy_saving_mode::EnergySavingMode;
use tetra_pdus::mm::fields::class_of_ms::ClassOfMs;

/// Frame-based energy-economy monitoring cycle length, in TDMA frames, per ETSI EN 300 392-2
/// Table 23.9 (an EE MS wakes for 1 frame then sleeps N: EG1 sleeps 1 → cycle 2, EG2 sleeps 2 →
/// cycle 3, EG3 sleeps 5 → cycle 6). The BS caps Eg4–Eg7 to Eg3 to keep call-setup latency bounded,
/// so only StayAlive / Eg1 / Eg2 / Eg3 are ever granted or stored. Returns `None` for StayAlive
/// (no monitoring window — the MS is always reachable). Single source of truth for both the grant
/// (`grant_energy_saving`) and the republished window, so the two can never drift apart.
// Was: Führt den Arbeitsschritt `ee_cycle_frames` für ee cycle frames aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub(crate) const fn ee_cycle_frames(mode: EnergySavingMode) -> Option<u8> {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match mode {
        EnergySavingMode::StayAlive => None,
        EnergySavingMode::Eg1 => Some(2),
        EnergySavingMode::Eg2 => Some(3),
        // Eg3..Eg7 capped to Eg3 (sleep 5 frames → cycle 6).
        _ => Some(6),
    }
}

#[derive(Debug)]
// Was: Listet die möglichen Varianten für client mgr err auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum ClientMgrErr {
    ClientNotFound { issi: u32 },
    GroupNotFound { gssi: u32 },
    IssiInGroupRange { issi: u32 },
    GssiInClientRange { gssi: u32 },
}

#[derive(Debug, PartialEq, Clone, Copy)]
// Was: Listet die möglichen Varianten für Mobilitätsverwaltung client Zustand auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum MmClientState {
    Unknown,
    Attached,
    Detached,
}


#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für Mobilitätsverwaltung client Mobilität Kontext in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MmClientMobilityContext {
    pub issi: u32,
    pub state: MmClientState,
    pub groups: Vec<u32>,
    pub energy_saving_mode: EnergySavingMode,
    pub monitoring_frame: Option<u8>,
    pub monitoring_multiframe: Option<u8>,
    pub class_of_ms: Option<ClassOfMs>,
    pub last_handle: u32,
    pub tei: Option<u64>,
}

// Was: Bündelt die zusammengehörigen Werte für Mobilitätsverwaltung client properties in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MmClientProperties {
    pub issi: u32,
    pub state: MmClientState,
    pub groups: HashSet<u32>,
    pub energy_saving_mode: EnergySavingMode,
    /// TDMA frame number (1..=18) at which this MS wakes up to monitor the MCCH.
    /// Set to None for StayAlive MSs. Used to gate/schedule unsolicited downlink (D-SETUP, SDS)
    /// to the MS's energy-economy monitoring window.
    pub monitoring_frame: Option<u8>,
    /// Multiframe offset within the Eg cycle at which this MS wakes up.
    /// Set to None for StayAlive MSs.
    pub monitoring_multiframe: Option<u8>,
    /// Last measured RSSI from this MS in dBFS (dB relative to ADC full scale).
    /// Updated on every UL burst received from this ISSI.
    /// None until first burst received after registration.
    pub last_rssi: Option<f32>,
    /// Monotonic timestamp of the last uplink burst heard from this MS (updated on every RSSI
    /// measurement — i.e. whenever the MS is observed transmitting). Distinct from
    /// `last_registration_time`: a present MS keeps transmitting (PTT / SDS / signalling bursts)
    /// without necessarily re-registering, so this is the authoritative "still on the air"
    /// signal. Used at T351 expiry to avoid dropping a radio that is demonstrably present but
    /// did not answer an unsolicited D-LOCATION-UPDATE-COMMAND (e.g. an energy-economy radio
    /// asleep when the COMMAND was sent) — which made present stations vanish from the dashboard.
    pub last_uplink_time: std::time::Instant,
    /// Timestamp (system time) when this MS last registered or re-registered.
    /// Used to enforce periodic registration expiry (T351).
    pub last_registration_time: std::time::Instant,
    pub class_of_ms: Option<ClassOfMs>,
    /// Layer-2 handle from the last successful location update.
    /// Required for sending downlink MM PDUs (D-LOCATION-UPDATE-COMMAND etc.)
    /// to this MS. Set to 0 until the first location update is received.
    pub last_handle: u32,
    /// Terminal Equipment Identity (60-bit hardware ID, like IMEI).
    /// Set when the MS sends U-TEI-PROVIDE. None if not yet received.
    pub tei: Option<u64>,
    /// True after BS sends D-LOCATION-UPDATE-COMMAND at T351 expiry.
    /// If the terminal re-registers, this is cleared. If T351 expires again
    /// while this is still true, the terminal is silently removed (no response).
    pub pending_command_sent: bool,
    /// When Some, terminal has until this instant to respond to D-LOCATION-UPDATE-COMMAND.
    pub grace_expires_at: Option<std::time::Instant>,
    // pub last_seen: TdmaTime,
}

// Was: Implementiert das zugehörige Verhalten für `MmClientProperties`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MmClientProperties {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(ssi: u32) -> Self {
        MmClientProperties {
            issi: ssi,
            state: MmClientState::Unknown,
            groups: HashSet::new(),
            energy_saving_mode: EnergySavingMode::StayAlive,
            monitoring_frame: None,
            monitoring_multiframe: None,
            last_rssi: None,
            last_uplink_time: std::time::Instant::now(),
            pending_command_sent: false,
            grace_expires_at: None,
            last_registration_time: std::time::Instant::now(),
            class_of_ms: None,
            last_handle: 0,
            tei: None,
            // last_seen: TdmaTime::default(),
        }
    }
}

/// Stub function, to be replaced with checks based on configuration file
// Was: Prüft, ob individual zutrifft.
// Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
fn is_individual(_issi: u32) -> bool {
    return true;
}
/// Stub function, to be replaced with checks based on configuration file
// Was: Führt den Arbeitsschritt `in_group_range` für in Gruppe range aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn in_group_range(_gssi: u32) -> bool {
    return true;
}
/// Stub function, to be replaced with checks based on configuration file
// Was: Prüft, ob Gruppe zutrifft.
// Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
fn is_group(_gssi: u32) -> bool {
    return true;
}
/// Stub function, to be replaced with checks based on configuration file
// Was: Führt den Arbeitsschritt `may_attach` für may attach aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn may_attach(_issi: u32, _gssi: u32) -> bool {
    return true;
}

// Was: Bündelt die zusammengehörigen Werte für Mobilitätsverwaltung client mgr in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MmClientMgr {
    clients: HashMap<u32, MmClientProperties>,
    telemetry_sink: Option<TelemetrySink>,
}

// Was: Implementiert das zugehörige Verhalten für `MmClientMgr`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MmClientMgr {
    // Was: Führt den Arbeitsschritt `telemetry_sink` für Telemetrie sink aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn telemetry_sink(&self) -> Option<&TelemetrySink> {
        self.telemetry_sink.as_ref()
    }

    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(telemetry_sink: Option<TelemetrySink>) -> Self {
        MmClientMgr {
            clients: HashMap::new(),
            telemetry_sink,
        }
    }

    // Was: Diese Funktion liest client by Teilnehmerkennung (ISSI).
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    pub fn get_client_by_issi(&mut self, issi: u32) -> Option<&MmClientProperties> {
        self.clients.get(&issi)
    }

    // Was: Führt den Arbeitsschritt `client_is_known` für client is known aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn client_is_known(&self, issi: u32) -> bool {
        self.clients.contains_key(&issi)
    }

    // Was: Diese Funktion setzt client Zustand.
    // Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
    pub fn set_client_state(&mut self, issi: u32, state: MmClientState) -> Result<(), ClientMgrErr> {
        if let Some(client) = self.clients.get_mut(&issi) {
            client.state = state;
            Ok(())
        } else {
            Err(ClientMgrErr::ClientNotFound { issi })
        }
    }

    // Was: Diese Funktion setzt client energy saving mode.
    // Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
    pub fn set_client_energy_saving_mode(&mut self, issi: u32, mode: EnergySavingMode) -> Result<(), ClientMgrErr> {
        if let Some(client) = self.clients.get_mut(&issi) {
            client.energy_saving_mode = mode;
            if let Some(sink) = &self.telemetry_sink {
                sink.send(TelemetryEvent::MsEnergySaving { issi, mode: mode as u8 });
            }
            Ok(())
        } else {
            Err(ClientMgrErr::ClientNotFound { issi })
        }
    }

    // Was: Diese Funktion setzt client monitoring window.
    // Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
    pub fn set_client_monitoring_window(&mut self, issi: u32, frame: Option<u8>, multiframe: Option<u8>) -> Result<(), ClientMgrErr> {
        if let Some(client) = self.clients.get_mut(&issi) {
            client.monitoring_frame = frame;
            client.monitoring_multiframe = multiframe;
            Ok(())
        } else {
            Err(ClientMgrErr::ClientNotFound { issi })
        }
    }

    /// Update RSSI for a known MS. Silently ignored if MS is not registered.
    // Was: Diese Funktion aktualisiert client rssi.
    // Warum: Bestehender Zustand wird dadurch kontrolliert und nach einheitlichen Regeln geändert.
    pub fn update_client_rssi(&mut self, issi: u32, rssi_dbfs: f32) {
        if let Some(client) = self.clients.get_mut(&issi) {
            // Every RSSI measurement is an uplink burst we just heard → the MS is on the air now.
            client.last_uplink_time = std::time::Instant::now();
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            let should_log = match client.last_rssi {
                None => true,                                  // First measurement
                Some(prev) => (rssi_dbfs - prev).abs() >= 3.0, // Log on >=3dB change
            };
            client.last_rssi = Some(rssi_dbfs);
            if should_log {
                tracing::info!("RSSI: ISSI {} = {:.1} dBFS", issi, rssi_dbfs);
            }
        }
    }

    /// True if an uplink burst from this MS was heard within `within` — a direct sign the radio
    /// is still on the air, independent of whether it answered an unsolicited
    /// D-LOCATION-UPDATE-COMMAND. Used to keep a present radio from being torn down at T351
    /// expiry (it may be an energy-economy radio that was asleep when the COMMAND was sent).
    // Was: Führt den Arbeitsschritt `heard_on_air_within` für heard on air within aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn heard_on_air_within(&self, issi: u32, within: std::time::Duration) -> bool {
        self.clients
            .get(&issi)
            .map(|c| c.last_uplink_time.elapsed() < within)
            .unwrap_or(false)
    }

    /// Reset the periodic registration timer for a MS (called on each U-LOCATION-UPDATING-DEMAND).
    // Was: Diese Funktion setzt registration Zeitüberwachung.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn reset_registration_timer(&mut self, issi: u32) {
        if let Some(client) = self.clients.get_mut(&issi) {
            client.last_registration_time = std::time::Instant::now();
            client.pending_command_sent = false;
            client.grace_expires_at = None;
        }
    }

    /// Returns true if a D-LOCATION-UPDATE-COMMAND was sent and terminal hasn't responded yet.
    // Was: Prüft, ob pending command zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_pending_command(&self, issi: u32) -> bool {
        self.clients.get(&issi).map(|c| c.pending_command_sent).unwrap_or(false)
    }

    /// Mark that we sent D-LOCATION-UPDATE-COMMAND at T351 expiry.
    /// Terminal has grace_secs to respond before being removed.
    // Was: Diese Funktion setzt pending command.
    // Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
    pub fn set_pending_command(&mut self, issi: u32, grace_secs: u32) {
        if let Some(client) = self.clients.get_mut(&issi) {
            client.pending_command_sent = true;
            // Set last_registration_time so elapsed() > interval after grace_secs.
            // Achieved by back-dating: last_registration_time = now - (interval - grace_secs)
            // But we don't know interval here, so we use a simpler approach:
            // collect_expired_registrations checks pending_command_sent + grace separately.
            client.grace_expires_at = Some(std::time::Instant::now() + std::time::Duration::from_secs(grace_secs as u64));
        }
    }

    /// Returns list of ISSIs whose periodic registration has expired.
    /// interval_secs=0 means disabled — always returns empty list.
    // Was: Diese Funktion sammelt expired registrations.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn collect_expired_registrations(&self, interval_secs: u32) -> Vec<u32> {
        if interval_secs == 0 {
            return Vec::new();
        }
        let threshold = std::time::Duration::from_secs(interval_secs as u64);
        let now = std::time::Instant::now();
        self.clients
            .iter()
            .filter(|(_, c)| {
                if c.pending_command_sent {
                    // Already sent COMMAND — remove if grace period expired
                    c.grace_expires_at.map(|d| now >= d).unwrap_or(true)
                } else {
                    // Normal T351 check
                    c.last_registration_time.elapsed() > threshold
                }
            })
            .map(|(&issi, _)| issi)
            .collect()
    }

    /// Decide whether the T351 D-LOCATION-UPDATE-COMMAND for `issi` should be sent on this tick.
    ///
    /// The COMMAND is an unsolicited downlink PDU. An energy-economy MS only listens to the
    /// downlink during its monitoring window, so a COMMAND sent while it sleeps is missed — it
    /// then never answers and is torn down at the second expiry, making a present radio vanish
    /// from the dashboard. So:
    ///   - StayAlive MS (always listening) or an EE MS whose monitoring window is not yet known
    ///     → send now (`true`);
    ///   - EE MS during its monitoring window → send now (`true`);
    ///   - EE MS asleep outside its window → defer (`false`) so the caller retries next tick and
    ///     sends in the wake window — UNLESS it is overdue by more than `window_wait_secs` beyond
    ///     the interval, in which case send blind (`true`) so a stale/incorrect window can never
    ///     suppress the probe forever.
    // Was: Prüft, ob send t351 command now zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn should_send_t351_command_now(&self, issi: u32, ts: TdmaTime, interval_secs: u32, window_wait_secs: u64) -> bool {
        let Some(c) = self.clients.get(&issi) else {
            // Unknown client — let the caller proceed; the send is addressed by ISSI and harmless.
            return true;
        };
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        let reachable_now = match ee_cycle_frames(c.energy_saving_mode) {
            None => true, // StayAlive — always reachable
            Some(cycle_len) => match (c.monitoring_frame, c.monitoring_multiframe) {
                (Some(f), Some(mf)) => ts.in_ee_monitoring_window(f, mf, cycle_len),
                _ => true, // EE but the window is not known yet — don't defer
            },
        };
        if reachable_now {
            return true;
        }
        // Sleeping EE MS outside its window: defer to a wake window, but bound the wait.
        c.last_registration_time.elapsed().as_secs() >= interval_secs as u64 + window_wait_secs
    }

    // Was: Diese Funktion setzt client class of ms.
    // Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
    pub fn set_client_class_of_ms(&mut self, issi: u32, class: Option<ClassOfMs>) -> Result<(), ClientMgrErr> {
        if let Some(client) = self.clients.get_mut(&issi) {
            client.class_of_ms = class;
            Ok(())
        } else {
            Err(ClientMgrErr::ClientNotFound { issi })
        }
    }

    /// Store the TEI (Terminal Equipment Identity) received from U-TEI-PROVIDE.
    /// If the ISSI is not registered yet, the TEI is silently ignored (can't fail critically).
    // Was: Diese Funktion setzt client tei.
    // Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
    pub fn set_client_tei(&mut self, issi: u32, tei: u64) -> Result<(), ClientMgrErr> {
        if let Some(client) = self.clients.get_mut(&issi) {
            client.tei = Some(tei);
            Ok(())
        } else {
            Err(ClientMgrErr::ClientNotFound { issi })
        }
    }

    /// Registers a fresh state for a client, based on ssi
    /// If client is already registered, previous state is discarded.
    // Was: Diese Funktion registriert client.
    // Warum: Die Zuordnung bleibt dadurch eindeutig und kann später sauber wieder entfernt werden.
    pub fn try_register_client(&mut self, issi: u32, attached: bool) -> Result<bool, ClientMgrErr> {
        if !is_individual(issi) {
            return Err(ClientMgrErr::IssiInGroupRange { issi });
        };

        // discard previous state if any
        self.clients.remove(&issi);

        // Create and insert new client state
        let mut elem = MmClientProperties::new(issi);
        elem.state = if attached {
            MmClientState::Attached
        } else {
            MmClientState::Unknown
        };
        self.clients.insert(issi, elem);

        // Send telemetry event
        if let Some(sink) = &self.telemetry_sink {
            sink.send(TelemetryEvent::MsRegistration { issi });
        }

        Ok(true)
    }

    /// Removes a client from the registry, returning its properties if found
    /// Returns every known client's ISSI. Used by mm_bs to re-register all MS by ISSI (the L2
    /// handle is inert: MLE addresses downlink MM PDUs by ISSI; `last_handle` is always 0).
    // Was: Führt den Arbeitsschritt `all_known_issis` für all known issis aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn all_known_issis(&self) -> Vec<u32> {
        self.clients.keys().copied().collect()
    }

    /// Per-MS energy-economy monitoring windows, for publishing into shared state so the downlink
    /// scheduler can defer unsolicited traffic to a sleeping MS's wake window. Yields
    /// (issi, monitoring_frame, monitoring_multiframe, cycle_len) for every client that is actually
    /// in an energy-saving mode (not StayAlive) and has a valid monitoring window. StayAlive MSs are
    /// omitted (their absence means "always reachable"). cycle_len is the FRAME-based cycle from
    /// [`ee_cycle_frames`] (Eg1=2, Eg2=3, Eg3=6 — ETSI Table 23.9).
    // Was: Führt den Arbeitsschritt `ee_monitoring_windows` für ee monitoring windows aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn ee_monitoring_windows(&self) -> impl Iterator<Item = (u32, u8, u8, u8)> + '_ {
        self.clients.values().filter_map(|c| {
            let cycle_len = ee_cycle_frames(c.energy_saving_mode)?;
            let frame = c.monitoring_frame?;
            let mframe = c.monitoring_multiframe?;
            // Guard against a malformed window so the scheduler never gates on garbage.
            if cycle_len < 2 || !(1..=18).contains(&frame) || !(1..=60).contains(&mframe) {
                return None;
            }
            Some((c.issi, frame, mframe, cycle_len))
        })
    }

    /// Update the last known L2 handle for a registered client.
    // Was: Diese Funktion setzt client handle.
    // Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
    pub fn set_client_handle(&mut self, issi: u32, handle: u32) {
        if let Some(client) = self.clients.get_mut(&issi) {
            client.last_handle = handle;
        }
    }


    /// Export a subscriber context for forward registration or migration.
    // Was: Führt den Arbeitsschritt `export_mobility_context` für export Mobilität Kontext aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn export_mobility_context(&self, issi: u32) -> Option<MmClientMobilityContext> {
        let client = self.clients.get(&issi)?;
        let mut groups: Vec<u32> = client.groups.iter().copied().collect();
        groups.sort_unstable();
        Some(MmClientMobilityContext {
            issi: client.issi,
            state: client.state,
            groups,
            energy_saving_mode: client.energy_saving_mode,
            monitoring_frame: client.monitoring_frame,
            monitoring_multiframe: client.monitoring_multiframe,
            class_of_ms: client.class_of_ms.clone(),
            last_handle: client.last_handle,
            tei: client.tei,
        })
    }

    /// Import a transferred subscriber context under the local SSI used by this cell.
    /// Runtime timestamps and pending-command state are intentionally re-initialised.
    // Was: Führt den Arbeitsschritt `import_mobility_context` für import Mobilität Kontext aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn import_mobility_context(&mut self, local_issi: u32, context: &MmClientMobilityContext) {
        let mut client = MmClientProperties::new(local_issi);
        client.state = MmClientState::Attached;
        client.energy_saving_mode = context.energy_saving_mode;
        client.monitoring_frame = context.monitoring_frame;
        client.monitoring_multiframe = context.monitoring_multiframe;
        client.class_of_ms = context.class_of_ms.clone();
        client.last_handle = context.last_handle;
        client.tei = context.tei;
        client.groups.extend(context.groups.iter().copied());
        self.clients.insert(local_issi, client);
    }

    /// Project the persistable recovery state of every known client: (issi, groups, energy mode).
    /// Used by restart recovery to snapshot the registry to disk. The L2 handle is intentionally
    /// omitted — it is inert (always 0) in this stack, so there is nothing to persist.
    // Was: Diese Funktion erzeugt for recovery.
    // Warum: Die Oberfläche und andere Dienste erhalten dadurch eine in sich stimmige Momentaufnahme.
    pub fn snapshot_for_recovery(&self) -> Vec<(u32, Vec<u32>, EnergySavingMode)> {
        self.clients
            .values()
            .map(|c| (c.issi, c.groups.iter().copied().collect(), c.energy_saving_mode))
            .collect()
    }

    /// Restore a client loaded from the recovery cache after a BS restart. The client is inserted
    /// as Detached with its persisted groups + energy mode, a FRESH registration timer, and
    /// cleared pending-command flags. This makes it `client_is_known()` — so the coverage-return
    /// re-affiliation fires when it answers our replayed D-LOCATION-UPDATE-COMMAND — while NOT
    /// flagging it for T351 expiry before the replay sweep completes (which would let the
    /// second-expiry REJECT+remove path wipe the restored groups). No CMCE/Brew affiliation is
    /// emitted here; that happens only when the MS actually re-registers.
    // Was: Diese Funktion stellt client.
    // Warum: Nach Verbindungsabbruch oder Neustart kann der vorherige Betriebszustand dadurch kontrolliert zurückkehren.
    pub fn restore_client(&mut self, issi: u32, groups: &[u32], esm: EnergySavingMode) {
        let mut client = MmClientProperties::new(issi);
        client.state = MmClientState::Detached;
        client.energy_saving_mode = esm;
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for &g in groups {
            client.groups.insert(g);
        }
        self.clients.insert(issi, client);
    }

    // Was: Diese Funktion entfernt client.
    // Warum: Das Entfernen bleibt damit vollständig und hinterlässt keinen widersprüchlichen Zustand.
    pub fn remove_client(&mut self, ssi: u32) -> Option<MmClientProperties> {
        if let Some(client) = self.clients.remove(&ssi) {
            // Send telemetry event
            if let Some(sink) = &self.telemetry_sink {
                sink.send(TelemetryEvent::MsDeregistration { issi: ssi });
            }
            Some(client)
        } else {
            None
        }
    }

    /// Detaches all groups from a client
    // Was: Führt den Arbeitsschritt `client_detach_all_groups` für client detach all groups aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn client_detach_all_groups(&mut self, issi: u32) -> Result<bool, ClientMgrErr> {
        if let Some(client) = self.clients.get_mut(&issi) {
            // Send telemetry event
            if let Some(sink) = &self.telemetry_sink {
                sink.send(TelemetryEvent::MsGroupDetach {
                    issi: client.issi,
                    gssis: client.groups.iter().cloned().collect(),
                });
            }
            client.groups.clear();
            Ok(true)
        } else {
            Err(ClientMgrErr::ClientNotFound { issi })
        }
    }

    /// Attaches or detaches a client from a group
    // Was: Führt den Arbeitsschritt `client_group_attach` für client Gruppe attach aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn client_group_attach(&mut self, issi: u32, gssi: u32, do_attach: bool) -> Result<bool, ClientMgrErr> {
        // Checks
        if !in_group_range(gssi) {
            return Err(ClientMgrErr::GssiInClientRange { gssi });
        };
        if !is_group(gssi) {
            return Err(ClientMgrErr::GroupNotFound { gssi });
        };
        if !may_attach(issi, gssi) {
            return Err(ClientMgrErr::GroupNotFound { gssi });
        };

        if let Some(client) = self.clients.get_mut(&issi) {
            if do_attach {
                // Send telemetry event
                if let Some(sink) = &self.telemetry_sink {
                    sink.send(TelemetryEvent::MsGroupAttach {
                        issi: client.issi,
                        gssis: vec![gssi].into_iter().collect(),
                    });
                }

                Ok(client.groups.insert(gssi))
            } else {
                Ok(client.groups.remove(&gssi))
            }
        } else {
            Err(ClientMgrErr::ClientNotFound { issi })
        }
    }
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;
    use std::time::Duration;

    /// The T351 presence guard: a radio heard transmitting recently is reported "on air" so it is
    /// never torn down at expiry (FH-BUG-044 — present stations vanishing from the dashboard).
    #[test]
    // Was: Führt den Arbeitsschritt `heard_on_air_tracks_uplink_presence` für heard on air tracks Uplink (Funkgerät zum Netz) presence aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn heard_on_air_tracks_uplink_presence() {
        let mut mgr = MmClientMgr::new(None);
        mgr.try_register_client(100, true).unwrap();

        // A freshly-registered MS counts as just heard on the air.
        assert!(mgr.heard_on_air_within(100, Duration::from_secs(300)));
        // An unknown ISSI is never "heard".
        assert!(!mgr.heard_on_air_within(999, Duration::from_secs(300)));

        // After a little real time, a very short window reports "not heard" (would fall through to
        // the COMMAND/teardown path), while a generous window still reports "present".
        std::thread::sleep(Duration::from_millis(15));
        assert!(!mgr.heard_on_air_within(100, Duration::from_millis(5)));
        assert!(mgr.heard_on_air_within(100, Duration::from_secs(300)));

        // A fresh uplink burst (RSSI measurement) re-stamps the radio as heard right now.
        std::thread::sleep(Duration::from_millis(15));
        assert!(!mgr.heard_on_air_within(100, Duration::from_millis(5)));
        mgr.update_client_rssi(100, -60.0);
        assert!(mgr.heard_on_air_within(100, Duration::from_millis(5)));
    }

    /// The T351 COMMAND is gated to a sleeping EE radio's wake window so it is never missed
    /// (FH-BUG-044 follow-up). StayAlive radios are always reachable; EE radios are sent only
    /// in their monitoring window.
    #[test]
    // Was: Führt den Arbeitsschritt `t351_command_gated_to_ee_monitoring_window` für t351 command gated to ee monitoring window aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn t351_command_gated_to_ee_monitoring_window() {
        let mut mgr = MmClientMgr::new(None);
        mgr.try_register_client(100, true).unwrap();
        let ts = TdmaTime::default();

        // StayAlive: always reachable → send now.
        assert!(mgr.should_send_t351_command_now(100, ts, 300, 6));
        // Unknown ISSI: caller proceeds.
        assert!(mgr.should_send_t351_command_now(999, ts, 300, 6));

        // Eg1 radio (cycle 2 frames) with a known window at frame 1 / multiframe 1.
        mgr.set_client_energy_saving_mode(100, EnergySavingMode::Eg1).unwrap();
        mgr.set_client_monitoring_window(100, Some(1), Some(1)).unwrap();
        // m=1,f=1 → cur_abs 0, in window (cycle 2) → send.
        assert!(mgr.should_send_t351_command_now(100, TdmaTime { t: 1, f: 1, m: 1, h: 0 }, 300, 6));
        // m=1,f=2 → cur_abs 1, out of window, freshly registered (not overdue) → defer.
        assert!(!mgr.should_send_t351_command_now(100, TdmaTime { t: 1, f: 2, m: 1, h: 0 }, 300, 6));

        // EE radio whose monitoring window is unknown is never deferred (sent immediately).
        mgr.set_client_monitoring_window(100, None, None).unwrap();
        assert!(mgr.should_send_t351_command_now(100, TdmaTime { t: 1, f: 2, m: 1, h: 0 }, 300, 6));
    }
}
