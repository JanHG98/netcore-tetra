// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für gespeicherte Aufzeichnungen, TTS- und Mediendateien.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::fs;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

// Was: Legt den festen Wert `OPEN_LAB_MODE` für open lab mode fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const OPEN_LAB_MODE: &str = "open_lab";
// Was: Legt den festen Wert `SHADOW_MODE` für shadow mode fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SHADOW_MODE: &str = "shadow";
// Was: Legt den festen Wert `AUTHORITATIVE_MODE` für authoritative mode fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const AUTHORITATIVE_MODE: &str = "authoritative";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für Audio- und Mediendaten library Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct MediaLibraryConfig {
    pub server: ServerConfig,
    pub security: SecurityConfig,
    pub storage: StorageConfig,
    pub runtime: RuntimeConfig,
    pub codec: CodecConfig,
    pub dependencies: DependencyConfig,
}

// Was: Implementiert das zugehörige Verhalten für `Default for MediaLibraryConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for MediaLibraryConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            security: SecurityConfig::default(),
            storage: StorageConfig::default(),
            runtime: RuntimeConfig::default(),
            codec: CodecConfig::default(),
            dependencies: DependencyConfig::default(),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `MediaLibraryConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MediaLibraryConfig {
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
                "unsupported security.mode={}; this package intentionally implements open_lab management only",
                self.security.mode
            ));
        }
        if self.security.token_auth || self.security.tls {
            return Err("token_auth and tls must remain false in the current open_lab package".into());
        }
        if !self.security.allow_remote_management && !self.server.bind.ip().is_loopback() {
            return Err("server.bind must be loopback when allow_remote_management=false".into());
        }
        if !matches!(self.runtime.operating_mode.as_str(), SHADOW_MODE | AUTHORITATIVE_MODE) {
            return Err("runtime.operating_mode must be shadow or authoritative".into());
        }
        if !self.server.public_base_url.starts_with("http://") {
            return Err("server.public_base_url must use http:// in the current open-lab package".into());
        }
        self.server.public_base_url = self.server.public_base_url.trim_end_matches('/').to_string();
        self.server.max_body_bytes = self.server.max_body_bytes.max(1_048_576);
        self.storage.max_asset_bytes = self.storage.max_asset_bytes.max(65_536);
        self.storage.max_total_bytes = self.storage.max_total_bytes.max(self.storage.max_asset_bytes);
        self.runtime.worker_interval_ms = self.runtime.worker_interval_ms.max(100);
        self.runtime.probe_interval_secs = self.runtime.probe_interval_secs.max(5);
        self.runtime.import_timeout_secs = self.runtime.import_timeout_secs.max(2);
        self.runtime.max_assets = self.runtime.max_assets.max(50);
        self.runtime.max_jobs = self.runtime.max_jobs.max(20);
        self.runtime.max_events = self.runtime.max_events.max(100);
        self.runtime.max_audit_records = self.runtime.max_audit_records.max(100);
        self.runtime.max_attempts = self.runtime.max_attempts.max(1);
        if self.runtime.frame_interval_ms != 60 {
            return Err("runtime.frame_interval_ms must remain 60 for packed TETRA speech frames".into());
        }
        self.codec.ffmpeg_command = clean_command(&self.codec.ffmpeg_command);
        self.codec.encoder_command = clean_command(&self.codec.encoder_command);
        self.codec.decoder_command = clean_command(&self.codec.decoder_command);
        if self.codec.frame_bytes != 35 {
            return Err("codec.frame_bytes must remain 35 for packed TETRA speech service 0".into());
        }
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for url in [
            &mut self.dependencies.media_switch_base_url,
            &mut self.dependencies.recorder_base_url,
            &mut self.dependencies.application_gateway_base_url,
        ] {
            *url = url.trim().trim_end_matches('/').to_string();
            if !url.starts_with("http://") && !url.starts_with("https://") {
                return Err(format!("dependency URL must use http:// or https://: {url}"));
            }
        }
        Ok(())
    }
}

// Was: Führt den Arbeitsschritt `clean_command` für clean command aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn clean_command(command: &[String]) -> Vec<String> {
    command
        .iter()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für server Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ServerConfig {
    pub bind: SocketAddr,
    pub public_base_url: String,
    pub max_body_bytes: usize,
}

// Was: Implementiert das zugehörige Verhalten für `Default for ServerConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for ServerConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            bind: "0.0.0.0:8230".parse().expect("valid media-library bind"),
            public_base_url: "http://127.0.0.1:8230".to_string(),
            max_body_bytes: 96 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für Sicherheit Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct SecurityConfig {
    pub mode: String,
    pub token_auth: bool,
    pub tls: bool,
    pub allow_remote_management: bool,
    pub allow_delete: bool,
    pub allow_url_import: bool,
    pub allow_private_import_urls: bool,
}

// Was: Implementiert das zugehörige Verhalten für `Default for SecurityConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for SecurityConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            mode: OPEN_LAB_MODE.to_string(),
            token_auth: false,
            tls: false,
            allow_remote_management: true,
            allow_delete: true,
            allow_url_import: true,
            allow_private_import_urls: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für storage Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct StorageConfig {
    pub root: PathBuf,
    pub state_file: PathBuf,
    pub temp_root: PathBuf,
    pub backup_root: PathBuf,
    pub archive_root: Option<PathBuf>,
    pub max_asset_bytes: u64,
    pub max_total_bytes: u64,
    pub fsync_imports: bool,
}

// Was: Implementiert das zugehörige Verhalten für `Default for StorageConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for StorageConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            root: PathBuf::from("/var/lib/netcore-media-library/assets"),
            state_file: PathBuf::from("/var/lib/netcore-media-library/state.json"),
            temp_root: PathBuf::from("/var/lib/netcore-media-library/tmp"),
            backup_root: PathBuf::from("/var/lib/netcore-media-library/backups"),
            archive_root: Some(PathBuf::from("/mnt/nfs-share/Media-Library")),
            max_asset_bytes: 64 * 1024 * 1024,
            max_total_bytes: 20 * 1024 * 1024 * 1024,
            fsync_imports: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für Laufzeit Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RuntimeConfig {
    pub operating_mode: String,
    pub worker_interval_ms: u64,
    pub probe_interval_secs: u64,
    pub import_timeout_secs: u64,
    pub max_assets: usize,
    pub max_jobs: usize,
    pub max_events: usize,
    pub max_audit_records: usize,
    pub max_attempts: u32,
    pub frame_interval_ms: u64,
    pub auto_approve_tts: bool,
}

// Was: Implementiert das zugehörige Verhalten für `Default for RuntimeConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for RuntimeConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            operating_mode: SHADOW_MODE.to_string(),
            worker_interval_ms: 500,
            probe_interval_secs: 15,
            import_timeout_secs: 20,
            max_assets: 10_000,
            max_jobs: 2_000,
            max_events: 5_000,
            max_audit_records: 10_000,
            max_attempts: 3,
            frame_interval_ms: 60,
            auto_approve_tts: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für codec Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CodecConfig {
    pub frame_bytes: usize,
    pub ffmpeg_command: Vec<String>,
    pub encoder_command: Vec<String>,
    pub decoder_command: Vec<String>,
}

// Was: Implementiert das zugehörige Verhalten für `Default for CodecConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for CodecConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            frame_bytes: 35,
            ffmpeg_command: vec![
                "/usr/bin/ffmpeg".to_string(),
                "-hide_banner".to_string(),
                "-loglevel".to_string(),
                "error".to_string(),
                "-y".to_string(),
                "-i".to_string(),
                "{input}".to_string(),
                "-ac".to_string(),
                "1".to_string(),
                "-ar".to_string(),
                "8000".to_string(),
                "-c:a".to_string(),
                "pcm_s16le".to_string(),
                "{output}".to_string(),
            ],
            encoder_command: Vec::new(),
            decoder_command: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
// Was: Bündelt die zusammengehörigen Werte für dependency Konfiguration in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct DependencyConfig {
    pub media_switch_base_url: String,
    pub recorder_base_url: String,
    pub application_gateway_base_url: String,
}

// Was: Implementiert das zugehörige Verhalten für `Default for DependencyConfig`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for DependencyConfig {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self {
            media_switch_base_url: "http://127.0.0.1:8130".to_string(),
            recorder_base_url: "http://127.0.0.1:8140".to_string(),
            application_gateway_base_url: "http://127.0.0.1:8220".to_string(),
        }
    }
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Führt den Arbeitsschritt `default_is_open_lab_shadow_on_port_8230` für default is open lab shadow on port und weitere Angaben aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default_is_open_lab_shadow_on_port_8230() {
        let mut config = MediaLibraryConfig::default();
        config.normalise().expect("default configuration");
        assert_eq!(config.security.mode, OPEN_LAB_MODE);
        assert!(!config.security.token_auth);
        assert!(!config.security.tls);
        assert_eq!(config.runtime.operating_mode, SHADOW_MODE);
        assert_eq!(config.server.bind.port(), 8230);
        assert_eq!(config.runtime.frame_interval_ms, 60);
    }

    #[test]
    // Was: Führt den Arbeitsschritt `non_tetra_frame_timing_is_rejected` für non TETRA Funkrahmen timing is rejected aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn non_tetra_frame_timing_is_rejected() {
        let mut config = MediaLibraryConfig::default();
        config.runtime.frame_interval_ms = 20;
        assert!(config.normalise().is_err());
    }

    #[test]
    // Was: Führt den Arbeitsschritt `loopback_management_remains_valid` für loopback management remains valid aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn loopback_management_remains_valid() {
        let mut config = MediaLibraryConfig::default();
        config.security.allow_remote_management = false;
        config.server.bind = "127.0.0.1:8230".parse().unwrap();
        config.normalise().expect("loopback-only management");
    }
}
