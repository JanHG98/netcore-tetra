// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 21.4.7.2 ACCESS-ASSIGN
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für access assign dl usage auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum AccessAssignDlUsage {
    Unallocated,
    AssignedControl,
    CommonControl,
    CommonAndAssigned,
    Traffic(u8),
}

// Was: Implementiert das zugehörige Verhalten für `AccessAssignDlUsage`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl AccessAssignDlUsage {
    // Was: Wandelt Eingangsdaten in usage marker um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_usage_marker(field: u8) -> Self {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match field {
            0 => AccessAssignDlUsage::Unallocated,
            1 => AccessAssignDlUsage::AssignedControl,
            2 => AccessAssignDlUsage::CommonControl,
            3 => AccessAssignDlUsage::CommonAndAssigned,
            _ => AccessAssignDlUsage::Traffic(field),
        }
    }

    // Was: Wandelt den vorhandenen Wert in usage marker um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_usage_marker(&self) -> u8 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            AccessAssignDlUsage::Unallocated => 0,
            AccessAssignDlUsage::AssignedControl => 1,
            AccessAssignDlUsage::CommonControl => 2,
            AccessAssignDlUsage::CommonAndAssigned => 3,
            AccessAssignDlUsage::Traffic(chan) => *chan,
        }
    }

    // Was: Prüft, ob Nutzdatenverkehr zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_traffic(&self) -> bool {
        matches!(self, AccessAssignDlUsage::Traffic(_))
    }

    // Was: Diese Funktion liest tchan.
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    pub fn get_tchan(&self) -> Option<u8> {
        if let AccessAssignDlUsage::Traffic(chan) = self {
            Some(*chan)
        } else {
            None
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for AccessAssignDlUsage`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for AccessAssignDlUsage {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            AccessAssignDlUsage::Unallocated => write!(f, "Unallocated"),
            AccessAssignDlUsage::AssignedControl => write!(f, "AssignedControl"),
            AccessAssignDlUsage::CommonControl => write!(f, "CommonControl"),
            AccessAssignDlUsage::CommonAndAssigned => write!(f, "CommonAndAssigned"),
            AccessAssignDlUsage::Traffic(chan) => write!(f, "Traffic({})", chan),
        }
    }
}
