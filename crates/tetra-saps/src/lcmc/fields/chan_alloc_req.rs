// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Nachrichten an den Schnittstellen zwischen TETRA-Protokollschichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use tetra_core::Todo;

use crate::lcmc::enums::{alloc_type::ChanAllocType, ul_dl_assignment::UlDlAssignment};

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für CMCE-Rufsteuerung chan alloc req in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CmceChanAllocReq {
    /// Set for new allocation, None for QuitAndGo
    pub usage: Option<u8>,
    /// Carrier frequency; by default, uses self
    pub carrier: Option<Todo>,
    /// Bitmap of slots to use.
    pub timeslots: [bool; 4],
    /// Alloc type.
    /// Additional: new allocation.
    /// Replace: update existing allocation, or create if it does not exist.
    /// QuitAndGo: remove existing allocation.
    pub alloc_type: ChanAllocType,
    pub ul_dl_assigned: UlDlAssignment,
}
