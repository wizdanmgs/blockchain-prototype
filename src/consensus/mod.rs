use crate::{block::Block, error::BlockchainError};

/// Consensus abstraction.
/// Allows swapping PoW with PoS or others later.
pub trait Consensus {
    fn mine(&self, block: &mut Block) -> Result<(), BlockchainError>;
    fn validate(&self, block: &Block) -> Result<(), BlockchainError>;
}

pub mod pow;

