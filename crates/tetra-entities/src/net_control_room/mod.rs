// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! NetCore Control-Room node side / shared protocol.
//!
//! Protocol structs are always available.  The base-station worker is gated
//! behind `runtime` so the Control Room Core can use the protocol without SDR,
//! Brew, dashboard or voice-codec dependencies.

// Was: Bindet das Untermodul codec in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod codec;
// Was: Bindet das Untermodul protocol in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod protocol;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul edge store in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod edge_store;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul Hintergrundverarbeitung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod worker;

use std::time::Duration;

pub use self::codec::{ControlRoomCodecError, ControlRoomCodecJson};
pub use self::protocol::*;
#[cfg(feature = "runtime")]
pub use self::edge_store::{EdgeEventSpool, load_edge_policy_cache, persist_edge_policy_cache};
#[cfg(feature = "runtime")]
pub use self::worker::ControlRoomWorker;

/// Sent as WebSocket subprotocol in the node <-> control-room handshake.
// Was: Legt den festen Wert `CONTROL_ROOM_PROTOCOL_VERSION` für Steuerung room protocol version fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CONTROL_ROOM_PROTOCOL_VERSION: &str = "netcore-control-room-node-v1";

/// Node heartbeat cadence.  The WebSocket transport also has its own ping/pong;
/// this application heartbeat is visible to the Leitstelle state model.
// Was: Legt den festen Wert `CONTROL_ROOM_HEARTBEAT_INTERVAL` für Steuerung room heartbeat interval fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CONTROL_ROOM_HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// Transport heartbeat timeout.  Keep this a little wider than the heartbeat
/// interval so brief RF/CPU spikes do not flap the Leitstelle connection.
// Was: Legt den festen Wert `CONTROL_ROOM_HEARTBEAT_TIMEOUT` für Steuerung room heartbeat timeout fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CONTROL_ROOM_HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(30);
