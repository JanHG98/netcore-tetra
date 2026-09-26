// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Einlesen und Prüfen der TETRA-Konfiguration.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::{BTreeSet, HashMap};

use serde::Deserialize;
use toml::Value;

use crate::bluestation::{SecretField, parse_ric_route_key};

/// Snom XML minibrowser notification bridge (`[snom_notify]`).
///
/// Sends FlowStation message events to one or more Asterisk PJSIP endpoints via AMI
/// `PJSIPNotify`. The generated SIP NOTIFY uses Snom's XML minibrowser format:
/// `Event: xml`, `Content-Type: application/snomxml`, body `SnomIPPhoneText`.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für cfg snom notify in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgSnomNotify {
    pub enabled: bool,
    pub ami_host: String,
    pub ami_port: u16,
    pub ami_username: String,
    pub ami_password: SecretField,
    pub endpoints: Vec<String>,
    pub notify_sds: bool,
    pub notify_dapnet: bool,
    pub notify_telegram: bool,
    pub sds_directions: Vec<String>,
    /// Optional DAPNET RIC allowlist for Snom notifications. Empty means all RICs.
    pub dapnet_allowed_rics: BTreeSet<u32>,
    /// Optional SDS ISSI allowlist for Snom notifications. Empty means all SDS.
    pub sds_allowed_issis: BTreeSet<u32>,
    pub title_prefix: String,
    pub notify_event: String,
    pub content_type: String,
    pub subscription_state: String,
    pub max_text_chars: usize,
    pub connect_timeout_secs: u64,
}

// Was: Implementiert das zugehörige Verhalten für `Default for CfgSnomNotify`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for CfgSnomNotify {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        apply_snom_notify_patch(CfgSnomNotifyDto::default()).expect("default snom_notify config must be valid")
    }
}

#[derive(Debug, Clone, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für cfg snom notify dto in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgSnomNotifyDto {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_ami_host")]
    pub ami_host: String,
    #[serde(default = "default_ami_port")]
    pub ami_port: u16,
    #[serde(default)]
    pub ami_username: String,
    #[serde(default)]
    pub ami_password: String,
    #[serde(default)]
    pub endpoints: Vec<String>,
    #[serde(default = "default_true")]
    pub notify_sds: bool,
    #[serde(default = "default_true")]
    pub notify_dapnet: bool,
    #[serde(default = "default_true")]
    pub notify_telegram: bool,
    #[serde(default = "default_sds_directions")]
    pub sds_directions: Vec<String>,
    #[serde(default)]
    pub dapnet_allowed_rics: Vec<Value>,
    #[serde(default)]
    pub sds_allowed_issis: Vec<u32>,
    #[serde(default = "default_title_prefix")]
    pub title_prefix: String,
    #[serde(default = "default_notify_event")]
    pub notify_event: String,
    #[serde(default = "default_content_type")]
    pub content_type: String,
    #[serde(default = "default_subscription_state")]
    pub subscription_state: String,
    #[serde(default = "default_max_text_chars")]
    pub max_text_chars: usize,
    #[serde(default = "default_connect_timeout_secs")]
    pub connect_timeout_secs: u64,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// Was: Implementiert das zugehörige Verhalten für `Default for CfgSnomNotifyDto`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for CfgSnomNotifyDto {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            enabled: false,
            ami_host: default_ami_host(),
            ami_port: default_ami_port(),
            ami_username: String::new(),
            ami_password: String::new(),
            endpoints: Vec::new(),
            notify_sds: true,
            notify_dapnet: true,
            notify_telegram: true,
            sds_directions: default_sds_directions(),
            dapnet_allowed_rics: Vec::new(),
            sds_allowed_issis: Vec::new(),
            title_prefix: default_title_prefix(),
            notify_event: default_notify_event(),
            content_type: default_content_type(),
            subscription_state: default_subscription_state(),
            max_text_chars: default_max_text_chars(),
            connect_timeout_secs: default_connect_timeout_secs(),
            extra: HashMap::new(),
        }
    }
}

// Was: Führt den Arbeitsschritt `default_ami_host` für default ami host aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_ami_host() -> String {
    "127.0.0.1".to_string()
}
// Was: Führt den Arbeitsschritt `default_ami_port` für default ami port aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_ami_port() -> u16 {
    5038
}
// Was: Führt den Arbeitsschritt `default_sds_directions` für default TETRA-Kurznachricht (SDS) directions aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_sds_directions() -> Vec<String> {
    vec!["rx".to_string(), "net".to_string(), "tx".to_string()]
}
// Was: Führt den Arbeitsschritt `default_true` für default true aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_true() -> bool {
    true
}
// Was: Führt den Arbeitsschritt `default_title_prefix` für default title prefix aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_title_prefix() -> String {
    "FlowStation".to_string()
}
// Was: Führt den Arbeitsschritt `default_notify_event` für default notify Ereignis aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_notify_event() -> String {
    "xml".to_string()
}
// Was: Führt den Arbeitsschritt `default_content_type` für default content type aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_content_type() -> String {
    "application/snomxml".to_string()
}
// Was: Führt den Arbeitsschritt `default_subscription_state` für default subscription Zustand aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_subscription_state() -> String {
    "active;expires=30000".to_string()
}
// Was: Führt den Arbeitsschritt `default_max_text_chars` für default max text chars aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_max_text_chars() -> usize {
    240
}
// Was: Führt den Arbeitsschritt `default_connect_timeout_secs` für default connect timeout secs aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_connect_timeout_secs() -> u64 {
    3
}

// Was: Diese Funktion wendet snom notify patch.
// Warum: Die Änderung wird dadurch nur über einen definierten und prüfbaren Weg wirksam.
pub fn apply_snom_notify_patch(src: CfgSnomNotifyDto) -> Result<CfgSnomNotify, String> {
    if src.ami_port == 0 {
        return Err("snom_notify: ami_port cannot be 0".to_string());
    }
    let ami_host = src.ami_host.trim().to_string();
    if src.enabled && ami_host.is_empty() {
        return Err("snom_notify: ami_host cannot be empty when enabled=true".to_string());
    }

    let endpoints: Vec<String> = src
        .endpoints
        .into_iter()
        .map(|e| e.trim().to_string())
        .filter(|e| !e.is_empty())
        .collect();

    let sds_directions: Vec<String> = src
        .sds_directions
        .into_iter()
        .map(|d| d.trim().to_ascii_lowercase())
        .filter(|d| !d.is_empty())
        .collect();
    let dapnet_allowed_rics = normalize_ric_value_list(src.dapnet_allowed_rics)?;
    let sds_allowed_issis = normalize_issi_list(src.sds_allowed_issis)?;

    Ok(CfgSnomNotify {
        enabled: src.enabled,
        ami_host,
        ami_port: src.ami_port,
        ami_username: src.ami_username.trim().to_string(),
        ami_password: SecretField::from(src.ami_password),
        endpoints,
        notify_sds: src.notify_sds,
        notify_dapnet: src.notify_dapnet,
        notify_telegram: src.notify_telegram,
        sds_directions,
        dapnet_allowed_rics,
        sds_allowed_issis,
        title_prefix: non_empty_or(src.title_prefix, default_title_prefix()),
        notify_event: non_empty_or(src.notify_event, default_notify_event()),
        content_type: non_empty_or(src.content_type, default_content_type()),
        subscription_state: non_empty_or(src.subscription_state, default_subscription_state()),
        max_text_chars: src.max_text_chars.clamp(40, 2000),
        connect_timeout_secs: src.connect_timeout_secs.clamp(1, 30),
    })
}

// Was: Führt den Arbeitsschritt `normalize_ric_value_list` für normalize ric value list aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn normalize_ric_value_list(values: Vec<Value>) -> Result<BTreeSet<u32>, String> {
    let mut out = BTreeSet::new();
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for value in values {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        let ric = match value {
            Value::String(s) => parse_ric_route_key(&s)?,
            Value::Integer(n) if n >= 0 => parse_ric_route_key(&n.to_string())?,
            other => return Err(format!("snom_notify: invalid RIC value {other:?}")),
        };
        out.insert(ric);
    }
    Ok(out)
}

// Was: Führt den Arbeitsschritt `normalize_issi_list` für normalize Teilnehmerkennung (ISSI) list aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn normalize_issi_list(values: Vec<u32>) -> Result<BTreeSet<u32>, String> {
    let mut out = BTreeSet::new();
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for issi in values {
        if issi > 16_777_215 {
            return Err(format!("snom_notify: SDS ISSI {} out of range", issi));
        }
        out.insert(issi);
    }
    Ok(out)
}

// Was: Führt den Arbeitsschritt `non_empty_or` für non empty or aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn non_empty_or(value: String, fallback: String) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() { fallback } else { trimmed.to_string() }
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Führt den Arbeitsschritt `defaults_are_disabled_snom_xml_notify` für defaults are disabled snom xml notify aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn defaults_are_disabled_snom_xml_notify() {
        let cfg = CfgSnomNotify::default();
        assert!(!cfg.enabled);
        assert_eq!(cfg.ami_host, "127.0.0.1");
        assert_eq!(cfg.ami_port, 5038);
        assert_eq!(cfg.notify_event, "xml");
        assert_eq!(cfg.content_type, "application/snomxml");
    }

    #[test]
    // Was: Führt den Arbeitsschritt `trims_endpoint_and_direction_lists` für trims endpoint and direction lists aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn trims_endpoint_and_direction_lists() {
        let dto = CfgSnomNotifyDto {
            endpoints: vec![" 385 ".to_string(), "".to_string()],
            sds_directions: vec![" RX ".to_string(), "net".to_string()],
            dapnet_allowed_rics: vec![Value::String("0632585".to_string())],
            sds_allowed_issis: vec![2632585, 9999],
            ..Default::default()
        };
        let cfg = apply_snom_notify_patch(dto).unwrap();
        assert_eq!(cfg.endpoints, vec!["385"]);
        assert_eq!(cfg.sds_directions, vec!["rx", "net"]);
        assert!(cfg.dapnet_allowed_rics.contains(&632585));
        assert!(cfg.sds_allowed_issis.contains(&2632585));
    }
}
