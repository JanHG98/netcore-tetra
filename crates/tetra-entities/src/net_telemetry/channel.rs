// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use crossbeam_channel::{Receiver, RecvTimeoutError, Sender, unbounded};
use std::time::Duration;

use crate::net_telemetry::events::TelemetryEvent;

// ---------------------------------------------------------------------------
// TelemetrySink  (cloneable, push‑only handle given to entities)
//
// crossbeam Sender is Arc‑backed; cloning is a single atomic increment.
// send() is lock‑free — it claims a slot via atomic FAA and memcpys the
// TelemetryEvent into it.  Small events require zero heap allocation.
// Larger events should use a Box to keep the TelemetryEvent size small
// and avoid heap allocation on send.
// ---------------------------------------------------------------------------

#[derive(Clone)]
// Was: Bündelt die zusammengehörigen Werte für Telemetrie sink in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TelemetrySink {
    tx: Sender<TelemetryEvent>,
}

// Was: Implementiert das zugehörige Verhalten für `TelemetrySink`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl TelemetrySink {
    /// Push a telemetry event. Lock‑free. Fire‑and‑forget: silently drops if the receiver is gone.
    #[inline]
    // Was: Diese Funktion sendet den vorgesehenen Arbeitsschritt.
    // Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
    pub fn send(&self, event: TelemetryEvent) {
        let _ = self.tx.send(event);
    }
}

// ---------------------------------------------------------------------------
// TelemetrySource  (receive side, owned by the Telemetry component)
// ---------------------------------------------------------------------------

// Was: Bündelt die zusammengehörigen Werte für Telemetrie source in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TelemetrySource {
    rx: Receiver<TelemetryEvent>,
}

/// Result of a receive-with-timeout operation.
// Was: Listet die möglichen Varianten für recv Ereignis auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum RecvEvent {
    /// A telemetry event was received.
    Event(TelemetryEvent),
    /// Timed out waiting — channel is still open.
    Timeout,
    /// All sinks were dropped — channel is closed.
    Closed,
}

// Was: Implementiert das zugehörige Verhalten für `TelemetrySource`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl TelemetrySource {
    /// Blocking receive.  Returns `None` when all sinks have been dropped.
    // Was: Führt den Arbeitsschritt `recv` für recv aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn recv(&self) -> Option<TelemetryEvent> {
        self.rx.recv().ok()
    }

    /// Blocking receive with timeout, distinguishing timeout from channel close.
    // Was: Führt den Arbeitsschritt `recv_timeout` für recv timeout aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn recv_timeout(&self, timeout: Duration) -> RecvEvent {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self.rx.recv_timeout(timeout) {
            Ok(event) => RecvEvent::Event(event),
            Err(RecvTimeoutError::Timeout) => RecvEvent::Timeout,
            Err(RecvTimeoutError::Disconnected) => RecvEvent::Closed,
        }
    }

    /// Non-blocking try_recv.
    // Was: Führt den Arbeitsschritt `try_recv` für try recv aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn try_recv(&self) -> Option<TelemetryEvent> {
        self.rx.try_recv().ok()
    }
}

// ---------------------------------------------------------------------------
// Channel constructor
// ---------------------------------------------------------------------------

/// Create a linked (sink, source) pair.
// Was: Führt den Arbeitsschritt `telemetry_channel` für Telemetrie Kanal aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn telemetry_channel() -> (TelemetrySink, TelemetrySource) {
    let (tx, rx) = unbounded();
    (TelemetrySink { tx }, TelemetrySource { rx })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Prüft automatisch den Fall send two events.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_send_two_events() {
        let (sink, source) = telemetry_channel();

        sink.send(TelemetryEvent::MsRegistration { issi: 12345 });

        // Clone the sink (simulating a second entity) and send an Attach event
        let sink2 = sink.clone();
        sink2.send(TelemetryEvent::MsGroupAttach {
            issi: 12345,
            gssis: vec![1, 2, 3],
        });

        // Receive and verify
        let a = source.try_recv().expect("should receive Registration");
        assert!(matches!(a, TelemetryEvent::MsRegistration { issi: 12345 }));

        let b = source.try_recv().expect("should receive Attach");
        if let TelemetryEvent::MsGroupAttach { issi, gssis } = &b {
            assert_eq!(*issi, 12345);
            assert_eq!(*gssis, vec![1, 2, 3]);
        } else {
            panic!("expected Attach variant");
        }

        // No more items
        assert!(source.try_recv().is_none());
    }
}
