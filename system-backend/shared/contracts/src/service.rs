// NETCORE-KOMMENTAR – Was: Enthält die Logik oder Einstellungen für Dienst.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für Sicherheit mode auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum SecurityMode {
    OpenLab,
    Authenticated,
    MutualTls,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für operating mode auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum OperatingMode {
    Shadow,
    Authoritative,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Dienst capability in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ServiceCapability {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub optional: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für Dienst descriptor in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ServiceDescriptor {
    pub name: String,
    pub instance: String,
    pub service_version: String,
    pub contract_version: String,
    pub security_mode: SecurityMode,
    pub operating_mode: OperatingMode,
    pub api_base: String,
    pub health_live: String,
    pub health_ready: String,
    pub metrics: String,
    #[serde(default)]
    pub capabilities: Vec<ServiceCapability>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für compatibility in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct Compatibility {
    pub compatible: bool,
    pub local_contract: String,
    pub remote_contract: String,
    #[serde(default)]
    pub missing_required_capabilities: Vec<String>,
    #[serde(default)]
    pub warnings: Vec<String>,
}

// Was: Implementiert das zugehörige Verhalten für `ServiceDescriptor`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl ServiceDescriptor {
    // Was: Führt den Arbeitsschritt `compatibility_with` für compatibility with aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn compatibility_with(&self, remote: &Self, required: &[&str]) -> Compatibility {
        let local_major = contract_major(&self.contract_version);
        let remote_major = contract_major(&remote.contract_version);
        let missing = required
            .iter()
            .filter(|required_name| {
                !remote.capabilities.iter().any(|capability| capability.name == **required_name)
            })
            .map(|value| (*value).to_owned())
            .collect::<Vec<_>>();
        let mut warnings = Vec::new();
        if self.security_mode != remote.security_mode {
            warnings.push("security_mode_mismatch".to_owned());
        }
        Compatibility {
            compatible: local_major.is_some() && local_major == remote_major && missing.is_empty(),
            local_contract: self.contract_version.clone(),
            remote_contract: remote.contract_version.clone(),
            missing_required_capabilities: missing,
            warnings,
        }
    }
}

// Was: Führt den Arbeitsschritt `contract_major` für contract major aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn contract_major(value: &str) -> Option<u64> {
    value.strip_prefix("netcore.v")?.split('.').next()?.parse().ok()
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    // Was: Führt den Arbeitsschritt `descriptor` für descriptor aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn descriptor(version: &str, capabilities: &[&str]) -> ServiceDescriptor {
        ServiceDescriptor {
            name: "test".into(),
            instance: "test-1".into(),
            service_version: "1.3.0".into(),
            contract_version: version.into(),
            security_mode: SecurityMode::OpenLab,
            operating_mode: OperatingMode::Shadow,
            api_base: "/api/v1".into(),
            health_live: "/health/live".into(),
            health_ready: "/health/ready".into(),
            metrics: "/metrics".into(),
            capabilities: capabilities
                .iter()
                .map(|name| ServiceCapability { name: (*name).into(), version: "1".into(), optional: false })
                .collect(),
        }
    }

    #[test]
    // Was: Führt den Arbeitsschritt `checks_major_version_and_capabilities` für checks major version and capabilities aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn checks_major_version_and_capabilities() {
        let local = descriptor("netcore.v1", &[]);
        let remote = descriptor("netcore.v1.2", &["calls"]);
        assert!(local.compatibility_with(&remote, &["calls"]).compatible);
        assert!(!local.compatibility_with(&remote, &["sds"]).compatible);
        assert!(!local.compatibility_with(&descriptor("netcore.v2", &["calls"]), &["calls"]).compatible);
    }
}
