// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Einlesen und Prüfen der TETRA-Konfiguration.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::{collections::HashMap, time::Duration};

use serde::Deserialize;
use toml::Value;

use crate::bluestation::SecretField;

/// Brew protocol (TetraPack/BrandMeister) configuration
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für cfg Brew-Verbindung in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgBrew {
    /// TetraPack server hostname or IP
    pub host: String,
    /// TetraPack server port
    pub port: u16,
    /// Use TLS (wss:// / https://)
    pub tls: bool,
    /// Optional username for HTTP Digest auth
    pub username: Option<String>,
    /// Optional password for HTTP Digest auth
    pub password: Option<SecretField>,
    /// Reconnection delay
    pub reconnect_delay: Duration,
    /// Extra initial jitter playout delay in frames (added on top of adaptive baseline)
    pub jitter_initial_latency_frames: u8,

    /// Set to true when SDS between local and Brew clients is enabled
    pub feature_sds_enabled: bool,
    /// If true, RSSI measurements are exported to the Brew server as Service (0xf4) JSON messages.
    /// Disabled by default. Enable only if the Brew server supports and expects RSSI data.
    pub feature_rssi_export: bool,
    /// If present, restrict Brew call to these remote SSIs
    pub whitelisted_ssis: Option<Vec<u32>>,
    /// Optional PBX gateway ISSIs that should be routable over Brew even if they don't match
    /// normal Tetrapack subscriber ISSI constraints.
    pub pbx_gateway_issis: Option<Vec<u32>>,
    /// Local TETRA ISSIs allowed to register and originate traffic over this Brew server.
    /// None keeps legacy single-Brew behaviour; with two Brew servers it must be set.
    pub local_issi_allowlist: Option<Vec<u32>>,
    /// Local TETRA ISSIs that must never register or originate traffic over this Brew server.
    pub local_issi_blocklist: Vec<u32>,
    /// Subscriber message type used by this Brew server for deregistration.
    pub subscriber_type_deregister: u8,
    /// Subscriber message type used by this Brew server for first registration.
    pub subscriber_type_register: u8,
    /// Subscriber message type used by this Brew server for re-registration.
    pub subscriber_type_reregister: u8,
    /// Subscriber message type used by this Brew server for group affiliation.
    pub subscriber_type_affiliate: u8,
    /// Subscriber message type used by this Brew server for group de-affiliation.
    pub subscriber_type_deaffiliate: u8,
}

#[derive(Default, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für cfg Brew-Verbindung dto in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgBrewDto {
    /// TetraPack server hostname or IP
    pub host: String,
    /// TetraPack server port
    #[serde(default = "default_brew_port")]
    pub port: u16,
    /// Use TLS (wss:// / https://)
    pub tls: bool,
    /// Optional username for HTTP Digest auth
    pub username: u32,
    /// Optional password for HTTP Digest auth
    pub password: String,
    /// Reconnection delay in seconds
    #[serde(default = "default_brew_reconnect_delay")]
    pub reconnect_delay_secs: u64,
    /// Extra initial jitter playout delay in frames (added on top of adaptive baseline)
    #[serde(default)]
    pub jitter_initial_latency_frames: u8,

    /// If present, restrict Brew call to these remote SSIs
    pub whitelisted_ssis: Option<Vec<u32>>,

    /// Set to true when SDS between local and Brew clients is enabled
    #[serde(default = "default_brew_feature_sds_enabled")]
    pub feature_sds_enabled: bool,

    /// Export RSSI measurements to the Brew server as Service JSON messages. Default: false.
    #[serde(default)]
    pub feature_rssi_export: bool,

    /// Optional PBX gateway ISSIs that should be routable over Brew even if they don't match
    /// normal Tetrapack subscriber ISSI constraints.
    #[serde(alias = "pbx_gateway_issi")]
    pub pbx_gateway_issis: Option<Vec<u32>>,

    /// Local TETRA ISSIs allowed to register and originate traffic over this Brew server.
    #[serde(default, alias = "local_issi_whitelist", alias = "issi_allowlist", alias = "issi_whitelist")]
    pub local_issi_allowlist: Option<Vec<u32>>,

    /// Local TETRA ISSIs that must never register or originate traffic over this Brew server.
    #[serde(default, alias = "local_issi_blacklist", alias = "issi_blocklist", alias = "issi_blacklist")]
    pub local_issi_blocklist: Vec<u32>,

    /// Subscriber message type mapping. Defaults are the classic Brew/TetraPack values:
    /// deregister=0, register=1, reregister=2, affiliate=8, deaffiliate=9.
    #[serde(default = "default_subscriber_type_deregister")]
    pub subscriber_type_deregister: u8,
    #[serde(default = "default_subscriber_type_register")]
    pub subscriber_type_register: u8,
    #[serde(default = "default_subscriber_type_reregister")]
    pub subscriber_type_reregister: u8,
    #[serde(default = "default_subscriber_type_affiliate")]
    pub subscriber_type_affiliate: u8,
    #[serde(default = "default_subscriber_type_deaffiliate")]
    pub subscriber_type_deaffiliate: u8,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// Was: Implementiert das zugehörige Verhalten für `CfgBrew`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl CfgBrew {
    // Was: Prüft, ob local Teilnehmerkennung (ISSI) allowlist zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn has_local_issi_allowlist(&self) -> bool {
        self.local_issi_allowlist.as_ref().is_some_and(|issis| !issis.is_empty())
    }

    // Was: Führt den Arbeitsschritt `local_issi_allowed` für local Teilnehmerkennung (ISSI) allowed aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn local_issi_allowed(&self, issi: u32) -> bool {
        if self.local_issi_blocklist.contains(&issi) {
            return false;
        }

        self.local_issi_allowlist
            .as_ref()
            .map_or(true, |allowlist| allowlist.contains(&issi))
    }

    // Was: Führt den Arbeitsschritt `effective_local_issi_allowlist` für effective local Teilnehmerkennung (ISSI) allowlist aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn effective_local_issi_allowlist(&self) -> Option<Vec<u32>> {
        self.local_issi_allowlist.as_ref().map(|allowlist| {
            allowlist
                .iter()
                .copied()
                .filter(|issi| !self.local_issi_blocklist.contains(issi))
                .collect()
        })
    }
}

// Was: Führt den Arbeitsschritt `default_brew_port` für default Brew-Verbindung port aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_brew_port() -> u16 {
    443
}

// Was: Führt den Arbeitsschritt `default_brew_reconnect_delay` für default Brew-Verbindung reconnect delay aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_brew_reconnect_delay() -> u64 {
    15
}

// Was: Führt den Arbeitsschritt `default_brew_feature_sds_enabled` für default Brew-Verbindung feature TETRA-Kurznachricht (SDS) enabled aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_brew_feature_sds_enabled() -> bool {
    true
}

// Was: Führt den Arbeitsschritt `default_subscriber_type_deregister` für default Teilnehmer type deregister aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_subscriber_type_deregister() -> u8 {
    0
}

// Was: Führt den Arbeitsschritt `default_subscriber_type_register` für default Teilnehmer type register aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_subscriber_type_register() -> u8 {
    1
}

// Was: Führt den Arbeitsschritt `default_subscriber_type_reregister` für default Teilnehmer type reregister aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_subscriber_type_reregister() -> u8 {
    2
}

// Was: Führt den Arbeitsschritt `default_subscriber_type_affiliate` für default Teilnehmer type affiliate aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_subscriber_type_affiliate() -> u8 {
    8
}

// Was: Führt den Arbeitsschritt `default_subscriber_type_deaffiliate` für default Teilnehmer type deaffiliate aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_subscriber_type_deaffiliate() -> u8 {
    9
}

/// Convert a CfgBrewDto (from TOML) into a CfgBrew (used in the stack config)
// Was: Diese Funktion wendet Brew-Verbindung patch.
// Warum: Die Änderung wird dadurch nur über einen definierten und prüfbaren Weg wirksam.
pub fn apply_brew_patch(src: CfgBrewDto) -> CfgBrew {
    CfgBrew {
        host: src.host,
        port: src.port,
        tls: src.tls,
        username: Some(src.username.to_string()),
        password: Some(SecretField::from(src.password)),
        reconnect_delay: Duration::from_secs(src.reconnect_delay_secs),
        jitter_initial_latency_frames: src.jitter_initial_latency_frames,
        feature_sds_enabled: src.feature_sds_enabled,
        feature_rssi_export: src.feature_rssi_export,
        whitelisted_ssis: src.whitelisted_ssis,
        pbx_gateway_issis: src.pbx_gateway_issis,
        local_issi_allowlist: src.local_issi_allowlist,
        local_issi_blocklist: src.local_issi_blocklist,
        subscriber_type_deregister: src.subscriber_type_deregister,
        subscriber_type_register: src.subscriber_type_register,
        subscriber_type_reregister: src.subscriber_type_reregister,
        subscriber_type_affiliate: src.subscriber_type_affiliate,
        subscriber_type_deaffiliate: src.subscriber_type_deaffiliate,
    }
}
