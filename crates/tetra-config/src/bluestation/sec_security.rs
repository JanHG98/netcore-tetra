// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Einlesen und Prüfen der TETRA-Konfiguration.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use serde::Deserialize;

/// Access control / security configuration
#[derive(Debug, Clone, Default)]
// Was: Bündelt die zusammengehörigen Werte für cfg Sicherheit in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgSecurity {
    /// ISSI whitelist. If non-empty, only these ISSIs are allowed to register.
    /// An empty list means all ISSIs are accepted (open network).
    /// Example config:
    ///   [security]
    ///   issi_whitelist = [2260571, 1001, 1002]
    pub issi_whitelist: Vec<u32>,
}

// Was: Implementiert das zugehörige Verhalten für `CfgSecurity`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl CfgSecurity {
    /// Returns true if the given ISSI is allowed to register.
    /// When the whitelist is empty, all ISSIs are allowed.
    // Was: Prüft, ob Teilnehmerkennung (ISSI) allowed zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_issi_allowed(&self, issi: u32) -> bool {
        if self.issi_whitelist.is_empty() {
            return true;
        }
        self.issi_whitelist.contains(&issi)
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für cfg Sicherheit dto in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgSecurityDto {
    #[serde(default)]
    pub issi_whitelist: Vec<u32>,
}

// Was: Diese Funktion wendet Sicherheit patch.
// Warum: Die Änderung wird dadurch nur über einen definierten und prüfbaren Weg wirksam.
pub fn apply_security_patch(dto: CfgSecurityDto) -> CfgSecurity {
    CfgSecurity {
        issi_whitelist: dto.issi_whitelist,
    }
}
