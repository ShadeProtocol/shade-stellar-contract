use crate::components::campaign;
use crate::errors::ContractError;
use crate::types::{Campaign, CampaignAffiliate, CampaignParticipant, CampaignStatus, DataKey};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env, String};

/// Helper: register a test admin address.
fn setup_admin(env: &Env) -> Address {
    let admin = Address::generate(env);
    env.storage()
        .persistent()
        .set(&DataKey::ContractInfo, &crate::types::ContractInfo {
            admin: admin.clone(),
            timestamp: 0,
        });
    env.storage().persistent().set(&DataKey::PlatformAccount, &admin);
    admin
}

/// Helper: register a test merchant and return its address + merchant ID.
fn setup_merchant(env: &Env, admin: &Address) -> (Address, u64) {
    let merchant = Address::generate(env);
    // Accept a test token so campaigns can be created
    let token = Address::generate(env);
    env.storage()
        .persistent()
        .set(&DataKey::AcceptedTokens, &soroban_sdk::vec![env, token.clone()]);
    // Register merchant
    let merchant_id = 1u64;
    env.storage().persistent().set(&DataKey::Merchant(merchant_id), &crate::types::Merchant {
        id: merchant_id,
        address: merchant.clone(),
        active: true,
        verified: true,
        date_registered: 12345,
        account: merchant.clone(),
        webhook: String::from_str(env, ""),
        auto_withdrawal_recipient: None,
        auto_withdrawal_thresholds: soroban_sdk::vec![env],
    });
    env.storage().persistent().set(&DataKey::MerchantCount, &merchant_id);
    env.storage().persistent().set(&DataKey::MerchantId(merchant.clone()), &merchant_id);
    (merchant, merchant_id)
}

/// Helper: create a staking campaign via the campaign component.
fn create_test_campaign(env: &Env, caller: &Address, token: &Address, deadline: u64) -> u64 {
    let name = String::from_str(env, "Test Campaign");
    let description = String::from_str(env, "For penalty testing");
    campaign::create_campaign(
        env,
        caller,
        &name,
        &description,
        1000i128,
        token,
        deadline,
        100i128, // stake_required
    )
}

// ── Stake & Slash Tests ───────────────────────────────────────────────────────

#[test]
fn test_stake_campaign_success() {
    let env = Env::default();
    let admin = setup_admin(&env);
    let (merchant, _) = setup_merchant(&env, &admin);
    let token = Address::generate(&env);
    let deadline = 200_000u64;

    let campaign_id = create_test_campaign(&env, &merchant, &token, deadline);

    // Stake 50 tokens
    campaign::stake_campaign(&env, &merchant, campaign_id, 50);
    let participant = campaign::get_campaign_participant(&env, campaign_id, &merchant);
    assert_eq!(participant.staked, 50);
    assert_eq!(participant.score, 50);
}

#[test]
#[should_panic(expected = "Error(Contract, #7)")]
fn test_stake_campaign_zero_amount_fails() {
    let env = Env::default();
    let admin = setup_admin(&env);
    let (merchant, _) = setup_merchant(&env, &admin);
    let token = Address::generate(&env);
    let deadline = 200_000u64;

    let campaign_id = create_test_campaign(&env, &merchant, &token, deadline);
    campaign::stake_campaign(&env, &merchant, campaign_id, 0);
}

// ── Financial Penalty (Slash) Tests (#360) ────────────────────────────────────

#[test]
fn test_slash_campaign_stake_success() {
    let env = Env::default();
    let admin = setup_admin(&env);
    let (merchant, _) = setup_merchant(&env, &admin);
    let token = Address::generate(&env);
    let deadline = 200_000u64;

    let campaign_id = create_test_campaign(&env, &merchant, &token, deadline);

    // Stake first
    let participant = Address::generate(&env);
    // Register participant in campaign participants list
    let mut participants = soroban_sdk::Vec::new(&env);
    participants.push_back(participant.clone());
    env.storage()
        .persistent()
        .set(&DataKey::CampaignParticipants(campaign_id), &participants);
    // Give participant some stake
    env.storage().persistent().set(
        &DataKey::CampaignParticipant(campaign_id, participant.clone()),
        &CampaignParticipant {
            campaign_id,
            participant: participant.clone(),
            contributed: 0,
            staked: 100,
            slashed: 0,
            commissions_paid: 0,
            score: 100,
        },
    );
    // Set campaign raised_amount
    let mut camp = campaign::get_campaign(&env, campaign_id);
    camp.raised_amount = 100;
    env.storage()
        .persistent()
        .set(&DataKey::StakeableCampaign(campaign_id), &camp);

    // Merchant slashes 30 from participant
    campaign::slash_campaign_stake(&env, &merchant, campaign_id, &participant, 30);

    let updated_participant = campaign::get_campaign_participant(&env, campaign_id, &participant);
    assert_eq!(updated_participant.staked, 70);
    assert_eq!(updated_participant.slashed, 30);
    assert_eq!(updated_participant.score, 70);

    let updated_campaign = campaign::get_campaign(&env, campaign_id);
    assert_eq!(updated_campaign.total_slashed, 30);
}

#[test]
#[should_panic(expected = "Error(Contract, #7)")]
fn test_slash_exceeds_staked_amount_fails() {
    let env = Env::default();
    let admin = setup_admin(&env);
    let (merchant, _) = setup_merchant(&env, &admin);
    let token = Address::generate(&env);
    let deadline = 200_000u64;

    let campaign_id = create_test_campaign(&env, &merchant, &token, deadline);

    let participant = Address::generate(&env);
    let mut participants = soroban_sdk::Vec::new(&env);
    participants.push_back(participant.clone());
    env.storage()
        .persistent()
        .set(&DataKey::CampaignParticipants(campaign_id), &participants);
    env.storage().persistent().set(
        &DataKey::CampaignParticipant(campaign_id, participant.clone()),
        &CampaignParticipant {
            campaign_id,
            participant: participant.clone(),
            contributed: 0,
            staked: 50,
            slashed: 0,
            commissions_paid: 0,
            score: 50,
        },
    );

    // Try to slash more than staked
    campaign::slash_campaign_stake(&env, &merchant, campaign_id, &participant, 100);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_slash_by_non_owner_fails() {
    let env = Env::default();
    let admin = setup_admin(&env);
    let (merchant, _) = setup_merchant(&env, &admin);
    let token = Address::generate(&env);
    let deadline = 200_000u64;

    let campaign_id = create_test_campaign(&env, &merchant, &token, deadline);

    let participant = Address::generate(&env);
    let mut participants = soroban_sdk::Vec::new(&env);
    participants.push_back(participant.clone());
    env.storage()
        .persistent()
        .set(&DataKey::CampaignParticipants(campaign_id), &participants);
    env.storage().persistent().set(
        &DataKey::CampaignParticipant(campaign_id, participant.clone()),
        &CampaignParticipant {
            campaign_id,
            participant: participant.clone(),
            contributed: 0,
            staked: 100,
            slashed: 0,
            commissions_paid: 0,
            score: 100,
        },
    );

    // Random address tries to slash
    let random = Address::generate(&env);
    campaign::slash_campaign_stake(&env, &random, campaign_id, &participant, 30);
}

#[test]
fn test_admin_can_slash() {
    let env = Env::default();
    let admin = setup_admin(&env);
    let (merchant, _) = setup_merchant(&env, &admin);
    let token = Address::generate(&env);
    let deadline = 200_000u64;

    let campaign_id = create_test_campaign(&env, &merchant, &token, deadline);

    let participant = Address::generate(&env);
    let mut participants = soroban_sdk::Vec::new(&env);
    participants.push_back(participant.clone());
    env.storage()
        .persistent()
        .set(&DataKey::CampaignParticipants(campaign_id), &participants);
    env.storage().persistent().set(
        &DataKey::CampaignParticipant(campaign_id, participant.clone()),
        &CampaignParticipant {
            campaign_id,
            participant: participant.clone(),
            contributed: 0,
            staked: 100,
            slashed: 0,
            commissions_paid: 0,
            score: 100,
        },
    );

    // Admin can also slash
    campaign::slash_campaign_stake(&env, &admin, campaign_id, &participant, 40);

    let updated_participant = campaign::get_campaign_participant(&env, campaign_id, &participant);
    assert_eq!(updated_participant.staked, 60);
    assert_eq!(updated_participant.slashed, 40);
}

// ── Affiliate Tests ──────────────────────────────────────────────────────────

#[test]
fn test_register_affiliate_success() {
    let env = Env::default();
    let admin = setup_admin(&env);
    let (merchant, _) = setup_merchant(&env, &admin);
    let token = Address::generate(&env);
    let deadline = 200_000u64;

    let campaign_id = create_test_campaign(&env, &merchant, &token, deadline);

    let affiliate = Address::generate(&env);
    campaign::register_affiliate(&env, &merchant, campaign_id, &affiliate, 500);

    let aff = campaign::get_campaign_affiliate(&env, campaign_id, &affiliate);
    assert_eq!(aff.commission_bps, 500);
    assert!(aff.active);
    assert_eq!(aff.total_paid, 0);
}

#[test]
fn test_pay_affiliate_commission_success() {
    let env = Env::default();
    let admin = setup_admin(&env);
    let (merchant, _) = setup_merchant(&env, &admin);
    let token = Address::generate(&env);
    let deadline = 200_000u64;

    let campaign_id = create_test_campaign(&env, &merchant, &token, deadline);

    let affiliate = Address::generate(&env);
    campaign::register_affiliate(&env, &merchant, campaign_id, &affiliate, 500);
    campaign::pay_affiliate_commission(&env, &merchant, campaign_id, &affiliate, 200);

    let aff = campaign::get_campaign_affiliate(&env, campaign_id, &affiliate);
    assert_eq!(aff.total_paid, 200);
}

// ── Leaderboard Tests ────────────────────────────────────────────────────────

#[test]
fn test_leaderboard_returns_sorted_by_score() {
    let env = Env::default();
    let admin = setup_admin(&env);
    let (merchant, _) = setup_merchant(&env, &admin);
    let token = Address::generate(&env);
    let deadline = 200_000u64;

    let campaign_id = create_test_campaign(&env, &merchant, &token, deadline);

    let p1 = Address::generate(&env);
    let p2 = Address::generate(&env);
    let p3 = Address::generate(&env);

    let mut participants = soroban_sdk::Vec::new(&env);
    participants.push_back(p1.clone());
    participants.push_back(p2.clone());
    participants.push_back(p3.clone());
    env.storage()
        .persistent()
        .set(&DataKey::CampaignParticipants(campaign_id), &participants);

    env.storage().persistent().set(
        &DataKey::CampaignParticipant(campaign_id, p1.clone()),
        &CampaignParticipant {
            campaign_id,
            participant: p1.clone(),
            contributed: 0,
            staked: 100,
            slashed: 0,
            commissions_paid: 0,
            score: 100,
        },
    );
    env.storage().persistent().set(
        &DataKey::CampaignParticipant(campaign_id, p2.clone()),
        &CampaignParticipant {
            campaign_id,
            participant: p2.clone(),
            contributed: 0,
            staked: 300,
            slashed: 0,
            commissions_paid: 0,
            score: 300,
        },
    );
    env.storage().persistent().set(
        &DataKey::CampaignParticipant(campaign_id, p3.clone()),
        &CampaignParticipant {
            campaign_id,
            participant: p3.clone(),
            contributed: 0,
            staked: 200,
            slashed: 0,
            commissions_paid: 0,
            score: 200,
        },
    );

    let leaderboard = campaign::get_campaign_leaderboard(&env, campaign_id, 3);
    assert_eq!(leaderboard.len(), 3);
    // First should be p2 (score 300)
    assert_eq!(leaderboard.get(0).unwrap(), (p2.clone(), 300));
    // Second should be p3 (score 200)
    assert_eq!(leaderboard.get(1).unwrap(), (p3.clone(), 200));
    // Third should be p1 (score 100)
    assert_eq!(leaderboard.get(2).unwrap(), (p1.clone(), 100));
}

#[test]
fn test_leaderboard_limited_results() {
    let env = Env::default();
    let admin = setup_admin(&env);
    let (merchant, _) = setup_merchant(&env, &admin);
    let token = Address::generate(&env);
    let deadline = 200_000u64;

    let campaign_id = create_test_campaign(&env, &merchant, &token, deadline);

    let p1 = Address::generate(&env);
    let p2 = Address::generate(&env);

    let mut participants = soroban_sdk::Vec::new(&env);
    participants.push_back(p1.clone());
    participants.push_back(p2.clone());
    env.storage()
        .persistent()
        .set(&DataKey::CampaignParticipants(campaign_id), &participants);

    env.storage().persistent().set(
        &DataKey::CampaignParticipant(campaign_id, p1.clone()),
        &CampaignParticipant {
            campaign_id,
            participant: p1.clone(),
            contributed: 0, staked: 100, slashed: 0, commissions_paid: 0, score: 100,
        },
    );
    env.storage().persistent().set(
        &DataKey::CampaignParticipant(campaign_id, p2.clone()),
        &CampaignParticipant {
            campaign_id,
            participant: p2.clone(),
            contributed: 0, staked: 200, slashed: 0, commissions_paid: 0, score: 200,
        },
    );

    let leaderboard = campaign::get_campaign_leaderboard(&env, campaign_id, 1);
    assert_eq!(leaderboard.len(), 1);
    assert_eq!(leaderboard.get(0).unwrap(), (p2.clone(), 200));
}

#[test]
fn test_campaign_created_with_zero_slashed() {
    let env = Env::default();
    let admin = setup_admin(&env);
    let (merchant, _) = setup_merchant(&env, &admin);
    let token = Address::generate(&env);
    let deadline = 200_000u64;

    let campaign_id = create_test_campaign(&env, &merchant, &token, deadline);
    let camp = campaign::get_campaign(&env, campaign_id);
    assert_eq!(camp.total_slashed, 0);
    assert_eq!(camp.penalty_count, 0);
}
