use crate::block::Block;

pub struct Layer2 {
    pub mempool: Vec<String>,
    pub threshold: usize,
    pub verified_chain: Vec<Block>,
}

impl Layer2 {
    pub fn new(threshold: usize) -> Self {
        Layer2 {
            mempool: vec![],
            threshold,
            verified_chain: vec![],
        }
    }

    pub fn add_transaction(&mut self, transaction: String) {
        println!("[Layer2] Adding transaction: {}", transaction);
        self.mempool.push(transaction);
    }

    pub fn commit_transactions(&mut self) -> Option<Vec<String>> {
        if self.mempool.len() >= self.threshold {
            let committed: Vec<String> = self.mempool.drain(..).collect();
            println!("[Layer2] Committing transactions: {:?}", committed);
            Some(committed)
        } else {
            println!(
                "[Layer2] Not enough transactions to commit. Current count: {}",
                self.mempool.len()
            );
            None
        }
    }

    pub fn select_optimal_block(&self, blocks: Vec<Block>) -> Option<Block> {
        println!("[Layer2] Comparing returned blocks from Layer1 chains...");
        if blocks.is_empty() {
            println!("[Layer2] No blocks received for comparison.");
            return None;
        }
        let mut optimal = blocks[0].clone();
        for block in blocks.iter() {
            println!(
                "[Layer2] Chain Block {}: Gas Fee = {}",
                block.index, block.gas_fee
            );
            if block.gas_fee < optimal.gas_fee {
                optimal = block.clone();
            }
        }
        println!(
            "[Layer2] Optimal block selected: Block {} with Gas Fee {}",
            optimal.index, optimal.gas_fee
        );
        Some(optimal)
    }

    pub fn update_verified_chain(&mut self, block: Block) {
        println!(
            "[Layer2] Updating verified chain with Block {} (Hash: {})",
            block.index, block.hash
        );
        self.verified_chain.push(block);
    }
}
