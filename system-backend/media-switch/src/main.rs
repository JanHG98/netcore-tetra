// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für die Audioverteilung zwischen Basisstationen und Rufen.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

// Was: Bindet das Untermodul Ruf Steuerung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod call_control;
// Was: Bindet das Untermodul Konfiguration in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod config;
// Was: Bindet das Untermodul Gateway in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod gateway;
// Was: Bindet das Untermodul HTTP in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod http;
// Was: Bindet das Untermodul protocol in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod protocol;
// Was: Bindet das Untermodul Zustand in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod state;

use std::path::PathBuf;
use std::sync::mpsc;

use clap::Parser;
use config::MediaSwitchConfig;
use state::SharedMedia;

#[derive(Debug, Parser)]
#[command(name = "netcore-media-switch")]
#[command(about = "NetCore central packed-TETRA speech-frame router and jitter buffer")]
// Was: Bündelt die zusammengehörigen Werte für args in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct Args {
    #[arg(long, default_value = "/etc/netcore/media-switch.toml")]
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
                .unwrap_or_else(|_| "netcore_media_switch=info".into()),
        )
        .init();

    let args = Args::parse();
    let config_path = if args.no_config {
        None
    } else {
        Some(args.config.as_path())
    };
    let mut config = MediaSwitchConfig::load(config_path)?;
    config.apply_bind_override(args.bind)?;

    tracing::warn!(
        "Media Switch starts in OPEN LAB mode: no authentication, no tokens and no TLS"
    );

    let media = SharedMedia::new(config.clone());
    let (gateway_tx, gateway_rx) = mpsc::channel();
    let _gateway = gateway::spawn_gateway_worker(config.clone(), media.clone(), gateway_rx);
    let _call_control = call_control::spawn_call_control_worker(config.clone(), media.clone());
    let http = http::spawn_http_server(config, media, gateway_tx)?;
    http.join().map_err(|_| -> Box<dyn std::error::Error> {
        "HTTP server thread panicked".into()
    })?;
    Ok(())
}
