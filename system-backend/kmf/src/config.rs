// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Schlüsselverwaltung und Schlüsselverteilung.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

// Was: Legt den festen Wert `OPEN_LAB_MODE` für open lab mode fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const OPEN_LAB_MODE: &str = "open_lab";
// Was: Legt den festen Wert `OPERATING_MODE_SHADOW` für operating mode shadow fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const OPERATING_MODE_SHADOW: &str = "shadow";
// Was: Legt den festen Wert `OPERATING_MODE_AUTHORITATIVE` für operating mode authoritative fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const OPERATING_MODE_AUTHORITATIVE: &str = "authoritative";
// Was: Legt den festen Wert `LAB_FILE_VAULT` für lab file vault fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const LAB_FILE_VAULT: &str = "lab_file_vault";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für Schlüsselverwaltung Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct KmfConfig {
    pub server: ServerConfig,
    pub storage: StorageConfig,
    pub policy: PolicyConfig,
    pub vault: VaultConfig,
    pub otar: OtarConfig,
    pub security: ManagementSecurityConfig,
    pub limits: LimitsConfig,
}

// Was: Implementiert das zugehörige Verhalten für `Default for KmfConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for KmfConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            storage: StorageConfig::default(),
            policy: PolicyConfig::default(),
            vault: VaultConfig::default(),
            otar: OtarConfig::default(),
            security: ManagementSecurityConfig::default(),
            limits: LimitsConfig::default(),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `KmfConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl KmfConfig {
    // Was: Diese Funktion lädt den vorgesehenen Arbeitsschritt.
    // Warum: Einlesen und Fehlerbehandlung bleiben dadurch an einer zentralen Stelle.
    pub fn load(path: Option<&Path>) -> Result<Self, Box<dyn std::error::Error>> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        let mut config = match path {
            Some(path) => toml::from_str::<Self>(&fs::read_to_string(path)?)?,
            None => Self::default(),
        };
        config
            .normalise()
            .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidInput, error))?;
        Ok(config)
    }

    // Was: Diese Funktion wendet bind override.
    // Warum: Die Änderung wird dadurch nur über einen definierten und prüfbaren Weg wirksam.
    pub fn apply_bind_override(&mut self, bind: Option<SocketAddr>) -> Result<(), String> {
        if let Some(bind) = bind {
            self.server.bind = bind;
        }
        self.normalise()
    }

    // Was: Führt den Arbeitsschritt `normalise` für normalise aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn normalise(&mut self) -> Result<(), String> {
        if self.security.mode != OPEN_LAB_MODE {
            return Err(format!(
                "unsupported security.mode={}; this package intentionally implements only open_lab management access",
                self.security.mode
            ));
        }
        if !matches!(
            self.policy.operating_mode.as_str(),
            OPERATING_MODE_SHADOW | OPERATING_MODE_AUTHORITATIVE
        ) {
            return Err("policy.operating_mode must be shadow or authoritative".to_string());
        }
        if !self.security.allow_remote_management && !self.server.bind.ip().is_loopback() {
            return Err(
                "server.bind must be loopback when security.allow_remote_management=false"
                    .to_string(),
            );
        }
        if self.security.expose_raw_keys {
            return Err("security.expose_raw_keys must remain false; raw-key management exposure is forbidden".to_string());
        }
        if self.vault.provider != LAB_FILE_VAULT {
            return Err(format!(
                "unsupported vault.provider={}; PKCS#11/HSM is intentionally a later provider",
                self.vault.provider
            ));
        }
        if self.vault.master_key_bytes != 32 {
            return Err("vault.master_key_bytes must be exactly 32 for lab_file_vault".to_string());
        }
        if self.policy.default_key_bytes < 8 || self.policy.default_key_bytes > 32 {
            return Err("policy.default_key_bytes must be between 8 and 32".to_string());
        }
        if self.policy.default_crypto_period_secs < 60 {
            return Err("policy.default_crypto_period_secs must be at least 60".to_string());
        }
        if self.otar.action_ttl_secs < 30 {
            return Err("otar.action_ttl_secs must be at least 30".to_string());
        }
        if self.otar.max_attempts == 0 {
            return Err("otar.max_attempts must be at least 1".to_string());
        }
        self.server.history_limit = self.server.history_limit.max(100);
        self.otar.max_claim_batch = self.otar.max_claim_batch.clamp(1, 1_000);
        self.otar.retry_backoff_secs = self.otar.retry_backoff_secs.max(1);
        self.limits.max_body_bytes = self.limits.max_body_bytes.max(4_096);
        self.limits.max_keys = self.limits.max_keys.max(8);
        self.limits.max_nodes = self.limits.max_nodes.max(1);
        self.limits.max_jobs = self.limits.max_jobs.max(8);
        self.limits.max_actions = self.limits.max_actions.max(32);
        self.limits.max_audit = self.limits.max_audit.max(100);
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für server Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ServerConfig {
    pub bind: SocketAddr,
    pub history_limit: usize,
}
// Was: Implementiert das zugehörige Verhalten für `Default for ServerConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for ServerConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            bind: "0.0.0.0:8190".parse().expect("valid default bind"),
            history_limit: 5_000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für storage Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct StorageConfig {
    pub database_path: PathBuf,
    pub vault_path: PathBuf,
    pub master_key_path: PathBuf,
    pub backup_dir: PathBuf,
    pub bootstrap_dir: PathBuf,
}
// Was: Implementiert das zugehörige Verhalten für `Default for StorageConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for StorageConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            database_path: "/var/lib/netcore-kmf/state.json".into(),
            vault_path: "/var/lib/netcore-kmf/vault.json".into(),
            master_key_path: "/var/lib/netcore-kmf/master.key".into(),
            backup_dir: "/var/lib/netcore-kmf/backups".into(),
            bootstrap_dir: "/var/lib/netcore-kmf/bootstrap".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für policy Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct PolicyConfig {
    pub operating_mode: String,
    pub default_key_bytes: usize,
    pub default_crypto_period_secs: u64,
    pub rotation_lead_secs: u64,
    pub require_dual_approval: bool,
    pub allow_overlapping_crypto_periods: bool,
    pub auto_retire_predecessor: bool,
}
// Was: Implementiert das zugehörige Verhalten für `Default for PolicyConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for PolicyConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            operating_mode: OPERATING_MODE_SHADOW.to_string(),
            default_key_bytes: 16,
            default_crypto_period_secs: 86_400,
            rotation_lead_secs: 3_600,
            require_dual_approval: true,
            allow_overlapping_crypto_periods: true,
            auto_retire_predecessor: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für vault Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct VaultConfig {
    pub provider: String,
    pub master_key_bytes: usize,
    pub fsync: bool,
    pub hsm_library: Option<PathBuf>,
    pub hsm_slot: Option<u64>,
}
// Was: Implementiert das zugehörige Verhalten für `Default for VaultConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for VaultConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            provider: LAB_FILE_VAULT.to_string(),
            master_key_bytes: 32,
            fsync: true,
            hsm_library: None,
            hsm_slot: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für otar Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct OtarConfig {
    pub action_ttl_secs: u64,
    pub max_attempts: u32,
    pub retry_backoff_secs: u64,
    pub max_claim_batch: usize,
}
// Was: Implementiert das zugehörige Verhalten für `Default for OtarConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for OtarConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            action_ttl_secs: 600,
            max_attempts: 5,
            retry_backoff_secs: 15,
            max_claim_batch: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für management Sicherheit Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ManagementSecurityConfig {
    pub mode: String,
    pub allow_remote_management: bool,
    pub expose_raw_keys: bool,
}
// Was: Implementiert das zugehörige Verhalten für `Default for ManagementSecurityConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for ManagementSecurityConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            mode: OPEN_LAB_MODE.to_string(),
            allow_remote_management: true,
            expose_raw_keys: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für limits Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LimitsConfig {
    pub max_body_bytes: usize,
    pub max_keys: usize,
    pub max_nodes: usize,
    pub max_jobs: usize,
    pub max_actions: usize,
    pub max_audit: usize,
}
// Was: Implementiert das zugehörige Verhalten für `Default for LimitsConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for LimitsConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            max_body_bytes: 1_048_576,
            max_keys: 100_000,
            max_nodes: 10_000,
            max_jobs: 100_000,
            max_actions: 500_000,
            max_audit: 100_000,
        }
    }
}
