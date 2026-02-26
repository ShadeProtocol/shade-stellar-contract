#![cfg(test)]
extern crate std;

use soroban_sdk::{testutils::Address as _, Address, Env};

// Assuming the main contract struct is `ShadeContract` 
// and the client is generated from it.
use crate::{ShadeContract, ShadeContractClient};

// Pulling from the modules you shared in the context
use crate::types::InvoiceStatus; 
// use crate::errors::ContractError; // Uncomment if your project expects specific error enums in should_panic

#[test]
fn test_merchant_can_amend_pending_invoice() {
    let env = Env::default();
    env.mock_all_auths(); // Bypass signature requirements for unit testing

    let contract_id = env.register_contract(None, ShadeContract);
    let client = ShadeContractClient::new(&env, &contract_id);

    let merchant = Address::generate(&env);
    let buyer = Address::generate(&env);

    // 1. Merchant creates a pending invoice
    let initial_amount = 500_0000000; // e.g., 500 tokens
    
    // Note: Adjust the parameters here to match your exact `create_invoice` signature 
    // (e.g., it might require a token address, an item description, or an expiry date)
    let invoice_id = client.create_invoice(&merchant, &buyer, &initial_amount);

    // Verify initial state
    let invoice = client.get_invoice(&invoice_id);
    assert_eq!(invoice.amount, initial_amount);
    assert_eq!(invoice.status, InvoiceStatus::Pending);

    // 2. Merchant updates the pending invoice
    let new_amount = 750_0000000;
    
    // Note: Adjust method name to `amend_invoice` if that is what the interface uses
    client.update_invoice(&merchant, &invoice_id, &new_amount); 

    // 3. Verify the amendment was successful and status remained Pending
    let updated_invoice = client.get_invoice(&invoice_id);
    assert_eq!(updated_invoice.amount, new_amount);
    assert_eq!(updated_invoice.status, InvoiceStatus::Pending);
}

#[test]
// IMPORTANT: Change "Invoice already paid" to the exact string or Error Enum 
// defined in your `crate::errors` module (e.g., expected = "Error(Contract, 4)")
#[should_panic(expected = "Invoice already paid")] 
fn test_amend_paid_invoice_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, ShadeContract);
    let client = ShadeContractClient::new(&env, &contract_id);

    let merchant = Address::generate(&env);
    let buyer = Address::generate(&env);

    // 1. Create a pending invoice
    let amount = 500_0000000;
    let invoice_id = client.create_invoice(&merchant, &buyer, &amount);

    // 2. Simulate the invoice being paid
    // (You may need to mock a token transfer or use your test harness's payment setup here)
    client.pay_invoice(&buyer, &invoice_id);

    // Verify it is actually paid
    let paid_invoice = client.get_invoice(&invoice_id);
    assert_eq!(paid_invoice.status, InvoiceStatus::Paid);

    // 3. Attempt to amend the paid invoice (This MUST panic)
    let new_amount = 900_0000000;
    client.update_invoice(&merchant, &invoice_id, &new_amount);
}