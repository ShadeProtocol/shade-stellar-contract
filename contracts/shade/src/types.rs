use soroban_sdk::{contracttype, Address, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    FeeInBasisPoints(Address),
    FeeAmount(Address),
    ContractInfo,
    AcceptedTokens,
    Merchant(u64),
    MerchantKey(Address),
    MerchantCount,
    MerchantId(Address),
    TokenFee(Address),
    MerchantTokens(Address),
    MerchantBalance(Address),
    MerchantAccount(u64),
    Invoice(u64),
    InvoiceCount,
    ReentrancyStatus,
    AccountWasmHash,
    Role(Address, Role),
    UsedNonce(Address, BytesN<32>),
    // --- Subscription engine ---
    SubscriptionPlan(u64),
    Subscription(u64),
    PlanCount,
    SubscriptionCount,
    // --- Time-locked fee updates ---
    PendingTokenFee(Address),
    // --- Fee discount system ---
    MerchantVolume(Address, Address),
    UserTransactions(Address),
    MerchantAnalytics(Address, Address),
    MerchantAnalyticsSummary(Address),
    PlatformAccount,
    /// Per-merchant platform fee override in basis points for a token.
    MerchantPlatformFee(u64, Address),
    TokenOracle(Address),
    MerchantAutoWithdrawalThreshold(u64, Address),
    MerchantAutoWithdrawalRecipient(u64),
    // --- Event system ---
    Event(u64),
    EventCount,
    Ticket(u64),
    TicketCount,
    EventTickets(u64),
    UserTickets(Address),
    // --- Global token analytics ---
    TokenAnalytics(Address),
    TokenVolume(Address),
    // --- Crowdfund Leaderboard ---
    CampaignOwner(u64),
    CampaignTopDonors(u64),
    CampaignDonorAmount(u64, Address),
    // --- Campaign categories & tagging system (#352) ---
    /// A predefined campaign category created by the admin.
    CampaignCategory(u64),
    /// Total number of campaign categories ever created (never decreases).
    CampaignCategoryCount,
    /// Name -> category_id, used to enforce unique category names.
    CampaignCategoryName(String),
    /// A free-form campaign tag created by a merchant.
    CampaignTag(u64),
    /// Total number of campaign tags ever created (never decreases).
    CampaignTagCount,
    /// Name -> tag_id, used to enforce unique tag names.
    CampaignTagName(String),
    /// A marketing/fundraising campaign registered by a merchant.
    Campaign(u64),
    /// Total number of campaigns ever created (never decreases).
    CampaignCount,
    /// Reverse index: category_id -> ordered list of campaign IDs.
    CategoryCampaigns(u64),
    /// Reverse index: tag_id -> ordered list of campaign IDs.
    TagCampaigns(u64),
    /// Reverse index: merchant_id -> ordered list of campaign IDs.
    MerchantCampaigns(u64),
    /// Reverse index: campaign_id -> ordered list of attached tag IDs.
    CampaignTagList(u64),
    // --- Cross-chain pledge system ---
    CrossChainPledge(u64),
    CrossChainPledgeCount,
    PledgeIdBySourceChain(String, u64), // source_chain, source_pledge_id
    // --- Multi-sig massive withdrawal ---
    /// Threshold (in token base units) above which a withdrawal requires multi-sig approval.
    MultiSigThreshold(Address),
    /// Ordered list of addresses that are registered as multi-sig signers.
    MultiSigSigners,
    /// Required number of approvals before a pending withdrawal can execute.
    MultiSigQuorum,
    /// A specific pending withdrawal proposal, keyed by proposal ID.
    WithdrawalProposal(u64),
    /// Running counter for withdrawal proposal IDs.
    WithdrawalProposalCount,
    /// Whether a particular signer has approved a particular proposal.
    WithdrawalApproval(u64, Address),
    // --- Escrow system ---
    Escrow(u64),
    EscrowCount,
    // --- Campaign fundraising engine (staking/slashing/penalties) ---
    StakeableCampaign(u64),
    StakeableCampaignCount,
    CampaignParticipants(u64),
    CampaignParticipant(u64, Address),
    CampaignAffiliate(u64, Address),
    // --- Financial penalties for malicious campaigns (#360) ---
    /// A report filed against a campaign for malicious behavior.
    CampaignPenaltyReport(u64),
    /// Total number of penalty reports ever created.
    CampaignPenaltyReportCount,
    /// Per-campaign penalty state tracking (total slashed, penalty count).
    CampaignPenaltyState(u64),
    // --- NFT reward system ---
    NftCollection(u64),
    NftCollectionCount,
    Nft(u64),
    NftCount,
    CollectionNfts(u64),
    UserNfts(Address),
    NftClaimed(u64, Address),
    // --- Backer rewards (crowdfunding tiers & perks) ---
    BackerCampaign(u64),
    BackerCampaignCount,
    BackerRewardTiers(u64),
    BackerPledge(u64, Address),
    BackerSelectedTier(u64, Address),
    BackerRewardFulfilled(u64, Address),
    BackerPerkClaimed(u64, Address, u32),
    BackerTierBackerCount(u64, u32),
}


#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractInfo {
    pub admin: Address,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Merchant {
    pub id: u64,
    pub address: Address,
    pub active: bool,
    pub verified: bool,
    pub date_registered: u64,
    pub account: Address,
    pub webhook: String,
    /// Optional recipient for auto-withdrawals. Defaults to the merchant
    /// address when unset.
    pub auto_withdrawal_recipient: Option<Address>,
    /// Per-token auto-withdrawal thresholds.
    pub auto_withdrawal_thresholds: Vec<AutoWithdrawalThreshold>,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum InvoiceStatus {
    Pending = 0,
    Paid = 1,
    Cancelled = 2,
    Refunded = 3,
    PartiallyRefunded = 4,
    PartiallyPaid = 5,
    Draft = 6,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum InvoicePricingMode {
    FixedCrypto = 0,
    FixedFiat = 1,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiatPricing {
    pub currency: String,
    pub amount: i128,
    pub decimals: u32,
}

/// Soroban-compatible optional wrapper for FiatPricing.
/// `Option<FiatPricing>` cannot be used directly inside a `#[contracttype]`
/// struct because the SDK does not implement the required XDR conversions for
/// `Option<T>` where T is a user-defined struct. An explicit enum variant is
/// the idiomatic workaround.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FiatPricingData {
    None,
    Some(FiatPricing),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Invoice {
    pub id: u64,
    pub description: soroban_sdk::String,
    pub amount: i128,
    pub token: Address,
    pub status: InvoiceStatus,
    pub merchant_id: u64,
    pub payer: Option<Address>,
    pub date_created: u64,
    pub date_paid: Option<u64>,
    pub amount_paid: i128,
    pub amount_refunded: i128,
    pub expires_at: Option<u64>,
    pub pricing_mode: InvoicePricingMode,
    pub fiat_pricing: FiatPricingData,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MerchantFilter {
    pub is_active: Option<bool>,
    pub is_verified: Option<bool>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvoiceFilter {
    pub status: Option<u32>,
    pub merchant: Option<Address>,
    pub min_amount: Option<u128>,
    pub max_amount: Option<u128>,
    pub start_date: Option<u64>,
    pub end_date: Option<u64>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Role {
    Admin,
    Manager,
    Operator,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VolumeDiscount {
    pub min_volume: i128,
    pub discount_bps: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OracleConfig {
    pub contract: Address,
    pub price_decimals: u32,
    pub token_decimals: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MerchantAnalytics {
    pub merchant: Address,
    pub token: Address,
    pub total_volume: i128,
    pub total_fees: i128,
    pub transaction_count: u64,
    pub last_updated: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MerchantAnalyticsSummary {
    pub merchant: Address,
    pub total_volume: i128,
    pub total_fees: i128,
    pub transaction_count: u64,
    pub last_updated: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CrossChainPledgeStatus {
    Pending,
    Completed,
    Failed,
    Refunded,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrossChainPledge {
    pub id: u64,
    pub source_chain: String,
    pub source_pledge_id: u64,
    pub destination_chain: String,
    pub merchant: Address,
    pub payer: Address,
    pub token: Address,
    pub amount: i128,
    pub status: CrossChainPledgeStatus,
    pub created_at: u64,
    pub updated_at: u64,
    pub memo: Option<String>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrossChainBridgePayload {
    pub invoice_id: u64,
    pub merchant: Address,
    pub payer: Option<Address>,
    pub source_chain: String,
    pub destination_chain: String,
    pub token: Address,
    pub amount: i128,
    pub destination_recipient: String,
    pub memo: Option<String>,
}

/// A confirmed external-chain deposit recorded by an authorized bridge listener.
///
/// The `source_tx_id` is the 32-byte transaction hash on the origin chain and
/// doubles as the global idempotency key (see `DataKey::ProcessedBridgeDeposit`).
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeDeposit {
    pub id: u64,
    pub source_chain: String,
    pub source_tx_id: BytesN<32>,
    pub listener: Address,
    pub token: Address,
    pub amount: i128,
    pub recipient: Address,
    pub timestamp: u64,
}

// ── Time-locked fee update ────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PendingFee {
    pub token: Address,
    pub fee: i128,
    pub proposed_at: u64,
}

// --- Subscription engine ---

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionPlan {
    pub id: u64,
    /// Numeric merchant ID - used to look up the merchant's account contract.
    pub merchant_id: u64,
    /// The merchant's wallet address - needed for event emission and auth checks.
    pub merchant: Address,
    /// Human-readable description of the plan.
    pub description: soroban_sdk::String,
    /// Token used for billing.
    pub token: Address,
    /// Amount charged per interval (in token base units).
    pub amount: i128,
    /// Billing interval in seconds (e.g. 2_592_000 = 30 days).
    pub interval: u64,
    /// Whether this plan is accepting new subscribers.
    pub active: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Subscription {
    pub id: u64,
    pub plan_id: u64,
    pub customer: Address,
    /// Copied from the plan for quick access during auth checks.
    pub merchant_id: u64,
    pub status: SubscriptionStatus,
    pub date_created: u64,
    /// Ledger timestamp of the last successful charge.
    /// Starts at 0 so the first charge is available immediately.
    pub last_charged: u64,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum SubscriptionStatus {
    Active = 0,
    Cancelled = 1,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenAnalytics {
    pub token: Address,
    pub total_volume: i128,
    pub total_fees: i128,
    pub transaction_count: u64,
    pub unique_merchants: u64,
    pub last_updated: u64,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum TransactionType {
    InvoicePayment = 0,
    SubscriptionCharge = 1,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transaction {
    pub transaction_type: TransactionType,
    pub ref_id: u64,
    pub amount: i128,
    pub token: Address,
    pub description: soroban_sdk::String,
    pub date: u64,
    pub merchant_id: u64,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EventStatus {
    Active = 0,
    Cancelled = 1,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EscrowStatus {
    Created = 0,
    Funded = 1,
    Released = 2,
    Refunded = 3,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Escrow {
    pub id: u64,
    pub buyer: Address,
    pub seller: Address,
    pub token: Address,
    pub amount: i128,
    pub status: EscrowStatus,
    pub invoice_id: Option<u64>,
    pub date_created: u64,
    pub date_funded: Option<u64>,
    pub date_released: Option<u64>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Event {
    pub id: u64,
    pub merchant_id: u64,
    pub name: String,
    pub ticket_price: i128,
    pub token: Address,
    pub capacity: u32,
    pub sold: u32,
    pub date: u64,
    /// Scheduled event date (unix seconds). Must be >= ledger timestamp at creation.
    pub event_date: u64,
    /// Royalty paid to the organizer on each resale, in basis points (10_000 = 100%).
    pub royalty_bps: u32,
    /// Early-bird cutoff timestamp. `0` disables early-bird pricing.
    pub early_bird_end: u64,
    /// Discount during early-bird period, in basis points.
    pub early_bird_discount_bps: u32,
    /// Markup applied after early-bird period, in basis points.
    pub late_markup_bps: u32,
    /// True once the event is cancelled.
    pub cancelled: bool,
    /// True once all ticket refunds have been processed.
    pub refunds_processed: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ticket {
    pub id: u64,
    pub event_id: u64,
    pub owner: Address,
    pub minted_at: u64,
    /// Amount paid on primary purchase, used for cancellation refunds.
    pub purchase_price: i128,
}

// ── Campaign system (consolidated) ────────────────────────────────────────────

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum CampaignStatus {
    Active = 0,
    Ended = 1,
    Cancelled = 2,
    /// Campaign has been reported and is under review for malicious behavior.
    UnderReview = 3,
    /// Campaign has been confirmed malicious and penalized.
    Penalized = 4,
}

/// Status of a penalty report filed against a campaign.
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum PenaltyReportStatus {
    /// Report filed, awaiting admin review.
    Pending = 0,
    /// Admin confirmed the report; penalties applied.
    Upheld = 1,
    /// Admin dismissed the report; no penalties.
    Dismissed = 2,
}

/// On-chain fundraising / promotional campaign created by a merchant.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Campaign {
    pub id: u64,
    pub merchant_id: u64,
    pub merchant: Address,
    pub title: String,
    pub description: String,
    pub category_id: u64,
    pub tags: Vec<u64>,
    /// Fundraising goal in token base units. 0 = open-ended (no specific goal).
    pub goal_amount: i128,
    pub token: Address,
    pub deadline: u64,
    pub raised_amount: i128,
    pub active: bool,
    pub created_at: u64,
    /// Current lifecycle status.
    pub status: CampaignStatus,
    /// Total amount slashed from this campaign via financial penalties.
    pub total_slashed: i128,
    /// Number of penalty reports upheld against this campaign.
    pub penalty_count: u32,
}

/// A timestamped update / news post published by the merchant on an active campaign.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CampaignAnnouncement {
    pub id: u64,
    pub campaign_id: u64,
    pub title: String,
    pub content: String,
    pub posted_at: u64,
}

/// A report filed against a campaign alleging malicious or fraudulent behavior.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CampaignPenaltyReport {
    pub id: u64,
    pub campaign_id: u64,
    /// Address that filed the report.
    pub reporter: Address,
    /// Reason / description of the alleged malicious behavior.
    pub reason: String,
    /// Amount suggested to be slashed as a penalty.
    pub suggested_penalty: i128,
    /// Current status of the report.
    pub status: PenaltyReportStatus,
    /// Ledger timestamp when the report was filed.
    pub filed_at: u64,
    /// Ledger timestamp when the report was resolved (upheld or dismissed).
    pub resolved_at: Option<u64>,
    /// If upheld, the address of the admin who resolved it.
    pub resolved_by: Option<Address>,
    /// If upheld, the actual penalty amount applied (may differ from suggested).
    pub applied_penalty: Option<i128>,
}

/// Per-campaign penalty state tracking.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CampaignPenaltyState {
    pub campaign_id: u64,
    pub total_slashed: i128,
    pub report_count: u32,
    pub upheld_count: u32,
    pub last_penalty_at: Option<u64>,
}

/// Campaign participant for staking/slashing system.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CampaignParticipant {
    pub campaign_id: u64,
    pub participant: Address,
    pub contributed: i128,
    pub staked: i128,
    pub slashed: i128,
    pub commissions_paid: i128,
    pub score: i128,
}

/// Affiliate registered for a campaign.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CampaignAffiliate {
    pub campaign_id: u64,
    pub affiliate: Address,
    pub commission_bps: u32,
    pub total_paid: i128,
    pub active: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PaymentRoute {
    Direct,
    Swap(SwapRoute),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapRoute {
    pub router: Address,
    pub path: Vec<Address>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlatformFeeSplit {
    pub gross_amount: i128,
    pub platform_fee: i128,
    pub merchant_amount: i128,
    pub fee_bps_applied: i128,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum PlatformFeeRouteKind {
    Invoice = 0,
    Subscription = 1,
    TicketPurchase = 2,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentPayload {
    pub input_token: Address,
    pub settlement_token: Address,
    pub route: PaymentRoute,
    pub max_slippage_bps: Option<u32>,
}

// ── Campaign categories & tagging (#352) ──────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CampaignCategory {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub active: bool,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CampaignTag {
    pub id: u64,
    pub name: String,
    pub creator: Address,
    pub timestamp: u64,
}

// ── Multi-sig massive withdrawal ──────────────────────────────────────────────

/// Current lifecycle state of a withdrawal proposal.
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum WithdrawalProposalStatus {
    /// Awaiting the required number of signer approvals.
    Pending = 0,
    /// Quorum reached; funds have been transferred.
    Executed = 1,
    /// Cancelled by the proposer or an admin before execution.
    Cancelled = 2,
}

/// A pending or completed massive-withdrawal proposal.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WithdrawalProposal {
    /// Unique, auto-incremented identifier.
    pub id: u64,
    /// Merchant whose balance is being withdrawn.
    pub merchant: Address,
    /// Token to withdraw.
    pub token: Address,
    /// Amount requested (in token base units).
    pub amount: i128,
    /// Destination address for the funds.
    pub recipient: Address,
    /// Number of approvals collected so far.
    pub approvals: u32,
    /// Current lifecycle status.
    pub status: WithdrawalProposalStatus,
    /// Ledger timestamp when the proposal was created.
    pub created_at: u64,
    /// Ledger timestamp of the last status change (approval/execution/cancellation).
    pub updated_at: u64,
    /// Optional human-readable note attached by the proposer.
    pub note: soroban_sdk::String,
}

/// Runtime configuration for the multi-sig guard.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultiSigConfig {
    /// Minimum withdrawal amount that triggers multi-sig review (per token).
    /// A value of 0 means multi-sig is disabled for that token.
    pub threshold: i128,
    /// Addresses authorised to approve withdrawal proposals.
    pub signers: soroban_sdk::Vec<Address>,
    /// Number of approvals required to execute a proposal.
    pub quorum: u32,
}

// ── On-chain search and filtering utilities ───────────────────────────────────

/// Filter parameters for querying subscription plans.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionPlanFilter {
    /// Restrict to plans owned by this merchant address.
    pub merchant: Option<Address>,
    /// Restrict to active (`true`) or inactive (`false`) plans.
    pub active: Option<bool>,
    /// Restrict to plans billed in this token.
    pub token: Option<Address>,
}

/// Filter parameters for querying individual subscriptions.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionFilter {
    /// Restrict to subscriptions belonging to this plan.
    pub plan_id: Option<u64>,
    /// Restrict to subscriptions held by this customer address.
    pub customer: Option<Address>,
    /// Restrict to Active (0) or Cancelled (1) subscriptions.
    pub status: Option<u32>,
}

/// Filter parameters for querying on-chain events (ticketing).
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventFilter {
    /// Restrict to events owned by this merchant address.
    pub merchant: Option<Address>,
    /// When `true` only cancelled events are returned; `false` for active ones.
    pub cancelled: Option<bool>,
    /// Earliest `event_date` (unix seconds) to include.
    pub start_date: Option<u64>,
    /// Latest `event_date` (unix seconds) to include.
    pub end_date: Option<u64>,
    /// Only include events with at least this many remaining seats.
    pub min_available: Option<u32>,
}

/// Filter parameters for querying withdrawal proposals.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WithdrawalProposalFilter {
    /// Restrict to proposals opened by this merchant address.
    pub merchant: Option<Address>,
    /// Restrict by proposal status (0=Pending, 1=Executed, 2=Cancelled).
    pub status: Option<u32>,
    /// Restrict to proposals for this token.
    pub token: Option<Address>,
    /// Only include proposals created at or after this timestamp.
    pub created_after: Option<u64>,
}

/// A page of results together with cursor metadata for keyset pagination.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageInfo {
    /// Total items returned in this page.
    pub count: u32,
    /// ID of the last item in this page; pass as `cursor` in the next call.
    /// `0` indicates there are no more pages.
    pub next_cursor: u64,
    /// Whether more pages follow.
    pub has_next_page: bool,
}

/// A paginated slice of invoices.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvoicePage {
    pub items: soroban_sdk::Vec<Invoice>,
    pub page_info: PageInfo,
}

/// A paginated slice of merchants.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MerchantPage {
    pub items: soroban_sdk::Vec<Merchant>,
    pub page_info: PageInfo,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CampaignFilter {
    pub is_active: Option<bool>,
    pub category_id: Option<u64>,
    pub tag_id: Option<u64>,
    pub merchant_id: Option<u64>,
}

/// Singleton governance configuration and counters. `voting_period == 0` is the
/// sentinel for "not yet configured".
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovState {
    pub voting_period: u64,
    pub quorum_bps: u32,
    pub member_count: u32,
    pub proposal_count: u64,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ProposalStatus {
    /// Open for voting.
    Active = 0,
    /// Passed quorum + majority and the upgrade was applied.
    Executed = 1,
    /// Failed quorum or majority after the voting window closed.
    Defeated = 2,
}

/// A council-governed proposal to upgrade the contract's WASM to `wasm_hash`.
/// Voting is one-member-one-vote; `approvals`/`rejections` are head counts.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UpgradeProposal {
    pub id: u64,
    pub proposer: Address,
    pub wasm_hash: BytesN<32>,
    pub created_at: u64,
    pub voting_ends_at: u64,
    pub approvals: u32,
    pub rejections: u32,
    pub status: ProposalStatus,
}

/// Donor info for leaderboard tracking.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DonorInfo {
    pub donor: Address,
    pub amount: i128,
}

/// A pledge made by a contributor to a crowdfunding campaign.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pledge {
    pub id: u64,
    pub campaign_id: u64,
    pub contributor: Address,
    pub amount: i128,
    pub token: Address,
    pub refunded: bool,
    pub created_at: u64,
}
