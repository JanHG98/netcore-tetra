// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für gemeinsame Dienstfunktionen.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Process-level helpers shared by independently deployable NetCore services.

use netcore_contracts::{ApiVersion, BuildInfo, OperatingMode, SecurityMode, ServiceCapability, ServiceDescriptor};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für management policy in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ManagementPolicy {
    pub security_mode: SecurityMode,
    pub token_auth: bool,
    pub tls: bool,
    pub warning: String,
}

// Was: Implementiert das zugehörige Verhalten für `ManagementPolicy`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl ManagementPolicy {
    // Was: Diese Funktion öffnet lab.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn open_lab(warning: impl Into<String>) -> Self {
        Self {
            security_mode: SecurityMode::OpenLab,
            token_auth: false,
            tls: false,
            warning: warning.into(),
        }
    }

    // Was: Diese Funktion prüft den vorgesehenen Arbeitsschritt.
    // Warum: Unzulässige Werte werden dadurch erkannt, bevor sie im Betrieb Schaden anrichten.
    pub fn validate(&self) -> Result<(), &'static str> {
        if self.security_mode == SecurityMode::OpenLab && (self.token_auth || self.tls) {
            return Err("open_lab policy must not pretend token or TLS enforcement is active");
        }
        if self.security_mode != SecurityMode::OpenLab && !self.token_auth && !self.tls {
            return Err("non-lab policy requires at least one management protection mechanism");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für Dienst identity in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ServiceIdentity {
    pub name: String,
    pub instance: String,
    pub api_base: String,
    pub operating_mode: OperatingMode,
    pub management: ManagementPolicy,
    pub capabilities: Vec<ServiceCapability>,
}

// Was: Implementiert das zugehörige Verhalten für `ServiceIdentity`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl ServiceIdentity {
    // Was: Führt den Arbeitsschritt `descriptor` für descriptor aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn descriptor(&self) -> ServiceDescriptor {
        ServiceDescriptor {
            name: self.name.clone(),
            instance: self.instance.clone(),
            service_version: env!("CARGO_PKG_VERSION").to_owned(),
            contract_version: ApiVersion::V1.to_owned(),
            security_mode: self.management.security_mode,
            operating_mode: self.operating_mode,
            api_base: self.api_base.clone(),
            health_live: "/health/live".to_owned(),
            health_ready: "/health/ready".to_owned(),
            metrics: "/metrics".to_owned(),
            capabilities: self.capabilities.clone(),
        }
    }
}

// Was: Diese Funktion erstellt info.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_info(service: impl Into<String>) -> BuildInfo {
    BuildInfo {
        service: service.into(),
        version: env!("CARGO_PKG_VERSION").to_owned(),
        git_commit: option_env!("NETCORE_GIT_COMMIT").map(str::to_owned),
        build_timestamp: option_env!("NETCORE_BUILD_TIMESTAMP").map(str::to_owned),
        contract_version: ApiVersion::V1.to_owned(),
    }
}

// Was: Diese Funktion fordert Kennung.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn request_id(provided: Option<&str>) -> String {
    provided
        .filter(|value| is_safe_request_id(value))
        .map(str::to_owned)
        .unwrap_or_else(|| Uuid::new_v4().to_string())
}

// Was: Prüft, ob safe request Kennung zutrifft.
// Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
fn is_safe_request_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b':'))
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Führt den Arbeitsschritt `rejects_fake_open_lab_security_flags` für rejects fake open lab Sicherheit flags aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn rejects_fake_open_lab_security_flags() {
        let policy = ManagementPolicy {
            security_mode: SecurityMode::OpenLab,
            token_auth: true,
            tls: false,
            warning: String::new(),
        };
        assert!(policy.validate().is_err());
    }

    #[test]
    // Was: Führt den Arbeitsschritt `preserves_safe_request_id_only` für preserves safe request Kennung only aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn preserves_safe_request_id_only() {
        assert_eq!(request_id(Some("abc-123")), "abc-123");
        assert_ne!(request_id(Some("bad request id")), "bad request id");
    }
}
