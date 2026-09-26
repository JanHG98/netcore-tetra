// NETCORE-KOMMENTAR – Was: Enthält die Logik oder Einstellungen für health.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für health Status auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum HealthStatus {
    Up,
    Degraded,
    Down,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für dependency health in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct DependencyHealth {
    pub name: String,
    pub status: HealthStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für build info in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct BuildInfo {
    pub service: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_commit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_timestamp: Option<String>,
    pub contract_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für health document in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct HealthDocument {
    pub status: HealthStatus,
    pub service: String,
    pub instance: String,
    pub observed_at: DateTime<Utc>,
    #[serde(default)]
    pub dependencies: Vec<DependencyHealth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<BuildInfo>,
}
