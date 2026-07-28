// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Einlesen und Prüfen der TETRA-Konfiguration.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use serde::Deserialize;
use std::collections::HashMap;
use toml::Value;

#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für cfg net info in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgNetInfo {
    /// 10 bits, from 18.4.2.1 D-MLE-SYNC
    pub mcc: u16,
    /// 14 bits, from 18.4.2.1 D-MLE-SYNC
    pub mnc: u16,
}

#[derive(Default, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für net info dto in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct NetInfoDto {
    pub mcc: u16,
    pub mnc: u16,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// Was: Führt den Arbeitsschritt `net_dto_to_cfg` für net dto to cfg aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn net_dto_to_cfg(ni: NetInfoDto) -> CfgNetInfo {
    CfgNetInfo { mcc: ni.mcc, mnc: ni.mnc }
}
