// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::VecDeque;

use tetra_core::Direction;
use tetra_saps::control::call_control::Circuit;

// Was: Bündelt die zusammengehörigen Werte für circuit mgr in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CircuitMgr {
    pub dl: [Option<Circuit>; 4],
    pub ul: [Option<Circuit>; 4],

    /// Data blocks queued to be transmitted, per timeslot
    pub tx_data: [VecDeque<Vec<u8>>; 4],
}

// Was: Implementiert das zugehörige Verhalten für `CircuitMgr`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl CircuitMgr {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new() -> Self {
        Self {
            dl: [None, None, None, None],
            ul: [None, None, None, None],
            tx_data: [VecDeque::new(), VecDeque::new(), VecDeque::new(), VecDeque::new()],
        }
    }

    // Was: Führt den Arbeitsschritt `ts_index` für ts index aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn ts_index(ts: u8) -> Option<usize> {
        if (1..=4).contains(&ts) {
            Some(ts as usize - 1)
        } else {
            None
        }
    }

    // Was: Prüft, ob active zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_active(&self, dir: Direction, ts: u8) -> bool {
        let Some(idx) = Self::ts_index(ts) else {
            tracing::warn!(
                "UMAC CircuitMgr: invalid physical timeslot {} for is_active({:?}); ignoring",
                ts,
                dir
            );
            return false;
        };

        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match dir {
            Direction::Dl => self.dl[idx].is_some(),
            Direction::Ul => self.ul[idx].is_some(),
            _ => {
                tracing::error!("UMAC CircuitMgr: called with non-specific direction {:?}", dir);
                false
            }
        }
    }

    // Was: Diese Funktion liest usage.
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    pub fn get_usage(&self, dir: Direction, ts: u8) -> Option<u8> {
        let Some(idx) = Self::ts_index(ts) else {
            tracing::warn!(
                "UMAC CircuitMgr: invalid physical timeslot {} for get_usage({:?}); ignoring",
                ts,
                dir
            );
            return None;
        };

        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match dir {
            Direction::Dl => self.dl[idx].as_ref().map(|circuit| circuit.usage),
            Direction::Ul => self.ul[idx].as_ref().map(|circuit| circuit.usage),
            _ => {
                tracing::error!("UMAC CircuitMgr: called with non-specific direction {:?}", dir);
                None
            }
        }
    }

    /// Closes an active circuit, and return the Circuit to the caller
    // Was: Diese Funktion schließt circuit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn close_circuit(&mut self, dir: Direction, ts: u8) -> Option<Circuit> {
        let Some(idx) = Self::ts_index(ts) else {
            tracing::warn!(
                "UMAC CircuitMgr: invalid physical timeslot {} for close_circuit({:?}); ignoring",
                ts,
                dir
            );
            return None;
        };

        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match dir {
            Direction::Dl => {
                self.tx_data[idx].clear();
                self.dl[idx].take()
            }
            Direction::Ul => self.ul[idx].take(),
            _ => {
                tracing::error!("UMAC CircuitMgr: called with non-specific direction {:?}", dir);
                None
            }
        }
    }

    /// Creates a new circuit on the given direction and timeslot.
    ///
    /// The UMAC scheduler is per carrier. Therefore this low-level manager only accepts
    /// physical air-interface timeslots 1..=4. Higher layers may use logical TS5..TS7
    /// for secondary-carrier traffic, but those must be mapped back to physical TS2..TS4
    /// before reaching this component.
    // Was: Diese Funktion erstellt circuit.
    // Warum: Neue Objekte erhalten so immer einen vollständigen und gültigen Ausgangszustand.
    pub fn create_circuit(&mut self, dir: Direction, circuit: Circuit) {
        let ts = circuit.ts;
        let Some(idx) = Self::ts_index(ts) else {
            tracing::warn!(
                "UMAC CircuitMgr: refusing to create {:?} circuit on invalid physical timeslot {}",
                dir,
                ts
            );
            return;
        };

        // Sanity check
        if self.is_active(dir, ts) {
            tracing::warn!("CircuitMgr::create had still active circuit on {:?} {}", dir, ts);
            self.close_circuit(dir, ts);
        }

        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match dir {
            Direction::Dl => {
                if !self.tx_data[idx].is_empty() {
                    tracing::warn!("CircuitMgr::create had pending tx_data on Dl {}", ts);
                    self.tx_data[idx].clear();
                }
                self.dl[idx] = Some(circuit);
            }
            Direction::Ul => self.ul[idx] = Some(circuit),
            _ => {
                tracing::error!("UMAC CircuitMgr: called with non-specific direction {:?}", dir);
            }
        }
    }

    /// Put a block in the queue for transmission on an associated channel
    // Was: Führt den Arbeitsschritt `put_block` für put block aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn put_block(&mut self, ts: u8, block: Vec<u8>) {
        let Some(idx) = Self::ts_index(ts) else {
            tracing::warn!(
                "UMAC CircuitMgr: refusing put_block on invalid physical timeslot {}",
                ts
            );
            return;
        };

        if !self.is_active(Direction::Dl, ts) {
            tracing::warn!("CircuitMgr::put_block on inactive circuit {:?} {}", Direction::Dl, ts);
            return;
        }
        self.tx_data[idx].push_back(block);
    }

    /// Take a to-be-transmitted block from the queue
    // Was: Führt den Arbeitsschritt `take_block` für take block aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn take_block(&mut self, ts: u8) -> Option<Vec<u8>> {
        let Some(idx) = Self::ts_index(ts) else {
            tracing::warn!(
                "UMAC CircuitMgr: refusing take_block on invalid physical timeslot {}",
                ts
            );
            return None;
        };

        if !self.is_active(Direction::Dl, ts) {
            tracing::warn!("CircuitMgr::take_block on inactive circuit {:?} {}", Direction::Dl, ts);
            return None;
        }
        self.tx_data[idx].pop_front()
    }
}
