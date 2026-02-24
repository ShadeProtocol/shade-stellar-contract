#![cfg(test)]

use crate::shade::{Shade, ShadeClient};
use soroban_sdk::testutils::{Address as _, Events as _};
use soroban_sdk::{Address, Env, Map, Symbol, TryIntoVal, Val};

pub fn assert_token_event(
    env: &Env,
    contract_id: &Address,
    expected_event_symbol: &str,
    expected_token: &Address,
    expected_timestamp: u64,
) {
    let events = env.events().all();
    let expected_symbol = Symbol::new(env, expected_event_symbol);

    let mut found = false;
    for i in (0..events.len()).rev() {
        let (event_contract_id, topics, data) = events.get(i).unwrap();
        if event_contract_id != contract_id.clone() {
            continue;
        }

        if topics.len() > 0 {
            let event_name: Symbol = topics.get(0).unwrap().try_into_val(env).unwrap();
            if event_name == expected_symbol {
                let data_map: Map<Symbol, Val> = data.try_into_val(env).unwrap();
                let token_val = data_map.get(Symbol::new(env, "token")).unwrap();
                let timestamp_val = data_map.get(Symbol::new(env, "timestamp")).unwrap();

                let token_in_event: Address = token_val.try_into_val(env).unwrap();
                let timestamp_in_event: u64 = timestamp_val.try_into_val(env).unwrap();

                assert_eq!(token_in_event, expected_token.clone());
                assert_eq!(timestamp_in_event, expected_timestamp);
                found = true;
                break;
            }
        }
    }

    assert!(
        found,
        "Event {:?} not found for contract {:?}",
        expected_event_symbol, contract_id
    );
}

fn create_token(env: &Env) -> Address {
    let admin = Address::generate(env);
    env.register_stellar_asset_contract_v2(admin).address()
}

#[test]
fn test_admin_adds_token_and_emits_event() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Shade, ());
    let client = ShadeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    let token = create_token(&env);
    let expected_timestamp = env.ledger().timestamp();

    client.add_accepted_token(&admin, &token);

    assert!(client.is_accepted_token(&token));
    assert_token_event(
        &env,
        &contract_id,
        "TokenAddedEvent",
        &token,
        expected_timestamp,
    );
}

#[test]
fn test_admin_removes_token_and_emits_event() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Shade, ());
    let client = ShadeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    let token = create_token(&env);
    client.add_accepted_token(&admin, &token);

    let expected_timestamp = env.ledger().timestamp();

    client.remove_accepted_token(&admin, &token);

    assert!(!client.is_accepted_token(&token));
    assert_token_event(
        &env,
        &contract_id,
        "TokenRemovedEvent",
        &token,
        expected_timestamp,
    );
}

#[test]
fn test_batch_add_tokens() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Shade, ());
    let client = ShadeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    let token1 = create_token(&env);
    let token2 = create_token(&env);
    let token3 = create_token(&env);

    let mut tokens = soroban_sdk::Vec::new(&env);
    tokens.push_back(token1.clone());
    tokens.push_back(token2.clone());
    tokens.push_back(token3.clone());

    client.add_accepted_tokens(&admin, &tokens);

    assert!(client.is_accepted_token(&token1));
    assert!(client.is_accepted_token(&token2));
    assert!(client.is_accepted_token(&token3));

    assert_token_event(
        &env,
        &contract_id,
        "TokenAddedEvent",
        &token3,
        env.ledger().timestamp(),
    );
}

#[test]
fn test_non_admin_cannot_add_or_remove_tokens() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Shade, ());
    let client = ShadeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    let non_admin = Address::generate(&env);
    let token = create_token(&env);

    let result = client.try_add_accepted_token(&non_admin, &token);
    assert!(result.is_err());

    client.add_accepted_token(&admin, &token);
    let result = client.try_remove_accepted_token(&non_admin, &token);
    assert!(result.is_err());
}

#[test]
fn test_duplicate_add_is_handled_gracefully() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Shade, ());
    let client = ShadeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.initialize(&admin);

    let token = create_token(&env);
    client.add_accepted_token(&admin, &token);

    client.add_accepted_token(&admin, &token);
    assert!(client.is_accepted_token(&token));
}
