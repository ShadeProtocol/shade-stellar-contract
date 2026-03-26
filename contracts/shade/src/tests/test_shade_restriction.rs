#![cfg(test)]

use crate::shade::{Shade, ShadeClient};
use crate::types::Role;
use soroban_sdk::testutils::{Address as _, Events as _};
use soroban_sdk::{Address, Env, Map, Symbol, TryIntoVal, Val};

fn setup_test() -> (
    Env,
    ShadeClient<'static>,
    Address, // Shade contract ID
    Address, // Admin
    Address, // Manager
    Address, // Merchant
    Address, // Account
) {
    let env = Env::default();
    env.mock_all_auths();

    // Deploy Shade
    let shade_id = env.register(Shade, ());
    let shade_client = ShadeClient::new(&env, &shade_id);

    // Initialize Shade
    let admin = Address::generate(&env);
    shade_client.initialize(&admin);

    // Create Manager and grant role
    let manager = Address::generate(&env);
    shade_client.grant_role(&admin, &manager, &Role::Manager);

    // Register Merchant
    let merchant = Address::generate(&env);
    shade_client.register_merchant(&merchant);

    // Deploy Account
    let account_id = env.register(account::account::MerchantAccount, ());
    let account_client = account::account::MerchantAccountClient::new(&env, &account_id);
    
    // Initialize Account
    let merchant_id = shade_client.get_merchant_id(&merchant);
    // MerchantAccount expects: (merchant: Address, shade_contract: Address, merchant_id: u64)
    account_client.initialize(&merchant, &shade_id, &merchant_id);

    // Link Account in Shade
    shade_client.set_merchant_account(&merchant, &account_id);

    (
        env,
        shade_client,
        shade_id,
        admin,
        manager,
        merchant,
        account_id,
    )
}

fn assert_latest_account_restricted_event(
    env: &Env,
    contract_id: &Address,
    expected_merchant: &Address,
    expected_status: bool,
    expected_caller: &Address,
) {
    let events = env.events().all();
    let mut found = false;
    for i in (0..events.len()).rev() {
        let (event_contract_id, topics, data) = events.get(i).unwrap();
        if event_contract_id == contract_id.clone() && topics.len() >= 1 {
            // Check if this is the AccountRestrictedEvent
            // For now, we rely on the data payload matching the expected types, avoiding fragile Symbol checks
            // as topic names depend on the Rust SDK version and exact struct name.
            if let Ok(data_map) = data.try_into_val::<Env, Map<Symbol, Val>>(env) {
                if let (Some(merchant_val), Some(status_val), Some(caller_val)) = (
                    data_map.get(Symbol::new(env, "merchant")),
                    data_map.get(Symbol::new(env, "status")),
                    data_map.get(Symbol::new(env, "caller")),
                ) {
                    if let (Ok(merchant_addr), Ok(status), Ok(caller_addr)) = (
                        merchant_val.try_into_val::<Env, Address>(env),
                        status_val.try_into_val::<Env, bool>(env),
                        caller_val.try_into_val::<Env, Address>(env),
                    ) {
                        if merchant_addr == expected_merchant.clone()
                            && status == expected_status
                            && caller_addr == expected_caller.clone()
                        {
                            found = true;
                            break;
                        }
                    }
                }
            }
        }
    }
    assert!(found, "AccountRestrictedEvent not found in events");
}

#[test]
fn test_admin_restrict_merchant_account() {
    let (env, shade_client, shade_id, admin, _manager, merchant, account_id) = setup_test();

    let account_client = account::account::MerchantAccountClient::new(&env, &account_id);

    // Initially un-restricted
    assert_eq!(account_client.is_restricted_account(), false);

    // Admin restricts
    shade_client.restrict_merchant_account(&admin, &merchant, &true);

    // Verify it is restricted
    assert_eq!(account_client.is_restricted_account(), true);

    // Verify event
    assert_latest_account_restricted_event(&env, &shade_id, &merchant, true, &admin);
}

#[test]
fn test_manager_restrict_merchant_account() {
    let (env, shade_client, _shade_id, _admin, manager, merchant, account_id) = setup_test();

    let account_client = account::account::MerchantAccountClient::new(&env, &account_id);

    // Initially un-restricted
    assert_eq!(account_client.is_restricted_account(), false);

    // Manager restricts
    shade_client.restrict_merchant_account(&manager, &merchant, &true);

    // Verify it is restricted
    assert_eq!(account_client.is_restricted_account(), true);

    // Manager un-restricts
    shade_client.restrict_merchant_account(&manager, &merchant, &false);

    // Verify it is un-restricted
    assert_eq!(account_client.is_restricted_account(), false);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #1)")] // ContractError::NotAuthorized is 1. We'll use prefix match or general ignore
fn test_unauthorized_restrict_merchant_account() {
    let (env, shade_client, _shade_id, _admin, _manager, merchant, _account_id) = setup_test();

    let random_user = Address::generate(&env);

    // Need to use try_... or expect panic
    shade_client.restrict_merchant_account(&random_user, &merchant, &true);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #6)")] // ContractError::MerchantNotFound
fn test_invalid_merchant_restriction() {
    let (env, shade_client, _shade_id, admin, _manager, _merchant, _account_id) = setup_test();

    let invalid_merchant = Address::generate(&env);

    shade_client.restrict_merchant_account(&admin, &invalid_merchant, &true);
}
