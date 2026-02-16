# Blockchain Prototype

A blockchain prototype written in Rust featuring:

- Hash-linked blocks
- Proof of Work (PoW) consensus
- Trait-based consensus abstraction
- Proper error handling (no panic / unwrap)
- Config-driven mining difficulty
- Structured logging with tracing
- CLI-based mining node

This project is designed as a clean foundation for building more advanced blockchain systems similar in architecture (conceptually) to Bitcoin and Ethereum, but intentionally simplified for learning and experimentation.

---

## Architecture Overview

blockchain-node/
├── Cargo.toml
└── src/
├── main.rs
├── config.rs
├── error.rs
├── blockchain.rs
├── block.rs
├── transaction.rs
├── consensus/
│ ├── mod.rs
│ └── pow.rs
└── utils/
└── hash.rs

### Core Components

- block.rs  
  Defines the Block structure and hashing logic.

- transaction.rs  
  Defines transactions and validation rules.

- blockchain.rs  
  Manages chain state and block addition.

- consensus/  
  Trait-based consensus abstraction.
  - pow.rs implements Proof of Work.

- config.rs  
  CLI configuration via clap.

- error.rs  
  Centralized error handling using thiserror.

- utils/hash.rs  
  SHA-256 hashing utilities.

---

## Features

### 1. Hash-Linked Blocks

Each block stores:

- index
- timestamp
- transactions
- previous_hash
- nonce
- hash

Changing any data in a block changes its hash. Since each block stores the previous block's hash, tampering breaks the entire chain.

### 2. Proof of Work (PoW)

Mining requires finding a nonce such that:

hash.starts_with("0" \* difficulty)

Difficulty is configurable via CLI.

### 3. Trait-Based Consensus

Consensus is abstracted:

```rust
pub trait Consensus {
    fn mine(&self, block: &mut Block) -> Result<(), BlockchainError>;
    fn validate(&self, block: &Block) -> Result<(), BlockchainError>;
}
```

This allows future replacement of PoW with:

- Proof of Stake
- BFT-based consensus
- Hybrid mechanisms

### 4. Proper Error Handling

No panic or unwrap is used in core logic.

All operations return:

```rust
Result<T, BlockchainError>
```

Errors are defined using thiserror.

### 5. Structured Logging

Uses tracing for structured logging:

- Mining progress
- Validation results
- Error reporting

---

## Installation

### 1. Clone Repository

```bash
git clone <your-repo-url>
cd blockchain-node
```

### 2. Build

```bash
cargo build --release
```

#### Usage

##### Run With Default Settings

```bash
cargo run
```

Default configuration:

- difficulty: 4
- blocks: 1

##### Custom Difficulty and Block Count

```bash
cargo run -- --difficulty 5 --blocks 3
```

Arguments:

`-d, --difficulty <N>`
Number of leading zeros required in block hash.

`-b, --blocks <N>`
Number of blocks to mine.

---

## Example Output

```bash
INFO Starting node with difficulty 4
INFO Mining block 1
INFO Mining block 2
INFO Blockchain valid
```

### How It Works

#### Block Creation Flow

1. Validate transactions
2. Create block with:
   - index
   - timestamp
   - previous_hash
3. Calculate initial hash
4. Mine block (adjust nonce until difficulty satisfied)
5. Append to chain

#### Chain Validation Flow

For each block:

- Recompute hash and compare
- Verify previous_hash matches prior block
- Validate consensus rules

If any check fails, validation returns an error.
