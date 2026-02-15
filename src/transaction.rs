use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
}

impl Transaction {
    pub fn is_valid(&self) -> bool {
        self.amount > 0 && !self.from.is_empty() && !self.to.is_empty()
    }
}
