#![cfg(test)]
extern crate std;

use crate::shade::{Shade, ShadeClient};
use soroban_sdk::testutils::{Address as _, Events, Ledger as _};
use soroban_sdk::{Address, Env, String};

fn setup_env() -> (Env, ShadeClient<'static>, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let shade_id = env.register(Shade, ());
    let client = ShadeClient::new(&env, &shade_id);
    let admin = Address::generate(&env);
    client.initialize(&admin);

    let merchant = Address::generate(&env);
    client.register_merchant(&merchant);

    (env, client, admin, merchant)
}

#[test]
fn test_scaffold() {
    let (_env, _client, _admin, _merchant) = setup_env();
    assert!(true);
}
