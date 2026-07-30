// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::process::{Command, Output};
use std::sync::Arc;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::time::Duration;

// Was: Legt den festen Wert `SERVICE_UNIT_ENV` für Dienst unit env fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const SERVICE_UNIT_ENV: &str = "FLOWSTATION_SERVICE_UNIT";
// Was: Legt den festen Wert `DEFAULT_SERVICE_UNIT` für default Dienst unit fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const DEFAULT_SERVICE_UNIT: &str = "tetra-bluestation.service";
// Was: Legt den festen Wert `NO_EXIT_REQUESTED` für no exit requested fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const NO_EXIT_REQUESTED: i32 = i32::MIN;
// Was: Legt den festen Wert `RESTART_EXIT_CODE` für restart exit code fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const RESTART_EXIT_CODE: i32 = 75;

// Was: Bündelt die zusammengehörigen Werte für lifecycle Steuerung in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct LifecycleControl {
    running: Arc<AtomicBool>,
    exit_code: AtomicI32,
}

// Was: Legt den festen Wert `LIFECYCLE_CONTROL` für lifecycle Steuerung fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
static LIFECYCLE_CONTROL: OnceLock<LifecycleControl> = OnceLock::new();

/// Service unit configured from the TOML config file (e.g. service_name = "tetra").
/// Takes precedence over cgroup auto-detection but is overridden by FLOWSTATION_SERVICE_UNIT env var.
// Was: Legt den festen Wert `CONFIGURED_SERVICE_UNIT` für configured Dienst unit fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
static CONFIGURED_SERVICE_UNIT: OnceLock<String> = OnceLock::new();

/// Set the service unit from config — should be called once at startup.
/// Subsequent calls are ignored (OnceLock).
// Was: Diese Funktion setzt configured Dienst unit.
// Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
pub fn set_configured_service_unit(unit: &str) {
    if let Some(normalized) = normalize_service_unit(unit) {
        let _ = CONFIGURED_SERVICE_UNIT.set(normalized);
    } else {
        tracing::warn!("Service control: ignoring invalid configured service_name={:?}", unit);
    }
}

#[derive(Debug, Clone, Copy)]
// Was: Listet die möglichen Varianten für Dienst action auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum ServiceAction {
    Restart,
    Stop,
}

// Was: Implementiert das zugehörige Verhalten für `ServiceAction`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl ServiceAction {
    // Was: Führt den Arbeitsschritt `systemctl_verb` für systemctl verb aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn systemctl_verb(self) -> &'static str {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            ServiceAction::Restart => "restart",
            ServiceAction::Stop => "stop",
        }
    }

    // Was: Führt den Arbeitsschritt `label` für label aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn label(self) -> &'static str {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            ServiceAction::Restart => "restart",
            ServiceAction::Stop => "shutdown",
        }
    }
}

// Was: Führt den Arbeitsschritt `install_lifecycle_control` für install lifecycle Steuerung aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn install_lifecycle_control(running: Arc<AtomicBool>) {
    let _ = LIFECYCLE_CONTROL.set(LifecycleControl {
        running,
        exit_code: AtomicI32::new(NO_EXIT_REQUESTED),
    });
}

// Was: Führt den Arbeitsschritt `requested_exit_code` für requested exit code aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn requested_exit_code() -> Option<i32> {
    let lifecycle = LIFECYCLE_CONTROL.get()?;
    let code = lifecycle.exit_code.load(Ordering::SeqCst);
    (code != NO_EXIT_REQUESTED).then_some(code)
}

// Was: Diese Funktion plant Dienst action.
// Warum: Zeitfenster und Ressourcen werden dadurch planbar statt zufällig vergeben.
pub fn schedule_service_action(action: ServiceAction, delay: Duration) {
    let unit = resolve_service_unit();
    let service_user = service_user(&unit).unwrap_or_else(|| "unknown".to_string());
    tracing::warn!(
        "Service control: scheduling {} for {} (unit User={}) in {:?}",
        action.label(),
        unit,
        service_user,
        delay
    );

    std::thread::Builder::new()
        .name("service-control".into())
        .spawn(move || {
            std::thread::sleep(delay);
            if let Some(lifecycle) = LIFECYCLE_CONTROL.get() {
                // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                let exit_code = match action {
                    ServiceAction::Restart => RESTART_EXIT_CODE,
                    ServiceAction::Stop => 0,
                };
                lifecycle.exit_code.store(exit_code, Ordering::SeqCst);
                lifecycle.running.store(false, Ordering::SeqCst);
                tracing::info!(
                    "Service control: {} requested internally for {} with exit code {}",
                    action.label(),
                    unit,
                    exit_code
                );
            } else {
                // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                match run_service_action(action, &unit) {
                    Ok(()) => tracing::info!("Service control: {} requested for {}", action.label(), unit),
                    Err(e) => tracing::error!("Service control: {} failed for {}: {}", action.label(), unit, e),
                }
            }
        })
        .ok();
}

// Was: Diese Funktion ermittelt Dienst unit.
// Warum: Unklare oder indirekte Angaben werden so vor der weiteren Verarbeitung eindeutig gemacht.
pub fn resolve_service_unit() -> String {
    if let Ok(value) = std::env::var(SERVICE_UNIT_ENV) {
        if let Some(unit) = normalize_service_unit(&value) {
            return unit;
        }
        tracing::warn!("Service control: ignoring invalid {}={:?}", SERVICE_UNIT_ENV, value);
    }

    if let Some(configured) = CONFIGURED_SERVICE_UNIT.get() {
        return configured.clone();
    }

    std::fs::read_to_string("/proc/self/cgroup")
        .ok()
        .and_then(|text| service_unit_from_cgroup_text(&text))
        .unwrap_or_else(|| DEFAULT_SERVICE_UNIT.to_string())
}

// Was: Diese Funktion führt Dienst action.
// Warum: Der Lebenszyklus des Dienstes bleibt so an einer zentralen Stelle steuerbar.
fn run_service_action(action: ServiceAction, unit: &str) -> Result<(), String> {
    let verb = action.systemctl_verb();
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match run_command("systemctl", &[verb, unit]) {
        Ok(()) => Ok(()),
        Err(systemctl_err) => match run_command("sudo", &["-n", "systemctl", verb, unit]) {
            Ok(()) => Ok(()),
            Err(sudo_err) => Err(format!("systemctl: {}; sudo -n: {}", systemctl_err, sudo_err)),
        },
    }
}

// Was: Diese Funktion führt command.
// Warum: Der Lebenszyklus des Dienstes bleibt so an einer zentralen Stelle steuerbar.
fn run_command(program: &str, args: &[&str]) -> Result<(), String> {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match Command::new(program).args(args).output() {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => Err(output_error(output)),
        Err(e) => Err(e.to_string()),
    }
}

// Was: Führt den Arbeitsschritt `output_error` für output error aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn output_error(output: Output) -> String {
    let status = output.status.to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if !stderr.is_empty() {
        format!("{}: {}", status, stderr)
    } else if !stdout.is_empty() {
        format!("{}: {}", status, stdout)
    } else {
        status
    }
}

// Was: Führt den Arbeitsschritt `service_user` für Dienst Benutzer aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn service_user(unit: &str) -> Option<String> {
    let output = Command::new("systemctl")
        .args(["show", unit, "--property=User", "--value"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let user = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if user.is_empty() { Some("root".to_string()) } else { Some(user) }
}

// Was: Führt den Arbeitsschritt `service_unit_from_cgroup_text` für Dienst unit from cgroup text aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn service_unit_from_cgroup_text(text: &str) -> Option<String> {
    text.lines().find_map(|line| {
        line.split('/')
            .find(|component| component.ends_with(".service"))
            .and_then(normalize_service_unit)
    })
}

// Was: Führt den Arbeitsschritt `normalize_service_unit` für normalize Dienst unit aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn normalize_service_unit(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.contains('/') || trimmed.contains('\0') {
        return None;
    }

    let unit = if trimmed.ends_with(".service") {
        trimmed.to_string()
    } else {
        format!("{}.service", trimmed)
    };

    if unit
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'_' | b'@' | b':' | b'\\'))
    {
        Some(unit)
    } else {
        None
    }
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::{normalize_service_unit, service_unit_from_cgroup_text};

    #[test]
    // Was: Führt den Arbeitsschritt `finds_service_unit_in_cgroup_v2` für finds Dienst unit in cgroup v2 aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn finds_service_unit_in_cgroup_v2() {
        let text = "0::/system.slice/tetra-bluestation.service\n";
        assert_eq!(service_unit_from_cgroup_text(text).as_deref(), Some("tetra-bluestation.service"));
    }

    #[test]
    // Was: Führt den Arbeitsschritt `normalizes_unit_without_suffix` für normalizes unit without suffix aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn normalizes_unit_without_suffix() {
        assert_eq!(
            normalize_service_unit("tetra-bluestation").as_deref(),
            Some("tetra-bluestation.service")
        );
    }

    #[test]
    // Was: Führt den Arbeitsschritt `rejects_path_like_unit_names` für rejects path like unit names aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn rejects_path_like_unit_names() {
        assert!(normalize_service_unit("../tetra").is_none());
    }
}
