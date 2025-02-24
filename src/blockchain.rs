use crate::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: vec![],
            difficulty: 3,
        };
        println!("[Layer1] Creating Genesis Block...");
        let mut genesis = Block::new(0, vec!["Genesis Block".to_string()], "0".to_string());
        genesis.mine_block(blockchain.difficulty);
        blockchain.chain.push(genesis);
        blockchain
    }

    pub fn latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    pub fn add_block(&mut self, transactions: Vec<String>) -> Block {
        let previous_hash = self.latest_block().hash.clone();
        let mut block = Block::new(self.chain.len() as u32, transactions, previous_hash);
        println!("[Layer1] Adding new Block {}...", block.index);
        block.mine_block(self.difficulty);
        self.chain.push(block.clone());
        println!("[Layer1] Block {} added to chain.", block.index);
        block
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            if current.hash != current.calculate_hash() {
                println!("[Layer1] Invalid hash at Block {}", current.index);
                return false;
            }
            if current.previous_hash != previous.hash {
                println!(
                    "[Layer1] Mismatched previous hash at Block {}",
                    current.index
                );
                return false;
            }
        }
        true
    }
}
