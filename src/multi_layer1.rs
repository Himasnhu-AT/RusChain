use crate::block::Block;
use crate::blockchain::Blockchain;

pub struct MultiLayer1 {
    pub chains: Vec<Blockchain>,
}

impl MultiLayer1 {
    pub fn new(num_chains: usize) -> Self {
        let mut chains = Vec::new();
        for i in 0..num_chains {
            println!("[MultiLayer1] Initializing Layer1 Chain {}...", i + 1);
            chains.push(Blockchain::new());
        }
        MultiLayer1 { chains }
    }

    pub fn process_transactions(&mut self, transactions: Vec<String>) -> Vec<Block> {
        println!(
            "[MultiLayer1] Distributing transactions to {} Layer1 chains...",
            self.chains.len()
        );
        let mut results = Vec::new();
        for (i, chain) in self.chains.iter_mut().enumerate() {
            println!(
                "[MultiLayer1] Processing transactions on Chain {}...",
                i + 1
            );
            let block = chain.add_block(transactions.clone());
            results.push(block);
        }
        results
    }
}
