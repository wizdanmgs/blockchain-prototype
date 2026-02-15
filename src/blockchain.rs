use crate::{block::Block, pow::mine_block, transaction::Transaction};

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut blockchain = Self {
            chain: Vec::new(),
            difficulty,
        };

        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis = Block::new(0, Vec::new(), "0".to_string());
        self.chain.push(genesis);
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        for tx in &transactions {
            if !tx.is_valid() {
                panic!("Invalid transaction detected");
            }
        }

        let previous_hash = self.chain.last().unwrap().hash.clone();
        let mut block = Block::new(self.chain.len() as u64, transactions, previous_hash);

        mine_block(&mut block, self.difficulty);

        self.chain.push(block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.hash != current.calculate_hash() {
                return false;
            }

            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }
}
