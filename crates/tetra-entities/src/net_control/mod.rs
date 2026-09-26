// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! External networked command component.
//!
//! `ControlCommand` and `ControlResponse` are protocol types and are available
//! without the `runtime` feature.  Channels/workers/codecs are only needed on
//! the base-station side and stay runtime-gated.

#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul Kanal in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod channel;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul codec in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod codec;
// Was: Bindet das Untermodul commands in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod commands;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul Hintergrundverarbeitung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod worker;

use std::time::Duration;

#[cfg(feature = "runtime")]
pub use self::channel::{CommandDispatcher, ControlEndpoint, make_control_link};
pub use self::commands::{
    ControlCommand, ControlResponse, GroupMembershipPolicy, GroupPolicyDefinition,
    ManagedCallKind, ManagedCallRestoreContextPayload, ManagedNetworkCircuitCallPayload,
    MobilityClassOfMs, MobilityClientState, MobilityContextPayload,
};
#[cfg(feature = "runtime")]
pub use self::worker::ControlWorker;

/// Sent as subprotocol in WebSocket handshake.
// Was: Legt den festen Wert `CONTROL_PROTOCOL_VERSION` für Steuerung protocol version fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CONTROL_PROTOCOL_VERSION: &str = "bluestation-control-v1";
// Was: Legt den festen Wert `CONTROL_HEARTBEAT_INTERVAL` für Steuerung heartbeat interval fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CONTROL_HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);
// Was: Legt den festen Wert `CONTROL_HEARTBEAT_TIMEOUT` für Steuerung heartbeat timeout fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CONTROL_HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(30);
