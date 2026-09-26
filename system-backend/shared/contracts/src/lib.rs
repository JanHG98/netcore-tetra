// NETCORE-KOMMENTAR – Was: Enthält die Logik oder Einstellungen für lib.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Versioned, transport-neutral contracts shared by NetCore-Tetra backend services.
//!
//! The crate deliberately contains no networking, storage, authentication or service-owned
//! state. It defines stable wire shapes and validated identifiers so backend services do not
//! silently drift into incompatible private JSON dialects.

// Was: Bindet das Untermodul address in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod address;
mod command;
// Was: Bindet das Untermodul envelope in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod envelope;
// Was: Bindet das Untermodul Ereignis in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod event;
// Was: Bindet das Untermodul health in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod health;
// Was: Bindet das Untermodul pagination in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod pagination;
// Was: Bindet das Untermodul problem in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod problem;
// Was: Bindet das Untermodul Dienst in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod service;

pub use address::{AddressError, Gssi, Issi, Ssi};
pub use command::{
    CommandAck, CommandAckStatus, CommandPolicyDecision, CommandSource, CommandTarget,
    CommandValidationError, NETCORE_COMMAND_ACK_SCHEMA_V1, NETCORE_COMMAND_SCHEMA_V1,
    NetCoreCommand, is_command_type,
};
pub use envelope::{ApiVersion, DeliverySemantics, Envelope, EnvelopeMeta, MessageKind, TraceContext};
pub use event::{
    AuditRecord, EventRecord, EventSource, EventSubject, EventValidationError, NetCoreEvent,
    NETCORE_EVENT_SCHEMA_V1, Severity, event_types, is_event_type, subject_types,
};
pub use health::{BuildInfo, DependencyHealth, HealthDocument, HealthStatus};
pub use pagination::{Page, PageRequest};
pub use problem::ProblemDetails;
pub use service::{Compatibility, OperatingMode, SecurityMode, ServiceCapability, ServiceDescriptor};
