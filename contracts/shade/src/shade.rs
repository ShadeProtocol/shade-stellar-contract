use crate::components::{
    access_control as access_control_component, admin as admin_component, core as core_component,
    invoice as invoice_component, merchant as merchant_component, pausable as pausable_component,
    upgrade as upgrade_component,
};
use crate::errors::ContractError;
use crate::events;
use crate::interface::ShadeTrait;
use crate::types::{ContractInfo, DataKey, Invoice, InvoiceFilter, Merchant, MerchantFilter, Role};
use soroban_sdk::{contract, contractimpl, panic_with_error, Address, BytesN, Env, String, Vec};

#[contract]
pub struct Shade;

#[contractimpl]
impl ShadeTrait for Shade {
    fn initialize(env: Env, admin: Address) {
        if env.storage().persistent().has(&DataKey::Admin) {
            panic_with_error!(&env, ContractError::AlreadyInitialized);
        }
        core_component::set_admin(&env, &admin);
        pausable_component::assert_not_paused(&env);
        events::publish_initialized_event(&env, admin, env.ledger().timestamp());
    }

    fn get_admin(env: Env) -> Address {
        core_component::get_admin(&env)
    }

    fn add_accepted_token(env: Env, admin: Address, token: Address) {
        admin.require_auth();
        pausable_component::assert_not_paused(&env);
        admin_component::add_accepted_token(&env, &admin, &token);
    }

    fn add_accepted_tokens(env: Env, admin: Address, tokens: Vec<Address>) {
        admin.require_auth();
        pausable_component::assert_not_paused(&env);
        admin_component::add_accepted_tokens(&env, &admin, &tokens);
    }

    fn remove_accepted_token(env: Env, admin: Address, token: Address) {
        admin.require_auth();
        pausable_component::assert_not_paused(&env);
        admin_component::remove_accepted_token(&env, &admin, &token);
    }

    fn is_accepted_token(env: Env, token: Address) -> bool {
        admin_component::is_accepted_token(&env, &token)
    }

    fn set_account_wasm_hash(env: Env, admin: Address, wasm_hash: BytesN<32>) {
        admin.require_auth();
        pausable_component::assert_not_paused(&env);
        admin_component::set_account_wasm_hash(&env, &admin, &wasm_hash);
    }

    fn set_fee(env: Env, admin: Address, token: Address, fee: i128) {
        admin.require_auth();
        pausable_component::assert_not_paused(&env);
        admin_component::set_fee(&env, &admin, &token, fee);
    }

    fn get_fee(env: Env, token: Address) -> i128 {
        admin_component::get_fee(&env, &token)
    }

    fn register_merchant(env: Env, merchant: Address) {
        merchant.require_auth();
        pausable_component::assert_not_paused(&env);
        merchant_component::register_merchant(&env, &merchant);
    }

    fn get_merchant(env: Env, merchant_id: u64) -> Merchant {
        merchant_component::get_merchant(&env, merchant_id)
    }

    fn get_merchants(env: Env, filter: MerchantFilter) -> Vec<Merchant> {
        merchant_component::get_merchants(&env, filter)
    }

    fn is_merchant(env: Env, merchant: Address) -> bool {
        merchant_component::is_merchant(&env, &merchant)
    }

    fn set_merchant_status(env: Env, admin: Address, merchant_id: u64, status: bool) {
        admin.require_auth();
        pausable_component::assert_not_paused(&env);
        merchant_component::set_merchant_status(&env, &admin, merchant_id, status);
    }

    fn is_merchant_active(env: Env, merchant_id: u64) -> bool {
        merchant_component::is_merchant_active(&env, merchant_id)
    }

    fn verify_merchant(env: Env, admin: Address, merchant_id: u64, status: bool) {
        admin.require_auth();
        pausable_component::assert_not_paused(&env);
        merchant_component::verify_merchant(&env, &admin, merchant_id, status);
    }

    fn is_merchant_verified(env: Env, merchant_id: u64) -> bool {
        merchant_component::is_merchant_verified(&env, merchant_id)
    }

    fn create_invoice(
        env: Env,
        merchant: Address,
        description: String,
        amount: i128,
        token: Address,
    ) -> u64 {
        merchant.require_auth();
        pausable_component::assert_not_paused(&env);
        invoice_component::create_invoice(&env, &merchant, &description, amount, &token)
    }

    fn get_invoice(env: Env, invoice_id: u64) -> Invoice {
        invoice_component::get_invoice(&env, invoice_id)
    }

    fn get_invoices(env: Env, filter: InvoiceFilter) -> Vec<Invoice> {
        invoice_component::get_invoices(&env, filter)
    }

    fn refund_invoice(env: Env, merchant: Address, invoice_id: u64) {
        merchant.require_auth();
        pausable_component::assert_not_paused(&env);
        invoice_component::refund_invoice(&env, &merchant, invoice_id);
    }

    fn set_merchant_key(env: Env, merchant: Address, key: BytesN<32>) {
        merchant.require_auth();
        merchant_component::set_merchant_key(&env, &merchant, &key);
    }

    fn get_merchant_key(env: Env, merchant: Address) -> BytesN<32> {
        merchant_component::get_merchant_key(&env, &merchant)
    }

    fn grant_role(env: Env, admin: Address, user: Address, role: Role) {
        admin.require_auth();
        access_control_component::grant_role(&env, &admin, &user, role);
    }

    fn revoke_role(env: Env, admin: Address, user: Address, role: Role) {
        admin.require_auth();
        access_control_component::revoke_role(&env, &admin, &user, role);
    }

    fn has_role(env: Env, user: Address, role: Role) -> bool {
        access_control_component::has_role(&env, &user, role)
    }

    fn refund_invoice_partial(env: Env, invoice_id: u64, amount: i128) {
        let invoice = invoice_component::get_invoice(&env, invoice_id);
        let merchant_data = merchant_component::get_merchant(&env, invoice.merchant_id);
        merchant_data.address.require_auth();

        pausable_component::assert_not_paused(&env);
        invoice_component::refund_invoice_partial(&env, invoice_id, amount);
    }

    fn pause(env: Env, admin: Address) {
        admin.require_auth();
        pausable_component::pause(&env, &admin);
    }

    fn unpause(env: Env, admin: Address) {
        admin.require_auth();
        pausable_component::unpause(&env, &admin);
    }

    fn is_paused(env: Env) -> bool {
        pausable_component::is_paused(&env)
    }

    fn upgrade(env: Env, new_wasm_hash: BytesN<32>) {
        let admin = core_component::get_admin(&env);
        admin.require_auth();
        upgrade_component::upgrade(&env, &new_wasm_hash);
    }

    fn restrict_merchant_account(
        env: Env,
        caller: Address,
        merchant_address: Address,
        status: bool,
    ) {
        caller.require_auth();
        merchant_component::restrict_merchant_account(&env, &caller, &merchant_address, status);
    }

    fn set_merchant_account(env: Env, merchant: Address, account: Address) {
        merchant.require_auth();
        pausable_component::assert_not_paused(&env);
        merchant_component::set_merchant_account(&env, &merchant, &account);
    }

    fn get_merchant_account(env: Env, merchant_id: u64) -> Address {
        merchant_component::get_merchant_account(&env, merchant_id)
    }

    fn pay_invoice(env: Env, payer: Address, invoice_id: u64) {
        payer.require_auth();
        pausable_component::assert_not_paused(&env);

        if !access_control_component::has_role(&env, &payer, Role::Admin)
            && !access_control_component::has_role(&env, &payer, Role::Manager)
        {
            #[cfg(test)]
            {
                std::println!(
                    "Shade::pay_invoice: NotAuthorized for payer {:?}. Admin is {:?}",
                    payer,
                    core_component::get_admin(&env)
                );
                std::println!(
                    "Is payer Admin? {}",
                    access_control_component::has_role(&env, &payer, Role::Admin)
                );
                std::println!(
                    "Is payer Manager? {}",
                    access_control_component::has_role(&env, &payer, Role::Manager)
                );
            }
            panic_with_error!(&env, ContractError::NotAuthorized);
        }

        invoice_component::pay_invoice(&env, &payer, invoice_id);
    }

    fn void_invoice(env: Env, merchant: Address, invoice_id: u64) {
        merchant.require_auth();
        pausable_component::assert_not_paused(&env);
        invoice_component::void_invoice(&env, &merchant, invoice_id);
    }

    fn amend_invoice(
        env: Env,
        merchant: Address,
        invoice_id: u64,
        new_amount: Option<i128>,
        new_description: Option<String>,
    ) {
        merchant.require_auth();
        pausable_component::assert_not_paused(&env);
        invoice_component::amend_invoice(&env, &merchant, invoice_id, new_amount, new_description);
    }
}
