// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Rufaufbau, Rufzustände und Rufwiederherstellung.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::fs;
use std::io::Write;
use std::sync::{Arc, Mutex, mpsc};
use std::time::{Duration, Instant};

use chrono::Utc;
use netcore_contracts::{
    EventSubject, NetCoreEvent, Severity, event_types, subject_types,
};
use netcore_service_common::event_source;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tetra_entities::net_control::{
    ControlCommand, ControlResponse, ManagedCallKind, ManagedCallRestoreContextPayload,
};
use tetra_entities::net_control_room::NodeToControlRoomMessage;
use tetra_entities::net_telemetry::TelemetryEvent;
use uuid::Uuid;

use crate::config::CallControlConfig;
use crate::protocol::{BackendEvent, BackendRequest, GatewaySnapshot};

// Was: Legt den festen Wert `DATABASE_SCHEMA_VERSION` für Datenbank schema version fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const DATABASE_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Deserialize)]
struct MobilityRouteResponse {
    issi: u32,
    state: String,
    registered: bool,
    serving_node: Option<String>,
    node_connected: bool,
    node_stale: bool,
    route_generation: u64,
    confidence: String,
}

fn resolve_mobility_route(config: &crate::config::MobilityCoreClientConfig, issi: u32) -> Result<String, String> {
    let url = format!("{}/api/v1/subscribers/{}/route", config.base_url.trim_end_matches('/'), issi);
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_millis(config.timeout_ms))
        .build()
        .map_err(|error| format!("cannot create Mobility Core client: {error}"))?;
    let response = client.get(&url).send()
        .map_err(|error| format!("Mobility Core route lookup failed for ISSI {issi}: {error}"))?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(format!("Mobility Core has no route for called ISSI {issi}"));
    }
    if !response.status().is_success() {
        return Err(format!("Mobility Core route lookup for ISSI {issi} returned HTTP {}", response.status()));
    }
    let route: MobilityRouteResponse = response.json()
        .map_err(|error| format!("invalid Mobility Core route response for ISSI {issi}: {error}"))?;
    if route.issi != issi {
        return Err(format!("Mobility Core returned route for ISSI {} instead of {issi}", route.issi));
    }
    let acceptable = route.registered && route.serving_node.is_some()
        && ((route.state == "confirmed" && route.node_connected && !route.node_stale)
            || (config.accept_stale_route && route.state == "stale"));
    if !acceptable {
        return Err(format!(
            "called ISSI {issi} has no usable route (state={}, confidence={}, generation={})",
            route.state, route.confidence, route.route_generation
        ));
    }
    Ok(route.serving_node.expect("checked above"))
}

#[derive(Debug, Clone, Serialize)]
// Was: Bündelt die zusammengehörigen Werte für Ruf Steuerung Status in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CallControlStatus {
    pub service: &'static str,
    pub started_at: String,
    pub security_mode: &'static str,
    pub warning: &'static str,
    pub node_gateway_connected: bool,
    pub node_gateway_last_error: Option<String>,
    pub nodes_connected: usize,
    pub nodes_call_control_capable: usize,
    pub participants_registered: usize,
    pub calls_total: usize,
    pub calls_active: usize,
    pub calls_managed: usize,
    pub call_legs_active: usize,
    pub pending_commands: usize,
    pub restores_pending: usize,
    pub database_revision: u64,
}

#[derive(Debug, Clone, Serialize)]
// Was: Bündelt die zusammengehörigen Werte für Netzknoten Datensatz in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct NodeRecord {
    pub node_id: String,
    pub station_name: String,
    pub site: Option<String>,
    pub connected: bool,
    pub stale: bool,
    pub last_seen: String,
    pub call_control_capable: bool,
    pub call_restore_capable: bool,
    pub mcc: u16,
    pub mnc: u16,
    pub location_area: u16,
    pub colour_code: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für participant Datensatz in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ParticipantRecord {
    pub node_id: String,
    pub issi: u32,
    pub registered: bool,
    pub groups: BTreeSet<u32>,
    pub last_seen: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für Ruf kind auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum CallKind {
    Group,
    Individual,
}

// Was: Implementiert das zugehörige Verhalten für `From<ManagedCallKind> for CallKind`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<ManagedCallKind> for CallKind {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(value: ManagedCallKind) -> Self {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match value {
            ManagedCallKind::Group => Self::Group,
            ManagedCallKind::Individual => Self::Individual,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für Ruf phase auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum CallPhase {
    Starting,
    Partial,
    Active,
    Releasing,
    Ended,
    Failed,
    Interrupted,
}

// Was: Implementiert das zugehörige Verhalten für `CallPhase`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl CallPhase {
    // Was: Prüft, ob terminal zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    fn is_terminal(self) -> bool {
        matches!(self, Self::Ended | Self::Failed | Self::Interrupted)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für Rufzweig phase auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum LegPhase {
    Requested,
    Starting,
    Active,
    Releasing,
    Ended,
    Failed,
    TimedOut,
    Offline,
}

// Was: Implementiert das zugehörige Verhalten für `LegPhase`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl LegPhase {
    // Was: Prüft, ob terminal zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    fn is_terminal(self) -> bool {
        matches!(self, Self::Ended | Self::Failed | Self::TimedOut | Self::Offline)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Ruf Rufzweig in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CallLeg {
    pub node_id: String,
    pub local_call_id: Option<u16>,
    pub operation_id: String,
    pub phase: LegPhase,
    pub timeslot: Option<u8>,
    pub carrier_num: Option<u16>,
    pub usage: Option<u8>,
    pub floor_holder: Option<u32>,
    pub queued_issi: Option<u32>,
    pub command_id: Option<String>,
    pub restored: bool,
    pub created_at: String,
    pub updated_at: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für logical Ruf in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LogicalCall {
    pub logical_call_id: String,
    pub operation_id: String,
    pub kind: CallKind,
    pub phase: CallPhase,
    pub managed: bool,
    pub source: String,
    pub source_issi: Option<u32>,
    pub gssi: Option<u32>,
    pub calling_issi: Option<u32>,
    pub called_issi: Option<u32>,
    pub simplex: Option<bool>,
    pub priority: u8,
    pub emergency: bool,
    pub floor_holder: Option<u32>,
    pub floor_queue: Vec<u32>,
    pub legs: BTreeMap<String, CallLeg>,
    pub created_at: String,
    pub updated_at: String,
    pub ended_at: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für Wiederherstellung phase auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum RestorePhase {
    ExportQueued,
    ExportRequested,
    ImportQueued,
    ImportRequested,
    Ready,
    Completed,
    Cancelled,
    Failed,
    TimedOut,
}

// Was: Implementiert das zugehörige Verhalten für `RestorePhase`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl RestorePhase {
    // Was: Prüft, ob terminal zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Cancelled | Self::Failed | Self::TimedOut
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Wiederherstellung operation in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RestoreOperation {
    pub restore_id: String,
    pub logical_call_id: String,
    pub source_node: String,
    pub target_node: String,
    pub source_call_id: u16,
    pub target_call_id: Option<u16>,
    pub phase: RestorePhase,
    pub context: Option<ManagedCallRestoreContextPayload>,
    pub created_at: String,
    pub updated_at: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
// Was: Bündelt die zusammengehörigen Werte für Ereignis Datensatz in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct EventRecord {
    pub seq: u64,
    pub timestamp: String,
    pub kind: String,
    pub node_id: Option<String>,
    pub logical_call_id: Option<String>,
    pub local_call_id: Option<u16>,
    pub detail: Value,
    pub canonical: NetCoreEvent,
}

#[derive(Debug, Clone, Serialize)]
// Was: Bündelt die zusammengehörigen Werte für Audio- und Mediendaten Steuerung Ereignis in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MediaControlEvent {
    pub kind: String,
    pub revision: u64,
    pub emitted_at: String,
    pub reason: String,
    pub logical_call_id: Option<String>,
    pub calls: Vec<LogicalCall>,
}

#[derive(Debug, Clone, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Audio- und Mediendaten Weiterleitung ready input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MediaRouteReadyInput {
    pub logical_call_id: String,
    pub revision: u64,
}

#[derive(Debug, Clone, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Gruppe Ruf input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct GroupCallInput {
    pub gssi: u32,
    pub source_issi: u32,
    #[serde(default)]
    pub priority: u8,
    #[serde(default)]
    pub target_nodes: BTreeSet<String>,
}

#[derive(Debug, Clone, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für individual Ruf input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct IndividualCallInput {
    pub calling_issi: u32,
    pub called_issi: u32,
    #[serde(default = "default_true")]
    pub simplex: bool,
    #[serde(default)]
    pub priority: u8,
    pub target_node: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für floor input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct FloorInput {
    pub source_issi: u32,
    #[serde(default)]
    pub force: bool,
}

#[derive(Debug, Clone, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Wiederherstellung input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RestoreInput {
    pub logical_call_id: String,
    pub source_node: String,
    pub target_node: String,
    pub source_call_id: Option<u16>,
}

// Was: Führt den Arbeitsschritt `default_true` für default true aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_true() -> bool {
    true
}

#[derive(Debug, Clone)]
// Was: Listet die möglichen Varianten für pending action auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
enum PendingAction {
    StartLeg { logical_call_id: String },
    ReleaseLeg { logical_call_id: String },
    FloorRequest { logical_call_id: String },
    FloorRelease { logical_call_id: String },
    ExportRestore { restore_id: String },
    ImportRestore { restore_id: String },
    RemoveRestore { restore_id: String },
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für pending command in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct PendingCommand {
    request_id: String,
    command_id: Option<String>,
    node_id: String,
    handle: u32,
    created: Instant,
    action: PendingAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für persisted Datenbank in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct PersistedDatabase {
    schema_version: u32,
    revision: u64,
    calls: Vec<LogicalCall>,
    restores: Vec<RestoreOperation>,
}

// Was: Implementiert das zugehörige Verhalten für `Default for PersistedDatabase`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for PersistedDatabase {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            schema_version: DATABASE_SCHEMA_VERSION,
            revision: 0,
            calls: Vec::new(),
            restores: Vec::new(),
        }
    }
}

// Was: Bündelt die zusammengehörigen Werte für Ruf Zustand in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct CallState {
    config: CallControlConfig,
    started_at: String,
    gateway_connected: bool,
    gateway_last_error: Option<String>,
    nodes: BTreeMap<String, NodeRecord>,
    participants: BTreeMap<(String, u32), ParticipantRecord>,
    calls: BTreeMap<String, LogicalCall>,
    restores: BTreeMap<String, RestoreOperation>,
    pending: HashMap<String, PendingCommand>,
    events: VecDeque<EventRecord>,
    next_event_seq: u64,
    next_handle: u32,
    database_revision: u64,
    media_revision: u64,
    media_required_revision: HashMap<String, u64>,
    media_ready_revision: HashMap<String, u64>,
    media_subscribers: Vec<mpsc::Sender<MediaControlEvent>>,
}

#[derive(Clone)]
// Was: Bündelt die zusammengehörigen Werte für shared calls in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct SharedCalls(Arc<Mutex<CallState>>);

// Was: Implementiert das zugehörige Verhalten für `SharedCalls`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl SharedCalls {
    // Was: Diese Funktion lädt den vorgesehenen Arbeitsschritt.
    // Warum: Einlesen und Fehlerbehandlung bleiben dadurch an einer zentralen Stelle.
    pub fn load(config: CallControlConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let database = load_database(&config)?;
        let now = now();
        let mut calls = BTreeMap::new();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for mut call in database.calls {
            if !call.phase.is_terminal() {
                call.phase = CallPhase::Interrupted;
                call.ended_at = Some(now.clone());
                call.updated_at = now.clone();
                call.message = "service restarted while call was active".to_string();
                // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
                // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
                for leg in call.legs.values_mut() {
                    if !leg.phase.is_terminal() {
                        leg.phase = LegPhase::Offline;
                        leg.updated_at = now.clone();
                        leg.message = "call-control restart".to_string();
                    }
                }
            }
            calls.insert(call.logical_call_id.clone(), call);
        }
        let mut restores = BTreeMap::new();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for mut operation in database.restores {
            if !operation.phase.is_terminal() {
                operation.phase = RestorePhase::Failed;
                operation.updated_at = now.clone();
                operation.message = "service restarted during restore coordination".to_string();
            }
            restores.insert(operation.restore_id.clone(), operation);
        }

        let media_required_revision = calls
            .values()
            .filter(|call| !call.phase.is_terminal())
            .map(|call| (call.logical_call_id.clone(), database.revision))
            .collect();

        Ok(Self(Arc::new(Mutex::new(CallState {
            config,
            started_at: now,
            gateway_connected: false,
            gateway_last_error: None,
            nodes: BTreeMap::new(),
            participants: BTreeMap::new(),
            calls,
            restores,
            pending: HashMap::new(),
            events: VecDeque::new(),
            next_event_seq: 1,
            next_handle: 1,
            database_revision: database.revision,
            media_revision: database.revision,
            media_required_revision,
            media_ready_revision: HashMap::new(),
            media_subscribers: Vec::new(),
        }))))
    }

    // Was: Diese Funktion abonniert Audio- und Mediendaten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn subscribe_media(&self) -> mpsc::Receiver<MediaControlEvent> {
        let (tx, rx) = mpsc::channel();
        let mut state = self.0.lock().expect("call state poisoned");
        state.media_revision = state.media_revision.saturating_add(1);
        let revision = state.media_revision;
        let active_calls = state
            .calls
            .values()
            .filter(|call| !call.phase.is_terminal())
            .map(|call| call.logical_call_id.clone())
            .collect::<Vec<_>>();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for logical_call_id in active_calls {
            state
                .media_required_revision
                .insert(logical_call_id.clone(), revision);
            state.media_ready_revision.remove(&logical_call_id);
        }
        let snapshot = state.media_event("snapshot", "initial_snapshot", None);
        state
            .media_subscribers
            .retain(|subscriber| subscriber.send(snapshot.clone()).is_ok());
        let _ = tx.send(snapshot);
        state.media_subscribers.push(tx);
        rx
    }

    // Was: Diese Funktion kennzeichnet Audio- und Mediendaten Weiterleitung ready.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn mark_media_route_ready(
        &self,
        input: MediaRouteReadyInput,
    ) -> Result<Value, String> {
        let mut state = self.0.lock().expect("call state poisoned");
        let call = state
            .calls
            .get(&input.logical_call_id)
            .cloned()
            .ok_or_else(|| "logical call not found".to_string())?;
        if call.phase.is_terminal() {
            return Err("logical call is terminal".to_string());
        }
        if input.revision > state.media_revision {
            return Err("media route acknowledgement is from a future revision".to_string());
        }
        let required_revision = state
            .media_required_revision
            .get(&input.logical_call_id)
            .copied()
            .unwrap_or(state.media_revision);
        if input.revision < required_revision {
            return Err(format!(
                "stale media route acknowledgement: revision {} is older than required revision {}",
                input.revision, required_revision
            ));
        }
        let nonterminal_legs = call
            .legs
            .values()
            .filter(|leg| !leg.phase.is_terminal())
            .collect::<Vec<_>>();
        let legs_ready = !nonterminal_legs.is_empty()
            && nonterminal_legs.iter().all(|leg| {
                leg.phase == LegPhase::Active
                    && leg.local_call_id.is_some()
                    && leg.timeslot.is_some()
            });
        if !legs_ready {
            return Err("media route is not ready for every non-terminal call leg".to_string());
        }

        state
            .media_ready_revision
            .insert(input.logical_call_id.clone(), input.revision);
        state.push_event(
            "media_route_ready",
            None,
            Some(input.logical_call_id.clone()),
            None,
            json!({"revision": input.revision}),
        );
        Ok(json!({
            "logical_call_id": input.logical_call_id,
            "revision": input.revision,
            "route_ready": true
        }))
    }

    // Was: Führt den Arbeitsschritt `status` für Status aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn status(&self) -> CallControlStatus {
        let state = self.0.lock().expect("call state poisoned");
        let calls_active = state.calls.values().filter(|call| !call.phase.is_terminal()).count();
        let calls_managed = state.calls.values().filter(|call| call.managed).count();
        let call_legs_active = state
            .calls
            .values()
            .flat_map(|call| call.legs.values())
            .filter(|leg| !leg.phase.is_terminal())
            .count();
        CallControlStatus {
            service: "netcore-call-control",
            started_at: state.started_at.clone(),
            security_mode: "open_lab",
            warning: "isolated test network only; every reachable client may control calls and floor",
            node_gateway_connected: state.gateway_connected,
            node_gateway_last_error: state.gateway_last_error.clone(),
            nodes_connected: state.nodes.values().filter(|node| node.connected).count(),
            nodes_call_control_capable: state
                .nodes
                .values()
                .filter(|node| node.connected && node.call_control_capable)
                .count(),
            participants_registered: state
                .participants
                .values()
                .filter(|participant| participant.registered)
                .count(),
            calls_total: state.calls.len(),
            calls_active,
            calls_managed,
            call_legs_active,
            pending_commands: state.pending.len(),
            restores_pending: state
                .restores
                .values()
                .filter(|operation| !operation.phase.is_terminal())
                .count(),
            database_revision: state.database_revision,
        }
    }

    // Was: Führt den Arbeitsschritt `config_view` für Konfiguration view aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn config_view(&self) -> Value {
        let state = self.0.lock().expect("call state poisoned");
        json!({
            "server": &state.config.server,
            "node_gateway": &state.config.node_gateway,
            "storage": &state.config.storage,
            "calls": &state.config.calls,
            "security": &state.config.security,
            "limits": &state.config.limits,
            "effective": {
                "token_auth": false,
                "tls": false,
                "login": false,
                "rbac": false
            }
        })
    }

    // Was: Führt den Arbeitsschritt `nodes` für nodes aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn nodes(&self) -> Vec<NodeRecord> {
        self.0
            .lock()
            .expect("call state poisoned")
            .nodes
            .values()
            .cloned()
            .collect()
    }

    // Was: Führt den Arbeitsschritt `participants` für participants aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn participants(&self) -> Vec<ParticipantRecord> {
        self.0
            .lock()
            .expect("call state poisoned")
            .participants
            .values()
            .cloned()
            .collect()
    }

    // Was: Führt den Arbeitsschritt `calls` für calls aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn calls(&self) -> Vec<LogicalCall> {
        let state = self.0.lock().expect("call state poisoned");
        let mut calls: Vec<_> = state.calls.values().cloned().collect();
        calls.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        calls
    }

    // Was: Führt den Arbeitsschritt `call` für Ruf aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn call(&self, logical_call_id: &str) -> Option<LogicalCall> {
        self.0
            .lock()
            .expect("call state poisoned")
            .calls
            .get(logical_call_id)
            .cloned()
    }

    // Was: Führt den Arbeitsschritt `restores` für restores aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn restores(&self) -> Vec<RestoreOperation> {
        let state = self.0.lock().expect("call state poisoned");
        let mut operations: Vec<_> = state.restores.values().cloned().collect();
        operations.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        operations
    }

    // Was: Führt den Arbeitsschritt `events` für events aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn events(&self, limit: usize) -> Vec<EventRecord> {
        self.0
            .lock()
            .expect("call state poisoned")
            .events
            .iter()
            .rev()
            .take(limit.max(1))
            .cloned()
            .collect()
    }

    /// Liefert die Ereignisse im gemeinsamen `netcore-event-v1`-Format.
    pub fn netcore_events(&self, limit: usize) -> Vec<NetCoreEvent> {
        self.0
            .lock()
            .expect("call state poisoned")
            .events
            .iter()
            .rev()
            .take(limit.max(1))
            .map(|event| event.canonical.clone())
            .collect()
    }

    // Was: Führt den Arbeitsschritt `metrics` für Messwerte aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn metrics(&self) -> String {
        let status = self.status();
        format!(
            concat!(
                "# HELP netcore_call_control_gateway_connected Node Gateway connection state.\n",
                "# TYPE netcore_call_control_gateway_connected gauge\n",
                "netcore_call_control_gateway_connected {}\n",
                "# HELP netcore_call_control_calls_active Active logical calls.\n",
                "# TYPE netcore_call_control_calls_active gauge\n",
                "netcore_call_control_calls_active {}\n",
                "# HELP netcore_call_control_call_legs_active Active TBS call legs.\n",
                "# TYPE netcore_call_control_call_legs_active gauge\n",
                "netcore_call_control_call_legs_active {}\n",
                "# HELP netcore_call_control_pending_commands Pending TBS commands.\n",
                "# TYPE netcore_call_control_pending_commands gauge\n",
                "netcore_call_control_pending_commands {}\n",
                "# HELP netcore_call_control_restores_pending Pending restore operations.\n",
                "# TYPE netcore_call_control_restores_pending gauge\n",
                "netcore_call_control_restores_pending {}\n"
            ),
            u8::from(status.node_gateway_connected),
            status.calls_active,
            status.call_legs_active,
            status.pending_commands,
            status.restores_pending,
        )
    }

    // Was: Diese Funktion erstellt Gruppe Ruf.
    // Warum: Neue Objekte erhalten so immer einen vollständigen und gültigen Ausgangszustand.
    pub fn create_group_call(
        &self,
        input: GroupCallInput,
    ) -> Result<(LogicalCall, Vec<BackendRequest>), String> {
        let mut state = self.0.lock().expect("call state poisoned");
        state.validate_ssi(input.gssi, "GSSI")?;
        state.validate_ssi(input.source_issi, "source ISSI")?;
        state.ensure_call_capacity()?;

        let targets = state.select_group_targets(input.gssi, input.target_nodes)?;
        let logical_call_id = Uuid::new_v4().to_string();
        let operation_id = Uuid::new_v4().to_string();
        let timestamp = now();
        let mut call = LogicalCall {
            logical_call_id: logical_call_id.clone(),
            operation_id: operation_id.clone(),
            kind: CallKind::Group,
            phase: CallPhase::Starting,
            managed: true,
            source: "operator".to_string(),
            source_issi: Some(input.source_issi),
            gssi: Some(input.gssi),
            calling_issi: None,
            called_issi: None,
            simplex: Some(true),
            priority: input.priority.min(15),
            emergency: input.priority >= 15,
            floor_holder: Some(input.source_issi),
            floor_queue: Vec::new(),
            legs: BTreeMap::new(),
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
            ended_at: None,
            message: "group-call start requested".to_string(),
        };

        let mut requests = Vec::new();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for node_id in targets {
            let leg = CallLeg {
                node_id: node_id.clone(),
                local_call_id: None,
                operation_id: operation_id.clone(),
                phase: LegPhase::Requested,
                timeslot: None,
                carrier_num: None,
                usage: None,
                floor_holder: Some(input.source_issi),
                queued_issi: None,
                command_id: None,
                restored: false,
                created_at: timestamp.clone(),
                updated_at: timestamp.clone(),
                message: "start command queued".to_string(),
            };
            call.legs.insert(node_id.clone(), leg);
            let handle = state.next_handle();
            let command = ControlCommand::CallControlGroupStart {
                handle,
                operation_id: operation_id.clone(),
                source_issi: input.source_issi,
                gssi: input.gssi,
                priority: input.priority.min(15),
            };
            requests.push(state.register_command(
                node_id,
                handle,
                command,
                PendingAction::StartLeg {
                    logical_call_id: logical_call_id.clone(),
                },
            )?);
        }
        state.calls.insert(logical_call_id.clone(), call.clone());
        state.push_event(
            "group_call_requested",
            None,
            Some(logical_call_id),
            None,
            json!({"gssi": input.gssi, "source_issi": input.source_issi, "targets": call.legs.keys().collect::<Vec<_>>() }),
        );
        state.bump_and_persist();
        Ok((call, requests))
    }

    // Was: Diese Funktion erstellt individual Ruf.
    // Warum: Neue Objekte erhalten so immer einen vollständigen und gültigen Ausgangszustand.
    pub fn create_individual_call(
        &self,
        input: IndividualCallInput,
    ) -> Result<(LogicalCall, Vec<BackendRequest>), String> {
        let explicit_target = input.target_node.clone();
        let mobility_config = {
            let state = self.0.lock().expect("call state poisoned");
            state.config.mobility_core.clone()
        };
        let canonical_target = if explicit_target.is_none() && mobility_config.enabled {
            match resolve_mobility_route(&mobility_config, input.called_issi) {
                Ok(node_id) => Some(node_id),
                Err(error) if mobility_config.allow_local_fallback => {
                    tracing::warn!("{}; falling back to local Call Control participant cache", error);
                    None
                }
                Err(error) => return Err(error),
            }
        } else {
            None
        };

        let mut state = self.0.lock().expect("call state poisoned");
        state.validate_ssi(input.calling_issi, "calling ISSI")?;
        state.validate_ssi(input.called_issi, "called ISSI")?;
        state.ensure_call_capacity()?;
        let target = state.select_individual_target(input.called_issi, explicit_target.or(canonical_target))?;

        let logical_call_id = Uuid::new_v4().to_string();
        let operation_id = Uuid::new_v4().to_string();
        let timestamp = now();
        let mut call = LogicalCall {
            logical_call_id: logical_call_id.clone(),
            operation_id: operation_id.clone(),
            kind: CallKind::Individual,
            phase: CallPhase::Starting,
            managed: true,
            source: "operator".to_string(),
            source_issi: Some(input.calling_issi),
            gssi: None,
            calling_issi: Some(input.calling_issi),
            called_issi: Some(input.called_issi),
            simplex: Some(input.simplex),
            priority: input.priority.min(15),
            emergency: input.priority >= 15,
            floor_holder: None,
            floor_queue: Vec::new(),
            legs: BTreeMap::new(),
            created_at: timestamp.clone(),
            updated_at: timestamp.clone(),
            ended_at: None,
            message: "individual-call setup requested".to_string(),
        };
        call.legs.insert(
            target.clone(),
            CallLeg {
                node_id: target.clone(),
                local_call_id: None,
                operation_id: operation_id.clone(),
                phase: LegPhase::Requested,
                timeslot: None,
                carrier_num: None,
                usage: None,
                floor_holder: None,
                queued_issi: None,
                command_id: None,
                restored: false,
                created_at: timestamp.clone(),
                updated_at: timestamp.clone(),
                message: "setup command queued".to_string(),
            },
        );
        let handle = state.next_handle();
        let request = state.register_command(
            target,
            handle,
            ControlCommand::CallControlIndividualStart {
                handle,
                operation_id: operation_id.clone(),
                calling_issi: input.calling_issi,
                called_issi: input.called_issi,
                simplex: input.simplex,
                priority: input.priority.min(15),
            },
            PendingAction::StartLeg {
                logical_call_id: logical_call_id.clone(),
            },
        )?;
        state.calls.insert(logical_call_id.clone(), call.clone());
        state.push_event(
            "individual_call_requested",
            None,
            Some(logical_call_id),
            None,
            json!({"calling_issi": input.calling_issi, "called_issi": input.called_issi, "target_node": call.legs.keys().next()}),
        );
        state.bump_and_persist();
        Ok((call, vec![request]))
    }

    // Was: Diese Funktion gibt Ruf.
    // Warum: Ressourcen werden dadurch rechtzeitig freigegeben und blockieren keine weiteren Vorgänge.
    pub fn release_call(
        &self,
        logical_call_id: &str,
    ) -> Result<Vec<BackendRequest>, String> {
        let mut state = self.0.lock().expect("call state poisoned");
        let call = state
            .calls
            .get(logical_call_id)
            .cloned()
            .ok_or_else(|| "logical call not found".to_string())?;
        if call.phase.is_terminal() {
            return Err("logical call is already terminal".to_string());
        }
        let mut requests = Vec::new();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for leg in call.legs.values().filter(|leg| !leg.phase.is_terminal()) {
            let Some(call_id) = leg.local_call_id else {
                continue;
            };
            let handle = state.next_handle();
            requests.push(state.register_command(
                leg.node_id.clone(),
                handle,
                ControlCommand::CallControlRelease {
                    handle,
                    call_id,
                    cause: 1,
                },
                PendingAction::ReleaseLeg {
                    logical_call_id: logical_call_id.to_string(),
                },
            )?);
        }
        if let Some(call) = state.calls.get_mut(logical_call_id) {
            call.phase = CallPhase::Releasing;
            call.updated_at = now();
            call.message = "release requested".to_string();
            // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
            // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
            for leg in call.legs.values_mut().filter(|leg| !leg.phase.is_terminal()) {
                leg.phase = LegPhase::Releasing;
                leg.updated_at = now();
            }
        }
        state.push_event(
            "call_release_requested",
            None,
            Some(logical_call_id.to_string()),
            None,
            json!({"commands": requests.len()}),
        );
        state.bump_and_persist();
        Ok(requests)
    }

    // Was: Diese Funktion fordert floor.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn request_floor(
        &self,
        logical_call_id: &str,
        input: FloorInput,
    ) -> Result<Vec<BackendRequest>, String> {
        let mut state = self.0.lock().expect("call state poisoned");
        state.validate_ssi(input.source_issi, "source ISSI")?;
        if input.force && !state.config.calls.allow_operator_force_floor {
            return Err("force-floor is disabled by configuration".to_string());
        }
        let call = state
            .calls
            .get(logical_call_id)
            .cloned()
            .ok_or_else(|| "logical call not found".to_string())?;
        if call.phase.is_terminal() {
            return Err("logical call is terminal".to_string());
        }
        let nonterminal_legs = call
            .legs
            .values()
            .filter(|leg| !leg.phase.is_terminal())
            .collect::<Vec<_>>();
        let media_route_ready = !nonterminal_legs.is_empty()
            && nonterminal_legs.iter().all(|leg| {
                leg.phase == LegPhase::Active
                    && leg.local_call_id.is_some()
                    && leg.timeslot.is_some()
            });
        if !media_route_ready {
            return Err(
                "floor is held until all non-terminal call legs are media-ready".to_string(),
            );
        }
        let required_revision = state
            .media_required_revision
            .get(logical_call_id)
            .copied()
            .unwrap_or(state.media_revision);
        let ready_revision = state
            .media_ready_revision
            .get(logical_call_id)
            .copied()
            .unwrap_or(0);
        if ready_revision < required_revision {
            return Err(format!(
                "floor is held until Media Switch confirms RouteReady for revision {}",
                required_revision
            ));
        }

        let mut requests = Vec::new();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for leg in call.legs.values().filter(|leg| leg.phase == LegPhase::Active) {
            let Some(call_id) = leg.local_call_id else {
                continue;
            };
            let handle = state.next_handle();
            requests.push(state.register_command(
                leg.node_id.clone(),
                handle,
                ControlCommand::CallControlFloorRequest {
                    handle,
                    call_id,
                    source_issi: input.source_issi,
                    force: input.force,
                },
                PendingAction::FloorRequest {
                    logical_call_id: logical_call_id.to_string(),
                },
            )?);
        }
        if requests.is_empty() {
            return Err("no active call leg can receive floor control".to_string());
        }
        state.push_event(
            "floor_requested",
            None,
            Some(logical_call_id.to_string()),
            None,
            json!({"source_issi": input.source_issi, "force": input.force, "commands": requests.len()}),
        );
        state.bump_and_persist();
        Ok(requests)
    }

    // Was: Diese Funktion gibt floor.
    // Warum: Ressourcen werden dadurch rechtzeitig freigegeben und blockieren keine weiteren Vorgänge.
    pub fn release_floor(
        &self,
        logical_call_id: &str,
    ) -> Result<Vec<BackendRequest>, String> {
        let mut state = self.0.lock().expect("call state poisoned");
        let call = state
            .calls
            .get(logical_call_id)
            .cloned()
            .ok_or_else(|| "logical call not found".to_string())?;
        let mut requests = Vec::new();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for leg in call.legs.values().filter(|leg| leg.phase == LegPhase::Active) {
            let Some(call_id) = leg.local_call_id else {
                continue;
            };
            let handle = state.next_handle();
            requests.push(state.register_command(
                leg.node_id.clone(),
                handle,
                ControlCommand::CallControlFloorRelease { handle, call_id },
                PendingAction::FloorRelease {
                    logical_call_id: logical_call_id.to_string(),
                },
            )?);
        }
        if requests.is_empty() {
            return Err("no active call leg can release floor".to_string());
        }
        state.push_event(
            "floor_release_requested",
            None,
            Some(logical_call_id.to_string()),
            None,
            json!({"commands": requests.len()}),
        );
        state.bump_and_persist();
        Ok(requests)
    }

    // Was: Diese Funktion erstellt Wiederherstellung.
    // Warum: Neue Objekte erhalten so immer einen vollständigen und gültigen Ausgangszustand.
    pub fn create_restore(
        &self,
        input: RestoreInput,
    ) -> Result<(RestoreOperation, Vec<BackendRequest>), String> {
        let mut state = self.0.lock().expect("call state poisoned");
        if input.source_node == input.target_node {
            return Err("source and target node must differ".to_string());
        }
        state.require_node(&input.source_node, true)?;
        state.require_node(&input.target_node, true)?;
        let call = state
            .calls
            .get(&input.logical_call_id)
            .cloned()
            .ok_or_else(|| "logical call not found".to_string())?;
        let source_call_id = input
            .source_call_id
            .or_else(|| call.legs.get(&input.source_node).and_then(|leg| leg.local_call_id))
            .ok_or_else(|| "source leg has no local call identifier".to_string())?;
        if call
            .legs
            .get(&input.target_node)
            .is_some_and(|leg| !leg.phase.is_terminal())
        {
            return Err("target node already has a non-terminal leg for this call".to_string());
        }
        if state
            .restores
            .values()
            .any(|operation| !operation.phase.is_terminal() && operation.logical_call_id == input.logical_call_id)
        {
            return Err("a restore operation is already active for this logical call".to_string());
        }

        let restore_id = Uuid::new_v4().to_string();
        let timestamp = now();
        let operation = RestoreOperation {
            restore_id: restore_id.clone(),
            logical_call_id: input.logical_call_id.clone(),
            source_node: input.source_node.clone(),
            target_node: input.target_node.clone(),
            source_call_id,
            target_call_id: None,
            phase: RestorePhase::ExportQueued,
            context: None,
            created_at: timestamp.clone(),
            updated_at: timestamp,
            message: "source context export queued".to_string(),
        };
        state.restores.insert(restore_id.clone(), operation.clone());
        if let Some(call) = state.calls.get_mut(&input.logical_call_id) {
            let timestamp = now();
            call.legs.entry(input.target_node.clone()).or_insert(CallLeg {
                node_id: input.target_node.clone(),
                local_call_id: None,
                operation_id: call.operation_id.clone(),
                phase: LegPhase::Starting,
                timeslot: None,
                carrier_num: None,
                usage: None,
                floor_holder: call.floor_holder,
                queued_issi: None,
                command_id: None,
                restored: true,
                created_at: timestamp.clone(),
                updated_at: timestamp,
                message: "restore context pending; awaiting target radio leg".to_string(),
            });
            call.updated_at = now();
            call.message = "call restore in progress".to_string();
        }
        let handle = state.next_handle();
        let request = state.register_command(
            input.source_node,
            handle,
            ControlCommand::CallControlExportRestoreContext {
                handle,
                call_id: source_call_id,
            },
            PendingAction::ExportRestore {
                restore_id: restore_id.clone(),
            },
        )?;
        if let Some(operation) = state.restores.get_mut(&restore_id) {
            operation.phase = RestorePhase::ExportRequested;
            operation.updated_at = now();
        }
        state.push_event(
            "restore_requested",
            None,
            Some(input.logical_call_id),
            Some(source_call_id),
            json!({"restore_id": restore_id, "source_node": operation.source_node, "target_node": operation.target_node}),
        );
        let operation = state
            .restores
            .get(&restore_id)
            .cloned()
            .expect("restore inserted before command registration");
        state.bump_and_persist();
        Ok((operation, vec![request]))
    }

    // Was: Diese Funktion bricht Wiederherstellung.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn cancel_restore(&self, restore_id: &str) -> Result<(), String> {
        let mut state = self.0.lock().expect("call state poisoned");
        let (logical_call_id, source_call_id, target_node) = {
            let operation = state
                .restores
                .get_mut(restore_id)
                .ok_or_else(|| "restore operation not found".to_string())?;
            if operation.phase.is_terminal() {
                return Err("restore operation is already terminal".to_string());
            }
            operation.phase = RestorePhase::Cancelled;
            operation.updated_at = now();
            operation.message = "cancelled by operator".to_string();
            (
                operation.logical_call_id.clone(),
                operation.source_call_id,
                operation.target_node.clone(),
            )
        };
        state.pending.retain(|_, pending| match &pending.action {
            PendingAction::ExportRestore { restore_id: value }
            | PendingAction::ImportRestore { restore_id: value }
            | PendingAction::RemoveRestore { restore_id: value } => value != restore_id,
            _ => true,
        });
        if let Some(call) = state.calls.get_mut(&logical_call_id) {
            let remove_placeholder = call
                .legs
                .get(&target_node)
                .is_some_and(|leg| leg.restored && leg.local_call_id.is_none());
            if remove_placeholder {
                call.legs.remove(&target_node);
            }
            CallState::recompute_call_phase(call);
        }
        state.push_event(
            "restore_cancelled",
            None,
            Some(logical_call_id),
            Some(source_call_id),
            json!({"restore_id": restore_id}),
        );
        state.bump_and_persist();
        Ok(())
    }

    // Was: Führt den Arbeitsschritt `gateway_connected` für Gateway connected aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn gateway_connected(&self) {
        let mut state = self.0.lock().expect("call state poisoned");
        state.gateway_connected = true;
        state.gateway_last_error = None;
        state.push_event("gateway_connected", None, None, None, json!({}));
    }

    // Was: Führt den Arbeitsschritt `gateway_disconnected` für Gateway disconnected aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn gateway_disconnected(&self, error: String) {
        let mut state = self.0.lock().expect("call state poisoned");
        state.gateway_connected = false;
        state.gateway_last_error = Some(error.clone());
        state.push_event(
            "gateway_disconnected",
            None,
            None,
            None,
            json!({"error": error}),
        );
    }

    // Was: Diese Funktion verarbeitet Hintergrunddienst Ereignis.
    // Warum: Die Reaktion auf dieses Ereignis bleibt damit an einer Stelle nachvollziehbar.
    pub fn handle_backend_event(&self, event: BackendEvent) -> Vec<BackendRequest> {
        let mut state = self.0.lock().expect("call state poisoned");
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        let requests = match event {
            BackendEvent::Snapshot { snapshot } => {
                state.apply_snapshot(snapshot);
                Vec::new()
            }
            BackendEvent::Event { event } => {
                if event.kind.contains("disconnect") {
                    if let Some(node_id) = event.node_id {
                        state.mark_node_disconnected(&node_id, &event.kind);
                    }
                }
                Vec::new()
            }
            BackendEvent::ActionResult {
                request_id,
                command_id,
                ok,
                message,
            } => {
                state.handle_action_result(request_id, command_id, ok, message);
                Vec::new()
            }
            BackendEvent::NodeMessage { node_id, message } => {
                state.handle_node_message(node_id, message)
            }
        };
        state.bump_and_persist_if_changed();
        requests
    }

    // Was: Führt den Arbeitsschritt `expire_operations` für expire operations aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn expire_operations(&self) -> Vec<BackendRequest> {
        let mut state = self.0.lock().expect("call state poisoned");
        let now_instant = Instant::now();
        let command_timeout = Duration::from_secs(state.config.calls.command_timeout_secs);
        let restore_timeout = Duration::from_secs(state.config.calls.restore_timeout_secs);
        let expired: Vec<String> = state
            .pending
            .iter()
            .filter_map(|(id, pending)| {
                (now_instant.duration_since(pending.created) >= command_timeout).then_some(id.clone())
            })
            .collect();
        let mut changed = !expired.is_empty();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for request_id in expired {
            if let Some(pending) = state.pending.remove(&request_id) {
                state.fail_pending(pending, "command timed out".to_string(), true);
            }
        }
        let timed_out: Vec<String> = state
            .restores
            .values()
            .filter(|operation| !operation.phase.is_terminal())
            .filter_map(|operation| {
                let age = chrono::DateTime::parse_from_rfc3339(&operation.created_at)
                    .ok()
                    .map(|created| Utc::now().signed_duration_since(created.with_timezone(&Utc)))
                    .unwrap_or_default();
                (age.to_std().unwrap_or_default() >= restore_timeout)
                    .then(|| operation.restore_id.clone())
            })
            .collect();
        changed |= !timed_out.is_empty();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for restore_id in timed_out {
            state.remove_pending_for_restore(&restore_id);
            let snapshot = state.restores.get_mut(&restore_id).map(|operation| {
                operation.phase = RestorePhase::TimedOut;
                operation.updated_at = now();
                operation.message = "restore coordination timed out".to_string();
                (
                    operation.logical_call_id.clone(),
                    operation.source_call_id,
                    operation.target_node.clone(),
                )
            });
            if let Some((logical_call_id, source_call_id, target_node)) = snapshot {
                if let Some(call) = state.calls.get_mut(&logical_call_id) {
                    let remove_placeholder = call
                        .legs
                        .get(&target_node)
                        .is_some_and(|leg| leg.restored && leg.local_call_id.is_none());
                    if remove_placeholder {
                        call.legs.remove(&target_node);
                    }
                    CallState::recompute_call_phase(call);
                }
                state.push_event(
                    "restore_timed_out",
                    None,
                    Some(logical_call_id),
                    Some(source_call_id),
                    json!({"restore_id": restore_id}),
                );
            }
        }
        if changed {
            state.bump_and_persist();
        }
        Vec::new()
    }
}

// Was: Implementiert das zugehörige Verhalten für `CallState`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl CallState {
    // Was: Diese Funktion prüft TETRA-Teilnehmerkennung (SSI).
    // Warum: Unzulässige Werte werden dadurch erkannt, bevor sie im Betrieb Schaden anrichten.
    fn validate_ssi(&self, value: u32, label: &str) -> Result<(), String> {
        if value == 0 || value > 0x00ff_ffff {
            Err(format!("{label} must be in 1..=16777215"))
        } else {
            Ok(())
        }
    }

    // Was: Diese Funktion stellt Ruf capacity.
    // Warum: So wird die notwendige Voraussetzung hergestellt, bevor abhängiger Code weiterläuft.
    fn ensure_call_capacity(&self) -> Result<(), String> {
        if self.calls.len() >= self.config.limits.max_calls {
            Err("call database limit reached".to_string())
        } else {
            Ok(())
        }
    }

    // Was: Führt den Arbeitsschritt `next_handle` für next handle aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn next_handle(&mut self) -> u32 {
        let handle = self.next_handle.max(1);
        self.next_handle = self.next_handle.wrapping_add(1).max(1);
        handle
    }

    // Was: Führt den Arbeitsschritt `require_node` für require Netzknoten aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn require_node(&self, node_id: &str, restore: bool) -> Result<(), String> {
        let node = self
            .nodes
            .get(node_id)
            .ok_or_else(|| format!("unknown node {node_id}"))?;
        if !node.connected || node.stale {
            return Err(format!("node {node_id} is not online"));
        }
        if !node.call_control_capable {
            return Err(format!("node {node_id} does not advertise call_control"));
        }
        if restore && !node.call_restore_capable {
            return Err(format!("node {node_id} does not advertise call_restore_context"));
        }
        Ok(())
    }

    // Was: Diese Funktion wählt Gruppe targets.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn select_group_targets(
        &self,
        gssi: u32,
        requested: BTreeSet<String>,
    ) -> Result<BTreeSet<String>, String> {
        let mut targets = requested;
        if targets.is_empty() && self.config.calls.auto_target_affiliated_nodes {
            // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
            // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
            for participant in self.participants.values() {
                if participant.registered && participant.groups.contains(&gssi) {
                    targets.insert(participant.node_id.clone());
                }
            }
        }
        if targets.is_empty() {
            targets.extend(
                self.nodes
                    .values()
                    .filter(|node| node.connected && !node.stale && node.call_control_capable)
                    .map(|node| node.node_id.clone()),
            );
        }
        if targets.is_empty() {
            return Err("no call-control capable TBS is online".to_string());
        }
        if targets.len() > self.config.limits.max_legs_per_call {
            return Err("requested call exceeds max_legs_per_call".to_string());
        }
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for node_id in &targets {
            self.require_node(node_id, false)?;
        }
        Ok(targets)
    }

    // Was: Diese Funktion wählt individual target.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn select_individual_target(
        &self,
        called_issi: u32,
        requested: Option<String>,
    ) -> Result<String, String> {
        if let Some(node_id) = requested {
            self.require_node(&node_id, false)?;
            return Ok(node_id);
        }
        self.participants
            .values()
            .find(|participant| participant.issi == called_issi && participant.registered)
            .map(|participant| participant.node_id.clone())
            .ok_or_else(|| format!("called ISSI {called_issi} is not observed on an online TBS"))
    }

    // Was: Diese Funktion registriert command.
    // Warum: Die Zuordnung bleibt dadurch eindeutig und kann später sauber wieder entfernt werden.
    fn register_command(
        &mut self,
        node_id: String,
        handle: u32,
        command: ControlCommand,
        action: PendingAction,
    ) -> Result<BackendRequest, String> {
        if self.pending.len() >= self.config.limits.max_pending_commands {
            return Err("pending command limit reached".to_string());
        }
        let request_id = Uuid::new_v4().to_string();
        self.pending.insert(
            request_id.clone(),
            PendingCommand {
                request_id: request_id.clone(),
                command_id: None,
                node_id: node_id.clone(),
                handle,
                created: Instant::now(),
                action,
            },
        );
        Ok(BackendRequest::Command {
            request_id: Some(request_id),
            node_id,
            command,
            operator_id: Some("call-control/open-lab".to_string()),
        })
    }

    // Was: Diese Funktion wendet snapshot.
    // Warum: Die Änderung wird dadurch nur über einen definierten und prüfbaren Weg wirksam.
    fn apply_snapshot(&mut self, snapshot: GatewaySnapshot) {
        self.gateway_connected = true;
        self.gateway_last_error = None;
        let mut seen = BTreeSet::new();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for node in snapshot.nodes {
            seen.insert(node.node_id.clone());
            self.nodes.insert(
                node.node_id.clone(),
                NodeRecord {
                    node_id: node.node_id,
                    station_name: node.identity.station_name,
                    site: node.identity.site,
                    connected: node.connected,
                    stale: node.stale,
                    last_seen: node.last_seen,
                    call_control_capable: node.capabilities.call_control,
                    call_restore_capable: node.capabilities.call_restore_context,
                    mcc: node.identity.mcc,
                    mnc: node.identity.mnc,
                    location_area: node.identity.location_area,
                    colour_code: node.identity.colour_code,
                },
            );
        }
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for (node_id, node) in &mut self.nodes {
            if !seen.contains(node_id) {
                node.connected = false;
                node.stale = true;
            }
        }
    }

    // Was: Diese Funktion kennzeichnet Netzknoten disconnected.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn mark_node_disconnected(&mut self, node_id: &str, reason: &str) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.connected = false;
            node.stale = true;
        }
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for call in self.calls.values_mut() {
            if let Some(leg) = call.legs.get_mut(node_id) {
                if !leg.phase.is_terminal() {
                    leg.phase = LegPhase::Offline;
                    leg.updated_at = now();
                    leg.message = reason.to_string();
                    Self::recompute_call_phase(call);
                }
            }
        }
    }

    // Was: Diese Funktion verarbeitet action result.
    // Warum: Die Reaktion auf dieses Ereignis bleibt damit an einer Stelle nachvollziehbar.
    fn handle_action_result(
        &mut self,
        request_id: Option<String>,
        command_id: Option<String>,
        ok: bool,
        message: String,
    ) {
        let Some(request_id) = request_id else {
            return;
        };
        if ok {
            let action = {
                let Some(pending) = self.pending.get_mut(&request_id) else {
                    return;
                };
                pending.command_id = command_id.clone();
                (pending.action.clone(), pending.node_id.clone())
            };
            self.apply_command_id_to_action(&action.0, &action.1, command_id);
        } else if let Some(pending) = self.pending.remove(&request_id) {
            self.fail_pending(pending, message, false);
        }
    }

    // Was: Diese Funktion wendet command Kennung to action.
    // Warum: Die Änderung wird dadurch nur über einen definierten und prüfbaren Weg wirksam.
    fn apply_command_id_to_action(
        &mut self,
        action: &PendingAction,
        node_id: &str,
        command_id: Option<String>,
    ) {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match action {
            PendingAction::StartLeg { logical_call_id }
            | PendingAction::ReleaseLeg { logical_call_id }
            | PendingAction::FloorRequest { logical_call_id }
            | PendingAction::FloorRelease { logical_call_id } => {
                if let Some(leg) = self
                    .calls
                    .get_mut(logical_call_id)
                    .and_then(|call| call.legs.get_mut(node_id))
                {
                    leg.command_id = command_id;
                    if leg.phase == LegPhase::Requested {
                        leg.phase = LegPhase::Starting;
                    }
                    leg.updated_at = now();
                }
            }
            PendingAction::ExportRestore { restore_id } => {
                if let Some(operation) = self.restores.get_mut(restore_id) {
                    operation.phase = RestorePhase::ExportRequested;
                    operation.updated_at = now();
                }
            }
            PendingAction::ImportRestore { restore_id } => {
                if let Some(operation) = self.restores.get_mut(restore_id) {
                    operation.phase = RestorePhase::ImportRequested;
                    operation.updated_at = now();
                }
            }
            PendingAction::RemoveRestore { .. } => {}
        }
    }

    // Was: Diese Funktion verarbeitet Netzknoten Nachricht.
    // Warum: Die Reaktion auf dieses Ereignis bleibt damit an einer Stelle nachvollziehbar.
    fn handle_node_message(
        &mut self,
        node_id: String,
        message: NodeToControlRoomMessage,
    ) -> Vec<BackendRequest> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match message {
            NodeToControlRoomMessage::Telemetry { envelope } => {
                self.handle_telemetry(node_id, envelope.event)
            }
            NodeToControlRoomMessage::ControlAck { ack } => {
                if !ack.accepted {
                    if let Some(request_id) = self
                        .pending
                        .iter()
                        .find(|(_, pending)| pending.command_id.as_deref() == Some(ack.command_id.as_str()))
                        .map(|(id, _)| id.clone())
                    {
                        if let Some(pending) = self.pending.remove(&request_id) {
                            self.fail_pending(pending, ack.message, false);
                        }
                    }
                }
                Vec::new()
            }
            NodeToControlRoomMessage::ControlResponse { envelope } => {
                self.handle_control_response(node_id, envelope.response)
            }
            NodeToControlRoomMessage::Error { message, .. } => {
                self.push_event(
                    "node_error",
                    Some(node_id),
                    None,
                    None,
                    json!({"message": message}),
                );
                Vec::new()
            }
            NodeToControlRoomMessage::Hello { .. }
            | NodeToControlRoomMessage::Heartbeat { .. }
            | NodeToControlRoomMessage::MediaFrame { .. } => Vec::new(),
        }
    }

    // Was: Diese Funktion verarbeitet Steuerung response.
    // Warum: Die Reaktion auf dieses Ereignis bleibt damit an einer Stelle nachvollziehbar.
    fn handle_control_response(
        &mut self,
        node_id: String,
        response: ControlResponse,
    ) -> Vec<BackendRequest> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match response {
            ControlResponse::CallControlLegStarted {
                handle,
                operation_id: _,
                kind,
                success,
                call_id,
                timeslot,
                usage,
                floor_holder,
                message,
            } => {
                if let Some(pending) = self.take_pending(&node_id, handle) {
                    if let PendingAction::StartLeg { logical_call_id } = pending.action {
                        if let Some(call) = self.calls.get_mut(&logical_call_id) {
                            if let Some(leg) = call.legs.get_mut(&node_id) {
                                leg.local_call_id = call_id;
                                leg.timeslot = timeslot;
                                leg.usage = usage;
                                leg.floor_holder = floor_holder;
                                leg.phase = if success { LegPhase::Active } else { LegPhase::Failed };
                                leg.message = message.clone();
                                leg.updated_at = now();
                            }
                            call.kind = kind.into();
                            call.floor_holder = floor_holder.or(call.floor_holder);
                            call.updated_at = now();
                            Self::recompute_call_phase(call);
                        }
                        self.push_event(
                            if success { "call_leg_started" } else { "call_leg_failed" },
                            Some(node_id),
                            Some(logical_call_id),
                            call_id,
                            json!({"message": message, "timeslot": timeslot, "usage": usage}),
                        );
                    }
                }
                Vec::new()
            }
            ControlResponse::CallControlLegReleased {
                handle,
                call_id,
                success,
                message,
            } => {
                if let Some(pending) = self.take_pending(&node_id, handle) {
                    if let PendingAction::ReleaseLeg { logical_call_id } = pending.action {
                        if let Some(call) = self.calls.get_mut(&logical_call_id) {
                            if let Some(leg) = call.legs.get_mut(&node_id) {
                                leg.phase = if success { LegPhase::Ended } else { LegPhase::Failed };
                                leg.message = message.clone();
                                leg.updated_at = now();
                            }
                            Self::recompute_call_phase(call);
                        }
                        self.push_event(
                            "call_leg_release_response",
                            Some(node_id),
                            Some(logical_call_id),
                            Some(call_id),
                            json!({"success": success, "message": message}),
                        );
                    }
                }
                Vec::new()
            }
            ControlResponse::CallControlFloorChanged {
                handle,
                call_id,
                success,
                floor_holder,
                queued_issi,
                message,
            } => {
                if let Some(pending) = self.take_pending(&node_id, handle) {
                    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                    let logical_call_id = match pending.action {
                        PendingAction::FloorRequest { logical_call_id }
                        | PendingAction::FloorRelease { logical_call_id } => logical_call_id,
                        _ => return Vec::new(),
                    };
                    if let Some(call) = self.calls.get_mut(&logical_call_id) {
                        if let Some(leg) = call.legs.get_mut(&node_id) {
                            leg.floor_holder = floor_holder;
                            leg.queued_issi = queued_issi;
                            leg.message = message.clone();
                            leg.updated_at = now();
                        }
                        call.floor_holder = floor_holder;
                        call.floor_queue = queued_issi.into_iter().collect();
                        call.updated_at = now();
                    }
                    self.push_event(
                        "floor_response",
                        Some(node_id),
                        Some(logical_call_id),
                        Some(call_id),
                        json!({"success": success, "floor_holder": floor_holder, "queued_issi": queued_issi, "message": message}),
                    );
                }
                Vec::new()
            }
            ControlResponse::CallControlRestoreContextExported {
                handle,
                call_id,
                found,
                context,
                message,
            } => {
                let Some(pending) = self.take_pending(&node_id, handle) else {
                    return Vec::new();
                };
                let PendingAction::ExportRestore { restore_id } = pending.action else {
                    return Vec::new();
                };
                if !found || context.is_none() {
                    self.fail_restore(&restore_id, message);
                    return Vec::new();
                }
                let context = context.expect("checked context");
                let Some(operation_snapshot) = self.restores.get(&restore_id).cloned() else {
                    return Vec::new();
                };
                if let Some(operation) = self.restores.get_mut(&restore_id) {
                    operation.context = Some(context.clone());
                    operation.phase = RestorePhase::ImportQueued;
                    operation.updated_at = now();
                    operation.message = "source context exported; target import queued".to_string();
                }
                let handle = self.next_handle();
                let target_node = operation_snapshot.target_node.clone();
                // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                let request = match self.register_command(
                    target_node,
                    handle,
                    ControlCommand::CallControlImportRestoreContext { handle, context },
                    PendingAction::ImportRestore {
                        restore_id: restore_id.clone(),
                    },
                ) {
                    Ok(request) => request,
                    Err(error) => {
                        self.fail_restore(&restore_id, error);
                        return Vec::new();
                    }
                };
                self.push_event(
                    "restore_context_exported",
                    Some(node_id),
                    Some(operation_snapshot.logical_call_id),
                    Some(call_id),
                    json!({"restore_id": restore_id}),
                );
                vec![request]
            }
            ControlResponse::CallControlRestoreContextImported {
                handle,
                call_id,
                success,
                message,
            } => {
                let Some(pending) = self.take_pending(&node_id, handle) else {
                    return Vec::new();
                };
                let PendingAction::ImportRestore { restore_id } = pending.action else {
                    return Vec::new();
                };
                if let Some(operation) = self.restores.get_mut(&restore_id) {
                    operation.target_call_id = Some(call_id);
                    operation.phase = if success { RestorePhase::Ready } else { RestorePhase::Failed };
                    operation.updated_at = now();
                    operation.message = message.clone();
                }
                let logical_call_id = self
                    .restores
                    .get(&restore_id)
                    .map(|operation| operation.logical_call_id.clone());
                self.push_event(
                    if success { "restore_ready" } else { "restore_import_failed" },
                    Some(node_id.clone()),
                    logical_call_id.clone(),
                    Some(call_id),
                    json!({"restore_id": restore_id, "message": message}),
                );
                let mut requests = Vec::new();
                if success {
                    if let Some(logical_call_id) = logical_call_id {
                        let observed_call_id = self
                            .calls
                            .get(&logical_call_id)
                            .and_then(|call| call.legs.get(&node_id))
                            .filter(|leg| leg.phase == LegPhase::Active)
                            .and_then(|leg| leg.local_call_id);
                        if let Some(observed_call_id) = observed_call_id {
                            if let Some(request) = self.complete_matching_restore(
                                &logical_call_id,
                                &node_id,
                                observed_call_id,
                            ) {
                                requests.push(request);
                            }
                        }
                    }
                }
                requests
            }
            ControlResponse::CallControlRestoreContextRemoved {
                handle,
                call_id,
                success,
                message,
            } => {
                if let Some(pending) = self.take_pending(&node_id, handle) {
                    if let PendingAction::RemoveRestore { restore_id } = pending.action {
                        let logical_call_id = self
                            .restores
                            .get(&restore_id)
                            .map(|operation| operation.logical_call_id.clone());
                        self.push_event(
                            "restore_context_cleanup",
                            Some(node_id),
                            logical_call_id,
                            Some(call_id),
                            json!({"restore_id": restore_id, "success": success, "message": message}),
                        );
                    }
                }
                Vec::new()
            }
            _ => Vec::new(),
        }
    }

    // Was: Diese Funktion verarbeitet Telemetrie.
    // Warum: Die Reaktion auf dieses Ereignis bleibt damit an einer Stelle nachvollziehbar.
    fn handle_telemetry(
        &mut self,
        node_id: String,
        event: TelemetryEvent,
    ) -> Vec<BackendRequest> {
        let mut requests = Vec::new();
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match event {
            TelemetryEvent::MsRegistration { issi } => {
                self.update_participant(&node_id, issi, true, None);
            }
            TelemetryEvent::MsDeregistration { issi }
            | TelemetryEvent::MsTimeoutDrop { issi } => {
                self.update_participant(&node_id, issi, false, None);
            }
            TelemetryEvent::MsGroupAttach { issi, gssis }
            | TelemetryEvent::MsGroupsSnapshot { issi, gssis } => {
                self.update_participant(&node_id, issi, true, Some(gssis));
            }
            TelemetryEvent::MsGroupDetach { issi, gssis } => {
                let key = (node_id.clone(), issi);
                let participant = self.participants.entry(key).or_insert(ParticipantRecord {
                    node_id: node_id.clone(),
                    issi,
                    registered: true,
                    groups: BTreeSet::new(),
                    last_seen: now(),
                });
                // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
                // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
                for gssi in gssis {
                    participant.groups.remove(&gssi);
                }
                participant.last_seen = now();
            }
            TelemetryEvent::GroupCallStarted {
                call_id,
                gssi,
                caller_issi,
                ts,
                carrier_num,
                priority,
                source,
            } => {
                requests.extend(self.observe_group_start(
                    node_id,
                    call_id,
                    gssi,
                    caller_issi,
                    ts,
                    carrier_num,
                    priority,
                    source,
                ));
            }
            TelemetryEvent::GroupCallSpeakerChanged {
                call_id,
                gssi: _,
                speaker_issi,
                source,
            } => {
                self.observe_speaker_change(node_id, call_id, speaker_issi, source);
            }
            TelemetryEvent::GroupCallEnded { call_id, gssi: _ } => {
                self.observe_call_end(node_id, call_id, CallKind::Group);
            }
            TelemetryEvent::IndividualCallStarted {
                call_id,
                calling_issi,
                called_issi,
                simplex,
                ts,
                carrier_num,
                priority,
                source,
            } => {
                requests.extend(self.observe_individual_start(
                    node_id,
                    call_id,
                    calling_issi,
                    called_issi,
                    simplex,
                    ts,
                    carrier_num,
                    priority,
                    source,
                ));
            }
            TelemetryEvent::IndividualCallEnded { call_id } => {
                self.observe_call_end(node_id, call_id, CallKind::Individual);
            }
            _ => {}
        }
        requests
    }

    // Was: Diese Funktion aktualisiert participant.
    // Warum: Bestehender Zustand wird dadurch kontrolliert und nach einheitlichen Regeln geändert.
    fn update_participant(
        &mut self,
        node_id: &str,
        issi: u32,
        registered: bool,
        groups: Option<Vec<u32>>,
    ) {
        let participant = self
            .participants
            .entry((node_id.to_string(), issi))
            .or_insert(ParticipantRecord {
                node_id: node_id.to_string(),
                issi,
                registered,
                groups: BTreeSet::new(),
                last_seen: now(),
            });
        participant.registered = registered;
        if let Some(groups) = groups {
            participant.groups = groups.into_iter().collect();
        }
        participant.last_seen = now();
    }

    #[allow(clippy::too_many_arguments)]
    // Was: Führt den Arbeitsschritt `observe_group_start` für observe Gruppe start aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn observe_group_start(
        &mut self,
        node_id: String,
        call_id: u16,
        gssi: u32,
        caller_issi: u32,
        ts: u8,
        carrier_num: u16,
        priority: u8,
        source: String,
    ) -> Vec<BackendRequest> {
        let logical_call_id = self
            .find_call_by_leg(&node_id, call_id, CallKind::Group)
            .or_else(|| self.find_starting_group_leg(&node_id, gssi))
            .unwrap_or_else(|| self.create_observed_group_call(gssi, caller_issi, priority, &source));
        if let Some(call) = self.calls.get_mut(&logical_call_id) {
            let timestamp = now();
            let leg = call.legs.entry(node_id.clone()).or_insert(CallLeg {
                node_id: node_id.clone(),
                local_call_id: Some(call_id),
                operation_id: call.operation_id.clone(),
                phase: LegPhase::Active,
                timeslot: Some(ts),
                carrier_num: Some(carrier_num),
                usage: None,
                floor_holder: Some(caller_issi),
                queued_issi: None,
                command_id: None,
                restored: false,
                created_at: timestamp.clone(),
                updated_at: timestamp.clone(),
                message: "observed from TBS telemetry".to_string(),
            });
            leg.local_call_id = Some(call_id);
            leg.phase = LegPhase::Active;
            leg.timeslot = Some(ts);
            leg.carrier_num = Some(carrier_num);
            leg.floor_holder = Some(caller_issi);
            leg.updated_at = timestamp.clone();
            call.source_issi = Some(caller_issi);
            call.floor_holder = Some(caller_issi);
            call.priority = priority;
            call.emergency = priority >= 15;
            call.updated_at = timestamp;
            Self::recompute_call_phase(call);
        }
        let cleanup = self.complete_matching_restore(&logical_call_id, &node_id, call_id);
        self.push_event(
            "group_call_started",
            Some(node_id),
            Some(logical_call_id),
            Some(call_id),
            json!({"gssi": gssi, "caller_issi": caller_issi, "priority": priority, "source": source}),
        );
        cleanup.into_iter().collect()
    }

    #[allow(clippy::too_many_arguments)]
    // Was: Führt den Arbeitsschritt `observe_individual_start` für observe individual start aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn observe_individual_start(
        &mut self,
        node_id: String,
        call_id: u16,
        calling_issi: u32,
        called_issi: u32,
        simplex: bool,
        ts: u8,
        carrier_num: u16,
        priority: u8,
        source: String,
    ) -> Vec<BackendRequest> {
        let logical_call_id = self
            .find_call_by_leg(&node_id, call_id, CallKind::Individual)
            .or_else(|| self.find_starting_individual_leg(&node_id, calling_issi, called_issi))
            .unwrap_or_else(|| {
                self.create_observed_individual_call(
                    calling_issi,
                    called_issi,
                    simplex,
                    priority,
                    &source,
                )
            });
        if let Some(call) = self.calls.get_mut(&logical_call_id) {
            let timestamp = now();
            let leg = call.legs.entry(node_id.clone()).or_insert(CallLeg {
                node_id: node_id.clone(),
                local_call_id: Some(call_id),
                operation_id: call.operation_id.clone(),
                phase: LegPhase::Active,
                timeslot: Some(ts),
                carrier_num: Some(carrier_num),
                usage: None,
                floor_holder: None,
                queued_issi: None,
                command_id: None,
                restored: false,
                created_at: timestamp.clone(),
                updated_at: timestamp.clone(),
                message: "observed from TBS telemetry".to_string(),
            });
            leg.local_call_id = Some(call_id);
            leg.phase = LegPhase::Active;
            leg.timeslot = Some(ts);
            leg.carrier_num = Some(carrier_num);
            leg.updated_at = timestamp.clone();
            call.priority = priority;
            call.emergency = priority >= 15;
            call.updated_at = timestamp;
            Self::recompute_call_phase(call);
        }
        let cleanup = self.complete_matching_restore(&logical_call_id, &node_id, call_id);
        self.push_event(
            "individual_call_started",
            Some(node_id),
            Some(logical_call_id),
            Some(call_id),
            json!({"calling_issi": calling_issi, "called_issi": called_issi, "simplex": simplex, "priority": priority, "source": source}),
        );
        cleanup.into_iter().collect()
    }

    // Was: Führt den Arbeitsschritt `observe_speaker_change` für observe speaker change aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn observe_speaker_change(
        &mut self,
        node_id: String,
        call_id: u16,
        speaker_issi: u32,
        source: String,
    ) {
        if let Some(logical_call_id) = self.find_call_by_leg(&node_id, call_id, CallKind::Group) {
            if let Some(call) = self.calls.get_mut(&logical_call_id) {
                call.floor_holder = Some(speaker_issi);
                call.source_issi = Some(speaker_issi);
                call.floor_queue.clear();
                call.updated_at = now();
                if let Some(leg) = call.legs.get_mut(&node_id) {
                    leg.floor_holder = Some(speaker_issi);
                    leg.queued_issi = None;
                    leg.updated_at = now();
                }
            }
            self.push_event(
                "floor_changed",
                Some(node_id),
                Some(logical_call_id),
                Some(call_id),
                json!({"speaker_issi": speaker_issi, "source": source}),
            );
        }
    }

    // Was: Führt den Arbeitsschritt `observe_call_end` für observe Ruf end aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn observe_call_end(&mut self, node_id: String, call_id: u16, kind: CallKind) {
        if let Some(logical_call_id) = self.find_call_by_leg(&node_id, call_id, kind) {
            if let Some(call) = self.calls.get_mut(&logical_call_id) {
                if let Some(leg) = call.legs.get_mut(&node_id) {
                    leg.phase = LegPhase::Ended;
                    leg.floor_holder = None;
                    leg.updated_at = now();
                    leg.message = "ended by TBS telemetry".to_string();
                }
                Self::recompute_call_phase(call);
            }
            self.push_event(
                "call_leg_ended",
                Some(node_id),
                Some(logical_call_id),
                Some(call_id),
                json!({"kind": kind}),
            );
        }
    }

    // Was: Führt den Arbeitsschritt `complete_matching_restore` für complete matching Wiederherstellung aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn complete_matching_restore(
        &mut self,
        logical_call_id: &str,
        node_id: &str,
        call_id: u16,
    ) -> Option<BackendRequest> {
        let operation_snapshot = self
            .restores
            .values()
            .find(|operation| {
                operation.logical_call_id == logical_call_id
                    && operation.target_node == node_id
                    && operation.phase == RestorePhase::Ready
            })
            .cloned()?;
        if let Some(operation) = self.restores.get_mut(&operation_snapshot.restore_id) {
            operation.phase = RestorePhase::Completed;
            operation.target_call_id = Some(call_id);
            operation.updated_at = now();
            operation.message = "restored call leg observed on target TBS".to_string();
        }
        if let Some(call) = self.calls.get_mut(logical_call_id) {
            if let Some(leg) = call.legs.get_mut(node_id) {
                leg.restored = true;
                leg.local_call_id = Some(call_id);
                leg.phase = LegPhase::Active;
                leg.updated_at = now();
                leg.message = "restored call leg active".to_string();
            }
            Self::recompute_call_phase(call);
        }
        self.push_event(
            "restore_completed",
            Some(node_id.to_string()),
            Some(logical_call_id.to_string()),
            Some(call_id),
            json!({"restore_id": operation_snapshot.restore_id}),
        );

        let handle = self.next_handle();
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self.register_command(
            node_id.to_string(),
            handle,
            ControlCommand::CallControlRemoveRestoreContext {
                handle,
                call_id: operation_snapshot.source_call_id,
            },
            PendingAction::RemoveRestore {
                restore_id: operation_snapshot.restore_id.clone(),
            },
        ) {
            Ok(request) => Some(request),
            Err(error) => {
                self.push_event(
                    "restore_cleanup_not_queued",
                    Some(node_id.to_string()),
                    Some(logical_call_id.to_string()),
                    Some(operation_snapshot.source_call_id),
                    json!({"restore_id": operation_snapshot.restore_id, "error": error}),
                );
                None
            }
        }
    }

    // Was: Diese Funktion sucht Ruf by Rufzweig.
    // Warum: Die Suchlogik bleibt damit wiederverwendbar und muss nicht an mehreren Stellen kopiert werden.
    fn find_call_by_leg(&self, node_id: &str, call_id: u16, kind: CallKind) -> Option<String> {
        self.calls.values().find_map(|call| {
            (call.kind == kind
                && call
                    .legs
                    .get(node_id)
                    .is_some_and(|leg| leg.local_call_id == Some(call_id)))
            .then(|| call.logical_call_id.clone())
        })
    }

    // Was: Diese Funktion sucht starting Gruppe Rufzweig.
    // Warum: Die Suchlogik bleibt damit wiederverwendbar und muss nicht an mehreren Stellen kopiert werden.
    fn find_starting_group_leg(&self, node_id: &str, gssi: u32) -> Option<String> {
        self.calls.values().find_map(|call| {
            (call.kind == CallKind::Group
                && call.gssi == Some(gssi)
                && call
                    .legs
                    .get(node_id)
                    .is_some_and(|leg| matches!(leg.phase, LegPhase::Requested | LegPhase::Starting)))
            .then(|| call.logical_call_id.clone())
        })
    }

    // Was: Diese Funktion sucht starting individual Rufzweig.
    // Warum: Die Suchlogik bleibt damit wiederverwendbar und muss nicht an mehreren Stellen kopiert werden.
    fn find_starting_individual_leg(
        &self,
        node_id: &str,
        calling_issi: u32,
        called_issi: u32,
    ) -> Option<String> {
        self.calls.values().find_map(|call| {
            (call.kind == CallKind::Individual
                && call.calling_issi == Some(calling_issi)
                && call.called_issi == Some(called_issi)
                && call
                    .legs
                    .get(node_id)
                    .is_some_and(|leg| matches!(leg.phase, LegPhase::Requested | LegPhase::Starting)))
            .then(|| call.logical_call_id.clone())
        })
    }

    // Was: Diese Funktion erstellt observed Gruppe Ruf.
    // Warum: Neue Objekte erhalten so immer einen vollständigen und gültigen Ausgangszustand.
    fn create_observed_group_call(
        &mut self,
        gssi: u32,
        caller_issi: u32,
        priority: u8,
        source: &str,
    ) -> String {
        let id = Uuid::new_v4().to_string();
        let timestamp = now();
        self.calls.insert(
            id.clone(),
            LogicalCall {
                logical_call_id: id.clone(),
                operation_id: Uuid::new_v4().to_string(),
                kind: CallKind::Group,
                phase: CallPhase::Active,
                managed: false,
                source: source.to_string(),
                source_issi: Some(caller_issi),
                gssi: Some(gssi),
                calling_issi: None,
                called_issi: None,
                simplex: Some(true),
                priority,
                emergency: priority >= 15,
                floor_holder: Some(caller_issi),
                floor_queue: Vec::new(),
                legs: BTreeMap::new(),
                created_at: timestamp.clone(),
                updated_at: timestamp,
                ended_at: None,
                message: "discovered from TBS telemetry".to_string(),
            },
        );
        id
    }

    // Was: Diese Funktion erstellt observed individual Ruf.
    // Warum: Neue Objekte erhalten so immer einen vollständigen und gültigen Ausgangszustand.
    fn create_observed_individual_call(
        &mut self,
        calling_issi: u32,
        called_issi: u32,
        simplex: bool,
        priority: u8,
        source: &str,
    ) -> String {
        let id = Uuid::new_v4().to_string();
        let timestamp = now();
        self.calls.insert(
            id.clone(),
            LogicalCall {
                logical_call_id: id.clone(),
                operation_id: Uuid::new_v4().to_string(),
                kind: CallKind::Individual,
                phase: CallPhase::Active,
                managed: false,
                source: source.to_string(),
                source_issi: Some(calling_issi),
                gssi: None,
                calling_issi: Some(calling_issi),
                called_issi: Some(called_issi),
                simplex: Some(simplex),
                priority,
                emergency: priority >= 15,
                floor_holder: None,
                floor_queue: Vec::new(),
                legs: BTreeMap::new(),
                created_at: timestamp.clone(),
                updated_at: timestamp,
                ended_at: None,
                message: "discovered from TBS telemetry".to_string(),
            },
        );
        id
    }

    // Was: Führt den Arbeitsschritt `take_pending` für take pending aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn take_pending(&mut self, node_id: &str, handle: u32) -> Option<PendingCommand> {
        let request_id = self
            .pending
            .iter()
            .find(|(_, pending)| pending.node_id == node_id && pending.handle == handle)
            .map(|(request_id, _)| request_id.clone())?;
        self.pending.remove(&request_id)
    }

    // Was: Führt den Arbeitsschritt `fail_pending` für fail pending aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fail_pending(&mut self, pending: PendingCommand, message: String, timeout: bool) {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match pending.action {
            PendingAction::StartLeg { logical_call_id }
            | PendingAction::ReleaseLeg { logical_call_id }
            | PendingAction::FloorRequest { logical_call_id }
            | PendingAction::FloorRelease { logical_call_id } => {
                if let Some(call) = self.calls.get_mut(&logical_call_id) {
                    if let Some(leg) = call.legs.get_mut(&pending.node_id) {
                        leg.phase = if timeout { LegPhase::TimedOut } else { LegPhase::Failed };
                        leg.updated_at = now();
                        leg.message = message.clone();
                    }
                    Self::recompute_call_phase(call);
                }
                self.push_event(
                    "call_command_failed",
                    Some(pending.node_id),
                    Some(logical_call_id),
                    None,
                    json!({"request_id": pending.request_id, "message": message, "timeout": timeout}),
                );
            }
            PendingAction::ExportRestore { restore_id }
            | PendingAction::ImportRestore { restore_id } => {
                self.fail_restore(&restore_id, message);
            }
            PendingAction::RemoveRestore { restore_id } => {
                let logical_call_id = self
                    .restores
                    .get(&restore_id)
                    .map(|operation| operation.logical_call_id.clone());
                if let Some(operation) = self.restores.get_mut(&restore_id) {
                    operation.updated_at = now();
                    operation.message = format!(
                        "call leg restored; temporary target context cleanup failed: {message}"
                    );
                }
                self.push_event(
                    "restore_context_cleanup_failed",
                    Some(pending.node_id),
                    logical_call_id,
                    None,
                    json!({
                        "restore_id": restore_id,
                        "request_id": pending.request_id,
                        "message": message,
                        "timeout": timeout,
                    }),
                );
            }
        }
    }

    // Was: Diese Funktion entfernt pending for Wiederherstellung.
    // Warum: Das Entfernen bleibt damit vollständig und hinterlässt keinen widersprüchlichen Zustand.
    fn remove_pending_for_restore(&mut self, restore_id: &str) {
        self.pending.retain(|_, pending| match &pending.action {
            PendingAction::ExportRestore { restore_id: value }
            | PendingAction::ImportRestore { restore_id: value }
            | PendingAction::RemoveRestore { restore_id: value } => value != restore_id,
            _ => true,
        });
    }

    // Was: Führt den Arbeitsschritt `fail_restore` für fail Wiederherstellung aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fail_restore(&mut self, restore_id: &str, message: String) {
        self.remove_pending_for_restore(restore_id);
        let snapshot = self.restores.get_mut(restore_id).map(|operation| {
            operation.phase = RestorePhase::Failed;
            operation.updated_at = now();
            operation.message = message.clone();
            (
                operation.logical_call_id.clone(),
                operation.source_call_id,
                operation.target_node.clone(),
            )
        });
        if let Some((logical_call_id, source_call_id, target_node)) = snapshot {
            if let Some(call) = self.calls.get_mut(&logical_call_id) {
                if let Some(leg) = call.legs.get(&target_node) {
                    if leg.restored && leg.local_call_id.is_none() {
                        call.legs.remove(&target_node);
                    }
                }
                Self::recompute_call_phase(call);
            }
            self.push_event(
                "restore_failed",
                None,
                Some(logical_call_id),
                Some(source_call_id),
                json!({"restore_id": restore_id, "message": message}),
            );
        }
    }

    // Was: Führt den Arbeitsschritt `recompute_call_phase` für recompute Ruf phase aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn recompute_call_phase(call: &mut LogicalCall) {
        let active = call.legs.values().filter(|leg| leg.phase == LegPhase::Active).count();
        let pending = call
            .legs
            .values()
            .filter(|leg| matches!(leg.phase, LegPhase::Requested | LegPhase::Starting | LegPhase::Releasing))
            .count();
        let failed = call
            .legs
            .values()
            .filter(|leg| matches!(leg.phase, LegPhase::Failed | LegPhase::TimedOut | LegPhase::Offline))
            .count();
        let ended = call.legs.values().filter(|leg| leg.phase == LegPhase::Ended).count();
        let total = call.legs.len();
        call.phase = if total > 0 && ended == total {
            CallPhase::Ended
        } else if active > 0 && failed > 0 {
            CallPhase::Partial
        } else if active > 0 {
            CallPhase::Active
        } else if pending > 0 && call.phase == CallPhase::Releasing {
            CallPhase::Releasing
        } else if pending > 0 {
            CallPhase::Starting
        } else if failed > 0 {
            CallPhase::Failed
        } else {
            call.phase
        };
        call.updated_at = now();
        if call.phase.is_terminal() && call.ended_at.is_none() {
            call.ended_at = Some(now());
            call.floor_holder = None;
            call.floor_queue.clear();
        }
    }

    // Was: Diese Funktion legt Ereignis.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn push_event(
        &mut self,
        kind: &str,
        node_id: Option<String>,
        logical_call_id: Option<String>,
        local_call_id: Option<u16>,
        detail: Value,
    ) {
        let seq = self.next_event_seq;
        let timestamp = now();
        let canonical = canonical_call_event(
            seq,
            kind,
            node_id.as_deref(),
            logical_call_id.as_deref(),
            local_call_id,
            &detail,
        );
        let event = EventRecord {
            seq,
            timestamp,
            kind: kind.to_string(),
            node_id,
            logical_call_id: logical_call_id.clone(),
            local_call_id,
            detail,
            canonical,
        };
        self.next_event_seq = self.next_event_seq.saturating_add(1);
        self.events.push_back(event);
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        while self.events.len() > self.config.server.history_limit {
            self.events.pop_front();
        }

        if let Some(media_kind) = media_event_kind(kind) {
            self.publish_media(media_kind, kind, logical_call_id);
        }
    }

    // Was: Führt den Arbeitsschritt `media_event` für Audio- und Mediendaten Ereignis aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn media_event(
        &self,
        kind: &str,
        reason: &str,
        logical_call_id: Option<String>,
    ) -> MediaControlEvent {
        MediaControlEvent {
            kind: kind.to_string(),
            revision: self.media_revision,
            emitted_at: now(),
            reason: reason.to_string(),
            logical_call_id,
            calls: self
                .calls
                .values()
                .filter(|call| !call.phase.is_terminal())
                .cloned()
                .collect(),
        }
    }

    // Was: Diese Funktion veröffentlicht Audio- und Mediendaten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn publish_media(
        &mut self,
        kind: &str,
        reason: &str,
        logical_call_id: Option<String>,
    ) {
        self.media_revision = self.media_revision.saturating_add(1);
        if let Some(logical_call_id) = logical_call_id.as_ref() {
            let terminal = self
                .calls
                .get(logical_call_id)
                .is_none_or(|call| call.phase.is_terminal());
            if terminal {
                self.media_required_revision.remove(logical_call_id);
                self.media_ready_revision.remove(logical_call_id);
            } else if matches!(
                kind,
                "call_created" | "leg_ready" | "call_updated" | "call_released"
            ) {
                self.media_required_revision
                    .insert(logical_call_id.clone(), self.media_revision);
                self.media_ready_revision.remove(logical_call_id);
            }
        }
        let event = self.media_event(kind, reason, logical_call_id);
        self.media_subscribers
            .retain(|subscriber| subscriber.send(event.clone()).is_ok());
    }

    // Was: Führt den Arbeitsschritt `bump_and_persist` für bump and persist aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn bump_and_persist(&mut self) {
        self.database_revision = self.database_revision.saturating_add(1);
        if let Err(error) = self.persist() {
            tracing::error!("failed to persist Call Control database: {}", error);
        }
    }

    // Was: Führt den Arbeitsschritt `bump_and_persist_if_changed` für bump and persist if changed aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn bump_and_persist_if_changed(&mut self) {
        // Backend messages are authoritative state changes. Persisting the compact
        // logical-call database is cheap and keeps crash recovery deterministic.
        self.bump_and_persist();
    }

    // Was: Diese Funktion speichert den vorgesehenen Arbeitsschritt.
    // Warum: Wichtiger Zustand bleibt dadurch über Neustarts hinweg erhalten.
    fn persist(&self) -> Result<(), String> {
        let database = PersistedDatabase {
            schema_version: DATABASE_SCHEMA_VERSION,
            revision: self.database_revision,
            calls: self.calls.values().cloned().collect(),
            restores: self.restores.values().cloned().collect(),
        };
        let data = serde_json::to_vec_pretty(&database)
            .map_err(|error| format!("database serialization failed: {error}"))?;
        if let Some(parent) = self.config.storage.database_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("create database directory failed: {error}"))?;
        }
        if self.config.storage.database_path.exists() {
            let _ = fs::copy(
                &self.config.storage.database_path,
                &self.config.storage.backup_path,
            );
        }
        let temporary = self.config.storage.database_path.with_extension("json.tmp");
        let mut file = fs::File::create(&temporary)
            .map_err(|error| format!("create temporary database failed: {error}"))?;
        file.write_all(&data)
            .map_err(|error| format!("write temporary database failed: {error}"))?;
        file.sync_all()
            .map_err(|error| format!("sync temporary database failed: {error}"))?;
        fs::rename(&temporary, &self.config.storage.database_path)
            .map_err(|error| format!("replace database failed: {error}"))?;
        Ok(())
    }
}

// Was: Führt den Arbeitsschritt `media_event_kind` für Audio- und Mediendaten Ereignis kind aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn canonical_call_event(
    seq: u64,
    kind: &str,
    node_id: Option<&str>,
    logical_call_id: Option<&str>,
    local_call_id: Option<u16>,
    detail: &Value,
) -> NetCoreEvent {
    let (event_type, severity) = match kind {
        "gateway_connected" => (event_types::SERVICE_DEPENDENCY_CONNECTED, Severity::Info),
        "gateway_disconnected" => (event_types::SERVICE_DEPENDENCY_DISCONNECTED, Severity::Warning),
        "media_route_ready" => (event_types::CALL_MEDIA_ROUTE_READY, Severity::Info),
        "group_call_requested" | "individual_call_requested" => {
            (event_types::CALL_REQUESTED, Severity::Info)
        }
        "group_call_started" | "individual_call_started" => {
            (event_types::CALL_STARTED, Severity::Info)
        }
        "call_release_requested" => (event_types::CALL_RELEASE_REQUESTED, Severity::Notice),
        "call_leg_ended" | "call_leg_release_response" => {
            (event_types::CALL_RELEASED, Severity::Info)
        }
        "call_command_failed" => (event_types::CALL_FAILED, Severity::Error),
        "floor_requested" => (event_types::FLOOR_REQUESTED, Severity::Info),
        "floor_release_requested" => (event_types::FLOOR_RELEASE_REQUESTED, Severity::Info),
        "floor_changed" | "floor_response" => (event_types::FLOOR_CHANGED, Severity::Info),
        "restore_requested" => (event_types::CALL_RESTORE_REQUESTED, Severity::Info),
        "restore_completed" => (event_types::CALL_RESTORED, Severity::Info),
        "restore_failed" | "restore_context_cleanup_failed" => {
            (event_types::CALL_RESTORE_FAILED, Severity::Error)
        }
        "restore_timed_out" => (event_types::CALL_RESTORE_FAILED, Severity::Warning),
        "restore_cancelled" => (event_types::CALL_RESTORE_FAILED, Severity::Notice),
        "node_error" => (event_types::NODE_DEGRADED, Severity::Error),
        _ => (event_types::SERVICE_STATE_CHANGED, Severity::Debug),
    };

    let subject = if kind.starts_with("floor_") {
        logical_call_id.map(|value| EventSubject::new(subject_types::FLOOR, value))
    } else if kind.starts_with("restore_") {
        logical_call_id.map(|value| EventSubject::new(subject_types::RESTORE, value))
    } else if let Some(value) = logical_call_id {
        Some(EventSubject::new(subject_types::CALL, value))
    } else if let Some(value) = node_id {
        Some(EventSubject::new(subject_types::NODE, value))
    } else {
        Some(EventSubject::new(subject_types::SERVICE, "netcore-call-control"))
    };

    let mut payload = detail.clone();
    if !payload.is_object() {
        payload = json!({ "detail": payload });
    }
    if let Some(object) = payload.as_object_mut() {
        object.entry("legacy_kind".to_string()).or_insert_with(|| json!(kind));
        if let Some(value) = node_id {
            object.entry("node_id".to_string()).or_insert_with(|| json!(value));
        }
        if let Some(value) = logical_call_id {
            object
                .entry("logical_call_id".to_string())
                .or_insert_with(|| json!(value));
        }
        if let Some(value) = local_call_id {
            object
                .entry("local_call_id".to_string())
                .or_insert_with(|| json!(value));
        }
    }

    let mut source = event_source("netcore-call-control");
    if let Some(value) = node_id {
        source = source.with_node_id(value);
    }
    let mut event = NetCoreEvent::new(event_type, source, severity, subject, payload)
        .with_sequence(seq)
        .with_label("legacy_kind", kind);
    if let Some(value) = logical_call_id.and_then(|value| Uuid::parse_str(value).ok()) {
        event = event.with_correlation_id(value);
    }
    event
}

fn media_event_kind(reason: &str) -> Option<&'static str> {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match reason {
        "group_call_requested" | "individual_call_requested" => Some("call_created"),
        "group_call_started" | "individual_call_started" | "call_leg_started" => {
            Some("leg_ready")
        }
        "call_release_requested" | "call_leg_release_response" | "call_leg_ended" => {
            Some("call_released")
        }
        "floor_requested" | "floor_release_requested" | "floor_response"
        | "floor_changed" => Some("floor_changed"),
        "restore_requested" | "restore_cancelled" | "restore_timed_out"
        | "restore_context_exported" | "restore_context_cleanup"
        | "restore_completed" | "restore_cleanup_not_queued"
        | "restore_context_cleanup_failed" | "restore_failed" => Some("call_updated"),
        "call_command_failed" | "call_leg_failed" | "node_error" => Some("call_updated"),
        _ => None,
    }
}

// Was: Diese Funktion lädt Datenbank.
// Warum: Einlesen und Fehlerbehandlung bleiben dadurch an einer zentralen Stelle.
fn load_database(config: &CallControlConfig) -> Result<PersistedDatabase, Box<dyn std::error::Error>> {
    if !config.storage.database_path.exists() {
        return Ok(PersistedDatabase::default());
    }
    let database: PersistedDatabase =
        serde_json::from_slice(&fs::read(&config.storage.database_path)?)?;
    if database.schema_version != DATABASE_SCHEMA_VERSION {
        return Err(format!(
            "unsupported Call Control database schema {}",
            database.schema_version
        )
        .into());
    }
    Ok(database)
}

// Was: Führt den Arbeitsschritt `now` für now aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn now() -> String {
    Utc::now().to_rfc3339()
}
