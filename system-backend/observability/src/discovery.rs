//! Read the existing controller API without making collection depend on the VM.
use std::collections::BTreeMap;
use std::net::Ipv4Addr;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::{collector::http_get, config::ObservabilityConfig, state::SharedObservability};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DiscoveryConfig {
    pub enabled: bool,
    pub controller_url: String,
    pub environment: String,
    pub interval_secs: u64,
    pub allowed_networks: Vec<String>,
}
impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self { enabled: false, controller_url: "http://10.0.20.35:8320".into(),
            environment: "netcore-openlab".into(), interval_secs: 30,
            allowed_networks: vec!["10.0.20.0/24".into()] }
    }
}

fn allowed_url(url: &str, networks: &[String]) -> bool {
    let Some(authority) = url.strip_prefix("http://") else { return false; };
    let Some((host, port)) = authority.split_once(':') else { return false; };
    let Ok(ip) = host.parse::<Ipv4Addr>() else { return false; };
    if !matches!(port.parse::<u16>(), Ok(1..=65535)) || ip.is_loopback() || ip.is_multicast() || ip.is_unspecified() { return false; }
    networks.iter().any(|network| {
        let Some((base, bits)) = network.split_once('/') else { return false; };
        let (Ok(base), Ok(bits)) = (base.parse::<Ipv4Addr>(), bits.parse::<u32>()) else { return false; };
        if bits > 32 { return false; }
        let mask = if bits == 0 { 0 } else { u32::MAX << (32 - bits) };
        u32::from(ip) & mask == u32::from(base) & mask
    })
}

fn resolve(config: &ObservabilityConfig, value: &Value) -> Result<BTreeMap<String, String>, String> {
    if value["protocol"] != "netcore.discovery.v1" || value["role"] != "controller"
        || value["security_mode"] != "open_lab" || value["environment"] != config.discovery.environment {
        return Err("Unexpected discovery controller or environment".into());
    }
    let endpoints = value["endpoints"].as_object().ok_or("Missing endpoints object")?;
    let conflicts = value["conflicts"].as_object().ok_or("Missing conflicts object")?;
    let mut result = BTreeMap::new();
    for target in &config.targets {
        if conflicts.contains_key(&target.service) { continue; }
        if let Some(url) = endpoints.get(&target.service).and_then(|e| e["url"].as_str()) {
            if !allowed_url(url, &config.discovery.allowed_networks) {
                return Err(format!("Invalid discovery address for {}", target.service));
            }
            result.insert(target.service.clone(), url.to_string());
        }
    }
    Ok(result)
}

pub fn sync(config: &ObservabilityConfig, state: &SharedObservability) -> Result<usize, String> {
    if !config.discovery.enabled { return Err("Discovery is disabled".into()); }
    let base = config.discovery.controller_url.trim_end_matches('/');
    if !allowed_url(base, &config.discovery.allowed_networks) { return Err("Controller URL is outside allowed_networks".into()); }
    let response = http_get(&format!("{base}/api/v1/status"), 3000, 1024 * 1024)?;
    if response.status != 200 { return Err(format!("Discovery HTTP {}", response.status)); }
    let value: Value = serde_json::from_slice(&response.body).map_err(|e| e.to_string())?;
    state.apply_discovery(&resolve(config, &value)?, &value["conflicts"])
}

pub fn spawn(config: ObservabilityConfig, state: SharedObservability) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        if !config.discovery.enabled { return; }
        loop {
            if let Err(error) = sync(&config, &state) { state.discovery_failed(error); }
            thread::sleep(Duration::from_secs(config.discovery.interval_secs.max(5)));
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn resolves_identity_not_port_and_skips_conflicts() {
        let cfg = ObservabilityConfig::default();
        let input = json!({"protocol":"netcore.discovery.v1", "role":"controller",
            "security_mode":"open_lab", "environment":"netcore-openlab",
            "endpoints":{"node-gateway":{"url":"http://10.0.20.10:8080"},
                "tbs":{"url":"http://10.0.20.40:8080"}, "call-control":{"url":"http://10.0.20.14:8120"}},
            "conflicts":{"call-control":["a","b"]}});
        let result = resolve(&cfg, &input).unwrap();
        assert_eq!(result["node-gateway"], "http://10.0.20.10:8080");
        assert!(!result.contains_key("call-control"));
    }
    #[test]
    fn rejects_external_addresses_and_wrong_environment() {
        let networks = vec!["10.0.20.0/24".into()];
        for url in ["https://10.0.20.1:80", "http://127.0.0.1:80", "http://10.0.2.1:80", "http://10.0.20.1:80/x", "http://10.0.20.1:0"] {
            assert!(!allowed_url(url, &networks));
        }
        assert!(resolve(&ObservabilityConfig::default(), &json!({})).is_err());
    }

    #[test]
    fn startup_preserves_custom_management_addresses_and_disabled_targets() {
        let root = std::env::temp_dir().join(format!("netcore-custom-discovery-{}", uuid::Uuid::new_v4()));
        let mut cfg = ObservabilityConfig::default();
        cfg.storage.state_path = root.join("state.json");
        cfg.targets.truncate(1);
        cfg.targets[0].base_url = "http://10.0.20.99:8080".into();
        cfg.targets[0].enabled = false;
        drop(SharedObservability::load(cfg.clone()).unwrap());
        cfg.discovery.enabled = true;
        cfg.targets[0].base_url = "http://10.0.20.10:8080".into();
        let state = SharedObservability::load(cfg).unwrap();
        let target = state.target("node-gateway").unwrap();
        assert_eq!(target.base_url, "http://10.0.20.99:8080");
        assert!(!target.enabled);
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn migrates_legacy_state_and_keeps_discovered_targets_across_restart() {
        let root = std::env::temp_dir().join(format!("netcore-discovery-{}", uuid::Uuid::new_v4()));
        let mut cfg = ObservabilityConfig::default();
        cfg.storage.state_path = root.join("state.json");
        cfg.targets.truncate(2);
        cfg.targets[1].labels.insert("discovery".into(), "manual".into());
        let initial = SharedObservability::load(cfg.clone()).unwrap();
        drop(initial);
        cfg.discovery.enabled = true;
        cfg.targets[0].base_url = "http://10.0.20.10:8080".into();
        let state = SharedObservability::load(cfg.clone()).unwrap();
        assert_eq!(state.target("node-gateway").unwrap().base_url, "http://10.0.20.10:8080");
        let manual = state.target("mobility-core").unwrap().base_url;
        state.record_scrape(crate::collector::ScrapeResult {
            target_id: "node-gateway".into(), base_url: "http://10.0.20.10:8080".into(),
            timestamp: chrono::Utc::now(), live: true, ready: true, metrics_ok: true,
            response_ms: 1.0, metrics: vec![], error: None,
        }).unwrap();
        let endpoints = BTreeMap::from([("node-gateway".into(), "http://10.0.20.199:8080".into()),
                                       ("mobility-core".into(), "http://10.0.20.200:8090".into())]);
        assert_eq!(state.apply_discovery(&endpoints, &json!({})).unwrap(), 1);
        let changed = state.target("node-gateway").unwrap();
        assert!(!changed.metrics_ok);
        assert_eq!(changed.last_success_at, None);
        state.record_scrape(crate::collector::ScrapeResult {
            target_id: "node-gateway".into(), base_url: "http://10.0.20.10:8080".into(),
            timestamp: chrono::Utc::now(), live: true, ready: true, metrics_ok: true,
            response_ms: 1.0, metrics: vec![], error: None,
        }).unwrap();
        assert!(!state.target("node-gateway").unwrap().live, "ignore stale result from the previous address");
        state.discovery_failed("VM offline".into());
        assert_eq!(state.discovery_status()["using_cache"], true);
        assert_eq!(state.target("mobility-core").unwrap().base_url, manual);
        drop(state);
        let restored = SharedObservability::load(cfg).unwrap();
        assert_eq!(restored.target("node-gateway").unwrap().base_url, "http://10.0.20.199:8080");
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn preview_retry_is_idempotent() {
        let root = std::env::temp_dir().join(format!("netcore-preview-{}", uuid::Uuid::new_v4()));
        let mut cfg = ObservabilityConfig::default();
        cfg.storage.state_path = root.join("state.json");
        let state = SharedObservability::load(cfg).unwrap();
        let batch = json!({"records":[{"service":"test", "message":"once", "fields":{"syslog_id":"retry-1"}}]});
        for _ in 0..2 {
            assert_eq!(state.ingest_logs(serde_json::from_value(batch.clone()).unwrap()).unwrap(), 1);
        }
        assert_eq!(state.logs(None, None, None, None, 100).len(), 1);
        std::fs::remove_dir_all(root).unwrap();
    }
}
