// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::HashMap;

use tetra_core::TetraAddress;
use tetra_pdus::umac::fields::EventLabel;

// Was: Bündelt die zusammengehörigen Werte für Ereignis label mapping in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct EventLabelMapping {
    // pub valid_until: TdmaTime,
    pub addr: TetraAddress,
    pub label: EventLabel,
}

// Was: Bündelt die zusammengehörigen Werte für Ereignis label store in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct EventLabelStore {
    labels: HashMap<EventLabel, EventLabelMapping>,
    next_label: EventLabel,
}

// Was: Implementiert das zugehörige Verhalten für `EventLabelStore`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl EventLabelStore {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new() -> Self {
        Self {
            labels: HashMap::new(),
            next_label: 1,
        }
    }

    /// Get the next free event label. Event labels are allocated linearly, and we assume the next one to be
    /// free. Upon rollover, we assume old labels to have been dropped by now. If not, we'll crash when inserting
    /// a label into the labels hashmap.
    // Was: Diese Funktion liest free label.
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    pub fn get_free_label(&mut self) -> EventLabel {
        let ret = self.next_label;
        self.next_label = (self.next_label + 1) % 0x1FF;
        ret
    }

    /// Create an event label for a TetraAddress. There should not yet exist a label for this address, or we
    /// crash. Returns the generated event label.
    // Was: Diese Funktion erstellt label for addr.
    // Warum: Neue Objekte erhalten so immer einen vollständigen und gültigen Ausgangszustand.
    fn create_label_for_addr(&mut self, addr: TetraAddress) -> EventLabel {
        assert!(self.get_label_by_ssi(addr.ssi).is_none(), "an event label for SSI already exists");

        let label = self.get_free_label();
        let entry = EventLabelMapping { addr, label };
        self.labels.insert(label, entry);

        label
    }

    /// Retrieve an address by its label. The returned address may be encrypted if
    /// the unencrypted variant was not known at the time of label creation
    // Was: Diese Funktion liest addr by label.
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    pub fn get_addr_by_label(&self, label: EventLabel) -> Option<TetraAddress> {
        self.labels.get(&label).map(|event_label| event_label.addr)
    }

    /// Find if a label is associated with some SSI.
    // Was: Diese Funktion liest label by TETRA-Teilnehmerkennung (SSI).
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    pub fn get_label_by_ssi(&self, ssi: u32) -> Option<EventLabel> {
        self.labels
            .values()
            .find(|event_label| event_label.addr.ssi == ssi)
            .map(|event_label| event_label.label)
    }

    // pub fn remove_label(&mut self, label: EventLabel) -> Option<EventLabel> {
    //     self.labels.remove(&label)
    // }

    // pub fn contains_label(&self, label: EventLabel) -> bool {
    //     self.labels.contains_key(&label)
    // }

    // pub fn len(&self) -> usize {
    //     self.labels.len()
    // }

    // pub fn is_empty(&self) -> bool {
    //     self.labels.is_empty()
    // }
}
