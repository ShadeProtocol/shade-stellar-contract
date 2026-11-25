#![cfg(test)]

use crate::components::history::MAX_TRANSACTION_PAGE_SIZE;
use crate::shade::{Shade, ShadeClient};
use crate::tests::support::LinkMerchantAccount as _;
use crate::types::TransactionType;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{token, Address, Env, String, Vec};

fn setup_test() -> (Env, ShadeClient<'static>, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let shade_contract_id = env.register(Shade, ());
    let shade_client = ShadeClient::new(&env, &shade_contract_id);

    let admin = Address::generate(&env);
    shade_client.initialize(&admin);

    let token_admin = Address::generate(&env);
    let token = env.register_stellar_asset_contract_v2(token_admin.clone());

    shade_client.add_accepted_token(&admin, &token.address());
    shade_client.set_fee(&admin, &token.address(), &500); // 5%

    (env, shade_client, shade_contract_id, admin, token.address())
}

#[test]
fn test_invoice_payment_records_history() {
    let (env, shade_client, _shade_contract_id, _admin, token) = setup_test();

    let merchant = Address::generate(&env);
    shade_client.register_merchant(&merchant);
    let merchant_account = Address::generate(&env);
    shade_client.link_merchant_account(&merchant, &merchant_account);

    let description = String::from_str(&env, "History Test");
    let invoice_id = shade_client.create_invoice(&merchant, &description, &1000, &token, &None);

    let customer = Address::generate(&env);
    let token_client = token::StellarAssetClient::new(&env, &token);
    token_client.mint(&customer, &1000);

    shade_client.pay_invoice(&customer, &invoice_id);

    let history = shade_client.get_user_transactions(&customer);
    assert_eq!(history.len(), 1);

    let tx = history.get(0).unwrap();
    assert_eq!(tx.transaction_type, TransactionType::InvoicePayment);
    assert_eq!(tx.ref_id, invoice_id);
    assert_eq!(tx.amount, 1000);
    assert_eq!(tx.token, token);
    assert_eq!(tx.description, description);
    assert_eq!(tx.merchant_id, 1);
}

#[test]
fn test_multiple_payments_record_history() {
    let (env, shade_client, _shade_contract_id, _admin, token) = setup_test();

    let merchant = Address::generate(&env);
    shade_client.register_merchant(&merchant);
    let merchant_account = Address::generate(&env);
    shade_client.link_merchant_account(&merchant, &merchant_account);

    let customer = Address::generate(&env);
    let token_client = token::StellarAssetClient::new(&env, &token);
    token_client.mint(&customer, &5000);

    for _i in 1..=3 {
        let description = String::from_str(&env, "Test Inv");
        let invoice_id = shade_client.create_invoice(&merchant, &description, &1000, &token, &None);
        shade_client.pay_invoice(&customer, &invoice_id);
    }

    let history = shade_client.get_user_transactions(&customer);
    assert_eq!(history.len(), 3);
}

#[test]
fn test_history_is_paginated_and_bounded() {
    let (env, shade_client, _shade_contract_id, _admin, token) = setup_test();

    let merchant = Address::generate(&env);
    shade_client.register_merchant(&merchant);
    let merchant_account = Address::generate(&env);
    shade_client.link_merchant_account(&merchant, &merchant_account);

    // More transactions than fit in two full pages.
    let max = MAX_TRANSACTION_PAGE_SIZE;
    let total = max * 2 + 3;

    let customer = Address::generate(&env);
    let token_client = token::StellarAssetClient::new(&env, &token);
    token_client.mint(&customer, &((total as i128) * 1000));

    let mut invoice_ids: Vec<u64> = Vec::new(&env);
    for _ in 0..total {
        let description = String::from_str(&env, "Paged");
        let invoice_id = shade_client.create_invoice(&merchant, &description, &1000, &token, &None);
        shade_client.pay_invoice(&customer, &invoice_id);
        invoice_ids.push_back(invoice_id);
    }

    assert_eq!(
        shade_client.get_user_transaction_count(&customer),
        total as u64
    );

    // Read everything back in full-size pages, oldest first.
    let mut cursor = 0u64;
    let mut seen = 0u32;
    let mut pages = 0u32;
    loop {
        let page = shade_client.get_user_transactions_page(&customer, &cursor, &max);
        assert!(page.items.len() <= max);
        assert_eq!(page.page_info.count, page.items.len());
        for tx in page.items.iter() {
            assert_eq!(tx.ref_id, invoice_ids.get(seen).unwrap());
            seen += 1;
        }
        pages += 1;
        if !page.page_info.has_next_page {
            assert_eq!(page.page_info.next_cursor, 0);
            break;
        }
        assert_eq!(page.page_info.next_cursor, seen as u64);
        cursor = page.page_info.next_cursor;
    }
    assert_eq!(seen, total);
    assert_eq!(pages, 3);

    // A page_size above the maximum, or 0, is treated as the maximum.
    let oversized = shade_client.get_user_transactions_page(&customer, &0, &(max * 10));
    assert_eq!(oversized.items.len(), max);
    let zero = shade_client.get_user_transactions_page(&customer, &0, &0);
    assert_eq!(zero.items.len(), max);

    // Small pages walk the same history.
    let mut cursor = 0u64;
    let mut seen = 0u32;
    loop {
        let page = shade_client.get_user_transactions_page(&customer, &cursor, &7);
        assert!(page.items.len() <= 7);
        for tx in page.items.iter() {
            assert_eq!(tx.ref_id, invoice_ids.get(seen).unwrap());
            seen += 1;
        }
        if !page.page_info.has_next_page {
            break;
        }
        cursor = page.page_info.next_cursor;
    }
    assert_eq!(seen, total);

    // A cursor at the end returns an empty page.
    let past = shade_client.get_user_transactions_page(&customer, &(total as u64), &max);
    assert_eq!(past.items.len(), 0);
    assert!(!past.page_info.has_next_page);
    assert_eq!(past.page_info.next_cursor, 0);

    // The deprecated reader is bounded to one page.
    assert_eq!(shade_client.get_user_transactions(&customer).len(), max);

    // A user with no payments has an empty history.
    let stranger = Address::generate(&env);
    assert_eq!(shade_client.get_user_transaction_count(&stranger), 0);
    let empty = shade_client.get_user_transactions_page(&stranger, &0, &max);
    assert_eq!(empty.items.len(), 0);
    assert!(!empty.page_info.has_next_page);
}
