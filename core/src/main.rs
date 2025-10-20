mod network;
mod server;
#[cfg(test)]
pub mod test;

use crate::server::rafka_config::RafkaConfig;
use crate::server::rafka_raft_server::RaftServer;
use crate::server::{Result, Server};
use clap::Parser;
use easy_config_def::FromConfigDef;
use rafka_clients::common::utils::utils::load_props;
use std::collections::HashMap;
use std::error::Error;
use std::iter::Map;
use tokio::signal;
use tracing::{debug, info};

/// A Kafka-compatible broker implemented in Rust.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The server properties file.
    #[arg(name = "server.properties")]
    server_properties_file: String,

    /// Optional configuration overrides.
    #[arg(long)]
    override_opt: Vec<String>, // Captures any --override options, though we won't use them yet.
}

#[tokio::main]
async fn main() -> Result<()> {
    set_up_logging()?;
    let server_props = get_props_from_args(Args::parse());
    debug!("{server_props:?}");
    let server = build_server(server_props);

    //server.startup().await?;

    // tokio::select! {
    //     _ = signal::ctrl_c() => {
    //         // The shutdown signal has been received.
    //         info!("shutting down");
    //     }
    // }

    //server.await_shutdown().await?;

    Ok(())
}

fn set_up_logging() -> std::result::Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // See https://docs.rs/tracing for more info
    tracing_subscriber::fmt::try_init()
}
fn get_props_from_args(args: Args) -> HashMap<String, String> {
    load_props(args.server_properties_file.as_str()).expect("Error loading properties file")
}

fn build_server(props: HashMap<String, String>) {
    let config = RafkaConfig::from_props(&props);
    debug!("{config:?}");
    //RaftServer::new()
}

async fn run_broker(args: Args) -> std::result::Result<(), Box<dyn std::error::Error>> {
    // --- The following sections are placeholders for future implementation steps ---

    // 2. Load Configuration from the properties file.
    // let server_config = load_config(&args.server_properties_file)?;
    println!(
        "[TODO] Load configuration from {}",
        args.server_properties_file
    );

    // 3. Initialize Logging.
    // The original code sets up log4j. We would use a crate like `tracing` or `log`.
    // setup_logging()?;
    println!("[TODO] Initialize logging framework.");

    // 4. Create and start the main Broker/Server component.
    // In the original code, this is `val server = new KafkaServer(...)`.
    // Here we would instantiate our main `BrokerServer` struct.
    // let mut broker_server = BrokerServer::new(server_config);
    println!("[TODO] Create the main BrokerServer instance.");

    // 5. Add a Shutdown Hook.
    // The original uses a JVM shutdown hook. In Rust, we can listen for signals
    // like SIGTERM and SIGINT to trigger a graceful shutdown.
    // This is often done using `tokio::signal`.
    // tokio::signal::ctrl_c().await?;
    // println!("Received shutdown signal...");
    // broker_server.shutdown().await;
    println!("[TODO] Add a shutdown hook to gracefully stop the server.");

    // 6. Start the server's main loop (e.g., the networking listener).
    // This is the equivalent of `server.startup()`.
    // broker_server.startup().await?;
    println!("[TODO] Start the server's main components (e.g., network listener).");

    // 7. Wait for the server to stop.
    // In a real implementation, the `startup` function might run indefinitely
    // until a shutdown is triggered.
    // broker_server.await_shutdown().await;

    println!("Broker shut down successfully.");

    Ok(())
}
