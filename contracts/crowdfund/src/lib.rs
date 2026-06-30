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

pub struct PledgeReceivedEvent {
    pub contributor: Address,
    pub amount: i128,
}

#[contractevent]
pub struct BatchRefundProcessedEvent {
    pub total_refunded: i128,
    pub contributor_count: u32,
}

// ── Financial penalty events (#360) ──────────────────────────────────────────
#[contractevent]
pub struct PenaltyConfiguredEvent {
    pub bps: u32,
}

#[contractevent]
pub struct MaliciousReportFiledEvent {
    pub reporter: Address,
    pub reason: String,
    pub vote_deadline: u64,
}

#[contractevent]
pub struct MaliceVoteCastEvent {
    pub voter: Address,
    pub approve: bool,
    pub weight: i128,
}

#[contractevent]
pub struct MaliceReportResolvedEvent {
    pub approved: bool,
    pub approval_weight: i128,
    pub rejection_weight: i128,
    pub snapshot_raised: i128,
}

#[contractevent]
pub struct PenaltySlashedEvent {
    pub amount: i128,
    pub source: Address,
    pub pool_balance: i128,
}

#[contractevent]
pub struct PenaltyRefundClaimedEvent {
    pub backer: Address,
    pub amount: i128,
    pub remaining_pool: i128,
}

#[contractevent]
pub struct PenaltySweptEvent {
    pub recipient: Address,
    pub amount: i128,
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
    // ── Financial penalty for malicious campaigns (#360) ─────────────────────
    /// Penalty in basis points (max 5_000 = 50%). Locked after first pledge.
    PenaltyBps,
    /// True once `PenaltyBps` is locked and cannot be modified.
    PenaltyLocked,
    /// Unix timestamp when the active malice voting window opens.
    MaliceVoteStart,
    /// Unix timestamp when the active malice voting window closes.
    MaliceVoteDeadline,
    /// Address that filed the active malice report.
    MaliceReporter,
    /// Off-chain evidence / reason string supplied by the reporter.
    MaliceReason,
    /// Cumulative pledge-weighted approvals for the active malice report.
    PenaltyApprovalWeight,
    /// Cumulative pledge-weighted rejections for the active malice report.
    PenaltyRejectionWeight,
    /// Tracks whether a backer has voted on the active malice report.
    PenaltyVote(Address),
    /// True when the active report has been resolved.
    PenaltyResolved,
    /// Outcome of the resolution: true if penalty was approved by voters.
    PenaltyApproved,
    /// Current balance of the on-chain penalty pool (slashed tokens).
    PenaltyPool,
    /// `Raised` snapshot taken at the moment the penalty was approved.
    PenaltySnapshotRaised,
    /// Sum of all backer penalty refunds already distributed.
    PenaltyTotalClaimed,
    /// Amount of penalty pool already claimed by a specific backer.
    BackerPenaltyClaimed(Address),
    /// Unix timestamp at which unclaimed penalty becomes sweepable.
    PenaltySweepUnlock,
    /// Address authorized to receive unclaimed penalty on sweep.
    PenaltyRecipient,
}

#[contract]
pub struct CrowdfundContract;

#[contractimpl]
impl CrowdfundContract {
    const MAX_COMMENT_BYTES: u32 = 280;
    /// Maximum self-imposed penalty (50% of payout) for malicious campaigns.
    const MAX_PENALTY_BPS: u32 = 5_000;
    /// Length of the malice voting window (7 days).
    const PENALTY_VOTE_WINDOW: u64 = 604_800;
    /// Time after resolution that unclaimed penalty may be swept (≈6 months).
    const PENALTY_SWEEP_AFTER_SECS: u64 = 15_778_800;
    /// Reporter must hold at least this fraction of raised (basis points) to
    /// open a voting window — anti-griefing floor (default 1% = 100 bps).
    const MIN_REPORTER_PLEDGE_BPS: u32 = 100;

    // ── Penalty helpers (#360) ─────────────────────────────────────────────────
    /// Returns true if a malice report is currently in flight.
    fn malice_report_active(env: &Env) -> bool {
        env.storage().persistent().has(&DataKey::MaliceVoteStart)
            && !env
                .storage()
                .persistent()
                .get(&DataKey::PenaltyResolved)
                .unwrap_or(false)
    }
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

    /// Withdraw funds to the organizer after deadline if goal was met (#303).
    /// If a financial penalty was approved by backer vote and a non-zero
    /// `PenaltyBps` is set, the slashed portion is detained in the on-chain
    /// penalty pool instead of being paid out.
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

        // Block withdrawal while a malice proposal is still being voted on so
        // the organizer cannot front-run the penalty (#360).
        if Self::malice_report_active(&env) {
            let deadline_ts: u64 = env
                .storage()
                .persistent()
                .get(&DataKey::MaliceVoteDeadline)
                .unwrap_or(0);
            if env.ledger().timestamp() <= deadline_ts {
                panic_with_error!(&env, CrowdfundError::MaliceVoteWindowActive);
            }
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

        // Compute and lock away penalty amount if approved (#360).
        let (payout, slashed) = Self::compute_and_lock_penalty(&env, raised);
        if slashed > 0 {
            PenaltySlashedEvent {
                amount: slashed,
                source: organizer.clone(),
                pool_balance: env
                    .storage()
                    .persistent()
                    .get(&DataKey::PenaltyPool)
                    .unwrap_or(0),
            }
            .publish(&env);
        }

        token::TokenClient::new(&env, &token_addr)
            .transfer(&contract_addr, &organizer, &payout);

        CampaignExecutedEvent { amount: payout }.publish(&env);
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
    /// If a financial penalty was approved by backer vote, the matching
    /// slice is detained in the on-chain penalty pool (#360).
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

        // Block release while a malice proposal is still being voted on (#360).
        if Self::malice_report_active(&env) {
            let vote_dl: u64 = env
                .storage()
                .persistent()
                .get(&DataKey::MaliceVoteDeadline)
                .unwrap_or(0);
            if env.ledger().timestamp() <= vote_dl {
                panic_with_error!(&env, CrowdfundError::MaliceVoteWindowActive);
            }
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

        // Apply milestone-scoped penalty to the released amount (#360).
        let (payout, slashed) = Self::compute_and_lock_penalty(&env, amount);
        if slashed > 0 {
            PenaltySlashedEvent {
                amount: slashed,
                source: organizer.clone(),
                pool_balance: env
                    .storage()
                    .persistent()
                    .get(&DataKey::PenaltyPool)
                    .unwrap_or(0),
            }
            .publish(&env);
        }

        token::TokenClient::new(&env, &token_addr)
            .transfer(&contract_addr, &organizer, &payout);

        MilestoneReleasedEvent {
            index,
            amount: payout,
        }
        .publish(&env);
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

    // ── Financial penalties for malicious campaigns (#360) ─────────────────────

    /// Set the self-imposed penalty basis points (≤ `MAX_PENALTY_BPS`) that
    /// the organizer commits to if backers vote the campaign malicious.
    /// Must be called **before** the first pledge lands (penalty is locked
    /// permanently once any contribution arrives) and **before** a malice
    /// proposal is filed. Pure organizer signal — values can be repeatedly
    /// re-tuned while still unlocked.
    pub fn set_penalty_bps(env: Env, bps: u32) {
        let organizer: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Organizer)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));
        organizer.require_auth();
        if env
            .storage()
            .persistent()
            .get(&DataKey::PenaltyLocked)
            .unwrap_or(false)
        {
            panic_with_error!(&env, CrowdfundError::PenaltyLocked);
        }
        if bps > Self::MAX_PENALTY_BPS {
            panic_with_error!(&env, CrowdfundError::PenaltyBpsInvalid);
        }
        if Self::malice_report_active(&env) {
            panic_with_error!(&env, CrowdfundError::MaliceReportActive);
        }
        env.storage().persistent().set(&DataKey::PenaltyBps, &bps);
        PenaltyConfiguredEvent { bps }.publish(&env);
    }

    /// File a malicious-campaign report. Must be invoked by a backer whose
    /// pledge is at least `MIN_REPORTER_PLEDGE_BPS` of the currently raised
    /// amount (anti-griefing floor). Organizers may not file against
    /// themselves. Opens the `PENALTY_VOTE_WINDOW`-long voting session.
    pub fn report_malicious(env: Env, reporter: Address, reason: String) {
        reporter.require_auth();
        if Self::malice_report_active(&env) {
            panic_with_error!(&env, CrowdfundError::MaliceReportActive);
        }
        let organizer: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Organizer)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));
        if reporter == organizer {
            panic_with_error!(&env, CrowdfundError::NotOrganizer);
        }
        let pledge: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Pledge(reporter.clone()))
            .unwrap_or(0);
        if pledge <= 0 {
            panic_with_error!(&env, CrowdfundError::NoPledge);
        }
        let raised: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Raised)
            .unwrap_or(0);
        let required = raised.saturating_mul(Self::MIN_REPORTER_PLEDGE_BPS as i128) / 10_000;
        if pledge < required {
            panic_with_error!(&env, CrowdfundError::InsufficientReporterStake);
        }
        let now = env.ledger().timestamp();
        env.storage()
            .persistent()
            .set(&DataKey::MaliceVoteStart, &now);
        env.storage().persistent().set(
            &DataKey::MaliceVoteDeadline,
            &now.saturating_add(Self::PENALTY_VOTE_WINDOW),
        );
        env.storage()
            .persistent()
            .set(&DataKey::MaliceReporter, &reporter.clone());
        env.storage()
            .persistent()
            .set(&DataKey::MaliceReason, &reason.clone());
        env.storage()
            .persistent()
            .set(&DataKey::PenaltyApprovalWeight, &0_i128);
        env.storage()
            .persistent()
            .set(&DataKey::PenaltyRejectionWeight, &0_i128);
        env.storage()
            .persistent()
            .set(&DataKey::PenaltyResolved, &false);
        env.storage()
            .persistent()
            .set(&DataKey::PenaltyApproved, &false);
        MaliciousReportFiledEvent {
            reporter,
            reason,
            vote_deadline: now.saturating_add(Self::PENALTY_VOTE_WINDOW),
        }
        .publish(&env);
    }

    /// Cast a backer's weighted vote on the active malice report. Vote window
    /// must still be open; one vote per backer (no coercion).
    pub fn vote_on_malice(env: Env, voter: Address, approve: bool) {
        voter.require_auth();
        if !Self::malice_report_active(&env) {
            panic_with_error!(&env, CrowdfundError::NoMaliceReport);
        }
        let deadline: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::MaliceVoteDeadline)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NoMaliceReport));
        if env.ledger().timestamp() > deadline {
            panic_with_error!(&env, CrowdfundError::MaliceVoteWindowExpired);
        }
        let vote_key = DataKey::PenaltyVote(voter.clone());
        if env.storage().persistent().has(&vote_key) {
            panic_with_error!(&env, CrowdfundError::PenaltyVoteAlreadyCast);
        }
        let weight: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Pledge(voter.clone()))
            .unwrap_or(0);
        if weight <= 0 {
            panic_with_error!(&env, CrowdfundError::NotBacker);
        }
        let tally_key = if approve {
            DataKey::PenaltyApprovalWeight
        } else {
            DataKey::PenaltyRejectionWeight
        };
        let current: i128 = env.storage().persistent().get(&tally_key).unwrap_or(0);
        env.storage()
            .persistent()
            .set(&tally_key, &current.saturating_add(weight));
        env.storage().persistent().set(&vote_key, &approve);
        MaliceVoteCastEvent {
            voter,
            approve,
            weight,
        }
        .publish(&env);
    }

    /// Finalize the active malice report. Callable by anyone once the vote
    /// window has closed. Approved iff approval weight is a strict majority
    /// of `Raised` at the moment of resolution. Approval locks-in the penalty
    /// to be applied on the next `execute_campaign` / `release_milestone`.
    pub fn resolve_malice_report(env: Env) {
        if !Self::malice_report_active(&env) {
            panic_with_error!(&env, CrowdfundError::NoMaliceReport);
        }
        let deadline: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::MaliceVoteDeadline)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NoMaliceReport));
        if env.ledger().timestamp() <= deadline {
            panic_with_error!(&env, CrowdfundError::MaliceVoteWindowActive);
        }
        let approval: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::PenaltyApprovalWeight)
            .unwrap_or(0);
        let rejection: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::PenaltyRejectionWeight)
            .unwrap_or(0);
        let raised: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Raised)
            .unwrap_or(0);
        let approved = approval > (raised / 2);

        env.storage()
            .persistent()
            .set(&DataKey::PenaltyResolved, &true);
        env.storage()
            .persistent()
            .set(&DataKey::PenaltyApproved, &approved);
        env.storage()
            .persistent()
            .set(&DataKey::PenaltySnapshotRaised, &raised);
        env.storage().persistent().set(
            &DataKey::PenaltySweepUnlock,
            &env
                .ledger()
                .timestamp()
                .saturating_add(Self::PENALTY_SWEEP_AFTER_SECS),
        );

        MaliceReportResolvedEvent {
            approved,
            approval_weight: approval,
            rejection_weight: rejection,
            snapshot_raised: raised,
        }
        .publish(&env);
    }

    /// Pro-rata claim from the penalty pool. Each backer is entitled to
    /// `(pledge_at_resolution / PenaltySnapshotRaised) * PoolBalance`, minus
    /// what they have already claimed. Safe against reentrancy (state is
    /// mutated before the cross-contract token transfer).
    pub fn claim_penalty_refund(env: Env, backer: Address) {
        backer.require_auth();
        let approved: bool = env
            .storage()
            .persistent()
            .get(&DataKey::PenaltyApproved)
            .unwrap_or(false);
        if !approved {
            panic_with_error!(&env, CrowdfundError::PenaltyNotApproved);
        }
        let pledge: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Pledge(backer.clone()))
            .unwrap_or(0);
        if pledge <= 0 {
            panic_with_error!(&env, CrowdfundError::NotBacker);
        }
        let pool: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::PenaltyPool)
            .unwrap_or(0);
        let snapshot: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::PenaltySnapshotRaised)
            .unwrap_or(0);
        if pool <= 0 || snapshot <= 0 {
            panic_with_error!(&env, CrowdfundError::NoPenaltyShareAvailable);
        }
        let total_share = pledge.saturating_mul(pool) / snapshot;
        let already: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::BackerPenaltyClaimed(backer.clone()))
            .unwrap_or(0);
        let claimable = total_share.saturating_sub(already);
        if claimable <= 0 {
            panic_with_error!(&env, CrowdfundError::NoPenaltyShareAvailable);
        }
        // Checks-Effects-Interactions: write storage before token transfer.
        env.storage().persistent().set(
            &DataKey::BackerPenaltyClaimed(backer.clone()),
            &already.saturating_add(claimable),
        );
        let total_claimed: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::PenaltyTotalClaimed)
            .unwrap_or(0);
        env.storage().persistent().set(
            &DataKey::PenaltyTotalClaimed,
            &total_claimed.saturating_add(claimable),
        );
        let token_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Token)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));
        let contract_addr = env.current_contract_address();
        token::TokenClient::new(&env, &token_addr)
            .transfer(&contract_addr, &backer, &claimable);
        let remaining = pool.saturating_sub(claimable);
        PenaltyRefundClaimedEvent {
            backer,
            amount: claimable,
            remaining_pool: remaining,
        }
        .publish(&env);
    }

    /// Sweep unclaimed penalty balance to `recipient` once the configured
    /// sweep window has elapsed. Anyone may trigger this — useful for
    /// protocol governance / treasury consolidation.
    pub fn sweep_unclaimed_penalty(env: Env, recipient: Address) {
        let approved: bool = env
            .storage()
            .persistent()
            .get(&DataKey::PenaltyApproved)
            .unwrap_or(false);
        if !approved {
            panic_with_error!(&env, CrowdfundError::PenaltyNotApproved);
        }
        let unlock: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::PenaltySweepUnlock)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::PenaltySweepTooEarly));
        if env.ledger().timestamp() < unlock {
            panic_with_error!(&env, CrowdfundError::PenaltySweepTooEarly);
        }
        let pool: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::PenaltyPool)
            .unwrap_or(0);
        let total_claimed: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::PenaltyTotalClaimed)
            .unwrap_or(0);
        let sweepable = pool.saturating_sub(total_claimed);
        if sweepable <= 0 {
            return;
        }
        env.storage()
            .persistent()
            .set(&DataKey::PenaltyPool, &total_claimed);
        env.storage()
            .persistent()
            .set(&DataKey::PenaltyRecipient, &recipient);
        let token_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::Token)
            .unwrap_or_else(|| panic_with_error!(&env, CrowdfundError::NotInitialized));
        let contract_addr = env.current_contract_address();
        token::TokenClient::new(&env, &token_addr)
            .transfer(&contract_addr, &recipient, &sweepable);
        PenaltySweptEvent {
            recipient,
            amount: sweepable,
        }
        .publish(&env);
    }

    // ── Penalty view accessors (#360) ─────────────────────────────────────────

    pub fn get_penalty_bps(env: Env) -> u32 {
        env.storage().persistent().get(&DataKey::PenaltyBps).unwrap_or(0)
    }

    pub fn penalty_locked(env: Env) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::PenaltyLocked)
            .unwrap_or(false)
    }

    pub fn malice_vote_start(env: Env) -> Option<u64> {
        env.storage().persistent().get(&DataKey::MaliceVoteStart)
    }

    pub fn malice_vote_deadline(env: Env) -> Option<u64> {
        env.storage().persistent().get(&DataKey::MaliceVoteDeadline)
    }

    pub fn malice_reporter(env: Env) -> Option<Address> {
        env.storage().persistent().get(&DataKey::MaliceReporter)
    }

    pub fn malice_reason(env: Env) -> Option<String> {
        env.storage().persistent().get(&DataKey::MaliceReason)
    }

    pub fn penalty_vote_counts(env: Env) -> (i128, i128) {
        (
            env.storage()
                .persistent()
                .get(&DataKey::PenaltyApprovalWeight)
                .unwrap_or(0),
            env.storage()
                .persistent()
                .get(&DataKey::PenaltyRejectionWeight)
                .unwrap_or(0),
        )
    }

    pub fn is_malice_report_active(env: Env) -> bool {
        Self::malice_report_active(&env)
    }

    pub fn is_penalty_approved(env: Env) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::PenaltyApproved)
            .unwrap_or(false)
    }

    pub fn penalty_pool_balance(env: Env) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::PenaltyPool)
            .unwrap_or(0)
    }

    pub fn penalty_snapshot_raised(env: Env) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::PenaltySnapshotRaised)
            .unwrap_or(0)
    }

    pub fn backer_penalty_claimed(env: Env, backer: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::BackerPenaltyClaimed(backer))
            .unwrap_or(0)
    }

    pub fn penalty_sweep_unlock(env: Env) -> Option<u64> {
        env.storage().persistent().get(&DataKey::PenaltySweepUnlock)
    }

    pub fn max_penalty_bps() -> u32 {
        Self::MAX_PENALTY_BPS
    }

    pub fn penalty_vote_window() -> u64 {
        Self::PENALTY_VOTE_WINDOW
    }

    pub fn penalty_sweep_after() -> u64 {
        Self::PENALTY_SWEEP_AFTER_SECS
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

        // Lock the self-imposed penalty rate after the first pledge lands so
        // backers can rely on the trust signal (#360).
        if !env
            .storage()
            .persistent()
            .get(&DataKey::PenaltyLocked)
            .unwrap_or(false)
        {
            env.storage()
                .persistent()
                .set(&DataKey::PenaltyLocked, &true);
        }

        new_raised
    }

    /// Compute the penalty slice for `amount`, credit the pool, and return
    /// `(payout_to_organizer, slashed_to_pool)`. Identity when no penalty is
    /// configured/approved (#360).
    fn compute_and_lock_penalty(env: &Env, amount: i128) -> (i128, i128) {
        let approved: bool = env
            .storage()
            .persistent()
            .get(&DataKey::PenaltyApproved)
            .unwrap_or(false);
        if !approved {
            return (amount, 0);
        }
        let bps: u32 = env
            .storage()
            .persistent()
            .get(&DataKey::PenaltyBps)
            .unwrap_or(0);
        if bps == 0 || amount <= 0 {
            return (amount, 0);
        }
        let slashed = amount.saturating_mul(bps as i128) / 10_000;
        let payout = amount.saturating_sub(slashed);
        let pool: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::PenaltyPool)
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&DataKey::PenaltyPool, &pool.saturating_add(slashed));
        (payout, slashed)
    }
}
