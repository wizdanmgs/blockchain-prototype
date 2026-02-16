use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockchainError {
    #[error("Invalid transaction detected")]
    InvalidTransaction,

    #[error("Chain validation failed")]
    InvalidChain,

    #[error("Mining failed")]
    MiningFailed,

    #[error("Serialization error: {0}")]
    Serialization(String),
}
