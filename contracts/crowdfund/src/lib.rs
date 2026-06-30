#![no_std]

mod errors;
#[cfg(test)]
mod test;

use errors::CrowdfundError;
use soroban_sdk::{
    contract, contractclient, contractevent, contractimpl, contracttype, panic_with_error, token,
    vec, Address, Env, String, Vec,
};

#[contractclient(name = "InvoicePaymentClient")]
trait InvoicePayment {
    fn pay_invoice(env: Env, payer: Address, invoice_id: u64);
}

#[contractclient(name = "MerchantAccountRefundClient")]
trait MerchantAccountRefund {
    fn refund(env: Env, token: Address, amount: i128, to: Address);
}

#[contractevent]
pub struct CampaignExecutedEvent {
    pub amount: i128,
}

#[contractevent]
pub struct RefundClaimedEvent {
    pub contributor: Address,
    pub amount: i128,
}

#[contractevent]
pub struct StretchGoalReachedEvent {
    pub milestone_index: u32,
    pub threshold: i128,
}

#[contractevent]
pub struct RewardFulfilledEvent {
    pub backer: Address,
}

#[contracttype]
#[derive(Clone)]
pub struct RewardTier {
    pub min_pledge: i128,
    pub name: String,
}

#[contractevent]
pub struct RewardTierSelectedEvent {
    pub contributor: Address,
    pub tier_index: u32,
}

#[contractevent]
pub struct MilestoneUnlockedEvent {
    pub index: u32,
}

#[contractevent]
pub struct MilestoneReleasedEvent {
    pub index: u32,
    pub amount: i128,
}

#[contractevent]
pub struct MilestoneVoteCastEvent {
    pub index: u32,
    pub voter: Address,
    pub approve: bool,
    pub weight: i128,
}

#[contractevent]
pub struct MatchingPoolFundedEvent {
    pub sponsor: Address,
    pub amount: i128,
}

#[contractevent]
pub struct MatchAppliedEvent {
    pub contributor: Address,
    pub matched_amount: i128,
}

#[contractevent]
pub struct PledgeCommentAddedEvent {
    pub contributor: Address,
    pub comment: String,
}

#[contractevent]
pub struct PledgeReceivedEvent {
    pub contributor: Address,
    pub amount: i128,
}

// ── Affiliate / referral tracking ─────────────────────────────────────────────

/// Immutable affiliate registration record.
#[contracttype]
#[derive(Clone)]
pub struct AffiliateInfo {
    /// The wallet address that will receive commission payouts.
    pub wallet: Address,
    /// Commission rate in basis points (1 bp = 0.01 %; max 10 000 = 100 %).
    pub commission_bps: u32,
    /// Total tokens contributed by backers who used this affiliate's code.
    pub total_referred_amount: i128,
    /// Total commission accrued (in token base units) — not yet paid out.
    pub total_commission_earned: i128,
    /// Number of unique contributors referred.
    pub referral_count: u32,
}

/// Per-contributor referral record written at contribution time.
#[contracttype]
#[derive(Clone)]
pub struct ReferralRecord {
    pub affiliate_code: String,
    pub amount: i128,
    pub timestamp: u64,
}

// ── Affiliate events ──────────────────────────────────────────────────────────

/// Emitted when the organizer registers a new affiliate.
#[contractevent]
pub struct AffiliateRegisteredEvent {
    pub affiliate_code: String,
    pub wallet: Address,
    pub commission_bps: u32,
    pub timestamp: u64,
}

/// Emitted when a contribution is made through an affiliate link.
#[contractevent]
pub struct AffiliateContributionEvent {
    pub affiliate_code: String,
    pub contributor: Address,
    pub amount: i128,
    pub timestamp: u64,
}

/// Emitted when a commission is accrued for an affiliate.
#[contractevent]
pub struct AffiliateCommissionEvent {
    pub affiliate_code: String,
    pub wallet: Address,
    pub commission: i128,
    pub timestamp: u64,
}

#[contractevent]
pub struct BatchRefundProcessedEvent {
    pub total_refunded: i128,
    pub contributor_count: u32,
}

#[contracttype]
enum DataKey {
    Organizer,
    Token,
    Goal,
    Deadline,
    Raised,
    // Tracks whether the campaign has been executed (funds withdrawn by organizer).
    Executed,
    // Stores per-contributor pledge amounts.
    Pledge(Address),
    // Ordered list of stretch goal thresholds.
    StretchGoals,
    // Tracks which stretch goal indexes have already been emitted.
    StretchTriggered(u32),
    // Tracks whether the organizer has fulfilled a specific backer's reward.
    RewardFulfilled(Address),
    // Ordered list of reward tiers set by the organizer.
    RewardTiers,
    // Tier index selected by a specific contributor.
    SelectedTier(Address),
    // Milestone percentages in basis points (set by organizer, must sum to 10_000).
    MilestonePercentages,
    // Whether the organizer has unlocked a specific milestone for release.
    MilestoneUnlocked(u32),
    // Whether a specific milestone's funds have been released.
    MilestoneReleased(u32),
    // Backer vote weight totals for a specific milestone.
    MilestoneApprovalWeight(u32),
    MilestoneRejectionWeight(u32),
    // Tracks whether a backer already voted for a specific milestone.
    MilestoneVote(u32, Address),
    // Shade gateway contract address for payment processing.
    ShadeGateway,
    // Merchant ID for this campaign (registered on Shade).
    MerchantId,
    // Merchant account address for refunds.
    MerchantAccount,
    // Ordered list of all contributors for batch refunds.
    Contributors,
    // Tracks whether batch refund has been processed.
    RefundProcessed,
    // Sponsor funds reserved to match incoming pledges.
    MatchingPool,
    // Public comment attached to a contributor pledge.
    PledgeComment(Address),
    // ── Affiliate / referral tracking ────────────────────────────────────────
    // Affiliate info keyed by the affiliate's string code.
    Affiliate(String),
    // List of all registered affiliate codes (for enumeration).
    AffiliateList,
    // Per-contributor referral record (code used when they contributed).
    ContributorReferral(Address),
    // Accrued commission per affiliate code (token base units, pending payout).
    AffiliateCommission(String),
}

#[contract]
pub struct CrowdfundContract;

#[contractimpl]
impl CrowdfundContract {
    const MAX_COMMENT_BYTES: u32 = 280;
    /// Initialise a campaign. Sets the funding goal (in token base units)
    /// and the deadline (Unix timestamp after which no contributions are
    /// accepted). Only callable once.
    ///
    /// # Arguments
    /// * `organizer` – address that will receive funds if the goal is met.
    /// * `token`     – accepted payment token.
    /// * `goal`      – target amount in token base units (must be > 0).
    /// * `deadline`  – Unix timestamp of the campaign end (must be in the future).
    pub fn init_campaign(
        env: Env,
        organizer: Address,
        token: Address,
        goal: i128,
        deadline: u64,
    ) {
        if env.storage().persistent().has(&DataKey::Organizer) {
            panic_with_error!(&env, CrowdfundError::AlreadyInitialized);
        }
        if goal <= 0 {
            panic_with_error!(&env, CrowdfundError::InvalidGoal);
        }
        if deadline <= env.ledger().timestamp() {
            panic_with_error!(&env, CrowdfundError::InvalidDeadline);
        }

        env.storage().persistent().set(&DataKey::Organizer, &organizer);
        env.storage().persistent().set(&DataKey::Token, &token);
        env.storage().persistent().set(&DataKey::Goal, &goal);
        env.storage().persistent().set(&DataKey::Deadline, &deadline);
        env.storage().persistent().set(&DataKey::Raised, &0_i128);
        env.storage().persistent().set(&DataKey::Executed, &false);
        env.storage().persistent().set(&DataKey::RefundProcessed, &false);
        env.storage().persistent().set(&DataKey::Contributors, &Vec::<Address>::new(&env));
    }

    /// Set the Shade gateway contract address. Only callable once by the organizer.
    pub fn set_shade_gateway(env: Env, shade_gateway: Address) {
        let organizer: Address = env.storage().persistent().get(&DataKey::Organizer)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));
        organizer.require_auth();
        if env.storage().persistent().has(&DataKey::ShadeGateway) {
            panic_with_error!(&env, CrowdfundError::AlreadyInitialized);
        }
        env.storage().persistent().set(&DataKey::ShadeGateway, &shade_gateway);
    }

    /// Register this campaign's Shade merchant ID. Only callable once by the organizer.
    pub fn set_merchant_id(env: Env, merchant_id: u64) {
        let organizer: Address = env.storage().persistent().get(&DataKey::Organizer)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));
        organizer.require_auth();
        if env.storage().persistent().has(&DataKey::MerchantId) {
            panic_with_error!(&env, CrowdfundError::AlreadyInitialized);
        }
        env.storage().persistent().set(&DataKey::MerchantId, &merchant_id);
    }

    /// Set the Shade merchant account address for refunds. Only callable once by the organizer.
    pub fn set_merchant_account(env: Env, merchant_account: Address) {
        let organizer: Address = env.storage().persistent().get(&DataKey::Organizer)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));
        organizer.require_auth();
        if env.storage().persistent().has(&DataKey::MerchantAccount) {
            panic_with_error!(&env, CrowdfundError::AlreadyInitialized);
        }
        env.storage().persistent().set(&DataKey::MerchantAccount, &merchant_account);
    }

    /// Process a pledge through the Shade gateway (#300).
    pub fn pledge(env: Env, contributor: Address, amount: i128, invoice_id: u64) {
        contributor.require_auth();
        if amount <= 0 { panic_with_error!(&env, CrowdfundError::InvalidAmount); }

        let deadline: u64 = env.storage().persistent().get(&DataKey::Deadline)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));
        if env.ledger().timestamp() > deadline { panic_with_error!(&env, CrowdfundError::CampaignEnded); }
        if env.storage().persistent().get(&DataKey::Executed).unwrap_or(false) {
            panic_with_error!(&env, CrowdfundError::AlreadyExecuted);
        }

        let shade_gateway: Address = env.storage().persistent().get(&DataKey::ShadeGateway)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::ShadeGatewayNotSet));
        let token_addr: Address = env.storage().persistent().get(&DataKey::Token)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        InvoicePaymentClient::new(&env, &shade_gateway).pay_invoice(&contributor, &invoice_id);

        let merchant_account: Address = env.storage().persistent().get(&DataKey::MerchantAccount)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::MerchantAccountNotSet));
        MerchantAccountRefundClient::new(&env, &merchant_account)
            .refund(&token_addr, &amount, &env.current_contract_address());

        let new_raised = Self::apply_pledge_with_matching(&env, contributor.clone(), amount);

        let prev: i128 = env.storage().persistent()
            .get(&DataKey::Pledge(contributor.clone())).unwrap_or(0);
        env.storage().persistent()
            .set(&DataKey::Pledge(contributor.clone()), &prev.saturating_add(amount));

        Self::track_contributor(&env, contributor.clone());
        Self::check_stretch_goals(&env, new_raised);
        PledgeReceivedEvent { contributor, amount }.publish(&env);
    }

    /// Contribute `amount` tokens to the campaign. The caller must have
    /// pre-approved the contract to spend at least `amount` from their
    /// balance. Panics after the deadline or if the campaign is not yet
    /// initialised.
    pub fn contribute(env: Env, contributor: Address, amount: i128) {
        contributor.require_auth();

        if amount <= 0 {
            panic_with_error!(&env, CrowdfundError::InvalidAmount);
        }

        let deadline: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::Deadline)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        if env.ledger().timestamp() > deadline {
            panic_with_error!(&env, CrowdfundError::CampaignEnded);
        }

        let token_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Token)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        let contract_addr = env.current_contract_address();
        token::TokenClient::new(&env, &token_addr)
            .transfer(&contributor, &contract_addr, &amount);

        let new_raised = Self::apply_pledge_with_matching(&env, contributor.clone(), amount);

        // Track contributor for batch refunds (#307).
        Self::track_contributor(&env, contributor);

        // Check and emit stretch goal events (#306).
        Self::check_stretch_goals(&env, new_raised);
    }

    /// Fund the sponsor matching pool used to amplify future pledges (#315).
    pub fn fund_matching_pool(env: Env, sponsor: Address, amount: i128) {
        sponsor.require_auth();
        if amount <= 0 {
            panic_with_error!(&env, CrowdfundError::InvalidAmount);
        }

        let token_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Token)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));
        let contract_addr = env.current_contract_address();
        token::TokenClient::new(&env, &token_addr).transfer(&sponsor, &contract_addr, &amount);

        let current: i128 = env.storage().persistent().get(&DataKey::MatchingPool).unwrap_or(0);
        env.storage()
            .persistent()
            .set(&DataKey::MatchingPool, &current.saturating_add(amount));
        MatchingPoolFundedEvent { sponsor, amount }.publish(&env);
    }

    /// Attach a public comment to a contributor pledge (#314).
    pub fn leave_comment(env: Env, contributor: Address, comment: String) {
        contributor.require_auth();
        let pledge: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Pledge(contributor.clone()))
            .unwrap_or(0);
        if pledge <= 0 {
            panic_with_error!(&env, CrowdfundError::NoPledge);
        }
        if comment.len() > Self::MAX_COMMENT_BYTES {
            panic_with_error!(&env, CrowdfundError::CommentTooLong);
        }

        env.storage()
            .persistent()
            .set(&DataKey::PledgeComment(contributor.clone()), &comment);
        PledgeCommentAddedEvent { contributor, comment }.publish(&env);
    }

    /// Retrieve a contributor's public pledge comment, if any.
    pub fn get_comment(env: Env, contributor: Address) -> Option<String> {
        env.storage()
            .persistent()
            .get(&DataKey::PledgeComment(contributor))
    }

    /// Read the currently available sponsor matching pool.
    pub fn matching_pool_balance(env: Env) -> i128 {
        env.storage().persistent().get(&DataKey::MatchingPool).unwrap_or(0)
    }

    // ── Affiliate / Referral Tracking (#349) ─────────────────────────────────

    /// Register a new affiliate. Only callable by the organizer.
    ///
    /// # Arguments
    /// * `affiliate_code`  – unique human-readable identifier (1–32 bytes).
    /// * `wallet`          – address that will earn commission.
    /// * `commission_bps`  – commission rate in basis points (1–10_000).
    pub fn register_affiliate(
        env: Env,
        affiliate_code: String,
        wallet: Address,
        commission_bps: u32,
    ) {
        let organizer: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Organizer)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));
        organizer.require_auth();

        // Validate code length: 1–32 bytes, non-empty.
        if affiliate_code.len() == 0 || affiliate_code.len() > 32 {
            panic_with_error!(&env, CrowdfundError::InvalidAffiliateCode);
        }

        // Validate commission bps.
        if commission_bps == 0 || commission_bps > 10_000 {
            panic_with_error!(&env, CrowdfundError::InvalidCommissionBps);
        }

        // Prevent duplicate registration.
        if env
            .storage()
            .persistent()
            .has(&DataKey::Affiliate(affiliate_code.clone()))
        {
            panic_with_error!(&env, CrowdfundError::AffiliateAlreadyExists);
        }

        let info = AffiliateInfo {
            wallet: wallet.clone(),
            commission_bps,
            total_referred_amount: 0,
            total_commission_earned: 0,
            referral_count: 0,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Affiliate(affiliate_code.clone()), &info);

        // Append code to the global list for enumeration.
        let mut list: Vec<String> = env
            .storage()
            .persistent()
            .get(&DataKey::AffiliateList)
            .unwrap_or_else(|| Vec::new(&env));
        list.push_back(affiliate_code.clone());
        env.storage()
            .persistent()
            .set(&DataKey::AffiliateList, &list);

        AffiliateRegisteredEvent {
            affiliate_code,
            wallet,
            commission_bps,
            timestamp: env.ledger().timestamp(),
        }
        .publish(&env);
    }

    /// Contribute to the campaign through an affiliate referral link.
    /// Records the referral and accrues commission to the affiliate.
    ///
    /// # Arguments
    /// * `contributor`     – address making the contribution.
    /// * `amount`          – token base-units to contribute (must be > 0).
    /// * `affiliate_code`  – the referral code provided by the affiliate.
    pub fn contribute_with_referral(
        env: Env,
        contributor: Address,
        amount: i128,
        affiliate_code: String,
    ) {
        contributor.require_auth();

        if amount <= 0 {
            panic_with_error!(&env, CrowdfundError::InvalidAmount);
        }

        let deadline: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::Deadline)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        if env.ledger().timestamp() > deadline {
            panic_with_error!(&env, CrowdfundError::CampaignEnded);
        }

        // Verify the affiliate code exists.
        let mut info: AffiliateInfo = env
            .storage()
            .persistent()
            .get(&DataKey::Affiliate(affiliate_code.clone()))
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::AffiliateNotFound));

        let token_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Token)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        let contract_addr = env.current_contract_address();
        token::TokenClient::new(&env, &token_addr)
            .transfer(&contributor, &contract_addr, &amount);

        // Apply matching (same as `contribute`).
        let new_raised = Self::apply_pledge_with_matching(&env, contributor.clone(), amount);

        // Track contributor for batch refunds.
        Self::track_contributor(&env, contributor.clone());

        // Check and emit stretch goal events.
        Self::check_stretch_goals(&env, new_raised);

        // Determine whether this contributor is new for this affiliate.
        let is_new_referral = !env
            .storage()
            .persistent()
            .has(&DataKey::ContributorReferral(contributor.clone()));

        // Persist the referral record for this contributor (first-touch wins).
        if is_new_referral {
            let record = ReferralRecord {
                affiliate_code: affiliate_code.clone(),
                amount,
                timestamp: env.ledger().timestamp(),
            };
            env.storage()
                .persistent()
                .set(&DataKey::ContributorReferral(contributor.clone()), &record);
            info.referral_count = info.referral_count.saturating_add(1);
        }

        // Accrue commission (calculated on the raw contributed amount, before matching).
        let commission = amount * info.commission_bps as i128 / 10_000;
        info.total_referred_amount = info.total_referred_amount.saturating_add(amount);
        info.total_commission_earned = info.total_commission_earned.saturating_add(commission);

        env.storage()
            .persistent()
            .set(&DataKey::Affiliate(affiliate_code.clone()), &info);

        // Accumulate global accrued commission for the code.
        let prev_commission: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::AffiliateCommission(affiliate_code.clone()))
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(
                &DataKey::AffiliateCommission(affiliate_code.clone()),
                &prev_commission.saturating_add(commission),
            );

        let ts = env.ledger().timestamp();

        AffiliateContributionEvent {
            affiliate_code: affiliate_code.clone(),
            contributor,
            amount,
            timestamp: ts,
        }
        .publish(&env);

        if commission > 0 {
            AffiliateCommissionEvent {
                affiliate_code,
                wallet: info.wallet,
                commission,
                timestamp: ts,
            }
            .publish(&env);
        }
    }

    /// Return the `AffiliateInfo` record for a given code.
    pub fn get_affiliate(env: Env, affiliate_code: String) -> AffiliateInfo {
        env.storage()
            .persistent()
            .get(&DataKey::Affiliate(affiliate_code))
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::AffiliateNotFound))
    }

    /// Return the total accrued commission for a given affiliate code.
    pub fn get_affiliate_commission(env: Env, affiliate_code: String) -> i128 {
        // Ensure the code exists.
        if !env
            .storage()
            .persistent()
            .has(&DataKey::Affiliate(affiliate_code.clone()))
        {
            panic_with_error!(&env, CrowdfundError::AffiliateNotFound);
        }
        env.storage()
            .persistent()
            .get(&DataKey::AffiliateCommission(affiliate_code))
            .unwrap_or(0)
    }

    /// Return the list of all registered affiliate codes.
    pub fn get_affiliate_codes(env: Env) -> Vec<String> {
        env.storage()
            .persistent()
            .get(&DataKey::AffiliateList)
            .unwrap_or_else(|| Vec::new(&env))
    }

    /// Return the referral record for a contributor, if they used a referral code.
    pub fn get_contributor_referral(env: Env, contributor: Address) -> Option<ReferralRecord> {
        env.storage()
            .persistent()
            .get(&DataKey::ContributorReferral(contributor))
    }
    /// Withdraw funds to the organizer after deadline if goal was met (#303).
    pub fn execute_campaign(env: Env) {
        let organizer: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Organizer)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        organizer.require_auth();

        let deadline: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::Deadline)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        if env.ledger().timestamp() <= deadline {
            panic_with_error!(&env, CrowdfundError::CampaignNotEnded);
        }

        let goal: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Goal)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        let raised: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Raised)
            .unwrap_or(0);

        if raised < goal {
            panic_with_error!(&env, CrowdfundError::GoalNotReached);
        }

        let executed: bool = env
            .storage()
            .persistent()
            .get(&DataKey::Executed)
            .unwrap_or(false);

        if executed {
            panic_with_error!(&env, CrowdfundError::AlreadyExecuted);
        }

        // Milestone mode: use release_milestone instead.
        if env.storage().persistent().has(&DataKey::MilestonePercentages) {
            panic_with_error!(&env, CrowdfundError::MilestonesActive);
        }

        env.storage().persistent().set(&DataKey::Executed, &true);

        let token_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Token)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        let contract_addr = env.current_contract_address();
        token::TokenClient::new(&env, &token_addr)
            .transfer(&contract_addr, &organizer, &raised);

        CampaignExecutedEvent { amount: raised }.publish(&env);
    }

    /// Allow a backer to reclaim their pledge after deadline if goal was not met (#304).
    pub fn claim_refund(env: Env, contributor: Address) {
        contributor.require_auth();

        let deadline: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::Deadline)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        if env.ledger().timestamp() <= deadline {
            panic_with_error!(&env, CrowdfundError::CampaignNotEnded);
        }

        let goal: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Goal)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        let raised: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Raised)
            .unwrap_or(0);

        if raised >= goal {
            panic_with_error!(&env, CrowdfundError::GoalReached);
        }

        let pledge: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Pledge(contributor.clone()))
            .unwrap_or(0);

        if pledge == 0 {
            panic_with_error!(&env, CrowdfundError::NoPledge);
        }

        // Zero out pledge before transfer to prevent double-claim.
        env.storage()
            .persistent()
            .set(&DataKey::Pledge(contributor.clone()), &0_i128);

        let token_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Token)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        let contract_addr = env.current_contract_address();
        token::TokenClient::new(&env, &token_addr)
            .transfer(&contract_addr, &contributor, &pledge);

        RefundClaimedEvent { contributor: contributor.clone(), amount: pledge }.publish(&env);
    }

    /// Batch refund all contributors after a failed campaign (#307).
    /// Callable by anyone once deadline has passed and goal was not met.
    pub fn batch_refund(env: Env) {
        let deadline: u64 = env.storage().persistent().get(&DataKey::Deadline)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));
        if env.ledger().timestamp() <= deadline {
            panic_with_error!(&env, CrowdfundError::CampaignNotEnded);
        }

        let goal: i128 = env.storage().persistent().get(&DataKey::Goal)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));
        let raised: i128 = env.storage().persistent().get(&DataKey::Raised).unwrap_or(0);
        if raised >= goal { panic_with_error!(&env, CrowdfundError::GoalReached); }

        if env.storage().persistent().get(&DataKey::RefundProcessed).unwrap_or(false) {
            panic_with_error!(&env, CrowdfundError::RefundAlreadyProcessed);
        }
        env.storage().persistent().set(&DataKey::RefundProcessed, &true);

        let token_addr: Address = env.storage().persistent().get(&DataKey::Token)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));
        let token_client = token::TokenClient::new(&env, &token_addr);
        let contract_addr = env.current_contract_address();

        let contributors: Vec<Address> = env.storage().persistent()
            .get(&DataKey::Contributors).unwrap_or_else(|| Vec::new(&env));
        let count = contributors.len();
        let mut total_refunded: i128 = 0;

        for contributor in contributors.iter() {
            let pledge: i128 = env.storage().persistent()
                .get(&DataKey::Pledge(contributor.clone())).unwrap_or(0);
            if pledge > 0 {
                env.storage().persistent().set(&DataKey::Pledge(contributor.clone()), &0_i128);
                token_client.transfer(&contract_addr, &contributor, &pledge);
                total_refunded = total_refunded.saturating_add(pledge);
            }
        }

        BatchRefundProcessedEvent { total_refunded, contributor_count: count }.publish(&env);
    }

    /// Add ordered stretch goal milestones (must be in ascending order, all > goal) (#306).
    /// Only the organizer can set these; must be called before deadline.
    pub fn set_stretch_goals(env: Env, milestones: Vec<i128>) {
        let organizer: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Organizer)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        organizer.require_auth();

        // Validate ascending order and all positive.
        let mut prev = 0_i128;
        for m in milestones.iter() {
            if m <= prev {
                panic_with_error!(&env, CrowdfundError::InvalidGoal);
            }
            prev = m;
        }

        env.storage()
            .persistent()
            .set(&DataKey::StretchGoals, &milestones);
    }

    /// Mark a backer's reward as fulfilled. Only callable by the organizer.
    /// Panics if called a second time for the same backer.
    pub fn fulfill_reward(env: Env, backer: Address) {
        let organizer: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Organizer)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        organizer.require_auth();

        if env
            .storage()
            .persistent()
            .get(&DataKey::RewardFulfilled(backer.clone()))
            .unwrap_or(false)
        {
            panic_with_error!(&env, CrowdfundError::AlreadyFulfilled);
        }

        env.storage()
            .persistent()
            .set(&DataKey::RewardFulfilled(backer.clone()), &true);

        RewardFulfilledEvent { backer }.publish(&env);
    }

    /// Returns `true` if the organizer has marked the backer's reward as fulfilled.
    pub fn is_fulfilled(env: Env, backer: Address) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::RewardFulfilled(backer))
            .unwrap_or(false)
    }

    /// Set reward tiers for the campaign. Tiers must be in ascending order by
    /// `min_pledge`. Only callable by the organizer.
    pub fn set_reward_tiers(env: Env, tiers: Vec<RewardTier>) {
        let organizer: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Organizer)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        organizer.require_auth();

        let mut prev = 0_i128;
        for tier in tiers.iter() {
            if tier.min_pledge <= prev {
                panic_with_error!(&env, CrowdfundError::InvalidGoal);
            }
            prev = tier.min_pledge;
        }

        env.storage().persistent().set(&DataKey::RewardTiers, &tiers);
    }

    /// Select a reward tier. The contributor's total pledge must meet the tier's
    /// `min_pledge`. Replaces any previously selected tier.
    pub fn select_reward_tier(env: Env, contributor: Address, tier_index: u32) {
        contributor.require_auth();

        let tiers: Vec<RewardTier> = env
            .storage()
            .persistent()
            .get(&DataKey::RewardTiers)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        let tier = tiers
            .get(tier_index)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::InvalidTier));

        let pledge: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Pledge(contributor.clone()))
            .unwrap_or(0);

        if pledge < tier.min_pledge {
            panic_with_error!(&env, CrowdfundError::PledgeBelowTierMinimum);
        }

        env.storage()
            .persistent()
            .set(&DataKey::SelectedTier(contributor.clone()), &tier_index);

        RewardTierSelectedEvent { contributor, tier_index }.publish(&env);
    }

    /// Returns the tier index selected by a contributor, or `None` if none selected.
    pub fn get_selected_tier(env: Env, contributor: Address) -> Option<u32> {
        env.storage()
            .persistent()
            .get(&DataKey::SelectedTier(contributor))
    }

    /// Define milestone percentages in basis points (1 bp = 0.01 %).
    /// Must sum to exactly 10 000, each entry > 0. Organizer-only.
    /// Locks the campaign into milestone mode; `execute_campaign` will be blocked.
    pub fn set_milestones(env: Env, percentages: Vec<u32>) {
        let organizer: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Organizer)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        organizer.require_auth();

        let mut sum: u32 = 0;
        for p in percentages.iter() {
            if p == 0 {
                panic_with_error!(&env, CrowdfundError::InvalidMilestonePercentages);
            }
            sum = sum.saturating_add(p);
        }
        if sum != 10_000 || percentages.len() == 0 {
            panic_with_error!(&env, CrowdfundError::InvalidMilestonePercentages);
        }

        env.storage()
            .persistent()
            .set(&DataKey::MilestonePercentages, &percentages);
    }

    /// Signal that a specific milestone is ready for release. Organizer-only.
    pub fn unlock_milestone(env: Env, index: u32) {
        let organizer: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Organizer)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        organizer.require_auth();

        let percentages: Vec<u32> = env
            .storage()
            .persistent()
            .get(&DataKey::MilestonePercentages)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::MilestonesNotSet));

        if index >= percentages.len() {
            panic_with_error!(&env, CrowdfundError::InvalidMilestonePercentages);
        }

        env.storage()
            .persistent()
            .set(&DataKey::MilestoneUnlocked(index), &true);

        MilestoneUnlockedEvent { index }.publish(&env);
    }

    /// Cast a backer governance vote for releasing a specific milestone.
    /// Vote weight is the backer's recorded pledge amount.
    pub fn vote_milestone(env: Env, voter: Address, index: u32, approve: bool) {
        voter.require_auth();

        let percentages: Vec<u32> = env
            .storage()
            .persistent()
            .get(&DataKey::MilestonePercentages)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::MilestonesNotSet));

        if index >= percentages.len() {
            panic_with_error!(&env, CrowdfundError::InvalidMilestonePercentages);
        }

        let weight: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Pledge(voter.clone()))
            .unwrap_or(0);

        if weight <= 0 {
            panic_with_error!(&env, CrowdfundError::NotBacker);
        }

        let vote_key = DataKey::MilestoneVote(index, voter.clone());
        if env.storage().persistent().has(&vote_key) {
            panic_with_error!(&env, CrowdfundError::MilestoneVoteAlreadyCast);
        }

        let tally_key = if approve {
            DataKey::MilestoneApprovalWeight(index)
        } else {
            DataKey::MilestoneRejectionWeight(index)
        };

        let current: i128 = env.storage().persistent().get(&tally_key).unwrap_or(0);
        env.storage()
            .persistent()
            .set(&tally_key, &current.saturating_add(weight));
        env.storage().persistent().set(&vote_key, &approve);

        MilestoneVoteCastEvent { index, voter, approve, weight }.publish(&env);
    }

    /// Release the proportional funds for an unlocked, unreleased milestone to the organizer.
    /// Can only be called after the campaign deadline and goal is met.
    pub fn release_milestone(env: Env, index: u32) {
        let organizer: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Organizer)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        organizer.require_auth();

        let deadline: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::Deadline)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        if env.ledger().timestamp() <= deadline {
            panic_with_error!(&env, CrowdfundError::CampaignNotEnded);
        }

        let goal: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Goal)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        let raised: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Raised)
            .unwrap_or(0);

        if raised < goal {
            panic_with_error!(&env, CrowdfundError::GoalNotReached);
        }

        let percentages: Vec<u32> = env
            .storage()
            .persistent()
            .get(&DataKey::MilestonePercentages)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::MilestonesNotSet));

        if index >= percentages.len() {
            panic_with_error!(&env, CrowdfundError::InvalidMilestonePercentages);
        }

        let unlocked: bool = env
            .storage()
            .persistent()
            .get(&DataKey::MilestoneUnlocked(index))
            .unwrap_or(false);

        if !unlocked {
            panic_with_error!(&env, CrowdfundError::MilestoneNotUnlocked);
        }

        let released: bool = env
            .storage()
            .persistent()
            .get(&DataKey::MilestoneReleased(index))
            .unwrap_or(false);

        if released {
            panic_with_error!(&env, CrowdfundError::MilestoneAlreadyReleased);
        }

        let approval_weight: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::MilestoneApprovalWeight(index))
            .unwrap_or(0);

        if approval_weight <= raised / 2 {
            panic_with_error!(&env, CrowdfundError::MilestoneNotApproved);
        }

        let bps = percentages.get(index).unwrap() as i128;
        let amount = raised * bps / 10_000;

        env.storage()
            .persistent()
            .set(&DataKey::MilestoneReleased(index), &true);

        let token_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Token)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));

        let contract_addr = env.current_contract_address();
        token::TokenClient::new(&env, &token_addr)
            .transfer(&contract_addr, &organizer, &amount);

        MilestoneReleasedEvent { index, amount }.publish(&env);
    }

    /// Returns the pledge amount recorded for a given contributor.
    pub fn pledge_of(env: Env, contributor: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Pledge(contributor))
            .unwrap_or(0)
    }

    // ── Read-only accessors ───────────────────────────────────────────────────

    pub fn goal(env: Env) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Goal)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized))
    }

    pub fn deadline(env: Env) -> u64 {
        env.storage()
            .persistent()
            .get(&DataKey::Deadline)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized))
    }

    pub fn raised(env: Env) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Raised)
            .unwrap_or(0)
    }

    pub fn organizer(env: Env) -> Address {
        env.storage()
            .persistent()
            .get(&DataKey::Organizer)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized))
    }

    pub fn is_executed(env: Env) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::Executed)
            .unwrap_or(false)
    }

    /// Returns `true` when the raised amount has reached or exceeded the goal.
    pub fn goal_reached(env: Env) -> bool {
        let goal: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Goal)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));
        let raised: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Raised)
            .unwrap_or(0);
        raised >= goal
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    /// Emit a `stretch / reached` event for each milestone crossed by `new_raised`
    /// that has not already been triggered.
    fn track_contributor(env: &Env, contributor: Address) {
        let mut contributors: Vec<Address> = env.storage().persistent()
            .get(&DataKey::Contributors).unwrap_or_else(|| Vec::new(env));
        for c in contributors.iter() {
            if c == contributor { return; }
        }
        contributors.push_back(contributor);
        env.storage().persistent().set(&DataKey::Contributors, &contributors);
    }

    fn check_stretch_goals(env: &Env, new_raised: i128) {
        let milestones: Vec<i128> = env
            .storage()
            .persistent()
            .get(&DataKey::StretchGoals)
            .unwrap_or_else(|| vec![env]);

        for (idx, threshold) in milestones.iter().enumerate() {
            let idx_u32 = idx as u32;
            let already: bool = env
                .storage()
                .persistent()
                .get(&DataKey::StretchTriggered(idx_u32))
                .unwrap_or(false);

            if !already && new_raised >= threshold {
                env.storage()
                    .persistent()
                    .set(&DataKey::StretchTriggered(idx_u32), &true);
                StretchGoalReachedEvent {
                    milestone_index: idx_u32,
                    threshold,
                }
                .publish(env);
            }
        }
    }

    fn apply_pledge_with_matching(env: &Env, contributor: Address, amount: i128) -> i128 {
        let matching_pool: i128 = env.storage().persistent().get(&DataKey::MatchingPool).unwrap_or(0);
        let matched_amount = if matching_pool >= amount {
            amount
        } else {
            matching_pool
        };
        if matched_amount > 0 {
            env.storage().persistent().set(
                &DataKey::MatchingPool,
                &matching_pool.saturating_sub(matched_amount),
            );
            MatchAppliedEvent {
                contributor: contributor.clone(),
                matched_amount,
            }
            .publish(env);
        }

        let effective_amount = amount.saturating_add(matched_amount);
        let raised: i128 = env.storage().persistent().get(&DataKey::Raised).unwrap_or(0);
        let new_raised = raised.saturating_add(effective_amount);
        env.storage().persistent().set(&DataKey::Raised, &new_raised);

        let prev_pledge: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Pledge(contributor.clone()))
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&DataKey::Pledge(contributor), &prev_pledge.saturating_add(effective_amount));

        new_raised
    }
}
