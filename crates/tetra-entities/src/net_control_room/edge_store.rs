// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Durable edge-autonomy state.
//!
//! The TBS must remain useful when its WAN/VPN/Internet path is unavailable.
//! This module persists the last-known admission/group policy and a bounded
//! control-plane event spool.  Media and high-rate RF telemetry are never
//! written to the spool.

use std::collections::VecDeque;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tetra_config::bluestation::{CentralGroupPolicy, SharedConfig, StackConfig, StackState};

use crate::net_telemetry::TelemetryEvent;

// Was: Legt den festen Wert `POLICY_SCHEMA_VERSION` für policy schema version fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const POLICY_SCHEMA_VERSION: u32 = 1;
// Was: Legt den festen Wert `SPOOL_SCHEMA_VERSION` für spool schema version fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const SPOOL_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für edge policy Zwischenspeicher file in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct EdgePolicyCacheFile {
    schema_version: u32,
    saved_at: String,
    subscriber_policy_revision: u64,
    issi_whitelist_override: Option<Vec<u32>>,
    issi_whitelist_deny_all: bool,
    group_policy_override: Option<CentralGroupPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für edge spool Datensatz in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct EdgeSpoolRecord {
    pub schema_version: u32,
    pub sequence: u64,
    pub timestamp: String,
    pub event: TelemetryEvent,
}

#[derive(Debug)]
// Was: Bündelt die zusammengehörigen Werte für edge Ereignis spool in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct EdgeEventSpool {
    path: PathBuf,
    max_entries: usize,
    max_bytes: usize,
    next_sequence: u64,
}

// Was: Implementiert das zugehörige Verhalten für `EdgeEventSpool`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl EdgeEventSpool {
    // Was: Wandelt Eingangsdaten in Konfiguration um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_config(config: &StackConfig) -> Self {
        let path = PathBuf::from(&config.edge_fallback.event_spool_path);
        let next_sequence = read_records(&path)
            .ok()
            .and_then(|records| records.back().map(|record| record.sequence.saturating_add(1)))
            .unwrap_or(1);
        Self {
            path,
            max_entries: config.edge_fallback.event_spool_max_entries,
            max_bytes: config.edge_fallback.event_spool_max_bytes,
            next_sequence,
        }
    }

    // Was: Führt den Arbeitsschritt `append` für append aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn append(&mut self, event: TelemetryEvent) -> Result<(), String> {
        if !is_replayable_event(&event) {
            return Ok(());
        }
        ensure_parent(&self.path)?;
        let record = EdgeSpoolRecord {
            schema_version: SPOOL_SCHEMA_VERSION,
            sequence: self.next_sequence,
            timestamp: now_iso(),
            event,
        };
        self.next_sequence = self.next_sequence.saturating_add(1);
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|error| format!("open edge spool {}: {error}", self.path.display()))?;
        serde_json::to_writer(&mut file, &record).map_err(|error| error.to_string())?;
        file.write_all(b"\n").map_err(|error| error.to_string())?;
        file.sync_data().map_err(|error| error.to_string())?;
        self.enforce_limits()
    }

    // Was: Führt den Arbeitsschritt `peek_batch` für peek batch aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn peek_batch(&self, limit: usize) -> Result<Vec<EdgeSpoolRecord>, String> {
        Ok(read_records(&self.path)?
            .into_iter()
            .take(limit.max(1))
            .collect())
    }

    // Was: Führt den Arbeitsschritt `acknowledge_through` für acknowledge through aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn acknowledge_through(&mut self, sequence: u64) -> Result<(), String> {
        let mut records = read_records(&self.path)?;
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        while records.front().is_some_and(|record| record.sequence <= sequence) {
            records.pop_front();
        }
        rewrite_records(&self.path, &records)
    }

    // Was: Führt den Arbeitsschritt `stats` für stats aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn stats(&self) -> (usize, u64) {
        let entries = read_records(&self.path).map(|records| records.len()).unwrap_or(0);
        let bytes = fs::metadata(&self.path).map(|metadata| metadata.len()).unwrap_or(0);
        (entries, bytes)
    }

    // Was: Führt den Arbeitsschritt `enforce_limits` für enforce limits aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn enforce_limits(&mut self) -> Result<(), String> {
        let mut records = read_records(&self.path)?;
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        while records.len() > self.max_entries {
            records.pop_front();
        }
        rewrite_records(&self.path, &records)?;
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        while fs::metadata(&self.path).map(|metadata| metadata.len() as usize).unwrap_or(0) > self.max_bytes
            && !records.is_empty()
        {
            records.pop_front();
            rewrite_records(&self.path, &records)?;
        }
        Ok(())
    }
}

/// Load the last-known policy before the protocol entities are constructed.
/// A stale policy is retained by default because silently reverting to an open
/// network is less safe than continuing the last explicit operator decision.
// Was: Diese Funktion lädt edge policy Zwischenspeicher.
// Warum: Einlesen und Fehlerbehandlung bleiben dadurch an einer zentralen Stelle.
pub fn load_edge_policy_cache(config: &StackConfig, state: &mut StackState) -> Result<bool, String> {
    if !config.edge_fallback.enabled {
        return Ok(false);
    }
    let path = Path::new(&config.edge_fallback.policy_cache_path);
    if !path.exists() {
        return Ok(false);
    }
    let bytes = fs::read(path).map_err(|error| format!("read policy cache {}: {error}", path.display()))?;
    let cache: EdgePolicyCacheFile = serde_json::from_slice(&bytes).map_err(|error| format!("parse policy cache: {error}"))?;
    if cache.schema_version != POLICY_SCHEMA_VERSION {
        return Err(format!(
            "unsupported edge policy cache schema {}; expected {}",
            cache.schema_version, POLICY_SCHEMA_VERSION
        ));
    }
    let age_secs = DateTime::parse_from_rfc3339(&cache.saved_at)
        .ok()
        .map(|saved| Utc::now().signed_duration_since(saved.with_timezone(&Utc)).num_seconds().max(0) as u64);
    let stale = age_secs.is_none_or(|age| age > config.edge_fallback.policy_cache_max_age_secs);
    if stale && !config.edge_fallback.keep_last_known_policy {
        return Ok(false);
    }

    state.subscriber_policy_revision = cache.subscriber_policy_revision;
    state.issi_whitelist_override = cache.issi_whitelist_override;
    state.issi_whitelist_deny_all = cache.issi_whitelist_deny_all;
    state.group_policy_override = cache.group_policy_override;
    state.edge_policy_loaded_from_cache = true;
    state.edge_policy_cache_saved_at = Some(cache.saved_at);
    state.edge_policy_cache_age_secs = age_secs;
    Ok(true)
}

// Was: Diese Funktion speichert edge policy Zwischenspeicher.
// Warum: Wichtiger Zustand bleibt dadurch über Neustarts hinweg erhalten.
pub fn persist_edge_policy_cache(config: &SharedConfig) -> Result<(), String> {
    if !config.config().edge_fallback.enabled {
        return Ok(());
    }
    let cfg = config.config();
    let state = config.state_read();
    let cache = EdgePolicyCacheFile {
        schema_version: POLICY_SCHEMA_VERSION,
        saved_at: now_iso(),
        subscriber_policy_revision: state.subscriber_policy_revision,
        issi_whitelist_override: state.issi_whitelist_override.clone(),
        issi_whitelist_deny_all: state.issi_whitelist_deny_all,
        group_policy_override: state.group_policy_override.clone(),
    };
    drop(state);
    let path = Path::new(&cfg.edge_fallback.policy_cache_path);
    ensure_parent(path)?;
    let temp = path.with_extension("json.tmp");
    let bytes = serde_json::to_vec_pretty(&cache).map_err(|error| error.to_string())?;
    let mut file = fs::File::create(&temp).map_err(|error| error.to_string())?;
    file.write_all(&bytes).map_err(|error| error.to_string())?;
    file.sync_all().map_err(|error| error.to_string())?;
    fs::rename(&temp, path).map_err(|error| error.to_string())?;
    sync_parent(path)?;
    Ok(())
}

// Was: Diese Funktion aktualisiert spool stats.
// Warum: Bestehender Zustand wird dadurch kontrolliert und nach einheitlichen Regeln geändert.
pub fn update_spool_stats(config: &SharedConfig, spool: &EdgeEventSpool) {
    let (entries, bytes) = spool.stats();
    let mut state = config.state_write();
    state.edge_event_spool_entries = entries;
    state.edge_event_spool_bytes = bytes;
}

// Was: Prüft, ob replayable Ereignis zutrifft.
// Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
pub fn is_replayable_event(event: &TelemetryEvent) -> bool {
    matches!(
        event,
        TelemetryEvent::MsRegistration { .. }
            | TelemetryEvent::MsDeregistration { .. }
            | TelemetryEvent::MsTimeoutDrop { .. }
            | TelemetryEvent::MsGroupAttach { .. }
            | TelemetryEvent::MsGroupsSnapshot { .. }
            | TelemetryEvent::MsGroupDetach { .. }
            | TelemetryEvent::GroupCallStarted { .. }
            | TelemetryEvent::GroupCallEnded { .. }
            | TelemetryEvent::GroupCallSpeakerChanged { .. }
            | TelemetryEvent::IndividualCallStarted { .. }
            | TelemetryEvent::IndividualCallEnded { .. }
            | TelemetryEvent::MsEnergySaving { .. }
            | TelemetryEvent::SdsActivity { .. }
            | TelemetryEvent::SdsLog { .. }
            | TelemetryEvent::EmergencyAlarm { .. }
            | TelemetryEvent::EmergencyCancel { .. }
            | TelemetryEvent::SdsEdgeIngress { .. }
    )
}

// Was: Diese Funktion liest records.
// Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
fn read_records(path: &Path) -> Result<VecDeque<EdgeSpoolRecord>, String> {
    if !path.exists() {
        return Ok(VecDeque::new());
    }
    let contents = fs::read(path).map_err(|error| error.to_string())?;
    let has_complete_final_line = contents.ends_with(b"\n");
    let lines: Vec<_> = contents.split(|byte| *byte == b'\n').collect();
    let mut records = VecDeque::new();
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for (index, line) in lines.iter().enumerate() {
        if line.iter().all(|byte| byte.is_ascii_whitespace()) {
            continue;
        }
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        let record: EdgeSpoolRecord = match serde_json::from_slice(line) {
            Ok(record) => record,
            Err(error) if index + 1 == lines.len() && !has_complete_final_line => {
                // A power loss may leave the last append torn, including in the
                // middle of an UTF-8 sequence. Earlier records remain valid and
                // replayable, so discard only that incomplete tail.
                tracing::warn!(
                    path = %path.display(),
                    line = index + 1,
                    %error,
                    "discarding incomplete final edge spool record"
                );
                break;
            }
            Err(error) => {
                return Err(format!("invalid edge spool line {}: {error}", index + 1));
            }
        };
        if record.schema_version == SPOOL_SCHEMA_VERSION {
            records.push_back(record);
        }
    }
    Ok(records)
}

// Was: Führt den Arbeitsschritt `rewrite_records` für rewrite records aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn rewrite_records(path: &Path, records: &VecDeque<EdgeSpoolRecord>) -> Result<(), String> {
    ensure_parent(path)?;
    let temp = path.with_extension("jsonl.tmp");
    {
        let mut file = fs::File::create(&temp).map_err(|error| error.to_string())?;
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for record in records {
            serde_json::to_writer(&mut file, record).map_err(|error| error.to_string())?;
            file.write_all(b"\n").map_err(|error| error.to_string())?;
        }
        file.sync_all().map_err(|error| error.to_string())?;
    }
    fs::rename(&temp, path).map_err(|error| error.to_string())?;
    sync_parent(path)?;
    Ok(())
}

// Was: Diese Funktion gleicht parent.
// Warum: Mehrere Zustandsquellen bleiben dadurch auf demselben Stand.
fn sync_parent(path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::File::open(parent)
            .and_then(|directory| directory.sync_all())
            .map_err(|error| format!("sync directory {}: {error}", parent.display()))?;
    }
    Ok(())
}

// Was: Diese Funktion stellt parent.
// Warum: So wird die notwendige Voraussetzung hergestellt, bevor abhängiger Code weiterläuft.
fn ensure_parent(path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("create {}: {error}", parent.display()))?;
    }
    Ok(())
}

// Was: Führt den Arbeitsschritt `now_iso` für now iso aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn now_iso() -> String {
    Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Führt den Arbeitsschritt `high_rate_rf_events_are_not_spooled` für high rate Funkstrecke events are not spooled aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn high_rate_rf_events_are_not_spooled() {
        assert!(!is_replayable_event(&TelemetryEvent::MsRssi { issi: 1, rssi_dbfs: -30.0 }));
        assert!(is_replayable_event(&TelemetryEvent::MsRegistration { issi: 1 }));
    }
}
