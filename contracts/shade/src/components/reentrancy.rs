use crate::errors::ContractError;
use crate::types::DataKey;
use soroban_sdk::{panic_with_error, Env};

pub fn enter(env: &Env) {
    if env.storage().instance().has(&DataKey::ReentrancyStatus) {
        panic_with_error!(env, ContractError::Reentrancy);
    }
    env.storage()
        .instance()
        .set(&DataKey::ReentrancyStatus, &true);
}

pub fn exit(env: &Env) {
    env.storage()
        .instance()
        .remove(&DataKey::ReentrancyStatus);
}
