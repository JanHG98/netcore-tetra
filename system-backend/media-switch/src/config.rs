// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für die Audioverteilung zwischen Basisstationen und Rufen.
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
// Was: Bündelt die zusammengehörigen Werte für Audio- und Mediendaten switch Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MediaSwitchConfig {
    pub server: ServerConfig,
    pub node_gateway: NodeGatewayConfig,
    pub call_control: CallControlConfig,
    pub media: MediaConfig,
    pub security: SecurityConfig,
    pub limits: LimitsConfig,
}

// Was: Implementiert das zugehörige Verhalten für `Default for MediaSwitchConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for MediaSwitchConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            node_gateway: NodeGatewayConfig::default(),
            call_control: CallControlConfig::default(),
            media: MediaConfig::default(),
            security: SecurityConfig::default(),
            limits: LimitsConfig::default(),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `MediaSwitchConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MediaSwitchConfig {
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
                "unsupported security.mode={}; this package intentionally implements only open_lab",
                self.security.mode
            ));
        }
        if !self.node_gateway.url.starts_with("ws://") {
            return Err("node_gateway.url must use ws:// in open_lab mode".to_string());
        }
        if !self.call_control.url.starts_with("http://") {
            return Err("call_control.url must use http:// in open_lab mode".to_string());
        }
        if !self.call_control.events_url.starts_with("ws://") {
            return Err("call_control.events_url must use ws:// in open_lab mode".to_string());
        }
        if !self.call_control.route_ready_url.starts_with("http://") {
            return Err(
                "call_control.route_ready_url must use http:// in open_lab mode".to_string(),
            );
        }
        if !self.security.allow_remote_management && !self.server.bind.ip().is_loopback() {
            return Err(
                "server.bind must use a loopback address when allow_remote_management=false"
                    .to_string(),
            );
        }

        self.server.history_limit = self.server.history_limit.max(100);
        self.node_gateway.reconnect_secs = self.node_gateway.reconnect_secs.max(1);
        self.call_control.reconcile_secs = self.call_control.reconcile_secs.max(5);
        self.call_control.reconnect_secs = self.call_control.reconnect_secs.max(1);
        self.call_control.request_timeout_secs = self.call_control.request_timeout_secs.max(1);
        self.media.frame_duration_ms = self.media.frame_duration_ms.clamp(10, 1_000);
        self.media.max_jitter_buffer_frames = self.media.max_jitter_buffer_frames.max(1);
        self.media.min_jitter_buffer_frames = self
            .media
            .min_jitter_buffer_frames
            .clamp(1, self.media.max_jitter_buffer_frames);
        self.media.jitter_buffer_frames = self
            .media
            .jitter_buffer_frames
            .clamp(
                self.media.min_jitter_buffer_frames,
                self.media.max_jitter_buffer_frames,
            );
        self.media.cold_start_buffer_frames = self.media.cold_start_buffer_frames.clamp(1, 32);
        self.media.cold_start_buffer_max_age_ms =
            self.media.cold_start_buffer_max_age_ms.clamp(60, 5_000);
        self.media.adaptive_jitter_up_threshold_ms =
            self.media.adaptive_jitter_up_threshold_ms.clamp(1, 1_000);
        self.media.adaptive_jitter_down_stable_frames =
            self.media.adaptive_jitter_down_stable_frames.max(10);
        self.media.session_idle_secs = self.media.session_idle_secs.max(5);
        self.media.max_frames_per_tick = self.media.max_frames_per_tick.max(1);
        self.media.tap_history_frames = self.media.tap_history_frames.max(16);
        self.media.recorder_tap_history_frames =
            self.media.recorder_tap_history_frames.max(256);
        self.limits.max_body_bytes = self.limits.max_body_bytes.max(1_024);
        self.limits.max_sessions = self.limits.max_sessions.max(1);
        self.limits.max_streams = self.limits.max_streams.max(2);
        self.limits.max_pending_frames = self.limits.max_pending_frames.max(32);
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
            bind: "0.0.0.0:8130".parse().expect("valid default bind"),
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
            reconnect_secs: 2,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für Ruf Steuerung Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CallControlConfig {
    /// HTTP snapshot endpoint used only for startup/fallback reconciliation.
    pub url: String,
    /// Event-driven WebSocket endpoint used by the live media path.
    pub events_url: String,
    /// Call Control endpoint used to confirm that all media legs are routable.
    pub route_ready_url: String,
    /// Safety reconciliation interval while the event socket is healthy.
    pub reconcile_secs: u64,
    pub reconnect_secs: u64,
    pub request_timeout_secs: u64,
}

// Was: Implementiert das zugehörige Verhalten für `Default for CallControlConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for CallControlConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            url: "http://127.0.0.1:8120/api/v1/calls".to_string(),
            events_url: "ws://127.0.0.1:8120/ws/media".to_string(),
            route_ready_url: "http://127.0.0.1:8120/api/v1/media/route-ready".to_string(),
            reconcile_secs: 15,
            reconnect_secs: 1,
            request_timeout_secs: 2,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für Audio- und Mediendaten Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MediaConfig {
    pub frame_duration_ms: u64,
    /// Startup target. Two packed 60-ms frames keep the normal path direct.
    pub jitter_buffer_frames: usize,
    pub min_jitter_buffer_frames: usize,
    pub max_jitter_buffer_frames: usize,
    pub adaptive_jitter: bool,
    pub adaptive_jitter_up_threshold_ms: u64,
    pub adaptive_jitter_down_stable_frames: u32,
    /// Preserve the first speech packets while the call/leg event is in flight.
    pub cold_start_buffer_frames: usize,
    pub cold_start_buffer_max_age_ms: u64,
    pub session_idle_secs: u64,
    pub max_frames_per_tick: usize,
    pub allow_same_leg_loopback: bool,
    pub tap_history_frames: usize,
    pub recorder_tap_history_frames: usize,
}

// Was: Implementiert das zugehörige Verhalten für `Default for MediaConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for MediaConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            frame_duration_ms: 60,
            jitter_buffer_frames: 2,
            min_jitter_buffer_frames: 1,
            max_jitter_buffer_frames: 12,
            adaptive_jitter: true,
            adaptive_jitter_up_threshold_ms: 18,
            adaptive_jitter_down_stable_frames: 120,
            cold_start_buffer_frames: 5,
            cold_start_buffer_max_age_ms: 600,
            session_idle_secs: 30,
            max_frames_per_tick: 256,
            allow_same_leg_loopback: false,
            tap_history_frames: 256,
            recorder_tap_history_frames: 20_000,
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
    pub max_sessions: usize,
    pub max_streams: usize,
    pub max_pending_frames: usize,
}

// Was: Implementiert das zugehörige Verhalten für `Default for LimitsConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for LimitsConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            max_body_bytes: 1_048_576,
            max_sessions: 10_000,
            max_streams: 50_000,
            max_pending_frames: 100_000,
        }
    }
}
