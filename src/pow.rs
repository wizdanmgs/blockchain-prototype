// Proof of Work
use crate::block::Block;

pub fn mine_block(block: &mut Block, difficulty: usize) {
    let target = "0".repeat(difficulty);
    while !block.hash.starts_with(&target) {
        block.nonce += 1;
        block.hash = block.calculate_hash();
    }

    println!("Block mined: {}", block.hash);
}
