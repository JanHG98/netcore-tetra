// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::HashMap;

use crossbeam_channel::{Receiver, Sender, unbounded};
use tetra_core::tetra_entities::TetraEntity;

use crate::net_control::commands::{ControlCommand, ControlResponse};

// ---------------------------------------------------------------------------
// CommandDispatcher  (worker side of a per‑entity link)
//
// The CommandWorker holds one CommandDispatcher per connected entity.
// It sends Commands toward the entity and collects CommandResponses back.
// ---------------------------------------------------------------------------

#[derive(Clone)]
// Was: Bündelt die zusammengehörigen Werte für command dispatcher in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CommandDispatcher {
    cmd_tx: Sender<ControlCommand>,
    resp_rx: Receiver<ControlResponse>,
}

// Was: Implementiert das zugehörige Verhalten für `CommandDispatcher`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl CommandDispatcher {
    /// Send a command to the linked entity. Fire‑and‑forget: silently drops
    /// if the entity's endpoint has been dropped.
    #[inline]
    // Was: Diese Funktion sendet den vorgesehenen Arbeitsschritt.
    // Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
    pub fn send(&self, command: ControlCommand) {
        let _ = self.cmd_tx.send(command);
    }

    /// Clone just the Sender half so a secondary source (e.g. dashboard) can
    /// inject commands into the same channel without owning the full Dispatcher.
    // Was: Führt den Arbeitsschritt `clone_sender` für clone sender aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn clone_sender(&self) -> Sender<ControlCommand> {
        self.cmd_tx.clone()
    }

    /// Non-blocking: collect all pending responses from the entity.
    // Was: Führt den Arbeitsschritt `try_recv_responses` für try recv responses aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn try_recv_responses(&self) -> Vec<ControlResponse> {
        let mut responses = Vec::new();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        while let Ok(resp) = self.resp_rx.try_recv() {
            responses.push(resp);
        }
        responses
    }

    /// Non-blocking: collect a single pending response, if any.
    // Was: Führt den Arbeitsschritt `try_recv_response` für try recv response aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn try_recv_response(&self) -> Option<ControlResponse> {
        self.resp_rx.try_recv().ok()
    }
}

// ---------------------------------------------------------------------------
// ControlEndpoint  (entity side of a per‑entity link)
//
// Each TetraEntity that participates in the control subsystem holds one
// ControlEndpoint.  It receives ControlCommands and sends ControlResponses back
// toward the worker.
// ---------------------------------------------------------------------------

#[derive(Clone)]
// Was: Bündelt die zusammengehörigen Werte für Steuerung endpoint in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ControlEndpoint {
    cmd_rx: Receiver<ControlCommand>,
    resp_tx: Sender<ControlResponse>,
}

// Was: Implementiert das zugehörige Verhalten für `ControlEndpoint`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl ControlEndpoint {
    /// Non-blocking: receive a pending command, if any.
    // Was: Führt den Arbeitsschritt `try_recv` für try recv aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn try_recv(&self) -> Option<ControlCommand> {
        self.cmd_rx.try_recv().ok()
    }

    /// Blocking receive.  Returns `None` when the dispatcher has been dropped.
    // Was: Führt den Arbeitsschritt `recv` für recv aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn recv(&self) -> Option<ControlCommand> {
        self.cmd_rx.recv().ok()
    }

    /// Send a response back to the worker. Fire‑and‑forget: silently drops
    /// if the dispatcher has been dropped.
    #[inline]
    // Was: Führt den Arbeitsschritt `respond` für respond aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn respond(&self, response: ControlResponse) {
        let _ = self.resp_tx.send(response);
    }
}

// ---------------------------------------------------------------------------
// Link constructor
// ---------------------------------------------------------------------------

/// Create a bidirectional (dispatcher, endpoint) pair.
///
/// - The **dispatcher** (worker side) sends [`ControlCommand`]s and receives [`ControlResponse`]s.
/// - The **endpoint** (entity side) receives [`ControlCommand`]s and sends [`ControlResponse`]s.
// Was: Führt den Arbeitsschritt `make_control_link` für make Steuerung link aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn make_control_link() -> (CommandDispatcher, ControlEndpoint) {
    let (cmd_tx, cmd_rx) = unbounded();
    let (resp_tx, resp_rx) = unbounded();
    (CommandDispatcher { cmd_tx, resp_rx }, ControlEndpoint { cmd_rx, resp_tx })
}

/// Build one CommandDispatcher / CommandEndpoint pair per TetraEntity
// Was: Diese Funktion erstellt all Steuerung links.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_all_control_links() -> (HashMap<TetraEntity, CommandDispatcher>, HashMap<TetraEntity, ControlEndpoint>) {
    let mut dispatchers = HashMap::new();
    let mut endpoints = HashMap::new();

    let (dispatcher, endpoint) = make_control_link();
    dispatchers.insert(TetraEntity::Phy, dispatcher);
    endpoints.insert(TetraEntity::Phy, endpoint);

    let (dispatcher, endpoint) = make_control_link();
    dispatchers.insert(TetraEntity::Lmac, dispatcher);
    endpoints.insert(TetraEntity::Lmac, endpoint);

    let (dispatcher, endpoint) = make_control_link();
    dispatchers.insert(TetraEntity::Umac, dispatcher);
    endpoints.insert(TetraEntity::Umac, endpoint);

    let (dispatcher, endpoint) = make_control_link();
    dispatchers.insert(TetraEntity::Llc, dispatcher);
    endpoints.insert(TetraEntity::Llc, endpoint);

    let (dispatcher, endpoint) = make_control_link();
    dispatchers.insert(TetraEntity::Mle, dispatcher);
    endpoints.insert(TetraEntity::Mle, endpoint);

    let (dispatcher, endpoint) = make_control_link();
    dispatchers.insert(TetraEntity::Mm, dispatcher);
    endpoints.insert(TetraEntity::Mm, endpoint);

    let (dispatcher, endpoint) = make_control_link();
    dispatchers.insert(TetraEntity::Cmce, dispatcher);
    endpoints.insert(TetraEntity::Cmce, endpoint);

    let (dispatcher, endpoint) = make_control_link();
    dispatchers.insert(TetraEntity::Sndcp, dispatcher);
    endpoints.insert(TetraEntity::Sndcp, endpoint);

    (dispatchers, endpoints)
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
    // Was: Prüft automatisch den Fall send command and receive response.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_send_command_and_receive_response() {
        let (dispatcher, endpoint) = make_control_link();

        // Worker sends a command
        dispatcher.send(ControlCommand::CommandA { handle: 1, parameter: 42 });

        // Entity receives it
        let cmd = endpoint.try_recv().unwrap();
        assert!(matches!(cmd, ControlCommand::CommandA { handle: 1, parameter: 42 }));

        // Entity sends a response
        endpoint.respond(ControlResponse::CommandAResponse { handle: 1, result: 42 });

        // Worker receives the response
        let resp = dispatcher.try_recv_response().unwrap();
        let ControlResponse::CommandAResponse { handle, result } = resp else {
            panic!("expected CommandAResponse");
        };
        assert_eq!(handle, 1);
        assert_eq!(result, 42);
    }

    #[test]
    // Was: Prüft automatisch den Fall multiple commands and responses.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_multiple_commands_and_responses() {
        let (dispatcher, endpoint) = make_control_link();

        dispatcher.send(ControlCommand::CommandA { handle: 1, parameter: 1 });
        dispatcher.send(ControlCommand::TestCmdB {
            handle: 2,
            source_ssi: 0,
            is_group: false,
            payload: vec![0xFF],
        });

        // Entity drains both
        let a = endpoint.try_recv().unwrap();
        assert!(matches!(a, ControlCommand::CommandA { handle: 1, parameter: 1 }));
        let b = endpoint.try_recv().unwrap();
        if let ControlCommand::TestCmdB {
            handle,
            source_ssi: ssi,
            is_group,
            payload,
        } = &b
        {
            assert_eq!(*handle, 2);
            assert_eq!(*ssi, 0);
            assert_eq!(*is_group, false);
            assert_eq!(*payload, vec![0xFF]);
        } else {
            panic!("expected Command::SendSds variant");
        }
        assert!(endpoint.try_recv().is_none());

        // Entity responds to both
        endpoint.respond(ControlResponse::CommandAResponse { handle: 1, result: 1 });
        endpoint.respond(ControlResponse::SendSdsResponse { handle: 2, success: true });

        // Worker drains all responses at once
        let responses = dispatcher.try_recv_responses();
        assert_eq!(responses.len(), 2);
    }
}
