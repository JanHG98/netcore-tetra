// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Common things used by both modulator and demodulator.

use super::dsp_types::*;

/// RRC channel filter taps, designed using design_channel_filter.py
// Was: Legt den festen Wert `CHANNEL_FILTER_TAPS` für Kanal filter taps fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const CHANNEL_FILTER_TAPS: [RealSample; 16] = [
    0.264_971_8,
    0.20002119,
    0.10064187,
    0.00998249,
    -0.04014123,
    -0.04405674,
    -0.01982716,
    0.00642452,
    0.01744363,
    0.01213436,
    0.00071221,
    -0.00609533,
    -0.00488494,
    0.00028619,
    0.00345407,
    0.00220812,
];
