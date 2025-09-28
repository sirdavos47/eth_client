use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub signature: String,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u64, signature: String) -> Self {
        Transaction { from, to, amount, signature }
    }
}
