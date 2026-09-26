// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Verbindungen zu anderen Netzen und Systemen.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

// Was: Legt den festen Wert `OPEN_LAB_MODE` für open lab mode fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const OPEN_LAB_MODE: &str = "open_lab";
// Was: Legt den festen Wert `MODE_SHADOW` für mode shadow fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const MODE_SHADOW: &str = "shadow";
// Was: Legt den festen Wert `MODE_AUTHORITATIVE` für mode authoritative fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const MODE_AUTHORITATIVE: &str = "authoritative";
// Was: Legt den festen Wert `TRANSIT_PROTOCOL_VERSION` für Netzübergang protocol version fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const TRANSIT_PROTOCOL_VERSION: &str = "netcore-transit-v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für Netzübergang Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TransitConfig {
    pub server: ServerConfig,
    pub storage: StorageConfig,
    pub region: RegionConfig,
    pub routing: RoutingConfig,
    pub transport: TransportConfig,
    pub security: SecurityConfig,
    pub limits: LimitsConfig,
}

// Was: Implementiert das zugehörige Verhalten für `Default for TransitConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for TransitConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            storage: StorageConfig::default(),
            region: RegionConfig::default(),
            routing: RoutingConfig::default(),
            transport: TransportConfig::default(),
            security: SecurityConfig::default(),
            limits: LimitsConfig::default(),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `TransitConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl TransitConfig {
    // Was: Diese Funktion lädt den vorgesehenen Arbeitsschritt.
    // Warum: Einlesen und Fehlerbehandlung bleiben dadurch an einer zentralen Stelle.
    pub fn load(path: Option<&Path>) -> Result<Self, Box<dyn std::error::Error>> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        let mut config = match path {
            Some(path) => toml::from_str::<Self>(&fs::read_to_string(path)?)?,
            None => Self::default(),
        };
        config
            .normalise()
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidInput, error))?;
        Ok(config)
    }

    // Was: Diese Funktion wendet bind override.
    // Warum: Die Änderung wird dadurch nur über einen definierten und prüfbaren Weg wirksam.
    pub fn apply_bind_override(&mut self, bind: Option<SocketAddr>) -> Result<(), String> {
        if let Some(bind) = bind {
            self.server.bind = bind;
        }
        self.normalise()
    }

    // Was: Führt den Arbeitsschritt `normalise` für normalise aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn normalise(&mut self) -> Result<(), String> {
        if self.security.mode != OPEN_LAB_MODE {
            return Err(format!(
                "unsupported security.mode={}; this phase intentionally supports open_lab only",
                self.security.mode
            ));
        }
        if self.security.tls || self.security.token_auth {
            return Err("security.tls and security.token_auth must remain false in the open_lab package".to_string());
        }
        if !matches!(
            self.region.operating_mode.as_str(),
            MODE_SHADOW | MODE_AUTHORITATIVE
        ) {
            return Err("region.operating_mode must be shadow or authoritative".to_string());
        }
        if self.region.region_id.trim().is_empty() || self.region.swmi_id.trim().is_empty() {
            return Err("region.region_id and region.swmi_id must not be empty".to_string());
        }
        if self.region.protocol_version != TRANSIT_PROTOCOL_VERSION {
            return Err(format!(
                "region.protocol_version must be {TRANSIT_PROTOCOL_VERSION} in this package"
            ));
        }
        if !self.region.advertised_endpoint.starts_with("http://") {
            return Err("region.advertised_endpoint must use http:// in open_lab mode".to_string());
        }
        if !self.security.allow_remote_management && !self.server.bind.ip().is_loopback() {
            return Err(
                "server.bind must be loopback when security.allow_remote_management=false"
                    .to_string(),
            );
        }
        if self.routing.max_hops == 0 || self.routing.max_hops > 64 {
            return Err("routing.max_hops must be between 1 and 64".to_string());
        }
        if self.transport.max_attempts == 0 {
            return Err("transport.max_attempts must be at least 1".to_string());
        }
        self.server.history_limit = self.server.history_limit.max(100);
        self.routing.dedupe_ttl_secs = self.routing.dedupe_ttl_secs.max(30);
        self.routing.session_idle_ttl_secs = self.routing.session_idle_ttl_secs.max(60);
        self.routing.route_stale_secs = self.routing.route_stale_secs.max(30);
        self.transport.connect_timeout_ms = self.transport.connect_timeout_ms.max(100);
        self.transport.io_timeout_ms = self.transport.io_timeout_ms.max(100);
        self.transport.heartbeat_interval_secs = self.transport.heartbeat_interval_secs.max(1);
        self.transport.peer_timeout_secs = self
            .transport
            .peer_timeout_secs
            .max(self.transport.heartbeat_interval_secs + 1);
        self.transport.retry_backoff_secs = self.transport.retry_backoff_secs.max(1);
        self.transport.max_batch = self.transport.max_batch.clamp(1, 1_000);
        self.limits.max_body_bytes = self.limits.max_body_bytes.max(4_096);
        self.limits.max_peers = self.limits.max_peers.max(1);
        self.limits.max_routes = self.limits.max_routes.max(1);
        self.limits.max_sessions = self.limits.max_sessions.max(16);
        self.limits.max_envelopes = self.limits.max_envelopes.max(100);
        self.limits.max_local_deliveries = self.limits.max_local_deliveries.max(100);
        self.limits.max_events = self.limits.max_events.max(100);
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für server Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ServerConfig {
    pub bind: SocketAddr,
    pub history_limit: usize,
}
// Was: Implementiert das zugehörige Verhalten für `Default for ServerConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for ServerConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            bind: "0.0.0.0:8200".parse().expect("valid default bind"),
            history_limit: 5_000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für storage Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct StorageConfig {
    pub database_path: PathBuf,
    pub backup_path: PathBuf,
}
// Was: Implementiert das zugehörige Verhalten für `Default for StorageConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for StorageConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            database_path: "/var/lib/netcore-transit/state.json".into(),
            backup_path: "/var/lib/netcore-transit/state.json.bak".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für region Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RegionConfig {
    pub region_id: String,
    pub swmi_id: String,
    pub display_name: String,
    pub advertised_endpoint: String,
    pub protocol_version: String,
    pub operating_mode: String,
    pub capabilities: Vec<String>,
}
// Was: Implementiert das zugehörige Verhalten für `Default for RegionConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for RegionConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            region_id: "region-a".to_string(),
            swmi_id: "netcore-swmi-a".to_string(),
            display_name: "NetCore Region A".to_string(),
            advertised_endpoint: "http://127.0.0.1:8200".to_string(),
            protocol_version: TRANSIT_PROTOCOL_VERSION.to_string(),
            operating_mode: MODE_SHADOW.to_string(),
            capabilities: vec![
                "mobility".to_string(),
                "individual_call".to_string(),
                "group_call".to_string(),
                "sds".to_string(),
                "media".to_string(),
                "supplementary_service".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für routing Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RoutingConfig {
    pub max_hops: u8,
    pub dedupe_ttl_secs: u64,
    pub session_idle_ttl_secs: u64,
    pub route_stale_secs: u64,
    pub prefer_direct_region_peer: bool,
    pub allow_transitive_routing: bool,
    pub allow_dynamic_peers: bool,
    pub fail_closed_on_loop: bool,
}
// Was: Implementiert das zugehörige Verhalten für `Default for RoutingConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for RoutingConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            max_hops: 8,
            dedupe_ttl_secs: 900,
            session_idle_ttl_secs: 3_600,
            route_stale_secs: 120,
            prefer_direct_region_peer: true,
            allow_transitive_routing: true,
            allow_dynamic_peers: false,
            fail_closed_on_loop: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für transport Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TransportConfig {
    pub connect_timeout_ms: u64,
    pub io_timeout_ms: u64,
    pub heartbeat_interval_secs: u64,
    pub peer_timeout_secs: u64,
    pub retry_backoff_secs: u64,
    pub max_attempts: u32,
    pub max_batch: usize,
}
// Was: Implementiert das zugehörige Verhalten für `Default for TransportConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for TransportConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            connect_timeout_ms: 2_000,
            io_timeout_ms: 5_000,
            heartbeat_interval_secs: 5,
            peer_timeout_secs: 20,
            retry_backoff_secs: 3,
            max_attempts: 5,
            max_batch: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für Sicherheit Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct SecurityConfig {
    pub mode: String,
    pub allow_remote_management: bool,
    pub tls: bool,
    pub token_auth: bool,
}
// Was: Implementiert das zugehörige Verhalten für `Default for SecurityConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for SecurityConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            mode: OPEN_LAB_MODE.to_string(),
            allow_remote_management: true,
            tls: false,
            token_auth: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für limits Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LimitsConfig {
    pub max_body_bytes: usize,
    pub max_peers: usize,
    pub max_routes: usize,
    pub max_sessions: usize,
    pub max_envelopes: usize,
    pub max_local_deliveries: usize,
    pub max_events: usize,
}
// Was: Implementiert das zugehörige Verhalten für `Default for LimitsConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for LimitsConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            max_body_bytes: 4_194_304,
            max_peers: 1_000,
            max_routes: 100_000,
            max_sessions: 100_000,
            max_envelopes: 500_000,
            max_local_deliveries: 500_000,
            max_events: 100_000,
        }
    }
}
