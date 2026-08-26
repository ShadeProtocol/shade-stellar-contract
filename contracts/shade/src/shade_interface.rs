//! Public contract interface.
//!
//! This trait is the single source of truth for the contract's exported
//! functions; `shade.rs` implements it verbatim. Keep the two in sync — adding
//! a method to the impl without declaring it here is a compile error.

use crate::types::{
    AnalyticsExport, BackerCampaign, BackerRewardTier, BridgeDeposit, Campaign, CampaignAffiliate,
    CampaignCategory, CampaignFiatGoal, CampaignFilter, CampaignParticipant, CampaignStats,
    CampaignTag, CreatorVesting, CrossChainBridgePayload, DonorInfo, Escrow, Event, EventFilter,
    ExportFormat, FeeCampaign, FiatGoalQuote, Invoice, InvoiceFilter, InvoicePage, Merchant,
    MerchantAnalytics, MerchantAnalyticsSummary, MerchantFilter, MerchantPage, Nft, NftCollection,
    OracleConfig, PendingFee, PlatformFeeSplit, Pledge, PledgeCampaign, Role, StretchGoal,
    StretchGoalReward, Subscription, SubscriptionFilter, SubscriptionPlan, SubscriptionPlanFilter,
    Ticket, TokenAnalytics, Transaction, UpgradeProposal, WithdrawalProposal,
    WithdrawalProposalFilter,
};
use soroban_sdk::{contracttrait, Address, BytesN, Env, Option, String, Vec};

#[contracttrait]
pub trait ShadeTrait {
    fn initialize(env: Env, admin: Address);
    fn get_admin(env: Env) -> Address;
    fn add_accepted_token(env: Env, admin: Address, token: Address);
    fn add_accepted_tokens(env: Env, admin: Address, tokens: Vec<Address>);
    fn remove_accepted_token(env: Env, admin: Address, token: Address);
    fn is_accepted_token(env: Env, token: Address) -> bool;
    fn set_account_wasm_hash(env: Env, admin: Address, wasm_hash: soroban_sdk::BytesN<32>);
    fn set_fee(env: Env, admin: Address, token: Address, fee: i128);
    fn get_fee(env: Env, token: Address) -> i128;
    fn set_platform_account(env: Env, admin: Address, account: Address);
    fn get_platform_account(env: Env) -> Address;
    fn set_token_oracle(env: Env, admin: Address, token: Address, oracle: OracleConfig);
    fn get_token_oracle(env: Env, token: Address) -> OracleConfig;
    fn propose_fee(env: Env, admin: Address, token: Address, fee: i128);
    fn execute_fee(env: Env, admin: Address, token: Address);
    fn get_pending_fee(env: Env, token: Address) -> PendingFee;
    fn register_merchant(env: Env, merchant: Address);
    fn get_merchant(env: Env, merchant_id: u64) -> Merchant;
    fn get_merchants(env: Env, filter: MerchantFilter) -> Vec<Merchant>;
    fn is_merchant(env: Env, merchant: Address) -> bool;
    fn set_merchant_status(env: Env, admin: Address, merchant_id: u64, status: bool);
    fn is_merchant_active(env: Env, merchant_id: u64) -> bool;
    fn verify_merchant(env: Env, admin: Address, merchant_id: u64, status: bool);
    fn is_merchant_verified(env: Env, merchant_id: u64) -> bool;
    fn create_invoice(
        env: Env,
        merchant: Address,
        description: String,
        amount: i128,
        token: Address,
        expires_at: Option<u64>,
    ) -> u64;
    fn create_fiat_invoice(
        env: Env,
        merchant: Address,
        description: String,
        fiat_amount: i128,
        fiat_currency: String,
        fiat_decimals: u32,
        token: Address,
        expires_at: Option<u64>,
    ) -> u64;
    fn create_invoice_draft(
        env: Env,
        merchant: Address,
        description: String,
        amount: i128,
        token: Address,
        expires_at: Option<u64>,
    ) -> u64;
    fn finalize_invoice(env: Env, merchant: Address, invoice_id: u64);
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
    ) -> u64;
    fn get_invoice(env: Env, invoice_id: u64) -> Invoice;
    fn resolve_invoice_amount(env: Env, invoice_id: u64) -> i128;
    fn refund_invoice(env: Env, merchant: Address, invoice_id: u64);
    fn claim_refund(env: Env, buyer: Address, invoice_id: u64);
    fn set_merchant_key(env: Env, merchant: Address, key: BytesN<32>);
    fn get_merchant_key(env: Env, merchant: Address) -> BytesN<32>;
    fn grant_role(env: Env, admin: Address, user: Address, role: Role);
    fn revoke_role(env: Env, admin: Address, user: Address, role: Role);
    fn has_role(env: Env, user: Address, role: Role) -> bool;
    fn get_invoices(env: Env, filter: InvoiceFilter) -> Vec<Invoice>;
    fn refund_invoice_partial(env: Env, merchant: Address, invoice_id: u64, amount: i128);
    fn pause(env: Env, admin: Address);
    fn unpause(env: Env, admin: Address);
    fn is_paused(env: Env) -> bool;
    fn upgrade(env: Env, new_wasm_hash: BytesN<32>);
    fn restrict_merchant_account(
        env: Env,
        caller: Address,
        merchant_address: Address,
        status: bool,
    );
    fn calculate_fee(env: Env, merchant: Address, token: Address, amount: i128) -> i128;
    fn compute_platform_fee_split(
        env: Env,
        merchant: Address,
        token: Address,
        amount: i128,
    ) -> PlatformFeeSplit;
    fn set_merchant_platform_fee(
        env: Env,
        caller: Address,
        merchant_id: u64,
        token: Address,
        fee_bps: i128,
    );
    fn get_merchant_platform_fee(env: Env, merchant_id: u64, token: Address) -> Option<i128>;
    fn clear_merchant_platform_fee(env: Env, caller: Address, merchant_id: u64, token: Address);
    fn get_merchant_volume(env: Env, merchant: Address, token: Address) -> i128;
    fn get_merchant_analytics(env: Env, merchant: Address, token: Address) -> MerchantAnalytics;
    fn get_merchant_analytics_summary(env: Env, merchant: Address) -> MerchantAnalyticsSummary;
    fn set_merchant_account(env: Env, merchant: Address, account: Address);
    fn get_merchant_account(env: Env, merchant_id: u64) -> Address;
    fn set_auto_withdrawal_threshold(env: Env, merchant: Address, token: Address, threshold: i128);
    fn get_auto_withdrawal_threshold(env: Env, merchant_id: u64, token: Address) -> Option<i128>;
    fn set_auto_withdrawal_recipient(env: Env, merchant: Address, recipient: Address);
    fn get_auto_withdrawal_recipient(env: Env, merchant_id: u64) -> Option<Address>;
    fn pay_invoice(env: Env, payer: Address, invoice_id: u64);
    fn pay_invoices_batch(env: Env, payer: Address, invoice_ids: Vec<u64>);
    fn pay_invoice_partial(env: Env, payer: Address, invoice_id: u64, amount: i128);
    fn validate_payment_payload(env: Env, payload: crate::types::PaymentPayload);
    fn void_invoice(env: Env, merchant: Address, invoice_id: u64);
    fn amend_invoice(
        env: Env,
        merchant: Address,
        invoice_id: u64,
        new_amount: Option<i128>,
        new_description: Option<String>,
    );
    fn propose_admin_transfer(env: Env, admin: Address, new_admin: Address);
    fn accept_admin_transfer(env: Env, new_admin: Address);
    fn create_subscription_plan(
        env: Env,
        merchant: Address,
        description: String,
        token: Address,
        amount: i128,
        interval: u64,
    ) -> u64;
    fn get_subscription_plan(env: Env, plan_id: u64) -> SubscriptionPlan;
    fn subscribe(env: Env, customer: Address, plan_id: u64) -> u64;
    fn get_subscription(env: Env, subscription_id: u64) -> Subscription;
    fn charge_subscription(env: Env, subscription_id: u64);
    fn cancel_subscription(env: Env, caller: Address, subscription_id: u64);
    fn deactivate_plan(env: Env, caller: Address, plan_id: u64);
    fn set_merchant_webhook(env: Env, merchant: Address, webhook: String);
    fn get_merchant_webhook(env: Env, merchant_id: u64) -> String;
    fn set_merchant_accepted_tokens(env: Env, merchant: Address, tokens: Vec<Address>);
    fn get_merchant_accepted_tokens(env: Env, merchant: Address) -> Vec<Address>;
    fn remove_merchant_accepted_token(env: Env, merchant: Address, token: Address);
    fn is_token_accepted_for_merchant(env: Env, merchant: Address, token: Address) -> bool;
    fn get_user_transactions(env: Env, user: Address) -> Vec<Transaction>;
    fn emit_bridge_placeholder(env: Env, caller: Address, payload: CrossChainBridgePayload);
    fn register_bridge_listener(env: Env, admin: Address, listener: Address);
    fn remove_bridge_listener(env: Env, admin: Address, listener: Address);
    fn is_bridge_listener(env: Env, listener: Address) -> bool;
    fn record_bridge_deposit(
        env: Env,
        listener: Address,
        source_chain: String,
        source_tx_id: BytesN<32>,
        token: Address,
        amount: i128,
        recipient: Address,
    ) -> u64;
    fn get_bridge_deposit(env: Env, deposit_id: u64) -> Option<BridgeDeposit>;
    fn is_bridge_deposit_processed(env: Env, source_tx_id: BytesN<32>) -> bool;
    fn get_bridge_deposit_count(env: Env) -> u64;
    fn get_bridge_credit(env: Env, recipient: Address, token: Address) -> i128;
    fn add_gov_member(env: Env, admin: Address, member: Address);
    fn remove_gov_member(env: Env, admin: Address, member: Address);
    fn is_gov_member(env: Env, member: Address) -> bool;
    fn get_gov_member_count(env: Env) -> u32;
    fn set_governance_config(env: Env, admin: Address, voting_period: u64, quorum_bps: u32);
    fn propose_upgrade(env: Env, proposer: Address, wasm_hash: BytesN<32>) -> u64;
    fn vote_on_upgrade(env: Env, voter: Address, proposal_id: u64, approve: bool);
    fn finalize_upgrade(env: Env, caller: Address, proposal_id: u64);
    fn get_upgrade_proposal(env: Env, proposal_id: u64) -> Option<UpgradeProposal>;
    fn has_voted_on_upgrade(env: Env, proposal_id: u64, member: Address) -> bool;
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
    ) -> u64;
    fn purchase_ticket(env: Env, event_id: u64, buyer: Address) -> u64;
    fn configure_dynamic_pricing(
        env: Env,
        merchant: Address,
        event_id: u64,
        early_bird_end: u64,
        early_bird_discount_bps: u32,
        late_markup_bps: u32,
    );
    fn get_current_ticket_price(env: Env, event_id: u64) -> i128;
    fn cancel_event_and_batch_refund(env: Env, merchant: Address, event_id: u64);
    fn resell_ticket(env: Env, seller: Address, buyer: Address, ticket_id: u64, resale_price: i128);
    fn get_event(env: Env, event_id: u64) -> Event;
    fn get_ticket(env: Env, ticket_id: u64) -> Ticket;
    fn get_event_tickets(env: Env, event_id: u64) -> Vec<u64>;
    fn get_user_tickets(env: Env, user: Address) -> Vec<u64>;
    fn purchase_tickets_bulk(
        env: Env,
        event_id: u64,
        buyer: Address,
        quantity: u32,
        shade_token: Address,
        merchant_account: Address,
    );
    fn get_token_analytics(env: Env, token: Address) -> TokenAnalytics;
    fn get_token_volume(env: Env, token: Address) -> i128;
    fn get_token_dominance_metrics(env: Env, tokens: Vec<Address>) -> Vec<(Address, i128)>;
    fn get_top_tokens_by_volume(env: Env, limit: u32) -> Vec<(Address, i128)>;
    fn get_token_market_share(env: Env, token: Address) -> i128;
    fn create_escrow(
        env: Env,
        seller: Address,
        buyer: Address,
        token: Address,
        amount: i128,
        invoice_id: Option<u64>,
    ) -> u64;
    fn get_escrow(env: Env, escrow_id: u64) -> Escrow;
    fn fund_escrow(env: Env, buyer: Address, escrow_id: u64);
    fn release_escrow(env: Env, buyer: Address, escrow_id: u64);
    fn refund_escrow(env: Env, seller: Address, escrow_id: u64);
    fn create_nft_collection(
        env: Env,
        merchant: Address,
        name: String,
        base_uri: String,
        max_supply: u64,
        royalty_bps: u32,
    ) -> u64;
    fn mint_nft(
        env: Env,
        merchant: Address,
        collection_id: u64,
        recipient: Address,
        token_uri: String,
    ) -> u64;
    fn batch_mint_nfts(
        env: Env,
        merchant: Address,
        collection_id: u64,
        recipients: Vec<Address>,
        token_uris: Vec<String>,
    ) -> Vec<u64>;
    fn transfer_nft(env: Env, from: Address, to: Address, nft_id: u64);
    fn burn_nft(env: Env, owner: Address, nft_id: u64);
    fn claim_nft_reward(env: Env, claimer: Address, nft_id: u64);
    fn deactivate_nft_collection(env: Env, merchant: Address, collection_id: u64);
    fn get_nft_collection(env: Env, collection_id: u64) -> NftCollection;
    fn get_nft(env: Env, nft_id: u64) -> Nft;
    fn get_collection_nfts(env: Env, collection_id: u64) -> Vec<u64>;
    fn get_user_nfts(env: Env, user: Address) -> Vec<u64>;
    fn create_backer_campaign(
        env: Env,
        merchant: Address,
        name: String,
        token: Address,
        deadline: u64,
    ) -> u64;
    fn get_backer_campaign(env: Env, campaign_id: u64) -> BackerCampaign;
    fn set_backer_reward_tiers(
        env: Env,
        merchant: Address,
        campaign_id: u64,
        tiers: Vec<BackerRewardTier>,
    );
    fn get_backer_reward_tiers(env: Env, campaign_id: u64) -> Vec<BackerRewardTier>;
    fn pledge_to_campaign(env: Env, backer: Address, campaign_id: u64, amount: i128);
    fn get_backer_pledge(env: Env, campaign_id: u64, backer: Address) -> i128;
    fn select_backer_reward_tier(env: Env, backer: Address, campaign_id: u64, tier_index: u32);
    fn get_backer_selected_tier(env: Env, campaign_id: u64, backer: Address) -> Option<u32>;
    fn fulfill_backer_reward(env: Env, merchant: Address, campaign_id: u64, backer: Address);
    fn is_backer_reward_fulfilled(env: Env, campaign_id: u64, backer: Address) -> bool;
    fn claim_backer_perk(env: Env, backer: Address, campaign_id: u64, perk_index: u32);
    fn is_backer_perk_claimed(env: Env, campaign_id: u64, backer: Address, perk_index: u32)
        -> bool;
    fn set_multisig_threshold(env: Env, admin: Address, token: Address, threshold: i128);
    fn get_multisig_threshold(env: Env, token: Address) -> i128;
    fn configure_multisig(env: Env, admin: Address, signers: Vec<Address>, quorum: u32);
    fn propose_withdrawal(
        env: Env,
        merchant: Address,
        token: Address,
        amount: i128,
        recipient: Address,
        note: String,
    ) -> u64;
    fn approve_withdrawal(env: Env, signer: Address, proposal_id: u64);
    fn cancel_withdrawal(env: Env, caller: Address, proposal_id: u64);
    fn get_withdrawal_proposal(env: Env, proposal_id: u64) -> WithdrawalProposal;
    fn has_approved_withdrawal(env: Env, signer: Address, proposal_id: u64) -> bool;
    fn get_withdrawal_proposal_count(env: Env) -> u64;
    fn search_invoices_paginated(
        env: Env,
        caller: Address,
        filter: InvoiceFilter,
        cursor: u64,
        page_size: u32,
    ) -> InvoicePage;
    fn search_merchants_paginated(
        env: Env,
        filter: MerchantFilter,
        cursor: u64,
        page_size: u32,
    ) -> MerchantPage;
    fn search_subscription_plans(
        env: Env,
        caller: Address,
        filter: SubscriptionPlanFilter,
    ) -> Vec<SubscriptionPlan>;
    fn search_subscriptions(env: Env, filter: SubscriptionFilter) -> Vec<Subscription>;
    fn search_events(env: Env, caller: Address, filter: EventFilter) -> Vec<Event>;
    fn search_withdrawal_proposals(
        env: Env,
        caller: Address,
        filter: WithdrawalProposalFilter,
    ) -> Vec<WithdrawalProposal>;
    fn find_merchant_id(env: Env, address: Address) -> u64;
    fn create_pledge_campaign(
        env: Env,
        merchant: Address,
        title: String,
        goal: i128,
        token: Address,
        deadline: u64,
    ) -> u64;
    fn get_pledge_campaign(env: Env, campaign_id: u64) -> PledgeCampaign;
    fn pledge(
        env: Env,
        contributor: Address,
        campaign_id: u64,
        amount: i128,
        token: Address,
    ) -> u64;
    fn execute_campaign(env: Env, merchant: Address, campaign_id: u64);
    fn cancel_pledge_campaign(env: Env, merchant: Address, campaign_id: u64);
    fn claim_pledge_refund(env: Env, contributor: Address, campaign_id: u64);
    fn batch_refund(env: Env, campaign_id: u64);
    fn get_pledge(env: Env, pledge_id: u64) -> Pledge;
    fn get_campaign_pledges(env: Env, campaign_id: u64) -> Vec<Pledge>;
    fn get_contributor_pledges(env: Env, contributor: Address) -> Vec<Pledge>;
    fn get_merchant_campaigns(env: Env, merchant: Address) -> Vec<Campaign>;
    fn create_campaign_category(env: Env, admin: Address, name: String, description: String)
        -> u64;
    fn update_campaign_category(
        env: Env,
        admin: Address,
        category_id: u64,
        name: Option<String>,
        description: Option<String>,
        active: Option<bool>,
    );
    fn get_campaign_category(env: Env, category_id: u64) -> CampaignCategory;
    fn get_campaign_categories(env: Env) -> Vec<CampaignCategory>;
    fn create_campaign_tag(env: Env, creator: Address, name: String) -> u64;
    fn get_campaign_tag(env: Env, tag_id: u64) -> CampaignTag;
    fn get_campaign_tags(env: Env) -> Vec<CampaignTag>;
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
    ) -> u64;
    fn update_campaign(
        env: Env,
        merchant: Address,
        campaign_id: u64,
        title: Option<String>,
        description: Option<String>,
    );
    fn set_campaign_active(env: Env, merchant: Address, campaign_id: u64, active: bool);
    fn add_campaign_tag(env: Env, merchant: Address, campaign_id: u64, tag_id: u64);
    fn remove_campaign_tag(env: Env, merchant: Address, campaign_id: u64, tag_id: u64);
    fn record_campaign_contribution(env: Env, campaign_id: u64, contributor: Address, amount: i128);
    fn get_campaign(env: Env, campaign_id: u64) -> Campaign;
    fn get_campaigns(env: Env, filter: CampaignFilter) -> Vec<Campaign>;
    fn init_campaign(env: Env, merchant: Address, campaign_id: u64);
    fn track_donation(env: Env, merchant: Address, campaign_id: u64, donor: Address, amount: i128);
    fn get_top_donors(env: Env, campaign_id: u64) -> Vec<DonorInfo>;
    /// Defines a funding milestone beyond a campaign's base goal.
    /// Only the campaign's owning merchant may call this.
    fn create_stretch_goal(
        env: Env,
        merchant: Address,
        campaign_id: u64,
        target_amount: i128,
        description: String,
        reward_description: String,
    ) -> u64;
    /// Unlocks a goal whose campaign has reached its target. The raised amount
    /// is read from the campaign, not supplied by the caller.
    fn unlock_stretch_goal(env: Env, merchant: Address, goal_id: u64);
    /// Retires a goal that has not been unlocked yet.
    fn cancel_stretch_goal(env: Env, merchant: Address, goal_id: u64);
    /// Grants a reward to one backer for an unlocked goal.
    fn grant_stretch_goal_reward(
        env: Env,
        merchant: Address,
        goal_id: u64,
        backer: Address,
        reward_amount: i128,
    );
    /// Marks the caller's reward for a goal as claimed.
    fn claim_stretch_goal_reward(env: Env, backer: Address, goal_id: u64);
    fn get_stretch_goal(env: Env, goal_id: u64) -> StretchGoal;
    fn get_campaign_stretch_goals(env: Env, campaign_id: u64) -> Vec<u64>;
    fn get_campaign_stretch_goal_data(env: Env, campaign_id: u64) -> Vec<StretchGoal>;
    /// The campaign's next un-unlocked milestone, if any.
    fn get_next_stretch_goal(env: Env, campaign_id: u64) -> Option<StretchGoal>;
    fn get_stretch_goal_reward(
        env: Env,
        goal_id: u64,
        backer: Address,
    ) -> Option<StretchGoalReward>;
    fn create_fee_campaign(
        env: Env,
        caller: Address,
        name: String,
        charity: bool,
        fee_waiver_bps: u32,
        discount_bps: u32,
        stake_required: i128,
    ) -> u64;
    fn get_fee_campaign(env: Env, campaign_id: u64) -> FeeCampaign;
    fn configure_campaign_fee_policy(
        env: Env,
        caller: Address,
        campaign_id: u64,
        fee_waiver_bps: u32,
        discount_bps: u32,
    );
    fn calculate_campaign_discount(env: Env, campaign_id: u64, amount: i128) -> i128;
    fn record_fee_campaign_contribution(env: Env, caller: Address, campaign_id: u64, amount: i128);
    fn stake_campaign(env: Env, caller: Address, campaign_id: u64, amount: i128);
    fn slash_campaign_stake(
        env: Env,
        caller: Address,
        campaign_id: u64,
        participant: Address,
        amount: i128,
    );
    fn register_affiliate(
        env: Env,
        caller: Address,
        campaign_id: u64,
        affiliate: Address,
        commission_bps: u32,
    );
    fn pay_affiliate_commission(
        env: Env,
        caller: Address,
        campaign_id: u64,
        affiliate: Address,
        amount: i128,
    );
    fn get_campaign_participant(
        env: Env,
        campaign_id: u64,
        participant: Address,
    ) -> CampaignParticipant;
    fn get_campaign_affiliate(env: Env, campaign_id: u64, affiliate: Address) -> CampaignAffiliate;
    fn get_campaign_leaderboard(env: Env, campaign_id: u64, limit: u32) -> Vec<(Address, i128)>;

    // ── Creator fund vesting ──────────────────────────────────────────────────

    /// Commits a backer campaign's raised funds to a cliff-plus-linear vesting
    /// schedule paying out to its creator. Owning merchant only; the terms are
    /// fixed once published.
    fn create_creator_vesting(
        env: Env,
        creator: Address,
        campaign_id: u64,
        total_amount: i128,
        start_time: u64,
        cliff_duration: u64,
        vesting_duration: u64,
        initial_unlock_bps: u32,
    );
    /// Pays the creator everything vested since their last release, returning
    /// the amount transferred.
    fn release_creator_vesting(env: Env, creator: Address, campaign_id: u64) -> i128;
    /// Freezes a schedule so nothing further vests. Admin only; the
    /// already-vested balance stays claimable by the creator.
    fn revoke_creator_vesting(env: Env, admin: Address, campaign_id: u64);
    fn get_creator_vesting(env: Env, campaign_id: u64) -> CreatorVesting;
    /// Amount vested as of now, released or not.
    fn get_vested_amount(env: Env, campaign_id: u64) -> i128;
    /// Amount the creator could release right now.
    fn get_releasable_amount(env: Env, campaign_id: u64) -> i128;
    /// Campaign IDs this creator vests funds from, in creation order.
    fn get_creator_vesting_campaigns(env: Env, creator: Address) -> Vec<u64>;

    // ── Fiat-pegged campaign goals ────────────────────────────────────────────

    /// Pegs a campaign's funding target to a fiat currency, valued through the
    /// campaign token's price oracle. Owning merchant only, once per campaign;
    /// the published target is immutable.
    fn set_campaign_fiat_goal(
        env: Env,
        merchant: Address,
        campaign_id: u64,
        currency: String,
        goal_amount: i128,
        decimals: u32,
    );
    /// Values a token contribution against a campaign's fiat peg and credits
    /// the fiat figure to the goal, returning it.
    fn record_fiat_contribution(
        env: Env,
        contributor: Address,
        campaign_id: u64,
        token_amount: i128,
    ) -> i128;
    /// Re-reads the oracle and publishes a fresh on-ledger valuation. Owning
    /// merchant only; use `get_campaign_fiat_goal_quote` for a read-only view.
    fn refresh_campaign_fiat_quote(env: Env, merchant: Address, campaign_id: u64) -> FiatGoalQuote;
    /// Stops further contributions being valued against a peg. Owning merchant
    /// or admin; the raised total is preserved.
    fn close_campaign_fiat_goal(env: Env, caller: Address, campaign_id: u64);
    fn get_campaign_fiat_goal(env: Env, campaign_id: u64) -> CampaignFiatGoal;
    /// Whether this campaign's goal is fiat-pegged.
    fn has_campaign_fiat_goal(env: Env, campaign_id: u64) -> bool;
    /// Live valuation of a peg at the current oracle price.
    fn get_campaign_fiat_goal_quote(env: Env, campaign_id: u64) -> FiatGoalQuote;
    /// Fiat a token contribution would be credited at right now.
    fn quote_fiat_contribution(env: Env, campaign_id: u64, token_amount: i128) -> i128;
    /// Cumulative fiat one backer has contributed to a campaign.
    fn get_backer_fiat_contribution(env: Env, campaign_id: u64, backer: Address) -> i128;
    /// Pegs across a merchant's campaigns, in campaign-creation order.
    fn get_merchant_fiat_goals(env: Env, merchant_id: u64) -> Vec<CampaignFiatGoal>;

    // ── Campaign analytics exports ────────────────────────────────────────────

    /// Snapshots a campaign's analytics into an immutable export record and
    /// emits it for off-chain rendering, returning the export's ID. Owning
    /// merchant only; each export reports the delta since the campaign's
    /// previous one alongside the cumulative figures.
    fn export_campaign_analytics(
        env: Env,
        creator: Address,
        campaign_id: u64,
        format: ExportFormat,
    ) -> u64;
    /// A campaign's running contribution aggregate, as exports snapshot it.
    fn get_campaign_stats(env: Env, campaign_id: u64) -> CampaignStats;
    fn get_analytics_export(env: Env, export_id: u64) -> AnalyticsExport;
    /// Export IDs for a campaign, in the order they were run.
    fn get_campaign_exports(env: Env, campaign_id: u64) -> Vec<u64>;
    /// The most recent export for a campaign.
    fn get_latest_campaign_export(env: Env, campaign_id: u64) -> AnalyticsExport;
}
