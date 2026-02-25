#![cfg(test)]

use crate::shade::{Shade, ShadeClient};
use crate::types::InvoiceStatus;
use soroban_sdk::testutils::{Address as _, Ledger as _};
use soroban_sdk::{token, Address, Env, String};

fn setup_test_with_payment() -> (Env, ShadeClient<'static>, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let shade_contract_id = env.register(Shade, ());
    let shade_client = ShadeClient::new(&env, &shade_contract_id);

    let admin = Address::generate(&env);
    shade_client.initialize(&admin);

    let token_admin = Address::generate(&env);
    let token = env.register_stellar_asset_contract_v2(token_admin.clone());

    shade_client.add_accepted_token(&admin, &token.address());
    shade_client.set_fee(&admin, &token.address(), &0);

    (env, shade_client, shade_contract_id, admin, token.address())
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #21)")]
fn test_pay_expired_invoice_fails() {
    let (env, client, _shade_contract_id, _admin, token) = setup_test_with_payment();

    let merchant = Address::generate(&env);
    client.register_merchant(&merchant);

    let merchant_account = Address::generate(&env);
    client.set_merchant_account(&merchant, &merchant_account);

    let description = String::from_str(&env, "Expiring Invoice");
    let expires_at: u64 = 1000;
    let invoice_id =
        client.create_invoice(&merchant, &description, &500, &token, &Some(expires_at));

    // Advance ledger past the expiry
    env.ledger().set_timestamp(1001);

    let customer = Address::generate(&env);
    let token_client = token::StellarAssetClient::new(&env, &token);
    token_client.mint(&customer, &500);

    // Should panic with InvoiceExpired (#21)
    client.pay_invoice(&customer, &invoice_id);
}

#[test]
fn test_pay_invoice_before_expiry_succeeds() {
    let (env, client, _shade_contract_id, _admin, token) = setup_test_with_payment();

    let merchant = Address::generate(&env);
    client.register_merchant(&merchant);

    let merchant_account = Address::generate(&env);
    client.set_merchant_account(&merchant, &merchant_account);

    let description = String::from_str(&env, "Expiring Invoice");
    let expires_at: u64 = 2000;
    let invoice_id =
        client.create_invoice(&merchant, &description, &500, &token, &Some(expires_at));

    // Set timestamp before expiry
    env.ledger().set_timestamp(1999);

    let customer = Address::generate(&env);
    let token_client = token::StellarAssetClient::new(&env, &token);
    token_client.mint(&customer, &500);

    client.pay_invoice(&customer, &invoice_id);

    let invoice = client.get_invoice(&invoice_id);
    assert_eq!(invoice.status, InvoiceStatus::Paid);
    assert_eq!(invoice.payer, Some(customer));
}

#[test]
fn test_invoice_no_expiry_always_payable() {
    let (env, client, _shade_contract_id, _admin, token) = setup_test_with_payment();

    let merchant = Address::generate(&env);
    client.register_merchant(&merchant);

    let merchant_account = Address::generate(&env);
    client.set_merchant_account(&merchant, &merchant_account);

    let description = String::from_str(&env, "No Expiry Invoice");
    let invoice_id = client.create_invoice(&merchant, &description, &500, &token, &None);

    // Set timestamp to a very large value
    env.ledger().set_timestamp(999_999_999);

    let customer = Address::generate(&env);
    let token_client = token::StellarAssetClient::new(&env, &token);
    token_client.mint(&customer, &500);

    client.pay_invoice(&customer, &invoice_id);

    let invoice = client.get_invoice(&invoice_id);
    assert_eq!(invoice.status, InvoiceStatus::Paid);
    assert_eq!(invoice.payer, Some(customer));
}
