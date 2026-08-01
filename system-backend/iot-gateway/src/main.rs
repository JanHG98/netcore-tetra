mod config;
mod http;
mod model;
mod mqtt;
mod poller;
mod state;

use std::path::PathBuf;

use clap::Parser;
use config::IotGatewayConfig;
use state::SharedGateway;

#[derive(Debug, Parser)]
#[command(name = "netcore-iot-gateway")]
#[command(about = "NetCore-Tetra event-to-MQTT bridge and IoT integration gateway")]
struct Args {
    #[arg(long, default_value = "/etc/netcore/iot-gateway.toml")]
    config: PathBuf,
    #[arg(long)]
    no_config: bool,
    #[arg(long)]
    bind: Option<std::net::SocketAddr>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "netcore_iot_gateway=info".into()),
        )
        .init();

    let args = Args::parse();
    let config_path = if args.no_config {
        None
    } else {
        Some(args.config.as_path())
    };
    let mut config = IotGatewayConfig::load(config_path)?;
    config
        .apply_bind_override(args.bind)
        .map_err(std::io::Error::other)?;

    tracing::warn!(
        "IoT Gateway starts in OPEN LAB mode: no login, no tokens, no MQTT credentials and no TLS"
    );
    tracing::warn!(
        "Phase 3 observes MQTT command topics but never executes commands; policy and acknowledgements follow in Phase 4"
    );
    tracing::info!(
        "IoT Gateway WebUI/API={} MQTT={}:{} prefix={} sources={}",
        config.server.bind,
        config.mqtt.host,
        config.mqtt.port,
        config.mqtt.topic_prefix,
        config.sources.iter().filter(|source| source.enabled).count()
    );

    let state = SharedGateway::new(config.clone())?;
    let (poll_control, _poller) = poller::spawn_poller(config.clone(), state.clone())
        .map_err(std::io::Error::other)?;
    let (_publisher, mqtt_control, _mqtt_workers) = mqtt::spawn_mqtt(config.clone(), state.clone());
    let http = http::spawn_http_server(config, state, poll_control, mqtt_control)?;
    http.join()
        .map_err(|_| -> Box<dyn std::error::Error> {
            "IoT Gateway HTTP server thread panicked".into()
        })?;
    Ok(())
}
