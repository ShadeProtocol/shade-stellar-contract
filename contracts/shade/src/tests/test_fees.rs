use crate::components::platform_fee as platform_fee_component;
use crate::errors::ContractError;
use crate::shade::Shade;
use crate::shade::ShadeClient;
use crate::tests::support::init_hub_with_token;
use soroban_sdk::testutils::{Address as _, Events as _};
use soroban_sdk::{Address, Env, Map, Symbol, TryIntoVal, Val};

fn assert_fee_set_event(
    env: &Env,
    contract_id: &Address,
    expected_token: &Address,
    expected_fee: i128,
    expected_timestamp: u64,
) {
    let events = env.events().all();
    assert!(!events.is_empty());

    let (event_contract_id, topics, data) = events.get(events.len() - 1).unwrap();
    assert_eq!(event_contract_id, contract_id.clone());
    assert_eq!(topics.len(), 1);

    let event_name: Symbol = topics.get(0).unwrap().try_into_val(env).unwrap();
    assert_eq!(event_name, Symbol::new(env, "fee_set_event"));

    let data_map: Map<Symbol, Val> = data.try_into_val(env).unwrap();
    let token_val = data_map.get(Symbol::new(env, "token")).unwrap();
    let fee_val = data_map.get(Symbol::new(env, "fee")).unwrap();
    let timestamp_val = data_map.get(Symbol::new(env, "timestamp")).unwrap();

    let token_in_event: Address = token_val.try_into_val(env).unwrap();
    let fee_in_event: i128 = fee_val.try_into_val(env).unwrap();
    let timestamp_in_event: u64 = timestamp_val.try_into_val(env).unwrap();

    assert_eq!(token_in_event, expected_token.clone());
    assert_eq!(fee_in_event, expected_fee);
    assert_eq!(timestamp_in_event, expected_timestamp);
}

#[test]
fn test_set_fee_success() {
    let env = Env::default();
    let (admin, client, token) = init_hub_with_token(&env);
    let contract_id = client.address.clone();
    let fee: i128 = 500;

    let expected_timestamp = env.ledger().timestamp();

    env.as_contract(&contract_id, || {
        platform_fee_component::set_fee(&env, &admin, &token, fee);
        assert_fee_set_event(&env, &contract_id, &token, fee, expected_timestamp);
    });

    assert_eq!(client.get_fee(&token), fee);
}

#[test]
fn test_set_fee_unaccepted_token() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Shade, ());
    let client = ShadeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    let unaccepted_token = Address::generate(&env);

    let expected_error =
        soroban_sdk::Error::from_contract_error(ContractError::TokenNotAccepted as u32);

    let result = client.try_set_fee(&admin, &unaccepted_token, &100);
    assert!(matches!(result, Err(Ok(err)) if err == expected_error));
}

#[test]
fn test_set_fee_unauthorized() {
    let env = Env::default();
    let (_admin, client, token) = init_hub_with_token(&env);

    let non_admin = Address::generate(&env);

    let expected_error =
        soroban_sdk::Error::from_contract_error(ContractError::NotAuthorized as u32);

    let result = client.try_set_fee(&non_admin, &token, &100);
    assert!(matches!(result, Err(Ok(err)) if err == expected_error));
}

#[test]
fn test_update_fee() {
    let env = Env::default();
    let (admin, client, token) = init_hub_with_token(&env);

    client.set_fee(&admin, &token, &200);
    assert_eq!(client.get_fee(&token), 200);

    client.set_fee(&admin, &token, &750);
    assert_eq!(client.get_fee(&token), 750);
}

#[test]
fn test_get_fee_default_zero() {
    let env = Env::default();
    let (_admin, client, token) = init_hub_with_token(&env);

    assert_eq!(client.get_fee(&token), 0);
}

// No fee set → always returns 0 regardless of amount.
#[test]
fn test_calculate_fee_returns_zero_when_no_fee_set() {
    let env = Env::default();
    let (_admin, client, token) = init_hub_with_token(&env);
    let merchant = Address::generate(&env);
    let contract_id = client.address.clone();
    env.as_contract(&contract_id, || {
        assert_eq!(
            platform_fee_component::calculate_fee(&env, &merchant, &token, 0),
            0
        );
        assert_eq!(
            platform_fee_component::calculate_fee(&env, &merchant, &token, 1_000),
            0
        );
        assert_eq!(
            platform_fee_component::calculate_fee(&env, &merchant, &token, 1_000_000),
            0
        );
    });
}

// Zero amount with any fee → always returns 0.
#[test]
fn test_calculate_fee_zero_amount() {
    let env = Env::default();
    let (admin, client, token) = init_hub_with_token(&env);
    let contract_id = client.address.clone();
    let merchant = Address::generate(&env);

    client.set_fee(&admin, &token, &500); // 5%

    env.as_contract(&contract_id, || {
        assert_eq!(
            platform_fee_component::calculate_fee(&env, &merchant, &token, 0),
            0
        );
    });
}

// 500 bps = 5%: fee on 1_000 should be 50.
#[test]
fn test_calculate_fee_5_percent() {
    let env = Env::default();
    let (admin, client, token) = init_hub_with_token(&env);
    let contract_id = client.address.clone();
    let merchant = Address::generate(&env);

    client.set_fee(&admin, &token, &500);

    env.as_contract(&contract_id, || {
        assert_eq!(
            platform_fee_component::calculate_fee(&env, &merchant, &token, 1_000),
            50
        );
    });
}

// 100 bps = 1%: fee on 10_000 should be 100.
#[test]
fn test_calculate_fee_1_percent() {
    let env = Env::default();
    let (admin, client, token) = init_hub_with_token(&env);
    let contract_id = client.address.clone();
    let merchant = Address::generate(&env);

    client.set_fee(&admin, &token, &100);

    env.as_contract(&contract_id, || {
        assert_eq!(
            platform_fee_component::calculate_fee(&env, &merchant, &token, 10_000),
            100
        );
    });
}

// 1_000 bps = 10%: fee on 5_000 should be 500.
#[test]
fn test_calculate_fee_10_percent() {
    let env = Env::default();
    let (admin, client, token) = init_hub_with_token(&env);
    let contract_id = client.address.clone();
    let merchant = Address::generate(&env);

    client.set_fee(&admin, &token, &1_000);

    env.as_contract(&contract_id, || {
        assert_eq!(
            platform_fee_component::calculate_fee(&env, &merchant, &token, 5_000),
            500
        );
    });
}

// 10_000 bps = 100%: fee equals the full amount.
#[test]
fn test_calculate_fee_100_percent() {
    let env = Env::default();
    let (admin, client, token) = init_hub_with_token(&env);
    let contract_id = client.address.clone();
    let merchant = Address::generate(&env);

    client.set_fee(&admin, &token, &10_000);

    env.as_contract(&contract_id, || {
        assert_eq!(
            platform_fee_component::calculate_fee(&env, &merchant, &token, 1_000),
            1_000
        );
    });
}

// 1 bps on an amount that produces a sub-unit result truncates toward zero.
// 1 bps on 1 → (1 * 1) / 10_000 = 0 (integer truncation).
#[test]
fn test_calculate_fee_truncates_fractional_result() {
    let env = Env::default();
    let (admin, client, token) = init_hub_with_token(&env);
    let contract_id = client.address.clone();
    let merchant = Address::generate(&env);

    client.set_fee(&admin, &token, &1); // 0.01%

    env.as_contract(&contract_id, || {
        // Too small to produce a whole unit
        assert_eq!(
            platform_fee_component::calculate_fee(&env, &merchant, &token, 1),
            0
        );
        assert_eq!(
            platform_fee_component::calculate_fee(&env, &merchant, &token, 9_999),
            0
        );
        // Exactly at the boundary: 10_000 * 1 / 10_000 = 1
        assert_eq!(
            platform_fee_component::calculate_fee(&env, &merchant, &token, 10_000),
            1
        );
    });
}

// Updating the fee changes the computed amount accordingly.
#[test]
fn test_calculate_fee_reflects_updated_fee() {
    let env = Env::default();
    let (admin, client, token) = init_hub_with_token(&env);
    let contract_id = client.address.clone();
    let merchant = Address::generate(&env);

    client.set_fee(&admin, &token, &200); // 2%

    env.as_contract(&contract_id, || {
        assert_eq!(
            platform_fee_component::calculate_fee(&env, &merchant, &token, 10_000),
            200
        );
    });

    client.set_fee(&admin, &token, &500); // updated to 5%

    env.as_contract(&contract_id, || {
        assert_eq!(
            platform_fee_component::calculate_fee(&env, &merchant, &token, 10_000),
            500
        );
    });
}

// Large amounts stay within i128 range and produce correct results.
// 250 bps (2.5%) on 1_000_000_000 should be 25_000_000.
#[test]
fn test_calculate_fee_large_amount() {
    let env = Env::default();
    let (admin, client, token) = init_hub_with_token(&env);
    let contract_id = client.address.clone();
    let merchant = Address::generate(&env);

    client.set_fee(&admin, &token, &250); // 2.5%

    env.as_contract(&contract_id, || {
        assert_eq!(
            platform_fee_component::calculate_fee(&env, &merchant, &token, 1_000_000_000),
            25_000_000
        );
    });
}

// Each token tracks its own fee independently.
#[test]
fn test_calculate_fee_per_token_independence() {
    let env = Env::default();
    let (admin, client, token_a) = init_hub_with_token(&env);
    let contract_id = client.address.clone();
    let merchant = Address::generate(&env);

    // Register a second token
    let token_b_admin = Address::generate(&env);
    let token_b = env
        .register_stellar_asset_contract_v2(token_b_admin)
        .address();
    client.add_accepted_token(&admin, &token_b);

    client.set_fee(&admin, &token_a, &300); // 3%
    client.set_fee(&admin, &token_b, &700); // 7%

    env.as_contract(&contract_id, || {
        assert_eq!(
            platform_fee_component::calculate_fee(&env, &merchant, &token_a, 10_000),
            300
        );
        assert_eq!(
            platform_fee_component::calculate_fee(&env, &merchant, &token_b, 10_000),
            700
        );
    });
}
