// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Brew protocol integration for TETRA group call bridging via a pluggable network transport
//!
//! The transport (WebSocket, QUIC, TCP, …) is injected at construction time.
//! See [`websocket_transport_config`] for the default WebSocket configuration
//! used with TetraPack/BrandMeister.

// Was: Bindet das Untermodul components in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod components;
// Was: Bindet das Untermodul entity in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod entity;
// Was: Bindet das Untermodul protocol in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod protocol;
// Was: Bindet das Untermodul Hintergrundverarbeitung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod worker;

/// Convenience re-export of commonly externally used functions
pub use components::brew_routable::{
    BREW_ENTITIES, is_active, is_active_for_entity, is_brew_entity, is_brew_gssi_routable, is_brew_gssi_routable_for_entity,
    is_brew_inbound_allowed, is_brew_inbound_allowed_for_entity, is_brew_issi_routable, is_brew_issi_routable_for_entity,
    is_brew_local_issi_allowed_for_entity, route_entity_for_local_issi,
};
pub use components::brew_routable::{brew_config_for_entity, feature_sds_enabled, feature_sds_enabled_for_entity};

use std::time::Duration;

use crate::network::transports::websocket::{WebSocketTransport, WebSocketTransportConfig};
use tetra_config::bluestation::CfgBrew;

// Was: Legt den festen Wert `BREW_PROTOCOL_VERSION` für Brew-Verbindung protocol version fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const BREW_PROTOCOL_VERSION: &str = "brew";

/// Build a [`WebSocketTransportConfig`] from the Brew section of the stack config.
///
/// This wires the Brew-specific defaults (endpoint path `/brew/`, subprotocol `"brew"`,
/// heartbeat intervals) into the generic WebSocket transport.
// Was: Führt den Arbeitsschritt `websocket_transport_config` für websocket transport Konfiguration aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn websocket_transport_config(cfg: &CfgBrew) -> WebSocketTransportConfig {
    WebSocketTransportConfig {
        host: cfg.host.clone(),
        port: cfg.port,
        use_tls: cfg.tls,
        digest_auth_credentials: match (&cfg.username, &cfg.password) {
            (Some(u), Some(p)) => Some((u.clone(), p.clone())),
            _ => None,
        },
        endpoint_path: "/brew/".to_string(),
        subprotocol: Some(BREW_PROTOCOL_VERSION.to_string()),
        user_agent: format!("FlowStation/{}", tetra_core::STACK_VERSION),
        heartbeat_interval: Duration::from_secs(10),
        heartbeat_timeout: Duration::from_secs(30),
        custom_root_certs: None,
        basic_auth_credentials: None,
    }
}

/// Create a [`WebSocketTransport`] configured for Brew from the stack config.
// Was: Diese Funktion erstellt websocket transport.
// Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
pub fn new_websocket_transport(cfg: &CfgBrew) -> WebSocketTransport {
    WebSocketTransport::new(websocket_transport_config(cfg))
}
