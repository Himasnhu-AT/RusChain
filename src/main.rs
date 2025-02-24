extern crate rand;
extern crate sha2;

use rand::Rng;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current timestamp in seconds.
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// The basic Block struct for Layer 1.
/// Each block holds an index, a timestamp, a list of transactions,
/// the hash of the previous block, its own hash, a nonce used for mining,
/// and a simulated gas fee as metadata.
#[derive(Clone)]
struct Block {
    index: u32,
    timestamp: u64,
    transactions: Vec<String>,
    previous_hash: String,
    hash: String,
    nonce: u64,
    gas_fee: u64,
}

impl Block {
    /// Creates a new block.
    fn new(index: u32, transactions: Vec<String>, previous_hash: String) -> Self {
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

    /// Calculates the SHA-256 hash of the block’s contents.
    fn calculate_hash(&self) -> String {
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

    /// Mines the block by finding a hash that starts with a given number of zeroes.
    /// Also simulates a gas fee (lower is better).
    fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty);
        println!(
            "[Layer1] Mining Block {}: target prefix = '{}'",
            self.index, target
        );
        // Initialize the hash before entering the loop.
        self.hash = self.calculate_hash();
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        // Simulate a gas fee as a random value between 10 and 100.
        self.gas_fee = rand::thread_rng().gen_range(10..100);
        println!(
            "[Layer1] Block {} mined! Nonce: {}, Hash: {}, Gas Fee: {}",
            self.index, self.nonce, self.hash, self.gas_fee
        );
    }
}

/// The Layer 1 Blockchain struct which holds a vector of blocks and a mining difficulty.
struct Blockchain {
    chain: Vec<Block>,
    difficulty: usize,
}

impl Blockchain {
    /// Initializes a new blockchain with a genesis block.
    fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: vec![],
            difficulty: 3, // Adjust difficulty as needed.
        };
        println!("[Layer1] Creating Genesis Block...");
        let mut genesis = Block::new(0, vec!["Genesis Block".to_string()], "0".to_string());
        genesis.mine_block(blockchain.difficulty);
        blockchain.chain.push(genesis);
        blockchain
    }

    /// Returns a reference to the latest block.
    fn latest_block(&self) -> &Block {
        self.chain.last().unwrap()
    }

    /// Adds a new block to the blockchain using the given transactions.
    /// Returns the newly added block.
    fn add_block(&mut self, transactions: Vec<String>) -> Block {
        let previous_hash = self.latest_block().hash.clone();
        let mut block = Block::new(self.chain.len() as u32, transactions, previous_hash);
        println!("[Layer1] Adding new Block {}...", block.index);
        block.mine_block(self.difficulty);
        self.chain.push(block.clone());
        println!("[Layer1] Block {} added to chain.", block.index);
        block
    }

    /// Validates the integrity of the blockchain.
    fn is_valid(&self) -> bool {
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

/// Represents multiple independent Layer 1 chains to distribute load.
struct MultiLayer1 {
    chains: Vec<Blockchain>,
}

impl MultiLayer1 {
    /// Initializes a MultiLayer1 with the specified number of Layer1 chains.
    fn new(num_chains: usize) -> Self {
        let mut chains = Vec::new();
        for i in 0..num_chains {
            println!("[MultiLayer1] Initializing Layer1 Chain {}...", i + 1);
            chains.push(Blockchain::new());
        }
        MultiLayer1 { chains }
    }

    /// Processes the given transactions on all Layer1 chains.
    /// Returns a vector of the newly mined blocks from each chain.
    fn process_transactions(&mut self, transactions: Vec<String>) -> Vec<Block> {
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

/// The Layer 2 aggregator which collects transactions off-chain.
/// Once a threshold is reached, it distributes them to multiple Layer1 chains,
/// compares the returned blocks (hashes with metadata), and updates its verified chain.
struct Layer2 {
    mempool: Vec<String>,
    threshold: usize,
    verified_chain: Vec<Block>,
}

impl Layer2 {
    /// Creates a new Layer2 aggregator with a given threshold.
    fn new(threshold: usize) -> Self {
        Layer2 {
            mempool: vec![],
            threshold,
            verified_chain: vec![],
        }
    }

    /// Adds a transaction to the mempool.
    fn add_transaction(&mut self, transaction: String) {
        println!("[Layer2] Adding transaction: {}", transaction);
        self.mempool.push(transaction);
    }

    /// If the number of transactions reaches the threshold,
    /// drains the mempool and returns the batch.
    fn commit_transactions(&mut self) -> Option<Vec<String>> {
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

    /// Compares blocks returned from multiple Layer1 chains and selects the optimal one.
    /// Here, we select the block with the lowest gas fee.
    fn select_optimal_block(&self, blocks: Vec<Block>) -> Option<Block> {
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

    /// Updates the verified chain with the selected block.
    fn update_verified_chain(&mut self, block: Block) {
        println!(
            "[Layer2] Updating verified chain with Block {} (Hash: {})",
            block.index, block.hash
        );
        self.verified_chain.push(block);
    }
}

fn main() {
    println!("=== Starting Two-Layer Blockchain Simulation ===");

    // Initialize MultiLayer1 with 3 independent Layer1 chains.
    let mut multi_layer1 = MultiLayer1::new(3);

    // Initialize Layer2 with a threshold of 3 transactions.
    let mut layer2 = Layer2::new(3);

    // Simulate off-chain transactions in Layer2.
    layer2.add_transaction("Alice pays Bob 10 coins".to_string());
    layer2.add_transaction("Bob pays Charlie 5 coins".to_string());

    // Attempt to commit (should not commit yet since threshold is not met).
    if let Some(transactions) = layer2.commit_transactions() {
        // Distribute transactions to multiple Layer1 chains.
        let blocks = multi_layer1.process_transactions(transactions);
        // Layer2 compares returned blocks and selects the optimal one.
        if let Some(optimal_block) = layer2.select_optimal_block(blocks) {
            layer2.update_verified_chain(optimal_block);
        } else {
            println!("[Layer2] No optimal block was selected.");
        }
    }

    // Add one more transaction to reach the threshold.
    layer2.add_transaction("Charlie pays Dave 3 coins".to_string());

    // Now that threshold is reached, commit these transactions to Layer1.
    if let Some(transactions) = layer2.commit_transactions() {
        let blocks = multi_layer1.process_transactions(transactions);
        if let Some(optimal_block) = layer2.select_optimal_block(blocks) {
            layer2.update_verified_chain(optimal_block);
        } else {
            println!("[Layer2] No optimal block was selected.");
        }
    }

    // Print the verified chain stored in Layer2.
    println!("\n=== Verified Chain in Layer2 ===");
    for block in &layer2.verified_chain {
        println!(
            "Verified Block {}: {} transaction(s), Hash: {}, Gas Fee: {}",
            block.index,
            block.transactions.len(),
            block.hash,
            block.gas_fee
        );
    }

    // Also, print the entire blockchain of each Layer1 chain.
    for (i, chain) in multi_layer1.chains.iter().enumerate() {
        println!("\n=== Layer1 Chain {} ===", i + 1);
        for block in &chain.chain {
            println!(
                "Block {}: {} transaction(s), Hash: {}, Gas Fee: {}",
                block.index,
                block.transactions.len(),
                block.hash,
                block.gas_fee
            );
        }
    }

    // Validate one of the Layer1 chains.
    println!(
        "\n[Validation] Overall Blockchain Validity (Chain 1): {}",
        multi_layer1.chains[0].is_valid()
    );
    println!("=== Simulation Complete ===");
}
