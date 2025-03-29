# Rust Basics and Solana Interaction

This project demonstrates the foundational concepts of Rust programming and basic Solana blockchain interaction. It serves as a learning tool for understanding Rust's core features and how to interact with the Solana blockchain.

## Prerequisites

Before running this project, make sure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (required only for Solana interaction features)
- A Solana wallet created with `solana-keygen new`

## Project Structure

The project consists of a single Rust application that offers two main demonstrations:

1. **Rust Basics**: Demonstrates core Rust concepts including:
   - Variables and mutability
   - Ownership and borrowing
   - Functions
   - Structs and enums
   - Pattern matching
   - Collection types

2. **Solana Interaction**: Demonstrates basic Solana blockchain interaction:
   - Connecting to Solana Devnet
   - Reading wallet information
   - Checking account balance
   - Viewing recent transactions

## Setup

### 1. Clone the repository

```bash
git clone <repository-url>
cd combined_rust_solana
```

### 2. Set up a Solana wallet (for Solana features)

```bash
solana-keygen new
solana config set --url https://api.devnet.solana.com
solana airdrop 2
```

## Running the Application

### Run with Rust basics only

```bash
cargo run
```

### Run with Solana interaction features

```bash
cargo run --features="solana"
```

## Using the Application

The application provides a simple menu interface:

1. **Rust Basics Demo** - Demonstrates basic Rust programming concepts
2. **Solana Interaction Demo** - Shows how to interact with the Solana blockchain
3. **Exit Program** - Exits the application

## Solana Architecture and Transaction Model

This project also includes documentation on Solana's architecture and transaction model, highlighting:

- Proof of History (PoH) and consensus mechanism
- Account structure and types
- Transaction flow and execution
- Transaction costs and compute budget

## Next Steps

After completing this project, you can:

1. Explore more advanced Rust concepts
2. Create your own Solana programs using Anchor Framework
3. Build a simple DApp on Solana
4. Experiment with token creation and management

## Resources

- [Rust Documentation](https://doc.rust-lang.org/book/)
- [Solana Documentation](https://docs.solana.com/)
- [Anchor Framework](https://www.anchor-lang.com/)
- [Solana Cookbook](https://solanacookbook.com/)

## License

This project is open source and available under the [MIT License](LICENSE).
