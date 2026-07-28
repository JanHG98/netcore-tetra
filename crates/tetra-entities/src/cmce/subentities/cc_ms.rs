// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use tetra_core::unimplemented_log;
use tetra_pdus::cmce::enums::cmce_pdu_type_dl::CmcePduTypeDl;
use tetra_saps::{SapMsg, SapMsgInner};

use crate::MessageQueue;

/// Clause 11 Call Control CMCE sub-entity
// Was: Bündelt die zusammengehörigen Werte für cc ms subentity in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CcMsSubentity {}

// Was: Implementiert das zugehörige Verhalten für `CcMsSubentity`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl CcMsSubentity {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new() -> Self {
        CcMsSubentity {}
    }

    // Was: Diese Funktion leitet rd deliver.
    // Warum: Nachrichten und Daten gelangen dadurch nachvollziehbar an das richtige Ziel.
    pub fn route_rd_deliver(&mut self, _queue: &mut MessageQueue, mut message: SapMsg) {
        tracing::trace!("route_rd_deliver");

        let SapMsgInner::LcmcMleUnitdataInd(prim) = &mut message.msg else {
            tracing::error!("BUG: unexpected message or state -- routing error");
            return;
        };
        let Some(bits) = prim.sdu.peek_bits(5) else {
            tracing::warn!("insufficient bits: {}", prim.sdu.dump_bin());
            return;
        };

        let Ok(pdu_type) = CmcePduTypeDl::try_from(bits) else {
            tracing::warn!("invalid pdu type: {} in {}", bits, prim.sdu.dump_bin());
            return;
        };

        // TODO FIXME: Besides these PDUs, we can also receive several signals (BUSY ind, CLOSE ind, etc)
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match pdu_type {
            CmcePduTypeDl::DAlert => {
                unimplemented_log!("{}", pdu_type);
            }
            CmcePduTypeDl::DCallProceeding => {
                unimplemented_log!("{}", pdu_type);
            }
            CmcePduTypeDl::DCallRestore => {
                unimplemented_log!("{}", pdu_type);
            }
            CmcePduTypeDl::DConnect => {
                unimplemented_log!("{}", pdu_type);
            }
            CmcePduTypeDl::DConnectAcknowledge => {
                unimplemented_log!("{}", pdu_type);
            }
            CmcePduTypeDl::DDisconnect => {
                unimplemented_log!("{}", pdu_type);
            }
            CmcePduTypeDl::DInfo => {
                unimplemented_log!("{}", pdu_type);
            }
            CmcePduTypeDl::DRelease => {
                unimplemented_log!("{}", pdu_type);
            }
            CmcePduTypeDl::DSetup => {
                unimplemented_log!("{}", pdu_type);
            }
            CmcePduTypeDl::DTxCeased => {
                unimplemented_log!("{}", pdu_type);
            }
            CmcePduTypeDl::DTxContinue => {
                unimplemented_log!("{}", pdu_type);
            }
            CmcePduTypeDl::DTxGranted => {
                unimplemented_log!("{}", pdu_type);
            }
            CmcePduTypeDl::DTxInterrupt => {
                unimplemented_log!("{}", pdu_type);
            }
            CmcePduTypeDl::DTxWait => {
                unimplemented_log!("{}", pdu_type);
            }
            _ => {
                tracing::error!("BUG: unexpected message or state -- routing error");
                return;
            }
        }
    }
}
