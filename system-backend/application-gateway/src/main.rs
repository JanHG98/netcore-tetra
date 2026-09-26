// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Anwendungsdienste wie TTS und externe Integrationen.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

// Was: Bindet das Untermodul Konfiguration in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod config;
// Was: Bindet das Untermodul HTTP in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod http;
// Was: Bindet das Untermodul model in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod model;
// Was: Bindet das Untermodul Zustand in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod state;
// Was: Bindet das Untermodul Hintergrundverarbeitung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod worker;

use std::path::PathBuf;

use clap::Parser;
use config::ApplicationGatewayConfig;
use state::SharedGateway;

#[derive(Debug, Parser)]
#[command(name = "netcore-application-gateway")]
#[command(about = "NetCore-Tetra connector, webhook, routing, template and TTS orchestration service")]
// Was: Bündelt die zusammengehörigen Werte für args in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct Args {
    #[arg(long, default_value = "/etc/netcore/application-gateway.toml")]
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
                .unwrap_or_else(|_| "netcore_application_gateway=info".into()),
        )
        .init();

    let args = Args::parse();
    let config_path = if args.no_config { None } else { Some(args.config.as_path()) };
    let mut config = ApplicationGatewayConfig::load(config_path)?;
    config.apply_bind_override(args.bind).map_err(std::io::Error::other)?;

    tracing::warn!("Application Gateway starts in OPEN LAB management mode: no login, no management tokens and no TLS");
    tracing::warn!("External connector credentials are still secrets and are stored separately with redacted management responses");
    tracing::info!(
        "Application Gateway WebUI/API bind={} mode={} worker={}ms",
        config.server.bind,
        config.runtime.operating_mode,
        config.runtime.worker_interval_ms
    );

    let gateway = SharedGateway::load(config.clone())?;
    let _worker = worker::spawn_worker(config.clone(), gateway.clone());
    let server = http::spawn_http_server(config, gateway)?;
    server.join().map_err(|_| -> Box<dyn std::error::Error> {
        "Application Gateway HTTP server thread panicked".into()
    })?;
    Ok(())
}
