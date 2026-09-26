// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use crate::MessageQueue;
use as_any::AsAny;
use tetra_config::bluestation::SharedConfig;
use tetra_core::{TdmaTime, tetra_entities::TetraEntity};
use tetra_saps::SapMsg;

/// Trait for TETRA entities
/// Used by MessageRouter for passing messages between entities
// Was: Beschreibt das gemeinsame Verhalten für TETRA entity trait.
// Warum: Unterschiedliche Implementierungen können dadurch über dieselbe verständliche Schnittstelle benutzt werden.
pub trait TetraEntityTrait: Send + AsAny {
    /// Returns the entity type identifier
    // Was: Führt den Arbeitsschritt `entity` für entity aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn entity(&self) -> TetraEntity;

    /// Handle incoming SAP primitive
    // Was: Führt den Arbeitsschritt `rx_prim` für rx prim aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn rx_prim(&mut self, queue: &mut MessageQueue, message: SapMsg);

    /// Update configuration (optional)
    #[allow(dead_code)]
    // Was: Diese Funktion setzt Konfiguration.
    // Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
    fn set_config(&mut self, _config: SharedConfig) {}

    /// Called at the start of each TDMA tick
    // Was: Diese Funktion bearbeitet start.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn tick_start(&mut self, _queue: &mut MessageQueue, _ts: TdmaTime) {}

    /// Called at the end of each TDMA tick
    // Was: Diese Funktion bearbeitet end.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn tick_end(&mut self, _queue: &mut MessageQueue, _ts: TdmaTime) -> bool {
        false
    }
}
