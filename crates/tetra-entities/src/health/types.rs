// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Data model for FlowStation's lite stack-health view.
//!
//! Coarse on purpose: a handful of domains, each at one of three levels, plus an `overall`
//! roll-up. These are surfaced to the dashboard and Telegram, so they derive the same
//! serialization traits as the rest of `TelemetryEvent`. FlowStation-original work.

use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

/// Health level for a domain (and the rolled-up overall). Ordered Ok < Degraded < Critical.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für health level auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum HealthLevel {
    Ok,
    Degraded,
    Critical,
}

// Was: Implementiert das zugehörige Verhalten für `HealthLevel`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl HealthLevel {
    /// The worse (higher) of two levels — used to roll the per-domain levels into `overall`.
    // Was: Führt den Arbeitsschritt `worst` für worst aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn worst(self, other: Self) -> Self {
        use HealthLevel::*;
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match (self, other) {
            (Critical, _) | (_, Critical) => Critical,
            (Degraded, _) | (_, Degraded) => Degraded,
            (Ok, Ok) => Ok,
        }
    }

    // Was: Wandelt den vorhandenen Wert in str um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn as_str(self) -> &'static str {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            HealthLevel::Ok => "ok",
            HealthLevel::Degraded => "degraded",
            HealthLevel::Critical => "critical",
        }
    }
}

/// The monitored domains. Lite set (4): the core loop, the Brew backhaul, attached radios, and
/// downlink/SDS congestion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Encode, Decode, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für health domain auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum HealthDomain {
    /// The TETRA core loop is ticking (liveness watchdog).
    Service,
    /// Brew/TetraPack backhaul link.
    Backhaul,
    /// Attached mobile radios.
    Radios,
    /// Downlink / SDS queue pressure.
    Congestion,
}

// Was: Implementiert das zugehörige Verhalten für `HealthDomain`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl HealthDomain {
    // Was: Wandelt den vorhandenen Wert in str um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn as_str(self) -> &'static str {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            HealthDomain::Service => "service",
            HealthDomain::Backhaul => "backhaul",
            HealthDomain::Radios => "radios",
            HealthDomain::Congestion => "congestion",
        }
    }
}

/// One domain's verdict plus a short human-readable detail line (shown in the dashboard tile).
#[derive(Debug, Clone, Encode, Decode, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für domain health in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct DomainHealth {
    pub domain: HealthDomain,
    pub level: HealthLevel,
    pub detail: String,
}

/// A point-in-time view of station health, emitted periodically through the telemetry channel.
#[derive(Debug, Clone, Encode, Decode, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für health snapshot in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct HealthSnapshot {
    /// Worst level across all domains.
    pub overall: HealthLevel,
    pub domains: Vec<DomainHealth>,
    /// The most recent remediation action taken (e.g. a service restart request), if any.
    pub last_action: Option<String>,
    /// Seconds since the health registry started (≈ process uptime).
    pub uptime_secs: u64,
}
