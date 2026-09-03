//! Contract events.
//!
//! Note: `#[contractevent]` derives the event topic Symbol from the struct name
//! in snake_case, and Soroban caps Symbols at 32 characters. A struct name whose
//! snake_case form exceeds 32 chars panics at compile time, so event names here
//! are deliberately kept short.

use crate::types::{AnalyticsExport, CampaignStats, ExportFormat};
use soroban_sdk::{contractevent, Address, BytesN, Env, String, Vec};

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
pub struct PlatformFeeClearedEvent {
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
    PlatformFeeClearedEvent {
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

pub fn publish_cross_chain_pledge_created(env: &Env, pledge: &crate::types::CrossChainPledge) {
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
    .publish(env);
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

pub fn publish_cross_chain_pledge_updated(env: &Env, pledge: &crate::types::CrossChainPledge) {
    CrossChainPledgeUpdatedEvent {
        pledge_id: pledge.id,
        status: pledge.status.clone(),
        timestamp: pledge.updated_at,
    }
    .publish(env);
}

#[contractevent]
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
        timestamp,
    }
    .publish(env);
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
    }
    .publish(env)
}
// ── Auto-withdrawal events ─────────────────────────────────────────────────────

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
    }
    .publish(env);
}

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
        timestamp,
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
        timestamp,
    }
    .publish(env);
}

/// Emitted when a subscription plan query is executed.
#[contractevent]
pub struct PlanSearchExecutedEvent {
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
    PlanSearchExecutedEvent {
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
        timestamp,
    }
    .publish(env);
}

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
        timestamp,
    }
    .publish(env);
}

// ── Escrow expired-refund event ───────────────────────────────────────────────

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
// ── Stretch goals ─────────────────────────────────────────────────────────────
//
// Each event carries the campaign_id alongside the goal_id so indexers can build
// a campaign's milestone ladder without a second lookup, and carries the running
// totals so a UI can render progress from the event stream alone.

/// Emitted when a merchant defines a new stretch goal on a campaign.
#[contractevent]
pub struct StretchGoalCreatedEvent {
    pub goal_id: u64,
    pub campaign_id: u64,
    /// Campaign owner; the only address permitted to manage this goal.
    pub merchant: Address,
    /// Cumulative campaign raise that unlocks this goal.
    pub target_amount: i128,
    /// The campaign's base funding goal, for computing the stretch delta.
    pub base_goal_amount: i128,
    /// Number of goals on the campaign after this one was added.
    pub goal_count: u32,
    pub timestamp: u64,
}

pub fn publish_stretch_goal_created_event(
    env: &Env,
    goal_id: u64,
    campaign_id: u64,
    merchant: Address,
    target_amount: i128,
    base_goal_amount: i128,
    goal_count: u32,
    timestamp: u64,
) {
    StretchGoalCreatedEvent {
        goal_id,
        campaign_id,
        merchant,
        target_amount,
        base_goal_amount,
        goal_count,
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
/// Emitted when a campaign's raise reaches a goal's target and it goes live.
#[contractevent]
pub struct StretchGoalUnlockedEvent {
    pub goal_id: u64,
    pub campaign_id: u64,
    pub merchant: Address,
    pub target_amount: i128,
    /// Campaign raise at the moment of unlocking; always >= target_amount.
    pub raised_amount: i128,
    pub timestamp: u64,
}

pub fn publish_stretch_goal_unlocked_event(
    env: &Env,
    goal_id: u64,
    campaign_id: u64,
    merchant: Address,
    target_amount: i128,
    raised_amount: i128,
    timestamp: u64,
) {
    StretchGoalUnlockedEvent {
        goal_id,
        campaign_id,
        merchant,
        target_amount,
        raised_amount,
        timestamp,
    }
    .publish(env);
}

/// Emitted when a merchant retires a goal before it was unlocked.
#[contractevent]
pub struct StretchGoalCancelledEvent {
    pub goal_id: u64,
    pub campaign_id: u64,
    pub merchant: Address,
    pub timestamp: u64,
}

pub fn publish_stretch_goal_cancelled_event(
    env: &Env,
    goal_id: u64,
    campaign_id: u64,
    merchant: Address,
    timestamp: u64,
) {
    StretchGoalCancelledEvent {
        goal_id,
        campaign_id,
        merchant,
        timestamp,
    }
    .publish(env);
}

/// Emitted when a backer is granted a reward for an unlocked goal.
#[contractevent]
pub struct StretchRewardGrantedEvent {
    pub goal_id: u64,
    pub campaign_id: u64,
    pub backer: Address,
    pub reward_amount: i128,
    /// Number of backers rewarded on this goal after this grant.
    pub reward_count: u32,
    /// Sum of all reward amounts granted on this goal after this grant.
    pub total_reward_amount: i128,
    pub timestamp: u64,
}

pub fn publish_stretch_reward_granted_event(
    env: &Env,
    goal_id: u64,
    campaign_id: u64,
    backer: Address,
    reward_amount: i128,
    reward_count: u32,
    total_reward_amount: i128,
    timestamp: u64,
) {
    StretchRewardGrantedEvent {
        goal_id,
        campaign_id,
        backer,
        reward_amount,
        reward_count,
        total_reward_amount,
        timestamp,
    }
    .publish(env);
}

/// Emitted when a backer claims their granted reward.
#[contractevent]
pub struct StretchRewardClaimedEvent {
    pub goal_id: u64,
    pub campaign_id: u64,
    pub backer: Address,
    pub reward_amount: i128,
    pub timestamp: u64,
}

pub fn publish_stretch_reward_claimed_event(
    env: &Env,
    goal_id: u64,
    campaign_id: u64,
    backer: Address,
    reward_amount: i128,
    timestamp: u64,
) {
    StretchRewardClaimedEvent {
        goal_id,
        campaign_id,
        backer,
        reward_amount,
        timestamp,
    }
    .publish(env);
}

// ── Events restored after merge damage ────────────────────────────────────────

#[contractevent]
pub struct AffiliateCommissionPaidEvent {
    pub campaign_id: u64,
    pub affiliate_address: Address,
    pub amount: i128,
    pub total_paid: i128,
    pub timestamp: u64,
}

pub fn publish_affiliate_commission_paid_event(
    env: &Env,
    campaign_id: u64,
    affiliate_address: Address,
    amount: i128,
    total_paid: i128,
    timestamp: u64,
) {
    AffiliateCommissionPaidEvent {
        campaign_id,
        affiliate_address,
        amount,
        total_paid,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct AffiliateRegisteredEvent {
    pub campaign_id: u64,
    pub affiliate: Address,
    pub affiliate_address: Address,
    pub commission_bps: u32,
    pub timestamp: u64,
}

pub fn publish_affiliate_registered_event(
    env: &Env,
    campaign_id: u64,
    affiliate: Address,
    affiliate_address: Address,
    commission_bps: u32,
    timestamp: u64,
) {
    AffiliateRegisteredEvent {
        campaign_id,
        affiliate,
        affiliate_address,
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
pub struct BackerCampaignCreatedEvent {
    pub campaign_id: u64,
    pub merchant_addr: Address,
    pub merchant_id: u64,
    pub name: String,
    pub token: Address,
    pub deadline: u64,
    pub timestamp: u64,
}

pub fn publish_backer_campaign_created_event(
    env: &Env,
    campaign_id: u64,
    merchant_addr: Address,
    merchant_id: u64,
    name: String,
    token: Address,
    deadline: u64,
    timestamp: u64,
) {
    BackerCampaignCreatedEvent {
        campaign_id,
        merchant_addr,
        merchant_id,
        name,
        token,
        deadline,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct BackerCommentCreatedEvent {
    pub comment_id: u64,
    pub crowdfund_id: u64,
    pub author: Address,
    pub content_len: u64,
    pub now: u64,
}

pub fn publish_backer_comment_created_event(
    env: &Env,
    comment_id: u64,
    crowdfund_id: u64,
    author: Address,
    content_len: u64,
    now: u64,
) {
    BackerCommentCreatedEvent {
        comment_id,
        crowdfund_id,
        author,
        content_len,
        now,
    }
    .publish(env);
}

#[contractevent]
pub struct BackerCommentFlaggedEvent {
    pub comment_id: u64,
    pub flagger: Address,
    pub reason_len: u64,
    pub flag_count: u32,
    pub now: u64,
}

pub fn publish_backer_comment_flagged_event(
    env: &Env,
    comment_id: u64,
    flagger: Address,
    reason_len: u64,
    flag_count: u32,
    now: u64,
) {
    BackerCommentFlaggedEvent {
        comment_id,
        flagger,
        reason_len,
        flag_count,
        now,
    }
    .publish(env);
}

#[contractevent]
pub struct BackerCommentRemovedEvent {
    pub comment_id: u64,
    pub crowdfund_id: u64,
    pub moderator: Address,
    pub now: u64,
}

pub fn publish_backer_comment_removed_event(
    env: &Env,
    comment_id: u64,
    crowdfund_id: u64,
    moderator: Address,
    now: u64,
) {
    BackerCommentRemovedEvent {
        comment_id,
        crowdfund_id,
        moderator,
        now,
    }
    .publish(env);
}

#[contractevent]
pub struct BackerPerkClaimedEvent {
    pub campaign_id: u64,
    pub backer: Address,
    pub tier_index: u32,
    pub perk_index: u32,
    pub name: String,
    pub timestamp: u64,
}

pub fn publish_backer_perk_claimed_event(
    env: &Env,
    campaign_id: u64,
    backer: Address,
    tier_index: u32,
    perk_index: u32,
    name: String,
    timestamp: u64,
) {
    BackerPerkClaimedEvent {
        campaign_id,
        backer,
        tier_index,
        perk_index,
        name,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct BackerPledgeRecordedEvent {
    pub campaign_id: u64,
    pub backer: Address,
    pub amount: i128,
    pub new_pledge: i128,
    pub timestamp: u64,
}

pub fn publish_backer_pledge_recorded_event(
    env: &Env,
    campaign_id: u64,
    backer: Address,
    amount: i128,
    new_pledge: i128,
    timestamp: u64,
) {
    BackerPledgeRecordedEvent {
        campaign_id,
        backer,
        amount,
        new_pledge,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct BackerRewardFulfilledEvent {
    pub campaign_id: u64,
    pub merchant_addr: Address,
    pub backer: Address,
    pub tier_index: u32,
    pub pledge: i128,
    pub timestamp: u64,
}

pub fn publish_backer_reward_fulfilled_event(
    env: &Env,
    campaign_id: u64,
    merchant_addr: Address,
    backer: Address,
    tier_index: u32,
    pledge: i128,
    timestamp: u64,
) {
    BackerRewardFulfilledEvent {
        campaign_id,
        merchant_addr,
        backer,
        tier_index,
        pledge,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct BackerTierSelectedEvent {
    pub campaign_id: u64,
    pub backer: Address,
    pub tier_index: u32,
    pub min_pledge: i128,
    pub tier_len: u32,
    pub timestamp: u64,
}

pub fn publish_backer_reward_tier_selected_event(
    env: &Env,
    campaign_id: u64,
    backer: Address,
    tier_index: u32,
    min_pledge: i128,
    tier_len: u32,
    timestamp: u64,
) {
    BackerTierSelectedEvent {
        campaign_id,
        backer,
        tier_index,
        min_pledge,
        tier_len,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct BackerRewardTiersSetEvent {
    pub campaign_id: u64,
    pub merchant_addr: Address,
    pub tiers_len: u32,
    pub timestamp: u64,
}

pub fn publish_backer_reward_tiers_set_event(
    env: &Env,
    campaign_id: u64,
    merchant_addr: Address,
    tiers_len: u32,
    timestamp: u64,
) {
    BackerRewardTiersSetEvent {
        campaign_id,
        merchant_addr,
        tiers_len,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct CampaignBatchRefundedEvent {
    pub campaign_id: u64,
    pub total_refunded: i128,
    pub count: u32,
    pub timestamp: u64,
}

pub fn publish_campaign_batch_refunded_event(
    env: &Env,
    campaign_id: u64,
    total_refunded: i128,
    count: u32,
    timestamp: u64,
) {
    CampaignBatchRefundedEvent {
        campaign_id,
        total_refunded,
        count,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct CampaignCancelledEvent {
    pub campaign_id: u64,
    pub merchant_address: Address,
    pub timestamp: u64,
}

pub fn publish_campaign_cancelled_event(
    env: &Env,
    campaign_id: u64,
    merchant_address: Address,
    timestamp: u64,
) {
    CampaignCancelledEvent {
        campaign_id,
        merchant_address,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct CampaignContribRecordedEvent {
    pub campaign_id: u64,
    pub caller: Address,
    pub amount: i128,
    pub total_raised: i128,
    pub timestamp: u64,
}

pub fn publish_campaign_contribution_recorded_event(
    env: &Env,
    campaign_id: u64,
    caller: Address,
    amount: i128,
    total_raised: i128,
    timestamp: u64,
) {
    CampaignContribRecordedEvent {
        campaign_id,
        caller,
        amount,
        total_raised,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct CampaignExecutedEvent {
    pub campaign_id: u64,
    pub merchant_address: Address,
    pub raised: i128,
    pub timestamp: u64,
}

pub fn publish_campaign_executed_event(
    env: &Env,
    campaign_id: u64,
    merchant_address: Address,
    raised: i128,
    timestamp: u64,
) {
    CampaignExecutedEvent {
        campaign_id,
        merchant_address,
        raised,
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
pub struct CampaignFeePolicySetEvent {
    pub campaign_id: u64,
    pub caller: Address,
    pub fee_waiver_bps: u32,
    pub discount_bps: u32,
    pub timestamp: u64,
}

pub fn publish_campaign_fee_policy_configured_event(
    env: &Env,
    campaign_id: u64,
    caller: Address,
    fee_waiver_bps: u32,
    discount_bps: u32,
    timestamp: u64,
) {
    CampaignFeePolicySetEvent {
        campaign_id,
        caller,
        fee_waiver_bps,
        discount_bps,
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
pub struct CampaignSlashedEvent {
    pub campaign_id: u64,
    pub participant_address: Address,
    pub amount: i128,
    pub staked: i128,
    pub timestamp: u64,
}

pub fn publish_campaign_slashed_event(
    env: &Env,
    campaign_id: u64,
    participant_address: Address,
    amount: i128,
    staked: i128,
    timestamp: u64,
) {
    CampaignSlashedEvent {
        campaign_id,
        participant_address,
        amount,
        staked,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct CampaignStakedEvent {
    pub campaign_id: u64,
    pub caller: Address,
    pub amount: i128,
    pub staked: i128,
    pub timestamp: u64,
}

pub fn publish_campaign_staked_event(
    env: &Env,
    campaign_id: u64,
    caller: Address,
    amount: i128,
    staked: i128,
    timestamp: u64,
) {
    CampaignStakedEvent {
        campaign_id,
        caller,
        amount,
        staked,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct CommentModerationAppliedEvent {
    pub comment_id: u64,
    pub moderator: Address,
    pub action: String,
    pub now: u64,
}

pub fn publish_comment_moderation_applied_event(
    env: &Env,
    comment_id: u64,
    moderator: Address,
    action: String,
    now: u64,
) {
    CommentModerationAppliedEvent {
        comment_id,
        moderator,
        action,
        now,
    }
    .publish(env);
}

#[contractevent]
pub struct CrowdfundVestingSetEvent {
    pub crowdfund_id: u64,
    pub timeline_id: u64,
    pub total_vesting_amount: i128,
    pub admin: Address,
    pub now: u64,
}

pub fn publish_crowdfund_vesting_configured_event(
    env: &Env,
    crowdfund_id: u64,
    timeline_id: u64,
    total_vesting_amount: i128,
    admin: Address,
    now: u64,
) {
    CrowdfundVestingSetEvent {
        crowdfund_id,
        timeline_id,
        total_vesting_amount,
        admin,
        now,
    }
    .publish(env);
}

#[contractevent]
pub struct DynamicHardCapUpdatedEvent {
    pub crowdfund_id: u64,
    pub proposed_cap: i128,
    pub current_cap: i128,
    pub now: u64,
}

pub fn publish_dynamic_hard_cap_updated_event(
    env: &Env,
    crowdfund_id: u64,
    proposed_cap: i128,
    current_cap: i128,
    now: u64,
) {
    DynamicHardCapUpdatedEvent {
        crowdfund_id,
        proposed_cap,
        current_cap,
        now,
    }
    .publish(env);
}

#[contractevent]
pub struct EscrowCreatedEvent {
    pub id: u64,
    pub seller: Address,
    pub buyer: Address,
    pub token: Address,
    pub amount: i128,
    pub invoice_id: Option<u64>,
    pub timestamp: u64,
}

pub fn publish_escrow_created_event(
    env: &Env,
    id: u64,
    seller: Address,
    buyer: Address,
    token: Address,
    amount: i128,
    invoice_id: Option<u64>,
    timestamp: u64,
) {
    EscrowCreatedEvent {
        id,
        seller,
        buyer,
        token,
        amount,
        invoice_id,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct EscrowFundedEvent {
    pub escrow_id: u64,
    pub buyer: Address,
    pub seller: Address,
    pub token: Address,
    pub amount: i128,
    pub timestamp: u64,
}

pub fn publish_escrow_funded_event(
    env: &Env,
    escrow_id: u64,
    buyer: Address,
    seller: Address,
    token: Address,
    amount: i128,
    timestamp: u64,
) {
    EscrowFundedEvent {
        escrow_id,
        buyer,
        seller,
        token,
        amount,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct EscrowRefundedEvent {
    pub escrow_id: u64,
    pub seller: Address,
    pub buyer: Address,
    pub token: Address,
    pub amount: i128,
    pub timestamp: u64,
}

pub fn publish_escrow_refunded_event(
    env: &Env,
    escrow_id: u64,
    seller: Address,
    buyer: Address,
    token: Address,
    amount: i128,
    timestamp: u64,
) {
    EscrowRefundedEvent {
        escrow_id,
        seller,
        buyer,
        token,
        amount,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct EscrowReleasedEvent {
    pub escrow_id: u64,
    pub buyer: Address,
    pub seller: Address,
    pub token: Address,
    pub merchant_amount: i128,
    pub fee: i128,
    pub timestamp: u64,
}

pub fn publish_escrow_released_event(
    env: &Env,
    escrow_id: u64,
    buyer: Address,
    seller: Address,
    token: Address,
    merchant_amount: i128,
    fee: i128,
    timestamp: u64,
) {
    EscrowReleasedEvent {
        escrow_id,
        buyer,
        seller,
        token,
        merchant_amount,
        fee,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct HardCapVotedEvent {
    pub crowdfund_id: u64,
    pub voter: Address,
    pub proposed_cap: i128,
    pub now: u64,
}

pub fn publish_hard_cap_voted_event(
    env: &Env,
    crowdfund_id: u64,
    voter: Address,
    proposed_cap: i128,
    now: u64,
) {
    HardCapVotedEvent {
        crowdfund_id,
        voter,
        proposed_cap,
        now,
    }
    .publish(env);
}

#[contractevent]
pub struct HardCapVotingFinalizedEvent {
    pub crowdfund_id: u64,
    pub votes_for: u32,
    pub votes_against: u32,
    pub votes_passed: bool,
    pub current_cap: i128,
    pub now: u64,
}

pub fn publish_hard_cap_voting_finalized_event(
    env: &Env,
    crowdfund_id: u64,
    votes_for: u32,
    votes_against: u32,
    votes_passed: bool,
    current_cap: i128,
    now: u64,
) {
    HardCapVotingFinalizedEvent {
        crowdfund_id,
        votes_for,
        votes_against,
        votes_passed,
        current_cap,
        now,
    }
    .publish(env);
}

#[contractevent]
pub struct HardCapVotingInitiatedEvent {
    pub crowdfund_id: u64,
    pub proposed_cap: i128,
    pub voting_duration: u64,
    pub voting_end: u64,
    pub now: u64,
}

pub fn publish_hard_cap_voting_initiated_event(
    env: &Env,
    crowdfund_id: u64,
    proposed_cap: i128,
    voting_duration: u64,
    voting_end: u64,
    now: u64,
) {
    HardCapVotingInitiatedEvent {
        crowdfund_id,
        proposed_cap,
        voting_duration,
        voting_end,
        now,
    }
    .publish(env);
}

#[contractevent]
pub struct InvoiceSearchExecutedEvent {
    pub caller: Address,
    pub count: u32,
    pub has_next: bool,
    pub timestamp: u64,
}

pub fn publish_invoice_search_executed_event(
    env: &Env,
    caller: Address,
    count: u32,
    has_next: bool,
    timestamp: u64,
) {
    InvoiceSearchExecutedEvent {
        caller,
        count,
        has_next,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct MultisigConfiguredEvent {
    pub signers: Vec<Address>,
    pub quorum: u32,
    pub admin: Address,
    pub timestamp: u64,
}

pub fn publish_multisig_configured_event(
    env: &Env,
    signers: Vec<Address>,
    quorum: u32,
    admin: Address,
    timestamp: u64,
) {
    MultisigConfiguredEvent {
        signers,
        quorum,
        admin,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct MultisigThresholdSetEvent {
    pub token: Address,
    pub threshold: i128,
    pub admin: Address,
    pub timestamp: u64,
}

pub fn publish_multisig_threshold_set_event(
    env: &Env,
    token: Address,
    threshold: i128,
    admin: Address,
    timestamp: u64,
) {
    MultisigThresholdSetEvent {
        token,
        threshold,
        admin,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct NftBatchMintedEvent {
    pub collection_id: u64,
    pub merchant_id: u64,
    pub count: u32,
    pub timestamp: u64,
}

pub fn publish_nft_batch_minted_event(
    env: &Env,
    collection_id: u64,
    merchant_id: u64,
    count: u32,
    timestamp: u64,
) {
    NftBatchMintedEvent {
        collection_id,
        merchant_id,
        count,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct NftBurnedEvent {
    pub nft_id: u64,
    pub collection_id: u64,
    pub owner: Address,
    pub timestamp: u64,
}

pub fn publish_nft_burned_event(
    env: &Env,
    nft_id: u64,
    collection_id: u64,
    owner: Address,
    timestamp: u64,
) {
    NftBurnedEvent {
        nft_id,
        collection_id,
        owner,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct NftCollectionCreatedEvent {
    pub id: u64,
    pub merchant_id: u64,
    pub merchant_addr: Address,
    pub name: String,
    pub base_uri: String,
    pub max_supply: u64,
    pub royalty_bps: u32,
    pub timestamp: u64,
}

pub fn publish_nft_collection_created_event(
    env: &Env,
    id: u64,
    merchant_id: u64,
    merchant_addr: Address,
    name: String,
    base_uri: String,
    max_supply: u64,
    royalty_bps: u32,
    timestamp: u64,
) {
    NftCollectionCreatedEvent {
        id,
        merchant_id,
        merchant_addr,
        name,
        base_uri,
        max_supply,
        royalty_bps,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct NftCollectionDeactivatedEvent {
    pub collection_id: u64,
    pub merchant_addr: Address,
    pub timestamp: u64,
}

pub fn publish_nft_collection_deactivated_event(
    env: &Env,
    collection_id: u64,
    merchant_addr: Address,
    timestamp: u64,
) {
    NftCollectionDeactivatedEvent {
        collection_id,
        merchant_addr,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct NftMintedEvent {
    pub nft_id: u64,
    pub collection_id: u64,
    pub merchant_id: u64,
    pub recipient: Address,
    pub token_uri: String,
    pub timestamp: u64,
}

pub fn publish_nft_minted_event(
    env: &Env,
    nft_id: u64,
    collection_id: u64,
    merchant_id: u64,
    recipient: Address,
    token_uri: String,
    timestamp: u64,
) {
    NftMintedEvent {
        nft_id,
        collection_id,
        merchant_id,
        recipient,
        token_uri,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct NftRewardClaimedEvent {
    pub nft_id: u64,
    pub collection_id: u64,
    pub claimer: Address,
    pub timestamp: u64,
}

pub fn publish_nft_reward_claimed_event(
    env: &Env,
    nft_id: u64,
    collection_id: u64,
    claimer: Address,
    timestamp: u64,
) {
    NftRewardClaimedEvent {
        nft_id,
        collection_id,
        claimer,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct NftTransferredEvent {
    pub nft_id: u64,
    pub collection_id: u64,
    pub from: Address,
    pub to: Address,
    pub timestamp: u64,
}

pub fn publish_nft_transferred_event(
    env: &Env,
    nft_id: u64,
    collection_id: u64,
    from: Address,
    to: Address,
    timestamp: u64,
) {
    NftTransferredEvent {
        nft_id,
        collection_id,
        from,
        to,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct PledgeMadeEvent {
    pub pledge_id: u64,
    pub campaign_id: u64,
    pub contributor: Address,
    pub amount: i128,
    pub token: Address,
    pub timestamp: u64,
}

pub fn publish_pledge_made_event(
    env: &Env,
    pledge_id: u64,
    campaign_id: u64,
    contributor: Address,
    amount: i128,
    token: Address,
    timestamp: u64,
) {
    PledgeMadeEvent {
        pledge_id,
        campaign_id,
        contributor,
        amount,
        token,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct PledgeRefundedEvent {
    pub pledge_id: u64,
    pub campaign_id: u64,
    pub contributor: Address,
    pub amount: i128,
    pub token: Address,
    pub timestamp: u64,
}

pub fn publish_pledge_refunded_event(
    env: &Env,
    pledge_id: u64,
    campaign_id: u64,
    contributor: Address,
    amount: i128,
    token: Address,
    timestamp: u64,
) {
    PledgeRefundedEvent {
        pledge_id,
        campaign_id,
        contributor,
        amount,
        token,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct TicketListedEvent {
    pub ticket_id: u64,
    pub seller: Address,
    pub price: i128,
    pub timestamp: u64,
}

pub fn publish_ticket_listed_event(
    env: &Env,
    ticket_id: u64,
    seller: Address,
    price: i128,
    timestamp: u64,
) {
    TicketListedEvent {
        ticket_id,
        seller,
        price,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct TicketListingCancelledEvent {
    pub ticket_id: u64,
    pub seller: Address,
    pub timestamp: u64,
}

pub fn publish_ticket_listing_cancelled_event(
    env: &Env,
    ticket_id: u64,
    seller: Address,
    timestamp: u64,
) {
    TicketListingCancelledEvent {
        ticket_id,
        seller,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct TicketListingSoldEvent {
    pub ticket_id: u64,
    pub seller: Address,
    pub buyer: Address,
    pub resale_price: i128,
    pub royalty: i128,
    pub timestamp: u64,
}

pub fn publish_ticket_listing_sold_event(
    env: &Env,
    ticket_id: u64,
    seller: Address,
    buyer: Address,
    resale_price: i128,
    royalty: i128,
    timestamp: u64,
) {
    TicketListingSoldEvent {
        ticket_id,
        seller,
        buyer,
        resale_price,
        royalty,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct VestingScheduleReleasedEvent {
    pub timeline_id: u64,
    pub tranche_index: u64,
    pub unlock_amount: i128,
    pub now: u64,
}

pub fn publish_vesting_schedule_released_event(
    env: &Env,
    timeline_id: u64,
    tranche_index: u64,
    unlock_amount: i128,
    now: u64,
) {
    VestingScheduleReleasedEvent {
        timeline_id,
        tranche_index,
        unlock_amount,
        now,
    }
    .publish(env);
}

#[contractevent]
pub struct VestingTimelineCreatedEvent {
    pub timeline_id: u64,
    pub name: String,
    pub cliff_duration: u64,
    pub vesting_duration: u64,
    pub admin: Address,
    pub now: u64,
}

pub fn publish_vesting_timeline_created_event(
    env: &Env,
    timeline_id: u64,
    name: String,
    cliff_duration: u64,
    vesting_duration: u64,
    admin: Address,
    now: u64,
) {
    VestingTimelineCreatedEvent {
        timeline_id,
        name,
        cliff_duration,
        vesting_duration,
        admin,
        now,
    }
    .publish(env);
}

#[contractevent]
pub struct VestingTimelineUpdatedEvent {
    pub timeline_id: u64,
    pub cliff_duration: u64,
    pub vesting_duration: u64,
    pub admin: Address,
    pub now: u64,
}

pub fn publish_vesting_timeline_updated_event(
    env: &Env,
    timeline_id: u64,
    cliff_duration: u64,
    vesting_duration: u64,
    admin: Address,
    now: u64,
) {
    VestingTimelineUpdatedEvent {
        timeline_id,
        cliff_duration,
        vesting_duration,
        admin,
        now,
    }
    .publish(env);
}

// ── Creator fund vesting ──────────────────────────────────────────────────────
//
// Every event carries the campaign_id, the creator and the running totals, so an
// indexer can reconstruct a schedule's full payout state — how much has vested,
// been paid and remains — from the event stream alone, with no contract reads.
// Absolute timestamps are emitted alongside the durations they derive from so a
// UI can render the unlock calendar without redoing the arithmetic.

/// Emitted when a campaign creator commits raised funds to a vesting schedule.
#[contractevent]
pub struct CreatorVestingCreatedEvent {
    pub campaign_id: u64,
    /// Beneficiary of the schedule; the campaign's owning merchant.
    pub creator: Address,
    pub token: Address,
    pub total_amount: i128,
    pub start_time: u64,
    /// Absolute timestamp of the cliff: `start_time + cliff_duration`.
    pub cliff_timestamp: u64,
    /// Absolute timestamp of full vesting: `start_time + vesting_duration`.
    pub end_timestamp: u64,
    pub initial_unlock_bps: u32,
    /// Tokens the cliff releases in one lump; the rest vests linearly after it.
    pub initial_unlock_amount: i128,
    /// The campaign's raise at creation time, for context on the commitment.
    pub campaign_raised: i128,
    pub timestamp: u64,
}

pub fn publish_creator_vesting_created_event(
    env: &Env,
    campaign_id: u64,
    creator: Address,
    token: Address,
    total_amount: i128,
    start_time: u64,
    cliff_timestamp: u64,
    end_timestamp: u64,
    initial_unlock_bps: u32,
    initial_unlock_amount: i128,
    campaign_raised: i128,
    timestamp: u64,
) {
    CreatorVestingCreatedEvent {
        campaign_id,
        creator,
        token,
        total_amount,
        start_time,
        cliff_timestamp,
        end_timestamp,
        initial_unlock_bps,
        initial_unlock_amount,
        campaign_raised,
        timestamp,
    }
    .publish(env);
}

/// Emitted on every payout from a creator's vesting schedule.
#[contractevent]
pub struct CreatorVestingReleasedEvent {
    pub campaign_id: u64,
    pub creator: Address,
    pub token: Address,
    /// Amount transferred by this release.
    pub amount: i128,
    /// Cumulative amount released across every payout so far.
    pub total_released: i128,
    /// Committed total that has vested as of this release.
    pub vested_to_date: i128,
    /// Still to be paid out: `total_amount - total_released`.
    pub remaining_amount: i128,
    /// True when this release drained the schedule.
    pub completed: bool,
    pub timestamp: u64,
}

pub fn publish_creator_vesting_released_event(
    env: &Env,
    campaign_id: u64,
    creator: Address,
    token: Address,
    amount: i128,
    total_released: i128,
    vested_to_date: i128,
    remaining_amount: i128,
    completed: bool,
    timestamp: u64,
) {
    CreatorVestingReleasedEvent {
        campaign_id,
        creator,
        token,
        amount,
        total_released,
        vested_to_date,
        remaining_amount,
        completed,
        timestamp,
    }
    .publish(env);
}

/// Emitted when the admin freezes a schedule. The vested-but-unreleased balance
/// stays claimable by the creator; `forfeited_amount` never vests.
#[contractevent]
pub struct CreatorVestingRevokedEvent {
    pub campaign_id: u64,
    pub creator: Address,
    /// Contract admin that revoked the schedule.
    pub admin: Address,
    /// Amount that had vested at revocation; the schedule's new total.
    pub vested_amount: i128,
    /// Vested but not yet paid out; the creator may still claim this.
    pub unreleased_amount: i128,
    /// Amount that will now never vest.
    pub forfeited_amount: i128,
    pub timestamp: u64,
}

pub fn publish_creator_vesting_revoked_event(
    env: &Env,
    campaign_id: u64,
    creator: Address,
    admin: Address,
    vested_amount: i128,
    unreleased_amount: i128,
    forfeited_amount: i128,
    timestamp: u64,
) {
    CreatorVestingRevokedEvent {
        campaign_id,
        creator,
        admin,
        vested_amount,
        unreleased_amount,
        forfeited_amount,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct WithdrawalCancelledEvent {
    pub proposal_id: u64,
    pub caller: Address,
    pub timestamp: u64,
}

pub fn publish_withdrawal_cancelled_event(
    env: &Env,
    proposal_id: u64,
    caller: Address,
    timestamp: u64,
) {
    WithdrawalCancelledEvent {
        proposal_id,
        caller,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct WithdrawalExecutedEvent {
    pub proposal_id: u64,
    pub merchant: Address,
    pub token: Address,
    pub amount: i128,
    pub recipient: Address,
    pub signer: Address,
    pub now: u64,
}

pub fn publish_withdrawal_executed_event(
    env: &Env,
    proposal_id: u64,
    merchant: Address,
    token: Address,
    amount: i128,
    recipient: Address,
    signer: Address,
    now: u64,
) {
    WithdrawalExecutedEvent {
        proposal_id,
        merchant,
        token,
        amount,
        recipient,
        signer,
        now,
    }
    .publish(env);
}

#[contractevent]
pub struct WithdrawalProposalSearchEvent {
    pub caller: Address,
    pub count: u32,
    pub timestamp: u64,
}

pub fn publish_withdrawal_proposal_search_event(
    env: &Env,
    caller: Address,
    count: u32,
    timestamp: u64,
) {
    WithdrawalProposalSearchEvent {
        caller,
        count,
        timestamp,
    }
    .publish(env);
}

#[contractevent]
pub struct WithdrawalProposedEvent {
    pub proposal_id: u64,
    pub merchant: Address,
    pub token: Address,
    pub amount: i128,
    pub recipient: Address,
    pub quorum: u32,
    pub now: u64,
}

pub fn publish_withdrawal_proposed_event(
    env: &Env,
    proposal_id: u64,
    merchant: Address,
    token: Address,
    amount: i128,
    recipient: Address,
    quorum: u32,
    now: u64,
) {
    WithdrawalProposedEvent {
        proposal_id,
        merchant,
        token,
        amount,
        recipient,
        quorum,
        now,
    }
    .publish(env);
}

/// Emitted when a fee-policy / staking campaign is created (see [`crate::types::FeeCampaign`]).
#[contractevent]
pub struct FeeCampaignCreatedEvent {
    pub campaign_id: u64,
    pub owner: Address,
    pub name: String,
    pub charity: bool,
    pub fee_waiver_bps: u32,
    pub discount_bps: u32,
    pub timestamp: u64,
}

pub fn publish_fee_campaign_created_event(
    env: &Env,
    campaign_id: u64,
    owner: Address,
    name: String,
    charity: bool,
    fee_waiver_bps: u32,
    discount_bps: u32,
    timestamp: u64,
) {
    FeeCampaignCreatedEvent {
        campaign_id,
        owner,
        name,
        charity,
        fee_waiver_bps,
        discount_bps,
        timestamp,
    }
    .publish(env);
}

/// Emitted when an all-or-nothing pledge campaign is created
/// (see [`crate::types::PledgeCampaign`]).
#[contractevent]
pub struct PledgeCampaignCreatedEvent {
    pub campaign_id: u64,
    pub merchant: Address,
    pub merchant_id: u64,
    pub title: String,
    pub goal: i128,
    pub token: Address,
    pub deadline: u64,
    pub timestamp: u64,
}

pub fn publish_pledge_campaign_created_event(
    env: &Env,
    campaign_id: u64,
    merchant: Address,
    merchant_id: u64,
    title: String,
    goal: i128,
    token: Address,
    deadline: u64,
    timestamp: u64,
) {
    PledgeCampaignCreatedEvent {
        campaign_id,
        merchant,
        merchant_id,
        title,
        goal,
        token,
        deadline,
        timestamp,
    }
    .publish(env);
}

// ── Fiat-pegged campaign goals ────────────────────────────────────────────────

/// Emitted when a merchant pegs a campaign's funding target to a fiat currency
/// (see [`crate::types::CampaignFiatGoal`]).
///
/// Carries the oracle and the seed price alongside the target so an indexer can
/// render the campaign, and reproduce every later valuation, from this one event.
#[contractevent]
pub struct CampaignFiatGoalSetEvent {
    pub campaign_id: u64,
    pub merchant: Address,
    pub token: Address,
    pub currency: String,
    pub goal_amount: i128,
    pub decimals: u32,
    pub oracle: Address,
    pub price: i128,
    pub price_decimals: u32,
    /// Token base units the target is worth at `price`, for display only — the
    /// figure moves with the price and is never stored.
    pub token_goal_estimate: i128,
    pub deadline: u64,
    pub timestamp: u64,
}

pub fn publish_campaign_fiat_goal_set_event(
    env: &Env,
    campaign_id: u64,
    merchant: Address,
    token: Address,
    currency: String,
    goal_amount: i128,
    decimals: u32,
    oracle: Address,
    price: i128,
    price_decimals: u32,
    token_goal_estimate: i128,
    deadline: u64,
    timestamp: u64,
) {
    CampaignFiatGoalSetEvent {
        campaign_id,
        merchant,
        token,
        currency,
        goal_amount,
        decimals,
        oracle,
        price,
        price_decimals,
        token_goal_estimate,
        deadline,
        timestamp,
    }
    .publish(env);
}

/// Emitted for every contribution valued against a fiat-pegged goal.
///
/// `price` is the snapshot the contribution was credited at, so the pair
/// (`token_amount`, `fiat_amount`) is independently verifiable after the fact.
#[contractevent]
pub struct FiatContributionEvent {
    pub campaign_id: u64,
    pub contributor: Address,
    pub token: Address,
    pub token_amount: i128,
    pub fiat_amount: i128,
    pub currency: String,
    pub price: i128,
    pub price_decimals: u32,
    pub raised_amount: i128,
    pub goal_amount: i128,
    pub progress_bps: u32,
    pub timestamp: u64,
}

pub fn publish_fiat_contribution_event(
    env: &Env,
    campaign_id: u64,
    contributor: Address,
    token: Address,
    token_amount: i128,
    fiat_amount: i128,
    currency: String,
    price: i128,
    price_decimals: u32,
    raised_amount: i128,
    goal_amount: i128,
    progress_bps: u32,
    timestamp: u64,
) {
    FiatContributionEvent {
        campaign_id,
        contributor,
        token,
        token_amount,
        fiat_amount,
        currency,
        price,
        price_decimals,
        raised_amount,
        goal_amount,
        progress_bps,
        timestamp,
    }
    .publish(env);
}

/// Emitted once, on the contribution that first carries a fiat-pegged goal to
/// its target.
#[contractevent]
pub struct FiatGoalReachedEvent {
    pub campaign_id: u64,
    pub merchant: Address,
    pub currency: String,
    pub goal_amount: i128,
    pub raised_amount: i128,
    /// Token base units it took to get there. Differs from what the target was
    /// originally worth whenever the price moved during the raise.
    pub raised_tokens: i128,
    pub contribution_count: u32,
    pub timestamp: u64,
}

pub fn publish_fiat_goal_reached_event(
    env: &Env,
    campaign_id: u64,
    merchant: Address,
    currency: String,
    goal_amount: i128,
    raised_amount: i128,
    raised_tokens: i128,
    contribution_count: u32,
    timestamp: u64,
) {
    FiatGoalReachedEvent {
        campaign_id,
        merchant,
        currency,
        goal_amount,
        raised_amount,
        raised_tokens,
        contribution_count,
        timestamp,
    }
    .publish(env);
}

/// Emitted when the owning merchant re-reads the oracle to publish a fresh
/// valuation of a fiat-pegged goal, for indexers that track the shortfall
/// between contributions.
#[contractevent]
pub struct FiatGoalQuoteEvent {
    pub campaign_id: u64,
    pub token: Address,
    pub currency: String,
    pub price: i128,
    pub price_decimals: u32,
    pub raised_amount: i128,
    pub goal_amount: i128,
    pub remaining_amount: i128,
    pub tokens_required: i128,
    pub progress_bps: u32,
    pub timestamp: u64,
}

pub fn publish_fiat_goal_quote_event(
    env: &Env,
    campaign_id: u64,
    token: Address,
    currency: String,
    price: i128,
    price_decimals: u32,
    raised_amount: i128,
    goal_amount: i128,
    remaining_amount: i128,
    tokens_required: i128,
    progress_bps: u32,
    timestamp: u64,
) {
    FiatGoalQuoteEvent {
        campaign_id,
        token,
        currency,
        price,
        price_decimals,
        raised_amount,
        goal_amount,
        remaining_amount,
        tokens_required,
        progress_bps,
        timestamp,
    }
    .publish(env);
}

/// Emitted when a fiat-pegged goal is wound down. `goal_reached` records
/// whether it made its target, which is what decides fulfilment off-chain.
#[contractevent]
pub struct FiatGoalClosedEvent {
    pub campaign_id: u64,
    pub caller: Address,
    pub currency: String,
    pub goal_amount: i128,
    pub raised_amount: i128,
    pub raised_tokens: i128,
    pub progress_bps: u32,
    pub goal_reached: bool,
    pub timestamp: u64,
}

pub fn publish_fiat_goal_closed_event(
    env: &Env,
    campaign_id: u64,
    caller: Address,
    currency: String,
    goal_amount: i128,
    raised_amount: i128,
    raised_tokens: i128,
    progress_bps: u32,
    goal_reached: bool,
    timestamp: u64,
) {
    FiatGoalClosedEvent {
        campaign_id,
        caller,
        currency,
        goal_amount,
        raised_amount,
        raised_tokens,
        progress_bps,
        goal_reached,
        timestamp,
    }
    .publish(env);
}

// ── Campaign analytics exports ────────────────────────────────────────────────

/// Emitted on every contribution an analytics-tracked campaign receives.
///
/// Carries the running aggregate rather than just the contribution, so an
/// indexer can keep a campaign's dashboard current from this event alone
/// without replaying the whole pledge history or reading contract state.
#[contractevent]
pub struct CampaignStatsUpdatedEvent {
    pub campaign_id: u64,
    pub backer: Address,
    pub amount: i128,
    /// Whether this contribution came from an address that had never
    /// contributed before, which is what moved `backer_count`.
    pub is_new_backer: bool,
    pub pledge_count: u32,
    pub backer_count: u32,
    pub tracked_raised: i128,
    pub average_pledge: i128,
    pub largest_pledge: i128,
    pub smallest_pledge: i128,
    pub timestamp: u64,
}

pub fn publish_campaign_stats_updated_event(
    env: &Env,
    campaign_id: u64,
    backer: Address,
    amount: i128,
    is_new_backer: bool,
    stats: &CampaignStats,
    average_pledge: i128,
    timestamp: u64,
) {
    CampaignStatsUpdatedEvent {
        campaign_id,
        backer,
        amount,
        is_new_backer,
        pledge_count: stats.pledge_count,
        backer_count: stats.backer_count,
        tracked_raised: stats.tracked_raised,
        average_pledge,
        largest_pledge: stats.largest_pledge,
        smallest_pledge: stats.smallest_pledge,
        timestamp,
    }
    .publish(env);
}

/// Emitted when a creator exports their campaign's analytics.
///
/// Mirrors the stored [`AnalyticsExport`] field for field: the record is the
/// export, and the event is how off-chain tooling picks it up without polling.
/// It carries the full structural context an indexer needs to render a file —
/// which campaign and creator, the requested `format`, the export's position in
/// its series, the window it covers, the cumulative figures, and the delta
/// since the previous export — so no follow-up contract read is required.
#[contractevent]
pub struct AnalyticsExportEvent {
    pub export_id: u64,
    pub campaign_id: u64,
    pub creator: Address,
    pub merchant_id: u64,
    pub token: Address,
    pub format: ExportFormat,
    pub sequence: u32,
    pub period_start: u64,
    pub period_end: u64,
    pub campaign_raised: i128,
    pub campaign_deadline: u64,
    pub campaign_active: bool,
    pub total_raised: i128,
    pub pledge_count: u32,
    pub backer_count: u32,
    pub average_pledge: i128,
    pub largest_pledge: i128,
    pub smallest_pledge: i128,
    pub first_pledge_at: u64,
    pub last_pledge_at: u64,
    pub period_raised: i128,
    pub period_pledges: u32,
    pub period_backers: u32,
    pub timestamp: u64,
}

/// Takes the export record itself rather than two dozen positional arguments:
/// at this field count a mistyped call site would silently transpose figures
/// that no compiler check would catch.
pub fn publish_analytics_export_event(env: &Env, export: &AnalyticsExport) {
    AnalyticsExportEvent {
        export_id: export.id,
        campaign_id: export.campaign_id,
        creator: export.creator.clone(),
        merchant_id: export.merchant_id,
        token: export.token.clone(),
        format: export.format,
        sequence: export.sequence,
        period_start: export.period_start,
        period_end: export.period_end,
        campaign_raised: export.campaign_raised,
        campaign_deadline: export.campaign_deadline,
        campaign_active: export.campaign_active,
        total_raised: export.total_raised,
        pledge_count: export.pledge_count,
        backer_count: export.backer_count,
        average_pledge: export.average_pledge,
        largest_pledge: export.largest_pledge,
        smallest_pledge: export.smallest_pledge,
        first_pledge_at: export.first_pledge_at,
        last_pledge_at: export.last_pledge_at,
        period_raised: export.period_raised,
        period_pledges: export.period_pledges,
        period_backers: export.period_backers,
        timestamp: export.created_at,
    }
    .publish(env);
}
