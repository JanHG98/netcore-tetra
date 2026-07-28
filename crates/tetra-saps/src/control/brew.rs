// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Nachrichten an den Schnittstellen zwischen TETRA-Protokollschichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für Brew-Verbindung Teilnehmer action auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum BrewSubscriberAction {
    Register,
    Deregister,
    Affiliate,
    Deaffiliate,
}

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für Mobilitätsverwaltung Teilnehmer update in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MmSubscriberUpdate {
    pub issi: u32,
    pub groups: Vec<u32>,
    pub action: BrewSubscriberAction,
}
