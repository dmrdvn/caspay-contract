//! Deploy CasPay Contract to Casper Testnet
//! 
//! Usage: cargo run --bin deploy_testnet --features=livenet

use odra::host::Deployer;
use odra::prelude::Addressable;
use caspay::CasPay;
use caspay::caspay::CasPayInitArgs;

fn main() {
    // Load environment from .env file
    let env = odra_casper_livenet_env::env();
    
    // Admin will be the deployer (env.caller())
    let admin = env.caller();
    
    println!("🚀 Deploying CasPay Contract to Casper Testnet...");
    println!("📋 Admin (Deployer): {:?}", admin);
    println!("🔑 Deployer will become contract admin");
    
    // Set gas limit for deployment (450 CSPR)
    env.set_gas(450_000_000_000u64);
    
    // Deploy contract with admin address
    let init_args = CasPayInitArgs { admin };
    let mut contract = CasPay::deploy(&env, init_args);
    
    println!("✅ Contract deployed successfully!");
    println!("📝 Contract address: {:?}", contract.address());
    println!();
    
    // Register first merchant to trigger auto-initialization
    println!("🔧 Registering first merchant to initialize contract...");
    env.set_gas(10_000_000_000u64); // 10 CSPR
    
    let test_merchant_id = String::from("00000000-0000-0000-0000-000000000001");
    let test_wallet = env.caller(); // Use deployer as test wallet
    
    contract.register_merchant(test_merchant_id.clone(), test_wallet);
    println!("✅ First merchant registered - contract initialized!");
    println!("   Test merchant ID: {}", test_merchant_id);
    println!("   Admin: {:?}", env.caller());
    println!();
    
    // Verify initialization
    println!("🔍 Verifying initialization...");
    let merchant_check = contract.get_merchant(test_merchant_id);
    println!("   Merchant registered: {:?}", merchant_check.is_some());
    println!();
    
    println!("🎉 Deployment complete!");
    println!("📊 Check your deployment on: https://testnet.cspr.live/");
}
