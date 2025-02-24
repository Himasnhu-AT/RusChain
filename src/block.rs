use crate::utils::current_timestamp;
use rand::Rng;
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub transactions: Vec<String>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub gas_fee: u64,
}

impl Block {
    pub fn new(index: u32, transactions: Vec<String>, previous_hash: String) -> Self {
        let timestamp = current_timestamp();
        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
            gas_fee: 0,
        }
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_string());
        hasher.update(self.timestamp.to_string());
        for tx in &self.transactions {
            hasher.update(tx);
        }
        hasher.update(&self.previous_hash);
        hasher.update(self.nonce.to_string());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        println!(
            "[Layer1] Mining Block {}: target prefix = '{}'",
            self.index, target
        );
        self.hash = self.calculate_hash();
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        self.gas_fee = rand::thread_rng().gen_range(10..100);
        println!(
            "[Layer1] Block {} mined! Nonce: {}, Hash: {}, Gas Fee: {}",
            self.index, self.nonce, self.hash, self.gas_fee
        );
    }
}
