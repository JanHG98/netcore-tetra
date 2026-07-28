// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Local TETRA speech recorder.
//!
//! The entity receives call/floor lifecycle metadata from CMCE and a passive copy of
//! valid uplink TMD speech blocks from UMAC. Recordings are stored as 8-kHz mono 16-bit
//! PCM WAV files with JSON metadata sidecars.

// Was: Bindet das Untermodul archive in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod archive;
// Was: Bindet das Untermodul entity in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod entity;
// Was: Bindet das Untermodul Dienst in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod service;
// Was: Bindet das Untermodul types in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod types;
// Was: Bindet das Untermodul wav in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod wav;

pub use entity::RecorderEntity;
pub use service::RecorderHandle;
pub use types::{RecorderStatus, RecordingMetadata, RecordingSegment};
