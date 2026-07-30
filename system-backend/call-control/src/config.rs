// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Rufaufbau, Rufzustände und Rufwiederherstellung.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

// Was: Legt den festen Wert `OPEN_LAB_MODE` für open lab mode fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const OPEN_LAB_MODE: &str = "open_lab";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für Ruf Steuerung Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CallControlConfig {
    pub server: ServerConfig,
    pub node_gateway: NodeGatewayConfig,
    pub mobility_core: MobilityCoreClientConfig,
    pub storage: StorageConfig,
    pub calls: CallsConfig,
    pub security: SecurityConfig,
    pub limits: LimitsConfig,
}

// Was: Implementiert das zugehörige Verhalten für `Default for CallControlConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for CallControlConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            node_gateway: NodeGatewayConfig::default(),
            mobility_core: MobilityCoreClientConfig::default(),
            storage: StorageConfig::default(),
            calls: CallsConfig::default(),
            security: SecurityConfig::default(),
            limits: LimitsConfig::default(),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `CallControlConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl CallControlConfig {
    // Was: Diese Funktion lädt den vorgesehenen Arbeitsschritt.
    // Warum: Einlesen und Fehlerbehandlung bleiben dadurch an einer zentralen Stelle.
    pub fn load(path: Option<&Path>) -> Result<Self, Box<dyn std::error::Error>> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        let mut config = match path {
            Some(path) => toml::from_str::<Self>(&fs::read_to_string(path)?)?,
            None => Self::default(),
        };
        config.normalise().map_err(|error| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, error)
        })?;
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
                "unsupported security.mode={}; this lab package intentionally implements only open_lab",
                self.security.mode
            ));
        }
        if !self.node_gateway.url.starts_with("ws://") {
            return Err("node_gateway.url must use ws:// in the open lab package".to_string());
        }
        if self.mobility_core.enabled && !self.mobility_core.base_url.starts_with("http://") {
            return Err("mobility_core.base_url must use http:// in the open lab package".to_string());
        }
        if !self.security.allow_remote_management && !self.server.bind.ip().is_loopback() {
            return Err(
                "server.bind must use a loopback address when allow_remote_management=false"
                    .to_string(),
            );
        }
        self.server.history_limit = self.server.history_limit.max(100);
        self.node_gateway.reconnect_secs = self.node_gateway.reconnect_secs.max(1);
        self.mobility_core.timeout_ms = self.mobility_core.timeout_ms.clamp(100, 10_000);
        self.calls.command_timeout_secs = self.calls.command_timeout_secs.max(5);
        self.calls.restore_timeout_secs = self.calls.restore_timeout_secs.max(10);
        self.calls.reconcile_interval_secs = self.calls.reconcile_interval_secs.max(1);
        self.limits.max_body_bytes = self.limits.max_body_bytes.max(1_024);
        self.limits.max_calls = self.limits.max_calls.max(1);
        self.limits.max_legs_per_call = self.limits.max_legs_per_call.max(1);
        self.limits.max_pending_commands = self.limits.max_pending_commands.max(10);
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
            bind: "0.0.0.0:8120".parse().expect("valid default bind"),
            history_limit: 2_000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für Netzknoten Gateway Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct NodeGatewayConfig {
    pub url: String,
    pub reconnect_secs: u64,
}
// Was: Implementiert das zugehörige Verhalten für `Default for NodeGatewayConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for NodeGatewayConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            url: "ws://127.0.0.1:8080/ws/backend".to_string(),
            reconnect_secs: 5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MobilityCoreClientConfig {
    pub enabled: bool,
    pub base_url: String,
    pub timeout_ms: u64,
    pub allow_local_fallback: bool,
    pub accept_stale_route: bool,
}
impl Default for MobilityCoreClientConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            base_url: "http://127.0.0.1:8090".to_string(),
            timeout_ms: 1500,
            allow_local_fallback: false,
            accept_stale_route: false,
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
            database_path: "/var/lib/netcore-call-control/calls.json".into(),
            backup_path: "/var/lib/netcore-call-control/calls.json.bak".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für calls Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CallsConfig {
    pub command_timeout_secs: u64,
    pub restore_timeout_secs: u64,
    pub reconcile_interval_secs: u64,
    pub auto_target_affiliated_nodes: bool,
    pub release_partial_start_on_failure: bool,
    pub allow_operator_force_floor: bool,
}
// Was: Implementiert das zugehörige Verhalten für `Default for CallsConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for CallsConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            command_timeout_secs: 30,
            restore_timeout_secs: 45,
            reconcile_interval_secs: 2,
            auto_target_affiliated_nodes: true,
            release_partial_start_on_failure: false,
            allow_operator_force_floor: true,
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
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für limits Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LimitsConfig {
    pub max_body_bytes: usize,
    pub max_calls: usize,
    pub max_legs_per_call: usize,
    pub max_pending_commands: usize,
}
// Was: Implementiert das zugehörige Verhalten für `Default for LimitsConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for LimitsConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            max_body_bytes: 2_097_152,
            max_calls: 100_000,
            max_legs_per_call: 1_024,
            max_pending_commands: 20_000,
        }
    }
}
