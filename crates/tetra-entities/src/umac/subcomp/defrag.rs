// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use tetra_core::{BitBuffer, SsiType, TdmaTime, TetraAddress, Todo};

// Was: Legt den festen Wert `DEFRAG_BUF_INITIAL_LEN` für defrag buf initial len fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const DEFRAG_BUF_INITIAL_LEN: usize = 512;

#[derive(Debug, PartialEq)]
// Was: Listet die möglichen Varianten für defrag buffer Zustand auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum DefragBufferState {
    Inactive,
    Active,
    Complete,
}

// Was: Bündelt die zusammengehörigen Werte für defrag buffer in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct DefragBuffer {
    pub state: DefragBufferState,
    pub addr: TetraAddress,
    pub t_first: TdmaTime,
    pub t_last: TdmaTime,
    pub num_frags: usize,
    pub aie_info: Option<Todo>,
    pub buffer: BitBuffer,
}

// Was: Implementiert das zugehörige Verhalten für `DefragBuffer`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl DefragBuffer {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new() -> Self {
        Self {
            state: DefragBufferState::Inactive,
            addr: TetraAddress {
                ssi: 0,
                ssi_type: SsiType::Issi,
            },
            t_first: TdmaTime::default(),
            t_last: TdmaTime::default(),
            num_frags: 0,
            aie_info: None,
            buffer: BitBuffer::new_autoexpand(DEFRAG_BUF_INITIAL_LEN),
        }
    }

    // Was: Diese Funktion setzt den vorgesehenen Arbeitsschritt.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn reset(&mut self) {
        self.state = DefragBufferState::Inactive;
        self.addr = TetraAddress {
            ssi: 0,
            ssi_type: SsiType::Issi,
        };
        self.t_first = TdmaTime::default();
        self.t_last = TdmaTime::default();
        self.num_frags = 0;
        self.aie_info = None;
        self.buffer = BitBuffer::new_autoexpand(DEFRAG_BUF_INITIAL_LEN);
    }
}
