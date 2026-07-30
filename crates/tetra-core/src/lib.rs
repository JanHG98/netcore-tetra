// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für grundlegende TETRA-Datentypen und Hilfsfunktionen.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Core utilities for TETRA BlueStation
//!
//! This crate provides fundamental types and utilities used across the TETRA stack

/// Short git commit hash, set at compile time (e.g. "2aad62c8"). No `g` prefix: the empty `--match=`
/// makes `git describe --always` emit the bare abbreviated commit hash, not a tag-relative name.
// Was: Legt den festen Wert `GIT_HASH` für git hash fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const GIT_HASH: &str = git_version::git_version!(
    // NetCore services are usually built from local, patched trees. Keep the visible
    // runtime version stable and commit-based instead of showing "-modified" in the
    // dashboard for every local operator patch. OTA still compares this abbreviated
    // hash with the repository HEAD.
    args = ["--always", "--match=", "--abbrev=8"],
    fallback = "unknown"
);

/// Product/branding used by the NetCore dashboard and OTA output.
// Was: Legt den festen Wert `STACK_NAME` für stack name fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const STACK_NAME: &str = "NetCore-Tetra";
// Was: Legt den festen Wert `STACK_CODENAME` für stack codename fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const STACK_CODENAME: &str = "Dual Carrier";

/// Full stack version string, e.g. "v1.3.0-2aad62c8".
// Was: Legt den festen Wert `STACK_VERSION` für stack version fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const STACK_VERSION: &str = const_format::formatcp!("v{}-{}", env!("CARGO_PKG_VERSION"), GIT_HASH);

/// Human-friendly product + version string.
// Was: Legt den festen Wert `STACK_DISPLAY` für stack display fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const STACK_DISPLAY: &str = const_format::formatcp!("{} {}", STACK_NAME, STACK_VERSION);

// Was: Bindet das Untermodul address in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod address;
// Was: Bindet das Untermodul bitbuffer in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod bitbuffer;
// Was: Bindet das Untermodul debug in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod debug;
// Was: Bindet das Untermodul direction in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod direction;
// Was: Bindet das Untermodul freqs in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod freqs;
// Was: Bindet das Untermodul Protokollnachricht (PDU) parse error in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod pdu_parse_error;
// Was: Bindet das Untermodul phy types in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod phy_types;
// Was: Bindet das Untermodul ranges in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod ranges;
// Was: Bindet das Untermodul sap fields in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sap_fields;
// Was: Bindet das Untermodul tdma time in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod tdma_time;
// Was: Bindet das Untermodul TETRA common in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod tetra_common;
// Was: Bindet das Untermodul TETRA entities in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod tetra_entities;
// Was: Bindet das Untermodul timeslot alloc in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod timeslot_alloc;
// Was: Bindet das Untermodul tx receipt in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod tx_receipt;
// Was: Bindet das Untermodul typed Protokollnachricht (PDU) fields in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod typed_pdu_fields;

// Re-export commonly used items
pub use address::*;
pub use bitbuffer::BitBuffer;
pub use direction::Direction;
pub use pdu_parse_error::PduParseErr;
pub use phy_types::*;
pub use sap_fields::*;
pub use tdma_time::TdmaTime;
pub use tetra_common::*;
pub use timeslot_alloc::*;
pub use tx_receipt::*;
