// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

/// Clause 21.4.7.2 ACCESS-ASSIGN

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
// Was: Listet die möglichen Varianten für access assign ul usage auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum AccessAssignUlUsage {
    CommonOnly,
    CommonAndAssigned,
    AssignedOnly,
    Unallocated,
    Traffic(u8),
}

// Was: Implementiert das zugehörige Verhalten für `AccessAssignUlUsage`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl AccessAssignUlUsage {
    // Was: Wandelt Eingangsdaten in usage marker um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_usage_marker(field: u8) -> Option<Self> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match field {
            0 => Some(AccessAssignUlUsage::Unallocated),
            _ => {
                if field < 4 {
                    tracing::warn!("Invalid usage marker for UL: {}", field);
                    None
                } else {
                    Some(AccessAssignUlUsage::Traffic(field))
                }
            }
        }
    }

    // Was: Wandelt den vorhandenen Wert in usage marker um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn to_usage_marker(&self) -> Option<u8> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            AccessAssignUlUsage::Unallocated => Some(0),
            AccessAssignUlUsage::Traffic(chan) => Some(*chan),
            _ => None,
        }
    }

    // Was: Prüft, ob Nutzdatenverkehr zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_traffic(&self) -> bool {
        matches!(self, AccessAssignUlUsage::Traffic(_))
    }

    // Was: Diese Funktion liest tchan.
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    pub fn get_tchan(&self) -> Option<u8> {
        if let AccessAssignUlUsage::Traffic(chan) = self {
            Some(*chan)
        } else {
            None
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `core::fmt::Display for AccessAssignUlUsage`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl core::fmt::Display for AccessAssignUlUsage {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            AccessAssignUlUsage::CommonOnly => write!(f, "CommonOnly"),
            AccessAssignUlUsage::CommonAndAssigned => write!(f, "CommonAndAssigned"),
            AccessAssignUlUsage::AssignedOnly => write!(f, "AssignedOnly"),
            AccessAssignUlUsage::Traffic(chan) => write!(f, "Traffic({})", chan),
            AccessAssignUlUsage::Unallocated => write!(f, "Unallocated"),
        }
    }
}
