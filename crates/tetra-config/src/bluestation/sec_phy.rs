// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Einlesen und Prüfen der TETRA-Konfiguration.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::HashMap;

use serde::Deserialize;
use toml::Value;

use crate::bluestation::{CfgSoapySdr, SoapySdrDto};

/// The PHY layer backend type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
// Was: Listet die möglichen Varianten für phy Hintergrunddienst auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum PhyBackend {
    Undefined,
    None,
    SoapySdr,
}

/// PHY layer I/O configuration
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für cfg phy io in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgPhyIo {
    /// Backend type: Soapysdr, File, or None
    pub backend: PhyBackend,

    pub dl_tx_file: Option<String>,
    pub ul_rx_file: Option<String>,
    pub ul_input_file: Option<String>,
    pub dl_input_file: Option<String>,

    /// For Soapysdr backend: SoapySDR configuration
    pub soapysdr: Option<CfgSoapySdr>,
}

#[derive(Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für phy io dto in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct PhyIoDto {
    pub backend: PhyBackend,

    pub dl_tx_file: Option<String>,
    pub ul_rx_file: Option<String>,
    pub ul_input_file: Option<String>,
    pub dl_input_file: Option<String>,

    pub soapysdr: Option<SoapySdrDto>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// Was: Führt den Arbeitsschritt `phy_dto_to_cfg` für phy dto to cfg aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn phy_dto_to_cfg(src: PhyIoDto) -> CfgPhyIo {
    let soapysdr = src.soapysdr.map(|soapy_dto| {
        CfgSoapySdr {
            ul_freq: soapy_dto.rx_freq,
            dl_freq: soapy_dto.tx_freq,
            ppm_err: soapy_dto.ppm_err.unwrap_or(0.0),
            rx_center_freq: soapy_dto.rx_center_freq,
            tx_center_freq: soapy_dto.tx_center_freq,
            device: soapy_dto.device,
            fs: soapy_dto.sample_rate,
            rx_ch: soapy_dto.rx_channel,
            tx_ch: soapy_dto.tx_channel,
            rx_ant: soapy_dto.rx_antenna,
            tx_ant: soapy_dto.tx_antenna,
            rx_gains: soapy_dto
                .extra
                .iter()
                .filter_map(|(key, value)| {
                    key.strip_prefix("rx_gain_").map(|gain_name| {
                        (
                            gain_name.to_string().to_lowercase(),
                            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                            match value {
                                Value::Integer(v) => *v as f64,
                                Value::Float(v) => *v,
                                // TODO: should this error be returned somehow?
                                _ => panic!("RX gain value must be a number"),
                            },
                        )
                    })
                })
                .collect(),
            tx_gains: soapy_dto
                .extra
                .iter()
                .filter_map(|(key, value)| {
                    key.strip_prefix("tx_gain_").map(|gain_name| {
                        (
                            gain_name.to_string().to_lowercase(),
                            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                            match value {
                                Value::Integer(v) => *v as f64,
                                Value::Float(v) => *v,
                                // TODO: should this error be returned somehow?
                                _ => panic!("TX gain value must be a number"),
                            },
                        )
                    })
                })
                .collect(),
        }
    });

    CfgPhyIo {
        backend: src.backend,
        dl_tx_file: src.dl_tx_file,
        ul_rx_file: src.ul_rx_file,
        ul_input_file: src.ul_input_file,
        dl_input_file: src.dl_input_file,
        soapysdr,
    }
}
