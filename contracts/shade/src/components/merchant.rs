use crate::components::account_factory;
use crate::errors::ContractError;
use crate::events;
use crate::types::{DataKey, Merchant};
use soroban_sdk::{panic_with_error, Address, BytesN, Env};

pub fn register_merchant(env: &Env, merchant_address: Address) -> Address {
    merchant_address.require_auth();

    if env
        .storage()
        .persistent()
        .has(&DataKey::MerchantId(merchant_address.clone()))
    {
        panic_with_error!(env, ContractError::MerchantAlreadyRegistered);
    }

    let merchant_count: u64 = env
        .storage()
        .persistent()
        .get(&DataKey::MerchantCount)
        .unwrap_or(0);

    let new_id = merchant_count + 1;

    let wasm_hash: BytesN<32> = env
        .storage()
        .persistent()
        .get(&DataKey::AccountWasmHash)
        .unwrap_or_else(|| panic_with_error!(env, ContractError::WasmHashNotSet));

    let contract_address = account_factory::deploy_account(
        env,
        merchant_address.clone(),
        env.current_contract_address(),
        new_id,
        wasm_hash,
    );

    let merchant_data = Merchant {
        id: new_id,
        address: merchant_address.clone(),
        active: true,
        verified: false,
        date_registered: env.ledger().timestamp(),
    };

    env.storage()
        .persistent()
        .set(&DataKey::Merchant(new_id), &merchant_data);
    env.storage()
        .persistent()
        .set(&DataKey::MerchantId(merchant_address.clone()), &new_id);
    env.storage()
        .persistent()
        .set(&DataKey::MerchantCount, &new_id);

    events::publish_merchant_registered_event(
        env,
        merchant_address.clone(),
        new_id,
        env.ledger().timestamp(),
    );

    contract_address
}

pub fn get_merchant(env: &Env, merchant_id: u64) -> Merchant {
    if merchant_id == 0 {
        panic_with_error!(env, ContractError::MerchantNotFound);
    }

    let merchant_count: u64 = env
        .storage()
        .persistent()
        .get(&DataKey::MerchantCount)
        .unwrap_or(0);

    if merchant_id > merchant_count {
        panic_with_error!(env, ContractError::MerchantNotFound);
    }

    env.storage()
        .persistent()
        .get(&DataKey::Merchant(merchant_id))
        .unwrap_or_else(|| panic_with_error!(env, ContractError::MerchantNotFound))
}

pub fn is_merchant(env: &Env, merchant: &Address) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::MerchantId(merchant.clone()))
}
