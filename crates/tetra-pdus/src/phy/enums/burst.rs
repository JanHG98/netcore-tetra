// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Re-export PHY types from tetra-core for backward compatibility
//!
//! These types are defined in tetra-core because they're used across multiple
//! layers (PHY, LMAC, UMAC) and in SAP primitives.

pub use tetra_core::{BurstType, PhyBlockNum, PhyBlockType, TrainingSequence};
