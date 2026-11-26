#![cfg(test)]

use crate::shade::Shade;
use crate::shade::ShadeClient;
use crate::tests::support::LinkMerchantAccount as _;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env, String};

fn setup(env: &Env, fee_bps: i128) -> (Address, ShadeClient<'_>, Address, Address, Address) {
    env.mock_all_auths();
    let contract_id = env.register(Shade, ());
    let client = ShadeClient::new(env, &contract_id);

    let admin = Address::generate(env);
    client.initialize(&admin);

    let token_admin = Address::generate(env);
    let token = env
        .register_stellar_asset_contract_v2(token_admin)
        .address();
    client.add_accepted_token(&admin, &token);
    if fee_bps > 0 {
        client.set_fee(&admin, &token, &fee_bps);
    }

    let merchant = Address::generate(env);
    client.register_merchant(&merchant);
    client.verify_merchant(&admin, &1, &true);

    // Set a distinct merchant account address (a plain wallet), same pattern as test_payment tests.
    let merchant_account = Address::generate(env);
    client.link_merchant_account(&merchant, &merchant_account);

    (admin, client, token, merchant, merchant_account)
}

#[test]
fn test_fee_discounts_based_on_volume() {
    let env = Env::default();
    let (_admin, client, token, merchant, merchant_account) = setup(&env, 1000);
    let payer = Address::generate(&env);

    // Fund the payer
    let token_client = soroban_sdk::token::StellarAssetClient::new(&env, &token);
    token_client.mint(&payer, &1_000_000);

    let token_balance_client = soroban_sdk::token::TokenClient::new(&env, &token);

    // Volume 0 -> Fee 10% (100). Merchant receives 900. Volume becomes 1,000.
    let inv1 = client.create_invoice(
        &merchant,
        &String::from_str(&env, "inv1"),
        &1000,
        &token,
        &None,
    );
    client.pay_invoice(&payer, &inv1);

    // Volume 1,000 -> Tier 0. Fee 10% (900). Merchant receives 8,100. Volume becomes 10,000.
    let inv2 = client.create_invoice(
        &merchant,
        &String::from_str(&env, "inv2"),
        &9000,
        &token,
        &None,
    );
    client.pay_invoice(&payer, &inv2);

    // Volume 10,000 -> Tier 1. Discount 10% -> Fee 9% (90). Merchant receives 910. Volume becomes 11,000.
    let inv3 = client.create_invoice(
        &merchant,
        &String::from_str(&env, "inv3"),
        &1000,
        &token,
        &None,
    );
    client.pay_invoice(&payer, &inv3);

    // 900 + 8100 + 910 = 9910.
    assert_eq!(token_balance_client.balance(&merchant_account), 9910);

    // Volume 11,000 -> Tier 1. Fee 9% (3510). Merchant receives 35,490. Volume becomes 50,000.
    let inv4 = client.create_invoice(
        &merchant,
        &String::from_str(&env, "inv4"),
        &39000,
        &token,
        &None,
    );
    client.pay_invoice(&payer, &inv4);

    // Volume 50,000 -> Tier 2. Discount 25% -> Fee 7.5% (75). Merchant receives 925. Volume becomes 51,000.
    let inv5 = client.create_invoice(
        &merchant,
        &String::from_str(&env, "inv5"),
        &1000,
        &token,
        &None,
    );
    client.pay_invoice(&payer, &inv5);

    // 9910 + 35490 + 925 = 46325.
    assert_eq!(token_balance_client.balance(&merchant_account), 46325);
}

#[test]
fn test_volume_tracking_tier_3() {
    let env = Env::default();
    let (_admin, client, token, merchant, merchant_account) = setup(&env, 1000);
    let payer = Address::generate(&env);

    let token_client = soroban_sdk::token::StellarAssetClient::new(&env, &token);
    token_client.mint(&payer, &1_000_000);

    let token_balance_client = soroban_sdk::token::TokenClient::new(&env, &token);

    // Volume 0 -> Tier 0. Fee 10% (20,000). Merchant receives 180,000. Volume becomes 200,000.
    let inv1 = client.create_invoice(
        &merchant,
        &String::from_str(&env, "inv_tier3"),
        &200000,
        &token,
        &None,
    );
    client.pay_invoice(&payer, &inv1);

    // Volume 200,000 -> Tier 3. Discount 50% -> Fee 5% (50). Merchant receives 950.
    let inv2 = client.create_invoice(
        &merchant,
        &String::from_str(&env, "inv_tier3_next"),
        &1000,
        &token,
        &None,
    );
    client.pay_invoice(&payer, &inv2);

    // 180,000 + 950 = 180,950.
    assert_eq!(token_balance_client.balance(&merchant_account), 180950);
}

#[test]
fn test_no_discount_below_threshold() {
    let env = Env::default();
    let (_admin, client, token, merchant, _) = setup(&env, 500);

    let fee = client.calculate_fee(&merchant, &token, &10_000_000);
    assert_eq!(fee, 10_000_000i128 * 500 / 10_000);
}

#[test]
fn test_zero_fee_returns_zero() {
    let env = Env::default();
    let (_admin, client, token, merchant, _) = setup(&env, 0);

    assert_eq!(client.calculate_fee(&merchant, &token, &10_000_000), 0);
}

// TODO: fix commented tests

// #[test]
// fn test_discount_tier_1_at_10k_volume() {
//     let env = Env::default();
//     let (_admin, client, contract_id, token) = setup(&env);
//     let merchant = Address::generate(&env);
//     client.register_merchant(&merchant);

//     env.as_contract(&contract_id, || {
//         admin_component::increment_merchant_volume(&env, &merchant, &token, 10_000);
//     });

//     let fee = client.calculate_fee(&merchant, &token, &10_000_000);
//     let base_fee = 10_000_000i128 * 500 / 10_000;
//     let expected = base_fee - (base_fee * 10 / 10_000);
//     assert_eq!(fee, expected);
// }

// #[test]
// fn test_discount_tier_2_at_50k_volume() {
//     let env = Env::default();
//     let (_admin, client, contract_id, token) = setup(&env);
//     let merchant = Address::generate(&env);
//     client.register_merchant(&merchant);

//     env.as_contract(&contract_id, || {
//         admin_component::increment_merchant_volume(&env, &merchant, &token, 50_000);
//     });

//     let fee = client.calculate_fee(&merchant, &token, &10_000_000);
//     let base_fee = 10_000_000i128 * 500 / 10_000;
//     let expected = base_fee - (base_fee * 25 / 10_000);
//     assert_eq!(fee, expected);
// }

// #[test]
// fn test_discount_tier_3_at_100k_volume() {
//     let env = Env::default();
//     let (_admin, client, contract_id, token) = setup(&env);
//     let merchant = Address::generate(&env);
//     client.register_merchant(&merchant);

//     env.as_contract(&contract_id, || {
//         admin_component::increment_merchant_volume(&env, &merchant, &token, 100_000);
//     });

//     let fee = client.calculate_fee(&merchant, &token, &10_000_000);
//     let base_fee = 10_000_000i128 * 500 / 10_000;
//     let expected = base_fee - (base_fee * 50 / 10_000);
//     assert_eq!(fee, expected);
// }

// #[test]
// fn test_volume_accumulates() {
//     let env = Env::default();
//     let (_admin, client, contract_id, token) = setup(&env);
//     let merchant = Address::generate(&env);
//     client.register_merchant(&merchant);

//     env.as_contract(&contract_id, || {
//         admin_component::increment_merchant_volume(&env, &merchant, &token, 10_000);
//         admin_component::increment_merchant_volume(&env, &merchant, &token, 20_000);
//     });

//     assert_eq!(client.get_merchant_volume(&merchant, &token), 60_000);

//     let fee = client.calculate_fee(&merchant, &token, &10_000_000);
//     let base_fee = 10_000_000i128 * 500 / 10_000;
//     let expected = base_fee - (base_fee * 25 / 10_000);
//     assert_eq!(fee, expected);
// }
