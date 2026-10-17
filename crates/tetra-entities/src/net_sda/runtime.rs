//! Bounded SDA worker. Disk writes and NetworkManager probes stay off the RF thread.

use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crossbeam_channel::Sender;
use tetra_config::bluestation::{AsteriskRuntimeStatus, SharedConfig};

use super::engine::{Engine, SdaRequest};
use super::reply_command;
use crate::net_control::ControlCommand;

pub fn start_worker(config: SharedConfig, replies: Sender<ControlCommand>, path: PathBuf) -> std::io::Result<Sender<SdaRequest>> {
    let (tx, rx) = crossbeam_channel::bounded::<SdaRequest>(32);
    std::thread::Builder::new().name("netcore-sda".into()).spawn(move || {
        let mut engine = Engine::new(path);
        let mut reference = 1u8;
        for request in rx {
            tracing::info!("SDA: request from ISSI {}: {:?}", request.source_issi, request.text);
            if let Some(reply) = engine.process(&request, |topic| query_status(&config, topic)) {
                let command = reply_command(request.source_issi, reference, &reply);
                reference = reference.wrapping_add(1).max(1);
                if let Err(e) = replies.try_send(command) {
                    tracing::warn!("SDA: cannot enqueue reply to ISSI {}: {e}", request.source_issi);
                } else {
                    tracing::info!("SDA: reply queued for ISSI {}: {:?}", request.source_issi, reply);
                }
            }
        }
    })?;
    Ok(tx)
}

fn query_status(config: &SharedConfig, topic: &str) -> String {
    if topic == "VPN" {
        return vpn_status();
    }
    let cfg = config.config();
    let state = config.state_read();
    let name = cfg
        .control_room
        .as_ref()
        .and_then(|c| c.station_name.as_deref().or(c.node_id.as_deref()))
        .filter(|s| !s.is_empty())
        .unwrap_or("Lokale Basisstation");
    match topic {
        "SYSTEM" => format!(
            "{name}; v{}; MS {}; Core {}",
            tetra_core::STACK_VERSION,
            state.subscribers.registered_count(),
            if state.core_gateway_connected { "verbunden" } else { "getrennt" }
        ),
        "BS" => format!(
            "Lokal {name}; MCC {}/{}; LA {}; CC {}; Traeger {}; {} Nachbarn konfiguriert",
            cfg.net.mcc,
            cfg.net.mnc,
            cfg.cell.location_area,
            cfg.cell.colour_code,
            cfg.cell.main_carrier,
            cfg.cell.neighbor_cells_ca.len()
        ),
        "SIP" => sip_status(cfg.asterisk.enabled, &state.asterisk_status),
        _ => "Unbekannte Abfrage".to_owned(),
    }
}

fn sip_status(config_enabled: bool, status: &AsteriskRuntimeStatus) -> String {
    if !config_enabled {
        return "SIP lokal: deaktiviert".to_owned();
    }
    if !status.configured {
        return "SIP lokal: aktiviert, noch kein Laufzeitstatus".to_owned();
    }
    if !status.enabled {
        return "SIP lokal: Laufzeit deaktiviert".to_owned();
    }
    format!("SIP lokal: {}; Dialoge {}", status.register_status, status.active_dialogs)
}

/// Report profile activation, not a verified VPN peer/route or central-service health.
fn vpn_summary(text: &str, expected_uuid: Option<&str>) -> String {
    let mut active_profiles = 0;
    for line in text.lines() {
        let mut fields = line.splitn(3, ':');
        let uuid = fields.next().unwrap_or_default();
        let kind = fields.next().unwrap_or_default();
        let state = fields.next().unwrap_or_default();
        if !matches!(kind, "vpn" | "wireguard") {
            continue;
        }
        if state == "activated" {
            active_profiles += 1;
        }
        if expected_uuid == Some(uuid) {
            return match state {
                "activated" => "VPN: NetCore-Profil aktiv (NetworkManager)".to_owned(),
                "activating" => "VPN: NetCore-Profil verbindet (NetworkManager)".to_owned(),
                _ => "VPN: NetCore-Profilstatus unbekannt".to_owned(),
            };
        }
    }
    if expected_uuid.is_some() {
        "VPN: NetCore-Profil nicht aktiv (NetworkManager)".to_owned()
    } else {
        format!("VPN (NM): {active_profiles} aktive Profile; NetCore-Zuordnung fehlt")
    }
}

fn vpn_status() -> String {
    let expected_uuid = std::fs::read_to_string("/etc/netcore/image-network.json")
        .ok()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .and_then(|v| v.get("vpn_uuid").and_then(|v| v.as_str()).map(str::to_owned));
    let mut child = match Command::new("nmcli")
        .args([
            "--escape",
            "no",
            "--wait",
            "2",
            "-t",
            "-f",
            "UUID,TYPE,STATE",
            "connection",
            "show",
            "--active",
        ])
        .env("LC_ALL", "C")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(child) => child,
        Err(_) => return "VPN: NetworkManager-Status nicht verfuegbar".to_owned(),
    };
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        match child.try_wait() {
            Ok(Some(_)) => {
                return match child.wait_with_output() {
                    Ok(out) if out.status.success() => vpn_summary(&String::from_utf8_lossy(&out.stdout), expected_uuid.as_deref()),
                    _ => "VPN: NetworkManager-Status nicht verfuegbar".to_owned(),
                };
            }
            Ok(None) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(20)),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                return "VPN: Statusabfrage hat Zeitlimit erreicht".to_owned();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sip_uses_actual_registration_and_excludes_credentials_and_diagnostics() {
        let status = AsteriskRuntimeStatus {
            configured: true,
            enabled: true,
            register_status: "registered".into(),
            active_dialogs: 2,
            remote: "sip:secret-user:secret-password@example.invalid".into(),
            last_error: Some("sensitive trace".into()),
            ..Default::default()
        };
        assert_eq!(sip_status(true, &status), "SIP lokal: registered; Dialoge 2");
        assert_eq!(sip_status(false, &status), "SIP lokal: deaktiviert");
        assert!(sip_status(true, &AsteriskRuntimeStatus::default()).contains("noch kein Laufzeitstatus"));
    }

    #[test]
    fn vpn_distinguishes_selected_profile_activation_from_other_profiles_and_unknown_mapping() {
        let rows = "other:vpn:activated\nselected:vpn:activating\nwifi:802-11-wireless:activated";
        assert!(vpn_summary(rows, Some("selected")).contains("verbindet"));
        assert!(vpn_summary(rows, Some("missing")).contains("nicht aktiv"));
        assert!(vpn_summary("selected:vpn:activated", Some("selected")).contains("Profil aktiv"));
        assert!(vpn_summary("selected:wireguard:activated", Some("selected")).contains("Profil aktiv"));
        assert!(vpn_summary(rows, None).contains("1 aktive Profile; NetCore-Zuordnung fehlt"));
    }
}
