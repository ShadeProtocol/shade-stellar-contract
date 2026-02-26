#![cfg(test)]

use crate::shade::{Shade, ShadeClient};
use crate::types::InvoiceFilter;
use soroban_sdk::testutils::{Address as _, Ledger as _};
use soroban_sdk::{Address, Env, String};

fn setup_test() -> (Env, ShadeClient<'static>, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(Shade, ());
    let client = ShadeClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.initialize(&admin);
    (env, client, contract_id, admin)
}

#[test]
fn test_invoice_date_filtering() {
    let (env, client, _contract_id, _admin) = setup_test();

    let merchant = Address::generate(&env);
    client.register_merchant(&merchant);

    let token = Address::generate(&env);
    let description = String::from_str(&env, "Test Invoice");
    let amount: i128 = 1000;

    // Create invoices at different timestamps
    // Invoice 1: T = 100
    env.ledger().set_timestamp(100);
    let id1 = client.create_invoice(&merchant, &description, &amount, &token, &None);
    client.finalize_invoice(&merchant, &id1);

    // Invoice 2: T = 200
    env.ledger().set_timestamp(200);
    let id2 = client.create_invoice(&merchant, &description, &amount, &token, &None);
    client.finalize_invoice(&merchant, &id2);

    // Invoice 3: T = 300
    env.ledger().set_timestamp(300);
    let id3 = client.create_invoice(&merchant, &description, &amount, &token, &None);
    client.finalize_invoice(&merchant, &id3);

    // --- Start Date Filtering ---

    // Filter from T=150 (should include ID 2 and 3)
    let filter_start_150 = InvoiceFilter {
        status: None,
        merchant: None,
        min_amount: None,
        max_amount: None,
        start_date: Some(150),
        end_date: None,
    };
    let results = client.get_invoices(&filter_start_150);
    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|i| i.id == id2));
    assert!(results.iter().any(|i| i.id == id3));

    // Filter from T=200 (Inclusive - should include ID 2 and 3)
    let filter_start_200 = InvoiceFilter {
        status: None,
        merchant: None,
        min_amount: None,
        max_amount: None,
        start_date: Some(200),
        end_date: None,
    };
    let results = client.get_invoices(&filter_start_200);
    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|i| i.id == id2));
    assert!(results.iter().any(|i| i.id == id3));

    // Filter from T=201 (Exclusive - should only include ID 3)
    let filter_start_201 = InvoiceFilter {
        status: None,
        merchant: None,
        min_amount: None,
        max_amount: None,
        start_date: Some(201),
        end_date: None,
    };
    let results = client.get_invoices(&filter_start_201);
    assert_eq!(results.len(), 1);
    assert_eq!(results.get(0).unwrap().id, id3);

    // --- End Date Filtering ---

    // Filter up to T=250 (should include ID 1 and 2)
    let filter_end_250 = InvoiceFilter {
        status: None,
        merchant: None,
        min_amount: None,
        max_amount: None,
        start_date: None,
        end_date: Some(250),
    };
    let results = client.get_invoices(&filter_end_250);
    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|i| i.id == id1));
    assert!(results.iter().any(|i| i.id == id2));

    // Filter up to T=200 (Inclusive - should include ID 1 and 2)
    let filter_end_200 = InvoiceFilter {
        status: None,
        merchant: None,
        min_amount: None,
        max_amount: None,
        start_date: None,
        end_date: Some(200),
    };
    let results = client.get_invoices(&filter_end_200);
    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|i| i.id == id1));
    assert!(results.iter().any(|i| i.id == id2));

    // Filter up to T=199 (Exclusive - should only include ID 1)
    let filter_end_199 = InvoiceFilter {
        status: None,
        merchant: None,
        min_amount: None,
        max_amount: None,
        start_date: None,
        end_date: Some(199),
    };
    let results = client.get_invoices(&filter_end_199);
    assert_eq!(results.len(), 1);
    assert_eq!(results.get(0).unwrap().id, id1);

    // --- Range Filtering ---

    // Filter between T=150 and T=250 (should only include ID 2)
    let filter_range = InvoiceFilter {
        status: None,
        merchant: None,
        min_amount: None,
        max_amount: None,
        start_date: Some(150),
        end_date: Some(250),
    };
    let results = client.get_invoices(&filter_range);
    assert_eq!(results.len(), 1);
    assert_eq!(results.get(0).unwrap().id, id2);

    // Filter between T=100 and T=300 (Inclusive - should include all)
    let filter_range_full = InvoiceFilter {
        status: None,
        merchant: None,
        min_amount: None,
        max_amount: None,
        start_date: Some(100),
        end_date: Some(300),
    };
    let results = client.get_invoices(&filter_range_full);
    assert_eq!(results.len(), 3);

    // Filter with no results
    let filter_no_results = InvoiceFilter {
        status: None,
        merchant: None,
        min_amount: None,
        max_amount: None,
        start_date: Some(400),
        end_date: Some(500),
    };
    let results = client.get_invoices(&filter_no_results);
    assert_eq!(results.len(), 0);
}
