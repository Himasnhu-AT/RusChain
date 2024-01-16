mod models;

use rand::Rng;
use std::{thread, time};

/// * `difficulty` - The difficulty level for mining new blocks in the blockchain.
///
/// # Example
///
/// ```
/// use models::blockchain::Blockchain;
///
/// let difficulty = 4;
/// let mut blockchain = Blockchain::new(difficulty);
/// ```

fn main() {
    println!("Hello, world!");

    let difficulty = 1;

    let mut blockchain = models::blockchain::Blockchain::new(difficulty);

    loop {
        // Sleep for a short duration to simulate real-time intervals
        thread::sleep(time::Duration::from_secs(2));

        let nonce = format!("{:x}", rand::thread_rng().gen::<u64>());
        blockchain.add_block(nonce);

        println!("Blockchain: {:#?}", blockchain);
    }
}
