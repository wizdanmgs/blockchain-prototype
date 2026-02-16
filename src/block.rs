use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{error::BlockchainError, transaction::Transaction, utils::hash};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    pub fn new(
        index: u64,
        transactions: Vec<Transaction>,
        previous_hash: String,
    ) -> Result<Self, BlockchainError> {
        let timestamp = Utc::now().timestamp();

        let mut block = Self {
            index,
            timestamp,
            transactions,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        };

        block.hash = block.calculate_hash()?;
        Ok(block)
    }

    pub fn calculate_hash(&self) -> Result<String, BlockchainError> {
        let data = serde_json::to_string(&(
            self.index,
            self.timestamp,
            &self.transactions,
            &self.previous_hash,
            self.nonce,
        ))
        .map_err(|e| BlockchainError::Serialization(e.to_string()))?;

        Ok(hash::calculate_hash(&data))
    }
}
