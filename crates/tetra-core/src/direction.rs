// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für grundlegende TETRA-Datentypen und Hilfsfunktionen.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

#[derive(Debug, Clone, PartialEq, Copy)]
// Was: Listet die möglichen Varianten für direction auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum Direction {
    None,
    /// Uplink
    Ul,
    /// Downlink
    Dl,
    Both,
}

// Was: Implementiert das zugehörige Verhalten für `Direction`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Direction {
    #[inline]
    // Was: Führt den Arbeitsschritt `includes_ul` für includes ul aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn includes_ul(&self) -> bool {
        matches!(self, Direction::Ul | Direction::Both)
    }

    #[inline]
    // Was: Führt den Arbeitsschritt `includes_dl` für includes dl aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn includes_dl(&self) -> bool {
        matches!(self, Direction::Dl | Direction::Both)
    }
}
