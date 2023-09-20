use clap::Parser;
use tracing::{error, info, subscriber::set_global_default, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// ID of this peer
    this: String,

    /// ID of peer we want to connect to
    that: String,

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

    println!("Started");

    // Install a subscriber for tracing
    let level = match (args.trace, args.debug) {
        (true, _) => Level::TRACE,
        (false, true) => Level::DEBUG,
        _ => Level::WARN,
    };
    let subscriber = FmtSubscriber::builder().with_max_level(level).finish();
    set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Started");

    if let Err(e) = rumpunch::Test::test(args.this.as_str(), args.that.as_str()).await {
        error!("Error: {:?}", e);
        error!("Source: {:?}", e.source());
        error!("Backtrace: {:?}", e.backtrace());
    }

    info!("Finished");
    Ok(())
}
