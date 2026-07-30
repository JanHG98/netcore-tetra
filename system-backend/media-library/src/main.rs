// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für gespeicherte Aufzeichnungen, TTS- und Mediendateien.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

// Was: Bindet das Untermodul Konfiguration in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod config;
// Was: Bindet das Untermodul HTTP in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod http;
// Was: Bindet das Untermodul Audio- und Mediendaten in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod media;
// Was: Bindet das Untermodul model in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod model;
// Was: Bindet das Untermodul Zustand in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod state;
mod tts;
// Was: Bindet das Untermodul Hintergrundverarbeitung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod worker;

use std::path::PathBuf;

use clap::Parser;
use config::MediaLibraryConfig;
use state::SharedLibrary;
use tts::TtsService;

#[derive(Debug, Parser)]
#[command(name = "netcore-media-library")]
#[command(about = "NetCore-Tetra media asset, preview, preparation and controlled playout service")]
// Was: Bündelt die zusammengehörigen Werte für args in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct Args {
    #[arg(long, default_value = "/etc/netcore/media-library.toml")]
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
                .unwrap_or_else(|_| "netcore_media_library=info".into()),
        )
        .init();

    let args = Args::parse();
    let config_path = if args.no_config { None } else { Some(args.config.as_path()) };
    let mut config = MediaLibraryConfig::load(config_path)?;
    config.apply_bind_override(args.bind).map_err(std::io::Error::other)?;

    tracing::warn!("Media Library starts in OPEN LAB management mode: no login, no tokens and no TLS");
    tracing::warn!("Every reachable management client may upload, approve, dispatch, archive or delete media according to configuration");
    tracing::info!(
        "Media Library WebUI/API bind={} mode={} worker={}ms",
        config.server.bind,
        config.runtime.operating_mode,
        config.runtime.worker_interval_ms
    );

    let library = SharedLibrary::load(config.clone())?;
    let tts = TtsService::new(config.tts.clone(), config.runtime.auto_approve_tts)
        .map_err(std::io::Error::other)?;
    if config.tts.enabled {
        tracing::info!(
            "Central Media Library TTS enabled endpoint={} templates={}",
            config.tts.endpoint,
            config.tts.template_directory.display()
        );
    }
    let _worker = worker::spawn_worker(config.clone(), library.clone());
    let server = http::spawn_http_server(config, library, tts)?;
    server.join().map_err(|_| -> Box<dyn std::error::Error> {
        "Media Library HTTP server thread panicked".into()
    })?;
    Ok(())
}
