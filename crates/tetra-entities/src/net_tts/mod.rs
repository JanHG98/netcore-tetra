// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Local Piper HTTP text-to-speech generation for the recording library.
//!
//! Piper always generates a complete canonical recording-format WAV first. The
//! finished file is imported into the local recorder with JSON metadata and can
//! only be transmitted later through the ordinary recording selection workflow.


// Was: Bindet das Untermodul Dienst in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod service;
// Was: Bindet das Untermodul templates in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod templates;
// Was: Bindet das Untermodul types in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod types;

pub use service::TtsHandle;
pub use templates::{TtsTemplate, TtsTemplateDraft};
pub use types::{TtsState, TtsStatus, TtsVoiceStatus};
