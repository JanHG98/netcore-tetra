// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Metriken, Protokolle und Betriebsüberwachung.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für target create input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TargetCreateInput {
    pub target_id: String,
    pub display_name: String,
    pub service: String,
    pub base_url: String,
    pub metrics_path: Option<String>,
    pub live_path: Option<String>,
    pub ready_path: Option<String>,
    pub events_path: Option<String>,
    pub enabled: Option<bool>,
    #[serde(default)]
    pub labels: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
// Was: Bündelt die zusammengehörigen Werte für action input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ActionInput {
    pub actor: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für rule input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RuleInput {
    pub rule_id: String,
    pub name: String,
    pub description: Option<String>,
    pub metric: String,
    pub comparator: String,
    pub threshold: f64,
    pub for_secs: Option<u64>,
    pub severity: String,
    pub service: Option<String>,
    pub target_id: Option<String>,
    pub enabled: Option<bool>,
    #[serde(default)]
    pub labels: BTreeMap<String, String>,
    #[serde(default)]
    pub annotations: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für silence input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct SilenceInput {
    pub comment: String,
    pub created_by: Option<String>,
    pub duration_secs: u64,
    pub rule_id: Option<String>,
    pub service: Option<String>,
    pub target_id: Option<String>,
    pub severity: Option<String>,
    #[serde(default)]
    pub match_labels: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für log ingest input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LogIngestInput {
    #[serde(default)]
    pub records: Vec<LogInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für log input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LogInput {
    pub timestamp: Option<String>,
    pub service: String,
    pub node: Option<String>,
    pub level: Option<String>,
    pub message: String,
    pub correlation_id: Option<String>,
    pub trace_id: Option<String>,
    #[serde(default)]
    pub fields: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für trace ingest input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TraceIngestInput {
    #[serde(default)]
    pub spans: Vec<TraceSpanInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für trace span input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TraceSpanInput {
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
    pub service: String,
    pub operation: String,
    pub started_at: Option<String>,
    pub duration_ms: f64,
    pub status: Option<String>,
    #[serde(default)]
    pub attributes: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
// Was: Bündelt die zusammengehörigen Werte für maintenance input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MaintenanceInput {
    pub actor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
// Was: Bündelt die zusammengehörigen Werte für diagnostic input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct DiagnosticInput {
    pub actor: Option<String>,
    pub reason: Option<String>,
    pub include_logs: Option<bool>,
    pub include_traces: Option<bool>,
    pub max_records: Option<usize>,
}
