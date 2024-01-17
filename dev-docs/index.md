### Project Structure

1. **Overview:**

   - The blockchain project aims to implement a simple 2-layered blockchain in Rust. The primary goal is to showcase the structure of a blockchain system with both a main chain (Layer 1) and a secondary chain (Layer 2).

2. **Directory Structure:**

   - The project follows a standard Rust directory structure:

   ```
   ├── src
   │   ├── main.rs
   │   ├── models/
   │   │   ├── blockchain.rs
   │   │   └── block.rs
   │   │   └── mod.rs
   ├── Cargo.toml
   └── README.md
   ```

   - `src/main.rs`: Entry point of the application.
   - `src/models/`: Module for blockchain and block implementations.
   - `src/models/blockchain.rs`: Implementation of the `Blockchain` struct.
   - `src/models/block.rs`: Implementation of the `Block` struct.
   - `src/mod.rs`: Module file for the main module.

3. **Modules:**
   - `Blockchain Module (blockchain.rs)`: The `Blockchain` struct represents the main chain with functions for creating a new blockchain and adding blocks.
   - `Block Module (block.rs)`: The `Block` struct represents individual blocks with functions for creating and mining a block.

### Code Structure

1. **Blockchain Module (`blockchain.rs`):**

   - The `Blockchain` struct contains:
     - `genesis_block`: The initial block in the chain.
     - `chain`: Vector to store blocks.
     - `difficulty`: Minimum amount of work required to validate a block.
   - Functions:
     - `new(difficulty: usize) -> Self`: Creates a new blockchain.
     - `add_block(nonce: String)`: Adds a new block to the blockchain.

2. **Block Module (`block.rs`):**
   - The `Block` struct contains:
     - `index`: Index of the block.
     - `timestamp`: Time the block is created.
     - `proof_of_work`: Proof of work for the block.
     - `previous_hash`: Hash of the previous block.
     - `hash`: Hash of the current block.
   - Functions:
     - `new(index: u64, previous_hash: String) -> Self`: Creates a new block.
     - `calculate_hash() -> String`: Calculates the hash of the block.
     - `mine(blockchain: Blockchain)`: Mines the block.

### Usage

1. **Initialization:**

   - To initialize a new blockchain, use the following code in `main.rs`:

   ```rust
   use models::blockchain::Blockchain;

   fn main() {
       let difficulty = 1;
       let mut blockchain = Blockchain::new(difficulty);

       // Continue with blockchain operations...
   }
   ```

2. **Adding Blocks:**

   - To add blocks to the blockchain, use the `add_block` function:

   ```rust
   let nonce = String::from("some_nonce_value");
   blockchain.add_block(nonce);
   ```

3. **Customization:**

   - Users can customize the blockchain parameters by adjusting the `difficulty` value when creating a new blockchain instance.

   ```rust
   let difficulty = 2; // Set the desired difficulty level
   let mut blockchain = Blockchain::new(difficulty);
   ```

### Contribution Guidelines

1. **Code Contributions:**

   - Contributors can submit code changes by creating a pull request on the GitHub repository.

2. **Bug Reports and Feature Requests:**
   - Users are encouraged to submit bug reports or suggest new features through GitHub issues.

### Building and Running

1. **Dependencies:**

   - The project uses the following dependencies, specified in `Cargo.toml`:

   ```toml
   [dependencies]
   chrono = "0.4"
   serde = { version = "1", features = ["derive"] }
   sha2 = "0.10"
   ```

2. **Building and Running:**

   - Build and run the project using Cargo commands:

   ```bash
   cargo build
   cargo run
   ```

<!-- 3. **Unit Tests:**
   - Run unit tests with the following command:
   ```bash
   cargo test
   ```
4. **Integration Tests:**
   - Conduct integration tests to ensure the seamless interaction between Layer 1 and Layer 2. -->

### Additional Resources

1. **External Libraries:**
   - The project uses external libraries such as `chrono`, `serde`, and `sha2`. Refer to their documentation for additional details.

### Conclusion

1. **Acknowledgments:**
   - Thank contributors for their efforts and acknowledge any external resources or libraries used in the project.
