// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! External networked telemetry component.
//!
//! `TelemetryEvent` is a protocol type and is available without `runtime` so a
//! Control Room Core can deserialize base-station events without linking RF or
//! audio libraries. Channels/workers/codecs are base-station runtime pieces.

#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul Kanal in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod channel;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul codec in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod codec;
// Was: Bindet das Untermodul events in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod events;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul Hintergrundverarbeitung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod worker;

use std::time::Duration;

#[cfg(feature = "runtime")]
pub use self::channel::{TelemetrySink, TelemetrySource, telemetry_channel};
pub use self::events::{TelemetryEvent, telemetry_source_for_entity};
#[cfg(feature = "runtime")]
pub use self::worker::TelemetryWorker;

/// Sent as subprotocol in WebSocket handshake.
// Was: Legt den festen Wert `TELEMETRY_PROTOCOL_VERSION` für Telemetrie protocol version fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const TELEMETRY_PROTOCOL_VERSION: &str = "bluestation-telemetry-v1";
// Was: Legt den festen Wert `TELEMETRY_HEARTBEAT_INTERVAL` für Telemetrie heartbeat interval fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const TELEMETRY_HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);
// Was: Legt den festen Wert `TELEMETRY_HEARTBEAT_TIMEOUT` für Telemetrie heartbeat timeout fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const TELEMETRY_HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(30);
