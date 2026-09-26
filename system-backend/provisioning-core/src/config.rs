use std::fs;
use std::net::SocketAddr;
use std::path::Path;
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ProvisioningConfig {
    pub server: ServerConfig,
    pub upstream: UpstreamConfig,
    pub security: SecurityConfig,
    pub limits: LimitsConfig,
}

impl Default for ProvisioningConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            upstream: UpstreamConfig::default(),
            security: SecurityConfig::default(),
            limits: LimitsConfig::default(),
        }
    }
}

impl ProvisioningConfig {
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut config: Self = toml::from_str(&fs::read_to_string(path)?)?;
        config
            .normalise()
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidInput, error))?;
        Ok(config)
    }

    pub fn apply_bind_override(&mut self, bind: Option<SocketAddr>) -> Result<(), String> {
        if let Some(bind) = bind {
            self.server.bind = bind;
        }
        self.normalise()
    }

    fn normalise(&mut self) -> Result<(), String> {
        if self.security.mode != "open_lab" {
            return Err("only security.mode=open_lab is supported in this lab service".into());
        }
        if !self.security.allow_remote_management && !self.server.bind.ip().is_loopback() {
            return Err("server.bind must be loopback when allow_remote_management=false".into());
        }
        self.upstream.subscriber_core = normalise_base_url(&self.upstream.subscriber_core)?;
        self.upstream.group_core = normalise_base_url(&self.upstream.group_core)?;
        self.upstream.timeout_secs = self.upstream.timeout_secs.clamp(1, 60);
        self.limits.max_body_bytes = self.limits.max_body_bytes.max(1024);
        Ok(())
    }

    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.upstream.timeout_secs)
    }
}

fn normalise_base_url(value: &str) -> Result<String, String> {
    let trimmed = value.trim().trim_end_matches('/');
    if !trimmed.starts_with("http://") {
        return Err(format!("upstream URL must use http:// in OPEN LAB mode: {trimmed}"));
    }
    if trimmed.len() <= "http://".len() {
        return Err("upstream URL has no host".into());
    }
    Ok(trimmed.to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    pub bind: SocketAddr,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self { bind: "0.0.0.0:8125".parse().expect("valid default bind") }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UpstreamConfig {
    pub subscriber_core: String,
    pub group_core: String,
    pub timeout_secs: u64,
}

impl Default for UpstreamConfig {
    fn default() -> Self {
        Self {
            subscriber_core: "http://127.0.0.1:8100".into(),
            group_core: "http://127.0.0.1:8110".into(),
            timeout_secs: 5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SecurityConfig {
    pub mode: String,
    pub allow_remote_management: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self { mode: "open_lab".into(), allow_remote_management: true }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LimitsConfig {
    pub max_body_bytes: usize,
}

impl Default for LimitsConfig {
    fn default() -> Self {
        Self { max_body_bytes: 2 * 1024 * 1024 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalises_http_upstream_urls() {
        assert_eq!(normalise_base_url(" http://127.0.0.1:8100/ ").unwrap(), "http://127.0.0.1:8100");
        assert!(normalise_base_url("https://127.0.0.1:8100").is_err());
    }

    #[test]
    fn rejects_remote_bind_when_remote_management_is_disabled() {
        let mut config = ProvisioningConfig::default();
        config.security.allow_remote_management = false;
        assert!(config.normalise().is_err());
        config.server.bind = "127.0.0.1:8125".parse().unwrap();
        assert!(config.normalise().is_ok());
    }
}
