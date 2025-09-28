use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub number: u64,
    pub previous_hash: String,
    pub data: String,
    pub hash: String,
}

impl Block {
    pub fn new(number: u64, previous_hash: String, data: String) -> Self {
        let hash = format!("{}:{}:{}", number, &previous_hash, &data); // Basit hash
        Block {
            number,
            previous_hash,
            data,
            hash,
        }
    }
}
