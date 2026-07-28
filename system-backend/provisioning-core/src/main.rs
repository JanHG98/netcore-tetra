mod config;
mod http;
mod upstream;

use std::path::PathBuf;

use clap::Parser;
use config::ProvisioningConfig;

#[derive(Debug, Parser)]
#[command(name = "netcore-provisioning-core")]
#[command(about = "Central NetCore device, group and membership matrix administration")]
struct Args {
    #[arg(long, default_value = "/etc/netcore/provisioning-core.toml")]
    config: PathBuf,
    #[arg(long)]
    bind: Option<std::net::SocketAddr>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "netcore_provisioning_core=info".into()),
        )
        .init();

    let args = Args::parse();
    let mut config = ProvisioningConfig::load(&args.config)?;
    config
        .apply_bind_override(args.bind)
        .map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidInput, error))?;
    tracing::warn!("Provisioning Core starts in OPEN LAB mode: no authentication, no token and no TLS");
    http::serve(config)?;
    Ok(())
}
