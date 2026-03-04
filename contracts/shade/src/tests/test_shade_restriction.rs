#![cfg(test)]

use crate::errors::ContractError;
use crate::shade::{Shade, ShadeClient};
use crate::types::Role;
use account::account::{MerchantAccount, MerchantAccountClient};
use soroban_sdk::testutils::{Address as _, Events as _};
use soroban_sdk::{Address, Env, FromVal, Symbol};

fn setup_test_with_account<'a>() -> (
    Env,
    ShadeClient<'a>,
    Address,
    MerchantAccountClient<'a>,
    Address,
    Address,
) {
    let env = Env::default();
    env.mock_all_auths();

    // Register Shade
    let shade_contract_id = env.register(Shade, ());
    let shade_client = ShadeClient::new(&env, &shade_contract_id);

    // Initialize Shade
    let admin = Address::generate(&env);
    shade_client.initialize(&admin);

    // Register Merchant
    let merchant = Address::generate(&env);
    shade_client.register_merchant(&merchant);

    // Register MerchantAccount
    let account_id = env.register(MerchantAccount, ());
    let account_client = MerchantAccountClient::new(&env, &account_id);

    // Initialize MerchantAccount with Shade as the manager
    account_client.initialize(&merchant, &shade_contract_id, &1_u64);

    // Link account to merchant in Shade
    shade_client.set_merchant_account(&merchant, &account_id);

    (
        env,
        shade_client,
        shade_contract_id,
        account_client,
        admin,
        merchant,
    )
}

#[test]
fn test_admin_restriction() {
    let (env, shade_client, shade_id, account_client, admin, merchant) =
        setup_test_with_account();

    assert!(!account_client.is_restricted_account());

    // Call from Admin address
    shade_client.restrict_merchant_account(&admin, &merchant, &true);

    // Verify Merchant Account state changes to true
    assert!(account_client.is_restricted_account());

    // Check for Shade event
    let events = env.events().all();
    let mut found = false;
    for event in events.iter() {
        if event.0 == shade_id {
            if let Some(topic_val) = event.1.first() {
                let topic = Symbol::from_val(&env, &topic_val);
                if topic == Symbol::new(&env, "AccountRestrictedEvent") {
                    found = true;
                    break;
                }
            }
        }
    }
    assert!(found, "AccountRestrictedEvent not emitted!");
}

#[test]
fn test_manager_unrestriction() {
    let (env, shade_client, _shade_id, account_client, admin, merchant) =
        setup_test_with_account();

    // Admin restricts first
    shade_client.restrict_merchant_account(&admin, &merchant, &true);
    assert!(account_client.is_restricted_account());

    // Create a Manager
    let manager = Address::generate(&env);
    shade_client.grant_role(&admin, &manager, &Role::Manager);

    // Manager unrestricts
    shade_client.restrict_merchant_account(&manager, &merchant, &false);

    // Verify state changes to false
    assert!(!account_client.is_restricted_account());
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")] // NotAuthorized
fn test_unauthorized_access_random_user() {
    let (env, shade_client, _shade_id, _account_client, _admin, merchant) =
        setup_test_with_account();

    let random_user = Address::generate(&env);

    // Using random_user to try and restrict
    shade_client.restrict_merchant_account(&random_user, &merchant, &true);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")] // NotAuthorized
fn test_unauthorized_access_merchant() {
    let (_env, shade_client, _shade_id, _account_client, _admin, merchant) =
        setup_test_with_account();

    // Using the merchant themselves
    shade_client.restrict_merchant_account(&merchant, &merchant, &true);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #6)")] // MerchantNotFound
fn test_invalid_merchant() {
    let (env, shade_client, _shade_id, _account_client, admin, _merchant) =
        setup_test_with_account();

    let unregistered_merchant = Address::generate(&env);

    // Call for an address that was never registered
    shade_client.restrict_merchant_account(&admin, &unregistered_merchant, &true);
}
