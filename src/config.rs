use clap::Parser;

/// CLI configuration
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
    /// Mining difficulty (number of leading zeros)
    #[arg(short, long, default_value = "4")]
    pub difficulty: usize,

    /// Number of blocks to mine
    #[arg(short, long, default_value = "1")]
    pub blocks: usize,
}
