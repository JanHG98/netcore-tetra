// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use tetra_core::tetra_entities::TetraEntity;
use tetra_entities::{MessageQueue, TetraEntityTrait};
use tetra_saps::sapmsg::SapMsg;

/// A TETRA component sink for testing purposes
/// Collects all received SapMsg messages for later inspection
// Was: Bündelt die zusammengehörigen Werte für sink in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct Sink {
    component: TetraEntity,
    msgqueue: Vec<SapMsg>,
}

// Was: Implementiert das zugehörige Verhalten für `Sink`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Sink {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(component: TetraEntity) -> Self {
        Self {
            component,
            msgqueue: vec![],
        }
    }

    // Was: Führt den Arbeitsschritt `take_msgqueue` für take msgqueue aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn take_msgqueue(&mut self) -> Vec<SapMsg> {
        std::mem::take(&mut self.msgqueue)
    }
}

// Was: Implementiert das zugehörige Verhalten für `TetraEntityTrait for Sink`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl TetraEntityTrait for Sink {
    // Was: Führt den Arbeitsschritt `entity` für entity aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn entity(&self) -> TetraEntity {
        self.component
    }

    // Was: Führt den Arbeitsschritt `rx_prim` für rx prim aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn rx_prim(&mut self, _queue: &mut MessageQueue, message: SapMsg) {
        tracing::debug!("rx_prim: {:?}", message);
        // tracing::debug!(ts=%message.dltime, "rx_prim: {:?}", message);

        self.msgqueue.push(message);
    }
}
