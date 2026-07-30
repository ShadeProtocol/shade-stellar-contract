use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    NotAuthorized = 1,
    AlreadyInitialized = 2,
    NotInitialized = 3,
    Reentrancy = 4,
    MerchantAlreadyRegistered = 5,
    MerchantNotFound = 6,
    InvalidAmount = 7,
    InvoiceNotFound = 8,
    ContractPaused = 9,
    ContractNotPaused = 10,
    MerchantKeyNotFound = 11,
    TokenNotAccepted = 12,
    NonceAlreadyUsed = 14,
    InvalidInvoiceStatus = 16,
    RefundPeriodExpired = 17,
    WasmHashNotSet = 18,
    MerchantAccountNotSet = 20,
    InvalidInterval = 21,
    PlanNotFound = 22,
    PlanNotActive = 23,
    SubscriptionNotFound = 24,
    SubscriptionNotActive = 25,
    ChargeTooEarly = 26,
    InvoiceExpired = 27,
    InvoiceNotPaid = 28,
    PayerNotAvailable = 29,
    InsufficientBalance = 30,
    MerchantNotActive = 32,
    InvalidDescription = 33,
    OracleNotConfigured = 34,
    OraclePriceUnavailable = 35,
    EscrowNotExpired = 36,
    EscrowAlreadyRefunded = 37,
    TicketNotListed = 38,
    TicketAlreadyListed = 39,
    TokenNotAcceptedByMerchant = 41,
    FeeUpdateTooEarly = 42,
    NoPendingFeeUpdate = 43,
    InvalidSwapPath = 44,
    InvalidSlippage = 45,
    EventNotFound = 46,
    EventSoldOut = 47,
    InvalidCapacity = 48,
    InvalidEventDate = 49,
    InvalidRoyaltyBps = 50,
    TicketNotFound = 51,
    NotTicketOwner = 52,
    InvalidResalePrice = 54,
    NotFound = 55,

    // ── Campaign categories & tagging (#352) ──────────────────────────────
    CampaignCategoryNotFound = 66,
    CampaignCategoryAlreadyExists = 67,
    CampaignCategoryInactive = 68,
    CampaignTagNotFound = 69,
    CampaignTagAlreadyExists = 70,

    // ── Campaign system ───────────────────────────────────────────────────
    CampaignNotFound = 71,
    InvalidCampaignGoal = 72,
    InvalidCampaignDeadline = 73,
    CampaignInactive = 74,
    NotCampaignMerchant = 75,
    CampaignExpired = 76,
    AffiliateNotFound = 77,
    CampaignEnded = 78,
    CampaignNotActive = 79,

    // ── Financial penalties for malicious campaigns (#360) ────────────────
    /// Campaign is not in a state that allows penalty reports.
    CampaignNotPenalizable = 80,
    /// The penalty report referenced does not exist.
    PenaltyReportNotFound = 81,
    /// The penalty report has already been resolved.
    PenaltyReportAlreadyResolved = 82,
    /// The caller is not authorized to resolve penalty reports (admin only).
    NotPenaltyResolver = 83,
    /// The suggested penalty amount exceeds the campaign's slasable funds.
    PenaltyExceedsSlasableFunds = 84,

    // ── Multi-sig massive withdrawal ─────────────────────────────────────
    BelowMultiSigThreshold = 85,
    MultiSigSignersNotSet = 86,
    InvalidQuorum = 87,
    NotASigner = 88,
    AlreadyApproved = 89,
    ProposalNotFound = 90,
    ProposalNotPending = 91,
    QuorumNotReached = 92,
    NotProposer = 93,
    ThresholdNotSet = 94,

    // ── Escrow ───────────────────────────────────────────────────────────
    EscrowNotFound = 95,
    InvalidEscrowStatus = 96,

    // ── Backer rewards ───────────────────────────────────────────────────
    InvalidRewardTier = 97,
    PledgeBelowTierMinimum = 98,
    RewardTierAtCapacity = 99,
    BackerRewardAlreadyFulfilled = 100,
    NotBacker = 101,
    PerkNotFound = 102,
    PerkAlreadyClaimed = 103,
    BackerRewardNotFulfilled = 104,
    InvalidTierOrdering = 105,

    // ── Bridge ───────────────────────────────────────────────────────────
    BridgeDepositProcessed = 106,
    NftError = 107,
}

/// DAO governance errors. Kept in a separate enum.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum GovernanceError {
    NotGovMember = 200,
    GovNotConfigured = 201,
    InvalidGovConfig = 202,
    ProposalNotFound = 203,
    ProposalNotActive = 204,
    VotingClosed = 205,
    VotingStillOpen = 206,
    AlreadyVoted = 207,
}

/// Escrow / expired-refund errors.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum EscrowError {
    EscrowNotExpired = 300,
    EscrowAlreadyRefunded = 301,
}
