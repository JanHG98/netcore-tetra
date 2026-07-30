// NETCORE-KOMMENTAR – Was: Enthält die Logik oder Einstellungen für main.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

// Was: Bindet das Untermodul Anmeldung und Berechtigung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod auth;
// Was: Bindet das Untermodul Konfiguration in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod config;
// Was: Bindet das Untermodul HTTP in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod http;
// Was: Bindet das Untermodul operations in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod operations;
// Was: Bindet das Untermodul persistence in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod persistence;
// Was: Bindet das Untermodul server in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod server;
// Was: Bindet das Untermodul Zustand in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod state;
// Was: Bindet das Untermodul Weboberfläche in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod webui;
// Was: Bindet das Untermodul ws in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod ws;

use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Parser;
use tracing_subscriber::EnvFilter;

use crate::auth::AuthState;
use crate::config::ControlRoomConfig;
use crate::operations::SharedOperations;
use crate::persistence::PersistenceHandle;

#[derive(Debug, Clone, Parser)]
#[command(name = "netcore-control-room")]
#[command(about = "NetCore-Tetra Control-Room Core server for FlowStation nodes")]
// Was: Bündelt die zusammengehörigen Werte für args in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct Args {
    /// Optional TOML config file. CLI flags override values from this file.
    #[arg(long)]
    config: Option<PathBuf>,

    /// Address to bind. Keep 127.0.0.1 for local testing; use 0.0.0.0 in the LXC/VPN/VLAN.
    #[arg(long)]
    bind: Option<SocketAddr>,

    /// WebSocket path used by base-station nodes.
    #[arg(long)]
    node_path: Option<String>,

    /// WebSocket path used by future Leitstelle/operator clients.
    #[arg(long)]
    ui_path: Option<String>,

    /// Number of recent event/audit entries retained in memory.
    #[arg(long)]
    history_limit: Option<usize>,

    /// Enable SQLite persistence at this database path, regardless of config file.
    #[arg(long)]
    database: Option<PathBuf>,

    /// Force-enable user/password + RBAC auth, regardless of config file.
    #[arg(long)]
    auth_enabled: bool,

    /// Force-disable user/password + RBAC auth, regardless of config file.
    #[arg(long)]
    no_auth: bool,

    /// Node token for BS -> Control Room WebSocket authentication. Prefer env/config in production.
    #[arg(long)]
    node_token: Option<String>,

    /// Bootstrap admin username for HTTP login. Prefer env/config in production.
    #[arg(long)]
    bootstrap_user: Option<String>,

    /// Bootstrap admin password for HTTP login. Prefer env/config in production.
    #[arg(long)]
    bootstrap_password: Option<String>,

    /// Force-disable SQLite persistence, regardless of config file.
    #[arg(long)]
    no_persistence: bool,
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

    let mut config = ControlRoomConfig::load(args.config.as_deref())?;
    config.apply_cli_overrides(
        args.bind,
        args.node_path,
        args.ui_path,
        args.history_limit,
        args.database,
        args.no_persistence,
        args.auth_enabled,
        args.no_auth,
        args.node_token,
        args.bootstrap_user,
        args.bootstrap_password,
    );

    let persistence = if config.persistence.enabled {
        let handle = PersistenceHandle::open(&config.persistence)?;
        tracing::info!(database = %config.persistence.database_path.display(), "SQLite persistence enabled");
        Some(handle)
    } else {
        tracing::info!("SQLite persistence disabled");
        None
    };

    let auth = AuthState::from_config(&config.auth, persistence.clone())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
    if auth.enabled() {
        tracing::info!(health_public = auth.allow_health_unauthenticated(), "Control Room user/password authentication enabled");
    } else {
        tracing::warn!("Control Room user/password authentication disabled");
    }

    let operations = SharedOperations::load(
        config.federation.clone(),
        config.operations.clone(),
        config.services.clone(),
    )?;
    operations.start_poller();

    if !auth.enabled() {
        tracing::warn!("Control Room starts in OPEN LAB mode: no login, no tokens and no TLS");
    }
    tracing::info!(services = config.services.len(), "Core-service federation configured");

    let state = state::SharedControlRoom::new_with_persistence(config.server.history_limit, persistence);
    let server = server::ControlRoomServer::new(
        config.server.bind,
        config.server.node_path,
        config.server.ui_path,
        state,
        auth,
        config.directory.clone(),
        operations,
    );
    server.run()?;
    Ok(())
}
