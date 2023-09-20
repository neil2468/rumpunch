use rumpunch::RendezvousServer;
use tracing::{info, subscriber::set_global_default, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Started");

    // Install a subscriber for tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Started");

    let ports = vec![4000, 4001, 4002];

    let server = RendezvousServer::new(ports);
    server.blocking_run().await?;

    info!("Finished");
    Ok(())
}
