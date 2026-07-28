// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use tetra_pdus::cmce::enums::cmce_pdu_type_ul::CmcePduTypeUl;
use tetra_saps::{SapMsg, SapMsgInner};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für lcmc Weiterleitung auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum LcmcRoute {
    /// rd route: PC -> CC / CC -> PC, clause 14.2.6.
    CcRd,
    /// re route: PC -> SS / SS -> PC, clause 14.2.6.
    SsRe,
    /// rf route: PC -> SDS / SDS -> PC, clause 14.2.6.
    SdsRf,
    /// U-STATUS is an SDS PDU on rf, kept distinct because the BS implementation has a dedicated handler.
    SdsStatus,
    Unsupported(CmcePduTypeUl),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für Steuerung Weiterleitung auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum ControlRoute {
    /// ra/TNCC-side call-control input, including the local Brew/ISI bridge.
    CcRa,
    /// MM subscriber state used by CC for local routeing decisions.
    CcSubscriberUpdate,
    /// rc/TNSDS-side SDS input from the local network bridge.
    SdsRc,
    Unsupported,
}

/// BS-side Protocol Control role from EN 300 392-2 clause 14.2.5.
///
/// The standard defines PC as the router between CC/SS/SDS and LCMC. This component
/// keeps that discrimination out of the CC subentity so call control only receives
/// traffic that belongs on CC routes.
// Was: Bündelt die zusammengehörigen Werte für pc Basisstation in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct PcBs;

// Was: Implementiert das zugehörige Verhalten für `PcBs`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl PcBs {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new() -> Self {
        Self
    }

    // Was: Diese Funktion leitet lcmc unitdata ind.
    // Warum: Nachrichten und Daten gelangen dadurch nachvollziehbar an das richtige Ziel.
    pub fn route_lcmc_unitdata_ind(&self, message: &mut SapMsg) -> Option<LcmcRoute> {
        let SapMsgInner::LcmcMleUnitdataInd(prim) = &mut message.msg else {
            tracing::warn!("CMCE PC received non-LCMC unitdata indication: {:?}", message.msg);
            return None;
        };
        let Some(bits) = prim.sdu.peek_bits(5) else {
            tracing::warn!("CMCE PC received insufficient bits: {}", prim.sdu.dump_bin());
            return None;
        };
        let Ok(pdu_type) = CmcePduTypeUl::try_from(bits) else {
            tracing::warn!("CMCE PC received invalid UL PDU type {} in {}", bits, prim.sdu.dump_bin());
            return None;
        };

        Some(match pdu_type {
            CmcePduTypeUl::UAlert
            | CmcePduTypeUl::UCallRestore
            | CmcePduTypeUl::UConnect
            | CmcePduTypeUl::UDisconnect
            | CmcePduTypeUl::UInfo
            | CmcePduTypeUl::URelease
            | CmcePduTypeUl::USetup
            | CmcePduTypeUl::UTxCeased
            | CmcePduTypeUl::UTxDemand => LcmcRoute::CcRd,
            CmcePduTypeUl::UFacility => LcmcRoute::SsRe,
            CmcePduTypeUl::USdsData => LcmcRoute::SdsRf,
            CmcePduTypeUl::UStatus => LcmcRoute::SdsStatus,
            CmcePduTypeUl::CmceFunctionNotSupported => LcmcRoute::Unsupported(pdu_type),
        })
    }

    // Was: Diese Funktion leitet Steuerung.
    // Warum: Nachrichten und Daten gelangen dadurch nachvollziehbar an das richtige Ziel.
    pub fn route_control(&self, message: &SapMsg) -> ControlRoute {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match &message.msg {
            SapMsgInner::CmceCallControl(_) => ControlRoute::CcRa,
            SapMsgInner::MmSubscriberUpdate(_) => ControlRoute::CcSubscriberUpdate,
            SapMsgInner::CmceSdsData(_) => ControlRoute::SdsRc,
            _ => ControlRoute::Unsupported,
        }
    }
}
