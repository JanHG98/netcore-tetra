// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Local WAV/MP3 dispatch into TETRA group and individual calls.
//!
//! Audio is fully decoded and encoded before CMCE resources are requested. The RF core
//! therefore never waits on disk I/O or ffmpeg while a traffic channel is active.

// Was: Bindet das Untermodul entity in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod entity;
// Was: Bindet das Untermodul Audio- und Mediendaten in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod media;
// Was: Bindet das Untermodul Dienst in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod service;
// Was: Bindet das Untermodul types in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod types;

pub use entity::AudioPlayerEntity;
pub(crate) use media::materialize_recording_wav;
pub use service::AudioPlayerHandle;
pub use types::{AudioPlayerState, AudioPlayerStatus, AudioSourceType, AudioTargetType, MediaEntry, MediaSourceInfo};
