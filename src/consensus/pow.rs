// Proof of Work
use super::Consensus;
use crate::{block::Block, error::BlockchainError};

pub struct ProofOfWork {
    pub difficulty: usize,
}

impl Consensus for ProofOfWork {
    fn mine(&self, block: &mut Block) -> Result<(), BlockchainError> {
        let target = "0".repeat(self.difficulty);
        while !block.hash.starts_with(&target) {
            block.nonce += 1;
            block.hash = block.calculate_hash()?;
        }

        Ok(())
    }

    fn validate(&self, block: &Block) -> Result<(), BlockchainError> {
        let target = "0".repeat(self.difficulty);
        if !block.hash.starts_with(&target) {
            return Err(BlockchainError::InvalidChain);
        }

        Ok(())
    }
}
