// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Einlesen und Prüfen der TETRA-Konfiguration.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::{HashMap, HashSet};

use serde::Deserialize;
use toml::Value;

/// Local autonomy policy used when the base station cannot reach the NetCore
/// backend services.  The feature is intentionally independent of Internet
/// access: only the configured Node Gateway and service-health matrix matter.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für cfg edge fallback in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgEdgeFallback {
    /// Enable automatic online/degraded/isolated/recovering transitions.
    pub enabled: bool,
    /// How long a required service may be unavailable before the node enters
    /// fully isolated mode.  Before this timer expires the state is degraded.
    pub enter_after_secs: u64,
    /// Continuous healthy time required before central routing is re-enabled.
    pub recover_after_secs: u64,
    /// Treat a service that has not yet reported health as unavailable.
    pub unknown_service_is_available: bool,
    /// Maximum time without a fresh complete Node-Gateway health matrix before
    /// the TBS treats the service plane as unknown and enters fallback. This
    /// protects against a wedged monitor thread while the WebSocket stays open.
    pub service_matrix_lease_secs: u64,
    /// Persisted last-known subscriber/group policy cache.
    pub policy_cache_path: String,
    /// Maximum cache age used for diagnostics.  By default the secure
    /// last-known policy is still retained after this age instead of silently
    /// opening the network.
    pub policy_cache_max_age_secs: u64,
    /// Keep the last-known central admission/group policy when stale.
    pub keep_last_known_policy: bool,
    /// Durable control-plane telemetry/SDS spool for replay after reconnect.
    pub event_spool_path: String,
    pub event_spool_max_entries: usize,
    pub event_spool_max_bytes: usize,
    pub replay_batch_size: usize,
    /// Services that must be healthy before the cell advertises system-wide
    /// service availability and central routing becomes authoritative again.
    pub required_services: Vec<String>,
    /// Human-readable service-specific fallback behaviour.  It is also exposed
    /// in diagnostics so operators can see what remains available offline.
    pub service_fallbacks: HashMap<String, String>,
}

#[derive(Debug, Default, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für cfg edge fallback dto in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgEdgeFallbackDto {
    pub enabled: Option<bool>,
    pub enter_after_secs: Option<u64>,
    pub recover_after_secs: Option<u64>,
    pub unknown_service_is_available: Option<bool>,
    pub service_matrix_lease_secs: Option<u64>,
    pub policy_cache_path: Option<String>,
    pub policy_cache_max_age_secs: Option<u64>,
    pub keep_last_known_policy: Option<bool>,
    pub event_spool_path: Option<String>,
    pub event_spool_max_entries: Option<usize>,
    pub event_spool_max_bytes: Option<usize>,
    pub replay_batch_size: Option<usize>,
    pub required_services: Option<Vec<String>>,
    pub service_fallbacks: Option<HashMap<String, String>>,

    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// Was: Implementiert das zugehörige Verhalten für `Default for CfgEdgeFallback`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for CfgEdgeFallback {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            enabled: true,
            enter_after_secs: 15,
            recover_after_secs: 20,
            unknown_service_is_available: false,
            service_matrix_lease_secs: 60,
            policy_cache_path: "/var/lib/flowstation/edge-policy-cache.json".to_string(),
            policy_cache_max_age_secs: 7 * 24 * 60 * 60,
            keep_last_known_policy: true,
            event_spool_path: "/var/lib/flowstation/edge-event-spool.jsonl".to_string(),
            event_spool_max_entries: 10_000,
            event_spool_max_bytes: 16 * 1024 * 1024,
            replay_batch_size: 128,
            required_services: vec![
                "subscriber-core".to_string(),
                "group-core".to_string(),
                "mobility-core".to_string(),
                "call-control".to_string(),
                "media-switch".to_string(),
                "sds-router".to_string(),
            ],
            service_fallbacks: default_service_fallbacks(),
        }
    }
}

// Was: Diese Funktion wendet edge fallback patch.
// Warum: Die Änderung wird dadurch nur über einen definierten und prüfbaren Weg wirksam.
pub fn apply_edge_fallback_patch(src: CfgEdgeFallbackDto) -> Result<CfgEdgeFallback, String> {
    if !src.extra.is_empty() {
        let mut keys: Vec<_> = src.extra.keys().cloned().collect();
        keys.sort();
        return Err(format!("Unrecognized fields in edge_fallback config: {keys:?}"));
    }

    let defaults = CfgEdgeFallback::default();
    let mut required_services = src.required_services.unwrap_or(defaults.required_services);
    required_services.retain(|name| !name.trim().is_empty());
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for name in &mut required_services {
        *name = name.trim().to_ascii_lowercase();
    }
    required_services.sort();
    required_services.dedup();
    if required_services.iter().any(|name| !safe_service_name(name)) {
        return Err("edge_fallback.required_services contains an invalid service name".to_string());
    }

    let mut service_fallbacks = src.service_fallbacks.unwrap_or(defaults.service_fallbacks);
    let mut normalized = HashMap::new();
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for (name, mode) in service_fallbacks.drain() {
        let name = name.trim().to_ascii_lowercase();
        let mode = mode.trim().to_string();
        if !safe_service_name(&name) || mode.is_empty() {
            return Err("edge_fallback.service_fallbacks contains an invalid entry".to_string());
        }
        normalized.insert(name, mode);
    }

    let required_set: HashSet<_> = required_services.iter().cloned().collect();
    if required_set.iter().any(|name| !normalized.contains_key(name)) {
        return Err("every edge_fallback.required_services entry needs a service_fallbacks mode".to_string());
    }

    let policy_cache_path = non_empty(
        src.policy_cache_path.unwrap_or(defaults.policy_cache_path),
        "edge_fallback.policy_cache_path",
    )?;
    let event_spool_path = non_empty(
        src.event_spool_path.unwrap_or(defaults.event_spool_path),
        "edge_fallback.event_spool_path",
    )?;

    Ok(CfgEdgeFallback {
        enabled: src.enabled.unwrap_or(defaults.enabled),
        enter_after_secs: src.enter_after_secs.unwrap_or(defaults.enter_after_secs).max(1),
        recover_after_secs: src.recover_after_secs.unwrap_or(defaults.recover_after_secs).max(1),
        unknown_service_is_available: src
            .unknown_service_is_available
            .unwrap_or(defaults.unknown_service_is_available),
        service_matrix_lease_secs: src
            .service_matrix_lease_secs
            .unwrap_or(defaults.service_matrix_lease_secs)
            .clamp(5, 3_600),
        policy_cache_path,
        policy_cache_max_age_secs: src
            .policy_cache_max_age_secs
            .unwrap_or(defaults.policy_cache_max_age_secs)
            .max(60),
        keep_last_known_policy: src.keep_last_known_policy.unwrap_or(defaults.keep_last_known_policy),
        event_spool_path,
        event_spool_max_entries: src
            .event_spool_max_entries
            .unwrap_or(defaults.event_spool_max_entries)
            .clamp(100, 1_000_000),
        event_spool_max_bytes: src
            .event_spool_max_bytes
            .unwrap_or(defaults.event_spool_max_bytes)
            .clamp(64 * 1024, 1024 * 1024 * 1024),
        replay_batch_size: src
            .replay_batch_size
            .unwrap_or(defaults.replay_batch_size)
            .clamp(1, 2_048),
        required_services,
        service_fallbacks: normalized,
    })
}

// Was: Führt den Arbeitsschritt `default_service_fallbacks` für default Dienst fallbacks aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_service_fallbacks() -> HashMap<String, String> {
    [
        ("node-gateway", "local_edge_autonomy"),
        ("subscriber-core", "cached_policy_then_static_config"),
        ("group-core", "cached_policy_then_local_affiliations"),
        ("mobility-core", "local_registration_and_location_area"),
        ("call-control", "local_cell_calls_only"),
        ("media-switch", "local_air_interface_media_only"),
        ("recorder", "local_recorder_continues"),
        ("sds-router", "local_delivery_and_durable_store_forward"),
        ("packet-core", "local_sndcp_contexts"),
        ("ip-gateway", "local_tun_gateway_when_configured"),
        ("security-core", "last_known_security_policy_no_downgrade"),
        ("kmf", "installed_keys_only_no_otar"),
        ("transit", "no_inter_region_routing"),
        ("control-room", "local_dashboard_and_audit"),
        ("observability", "local_logs_and_health_continue"),
        ("application-gateway", "local_integrations_only"),
        ("media-library", "local_media_cache_and_playout"),
    ]
    .into_iter()
    .map(|(name, mode)| (name.to_string(), mode.to_string()))
    .collect()
}

// Was: Führt den Arbeitsschritt `safe_service_name` für safe Dienst name aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn safe_service_name(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

// Was: Führt den Arbeitsschritt `non_empty` für non empty aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn non_empty(value: String, field: &str) -> Result<String, String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(format!("{field} must not be empty"))
    } else {
        Ok(trimmed.to_string())
    }
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Führt den Arbeitsschritt `defaults_cover_every_runtime_service` für defaults cover every Laufzeit Dienst aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn defaults_cover_every_runtime_service() {
        let cfg = CfgEdgeFallback::default();
        assert_eq!(cfg.service_fallbacks.len(), 17);
        assert!(cfg.required_services.iter().all(|name| cfg.service_fallbacks.contains_key(name)));
        assert!(!cfg.unknown_service_is_available);
        assert_eq!(cfg.service_matrix_lease_secs, 60);
    }

    #[test]
    // Was: Führt den Arbeitsschritt `rejects_required_service_without_fallback_mode` für rejects required Dienst without fallback mode aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn rejects_required_service_without_fallback_mode() {
        let dto: CfgEdgeFallbackDto = toml::from_str(
            r#"
            required_services = ["missing-core"]
            service_fallbacks = { subscriber-core = "cached" }
            "#,
        )
        .unwrap();
        assert!(apply_edge_fallback_patch(dto).is_err());
    }
}
