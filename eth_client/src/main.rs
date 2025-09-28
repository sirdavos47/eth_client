mod block;
mod network;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Node is running in debug mode
    #[arg(short, long)]
    debug: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if cli.debug {
        println!("Debug modu aktif.");
    }
    println!("Ethereum istemcisi başlatılıyor...");
    network::start().await;
}
