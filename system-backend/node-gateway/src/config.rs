// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für die Verbindung zwischen Basisstationen und Backend-Diensten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::fs;
use std::net::SocketAddr;
use std::path::Path;

use serde::{Deserialize, Serialize};

// Was: Legt den festen Wert `OPEN_LAB_MODE` für open lab mode fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const OPEN_LAB_MODE: &str = "open_lab";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für Netzknoten Gateway Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct NodeGatewayConfig {
    pub server: ServerConfig,
    pub security: SecurityConfig,
    pub limits: LimitsConfig,
    pub service_monitor: ServiceMonitorConfig,
}

// Was: Implementiert das zugehörige Verhalten für `Default for NodeGatewayConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for NodeGatewayConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            security: SecurityConfig::default(),
            limits: LimitsConfig::default(),
            service_monitor: ServiceMonitorConfig::default(),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `NodeGatewayConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl NodeGatewayConfig {
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
        self.server.node_path = normalise_path(&self.server.node_path);
        self.server.backend_path = normalise_path(&self.server.backend_path);
        if self.server.node_path == self.server.backend_path {
            return Err("server.node_path and server.backend_path must differ".to_string());
        }
        if self.server.history_limit == 0 {
            self.server.history_limit = 1_000;
        }
        if self.server.stale_after_secs < 10 {
            self.server.stale_after_secs = 10;
        }
        if self.server.hello_timeout_secs < 2 {
            self.server.hello_timeout_secs = 2;
        }
        if self.server.application_ping_secs < 5 {
            self.server.application_ping_secs = 5;
        }
        if self.limits.max_message_bytes < 4_096 {
            self.limits.max_message_bytes = 4_096;
        }
        if self.limits.max_http_body_bytes < 4_096 {
            self.limits.max_http_body_bytes = 4_096;
        }
        if self.service_monitor.interval_secs < 2 {
            self.service_monitor.interval_secs = 2;
        }
        if self.service_monitor.timeout_ms < 100 {
            self.service_monitor.timeout_ms = 100;
        }
        self.service_monitor.failure_threshold = self.service_monitor.failure_threshold.max(1);
        self.service_monitor.recovery_threshold = self.service_monitor.recovery_threshold.max(1);
        let mut names = std::collections::HashSet::new();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for target in &mut self.service_monitor.targets {
            target.name = target.name.trim().to_ascii_lowercase();
            target.url = target.url.trim().to_string();
            target.fallback_mode = target.fallback_mode.trim().to_string();
            if !safe_service_name(&target.name) || !names.insert(target.name.clone()) {
                return Err(format!("invalid or duplicate service_monitor target name {:?}", target.name));
            }
            if !target.url.starts_with("http://") {
                return Err(format!("service_monitor target {} must use an explicit http:// URL in open_lab", target.name));
            }
            if target.fallback_mode.is_empty() {
                return Err(format!("service_monitor target {} needs fallback_mode", target.name));
            }
        }
        if self.security.mode.trim().to_ascii_lowercase() != OPEN_LAB_MODE {
            return Err(format!(
                "unsupported security.mode={:?}; this package intentionally implements only open_lab and does not pretend to provide token security",
                self.security.mode
            ));
        }
        self.security.mode = OPEN_LAB_MODE.to_string();
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für server Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ServerConfig {
    pub bind: SocketAddr,
    pub node_path: String,
    pub backend_path: String,
    pub history_limit: usize,
    pub stale_after_secs: u64,
    pub hello_timeout_secs: u64,
    pub application_ping_secs: u64,
}

// Was: Implementiert das zugehörige Verhalten für `Default for ServerConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for ServerConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            bind: "0.0.0.0:8080".parse().expect("static bind address is valid"),
            node_path: "/ws/node".to_string(),
            backend_path: "/ws/backend".to_string(),
            history_limit: 1_000,
            stale_after_secs: 20,
            hello_timeout_secs: 10,
            application_ping_secs: 15,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für Sicherheit Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct SecurityConfig {
    /// Deliberately only `open_lab` in this package. There are no tokens, users or certificates.
    pub mode: String,
    /// Allows write operations from the WebUI/API. Keep true only in an isolated test network.
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
// Was: Bündelt die zusammengehörigen Werte für Dienst monitor Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ServiceMonitorConfig {
    pub enabled: bool,
    pub interval_secs: u64,
    pub timeout_ms: u64,
    pub failure_threshold: u32,
    pub recovery_threshold: u32,
    pub targets: Vec<ServiceTargetConfig>,
}

// Was: Implementiert das zugehörige Verhalten für `Default for ServiceMonitorConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for ServiceMonitorConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            enabled: true,
            interval_secs: 5,
            timeout_ms: 1_500,
            failure_threshold: 2,
            recovery_threshold: 2,
            targets: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für Dienst target Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ServiceTargetConfig {
    pub name: String,
    pub url: String,
    pub critical_for_edge: bool,
    pub fallback_mode: String,
}

// Was: Implementiert das zugehörige Verhalten für `Default for ServiceTargetConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for ServiceTargetConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            name: String::new(),
            url: String::new(),
            critical_for_edge: false,
            fallback_mode: "local_degraded_operation".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für limits Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LimitsConfig {
    pub max_message_bytes: usize,
    pub max_http_body_bytes: usize,
}

// Was: Implementiert das zugehörige Verhalten für `Default for LimitsConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for LimitsConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            max_message_bytes: 1_048_576,
            max_http_body_bytes: 1_048_576,
        }
    }
}

// Was: Führt den Arbeitsschritt `safe_service_name` für safe Dienst name aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn safe_service_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

// Was: Führt den Arbeitsschritt `normalise_path` für normalise path aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn normalise_path(path: &str) -> String {
    let trimmed = path.trim();
    if trimmed.starts_with('/') {
        trimmed.to_string()
    } else {
        format!("/{trimmed}")
    }
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Führt den Arbeitsschritt `defaults_are_explicitly_open_lab_without_tokens` für defaults are explicitly open lab without tokens aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn defaults_are_explicitly_open_lab_without_tokens() {
        let cfg = NodeGatewayConfig::default();
        assert_eq!(cfg.security.mode, OPEN_LAB_MODE);
        assert!(cfg.security.allow_remote_management);
        assert_eq!(cfg.server.bind.port(), 8080);
    }

    #[test]
    // Was: Führt den Arbeitsschritt `normalises_service_monitor_targets` für normalises Dienst monitor targets aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn normalises_service_monitor_targets() {
        let mut cfg = NodeGatewayConfig::default();
        cfg.service_monitor.targets.push(ServiceTargetConfig {
            name: " SDS-Router ".to_string(),
            url: "http://127.0.0.1:8150/health/ready".to_string(),
            critical_for_edge: true,
            fallback_mode: "local_delivery".to_string(),
        });
        cfg.normalise().unwrap();
        assert_eq!(cfg.service_monitor.targets[0].name, "sds-router");
    }

    #[test]
    // Was: Führt den Arbeitsschritt `refuses_fake_secure_modes` für refuses fake secure modes aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn refuses_fake_secure_modes() {
        let mut cfg = NodeGatewayConfig::default();
        cfg.security.mode = "token".to_string();
        assert!(cfg.normalise().is_err());
    }
}
