use crate::components::{
    access_control as access_control_component,
    admin as admin_component,
    auto_withdrawal as auto_withdrawal_component,
    backer_rewards as backer_rewards_component,
    bridge as bridge_component,
    campaign as campaign_component,
    campaigns as campaigns_component,
    core as core_component,
    cross_chain_pledge as cross_chain_pledge_component,
    escrow as escrow_component,
    event as event_component,
    governance as governance_component,
    history as history_component,
    invoice as invoice_component,
    leaderboard as leaderboard_component,
    merchant as merchant_component,
    multisig_withdrawal as multisig_component,
    nft as nft_component,
    pausable as pausable_component,
    platform_fee as platform_fee_component,
    pledge as pledge_component,
    search as search_component,
    subscription as subscription_component,
    upgrade as upgrade_component,
};
use crate::errors::ContractError;
use crate::events;
use crate::shade_interface::ShadeTrait;
use crate::types::{
    BackerCampaign, BackerRewardTier, BridgeDeposit, Campaign, CampaignAnnouncement,
    CampaignCategory, CampaignFilter, CampaignPenaltyReport, CampaignTag,
    ContractInfo, CrossChainBridgePayload, CrossChainPledge, CrossChainPledgeStatus,
    DataKey, DonorInfo, Escrow, Event, EventFilter, Invoice, InvoiceFilter,
    InvoicePage, Merchant, MerchantAnalytics, MerchantAnalyticsSummary,
    MerchantFilter, MerchantPage, Nft, NftCollection, OracleConfig,
    PaymentPayload, PendingFee, Pledge, PlatformFeeSplit, Role,
    Subscription, SubscriptionFilter, SubscriptionPlan, SubscriptionPlanFilter,
    Ticket, TokenAnalytics, Transaction, UpgradeProposal, WithdrawalProposal,
    WithdrawalProposalFilter,
};
use soroban_sdk::{contract, contractimpl, panic_with_error, Address, BytesN, Env, String, Vec};

#[contract]
pub struct Shade;

#[contractimpl]
impl ShadeTrait for Shade {
    fn initialize(env: Env, admin: Address) {
        if env.storage().persistent().has(&DataKey::ContractInfo) {
            panic_with_error!(&env, ContractError::AlreadyInitialized);
        }
        let contract_info = ContractInfo {
            admin: admin.clone(),
            timestamp: env.ledger().timestamp(),
        };
        env.storage()
            .persistent()
            .set(&DataKey::PlatformAccount, &admin);
        env.storage()
            .persistent()
            .set(&DataKey::ContractInfo, &contract_info);
        events::publish_initialized_event(&env, admin, env.ledger().timestamp());
    }

    fn get_admin(env: Env) -> Address {
        core_component::get_admin(&env)
    }

    fn add_accepted_token(env: Env, admin: Address, token: Address) {
        pausable_component::assert_not_paused(&env);
        admin_component::add_accepted_token(&env, &admin, &token);
    }

    fn add_accepted_tokens(env: Env, admin: Address, tokens: Vec<Address>) {
        pausable_component::assert_not_paused(&env);
        admin_component::add_accepted_tokens(&env, &admin, &tokens);
    }

    fn remove_accepted_token(env: Env, admin: Address, token: Address) {
        pausable_component::assert_not_paused(&env);
        admin_component::remove_accepted_token(&env, &admin, &token);
    }

    fn is_accepted_token(env: Env, token: Address) -> bool {
        admin_component::is_accepted_token(&env, &token)
    }

    fn set_account_wasm_hash(env: Env, admin: Address, wasm_hash: soroban_sdk::BytesN<32>) {
        admin_component::set_account_wasm_hash(&env, &admin, &wasm_hash);
    }

    fn set_fee(env: Env, admin: Address, token: Address, fee: i128) {
        pausable_component::assert_not_paused(&env);
        admin_component::set_fee(&env, &admin, &token, fee);
    }

    fn get_fee(env: Env, token: Address) -> i128 {
        admin_component::get_fee(&env, &token)
    }

    fn set_platform_account(env: Env, admin: Address, account: Address) {
        pausable_component::assert_not_paused(&env);
        admin_component::set_platform_account(&env, &admin, &account);
    }

    fn get_platform_account(env: Env) -> Address {
        admin_component::get_platform_account(&env)
    }

    fn set_token_oracle(env: Env, admin: Address, token: Address, oracle: OracleConfig) {
        pausable_component::assert_not_paused(&env);
        admin_component::set_token_oracle(&env, &admin, &token, &oracle);
    }

    fn get_token_oracle(env: Env, token: Address) -> OracleConfig {
        admin_component::get_token_oracle(&env, &token)
    }

    fn propose_fee(env: Env, admin: Address, token: Address, fee: i128) {
        pausable_component::assert_not_paused(&env);
        admin_component::propose_fee(&env, &admin, &token, fee);
    }

    fn execute_fee(env: Env, admin: Address, token: Address) {
        pausable_component::assert_not_paused(&env);
        admin_component::execute_fee(&env, &admin, &token);
    }

    fn get_pending_fee(env: Env, token: Address) -> PendingFee {
        admin_component::get_pending_fee(&env, &token)
    }

    fn register_merchant(env: Env, merchant: Address) {
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
        merchant_component::set_merchant_status(&env, &admin, merchant_id, status);
    }

    fn is_merchant_active(env: Env, merchant_id: u64) -> bool {
        merchant_component::is_merchant_active(&env, merchant_id)
    }

    fn verify_merchant(env: Env, admin: Address, merchant_id: u64, status: bool) {
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
        expires_at: Option<u64>,
    ) -> u64 {
        pausable_component::assert_not_paused(&env);
        invoice_component::create_invoice(&env, &merchant, &description, amount, &token, expires_at)
    }

    fn create_fiat_invoice(
        env: Env,
        merchant: Address,
        description: String,
        fiat_amount: i128,
        fiat_currency: String,
        fiat_decimals: u32,
        token: Address,
        expires_at: Option<u64>,
    ) -> u64 {
        pausable_component::assert_not_paused(&env);
        invoice_component::create_fiat_invoice(
            &env,
            &merchant,
            &description,
            fiat_amount,
            &fiat_currency,
            fiat_decimals,
            &token,
            expires_at,
        )
    }

    fn create_invoice_draft(
        env: Env,
        merchant: Address,
        description: String,
        amount: i128,
        token: Address,
        expires_at: Option<u64>,
    ) -> u64 {
        pausable_component::assert_not_paused(&env);
        invoice_component::create_invoice_draft(&env, &merchant, &description, amount, &token, expires_at)
    }

    fn finalize_invoice(env: Env, merchant: Address, invoice_id: u64) {
        pausable_component::assert_not_paused(&env);
        invoice_component::finalize_invoice(&env, &merchant, invoice_id);
    }

    #[allow(clippy::too_many_arguments)]
    fn create_invoice_signed(
        env: Env,
        caller: Address,
        merchant: Address,
        description: String,
        amount: i128,
        token: Address,
        nonce: BytesN<32>,
        signature: BytesN<64>,
    ) -> u64 {
        pausable_component::assert_not_paused(&env);
        invoice_component::create_invoice_signed(
            &env, &caller, &merchant, &description, amount, &token, &nonce, &signature,
        )
    }

    fn get_invoice(env: Env, invoice_id: u64) -> Invoice {
        invoice_component::get_invoice(&env, invoice_id)
    }

    fn resolve_invoice_amount(env: Env, invoice_id: u64) -> i128 {
        invoice_component::resolve_invoice_amount(&env, invoice_id)
    }

    fn refund_invoice(env: Env, merchant: Address, invoice_id: u64) {
        pausable_component::assert_not_paused(&env);
        invoice_component::refund_invoice(&env, &merchant, invoice_id);
    }

    fn claim_refund(env: Env, buyer: Address, invoice_id: u64) {
        pausable_component::assert_not_paused(&env);
        invoice_component::claim_refund(&env, &buyer, invoice_id);
    }

    fn set_merchant_key(env: Env, merchant: Address, key: BytesN<32>) {
        merchant_component::set_merchant_key(&env, &merchant, &key);
    }

    fn get_merchant_key(env: Env, merchant: Address) -> BytesN<32> {
        merchant_component::get_merchant_key(&env, &merchant)
    }

    fn grant_role(env: Env, admin: Address, user: Address, role: Role) {
        access_control_component::grant_role(&env, &admin, &user, role);
    }

    fn revoke_role(env: Env, admin: Address, user: Address, role: Role) {
        access_control_component::revoke_role(&env, &admin, &user, role);
    }

    fn has_role(env: Env, user: Address, role: Role) -> bool {
        access_control_component::has_role(&env, &user, role)
    }

    fn get_invoices(env: Env, filter: InvoiceFilter) -> Vec<Invoice> {
        invoice_component::get_invoices(&env, filter)
    }

    fn refund_invoice_partial(env: Env, merchant: Address, invoice_id: u64, amount: i128) {
        pausable_component::assert_not_paused(&env);
        invoice_component::refund_invoice_partial(&env, &merchant, invoice_id, amount);
    }

    fn pause(env: Env, admin: Address) {
        pausable_component::pause(&env, &admin);
    }

    fn unpause(env: Env, admin: Address) {
        pausable_component::unpause(&env, &admin);
    }

    fn is_paused(env: Env) -> bool {
        pausable_component::is_paused(&env)
    }

    fn upgrade(env: Env, new_wasm_hash: BytesN<32>) {
        upgrade_component::upgrade(&env, &new_wasm_hash);
    }

    fn restrict_merchant_account(
        env: Env,
        caller: Address,
        merchant_address: Address,
        status: bool,
    ) {
        merchant_component::restrict_merchant_account(&env, &caller, &merchant_address, status);
    }

    fn calculate_fee(env: Env, merchant: Address, token: Address, amount: i128) -> i128 {
        admin_component::calculate_fee(&env, &merchant, &token, amount)
    }

    fn compute_platform_fee_split(
        env: Env,
        merchant: Address,
        token: Address,
        amount: i128,
    ) -> PlatformFeeSplit {
        platform_fee_component::compute_split(&env, &merchant, &token, amount)
    }

    fn set_merchant_platform_fee(
        env: Env,
        caller: Address,
        merchant_id: u64,
        token: Address,
        fee_bps: i128,
    ) {
        pausable_component::assert_not_paused(&env);
        platform_fee_component::set_merchant_platform_fee(&env, &caller, merchant_id, &token, fee_bps);
    }

    fn get_merchant_platform_fee(env: Env, merchant_id: u64, token: Address) -> Option<i128> {
        platform_fee_component::get_merchant_platform_fee(&env, merchant_id, &token)
    }

    fn clear_merchant_platform_fee(env: Env, caller: Address, merchant_id: u64, token: Address) {
        pausable_component::assert_not_paused(&env);
        platform_fee_component::clear_merchant_platform_fee(&env, &caller, merchant_id, &token);
    }

    fn get_merchant_volume(env: Env, merchant: Address, token: Address) -> i128 {
        admin_component::get_merchant_volume(&env, &merchant, &token)
    }

    fn get_merchant_analytics(env: Env, merchant: Address, token: Address) -> MerchantAnalytics {
        admin_component::get_merchant_analytics(&env, &merchant, &token)
    }

    fn get_merchant_analytics_summary(env: Env, merchant: Address) -> MerchantAnalyticsSummary {
        admin_component::get_merchant_analytics_summary(&env, &merchant)
    }

    fn set_merchant_account(env: Env, merchant: Address, account: Address) {
        merchant_component::set_merchant_account(&env, &merchant, &account);
    }

    fn get_merchant_account(env: Env, merchant_id: u64) -> Address {
        merchant_component::get_merchant_account(&env, merchant_id)
    }

    fn set_auto_withdrawal_threshold(env: Env, merchant: Address, token: Address, threshold: i128) {
        pausable_component::assert_not_paused(&env);
        auto_withdrawal_component::set_auto_withdrawal_threshold(&env, &merchant, &token, threshold);
    }

    fn get_auto_withdrawal_threshold(env: Env, merchant_id: u64, token: Address) -> Option<i128> {
        auto_withdrawal_component::get_auto_withdrawal_threshold(&env, merchant_id, &token)
    }

    fn set_auto_withdrawal_recipient(env: Env, merchant: Address, recipient: Address) {
        pausable_component::assert_not_paused(&env);
        auto_withdrawal_component::set_auto_withdrawal_recipient(&env, &merchant, &recipient);
    }

    fn get_auto_withdrawal_recipient(env: Env, merchant_id: u64) -> Option<Address> {
        auto_withdrawal_component::get_auto_withdrawal_recipient(&env, merchant_id)
    }

    fn pay_invoice(env: Env, payer: Address, invoice_id: u64) {
        pausable_component::assert_not_paused(&env);
        invoice_component::pay_invoice(&env, &payer, invoice_id);
    }

    fn pay_invoices_batch(env: Env, payer: Address, invoice_ids: Vec<u64>) {
        pausable_component::assert_not_paused(&env);
        invoice_component::pay_invoices_batch(&env, &payer, &invoice_ids);
    }

    fn pay_invoice_partial(env: Env, payer: Address, invoice_id: u64, amount: i128) {
        pausable_component::assert_not_paused(&env);
        invoice_component::pay_invoice_partial(&env, &payer, invoice_id, amount);
    }

    fn validate_payment_payload(env: Env, payload: PaymentPayload) {
        crate::components::payment::validate_payment_payload(&env, &payload);
    }

    fn void_invoice(env: Env, merchant: Address, invoice_id: u64) {
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
        pausable_component::assert_not_paused(&env);
        invoice_component::amend_invoice(&env, &merchant, invoice_id, new_amount, new_description);
    }

    fn propose_admin_transfer(env: Env, admin: Address, new_admin: Address) {
        admin_component::propose_admin_transfer(&env, &admin, &new_admin);
    }

    fn accept_admin_transfer(env: Env, new_admin: Address) {
        admin_component::accept_admin_transfer(&env, &new_admin);
    }

    // ── Subscription engine ───────────────────────────────────────────────────

    fn create_subscription_plan(
        env: Env,
        merchant: Address,
        description: String,
        token: Address,
        amount: i128,
        interval: u64,
    ) -> u64 {
        pausable_component::assert_not_paused(&env);
        subscription_component::create_subscription_plan(&env, merchant, description, token, amount, interval)
    }

    fn get_subscription_plan(env: Env, plan_id: u64) -> SubscriptionPlan {
        subscription_component::get_subscription_plan(&env, plan_id)
    }

    fn subscribe(env: Env, customer: Address, plan_id: u64) -> u64 {
        pausable_component::assert_not_paused(&env);
        subscription_component::subscribe(&env, customer, plan_id)
    }

    fn get_subscription(env: Env, subscription_id: u64) -> Subscription {
        subscription_component::get_subscription(&env, subscription_id)
    }

    fn charge_subscription(env: Env, subscription_id: u64) {
        pausable_component::assert_not_paused(&env);
        subscription_component::charge_subscription(&env, subscription_id);
    }

    fn cancel_subscription(env: Env, caller: Address, subscription_id: u64) {
        pausable_component::assert_not_paused(&env);
        subscription_component::cancel_subscription(&env, caller, subscription_id);
    }

    fn deactivate_plan(env: Env, caller: Address, plan_id: u64) {
        pausable_component::assert_not_paused(&env);
        subscription_component::deactivate_plan(&env, caller, plan_id);
    }

    fn set_merchant_webhook(env: Env, merchant: Address, webhook: String) {
        pausable_component::assert_not_paused(&env);
        merchant_component::set_merchant_webhook(&env, &merchant, &webhook);
    }

    fn get_merchant_webhook(env: Env, merchant_id: u64) -> String {
        merchant_component::get_merchant_webhook(&env, merchant_id)
    }

    fn set_merchant_accepted_tokens(env: Env, merchant: Address, tokens: Vec<Address>) {
        pausable_component::assert_not_paused(&env);
        merchant_component::set_merchant_accepted_tokens(&env, &merchant, &tokens);
    }

    fn get_merchant_accepted_tokens(env: Env, merchant: Address) -> Vec<Address> {
        merchant_component::get_merchant_accepted_tokens(&env, &merchant)
    }

    fn remove_merchant_accepted_token(env: Env, merchant: Address, token: Address) {
        pausable_component::assert_not_paused(&env);
        merchant_component::remove_merchant_accepted_token(&env, &merchant, &token);
    }

    fn is_token_accepted_for_merchant(env: Env, merchant: Address, token: Address) -> bool {
        merchant_component::is_token_accepted_for_merchant(&env, &merchant, &token)
    }

    fn get_user_transactions(env: Env, user: Address) -> Vec<Transaction> {
        history_component::get_user_transactions(&env, user)
    }

    fn emit_bridge_placeholder(env: Env, caller: Address, payload: CrossChainBridgePayload) {
        pausable_component::assert_not_paused(&env);
        caller.require_auth();
        events::publish_bridge_placeholder_event(&env, caller, payload, env.ledger().timestamp());
    }

    // ── Bridge listener / external deposits ──────────────────────────────────

    fn register_bridge_listener(env: Env, admin: Address, listener: Address) {
        pausable_component::assert_not_paused(&env);
        bridge_component::register_bridge_listener(&env, &admin, &listener);
    }

    fn remove_bridge_listener(env: Env, admin: Address, listener: Address) {
        pausable_component::assert_not_paused(&env);
        bridge_component::remove_bridge_listener(&env, &admin, &listener);
    }

    fn is_bridge_listener(env: Env, listener: Address) -> bool {
        bridge_component::is_bridge_listener(&env, &listener)
    }

    fn record_bridge_deposit(
        env: Env,
        listener: Address,
        source_chain: String,
        source_tx_id: BytesN<32>,
        token: Address,
        amount: i128,
        recipient: Address,
    ) -> u64 {
        pausable_component::assert_not_paused(&env);
        bridge_component::record_bridge_deposit(
            &env, &listener, source_chain, source_tx_id, token, amount, recipient,
        )
    }

    fn get_bridge_deposit(env: Env, deposit_id: u64) -> Option<BridgeDeposit> {
        bridge_component::get_bridge_deposit(&env, deposit_id)
    }

    fn is_bridge_deposit_processed(env: Env, source_tx_id: BytesN<32>) -> bool {
        bridge_component::is_bridge_deposit_processed(&env, &source_tx_id)
    }

    fn get_bridge_deposit_count(env: Env) -> u64 {
        bridge_component::get_bridge_deposit_count(&env)
    }

    fn get_bridge_credit(env: Env, recipient: Address, token: Address) -> i128 {
        bridge_component::get_bridge_credit(&env, &recipient, &token)
    }

    // ── DAO governance ────────────────────────────────────────────────────────

    fn add_gov_member(env: Env, admin: Address, member: Address) {
        pausable_component::assert_not_paused(&env);
        governance_component::add_gov_member(&env, &admin, &member);
    }

    fn remove_gov_member(env: Env, admin: Address, member: Address) {
        pausable_component::assert_not_paused(&env);
        governance_component::remove_gov_member(&env, &admin, &member);
    }

    fn is_gov_member(env: Env, member: Address) -> bool {
        governance_component::is_gov_member(&env, &member)
    }

    fn get_gov_member_count(env: Env) -> u32 {
        governance_component::get_gov_member_count(&env)
    }

    fn set_governance_config(env: Env, admin: Address, voting_period: u64, quorum_bps: u32) {
        pausable_component::assert_not_paused(&env);
        governance_component::set_governance_config(&env, &admin, voting_period, quorum_bps);
    }

    fn propose_upgrade(env: Env, proposer: Address, wasm_hash: BytesN<32>) -> u64 {
        pausable_component::assert_not_paused(&env);
        governance_component::propose_upgrade(&env, &proposer, wasm_hash)
    }

    fn vote_on_upgrade(env: Env, voter: Address, proposal_id: u64, approve: bool) {
        pausable_component::assert_not_paused(&env);
        governance_component::vote_on_upgrade(&env, &voter, proposal_id, approve);
    }

    fn finalize_upgrade(env: Env, caller: Address, proposal_id: u64) {
        pausable_component::assert_not_paused(&env);
        governance_component::finalize_upgrade(&env, &caller, proposal_id);
    }

    fn get_upgrade_proposal(env: Env, proposal_id: u64) -> Option<UpgradeProposal> {
        governance_component::get_upgrade_proposal(&env, proposal_id)
    }

    fn has_voted_on_upgrade(env: Env, proposal_id: u64, member: Address) -> bool {
        governance_component::has_voted(&env, proposal_id, &member)
    }

    // ── Event ticketing ───────────────────────────────────────────────────────

    #[allow(clippy::too_many_arguments)]
    fn create_event(
        env: Env,
        merchant: Address,
        name: String,
        ticket_price: i128,
        token: Address,
        capacity: u32,
        event_date: u64,
        royalty_bps: u32,
    ) -> u64 {
        pausable_component::assert_not_paused(&env);
        event_component::create_event(
            &env, &merchant, &name, &ticket_price, &token, &capacity, &event_date, &royalty_bps,
        )
    }

    fn purchase_ticket(env: Env, event_id: u64, buyer: Address) -> u64 {
        pausable_component::assert_not_paused(&env);
        event_component::purchase_ticket(&env, &event_id, &buyer)
    }

    fn configure_dynamic_pricing(
        env: Env,
        merchant: Address,
        event_id: u64,
        early_bird_end: u64,
        early_bird_discount_bps: u32,
        late_markup_bps: u32,
    ) {
        pausable_component::assert_not_paused(&env);
        event_component::configure_dynamic_pricing(
            &env, &merchant, event_id, early_bird_end, early_bird_discount_bps, late_markup_bps,
        );
    }

    fn get_current_ticket_price(env: Env, event_id: u64) -> i128 {
        event_component::get_current_ticket_price(&env, event_id)
    }

    fn cancel_event_and_batch_refund(env: Env, merchant: Address, event_id: u64) {
        pausable_component::assert_not_paused(&env);
        event_component::cancel_event_and_batch_refund(&env, &merchant, event_id);
    }

    fn resell_ticket(env: Env, seller: Address, buyer: Address, ticket_id: u64, resale_price: i128) {
        pausable_component::assert_not_paused(&env);
        event_component::resell_ticket(&env, &seller, &buyer, ticket_id, resale_price);
    }

    fn get_event(env: Env, event_id: u64) -> Event {
        event_component::get_event(&env, &event_id)
    }

    fn get_ticket(env: Env, ticket_id: u64) -> Ticket {
        event_component::get_ticket(&env, ticket_id)
    }

    fn get_event_tickets(env: Env, event_id: u64) -> Vec<u64> {
        event_component::get_event_tickets(&env, event_id)
    }

    fn get_user_tickets(env: Env, user: Address) -> Vec<u64> {
        event_component::get_user_tickets(&env, &user)
    }

    fn purchase_tickets_bulk(
        env: Env,
        event_id: u64,
        buyer: Address,
        quantity: u32,
        shade_token: Address,
        merchant_account: Address,
    ) {
        pausable_component::assert_not_paused(&env);
        event_component::purchase_tickets_bulk(
            &env, &event_id, &buyer, quantity, &shade_token, &merchant_account,
        );
    }

    // ── Token analytics ───────────────────────────────────────────────────────

    fn get_token_analytics(env: Env, token: Address) -> TokenAnalytics {
        admin_component::get_token_analytics(&env, &token)
    }

    fn get_token_volume(env: Env, token: Address) -> i128 {
        admin_component::get_token_volume(&env, &token)
    }

    fn get_token_dominance_metrics(env: Env, tokens: Vec<Address>) -> Vec<(Address, i128)> {
        admin_component::get_token_dominance_metrics(&env, &tokens)
    }

    fn get_top_tokens_by_volume(env: Env, limit: u32) -> Vec<(Address, i128)> {
        admin_component::get_top_tokens_by_volume(&env, limit)
    }

    fn get_token_market_share(env: Env, token: Address) -> i128 {
        admin_component::get_token_market_share(&env, token)
    }

    // ── Escrow ────────────────────────────────────────────────────────────────

    fn create_escrow(
        env: Env,
        seller: Address,
        buyer: Address,
        token: Address,
        amount: i128,
        invoice_id: Option<u64>,
    ) -> u64 {
        pausable_component::assert_not_paused(&env);
        escrow_component::create_escrow(&env, &seller, &buyer, &token, amount, invoice_id)
    }

    fn get_escrow(env: Env, escrow_id: u64) -> Escrow {
        escrow_component::get_escrow(&env, escrow_id)
    }

    fn fund_escrow(env: Env, buyer: Address, escrow_id: u64) {
        pausable_component::assert_not_paused(&env);
        escrow_component::fund_escrow(&env, &buyer, escrow_id)
    }

    fn release_escrow(env: Env, buyer: Address, escrow_id: u64) {
        pausable_component::assert_not_paused(&env);
        escrow_component::release_escrow(&env, &buyer, escrow_id)
    }

    fn refund_escrow(env: Env, seller: Address, escrow_id: u64) {
        pausable_component::assert_not_paused(&env);
        escrow_component::refund_escrow(&env, &seller, escrow_id)
    }

    // ── NFT minting & distribution ────────────────────────────────────────────

    fn create_nft_collection(
        env: Env,
        merchant: Address,
        name: String,
        base_uri: String,
        max_supply: u64,
        royalty_bps: u32,
    ) -> u64 {
        pausable_component::assert_not_paused(&env);
        nft_component::create_nft_collection(&env, &merchant, &name, &base_uri, max_supply, royalty_bps)
    }

    fn mint_nft(
        env: Env,
        merchant: Address,
        collection_id: u64,
        recipient: Address,
        token_uri: String,
    ) -> u64 {
        pausable_component::assert_not_paused(&env);
        nft_component::mint_nft(&env, &merchant, collection_id, &recipient, &token_uri)
    }

    fn batch_mint_nfts(
        env: Env,
        merchant: Address,
        collection_id: u64,
        recipients: Vec<Address>,
        token_uris: Vec<String>,
    ) -> Vec<u64> {
        pausable_component::assert_not_paused(&env);
        nft_component::batch_mint_nfts(&env, &merchant, collection_id, &recipients, &token_uris)
    }

    fn transfer_nft(env: Env, from: Address, to: Address, nft_id: u64) {
        pausable_component::assert_not_paused(&env);
        nft_component::transfer_nft(&env, &from, &to, nft_id)
    }

    fn burn_nft(env: Env, owner: Address, nft_id: u64) {
        pausable_component::assert_not_paused(&env);
        nft_component::burn_nft(&env, &owner, nft_id)
    }

    fn claim_nft_reward(env: Env, claimer: Address, nft_id: u64) {
        pausable_component::assert_not_paused(&env);
        nft_component::claim_nft_reward(&env, &claimer, nft_id)
    }

    fn deactivate_nft_collection(env: Env, merchant: Address, collection_id: u64) {
        pausable_component::assert_not_paused(&env);
        nft_component::deactivate_nft_collection(&env, &merchant, collection_id)
    }

    fn get_nft_collection(env: Env, collection_id: u64) -> NftCollection {
        nft_component::get_nft_collection(&env, collection_id)
    }

    fn get_nft(env: Env, nft_id: u64) -> Nft {
        nft_component::get_nft(&env, nft_id)
    }

    fn get_collection_nfts(env: Env, collection_id: u64) -> Vec<u64> {
        nft_component::get_collection_nfts(&env, collection_id)
    }

    fn get_user_nfts(env: Env, user: Address) -> Vec<u64> {
        nft_component::get_user_nfts(&env, &user)
    }

    // ── Backer rewards ────────────────────────────────────────────────────────

    fn create_backer_campaign(
        env: Env,
        merchant: Address,
        name: String,
        token: Address,
        deadline: u64,
    ) -> u64 {
        pausable_component::assert_not_paused(&env);
        backer_rewards_component::create_backer_campaign(&env, merchant, name, token, deadline)
    }

    fn get_backer_campaign(env: Env, campaign_id: u64) -> BackerCampaign {
        backer_rewards_component::get_backer_campaign(&env, campaign_id)
    }

    fn set_backer_reward_tiers(
        env: Env,
        merchant: Address,
        campaign_id: u64,
        tiers: Vec<BackerRewardTier>,
    ) {
        pausable_component::assert_not_paused(&env);
        backer_rewards_component::set_backer_reward_tiers(&env, merchant, campaign_id, tiers);
    }

    fn get_backer_reward_tiers(env: Env, campaign_id: u64) -> Vec<BackerRewardTier> {
        backer_rewards_component::get_backer_reward_tiers(&env, campaign_id)
    }

    fn pledge_to_campaign(env: Env, backer: Address, campaign_id: u64, amount: i128) {
        pausable_component::assert_not_paused(&env);
        backer_rewards_component::pledge_to_campaign(&env, backer, campaign_id, amount);
    }

    fn get_backer_pledge(env: Env, campaign_id: u64, backer: Address) -> i128 {
        backer_rewards_component::get_backer_pledge(&env, campaign_id, backer)
    }

    fn select_backer_reward_tier(env: Env, backer: Address, campaign_id: u64, tier_index: u32) {
        pausable_component::assert_not_paused(&env);
        backer_rewards_component::select_backer_reward_tier(&env, backer, campaign_id, tier_index);
    }

    fn get_backer_selected_tier(env: Env, campaign_id: u64, backer: Address) -> Option<u32> {
        backer_rewards_component::get_backer_selected_tier(&env, campaign_id, backer)
    }

    fn fulfill_backer_reward(env: Env, merchant: Address, campaign_id: u64, backer: Address) {
        pausable_component::assert_not_paused(&env);
        backer_rewards_component::fulfill_backer_reward(&env, merchant, campaign_id, backer);
    }

    fn is_backer_reward_fulfilled(env: Env, campaign_id: u64, backer: Address) -> bool {
        backer_rewards_component::is_backer_reward_fulfilled(&env, campaign_id, backer)
    }

    fn claim_backer_perk(env: Env, backer: Address, campaign_id: u64, perk_index: u32) {
        pausable_component::assert_not_paused(&env);
        backer_rewards_component::claim_backer_perk(&env, backer, campaign_id, perk_index);
    }

    fn is_backer_perk_claimed(env: Env, campaign_id: u64, backer: Address, perk_index: u32) -> bool {
        backer_rewards_component::is_backer_perk_claimed(&env, campaign_id, backer, perk_index)
    }

    // ── Multi-sig massive withdrawal ─────────────────────────────────────────

    fn set_multisig_threshold(env: Env, admin: Address, token: Address, threshold: i128) {
        pausable_component::assert_not_paused(&env);
        multisig_component::set_multisig_threshold(&env, &admin, &token, threshold);
    }

    fn get_multisig_threshold(env: Env, token: Address) -> i128 {
        multisig_component::get_multisig_threshold(&env, &token)
            .unwrap_or_else(|| panic_with_error!(&env, ContractError::ThresholdNotSet))
    }

    fn configure_multisig(env: Env, admin: Address, signers: Vec<Address>, quorum: u32) {
        pausable_component::assert_not_paused(&env);
        multisig_component::configure_multisig(&env, &admin, signers, quorum);
    }

    fn propose_withdrawal(
        env: Env,
        merchant: Address,
        token: Address,
        amount: i128,
        recipient: Address,
        note: String,
    ) -> u64 {
        pausable_component::assert_not_paused(&env);
        multisig_component::propose_withdrawal(&env, &merchant, &token, amount, &recipient, note)
    }

    fn approve_withdrawal(env: Env, signer: Address, proposal_id: u64) {
        pausable_component::assert_not_paused(&env);
        multisig_component::approve_withdrawal(&env, &signer, proposal_id);
    }

    fn cancel_withdrawal(env: Env, caller: Address, proposal_id: u64) {
        pausable_component::assert_not_paused(&env);
        multisig_component::cancel_withdrawal(&env, &caller, proposal_id);
    }

    fn get_withdrawal_proposal(env: Env, proposal_id: u64) -> WithdrawalProposal {
        multisig_component::get_withdrawal_proposal(&env, proposal_id)
    }

    fn has_approved_withdrawal(env: Env, signer: Address, proposal_id: u64) -> bool {
        multisig_component::has_approved(&env, &signer, proposal_id)
    }

    fn get_withdrawal_proposal_count(env: Env) -> u64 {
        multisig_component::get_proposal_count(&env)
    }

    // ── On-chain search and filtering ─────────────────────────────────────────

    fn search_invoices_paginated(
        env: Env,
        caller: Address,
        filter: InvoiceFilter,
        cursor: u64,
        page_size: u32,
    ) -> InvoicePage {
        search_component::search_invoices_paginated(&env, &caller, filter, cursor, page_size)
    }

    fn search_merchants_paginated(
        env: Env,
        filter: MerchantFilter,
        cursor: u64,
        page_size: u32,
    ) -> MerchantPage {
        search_component::search_merchants_paginated(&env, filter, cursor, page_size)
    }

    fn search_subscription_plans(
        env: Env,
        caller: Address,
        filter: SubscriptionPlanFilter,
    ) -> Vec<SubscriptionPlan> {
        search_component::search_subscription_plans(&env, &caller, filter)
    }

    fn search_subscriptions(env: Env, filter: SubscriptionFilter) -> Vec<Subscription> {
        search_component::search_subscriptions(&env, filter)
    }

    fn search_events(env: Env, caller: Address, filter: EventFilter) -> Vec<Event> {
        search_component::search_events(&env, &caller, filter)
    }

    fn search_withdrawal_proposals(
        env: Env,
        caller: Address,
        filter: WithdrawalProposalFilter,
    ) -> Vec<WithdrawalProposal> {
        search_component::search_withdrawal_proposals(&env, &caller, filter)
    }

    fn find_merchant_id(env: Env, address: Address) -> u64 {
        search_component::find_merchant_id(&env, &address).unwrap_or(0)
    }

    // ── Campaign categories & tagging (#352) ──────────────────────────────────

    fn create_campaign_category(env: Env, admin: Address, name: String, description: String) -> u64 {
        pausable_component::assert_not_paused(&env);
        campaigns_component::create_category(&env, &admin, &name, &description)
    }

    fn update_campaign_category(
        env: Env,
        admin: Address,
        category_id: u64,
        name: Option<String>,
        description: Option<String>,
        active: Option<bool>,
    ) {
        pausable_component::assert_not_paused(&env);
        campaigns_component::update_category(&env, &admin, category_id, name, description, active);
    }

    fn get_campaign_category(env: Env, category_id: u64) -> CampaignCategory {
        campaigns_component::get_category(&env, category_id)
    }

    fn get_campaign_categories(env: Env) -> Vec<CampaignCategory> {
        campaigns_component::get_categories(&env)
    }

    fn create_campaign_tag(env: Env, creator: Address, name: String) -> u64 {
        pausable_component::assert_not_paused(&env);
        campaigns_component::create_tag(&env, &creator, &name)
    }

    fn get_campaign_tag(env: Env, tag_id: u64) -> CampaignTag {
        campaigns_component::get_tag(&env, tag_id)
    }

    fn get_campaign_tags(env: Env) -> Vec<CampaignTag> {
        campaigns_component::get_tags(&env)
    }

    #[allow(clippy::too_many_arguments)]
    fn create_campaign(
        env: Env,
        merchant: Address,
        title: String,
        description: String,
        category_id: u64,
        tags: Vec<u64>,
        goal_amount: i128,
        token: Address,
        deadline: u64,
    ) -> u64 {
        pausable_component::assert_not_paused(&env);
        campaigns_component::create_campaign(
            &env, &merchant, &title, &description, category_id, &tags, goal_amount, &token, deadline,
        )
    }

    fn update_campaign(
        env: Env,
        merchant: Address,
        campaign_id: u64,
        title: Option<String>,
        description: Option<String>,
    ) {
        pausable_component::assert_not_paused(&env);
        campaigns_component::update_campaign(&env, &merchant, campaign_id, title, description);
    }

    fn set_campaign_active(env: Env, merchant: Address, campaign_id: u64, active: bool) {
        pausable_component::assert_not_paused(&env);
        campaigns_component::set_campaign_active(&env, &merchant, campaign_id, active);
    }

    fn add_campaign_tag(env: Env, merchant: Address, campaign_id: u64, tag_id: u64) {
        pausable_component::assert_not_paused(&env);
        campaigns_component::add_campaign_tag(&env, &merchant, campaign_id, tag_id);
    }

    fn remove_campaign_tag(env: Env, merchant: Address, campaign_id: u64, tag_id: u64) {
        pausable_component::assert_not_paused(&env);
        campaigns_component::remove_campaign_tag(&env, &merchant, campaign_id, tag_id);
    }

    fn record_campaign_contribution(
        env: Env,
        campaign_id: u64,
        contributor: Address,
        amount: i128,
    ) {
        pausable_component::assert_not_paused(&env);
        contributor.require_auth();
        campaigns_component::record_contribution(&env, campaign_id, &contributor, amount);
    }

    fn get_campaign(env: Env, campaign_id: u64) -> Campaign {
        campaigns_component::get_campaign(&env, campaign_id)
    }

    fn get_campaigns(env: Env, filter: CampaignFilter) -> Vec<Campaign> {
        campaigns_component::get_campaigns(&env, filter)
    }

    // ── Leaderboard ───────────────────────────────────────────────────────────

    fn init_campaign(env: Env, merchant: Address, campaign_id: u64) {
        leaderboard_component::init_campaign(&env, merchant, campaign_id);
    }

    fn track_donation(
        env: Env,
        merchant: Address,
        campaign_id: u64,
        donor: Address,
        amount: i128,
    ) {
        leaderboard_component::track_donation(&env, merchant, campaign_id, donor, amount);
    }

    fn get_top_donors(env: Env, campaign_id: u64) -> Vec<DonorInfo> {
        leaderboard_component::get_top_donors(&env, campaign_id)
    }

    // ── Financial penalties for malicious campaigns (#360) ────────────────────

    fn create_campaign(
        env: Env,
        caller: Address,
        name: String,
        charity: bool,
        fee_waiver_bps: u32,
        discount_bps: u32,
        stake_required: i128,
    ) -> u64 {
        pausable_component::assert_not_paused(&env);
        campaign_component::create_campaign(
            &env, &caller, &name, charity, fee_waiver_bps, discount_bps, stake_required,
        )
    }

    fn stake_campaign(env: Env, caller: Address, campaign_id: u64, amount: i128) {
        pausable_component::assert_not_paused(&env);
        campaign_component::stake_campaign(&env, &caller, campaign_id, amount);
    }

    fn slash_campaign_stake(
        env: Env,
        caller: Address,
        campaign_id: u64,
        participant: Address,
        amount: i128,
    ) {
        pausable_component::assert_not_paused(&env);
        campaign_component::slash_campaign_stake(&env, &caller, campaign_id, &participant, amount);
    }

    fn register_affiliate(
        env: Env,
        caller: Address,
        campaign_id: u64,
        affiliate: Address,
        commission_bps: u32,
    ) {
        pausable_component::assert_not_paused(&env);
        campaign_component::register_affiliate(&env, &caller, campaign_id, &affiliate, commission_bps);
    }

    fn pay_affiliate_commission(
        env: Env,
        caller: Address,
        campaign_id: u64,
        affiliate: Address,
        amount: i128,
    ) {
        pausable_component::assert_not_paused(&env);
        campaign_component::pay_affiliate_commission(&env, &caller, campaign_id, &affiliate, amount);
    }

    fn get_campaign_participant(env: Env, campaign_id: u64, participant: Address) -> CampaignParticipant {
        campaign_component::get_campaign_participant(&env, campaign_id, &participant)
    }

    fn get_campaign_affiliate(env: Env, campaign_id: u64, affiliate: Address) -> CampaignAffiliate {
        campaign_component::get_campaign_affiliate(&env, campaign_id, &affiliate)
    }

    fn get_campaign_leaderboard(env: Env, campaign_id: u64, limit: u32) -> Vec<(Address, i128)> {
        campaign_component::get_campaign_leaderboard(&env, campaign_id, limit)
    }

    // ── Cross-chain pledge ────────────────────────────────────────────────────

    fn create_cross_chain_pledge(
        env: Env,
        source_chain: String,
        source_pledge_id: u64,
        destination_chain: String,
        merchant: Address,
        payer: Address,
        token: Address,
        amount: i128,
        memo: Option<String>,
    ) -> u64 {
        pausable_component::assert_not_paused(&env);
        cross_chain_pledge_component::create_pledge(
            &env, source_chain, source_pledge_id, destination_chain, merchant, payer, token, amount, memo,
        )
    }

    fn update_cross_chain_pledge_status(
        env: Env,
        pledge_id: u64,
        new_status: CrossChainPledgeStatus,
    ) {
        pausable_component::assert_not_paused(&env);
        cross_chain_pledge_component::update_pledge_status(&env, pledge_id, new_status);
    }

    fn get_cross_chain_pledge(env: Env, pledge_id: u64) -> CrossChainPledge {
        cross_chain_pledge_component::get_pledge(&env, pledge_id)
    }

    fn get_cross_chain_pledge_by_source(
        env: Env,
        source_chain: String,
        source_pledge_id: u64,
    ) -> CrossChainPledge {
        cross_chain_pledge_component::get_pledge_by_source(&env, source_chain, source_pledge_id)
    }

    fn get_all_cross_chain_pledges(env: Env) -> Vec<CrossChainPledge> {
        cross_chain_pledge_component::get_all_pledges(&env)
    }
}
