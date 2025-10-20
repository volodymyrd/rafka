use crate::server::rafka_config::RafkaConfig;
use tokio::net::TcpListener;
use tokio::sync::{broadcast, mpsc};

#[derive(Debug)]
pub struct SocketServer {}

impl SocketServer {
    // pub async fn run(config: &RafkaConfig, listener: TcpListener, shutdown: impl Future) {
    //     // When the provided `shutdown` future completes, we must send a shutdown
    //     // message to all active connections. We use a broadcast channel for this
    //     // purpose. The call below ignores the receiver of the broadcast pair, and when
    //     // a receiver is needed, the subscribe() method on the sender is used to create
    //     // one.
    //     let (notify_shutdown, _) = broadcast::channel(1);
    //     let (shutdown_complete_tx, mut shutdown_complete_rx) = mpsc::channel(1);
    // }
}
