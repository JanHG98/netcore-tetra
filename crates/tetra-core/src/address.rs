// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für grundlegende TETRA-Datentypen und Hilfsfunktionen.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

#[allow(dead_code)]
#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
// Was: Listet die möglichen Varianten für TETRA-Teilnehmerkennung (SSI) type auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum SsiType {
    Unknown,
    /// Generic type when specific type unknown. Avoid using where possible.
    Ssi,
    /// Individual Short Subscriber Identity
    Issi,
    /// Group Short Subscriber Identity
    Gssi,
    Ussi,
    Smi,

    /// Any type of encrypted SSI
    Esi,

    /// Only usable in Umac, needs to be replaced with true SSI
    EventLabel,
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for SsiType`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for SsiType {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            SsiType::Unknown => write!(f, "Unknown"),
            SsiType::Ssi => write!(f, "SSI"),
            SsiType::Issi => write!(f, "ISSI"),
            SsiType::Gssi => write!(f, "GSSI"),
            SsiType::Ussi => write!(f, "USSI"),
            SsiType::Smi => write!(f, "SMI"),
            SsiType::Esi => write!(f, "ESI"),
            SsiType::EventLabel => write!(f, "EventLabel"),
        }
    }
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
// Was: Bündelt die zusammengehörigen Werte für TETRA address in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TetraAddress {
    pub ssi: u32,
    pub ssi_type: SsiType,
}

// Was: Implementiert das zugehörige Verhalten für `TetraAddress`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl TetraAddress {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(ssi: u32, ssi_type: SsiType) -> Self {
        Self { ssi, ssi_type }
    }

    /// Convenience constructor to create ISSI type address
    // Was: Führt den Arbeitsschritt `issi` für Teilnehmerkennung (ISSI) aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn issi(ssi: u32) -> Self {
        Self::new(ssi, SsiType::Issi)
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for TetraAddress`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for TetraAddress {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}", self.ssi_type, self.ssi)
    }
}
