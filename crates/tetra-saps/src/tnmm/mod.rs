// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Nachrichten an den Schnittstellen zwischen TETRA-Protokollschichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

// Placeholder for testing
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tnmm test demand in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TnmmTestDemand {
    pub issi: u32,
}
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tnmm test response in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TnmmTestResponse {
    pub issi: u32,
    pub data: u32,
}
