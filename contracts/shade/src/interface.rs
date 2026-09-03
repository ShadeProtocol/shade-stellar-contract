use crate::types::{
    BackerCampaign, BackerRewardTier, BridgeDeposit, Campaign, CampaignAnnouncement,
    CampaignCategory, CampaignFilter, CampaignPenaltyReport, CampaignTag,
    CrossChainBridgePayload, CrossChainPledge, CrossChainPledgeStatus,
    DonorInfo, Escrow, Event, EventFilter, Invoice, InvoiceFilter, InvoicePage,
    Merchant, MerchantAnalytics, MerchantAnalyticsSummary, MerchantFilter,
    MerchantPage, MultiSigConfig, Nft, NftCollection, OracleConfig,
    PaymentPayload, PendingFee, Pledge, PlatformFeeSplit, Role, Subscription,
    SubscriptionFilter, SubscriptionPlan, SubscriptionPlanFilter, Ticket,
    TokenAnalytics, Transaction, UpgradeProposal, WithdrawalProposal,
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
    fn clear_merchant_platform_fee(
        env: Env,
        caller: Address,
        merchant_id: u64,
        token: Address,
    );
    fn get_merchant_volume(env: Env, merchant: Address, token: Address) -> i128;
    fn get_merchant_analytics(env: Env, merchant: Address, token: Address) -> MerchantAnalytics;
    fn get_merchant_analytics_summary(env: Env, merchant: Address) -> MerchantAnalyticsSummary;
    fn set_merchant_account(env: Env, merchant: Address, account: Address);
    fn get_merchant_account(env: Env, merchant_id: u64) -> Address;
    fn set_auto_withdrawal_threshold(env: Env, merchant: Address, token: Address, threshold: i128);
    fn get_auto_withdrawal_threshold(env: Env, merchant_id: u64, token: Address) -> Option<i128>;
    fn set_auto_withdrawal_recipient(env: Env, merchant: Address, recipient: Address);
    fn get_auto_withdrawal_recipient(env: Env, merchant_id: u64) -> Option<Address>;
    fn claim_refund(env: Env, buyer: Address, invoice_id: u64);
    fn pay_invoice(env: Env, payer: Address, invoice_id: u64);
    fn pay_invoices_batch(env: Env, payer: Address, invoice_ids: Vec<u64>);
    fn pay_invoice_partial(env: Env, payer: Address, invoice_id: u64, amount: i128);
    fn validate_payment_payload(env: Env, payload: PaymentPayload);
    fn void_invoice(env: Env, merchant: Address, invoice_id: u64);
    fn amend_invoice(
        env: Env,
        merchant: Address,
        invoice_id: u64,
        new_amount: Option<i128>,
        new_description: Option<String>,
    );

    fn set_merchant_webhook(env: Env, merchant: Address, webhook: String);
    fn get_merchant_webhook(env: Env, merchant_id: u64) -> String;

    fn set_merchant_accepted_tokens(env: Env, merchant: Address, tokens: Vec<Address>);
    fn get_merchant_accepted_tokens(env: Env, merchant: Address) -> Vec<Address>;
    fn remove_merchant_accepted_token(env: Env, merchant: Address, token: Address);
    fn is_token_accepted_for_merchant(env: Env, merchant: Address, token: Address) -> bool;

    // ── Admin transfer (two-step handover) ───────────────────────────────────

    /// Step 1: Current admin proposes a new admin address.
    fn propose_admin_transfer(env: Env, admin: Address, new_admin: Address);

    /// Step 2: Proposed new admin accepts and takes ownership.
    fn accept_admin_transfer(env: Env, new_admin: Address);

    // ── Subscription engine ───────────────────────────────────────────────────

    /// Create a recurring billing plan.
    /// Only `merchant` can call this (requires auth). Returns new plan ID.
    fn create_subscription_plan(
        env: Env,
        merchant: Address,
        description: String,
        token: Address,
        amount: i128,
        interval: u64,
    ) -> u64;

    /// Fetch a plan by ID.
    fn get_subscription_plan(env: Env, plan_id: u64) -> SubscriptionPlan;

    /// Subscribe a customer to a plan.
    /// The customer must have already called `token.approve` to grant the Shade
    /// contract sufficient allowance for recurring charges.
    /// Returns the new subscription ID.
    fn subscribe(env: Env, customer: Address, plan_id: u64) -> u64;

    /// Fetch a subscription by ID.
    fn get_subscription(env: Env, subscription_id: u64) -> Subscription;

    /// Trigger a charge for a subscription.
    /// Callable by anyone (merchant or automated bot).
    /// Panics if the billing interval has not yet elapsed or subscription is not active.
    fn charge_subscription(env: Env, subscription_id: u64);

    /// Cancel a subscription. Either the customer or the merchant may call this.
    fn cancel_subscription(env: Env, caller: Address, subscription_id: u64);

    /// Deactivate a subscription plan so that no new customers can enroll.
    /// Only the merchant who owns the plan may call this.
    fn deactivate_plan(env: Env, caller: Address, plan_id: u64);

    /// Get all transactions executed by a specific customer address.
    fn get_user_transactions(env: Env, user: Address) -> Vec<Transaction>;

    // ── Cross-chain bridge placeholder ───────────────────────────────────────
    fn emit_bridge_placeholder(env: Env, caller: Address, payload: CrossChainBridgePayload);

    // ── Bridge listener / external deposits ──────────────────────────────────

    /// Register an authorized bridge listener (relayer). Admin only.
    fn register_bridge_listener(env: Env, admin: Address, listener: Address);

    /// Revoke a bridge listener's authorization. Admin only.
    fn remove_bridge_listener(env: Env, admin: Address, listener: Address);

    /// Whether `listener` is a currently registered bridge listener.
    fn is_bridge_listener(env: Env, listener: Address) -> bool;

    /// Record a confirmed external-chain deposit. Callable only by a registered
    /// bridge listener. De-duplicated on `source_tx_id`. Returns the deposit id.
    fn record_bridge_deposit(
        env: Env,
        listener: Address,
        source_chain: String,
        source_tx_id: BytesN<32>,
        token: Address,
        amount: i128,
        recipient: Address,
    ) -> u64;

    /// Fetch a recorded external deposit by id, or `None` if it does not exist.
    fn get_bridge_deposit(env: Env, deposit_id: u64) -> Option<BridgeDeposit>;

    /// Whether an origin-chain transaction hash has already been credited.
    fn is_bridge_deposit_processed(env: Env, source_tx_id: BytesN<32>) -> bool;

    /// Total number of external deposits recorded so far.
    fn get_bridge_deposit_count(env: Env) -> u64;

    /// Cumulative amount credited to `recipient` for `token` via the bridge.
    fn get_bridge_credit(env: Env, recipient: Address, token: Address) -> i128;

    // ── DAO governance for protocol upgrades ─────────────────────────────────

    /// Register a governance council member. Admin only; idempotent.
    fn add_gov_member(env: Env, admin: Address, member: Address);

    /// Revoke a governance council member. Admin only; idempotent.
    fn remove_gov_member(env: Env, admin: Address, member: Address);

    /// Whether `member` is a current governance council member.
    fn is_gov_member(env: Env, member: Address) -> bool;

    /// Number of governance council members.
    fn get_gov_member_count(env: Env) -> u32;

    /// Configure the voting window (seconds) and approval quorum (bps). Admin only.
    fn set_governance_config(env: Env, admin: Address, voting_period: u64, quorum_bps: u32);

    /// Open an upgrade proposal for the given WASM hash. Member only. Returns id.
    fn propose_upgrade(env: Env, proposer: Address, wasm_hash: BytesN<32>) -> u64;

    /// Cast a vote on an active proposal within its window. Member only.
    fn vote_on_upgrade(env: Env, voter: Address, proposal_id: u64, approve: bool);

    /// Finalize a proposal after voting closes: apply the upgrade if it passed,
    /// otherwise mark it defeated. Member only.
    fn finalize_upgrade(env: Env, caller: Address, proposal_id: u64);

    /// Fetch an upgrade proposal by id, or `None` if it does not exist.
    fn get_upgrade_proposal(env: Env, proposal_id: u64) -> Option<UpgradeProposal>;

    /// Whether `member` has already voted on the given proposal.
    fn has_voted_on_upgrade(env: Env, proposal_id: u64, member: Address) -> bool;

    // --- Event ticketing system ---
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

    /// Purchase multiple tickets in a single call.
    /// Applies automatic group discount in Shade tokens:
    /// 5–9 tickets → 5%, 10–19 → 10%, 20+ → 15%.
    fn purchase_tickets_bulk(
        env: Env,
        event_id: u64,
        buyer: Address,
        quantity: u32,
        shade_token: Address,
        merchant_account: Address,
    );

    // ── Token analytics ────────────────────────────────────────────────────────

    /// Get comprehensive analytics for a specific token
    fn get_token_analytics(env: Env, token: Address) -> TokenAnalytics;

    /// Get total volume for a specific token
    fn get_token_volume(env: Env, token: Address) -> i128;

    /// Get token dominance metrics sorted by volume (descending)
    fn get_token_dominance_metrics(env: Env, tokens: Vec<Address>) -> Vec<(Address, i128)>;

    /// Get top tokens by volume with limit
    fn get_top_tokens_by_volume(env: Env, limit: u32) -> Vec<(Address, i128)>;

    /// Get market share of a token as basis points (10000 = 100%)
    fn get_token_market_share(env: Env, token: Address) -> i128;

    // ── Campaign categories & tagging (#352) ──────────────────────────────

    /// Create a new campaign category. Admin-only. Returns the new category ID.
    fn create_campaign_category(
        env: Env,
        admin: Address,
        name: String,
        description: String,
    ) -> u64;

    /// Update an existing campaign category. Admin-only.
    /// All fields are optional; only `Some` values are written.
    fn update_campaign_category(
        env: Env,
        admin: Address,
        category_id: u64,
        name: Option<String>,
        description: Option<String>,
        active: Option<bool>,
    );

    /// Fetch a campaign category by ID.
    fn get_campaign_category(env: Env, category_id: u64) -> CampaignCategory;

    /// List every campaign category ever created.
    fn get_campaign_categories(env: Env) -> Vec<CampaignCategory>;

    /// Create a new campaign tag. Callable by the admin or any registered
    /// merchant. Returns the new tag ID.
    fn create_campaign_tag(env: Env, creator: Address, name: String) -> u64;

    /// Fetch a campaign tag by ID.
    fn get_campaign_tag(env: Env, tag_id: u64) -> CampaignTag;

    /// List every campaign tag ever created.
    fn get_campaign_tags(env: Env) -> Vec<CampaignTag>;

    /// Create a new campaign. Merchant-only. The category must exist and be
    /// active; every tag ID must reference an existing tag. Returns the new
    /// campaign ID.
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

    /// Update the title and/or description of an existing campaign. Owner-only.
    /// Goal/token/deadline are immutable to preserve the published target.
    fn update_campaign(
        env: Env,
        merchant: Address,
        campaign_id: u64,
        title: Option<String>,
        description: Option<String>,
    );

    /// Toggle a campaign's active flag. Owner-only.
    fn set_campaign_active(env: Env, merchant: Address, campaign_id: u64, active: bool);

    /// Attach a tag to a campaign. Owner-only. De-duplicated.
    fn add_campaign_tag(env: Env, merchant: Address, campaign_id: u64, tag_id: u64);

    /// Detach a tag from a campaign. Owner-only.
    fn remove_campaign_tag(env: Env, merchant: Address, campaign_id: u64, tag_id: u64);

    /// Record a contribution against a campaign. Open to any caller
    /// (backers, payment gateways, etc.).
    fn record_campaign_contribution(
        env: Env,
        campaign_id: u64,
        contributor: Address,
        amount: i128,
    );

    /// Fetch a campaign by ID.
    fn get_campaign(env: Env, campaign_id: u64) -> Campaign;

    /// Filtered campaign listing. Any of the filter fields may be `None`.
    fn get_campaigns(env: Env, filter: CampaignFilter) -> Vec<Campaign>;
    // ── Multi-sig massive withdrawal ─────────────────────────────────────────

    /// Set the per-token withdrawal threshold above which multi-sig is required.
    /// Admin-only.
    fn set_multisig_threshold(env: Env, admin: Address, token: Address, threshold: i128);

    /// Return the configured threshold for a token, or panic if not set.
    fn get_multisig_threshold(env: Env, token: Address) -> i128;

    /// Replace the signer list and required quorum.
    /// Admin-only.  `signers` must be non-empty and `quorum` must be ≤ signers.len().
    fn configure_multisig(env: Env, admin: Address, signers: Vec<Address>, quorum: u32);

    /// Open a new withdrawal proposal for a large merchant withdrawal.
    /// The amount must be ≥ the configured threshold for the token.
    /// Returns the new proposal ID.
    fn propose_withdrawal(
        env: Env,
        merchant: Address,
        token: Address,
        amount: i128,
        recipient: Address,
        note: String,
    ) -> u64;

    /// Cast an approval vote on a pending withdrawal proposal.
    /// Caller must be a registered signer.
    /// When approvals reach quorum, funds are transferred automatically.
    fn approve_withdrawal(env: Env, signer: Address, proposal_id: u64);

    /// Cancel a pending withdrawal proposal.
    /// Only the proposing merchant or the contract admin may cancel.
    fn cancel_withdrawal(env: Env, caller: Address, proposal_id: u64);

    /// Fetch a withdrawal proposal by ID.
    fn get_withdrawal_proposal(env: Env, proposal_id: u64) -> WithdrawalProposal;

    /// Check whether a signer has already approved a specific proposal.
    fn has_approved_withdrawal(env: Env, signer: Address, proposal_id: u64) -> bool;

    /// Return the total number of withdrawal proposals ever created.
    fn get_withdrawal_proposal_count(env: Env) -> u64;

    // ── On-chain search and filtering utilities (#353) ───────────────────────

    /// Paginated invoice search with full filter support.
    /// Pass `cursor = 0` for the first page.
    fn search_invoices_paginated(
        env: Env,
        caller: Address,
        filter: InvoiceFilter,
        cursor: u64,
        page_size: u32,
    ) -> InvoicePage;

    /// Paginated merchant search with active/verified filter support.
    fn search_merchants_paginated(
        env: Env,
        filter: MerchantFilter,
        cursor: u64,
        page_size: u32,
    ) -> MerchantPage;

    /// Filter subscription plans by merchant, active status, or token.
    fn search_subscription_plans(
        env: Env,
        caller: Address,
        filter: SubscriptionPlanFilter,
    ) -> Vec<SubscriptionPlan>;

    /// Filter subscriptions by plan ID, customer address, or status.
    fn search_subscriptions(env: Env, filter: SubscriptionFilter) -> Vec<Subscription>;

    /// Filter on-chain events by merchant, cancelled status, date range,
    /// or minimum available seats.
    fn search_events(env: Env, caller: Address, filter: EventFilter) -> Vec<Event>;

    /// Filter withdrawal proposals by merchant, status, token, or creation time.
    fn search_withdrawal_proposals(
        env: Env,
        caller: Address,
        filter: WithdrawalProposalFilter,
    ) -> Vec<WithdrawalProposal>;

    /// Look up a merchant ID from their address. Returns 0 if not registered.
    fn find_merchant_id(env: Env, address: Address) -> u64;
    /// Create an escrow for physical goods
    fn create_escrow(
        env: Env,
        seller: Address,
        buyer: Address,
        token: Address,
        amount: i128,
        invoice_id: Option<u64>,
    ) -> u64;

    /// Get an escrow by ID
    fn get_escrow(env: Env, escrow_id: u64) -> Escrow;

    /// Fund an escrow
    fn fund_escrow(env: Env, buyer: Address, escrow_id: u64);

    /// Release escrow to seller
    fn release_escrow(env: Env, buyer: Address, escrow_id: u64);

    /// Refund escrow to buyer (called by seller)
    fn refund_escrow(env: Env, seller: Address, escrow_id: u64);
    // ── NFT minting & distribution ────────────────────────────────────────────

    /// Create a new NFT collection for crowdfunding rewards. Only the merchant can call this.
    fn create_nft_collection(
        env: Env,
        merchant: Address,
        name: String,
        base_uri: String,
        max_supply: u64,
        royalty_bps: u32,
    ) -> u64;

    /// Mint a single NFT from a collection to a recipient (backer reward).
    fn mint_nft(
        env: Env,
        merchant: Address,
        collection_id: u64,
        recipient: Address,
        token_uri: String,
    ) -> u64;

    /// Mint NFTs to multiple backers in one call.
    fn batch_mint_nfts(
        env: Env,
        merchant: Address,
        collection_id: u64,
        recipients: Vec<Address>,
        token_uris: Vec<String>,
    ) -> Vec<u64>;

    /// Transfer an NFT from one address to another.
    fn transfer_nft(env: Env, from: Address, to: Address, nft_id: u64);

    /// Burn (permanently destroy) an NFT. Only the owner can do this.
    fn burn_nft(env: Env, owner: Address, nft_id: u64);

    /// Claim a reward NFT assigned to the caller.
    fn claim_nft_reward(env: Env, claimer: Address, nft_id: u64);

    /// Deactivate a collection so no further minting is possible.
    fn deactivate_nft_collection(env: Env, merchant: Address, collection_id: u64);

    /// Fetch a collection by ID.
    fn get_nft_collection(env: Env, collection_id: u64) -> NftCollection;

    /// Fetch a single NFT by its global token ID.
    fn get_nft(env: Env, nft_id: u64) -> Nft;

    /// List all token IDs belonging to a collection.
    fn get_collection_nfts(env: Env, collection_id: u64) -> Vec<u64>;

    /// List all NFT IDs owned by a user.
    fn get_user_nfts(env: Env, user: Address) -> Vec<u64>;
}
    // ── Backer rewards (crowdfunding tiers & perks) ───────────────────────────

    /// Create a crowdfunding campaign for tiered backer rewards.
    fn create_backer_campaign(
        env: Env,
        source_chain: String,
        source_pledge_id: u64,
        destination_chain: String,
        merchant: Address,
        payer: Address,
        token: Address,
        amount: i128,
        memo: Option<String>,
    ) -> u64;

    /// Update the status of a cross-chain pledge
    fn update_cross_chain_pledge_status(
        env: Env,
        pledge_id: u64,
        new_status: CrossChainPledgeStatus,
    );

    /// Get a cross-chain pledge by ID
    fn get_cross_chain_pledge(env: Env, pledge_id: u64) -> CrossChainPledge;

    /// Get a cross-chain pledge by source chain and source pledge ID
    fn get_cross_chain_pledge_by_source(
        env: Env,
        source_chain: String,
        source_pledge_id: u64,
    ) -> CrossChainPledge;

    /// Get all cross-chain pledges
    fn get_all_cross_chain_pledges(env: Env) -> Vec<CrossChainPledge>;
}

