use soroban_sdk::{contractevent, Address, BytesN, Env, Option, String, Vec};

// ── Existing events ───────────────────────────────────────────────────────────

#[contractevent]
pub struct InitalizedEvent {
    pub admin: Address,
    pub timestamp: u64,
}

pub fn publish_initialized_event(env: &Env, admin: Address, timestamp: u64) {
    InitalizedEvent { admin, timestamp }.publish(env);
}
// no new changes to add

#[contractevent]
pub struct TokenAddedEvent {
    pub token: Address,
    pub timestamp: u64,
}

pub fn publish_token_added_event(env: &Env, token: Address, timestamp: u64) {
    TokenAddedEvent { token, timestamp }.publish(env);
}

#[contractevent]
pub struct TokenRemovedEvent {
    pub token: Address,
    pub timestamp: u64,
}

pub fn publish_token_removed_event(env: &Env, token: Address, timestamp: u64) {
    TokenRemovedEvent { token, timestamp }.publish(env);
}

#[contractevent]
pub struct MerchantRegisteredEvent {
    pub merchant: Address,
    pub merchant_id: u64,
    pub timestamp: u64,
}

pub fn publish_merchant_registered_event(
    env: &Env,
    merchant: Address,
    merchant_id: u64,
    timestamp: u64,
) {
    MerchantRegisteredEvent {
        merchant,
        merchant_id,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct MerchantAccountDeployedEvent {
    pub merchant: Address,
    pub contract: Address,
    pub timestamp: u64,
}

pub fn publish_merchant_account_deployed_event(
    env: &Env,
    merchant: Address,
    contract: Address,
    timestamp: u64,
) {
    MerchantAccountDeployedEvent {
        merchant,
        contract,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct MerchantStatusChangedEvent {
    pub merchant_id: u64,
    pub active: bool,
    pub timestamp: u64,
}

pub fn publish_merchant_status_changed_event(
    env: &Env,
    merchant_id: u64,
    active: bool,
    timestamp: u64,
) {
    MerchantStatusChangedEvent {
        merchant_id,
        active,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct InvoiceCreatedEvent {
    pub invoice_id: u64,
    pub merchant: Address,
    pub amount: i128,
    pub token: Address,
}

pub fn publish_invoice_created_event(
    env: &Env,
    invoice_id: u64,
    merchant: Address,
    amount: i128,
    token: Address,
) {
    InvoiceCreatedEvent {
        invoice_id,
        merchant,
        amount,
        token,
    }
    .publish(env);
}

#[contractevent]
pub struct InvoiceRefundedEvent {
    pub invoice_id: u64,
    pub merchant: Address,
    pub amount: i128,
    pub timestamp: u64,
}

pub fn publish_invoice_refunded_event(
    env: &Env,
    invoice_id: u64,
    merchant: Address,
    amount: i128,
    timestamp: u64,
) {
    InvoiceRefundedEvent {
        invoice_id,
        merchant,
        amount,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct InvoicePartiallyRefundedEvent {
    pub invoice_id: u64,
    pub merchant: Address,
    pub amount: i128,
    pub total_amount_refunded: i128,
    pub timestamp: u64,
}

pub fn publish_invoice_partially_refunded_event(
    env: &Env,
    invoice_id: u64,
    merchant: Address,
    amount: i128,
    total_amount_refunded: i128,
    timestamp: u64,
) {
    InvoicePartiallyRefundedEvent {
        invoice_id,
        merchant,
        amount,
        total_amount_refunded,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct MerchantVerifiedEvent {
    pub merchant_id: u64,
    pub status: bool,
    pub timestamp: u64,
}

pub fn publish_merchant_verified_event(env: &Env, merchant_id: u64, status: bool, timestamp: u64) {
    MerchantVerifiedEvent {
        merchant_id,
        status,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct MerchantWebhookSetEvent {
    pub merchant: Address,
    pub merchant_id: u64,
    pub webhook: String,
    pub timestamp: u64,
}

pub fn publish_merchant_webhook_set_event(
    env: &Env,
    merchant: Address,
    merchant_id: u64,
    webhook: String,
    timestamp: u64,
) {
    MerchantWebhookSetEvent {
        merchant,
        merchant_id,
        webhook,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct MerchantKeySetEvent {
    pub merchant: Address,
    pub key: BytesN<32>,
    pub timestamp: u64,
}

pub fn publish_merchant_key_set_event(
    env: &Env,
    merchant: Address,
    key: BytesN<32>,
    timestamp: u64,
) {
    MerchantKeySetEvent {
        merchant,
        key,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct RoleGrantedEvent {
    pub admin: Address,
    pub user: Address,
    pub role: crate::types::Role,
    pub timestamp: u64,
}

pub fn publish_role_granted_event(
    env: &Env,
    admin: Address,
    user: Address,
    role: crate::types::Role,
    timestamp: u64,
) {
    RoleGrantedEvent {
        admin,
        user,
        role,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct RoleRevokedEvent {
    pub admin: Address,
    pub user: Address,
    pub role: crate::types::Role,
    pub timestamp: u64,
}

pub fn publish_role_revoked_event(
    env: &Env,
    admin: Address,
    user: Address,
    role: crate::types::Role,
    timestamp: u64,
) {
    RoleRevokedEvent {
        admin,
        user,
        role,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct ContractPausedEvent {
    pub admin: Address,
    pub timestamp: u64,
}

pub fn publish_contract_paused_event(env: &Env, admin: Address, timestamp: u64) {
    ContractPausedEvent { admin, timestamp }.publish(env);
}

#[contractevent]
pub struct ContractUnpausedEvent {
    pub admin: Address,
    pub timestamp: u64,
}

pub fn publish_contract_unpaused_event(env: &Env, admin: Address, timestamp: u64) {
    ContractUnpausedEvent { admin, timestamp }.publish(env);
}

#[contractevent]
pub struct FeeProposedEvent {
    pub admin: Address,
    pub token: Address,
    pub fee: i128,
    pub timestamp: u64,
}

pub fn publish_fee_proposed_event(
    env: &Env,
    admin: Address,
    token: Address,
    fee: i128,
    timestamp: u64,
) {
    FeeProposedEvent {
        admin,
        token,
        fee,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct FeeSetEvent {
    pub admin: Address,
    pub token: Address,
    pub fee: i128,
    pub timestamp: u64,
}

pub fn publish_fee_set_event(env: &Env, admin: Address, token: Address, fee: i128, timestamp: u64) {
    FeeSetEvent {
        admin,
        token,
        fee,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct PlatformAccountSetEvent {
    pub admin: Address,
    pub account: Address,
    pub timestamp: u64,
}

pub fn publish_platform_account_set_event(
    env: &Env,
    admin: Address,
    account: Address,
    timestamp: u64,
) {
    PlatformAccountSetEvent {
        admin,
        account,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct TokenOracleSetEvent {
    pub admin: Address,
    pub token: Address,
    pub oracle: Address,
    pub timestamp: u64,
}

pub fn publish_token_oracle_set_event(
    env: &Env,
    admin: Address,
    token: Address,
    oracle: Address,
    timestamp: u64,
) {
    TokenOracleSetEvent {
        admin,
        token,
        oracle,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct ContractUpgradedEvent {
    pub new_wasm_hash: BytesN<32>,
    pub timestamp: u64,
}

pub fn publish_contract_upgraded_event(env: &Env, new_wasm_hash: BytesN<32>, timestamp: u64) {
    ContractUpgradedEvent {
        new_wasm_hash,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct AccountRestrictedEvent {
    pub merchant: Address,
    pub status: bool,
    pub caller: Address,
    pub timestamp: u64,
}

pub fn publish_account_restricted_event(
    env: &Env,
    merchant: Address,
    status: bool,
    caller: Address,
    timestamp: u64,
) {
    AccountRestrictedEvent {
        merchant,
        status,
        caller,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct FeeDiscountAppliedEvent {
    pub merchant: Address,
    pub volume: i128,
    pub discount_bps: i128,
    pub timestamp: u64,
}

pub fn publish_fee_discount_applied_event(
    env: &Env,
    merchant: Address,
    volume: i128,
    discount_bps: i128,
    timestamp: u64,
) {
    FeeDiscountAppliedEvent {
        merchant,
        volume,
        discount_bps,
        timestamp,
    }
    .publish(env);
}

// Kept merchant_amount from your branch AND merchant_account from main — both are useful.
#[contractevent]
pub struct InvoicePaidEvent {
    pub invoice_id: u64,
    pub merchant_id: u64,
    pub merchant_account: Address,
    pub payer: Address,
    pub amount: i128,
    pub fee: i128,
    pub merchant_amount: i128,
    pub token: Address,
    pub timestamp: u64,
}

#[allow(clippy::too_many_arguments)]
pub fn publish_invoice_paid_event(
    env: &Env,
    invoice_id: u64,
    merchant_id: u64,
    merchant_account: Address,
    payer: Address,
    amount: i128,
    fee: i128,
    merchant_amount: i128,
    token: Address,
    timestamp: u64,
) {
    InvoicePaidEvent {
        invoice_id,
        merchant_id,
        merchant_account,
        payer,
        amount,
        fee,
        merchant_amount,
        token,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct FiatInvoicePricedEvent {
    pub invoice_id: u64,
    pub token: Address,
    pub resolved_amount: i128,
    pub timestamp: u64,
}

pub fn publish_fiat_invoice_priced_event(
    env: &Env,
    invoice_id: u64,
    token: Address,
    resolved_amount: i128,
    timestamp: u64,
) {
    FiatInvoicePricedEvent {
        invoice_id,
        token,
        resolved_amount,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct PaymentSplitRoutedEvent {
    pub invoice_id: u64,
    pub merchant_account: Address,
    pub platform_account: Address,
    pub merchant_amount: i128,
    pub platform_amount: i128,
    pub token: Address,
    pub timestamp: u64,
}

pub fn publish_payment_split_routed_event(
    env: &Env,
    invoice_id: u64,
    merchant_account: Address,
    platform_account: Address,
    merchant_amount: i128,
    platform_amount: i128,
    token: Address,
    timestamp: u64,
) {
    PaymentSplitRoutedEvent {
        invoice_id,
        merchant_account,
        platform_account,
        merchant_amount,
        platform_amount,
        token,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct PlatformFeeRoutedEvent {
    pub route_kind: u32,
    pub ref_id: u64,
    pub merchant_id: u64,
    pub merchant: Address,
    pub merchant_account: Address,
    pub platform_account: Address,
    pub payer: Address,
    pub gross_amount: i128,
    pub platform_fee: i128,
    pub merchant_amount: i128,
    pub token: Address,
    pub fee_bps_applied: i128,
    pub timestamp: u64,
}

pub fn publish_platform_fee_routed_event(
    env: &Env,
    route_kind: crate::types::PlatformFeeRouteKind,
    ref_id: u64,
    merchant_id: u64,
    merchant: Address,
    merchant_account: Address,
    platform_account: Address,
    payer: Address,
    gross_amount: i128,
    platform_fee: i128,
    merchant_amount: i128,
    token: Address,
    fee_bps_applied: i128,
    timestamp: u64,
) {
    PlatformFeeRoutedEvent {
        route_kind: route_kind as u32,
        ref_id,
        merchant_id,
        merchant,
        merchant_account,
        platform_account,
        payer,
        gross_amount,
        platform_fee,
        merchant_amount,
        token,
        fee_bps_applied,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct MerchantPlatformFeeSetEvent {
    pub caller: Address,
    pub merchant_id: u64,
    pub token: Address,
    pub fee_bps: i128,
    pub timestamp: u64,
}

pub fn publish_merchant_platform_fee_set_event(
    env: &Env,
    caller: Address,
    merchant_id: u64,
    token: Address,
    fee_bps: i128,
    timestamp: u64,
) {
    MerchantPlatformFeeSetEvent {
        caller,
        merchant_id,
        token,
        fee_bps,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct MerchantPlatformFeeClearedEvent {
    pub caller: Address,
    pub merchant_id: u64,
    pub token: Address,
    pub timestamp: u64,
}

pub fn publish_merchant_platform_fee_cleared_event(
    env: &Env,
    caller: Address,
    merchant_id: u64,
    token: Address,
    timestamp: u64,
) {
    MerchantPlatformFeeClearedEvent {
        caller,
        merchant_id,
        token,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct InvoiceCancelledEvent {
    pub invoice_id: u64,
    pub merchant: Address,
    pub timestamp: u64,
}

pub fn publish_invoice_cancelled_event(
    env: &Env,
    invoice_id: u64,
    merchant: Address,
    timestamp: u64,
) {
    InvoiceCancelledEvent {
        invoice_id,
        merchant,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct InvoiceAmendedEvent {
    pub invoice_id: u64,
    pub merchant: Address,
    pub old_amount: i128,
    pub new_amount: i128,
    pub timestamp: u64,
}

pub fn publish_invoice_amended_event(
    env: &Env,
    invoice_id: u64,
    merchant: Address,
    old_amount: i128,
    new_amount: i128,
    timestamp: u64,
) {
    InvoiceAmendedEvent {
        invoice_id,
        merchant,
        old_amount,
        new_amount,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct NonceInvalidatedEvent {
    pub merchant: Address,
    pub nonce: BytesN<32>,
    pub timestamp: u64,
}

pub fn publish_nonce_invalidated_event(
    env: &Env,
    merchant: Address,
    nonce: BytesN<32>,
    timestamp: u64,
) {
    NonceInvalidatedEvent {
        merchant,
        nonce,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct BridgePlaceholderEvent {
    pub caller: Address,
    pub payload: crate::types::CrossChainBridgePayload,
    pub timestamp: u64,
}

pub fn publish_bridge_placeholder_event(
    env: &Env,
    caller: Address,
    payload: crate::types::CrossChainBridgePayload,
    timestamp: u64,
) {
    BridgePlaceholderEvent {
        caller,
        payload,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct CrossChainPledgeCreatedEvent {
    pub pledge_id: u64,
    pub source_chain: String,
    pub source_pledge_id: u64,
    pub merchant: Address,
    pub payer: Address,
    pub token: Address,
    pub amount: i128,
    pub timestamp: u64,
}

pub fn publish_cross_chain_pledge_created(
    env: &Env,
    pledge: &crate::types::CrossChainPledge,
) {
    CrossChainPledgeCreatedEvent {
        pledge_id: pledge.id,
        source_chain: pledge.source_chain.clone(),
        source_pledge_id: pledge.source_pledge_id,
        merchant: pledge.merchant.clone(),
        payer: pledge.payer.clone(),
        token: pledge.token.clone(),
        amount: pledge.amount,
        timestamp: pledge.created_at,
    }
}
// ── Bridge listener / external deposit events ─────────────────────────────────

#[contractevent]
pub struct BridgeListenerRegisteredEvent {
    pub admin: Address,
    pub listener: Address,
    pub timestamp: u64,
}

pub fn publish_bridge_listener_registered_event(
    env: &Env,
    admin: Address,
    listener: Address,
    timestamp: u64,
) {
    BridgeListenerRegisteredEvent {
        admin,
        listener,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct BridgeListenerRemovedEvent {
    pub admin: Address,
    pub listener: Address,
    pub timestamp: u64,
}

pub fn publish_bridge_listener_removed_event(
    env: &Env,
    admin: Address,
    listener: Address,
    timestamp: u64,
) {
    BridgeListenerRemovedEvent {
        admin,
        listener,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct BridgeDepositRecordedEvent {
    pub deposit_id: u64,
    pub listener: Address,
    pub source_chain: String,
    pub source_tx_id: BytesN<32>,
    pub token: Address,
    pub amount: i128,
    pub recipient: Address,
    pub timestamp: u64,
}

#[allow(clippy::too_many_arguments)]
pub fn publish_bridge_deposit_recorded_event(
    env: &Env,
    deposit_id: u64,
    listener: Address,
    source_chain: String,
    source_tx_id: BytesN<32>,
    token: Address,
    amount: i128,
    recipient: Address,
    timestamp: u64,
) {
    BridgeDepositRecordedEvent {
        deposit_id,
        listener,
        source_chain,
        source_tx_id,
        token,
        amount,
        recipient,
        timestamp,
    }
    .publish(env);
}

// ── DAO governance events ─────────────────────────────────────────────────────

#[contractevent]
pub struct GovMemberAddedEvent {
    pub admin: Address,
    pub member: Address,
    pub member_count: u32,
    pub timestamp: u64,
}

pub fn publish_gov_member_added_event(
    env: &Env,
    admin: Address,
    member: Address,
    member_count: u32,
    timestamp: u64,
) {
    GovMemberAddedEvent {
        admin,
        member,
        member_count,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct GovMemberRemovedEvent {
    pub admin: Address,
    pub member: Address,
    pub member_count: u32,
    pub timestamp: u64,
}

pub fn publish_gov_member_removed_event(
    env: &Env,
    admin: Address,
    member: Address,
    member_count: u32,
    timestamp: u64,
) {
    GovMemberRemovedEvent {
        admin,
        member,
        member_count,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct GovConfigSetEvent {
    pub admin: Address,
    pub voting_period: u64,
    pub quorum_bps: u32,
    pub timestamp: u64,
}

pub fn publish_gov_config_set_event(
    env: &Env,
    admin: Address,
    voting_period: u64,
    quorum_bps: u32,
    timestamp: u64,
) {
    GovConfigSetEvent {
        admin,
        voting_period,
        quorum_bps,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct UpgradeProposedEvent {
    pub proposal_id: u64,
    pub proposer: Address,
    pub wasm_hash: BytesN<32>,
    pub voting_ends_at: u64,
    pub timestamp: u64,
}

pub fn publish_upgrade_proposed_event(
    env: &Env,
    proposal_id: u64,
    proposer: Address,
    wasm_hash: BytesN<32>,
    voting_ends_at: u64,
    timestamp: u64,
) {
    UpgradeProposedEvent {
        proposal_id,
        proposer,
        wasm_hash,
        voting_ends_at,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct UpgradeVoteCastEvent {
    pub proposal_id: u64,
    pub voter: Address,
    pub approve: bool,
    pub approvals: u32,
    pub rejections: u32,
    pub timestamp: u64,
}

pub fn publish_upgrade_vote_cast_event(
    env: &Env,
    proposal_id: u64,
    voter: Address,
    approve: bool,
    approvals: u32,
    rejections: u32,
    timestamp: u64,
) {
    UpgradeVoteCastEvent {
        proposal_id,
        voter,
        approve,
        approvals,
        rejections,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct CrossChainPledgeUpdatedEvent {
    pub pledge_id: u64,
    pub status: crate::types::CrossChainPledgeStatus,
    pub timestamp: u64,
}

pub fn publish_cross_chain_pledge_updated(
    env: &Env,
    pledge: &crate::types::CrossChainPledge,
) {
    CrossChainPledgeUpdatedEvent {
        pledge_id: pledge.id,
        status: pledge.status.clone(),
        timestamp: pledge.updated_at,
    }
}
pub struct UpgradeProposalFinalizedEvent {
    pub proposal_id: u64,
    pub executor: Address,
    pub approved: bool,
    pub approvals: u32,
    pub rejections: u32,
    pub member_count: u32,
    pub timestamp: u64,
}

pub fn publish_upgrade_proposal_finalized_event(
    env: &Env,
    proposal_id: u64,
    executor: Address,
    approved: bool,
    approvals: u32,
    rejections: u32,
    member_count: u32,
    timestamp: u64,
) {
    UpgradeProposalFinalizedEvent {
        proposal_id,
        executor,
        approved,
        approvals,
        rejections,
        member_count,
        timestamp,
    }
    .publish(env);
}

// ── Subscription events ───────────────────────────────────────────────────────

// Kept token field from your branch (more informative than main's leaner version).
#[contractevent]
pub struct SubscriptionPlanCreatedEvent {
    pub plan_id: u64,
    pub merchant: Address,
    pub token: Address,
    pub amount: i128,
    pub interval: u64,
    pub timestamp: u64,
}

pub fn publish_subscription_plan_created_event(
    env: &Env,
    plan_id: u64,
    merchant: Address,
    token: Address,
    amount: i128,
    interval: u64,
    timestamp: u64,
) {
    SubscriptionPlanCreatedEvent {
        plan_id,
        merchant,
        token,
        amount,
        interval,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct SubscribedEvent {
    pub subscription_id: u64,
    pub plan_id: u64,
    pub customer: Address,
    pub timestamp: u64,
}

pub fn publish_subscribed_event(
    env: &Env,
    subscription_id: u64,
    plan_id: u64,
    customer: Address,
    timestamp: u64,
) {
    SubscribedEvent {
        subscription_id,
        plan_id,
        customer,
        timestamp,
    }
    .publish(env);
}

// Kept the richer version from your branch (plan_id, customer, merchant, token).
#[contractevent]
pub struct SubscriptionChargedEvent {
    pub subscription_id: u64,
    pub plan_id: u64,
    pub customer: Address,
    pub merchant: Address,
    pub amount: i128,
    pub fee: i128,
    pub token: Address,
    pub timestamp: u64,
}

#[allow(clippy::too_many_arguments)]
pub fn publish_subscription_charged_event(
    env: &Env,
    subscription_id: u64,
    plan_id: u64,
    customer: Address,
    merchant: Address,
    amount: i128,
    fee: i128,
    token: Address,
    timestamp: u64,
) {
    SubscriptionChargedEvent {
        subscription_id,
        plan_id,
        customer,
        merchant,
        amount,
        fee,
        token,
        timestamp,
    }
    .publish(env);
}

// Used "caller" from your branch — more accurate than "cancelled_by".
#[contractevent]
pub struct SubscriptionCancelledEvent {
    pub subscription_id: u64,
    pub caller: Address,
    pub timestamp: u64,
}

pub fn publish_subscription_cancelled_event(
    env: &Env,
    subscription_id: u64,
    caller: Address,
    timestamp: u64,
) {
    SubscriptionCancelledEvent {
        subscription_id,
        caller,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct AccountWasmHashSetEvent {
    pub admin: Address,
    pub wasm_hash: BytesN<32>,
    pub timestamp: u64,
}

pub fn publish_account_wasm_hash_set_event(
    env: &Env,
    admin: Address,
    wasm_hash: BytesN<32>,
    timestamp: u64,
) {
    AccountWasmHashSetEvent {
        admin,
        wasm_hash,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct PlanDeactivatedEvent {
    pub plan_id: u64,
    pub merchant: Address,
    pub timestamp: u64,
}

pub fn publish_plan_deactivated_event(env: &Env, plan_id: u64, merchant: Address, timestamp: u64) {
    PlanDeactivatedEvent {
        plan_id,
        merchant,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct MerchantTokensSetEvent {
    pub merchant: Address,
    pub tokens: Vec<Address>,
    pub timestamp: u64,
}

pub fn publish_merchant_tokens_set_event(
    env: &Env,
    merchant: Address,
    tokens: Vec<Address>,
    timestamp: u64,
) {
    MerchantTokensSetEvent {
        merchant,
        tokens,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct MerchantTokenRemovedEvent {
    pub merchant: Address,
    pub token: Address,
    pub timestamp: u64,
}

pub fn publish_merchant_token_removed_event(
    env: &Env,
    merchant: Address,
    token: Address,
    timestamp: u64,
) {
    MerchantTokenRemovedEvent {
        merchant,
        token,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct AutoWithdrawThresholdEvent {
    pub merchant_id: u64,
    pub token: Address,
    pub threshold: i128,
}

pub fn publish_auto_withdrawal_threshold_set_event(
    env: &Env,
    merchant_id: u64,
    token: Address,
    threshold: i128,
) {
    AutoWithdrawThresholdEvent {
        merchant_id,
        token,
        threshold,
    }
    .publish(env);
}

#[contractevent]
pub struct AutoWithdrawRecipientEvent {
    pub merchant_id: u64,
    pub recipient: Address,
}

pub fn publish_auto_withdrawal_recipient_set_event(
    env: &Env,
    merchant_id: u64,
    recipient: Address,
) {
    AutoWithdrawRecipientEvent {
        merchant_id,
        recipient,
    }
    .publish(env);
}

#[contractevent]
pub struct AutoWithdrawalTriggeredEvent {
    pub merchant_id: u64,
    pub token: Address,
    pub amount: i128,
    pub recipient: Address,
}

pub fn publish_auto_withdrawal_triggered_event(
    env: &Env,
    merchant_id: u64,
    token: Address,
    amount: i128,
    recipient: Address,
) {
    AutoWithdrawalTriggeredEvent {
        merchant_id,
        token,
        amount,
        recipient,
    }
    .publish(env);
}

// ── Admin transfer events ────────────────────────────────────────────────────

#[contractevent]
pub struct AdminTransferProposedEvent {
    pub current_admin: Address,
    pub proposed_admin: Address,
    pub timestamp: u64,
}

pub fn publish_admin_transfer_proposed_event(
    env: &Env,
    current_admin: Address,
    proposed_admin: Address,
    timestamp: u64,
) {
    AdminTransferProposedEvent {
        current_admin,
        proposed_admin,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct AdminTransferAcceptedEvent {
    pub old_admin: Address,
    pub new_admin: Address,
    pub timestamp: u64,
}

pub fn publish_admin_transfer_accepted_event(
    env: &Env,
    old_admin: Address,
    new_admin: Address,
    timestamp: u64,
) {
    AdminTransferAcceptedEvent {
        old_admin,
        new_admin,
        timestamp,
    }
    .publish(env);
}

// ── Event ticketing system ────────────────────────────────────────────────────

#[contractevent]
pub struct EventCreatedEvent {
    pub event_id: u64,
    pub merchant: Address,
    pub merchant_id: u64,
    pub name: String,
    pub ticket_price: i128,
    pub token: Address,
    pub capacity: u32,
    pub event_date: u64,
    pub royalty_bps: u32,
    pub timestamp: u64,
}

#[allow(clippy::too_many_arguments)]
pub fn publish_event_created_event(
    env: &Env,
    event_id: u64,
    merchant: Address,
    merchant_id: u64,
    name: String,
    ticket_price: i128,
    token: Address,
    capacity: u32,
    event_date: u64,
    royalty_bps: u32,
    timestamp: u64,
) {
    EventCreatedEvent {
        event_id,
        merchant,
        merchant_id,
        name,
        ticket_price,
        token,
        capacity,
        event_date,
        royalty_bps,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct TicketPurchasedEvent {
    pub ticket_id: u64,
    pub event_id: u64,
    pub merchant_id: u64,
    pub buyer: Address,
    pub amount: i128,
    pub fee: i128,
    pub merchant_amount: i128,
    pub token: Address,
    pub timestamp: u64,
}

#[allow(clippy::too_many_arguments)]
pub fn publish_ticket_purchased_event(
    env: &Env,
    ticket_id: u64,
    event_id: u64,
    merchant_id: u64,
    buyer: Address,
    amount: i128,
    fee: i128,
    merchant_amount: i128,
    token: Address,
    timestamp: u64,
) {
    TicketPurchasedEvent {
        ticket_id,
        event_id,
        merchant_id,
        buyer,
        amount,
        fee,
        merchant_amount,
        token,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct TicketResoldEvent {
    pub ticket_id: u64,
    pub event_id: u64,
    pub merchant_id: u64,
    pub seller: Address,
    pub buyer: Address,
    pub resale_price: i128,
    pub royalty: i128,
    pub seller_proceeds: i128,
    pub token: Address,
    pub timestamp: u64,
}

#[allow(clippy::too_many_arguments)]
pub fn publish_ticket_resold_event(
    env: &Env,
    ticket_id: u64,
    event_id: u64,
    merchant_id: u64,
    seller: Address,
    buyer: Address,
    resale_price: i128,
    royalty: i128,
    seller_proceeds: i128,
    token: Address,
    timestamp: u64,
) {
    TicketResoldEvent {
        ticket_id,
        event_id,
        merchant_id,
        seller,
        buyer,
        resale_price,
        royalty,
        seller_proceeds,
        token,
        timestamp,
    }
    .publish(env);
}

// ── Leaderboard Events ────────────────────────────────────────────────────────

#[contractevent]
pub struct LeaderboardUpdatedEvent {
    pub campaign_id: u64,
    pub donor: Address,
    pub amount: i128,
    pub new_total: i128,
    pub timestamp: u64,
}

pub fn publish_leaderboard_updated_event(
    env: &Env,
    campaign_id: u64,
    donor: Address,
    amount: i128,
    new_total: i128,
    timestamp: u64,
) {
    LeaderboardUpdatedEvent {
        campaign_id,
        donor,
        amount,
        new_total,
    }
}

// ── Campaign categories & tagging (#352) ──────────────────────────────────────

#[contractevent]
pub struct CampaignCategoryCreatedEvent {
    pub category_id: u64,
    pub admin: Address,
    pub name: String,
    pub description: String,
    pub timestamp: u64,
}

pub fn publish_campaign_category_created_event(
    env: &Env,
    category_id: u64,
    admin: Address,
    name: String,
    description: String,
    timestamp: u64,
) {
    CampaignCategoryCreatedEvent {
        category_id,
        admin,
        name,
        description,
        timestamp,
    }.publish(env)
}
// ── Auto-withdrawal events ─────────────────────────────────────────────────────

#[contractevent]
pub struct WithdrawalThresholdSetEvent {
    pub merchant_id: u64,
    pub token: Address,
    pub threshold: i128,
}

pub fn publish_auto_withdrawal_threshold_set_event(
    env: &Env,
    merchant_id: u64,
    token: Address,
    threshold: i128,
) {
    WithdrawalThresholdSetEvent {
        merchant_id,
        token,
        threshold,
    }
    .publish(env);
}

#[contractevent]
pub struct CampaignCategoryUpdatedEvent {
    pub category_id: u64,
    pub admin: Address,
    pub name: String,
    pub description: String,
    pub active: bool,
    pub timestamp: u64,
}

pub fn publish_campaign_category_updated_event(
    env: &Env,
    category_id: u64,
    admin: Address,
    name: String,
    description: String,
    active: bool,
    timestamp: u64,
) {
    CampaignCategoryUpdatedEvent {
        category_id,
        admin,
        name,
        description,
        active,
        timestamp,
    }.publish(env);
}


#[contractevent]
pub struct WithdrawalRecipientSetEvent {
    pub merchant_id: u64,
    pub recipient: Address,
}

pub fn publish_auto_withdrawal_recipient_set_event(
    env: &Env,
    merchant_id: u64,
    recipient: Address,
) {
    WithdrawalRecipientSetEvent {
        merchant_id,
        recipient,
    }
    .publish(env);
}

#[contractevent]
pub struct CampaignTagCreatedEvent {
    pub tag_id: u64,
    pub creator: Address,
    pub name: String,
    pub timestamp: u64,
}

pub fn publish_campaign_tag_created_event(
    env: &Env,
    tag_id: u64,
    creator: Address,
    name: String,
    timestamp: u64,
) {
    CampaignTagCreatedEvent {
        tag_id,
        creator,
        name,
        timestamp,
    }
    .publish(env);
}

#[allow(clippy::too_many_arguments)]
#[contractevent]
pub struct CampaignCreatedEvent {
    pub campaign_id: u64,
    pub merchant: Address,
    pub merchant_id: u64,
    pub title: String,
    pub description: String,
    pub category_id: u64,
    pub tags: Vec<u64>,
    pub goal_amount: i128,
    pub token: Address,
    pub deadline: u64,
    pub timestamp: u64,
}

#[allow(clippy::too_many_arguments)]
pub fn publish_campaign_created_event(
    env: &Env,
    campaign_id: u64,
    merchant: Address,
    merchant_id: u64,
    title: String,
    description: String,
    category_id: u64,
    tags: Vec<u64>,
    goal_amount: i128,
    token: Address,
    deadline: u64,
    timestamp: u64,
) {
    CampaignCreatedEvent {
        campaign_id,
        merchant,
        merchant_id,
        title,
        description,
        category_id,
        tags,
        goal_amount,
        token,
        deadline,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct CampaignUpdatedEvent {
    pub campaign_id: u64,
    pub merchant: Address,
    pub title: String,
    pub description: String,
    pub timestamp: u64,
}

pub fn publish_campaign_updated_event(
    env: &Env,
    campaign_id: u64,
    merchant: Address,
    title: String,
    description: String,
    timestamp: u64,
) {
    CampaignUpdatedEvent {
        campaign_id,
        merchant,
        title,
        description,
        timestamp,
/// Emitted each time a registered signer approves a proposal.
#[contractevent]
pub struct WithdrawalApprovedEvent {
    pub proposal_id: u64,
    /// Signer that cast this approval.
    pub signer: Address,
    /// Running approval count after this vote.
    pub approvals_so_far: u32,
    /// Quorum still needed (0 means ready to execute).
    pub quorum_required: u32,
    pub timestamp: u64,
}

pub fn publish_withdrawal_approved_event(
    env: &Env,
    proposal_id: u64,
    signer: Address,
    approvals_so_far: u32,
    quorum_required: u32,
    timestamp: u64,
) {
    WithdrawalApprovedEvent {
        proposal_id,
        signer,
        approvals_so_far,
        quorum_required,
#[contractevent]
pub struct AutoWithdrawalTriggeredEvent {
    pub merchant_id: u64,
    pub token: Address,
    pub amount: i128,
    pub recipient: Address,
}

pub fn publish_auto_withdrawal_triggered_event(
    env: &Env,
    merchant_id: u64,
    token: Address,
    amount: i128,
    recipient: Address,
) {
    AutoWithdrawalTriggeredEvent {
        merchant_id,
        token,
        amount,
        recipient,
    }
    .publish(env);
}

#[contractevent]
pub struct CampaignStatusChangedEvent {
    pub campaign_id: u64,
    pub merchant: Address,
    pub active: bool,
    pub timestamp: u64,
}

pub fn publish_campaign_status_changed_event(
    env: &Env,
    campaign_id: u64,
    merchant: Address,
    active: bool,
    timestamp: u64,
) {
    CampaignStatusChangedEvent {
        campaign_id,
        merchant,
        active,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct CampaignTagAddedEvent {
    pub campaign_id: u64,
    pub merchant: Address,
    pub tag_id: u64,
    pub timestamp: u64,
}

pub fn publish_campaign_tag_added_event(
    env: &Env,
    campaign_id: u64,
    merchant: Address,
    tag_id: u64,
    timestamp: u64,
) {
    CampaignTagAddedEvent {
        campaign_id,
        merchant,
        tag_id,
// ── Escrow expired-refund event ────────────────────────────────────────────────

/// Emitted when a subscription plan query is executed.
#[contractevent]
pub struct SubscriptionPlanSearchExecutedEvent {
    pub caller: Address,
    pub result_count: u32,
    pub timestamp: u64,
}

pub fn publish_subscription_plan_search_event(
    env: &Env,
    caller: Address,
    result_count: u32,
    timestamp: u64,
) {
    SubscriptionPlanSearchExecutedEvent {
        caller,
        result_count,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct CampaignTagRemovedEvent {
    pub campaign_id: u64,
    pub merchant: Address,
    pub tag_id: u64,
    pub timestamp: u64,
}

pub fn publish_campaign_tag_removed_event(
    env: &Env,
    campaign_id: u64,
    merchant: Address,
    tag_id: u64,
    timestamp: u64,
) {
    CampaignTagRemovedEvent {
        campaign_id,
        merchant,
        tag_id,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct CampaignContributionEvent {
    pub campaign_id: u64,
    pub contributor: Address,
    pub amount: i128,
    pub raised_amount: i128,
    pub goal_amount: i128,
    pub timestamp: u64,
}

pub fn publish_campaign_contribution_event(
    env: &Env,
    campaign_id: u64,
    contributor: Address,
    amount: i128,
    raised_amount: i128,
    goal_amount: i128,
    timestamp: u64,
) {
    CampaignContributionEvent {
        campaign_id,
        contributor,
        amount,
        raised_amount,
        goal_amount,
/// Emitted when an event (ticketing) query is executed.
#[contractevent]
pub struct EventSearchExecutedEvent {
    pub caller: Address,
    pub result_count: u32,
    pub timestamp: u64,
}

pub fn publish_event_search_executed_event(
    env: &Env,
    caller: Address,
    result_count: u32,
    timestamp: u64,
) {
    EventSearchExecutedEvent {
        caller,
        result_count,
#[contractevent]
pub struct EscrowExpiredRefundEvent {
    pub invoice_id: u64,
    pub buyer: Address,
    pub amount: i128,
    pub token: Address,
    pub timestamp: u64,
}

pub fn publish_escrow_expired_refund_event(
    env: &Env,
    invoice_id: u64,
    buyer: Address,
    amount: i128,
    token: Address,
    timestamp: u64,
) {
    EscrowExpiredRefundEvent {
        invoice_id,
        buyer,
        amount,
        token,
        timestamp,
    }
    .publish(env);
}

// ── Campaign financial penalties & slashing events (#360) ─────────────────────

#[contractevent]
pub struct CampaignStakedEvent {
    pub campaign_id: u64,
    pub participant: Address,
    pub amount: i128,
    pub total_staked: i128,
    pub timestamp: u64,
}

pub fn publish_campaign_staked_event(
    env: &Env,
    campaign_id: u64,
    participant: Address,
    amount: i128,
    total_staked: i128,
    timestamp: u64,
) {
    CampaignStakedEvent {
        campaign_id,
        participant,
        amount,
        total_staked,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct CampaignSlashedEvent {
    pub campaign_id: u64,
    pub participant: Address,
    pub amount: i128,
    pub remaining_stake: i128,
    pub timestamp: u64,
}

pub fn publish_campaign_slashed_event(
    env: &Env,
    campaign_id: u64,
    participant: Address,
    amount: i128,
    remaining_stake: i128,
    timestamp: u64,
) {
    CampaignSlashedEvent {
        campaign_id,
        participant,
        amount,
        remaining_stake,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct AffiliateRegisteredEvent {
    pub campaign_id: u64,
    pub affiliate: Address,
    pub commission_bps: u32,
    pub timestamp: u64,
}

pub fn publish_affiliate_registered_event(
    env: &Env,
    campaign_id: u64,
    affiliate: Address,
    commission_bps: u32,
    timestamp: u64,
) {
    AffiliateRegisteredEvent {
        campaign_id,
        affiliate,
        commission_bps,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct AffiliateCommissionPaidEvent {
    pub campaign_id: u64,
    pub affiliate: Address,
    pub amount: i128,
    pub total_paid: i128,
    pub timestamp: u64,
}

pub fn publish_affiliate_commission_paid_event(
    env: &Env,
    campaign_id: u64,
    affiliate: Address,
    amount: i128,
    total_paid: i128,
    timestamp: u64,
) {
    AffiliateCommissionPaidEvent {
        campaign_id,
        affiliate,
        amount,
        total_paid,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct CampaignPenaltyReportedEvent {
    pub report_id: u64,
    pub campaign_id: u64,
    pub reporter: Address,
    pub reason: String,
    pub suggested_penalty: i128,
    pub timestamp: u64,
}

pub fn publish_campaign_penalty_reported_event(
    env: &Env,
    report_id: u64,
    campaign_id: u64,
    reporter: Address,
    reason: String,
    suggested_penalty: i128,
    timestamp: u64,
) {
    CampaignPenaltyReportedEvent {
        report_id,
        campaign_id,
        reporter,
        reason,
        suggested_penalty,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct CampaignPenaltyResolvedEvent {
    pub report_id: u64,
    pub campaign_id: u64,
    pub resolved_by: Address,
    pub upheld: bool,
    pub applied_penalty: i128,
    pub timestamp: u64,
}

pub fn publish_campaign_penalty_resolved_event(
    env: &Env,
    report_id: u64,
    campaign_id: u64,
    resolved_by: Address,
    upheld: bool,
    applied_penalty: i128,
    timestamp: u64,
) {
    CampaignPenaltyResolvedEvent {
        report_id,
        campaign_id,
        resolved_by,
        upheld,
        applied_penalty,
        timestamp,
    }
    .publish(env);
}
