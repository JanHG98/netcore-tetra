// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Lite stack-health for FlowStation.
//!
//! The serialisable health data types are always available because telemetry
//! protocol consumers need to deserialize `TelemetryEvent::HealthSnapshot`.
//! The live registry/supervisor are base-station runtime concerns and are only
//! compiled with the `runtime` feature.

// Was: Bindet das Untermodul types in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod types;

#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul registry in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod registry;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul supervisor in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod supervisor;

#[cfg(feature = "runtime")]
pub use registry::{HealthRegistry, HealthThresholds, registry};
#[cfg(feature = "runtime")]
pub use supervisor::{HealthMonitorConfig, spawn_health_monitor};
pub use types::{DomainHealth, HealthDomain, HealthLevel, HealthSnapshot};
