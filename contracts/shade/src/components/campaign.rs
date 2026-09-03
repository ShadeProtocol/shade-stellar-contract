use crate::components::{core, reentrancy};
use crate::errors::{CampaignError, ContractError};
use crate::events;
use crate::types::{CampaignAffiliate, CampaignKey, CampaignParticipant, FeeCampaign};
use soroban_sdk::{panic_with_error, Address, Env, String, Vec};

/// Validation bounds for free-form user strings.
const MAX_NAME_LEN: u32 = 128;
const MAX_DESCRIPTION_LEN: u32 = 512;

// ── Helpers ───────────────────────────────────────────────────────────────────

fn get_campaign_count(env: &Env) -> u64 {
    env.storage()
        .persistent()
        .get(&DataKey::StakeableCampaignCount)
        .unwrap_or(0)
}

fn get_participant(env: &Env, campaign_id: u64, participant: &Address) -> CampaignParticipant {
    env.storage()
        .persistent()
        .get(&DataKey::CampaignParticipant(campaign_id, participant.clone()))
        .unwrap_or(CampaignParticipant {
            campaign_id,
            participant: participant.clone(),
            contributed: 0,
            staked: 0,
            slashed: 0,
            commissions_paid: 0,
            score: 0,
        })
}

fn store_participant(env: &Env, campaign_id: u64, participant: &CampaignParticipant) {
    let participant_ids = env
        .storage()
        .persistent()
        .get(&DataKey::CampaignParticipants(campaign_id))
        .unwrap_or_else(|| Vec::new(env));

    let mut exists = false;
    for existing in participant_ids.iter() {
        if existing == participant.participant {
            exists = true;
            break;
        }
    }

    if !exists {
        let mut updated_ids = Vec::new(env);
        for existing in participant_ids.iter() {
            updated_ids.push_back(existing);
        }
        updated_ids.push_back(participant.participant.clone());
        env.storage()
            .persistent()
            .set(&DataKey::CampaignParticipants(campaign_id), &updated_ids);
    }

    env.storage().persistent().set(
        &DataKey::CampaignParticipant(campaign_id, participant.participant.clone()),
        participant,
    );
}

// ── Campaign management (stakeable campaigns with financial penalties) ────────

/// Create a campaign with staking requirements. Only registered merchants may call.
/// Matches the ShadeTrait signature: (env, caller, name, charity, fee_waiver_bps, discount_bps, stake_required) -> u64
pub fn create_campaign(
    env: &Env,
    caller: &Address,
    name: &String,
    _charity: bool,
    fee_waiver_bps: u32,
    discount_bps: u32,
    stake_required: i128,
) -> u64 {
    reentrancy::enter(env);
    caller.require_auth();

    if !merchant_component::is_merchant(env, caller) {
        panic_with_error!(env, ContractError::MerchantNotFound);
    }
    let merchant_id = merchant_component::get_merchant_id(env, caller);
    if !merchant_component::is_merchant_active(env, merchant_id) {
        panic_with_error!(env, ContractError::MerchantNotActive);
    }

    if name.len() == 0 || name.len() > MAX_NAME_LEN {
        panic_with_error!(env, ContractError::InvalidDescription);
    }
    if fee_waiver_bps > 10_000 || discount_bps > 10_000 {
        panic_with_error!(env, ContractError::InvalidAmount);
    }
    if stake_required < 0 {
        panic_with_error!(env, ContractError::InvalidAmount);
    }

    let campaign_id = get_campaign_count(env) + 1;
    let desc = String::from_str(env, "");
    let campaign = Campaign {
    let campaign = FeeCampaign {
        id: campaign_id,
        merchant_id,
        merchant: caller.clone(),
        title: name.clone(),
        description: desc,
        category_id: 0,
        tags: Vec::new(env),
        goal_amount: 0,
        // Placeholder token; campaigns created via this minimal factory should
        // have their token set before accepting contributions.
        token: caller.clone(),
        deadline: u64::MAX,
        raised_amount: 0,
        active: true,
        created_at: env.ledger().timestamp(),
        status: CampaignStatus::Active,
        total_slashed: 0,
        penalty_count: 0,
        fee_waiver_bps,
        discount_bps,
        stake_required,
    };

    env.storage()
        .persistent()
        .set(&DataKey::StakeableCampaign(campaign_id), &campaign);
    env.storage()
        .persistent()
        .set(&DataKey::StakeableCampaignCount, &campaign_id);
    env.storage()
        .persistent()
        .set(&CampaignKey::FeeCampaign(campaign_id), &campaign);
    env.storage()
        .persistent()
        .set(&CampaignKey::FeeCampaignCount, &campaign_id);
    env.storage().persistent().set(
        &CampaignKey::CampaignParticipants(campaign_id),
        &Vec::<Address>::new(env),
    );

    events::publish_fee_campaign_created_event(
        env,
        campaign_id,
        caller.clone(),
        merchant_id,
        name.clone(),
        String::from_str(env, ""),
        0,
        Vec::new(env),
        0,
        caller.clone(),
        u64::MAX,
        env.ledger().timestamp(),
    );

    reentrancy::exit(env);
    campaign_id
}

/// Configure fee policy for an existing campaign. Only owner or admin.
pub fn configure_campaign_fee_policy(
    env: &Env,
    caller: &Address,
    campaign_id: u64,
    fee_waiver_bps: u32,
    discount_bps: u32,
) {
    reentrancy::enter(env);
    caller.require_auth();

    if fee_waiver_bps > 10_000 || discount_bps > 10_000 {
        panic_with_error!(env, ContractError::InvalidAmount);
    }

    let mut campaign = get_campaign(env, campaign_id);
    let admin = core::get_admin(env);
    if *caller != campaign.merchant && *caller != admin {
        panic_with_error!(env, ContractError::NotAuthorized);
    }

    campaign.fee_waiver_bps = fee_waiver_bps;
    campaign.discount_bps = discount_bps;
    env.storage()
        .persistent()
        .set(&DataKey::StakeableCampaign(campaign_id), &campaign);
        .set(&CampaignKey::FeeCampaign(campaign_id), &campaign);

    reentrancy::exit(env);
}

/// Calculate the discounted amount for a campaign contribution.
pub fn calculate_campaign_discounted_amount(env: &Env, campaign_id: u64, amount: i128) -> i128 {
pub fn calculate_campaign_discount(env: &Env, campaign_id: u64, amount: i128) -> i128 {
    if amount <= 0 {
        return 0;
    }

    let campaign = get_campaign(env, campaign_id);
    let waiver = (amount * i128::from(campaign.fee_waiver_bps)) / 10_000i128;
    let discount = (amount * i128::from(campaign.discount_bps)) / 10_000i128;
    amount - waiver - discount
}

/// Stake tokens on a campaign. Increases participant score.
pub fn stake_campaign(env: &Env, caller: &Address, campaign_id: u64, amount: i128) {
    reentrancy::enter(env);
    caller.require_auth();

    if amount <= 0 {
        panic_with_error!(env, ContractError::InvalidAmount);
    }

    let mut campaign = get_campaign(env, campaign_id);
    if !campaign.active {
        panic_with_error!(env, ContractError::CampaignInactive);
    }

    let mut participant = get_participant(env, campaign_id, caller);

    participant.staked += amount;
    participant.score += amount;
    campaign.raised_amount += amount;

    store_participant(env, campaign_id, &participant);
    env.storage()
        .persistent()
        .set(&DataKey::StakeableCampaign(campaign_id), &campaign);
        .set(&CampaignKey::FeeCampaign(campaign_id), &campaign);

    events::publish_campaign_staked_event(
        env,
        campaign_id,
        caller.clone(),
        amount,
        participant.staked,
        env.ledger().timestamp(),
    );

    reentrancy::exit(env);
}

/// Record a contribution to a campaign.
pub fn record_campaign_contribution(env: &Env, caller: &Address, campaign_id: u64, amount: i128) {
    reentrancy::enter(env);
    caller.require_auth();

    if amount <= 0 {
        panic_with_error!(env, ContractError::InvalidAmount);
    }

    let mut campaign = get_campaign(env, campaign_id);
    if !campaign.active {
        panic_with_error!(env, ContractError::CampaignInactive);
    }
    if env.ledger().timestamp() > campaign.deadline {
        panic_with_error!(env, ContractError::CampaignExpired);
    }

    let mut participant = get_participant(env, campaign_id, caller);

    campaign.raised_amount += amount;
    participant.contributed += amount;
    participant.score += amount;

    store_participant(env, campaign_id, &participant);
    env.storage()
        .persistent()
        .set(&DataKey::StakeableCampaign(campaign_id), &campaign);
        .set(&CampaignKey::FeeCampaign(campaign_id), &campaign);

    events::publish_campaign_contribution_event(
        env,
        campaign_id,
        caller.clone(),
        amount,
        campaign.raised_amount,
        campaign.goal_amount,
        env.ledger().timestamp(),
    );

    reentrancy::exit(env);
}

// ── Financial penalties for malicious campaigns (#360) ────────────────────────

/// Slash (penalize) a participant's staked amount. Only the campaign owner or
/// admin may call this. This is the core financial penalty mechanism.
pub fn slash_campaign_stake(
    env: &Env,
    caller: &Address,
    campaign_id: u64,
    participant_address: &Address,
    amount: i128,
) {
    reentrancy::enter(env);
    caller.require_auth();

    if amount <= 0 {
        panic_with_error!(env, ContractError::InvalidAmount);
    }

    let mut campaign = get_campaign(env, campaign_id);
    let admin = core::get_admin(env);
    if *caller != campaign.merchant && *caller != admin {
        panic_with_error!(env, ContractError::NotAuthorized);
    }

    let mut participant = get_participant(env, campaign_id, participant_address);
    if participant.staked < amount {
        panic_with_error!(env, ContractError::InvalidAmount);
    }

    participant.staked -= amount;
    participant.slashed += amount;
    participant.score -= amount;
    campaign.total_slashed += amount;

    store_participant(env, campaign_id, &participant);
    env.storage()
        .persistent()
        .set(&DataKey::StakeableCampaign(campaign_id), &campaign);
        .set(&CampaignKey::FeeCampaign(campaign_id), &campaign);

    events::publish_campaign_slashed_event(
        env,
        campaign_id,
        participant_address.clone(),
        amount,
        participant.staked,
        env.ledger().timestamp(),
    );

    reentrancy::exit(env);
}

// ── Affiliate system ──────────────────────────────────────────────────────────

/// Register an affiliate for a campaign.
pub fn register_affiliate(
    env: &Env,
    caller: &Address,
    campaign_id: u64,
    affiliate_address: &Address,
    commission_bps: u32,
) {
    reentrancy::enter(env);
    caller.require_auth();

    if commission_bps > 10_000 {
        panic_with_error!(env, ContractError::InvalidAmount);
    }

    let campaign = get_campaign(env, campaign_id);
    let admin = core::get_admin(env);
    if *caller != campaign.merchant && *caller != admin {
        panic_with_error!(env, ContractError::NotAuthorized);
    }

    let affiliate = CampaignAffiliate {
        campaign_id,
        affiliate: affiliate_address.clone(),
        commission_bps,
        total_paid: 0,
        active: true,
    };

    env.storage().persistent().set(
        &CampaignKey::CampaignAffiliate(campaign_id, affiliate_address.clone()),
        &affiliate,
    );

    events::publish_affiliate_registered_event(
        env,
        campaign_id,
        affiliate_address.clone(),
        commission_bps,
        env.ledger().timestamp(),
    );

    reentrancy::exit(env);
}

/// Pay commission to an affiliate.
pub fn pay_affiliate_commission(
    env: &Env,
    caller: &Address,
    campaign_id: u64,
    affiliate_address: &Address,
    amount: i128,
) {
    reentrancy::enter(env);
    caller.require_auth();

    if amount <= 0 {
        panic_with_error!(env, ContractError::InvalidAmount);
    }

    let mut campaign = get_campaign(env, campaign_id);
    let admin = core::get_admin(env);
    if *caller != campaign.merchant && *caller != admin {
        panic_with_error!(env, ContractError::NotAuthorized);
    }

    let mut affiliate: CampaignAffiliate = env
        .storage()
        .persistent()
        .get(&CampaignKey::CampaignAffiliate(
            campaign_id,
            affiliate_address.clone(),
        ))
        .unwrap_or_else(|| panic_with_error!(env, CampaignError::AffiliateNotFound));

    affiliate.total_paid += amount;

    env.storage().persistent().set(
        &CampaignKey::CampaignAffiliate(campaign_id, affiliate_address.clone()),
        &affiliate,
    );
    env.storage()
        .persistent()
        .set(&CampaignKey::FeeCampaign(campaign_id), &campaign);

    events::publish_affiliate_commission_paid_event(
        env,
        campaign_id,
        affiliate_address.clone(),
        amount,
        affiliate.total_paid,
        env.ledger().timestamp(),
    );

    reentrancy::exit(env);
}

// ── Leaderboard ───────────────────────────────────────────────────────────────
pub fn get_campaign(env: &Env, campaign_id: u64) -> FeeCampaign {
    env.storage()
        .persistent()
        .get(&CampaignKey::FeeCampaign(campaign_id))
        .unwrap_or_else(|| panic_with_error!(env, CampaignError::CampaignNotFound))
}

pub fn get_campaign_participant(
    env: &Env,
    campaign_id: u64,
    participant: &Address,
) -> CampaignParticipant {
    get_participant(env, campaign_id, participant)
}

pub fn get_campaign_affiliate(
    env: &Env,
    campaign_id: u64,
    affiliate: &Address,
) -> CampaignAffiliate {
    env.storage()
        .persistent()
        .get(&CampaignKey::CampaignAffiliate(
            campaign_id,
            affiliate.clone(),
        ))
        .unwrap_or_else(|| panic_with_error!(env, CampaignError::AffiliateNotFound))
}

/// Get top participants by score (descending), limited to `limit` entries.
pub fn get_campaign_leaderboard(env: &Env, campaign_id: u64, limit: u32) -> Vec<(Address, i128)> {
    let participant_ids = env
        .storage()
        .persistent()
        .get(&CampaignKey::CampaignParticipants(campaign_id))
        .unwrap_or_else(|| Vec::new(env));

    let mut rows: Vec<(Address, i128)> = Vec::new(env);
    for participant_id in participant_ids.iter() {
        let participant = get_participant(env, campaign_id, &participant_id);
        rows.push_back((participant_id.clone(), participant.score));
    }

    // Simple insertion sort for leaderboard ordering
    let n = rows.len();
    let mut i: u32 = 1;
    while i < n {
        let mut j = i;
        while j > 0 {
            let prev = rows.get_unchecked(j - 1);
            let curr = rows.get_unchecked(j);
            if curr.1 > prev.1 {
                rows.set(j - 1, curr);
                rows.set(j, prev);
                j -= 1;
            } else {
                break;
            }
        }
        i += 1;
    }

    while rows.len() > limit {
        rows.pop_back();
    }
    rows
}

// ── Read accessors ────────────────────────────────────────────────────────────

pub fn get_campaign(env: &Env, campaign_id: u64) -> Campaign {
    env.storage()
        .persistent()
        .get(&DataKey::StakeableCampaign(campaign_id))
        .unwrap_or_else(|| panic_with_error!(env, ContractError::CampaignNotFound))
}

pub fn get_campaign_participant(env: &Env, campaign_id: u64, participant: &Address) -> CampaignParticipant {
    get_participant(env, campaign_id, participant)
        .get(&CampaignKey::FeeCampaignCount)
        .unwrap_or(0)
}

fn get_participant(env: &Env, campaign_id: u64, participant: &Address) -> CampaignParticipant {
    env.storage()
        .persistent()
        .get(&CampaignKey::CampaignParticipant(
            campaign_id,
            participant.clone(),
        ))
        .unwrap_or(CampaignParticipant {
            campaign_id,
            participant: participant.clone(),
            contributed: 0,
            staked: 0,
            slashed: 0,
            commissions_paid: 0,
            score: 0,
        })
}

pub fn get_campaign_affiliate(env: &Env, campaign_id: u64, affiliate: &Address) -> CampaignAffiliate {
    env.storage()
        .persistent()
        .get(&DataKey::CampaignAffiliate(campaign_id, affiliate.clone()))
        .unwrap_or_else(|| panic_with_error!(env, ContractError::AffiliateNotFound))
        .get(&CampaignKey::CampaignParticipants(campaign_id))
        .unwrap_or_else(|| Vec::new(env));

    let mut exists = false;
    for existing in participant_ids.iter() {
        if existing == participant.participant {
            exists = true;
            break;
        }
    }

    if !exists {
        let mut updated_ids = Vec::new(env);
        for existing in participant_ids.iter() {
            updated_ids.push_back(existing);
        }
        updated_ids.push_back(participant.participant.clone());
        env.storage().persistent().set(
            &CampaignKey::CampaignParticipants(campaign_id),
            &updated_ids,
        );
    }

    env.storage().persistent().set(
        &CampaignKey::CampaignParticipant(campaign_id, participant.participant.clone()),
        participant,
    );
}
