use tracing::{info, subscriber::set_global_default, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Started");

    // Install a subscriber for tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Started");

    rumpunch::Test::test().await?;

    info!("Finished");
    Ok(())
}
