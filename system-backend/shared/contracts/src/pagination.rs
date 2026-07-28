// NETCORE-KOMMENTAR – Was: Enthält die Logik oder Einstellungen für pagination.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für page request in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct PageRequest {
    pub offset: usize,
    pub limit: usize,
}

// Was: Implementiert das zugehörige Verhalten für `PageRequest`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl PageRequest {
    // Was: Legt den festen Wert `DEFAULT_LIMIT` für default limit fest.
    // Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
    pub const DEFAULT_LIMIT: usize = 100;
    // Was: Legt den festen Wert `MAX_LIMIT` für max limit fest.
    // Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
    pub const MAX_LIMIT: usize = 1000;

    // Was: Führt den Arbeitsschritt `normalized` für normalized aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn normalized(self) -> Self {
        Self {
            offset: self.offset,
            limit: self.limit.clamp(1, Self::MAX_LIMIT),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `Default for PageRequest`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for PageRequest {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self { offset: 0, limit: Self::DEFAULT_LIMIT }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für page in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct Page<T> {
    pub items: Vec<T>,
    pub offset: usize,
    pub limit: usize,
    pub total: usize,
    pub has_more: bool,
}
