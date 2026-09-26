// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Einlesen und Prüfen der TETRA-Konfiguration.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::HashMap;

use serde::Deserialize;
use toml::Value;

/// Lite stack-health monitor configuration (`[health]`).
///
/// A background sampler rolls a few coarse domains (core-loop liveness, Brew backhaul, attached
/// radios, downlink/SDS congestion) into a periodic snapshot that feeds the dashboard tile and
/// the Telegram alerter. Optionally it also acts as a software watchdog that restarts the service
/// if the core loop stalls.
///
/// The monitor itself is harmless (observe-only) so it defaults ON, but the **restart watchdog**
/// defaults OFF — proactively rebooting the station is a deliberate operator choice, consistent
/// with the other opt-in, RF-/service-affecting capabilities in this stack.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für cfg health in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgHealth {
    /// Master on/off for the health monitor (snapshots + dashboard tile + Telegram alerts).
    pub enabled: bool,
    /// How often the sampler emits a snapshot, seconds. Clamped 1..=300.
    pub snapshot_interval_secs: u64,
    /// Software watchdog: restart the service if the core loop stalls. Default OFF.
    pub restart_on_core_stall: bool,
    /// Core loop is Critical if no TDMA tick for this long, seconds. Clamped 2..=600.
    pub core_stall_secs: u64,
    /// How long the core must stay stalled before a restart is requested, seconds. Clamped 1..=3600.
    pub restart_after_critical_secs: u64,
    /// Minimum spacing between restart requests (anti-reboot-loop), seconds. Clamped 10..=86400.
    pub restart_cooldown_secs: u64,
    /// Floor for the "radios attached but silent" Degraded signal, seconds. 0 = disabled.
    /// The EFFECTIVE window is `max(this, 1.5 * periodic_registration_secs)` (the T351
    /// re-registration interval), so a radio that is simply quiet between its periodic
    /// registrations is never flagged — e.g. with T351 = 24 h it is not "silent" until ~36 h.
    /// Clamped 0..=86400.
    pub radios_silent_secs: u64,
    /// Downlink queue depth at/above which Congestion is Degraded / Critical.
    pub dl_queue_degraded: u32,
    pub dl_queue_critical: u32,
    /// Live-SDS queue depth at/above which Congestion is Degraded / Critical.
    pub sds_queue_degraded: u32,
    pub sds_queue_critical: u32,
}

// Was: Implementiert das zugehörige Verhalten für `Default for CfgHealth`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for CfgHealth {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        CfgHealth {
            enabled: true,
            snapshot_interval_secs: 5,
            restart_on_core_stall: false,
            core_stall_secs: 10,
            restart_after_critical_secs: 30,
            restart_cooldown_secs: 600,
            radios_silent_secs: 900,
            dl_queue_degraded: 64,
            dl_queue_critical: 192,
            sds_queue_degraded: 32,
            sds_queue_critical: 128,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für cfg health dto in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct CfgHealthDto {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_snapshot_interval")]
    pub snapshot_interval_secs: u64,
    #[serde(default)]
    pub restart_on_core_stall: bool,
    #[serde(default = "default_core_stall")]
    pub core_stall_secs: u64,
    #[serde(default = "default_restart_after")]
    pub restart_after_critical_secs: u64,
    #[serde(default = "default_restart_cooldown")]
    pub restart_cooldown_secs: u64,
    #[serde(default = "default_radios_silent")]
    pub radios_silent_secs: u64,
    #[serde(default = "default_dl_degraded")]
    pub dl_queue_degraded: u32,
    #[serde(default = "default_dl_critical")]
    pub dl_queue_critical: u32,
    #[serde(default = "default_sds_degraded")]
    pub sds_queue_degraded: u32,
    #[serde(default = "default_sds_critical")]
    pub sds_queue_critical: u32,

    /// Captures any unrecognised key so parsing.rs can reject typos rather than silently
    /// leaving the feature mis-configured.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// `serde(default)` on a missing whole `[health]` table needs Default on the DTO; when the table
// is present but a field is missing, the per-field defaults above apply.
// Was: Implementiert das zugehörige Verhalten für `Default for CfgHealthDto`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for CfgHealthDto {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        CfgHealthDto {
            enabled: true,
            snapshot_interval_secs: default_snapshot_interval(),
            restart_on_core_stall: false,
            core_stall_secs: default_core_stall(),
            restart_after_critical_secs: default_restart_after(),
            restart_cooldown_secs: default_restart_cooldown(),
            radios_silent_secs: default_radios_silent(),
            dl_queue_degraded: default_dl_degraded(),
            dl_queue_critical: default_dl_critical(),
            sds_queue_degraded: default_sds_degraded(),
            sds_queue_critical: default_sds_critical(),
            extra: HashMap::new(),
        }
    }
}

// Was: Führt den Arbeitsschritt `default_true` für default true aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_true() -> bool {
    true
}
// Was: Führt den Arbeitsschritt `default_snapshot_interval` für default snapshot interval aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_snapshot_interval() -> u64 {
    5
}
// Was: Führt den Arbeitsschritt `default_core_stall` für default core stall aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_core_stall() -> u64 {
    10
}
// Was: Führt den Arbeitsschritt `default_restart_after` für default restart after aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_restart_after() -> u64 {
    30
}
// Was: Führt den Arbeitsschritt `default_restart_cooldown` für default restart cooldown aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_restart_cooldown() -> u64 {
    600
}
// Was: Führt den Arbeitsschritt `default_radios_silent` für default radios silent aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_radios_silent() -> u64 {
    900
}
// Was: Führt den Arbeitsschritt `default_dl_degraded` für default dl degraded aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_dl_degraded() -> u32 {
    64
}
// Was: Führt den Arbeitsschritt `default_dl_critical` für default dl critical aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_dl_critical() -> u32 {
    192
}
// Was: Führt den Arbeitsschritt `default_sds_degraded` für default TETRA-Kurznachricht (SDS) degraded aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_sds_degraded() -> u32 {
    32
}
// Was: Führt den Arbeitsschritt `default_sds_critical` für default TETRA-Kurznachricht (SDS) critical aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn default_sds_critical() -> u32 {
    128
}

// Was: Diese Funktion wendet health patch.
// Warum: Die Änderung wird dadurch nur über einen definierten und prüfbaren Weg wirksam.
pub fn apply_health_patch(dto: CfgHealthDto) -> CfgHealth {
    // Clamp everything so a bad TOML value can't wedge the monitor (house style — same as
    // periodic_registration_secs / recovery clamps).
    CfgHealth {
        enabled: dto.enabled,
        snapshot_interval_secs: dto.snapshot_interval_secs.clamp(1, 300),
        restart_on_core_stall: dto.restart_on_core_stall,
        core_stall_secs: dto.core_stall_secs.clamp(2, 600),
        restart_after_critical_secs: dto.restart_after_critical_secs.clamp(1, 3600),
        restart_cooldown_secs: dto.restart_cooldown_secs.clamp(10, 86_400),
        radios_silent_secs: dto.radios_silent_secs.min(86_400),
        dl_queue_degraded: dto.dl_queue_degraded.min(1_000_000),
        dl_queue_critical: dto.dl_queue_critical.min(1_000_000),
        sds_queue_degraded: dto.sds_queue_degraded.min(1_000_000),
        sds_queue_critical: dto.sds_queue_critical.min(1_000_000),
    }
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Führt den Arbeitsschritt `serde_defaults_apply_when_only_one_field_set` für serde defaults apply when only one field und weitere Angaben aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn serde_defaults_apply_when_only_one_field_set() {
        let dto: CfgHealthDto = toml::from_str("restart_on_core_stall = true").unwrap();
        let c = apply_health_patch(dto);
        assert!(c.enabled); // defaults on
        assert!(c.restart_on_core_stall);
        assert_eq!(c.snapshot_interval_secs, 5);
        assert_eq!(c.core_stall_secs, 10);
        assert_eq!(c.restart_cooldown_secs, 600);
        assert_eq!(c.dl_queue_critical, 192);
    }

    #[test]
    // Was: Führt den Arbeitsschritt `clamps_out_of_range` für clamps out of range aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn clamps_out_of_range() {
        let dto = CfgHealthDto {
            snapshot_interval_secs: 0,
            core_stall_secs: 1,
            restart_after_critical_secs: 0,
            restart_cooldown_secs: 0,
            ..Default::default()
        };
        let c = apply_health_patch(dto);
        assert_eq!(c.snapshot_interval_secs, 1);
        assert_eq!(c.core_stall_secs, 2);
        assert_eq!(c.restart_after_critical_secs, 1);
        assert_eq!(c.restart_cooldown_secs, 10);
    }
}
