#![cfg(test)]

use crate::components::reentrancy;
use crate::errors::ContractError;
use crate::shade::Shade;
use crate::shade::ShadeClient;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env};

/// Helper: registers and returns a fresh env + contract + client + admin.
fn setup() -> (Env, ShadeClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Shade, ());
    let client = ShadeClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.initialize(&admin);
    (env, client, admin)
}

// ── enter() panics when called twice without exit() ──────────────────────────

#[should_panic(expected = "HostError: Error(Contract, #4)")]
#[test]
fn test_reentrancy_enter_twice_panics() {
    let env = Env::default();
    let contract_id = env.register(Shade, ());

    env.as_contract(&contract_id, || {
        reentrancy::enter(&env); // first enter – OK
        reentrancy::enter(&env); // second enter – must panic with Reentrancy (#4)
    });
}

// ── exit() clears the lock; a subsequent enter() must succeed ─────────────────

#[test]
fn test_reentrancy_enter_exit_enter_ok() {
    let env = Env::default();
    let contract_id = env.register(Shade, ());

    env.as_contract(&contract_id, || {
        reentrancy::enter(&env);
        reentrancy::exit(&env);
        reentrancy::enter(&env); // should not panic
        reentrancy::exit(&env);
    });
}

// ── High-level: add_accepted_token guard ────────────────────────────────────

#[should_panic(expected = "HostError: Error(Contract, #4)")]
#[test]
fn test_add_accepted_token_reentrancy_guard() {
    let (env, _client, admin) = setup();
    let contract_id = env.register(Shade, ());

    // Manually set the reentrancy lock so the next high-level call sees it locked.
    env.as_contract(&contract_id, || {
        reentrancy::enter(&env);
    });

    // Re-initialize this second contract so admin works.
    let client2 = ShadeClient::new(&env, &contract_id);
    client2.initialize(&admin);

    let token_admin = Address::generate(&env);
    let token = env
        .register_stellar_asset_contract_v2(token_admin)
        .address();

    // Should panic with Reentrancy (#4) because the lock is already set.
    client2.add_accepted_token(&admin, &token);
}

// ── High-level: register_merchant guard ─────────────────────────────────────

#[should_panic(expected = "HostError: Error(Contract, #4)")]
#[test]
fn test_register_merchant_reentrancy_guard() {
    let (env, _client, _admin) = setup();
    let contract_id = env.register(Shade, ());

    // Pre-lock the guard.
    env.as_contract(&contract_id, || {
        reentrancy::enter(&env);
    });

    let client2 = ShadeClient::new(&env, &contract_id);
    let admin2 = Address::generate(&env);
    client2.initialize(&admin2);

    let merchant = Address::generate(&env);
    // Should panic with Reentrancy (#4).
    client2.register_merchant(&merchant);
}

// ── High-level: create_invoice guard ────────────────────────────────────────

#[should_panic(expected = "HostError: Error(Contract, #4)")]
#[test]
fn test_create_invoice_reentrancy_guard() {
    let (env, client, _admin) = setup();
    let contract_id = env.register(Shade, ());

    let merchant = Address::generate(&env);
    client.register_merchant(&merchant);

    // Pre-lock on the SAME contract used by client.
    // We need the contract_id that client uses. Re-derive it from the original setup.
    // Instead, lock directly via as_contract on the first registered contract.
    // Since setup() uses env.register() internally, we do it differently:
    let env2 = Env::default();
    env2.mock_all_auths();
    let cid2 = env2.register(Shade, ());
    let client2 = ShadeClient::new(&env2, &cid2);
    let admin2 = Address::generate(&env2);
    client2.initialize(&admin2);

    let merchant2 = Address::generate(&env2);
    client2.register_merchant(&merchant2);

    // Pre-lock
    env2.as_contract(&cid2, || {
        reentrancy::enter(&env2);
    });

    use soroban_sdk::String;
    // Should panic with Reentrancy (#4).
    client2.create_invoice(
        &merchant2,
        &String::from_str(&env2, "test"),
        &100,
        &Address::generate(&env2),
    );
}

// ── Verify error code is exactly #4 ─────────────────────────────────────────

#[test]
fn test_reentrancy_error_code_is_4() {
    assert_eq!(ContractError::Reentrancy as u32, 4);
}
