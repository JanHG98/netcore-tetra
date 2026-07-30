// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Verbindungen zu anderen Netzen und Systemen.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

// Was: Bindet das Untermodul Konfiguration in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod config;
// Was: Bindet das Untermodul HTTP in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod http;
// Was: Bindet das Untermodul protocol in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod protocol;
// Was: Bindet das Untermodul Zustand in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod state;
// Was: Bindet das Untermodul transport in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod transport;

use std::path::PathBuf;

use clap::Parser;
use config::TransitConfig;
use state::SharedTransit;

#[derive(Debug, Parser)]
#[command(name = "netcore-transit")]
#[command(about = "NetCore inter-region call, SDS, media and mobility transit service")]
// Was: Bündelt die zusammengehörigen Werte für args in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct Args {
    #[arg(long, default_value = "/etc/netcore/transit.toml")]
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
                .unwrap_or_else(|_| "netcore_transit=info".into()),
        )
        .init();

    let args = Args::parse();
    let config_path = if args.no_config {
        None
    } else {
        Some(args.config.as_path())
    };
    let mut config = TransitConfig::load(config_path)?;
    config
        .apply_bind_override(args.bind)
        .map_err(std::io::Error::other)?;

    tracing::warn!(
        "Transit management and peer transport start in OPEN LAB mode: no login, no tokens and no TLS"
    );
    tracing::warn!(
        "The regional transport protocol is NetCore-native and not yet standardized ETSI ISI"
    );
    tracing::info!(
        "Transit region={} swmi={} operating_mode={}",
        config.region.region_id,
        config.region.swmi_id,
        config.region.operating_mode
    );

    let transit = SharedTransit::load(config.clone())?;
    let _transport = transport::spawn_transport_worker(config.clone(), transit.clone());
    let http = http::spawn_http_server(config, transit)?;
    http.join().map_err(|_| -> Box<dyn std::error::Error> {
        "Transit HTTP server thread panicked".into()
    })?;
    Ok(())
}
