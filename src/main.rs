mod block;
mod blockchain;
mod config;
mod consensus;
mod error;
mod transaction;
mod utils;

use clap::Parser;
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;

use blockchain::Blockchain;
use config::Config;
use consensus::pow::ProofOfWork;
use transaction::Transaction;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Structured logging setup
    let subscriber = FmtSubscriber::builder().with_target(false).finish();

    tracing::subscriber::set_global_default(subscriber)?;

    // Parse CLI arguments
    let config = Config::parse();

    info!("Starting node with difficulty {}", config.difficulty);

    let consensus = ProofOfWork {
        difficulty: config.difficulty,
    };

    let mut blockchain = Blockchain::new(consensus)?;

    for i in 0..config.blocks {
        let tx = Transaction {
            from: "Alice".into(),
            to: "Bob".into(),
            amount: 10 + i as u64,
        };

        if let Err(e) = blockchain.add_block(vec![tx]) {
            error!("Failed to add block: {}", e);
        }
    }

    match blockchain.validate() {
        Ok(_) => info!("Blockchain valid"),
        Err(e) => error!("Blockchain invalid: {}", e),
    }

    Ok(())
}
