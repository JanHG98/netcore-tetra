// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für grundlegende TETRA-Datentypen und Hilfsfunktionen.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

/// The three states a transmit receipt can be in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für tx Zustand auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum TxState {
    /// Message is queued but not yet sent over the air.
    Pending = 0,
    /// MAC layer had to discard this message
    Discarded = 1,
    /// MAC layer has sent the PDU over the air.
    Transmitted = 2,
    /// Message was transmitted but acknowledgement never came
    Lost = 3,
    /// The remote side has acknowledged receipt.
    Acknowledged = 4,
}

// Was: Implementiert das zugehörige Verhalten für `TxState`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl TxState {
    // Was: Wandelt Eingangsdaten in raw um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from_raw(v: u8) -> Self {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match v {
            0 => Self::Pending,
            1 => Self::Discarded,
            2 => Self::Transmitted,
            3 => Self::Lost,
            _ => Self::Acknowledged,
        }
    }
}

/// A transmit receipt kept by the originator (e.g. CMCE) to query whether the
/// message was sent and/or acknowledged.
///
/// State machine (transitions driven by the paired [`TxSignal`]):
///
/// ```text
/// Pending -> Transmitted | Discarded
///   Transmitted: MAC has sent the PDU over the air.
///   Discarded:   MAC was too busy. Final state.
///
/// expects_ack == true:
///   Transmitted -> Acknowledged | Lost
///     Acknowledged: LLC received ACK from remote. Final state.
///     Lost:         LLC timed out waiting for ACK. Final state.
///
/// expects_ack == false:
///   Transmitted is the final state.
/// ```

/// The reporting half of a transmit receipt, carried alongside the PDU down
/// through MAC and LLC. These layers call the `mark_*` methods to drive state
/// transitions that the paired [`TxReceipt`] can observe.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für tx reporter in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TxReporter {
    expects_ack: bool,
    state: Arc<AtomicU8>,
    // t_tx: Option<TdmaTime>,
    // t_ack: Option<TdmaTime>
}

// Was: Implementiert das zugehörige Verhalten für `TxReporter`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl TxReporter {
    /// Creates a clonable TxReporter for acknowledged service. All clones share the same internal state.
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new() -> Self {
        let state = Arc::new(AtomicU8::new(TxState::Pending as u8));
        Self { expects_ack: true, state }
    }

    /// Creates a clonable TxReporter for unacknowledged service. All clones share the same internal state.
    // Was: Diese Funktion erstellt unacked.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new_unacked() -> Self {
        let mut ret = Self::new();
        ret.expects_ack = false;
        ret
    }

    /// Returns the current state.
    // Was: Diese Funktion liest Zustand.
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    pub fn get_state(&self) -> TxState {
        TxState::from_raw(self.state.load(Ordering::Relaxed))
    }

    /// True if the PDU was discarded by the Umac due to congestion
    // Was: Prüft, ob discarded zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_discarded(&self) -> bool {
        self.state.load(Ordering::Relaxed) == TxState::Discarded as u8
    }

    /// True once the PDU has been sent over the air (or further).
    // Was: Prüft, ob transmitted zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_transmitted(&self) -> bool {
        self.state.load(Ordering::Relaxed) >= TxState::Transmitted as u8
    }

    /// True once the remote side has acknowledged receipt.
    // Was: Prüft, ob acknowledged zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_acknowledged(&self) -> bool {
        self.state.load(Ordering::Relaxed) >= TxState::Acknowledged as u8
    }

    /// Returns true if this is the final state for this message.
    // Was: Prüft, ob in final Zustand zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_in_final_state(&self) -> bool {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self.get_state() {
            TxState::Pending => false,
            TxState::Discarded => true,
            TxState::Transmitted => !self.expects_ack,
            TxState::Lost => true,
            TxState::Acknowledged => true,
        }
    }

    // Was: Diese Funktion kennzeichnet den vorgesehenen Arbeitsschritt.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn mark(&self, curr_state: TxState, new_state: TxState) {
        // tracing::info!("TxReporter: marking {:?} -> {:?}", curr_state, new_state);
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self
            .state
            .compare_exchange(curr_state as u8, new_state as u8, Ordering::Relaxed, Ordering::Relaxed)
        {
            Ok(_) => {}
            Err(_) => {
                panic!(
                    "TxReporter: invalid transition {:?} -> {:?} (actual state: {:?})",
                    curr_state,
                    new_state,
                    self.get_state()
                );
            }
        }
    }

    // Was: Diese Funktion kennzeichnet unchecked.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn mark_unchecked(&self, new_state: TxState) {
        self.state.store(new_state as u8, Ordering::Relaxed);
    }

    /// Pending → Transmitted: MAC layer has sent the PDU over the air.
    // Was: Diese Funktion kennzeichnet transmitted.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn mark_transmitted(&self) {
        self.mark(TxState::Pending, TxState::Transmitted);
    }

    /// Pending → Discarded: MAC layer was too busy to transmit.
    // Was: Diese Funktion kennzeichnet discarded.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn mark_discarded(&self) {
        self.mark(TxState::Pending, TxState::Discarded);
    }

    /// Transmitted → Acknowledged: LLC received an ACK from the remote side.
    // Was: Diese Funktion kennzeichnet acknowledged.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn mark_acknowledged(&self) {
        assert!(
            self.expects_ack,
            "TxReporter: cannot mark as acknowledged a message that does not expect an ACK"
        );
        self.mark(TxState::Transmitted, TxState::Acknowledged);
    }

    /// Transmitted → Lost: LLC did not receive an ACK within the expected time window.
    // Was: Diese Funktion kennzeichnet lost.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn mark_lost(&self) {
        assert!(
            self.expects_ack,
            "TxReporter: cannot mark as lost a message that does not expect an ACK"
        );
        self.mark(TxState::Transmitted, TxState::Lost);
    }

    /// Tricky function to re-use linked TxReporters. Resets state to the initial state.
    /// Be very careful when using this.
    // Was: Diese Funktion setzt den vorgesehenen Arbeitsschritt.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn reset(&self) {
        self.mark_unchecked(TxState::Pending);
    }
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Führt den Arbeitsschritt `receipt_observes_signal_transitions` für receipt observes signal transitions aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn receipt_observes_signal_transitions() {
        let receipt = TxReporter::new();
        let reporter = receipt.clone();
        assert_eq!(reporter.get_state(), TxState::Pending);
        reporter.mark_transmitted();
        assert_eq!(receipt.get_state(), TxState::Transmitted);
        reporter.mark_acknowledged();
        assert_eq!(receipt.get_state(), TxState::Acknowledged);
    }

    #[test]
    // Was: Führt den Arbeitsschritt `cloned_signal_shares_state` für cloned signal shares Zustand aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn cloned_signal_shares_state() {
        let receipt = TxReporter::new();
        let reporter = receipt.clone();
        let reporter2 = reporter.clone();
        reporter2.mark_transmitted();
        assert_eq!(receipt.get_state(), TxState::Transmitted);
        assert_eq!(reporter.get_state(), TxState::Transmitted);
    }

    #[test]
    #[should_panic(expected = "invalid transition")]
    // Was: Führt den Arbeitsschritt `double_mark_transmitted_panics` für double mark transmitted panics aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn double_mark_transmitted_panics() {
        let receipt = TxReporter::new();
        let reporter = receipt.clone();
        reporter.mark_transmitted();
        reporter.mark_transmitted();
    }

    #[test]
    #[should_panic(expected = "cannot mark as acknowledged")]
    // Was: Führt den Arbeitsschritt `unacked_mark_acked_panics` für unacked mark acked panics aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn unacked_mark_acked_panics() {
        let receipt = TxReporter::new_unacked();
        let reporter = receipt.clone();
        reporter.mark_transmitted();
        reporter.mark_acknowledged();
    }

    #[test]
    #[should_panic(expected = "invalid transition")]
    // Was: Diese Funktion kennzeichnet acknowledged from pending panics.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn mark_acknowledged_from_pending_panics() {
        let receipt = TxReporter::new();
        let reporter = receipt.clone();
        reporter.mark_acknowledged(); // must be Transmitted first
    }
}
