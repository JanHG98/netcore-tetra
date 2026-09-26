// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Mock transport for testing. Records sent payloads and stubs all receives.

use std::collections::VecDeque;
use std::time::Instant;

use super::{NetworkAddress, NetworkError, NetworkMessage, NetworkTransport};

// Was: Bündelt die zusammengehörigen Werte für mock transport in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MockTransport {
    connected: bool,
    sent: Vec<Vec<u8>>,
    inbound: VecDeque<Vec<u8>>,
}

// Was: Implementiert das zugehörige Verhalten für `MockTransport`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MockTransport {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new() -> Self {
        Self {
            connected: false,
            sent: Vec::new(),
            inbound: VecDeque::new(),
        }
    }

    // Was: Führt den Arbeitsschritt `sent_payloads` für sent payloads aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn sent_payloads(&self) -> &[Vec<u8>] {
        &self.sent
    }

    /// Queue a raw payload that will be returned by the next `receive_reliable()` call.
    // Was: Diese Funktion legt inbound.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn push_inbound(&mut self, payload: Vec<u8>) {
        self.inbound.push_back(payload);
    }
}

// Was: Implementiert das zugehörige Verhalten für `NetworkTransport for MockTransport`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl NetworkTransport for MockTransport {
    // Was: Diese Funktion verbindet den vorgesehenen Arbeitsschritt.
    // Warum: Der Verbindungsaufbau wird dadurch zentral überwacht und kann sauber fehlschlagen.
    fn connect(&mut self) -> Result<(), NetworkError> {
        self.connected = true;
        Ok(())
    }

    // Was: Diese Funktion sendet reliable.
    // Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
    fn send_reliable(&mut self, payload: &[u8]) -> Result<(), NetworkError> {
        if !self.connected {
            return Err(NetworkError::SendFailed("not connected".into()));
        }
        self.sent.push(payload.to_vec());
        Ok(())
    }

    // Was: Diese Funktion sendet unreliable.
    // Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
    fn send_unreliable(&mut self, payload: &[u8]) -> Result<(), NetworkError> {
        self.send_reliable(payload)
    }

    // Was: Diese Funktion empfängt reliable.
    // Warum: Eingehende Daten werden so geordnet geprüft, bevor sie weiterverteilt werden.
    fn receive_reliable(&mut self) -> Vec<NetworkMessage> {
        self.inbound
            .drain(..)
            .map(|payload| NetworkMessage {
                source: NetworkAddress::Custom {
                    scheme: "mock".into(),
                    address: "test".into(),
                },
                payload,
                timestamp: Instant::now(),
            })
            .collect()
    }

    // Was: Diese Funktion empfängt unreliable.
    // Warum: Eingehende Daten werden so geordnet geprüft, bevor sie weiterverteilt werden.
    fn receive_unreliable(&mut self) -> Vec<NetworkMessage> {
        vec![]
    }

    // Was: Diese Funktion wartet for response reliable.
    // Warum: Nachfolgende Schritte laufen dadurch erst weiter, wenn ihre Voraussetzung wirklich erfüllt ist.
    fn wait_for_response_reliable(&mut self) -> Result<NetworkMessage, NetworkError> {
        Err(NetworkError::Timeout)
    }

    // Was: Prüft, ob connected zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    fn is_connected(&self) -> bool {
        self.connected
    }

    // Was: Diese Funktion trennt den vorgesehenen Arbeitsschritt.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn disconnect(&mut self) {
        self.connected = false;
    }
}
