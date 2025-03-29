use std::io;
use std::path::Path;

// Import Solana dependencies when in Solana mode
#[cfg(feature = "solana")]
use {
    solana_client::rpc_client::RpcClient,
    solana_sdk::pubkey::Pubkey,
    solana_sdk::signature::{Keypair, read_keypair_file, Signer},
};

// Struct definition for Rust demo
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Enum definition for Rust demo
enum AccountStatus {
    Active,
    Inactive,
    Locked,
}

fn main() {
    println!("Combined Rust Basics and Solana Interaction Program");
    println!("==================================================\n");
    
    // Simple menu for demo selection
    loop {
        println!("\nChoose a demo:");
        println!("1. Rust Basics (variables, functions, structs, enums, ownership)");
        println!("2. Solana Interaction (wallet info, balance)");
        println!("3. Exit Program");
        
        let choice = get_user_input("Enter your choice (1-3): ");
        
        match choice.trim() {
            "1" => rust_basics_demo(),
            "2" => solana_interaction_demo(),
            "3" => {
                println!("Exiting program. Goodbye!");
                break;
            },
            _ => println!("Invalid choice. Please select 1, 2, or 3."),
        }
    }
}

// Demo 1: Rust Basics
fn rust_basics_demo() {
    println!("\nRUST BASICS DEMO");
    println!("===============\n");
    
    // Variables and mutability
    let immutable_var = 5;
    let mut mutable_var = 10;
    println!("Variables Demo:");
    println!("  Immutable: {}, Mutable: {}", immutable_var, mutable_var);
    
    mutable_var = 15;
    println!("  Updated mutable: {}", mutable_var);
    
    // Ownership example
    println!("\nOwnership Demo:");
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    
    // This would cause an error:
    // println!("  s1: {}", s1);
    println!("  s2: {}", s2);
    
    // Function with ownership
    let s3 = String::from("world");
    take_ownership(s3);
    // s3 is no longer valid
    
    // Returning ownership
    let s4 = give_ownership();
    println!("  s4: {}", s4);
    
    // Using structs
    println!("\nStruct Demo:");
    let user = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        sign_in_count: 1,
        active: true,
    };
    
    println!("  User: {}, {}", user.username, user.email);
    
    // Using enums and pattern matching
    println!("\nEnum and Pattern Matching Demo:");
    let status = AccountStatus::Active;
    print_status(status);
    
    // Collection types demo
    println!("\nCollection Types Demo:");
    
    // Vector
    let mut numbers = vec![1, 2, 3, 4, 5];
    numbers.push(6);
    println!("  Vector: {:?}", numbers);
    
    // Hash Map
    use std::collections::HashMap;
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    println!("  HashMap: {:?}", scores);
}

// Demo 2: Solana Interaction
fn solana_interaction_demo() {
    println!("\nSOLANA INTERACTION DEMO");
    println!("======================\n");
    
    #[cfg(feature = "solana")]
    {
        // Connect to the Solana Devnet
        let url = "https://api.devnet.solana.com".to_string();
        let client = RpcClient::new(url);
        
        // Get the default keypair path
        let default_keypair_path = shellexpand::tilde("~/.config/solana/id.json").to_string();
        
        // Load the wallet keypair
        let keypair_path = Path::new(&default_keypair_path);
        let keypair = match read_keypair_file(keypair_path) {
            Ok(kp) => kp,
            Err(_) => {
                println!("Failed to read keypair from {}", default_keypair_path);
                println!("Make sure you've created a wallet using 'solana-keygen new'");
                return;
            }
        };
        
        // Print wallet information
        let pubkey = keypair.pubkey();
        println!("Wallet Public Key: {}", pubkey);
        
        // Get and print wallet balance
        match client.get_balance(&pubkey) {
            Ok(balance) => {
                println!("Balance: {} SOL", balance as f64 / 1_000_000_000.0);
            },
            Err(err) => {
                println!("Failed to get balance: {}", err);
            }
        }
        
        // Get recent block hash for transaction
        let recent_blockhash = match client.get_latest_blockhash() {
            Ok(hash) => hash,
            Err(err) => {
                println!("Failed to get recent blockhash: {}", err);
                return;
            }
        };
        
        println!("Recent blockhash: {:?}", recent_blockhash);
        
        // Find information about recent transactions
        match client.get_signatures_for_address(&pubkey) {
            Ok(signatures) => {
                println!("\nRecent Transactions:");
                if signatures.is_empty() {
                    println!("No recent transactions found.");
                } else {
                    for (i, sig_info) in signatures.iter().take(5).enumerate() {
                        println!("{}. Signature: {}", i+1, sig_info.signature);
                    }
                }
            },
            Err(err) => {
                println!("Failed to get transaction history: {}", err);
            }
        }
    }
    
    #[cfg(not(feature = "solana"))]
    {
        println!("Solana features are not enabled. To use Solana features:");
        println!("1. Add these dependencies to your Cargo.toml:");
        println!("   solana-sdk = \"1.17.0\"");
        println!("   solana-client = \"1.17.0\"");
        println!("   shellexpand = \"3.1.0\"");
        println!("2. Run cargo with --features=\"solana\"");
        println!("   Or add [features] section to Cargo.toml: solana = []");
    }
}

// Helper functions for Rust basics demo
fn take_ownership(s: String) {
    println!("  Took ownership of: {}", s);
}

fn give_ownership() -> String {
    let s = String::from("returning ownership");
    s // Return s to the caller
}

fn print_status(status: AccountStatus) {
    match status {
        AccountStatus::Active => println!("  Account is active"),
        AccountStatus::Inactive => println!("  Account is inactive"),
        AccountStatus::Locked => println!("  Account is locked"),
    }
}

// Helper function to get user input
fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}