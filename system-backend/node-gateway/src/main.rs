// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für die Verbindung zwischen Basisstationen und Backend-Diensten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

// Was: Bindet das Untermodul Konfiguration in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod config;
// Was: Bindet das Untermodul HTTP in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod http;
// Was: Bindet das Untermodul server in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod server;
// Was: Bindet das Untermodul Dienst monitor in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod service_monitor;
// Was: Bindet das Untermodul Zustand in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod state;
// Was: Bindet das Untermodul ws in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod ws;

use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Parser;
use tracing_subscriber::EnvFilter;

use crate::config::NodeGatewayConfig;
use crate::server::NodeGatewayServer;
use crate::state::SharedGateway;

#[derive(Debug, Parser)]
#[command(name = "netcore-node-gateway")]
#[command(about = "NetCore-Tetra TBS node gateway with an open lab WebUI")]
// Was: Bündelt die zusammengehörigen Werte für args in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct Args {
    /// Optional TOML configuration file.
    #[arg(long)]
    config: Option<PathBuf>,

    /// Override the configured listener address.
    #[arg(long)]
    bind: Option<SocketAddr>,

    /// Print the effective configuration and exit.
    #[arg(long)]
    check_config: bool,
}

// Was: Startet das Programm, lädt die benötigten Einstellungen und übergibt an den eigentlichen Dienstablauf.
// Warum: Ein klarer Einstiegspunkt hält Startreihenfolge, Fehlerausgabe und geordnetes Beenden zusammen.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with_target(false)
        .compact()
        .init();

    let mut config = NodeGatewayConfig::load(args.config.as_deref())?;
    config
        .apply_bind_override(args.bind)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidInput, error))?;

    if args.check_config {
        println!("{}", toml::to_string_pretty(&config)?);
        return Ok(());
    }

    tracing::warn!(
        "OPEN LAB MODE ACTIVE: every client on the reachable network can view nodes and execute enabled management actions"
    );
    let gateway = SharedGateway::new(config.clone());
    service_monitor::spawn_service_monitor(gateway.clone(), config.clone());
    NodeGatewayServer::new(config, gateway).run()?;
    Ok(())
}
