#![cfg(test)]

use crate::{
    shade::{Shade, ShadeClient},
    types::{DataKey, Merchant},
};
use soroban_sdk::{
    testutils::{Address as _, Events},
    vec, Address, BytesN, Env, IntoVal, Symbol,
};

// We need the account contract compiled so we can deploy it.
// The account contract seems to be compiled into `target/wasm32-unknown-unknown/release/account.wasm`.
// Actually, Soroban provides a way to register contracts directly for testing.

// Stub the account contract for testing the deployed contract initialization
mod account_contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/account.wasm"
    );
}

// We need to alias the imported WASM const to use it
pub use account_contract::WASM as ACCOUNT_WASM;

#[test]
fn test_register_merchant() {
    let env = Env::default();
    env.mock_all_auths();

    // Register Shade contract
    let shade_id = env.register(Shade, ());
    let shade_client = ShadeClient::new(&env, &shade_id);

    // Initialize Shade contract
    let admin = Address::generate(&env);
    shade_client.initialize(&admin);

    // Register Account contract to simulate getting its wasm hash
    let wasm_hash = env.deployer().upload_contract_wasm(ACCOUNT_WASM);

    // Define merchant properties
    let merchant_address = Address::generate(&env);
    let manager_address = Address::generate(&env);

    // Register a new merchant
    let deployed_contract: Address = shade_client.register_merchant(
        &merchant_address,
        &manager_address,
        &wasm_hash,
    );

    // Check emitted events
    let events = env.events().all();
    assert!(events.len() > 0);
    // You could assert the exact structure of the `MerchantAccountDeployedEvent` here

    // Ensure state updated properly in the Shade contract (would require `get_merchant` or peeking at storage, 
    // assuming Shade has a way to retrieve merchants. If not, this serves as integration.)

    // Also assert that the deployed contract has the initialized data

    // Verify it isn't deployed to the same address if another merchant is registered
    let merchant2_address = Address::generate(&env);
    let manager2_address = Address::generate(&env);
    
    let deployed_contract_2: Address = shade_client.register_merchant(
        &merchant2_address,
        &manager2_address,
        &wasm_hash,
    );

    assert_ne!(deployed_contract, deployed_contract_2);
}
