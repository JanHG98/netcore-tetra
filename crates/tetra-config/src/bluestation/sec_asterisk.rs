// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Einlesen und Prüfen der TETRA-Konfiguration.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::HashMap;

use serde::Deserialize;
use toml::Value;

use crate::bluestation::SecretField;

/// Asterisk SIP/RTP bridge configuration.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für cfg asterisk in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgAsterisk {
    pub enabled: bool,
    pub outbound_prefix: String,
    pub strip_outbound_prefix: bool,
    pub inbound_prefix: String,
    pub register: bool,
    pub codec: String,
    pub service_numbers: Vec<String>,
    pub rtp_port_min: u16,
    pub rtp_port_max: u16,
    pub bind_addr: String,
    pub bind_port: u16,
    pub remote_host: String,
    pub remote_port: u16,
    pub contact_host: String,
    pub from_domain: String,
    pub local_user: String,
    pub auth_user: String,
    pub password: SecretField,
    pub realm: String,
    pub options_interval_secs: u64,
    /// Timeout for Asterisk-originated calls while waiting for the called TETRA MS to answer D-SETUP.
    pub inbound_setup_timeout_secs: u32,
}

// Was: Implementiert das zugehörige Verhalten für `Default for CfgAsterisk`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for CfgAsterisk {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        apply_asterisk_patch(CfgAsteriskDto::default()).expect("default asterisk config must be valid")
    }
}

// Was: Implementiert das zugehörige Verhalten für `CfgAsterisk`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl CfgAsterisk {
    /// Route a TETRA dial string to a SIP user according to the Asterisk outbound rules.
    ///
    /// Matching modes:
    /// - `outbound_prefix = "91"` and empty `service_numbers` routes every `91...` dial.
    /// - `outbound_prefix = "91*"` explicitly routes every `91...` dial.
    /// - `service_numbers = ["*"]` routes every dial behind the configured prefix.
    /// - `service_numbers = ["38*"]` routes every stripped number starting with `38`.
    /// - Exact `service_numbers` entries keep their old allowlist behaviour.
    // Was: Diese Funktion leitet outbound raw.
    // Warum: Nachrichten und Daten gelangen dadurch nachvollziehbar an das richtige Ziel.
    pub fn route_outbound_raw(&self, raw: &str) -> Option<String> {
        if !self.enabled {
            return None;
        }

        let raw = raw.trim();
        if raw.is_empty() {
            return None;
        }

        let configured_prefix = self.outbound_prefix.trim();
        let prefix_wildcard = configured_prefix.ends_with('*');
        let outbound_prefix = configured_prefix.trim_end_matches('*');
        let prefix_matched = if outbound_prefix.is_empty() {
            prefix_wildcard
        } else {
            raw.starts_with(outbound_prefix)
        };

        let routed = if prefix_matched && self.strip_outbound_prefix {
            &raw[outbound_prefix.len()..]
        } else {
            raw
        }
        .trim();

        if routed.is_empty() {
            return None;
        }

        if prefix_wildcard && prefix_matched {
            return Some(routed.to_string());
        }

        if self.service_numbers.is_empty() {
            if prefix_matched {
                return Some(routed.to_string());
            }
            return None;
        }

        if self.service_numbers.iter().any(|n| n == routed) {
            return Some(routed.to_string());
        }

        let wildcard_allowed = outbound_prefix.is_empty() || prefix_matched;
        if wildcard_allowed
            && self.service_numbers.iter().any(|n| {
                n == "*"
                    || n.strip_suffix('*')
                        .is_some_and(|prefix| !prefix.is_empty() && routed.starts_with(prefix))
            })
        {
            return Some(routed.to_string());
        }

        None
    }
}

#[derive(Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für cfg asterisk dto in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgAsteriskDto {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_outbound_prefix")]
    pub outbound_prefix: String,
    #[serde(default = "default_strip_outbound_prefix")]
    pub strip_outbound_prefix: bool,
    #[serde(default = "default_inbound_prefix")]
    pub inbound_prefix: String,
    #[serde(default = "default_register")]
    pub register: bool,
    #[serde(default = "default_codec")]
    pub codec: String,
    #[serde(default)]
    pub service_numbers: Vec<String>,
    #[serde(default = "default_rtp_port_min")]
    pub rtp_port_min: u16,
    #[serde(default = "default_rtp_port_max")]
    pub rtp_port_max: u16,
    #[serde(default = "default_bind_addr")]
    pub bind_addr: String,
    #[serde(default = "default_bind_port")]
    pub bind_port: u16,
    #[serde(default = "default_remote_host")]
    pub remote_host: String,
    #[serde(default = "default_remote_port")]
    pub remote_port: u16,
    #[serde(default = "default_contact_host")]
    pub contact_host: String,
    #[serde(default = "default_from_domain")]
    pub from_domain: String,
    #[serde(default = "default_local_user")]
    pub local_user: String,
    #[serde(default = "default_auth_user")]
    pub auth_user: String,
    #[serde(default)]
    pub password: String,
    #[serde(default = "default_realm")]
    pub realm: String,
    #[serde(default = "default_options_interval_secs")]
    pub options_interval_secs: u64,
    #[serde(default = "default_inbound_setup_timeout_secs")]
    pub inbound_setup_timeout_secs: u32,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// Was: Implementiert das zugehörige Verhalten für `Default for CfgAsteriskDto`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for CfgAsteriskDto {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            enabled: false,
            outbound_prefix: default_outbound_prefix(),
            strip_outbound_prefix: default_strip_outbound_prefix(),
            inbound_prefix: default_inbound_prefix(),
            register: default_register(),
            codec: default_codec(),
            service_numbers: Vec::new(),
            rtp_port_min: default_rtp_port_min(),
            rtp_port_max: default_rtp_port_max(),
            bind_addr: default_bind_addr(),
            bind_port: default_bind_port(),
            remote_host: default_remote_host(),
            remote_port: default_remote_port(),
            contact_host: default_contact_host(),
            from_domain: default_from_domain(),
            local_user: default_local_user(),
            auth_user: default_auth_user(),
            password: String::new(),
            realm: default_realm(),
            options_interval_secs: default_options_interval_secs(),
            inbound_setup_timeout_secs: default_inbound_setup_timeout_secs(),
            extra: HashMap::new(),
        }
    }
}

// Was: Führt den Arbeitsschritt `default_outbound_prefix` für default outbound prefix aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_outbound_prefix() -> String {
    "91".to_string()
}

// Was: Führt den Arbeitsschritt `default_strip_outbound_prefix` für default strip outbound prefix aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_strip_outbound_prefix() -> bool {
    true
}

// Was: Führt den Arbeitsschritt `default_inbound_prefix` für default inbound prefix aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_inbound_prefix() -> String {
    "T".to_string()
}

// Was: Führt den Arbeitsschritt `default_register` für default register aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_register() -> bool {
    true
}

// Was: Führt den Arbeitsschritt `default_codec` für default codec aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_codec() -> String {
    "PCMU".to_string()
}

// Was: Führt den Arbeitsschritt `default_rtp_port_min` für default rtp port min aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_rtp_port_min() -> u16 {
    30000
}

// Was: Führt den Arbeitsschritt `default_rtp_port_max` für default rtp port max aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_rtp_port_max() -> u16 {
    30100
}

// Was: Führt den Arbeitsschritt `default_bind_addr` für default bind addr aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_bind_addr() -> String {
    "0.0.0.0".to_string()
}

// Was: Führt den Arbeitsschritt `default_bind_port` für default bind port aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_bind_port() -> u16 {
    5062
}

// Was: Führt den Arbeitsschritt `default_remote_host` für default remote host aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_remote_host() -> String {
    "127.0.0.1".to_string()
}

// Was: Führt den Arbeitsschritt `default_remote_port` für default remote port aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_remote_port() -> u16 {
    5060
}

// Was: Führt den Arbeitsschritt `default_contact_host` für default contact host aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_contact_host() -> String {
    "127.0.0.1".to_string()
}

// Was: Führt den Arbeitsschritt `default_from_domain` für default from domain aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_from_domain() -> String {
    "127.0.0.1".to_string()
}

// Was: Führt den Arbeitsschritt `default_local_user` für default local Benutzer aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_local_user() -> String {
    "flowstation".to_string()
}

// Was: Führt den Arbeitsschritt `default_auth_user` für default Anmeldung und Berechtigung Benutzer aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_auth_user() -> String {
    "flowstation".to_string()
}

// Was: Führt den Arbeitsschritt `default_realm` für default realm aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_realm() -> String {
    "asterisk".to_string()
}

// Was: Führt den Arbeitsschritt `default_options_interval_secs` für default options interval secs aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_options_interval_secs() -> u64 {
    30
}

// Was: Führt den Arbeitsschritt `default_inbound_setup_timeout_secs` für default inbound setup timeout secs aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_inbound_setup_timeout_secs() -> u32 {
    20
}

// Was: Diese Funktion wendet asterisk patch.
// Warum: Die Änderung wird dadurch nur über einen definierten und prüfbaren Weg wirksam.
pub fn apply_asterisk_patch(src: CfgAsteriskDto) -> Result<CfgAsterisk, String> {
    if src.enabled {
        if src.bind_port == 0 {
            return Err("asterisk: bind_port cannot be 0".to_string());
        }
        if src.remote_port == 0 {
            return Err("asterisk: remote_port cannot be 0".to_string());
        }
        if src.rtp_port_min == 0 || src.rtp_port_max == 0 || src.rtp_port_min > src.rtp_port_max {
            return Err("asterisk: rtp_port_min/rtp_port_max must define a valid non-zero range".to_string());
        }
        if src.remote_host.trim().is_empty() {
            return Err("asterisk: remote_host cannot be empty when enabled".to_string());
        }
        if src.contact_host.trim().is_empty() {
            return Err("asterisk: contact_host cannot be empty when enabled".to_string());
        }
        if src.local_user.trim().is_empty() {
            return Err("asterisk: local_user cannot be empty when enabled".to_string());
        }
        if src.auth_user.trim().is_empty() {
            return Err("asterisk: auth_user cannot be empty when enabled".to_string());
        }
    }

    let codec = src.codec.trim().to_ascii_uppercase();
    if codec != "PCMU" {
        return Err("asterisk: only codec = \"PCMU\" is currently supported".to_string());
    }

    let service_numbers = src
        .service_numbers
        .into_iter()
        .map(|n| n.trim().to_string())
        .filter(|n| !n.is_empty())
        .collect();

    Ok(CfgAsterisk {
        enabled: src.enabled,
        outbound_prefix: src.outbound_prefix,
        strip_outbound_prefix: src.strip_outbound_prefix,
        inbound_prefix: src.inbound_prefix,
        register: src.register,
        codec,
        service_numbers,
        rtp_port_min: src.rtp_port_min,
        rtp_port_max: src.rtp_port_max,
        bind_addr: src.bind_addr,
        bind_port: src.bind_port,
        remote_host: src.remote_host,
        remote_port: src.remote_port,
        contact_host: src.contact_host,
        from_domain: src.from_domain,
        local_user: src.local_user,
        auth_user: src.auth_user,
        password: SecretField::from(src.password),
        realm: src.realm,
        options_interval_secs: src.options_interval_secs,
        inbound_setup_timeout_secs: src.inbound_setup_timeout_secs.clamp(1, 60),
    })
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    // Was: Führt den Arbeitsschritt `enabled_cfg` für enabled cfg aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn enabled_cfg(service_numbers: Vec<&str>) -> CfgAsterisk {
        let dto = CfgAsteriskDto {
            enabled: true,
            service_numbers: service_numbers.into_iter().map(str::to_string).collect(),
            ..CfgAsteriskDto::default()
        };
        apply_asterisk_patch(dto).expect("test asterisk config should be valid")
    }

    #[test]
    // Was: Führt den Arbeitsschritt `exact_service_numbers_still_allow_direct_and_prefixed_dials` für exact Dienst numbers still allow direct and und weitere Angaben aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn exact_service_numbers_still_allow_direct_and_prefixed_dials() {
        let cfg = enabled_cfg(vec!["385"]);
        assert_eq!(cfg.route_outbound_raw("91385"), Some("385".to_string()));
        assert_eq!(cfg.route_outbound_raw("385"), Some("385".to_string()));
        assert_eq!(cfg.route_outbound_raw("91600"), None);
    }

    #[test]
    // Was: Führt den Arbeitsschritt `star_service_number_routes_everything_behind_prefix_only` für star Dienst number routes everything behind prefix und weitere Angaben aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn star_service_number_routes_everything_behind_prefix_only() {
        let cfg = enabled_cfg(vec!["*"]);
        assert_eq!(cfg.route_outbound_raw("91385"), Some("385".to_string()));
        assert_eq!(cfg.route_outbound_raw("91600"), Some("600".to_string()));
        assert_eq!(cfg.route_outbound_raw("385"), None);
    }

    #[test]
    // Was: Führt den Arbeitsschritt `outbound_prefix_star_routes_everything_behind_prefix` für outbound prefix star routes everything behind prefix aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn outbound_prefix_star_routes_everything_behind_prefix() {
        let dto = CfgAsteriskDto {
            enabled: true,
            outbound_prefix: "91*".to_string(),
            service_numbers: vec!["385".to_string()],
            ..CfgAsteriskDto::default()
        };
        let cfg = apply_asterisk_patch(dto).expect("test asterisk config should be valid");
        assert_eq!(cfg.route_outbound_raw("91385"), Some("385".to_string()));
        assert_eq!(cfg.route_outbound_raw("91600"), Some("600".to_string()));
        assert_eq!(cfg.route_outbound_raw("385"), Some("385".to_string()));
    }

    #[test]
    // Was: Führt den Arbeitsschritt `service_number_prefix_wildcard_matches_stripped_number` für Dienst number prefix wildcard matches stripped number aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn service_number_prefix_wildcard_matches_stripped_number() {
        let cfg = enabled_cfg(vec!["38*"]);
        assert_eq!(cfg.route_outbound_raw("91385"), Some("385".to_string()));
        assert_eq!(cfg.route_outbound_raw("91600"), None);
    }
}
