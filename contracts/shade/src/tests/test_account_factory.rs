#![cfg(test)]

use crate::errors::ContractError;
use crate::shade::{Shade, ShadeClient};
use soroban_sdk::testutils::{Address as _, Events as _};
use soroban_sdk::{Address, BytesN, Env, Map, Symbol, TryIntoVal, Val};

fn setup() -> (Env, ShadeClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Shade, ());
    let client = ShadeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let account_wasm_hash = BytesN::from_array(&env, &[0; 32]);
    client.initialize(&admin, &account_wasm_hash);

    (env, client, contract_id)
}

fn assert_latest_merchant_account_deployed_event(
    env: &Env,
    contract_id: &Address,
    expected_merchant: &Address,
    expected_contract: &Address,
) {
    let events = env.events().all();
    assert!(!events.is_empty());

    let mut found = false;
    for i in (0..events.len()).rev() {
        let (event_contract_id_i, topics_i, data_i) = events.get(i).unwrap();
        if topics_i.len() == 1 {
            let event_name: Symbol = topics_i.get(0).unwrap().try_into_val(env).unwrap();
            if event_name == Symbol::new(env, "merchant_account_deployed_event") {
                assert_eq!(event_contract_id_i, contract_id.clone());
                let data_map: Map<Symbol, Val> = data_i.try_into_val(env).unwrap();
                let merchant_val = data_map.get(Symbol::new(env, "merchant")).unwrap();
                let contract_val = data_map.get(Symbol::new(env, "contract")).unwrap();

                let merchant_in_event: Address = merchant_val.try_into_val(env).unwrap();
                let contract_in_event: Address = contract_val.try_into_val(env).unwrap();

                assert_eq!(merchant_in_event, expected_merchant.clone());
                assert_eq!(contract_in_event, expected_contract.clone());
                found = true;
                break;
            }
        }
    }
    assert!(found, "merchant_account_deployed_event not found in events");
}

#[test]
fn test_register_merchant_deploys_account_and_links() {
    let (env, client, contract_id) = setup();

    let merchant = Address::generate(&env);
    client.register_merchant(&merchant);

    let merchant_data = client.get_merchant(&1u64);
    let account_addr = client.get_merchant_account(&1u64);

    assert_eq!(merchant_data.account, account_addr);
    assert_latest_merchant_account_deployed_event(&env, &contract_id, &merchant, &account_addr);

    let account_client = account::account::MerchantAccountClient::new(&env, &account_addr);
    assert_eq!(account_client.get_merchant(), merchant);
}

#[test]
fn test_register_multiple_merchants_have_unique_accounts() {
    let (env, client, _contract_id) = setup();

    let merchant_a = Address::generate(&env);
    let merchant_b = Address::generate(&env);

    client.register_merchant(&merchant_a);
    client.register_merchant(&merchant_b);

    let account_a = client.get_merchant_account(&1u64);
    let account_b = client.get_merchant_account(&2u64);
    assert_ne!(account_a, account_b);
}

#[test]
fn test_register_merchant_missing_wasm_hash_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Shade, ());
    let client = ShadeClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let account_wasm_hash = BytesN::from_array(&env, &[0; 32]);
    client.initialize(&admin, &account_wasm_hash);

    env.as_contract(&contract_id, || {
        env.storage()
            .persistent()
            .remove(&crate::types::DataKey::MerchantAccountWasmHash);
    });

    let merchant = Address::generate(&env);
    let expected_error =
        soroban_sdk::Error::from_contract_error(ContractError::WasmHashNotSet as u32);
    let result = client.try_register_merchant(&merchant);
    assert!(matches!(result, Err(Ok(err)) if err == expected_error));
}
