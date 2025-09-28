
mod block;
mod network;
mod blockchain;
mod wallet;
mod transaction;

use clap::Parser;


use clap::{Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Zinciri görüntüle
    ShowChain,
    /// Yeni blok ekle
    AddBlock { data: String },
    /// Yeni cüzdan oluştur
    NewWallet,
}


use blockchain::Blockchain;
use wallet::Wallet;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    match &cli.command {
        Some(Commands::ShowChain) => {
            let chain = blockchain.lock().unwrap();
            for block in chain.get_chain() {
                println!("Blok: {:?}", block);
            }
        }
        Some(Commands::AddBlock { data }) => {
            let mut chain = blockchain.lock().unwrap();
            chain.add_block(data.clone());
            println!("Yeni blok eklendi!");
        }
        Some(Commands::NewWallet) => {
            let wallet = Wallet::new();
            println!("Yeni cüzdan adresi: {}", wallet.address);
        }
        None => {
            println!("Ethereum istemcisi başlatılıyor...");
            network::start().await;
        }
    }
}
