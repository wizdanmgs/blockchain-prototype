use crate::{block::Block, consensus::Consensus, error::BlockchainError, transaction::Transaction};
use tracing::info;

pub struct Blockchain<C: Consensus> {
    pub chain: Vec<Block>,
    pub consensus: C,
}

impl<C: Consensus> Blockchain<C> {
    pub fn new(consensus: C) -> Result<Self, BlockchainError> {
        let mut blockchain = Self {
            chain: Vec::new(),
            consensus,
        };

        let genesis = Block::new(0, Vec::new(), "0".to_string())?;
        blockchain.chain.push(genesis);
        Ok(blockchain)
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> Result<(), BlockchainError> {
        for tx in &transactions {
            if !tx.is_valid() {
                return Err(BlockchainError::InvalidTransaction);
            }
        }

        let previous_hash = self.chain.last().unwrap().hash.clone();
        let mut block = Block::new(self.chain.len() as u64, transactions, previous_hash)?;

        info!("Mining block {}", block.index);

        self.consensus.mine(&mut block)?;
        self.chain.push(block);

        Ok(())
    }

    pub fn validate(&self) -> Result<(), BlockchainError> {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.previous_hash != previous.hash {
                return Err(BlockchainError::InvalidChain);
            }

            self.consensus.validate(current)?;
        }
        Ok(())
    }
}
