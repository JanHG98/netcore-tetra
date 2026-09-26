// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Shared TETRA speech-codec helpers used by the SIP bridge, recorder and future media player.

// Was: Bindet das Untermodul codec in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod codec;

pub use codec::{
    TETRA_CODED_BITS_PER_FRAME, TETRA_PCM_SAMPLE_RATE, TETRA_PCM_SAMPLES_PER_BLOCK, TETRA_PCM_SAMPLES_PER_FRAME,
    TetraSpeechCodec, TetraSpeechDecoder, TetraSpeechEncoder,
};
