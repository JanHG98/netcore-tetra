// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use super::*;

// Was: Implementiert das zugehörige Verhalten für `CcBsSubentity`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl CcBsSubentity {
    // Was: Führt den Arbeitsschritt `rx_call_control` für rx Ruf Steuerung aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn rx_call_control(&mut self, queue: &mut MessageQueue, message: SapMsg) {
        let network_entity = message.src;
        let SapMsgInner::CmceCallControl(call_control) = message.msg else {
            tracing::warn!("CMCE CC control ingress received non-call-control message");
            return;
        };

        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match call_control {
            CallControl::NetworkCallStart {
                brew_uuid,
                source_issi,
                dest_gssi,
                priority,
            } => {
                self.rx_network_call_start(queue, network_entity, brew_uuid, source_issi, dest_gssi, priority);
            }
            CallControl::NetworkCallEnd { brew_uuid } => {
                self.rx_network_call_end(queue, brew_uuid);
            }
            CallControl::UlInactivityTimeout { ts } => {
                self.handle_ul_inactivity_timeout(queue, ts);
            }
            CallControl::NetworkCircuitSetupRequest { brew_uuid, call } => {
                self.rx_network_circuit_setup_request(queue, network_entity, brew_uuid, call);
            }
            CallControl::NetworkCircuitSetupAccept { brew_uuid } => {
                self.rx_network_circuit_setup_accept(brew_uuid);
            }
            CallControl::NetworkCircuitSetupReject { brew_uuid, cause } => {
                self.rx_network_circuit_setup_reject(queue, brew_uuid, cause);
            }
            CallControl::NetworkCircuitAlert { brew_uuid } => {
                self.rx_network_circuit_alert(queue, brew_uuid);
            }
            CallControl::NetworkCircuitConnectRequest { brew_uuid, call } => {
                self.rx_network_circuit_connect_request(queue, brew_uuid, call);
            }
            CallControl::NetworkCircuitConnectConfirm {
                brew_uuid,
                grant,
                permission,
            } => {
                self.rx_network_circuit_connect_confirm(queue, brew_uuid, grant, permission);
            }
            CallControl::NetworkCircuitSimplexGranted {
                brew_uuid,
                grant,
                permission,
            } => {
                self.rx_network_circuit_simplex_granted(queue, brew_uuid, grant, permission);
            }
            CallControl::NetworkCircuitSimplexIdle {
                brew_uuid,
                grant,
                permission,
            } => {
                self.rx_network_circuit_simplex_idle(queue, brew_uuid, grant, permission);
            }
            CallControl::NetworkCircuitMediaReady { brew_uuid, .. } => {
                tracing::trace!("CMCE: ignoring unexpected NetworkCircuitMediaReady uuid={}", brew_uuid);
            }
            CallControl::NetworkCircuitRelease { brew_uuid, cause } => {
                self.rx_network_circuit_release(queue, brew_uuid, cause);
            }
            _ => {
                tracing::warn!("Unexpected CallControl message: {:?}", call_control);
            }
        }
    }
}
