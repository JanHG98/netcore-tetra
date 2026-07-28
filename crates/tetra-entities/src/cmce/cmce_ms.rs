// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use crate::{MessageQueue, TetraEntityTrait};
use tetra_config::bluestation::SharedConfig;
use tetra_core::Sap;
use tetra_core::tetra_entities::TetraEntity;
use tetra_saps::{SapMsg, SapMsgInner};

use tetra_pdus::cmce::enums::cmce_pdu_type_dl::CmcePduTypeDl;

use super::subentities::cc_ms::CcMsSubentity;
use super::subentities::sds_ms::SdsMsSubentity;
use super::subentities::ss_ms::SsMsSubentity;

// Was: Bündelt die zusammengehörigen Werte für CMCE-Rufsteuerung ms in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CmceMs {
    config: SharedConfig,

    sds: SdsMsSubentity,
    cc: CcMsSubentity,
    ss: SsMsSubentity,
}

// Was: Implementiert das zugehörige Verhalten für `CmceMs`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl CmceMs {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(config: SharedConfig) -> Self {
        Self {
            config,
            sds: SdsMsSubentity::new(),
            cc: CcMsSubentity::new(),
            ss: SsMsSubentity::new(),
        }
    }

    // Was: Führt den Arbeitsschritt `rx_unitdata_ind` für rx unitdata ind aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn rx_unitdata_ind(&mut self, queue: &mut MessageQueue, mut message: SapMsg) {
        tracing::trace!("rx_unitdata_ind");

        // Handle the incoming unit data indication
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

        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match pdu_type {
            CmcePduTypeDl::DSdsData | CmcePduTypeDl::DStatus => {
                self.sds.route_rf_deliver(queue, message);
            }
            CmcePduTypeDl::DFacility => {
                self.ss.route_re_deliver(queue, message);
            }
            CmcePduTypeDl::DAlert
            | CmcePduTypeDl::DCallProceeding
            | CmcePduTypeDl::DCallRestore
            | CmcePduTypeDl::DConnect
            | CmcePduTypeDl::DConnectAcknowledge
            | CmcePduTypeDl::DDisconnect
            | CmcePduTypeDl::DInfo
            | CmcePduTypeDl::DRelease
            | CmcePduTypeDl::DSetup
            | CmcePduTypeDl::DTxCeased
            | CmcePduTypeDl::DTxContinue
            | CmcePduTypeDl::DTxGranted
            | CmcePduTypeDl::DTxInterrupt
            | CmcePduTypeDl::DTxWait => {
                self.cc.route_rd_deliver(queue, message);
            }
            _ => {
                tracing::error!("BUG: unexpected message or state -- routing error");
                return;
            }
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `TetraEntityTrait for CmceMs`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl TetraEntityTrait for CmceMs {
    // Was: Führt den Arbeitsschritt `entity` für entity aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn entity(&self) -> TetraEntity {
        TetraEntity::Cmce
    }

    // Was: Diese Funktion setzt Konfiguration.
    // Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
    fn set_config(&mut self, config: SharedConfig) {
        self.config = config;
    }

    // Was: Führt den Arbeitsschritt `rx_prim` für rx prim aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn rx_prim(&mut self, queue: &mut MessageQueue, message: SapMsg) {
        tracing::debug!("rx_prim: {:?}", message);
        // tracing::debug!(ts=%message.dltime, "rx_prim: {:?}", message);

        // There is only one SAP for CMCE
        assert!(message.sap == Sap::LcmcSap);

        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match message.msg {
            SapMsgInner::LcmcMleUnitdataInd(_) => {
                self.rx_unitdata_ind(queue, message);
            }
            _ => {
                tracing::error!("BUG: unexpected message or state -- routing error");
                return;
            }
        }
    }
}
