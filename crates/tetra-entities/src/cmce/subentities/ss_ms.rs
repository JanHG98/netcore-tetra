// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use tetra_saps::SapMsg;

use crate::MessageQueue;

/// Clause 12 Supplementary Services CMCE sub-entity
// Was: Bündelt die zusammengehörigen Werte für ss ms subentity in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct SsMsSubentity {}

// Was: Implementiert das zugehörige Verhalten für `SsMsSubentity`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl SsMsSubentity {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new() -> Self {
        SsMsSubentity {}
    }

    // Was: Diese Funktion leitet re deliver.
    // Warum: Nachrichten und Daten gelangen dadurch nachvollziehbar an das richtige Ziel.
    pub fn route_re_deliver(&mut self, _queue: &mut MessageQueue, mut _message: SapMsg) {
        tracing::trace!("route_re_deliver");

        // Handle the incoming unit data indication
        unimplemented!();
    }
}
