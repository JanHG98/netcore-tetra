// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Nachrichten an den Schnittstellen zwischen TETRA-Protokollschichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use crate::control::enums::sds_user_data::SdsUserData;

/// SDS data routing between CMCE SDS subentity and Brew entity
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für CMCE-Rufsteuerung TETRA-Kurznachricht (SDS) data in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CmceSdsData {
    /// Source ISSI (calling party)
    pub source_issi: u32,
    /// Destination ISSI (called party)
    pub dest_issi: u32,
    /// User-defined data (type1, type2, type3, or type4)
    pub user_defined_data: SdsUserData,
}
