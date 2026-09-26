// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Einlesen und Prüfen der TETRA-Konfiguration.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::HashMap;

use serde::Deserialize;
use toml::Value;

/// Control endpoint configuration
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für cfg Steuerung in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgControl {
    /// Control server hostname or IP
    pub host: String,
    /// Control server port
    pub port: u16,
    /// Use TLS (wss://)
    pub use_tls: bool,
    /// Optional path to a DER-encoded CA certificate for self-signed TLS
    pub ca_cert: Option<String>,
    /// Optional (username, password) for HTTP Basic authentication
    pub credentials: Option<(String, String)>,
}

#[derive(Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für cfg Steuerung dto in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgControlDto {
    /// Control server hostname or IP
    pub host: String,
    /// Control server port
    pub port: u16,
    /// Use TLS (wss://)
    #[serde(default)]
    pub use_tls: bool,
    /// Optional path to a DER-encoded CA certificate for self-signed TLS
    pub ca_cert: Option<String>,
    /// Optional username for HTTP Basic auth
    pub username: Option<String>,
    /// Optional password for HTTP Basic auth
    pub password: Option<String>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Convert a [`CfgControlDto`] (from TOML) into a [`CfgControl`].
///
/// Returns an error string if `ca_cert` is set but `use_tls` is `false`.
// Was: Diese Funktion wendet Steuerung patch.
// Warum: Die Änderung wird dadurch nur über einen definierten und prüfbaren Weg wirksam.
pub fn apply_control_patch(src: CfgControlDto) -> Result<CfgControl, String> {
    if src.ca_cert.is_some() && !src.use_tls {
        return Err("control: ca_cert requires use_tls = true".to_string());
    }

    Ok(CfgControl {
        host: src.host,
        port: src.port,
        use_tls: src.use_tls,
        credentials: match (src.username, src.password) {
            (Some(u), Some(p)) => Some((u, p)),
            (None, None) => None,
            _ => return Err("control: both username and password must be set for credentials".to_string()),
        },
        ca_cert: src.ca_cert,
    })
}
