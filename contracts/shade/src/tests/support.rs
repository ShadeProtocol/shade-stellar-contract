use crate::components::merchant;
use crate::shade::{Shade, ShadeClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env};

/// Links an arbitrary address as a merchant's account, skipping the factory.
/// Production only links accounts it deployed; tests that exercise payment routing
/// rather than the account itself can use a plain address and read its balance.
pub trait LinkMerchantAccount {
    fn link_merchant_account(&self, merchant: &Address, account: &Address);
}

impl LinkMerchantAccount for ShadeClient<'_> {
    fn link_merchant_account(&self, merchant_address: &Address, account: &Address) {
        self.env.as_contract(&self.address, || {
            let merchant_id = merchant::get_merchant_id(&self.env, merchant_address);
            merchant::link_account(&self.env, merchant_id, account);
        });
    }
}

/// Deploys the hub with all auths mocked and initializes it with a fresh admin.
pub fn init_hub(env: &Env) -> (Address, ShadeClient<'_>) {
    env.mock_all_auths();
    let contract_id = env.register(Shade, ());
    let client = ShadeClient::new(env, &contract_id);
    let admin = Address::generate(env);
    client.initialize(&admin);
    (admin, client)
}

/// Deploys a Stellar asset contract and returns its address.
pub fn deploy_token(env: &Env) -> Address {
    let token_admin = Address::generate(env);
    env.register_stellar_asset_contract_v2(token_admin)
        .address()
}

/// Hub plus one accepted token.
pub fn init_hub_with_token(env: &Env) -> (Address, ShadeClient<'_>, Address) {
    let (admin, client) = init_hub(env);
    let token = deploy_token(env);
    client.add_accepted_token(&admin, &token);
    (admin, client, token)
}
