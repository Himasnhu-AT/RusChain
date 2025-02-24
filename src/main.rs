extern crate rand;
extern crate sha2;

mod block;
mod blockchain;
mod layer2;
mod multi_layer1;
mod utils;

use layer2::Layer2;
use multi_layer1::MultiLayer1;

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
