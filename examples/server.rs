use rumpunch::RendezvousServer;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    println!("Started");

    // Install a subscriber for tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Started");

    let ports = vec![4000, 4001, 4002];

    let server = RendezvousServer::new(ports);
    server.blocking_run().await;

    // tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;

    info!("Finished");
}
