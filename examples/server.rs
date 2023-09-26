use clap::Parser;
use rumpunch::RendezvousServer;
use tracing::{info, subscriber::set_global_default, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Start port number
    port_from: u16,

    /// End port number
    port_to: u16,

    /// Enable debug output
    #[arg(long, action = clap::ArgAction::SetTrue)]
    debug: bool,

    /// Enable trace output
    #[arg(long, action = clap::ArgAction::SetTrue)]
    trace: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Install a subscriber for tracing
    let level = match (args.trace, args.debug) {
        (true, _) => Level::TRACE,
        (false, true) => Level::DEBUG,
        _ => Level::WARN,
    };
    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();
    set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Started");

    let ports = args.port_from..=args.port_to;
    let server = RendezvousServer::new(ports);
    server.blocking_run().await?;

    info!("Finished");
    Ok(())
}
