use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub proof_of_work: u64,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp_millis() as u64;
        let block = Block {
            index,
            timestamp,
            proof_of_work: 0,
            previous_hash,
            hash: String::default(),
        };
        block
    }

    pub fn calculate_hash(&self) -> String {
        let serialized_block_data = serde_json::to_string(self).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(serialized_block_data);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn mine(&mut self, difficulty: usize) {
        while !self.hash.starts_with(&"0".repeat(difficulty)) {
            self.proof_of_work += 1;
            self.hash = self.calculate_hash();
        }
    }
}
