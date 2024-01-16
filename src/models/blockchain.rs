use chrono::prelude::*;

use super::block::Block;

type Blocks = Vec<Block>;

#[derive(Debug)]
pub struct Blockchain {
    pub genesis_block: Block,
    pub chain: Blocks,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let genesis_block = Block::new(0, String::default());
        let mut chain = Vec::new();
        chain.push(genesis_block.clone());

        Blockchain {
            genesis_block,
            chain,
            difficulty,
        }
    }

    pub fn add_block(&mut self, nonce: String) {
        let new_block = Block::new(
            self.chain.len() as u64,
            self.chain.last().unwrap().hash.clone(),
        );
        let mut new_block = new_block.clone();
        new_block.mine(self.difficulty);
        self.chain.push(new_block.clone());
        println!("New block added to chain -> {:?}", new_block);
    }
}
