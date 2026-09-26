// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Aufzeichnung und Ereignisprotokollierung.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

// Was: Bindet das Untermodul Konfiguration in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod config;
// Was: Bindet das Untermodul HTTP in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod http;
// Was: Bindet das Untermodul Audio- und Mediendaten switch in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod media_switch;
// Was: Bindet das Untermodul protocol in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod protocol;
// Was: Bindet das Untermodul Zustand in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod state;
// Was: Bindet das Untermodul tar in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tar;

use std::path::PathBuf;

use clap::Parser;
use config::RecorderConfig;
use state::SharedRecorder;

#[derive(Debug, Parser)]
#[command(name = "netcore-recorder")]
#[command(about = "NetCore passive central TETRA media recorder and retention service")]
// Was: Bündelt die zusammengehörigen Werte für args in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct Args {
    #[arg(long, default_value = "/etc/netcore/recorder.toml")]
    config: PathBuf,
    #[arg(long)]
    no_config: bool,
    #[arg(long)]
    bind: Option<std::net::SocketAddr>,
}

// Was: Startet das Programm, lädt die benötigten Einstellungen und übergibt an den eigentlichen Dienstablauf.
// Warum: Ein klarer Einstiegspunkt hält Startreihenfolge, Fehlerausgabe und geordnetes Beenden zusammen.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "netcore_recorder=info".into()),
        )
        .init();

    let args = Args::parse();
    let config_path = if args.no_config {
        None
    } else {
        Some(args.config.as_path())
    };
    let mut config = RecorderConfig::load(config_path)?;
    config.apply_bind_override(args.bind)?;

    tracing::warn!(
        "Recorder starts in OPEN LAB mode: no authentication, no tokens and no TLS"
    );

    let recorder = SharedRecorder::load(config.clone())?;
    let _media_worker =
        media_switch::spawn_media_switch_worker(config.clone(), recorder.clone());
    let _maintenance = state::spawn_maintenance_worker(config.clone(), recorder.clone());
    let http = http::spawn_http_server(config, recorder)?;
    http.join().map_err(|_| -> Box<dyn std::error::Error> {
        "HTTP server thread panicked".into()
    })?;
    Ok(())
}
