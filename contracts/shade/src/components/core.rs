use crate::errors::ContractError;
use crate::types::DataKey;
use soroban_sdk::{panic_with_error, Address, Env};

pub fn get_admin(env: &Env) -> Address {
    env.storage()
        .persistent()
        .get(&DataKey::Admin)
        .unwrap_or_else(|| panic_with_error!(env, ContractError::NotInitialized))
}

pub fn assert_admin(env: &Env, admin: &Address) {
    if *admin != get_admin(env) {
        panic_with_error!(env, ContractError::NotAuthorized);
    }
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().persistent().set(&DataKey::Admin, admin);
}
