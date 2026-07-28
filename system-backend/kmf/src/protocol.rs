// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Schlüsselverwaltung und Schlüsselverteilung.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use serde::{Deserialize, Serialize};

// Was: Legt den festen Wert `OTAR_EDGE_PROTOCOL_VERSION` für otar edge protocol version fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const OTAR_EDGE_PROTOCOL_VERSION: &str = "netcore-kmf-otar-edge-v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für policy input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct PolicyInput {
    pub operating_mode: String,
    pub default_key_bytes: usize,
    pub default_crypto_period_secs: u64,
    pub rotation_lead_secs: u64,
    pub require_dual_approval: bool,
    pub allow_overlapping_crypto_periods: bool,
    pub auto_retire_predecessor: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für key create input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct KeyCreateInput {
    pub kind: String,
    pub scope: String,
    pub scope_value: Option<String>,
    pub label: String,
    pub algorithm_profile: Option<String>,
    pub key_bytes: Option<usize>,
    pub crypto_period_start: Option<String>,
    pub crypto_period_end: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
// Was: Bündelt die zusammengehörigen Werte für key rotate input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct KeyRotateInput {
    pub actor: Option<String>,
    pub activate_at: Option<String>,
    pub crypto_period_end: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
// Was: Bündelt die zusammengehörigen Werte für lifecycle input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LifecycleInput {
    pub actor: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Netzknoten create input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct NodeCreateInput {
    pub node_id: String,
    pub display_name: String,
    pub actor: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
// Was: Bündelt die zusammengehörigen Werte für Netzknoten Zustand input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct NodeStateInput {
    pub actor: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für otar job create input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct OtarJobCreateInput {
    pub key_id: String,
    pub target_nodes: Vec<String>,
    pub target_issis: Vec<u32>,
    pub target_gssis: Vec<u32>,
    pub not_before: Option<String>,
    pub expires_at: Option<String>,
    pub actor: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für otar approval input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct OtarApprovalInput {
    pub actor: String,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
// Was: Bündelt die zusammengehörigen Werte für otar Warteschlange input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct OtarQueueInput {
    pub actor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für edge claim input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct EdgeClaimInput {
    pub node_id: String,
    pub max_actions: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für edge action ack input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct EdgeActionAckInput {
    pub success: bool,
    pub error: Option<String>,
    pub applied_at: Option<String>,
}

// Was: Implementiert das zugehörige Verhalten für `Default for EdgeActionAckInput`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for EdgeActionAckInput {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            success: true,
            error: None,
            applied_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
// Was: Bündelt die zusammengehörigen Werte für backup input in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct BackupInput {
    pub actor: Option<String>,
    pub note: Option<String>,
}
